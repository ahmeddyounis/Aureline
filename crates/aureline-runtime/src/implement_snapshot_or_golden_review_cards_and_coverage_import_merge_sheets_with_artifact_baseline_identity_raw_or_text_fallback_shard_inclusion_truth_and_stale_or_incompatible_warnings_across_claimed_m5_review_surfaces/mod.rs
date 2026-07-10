//! Two reusable M5 review primitives — the snapshot / golden review card and the
//! coverage-import / merge sheet — so a reviewer sees the baseline and the merge scope *before*
//! trusting a derived quality signal. A snapshot / golden review card always names its artifact
//! kind, the baseline identity it compares against, its diff count, its render / raw fallback
//! mode, its environment / viewport / theme / serializer scope, and its accept / reject / export
//! actions; a coverage-import / merge sheet always names its included and excluded runs, its
//! commit / build identity, its stale-or-incompatible warnings, its line-versus-branch support,
//! and the resulting scope truth.
//!
//! Aureline's frozen test-intelligence component matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! names the snapshot-review card and the coverage-import / merge sheet as two governed component
//! families and freezes their controlled vocabulary — the snapshot baseline identities, the
//! snapshot diff states, the coverage import sources, and the merge-resolution states, plus the
//! provenance classes, surface families, deployment lines, consumer surfaces, accessibility
//! routes, qualification classes, and downgrade triggers. This module *implements* that contract
//! as two reusable resolvers so a user can tell — from the card alone — which baseline a snapshot
//! is being compared against, how many artifacts changed, and whether a raw / text fallback is
//! available, and — from the sheet alone — which runs were included, which were omitted, and
//! whether the merged number can be treated as exact current truth. Above all, a snapshot
//! acceptance never collapses to a blind `Accept all` without artifact count, scope, and fallback
//! visibility, and a merged coverage number is never treated as exact current truth while a shard
//! omission, an incompatible artifact, or a stale report is still unresolved.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_snapshot_review_card`] — takes one card's artifact kind, baseline identity, diff
//!    state, render / raw fallback mode, environment / viewport / theme / serializer scope
//!    dimensions, diff count, provenance class, opaque card identity, and opaque baseline ref, and
//!    produces one [`M5ResolvedSnapshotCard`] carrying the derived review posture (a
//!    matches-baseline, diff-detected, new-snapshot, obsolete-snapshot, render-unavailable, or
//!    raw-text-fallback card — one distinct posture per diff state), whether the scope is
//!    disclosed, whether a raw / text fallback path exists, whether the card is an acceptance
//!    decision, and the bounded reveal / accept-baseline / reject-change / open-raw-fallback /
//!    export actions. It refuses to resolve an acceptance decision that lacks a disclosed scope,
//!    and refuses an opaque / render-unavailable artifact that lacks a raw / text fallback, so a
//!    snapshot acceptance can never present as a blind `Accept all`.
//! 2. [`resolve_coverage_import_merge_sheet`] — takes one sheet's coverage import source,
//!    merge-resolution state, line-versus-branch metric kinds, included and excluded run labels,
//!    commit / build identity, stale and incompatible flags, an exact-current-truth claim, and an
//!    opaque sheet identity, and produces one [`M5ResolvedMergeSheet`] carrying the derived merge
//!    posture (a merged-clean, shard-omission, conflicting-overlap, partial-merge, superseded, or
//!    merge-unavailable sheet — one distinct posture per merge-resolution state), whether omitted
//!    shards are exposed, whether the report is stale or incompatible, whether it discloses its
//!    line-versus-branch metric dimension, whether it is imported, and whether the merged number
//!    may be treated as exact current truth. It refuses to resolve a shard omission that names no
//!    excluded runs, and refuses to treat a merged number as exact current truth while an
//!    omission, an incompatible artifact, or a stale report is still unresolved.
//!
//! A single parity matrix — [`M5SnapshotMergeComponentsPacket`] — binds one row per claimed M5
//! review consumer (the snapshot review panel, the editor snapshot diff, the coverage-import /
//! merge panel, the headless / CLI review surface, and the review export) to the shared card and
//! sheet anatomy, the same snapshot baseline identities, diff states, artifact kinds, fallback
//! modes, scope dimensions, review postures, coverage import sources, merge-resolution states,
//! metric kinds, merge postures, bounded actions, export fields, and non-visual accessibility
//! routes, so the snapshot and merge vocabulary stays identical across the review panel, the
//! editor, the merge panel, CI / headless, and support consumers — the acceptance-criterion
//! parity that keeps baseline and merge review explicit everywhere with one vocabulary.
//!
//! The snapshot baseline identity ([`M5SnapshotBaselineIdentity`]), snapshot diff state
//! ([`M5SnapshotDiffState`]), coverage import source ([`M5CoverageImportSource`]),
//! merge-resolution state ([`M5MergeResolutionState`]), coverage metric kind
//! ([`M5CoverageMetricKind`]), provenance class ([`M5TestIntelligenceProvenanceClass`]), surface
//! family ([`M5TestIntelligenceSurfaceFamily`]), deployment line
//! ([`M5TestIntelligenceDeploymentLine`]), consumer surface ([`M5TestIntelligenceConsumerSurface`]),
//! accessibility route ([`M5TestIntelligenceAccessibilityRoute`]), qualification class
//! ([`M5TestIntelligenceQualificationClass`]), and downgrade trigger
//! ([`M5TestIntelligenceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: their review consumers, the artifact kind, the fallback mode, the scope
//! dimensions, the two derived postures, the two bounded action sets, the two anatomies, and the
//! two export field sets. No M5 review surface invents a second snapshot-card or merge-sheet
//! grammar.
//!
//! Raw snapshot payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every card identity, baseline ref, sheet identity, commit / build identity,
//! and run label is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed,
    seeded_m5_snapshot_merge_components_packet,
    seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed,
    M5_SNAPSHOT_MERGE_COMPONENTS_PACKET_ID,
};

