//! Placeholder-state record types used by shell surfaces.
//!
//! This module mirrors the `placeholder_state_record` boundary schema defined in
//! `schemas/ux/placeholder_state.schema.json` so shell surfaces can share a
//! typed placeholder/degraded-state contract without inventing per-surface
//! wording.

use serde::{Deserialize, Serialize};

use super::DegradedStateToken;

/// Fixture metadata carried by UX JSON examples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureMetadata {
    /// Fixture name (unique within a fixture directory).
    pub name: String,
    /// Human-readable scenario description.
    pub scenario: String,
    /// Optional fixture axes used by reviewers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exercised_axes: Option<serde_json::Value>,
    /// Optional contract section pointers used during review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_sections: Option<Vec<String>>,
}

/// Closed surface-family vocabulary for placeholder records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    Shell,
    StartCenter,
    Search,
    DocsHelp,
    InstallReview,
    ProviderBacked,
    TreeOrList,
    StatusStrip,
    GenericSurface,
}

/// Closed state-class vocabulary for placeholder records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderStateClass {
    EmptyNoResults,
    EmptyFirstRun,
    Loading,
    Indexing,
    Probing,
    Blocked,
    Degraded,
    MissingExtensionOrProvider,
    PartialData,
    StaleCachedContent,
}

/// Closed truth-class vocabulary carried by placeholder records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthClass {
    UserAuthoredDurableTruth,
    WorkspaceVcsTruth,
    RuntimeObservedTruth,
    DerivedIndexedTruth,
    SessionCollaborationTruth,
    AiInferredTruth,
}

/// Summary block describing why the placeholder is visible and what remains usable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderStateSummary {
    pub area_purpose: String,
    pub cause_class: PlaceholderCauseClass,
    pub cause_summary: String,
    pub preserved_capability: Vec<String>,
    pub reduced_or_blocked_capability: Vec<String>,
    pub readiness_basis: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_ref: Option<String>,
}

/// Closed cause vocabulary for placeholder summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderCauseClass {
    NoWorkStarted,
    FilterOrScopeZero,
    NotReadyWarming,
    BackgroundIndexIncomplete,
    ProbeInProgress,
    ProviderUnreachable,
    ExtensionMissing,
    PolicyOrTrustBlocked,
    PartialScope,
    StalePriorContent,
    CachedOwnerUnreachable,
    FailedLastAttempt,
}

/// Presentation policy for a placeholder surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentPolicy {
    pub presentation_pattern: PresentationPattern,
    pub animation_policy: AnimationPolicy,
    pub content_shaped_placeholder_allowed: bool,
    pub simulates_real_content: bool,
    pub must_downgrade_to_explicit_text: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_label: Option<String>,
    pub progressive_hydration: ProgressiveHydrationPolicy,
}

/// Closed presentation-pattern vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationPattern {
    ExplicitEmptyText,
    SkeletonRows,
    StaticPlaceholder,
    ProgressivePartialResults,
    TopOfPaneProgress,
    RetainedPriorContent,
    PlaceholderRow,
    ExplicitBlockedText,
}

/// Closed animation-policy vocabulary for placeholder surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationPolicy {
    None,
    Static,
    ReducedMotionStaticWithProgressText,
    SubtleShimmerAllowed,
    ShimmerForbidden,
}

/// Progressive hydration policy for placeholder surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressiveHydrationPolicy {
    pub allowed: bool,
    pub hydration_label_required: bool,
    pub row_reorder_above_fold_allowed: bool,
}

/// Prior-content coexistence contract for a placeholder surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorContent {
    pub coexistence_class: PriorContentCoexistenceClass,
    pub label_required: bool,
    pub basis_label: Option<String>,
    pub freshness_label: Option<String>,
    pub staleness_or_cache_cause: Option<String>,
    pub mutating_actions_require_revalidation: bool,
}

/// Closed prior-content coexistence vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorContentCoexistenceClass {
    NoPriorContent,
    PriorContentHidden,
    CachedPriorContentVisible,
    StalePriorContentVisible,
    PartialCurrentContentVisible,
    MixedCachedAndLiveVisible,
    PolicyHiddenPriorContent,
}

/// Visible cues required on placeholder surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibleCues {
    pub primary_label: String,
    pub cause_label: String,
    pub readiness_badge_label: String,
    pub accessible_name: String,
    pub details_route_label: String,
}

/// A safe next action offered by a placeholder surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextAction {
    pub action_id: String,
    pub label: String,
    pub action_kind: NextActionKind,
    pub command_id: String,
    pub safe_without_revalidation: bool,
    pub preserves_current_work: bool,
}

/// Closed next-action kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextActionKind {
    Open,
    ClearFilter,
    WidenScope,
    NarrowScope,
    OpenDetails,
    Retry,
    Cancel,
    ContinueLocally,
    ContinueReadOnly,
    InstallOrEnable,
    OpenOfflineFallback,
    RequestAccess,
    ExportEvidence,
    RepairOrRebuild,
}

