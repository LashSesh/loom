//! Die 14 Pflichtgates (C.8): boolesch, fail-closed, begruendet;
//! Verdikt allow|hold|reject|quarantine. PromptContextGate laeuft
//! STRIKT VOR jedem Egress-Pfad (F.1 b).

use crate::manifest::{ModelManifest, ReplayPolicy};
use crate::request::InferenceRequest;
use crate::residues::model_residue;
use cce_core::capability::CapabilityLock;
use cce_core::residue::Residue;

/// Vier-Wege-Verdikt (deckungsgleich mit der CSA-Form, C.8).
#[derive(Debug, Clone)]
pub enum InfVerdict {
    Allow { gate: String, reason: String },
    Hold { gate: String, residue: Box<Residue> },
    Reject { gate: String, residue: Box<Residue> },
    Quarantine { gate: String, residue: Box<Residue> },
}

impl InfVerdict {
    pub fn allows(&self) -> bool {
        matches!(self, InfVerdict::Allow { .. })
    }

    pub fn gate(&self) -> &str {
        match self {
            InfVerdict::Allow { gate, .. }
            | InfVerdict::Hold { gate, .. }
            | InfVerdict::Reject { gate, .. }
            | InfVerdict::Quarantine { gate, .. } => gate,
        }
    }

    pub fn residue(&self) -> Option<&Residue> {
        match self {
            InfVerdict::Allow { .. } => None,
            InfVerdict::Hold { residue, .. }
            | InfVerdict::Reject { residue, .. }
            | InfVerdict::Quarantine { residue, .. } => Some(residue),
        }
    }
}

pub const ALL_INFERENCE_GATES: [&str; 14] = [
    "ProviderManifestGate",
    "ProviderTermsGate",
    "ModelPrivacyGate",
    "PromptContextGate",
    "ModelBudgetGate",
    "ModelRateGate",
    "ModelCapabilityGate",
    "ModelReplayGate",
    "OutputSchemaGate",
    "NoDirectCommitGate",
    "NoGateOverrideGate",
    "ToolCapabilityGate",
    "ToolEgressGate",
    "HumanConfirmationGate",
];

/// 1/14 ProviderManifestGate.
pub fn provider_manifest_gate(m: &ModelManifest) -> InfVerdict {
    match m.validate() {
        Ok(()) => InfVerdict::Allow {
            gate: "ProviderManifestGate".into(),
            reason: "Manifest vollstaendig + valide".into(),
        },
        Err(e) => InfVerdict::Hold {
            gate: "ProviderManifestGate".into(),
            residue: Box::new(model_residue(
                "provider_manifest_missing",
                &format!(
                    "Pflichtfelder fehlen: {:?} — Provider bleibt passiv",
                    e.missing
                ),
            )),
        },
    }
}

/// 2/14 ProviderTermsGate.
pub fn provider_terms_gate(m: &ModelManifest) -> InfVerdict {
    match m.provider_terms_ref.as_deref() {
        Some(t) if t.starts_with("terms:known") => InfVerdict::Allow {
            gate: "ProviderTermsGate".into(),
            reason: format!("{t}: bekannt+kompatibel"),
        },
        _ => InfVerdict::Hold {
            gate: "ProviderTermsGate".into(),
            residue: Box::new(model_residue(
                "model_terms_unknown",
                "provider_terms_ref unbekannt/unklar — Hold",
            )),
        },
    }
}

/// 3/14 ModelPrivacyGate: privacy/retention/logging kompatibel zur
/// Lauf-Datenpolicy.
pub fn model_privacy_gate(m: &ModelManifest, run_data_policy: &str) -> InfVerdict {
    if run_data_policy == "no_pii" && m.privacy_mode.as_deref() != Some("no_training_use") {
        return InfVerdict::Hold {
            gate: "ModelPrivacyGate".into(),
            residue: Box::new(model_residue(
                "model_privacy_block",
                "privacy_mode inkompatibel zur Lauf-Datenpolicy no_pii",
            )),
        };
    }
    if m.data_retention_mode.as_deref() == Some("indefinite_retention") {
        return InfVerdict::Hold {
            gate: "ModelPrivacyGate".into(),
            residue: Box::new(model_residue(
                "model_retention_incompatible",
                "data_retention_mode=indefinite_retention unzulaessig",
            )),
        };
    }
    InfVerdict::Allow {
        gate: "ModelPrivacyGate".into(),
        reason: "privacy/retention/logging kompatibel".into(),
    }
}

