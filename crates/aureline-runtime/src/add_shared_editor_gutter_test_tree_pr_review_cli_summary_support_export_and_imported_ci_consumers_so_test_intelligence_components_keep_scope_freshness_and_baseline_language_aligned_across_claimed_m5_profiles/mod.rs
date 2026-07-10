//! Shared editor-gutter, test-tree, PR/review, CLI-summary, imported-CI-detail,
//! and support/export consumers for the frozen M5 test-intelligence components.
//!
//! This module is the M05-1033 consumer-adoption lane over the frozen M5
//! test-intelligence component matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! and the four M05-1029..1032 primitive resolvers (coverage summary bar /
//! coverage overlay marker, flaky-state badge / retry-history row, snapshot-or-
//! golden review card / coverage-import merge sheet, and test-generation
//! suggestion card). Where the freeze matrix defines the seven reusable
//! quality-evidence component families and 1029-1032 resolve their per-surface
//! truth, this lane proves those families are reusable *primitives* by adopting
//! them across the claimed M5 quality surfaces users actually inspect before
//! trusting a green bar, a flaky verdict, or a generated test:
//!
//! 1. editor gutters and inline coverage summaries (the editor surface),
//! 2. the test tree (flaky and retry evidence),
//! 3. PR / review views (coverage diffs and snapshot review),
//! 4. CLI summaries (headless coverage / merge / generation),
//! 5. imported-CI detail views, and
//! 6. support / export packets.
//!
//! Each [`IntelConsumerRow`] points back to exactly one canonical component
//! family (its primitive schema + release-proof packet) instead of cloning
//! surface-local test vocabulary, and every consumer keeps the identical
//! controlled label families for provenance / freshness, included-run scope,
//! artifact baseline identity, raw-or-text fallback, and generated-test
//! assumption boundaries, plus one shared state lexicon so `imported`,
//! `suspected flaky`, and `generated` mean the same thing on every surface.
//!
//! When a consumer's evidence is weaker than a verified current-run claim — the
//! result was imported, a shard was omitted from the included run set, the
//! provenance is cached or stale, the flakiness is only suspected rather than
//! reproduced, or a generated test still carries unverified assumptions — the
//! row **auto-narrows** its visible claim language and discloses the reduction
//! with an auto-narrow banner naming the reason and a recovery hint, rather than
//! renaming or dropping the governed scope / freshness / baseline label, letting
//! a single percentage hide a shard omission, labeling one intermittent failure
//! as confirmed flakiness, or bundling generated assertion, fixture, and
//! snapshot changes into one opaque apply path.
//!
//! The packet is metadata-only: raw runner output, coverage line data, assertion
//! diffs, snapshot bytes, stack frames, credentials, and provider payloads never
//! cross this boundary; the packet carries only typed class tokens, opaque
//! summary / evidence refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-test-intelligence-component-consumer.schema.json`](../../../../schemas/ui/m5-test-intelligence-component-consumer.schema.json).
//! The contract doc is
//! [`docs/testing/m5_test_intelligence_component_consumer_contract.md`](../../../../docs/testing/m5_test_intelligence_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5TestIntelligenceComponentFamily, M5TestIntelligenceProvenanceClass,
    M5_TEST_INTELLIGENCE_COMPONENT_DOC_REF, M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
};
use crate::implement_coverage_summary_bars_and_coverage_overlay_markers_with_included_run_provenance_line_versus_branch_or_partial_truth_changed_file_emphasis_and_open_report_continuity_across_claimed_m5_test_surfaces::{
    M5_COVERAGE_COMPONENTS_ARTIFACT_REF, M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF,
    M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF,
};
use crate::implement_flaky_state_badges_and_retry_history_rows_with_controlled_verdict_vocabulary_classifier_confidence_retry_window_visibility_environment_drift_notes_and_rerun_or_open_logs_parity_across_claimed_m5_quality_surfaces::{
    M5_FLAKY_RETRY_COMPONENTS_ARTIFACT_REF, M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF,
    M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF,
};
use crate::implement_snapshot_or_golden_review_cards_and_coverage_import_merge_sheets_with_artifact_baseline_identity_raw_or_text_fallback_shard_inclusion_truth_and_stale_or_incompatible_warnings_across_claimed_m5_review_surfaces::{
    M5_SNAPSHOT_MERGE_COMPONENTS_ARTIFACT_REF, M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF,
    M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF,
};
use crate::implement_test_generation_suggestion_cards_with_uncovered_path_or_bug_trigger_truth_assumption_summaries_helper_fixture_snapshot_separation_sandbox_validation_and_diff_first_apply_parity_across_claimed_m5_ai_test_flows::{
    M5_SUGGESTION_CARD_COMPONENTS_ARTIFACT_REF, M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF,
};

/// Schema version stamped on the M05-1033 consumer packet.
pub const TEST_INTEL_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`IntelConsumerPacket`].
pub const TEST_INTEL_CONSUMER_RECORD_KIND: &str = "m5_test_intelligence_component_consumer_packet";

/// Stable record-kind tag carried by each [`IntelConsumerRow`].
pub const TEST_INTEL_CONSUMER_ROW_RECORD_KIND: &str = "m5_test_intelligence_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const TEST_INTEL_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-test-intelligence-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_INTEL_CONSUMER_DOC_REF: &str =
    "docs/testing/m5_test_intelligence_component_consumer_contract.md";

