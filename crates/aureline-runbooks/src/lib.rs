//! Governed runbook object model for claimed incident and operator surfaces.
//!
//! Aureline markets runbooks as *governed executable guidance*, not rich-text
//! suggestions. This crate freezes the canonical object model that backs that
//! claim: every runbook declares where its authority comes from (a
//! [source class](m5_runbook_governance::RunbookSourceClass)), what class of
//! step is being run (a [step class](m5_runbook_governance::RunbookStepClass)),
//! what scope or approval each step requires
//! ([`RunbookApprovalScope`](m5_runbook_governance::RunbookApprovalScope)), what
//! evidence outputs are expected, and how console/browser pivots
//! ([control-plane boundary](m5_runbook_governance::ControlPlaneBoundaryClass))
//! or archived execution history stay attributable. Companions may follow or
//! request within declared scope but cannot mint hidden privileged mutate
//! channels.
//!
//! The [`m5_runbook_governance`] module publishes the contract matrix that names
//! every governed runbook object, its owner, its first consumer, and the proof
//! packet that keeps it current, plus a release gate that blocks Stable
//! promotion when a claimed runbook-backed surface lacks a mapped object or
//! current proof. Incident workspaces, operator dashboards, docs/help,
//! companions, and support bundles consume this one inventory rather than local
//! prose or screenshots.
//!
//! Runbook authority depends first on *where* a runbook came from. The
//! [`m5_runbook_sources`] module publishes the source register: every runbook
//! source declares its provenance class (repo-local, mirrored docs-pack,
//! managed-catalog, or browser-reference), its version, a signer/provenance
//! block, a freshness window, its owning scope, and its export rights, and the
//! register *derives* an effective authority posture — authoritative, mirrored,
//! managed, or reference-only — that every consuming surface renders identically.
//! Browser-only vendor docs stay reference-only unless another governed source
//! promotes them, and a stale source narrows back to reference-only, so a
//! reference cannot masquerade as a first-party executable runbook.
//!
//! The records this crate produces are inspectable, serde-serializable truth
//! packets that carry no credential bodies or raw provider/console payloads.

#![doc(html_root_url = "https://docs.rs/aureline-runbooks/0.0.0")]

pub mod m5_runbook_governance;
pub mod m5_runbook_sources;
