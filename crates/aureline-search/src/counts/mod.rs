//! Scope-aware count and warning records shared by search-backed surfaces.
//!
//! This module owns the query-result count contract for alpha search lanes:
//! visible rows, loaded rows, all matching rows when known, and rows hidden
//! by the active workset, sparse slice, policy view, or remote cache. Search,
//! graph-backed candidates, and AI context candidates embed these records so
//! they render one scope vocabulary instead of inventing surface-local labels.

use serde::{Deserialize, Serialize};

use crate::lexical::scope::ScopeClass;

/// Schema version for scope-aware count and candidate truth records.
pub const SCOPE_TRUTH_COUNTS_SCHEMA_VERSION: u32 = 1;

/// Surface family embedding a scope-truth candidate record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeTruthSurface {
    /// Workspace search result set or search-shell snapshot.
    SearchResults,
    /// Graph-backed navigation or graph candidate row.
    GraphCandidate,
    /// AI context candidate or attachment sourced from search/graph.
    AiContextCandidate,
}

impl ScopeTruthSurface {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::GraphCandidate => "graph_candidate",
            Self::AiContextCandidate => "ai_context_candidate",
        }
    }
}

/// Shared scope label vocabulary used by search, graph, and AI candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeTruthLabel {
    /// The current repository/root lane.
    CurrentRepo,
    /// A named selected workset.
    SelectedWorkset,
    /// The full workspace lane.
    FullWorkspace,
    /// A remote or imported cache lane.
    RemoteCache,
    /// A result or action outside the current scope.
    OutsideCurrentScope,
    /// A policy-limited view.
    PolicyLimitedView,
}

impl ScopeTruthLabel {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepo => "current_repo",
            Self::SelectedWorkset => "selected_workset",
            Self::FullWorkspace => "full_workspace",
            Self::RemoteCache => "remote_cache",
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::PolicyLimitedView => "policy_limited_view",
        }
    }

    /// User-facing label that every scope-aware candidate surface quotes.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::CurrentRepo => "Current repo",
            Self::SelectedWorkset => "Selected workset",
            Self::FullWorkspace => "Full workspace",
            Self::RemoteCache => "Remote cache",
            Self::OutsideCurrentScope => "Outside current scope",
            Self::PolicyLimitedView => "Policy-limited view",
        }
    }

    /// Project from the canonical search scope class.
    pub const fn from_scope_class(scope: ScopeClass) -> Self {
        match scope {
            ScopeClass::CurrentRepo => Self::CurrentRepo,
            ScopeClass::SelectedWorkset | ScopeClass::SparseSlice => Self::SelectedWorkset,
            ScopeClass::FullWorkspace => Self::FullWorkspace,
            ScopeClass::PolicyLimitedView => Self::PolicyLimitedView,
        }
    }
}

/// Classification of a count disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchScopeCountsClass {
    /// The visible, loaded, and all-matching counts collapse with no hidden rows.
    GloballyAuthoritative,
    /// At least one count is partial, hidden, or otherwise not globally exact.
    PartialTruth,
    /// No query result count has been computed yet.
    NotComputed,
}

impl SearchScopeCountsClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GloballyAuthoritative => "globally_authoritative",
            Self::PartialTruth => "partial_truth",
            Self::NotComputed => "not_computed",
        }
    }
}

/// Inputs for [`SearchScopeCountsRecord::derive`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchScopeCountsInputs {
    /// Rows currently visible after grouping, truncation, or viewport limits.
    pub visible_rows: u64,
    /// Rows loaded by the active scope before viewport/group truncation.
    pub loaded_rows: Option<u64>,
    /// Rows the same query would match in the full workspace, when known.
    pub all_matching_rows: Option<u64>,
    /// Query-matching rows hidden by the active workset/slice/scope.
    pub hidden_by_current_scope_rows: u64,
    /// Query-matching rows hidden or blocked by policy.
    pub hidden_by_policy_rows: u64,
    /// Query-matching rows known only through a remote cache boundary.
    pub hidden_by_remote_cache_rows: u64,
    /// True when the answering lane is ready enough to claim exact counts.
    pub readiness_is_ready: bool,
}

/// Serializable count record carried by search result sets and candidates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchScopeCountsRecord {
    pub counts_class_token: String,
    pub visible_rows: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded_rows: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_matching_rows: Option<u64>,
    pub hidden_by_current_scope_rows: u64,
    pub hidden_by_policy_rows: u64,
    pub hidden_by_remote_cache_rows: u64,
    pub readiness_is_ready: bool,
}