/// 4/14 PromptContextGate — VOR jedem Egress: Egress ⊆ allowed,
/// ∩ forbidden = ∅ (Pruefvorrang), ⊆ Projektions-Boundary.
pub fn prompt_context_gate(req: &InferenceRequest, projection_boundary: &[String]) -> InfVerdict {
    // Pruefvorrang: forbidden zuerst.
    for slice in &req.context {
        if req.forbidden_context.contains(&slice.name) {
            return InfVerdict::Reject {
                gate: "PromptContextGate".into(),
                residue: Box::new(model_residue(
                    "sensitive_context_egress_blocked",
                    &format!(
                        "Kontext '{}' ist forbidden_context — kein Egress",
                        slice.name
                    ),
                )),
            };
        }
    }
    for slice in &req.context {
        if !req.allowed_context.contains(&slice.name) {
            return InfVerdict::Reject {
                gate: "PromptContextGate".into(),
                residue: Box::new(model_residue(
                    "prompt_context_boundary_violation",
                    &format!("Kontext '{}' nicht in allowed_context", slice.name),
                )),
            };
        }
        if !projection_boundary.contains(&slice.name) {
            return InfVerdict::Reject {
                gate: "PromptContextGate".into(),
                residue: Box::new(model_residue(
                    "prompt_context_boundary_violation",
                    &format!(
                        "Kontext '{}' liegt ausserhalb der ProjectionPacket-Boundary",
                        slice.name
                    ),
                )),
            };
        }
    }
    InfVerdict::Allow {
        gate: "PromptContextGate".into(),
        reason: "Egress ⊆ allowed ∧ ∩ forbidden = ∅ ∧ ⊆ Boundary".into(),
    }
}

/// 5/14 ModelBudgetGate. Kontext-Overflow ist HOLD, kein stilles
/// Kuerzen (F.1 Verbote).
pub fn model_budget_gate(req: &InferenceRequest, m: &ModelManifest) -> InfVerdict {
    let tokens = req.context_tokens();
    if tokens > m.context_window {
        return InfVerdict::Hold {
            gate: "ModelBudgetGate".into(),
            residue: Box::new(model_residue(
                "model_context_overflow",
                &format!(
                    "Kontext {tokens} Tokens > Fenster {} — Hold, KEIN stilles Kuerzen",
                    m.context_window
                ),
            )),
        };
    }
    match m.token_budget {
        Some(b) if u64::from(req.budget_tokens) <= u64::from(b) => InfVerdict::Allow {
            gate: "ModelBudgetGate".into(),
            reason: format!("{} ≤ Budget {b}", req.budget_tokens),
        },
        Some(b) => InfVerdict::Hold {
            gate: "ModelBudgetGate".into(),
            residue: Box::new(model_residue(
                "model_budget_exceeded",
                &format!("{} > Token-Budget {b} — Hold VOR Egress", req.budget_tokens),
            )),
        },
        None => InfVerdict::Hold {
            gate: "ModelBudgetGate".into(),
            residue: Box::new(model_residue(
                "model_budget_exceeded",
                "kein Budget deklariert",
            )),
        },
    }
}

/// 6/14 ModelRateGate.
pub fn model_rate_gate(requests_this_run: u32, m: &ModelManifest) -> InfVerdict {
    match m.rate_limit {
        Some(r) if requests_this_run < r => InfVerdict::Allow {
            gate: "ModelRateGate".into(),
            reason: format!("{requests_this_run} < Rate {r}"),
        },
        Some(r) => InfVerdict::Hold {
            gate: "ModelRateGate".into(),
            residue: Box::new(model_residue(
                "model_rate_limited",
                &format!("{requests_this_run} ≥ Rate {r} — Hold, keine Umgehung"),
            )),
        },
        None => InfVerdict::Hold {
            gate: "ModelRateGate".into(),
            residue: Box::new(model_residue(
                "model_rate_limited",
                "kein Rate-Limit deklariert",
            )),
        },
    }
}

/// 7/14 ModelCapabilityGate: model_egress-Lock offen (falls die Klasse
/// Egress braucht) + supported_ops decken die Anfrage.
pub fn model_capability_gate(
    m: &ModelManifest,
    lock: Option<&CapabilityLock>,
    requested_op: &str,
) -> InfVerdict {
    if m.provider_class.needs_egress() {
        match lock {
            Some(l) if l.is_open() => {}
            _ => {
                return InfVerdict::Hold {
                    gate: "ModelCapabilityGate".into(),
                    residue: Box::new(model_residue(
                        "provider_unavailable",
                        "model_egress-Lock geschlossen — kein Egress (S13-A7)",
                    )),
                }
            }
        }
    }
    if !m.supported_ops.iter().any(|o| o == requested_op) {
        return InfVerdict::Hold {
            gate: "ModelCapabilityGate".into(),
            residue: Box::new(model_residue(
                "provider_unavailable",
                &format!("Operation '{requested_op}' nicht in supported_ops"),
            )),
        };
    }
    InfVerdict::Allow {
        gate: "ModelCapabilityGate".into(),
        reason: "Lock offen (falls noetig) + Operation gedeckt".into(),
    }
}

/// 8/14 ModelReplayGate: Aufzeichnung vollstaendig; replay_policy erfuellt.
pub fn model_replay_gate(m: &ModelManifest, recording_complete: bool) -> InfVerdict {
    if !recording_complete {
        return InfVerdict::Hold {
            gate: "ModelReplayGate".into(),
            residue: Box::new(model_residue(
                "model_trace_missing",
                "Aufzeichnung unvollstaendig — Kandidat nicht gate-faehig",
            )),
        };
    }
    match m.replay_policy {
        ReplayPolicy::Recorded | ReplayPolicy::Strict => InfVerdict::Allow {
            gate: "ModelReplayGate".into(),
            reason: "Aufzeichnung vollstaendig, Policy erfuellt".into(),
        },
        ReplayPolicy::Weak => InfVerdict::Allow {
            gate: "ModelReplayGate".into(),
            reason: "weak: zulaessig, Live-Abweichung wird model_replay_weak (sichtbar)".into(),
        },
    }
}