/// Repo-relative path of the frozen test-intelligence component matrix these
/// consumers adopt.
pub const TEST_INTEL_CONSUMER_MATRIX_REF: &str = M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen-matrix contract doc these consumers adopt.
pub const TEST_INTEL_CONSUMER_MATRIX_DOC_REF: &str = M5_TEST_INTELLIGENCE_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const TEST_INTEL_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-intelligence-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const TEST_INTEL_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_INTEL_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-test-intelligence-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_INTEL_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-consumer-proof/report.md";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// test-intelligence components: provenance / freshness, included-run scope,
/// artifact baseline identity (including line-versus-branch), raw-or-text
/// fallback, and generated-test assumption boundaries. The union of every row's
/// `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "provenance_and_freshness",
    "included_run_scope",
    "baseline_identity",
    "raw_or_text_fallback",
    "assumption_boundary",
];

/// The single shared state lexicon every consumer keeps verbatim so an imported
/// result, a merely-suspected flaky test, and a generated test read the same on
/// every surface. This is the AC anchor that quality surfaces stop diverging on
/// what `imported`, `suspected flaky`, and `generated` mean for the same
/// underlying evidence object.
pub const SHARED_STATE_LEXICON: [&str; 3] = [
    "imported_not_local",
    "suspected_not_confirmed",
    "generated_review_first",
];

/// The canonical primitive schema that defines a component family's contract.
/// Consumers must point at this schema instead of inventing a surface-local one.
pub const fn family_canonical_schema_ref(
    family: M5TestIntelligenceComponentFamily,
) -> &'static str {
    use M5TestIntelligenceComponentFamily::*;
    match family {
        CoverageSummaryBar => M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF,
        CoverageOverlayMarker => M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF,
        FlakyStateBadge => M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF,
        RetryHistoryRow => M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF,
        SnapshotReviewCard => M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF,
        CoverageImportMergeSheet => M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF,
        TestGenerationSuggestionCard => M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF,
    }
}

/// The canonical release-proof packet that defines a component family's first
/// resolved truth. Consumers point back to this packet rather than cloning it.
/// Twin primitives share one packet.
pub const fn family_canonical_packet_ref(
    family: M5TestIntelligenceComponentFamily,
) -> &'static str {
    use M5TestIntelligenceComponentFamily::*;
    match family {
        // Coverage summary bar and coverage overlay marker are the two halves of
        // the same 1029 coverage-components primitive.
        CoverageSummaryBar | CoverageOverlayMarker => M5_COVERAGE_COMPONENTS_ARTIFACT_REF,
        // Flaky-state badge and retry-history row are the two halves of the same
        // 1030 flaky/retry primitive.
        FlakyStateBadge | RetryHistoryRow => M5_FLAKY_RETRY_COMPONENTS_ARTIFACT_REF,
        // Snapshot-review card and coverage-import merge sheet are the two halves
        // of the same 1031 snapshot/merge primitive.
        SnapshotReviewCard | CoverageImportMergeSheet => M5_SNAPSHOT_MERGE_COMPONENTS_ARTIFACT_REF,
        TestGenerationSuggestionCard => M5_SUGGESTION_CARD_COMPONENTS_ARTIFACT_REF,
    }
}

/// A short human-readable label for a component family, for the Markdown report.
pub const fn family_label(family: M5TestIntelligenceComponentFamily) -> &'static str {
    use M5TestIntelligenceComponentFamily::*;
    match family {
        CoverageSummaryBar => "Coverage-summary bar",
        CoverageOverlayMarker => "Coverage-overlay marker",
        FlakyStateBadge => "Flaky-state badge",
        RetryHistoryRow => "Retry-history row",
        SnapshotReviewCard => "Snapshot-review card",
        CoverageImportMergeSheet => "Coverage-import / merge sheet",
        TestGenerationSuggestionCard => "Test-generation suggestion card",
    }
}

/// The explicit actions a consumer of a family must keep reachable so a green
/// bar, flaky verdict, or generated test never strands its user without a way
/// back to raw evidence, a rerun, the logs, a report, or a rollback (spec
/// guardrail: keep raw/text fallback and rerun/open-logs actions explicit).
pub const fn required_actions_for(
    family: M5TestIntelligenceComponentFamily,
) -> &'static [&'static str] {
    use M5TestIntelligenceComponentFamily::*;
    match family {
        CoverageSummaryBar | CoverageOverlayMarker | CoverageImportMergeSheet => {
            &["rerun", "open_report"]
        }
        FlakyStateBadge | RetryHistoryRow => &["rerun", "open_logs"],
        SnapshotReviewCard => &["open_raw_or_text_fallback", "rerun"],
        TestGenerationSuggestionCard => &["open_diff_preview", "rollback"],
    }
}

/// The narrow reason a provenance class forces a consumer to disclose. A weaker-
/// than-current provenance may never read as a full verified current-run claim.
pub const fn provenance_forced_reason(
    provenance: M5TestIntelligenceProvenanceClass,
) -> Option<M5IntelClaimNarrowReason> {
    use M5TestIntelligenceProvenanceClass::*;
    match provenance {
        ImportedCiArtifact => Some(M5IntelClaimNarrowReason::EvidenceImported),
        CachedLocalResult | StalePriorResult => Some(M5IntelClaimNarrowReason::ProvenanceStale),
        SuspectedFlaky => Some(M5IntelClaimNarrowReason::FlakinessUnconfirmed),
        VerifiedCurrentRun | ReproducedFlaky | StableAgain | ManuallyMuted | Unknown => None,
    }
}

