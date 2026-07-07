//! One reusable M5 test-explorer primitive — the inline result marker — so an editor or
//! notebook surface stays honest about a test's freshness and provenance: a source
//! decoration never implies a current local result when the evidence came from an imported
//! or stale run, or maps only approximately to the current file/cell state.
//!
//! Aureline's frozen test-explorer / watch / triage component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! names the inline result marker as one governed component family and freezes its
//! controlled vocabulary — the marker verdicts, the imported/live result origins, the
//! result freshness states, the failure categories, the test target classes and environment
//! lanes, the attempt lineage kinds, the quarantine ownership classes and release impacts,
//! plus the surface families, the deployment lines, the consumer surfaces, the accessibility
//! routes, the qualification classes, and the downgrade triggers. This module *implements*
//! that contract as one reusable resolver so a user can tell — from the inline marker alone
//! — the pass/fail/error/timeout state, the stability-or-flaky chip, the target/environment
//! shorthand, the imported-versus-live class, the last-result freshness, how faithfully the
//! marker maps to the current file/cell state, and the recent-attempt lineage behind an
//! open-recent-attempts action — and, above all, whether the decoration may honestly read
//! as a current live-local result at all, without ever letting an imported, stale, or
//! approximately-mapped run inherit the same visual certainty as a fresh live-local one.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_inline_result_marker`] — takes one marker's verdict, optional failure
//!    category, stability chip, result origin, result freshness, source-mapping fidelity,
//!    target class, environment lane, attempt lineage, quarantine ownership and release
//!    impact, recent-attempt count, mute flag, opaque marker label, and opaque stable
//!    marker identity, and produces one [`M5ResolvedInlineResultMarker`] carrying the
//!    derived marker posture (a quarantined, unmapped, approximate-mapping, imported-
//!    evidence, stale-result, or live-local marker), whether the marker may be rerun from
//!    the decoration, whether recent attempts can be opened from it, whether it carries
//!    reduced (imported / stale / approximate / unmapped) certainty rather than live
//!    certainty, whether it may honestly imply a current live-local result, and the bounded
//!    reveal-evidence / open-recent-attempts / rerun / review-quarantine / export actions.
//!    It never masks the marker's verdict or imported/live origin, never hides a
//!    quarantine's release impact, never overstates an imported, stale, or
//!    approximately-mapped run as a current live-local result, and never drops the
//!    attempt lineage a triage or tree consumer would show.
//!
//! A single parity matrix — [`M5InlineResultMarkerPacket`] — binds one row per claimed M5
//! editor / notebook consumer (the editor-gutter marker, the editor inline marker, the
//! notebook-cell marker, the headless/CLI marker, and the marker-report export) to the
//! shared marker anatomy, the same verdicts, stability chips, result origins, freshness
//! states, source mappings, marker postures, attempt lineage kinds, bounded actions, export
//! fields, and non-visual accessibility routes, so the state / origin / freshness /
//! mapping / attempt-lineage vocabulary stays identical across the editor, notebook,
//! headless/export, and report consumers — the acceptance-criterion parity with the test
//! tree and triage consumers.
//!
//! The marker verdict ([`M5InlineMarkerVerdict`]), failure category ([`M5FailureCategory`]),
//! result origin ([`M5TestResultOrigin`]), result freshness ([`M5TestResultFreshness`]),
//! target class ([`M5TestTargetClass`]), environment lane ([`M5TestEnvironmentLane`]),
//! attempt lineage kind ([`M5AttemptLineageKind`]), quarantine ownership
//! ([`M5QuarantineOwnership`]), release impact ([`M5TestReleaseImpact`]), surface family
//! ([`M5TestSurfaceFamily`]), deployment line ([`M5TestDeploymentLine`]), consumer surface
//! ([`M5TestConsumerSurface`]), accessibility route ([`M5TestAccessibilityRoute`]),
//! qualification class ([`M5TestQualificationClass`]), and downgrade trigger
//! ([`M5TestDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the inline marker
//! itself: the marker's editor/notebook consumers, its stability chip, its source-mapping
//! fidelity, its anatomy parts, its derived posture, its bounded actions, and its export
//! fields. No M5 test surface invents a second inline-marker grammar.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every marker label and marker identity is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed,
    seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed,
    seeded_m5_inline_result_marker_packet, M5_INLINE_RESULT_MARKER_PACKET_ID,
};