impl SearchScopeCountsRecord {
    /// Derive a count record from raw count inputs.
    pub fn derive(inputs: SearchScopeCountsInputs) -> Self {
        let class = if inputs.visible_rows == 0
            && inputs.loaded_rows.is_none()
            && inputs.all_matching_rows.is_none()
        {
            SearchScopeCountsClass::NotComputed
        } else if inputs.readiness_is_ready
            && inputs.hidden_by_current_scope_rows == 0
            && inputs.hidden_by_policy_rows == 0
            && inputs.hidden_by_remote_cache_rows == 0
            && counts_collapse(
                inputs.visible_rows,
                inputs.loaded_rows,
                inputs.all_matching_rows,
            )
        {
            SearchScopeCountsClass::GloballyAuthoritative
        } else {
            SearchScopeCountsClass::PartialTruth
        };
        Self {
            counts_class_token: class.as_str().to_string(),
            visible_rows: inputs.visible_rows,
            loaded_rows: inputs.loaded_rows,
            all_matching_rows: inputs.all_matching_rows,
            hidden_by_current_scope_rows: inputs.hidden_by_current_scope_rows,
            hidden_by_policy_rows: inputs.hidden_by_policy_rows,
            hidden_by_remote_cache_rows: inputs.hidden_by_remote_cache_rows,
            readiness_is_ready: inputs.readiness_is_ready,
        }
    }

    /// Build a not-computed count record.
    pub fn not_computed(readiness_is_ready: bool) -> Self {
        Self::derive(SearchScopeCountsInputs {
            visible_rows: 0,
            loaded_rows: None,
            all_matching_rows: None,
            hidden_by_current_scope_rows: 0,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            readiness_is_ready,
        })
    }

    /// True when some query-matching row is known to be hidden.
    pub const fn has_hidden_rows(&self) -> bool {
        self.hidden_by_current_scope_rows > 0
            || self.hidden_by_policy_rows > 0
            || self.hidden_by_remote_cache_rows > 0
    }
}

/// Empty-state vocabulary for the first alpha search lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchNoResultsState {
    /// At least one result is visible.
    ResultsPresent,
    /// No result matched the query in the searched full scope.
    NoResults,
    /// Results exist outside the selected workset/sparse scope.
    NoResultsInThisWorkset,
    /// Excluded roots were not indexed, so the search cannot claim full coverage.
    IndexNotBuiltForExcludedRoots,
    /// Trust or policy blocks the result/action.
    BlockedByTrustOrPolicy,
}

impl SearchNoResultsState {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultsPresent => "results_present",
            Self::NoResults => "no_results",
            Self::NoResultsInThisWorkset => "no_results_in_this_workset",
            Self::IndexNotBuiltForExcludedRoots => "index_not_built_for_excluded_roots",
            Self::BlockedByTrustOrPolicy => "blocked_by_trust_or_policy",
        }
    }

    /// User-facing label for empty-state surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ResultsPresent => "Results present",
            Self::NoResults => "No results",
            Self::NoResultsInThisWorkset => "No results in this workset",
            Self::IndexNotBuiltForExcludedRoots => "Index not built for excluded roots",
            Self::BlockedByTrustOrPolicy => "Blocked by trust or policy",
        }
    }

    /// Derive the no-results state from scope counts and readiness.
    pub fn derive(
        counts: &SearchScopeCountsRecord,
        partial_index_note: Option<&str>,
        readiness_unavailable: bool,
        policy_limited: bool,
    ) -> Self {
        if counts.visible_rows > 0 {
            return Self::ResultsPresent;
        }
        if readiness_unavailable || policy_limited {
            return Self::BlockedByTrustOrPolicy;
        }
        if partial_index_note.is_some() && counts.hidden_by_current_scope_rows > 0 {
            return Self::IndexNotBuiltForExcludedRoots;
        }
        if counts.hidden_by_current_scope_rows > 0 {
            return Self::NoResultsInThisWorkset;
        }
        Self::NoResults
    }
}

/// Why rows are absent from the visible result list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenScopeReason {
    /// Rows matched outside the active workset/scope.
    OutsideCurrentScope,
    /// Rows are hidden by policy.
    PolicyLimitedView,
    /// Rows live under roots whose indexes are not built.
    IndexNotBuiltForExcludedRoots,
    /// Rows are behind a remote cache boundary.
    RemoteCache,
    /// Trust or policy blocks the lane.
    BlockedByTrustOrPolicy,
}

impl HiddenScopeReason {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::PolicyLimitedView => "policy_limited_view",
            Self::IndexNotBuiltForExcludedRoots => "index_not_built_for_excluded_roots",
            Self::RemoteCache => "remote_cache",
            Self::BlockedByTrustOrPolicy => "blocked_by_trust_or_policy",
        }
    }
}

/// Disclosure block for rows hidden by the active scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenScopeDisclosure {
    pub hidden_rows: u64,
    pub reason_tokens: Vec<String>,
    pub summary_label: String,
}

