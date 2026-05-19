//! Filter bar state record.
//!
//! The filter bar primitive carries one active query text, source-labeled
//! chips, hidden narrowing from policy/workset/provider/client limits, an
//! explicit reset action token, and one [`CountSummaryClass`] describing
//! whether the surface's totals are exact, approximate, partial, or
//! provider-limited. Surfaces project from this record verbatim; they
//! never invent a parallel chip class or summary class.

use serde::{Deserialize, Serialize};

use super::{CollectionTruthSurfaceFamily, COLLECTION_TRUTH_BETA_SCHEMA_VERSION};

/// Stable record kind tag for [`FilterBarStateRecord`].
pub const FILTER_BAR_STATE_RECORD_KIND: &str = "shell_collection_filter_bar_state_record";

/// Closed source class for one filter chip.
///
/// Mirrors the spec's source vocabulary (`user`, `saved view`, `policy`,
/// `client scope`, `provider limit`) plus the partial-data class so the
/// chip can disclose warming or stale state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingSourceClass {
    /// Chip is a user-applied query term or facet.
    User,
    /// Chip is restored from a saved view and read-only on consumer surfaces.
    SavedView,
    /// Chip is narrowed by admin policy.
    Policy,
    /// Chip is narrowed by the active workset.
    Workset,
    /// Chip is narrowed by a client-side window or pagination limit.
    ClientLimit,
    /// Chip discloses a provider-side capacity, retention, or sampling limit.
    ProviderLimit,
    /// Chip discloses that data is partial, warming, stale, or cached.
    PartialData,
}

impl NarrowingSourceClass {
    /// Stable token used in fixtures, packets, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::SavedView => "saved_view",
            Self::Policy => "policy",
            Self::Workset => "workset",
            Self::ClientLimit => "client_limit",
            Self::ProviderLimit => "provider_limit",
            Self::PartialData => "partial_data",
        }
    }

    /// True when the source class represents hidden narrowing that MUST
    /// remain visible and explainable rather than collapsed into the
    /// user-applied filter chip family.
    pub const fn is_hidden_narrowing(self) -> bool {
        matches!(
            self,
            Self::Policy
                | Self::Workset
                | Self::ClientLimit
                | Self::ProviderLimit
                | Self::PartialData
        )
    }
}

/// Frozen class for the active filter bar's count summary truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountSummaryClass {
    /// Counts are exact and locally authoritative.
    ExactLocal,
    /// Counts are exact while the workset narrowing is the cause of the
    /// reduction.
    ExactWithWorksetNarrowing,
    /// Counts are exact while policy pinning is the cause of the
    /// reduction.
    ExactWithPolicyPinning,
    /// Counts are approximate because the provider sampled or capped.
    ApproximateProviderLimited,
    /// Counts are partial because indexing or warming has not completed.
    PartialIndexing,
    /// Counts come from a provider retention window that excludes older
    /// rows from the totals.
    ProviderRetentionWindowed,
    /// Counts are unknown — the surface refuses to invent a value.
    Unknown,
}

impl CountSummaryClass {
    /// Stable token used in fixtures, packets, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocal => "exact_local",
            Self::ExactWithWorksetNarrowing => "exact_with_workset_narrowing",
            Self::ExactWithPolicyPinning => "exact_with_policy_pinning",
            Self::ApproximateProviderLimited => "approximate_provider_limited",
            Self::PartialIndexing => "partial_indexing",
            Self::ProviderRetentionWindowed => "provider_retention_windowed",
            Self::Unknown => "unknown",
        }
    }
}

/// One chip rendered by the filter bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterBarChipRecord {
    /// Stable chip id.
    pub chip_id: String,
    /// Short field or facet label.
    pub label: String,
    /// Optional value label shown on the chip (already redacted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_label: Option<String>,
    /// Source class explaining where the chip came from.
    pub source_class: NarrowingSourceClass,
    /// True when the chip is locked from direct removal (saved view,
    /// policy, workset, provider/client limit, partial data).
    pub locked: bool,
    /// True when the chip is hidden narrowing.
    pub is_hidden_narrowing: bool,
    /// Optional explanation rendered on hover or in support exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl FilterBarChipRecord {
    fn from_source(
        chip_id: impl Into<String>,
        label: impl Into<String>,
        value_label: Option<String>,
        source_class: NarrowingSourceClass,
        explanation: Option<String>,
    ) -> Self {
        let is_hidden_narrowing = source_class.is_hidden_narrowing();
        Self {
            chip_id: chip_id.into(),
            label: label.into(),
            value_label,
            source_class,
            locked: is_hidden_narrowing || source_class == NarrowingSourceClass::SavedView,
            is_hidden_narrowing,
            explanation,
        }
    }

    /// Builds a user-applied free-text chip.
    pub fn user_text(label: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_source(
            format!("chip:user:text:{}", slug(&value)),
            label,
            Some(value),
            NarrowingSourceClass::User,
            None,
        )
    }

    /// Builds a user-applied facet chip.
    pub fn user_facet(label: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_source(
            format!("chip:user:facet:{}", slug(&value)),
            label,
            Some(value),
            NarrowingSourceClass::User,
            None,
        )
    }

    /// Builds a chip pinned by a saved view.
    pub fn saved_view_pinned(label: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_source(
            format!("chip:saved-view:{}", slug(&value)),
            label,
            Some(value),
            NarrowingSourceClass::SavedView,
            Some("Pinned by saved view".to_string()),
        )
    }

    /// Builds a policy-narrowed chip with explanation.
    pub fn policy_narrowed(
        label: impl Into<String>,
        value: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        let value = value.into();
        let explanation = explanation.into();
        Self::from_source(
            format!("chip:policy:{}", slug(&value)),
            label,
            Some(value),
            NarrowingSourceClass::Policy,
            Some(explanation),
        )
    }

    /// Builds a workset-narrowed chip with explanation.
    pub fn workset_narrowed(
        label: impl Into<String>,
        value: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        let value = value.into();
        let explanation = explanation.into();
        Self::from_source(
            format!("chip:workset:{}", slug(&value)),
            label,
            Some(value),
            NarrowingSourceClass::Workset,
            Some(explanation),
        )
    }

    /// Builds a chip disclosing a client-side limit.
    pub fn client_limit_disclosed(
        label: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        let label = label.into();
        let explanation = explanation.into();
        Self::from_source(
            format!("chip:client-limit:{}", slug(&label)),
            label,
            None,
            NarrowingSourceClass::ClientLimit,
            Some(explanation),
        )
    }

    /// Builds a chip disclosing a provider-side limit.
    pub fn provider_limit_disclosed(
        label: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        let label = label.into();
        let explanation = explanation.into();
        Self::from_source(
            format!("chip:provider-limit:{}", slug(&label)),
            label,
            None,
            NarrowingSourceClass::ProviderLimit,
            Some(explanation),
        )
    }

    /// Builds a chip disclosing partial/stale data.
    pub fn partial_data_disclosed(
        label: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        let label = label.into();
        let explanation = explanation.into();
        Self::from_source(
            format!("chip:partial-data:{}", slug(&label)),
            label,
            None,
            NarrowingSourceClass::PartialData,
            Some(explanation),
        )
    }
}