/// True when a provenance class is an imported CI artifact and may never claim a
/// full verified current-run parity.
pub const fn provenance_is_imported(provenance: M5TestIntelligenceProvenanceClass) -> bool {
    matches!(
        provenance,
        M5TestIntelligenceProvenanceClass::ImportedCiArtifact
    )
}

/// The six claimed M5 quality consumer classes that must each adopt at least one
/// canonical component family beyond the primitive resolvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedIntelConsumerClass {
    /// Editor gutters and inline coverage summaries.
    EditorSurface,
    /// The test tree (flaky and retry evidence).
    TestTree,
    /// PR / review views (coverage diffs and snapshot review).
    ReviewSurface,
    /// CLI / headless summaries.
    CliSummary,
    /// Imported-CI detail views.
    ImportedCiDetail,
    /// Support / export packets.
    SupportExport,
}

impl SharedIntelConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [SharedIntelConsumerClass; 6] = [
        SharedIntelConsumerClass::EditorSurface,
        SharedIntelConsumerClass::TestTree,
        SharedIntelConsumerClass::ReviewSurface,
        SharedIntelConsumerClass::CliSummary,
        SharedIntelConsumerClass::ImportedCiDetail,
        SharedIntelConsumerClass::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorSurface => "editor_surface",
            Self::TestTree => "test_tree",
            Self::ReviewSurface => "review_surface",
            Self::CliSummary => "cli_summary",
            Self::ImportedCiDetail => "imported_ci_detail",
            Self::SupportExport => "support_export",
        }
    }
}

/// The concrete M5 quality consumer surface a component is embedded in. Each
/// surface belongs to exactly one [`SharedIntelConsumerClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedIntelConsumerSurface {
    /// The editor gutter coverage overlay.
    EditorGutterOverlay,
    /// The inline editor coverage summary.
    EditorCoverageSummary,
    /// The test-tree panel (flaky / retry evidence).
    TestTreePanel,
    /// A PR / review coverage-diff surface.
    ReviewCoverageDiff,
    /// A PR / review snapshot-review card surface.
    ReviewSnapshotCard,
    /// A CLI / headless quality summary.
    CliQualitySummary,
    /// An imported-CI detail view.
    ImportedCiDetailView,
    /// A support / export packet.
    SupportExportPacket,
}

impl SharedIntelConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [SharedIntelConsumerSurface; 8] = [
        SharedIntelConsumerSurface::EditorGutterOverlay,
        SharedIntelConsumerSurface::EditorCoverageSummary,
        SharedIntelConsumerSurface::TestTreePanel,
        SharedIntelConsumerSurface::ReviewCoverageDiff,
        SharedIntelConsumerSurface::ReviewSnapshotCard,
        SharedIntelConsumerSurface::CliQualitySummary,
        SharedIntelConsumerSurface::ImportedCiDetailView,
        SharedIntelConsumerSurface::SupportExportPacket,
    ];

    /// The consumer class this surface belongs to.
    pub const fn consumer_class(self) -> SharedIntelConsumerClass {
        match self {
            Self::EditorGutterOverlay | Self::EditorCoverageSummary => {
                SharedIntelConsumerClass::EditorSurface
            }
            Self::TestTreePanel => SharedIntelConsumerClass::TestTree,
            Self::ReviewCoverageDiff | Self::ReviewSnapshotCard => {
                SharedIntelConsumerClass::ReviewSurface
            }
            Self::CliQualitySummary => SharedIntelConsumerClass::CliSummary,
            Self::ImportedCiDetailView => SharedIntelConsumerClass::ImportedCiDetail,
            Self::SupportExportPacket => SharedIntelConsumerClass::SupportExport,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorGutterOverlay => "editor_gutter_overlay",
            Self::EditorCoverageSummary => "editor_coverage_summary",
            Self::TestTreePanel => "test_tree_panel",
            Self::ReviewCoverageDiff => "review_coverage_diff",
            Self::ReviewSnapshotCard => "review_snapshot_card",
            Self::CliQualitySummary => "cli_quality_summary",
            Self::ImportedCiDetailView => "imported_ci_detail_view",
            Self::SupportExportPacket => "support_export_packet",
        }
    }
}

/// Why a consumer auto-narrows its visible claim language below a verified
/// current-run claim. These are the five spec-named auto-narrow conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelClaimNarrowReason {
    /// The evidence was imported from a CI artifact rather than produced by a
    /// verified current local run.
    EvidenceImported,
    /// A shard was omitted from the included run set, so a single percentage
    /// would hide the omission.
    ShardScopeOmitted,
    /// The provenance is a cached or stale prior result rather than fresh.
    ProvenanceStale,
    /// The flakiness is only suspected from a single occurrence, not reproduced
    /// across attempts.
    FlakinessUnconfirmed,
    /// A generated test still carries unverified assumptions and its assertion,
    /// fixture, and snapshot changes stay review-first rather than one opaque
    /// apply.
    GeneratedAssumptionsUnverified,
}

