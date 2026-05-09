//! Shared state-card vocabulary and placeholder-card helpers.
//!
//! Shell surfaces must not invent local copy or ad-hoc labels when a surface is
//! warming, partial, offline, blocked, or otherwise degraded. This module
//! centralizes the shared degraded-state vocabulary and render-side helpers so
//! core shell surfaces can reuse the same tokens and labels.

pub mod degraded_state;
pub mod placeholder_state;
pub mod shell_placeholders;

pub use degraded_state::DegradedStateToken;
pub use shell_placeholders::{shell_slot_label, ShellPlaceholderCard};