// The marker verdict, failure category, result origin, freshness, target class, environment
// lane, attempt lineage kind, quarantine ownership, release impact, surface family,
// deployment line, consumer surface, accessibility route, qualification class, and downgrade
// triggers are frozen once, in the test-explorer / watch / triage component matrix. This
// primitive reuses them verbatim so it never invents a parallel inline-marker vocabulary.
pub use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5AttemptLineageKind, M5FailureCategory, M5InlineMarkerVerdict, M5QuarantineOwnership,
    M5TestAccessibilityRoute, M5TestConsumerSurface, M5TestDeploymentLine, M5TestDowngradeTrigger,
    M5TestEnvironmentLane, M5TestQualificationClass, M5TestReleaseImpact, M5TestResultFreshness,
    M5TestResultOrigin, M5TestSurfaceFamily, M5TestTargetClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5InlineResultMarkerPacket`].
pub const M5_INLINE_RESULT_MARKER_RECORD_KIND: &str =
    "implement_m5_inline_result_markers_with_live_versus_imported_versus_stale_stability_chips_open_recent_attempts_and_target_env_shorthand_across_claimed_m5_editors_and_notebook_views";

/// Schema version for M5 inline-result-marker records.
pub const M5_INLINE_RESULT_MARKER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the inline-marker boundary schema.
pub const M5_INLINE_RESULT_MARKER_SCHEMA_REF: &str =
    "schemas/ui/m5-inline-result-marker.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_INLINE_RESULT_MARKER_DOC_REF: &str =
    "docs/testing/m5_inline_result_marker_primitive.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix this
/// primitive narrows from.
pub const M5_INLINE_RESULT_MARKER_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the test-item-identity contract this primitive binds its
/// origin / class truth against.
pub const M5_INLINE_RESULT_MARKER_TEST_ITEM_IDENTITY_REF: &str =
    "schemas/testing/test_item_identity.schema.json";

/// Repo-relative path of the test-attempt contract this primitive binds its attempt-lineage
/// and open-recent-attempts truth against.
pub const M5_INLINE_RESULT_MARKER_TEST_ATTEMPT_REF: &str =
    "schemas/testing/test_attempt.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_INLINE_RESULT_MARKER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-inline-result-marker-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INLINE_RESULT_MARKER_ARTIFACT_REF: &str =
    "artifacts/release/m5-inline-result-marker-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_INLINE_RESULT_MARKER_CSV_REF: &str =
    "artifacts/release/m5-inline-result-marker-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INLINE_RESULT_MARKER_REPORT_REF: &str =
    "artifacts/design/m5-inline-result-marker-primitive.md";

/// One claimed M5 editor / notebook consumer that renders the shared inline result marker.
/// These are the consumers the acceptance criteria name — the editor-gutter marker, the
/// editor inline marker, the notebook-cell marker, the headless/CLI marker, and the
/// marker-report export — so the same marker grammar works across every claimed editor and
/// notebook surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerConsumerSurface {
    /// The editor-gutter marker surface.
    EditorGutterMarker,
    /// The editor inline (code-lens / decoration) marker surface.
    EditorInlineMarker,
    /// The notebook-cell marker surface.
    NotebookCellMarker,
    /// The headless / CLI marker surface.
    HeadlessCliMarker,
    /// The marker-report export surface.
    MarkerReportExport,
}

impl M5InlineMarkerConsumerSurface {
    /// Every claimed editor / notebook consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EditorGutterMarker,
        Self::EditorInlineMarker,
        Self::NotebookCellMarker,
        Self::HeadlessCliMarker,
        Self::MarkerReportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorGutterMarker => "editor_gutter_marker",
            Self::EditorInlineMarker => "editor_inline_marker",
            Self::NotebookCellMarker => "notebook_cell_marker",
            Self::HeadlessCliMarker => "headless_cli_marker",
            Self::MarkerReportExport => "marker_report_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorGutterMarker => "Editor Gutter Marker",
            Self::EditorInlineMarker => "Editor Inline Marker",
            Self::NotebookCellMarker => "Notebook Cell Marker",
            Self::HeadlessCliMarker => "Headless / CLI Marker",
            Self::MarkerReportExport => "Marker Report Export",
        }
    }
}

/// Controlled stability-or-flaky chip an inline marker shows alongside its verdict, so a
/// marker never leaves a test's flakiness / quarantine history implicit behind a single
/// pass/fail glyph. This is the stability chip the implementation requirements name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarkerStabilityChip {
    /// The test has a consistently stable history.
    StableChip,
    /// The test is suspected flaky.
    FlakySuspectedChip,
    /// The test is a known, confirmed flaky.
    KnownFlakyChip,
    /// The test is muted / quarantined.
    QuarantinedChip,
    /// The test is newly added and has no stability history yet.
    NewlyAddedChip,
    /// The test's stability is unknown.
    UnknownStabilityChip,
}

impl M5MarkerStabilityChip {
    /// Every stability chip, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableChip,
        Self::FlakySuspectedChip,
        Self::KnownFlakyChip,
        Self::QuarantinedChip,
        Self::NewlyAddedChip,
        Self::UnknownStabilityChip,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableChip => "stable_chip",
            Self::FlakySuspectedChip => "flaky_suspected_chip",
            Self::KnownFlakyChip => "known_flaky_chip",
            Self::QuarantinedChip => "quarantined_chip",
            Self::NewlyAddedChip => "newly_added_chip",
            Self::UnknownStabilityChip => "unknown_stability_chip",
        }
    }
}

/// Controlled source-mapping fidelity — how faithfully a marker maps to the current
/// file/cell state, so a decoration is never placed as if it exactly described the current
/// buffer when the source drifted, moved, or has no local buffer at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarkerSourceMapping {
    /// The marker maps exactly to the current file/cell state.
    ExactMapping,
    /// The marker maps only approximately — the source drifted since the result was
    /// produced.
    ApproximateMapping,
    /// The marker no longer maps to any location in the current buffer.
    UnmappedToBuffer,
    /// There is no local buffer to map to (a headless / CLI / report consumer).
    NoLocalBuffer,
}

impl M5MarkerSourceMapping {
    /// Every source-mapping fidelity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactMapping,
        Self::ApproximateMapping,
        Self::UnmappedToBuffer,
        Self::NoLocalBuffer,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMapping => "exact_mapping",
            Self::ApproximateMapping => "approximate_mapping",
            Self::UnmappedToBuffer => "unmapped_to_buffer",
            Self::NoLocalBuffer => "no_local_buffer",
        }
    }

    /// True when the mapping is only approximate — the marker's location drifted.
    pub const fn is_approximate(self) -> bool {
        matches!(self, Self::ApproximateMapping)
    }

    /// True when the marker no longer maps to any location in the current buffer.
    pub const fn is_unmapped(self) -> bool {
        matches!(self, Self::UnmappedToBuffer)
    }
}

/// The derived posture of an inline result marker — the resolver's verdict about how much
/// certainty the decoration carries and how visibly it must degrade. Computed in a fixed
/// honesty-first order, so a quarantined, unmapped, approximately-mapped, imported, or
/// stale marker never reads with the same visual certainty as a fresh live-local result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerPosture {
    /// The test is muted / quarantined; its ownership and release impact head the marker.
    QuarantinedMarker,
    /// The marker no longer maps to any location in the current buffer.
    UnmappedMarker,
    /// The marker maps only approximately to the current file/cell state.
    ApproximateMappingMarker,
    /// The result is imported from an external run, not produced live here; reduced
    /// certainty.
    ImportedEvidenceMarker,
    /// The live-local result is stale, outdated, or expired relative to its source.
    StaleResultMarker,
    /// A fresh, live-local, exactly-mapped result — the only posture that may honestly
    /// read as a current local result.
    LiveLocalMarker,
}

impl M5InlineMarkerPosture {
    /// Every marker posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::QuarantinedMarker,
        Self::UnmappedMarker,
        Self::ApproximateMappingMarker,
        Self::ImportedEvidenceMarker,
        Self::StaleResultMarker,
        Self::LiveLocalMarker,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuarantinedMarker => "quarantined_marker",
            Self::UnmappedMarker => "unmapped_marker",
            Self::ApproximateMappingMarker => "approximate_mapping_marker",
            Self::ImportedEvidenceMarker => "imported_evidence_marker",
            Self::StaleResultMarker => "stale_result_marker",
            Self::LiveLocalMarker => "live_local_marker",
        }
    }

    /// True only for a fresh, live-local, exactly-mapped marker — the one posture that may
    /// present full live certainty and imply a current local result. Imported, stale, and
    /// approximately-mapped markers deliberately never qualify.
    pub const fn shows_live_certainty(self) -> bool {
        matches!(self, Self::LiveLocalMarker)
    }

    /// True when the marker carries reduced (imported / stale / approximate / unmapped)
    /// certainty and must therefore visibly degrade rather than read as a current local
    /// result.
    pub const fn carries_reduced_certainty(self) -> bool {
        matches!(
            self,
            Self::ImportedEvidenceMarker
                | Self::StaleResultMarker
                | Self::ApproximateMappingMarker
                | Self::UnmappedMarker
        )
    }

    /// True when the decoration needs to degrade visibly before it is trusted as a current
    /// local result.
    pub const fn needs_attention(self) -> bool {
        !matches!(self, Self::LiveLocalMarker)
    }
}

/// One bounded action an inline result marker offers, so a marker never hides its
/// reveal-evidence / open-recent-attempts / rerun / review-quarantine / export affordances,
/// and a user can act on the decoration without leaving the editor or notebook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerAction {
    /// Reveal the marker's verdict, origin, freshness, mapping, and target/environment.
    RevealMarkerEvidence,
    /// Open the recent attempts behind this marker.
    OpenRecentAttempts,
    /// Rerun the test from the marker.
    RerunFromMarker,
    /// Review the test's mute / quarantine and its release impact.
    ReviewQuarantine,
    /// Export the inline marker as test evidence.
    ExportMarker,
}

impl M5InlineMarkerAction {
    /// Every marker action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealMarkerEvidence,
        Self::OpenRecentAttempts,
        Self::RerunFromMarker,
        Self::ReviewQuarantine,
        Self::ExportMarker,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealMarkerEvidence => "reveal_marker_evidence",
            Self::OpenRecentAttempts => "open_recent_attempts",
            Self::RerunFromMarker => "rerun_from_marker",
            Self::ReviewQuarantine => "review_quarantine",
            Self::ExportMarker => "export_marker",
        }
    }
}

/// Controlled inline-marker anatomy part the shared marker surfaces. The parts in
/// [`M5InlineMarkerAnatomyPart::MANDATORY`] are required on every marker so the verdict
/// state, stability chip, imported/live origin, freshness, and recent-attempt lineage are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerAnatomyPart {
    /// The pass/fail/error/timeout verdict-state cue.
    VerdictStateCue,
    /// The stability-or-flaky chip cue.
    StabilityChipCue,
    /// The target / environment shorthand cue.
    TargetEnvironmentCue,
    /// The imported/live origin-class cue.
    OriginClassCue,
    /// The last-result freshness cue.
    FreshnessCue,
    /// The source-mapping-fidelity cue.
    SourceMappingCue,
    /// The recent-attempts (open-recent-attempts) cue.
    RecentAttemptsCue,
    /// The mute / quarantine + release-impact cue.
    QuarantineCue,
}

impl M5InlineMarkerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::VerdictStateCue,
        Self::StabilityChipCue,
        Self::TargetEnvironmentCue,
        Self::OriginClassCue,
        Self::FreshnessCue,
        Self::SourceMappingCue,
        Self::RecentAttemptsCue,
        Self::QuarantineCue,
    ];

    /// The anatomy parts every marker must render.
    pub const MANDATORY: [Self; 5] = [
        Self::VerdictStateCue,
        Self::StabilityChipCue,
        Self::OriginClassCue,
        Self::FreshnessCue,
        Self::RecentAttemptsCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerdictStateCue => "verdict_state_cue",
            Self::StabilityChipCue => "stability_chip_cue",
            Self::TargetEnvironmentCue => "target_environment_cue",
            Self::OriginClassCue => "origin_class_cue",
            Self::FreshnessCue => "freshness_cue",
            Self::SourceMappingCue => "source_mapping_cue",
            Self::RecentAttemptsCue => "recent_attempts_cue",
            Self::QuarantineCue => "quarantine_cue",
        }
    }
}

/// A field the marker export carries so inline-marker truth is reconstructable. The fields
/// in [`M5InlineMarkerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerExportField {
    /// The marker verdict.
    Verdict,
    /// The stability chip.
    StabilityChip,
    /// The imported/live result origin.
    ResultOrigin,
    /// The last-result freshness.
    ResultFreshness,
    /// The source-mapping fidelity.
    SourceMapping,
    /// The derived marker posture.
    MarkerPosture,
    /// The attempt lineage kind.
    AttemptLineage,
    /// The bounded available actions.
    AvailableActions,
}