// The snapshot baseline identity, snapshot diff state, coverage import source, merge-resolution
// state, coverage metric kind, provenance class, surface family, deployment line, consumer
// surface, accessibility route, qualification class, and downgrade triggers are frozen once, in
// the test-intelligence component matrix. These primitives reuse them verbatim so they never
// invent parallel snapshot / merge vocabulary.
pub use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5CoverageImportSource, M5CoverageMetricKind, M5MergeResolutionState,
    M5SnapshotBaselineIdentity, M5SnapshotDiffState, M5TestIntelligenceAccessibilityRoute,
    M5TestIntelligenceConsumerSurface, M5TestIntelligenceDeploymentLine,
    M5TestIntelligenceDowngradeTrigger, M5TestIntelligenceProvenanceClass,
    M5TestIntelligenceQualificationClass, M5TestIntelligenceSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SnapshotMergeComponentsPacket`].
pub const M5_SNAPSHOT_MERGE_COMPONENTS_RECORD_KIND: &str =
    "implement_m5_snapshot_or_golden_review_cards_and_coverage_import_merge_sheets_with_artifact_baseline_identity_raw_or_text_fallback_shard_inclusion_truth_and_stale_or_incompatible_warnings_across_claimed_m5_review_surfaces";

/// Schema version for M5 snapshot-review-card / coverage-import-merge-sheet records.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the snapshot-review-card boundary schema (the canonical packet schema).
pub const M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF: &str =
    "schemas/ui/m5-snapshot-review-card.schema.json";

/// Repo-relative path of the coverage-import-merge-sheet companion schema.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF: &str =
    "schemas/ui/m5-coverage-import-merge-sheet.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_DOC_REF: &str =
    "docs/testing/m5_snapshot_coverage_import_primitive.md";

/// Repo-relative path of the frozen test-intelligence component matrix these primitives narrow
/// from.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the snapshot-acceptance-review contract the card binds its artifact /
/// baseline / diff truth against.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_REVIEW_REF: &str =
    "schemas/testing/snapshot_acceptance_review.schema.json";

/// Repo-relative path of the coverage-merge-result contract the sheet binds its included /
/// excluded / scope truth against.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_COVERAGE_MERGE_REF: &str =
    "schemas/testing/coverage_merge_result.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-snapshot-coverage-import-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-snapshot-coverage-import-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_CSV_REF: &str =
    "artifacts/release/m5-snapshot-coverage-import-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_REPORT_REF: &str =
    "artifacts/design/m5-snapshot-coverage-import-primitive.md";

/// One claimed M5 review consumer that renders the shared snapshot-review card and
/// coverage-import / merge sheet. These are the consumers the acceptance criteria name — the
/// snapshot review panel, the editor snapshot diff, the coverage-import / merge panel, the
/// headless / CLI review surface, and the review export — so the same snapshot / merge grammar
/// works across every claimed review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotMergeComponentConsumerSurface {
    /// The snapshot / golden review panel surface.
    SnapshotReviewPanel,
    /// The editor snapshot-diff surface.
    EditorSnapshotDiff,
    /// The coverage-import / merge panel surface.
    CoverageImportMergePanel,
    /// The headless / CLI review surface.
    HeadlessCliReview,
    /// The review export surface.
    ReviewExport,
}

impl M5SnapshotMergeComponentConsumerSurface {
    /// Every claimed review consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SnapshotReviewPanel,
        Self::EditorSnapshotDiff,
        Self::CoverageImportMergePanel,
        Self::HeadlessCliReview,
        Self::ReviewExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotReviewPanel => "snapshot_review_panel",
            Self::EditorSnapshotDiff => "editor_snapshot_diff",
            Self::CoverageImportMergePanel => "coverage_import_merge_panel",
            Self::HeadlessCliReview => "headless_cli_review",
            Self::ReviewExport => "review_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SnapshotReviewPanel => "Snapshot Review Panel",
            Self::EditorSnapshotDiff => "Editor Snapshot Diff",
            Self::CoverageImportMergePanel => "Coverage Import / Merge Panel",
            Self::HeadlessCliReview => "Headless / CLI Review",
            Self::ReviewExport => "Review Export",
        }
    }
}

/// Controlled snapshot artifact kind — what kind of artifact a snapshot / golden review card
/// compares, so an opaque binary artifact never presents as a readable rendered diff without a
/// raw / text fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotArtifactKind {
    /// An image snapshot.
    ImageSnapshot,
    /// A DOM / HTML snapshot.
    DomSnapshot,
    /// A text-serializer snapshot.
    TextSerializerSnapshot,
    /// A JSON snapshot.
    JsonSnapshot,
    /// An inline snapshot.
    InlineSnapshot,
    /// An opaque binary snapshot.
    BinarySnapshot,
}

impl M5SnapshotArtifactKind {
    /// Every artifact kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ImageSnapshot,
        Self::DomSnapshot,
        Self::TextSerializerSnapshot,
        Self::JsonSnapshot,
        Self::InlineSnapshot,
        Self::BinarySnapshot,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImageSnapshot => "image_snapshot",
            Self::DomSnapshot => "dom_snapshot",
            Self::TextSerializerSnapshot => "text_serializer_snapshot",
            Self::JsonSnapshot => "json_snapshot",
            Self::InlineSnapshot => "inline_snapshot",
            Self::BinarySnapshot => "binary_snapshot",
        }
    }

    /// True when the artifact is an opaque binary that cannot be diffed inline without a raw /
    /// text fallback.
    pub const fn is_opaque_binary(self) -> bool {
        matches!(self, Self::BinarySnapshot)
    }
}

/// Controlled render / raw fallback mode a snapshot card shows, so a rendered diff never hides
/// whether a raw / text fallback path exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotFallbackMode {
    /// Only the rendered diff is shown.
    RenderedDiff,
    /// A raw / text fallback is shown alongside the rendered diff.
    SideBySide,
    /// A raw / text fallback is shown because the rendered diff is unavailable.
    RawTextFallback,
    /// Only the raw / text view is available.
    RawTextOnly,
}

impl M5SnapshotFallbackMode {
    /// Every fallback mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RenderedDiff,
        Self::SideBySide,
        Self::RawTextFallback,
        Self::RawTextOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderedDiff => "rendered_diff",
            Self::SideBySide => "side_by_side",
            Self::RawTextFallback => "raw_text_fallback",
            Self::RawTextOnly => "raw_text_only",
        }
    }

    /// True when a raw / text fallback path is available (anything but a rendered-only diff).
    pub const fn has_raw_path(self) -> bool {
        !matches!(self, Self::RenderedDiff)
    }
}

/// Controlled snapshot scope dimension — the environment / viewport / theme / serializer / locale
/// scope a snapshot was captured under, so a snapshot from one viewport never silently reads as
/// the same result under another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotScopeDimension {
    /// The environment scope.
    Environment,
    /// The viewport scope.
    Viewport,
    /// The theme scope.
    Theme,
    /// The serializer scope.
    Serializer,
    /// The locale scope.
    Locale,
}

impl M5SnapshotScopeDimension {
    /// Every scope dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Environment,
        Self::Viewport,
        Self::Theme,
        Self::Serializer,
        Self::Locale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Environment => "environment",
            Self::Viewport => "viewport",
            Self::Theme => "theme",
            Self::Serializer => "serializer",
            Self::Locale => "locale",
        }
    }
}

/// The derived posture of a snapshot / golden review card — one distinct posture per snapshot
/// diff state so a new snapshot never reads as a matched baseline. Computed 1:1 from the snapshot
/// diff state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotReviewPosture {
    /// A matches-baseline card.
    MatchesBaselineCard,
    /// A diff-detected card.
    DiffDetectedCard,
    /// A new-snapshot card.
    NewSnapshotCard,
    /// An obsolete-snapshot card.
    ObsoleteSnapshotCard,
    /// A render-unavailable card.
    RenderUnavailableCard,
    /// A raw-text-fallback card.
    RawTextFallbackCard,
}

impl M5SnapshotReviewPosture {
    /// Every snapshot posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MatchesBaselineCard,
        Self::DiffDetectedCard,
        Self::NewSnapshotCard,
        Self::ObsoleteSnapshotCard,
        Self::RenderUnavailableCard,
        Self::RawTextFallbackCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatchesBaselineCard => "matches_baseline_card",
            Self::DiffDetectedCard => "diff_detected_card",
            Self::NewSnapshotCard => "new_snapshot_card",
            Self::ObsoleteSnapshotCard => "obsolete_snapshot_card",
            Self::RenderUnavailableCard => "render_unavailable_card",
            Self::RawTextFallbackCard => "raw_text_fallback_card",
        }
    }

    /// The frozen snapshot diff state this posture maps 1:1 to.
    pub const fn diff_state(self) -> M5SnapshotDiffState {
        match self {
            Self::MatchesBaselineCard => M5SnapshotDiffState::MatchesBaseline,
            Self::DiffDetectedCard => M5SnapshotDiffState::DiffDetected,
            Self::NewSnapshotCard => M5SnapshotDiffState::NewSnapshot,
            Self::ObsoleteSnapshotCard => M5SnapshotDiffState::ObsoleteSnapshot,
            Self::RenderUnavailableCard => M5SnapshotDiffState::RenderUnavailable,
            Self::RawTextFallbackCard => M5SnapshotDiffState::RawTextFallback,
        }
    }

    /// True when the card asks the reviewer to accept or reject a baseline change (a detected diff
    /// or a brand-new snapshot).
    pub const fn is_acceptance_decision(self) -> bool {
        matches!(self, Self::DiffDetectedCard | Self::NewSnapshotCard)
    }

    /// True when the card flags a state a reviewer should act on before trusting it.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::DiffDetectedCard
                | Self::NewSnapshotCard
                | Self::ObsoleteSnapshotCard
                | Self::RenderUnavailableCard
        )
    }
}

/// One bounded action a snapshot / golden review card offers, so a card never hides its reveal /
/// accept-baseline / reject-change / open-raw-fallback / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotCardAction {
    /// Reveal the card's artifact kind, baseline identity, diff count, fallback mode, and scope.
    RevealSnapshotDetails,
    /// Accept the new snapshot as the baseline.
    AcceptBaseline,
    /// Reject the change and keep the current baseline.
    RejectChange,
    /// Open the raw / text fallback view.
    OpenRawFallback,
    /// Export the snapshot review as test evidence.
    ExportSnapshotReview,
}

impl M5SnapshotCardAction {
    /// Every snapshot action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealSnapshotDetails,
        Self::AcceptBaseline,
        Self::RejectChange,
        Self::OpenRawFallback,
        Self::ExportSnapshotReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealSnapshotDetails => "reveal_snapshot_details",
            Self::AcceptBaseline => "accept_baseline",
            Self::RejectChange => "reject_change",
            Self::OpenRawFallback => "open_raw_fallback",
            Self::ExportSnapshotReview => "export_snapshot_review",
        }
    }
}

/// Controlled snapshot-card anatomy part. The parts in [`M5SnapshotCardAnatomyPart::MANDATORY`]
/// are required on every card so the artifact kind, baseline identity, diff count, fallback mode,
/// scope, and accept / reject affordance are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotCardAnatomyPart {
    /// The artifact-kind cue.
    ArtifactKindCue,
    /// The baseline-identity cue.
    BaselineIdentityCue,
    /// The diff-count cue.
    DiffCountCue,
    /// The fallback-mode cue.
    FallbackModeCue,
    /// The scope cue.
    ScopeCue,
    /// The accept / reject affordance cue.
    AcceptRejectCue,
    /// The provenance cue.
    ProvenanceCue,
    /// The review-state cue.
    ReviewStateCue,
}

impl M5SnapshotCardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ArtifactKindCue,
        Self::BaselineIdentityCue,
        Self::DiffCountCue,
        Self::FallbackModeCue,
        Self::ScopeCue,
        Self::AcceptRejectCue,
        Self::ProvenanceCue,
        Self::ReviewStateCue,
    ];

    /// The anatomy parts every snapshot card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::ArtifactKindCue,
        Self::BaselineIdentityCue,
        Self::DiffCountCue,
        Self::FallbackModeCue,
        Self::ScopeCue,
        Self::AcceptRejectCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactKindCue => "artifact_kind_cue",
            Self::BaselineIdentityCue => "baseline_identity_cue",
            Self::DiffCountCue => "diff_count_cue",
            Self::FallbackModeCue => "fallback_mode_cue",
            Self::ScopeCue => "scope_cue",
            Self::AcceptRejectCue => "accept_reject_cue",
            Self::ProvenanceCue => "provenance_cue",
            Self::ReviewStateCue => "review_state_cue",
        }
    }
}

/// A field the snapshot-card export carries so card truth is reconstructable. The fields in
/// [`M5SnapshotCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotCardExportField {
    /// The artifact kind.
    ArtifactKind,
    /// The baseline identity.
    BaselineIdentity,
    /// The diff state.
    DiffState,
    /// The diff count.
    DiffCount,
    /// The fallback mode.
    FallbackMode,
    /// The scope dimensions.
    ScopeDimensions,
    /// The provenance class.
    ProvenanceClass,
    /// The derived review posture.
    ReviewPosture,
    /// The available actions.
    AvailableActions,
}