impl HiddenScopeDisclosure {
    /// Derive a disclosure block when a count has hidden rows or policy limits.
    pub fn derive(
        scope_label: &str,
        counts: &SearchScopeCountsRecord,
        partial_index_note: Option<&str>,
        policy_limited: bool,
    ) -> Option<Self> {
        let mut reasons = Vec::new();
        if policy_limited || counts.hidden_by_policy_rows > 0 {
            reasons.push(HiddenScopeReason::PolicyLimitedView);
        }
        if counts.hidden_by_current_scope_rows > 0 {
            reasons.push(HiddenScopeReason::OutsideCurrentScope);
        }
        if partial_index_note.is_some() && counts.hidden_by_current_scope_rows > 0 {
            reasons.push(HiddenScopeReason::IndexNotBuiltForExcludedRoots);
        }
        if counts.hidden_by_remote_cache_rows > 0 {
            reasons.push(HiddenScopeReason::RemoteCache);
        }

        if reasons.is_empty() {
            return None;
        }

        let hidden_rows = counts
            .hidden_by_current_scope_rows
            .max(counts.hidden_by_policy_rows)
            .max(counts.hidden_by_remote_cache_rows);
        let primary = reasons[0];
        let summary_label = match primary {
            HiddenScopeReason::PolicyLimitedView => {
                format!("{hidden_rows} rows hidden by Policy-limited view")
            }
            HiddenScopeReason::IndexNotBuiltForExcludedRoots => {
                format!("{hidden_rows} rows hidden because indexes are missing for excluded roots")
            }
            HiddenScopeReason::RemoteCache => {
                format!("{hidden_rows} rows only known through Remote cache")
            }
            HiddenScopeReason::OutsideCurrentScope => {
                format!("{hidden_rows} rows outside {scope_label}")
            }
            HiddenScopeReason::BlockedByTrustOrPolicy => {
                format!("{hidden_rows} rows blocked by trust or policy")
            }
        };
        Some(Self {
            hidden_rows,
            reason_tokens: reasons
                .into_iter()
                .map(|reason| reason.as_str().to_string())
                .collect(),
            summary_label,
        })
    }
}

/// Warning kind emitted when a result or action crosses a scope boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeWarningKind {
    /// Result/action is outside the active workset or sparse scope.
    OutsideCurrentScope,
    /// Policy narrows or blocks the visible result/action.
    PolicyLimitedView,
    /// Missing indexes prevent full excluded-root coverage.
    IndexNotBuiltForExcludedRoots,
    /// Remote cache is present but not authoritative for the current scope.
    RemoteCachePartial,
    /// Trust or policy blocks the lane.
    BlockedByTrustOrPolicy,
}

impl ScopeWarningKind {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::PolicyLimitedView => "policy_limited_view",
            Self::IndexNotBuiltForExcludedRoots => "index_not_built_for_excluded_roots",
            Self::RemoteCachePartial => "remote_cache_partial",
            Self::BlockedByTrustOrPolicy => "blocked_by_trust_or_policy",
        }
    }
}

/// Warning emitted next to result rows, batch actions, or candidate attachments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeWarningRecord {
    pub warning_kind_token: String,
    pub scope_label: String,
    pub message: String,
    pub action_tokens: Vec<String>,
}

impl ScopeWarningRecord {
    /// Build an outside-current-scope warning.
    pub fn outside_current_scope(active_scope_label: &str) -> Self {
        Self {
            warning_kind_token: ScopeWarningKind::OutsideCurrentScope.as_str().to_string(),
            scope_label: ScopeTruthLabel::OutsideCurrentScope
                .display_label()
                .to_string(),
            message: format!(
                "Result is outside {active_scope_label}; review scope before widening or jumping"
            ),
            action_tokens: vec![
                "widen_with_review".to_string(),
                "widen_to_full_workspace".to_string(),
            ],
        }
    }

    /// Build a policy-limited warning.
    pub fn policy_limited() -> Self {
        Self {
            warning_kind_token: ScopeWarningKind::PolicyLimitedView.as_str().to_string(),
            scope_label: ScopeTruthLabel::PolicyLimitedView
                .display_label()
                .to_string(),
            message: "Policy-limited view hides or blocks some matching rows".to_string(),
            action_tokens: vec!["open_policy_details".to_string()],
        }
    }

    /// Build a missing-index warning for excluded roots.
    pub fn missing_excluded_root_indexes() -> Self {
        Self {
            warning_kind_token: ScopeWarningKind::IndexNotBuiltForExcludedRoots
                .as_str()
                .to_string(),
            scope_label: ScopeTruthLabel::OutsideCurrentScope
                .display_label()
                .to_string(),
            message: "Indexes are not built for excluded roots".to_string(),
            action_tokens: vec!["build_missing_indexes".to_string()],
        }
    }

