//! Crash journals, dirty-buffer recovery, and session-restore skeleton persistence.
//!
//! This crate provides prototype, file-backed stores for two related state
//! families:
//!
//! - Dirty-buffer crash journals (autosave journal entries and crash sentinels).
//! - Session-restore skeleton snapshots (workspace authority checkpoints and
//!   window-topology snapshot bodies).
//!
//! The record shapes are designed to match the boundary schemas and fixtures
//! under `schemas/` and `fixtures/` so shell surfaces can exercise recovery
//! flows without relying on ad hoc logs.

#![doc(html_root_url = "https://docs.rs/aureline-recovery/0.0.0")]

pub mod crash_journal;
pub mod session_restore;

