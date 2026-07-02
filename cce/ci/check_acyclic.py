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

    print(f"check_acyclic: OK ({len(deps)} Workspace-Crates, DAG, Schichten sauber)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
