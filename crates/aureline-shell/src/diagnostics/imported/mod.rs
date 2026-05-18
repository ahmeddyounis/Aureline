//! Shell facade for imported scanner diagnostics.
//!
//! The runtime crate owns scanner import normalization and export-safe packet
//! construction. The shell re-exports those records so Problems, diagnostics,
//! and support-center surfaces consume the same model as CLI and release paths.

pub use aureline_runtime::scanner_import::*;