impl M5InlineMarkerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Verdict,
        Self::StabilityChip,
        Self::ResultOrigin,
        Self::ResultFreshness,
        Self::SourceMapping,
        Self::MarkerPosture,
        Self::AttemptLineage,
        Self::AvailableActions,
    ];

    /// The export fields every marker must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Verdict,
        Self::ResultOrigin,
        Self::ResultFreshness,
        Self::MarkerPosture,
        Self::AttemptLineage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verdict => "verdict",
            Self::StabilityChip => "stability_chip",
            Self::ResultOrigin => "result_origin",
            Self::ResultFreshness => "result_freshness",
            Self::SourceMapping => "source_mapping",
            Self::MarkerPosture => "marker_posture",
            Self::AttemptLineage => "attempt_lineage",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a result origin is anything other than a live-local run — imported, replayed,
/// synthetic, or unattributed — and therefore must not read as a current local result.
pub const fn result_origin_is_not_live_local(origin: M5TestResultOrigin) -> bool {
    !matches!(origin, M5TestResultOrigin::LiveLocal)
}

/// True when a freshness state means the result no longer matches its current source.
pub const fn freshness_is_stale(freshness: M5TestResultFreshness) -> bool {
    matches!(
        freshness,
        M5TestResultFreshness::Stale
            | M5TestResultFreshness::OutdatedSource
            | M5TestResultFreshness::Expired
    )
}