impl M5SnapshotCardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ArtifactKind,
        Self::BaselineIdentity,
        Self::DiffState,
        Self::DiffCount,
        Self::FallbackMode,
        Self::ScopeDimensions,
        Self::ProvenanceClass,
        Self::ReviewPosture,
        Self::AvailableActions,
    ];

    /// The export fields every snapshot card must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::ArtifactKind,
        Self::BaselineIdentity,
        Self::DiffCount,
        Self::FallbackMode,
        Self::ScopeDimensions,
        Self::ReviewPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactKind => "artifact_kind",
            Self::BaselineIdentity => "baseline_identity",
            Self::DiffState => "diff_state",
            Self::DiffCount => "diff_count",
            Self::FallbackMode => "fallback_mode",
            Self::ScopeDimensions => "scope_dimensions",
            Self::ProvenanceClass => "provenance_class",
            Self::ReviewPosture => "review_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// The derived posture of a coverage-import / merge sheet — one distinct posture per
/// merge-resolution state, so a shard omission never reads as a clean merge. Computed 1:1 from the
/// merge-resolution state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageMergePosture {
    /// A merged-clean sheet.
    MergedCleanSheet,
    /// A shard-omission sheet.
    ShardOmissionSheet,
    /// A conflicting-overlap sheet.
    ConflictingOverlapSheet,
    /// A partial-merge sheet.
    PartialMergeSheet,
    /// A superseded-by-newer sheet.
    SupersededSheet,
    /// A merge-unavailable sheet.
    MergeUnavailableSheet,
}

impl M5CoverageMergePosture {
    /// Every merge posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MergedCleanSheet,
        Self::ShardOmissionSheet,
        Self::ConflictingOverlapSheet,
        Self::PartialMergeSheet,
        Self::SupersededSheet,
        Self::MergeUnavailableSheet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MergedCleanSheet => "merged_clean_sheet",
            Self::ShardOmissionSheet => "shard_omission_sheet",
            Self::ConflictingOverlapSheet => "conflicting_overlap_sheet",
            Self::PartialMergeSheet => "partial_merge_sheet",
            Self::SupersededSheet => "superseded_sheet",
            Self::MergeUnavailableSheet => "merge_unavailable_sheet",
        }
    }

    /// The frozen merge-resolution state this posture maps 1:1 to.
    pub const fn merge_resolution(self) -> M5MergeResolutionState {
        match self {
            Self::MergedCleanSheet => M5MergeResolutionState::MergedClean,
            Self::ShardOmissionSheet => M5MergeResolutionState::ShardOmissionDetected,
            Self::ConflictingOverlapSheet => M5MergeResolutionState::ConflictingOverlap,
            Self::PartialMergeSheet => M5MergeResolutionState::PartialMerge,
            Self::SupersededSheet => M5MergeResolutionState::SupersededByNewer,
            Self::MergeUnavailableSheet => M5MergeResolutionState::MergeUnavailable,
        }
    }

    /// True when the posture itself signals that some runs / shards were omitted from the merge.
    pub const fn exposes_omission(self) -> bool {
        matches!(self, Self::ShardOmissionSheet | Self::PartialMergeSheet)
    }

    /// True when the merged number cannot yet stand as exact current truth (anything but a clean
    /// merge).
    pub const fn is_incomplete(self) -> bool {
        !matches!(self, Self::MergedCleanSheet)
    }

    /// True when the sheet flags a state a reviewer should act on before trusting it.
    pub const fn needs_attention(self) -> bool {
        self.is_incomplete()
    }
}

/// One bounded action a coverage-import / merge sheet offers, so a sheet never hides its reveal /
/// review-run-scope / open-incompatible / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MergeSheetAction {
    /// Reveal the sheet's included / excluded runs, commit / build identity, warnings, metric
    /// dimension, and resulting scope.
    RevealMergeDetails,
    /// Review the included and excluded run scope.
    ReviewRunScope,
    /// Open the incompatible / stale / omitted-shard report.
    OpenIncompatibleReport,
    /// Export the merge sheet as coverage evidence.
    ExportMergeSheet,
}

impl M5MergeSheetAction {
    /// Every merge action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealMergeDetails,
        Self::ReviewRunScope,
        Self::OpenIncompatibleReport,
        Self::ExportMergeSheet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealMergeDetails => "reveal_merge_details",
            Self::ReviewRunScope => "review_run_scope",
            Self::OpenIncompatibleReport => "open_incompatible_report",
            Self::ExportMergeSheet => "export_merge_sheet",
        }
    }
}

/// Controlled merge-sheet anatomy part. The parts in [`M5MergeSheetAnatomyPart::MANDATORY`] are
/// required on every sheet so the included / excluded runs, the commit / build identity, the
/// stale-or-incompatible warning, the line-versus-branch support, and the resulting scope are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MergeSheetAnatomyPart {
    /// The included / excluded runs cue.
    IncludedExcludedRunsCue,
    /// The commit / build identity cue.
    CommitBuildIdentityCue,
    /// The stale-or-incompatible warning cue.
    StaleIncompatibleWarningCue,
    /// The line-versus-branch support cue.
    LineVersusBranchCue,
    /// The resulting-scope cue.
    ResultingScopeCue,
    /// The import-source cue.
    ImportSourceCue,
}

impl M5MergeSheetAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncludedExcludedRunsCue,
        Self::CommitBuildIdentityCue,
        Self::StaleIncompatibleWarningCue,
        Self::LineVersusBranchCue,
        Self::ResultingScopeCue,
        Self::ImportSourceCue,
    ];

    /// The anatomy parts every merge sheet must render.
    pub const MANDATORY: [Self; 5] = [
        Self::IncludedExcludedRunsCue,
        Self::CommitBuildIdentityCue,
        Self::StaleIncompatibleWarningCue,
        Self::LineVersusBranchCue,
        Self::ResultingScopeCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedExcludedRunsCue => "included_excluded_runs_cue",
            Self::CommitBuildIdentityCue => "commit_build_identity_cue",
            Self::StaleIncompatibleWarningCue => "stale_incompatible_warning_cue",
            Self::LineVersusBranchCue => "line_versus_branch_cue",
            Self::ResultingScopeCue => "resulting_scope_cue",
            Self::ImportSourceCue => "import_source_cue",
        }
    }
}

/// A field the merge-sheet export carries so sheet truth is reconstructable. The fields in
/// [`M5MergeSheetExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MergeSheetExportField {
    /// The coverage import source.
    ImportSource,
    /// The merge-resolution state.
    MergeResolution,
    /// The line-versus-branch metric kinds.
    MetricKinds,
    /// The included runs.
    IncludedRuns,
    /// The excluded runs.
    ExcludedRuns,
    /// The derived merge posture.
    MergePosture,
}

impl M5MergeSheetExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ImportSource,
        Self::MergeResolution,
        Self::MetricKinds,
        Self::IncludedRuns,
        Self::ExcludedRuns,
        Self::MergePosture,
    ];

    /// The export fields every merge sheet must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ImportSource,
        Self::MergeResolution,
        Self::MetricKinds,
        Self::IncludedRuns,
        Self::MergePosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportSource => "import_source",
            Self::MergeResolution => "merge_resolution",
            Self::MetricKinds => "metric_kinds",
            Self::IncludedRuns => "included_runs",
            Self::ExcludedRuns => "excluded_runs",
            Self::MergePosture => "merge_posture",
        }
    }
}

/// True when a provenance class marks the report / snapshot as imported rather than a live local
/// run.
pub const fn provenance_is_imported(provenance: M5TestIntelligenceProvenanceClass) -> bool {
    matches!(
        provenance,
        M5TestIntelligenceProvenanceClass::ImportedCiArtifact
    )
}

/// True when a coverage import source is not a live local run.
pub const fn import_source_is_imported(source: M5CoverageImportSource) -> bool {
    matches!(
        source,
        M5CoverageImportSource::ImportedCiArtifact | M5CoverageImportSource::UploadedReport
    )
}

// ---- snapshot-review-card resolver ---------------------------------------

/// The full input to the snapshot-review-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotCardResolutionInput {
    /// The artifact kind the snapshot compares.
    pub artifact_kind: M5SnapshotArtifactKind,
    /// The baseline identity the snapshot is compared against.
    pub baseline_identity: M5SnapshotBaselineIdentity,
    /// The snapshot diff state.
    pub diff_state: M5SnapshotDiffState,
    /// The render / raw fallback mode.
    pub fallback_mode: M5SnapshotFallbackMode,
    /// The environment / viewport / theme / serializer / locale scope dimensions.
    pub scope_dimensions: Vec<M5SnapshotScopeDimension>,
    /// The number of changed artifacts in this review.
    pub diff_count: u32,
    /// The provenance class behind the snapshot.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The opaque stable card identity (must be non-empty).
    pub card_identity_ref: String,
    /// The opaque durable ref to the baseline (must be non-empty).
    pub baseline_ref: String,
}

