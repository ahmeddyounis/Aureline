// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Resolution of configured Aureline state-family roots.
//!
//! The normative state map names `$AURELINE_STATE` as a location concept while
//! deliberately deferring final per-platform expansion. This module therefore
//! honors an explicitly configured root and otherwise preserves the current
//! repository-local paths until the platform resolver lands.

use std::ffi::OsStr;
use std::path::PathBuf;

/// Environment variable that selects the Aureline local-state root.
pub const AURELINE_STATE_ENV: &str = "AURELINE_STATE";

/// State families currently consumed through the shared resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatePathFamily {
    /// Disposable diagnostic logs and traces.
    Logs,
    /// User-owned recent-work continuity metadata.
    RecentWork,
}

impl StatePathFamily {
    const fn configured_component(self) -> &'static str {
        match self {
            Self::Logs => "logs",
            Self::RecentWork => "recent_work",
        }
    }

    fn legacy_root(self) -> PathBuf {
        match self {
            Self::Logs => PathBuf::from(".logs"),
            Self::RecentWork => PathBuf::from(".logs").join("recent_work"),
        }
    }
}

/// Resolves a state-family root from an explicit environment value.
///
/// This pure helper exists so callers and tests do not need to mutate the
/// process environment. An absent or empty value preserves the legacy path.
pub fn resolve_state_family_root_from(
    configured_state_root: Option<&OsStr>,
    family: StatePathFamily,
) -> PathBuf {
    configured_state_root
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .map(|root| root.join(family.configured_component()))
        .unwrap_or_else(|| family.legacy_root())
}

/// Resolves a state-family root from the current process configuration.
pub fn state_family_root(family: StatePathFamily) -> PathBuf {
    let configured = std::env::var_os(AURELINE_STATE_ENV);
    resolve_state_family_root_from(configured.as_deref(), family)
}

/// Returns the configured logs root, or the legacy `.logs` root when unset.
pub fn logs_root() -> PathBuf {
    state_family_root(StatePathFamily::Logs)
}

/// Returns the configured recent-work root, or its legacy location when unset.
pub fn recent_work_root() -> PathBuf {
    state_family_root(StatePathFamily::RecentWork)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn explicit_state_root_routes_each_family_without_environment_mutation() {
        let configured = OsStr::new("configured-state");

        assert_eq!(
            resolve_state_family_root_from(Some(configured), StatePathFamily::Logs),
            Path::new("configured-state").join("logs")
        );
        assert_eq!(
            resolve_state_family_root_from(Some(configured), StatePathFamily::RecentWork),
            Path::new("configured-state").join("recent_work")
        );
    }

    #[test]
    fn absent_or_empty_state_root_preserves_legacy_paths() {
        assert_eq!(
            resolve_state_family_root_from(None, StatePathFamily::Logs),
            PathBuf::from(".logs")
        );
        assert_eq!(
            resolve_state_family_root_from(Some(OsStr::new("")), StatePathFamily::RecentWork),
            PathBuf::from(".logs").join("recent_work")
        );
    }
}