impl M5IntelClaimNarrowReason {
    /// Every auto-narrow reason, in declaration order.
    pub const ALL: [M5IntelClaimNarrowReason; 5] = [
        M5IntelClaimNarrowReason::EvidenceImported,
        M5IntelClaimNarrowReason::ShardScopeOmitted,
        M5IntelClaimNarrowReason::ProvenanceStale,
        M5IntelClaimNarrowReason::FlakinessUnconfirmed,
        M5IntelClaimNarrowReason::GeneratedAssumptionsUnverified,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceImported => "evidence_imported",
            Self::ShardScopeOmitted => "shard_scope_omitted",
            Self::ProvenanceStale => "provenance_stale",
            Self::FlakinessUnconfirmed => "flakiness_unconfirmed",
            Self::GeneratedAssumptionsUnverified => "generated_assumptions_unverified",
        }
    }

    /// The honest, non-generic claim phrase this narrowing shows.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::EvidenceImported => "imported CI evidence — not a verified local run",
            Self::ShardScopeOmitted => "a shard is omitted from the included run set",
            Self::ProvenanceStale => "cached or stale result — not the current source",
            Self::FlakinessUnconfirmed => "suspected flaky — not reproduced across attempts",
            Self::GeneratedAssumptionsUnverified => {
                "generated test carries unverified assumptions — review before apply"
            }
        }
    }

    /// The recovery hint the narrowed consumer offers to restore full certainty.
    pub const fn recovery(self) -> &'static str {
        match self {
            Self::EvidenceImported => "rerun locally to produce a verified current-run result",
            Self::ShardScopeOmitted => {
                "include the omitted shard, or open the merge sheet for scope"
            }
            Self::ProvenanceStale => "rerun to refresh the provenance to the current source",
            Self::FlakinessUnconfirmed => "rerun to reproduce before confirming the flaky verdict",
            Self::GeneratedAssumptionsUnverified => {
                "open the diff preview and review the assumptions before applying"
            }
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full scope / freshness / baseline / fallback / assumption parity; a
    /// verified current-run claim.
    Preserved,
    /// The claim was auto-narrowed and disclosed, but the labels are still
    /// preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// scope, provenance, and baseline identity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The five hard invariants a consumer must never violate. Each field must be
/// `false`; a `true` value fails validation with a [`IntelConsumerViolation`].
/// These are the spec guardrails: a single percentage may not hide a shard
/// omission or stale provenance, one intermittent failure may not read as
/// confirmed flakiness, and generated changes may not collapse into one opaque
/// apply — nor may any surface reword the shared scope / freshness / baseline
/// language or invent an alternate state label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowInvariants {
    /// True if this consumer collapses a shard omission behind a single
    /// percentage (must be false).
    pub collapses_shard_omission_into_single_percentage: bool,
    /// True if this consumer labels one intermittent failure as confirmed
    /// flakiness (must be false).
    pub labels_intermittent_as_confirmed_flaky: bool,
    /// True if this consumer bundles generated assertion / fixture / snapshot
    /// changes into one opaque apply path (must be false).
    pub bundles_generated_changes_into_opaque_apply: bool,
    /// True if this consumer rewords the shared scope / freshness / baseline
    /// language for its own surface (must be false).
    pub rewords_scope_freshness_or_baseline_per_surface: bool,
    /// True if this consumer invents an alternate state label outside the frozen
    /// vocabulary (must be false).
    pub invents_alternate_state_label: bool,
}

impl RowInvariants {
    /// The default, honest state: every invariant holds (all `false`).
    pub const fn honest() -> Self {
        Self {
            collapses_shard_omission_into_single_percentage: false,
            labels_intermittent_as_confirmed_flaky: false,
            bundles_generated_changes_into_opaque_apply: false,
            rewords_scope_freshness_or_baseline_per_surface: false,
            invents_alternate_state_label: false,
        }
    }

    /// Whether every hard invariant holds (all fields `false`).
    pub const fn holds(&self) -> bool {
        !self.collapses_shard_omission_into_single_percentage
            && !self.labels_intermittent_as_confirmed_flaky
            && !self.bundles_generated_changes_into_opaque_apply
            && !self.rewords_scope_freshness_or_baseline_per_surface
            && !self.invents_alternate_state_label
    }
}

/// The auto-narrow banner a consumer shows when its claim is weaker than a
/// verified current-run claim. It names every narrow reason and a recovery hint
/// so the reduction is disclosed rather than silently applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoNarrowBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The narrow-reason tokens; must equal the row's `claim_narrow_reasons`.
    #[serde(default)]
    pub reasons: Vec<String>,
    /// The recovery hint(s) that would restore full certainty.
    #[serde(default)]
    pub recovery_hints: Vec<String>,
}