/// 9/14 OutputSchemaGate: kein stilles Umformen.
pub fn output_schema_gate(expected_schema: &str, actual_schema: &str) -> InfVerdict {
    if expected_schema == actual_schema {
        InfVerdict::Allow {
            gate: "OutputSchemaGate".into(),
            reason: format!("Schema {expected_schema} erfuellt"),
        }
    } else {
        InfVerdict::Hold {
            gate: "OutputSchemaGate".into(),
            residue: Box::new(model_residue(
                "model_output_schema_invalid",
                &format!(
                    "erwartet {expected_schema}, geliefert {actual_schema} — kein stilles Umformen"
                ),
            )),
        }
    }
}

/// 10/14 NoDirectCommitGate: der Versuch, einen Kandidaten als Commit
/// zu deklarieren, ist ein REJECT (strukturell existiert der Pfad nicht;
/// dieses Gate faengt den deklarativen Versuch).
pub fn no_direct_commit_gate(claims_commit: bool) -> InfVerdict {
    if claims_commit {
        InfVerdict::Reject {
            gate: "NoDirectCommitGate".into(),
            residue: Box::new(model_residue(
                "model_attempted_direct_commit",
                "Kandidat→Commit ohne Motor-Gates — Reject (C.7)",
            )),
        }
    } else {
        InfVerdict::Allow {
            gate: "NoDirectCommitGate".into(),
            reason: "Kandidat bleibt Kandidat; Weg fuehrt durch Motor-Gates".into(),
        }
    }
}

/// 11/14 NoGateOverrideGate: kein Provider-/Kanzel-Pfad schreibt
/// Verdikte/Residuen/Ledger.
pub fn no_gate_override_gate(attempts_verdict_write: bool) -> InfVerdict {
    if attempts_verdict_write {
        InfVerdict::Reject {
            gate: "NoGateOverrideGate".into(),
            residue: Box::new(model_residue(
                "model_attempted_gate_override",
                "Provider-/Kanzel-Pfad versucht Verdikt-/Residuen-/Ledger-Schreibzugriff — Reject",
            )),
        }
    } else {
        InfVerdict::Allow {
            gate: "NoGateOverrideGate".into(),
            reason: "kein Schreibpfad auf Urteile/Residuen/Ledger".into(),
        }
    }
}

/// 12/14 ToolCapabilityGate: Tool-Klasse hat offenen Lock im Scope.
pub fn tool_capability_gate(tool_class: &str, lock: Option<&CapabilityLock>) -> InfVerdict {
    match lock {
        Some(l) if l.is_open() => InfVerdict::Allow {
            gate: "ToolCapabilityGate".into(),
            reason: format!("Lock fuer Tool-Klasse {tool_class} offen"),
        },
        _ => InfVerdict::Hold {
            gate: "ToolCapabilityGate".into(),
            residue: Box::new(model_residue(
                "provider_unavailable",
                &format!("Tool-Klasse {tool_class} ohne offenen ToolCapabilityLock — niemals implizit frei"),
            )),
        },
    }
}

/// 13/14 ToolEgressGate: Remote-Tool explizit erlaubt, budgetiert,
/// aufgezeichnet.
pub fn tool_egress_gate(
    remote: bool,
    explicitly_allowed: bool,
    budgeted: bool,
    recorded: bool,
) -> InfVerdict {
    if !remote {
        return InfVerdict::Allow {
            gate: "ToolEgressGate".into(),
            reason: "lokales Tool, kein Egress".into(),
        };
    }
    if explicitly_allowed && budgeted && recorded {
        InfVerdict::Allow {
            gate: "ToolEgressGate".into(),
            reason: "Remote explizit erlaubt + budgetiert + aufgezeichnet".into(),
        }
    } else {
        InfVerdict::Hold {
            gate: "ToolEgressGate".into(),
            residue: Box::new(model_residue(
                "provider_unavailable",
                "Remote-Tool ohne explizite Erlaubnis/Budget/Aufzeichnung — Hold",
            )),
        }
    }
}

/// 14/14 HumanConfirmationGate: jede materielle Aktion aus Modell-/
/// Tool-Kandidaten traegt eine AUFGEZEICHNETE Operator-Bestaetigung.
pub fn human_confirmation_gate(confirmation_ref: Option<&str>) -> InfVerdict {
    match confirmation_ref {
        Some(r) if !r.is_empty() => InfVerdict::Allow {
            gate: "HumanConfirmationGate".into(),
            reason: format!("Bestaetigung aufgezeichnet: {r}"),
        },
        _ => InfVerdict::Hold {
            gate: "HumanConfirmationGate".into(),
            residue: Box::new(model_residue(
                "model_trace_missing",
                "materielle Aktion ohne aufgezeichnete Operator-Bestaetigung",
            )),
        },
    }
}
