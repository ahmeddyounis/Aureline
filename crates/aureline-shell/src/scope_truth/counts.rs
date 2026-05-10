//! Visible / loaded / all-matching count truth for scope-truth chips.
//!
//! The triplet exists to disclose the gap between what the user sees, what
//! the active scope index actually holds, and what the same query would
//! match against the full workspace. Surfaces that only know one of the
//! counts MUST leave the rest as `None` rather than copying the known
//! value across — that would hide scope reduction behind a single number.

use serde::{Deserialize, Serialize};

/// Class of count disclosure for a scope-truth chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCountsClass {
    /// Active scope covers the full workspace, readiness is ready, and the
    /// three counts collapse. The chip may render globally authoritative
    /// counts.
    GloballyAuthoritative,
    /// All three counts are known but at least one differs (viewport
    /// truncated rows, or scope is narrower than the workspace, or
    /// readiness is below ready).
    PartialTruth,
    /// Counts are not yet computed (no query has run, scope just opened,
    /// etc.). Surfaces MUST render this as "—" rather than as zero.
    NotComputed,
}

impl ScopeCountsClass {
    /// Stable token used in records, fixtures, and shell snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GloballyAuthoritative => "globally_authoritative",
            Self::PartialTruth => "partial_truth",
            Self::NotComputed => "not_computed",
        }
    }
}

/// Inputs the chip projection feeds into [`ScopeCountsRecord::derive`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeCountsInputs {
    /// Rows currently rendered in the surface (post viewport truncation).
    pub visible_in_view: u64,
    /// Total rows produced by the active query against the loaded scope
    /// index, before viewport truncation. `None` when no query has run or
    /// when the index has not yet computed a query result.
    pub loaded_in_scope: Option<u64>,
    /// Total rows the same query would match against the *workspace* (i.e.
    /// after widening to `full_workspace`). `None` when unknown — surfaces
    /// MUST NOT default this to `loaded_in_scope`.
    pub all_matching_in_workspace: Option<u64>,
    /// True when the active scope == workspace.
    pub scope_covers_workspace: bool,
    /// True when the active scope's readiness == ready (no warming /
    /// partial / unavailable lanes contributed to the result).
    pub readiness_is_ready: bool,
}

/// Serializable shell-facing count record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCountsRecord {
    pub counts_class_token: String,
    pub visible_in_view: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded_in_scope: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_matching_in_workspace: Option<u64>,
    pub scope_covers_workspace: bool,
    pub readiness_is_ready: bool,
}

impl ScopeCountsRecord {
    /// Derive a count record from the supplied inputs.
    ///
    /// The classification is total: if no query has computed a count, the
    /// record is `not_computed`; if the scope and readiness are both
    /// authoritative AND the three counts agree, the record is
    /// `globally_authoritative`; otherwise it is `partial_truth`.
    pub fn derive(inputs: ScopeCountsInputs) -> Self {
        let class = if inputs.loaded_in_scope.is_none()
            && inputs.all_matching_in_workspace.is_none()
            && inputs.visible_in_view == 0
        {
            ScopeCountsClass::NotComputed
        } else if inputs.scope_covers_workspace
            && inputs.readiness_is_ready
            && counts_collapse(
                inputs.visible_in_view,
                inputs.loaded_in_scope,
                inputs.all_matching_in_workspace,
            )
        {
            ScopeCountsClass::GloballyAuthoritative
        } else {
            ScopeCountsClass::PartialTruth
        };
        Self {
            counts_class_token: class.as_str().to_string(),
            visible_in_view: inputs.visible_in_view,
            loaded_in_scope: inputs.loaded_in_scope,
            all_matching_in_workspace: inputs.all_matching_in_workspace,
            scope_covers_workspace: inputs.scope_covers_workspace,
            readiness_is_ready: inputs.readiness_is_ready,
        }
    }

    /// Convenience constructor for surfaces that have not yet computed any
    /// counts (e.g. quick open before the user has typed a query).
    pub fn not_computed(scope_covers_workspace: bool, readiness_is_ready: bool) -> Self {
        Self::derive(ScopeCountsInputs {
            visible_in_view: 0,
            loaded_in_scope: None,
            all_matching_in_workspace: None,
            scope_covers_workspace,
            readiness_is_ready,
        })
    }

    /// True when the record's class indicates the surface is not currently
    /// promising globally authoritative counts.
    pub fn is_partial(&self) -> bool {
        self.counts_class_token != ScopeCountsClass::GloballyAuthoritative.as_str()
    }
}

fn counts_collapse(
    visible_in_view: u64,
    loaded_in_scope: Option<u64>,
    all_matching_in_workspace: Option<u64>,
) -> bool {
    match (loaded_in_scope, all_matching_in_workspace) {
        (Some(loaded), Some(all)) => visible_in_view == loaded && loaded == all,
        // We require all three counts to be known before we let a chip
        // claim globally authoritative truth.
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_counts_yet_classifies_as_not_computed() {
        let record = ScopeCountsRecord::not_computed(true, true);
        assert_eq!(record.counts_class_token, "not_computed");
        assert!(record.is_partial());
    }

    #[test]
    fn full_workspace_with_collapsed_counts_is_globally_authoritative() {
        let record = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 12,
            loaded_in_scope: Some(12),
            all_matching_in_workspace: Some(12),
            scope_covers_workspace: true,
            readiness_is_ready: true,
        });
        assert_eq!(record.counts_class_token, "globally_authoritative");
        assert!(!record.is_partial());
    }

    #[test]
    fn narrowed_scope_classifies_as_partial_truth() {
        let record = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 8,
            loaded_in_scope: Some(8),
            all_matching_in_workspace: Some(45),
            scope_covers_workspace: false,
            readiness_is_ready: true,
        });
        assert_eq!(record.counts_class_token, "partial_truth");
        assert!(record.is_partial());
    }

    #[test]
    fn truncated_viewport_classifies_as_partial_truth() {
        let record = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 12,
            loaded_in_scope: Some(87),
            all_matching_in_workspace: Some(87),
            scope_covers_workspace: true,
            readiness_is_ready: true,
        });
        assert_eq!(record.counts_class_token, "partial_truth");
    }

    #[test]
    fn warming_readiness_blocks_globally_authoritative() {
        let record = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 0,
            loaded_in_scope: Some(0),
            all_matching_in_workspace: Some(0),
            scope_covers_workspace: true,
            readiness_is_ready: false,
        });
        assert_eq!(record.counts_class_token, "partial_truth");
    }

    #[test]
    fn unknown_all_matching_blocks_globally_authoritative() {
        let record = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 5,
            loaded_in_scope: Some(5),
            all_matching_in_workspace: None,
            scope_covers_workspace: true,
            readiness_is_ready: true,
        });
        assert_eq!(record.counts_class_token, "partial_truth");
    }
}
