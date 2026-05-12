//! Scope vocabulary projected onto search-shell rows.
//!
//! The vocabulary mirrors [`aureline_workspace::ScopeClass`]; we re-expose the
//! tokens here so consumers of the search crate do not have to depend on the
//! workspace crate just to render a chip label, and so the search shell can
//! convert a workset projection into the same token vocabulary the workset
//! switcher uses without forking truth.

use serde::{Deserialize, Serialize};

use aureline_workspace::ScopeClass as WorkspaceScopeClass;

/// Stable scope-class vocabulary used by the search shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    CurrentRepo,
    SelectedWorkset,
    SparseSlice,
    FullWorkspace,
    PolicyLimitedView,
}

impl ScopeClass {
    /// Stable token; matches the workspace-crate vocabulary verbatim.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepo => "current_repo",
            Self::SelectedWorkset => "selected_workset",
            Self::SparseSlice => "sparse_slice",
            Self::FullWorkspace => "full_workspace",
            Self::PolicyLimitedView => "policy_limited_view",
        }
    }

    /// Project from the canonical workspace `ScopeClass`. The search shell
    /// MUST use this conversion rather than re-deriving the token from a
    /// chip label.
    pub const fn from_workspace(scope: WorkspaceScopeClass) -> Self {
        match scope {
            WorkspaceScopeClass::CurrentRepo => Self::CurrentRepo,
            WorkspaceScopeClass::SelectedWorkset => Self::SelectedWorkset,
            WorkspaceScopeClass::SparseSlice => Self::SparseSlice,
            WorkspaceScopeClass::FullWorkspace => Self::FullWorkspace,
            WorkspaceScopeClass::PolicyLimitedView => Self::PolicyLimitedView,
        }
    }

    /// Human-readable family for a scope chip (matches the workspace
    /// chip-label family verbatim).
    pub const fn chip_label_family(self) -> &'static str {
        match self {
            Self::CurrentRepo => "Current repo",
            Self::SelectedWorkset => "Selected workset",
            Self::SparseSlice => "Sparse slice",
            Self::FullWorkspace => "Full workspace",
            Self::PolicyLimitedView => "Policy-limited view",
        }
    }

    /// True when the scope hides files from the search-shell view.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::CurrentRepo | Self::SelectedWorkset | Self::SparseSlice | Self::PolicyLimitedView
        )
    }
}
