//! Runtime discovery contracts for launch-capable workspace artifacts.
//!
//! Discovery modules are read-only: they inspect workspace declarations,
//! bind findings to an [`crate::execution_context::ExecutionContext`], and
//! emit explicit launch contracts for downstream task/test/debug surfaces.

pub mod package_scripts;