/// One consumer adopting one canonical test-intelligence component family on one
/// M5 quality consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelConsumerRow {
    /// Record kind; must equal [`TEST_INTEL_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_INTEL_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: SharedIntelConsumerClass,
    /// The concrete consumer surface; must belong to `consumer_class`.
    pub consumer_surface: SharedIntelConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5TestIntelligenceComponentFamily,
    /// The canonical primitive schema for the family. Must equal
    /// `family_canonical_schema_ref(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain `family_canonical_packet_ref(component_family)`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local test prose.
    pub references_canonical_not_local_prose: bool,
    /// The provenance / freshness class of the evidence this consumer renders.
    pub result_provenance: M5TestIntelligenceProvenanceClass,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The shared state lexicon the consumer keeps identical to every other
    /// surface (must equal [`SHARED_STATE_LEXICON`]).
    #[serde(default)]
    pub shared_state_lexicon: Vec<String>,
    /// The explicit actions the consumer keeps reachable (must be a superset of
    /// `required_actions_for(component_family)`).
    #[serde(default)]
    pub preserved_actions: Vec<String>,
    /// The reasons this consumer auto-narrows its claim, if any.
    #[serde(default)]
    pub claim_narrow_reasons: Vec<M5IntelClaimNarrowReason>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The auto-narrow banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrow_banner: Option<AutoNarrowBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// The five hard invariants this consumer must never violate.
    pub invariants: RowInvariants,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl IntelConsumerRow {
    /// Returns true when the consumer auto-narrows its claim.
    pub fn is_narrowed(&self) -> bool {
        !self.claim_narrow_reasons.is_empty()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        self.consumer_surface.consumer_class() == self.consumer_class
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == family_canonical_schema_ref(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == family_canonical_packet_ref(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label
    /// families and the shared state lexicon rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && self
                .shared_state_lexicon
                .iter()
                .map(String::as_str)
                .eq(SHARED_STATE_LEXICON)
    }

    /// Guardrail: the family-appropriate raw-fallback / rerun / open-logs /
    /// report / rollback actions all remain reachable.
    pub fn preserves_required_actions(&self) -> bool {
        required_actions_for(self.component_family)
            .iter()
            .all(|a| self.preserved_actions.iter().any(|p| p == a))
    }

    /// AC2 (provenance truth): a weaker-than-current provenance carries its
    /// forced narrow reason and never claims a full current-run parity; a
    /// verified current-run result never claims imported or stale provenance.
    pub fn provenance_claim_consistent(&self) -> bool {
        let has = |r: M5IntelClaimNarrowReason| self.claim_narrow_reasons.contains(&r);
        if let Some(forced) = provenance_forced_reason(self.result_provenance) {
            if !has(forced) {
                return false;
            }
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        }
        if matches!(
            self.result_provenance,
            M5TestIntelligenceProvenanceClass::VerifiedCurrentRun
        ) && (has(M5IntelClaimNarrowReason::EvidenceImported)
            || has(M5IntelClaimNarrowReason::ProvenanceStale))
        {
            return false;
        }
        true
    }

    /// AC2 (auto-narrow disclosure): a narrowed consumer discloses the reduction
    /// with an auto-narrow banner whose reasons match the row and that carries a
    /// recovery hint; a full consumer carries no banner.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            let expected: Vec<String> = self
                .claim_narrow_reasons
                .iter()
                .map(|r| r.as_str().to_owned())
                .collect();
            match &self.auto_narrow_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.reasons != expected
                        || banner.recovery_hints.is_empty()
                        || banner.recovery_hints.iter().any(|h| h.trim().is_empty())
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.auto_narrow_banner.is_some() {
            // A full-claim consumer must not carry a spurious banner.
            return false;
        } else if self.label_parity != LabelParityState::Preserved {
            // A consumer with no narrow reasons is a verified current-run claim.
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_INTEL_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == TEST_INTEL_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.preserved_actions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.source_refs.is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        let reasons = if self.claim_narrow_reasons.is_empty() {
            "full".to_owned()
        } else {
            self.claim_narrow_reasons
                .iter()
                .map(|r| r.as_str())
                .collect::<Vec<_>>()
                .join("+")
        };
        format!(
            "surface={surface} class={class} family={family} provenance={provenance} \
label_parity={label_parity} narrow={reasons}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            provenance = self.result_provenance.as_str(),
            label_parity = self.label_parity.as_str(),
            reasons = reasons,
        )
    }
}

/// Rolled-up summary of an M05-1033 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_preserve_required_actions: bool,
    pub all_rows_provenance_claim_consistent: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_row_invariants_hold: bool,
    pub editor_surface_present: bool,
    pub test_tree_present: bool,
    pub review_surface_present: bool,
    pub cli_summary_present: bool,
    pub imported_ci_detail_present: bool,
    pub support_export_present: bool,
    pub label_family_coverage_complete: bool,
    pub all_narrow_reasons_demonstrated: bool,
    pub imported_and_current_both_present: bool,
    pub shared_lexicon_uniform: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`IntelConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntelConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<IntelConsumerRow>,
}

/// Checked-in M05-1033 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<IntelConsumerRow>,
    pub summary: IntelConsumerSummary,
}

impl IntelConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: IntelConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_INTEL_CONSUMER_SCHEMA_VERSION,
            record_kind: TEST_INTEL_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: IntelConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_preserve_required_actions: false,
                all_rows_provenance_claim_consistent: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_row_invariants_hold: false,
                editor_surface_present: false,
                test_tree_present: false,
                review_surface_present: false,
                cli_summary_present: false,
                imported_ci_detail_present: false,
                support_export_present: false,
                label_family_coverage_complete: false,
                all_narrow_reasons_demonstrated: false,
                imported_and_current_both_present: false,
                shared_lexicon_uniform: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5TestIntelligenceComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The narrow reasons demonstrated by some row.
    pub fn demonstrated_narrow_reasons(&self) -> BTreeSet<M5IntelClaimNarrowReason> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_narrow_reasons.iter().copied())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5TestIntelligenceComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<SharedIntelConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether at least one imported-provenance auto-narrowing row and one
    /// verified current-run row are both present (AC: the two stop diverging).
    pub fn imported_and_current_both_present(&self) -> bool {
        let imported = self
            .rows
            .iter()
            .any(|r| provenance_is_imported(r.result_provenance) && r.is_narrowed());
        let current = self
            .rows
            .iter()
            .any(|r| r.result_provenance == M5TestIntelligenceProvenanceClass::VerifiedCurrentRun);
        imported && current
    }

    /// Whether every row carries the identical shared state lexicon.
    pub fn shared_lexicon_uniform(&self) -> bool {
        self.rows.iter().all(|r| {
            r.shared_state_lexicon
                .iter()
                .map(String::as_str)
                .eq(SHARED_STATE_LEXICON)
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> IntelConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_class = |c: SharedIntelConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let demonstrated = self.demonstrated_narrow_reasons();

        IntelConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(IntelConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(IntelConsumerRow::preserves_labels),
            all_rows_preserve_required_actions: self
                .rows
                .iter()
                .all(IntelConsumerRow::preserves_required_actions),
            all_rows_provenance_claim_consistent: self
                .rows
                .iter()
                .all(IntelConsumerRow::provenance_claim_consistent),
            all_narrowed_rows_disclose: self.rows.iter().all(IntelConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_row_invariants_hold: self.rows.iter().all(|r| r.invariants.holds()),
            editor_surface_present: has_class(SharedIntelConsumerClass::EditorSurface),
            test_tree_present: has_class(SharedIntelConsumerClass::TestTree),
            review_surface_present: has_class(SharedIntelConsumerClass::ReviewSurface),
            cli_summary_present: has_class(SharedIntelConsumerClass::CliSummary),
            imported_ci_detail_present: has_class(SharedIntelConsumerClass::ImportedCiDetail),
            support_export_present: has_class(SharedIntelConsumerClass::SupportExport),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            all_narrow_reasons_demonstrated: M5IntelClaimNarrowReason::ALL
                .iter()
                .all(|r| demonstrated.contains(r)),
            imported_and_current_both_present: self.imported_and_current_both_present(),
            shared_lexicon_uniform: self.shared_lexicon_uniform(),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<IntelConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_INTEL_CONSUMER_SCHEMA_VERSION {
            violations.push(IntelConsumerViolation::SchemaVersion {
                expected: TEST_INTEL_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_INTEL_CONSUMER_RECORD_KIND {
            violations.push(IntelConsumerViolation::RecordKind {
                expected: TEST_INTEL_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(IntelConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(IntelConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(IntelConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }
            if !row.surface_class_consistent() {
                violations.push(IntelConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }
            if !row.points_to_canonical_family() {
                violations.push(IntelConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }
            if !row.preserves_labels() {
                violations.push(IntelConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }
            if !row.preserves_required_actions() {
                violations.push(IntelConsumerViolation::MissingRequiredActions {
                    id: row.row_id.clone(),
                });
            }
            if !row.provenance_claim_consistent() {
                violations.push(IntelConsumerViolation::ProvenanceClaimDivergent {
                    id: row.row_id.clone(),
                });
            }
            if !row.discloses_narrowing() {
                violations.push(IntelConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }
            if !row.copy_export.is_complete() {
                violations.push(IntelConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
            if !row.invariants.holds() {
                violations.push(IntelConsumerViolation::RowInvariantViolated {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all six claimed consumer classes.
        for class in SharedIntelConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(IntelConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5TestIntelligenceComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(IntelConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes.
        if self.families_reused_across_classes() == 0 {
            violations.push(IntelConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(IntelConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC2: every auto-narrow condition is demonstrated somewhere.
        let demonstrated = self.demonstrated_narrow_reasons();
        for reason in M5IntelClaimNarrowReason::ALL {
            if !demonstrated.contains(&reason) {
                violations.push(IntelConsumerViolation::NarrowReasonNotDemonstrated { reason });
            }
        }

        // AC2: an imported auto-narrowing consumer and a verified current-run
        // consumer both exist so the two stop diverging on shared state meaning.
        if !self.imported_and_current_both_present() {
            violations.push(IntelConsumerViolation::ImportedAndCurrentNotBothPresent);
        }

        // AC1: the shared state lexicon is one truth on every surface.
        if !self.shared_lexicon_uniform() {
            violations.push(IntelConsumerViolation::SharedLexiconDivergent);
        }

        if self.summary != self.computed_summary() {
            violations.push(IntelConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(IntelConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,result_provenance,label_parity,narrow_reasons\n",
        );
        for row in &self.rows {
            let reasons = if row.claim_narrow_reasons.is_empty() {
                "full".to_owned()
            } else {
                row.claim_narrow_reasons
                    .iter()
                    .map(|r| r.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            };
            out.push_str(&format!(
                "{id},{class},{surface},{family},{provenance},{label_parity},{reasons}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                provenance = row.result_provenance.as_str(),
                label_parity = row.label_parity.as_str(),
                reasons = reasons,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Intelligence Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5TestIntelligenceComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str(&format!(
            "- Imported + verified current-run both present: {}\n",
            self.summary.imported_and_current_both_present,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                family_label(row.component_family),
                row.chip_tokens()
            ));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_test_intelligence_component_consumers_export(
) -> Result<IntelConsumerPacket, IntelConsumerArtifactError> {
    let packet: IntelConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-intelligence-component-consumer-proof/support_export.json"
    )))
    .map_err(IntelConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(IntelConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum IntelConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<IntelConsumerViolation>),
}

impl fmt::Display for IntelConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => write!(f, "consumer export parse failed: {error}"),
            Self::Validation(violations) => write!(
                f,
                "consumer export failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl Error for IntelConsumerArtifactError {}

/// Validation failure for M05-1033 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntelConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SurfaceClassMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    MissingRequiredActions {
        id: String,
    },
    ProvenanceClaimDivergent {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    RowInvariantViolated {
        id: String,
    },
    MissingConsumerClass {
        class: SharedIntelConsumerClass,
    },
    MissingFamilyCoverage {
        family: M5TestIntelligenceComponentFamily,
    },
    NoFamilyReusedAcrossClasses,
    MissingLabelFamily {
        family: String,
    },
    NarrowReasonNotDemonstrated {
        reason: M5IntelClaimNarrowReason,
    },
    ImportedAndCurrentNotBothPresent,
    SharedLexiconDivergent,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for IntelConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(f, "row {id} declares a surface that does not belong to its consumer class")
            }
            Self::NotCanonicalFamily { id } => {
                write!(f, "row {id} does not point back to exactly one canonical component family")
            }
            Self::LabelParityBroken { id } => write!(
                f,
                "row {id} renames or drops a canonical provenance-freshness, included-run-scope, \
baseline-identity, raw-or-text-fallback, assumption-boundary, or shared-lexicon label"
            ),
            Self::MissingRequiredActions { id } => write!(
                f,
                "row {id} drops a required rerun / open-logs / open-report / raw-fallback / \
rollback action for its family"
            ),
            Self::ProvenanceClaimDivergent { id } => write!(
                f,
                "row {id} lets an imported, cached, stale, or suspected-flaky result read as a \
verified current run (or vice versa)"
            ),
            Self::NarrowedWithoutDisclosure { id } => {
                write!(f, "row {id} auto-narrows without an auto-narrow banner naming its reasons")
            }
            Self::MissingCopyExportParity { id } => {
                write!(f, "row {id} is missing text / JSON / Markdown copy-export parity")
            }
            Self::RowInvariantViolated { id } => write!(
                f,
                "row {id} violates a hard invariant (hides a shard omission behind a single \
percentage, labels an intermittent failure as confirmed flakiness, bundles generated changes \
into one opaque apply, rewords the shared language, or invents an alternate state label)"
            ),
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(f, "component family {family:?} is not adopted in the packet")
            }
            Self::NoFamilyReusedAcrossClasses => {
                write!(f, "no component family is adopted across two or more consumer classes")
            }
            Self::MissingLabelFamily { family } => {
                write!(f, "controlled label family {family} is not preserved anywhere")
            }
            Self::NarrowReasonNotDemonstrated { reason } => {
                write!(f, "auto-narrow reason {reason:?} is not demonstrated by any consumer")
            }
            Self::ImportedAndCurrentNotBothPresent => write!(
                f,
                "the packet must carry both an imported auto-narrowing consumer and a verified current-run consumer"
            ),
            Self::SharedLexiconDivergent => {
                write!(f, "a consumer diverges from the shared imported / suspected-flaky / generated lexicon")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => write!(f, "export contains raw boundary material"),
        }
    }
}

impl Error for IntelConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "imported"
            | "stale"
            | "flaky"
            | "generated"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests, the bin, and the on-disk support export so all
/// three stay byte-aligned.
pub fn seeded_m5_test_intelligence_component_consumers_packet() -> IntelConsumerPacket {
    IntelConsumerPacket::new(IntelConsumerPacketInput {
        packet_id: "m5-test-intelligence-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: TEST_INTEL_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!(
        "evidence:test-intelligence-component-consumer:{id}"
    )]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn actions(family: M5TestIntelligenceComponentFamily) -> Vec<String> {
    required_actions_for(family)
        .iter()
        .map(|a| (*a).to_owned())
        .collect()
}

fn lexicon() -> Vec<String> {
    SHARED_STATE_LEXICON
        .iter()
        .map(|t| (*t).to_owned())
        .collect()
}

fn banner(id: &str, label: &str, reasons: &[M5IntelClaimNarrowReason]) -> AutoNarrowBanner {
    AutoNarrowBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        reasons: reasons.iter().map(|r| r.as_str().to_owned()).collect(),
        recovery_hints: reasons.iter().map(|r| r.recovery().to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: SharedIntelConsumerSurface,
    component_family: M5TestIntelligenceComponentFamily,
    result_provenance: M5TestIntelligenceProvenanceClass,
    label_families: &[&str],
    export_fields: &[&str],
    narrow_reasons: &[M5IntelClaimNarrowReason],
    banner_label: &str,
) -> IntelConsumerRow {
    let is_narrowed = !narrow_reasons.is_empty();
    let (label_parity, auto_narrow_banner) = if is_narrowed {
        (
            LabelParityState::DisclosedNarrowed,
            Some(banner(
                &format!("banner:{row_id}"),
                banner_label,
                narrow_reasons,
            )),
        )
    } else {
        (LabelParityState::Preserved, None)
    };
    IntelConsumerRow {
        record_kind: TEST_INTEL_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: TEST_INTEL_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_surface.consumer_class(),
        consumer_surface,
        component_family,
        canonical_family_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_packet_refs: vec![family_canonical_packet_ref(component_family).to_owned()],
        references_canonical_not_local_prose: true,
        result_provenance,
        preserved_label_families: labels(label_families),
        shared_state_lexicon: lexicon(),
        preserved_actions: actions(component_family),
        claim_narrow_reasons: narrow_reasons.to_vec(),
        label_parity,
        auto_narrow_banner,
        copy_export: copy_export(export_fields),
        invariants: RowInvariants::honest(),
        source_refs: vec![
            TEST_INTEL_CONSUMER_MATRIX_REF.to_owned(),
            family_canonical_schema_ref(component_family).to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<IntelConsumerRow> {
    use M5IntelClaimNarrowReason::*;
    use M5TestIntelligenceComponentFamily::*;
    use M5TestIntelligenceProvenanceClass::*;
    use SharedIntelConsumerSurface::*;

    vec![
        // --- Editor surface: gutter overlay + inline coverage summary --------
        row(
            "consumer:editor-gutter:coverage-overlay-marker",
            EditorGutterOverlay,
            CoverageOverlayMarker,
            VerifiedCurrentRun,
            &["provenance_and_freshness", "baseline_identity", "included_run_scope"],
            &["marker_id", "overlay_state", "provenance_class", "changed_line_emphasis"],
            &[],
            "",
        ),
        row(
            "consumer:editor-summary:coverage-summary-bar",
            EditorCoverageSummary,
            CoverageSummaryBar,
            VerifiedCurrentRun,
            &["provenance_and_freshness", "included_run_scope", "baseline_identity"],
            &["bar_id", "included_run_scope", "metric_kind", "provenance_class"],
            &[ShardScopeOmitted],
            "Coverage summary omits a shard from the included run set — include the omitted shard, or open the merge sheet for scope",
        ),
        // --- Test tree: flaky + retry ---------------------------------------
        row(
            "consumer:test-tree:flaky-state-badge",
            TestTreePanel,
            FlakyStateBadge,
            SuspectedFlaky,
            &["provenance_and_freshness"],
            &["badge_id", "flaky_classification", "classifier_confidence", "provenance_class"],
            &[FlakinessUnconfirmed],
            "Suspected flaky from a single occurrence — rerun to reproduce before confirming the flaky verdict",
        ),
        row(
            "consumer:test-tree:retry-history-row",
            TestTreePanel,
            RetryHistoryRow,
            VerifiedCurrentRun,
            &["provenance_and_freshness"],
            &["row_id", "attempt_outcome", "retry_scope", "provenance_class"],
            &[],
            "",
        ),
        // --- Review surface: coverage diff + snapshot review ----------------
        row(
            "consumer:review-diff:coverage-summary-bar",
            ReviewCoverageDiff,
            CoverageSummaryBar,
            ImportedCiArtifact,
            &["provenance_and_freshness", "included_run_scope"],
            &["bar_id", "included_run_scope", "metric_kind", "provenance_class"],
            &[EvidenceImported],
            "Imported CI coverage on the diff — not a verified local run; rerun locally to produce a verified current-run result",
        ),
        row(
            "consumer:review-snapshot:snapshot-review-card",
            ReviewSnapshotCard,
            SnapshotReviewCard,
            VerifiedCurrentRun,
            &["provenance_and_freshness", "baseline_identity", "raw_or_text_fallback"],
            &["card_id", "baseline_identity", "diff_state", "raw_or_text_fallback"],
            &[],
            "",
        ),
        // --- CLI summary: import/merge sheet + generated suggestion ---------
        row(
            "consumer:cli-summary:coverage-import-merge-sheet",
            CliQualitySummary,
            CoverageImportMergeSheet,
            ImportedCiArtifact,
            &["provenance_and_freshness", "included_run_scope", "baseline_identity"],
            &["sheet_id", "import_source", "merge_resolution", "included_run_scope", "provenance_class"],
            &[EvidenceImported, ShardScopeOmitted],
            "Imported merge sheet with an omitted shard — not a verified local run; include the omitted shard, or open the merge sheet for scope",
        ),
        row(
            "consumer:cli-summary:test-generation-suggestion-card",
            CliQualitySummary,
            TestGenerationSuggestionCard,
            VerifiedCurrentRun,
            &["provenance_and_freshness", "assumption_boundary"],
            &["card_id", "assumption_classes", "apply_scope", "review_classes"],
            &[GeneratedAssumptionsUnverified],
            "Generated test carries unverified assumptions — open the diff preview and review the assumptions before applying",
        ),
        // --- Imported-CI detail ---------------------------------------------
        row(
            "consumer:imported-ci:coverage-overlay-marker",
            ImportedCiDetailView,
            CoverageOverlayMarker,
            ImportedCiArtifact,
            &["provenance_and_freshness", "baseline_identity", "included_run_scope"],
            &["marker_id", "overlay_state", "provenance_class", "changed_line_emphasis"],
            &[EvidenceImported],
            "Imported CI overlay — not a verified local run; rerun locally to produce a verified current-run result",
        ),
        row(
            "consumer:imported-ci:flaky-state-badge",
            ImportedCiDetailView,
            FlakyStateBadge,
            ReproducedFlaky,
            &["provenance_and_freshness"],
            &["badge_id", "flaky_classification", "classifier_confidence", "provenance_class"],
            &[],
            "",
        ),
        // --- Support / export ------------------------------------------------
        row(
            "consumer:support:snapshot-review-card",
            SupportExportPacket,
            SnapshotReviewCard,
            StalePriorResult,
            &["provenance_and_freshness", "baseline_identity", "raw_or_text_fallback"],
            &["card_id", "baseline_identity", "diff_state", "raw_or_text_fallback"],
            &[ProvenanceStale],
            "Stale snapshot result in the export — rerun to refresh the provenance to the current source",
        ),
        row(
            "consumer:support:test-generation-suggestion-card",
            SupportExportPacket,
            TestGenerationSuggestionCard,
            CachedLocalResult,
            &["provenance_and_freshness", "assumption_boundary"],
            &["card_id", "assumption_classes", "apply_scope", "review_classes"],
            &[ProvenanceStale, GeneratedAssumptionsUnverified],
            "Cached generated suggestion with unverified assumptions — rerun to refresh the provenance, and open the diff preview before applying",
        ),
    ]
}