/// The resolved snapshot-review-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSnapshotCard {
    /// The artifact kind.
    pub artifact_kind: M5SnapshotArtifactKind,
    /// The baseline identity.
    pub baseline_identity: M5SnapshotBaselineIdentity,
    /// The snapshot diff state.
    pub diff_state: M5SnapshotDiffState,
    /// The fallback mode.
    pub fallback_mode: M5SnapshotFallbackMode,
    /// The scope dimensions, preserved exactly from the input.
    pub scope_dimensions: Vec<M5SnapshotScopeDimension>,
    /// The diff count, preserved from the input.
    pub diff_count: u32,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The opaque card identity, preserved exactly from the input.
    pub card_identity_ref: String,
    /// The opaque baseline ref, preserved exactly from the input.
    pub baseline_ref: String,
    /// The derived review posture.
    pub review_posture: M5SnapshotReviewPosture,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5SnapshotCardAction>,
    /// True when the card discloses at least one scope dimension.
    pub has_scope_disclosed: bool,
    /// True when a raw / text fallback path is available.
    pub has_raw_fallback: bool,
    /// True when the card asks the reviewer to accept or reject a baseline change.
    pub is_acceptance_decision: bool,
    /// True when an acceptance decision discloses its scope (always true after resolution — a
    /// scopeless acceptance fails resolution).
    pub acceptance_is_scoped: bool,
    /// True when the artifact is an opaque binary.
    pub is_opaque_binary: bool,
    /// True when the card always carries its render / raw fallback visibility (const true — the
    /// fallback mode is a mandatory anatomy and export field).
    pub preserves_fallback_visibility: bool,
    /// True when the card flags a state a reviewer should act on before trusting it.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_snapshot_review_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SnapshotCardResolutionError {
    /// The card identity ref was empty.
    EmptyCardIdentity,
    /// The baseline ref was empty.
    EmptyBaselineReference,
    /// An acceptance decision was resolved without a disclosed scope — a snapshot acceptance would
    /// collapse to a blind `Accept all`.
    BlindAcceptanceWithoutScope,
    /// An opaque binary or render-unavailable card lacked a raw / text fallback.
    RawFallbackMissingForOpaqueArtifact,
    /// A card descriptor carried forbidden material.
    ForbiddenSnapshotMaterial,
}

impl M5SnapshotCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCardIdentity => "empty_card_identity",
            Self::EmptyBaselineReference => "empty_baseline_reference",
            Self::BlindAcceptanceWithoutScope => "blind_acceptance_without_scope",
            Self::RawFallbackMissingForOpaqueArtifact => "raw_fallback_missing_for_opaque_artifact",
            Self::ForbiddenSnapshotMaterial => "forbidden_snapshot_material",
        }
    }
}

impl fmt::Display for M5SnapshotCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "snapshot review card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SnapshotCardResolutionError {}

/// Resolves one snapshot / golden review card from its declared review state.
///
/// The derived review posture is 1:1 with the snapshot diff state — matches-baseline,
/// diff-detected, new-snapshot, obsolete-snapshot, render-unavailable, or raw-text-fallback — so a
/// new snapshot never reads as a matched baseline. An acceptance decision (a detected diff or a
/// new snapshot) is only accepted when it discloses at least one scope dimension; otherwise
/// resolution fails, so a snapshot acceptance can never collapse to a blind `Accept all` without
/// its artifact count, scope, and fallback visibility. An opaque binary artifact or a
/// render-unavailable card is only accepted when a raw / text fallback path exists; otherwise
/// resolution fails, so a binary-only change always keeps a raw / text fallback. The artifact
/// kind, baseline identity, diff count, fallback mode, and scope are always carried, so none of
/// the evidence behind a review is hidden.
pub fn resolve_snapshot_review_card(
    input: &M5SnapshotCardResolutionInput,
) -> Result<M5ResolvedSnapshotCard, M5SnapshotCardResolutionError> {
    if input.card_identity_ref.trim().is_empty() {
        return Err(M5SnapshotCardResolutionError::EmptyCardIdentity);
    }
    if input.baseline_ref.trim().is_empty() {
        return Err(M5SnapshotCardResolutionError::EmptyBaselineReference);
    }
    if value_repr_is_forbidden(&input.card_identity_ref)
        || value_repr_is_forbidden(&input.baseline_ref)
    {
        return Err(M5SnapshotCardResolutionError::ForbiddenSnapshotMaterial);
    }

    let review_posture = derive_snapshot_posture(input.diff_state);
    let has_scope_disclosed = !input.scope_dimensions.is_empty();
    let has_raw_fallback = input.fallback_mode.has_raw_path();
    let is_acceptance_decision = review_posture.is_acceptance_decision();

    if is_acceptance_decision && !has_scope_disclosed {
        return Err(M5SnapshotCardResolutionError::BlindAcceptanceWithoutScope);
    }
    let requires_raw_fallback = input.artifact_kind.is_opaque_binary()
        || matches!(input.diff_state, M5SnapshotDiffState::RenderUnavailable);
    if requires_raw_fallback && !has_raw_fallback {
        return Err(M5SnapshotCardResolutionError::RawFallbackMissingForOpaqueArtifact);
    }

    let available_actions = derive_snapshot_actions(is_acceptance_decision);

    Ok(M5ResolvedSnapshotCard {
        artifact_kind: input.artifact_kind,
        baseline_identity: input.baseline_identity,
        diff_state: input.diff_state,
        fallback_mode: input.fallback_mode,
        scope_dimensions: input.scope_dimensions.clone(),
        diff_count: input.diff_count,
        provenance_class: input.provenance_class,
        card_identity_ref: input.card_identity_ref.clone(),
        baseline_ref: input.baseline_ref.clone(),
        review_posture,
        available_actions,
        has_scope_disclosed,
        has_raw_fallback,
        is_acceptance_decision,
        acceptance_is_scoped: !is_acceptance_decision || has_scope_disclosed,
        is_opaque_binary: input.artifact_kind.is_opaque_binary(),
        preserves_fallback_visibility: true,
        needs_attention: review_posture.needs_attention(),
    })
}

/// The 1:1 snapshot-diff-state → snapshot-posture map.
fn derive_snapshot_posture(diff_state: M5SnapshotDiffState) -> M5SnapshotReviewPosture {
    match diff_state {
        M5SnapshotDiffState::MatchesBaseline => M5SnapshotReviewPosture::MatchesBaselineCard,
        M5SnapshotDiffState::DiffDetected => M5SnapshotReviewPosture::DiffDetectedCard,
        M5SnapshotDiffState::NewSnapshot => M5SnapshotReviewPosture::NewSnapshotCard,
        M5SnapshotDiffState::ObsoleteSnapshot => M5SnapshotReviewPosture::ObsoleteSnapshotCard,
        M5SnapshotDiffState::RenderUnavailable => M5SnapshotReviewPosture::RenderUnavailableCard,
        M5SnapshotDiffState::RawTextFallback => M5SnapshotReviewPosture::RawTextFallbackCard,
    }
}

/// Derives the bounded snapshot-action set. Accept / reject are offered only on an acceptance
/// decision; reveal, open-raw-fallback, and export are always offered.
fn derive_snapshot_actions(is_acceptance_decision: bool) -> Vec<M5SnapshotCardAction> {
    use M5SnapshotCardAction as Action;
    let mut actions = vec![Action::RevealSnapshotDetails];
    if is_acceptance_decision {
        actions.push(Action::AcceptBaseline);
        actions.push(Action::RejectChange);
    }
    actions.push(Action::OpenRawFallback);
    actions.push(Action::ExportSnapshotReview);
    actions
}

// ---- coverage-import-merge-sheet resolver --------------------------------

/// The full input to the coverage-import-merge-sheet resolver for one sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MergeSheetResolutionInput {
    /// The coverage import source.
    pub import_source: M5CoverageImportSource,
    /// The merge-resolution state.
    pub merge_resolution: M5MergeResolutionState,
    /// The line-versus-branch metric kinds this sheet supports (must be non-empty).
    pub metric_kinds: Vec<M5CoverageMetricKind>,
    /// The included run labels (opaque; must be non-empty).
    pub included_runs: Vec<String>,
    /// The excluded / omitted run labels (opaque).
    pub excluded_runs: Vec<String>,
    /// The provenance class behind the sheet.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether the merged report is stale.
    pub is_stale: bool,
    /// Whether any merged artifact is incompatible.
    pub is_incompatible: bool,
    /// Whether the caller claims the merged number is exact current truth.
    pub claims_exact_current_truth: bool,
    /// The opaque commit identity ref (must be non-empty).
    pub commit_identity_ref: String,
    /// The opaque build identity ref (must be non-empty).
    pub build_identity_ref: String,
    /// The opaque stable sheet identity (must be non-empty).
    pub sheet_identity_ref: String,
}