// ---- inline-result-marker resolver --------------------------------------

/// The full input to the inline-result-marker resolver for one marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerResolutionInput {
    /// The pass/fail/error/timeout verdict.
    pub verdict: M5InlineMarkerVerdict,
    /// The specific failure category, when the verdict is a failure (e.g. timeout).
    pub failure_category: Option<M5FailureCategory>,
    /// The stability-or-flaky chip.
    pub stability_chip: M5MarkerStabilityChip,
    /// The imported/live result origin.
    pub result_origin: M5TestResultOrigin,
    /// The last-result freshness.
    pub result_freshness: M5TestResultFreshness,
    /// How faithfully the marker maps to the current file/cell state.
    pub source_mapping: M5MarkerSourceMapping,
    /// The test target class (target chip).
    pub target_class: M5TestTargetClass,
    /// The test environment lane (environment chip).
    pub environment_lane: M5TestEnvironmentLane,
    /// The attempt lineage kind behind this marker.
    pub attempt_lineage: M5AttemptLineageKind,
    /// The quarantine ownership behind the mute/quarantine state.
    pub quarantine_ownership: M5QuarantineOwnership,
    /// The release impact of the mute/quarantine.
    pub release_impact: M5TestReleaseImpact,
    /// The number of recent attempts an open-recent-attempts action can reveal.
    pub recent_attempt_count: u32,
    /// True when the test is muted / quarantined.
    pub item_muted: bool,
    /// The opaque user-facing marker label (must be non-empty).
    pub marker_label: String,
    /// The opaque stable marker identity (must be non-empty).
    pub marker_identity_ref: String,
}

/// The resolved inline-result-marker truth for one marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInlineResultMarker {
    /// The verdict.
    pub verdict: M5InlineMarkerVerdict,
    /// The specific failure category, preserved from the input.
    pub failure_category: Option<M5FailureCategory>,
    /// The stability chip.
    pub stability_chip: M5MarkerStabilityChip,
    /// The imported/live result origin.
    pub result_origin: M5TestResultOrigin,
    /// The last-result freshness.
    pub result_freshness: M5TestResultFreshness,
    /// The source-mapping fidelity.
    pub source_mapping: M5MarkerSourceMapping,
    /// The test target class.
    pub target_class: M5TestTargetClass,
    /// The test environment lane.
    pub environment_lane: M5TestEnvironmentLane,
    /// The attempt lineage kind, preserved from the input.
    pub attempt_lineage: M5AttemptLineageKind,
    /// The quarantine ownership.
    pub quarantine_ownership: M5QuarantineOwnership,
    /// The release impact.
    pub release_impact: M5TestReleaseImpact,
    /// The number of recent attempts, preserved from the input.
    pub recent_attempt_count: u32,
    /// The opaque marker label, preserved exactly from the input.
    pub marker_label: String,
    /// The opaque stable marker identity, preserved exactly from the input.
    pub marker_identity_ref: String,
    /// The derived marker posture.
    pub marker_posture: M5InlineMarkerPosture,
    /// The bounded actions this marker offers.
    pub available_actions: Vec<M5InlineMarkerAction>,
    /// True when the test can be rerun from the marker.
    pub can_rerun_from_marker: bool,
    /// True when recent attempts can be opened from the marker.
    pub can_open_recent_attempts: bool,
    /// True when the test is muted / quarantined.
    pub is_muted: bool,
    /// True only for a fresh, live-local, exactly-mapped marker — never for imported,
    /// stale, or approximately-mapped markers.
    pub shows_live_certainty: bool,
    /// True when the marker carries reduced (imported / stale / approximate / unmapped)
    /// certainty.
    pub carries_reduced_certainty: bool,
    /// True only when the decoration may honestly imply a current live-local result — the
    /// acceptance-criterion promise that imported or stale runs never do. Equal to
    /// `shows_live_certainty`.
    pub implies_current_local_result: bool,
    /// True when the marker must degrade visibly before it is trusted as a current local
    /// result.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_inline_result_marker`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InlineMarkerResolutionError {
    /// The marker label was empty.
    EmptyMarkerLabel,
    /// The marker identity ref was empty.
    EmptyMarkerIdentity,
    /// A marker descriptor carried forbidden material.
    ForbiddenMarkerMaterial,
}

impl M5InlineMarkerResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyMarkerLabel => "empty_marker_label",
            Self::EmptyMarkerIdentity => "empty_marker_identity",
            Self::ForbiddenMarkerMaterial => "forbidden_marker_material",
        }
    }
}

impl fmt::Display for M5InlineMarkerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "inline result marker resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5InlineMarkerResolutionError {}

