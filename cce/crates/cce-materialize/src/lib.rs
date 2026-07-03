//! cce-materialize — ADAPTER: Export-/Materialisierungsprofile (Materialize)
//! sowie der typstarke DomainAdapter-Vertrag (S1.8, 11 Punkte) und die
//! Dokument-Referenzimplementierung (S1, erste geschlossene Domaene).
//! Materialisierung NUR nach Gate/Evidence/Trace/Replay; reobserve-Pflicht.

pub mod adapter;
pub mod catalog;
pub mod document;
pub mod family_a;
pub mod family_a_domains;
pub mod family_b_domains;
pub mod family_c_domains;
pub mod family_d_domains;
pub mod family_e_domains;
pub mod family_f_domains;
pub mod family_g_domains;
pub mod family_h_domains;
pub mod family_i_domains;
pub mod family_j_domains;
pub mod family_k_domains;
pub mod family_l_domains;
pub mod family_m_domains;
pub mod family_n_domains;
pub mod family_o_domains;
pub mod family_p_domains;
pub mod scale2_folder;

pub use adapter::{check_adapter_parity_typed, DomainAdapter, OpenAction};
pub use document::{DocCrystal, DocUnit, DocumentAdapter, UnitType};
