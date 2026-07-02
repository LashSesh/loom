//! Deterministisches Scheduling (Rebase §1.1): topologische Ordnung mit
//! festem lexikographischem Tie-Break — keine nichtdeterministische
//! Nebenlaeufigkeit auf dem Abschlusspfad (P9).

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TaskNode {
    pub id: String,
    pub deps: Vec<String>,
    pub operator: String,
}

/// Operator-/Gate-Registry: nur registrierte Operatoren sind planbar.
#[derive(Debug, Default)]
pub struct OperatorRegistry {
    ops: BTreeMap<String, String>,
}

impl OperatorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: &str, description: &str) {
        self.ops.insert(id.to_string(), description.to_string());
    }

    pub fn contains(&self, id: &str) -> bool {
        self.ops.contains_key(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleError {
    Cycle(Vec<String>),
    UnknownOperator { task: String, operator: String },
    UnknownDependency { task: String, dep: String },
}

/// Deterministischer Plan: Kahn-Topologie, Tie-Break lexikographisch.
pub fn schedule(
    tasks: &[TaskNode],
    registry: &OperatorRegistry,
) -> Result<Vec<String>, ScheduleError> {
    let ids: BTreeMap<&str, &TaskNode> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    for t in tasks {
        if !registry.contains(&t.operator) {
            return Err(ScheduleError::UnknownOperator {
                task: t.id.clone(),
                operator: t.operator.clone(),
            });
        }
        for d in &t.deps {
            if !ids.contains_key(d.as_str()) {
                return Err(ScheduleError::UnknownDependency {
                    task: t.id.clone(),
                    dep: d.clone(),
                });
            }
        }
    }
    let mut indeg: BTreeMap<&str, usize> = tasks
        .iter()
        .map(|t| (t.id.as_str(), t.deps.len()))
        .collect();
    let mut plan = Vec::new();
    while plan.len() < tasks.len() {
        // deterministisch: kleinster (lexikographisch) freier Knoten
        let next = indeg
            .iter()
            .filter(|(id, deg)| **deg == 0 && !plan.contains(&(**id).to_string()))
            .map(|(id, _)| *id)
            .min();
        let Some(next) = next else {
            let stuck: Vec<String> = indeg
                .iter()
                .filter(|(id, deg)| **deg > 0 && !plan.contains(&(**id).to_string()))
                .map(|(id, _)| id.to_string())
                .collect();
            return Err(ScheduleError::Cycle(stuck));
        };
        plan.push(next.to_string());
        for t in tasks {
            if t.deps.iter().any(|d| d == next) {
                *indeg.get_mut(t.id.as_str()).expect("known") -= 1;
            }
        }
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> OperatorRegistry {
        let mut r = OperatorRegistry::new();
        r.register("extract", "Extraktion");
        r.register("gate", "Gate-Pruefung");
        r
    }

    /// P9: gleiche Eingabe ⇒ identischer Plan (deterministischer Tie-Break).
    #[test]
    fn schedule_is_deterministic() {
        let tasks = vec![
            TaskNode {
                id: "b".into(),
                deps: vec!["a".into()],
                operator: "gate".into(),
            },
            TaskNode {
                id: "a".into(),
                deps: vec![],
                operator: "extract".into(),
            },
            TaskNode {
                id: "c".into(),
                deps: vec!["a".into()],
                operator: "gate".into(),
            },
        ];
        let p1 = schedule(&tasks, &registry()).unwrap();
        let p2 = schedule(&tasks, &registry()).unwrap();
        assert_eq!(p1, p2);
        assert_eq!(p1, vec!["a", "b", "c"]);
    }

    /// Zyklus wird erkannt (kein stiller Fortschritt).
    #[test]
    fn cycle_is_detected() {
        let tasks = vec![
            TaskNode {
                id: "a".into(),
                deps: vec!["b".into()],
                operator: "extract".into(),
            },
            TaskNode {
                id: "b".into(),
                deps: vec!["a".into()],
                operator: "extract".into(),
            },
        ];
        assert!(matches!(
            schedule(&tasks, &registry()),
            Err(ScheduleError::Cycle(_))
        ));
    }

    /// Unregistrierter Operator wird abgewiesen (Registry-Pflicht).
    #[test]
    fn unknown_operator_rejected() {
        let tasks = vec![TaskNode {
            id: "a".into(),
            deps: vec![],
            operator: "unbekannt".into(),
        }];
        assert!(matches!(
            schedule(&tasks, &registry()),
            Err(ScheduleError::UnknownOperator { .. })
        ));
    }
}