/// Resolves one inline result marker from its declared marker state.
///
/// The derived marker posture is computed in a fixed honesty-first order: a muted /
/// quarantined test wins first (its ownership and release impact head the marker), then a
/// marker that no longer maps to the current buffer, then a marker that maps only
/// approximately, then an imported result (reduced certainty, not a live-local result),
/// then a stale / outdated / expired live-local result, and otherwise a fresh, live-local,
/// exactly-mapped marker. Only that last posture may honestly imply a current local result;
/// every other posture carries reduced certainty and must degrade visibly. The marker may
/// be rerun only from a live-local result whose source still maps to the buffer, can open
/// recent attempts only when some exist, always offers reveal-evidence and export, and
/// offers review-quarantine only when the test is muted — so a source decoration never
/// implies a current local result when the evidence came from an imported or stale run, or
/// maps only approximately to the current file/cell state.
pub fn resolve_inline_result_marker(
    input: &M5InlineMarkerResolutionInput,
) -> Result<M5ResolvedInlineResultMarker, M5InlineMarkerResolutionError> {
    if input.marker_label.trim().is_empty() {
        return Err(M5InlineMarkerResolutionError::EmptyMarkerLabel);
    }
    if input.marker_identity_ref.trim().is_empty() {
        return Err(M5InlineMarkerResolutionError::EmptyMarkerIdentity);
    }
    if value_repr_is_forbidden(&input.marker_label)
        || value_repr_is_forbidden(&input.marker_identity_ref)
    {
        return Err(M5InlineMarkerResolutionError::ForbiddenMarkerMaterial);
    }

    let marker_posture = derive_marker_posture(
        input.result_origin,
        input.result_freshness,
        input.source_mapping,
        input.item_muted,
    );
    let can_rerun_from_marker = !result_origin_is_not_live_local(input.result_origin)
        && !input.source_mapping.is_unmapped();
    let can_open_recent_attempts = input.recent_attempt_count > 0;
    let available_actions = derive_marker_actions(
        can_rerun_from_marker,
        can_open_recent_attempts,
        input.item_muted,
    );

    Ok(M5ResolvedInlineResultMarker {
        verdict: input.verdict,
        failure_category: input.failure_category,
        stability_chip: input.stability_chip,
        result_origin: input.result_origin,
        result_freshness: input.result_freshness,
        source_mapping: input.source_mapping,
        target_class: input.target_class,
        environment_lane: input.environment_lane,
        attempt_lineage: input.attempt_lineage,
        quarantine_ownership: input.quarantine_ownership,
        release_impact: input.release_impact,
        recent_attempt_count: input.recent_attempt_count,
        marker_label: input.marker_label.clone(),
        marker_identity_ref: input.marker_identity_ref.clone(),
        marker_posture,
        available_actions,
        can_rerun_from_marker,
        can_open_recent_attempts,
        is_muted: input.item_muted,
        shows_live_certainty: marker_posture.shows_live_certainty(),
        carries_reduced_certainty: marker_posture.carries_reduced_certainty(),
        implies_current_local_result: marker_posture.shows_live_certainty(),
        needs_attention: marker_posture.needs_attention(),
    })
}

/// The fixed honesty-first marker-posture ladder.
fn derive_marker_posture(
    result_origin: M5TestResultOrigin,
    result_freshness: M5TestResultFreshness,
    source_mapping: M5MarkerSourceMapping,
    item_muted: bool,
) -> M5InlineMarkerPosture {
    use M5InlineMarkerPosture as Posture;
    if item_muted {
        Posture::QuarantinedMarker
    } else if source_mapping.is_unmapped() {
        Posture::UnmappedMarker
    } else if source_mapping.is_approximate() {
        Posture::ApproximateMappingMarker
    } else if result_origin_is_not_live_local(result_origin) {
        Posture::ImportedEvidenceMarker
    } else if freshness_is_stale(result_freshness) {
        Posture::StaleResultMarker
    } else {
        Posture::LiveLocalMarker
    }
}

/// Derives the bounded action set from the rerun / recent-attempts / mute signals.
///
/// Reveal-evidence is always offered so the verdict, origin, freshness, mapping, and
/// target/environment are always inspectable; open-recent-attempts is offered only when
/// some recent attempts exist; rerun-from-marker is offered only when the result is a
/// live-local run that still maps to the buffer; review-quarantine is offered only when the
/// test is muted; export-marker is always offered.
fn derive_marker_actions(
    can_rerun_from_marker: bool,
    can_open_recent_attempts: bool,
    item_muted: bool,
) -> Vec<M5InlineMarkerAction> {
    use M5InlineMarkerAction as Action;
    let mut actions = vec![Action::RevealMarkerEvidence];
    if can_open_recent_attempts {
        actions.push(Action::OpenRecentAttempts);
    }
    if can_rerun_from_marker {
        actions.push(Action::RerunFromMarker);
    }
    if item_muted {
        actions.push(Action::ReviewQuarantine);
    }
    actions.push(Action::ExportMarker);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked inline-result-marker resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerResolutionCase {
    /// The resolver input.
    pub input: M5InlineMarkerResolutionInput,
    /// The resolved truth. Must equal `resolve_inline_result_marker(&input)`.
    pub resolved: M5ResolvedInlineResultMarker,
}

impl M5InlineMarkerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5InlineMarkerResolutionInput) -> Self {
        let resolved = resolve_inline_result_marker(&input).expect("seed marker case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_inline_result_marker(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved marker identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.marker_identity_ref == self.input.marker_identity_ref
            && self.resolved.marker_label == self.input.marker_label
    }
}

