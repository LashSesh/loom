//! cce-toolgateway — L9c: das ToolGateway (C.10). Einziger Port fuer
//! Werkzeug-Ausfuehrung, nie stillschweigend. Jede Tool-Klasse hat
//! ihren EIGENEN ToolCapabilityLock — Paketmanager, Git, Browser,
//! Shell, CI, Dateisystem, Netzwerktools sind NIEMALS implizit frei;
//! Remote zusaetzlich hinter dem ToolEgressGate. Tools beschaffen
//! keine Quellen (dafuer CSA — Tor-Trennung IG-A1).

pub mod gateway;
pub mod manifest;