/// The resolved coverage-import-merge-sheet truth for one sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMergeSheet {
    /// The coverage import source.
    pub import_source: M5CoverageImportSource,
    /// The merge-resolution state.
    pub merge_resolution: M5MergeResolutionState,
    /// The metric kinds, preserved exactly from the input.
    pub metric_kinds: Vec<M5CoverageMetricKind>,
    /// The included runs, preserved exactly from the input.
    pub included_runs: Vec<String>,
    /// The excluded runs, preserved exactly from the input.
    pub excluded_runs: Vec<String>,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether the merged report is stale, preserved from the input.
    pub is_stale: bool,
    /// Whether any merged artifact is incompatible, preserved from the input.
    pub is_incompatible: bool,
    /// The opaque commit identity ref, preserved exactly from the input.
    pub commit_identity_ref: String,
    /// The opaque build identity ref, preserved exactly from the input.
    pub build_identity_ref: String,
    /// The opaque sheet identity, preserved exactly from the input.
    pub sheet_identity_ref: String,
    /// The derived merge posture.
    pub merge_posture: M5CoverageMergePosture,
    /// The bounded actions this sheet offers.
    pub available_actions: Vec<M5MergeSheetAction>,
    /// True when the sheet exposes omitted shards / runs.
    pub exposes_omitted_shards: bool,
    /// True when the sheet discloses a stale-or-incompatible warning.
    pub discloses_stale_or_incompatible: bool,
    /// True when the sheet discloses its line-versus-branch metric dimension (always true — a
    /// scopeless sheet fails resolution).
    pub discloses_metric_dimension: bool,
    /// True when the report is imported rather than a live local run.
    pub is_imported: bool,
    /// True when the merged number may be treated as exact current truth.
    pub is_exact_current_truth: bool,
    /// True when an exact-current-truth claim is backed by a clean, fully disclosed merge (always
    /// true after resolution — an unresolved exact-truth claim fails resolution).
    pub exact_truth_is_qualified: bool,
    /// True when the sheet always discloses its resulting scope (const true — the resulting scope
    /// is a mandatory anatomy part).
    pub discloses_resulting_scope: bool,
    /// True when the sheet flags a state a reviewer should act on before trusting it.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_coverage_import_merge_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MergeSheetResolutionError {
    /// The sheet identity ref was empty.
    EmptySheetIdentity,
    /// The commit or build identity ref was empty.
    EmptyCommitOrBuildIdentity,
    /// The included run scope was empty — the sheet would lose the runs behind the merged number.
    EmptyRunScope,
    /// The metric-kind support was empty — the sheet would drop its line-versus-branch dimension.
    EmptyMetricSupport,
    /// A shard-omission / partial-merge sheet named no excluded runs — the omitted shards would be
    /// hidden behind the merged number.
    OmittedShardsWithoutDisclosure,
    /// A merged number was claimed as exact current truth while an omission, an incompatible
    /// artifact, or a stale report was still unresolved.
    ExactTruthWithUnresolvedWarnings,
    /// A sheet descriptor carried forbidden material.
    ForbiddenMergeMaterial,
}

impl M5MergeSheetResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySheetIdentity => "empty_sheet_identity",
            Self::EmptyCommitOrBuildIdentity => "empty_commit_or_build_identity",
            Self::EmptyRunScope => "empty_run_scope",
            Self::EmptyMetricSupport => "empty_metric_support",
            Self::OmittedShardsWithoutDisclosure => "omitted_shards_without_disclosure",
            Self::ExactTruthWithUnresolvedWarnings => "exact_truth_with_unresolved_warnings",
            Self::ForbiddenMergeMaterial => "forbidden_merge_material",
        }
    }
}

impl fmt::Display for M5MergeSheetResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "coverage import merge sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MergeSheetResolutionError {}

/// Resolves one coverage-import / merge sheet from its declared import and merge state.
///
/// The derived merge posture is 1:1 with the merge-resolution state — merged-clean, shard-omission,
/// conflicting-overlap, partial-merge, superseded, or merge-unavailable — so a shard omission never
/// reads as a clean merge. A shard-omission or partial-merge sheet must name at least one excluded
/// run; otherwise resolution fails, so omitted shards are never hidden behind the merged number. A
/// merged number is only treated as exact current truth when the merge is clean and no omission,
/// incompatible artifact, or stale report is unresolved; otherwise resolution fails, so a merged
/// coverage number is never treated as exact current truth before the omissions and incompatible
/// artifacts are exposed. The included and excluded runs, the commit / build identity, the
/// stale-or-incompatible warnings, and the line-versus-branch support are always carried.
pub fn resolve_coverage_import_merge_sheet(
    input: &M5MergeSheetResolutionInput,
) -> Result<M5ResolvedMergeSheet, M5MergeSheetResolutionError> {
    if input.sheet_identity_ref.trim().is_empty() {
        return Err(M5MergeSheetResolutionError::EmptySheetIdentity);
    }
    if input.commit_identity_ref.trim().is_empty() || input.build_identity_ref.trim().is_empty() {
        return Err(M5MergeSheetResolutionError::EmptyCommitOrBuildIdentity);
    }
    if input.included_runs.is_empty() {
        return Err(M5MergeSheetResolutionError::EmptyRunScope);
    }
    if input.metric_kinds.is_empty() {
        return Err(M5MergeSheetResolutionError::EmptyMetricSupport);
    }
    if value_repr_is_forbidden(&input.sheet_identity_ref)
        || value_repr_is_forbidden(&input.commit_identity_ref)
        || value_repr_is_forbidden(&input.build_identity_ref)
        || input
            .included_runs
            .iter()
            .any(|run| value_repr_is_forbidden(run))
        || input
            .excluded_runs
            .iter()
            .any(|run| value_repr_is_forbidden(run))
    {
        return Err(M5MergeSheetResolutionError::ForbiddenMergeMaterial);
    }

    let merge_posture = derive_merge_posture(input.merge_resolution);
    let exposes_omitted_shards =
        merge_posture.exposes_omission() || !input.excluded_runs.is_empty();
    if merge_posture.exposes_omission() && input.excluded_runs.is_empty() {
        return Err(M5MergeSheetResolutionError::OmittedShardsWithoutDisclosure);
    }

    let discloses_stale_or_incompatible = input.is_stale || input.is_incompatible;
    let has_unresolved_warning =
        exposes_omitted_shards || discloses_stale_or_incompatible || merge_posture.is_incomplete();
    if input.claims_exact_current_truth && has_unresolved_warning {
        return Err(M5MergeSheetResolutionError::ExactTruthWithUnresolvedWarnings);
    }

    let is_imported = import_source_is_imported(input.import_source)
        || provenance_is_imported(input.provenance_class);
    let available_actions = derive_merge_actions(has_unresolved_warning);

    Ok(M5ResolvedMergeSheet {
        import_source: input.import_source,
        merge_resolution: input.merge_resolution,
        metric_kinds: input.metric_kinds.clone(),
        included_runs: input.included_runs.clone(),
        excluded_runs: input.excluded_runs.clone(),
        provenance_class: input.provenance_class,
        is_stale: input.is_stale,
        is_incompatible: input.is_incompatible,
        commit_identity_ref: input.commit_identity_ref.clone(),
        build_identity_ref: input.build_identity_ref.clone(),
        sheet_identity_ref: input.sheet_identity_ref.clone(),
        merge_posture,
        available_actions,
        exposes_omitted_shards,
        discloses_stale_or_incompatible,
        discloses_metric_dimension: !input.metric_kinds.is_empty(),
        is_imported,
        is_exact_current_truth: input.claims_exact_current_truth,
        exact_truth_is_qualified: !input.claims_exact_current_truth || !has_unresolved_warning,
        discloses_resulting_scope: true,
        needs_attention: merge_posture.needs_attention() || discloses_stale_or_incompatible,
    })
}

/// The 1:1 merge-resolution-state → merge-posture map.
fn derive_merge_posture(state: M5MergeResolutionState) -> M5CoverageMergePosture {
    match state {
        M5MergeResolutionState::MergedClean => M5CoverageMergePosture::MergedCleanSheet,
        M5MergeResolutionState::ShardOmissionDetected => M5CoverageMergePosture::ShardOmissionSheet,
        M5MergeResolutionState::ConflictingOverlap => {
            M5CoverageMergePosture::ConflictingOverlapSheet
        }
        M5MergeResolutionState::PartialMerge => M5CoverageMergePosture::PartialMergeSheet,
        M5MergeResolutionState::SupersededByNewer => M5CoverageMergePosture::SupersededSheet,
        M5MergeResolutionState::MergeUnavailable => M5CoverageMergePosture::MergeUnavailableSheet,
    }
}

/// Derives the bounded merge-action set. The open-incompatible-report action is offered whenever a
/// warning is unresolved; reveal, review-run-scope, and export are always offered.
fn derive_merge_actions(has_unresolved_warning: bool) -> Vec<M5MergeSheetAction> {
    use M5MergeSheetAction as Action;
    let mut actions = vec![Action::RevealMergeDetails, Action::ReviewRunScope];
    if has_unresolved_warning {
        actions.push(Action::OpenIncompatibleReport);
    }
    actions.push(Action::ExportMergeSheet);
    actions
}

// ---- worked cases --------------------------------------------------------

/// One worked snapshot-review-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotCardResolutionCase {
    /// The resolver input.
    pub input: M5SnapshotCardResolutionInput,
    /// The resolved truth. Must equal `resolve_snapshot_review_card(&input)`.
    pub resolved: M5ResolvedSnapshotCard,
}

impl M5SnapshotCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SnapshotCardResolutionInput) -> Self {
        let resolved = resolve_snapshot_review_card(&input).expect("seed snapshot case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_snapshot_review_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved card preserves the input identity, baseline ref, and scope exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.card_identity_ref == self.input.card_identity_ref
            && self.resolved.baseline_ref == self.input.baseline_ref
            && self.resolved.scope_dimensions == self.input.scope_dimensions
    }
}

/// One worked coverage-import-merge-sheet resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MergeSheetResolutionCase {
    /// The resolver input.
    pub input: M5MergeSheetResolutionInput,
    /// The resolved truth. Must equal `resolve_coverage_import_merge_sheet(&input)`.
    pub resolved: M5ResolvedMergeSheet,
}