/// One row in the primitive matrix: one editor / notebook consumer bound to the shared
/// marker anatomy, verdicts, stability chips, result origins, freshness states, source
/// mappings, marker postures, attempt lineage kinds, bounded actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerConsumerRow {
    /// Editor / notebook consumer family.
    pub consumer_surface: M5InlineMarkerConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume this marker.
    pub surface_families: Vec<M5TestSurfaceFamily>,
    /// Deployment lines this marker keeps the same truth across.
    pub deployment_lines: Vec<M5TestDeploymentLine>,
    /// Anatomy parts this marker renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5InlineMarkerAnatomyPart>,
    /// Marker verdicts this consumer distinguishes.
    pub marker_verdicts: Vec<M5InlineMarkerVerdict>,
    /// Stability chips this consumer distinguishes.
    pub stability_chips: Vec<M5MarkerStabilityChip>,
    /// Result origins this consumer distinguishes.
    pub result_origins: Vec<M5TestResultOrigin>,
    /// Result freshness states this consumer distinguishes.
    pub result_freshness: Vec<M5TestResultFreshness>,
    /// Source-mapping fidelities this consumer distinguishes.
    pub source_mappings: Vec<M5MarkerSourceMapping>,
    /// Marker postures this consumer distinguishes.
    pub marker_postures: Vec<M5InlineMarkerPosture>,
    /// Attempt lineage kinds this consumer distinguishes.
    pub attempt_lineage_kinds: Vec<M5AttemptLineageKind>,
    /// Bounded marker actions this consumer offers.
    pub marker_actions: Vec<M5InlineMarkerAction>,
    /// Export fields this marker carries (must include the mandatory fields).
    pub export_fields: Vec<M5InlineMarkerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestAccessibilityRoute>,
    /// Test / triage subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestDowngradeTrigger>,
    /// Proof packet refs that keep this marker current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this marker.
    pub source_contract_refs: Vec<String>,
    /// Worked marker resolutions proving the resolver on this consumer.
    pub marker_examples: Vec<M5InlineMarkerResolutionCase>,
    /// Hard invariant: this consumer never masks its verdict or imported/live origin. MUST
    /// be `false`.
    pub masks_verdict_or_origin: bool,
    /// Hard invariant: this consumer never hides a quarantine's release impact. MUST be
    /// `false`.
    pub hides_quarantine_release_impact: bool,
    /// Hard invariant: this consumer never renders an imported, stale, or
    /// approximately-mapped run as a current live-local result. MUST be `false`.
    pub overstates_imported_or_stale_as_live: bool,
    /// Hard invariant: this consumer never drops the attempt lineage a tree or triage
    /// consumer would show. MUST be `false`.
    pub drops_attempt_lineage: bool,
}

