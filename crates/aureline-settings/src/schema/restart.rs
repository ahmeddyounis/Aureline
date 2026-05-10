//! Restart-posture vocabulary.
//!
//! `restart_posture` declares what a settings consumer must do for
//! the change to take effect. Surfaces MUST quote the declared
//! posture verbatim; a silent restart that was not declared is a
//! bug.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartPosture {
    /// Change is picked up live; no restart required.
    NoRestart,
    /// Reload the active workspace to apply.
    ReloadWorkspace,
    /// Restart extension/runtime hosts to apply.
    RestartExtensions,
    /// Restart the desktop shell process to apply.
    RestartShell,
}

impl RestartPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRestart => "no_restart",
            Self::ReloadWorkspace => "reload_workspace",
            Self::RestartExtensions => "restart_extensions",
            Self::RestartShell => "restart_shell",
        }
    }
}

/// Lifecycle label declared by every setting definition. Mirrors the
/// `lifecycle_label` token set in the schema-registry seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    Stable,
    Preview,
    Experimental,
    Deprecated,
    Retired,
}

impl LifecycleLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }
}
