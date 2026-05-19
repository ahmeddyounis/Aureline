//! Result-scope counter record.
//!
//! Scope counters distinguish visible, loaded, matching, total, partial,
//! and provider-owned counts across virtualized and paginated
//! collections. Surfaces emit one row per axis and never collapse two
//! axes into one number — that hides scope-reduction from the user.

use serde::{Deserialize, Serialize};

use super::{CollectionTruthSurfaceFamily, COLLECTION_TRUTH_BETA_SCHEMA_VERSION};

/// Stable record kind tag for [`CollectionScopeCounterRecord`].
pub const COLLECTION_SCOPE_COUNTER_RECORD_KIND: &str =
    "shell_collection_scope_counter_beta_record";

/// Frozen counter class for one counter row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCounterClass {
    /// Rows currently visible in the surface viewport.
    Visible,
    /// Rows materialised on the client (visible plus virtualised).
    Loaded,
    /// Rows the current query/filter matches in the authoritative source.
    Matching,
    /// Total rows in the authoritative source (when computable).
    Total,
    /// Rows that are partial because of indexing, warming, or paging.
    Partial,
    /// Rows whose count is owned by an external provider (sampled,
    /// capped, retention-bound, or otherwise out of client authority).
    ProviderOwned,
}

impl ScopeCounterClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::Matching => "matching",
            Self::Total => "total",
            Self::Partial => "partial",
            Self::ProviderOwned => "provider_owned",
        }
    }
}

/// Frozen status for one counter row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCounterStatus {
    /// Count is exact.
    Exact,
    /// Count is approximate (provider sampled, indexer estimated).
    Approximate,
    /// Count is provider-capped or sampled at a maximum.
    ProviderLimited,
    /// Count is partial — not all rows have been counted yet.
    Partial,
    /// Count is unknown — the surface refuses to invent a value.
    Unknown,
}

impl ScopeCounterStatus {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::ProviderLimited => "provider_limited",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
        }
    }
}

/// One row of the scope counter strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionScopeCounterRow {
    /// Counter class.
    pub counter_class: ScopeCounterClass,
    /// Counter status.
    pub status: ScopeCounterStatus,
    /// Counter value when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<u64>,
    /// Optional explanation rendered next to the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl CollectionScopeCounterRow {
    /// Builds a row with the supplied class/status/value.
    pub fn new(
        counter_class: ScopeCounterClass,
        status: ScopeCounterStatus,
        value: Option<u64>,
        explanation: Option<String>,
    ) -> Self {
        Self {
            counter_class,
            status,
            value,
            explanation,
        }
    }

    /// Convenience: known exact visible row count.
    pub fn visible(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Visible,
            ScopeCounterStatus::Exact,
            Some(count),
            None,
        )
    }

    /// Convenience: known exact loaded row count.
    pub fn loaded(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Loaded,
            ScopeCounterStatus::Exact,
            Some(count),
            None,
        )
    }

    /// Convenience: matching exact count.
    pub fn matching_exact(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Matching,
            ScopeCounterStatus::Exact,
            Some(count),
            None,
        )
    }

    /// Convenience: matching approximate count.
    pub fn matching_approximate(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Matching,
            ScopeCounterStatus::Approximate,
            Some(count),
            Some("approximate due to provider sampling".to_string()),
        )
    }

    /// Convenience: matching provider-limited count with retention hint.
    pub fn matching_provider_limited(count: u64, retention_hours: u32) -> Self {
        Self::new(
            ScopeCounterClass::Matching,
            ScopeCounterStatus::ProviderLimited,
            Some(count),
            Some(format!(
                "provider retained matches for the last {retention_hours} h only"
            )),
        )
    }

    /// Convenience: total exact count.
    pub fn total_exact(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Total,
            ScopeCounterStatus::Exact,
            Some(count),
            None,
        )
    }

    /// Convenience: total partial count.
    pub fn total_partial(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::Total,
            ScopeCounterStatus::Partial,
            Some(count),
            Some("total is partial until indexing completes".to_string()),
        )
    }

    /// Convenience: total unknown count due to provider retention.
    pub fn total_unknown_due_to_retention() -> Self {
        Self::new(
            ScopeCounterClass::Total,
            ScopeCounterStatus::Unknown,
            None,
            Some("total unknown because provider does not expose pre-retention totals".to_string()),
        )
    }

    /// Convenience: partial count with explanation.
    pub fn partial(count: u64, explanation: impl Into<String>) -> Self {
        Self::new(
            ScopeCounterClass::Partial,
            ScopeCounterStatus::Partial,
            Some(count),
            Some(explanation.into()),
        )
    }

    /// Convenience: provider-owned exact count.
    pub fn provider_owned_exact(count: u64) -> Self {
        Self::new(
            ScopeCounterClass::ProviderOwned,
            ScopeCounterStatus::Exact,
            Some(count),
            Some("count maintained by provider".to_string()),
        )
    }

    /// Convenience: provider-owned unknown count.
    pub fn provider_owned_unknown() -> Self {
        Self::new(
            ScopeCounterClass::ProviderOwned,
            ScopeCounterStatus::Unknown,
            None,
            Some("provider did not return an authoritative count".to_string()),
        )
    }
}

/// Collection scope counter record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionScopeCounterRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable scope counter id.
    pub scope_counter_id: String,
    /// Surface family this counter belongs to.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Counter rows in display order.
    pub rows: Vec<CollectionScopeCounterRow>,
    /// Accessibility-narration summary.
    pub accessibility_summary: String,
}

impl CollectionScopeCounterRecord {
    /// Builds a scope counter record from the supplied rows.
    pub fn new(
        scope_counter_id: impl Into<String>,
        surface_family: CollectionTruthSurfaceFamily,
        rows: Vec<CollectionScopeCounterRow>,
        accessibility_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: COLLECTION_SCOPE_COUNTER_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_BETA_SCHEMA_VERSION,
            scope_counter_id: scope_counter_id.into(),
            surface_family,
            rows,
            accessibility_summary: accessibility_summary.into(),
        }
    }

    /// True when at least one row admits an approximate, provider-limited,
    /// partial, or unknown status.
    pub fn has_non_exact_row(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.status != ScopeCounterStatus::Exact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_only_rows_register_exact() {
        let record = CollectionScopeCounterRecord::new(
            "counter:test",
            CollectionTruthSurfaceFamily::ReviewInbox,
            vec![
                CollectionScopeCounterRow::visible(10),
                CollectionScopeCounterRow::loaded(10),
                CollectionScopeCounterRow::matching_exact(10),
            ],
            "10 visible matches",
        );
        assert!(!record.has_non_exact_row());
    }

    #[test]
    fn provider_limited_rows_register_non_exact() {
        let record = CollectionScopeCounterRecord::new(
            "counter:test",
            CollectionTruthSurfaceFamily::SearchOrResultGrid,
            vec![
                CollectionScopeCounterRow::visible(10),
                CollectionScopeCounterRow::loaded(10),
                CollectionScopeCounterRow::matching_provider_limited(10, 24),
            ],
            "provider-limited",
        );
        assert!(record.has_non_exact_row());
    }
}
