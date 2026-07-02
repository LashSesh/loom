#!/usr/bin/env python3
"""ci/check_acyclic — INV-11: Der Crate-Abhaengigkeitsgraph ist azyklisch,
und kein Kern-Crate (cce-*) haengt von Adapter-/Aussenschichten (loom-*,
nexus-*, cockpit-*) ab, ausser ueber die deklarierten Ports:
  - cce-runner / cce-observe duerfen den loom-SDK-Port nutzen (LOOM Teil 11).
Fail-closed: jeder Verstoss beendet mit Exitcode 1.
"""
import json
import subprocess
import sys

ALLOWED_CORE_TO_OUTER = {
    # cce-conformance ist der Waechter: die Zeugensuite waechst per Spez
    # mit JEDER Phase (G8 CSA, G8a Inference, G9 .loom) und steht damit
    # OBERHALB aller Schichten — sie ist Pruefling-Konsument, kein Kern.
    "cce-conformance": {"*"},
    # LOOM Teil 11: "kein cce-*-Crate haengt von loom-* ab ausser
    # cce-runner/cce-observe ueber den SDK-Port"
    "cce-runner": {"loom-codec", "loom-verify", "loom-format", "loom-canon"},
    "cce-observe": {"loom-codec", "loom-verify", "loom-format", "loom-canon"},
}


def main() -> int:
    meta = json.loads(
        subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            cwd=sys.path[0] + "/..",
        )
    )
    ws = {p["name"]: p for p in meta["packages"]}
    deps = {
        name: sorted(d["name"] for d in p["dependencies"] if d["name"] in ws)
        for name, p in ws.items()
    }

    # 1. Zyklenfreiheit (Kahn)
    indeg = {n: 0 for n in deps}
    for n, ds in deps.items():
        for d in ds:
            indeg[n] = indeg[n]  # keep key
    # edges: n depends on d  => d -> n
    incoming = {n: set() for n in deps}
    for n, ds in deps.items():
        for d in ds:
            incoming[n].add(d)
    resolved: set = set()
    changed = True
    while changed:
        changed = False
        for n in list(deps):
            if n not in resolved and incoming[n] <= resolved:
                resolved.add(n)
                changed = True
    cyclic = set(deps) - resolved
    if cyclic:
        print(f"INV-11 VERLETZT: Zyklus im Crate-Graph: {sorted(cyclic)}")
        return 1

    # 2. Schichtenregel
    bad = []
    for n, ds in deps.items():
        if n.startswith("cce-"):
            for d in ds:
                if d.startswith(("loom-", "nexus-", "cockpit-")):
                    allowed = ALLOWED_CORE_TO_OUTER.get(n, set())
                    if "*" not in allowed and d not in allowed:
                        bad.append((n, d))
    if bad:
        print(f"INV-11 VERLETZT: Kern haengt ohne Port an Aussenschicht: {bad}")
        return 1

    # 3. Tor-Trennung (IG-A1, Overlay 05): kein Pfad bedient zwei Tore.
    #    source_acquisition (nexus-*) / model_egress (cce-inference) /
    #    tool_egress (cce-toolgateway) sind im Crate-Graph disjunkt.
    gate_bad = []
    for n, ds in deps.items():
        if n in ("cce-inference", "cce-toolgateway"):
            for d in ds:
                if d.startswith("nexus-"):
                    gate_bad.append((n, d, "gateway haengt an CSA-Tor"))
        if n.startswith("nexus-"):
            for d in ds:
                if d in ("cce-inference", "cce-toolgateway"):
                    gate_bad.append((n, d, "CSA-Tor haengt an Gateway"))
        if n == "cce-inference" and "cce-toolgateway" in ds:
            gate_bad.append((n, "cce-toolgateway", "model- und tool-Tor verschmolzen"))
        if n == "cce-toolgateway" and "cce-inference" in ds:
            gate_bad.append((n, "cce-inference", "tool- und model-Tor verschmolzen"))
    if gate_bad:
        print(f"IG-A1 VERLETZT (Tor-Trennung): {gate_bad}")
        return 1

    # 3b. Reader-Prinzip (LOOM Teil 6 / G9-Gate): der Viewer-Pfad ist
    #     MOTORFREI — loom-viewer und seine loom-*-Kette haengen an
    #     keinem cce-*/nexus-*/cockpit-*-Crate.
    viewer_closure = set()
    stack = ["loom-viewer"]
    while stack:
        n = stack.pop()
        if n in viewer_closure or n not in deps:
            continue
        viewer_closure.add(n)
        stack.extend(deps[n])
    motorful = sorted(
        d for d in viewer_closure
        if d.startswith(("cce-", "nexus-", "cockpit-"))
    )
    if motorful:
        print(f"READER-PRINZIP VERLETZT: Viewer-Pfad haengt am Motor: {motorful}")
        return 1

    # 4. Symbol-Scan (F.1 Ausgangs-Gate G8a): kein Modell-/Tool-Socket
    #    ausserhalb der Gateways — der Workspace ist dependency-frei;
    #    jede Socket-/HTTP-Primitive ausserhalb providers/ bzw.
    #    nexus-fetch ist ein Verstoss.
    import os
    root = os.path.join(sys.path[0], "..")
    needles = ("TcpStream", "UdpSocket", "reqwest", "hyper::", "curl", "ureq")
    sock_bad = []
    for base, _dirs, files in os.walk(root):
        if "target" in base.split(os.sep):
            continue
        rel = os.path.relpath(base, root)
        exempt = rel.startswith(
            (os.path.join("crates", "cce-inference", "src", "providers"),
             os.path.join("nexus", "nexus-fetch"))
        )
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = os.path.join(base, f)
            with open(path, encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            for needle in needles:
                if needle in text and not exempt:
                    sock_bad.append((os.path.join(rel, f), needle))
    if sock_bad:
        print(f"G8a VERLETZT: Socket-/HTTP-Symbol ausserhalb der Gateways: {sock_bad}")
        return 1

    print(
        f"check_acyclic: OK ({len(deps)} Workspace-Crates, DAG, Schichten sauber, "
        "Tor-Trennung + Socket-Scan sauber)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