impl M5MergeSheetResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MergeSheetResolutionInput) -> Self {
        let resolved =
            resolve_coverage_import_merge_sheet(&input).expect("seed merge case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_coverage_import_merge_sheet(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved sheet preserves the input identity, commit / build identity, and run
    /// scope exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.sheet_identity_ref == self.input.sheet_identity_ref
            && self.resolved.commit_identity_ref == self.input.commit_identity_ref
            && self.resolved.build_identity_ref == self.input.build_identity_ref
            && self.resolved.included_runs == self.input.included_runs
            && self.resolved.excluded_runs == self.input.excluded_runs
    }
}

/// One row in the primitive matrix: one review consumer bound to the shared card and sheet
/// anatomy, snapshot baseline identities, diff states, artifact kinds, fallback modes, scope
/// dimensions, review postures, coverage import sources, merge-resolution states, metric kinds,
/// merge postures, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentConsumerRow {
    /// Review consumer family.
    pub consumer_surface: M5SnapshotMergeComponentConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestIntelligenceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 review surface families that render / consume these components.
    pub surface_families: Vec<M5TestIntelligenceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5TestIntelligenceDeploymentLine>,
    /// Snapshot-card anatomy parts this consumer renders (must include the mandatory parts).
    pub snapshot_anatomy_parts: Vec<M5SnapshotCardAnatomyPart>,
    /// Merge-sheet anatomy parts this consumer renders (must include the mandatory parts).
    pub merge_anatomy_parts: Vec<M5MergeSheetAnatomyPart>,
    /// Snapshot artifact kinds this consumer distinguishes.
    pub snapshot_artifact_kinds: Vec<M5SnapshotArtifactKind>,
    /// Snapshot baseline identities this consumer distinguishes.
    pub snapshot_baseline_identities: Vec<M5SnapshotBaselineIdentity>,
    /// Snapshot diff states this consumer distinguishes.
    pub snapshot_diff_states: Vec<M5SnapshotDiffState>,
    /// Snapshot fallback modes this consumer distinguishes.
    pub snapshot_fallback_modes: Vec<M5SnapshotFallbackMode>,
    /// Snapshot scope dimensions this consumer distinguishes.
    pub snapshot_scope_dimensions: Vec<M5SnapshotScopeDimension>,
    /// Snapshot review postures this consumer distinguishes.
    pub snapshot_review_postures: Vec<M5SnapshotReviewPosture>,
    /// Coverage import sources this consumer distinguishes.
    pub coverage_import_sources: Vec<M5CoverageImportSource>,
    /// Merge-resolution states this consumer distinguishes.
    pub merge_resolution_states: Vec<M5MergeResolutionState>,
    /// Coverage metric kinds this consumer distinguishes.
    pub coverage_metric_kinds: Vec<M5CoverageMetricKind>,
    /// Merge postures this consumer distinguishes.
    pub merge_postures: Vec<M5CoverageMergePosture>,
    /// Provenance classes this consumer distinguishes.
    pub provenance_classes: Vec<M5TestIntelligenceProvenanceClass>,
    /// Bounded snapshot actions this consumer offers.
    pub snapshot_actions: Vec<M5SnapshotCardAction>,
    /// Bounded merge actions this consumer offers.
    pub merge_actions: Vec<M5MergeSheetAction>,
    /// Snapshot export fields this consumer carries (must include the mandatory fields).
    pub snapshot_export_fields: Vec<M5SnapshotCardExportField>,
    /// Merge export fields this consumer carries (must include the mandatory fields).
    pub merge_export_fields: Vec<M5MergeSheetExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestIntelligenceAccessibilityRoute>,
    /// Test-intelligence subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestIntelligenceConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestIntelligenceDowngradeTrigger>,
    /// Proof packet refs that keep these components current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by these components.
    pub source_contract_refs: Vec<String>,
    /// Worked snapshot-card resolutions proving the resolver on this consumer.
    pub snapshot_examples: Vec<M5SnapshotCardResolutionCase>,
    /// Worked merge-sheet resolutions proving the resolver on this consumer.
    pub merge_examples: Vec<M5MergeSheetResolutionCase>,
    /// Hard invariant: this consumer never collapses a snapshot acceptance to a blind accept
    /// without artifact count, scope, and fallback visibility. MUST be `false`.
    pub collapses_snapshot_accept_without_scope_or_fallback: bool,
    /// Hard invariant: this consumer never hides the baseline identity or the artifact count
    /// behind a bare verdict. MUST be `false`.
    pub hides_baseline_identity_or_artifact_count: bool,
    /// Hard invariant: this consumer never hides a shard omission or an incompatible-artifact
    /// warning behind a merged number. MUST be `false`.
    pub hides_shard_omission_or_incompatible_warning: bool,
    /// Hard invariant: this consumer never invents an alternate label for a governed snapshot or
    /// merge state. MUST be `false`.
    pub invents_alternate_snapshot_or_merge_state_label: bool,
}