    /// Build a blocked warning for unavailable trust/policy lanes.
    pub fn blocked_by_trust_or_policy() -> Self {
        Self {
            warning_kind_token: ScopeWarningKind::BlockedByTrustOrPolicy
                .as_str()
                .to_string(),
            scope_label: ScopeTruthLabel::PolicyLimitedView
                .display_label()
                .to_string(),
            message: "Trust or policy blocks this result lane".to_string(),
            action_tokens: vec!["open_policy_details".to_string()],
        }
    }

    /// Derive warnings from counts and scope state.
    pub fn derive_for_counts(
        active_scope_label: &str,
        counts: &SearchScopeCountsRecord,
        partial_index_note: Option<&str>,
        readiness_unavailable: bool,
        policy_limited: bool,
    ) -> Vec<Self> {
        let mut warnings = Vec::new();
        if counts.hidden_by_current_scope_rows > 0 {
            warnings.push(Self::outside_current_scope(active_scope_label));
        }
        if partial_index_note.is_some() && counts.hidden_by_current_scope_rows > 0 {
            warnings.push(Self::missing_excluded_root_indexes());
        }
        if policy_limited || counts.hidden_by_policy_rows > 0 {
            warnings.push(Self::policy_limited());
        }
        if readiness_unavailable {
            warnings.push(Self::blocked_by_trust_or_policy());
        }
        warnings
    }
}

/// Scope truth packet for one search, graph, or AI context candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCandidateTruthRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub surface_token: String,
    pub active_scope_label: String,
    pub active_scope_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_mode_token: Option<String>,
    pub candidate_scope_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_or_module_ref: Option<String>,
    pub freshness_token: String,
    pub outside_current_scope: bool,
    pub policy_limited: bool,
    pub counts: SearchScopeCountsRecord,
    pub empty_state_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_scope_disclosure: Option<HiddenScopeDisclosure>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<ScopeWarningRecord>,
}

impl ScopeCandidateTruthRecord {
    /// Stable record-kind tag carried in serialized candidate truth records.
    pub const RECORD_KIND: &'static str = "scope_candidate_truth";

    /// Construct a candidate truth record from already-derived scope fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        surface: ScopeTruthSurface,
        active_scope_label: impl Into<String>,
        active_scope_class_token: impl Into<String>,
        stable_scope_id: Option<String>,
        scope_mode_token: Option<String>,
        repo_or_module_ref: Option<String>,
        freshness_token: impl Into<String>,
        outside_current_scope: bool,
        policy_limited: bool,
        counts: SearchScopeCountsRecord,
        empty_state: SearchNoResultsState,
        hidden_scope_disclosure: Option<HiddenScopeDisclosure>,
        warnings: Vec<ScopeWarningRecord>,
    ) -> Self {
        let active_scope_label = active_scope_label.into();
        let candidate_scope_label = if outside_current_scope {
            ScopeTruthLabel::OutsideCurrentScope
                .display_label()
                .to_string()
        } else if policy_limited {
            ScopeTruthLabel::PolicyLimitedView
                .display_label()
                .to_string()
        } else {
            active_scope_label.clone()
        };
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SCOPE_TRUTH_COUNTS_SCHEMA_VERSION,
            surface_token: surface.as_str().to_string(),
            active_scope_label,
            active_scope_class_token: active_scope_class_token.into(),
            stable_scope_id,
            scope_mode_token,
            candidate_scope_label,
            repo_or_module_ref,
            freshness_token: freshness_token.into(),
            outside_current_scope,
            policy_limited,
            counts,
            empty_state_token: empty_state.as_str().to_string(),
            hidden_scope_disclosure,
            warnings,
        }
    }
}

fn counts_collapse(
    visible_rows: u64,
    loaded_rows: Option<u64>,
    all_matching_rows: Option<u64>,
) -> bool {
    match (loaded_rows, all_matching_rows) {
        (Some(loaded), Some(all)) => visible_rows == loaded && loaded == all,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn globally_authoritative_counts_require_all_three_counts() {
        let counts = SearchScopeCountsRecord::derive(SearchScopeCountsInputs {
            visible_rows: 3,
            loaded_rows: Some(3),
            all_matching_rows: Some(3),
            hidden_by_current_scope_rows: 0,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            readiness_is_ready: true,
        });
        assert_eq!(counts.counts_class_token, "globally_authoritative");
    }

    #[test]
    fn hidden_rows_force_partial_truth() {
        let counts = SearchScopeCountsRecord::derive(SearchScopeCountsInputs {
            visible_rows: 0,
            loaded_rows: Some(0),
            all_matching_rows: Some(2),
            hidden_by_current_scope_rows: 2,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            readiness_is_ready: true,
        });
        assert_eq!(counts.counts_class_token, "partial_truth");
        assert_eq!(
            SearchNoResultsState::derive(&counts, None, false, false),
            SearchNoResultsState::NoResultsInThisWorkset
        );
    }
}
