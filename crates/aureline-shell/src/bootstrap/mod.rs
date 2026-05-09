//! Native desktop shell bootstrap.
//!
//! This module is the canonical entry point for the native desktop shell:
//! window creation, event-loop wiring, input dispatch, and startup-milestone
//! emission.

pub mod native_shell;
pub(crate) mod appearance_golden;
pub(crate) mod startup_trace;

pub use native_shell::run_native_shell;