/// Filter bar state record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterBarStateRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable filter bar id.
    pub filter_bar_id: String,
    /// Surface family this filter bar belongs to.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Scope label rendered next to the filter bar.
    pub scope_label: String,
    /// Chips rendered by the filter bar in stable visual order.
    pub chips: Vec<FilterBarChipRecord>,
    /// Frozen count summary class for the active filter bar.
    pub count_summary_class: CountSummaryClass,
    /// Accessibility-narration summary covering hidden narrowing.
    pub hidden_narrowing_summary: String,
    /// Reset behavior label exposed to users and exports.
    pub reset_action_label: String,
}

impl FilterBarStateRecord {
    /// Builds a filter bar state record.
    pub fn new(
        filter_bar_id: impl Into<String>,
        surface_family: CollectionTruthSurfaceFamily,
        scope_label: impl Into<String>,
        chips: Vec<FilterBarChipRecord>,
        count_summary_class: CountSummaryClass,
        reset_action_label: impl Into<String>,
    ) -> Self {
        let hidden_narrowing_summary = hidden_narrowing_summary_text(&chips);
        Self {
            record_kind: FILTER_BAR_STATE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_BETA_SCHEMA_VERSION,
            filter_bar_id: filter_bar_id.into(),
            surface_family,
            scope_label: scope_label.into(),
            chips,
            count_summary_class,
            hidden_narrowing_summary,
            reset_action_label: reset_action_label.into(),
        }
    }

    /// Returns hidden-narrowing chip labels (suitable for support export).
    pub fn hidden_narrowing_labels(&self) -> Vec<String> {
        self.chips
            .iter()
            .filter(|chip| chip.is_hidden_narrowing)
            .map(|chip| match chip.value_label.as_ref() {
                Some(value) => format!("{}: {value}", chip.label),
                None => chip.label.clone(),
            })
            .collect()
    }
}

fn hidden_narrowing_summary_text(chips: &[FilterBarChipRecord]) -> String {
    let labels: Vec<String> = chips
        .iter()
        .filter(|chip| chip.is_hidden_narrowing)
        .map(|chip| match chip.value_label.as_ref() {
            Some(value) => format!("{}: {value}", chip.label),
            None => chip.label.clone(),
        })
        .collect();
    if labels.is_empty() {
        String::new()
    } else {
        labels.join("; ")
    }
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'A'..='Z' => ch.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => ch,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_narrowing_chips_lock_and_classify() {
        let chip = FilterBarChipRecord::policy_narrowed("tenant", "Tenant A", "policy");
        assert!(chip.locked);
        assert!(chip.is_hidden_narrowing);
        assert_eq!(chip.source_class, NarrowingSourceClass::Policy);
    }

    #[test]
    fn user_chips_are_unlocked_and_not_hidden() {
        let chip = FilterBarChipRecord::user_text("query", "foo");
        assert!(!chip.locked);
        assert!(!chip.is_hidden_narrowing);
    }

    #[test]
    fn record_collects_hidden_narrowing_summary() {
        let record = FilterBarStateRecord::new(
            "filter-bar:test",
            CollectionTruthSurfaceFamily::AdminOrSettingsGrid,
            "Admins",
            vec![
                FilterBarChipRecord::user_facet("role", "admin"),
                FilterBarChipRecord::policy_narrowed("tenant", "Tenant A", "policy"),
            ],
            CountSummaryClass::ExactWithPolicyPinning,
            "reset",
        );
        assert_eq!(record.hidden_narrowing_summary, "tenant: Tenant A");
        assert_eq!(record.hidden_narrowing_labels(), vec!["tenant: Tenant A"]);
    }
}