/// Rendering guardrails for placeholder surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessGuards {
    pub claims_ready_without_evidence: bool,
    pub green_status_allowed: bool,
    pub rendered_ready_label_allowed: bool,
    pub success_icon_allowed: bool,
    pub state_tooltip_only_allowed: bool,
    pub forbidden_renderings: Vec<String>,
}

/// Cross-surface projection rules for placeholder records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceProjection {
    pub projection_class: ProjectionClass,
    pub status_strip_state: Option<StatusStripState>,
    pub view_freshness_class: Option<ViewFreshnessClass>,
    pub search_readiness_state: Option<SearchReadinessState>,
    pub reuses_shared_readiness_language: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_contract_refs: Option<Vec<String>>,
}

/// Closed projection class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionClass {
    Shell,
    StartCenter,
    Search,
    DocsHelp,
    InstallReview,
    ProviderBacked,
    Generic,
}

/// Status strip state projected from a placeholder record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusStripState {
    Loading,
    Warm,
    Partial,
    Stale,
    Degraded,
    Blocked,
    Ready,
}

/// View freshness class projected from a placeholder record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewFreshnessClass {
    LiveExact,
    SnapshotExact,
    Partial,
    Stale,
    ApproximateDerived,
}

/// Search readiness state projected from a placeholder record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchReadinessState {
    NotIndexed,
    HotSetReady,
    PartialIndex,
    WarmIndex,
    FullyIndexed,
    StaleIndex,
    Reindexing,
    IndexUnavailable,
}

/// Accessibility contract for a placeholder surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Accessibility {
    pub focus_target_id: String,
    pub state_announced: bool,
    pub next_action_keyboard_reachable: bool,
    pub details_keyboard_reachable: bool,
    pub reduced_motion_equivalent: String,
}

/// Support/export fields that must preserve placeholder state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExport {
    pub export_field_refs: Vec<String>,
    pub redaction_class: SupportRedactionClass,
    pub state_class_preserved: bool,
    pub content_basis_preserved: bool,
    pub stale_or_cached_basis_preserved: bool,
    pub next_action_preserved: bool,
}

/// Closed redaction class vocabulary for placeholder support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportRedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    SupportRestricted,
}

/// `placeholder_state_record` instances describe empty/loading/degraded placeholder states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderStateRecord {
    /// Optional fixture metadata included in worked examples.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub __fixture__: Option<FixtureMetadata>,
    pub record_kind: String,
    pub placeholder_state_schema_version: u32,
    pub case_id: String,
    pub surface_family: SurfaceFamily,
    pub state_class: PlaceholderStateClass,
    pub truth_classes: Vec<TruthClass>,
    pub degraded_state_tokens: Vec<DegradedStateToken>,
    pub state_summary: PlaceholderStateSummary,
    pub content_policy: ContentPolicy,
    pub prior_content: PriorContent,
    pub visible_cues: VisibleCues,
    pub next_actions: Vec<NextAction>,
    pub readiness_guards: ReadinessGuards,
    pub cross_surface_projection: CrossSurfaceProjection,
    pub accessibility: Accessibility,
    pub support_export: SupportExport,
    pub narrative_refs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn placeholder_state_fixtures_parse() {
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux");
        let fixtures = [
            fixtures_root.join("placeholder_cases/first_run_start_center.json"),
            fixtures_root.join("placeholder_cases/indexing_in_progress_search.json"),
            fixtures_root.join("placeholder_cases/no_results_exact_search.json"),
            fixtures_root.join("placeholder_cases/provider_unavailable_docs_help.json"),
            fixtures_root.join("placeholder_cases/stale_cached_content_shell.json"),
            fixtures_root.join("state_cards/shell_unsupported_surface_placeholder.json"),
        ];

        for path in fixtures {
            let payload = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("fixture read failed ({}): {err}", path.display()));
            let record: PlaceholderStateRecord = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture parse failed ({}): {err}", path.display()));

            assert_eq!(record.record_kind, "placeholder_state_record");
            assert_eq!(record.placeholder_state_schema_version, 1);
        }
    }

    #[test]
    fn state_cards_fixture_exercises_unsupported_token() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ux/state_cards/shell_unsupported_surface_placeholder.json");
        let payload = std::fs::read_to_string(&fixture_path).unwrap_or_else(|err| {
            panic!("fixture read failed ({}): {err}", fixture_path.display())
        });
        let record: PlaceholderStateRecord = serde_json::from_str(&payload).unwrap_or_else(|err| {
            panic!("fixture parse failed ({}): {err}", fixture_path.display())
        });
        assert!(record
            .degraded_state_tokens
            .iter()
            .any(|token| *token == DegradedStateToken::Unsupported));
    }
}
