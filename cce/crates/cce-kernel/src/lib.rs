//! cce-kernel — L3 PSP Execution-Kernel (REBASE_KONSOLIDIERUNG §1, Klasse B):
//! `PSPcore = (Σ, Ω, RD, Trace, Evidence, Manifest)` — das Maschinengewinde
//! der PhaseBlock-Erzeugung UNTER LOOM/CCE. Keine neue Theorie-Spitze (D1).
//! MEF-Payload mit deklariertem Byte-Encoding (R-6 hier geschlossen, siehe
//! reports/G02_bericht.md).

pub mod assembly;
pub mod mef;
pub mod pspcore;
pub mod scheduler;

pub use mef::{MefBlock, MEF_MAGIC, MEF_VERSION};
pub use pspcore::PspCore;
pub use scheduler::{schedule, OperatorRegistry, TaskNode};
