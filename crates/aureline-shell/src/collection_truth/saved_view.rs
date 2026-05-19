//! Saved-view artifact for portable collection state.
//!
//! Saved views preserve query, filters, sort/group, visible columns, and
//! scope while explicitly excluding transient row selection, stale
//! provider cursors, and secret-bearing values. Drift states pair to one
//! [`SavedViewFallbackBehavior`] so a restore against drifted state
//! never silently wipes user state or pretends the view is bound.

use serde::{Deserialize, Serialize};

use super::{CollectionTruthSurfaceFamily, COLLECTION_TRUTH_BETA_SCHEMA_VERSION};

/// Stable record kind tag for [`SavedCollectionViewRecord`].
pub const SAVED_COLLECTION_VIEW_RECORD_KIND: &str = "shell_saved_collection_view_beta_record";

/// Frozen scope class for a saved view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewScopeClass {
    /// View belongs to a single user.
    User,
    /// View belongs to the current workspace.
    Workspace,
    /// View is shared with a team or organization.
    Shared,
    /// View is pinned by policy or administration.
    PolicyPinned,
    /// View is provider-owned and read-only.
    ProviderOwned,
}

impl SavedViewScopeClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Workspace => "workspace",
            Self::Shared => "shared",
            Self::PolicyPinned => "policy_pinned",
            Self::ProviderOwned => "provider_owned",
        }
    }
}

/// Frozen drift state for a restored saved view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewDriftState {
    /// Captured state still matches the current state exactly.
    BoundCurrentStateMatchesCaptured,
    /// Provider columns, operators, or values drifted; disclosed to user.
    ProviderStateDriftedDisclosed,
    /// One or more captured columns no longer exist; disclosed to user.
    ColumnSetDriftedDisclosed,
    /// Policy narrowing changed since capture; disclosed to user.
    PolicyNarrowingChangedDisclosed,
    /// View archived; restore-or-recreate prompt shown to user.
    ViewArchivedOfferedRestore,
    /// View cannot be resolved; recreate prompt shown to user.
    ViewUnresolvableOfferedRecreate,
    /// Provider is offline; restore deferred with disclosure.
    ViewUnavailableProviderOfflineDisclosed,
}

impl SavedViewDriftState {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundCurrentStateMatchesCaptured => "bound_current_state_matches_captured",
            Self::ProviderStateDriftedDisclosed => "provider_state_drifted_disclosed",
            Self::ColumnSetDriftedDisclosed => "column_set_drifted_disclosed",
            Self::PolicyNarrowingChangedDisclosed => "policy_narrowing_changed_disclosed",
            Self::ViewArchivedOfferedRestore => "view_archived_offered_restore",
            Self::ViewUnresolvableOfferedRecreate => "view_unresolvable_offered_recreate",
            Self::ViewUnavailableProviderOfflineDisclosed => {
                "view_unavailable_provider_offline_disclosed"
            }
        }
    }

    /// True when drift requires the restored view to render disclosure.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::BoundCurrentStateMatchesCaptured)
    }
}

/// Fallback behavior when a saved view cannot bind exactly to current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewFallbackBehavior {
    /// Preserve the view and label degraded clauses.
    PreserveAndLabelDegraded,
    /// Load the portable subset and label omitted clauses.
    LoadPortableSubsetWithLabels,
    /// Refuse to load the view until a rebind decision is made.
    RefuseUntilRebound,
    /// Ask the provider to re-resolve unsupported fields or columns.
    ProviderRebindRequired,
    /// Offer to recreate the view from current state.
    OfferRecreateFromCurrent,
}

impl SavedViewFallbackBehavior {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveAndLabelDegraded => "preserve_and_label_degraded",
            Self::LoadPortableSubsetWithLabels => "load_portable_subset_with_labels",
            Self::RefuseUntilRebound => "refuse_until_rebound",
            Self::ProviderRebindRequired => "provider_rebind_required",
            Self::OfferRecreateFromCurrent => "offer_recreate_from_current",
        }
    }
}

/// Frozen pinned count axes that a saved view promises to restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewPinnedCountAxis {
    /// Visible row count.
    Visible,
    /// Loaded row count.
    Loaded,
    /// Matching row count.
    Matching,
    /// Total row count when available.
    Total,
}

impl SavedViewPinnedCountAxis {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::Matching => "matching",
            Self::Total => "total",
        }
    }
}

/// One captured column preset row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedViewColumnPreset {
    /// Stable column id.
    pub column_id: String,
    /// Reviewable column label.
    pub label: String,
    /// True when the column is pinned (cannot be hidden by drift).
    pub pinned: bool,
}

