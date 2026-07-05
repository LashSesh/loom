//! cce-repointel — P5/Dokument 23 Track B: RepoIntelligence. CCE
//! bearbeitet ein Repository nicht nur — es versteht es BEWEISBAR:
//!
//!   fremdes Repo ──(bestehende CSA-Adapter, Gates unveraendert)──►
//!   NexusSourceBundle ──(bestehende HBM-Kette)──► zertifizierte
//!   Blueprint-Kandidaten ──(Dokument 21)──► GroundingPacket ──►
//!   versiegelter Bauplan-Workbody (Klasse "blueprint",
//!   verify == Valid, replay-identisch, cites auf die Quell-Evidence).
//!
//! Blatt-Crate, reine Komposition: kein Kern-Crate wird angefasst,
//! kein Gate dupliziert, kein Netzpfad ausser dem versiegelten
//! `fetch(&ApprovedFetchPlan, …)` der CSA-Kette.

pub mod distill;
pub mod ingest;
pub mod kette;
pub mod observe;
pub mod residues;
pub mod workbody;
