//! Content-design lanes: typed, export-safe wording objects the shell renders.
//!
//! Each submodule materializes one governed wording contract as inspectable,
//! serde-serializable truth packets — never hand-maintained prose islands. The
//! packets carry no credential bodies or raw provider payloads, so UI, CLI/help,
//! docs, support exports, and screenshot/demo surfaces can all resolve the same
//! wording objects.
//!
//! - [`error_patterns`] — reusable error/recovery copy objects (what-failed,
//!   why-likely, what-still-works, next-action, recovery-link) plus reusable
//!   degraded-state reason chips shared across runtime, network, repair, install,
//!   review, and docs/help recovery surfaces.
//! - [`ai_copy_guardrails`] — controlled AI wording (Suggested, Proposed, Draft,
//!   Context used, Validation, Low confidence, Review required, Revert/Undo
//!   availability) plus a forbidden high-trust phrase register and lint that rejects
//!   overclaiming copy across prompt composer, patch review, notebook help,
//!   docs/help, and provider/account surfaces.

pub mod ai_copy_guardrails;
pub mod error_patterns;
