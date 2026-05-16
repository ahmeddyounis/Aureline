//! Shell-side runtime projections shared by terminal, task, and debug-prep
//! seed surfaces.
//!
//! The shell does not own runtime truth; it projects records minted by
//! [`aureline_runtime`] onto reviewable shell surfaces. The pieces here keep
//! every run-capable lane reading the same execution-context shape rather
//! than forking a private terminal-only, task-only, or debug-only copy.

pub mod context_inspector;
pub mod evidence_packet;
pub mod replay_pack;