impl M5SnapshotMergeComponentConsumerRow {
    /// True when the row declares every mandatory snapshot anatomy part.
    fn declares_mandatory_snapshot_anatomy(&self) -> bool {
        let present: BTreeSet<M5SnapshotCardAnatomyPart> =
            self.snapshot_anatomy_parts.iter().copied().collect();
        M5SnapshotCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory merge anatomy part.
    fn declares_mandatory_merge_anatomy(&self) -> bool {
        let present: BTreeSet<M5MergeSheetAnatomyPart> =
            self.merge_anatomy_parts.iter().copied().collect();
        M5MergeSheetAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory snapshot export field.
    fn declares_mandatory_snapshot_export(&self) -> bool {
        let present: BTreeSet<M5SnapshotCardExportField> =
            self.snapshot_export_fields.iter().copied().collect();
        M5SnapshotCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory merge export field.
    fn declares_mandatory_merge_export(&self) -> bool {
        let present: BTreeSet<M5MergeSheetExportField> =
            self.merge_export_fields.iter().copied().collect();
        M5MergeSheetExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_snapshot_accept_without_scope_or_fallback
            && !self.hides_baseline_identity_or_artifact_count
            && !self.hides_shard_omission_or_incompatible_warning
            && !self.invents_alternate_snapshot_or_merge_state_label
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentVocabularySet {
    /// Review consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Snapshot-anatomy-part tokens.
    pub snapshot_anatomy_parts: Vec<String>,
    /// Merge-anatomy-part tokens.
    pub merge_anatomy_parts: Vec<String>,
    /// Snapshot-review-posture tokens.
    pub snapshot_review_postures: Vec<String>,
    /// Merge-posture tokens.
    pub merge_postures: Vec<String>,
    /// Artifact-kind tokens.
    pub snapshot_artifact_kinds: Vec<String>,
    /// Fallback-mode tokens.
    pub snapshot_fallback_modes: Vec<String>,
    /// Scope-dimension tokens.
    pub snapshot_scope_dimensions: Vec<String>,
    /// Snapshot-action tokens.
    pub snapshot_actions: Vec<String>,
    /// Merge-action tokens.
    pub merge_actions: Vec<String>,
    /// Snapshot-export-field tokens.
    pub snapshot_export_fields: Vec<String>,
    /// Merge-export-field tokens.
    pub merge_export_fields: Vec<String>,
    /// Snapshot-baseline-identity tokens (reused from the frozen matrix).
    pub snapshot_baseline_identities: Vec<String>,
    /// Snapshot-diff-state tokens (reused from the frozen matrix).
    pub snapshot_diff_states: Vec<String>,
    /// Coverage-import-source tokens (reused from the frozen matrix).
    pub coverage_import_sources: Vec<String>,
    /// Merge-resolution-state tokens (reused from the frozen matrix).
    pub merge_resolution_states: Vec<String>,
    /// Coverage-metric-kind tokens (reused from the frozen matrix).
    pub coverage_metric_kinds: Vec<String>,
    /// Provenance-class tokens (reused from the frozen matrix).
    pub provenance_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SnapshotMergeComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5SnapshotMergeComponentConsumerSurface::ALL, |v| {
                v.as_str()
            }),
            snapshot_anatomy_parts: tokens(&M5SnapshotCardAnatomyPart::ALL, |v| v.as_str()),
            merge_anatomy_parts: tokens(&M5MergeSheetAnatomyPart::ALL, |v| v.as_str()),
            snapshot_review_postures: tokens(&M5SnapshotReviewPosture::ALL, |v| v.as_str()),
            merge_postures: tokens(&M5CoverageMergePosture::ALL, |v| v.as_str()),
            snapshot_artifact_kinds: tokens(&M5SnapshotArtifactKind::ALL, |v| v.as_str()),
            snapshot_fallback_modes: tokens(&M5SnapshotFallbackMode::ALL, |v| v.as_str()),
            snapshot_scope_dimensions: tokens(&M5SnapshotScopeDimension::ALL, |v| v.as_str()),
            snapshot_actions: tokens(&M5SnapshotCardAction::ALL, |v| v.as_str()),
            merge_actions: tokens(&M5MergeSheetAction::ALL, |v| v.as_str()),
            snapshot_export_fields: tokens(&M5SnapshotCardExportField::ALL, |v| v.as_str()),
            merge_export_fields: tokens(&M5MergeSheetExportField::ALL, |v| v.as_str()),
            snapshot_baseline_identities: tokens(&M5SnapshotBaselineIdentity::ALL, |v| v.as_str()),
            snapshot_diff_states: tokens(&M5SnapshotDiffState::ALL, |v| v.as_str()),
            coverage_import_sources: tokens(&M5CoverageImportSource::ALL, |v| v.as_str()),
            merge_resolution_states: tokens(&M5MergeResolutionState::ALL, |v| v.as_str()),
            coverage_metric_kinds: tokens(&M5CoverageMetricKind::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5TestIntelligenceProvenanceClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestIntelligenceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestIntelligenceDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestIntelligenceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentGovernanceReview {
    /// The snapshot card shows its artifact kind and baseline identity.
    pub card_shows_artifact_kind_and_baseline_identity: bool,
    /// The snapshot card shows its diff count.
    pub card_shows_diff_count: bool,
    /// The snapshot card shows its fallback mode and scope.
    pub card_shows_fallback_mode_and_scope: bool,
    /// The snapshot card offers accept / reject / export actions.
    pub card_offers_accept_reject_export: bool,
    /// A snapshot acceptance never presents as a blind accept without scope and fallback
    /// visibility.
    pub acceptance_never_blind_without_scope_and_fallback: bool,
    /// The merge sheet shows its included and excluded runs.
    pub merge_sheet_shows_included_and_excluded_runs: bool,
    /// The merge sheet shows its commit and build identity.
    pub merge_sheet_shows_commit_and_build_identity: bool,
    /// The merge sheet shows its stale-or-incompatible warnings.
    pub merge_sheet_shows_stale_or_incompatible_warnings: bool,
    /// The merge sheet shows its line-versus-branch support.
    pub merge_sheet_shows_line_versus_branch_support: bool,
    /// A merged number is never treated as exact current truth while a warning is unresolved.
    pub merge_never_exact_truth_with_unresolved_warnings: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across every review consumer surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs snapshot / merge truth.
    pub support_export_reconstructs_snapshot_merge_truth: bool,
    /// Later M5 review components cannot invent parallel snapshot / merge vocabulary.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentConsumerProjection {
    /// Snapshot and merge surfaces consume the shared baseline / diff / merge vocabulary.
    pub snapshot_and_merge_surfaces_consume_shared_vocabulary: bool,
    /// The snapshot-posture resolver reads a single canonical source.
    pub snapshot_posture_reads_single_source: bool,
    /// The merge-posture resolver reads a single canonical source.
    pub merge_posture_reads_single_source: bool,
    /// The CI and support/export consumers read the same snapshot / merge vocabulary.
    pub ci_and_support_read_same_snapshot_merge_vocabulary: bool,
    /// Headless and desktop review read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two review components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SnapshotMergeComponentsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SnapshotMergeComponentsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Review consumer rows.
    pub rows: Vec<M5SnapshotMergeComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SnapshotMergeComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SnapshotMergeComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SnapshotMergeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SnapshotMergeComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SnapshotMergeComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 snapshot-review-card / coverage-import-merge-sheet primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SnapshotMergeComponentsPacket {
    /// Record kind; must equal [`M5_SNAPSHOT_MERGE_COMPONENTS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SNAPSHOT_MERGE_COMPONENTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Review consumer rows.
    pub rows: Vec<M5SnapshotMergeComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SnapshotMergeComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SnapshotMergeComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SnapshotMergeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SnapshotMergeComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SnapshotMergeComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SnapshotMergeComponentsPacket {
    /// Builds an M5 snapshot-merge-components primitive packet from stable-lane input.
    pub fn new(input: M5SnapshotMergeComponentsPacketInput) -> Self {
        Self {
            record_kind: M5_SNAPSHOT_MERGE_COMPONENTS_RECORD_KIND.to_owned(),
            schema_version: M5_SNAPSHOT_MERGE_COMPONENTS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 snapshot-merge-components primitive invariants.
    pub fn validate(&self) -> Vec<M5SnapshotMergeComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SNAPSHOT_MERGE_COMPONENTS_RECORD_KIND {
            violations.push(M5SnapshotMergeComponentViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SNAPSHOT_MERGE_COMPONENTS_SCHEMA_VERSION {
            violations.push(M5SnapshotMergeComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SnapshotMergeComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_snapshot_posture_coverage(self, &mut violations);
        validate_merge_posture_coverage(self, &mut violations);
        validate_acceptance_scope_disclosure(self, &mut violations);
        validate_raw_fallback_disclosure(self, &mut violations);
        validate_omission_disclosure(self, &mut violations);
        validate_import_source_coverage(self, &mut violations);
        validate_metric_dimension_continuity(self, &mut violations);
        validate_baseline_identity_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 snapshot merge components packet serializes"),
        ) {
            violations.push(M5SnapshotMergeComponentViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 snapshot merge components packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per review consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,snapshot_anatomy,snapshot_postures,fallback_modes,merge_postures,import_sources,snapshot_actions,merge_actions,snapshot_examples,merge_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.snapshot_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.snapshot_review_postures, |v| v.as_str()),
                join_tokens(&row.snapshot_fallback_modes, |v| v.as_str()),
                join_tokens(&row.merge_postures, |v| v.as_str()),
                join_tokens(&row.coverage_import_sources, |v| v.as_str()),
                join_tokens(&row.snapshot_actions, |v| v.as_str()),
                join_tokens(&row.merge_actions, |v| v.as_str()),
                row.snapshot_examples.len(),
                row.merge_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Snapshot-Review-Card / Coverage-Import-Merge-Sheet Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Review consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Snapshot postures: {}\n",
            self.vocabulary_set.snapshot_review_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Merge postures: {}\n",
            self.vocabulary_set.merge_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Fallback modes: {}\n",
            self.vocabulary_set.snapshot_fallback_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Review consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cards: {} / sheets: {}\n",
                row.snapshot_examples.len(),
                row.merge_examples.len()
            ));
            for case in &row.snapshot_examples {
                out.push_str(&format!(
                    "    - card `{}` (`{}`) -> `{}` (acceptance `{}`, scope `{}`, raw fallback `{}`)\n",
                    case.resolved.card_identity_ref,
                    case.resolved.diff_state.as_str(),
                    case.resolved.review_posture.as_str(),
                    case.resolved.is_acceptance_decision,
                    case.resolved.has_scope_disclosed,
                    case.resolved.has_raw_fallback,
                ));
            }
            for case in &row.merge_examples {
                out.push_str(&format!(
                    "    - sheet `{}` (`{}`) -> `{}` (omission `{}`, warning `{}`, exact `{}`)\n",
                    case.resolved.sheet_identity_ref,
                    case.resolved.merge_resolution.as_str(),
                    case.resolved.merge_posture.as_str(),
                    case.resolved.exposes_omitted_shards,
                    case.resolved.discloses_stale_or_incompatible,
                    case.resolved.is_exact_current_truth,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 snapshot-merge-components export.
#[derive(Debug)]
pub enum M5SnapshotMergeComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SnapshotMergeComponentViolation>),
}

impl fmt::Display for M5SnapshotMergeComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 snapshot merge components export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 snapshot merge components export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SnapshotMergeComponentArtifactError {}

/// Validation failures emitted by [`M5SnapshotMergeComponentsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SnapshotMergeComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required review consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A review consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory snapshot anatomy parts.
    MandatorySnapshotAnatomyMissing,
    /// A row omits one of the mandatory merge anatomy parts.
    MandatoryMergeAnatomyMissing,
    /// A row omits one of the mandatory snapshot export fields.
    MandatorySnapshotExportMissing,
    /// A row omits one of the mandatory merge export fields.
    MandatoryMergeExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked snapshot or merge resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every snapshot posture.
    SnapshotPostureCoverageUnproven,
    /// The worked resolutions do not exercise every merge posture.
    MergePostureCoverageUnproven,
    /// The worked resolutions do not prove both a scoped acceptance decision and a matched
    /// baseline.
    AcceptanceScopeDisclosureUnproven,
    /// The worked resolutions do not prove a raw / text fallback preserved for an opaque or
    /// render-unavailable card.
    RawFallbackDisclosureUnproven,
    /// The worked resolutions do not prove a merge sheet that exposes omitted shards with disclosed
    /// excluded runs.
    OmissionDisclosureUnproven,
    /// The worked resolutions do not exercise the local, imported-CI, cached, and stale import
    /// sources.
    ImportSourceCoverageUnproven,
    /// A worked merge resolution does not disclose its line-versus-branch metric dimension.
    MetricDimensionUnproven,
    /// The worked resolutions do not exercise the committed, imported, pending-new, and missing
    /// baseline identities.
    BaselineIdentityCoverageUnproven,
    /// A worked resolution does not preserve its exact identity and scope.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SnapshotMergeComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatorySnapshotAnatomyMissing => "mandatory_snapshot_anatomy_missing",
            Self::MandatoryMergeAnatomyMissing => "mandatory_merge_anatomy_missing",
            Self::MandatorySnapshotExportMissing => "mandatory_snapshot_export_missing",
            Self::MandatoryMergeExportMissing => "mandatory_merge_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::SnapshotPostureCoverageUnproven => "snapshot_posture_coverage_unproven",
            Self::MergePostureCoverageUnproven => "merge_posture_coverage_unproven",
            Self::AcceptanceScopeDisclosureUnproven => "acceptance_scope_disclosure_unproven",
            Self::RawFallbackDisclosureUnproven => "raw_fallback_disclosure_unproven",
            Self::OmissionDisclosureUnproven => "omission_disclosure_unproven",
            Self::ImportSourceCoverageUnproven => "import_source_coverage_unproven",
            Self::MetricDimensionUnproven => "metric_dimension_unproven",
            Self::BaselineIdentityCoverageUnproven => "baseline_identity_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 snapshot-merge-components export.
pub fn current_stable_m5_snapshot_merge_components_export(
) -> Result<M5SnapshotMergeComponentsPacket, M5SnapshotMergeComponentArtifactError> {
    let packet: M5SnapshotMergeComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-snapshot-coverage-import-primitive-proof/support_export.json"
    )))
    .map_err(M5SnapshotMergeComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SnapshotMergeComponentArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_DOC_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_REVIEW_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_COVERAGE_MERGE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SnapshotMergeComponentViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SnapshotMergeComponentViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let present: BTreeSet<M5SnapshotMergeComponentConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5SnapshotMergeComponentConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5SnapshotMergeComponentViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.snapshot_anatomy_parts.is_empty()
            || row.merge_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.snapshot_artifact_kinds.is_empty()
            || row.snapshot_baseline_identities.is_empty()
            || row.snapshot_diff_states.is_empty()
            || row.snapshot_fallback_modes.is_empty()
            || row.snapshot_scope_dimensions.is_empty()
            || row.snapshot_review_postures.is_empty()
            || row.coverage_import_sources.is_empty()
            || row.merge_resolution_states.is_empty()
            || row.coverage_metric_kinds.is_empty()
            || row.merge_postures.is_empty()
            || row.provenance_classes.is_empty()
            || row.snapshot_actions.is_empty()
            || row.merge_actions.is_empty()
            || row.snapshot_export_fields.is_empty()
            || row.merge_export_fields.is_empty()
        {
            violations.push(M5SnapshotMergeComponentViolation::RowIncomplete);
        }
        if !row.declares_mandatory_snapshot_anatomy() {
            violations.push(M5SnapshotMergeComponentViolation::MandatorySnapshotAnatomyMissing);
        }
        if !row.declares_mandatory_merge_anatomy() {
            violations.push(M5SnapshotMergeComponentViolation::MandatoryMergeAnatomyMissing);
        }
        if !row.declares_mandatory_snapshot_export() {
            violations.push(M5SnapshotMergeComponentViolation::MandatorySnapshotExportMissing);
        }
        if !row.declares_mandatory_merge_export() {
            violations.push(M5SnapshotMergeComponentViolation::MandatoryMergeExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SnapshotMergeComponentViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SnapshotMergeComponentViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SnapshotMergeComponentViolation::DowngradeTriggersMissing);
        }
        if row.snapshot_examples.is_empty() || row.merge_examples.is_empty() {
            violations.push(M5SnapshotMergeComponentViolation::ExampleMissing);
        }
        if row
            .snapshot_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .merge_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SnapshotMergeComponentViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SnapshotMergeComponentViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SnapshotMergeComponentViolation::RowInvariantViolated);
        }
    }
}

/// Every snapshot posture must be exercised by some worked resolution — the proof that a
/// matches-baseline, diff-detected, new-snapshot, obsolete, render-unavailable, and raw-text
/// fallback card each get a distinct posture rather than one collapsed verdict.
fn validate_snapshot_posture_coverage(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let exercised: BTreeSet<M5SnapshotReviewPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.snapshot_examples.iter())
        .map(|case| case.resolved.review_posture)
        .collect();
    let covered = M5SnapshotReviewPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5SnapshotMergeComponentViolation::SnapshotPostureCoverageUnproven);
    }
}

/// Every merge posture must be exercised by some worked resolution — the proof that a shard
/// omission never collapses into a clean merge.
fn validate_merge_posture_coverage(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let exercised: BTreeSet<M5CoverageMergePosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.merge_examples.iter())
        .map(|case| case.resolved.merge_posture)
        .collect();
    let covered = M5CoverageMergePosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5SnapshotMergeComponentViolation::MergePostureCoverageUnproven);
    }
}

/// At least one worked snapshot resolution must prove an acceptance decision that discloses its
/// scope, and at least one must prove a matched baseline — the acceptance-criterion example that a
/// snapshot acceptance can never collapse to a blind `Accept all` without artifact count, scope,
/// and fallback visibility.
fn validate_acceptance_scope_disclosure(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let has_scoped_acceptance = packet.rows.iter().any(|row| {
        row.snapshot_examples
            .iter()
            .any(|case| case.resolved.is_acceptance_decision && case.resolved.has_scope_disclosed)
    });
    let has_matched_baseline = packet.rows.iter().any(|row| {
        row.snapshot_examples.iter().any(|case| {
            case.resolved.review_posture == M5SnapshotReviewPosture::MatchesBaselineCard
        })
    });
    if !(has_scoped_acceptance && has_matched_baseline) {
        violations.push(M5SnapshotMergeComponentViolation::AcceptanceScopeDisclosureUnproven);
    }
}

/// At least one worked snapshot resolution must prove a raw / text fallback preserved for an
/// opaque binary or render-unavailable card — the guardrail that a binary-only change always keeps
/// a raw / text fallback.
fn validate_raw_fallback_disclosure(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let has_preserved_fallback = packet.rows.iter().any(|row| {
        row.snapshot_examples.iter().any(|case| {
            (case.resolved.is_opaque_binary
                || case.resolved.review_posture == M5SnapshotReviewPosture::RenderUnavailableCard)
                && case.resolved.has_raw_fallback
        })
    });
    if !has_preserved_fallback {
        violations.push(M5SnapshotMergeComponentViolation::RawFallbackDisclosureUnproven);
    }
}

/// At least one worked merge resolution must prove a sheet that exposes omitted shards with
/// disclosed excluded runs — the acceptance-criterion requirement that a coverage merge exposes
/// omitted shards / platforms before any merged result is treated as exact current truth.
fn validate_omission_disclosure(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let has_disclosed_omission = packet.rows.iter().any(|row| {
        row.merge_examples.iter().any(|case| {
            case.resolved.exposes_omitted_shards && !case.resolved.excluded_runs.is_empty()
        })
    });
    if !has_disclosed_omission {
        violations.push(M5SnapshotMergeComponentViolation::OmissionDisclosureUnproven);
    }
}

/// The worked merge resolutions must exercise the local, imported-CI, cached, and stale import
/// sources — the requirement that a merge sheet names where each report came from.
fn validate_import_source_coverage(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let exercised: BTreeSet<M5CoverageImportSource> = packet
        .rows
        .iter()
        .flat_map(|row| row.merge_examples.iter())
        .map(|case| case.resolved.import_source)
        .collect();
    let covered = [
        M5CoverageImportSource::LocalRun,
        M5CoverageImportSource::ImportedCiArtifact,
        M5CoverageImportSource::CachedLocal,
        M5CoverageImportSource::StalePrior,
    ]
    .iter()
    .all(|source| exercised.contains(source));
    if !covered {
        violations.push(M5SnapshotMergeComponentViolation::ImportSourceCoverageUnproven);
    }
}

/// Every worked merge resolution must disclose its line-versus-branch metric dimension — the
/// acceptance-criterion requirement that a merge sheet always names its line-versus-branch support.
fn validate_metric_dimension_continuity(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let disclosed = packet
        .rows
        .iter()
        .flat_map(|row| row.merge_examples.iter())
        .all(|case| case.resolved.discloses_metric_dimension);
    if !disclosed {
        violations.push(M5SnapshotMergeComponentViolation::MetricDimensionUnproven);
    }
}

/// The worked snapshot resolutions must exercise the committed, imported, pending-new, and missing
/// baseline identities — the acceptance-criterion requirement that an imported baseline never reads
/// as a local accept.
fn validate_baseline_identity_coverage(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let exercised: BTreeSet<M5SnapshotBaselineIdentity> = packet
        .rows
        .iter()
        .flat_map(|row| row.snapshot_examples.iter())
        .map(|case| case.resolved.baseline_identity)
        .collect();
    let covered = [
        M5SnapshotBaselineIdentity::CommittedBaseline,
        M5SnapshotBaselineIdentity::ImportedBaseline,
        M5SnapshotBaselineIdentity::PendingNewBaseline,
        M5SnapshotBaselineIdentity::MissingBaseline,
    ]
    .iter()
    .all(|identity| exercised.contains(identity));
    if !covered {
        violations.push(M5SnapshotMergeComponentViolation::BaselineIdentityCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and scope — the invariant that neither
/// component rewrites the user's snapshot or merge identity.
fn validate_identity_preservation(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let snapshot_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.snapshot_examples.iter())
        .all(|case| case.preserves_identity());
    let merge_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.merge_examples.iter())
        .all(|case| case.preserves_identity());
    if !(snapshot_preserved && merge_preserved) {
        violations.push(M5SnapshotMergeComponentViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_shows_artifact_kind_and_baseline_identity,
        review.card_shows_diff_count,
        review.card_shows_fallback_mode_and_scope,
        review.card_offers_accept_reject_export,
        review.acceptance_never_blind_without_scope_and_fallback,
        review.merge_sheet_shows_included_and_excluded_runs,
        review.merge_sheet_shows_commit_and_build_identity,
        review.merge_sheet_shows_stale_or_incompatible_warnings,
        review.merge_sheet_shows_line_versus_branch_support,
        review.merge_never_exact_truth_with_unresolved_warnings,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_snapshot_merge_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SnapshotMergeComponentViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.snapshot_and_merge_surfaces_consume_shared_vocabulary,
        projection.snapshot_posture_reads_single_source,
        projection.merge_posture_reads_single_source,
        projection.ci_and_support_read_same_snapshot_merge_vocabulary,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5SnapshotMergeComponentViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SnapshotMergeComponentViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SnapshotMergeComponentsPacket,
    violations: &mut Vec<M5SnapshotMergeComponentViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SnapshotMergeComponentViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