impl SavedViewColumnPreset {
    /// Builds a column preset row.
    pub fn new(column_id: impl Into<String>, label: impl Into<String>, pinned: bool) -> Self {
        Self {
            column_id: column_id.into(),
            label: label.into(),
            pinned,
        }
    }
}

/// One captured sort key row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedViewSortKey {
    /// Stable column id.
    pub column_id: String,
    /// True when the sort direction is descending.
    pub descending: bool,
}

/// Saved view record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedCollectionViewRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable saved view id.
    pub saved_view_id: String,
    /// Surface family this view belongs to.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Reviewable view name.
    pub name: String,
    /// Scope class for sync, export, and sharing decisions.
    pub scope_class: SavedViewScopeClass,
    /// Drift state at restore.
    pub drift_state: SavedViewDriftState,
    /// Fallback behavior when restore cannot bind exactly.
    pub fallback_behavior: SavedViewFallbackBehavior,
    /// Captured column presets in order.
    pub column_presets: Vec<SavedViewColumnPreset>,
    /// Captured sort keys in order.
    pub sort_keys: Vec<SavedViewSortKey>,
    /// Count axes the view promises to restore.
    pub pinned_count_axes: Vec<SavedViewPinnedCountAxis>,
    /// Stale or degraded labels rendered next to the view.
    pub stale_or_degraded_labels: Vec<String>,
    /// True when the view captured transient row selection (forbidden).
    pub captures_transient_selection: bool,
    /// True when the view captured a provider cursor (forbidden).
    pub captures_provider_cursor: bool,
    /// True when the view captured a secret-bearing value (forbidden).
    pub captures_secret_bearing_value: bool,
}

impl SavedCollectionViewRecord {
    /// Builds a saved view record from the captured parts.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        saved_view_id: impl Into<String>,
        surface_family: CollectionTruthSurfaceFamily,
        name: impl Into<String>,
        scope_class: SavedViewScopeClass,
        drift_state: SavedViewDriftState,
        fallback_behavior: SavedViewFallbackBehavior,
        column_presets: Vec<SavedViewColumnPreset>,
        sort_keys: Vec<(&str, bool)>,
        pinned_count_axes: Vec<SavedViewPinnedCountAxis>,
        stale_or_degraded_labels: Vec<String>,
    ) -> Self {
        Self {
            record_kind: SAVED_COLLECTION_VIEW_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_BETA_SCHEMA_VERSION,
            saved_view_id: saved_view_id.into(),
            surface_family,
            name: name.into(),
            scope_class,
            drift_state,
            fallback_behavior,
            column_presets,
            sort_keys: sort_keys
                .into_iter()
                .map(|(column_id, descending)| SavedViewSortKey {
                    column_id: column_id.to_string(),
                    descending,
                })
                .collect(),
            pinned_count_axes,
            stale_or_degraded_labels,
            captures_transient_selection: false,
            captures_provider_cursor: false,
            captures_secret_bearing_value: false,
        }
    }

    /// True when restoring this view requires disclosure.
    pub fn requires_disclosure(&self) -> bool {
        self.drift_state.requires_disclosure() || !self.stale_or_degraded_labels.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_views_skip_disclosure() {
        let view = SavedCollectionViewRecord::new(
            "view:test",
            CollectionTruthSurfaceFamily::ReviewInbox,
            "My reviews",
            SavedViewScopeClass::User,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![SavedViewColumnPreset::new("a", "A", true)],
            vec![("a", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        assert!(!view.requires_disclosure());
    }

    #[test]
    fn drifted_views_require_disclosure() {
        let view = SavedCollectionViewRecord::new(
            "view:test",
            CollectionTruthSurfaceFamily::ReviewInbox,
            "Drifted",
            SavedViewScopeClass::User,
            SavedViewDriftState::ColumnSetDriftedDisclosed,
            SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
            vec![SavedViewColumnPreset::new("a", "A", true)],
            vec![("a", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec!["column removed".to_string()],
        );
        assert!(view.requires_disclosure());
    }

    #[test]
    fn views_default_to_refusing_transient_selection_and_cursors() {
        let view = SavedCollectionViewRecord::new(
            "view:test",
            CollectionTruthSurfaceFamily::ReviewInbox,
            "Drifted",
            SavedViewScopeClass::User,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![SavedViewColumnPreset::new("a", "A", true)],
            vec![("a", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        assert!(!view.captures_transient_selection);
        assert!(!view.captures_provider_cursor);
        assert!(!view.captures_secret_bearing_value);
    }
}