impl M5InlineMarkerConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5InlineMarkerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5InlineMarkerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5InlineMarkerExportField> =
            self.export_fields.iter().copied().collect();
        M5InlineMarkerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_verdict_or_origin
            && !self.hides_quarantine_release_impact
            && !self.overstates_imported_or_stale_as_live
            && !self.drops_attempt_lineage
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerVocabularySet {
    /// Editor / notebook consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Marker-posture tokens.
    pub marker_postures: Vec<String>,
    /// Source-mapping tokens.
    pub source_mappings: Vec<String>,
    /// Stability-chip tokens.
    pub stability_chips: Vec<String>,
    /// Marker-action tokens.
    pub marker_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Marker-verdict tokens (reused from the frozen matrix).
    pub marker_verdicts: Vec<String>,
    /// Failure-category tokens (reused from the frozen matrix).
    pub failure_categories: Vec<String>,
    /// Result-origin tokens (reused from the frozen matrix).
    pub result_origins: Vec<String>,
    /// Result-freshness tokens (reused from the frozen matrix).
    pub result_freshness: Vec<String>,
    /// Attempt-lineage tokens (reused from the frozen matrix).
    pub attempt_lineage_kinds: Vec<String>,
    /// Target-class tokens (reused from the frozen matrix).
    pub target_classes: Vec<String>,
    /// Environment-lane tokens (reused from the frozen matrix).
    pub environment_lanes: Vec<String>,
    /// Quarantine-ownership tokens (reused from the frozen matrix).
    pub quarantine_ownership_classes: Vec<String>,
    /// Release-impact tokens (reused from the frozen matrix).
    pub release_impacts: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5InlineMarkerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5InlineMarkerConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5InlineMarkerAnatomyPart::ALL, |v| v.as_str()),
            marker_postures: tokens(&M5InlineMarkerPosture::ALL, |v| v.as_str()),
            source_mappings: tokens(&M5MarkerSourceMapping::ALL, |v| v.as_str()),
            stability_chips: tokens(&M5MarkerStabilityChip::ALL, |v| v.as_str()),
            marker_actions: tokens(&M5InlineMarkerAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5InlineMarkerExportField::ALL, |v| v.as_str()),
            marker_verdicts: tokens(&M5InlineMarkerVerdict::ALL, |v| v.as_str()),
            failure_categories: tokens(&M5FailureCategory::ALL, |v| v.as_str()),
            result_origins: tokens(&M5TestResultOrigin::ALL, |v| v.as_str()),
            result_freshness: tokens(&M5TestResultFreshness::ALL, |v| v.as_str()),
            attempt_lineage_kinds: tokens(&M5AttemptLineageKind::ALL, |v| v.as_str()),
            target_classes: tokens(&M5TestTargetClass::ALL, |v| v.as_str()),
            environment_lanes: tokens(&M5TestEnvironmentLane::ALL, |v| v.as_str()),
            quarantine_ownership_classes: tokens(&M5QuarantineOwnership::ALL, |v| v.as_str()),
            release_impacts: tokens(&M5TestReleaseImpact::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5InlineMarkerGovernanceReview {
    /// The marker shows its pass/fail/error/timeout verdict state.
    pub marker_shows_verdict_state: bool,
    /// The marker shows its stability-or-flaky chip.
    pub marker_shows_stability_chip: bool,
    /// The marker shows its imported/live origin class and last-result freshness.
    pub marker_shows_origin_class_and_freshness: bool,
    /// The marker shows its target / environment shorthand.
    pub marker_shows_target_and_environment: bool,
    /// The marker shows how faithfully it maps to the current file/cell state.
    pub marker_shows_source_mapping: bool,
    /// The marker exposes an open-recent-attempts action over its attempt lineage.
    pub marker_exposes_recent_attempts: bool,
    /// The marker shows its mute / quarantine state and release impact.
    pub marker_shows_mute_and_release_impact: bool,
    /// Imported, stale, and approximately-mapped runs never read as a current local result.
    pub imported_or_stale_never_reads_as_live: bool,
    /// Editor and notebook markers keep parity with the tree and triage consumers on state
    /// labels and attempt lineage.
    pub markers_keep_parity_with_tree_and_triage: bool,
    /// Markers keep the same truth across every deployment line.
    pub markers_stable_across_deployment_lines: bool,
    /// Markers keep the same truth across editor, notebook, headless/export, and report
    /// consumers.
    pub markers_stable_across_consumer_surfaces: bool,
    /// Every marker declares a non-visual accessibility route.
    pub every_marker_declares_accessibility_route: bool,
    /// The support / export packet reconstructs verdict, origin, freshness, mapping, and
    /// attempt-lineage truth.
    pub support_export_reconstructs_marker_truth: bool,
    /// Later M5 markers cannot invent parallel inline-marker vocabulary.
    pub later_markers_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerConsumerProjection {
    /// Editor and notebook surfaces consume the shared marker vocabulary.
    pub editor_and_notebook_surfaces_consume_marker_vocabulary: bool,
    /// The marker-posture resolver reads a single canonical source.
    pub marker_posture_reads_single_source: bool,
    /// The tree and triage consumers read the same state labels and attempt lineage.
    pub tree_and_triage_read_same_labels: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop markers read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the inline marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineMarkerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InlineResultMarkerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InlineResultMarkerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Editor / notebook rows.
    pub rows: Vec<M5InlineMarkerConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InlineMarkerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InlineMarkerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InlineMarkerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InlineMarkerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InlineMarkerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 inline-result-marker primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InlineResultMarkerPacket {
    /// Record kind; must equal [`M5_INLINE_RESULT_MARKER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INLINE_RESULT_MARKER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Editor / notebook rows.
    pub rows: Vec<M5InlineMarkerConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InlineMarkerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InlineMarkerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InlineMarkerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InlineMarkerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InlineMarkerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InlineResultMarkerPacket {
    /// Builds an M5 inline-marker-primitive packet from stable-lane input.
    pub fn new(input: M5InlineResultMarkerPacketInput) -> Self {
        Self {
            record_kind: M5_INLINE_RESULT_MARKER_RECORD_KIND.to_owned(),
            schema_version: M5_INLINE_RESULT_MARKER_SCHEMA_VERSION,
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

    /// Validates the M5 inline-marker-primitive invariants.
    pub fn validate(&self) -> Vec<M5InlineMarkerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INLINE_RESULT_MARKER_RECORD_KIND {
            violations.push(M5InlineMarkerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INLINE_RESULT_MARKER_SCHEMA_VERSION {
            violations.push(M5InlineMarkerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InlineMarkerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_mapping_coverage(self, &mut violations);
        validate_certainty_coverage(self, &mut violations);
        validate_recent_attempts_coverage(self, &mut violations);
        validate_quarantine_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 inline marker primitive packet serializes"),
        ) {
            violations.push(M5InlineMarkerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 inline marker primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per editor / notebook consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,verdicts,result_origins,marker_postures,source_mappings,marker_actions,marker_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.marker_verdicts, |v| v.as_str()),
                join_tokens(&row.result_origins, |v| v.as_str()),
                join_tokens(&row.marker_postures, |v| v.as_str()),
                join_tokens(&row.source_mappings, |v| v.as_str()),
                join_tokens(&row.marker_actions, |v| v.as_str()),
                row.marker_examples.len(),
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
        out.push_str("# M5 Inline-Result-Marker Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Editor / notebook consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Marker postures: {}\n",
            self.vocabulary_set.marker_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Source mappings: {}\n",
            self.vocabulary_set.source_mappings.join(", ")
        ));
        out.push_str(&format!(
            "- Stability chips: {}\n",
            self.vocabulary_set.stability_chips.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Editor / notebook consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked markers: {}\n",
                row.marker_examples.len()
            ));
            for case in &row.marker_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (mapping `{}`, live-certainty `{}`, muted `{}`)\n",
                    case.resolved.marker_identity_ref,
                    case.resolved.verdict.as_str(),
                    case.resolved.result_origin.as_str(),
                    case.resolved.marker_posture.as_str(),
                    case.resolved.source_mapping.as_str(),
                    case.resolved.shows_live_certainty,
                    case.resolved.is_muted,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 inline-marker-primitive export.
#[derive(Debug)]
pub enum M5InlineMarkerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InlineMarkerViolation>),
}

impl fmt::Display for M5InlineMarkerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 inline marker primitive export parse failed: {error}"
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
                    "m5 inline marker primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InlineMarkerArtifactError {}

/// Validation failures emitted by [`M5InlineResultMarkerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InlineMarkerViolation {
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
    /// A required editor / notebook consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// An editor / notebook row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked marker resolutions.
    MarkerExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every source-mapping fidelity.
    MappingCoverageUnproven,
    /// The worked resolutions do not prove both a live-certainty and an imported/stale/
    /// approximate reduced-certainty marker.
    CertaintyCoverageUnproven,
    /// The worked resolutions do not prove both a marker offering open-recent-attempts and
    /// one withholding it.
    RecentAttemptsCoverageUnproven,
    /// The worked resolutions do not prove both a muted and a non-muted marker.
    QuarantineCoverageUnproven,
    /// A worked resolution does not preserve its exact marker identity and label.
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

impl M5InlineMarkerViolation {
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
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::MarkerExampleMissing => "marker_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::MappingCoverageUnproven => "mapping_coverage_unproven",
            Self::CertaintyCoverageUnproven => "certainty_coverage_unproven",
            Self::RecentAttemptsCoverageUnproven => "recent_attempts_coverage_unproven",
            Self::QuarantineCoverageUnproven => "quarantine_coverage_unproven",
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

/// Reads and validates the checked-in stable M5 inline-marker-primitive export.
pub fn current_stable_m5_inline_result_marker_export(
) -> Result<M5InlineResultMarkerPacket, M5InlineMarkerArtifactError> {
    let packet: M5InlineResultMarkerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-inline-result-marker-primitive-proof/support_export.json"
    )))
    .map_err(M5InlineMarkerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InlineMarkerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INLINE_RESULT_MARKER_SCHEMA_REF,
        M5_INLINE_RESULT_MARKER_DOC_REF,
        M5_INLINE_RESULT_MARKER_COMPONENT_MATRIX_REF,
        M5_INLINE_RESULT_MARKER_TEST_ITEM_IDENTITY_REF,
        M5_INLINE_RESULT_MARKER_TEST_ATTEMPT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InlineMarkerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5InlineMarkerViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let present: BTreeSet<M5InlineMarkerConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5InlineMarkerConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5InlineMarkerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.marker_verdicts.is_empty()
            || row.stability_chips.is_empty()
            || row.result_origins.is_empty()
            || row.result_freshness.is_empty()
            || row.source_mappings.is_empty()
            || row.marker_postures.is_empty()
            || row.attempt_lineage_kinds.is_empty()
            || row.marker_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5InlineMarkerViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5InlineMarkerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5InlineMarkerViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5InlineMarkerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5InlineMarkerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5InlineMarkerViolation::DowngradeTriggersMissing);
        }
        if row.marker_examples.is_empty() {
            violations.push(M5InlineMarkerViolation::MarkerExampleMissing);
        }
        if row
            .marker_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5InlineMarkerViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5InlineMarkerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5InlineMarkerViolation::RowInvariantViolated);
        }
    }
}

/// Every source-mapping fidelity must be exercised by some worked resolution — the
/// implementation requirement that a marker degrades visibly when it maps only approximately
/// (or not at all) to the current file/cell state.
fn validate_mapping_coverage(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let exercised: BTreeSet<M5MarkerSourceMapping> = packet
        .rows
        .iter()
        .flat_map(|row| row.marker_examples.iter())
        .map(|case| case.resolved.source_mapping)
        .collect();
    let covered = M5MarkerSourceMapping::ALL
        .iter()
        .all(|mapping| exercised.contains(mapping));
    if !covered {
        violations.push(M5InlineMarkerViolation::MappingCoverageUnproven);
    }
}

/// At least one worked resolution must prove a live-certainty marker and at least one must
/// prove an imported / stale / approximate reduced-certainty marker — the acceptance-
/// criterion example that a source decoration no longer implies a current local result when
/// the evidence came from an imported or stale run.
fn validate_certainty_coverage(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let has_live = packet.rows.iter().any(|row| {
        row.marker_examples.iter().any(|case| {
            case.resolved.shows_live_certainty
                && case.resolved.implies_current_local_result
                && !case.resolved.carries_reduced_certainty
        })
    });
    let has_reduced = packet.rows.iter().any(|row| {
        row.marker_examples.iter().any(|case| {
            case.resolved.carries_reduced_certainty
                && !case.resolved.shows_live_certainty
                && !case.resolved.implies_current_local_result
        })
    });
    if !(has_live && has_reduced) {
        violations.push(M5InlineMarkerViolation::CertaintyCoverageUnproven);
    }
}

/// At least one worked resolution must prove a marker offering the open-recent-attempts
/// action and at least one must prove a marker withholding it (no recent attempts) — the
/// implementation requirement that the open-recent-attempts action is present exactly when
/// there is attempt lineage to open.
fn validate_recent_attempts_coverage(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let has_attempts = packet.rows.iter().any(|row| {
        row.marker_examples.iter().any(|case| {
            case.resolved.can_open_recent_attempts
                && case
                    .resolved
                    .available_actions
                    .contains(&M5InlineMarkerAction::OpenRecentAttempts)
        })
    });
    let has_no_attempts = packet.rows.iter().any(|row| {
        row.marker_examples.iter().any(|case| {
            !case.resolved.can_open_recent_attempts
                && !case
                    .resolved
                    .available_actions
                    .contains(&M5InlineMarkerAction::OpenRecentAttempts)
        })
    });
    if !(has_attempts && has_no_attempts) {
        violations.push(M5InlineMarkerViolation::RecentAttemptsCoverageUnproven);
    }
}

/// At least one worked resolution must prove a muted marker (offering review-quarantine) and
/// at least one must prove a non-muted marker — the implementation requirement that
/// mute/quarantine state and its release impact are never left implicit.
fn validate_quarantine_coverage(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let has_muted = packet.rows.iter().any(|row| {
        row.marker_examples.iter().any(|case| {
            case.resolved.is_muted
                && case
                    .resolved
                    .available_actions
                    .contains(&M5InlineMarkerAction::ReviewQuarantine)
        })
    });
    let has_unmuted = packet.rows.iter().any(|row| {
        row.marker_examples
            .iter()
            .any(|case| !case.resolved.is_muted)
    });
    if !(has_muted && has_unmuted) {
        violations.push(M5InlineMarkerViolation::QuarantineCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact marker identity and label — the invariant
/// that the inline marker never rewrites the user's test identity.
fn validate_identity_preservation(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.marker_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5InlineMarkerViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.marker_shows_verdict_state,
        review.marker_shows_stability_chip,
        review.marker_shows_origin_class_and_freshness,
        review.marker_shows_target_and_environment,
        review.marker_shows_source_mapping,
        review.marker_exposes_recent_attempts,
        review.marker_shows_mute_and_release_impact,
        review.imported_or_stale_never_reads_as_live,
        review.markers_keep_parity_with_tree_and_triage,
        review.markers_stable_across_deployment_lines,
        review.markers_stable_across_consumer_surfaces,
        review.every_marker_declares_accessibility_route,
        review.support_export_reconstructs_marker_truth,
        review.later_markers_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5InlineMarkerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_and_notebook_surfaces_consume_marker_vocabulary,
        projection.marker_posture_reads_single_source,
        projection.tree_and_triage_read_same_labels,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5InlineMarkerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InlineMarkerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InlineResultMarkerPacket,
    violations: &mut Vec<M5InlineMarkerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InlineMarkerViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
