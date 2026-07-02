//! cce-lattice — GRUNDSTOCK/COLLECT: Constraint Lattice (F-07..F-15).
//! Cube G0–G3, Propagationshuelle Γ_𝒦 (extensiv/monoton/idempotent, lfp),
//! Feasible Set (monoton schrumpfend), Chordalitaet/PEO/Junction Tree/
//! Separatoren, Chameleon-Projektion (No-Horizon-Leakage), Collapse-Zertifikat,
//! Export-Funktor. Abnahmekatalog CL K1–K18 in tests/k_catalog.rs.

pub mod collapse;
pub mod cube;
pub mod export_functor;
pub mod feasible;
pub mod graph;
pub mod projection;
pub mod propagation;
pub mod unfold;

pub use cube::{Coupling, Cube, Dimension, Grade};
pub use feasible::enumerate_feasible;
pub use graph::{Graph, JunctionTree, Peo};
pub use propagation::{propagate_hull, Constraint, DomainState};
