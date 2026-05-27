//! Artifact-manager, preview/runtime inspector, and evidence-export
//! truth packet for the M4 stable lane.
//!
//! This module pins how the artifact-manager, preview/runtime
//! inspector, signal-slice, and evidence-export lanes serialize one
//! canonical truth across the four wedges
//! (`artifact_chronology_replay_truth`, `signal_slice_identity_truth`,
//! `evidence_export_review_truth`,
//! `cross_surface_evidence_lineage_truth`) so the artifact manager,
//! preview/runtime inspectors, evidence-export sheet, CLI/headless
//! inspector, support export bundle, Help/About proof card, and
//! conformance dashboard all read one evidence-export truth. Surfaces
//! MUST NOT mint local copies, paraphrase slice freshness, collapse
//! replay/chronology into a single `recorded`/`not_recorded` bit,
//! mistake exported copies for live runtime truth, or treat logs /
//! traces / test artifacts as pane-local blobs.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `evidence_export_quality` row cannot prove:
//!
//! - the four wedges each have a structured `wedge_admission` row so
//!   reviewers can audit chronology, slice identity, export-review,
//!   and cross-surface lineage posture without support-only knowledge,
//! - the four signal-slice kinds (`logs_slice`, `metrics_slice`,
//!   `traces_slice`, `test_artifact_slice`) each have a structured
//!   `signal_slice_kind_admission` row so the lane discloses every
//!   slice kind it serves,
//! - the six slice-freshness classes (`live_stream`,
//!   `buffered_replay`, `cached_snapshot`, `imported_evidence`,
//!   `truncated_view`, `exported_copy`) each have a structured
//!   `slice_freshness_admission` row so users never mistake an
//!   exported copy or a cached snapshot for live runtime truth,
//! - the five replay-chronology states (`recorded`, `not_recorded`,
//!   `unsupported`, `restart_with_recording_available`,
//!   `partially_recorded`) each have a structured
//!   `replay_chronology_admission` row so chronology posture stays
//!   explicit through restore, replay, and reopen,
//! - the five retention classes (`session_only_retention`,
//!   `session_plus_window_retention`, `policy_bounded_retention`,
//!   `archived_retention`, `imported_external_retention`) each have a
//!   structured `retention_class_admission` row so reviewers can audit
//!   retention posture without support-only knowledge,
//! - each of the seven consumer surfaces (artifact manager, preview /
//!   runtime inspector, evidence-export sheet, CLI/headless inspect,
//!   support export, Help/About, conformance dashboard) has a
//!   `consumer_surface_binding` row attesting it reads this packet
//!   verbatim,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every emitted artifact, signal slice, evidence
//!   export, and support packet via a `lineage_admission` row.
//!
//! Every row binds closed `evidence_export_lane_class`,
//! `evidence_export_row_class`, `support_class`, `wedge_class`,
//! `signal_slice_kind_class`, `slice_freshness_class`,
//! `replay_chronology_state_class`, `retention_class`,
//! `consumer_surface_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `confidence_class` vocabularies
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch-stable, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! log bodies, raw trace payloads, raw test-artifact bytes, raw
//! command lines, raw process environment bytes, secrets, or ambient
//! credentials past the boundary. A row that claims `launch_stable`
//! while leaving its known limit, downgrade automation, or evidence
//! class unbound is refused; the validator narrows below launch-stable
//! instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`EvidenceExportTruthPacket`].
pub const EVIDENCE_EXPORT_TRUTH_PACKET_RECORD_KIND: &str =
    "stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_stable_packet";

/// Stable record-kind tag for [`EvidenceExportTruthSupportExport`].
pub const EVIDENCE_EXPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_support_export";

/// Integer schema version for the evidence-export truth packet.
pub const EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EVIDENCE_EXPORT_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const EVIDENCE_EXPORT_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/stabilize-the-artifact-manager-preview-runtime-inspectors-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const EVIDENCE_EXPORT_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-the-artifact-manager-preview-runtime-inspectors-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const EVIDENCE_EXPORT_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and";

/// Repo-relative path of the checked-in stable packet.
pub const EVIDENCE_EXPORT_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json";

/// Closed evidence-export lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceExportLaneClass {
    /// Artifact-manager lane: chronology, replay packets, capture-vs-
    /// no-capture state, timeline bookmarks, crash-viewer compare
    /// cards, and redaction/export sheets owned by the artifact
    /// manager.
    ArtifactManagerLane,
    /// Preview / runtime inspector lane: panes that render logs,
    /// metrics, traces, and test-artifact slices.
    PreviewRuntimeInspectorLane,
    /// Signal-slice lane: typed slice objects with source identity,
    /// target scope, freshness, time window, sample / truncation
    /// posture, and incident-timeline refs.
    SignalSliceLane,
    /// Evidence-export lane: sheets reviewing included channels,
    /// problems, artifacts, mapping refs, and redaction profile before
    /// share/open actions.
    EvidenceExportLane,
}

impl EvidenceExportLaneClass {
    /// Every required evidence-export lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::ArtifactManagerLane,
        Self::PreviewRuntimeInspectorLane,
        Self::SignalSliceLane,
        Self::EvidenceExportLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactManagerLane => "artifact_manager_lane",
            Self::PreviewRuntimeInspectorLane => "preview_runtime_inspector_lane",
            Self::SignalSliceLane => "signal_slice_lane",
            Self::EvidenceExportLane => "evidence_export_lane",
        }
    }
}

/// Closed row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceExportRowClass {
    /// The lane's headline evidence-export qualification row.
    EvidenceExportQuality,
    /// A row admitting one of the four required wedges.
    WedgeAdmission,
    /// A row admitting one of the four signal-slice kinds.
    SignalSliceKindAdmission,
    /// A row admitting one of the six slice-freshness classes.
    SliceFreshnessAdmission,
    /// A row admitting one of the five replay-chronology states.
    ReplayChronologyAdmission,
    /// A row admitting one of the five retention classes.
    RetentionClassAdmission,
    /// A row binding one consumer surface.
    ConsumerSurfaceBinding,
    /// A row binding the stable `execution_context_id` lineage.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl EvidenceExportRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceExportQuality => "evidence_export_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::SignalSliceKindAdmission => "signal_slice_kind_admission",
            Self::SliceFreshnessAdmission => "slice_freshness_admission",
            Self::ReplayChronologyAdmission => "replay_chronology_admission",
            Self::RetentionClassAdmission => "retention_class_admission",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    pub const fn requires_signal_slice_kind(self) -> bool {
        matches!(self, Self::SignalSliceKindAdmission)
    }

    pub const fn requires_slice_freshness(self) -> bool {
        matches!(self, Self::SliceFreshnessAdmission)
    }

    pub const fn requires_replay_chronology_state(self) -> bool {
        matches!(self, Self::ReplayChronologyAdmission)
    }

    pub const fn requires_retention_class(self) -> bool {
        matches!(self, Self::RetentionClassAdmission)
    }

    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to an evidence-export row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    LaunchStable,
    LaunchStableBelow,
    BetaGradeOnly,
    PreviewOnly,
    Unsupported,
    SupportUnbound,
}

impl SupportClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed wedge vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    ArtifactChronologyReplayTruth,
    SignalSliceIdentityTruth,
    EvidenceExportReviewTruth,
    CrossSurfaceEvidenceLineageTruth,
    NotApplicable,
}

impl WedgeClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::ArtifactChronologyReplayTruth,
        Self::SignalSliceIdentityTruth,
        Self::EvidenceExportReviewTruth,
        Self::CrossSurfaceEvidenceLineageTruth,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactChronologyReplayTruth => "artifact_chronology_replay_truth",
            Self::SignalSliceIdentityTruth => "signal_slice_identity_truth",
            Self::EvidenceExportReviewTruth => "evidence_export_review_truth",
            Self::CrossSurfaceEvidenceLineageTruth => "cross_surface_evidence_lineage_truth",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed signal-slice kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalSliceKindClass {
    LogsSlice,
    MetricsSlice,
    TracesSlice,
    TestArtifactSlice,
    NotApplicable,
}

impl SignalSliceKindClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::LogsSlice,
        Self::MetricsSlice,
        Self::TracesSlice,
        Self::TestArtifactSlice,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogsSlice => "logs_slice",
            Self::MetricsSlice => "metrics_slice",
            Self::TracesSlice => "traces_slice",
            Self::TestArtifactSlice => "test_artifact_slice",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed slice-freshness vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SliceFreshnessClass {
    LiveStream,
    BufferedReplay,
    CachedSnapshot,
    ImportedEvidence,
    TruncatedView,
    ExportedCopy,
    NotApplicable,
}

impl SliceFreshnessClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::LiveStream,
        Self::BufferedReplay,
        Self::CachedSnapshot,
        Self::ImportedEvidence,
        Self::TruncatedView,
        Self::ExportedCopy,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveStream => "live_stream",
            Self::BufferedReplay => "buffered_replay",
            Self::CachedSnapshot => "cached_snapshot",
            Self::ImportedEvidence => "imported_evidence",
            Self::TruncatedView => "truncated_view",
            Self::ExportedCopy => "exported_copy",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed replay-chronology state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayChronologyStateClass {
    Recorded,
    NotRecorded,
    Unsupported,
    RestartWithRecordingAvailable,
    PartiallyRecorded,
    NotApplicable,
}

impl ReplayChronologyStateClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Recorded,
        Self::NotRecorded,
        Self::Unsupported,
        Self::RestartWithRecordingAvailable,
        Self::PartiallyRecorded,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
            Self::NotRecorded => "not_recorded",
            Self::Unsupported => "unsupported",
            Self::RestartWithRecordingAvailable => "restart_with_recording_available",
            Self::PartiallyRecorded => "partially_recorded",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed retention-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    SessionOnlyRetention,
    SessionPlusWindowRetention,
    PolicyBoundedRetention,
    ArchivedRetention,
    ImportedExternalRetention,
    NotApplicable,
}

impl RetentionClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::SessionOnlyRetention,
        Self::SessionPlusWindowRetention,
        Self::PolicyBoundedRetention,
        Self::ArchivedRetention,
        Self::ImportedExternalRetention,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOnlyRetention => "session_only_retention",
            Self::SessionPlusWindowRetention => "session_plus_window_retention",
            Self::PolicyBoundedRetention => "policy_bounded_retention",
            Self::ArchivedRetention => "archived_retention",
            Self::ImportedExternalRetention => "imported_external_retention",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed consumer-surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    ArtifactManagerSurface,
    PreviewRuntimeInspectorSurface,
    EvidenceExportSheetSurface,
    CliHeadlessInspect,
    SupportExport,
    HelpAbout,
    ConformanceDashboard,
    NotApplicable,
}

impl ConsumerSurfaceClass {
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 7] = [
        Self::ArtifactManagerSurface,
        Self::PreviewRuntimeInspectorSurface,
        Self::EvidenceExportSheetSurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactManagerSurface => "artifact_manager_surface",
            Self::PreviewRuntimeInspectorSurface => "preview_runtime_inspector_surface",
            Self::EvidenceExportSheetSurface => "evidence_export_sheet_surface",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    AutomatedFunctionalEvidence,
    ConformanceSuiteEvidence,
    FailureRecoveryDrillEvidence,
    DesignQaEvidence,
    ReleaseEvidenceReview,
    FixtureRepoEvidence,
    DocsDisclosureEvidence,
    EvidenceUnbound,
}

impl EvidenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    NoneDeclared,
    ArtifactManagerSubsetOnly,
    PreviewRuntimeInspectorSubsetOnly,
    SignalSliceSubsetOnly,
    EvidenceExportSubsetOnly,
    WedgeSubsetOnly,
    SignalSliceKindSubsetOnly,
    SliceFreshnessSubsetOnly,
    ReplayChronologyStateSubsetOnly,
    RetentionClassSubsetOnly,
    ConsumerSurfaceSubsetOnly,
    ImportedProviderOnly,
    BetaCapabilitySampleOnly,
    LimitUnbound,
}

impl KnownLimitClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::ArtifactManagerSubsetOnly => "artifact_manager_subset_only",
            Self::PreviewRuntimeInspectorSubsetOnly => "preview_runtime_inspector_subset_only",
            Self::SignalSliceSubsetOnly => "signal_slice_subset_only",
            Self::EvidenceExportSubsetOnly => "evidence_export_subset_only",
            Self::WedgeSubsetOnly => "wedge_subset_only",
            Self::SignalSliceKindSubsetOnly => "signal_slice_kind_subset_only",
            Self::SliceFreshnessSubsetOnly => "slice_freshness_subset_only",
            Self::ReplayChronologyStateSubsetOnly => "replay_chronology_state_subset_only",
            Self::RetentionClassSubsetOnly => "retention_class_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::ImportedProviderOnly => "imported_provider_only",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    None,
    AutoNarrowOnSignalSliceKindGap,
    AutoNarrowOnSliceFreshnessGap,
    AutoNarrowOnReplayChronologyGap,
    AutoNarrowOnRetentionClassGap,
    AutoNarrowOnSignalSliceIdentityGap,
    AutoNarrowOnEvidenceExportReviewGap,
    AutoNarrowOnConsumerSurfaceGap,
    AutoNarrowOnCrossSurfaceEvidenceLineageDrift,
    AutoNarrowOnLineageBreak,
    AutoBlockOnMissingEvidence,
    ManualOnlyPendingReview,
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnSignalSliceKindGap => "auto_narrow_on_signal_slice_kind_gap",
            Self::AutoNarrowOnSliceFreshnessGap => "auto_narrow_on_slice_freshness_gap",
            Self::AutoNarrowOnReplayChronologyGap => "auto_narrow_on_replay_chronology_gap",
            Self::AutoNarrowOnRetentionClassGap => "auto_narrow_on_retention_class_gap",
            Self::AutoNarrowOnSignalSliceIdentityGap => "auto_narrow_on_signal_slice_identity_gap",
            Self::AutoNarrowOnEvidenceExportReviewGap => {
                "auto_narrow_on_evidence_export_review_gap"
            }
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnCrossSurfaceEvidenceLineageDrift => {
                "auto_narrow_on_cross_surface_evidence_lineage_drift"
            }
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for an evidence-export row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    HighConfidence,
    MediumConfidence,
    LowConfidence,
}

impl ConfidenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    Stable,
    NarrowedBelowStable,
    BlocksStable,
}

impl PromotionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Info,
    Warning,
    Blocker,
}

/// Closed validation-finding vocabulary for the evidence-export truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    WrongRecordKind,
    WrongSchemaVersion,
    MissingIdentity,
    MissingEvidenceExportLaneCoverage,
    MissingWedgeCoverage,
    MissingSignalSliceKindCoverage,
    MissingSliceFreshnessCoverage,
    MissingReplayChronologyStateCoverage,
    MissingRetentionClassCoverage,
    MissingConsumerSurfaceCoverage,
    MissingLineageAdmission,
    MissingSupportClass,
    MissingKnownLimit,
    MissingDowngradeAutomation,
    MissingEvidenceClass,
    LaunchStableWithUnboundBinding,
    NarrowedRowMissingDisclosureRef,
    KnownLimitMissingDisclosureRef,
    DowngradeAutomationMissingDisclosureRef,
    MissingEvidenceRefs,
    WedgeNotApplicable,
    WedgeNotPermittedOnRowClass,
    SignalSliceKindNotApplicable,
    SignalSliceKindNotPermittedOnRowClass,
    SliceFreshnessNotApplicable,
    SliceFreshnessNotPermittedOnRowClass,
    ReplayChronologyStateNotApplicable,
    ReplayChronologyStateNotPermittedOnRowClass,
    RetentionClassNotApplicable,
    RetentionClassNotPermittedOnRowClass,
    ConsumerSurfaceNotApplicable,
    ConsumerSurfaceNotPermittedOnRowClass,
    LineageAdmissionMissingExecutionContextId,
    CrossSurfaceEvidenceLineageNotAttested,
    RawSourceMaterialPresent,
    SecretsPresent,
    AmbientAuthorityPresent,
    MissingConsumerProjection,
    ConsumerProjectionDrift,
    LaneVocabularyCollapsed,
    RowClassVocabularyCollapsed,
    SupportClassVocabularyCollapsed,
    WedgeVocabularyCollapsed,
    SignalSliceKindVocabularyCollapsed,
    SliceFreshnessVocabularyCollapsed,
    ReplayChronologyStateVocabularyCollapsed,
    RetentionClassVocabularyCollapsed,
    ConsumerSurfaceVocabularyCollapsed,
    KnownLimitVocabularyCollapsed,
    DowngradeAutomationVocabularyCollapsed,
    EvidenceClassVocabularyCollapsed,
    PromotionStateMismatch,
}

impl FindingKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingEvidenceExportLaneCoverage => "missing_evidence_export_lane_coverage",
            Self::MissingWedgeCoverage => "missing_wedge_coverage",
            Self::MissingSignalSliceKindCoverage => "missing_signal_slice_kind_coverage",
            Self::MissingSliceFreshnessCoverage => "missing_slice_freshness_coverage",
            Self::MissingReplayChronologyStateCoverage => {
                "missing_replay_chronology_state_coverage"
            }
            Self::MissingRetentionClassCoverage => "missing_retention_class_coverage",
            Self::MissingConsumerSurfaceCoverage => "missing_consumer_surface_coverage",
            Self::MissingLineageAdmission => "missing_lineage_admission",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::SignalSliceKindNotApplicable => "signal_slice_kind_not_applicable",
            Self::SignalSliceKindNotPermittedOnRowClass => {
                "signal_slice_kind_not_permitted_on_row_class"
            }
            Self::SliceFreshnessNotApplicable => "slice_freshness_not_applicable",
            Self::SliceFreshnessNotPermittedOnRowClass => {
                "slice_freshness_not_permitted_on_row_class"
            }
            Self::ReplayChronologyStateNotApplicable => "replay_chronology_state_not_applicable",
            Self::ReplayChronologyStateNotPermittedOnRowClass => {
                "replay_chronology_state_not_permitted_on_row_class"
            }
            Self::RetentionClassNotApplicable => "retention_class_not_applicable",
            Self::RetentionClassNotPermittedOnRowClass => {
                "retention_class_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::CrossSurfaceEvidenceLineageNotAttested => {
                "cross_surface_evidence_lineage_not_attested"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::SignalSliceKindVocabularyCollapsed => "signal_slice_kind_vocabulary_collapsed",
            Self::SliceFreshnessVocabularyCollapsed => "slice_freshness_vocabulary_collapsed",
            Self::ReplayChronologyStateVocabularyCollapsed => {
                "replay_chronology_state_vocabulary_collapsed"
            }
            Self::RetentionClassVocabularyCollapsed => "retention_class_vocabulary_collapsed",
            Self::ConsumerSurfaceVocabularyCollapsed => "consumer_surface_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the evidence-export packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerProjectionSurface {
    ArtifactManagerSurface,
    PreviewRuntimeInspectorSurface,
    EvidenceExportSheetSurface,
    CliHeadlessInspect,
    SupportExport,
    HelpAbout,
    ConformanceDashboard,
}

impl ConsumerProjectionSurface {
    pub const REQUIRED: [Self; 7] = [
        Self::ArtifactManagerSurface,
        Self::PreviewRuntimeInspectorSurface,
        Self::EvidenceExportSheetSurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactManagerSurface => "artifact_manager_surface",
            Self::PreviewRuntimeInspectorSurface => "preview_runtime_inspector_surface",
            Self::EvidenceExportSheetSurface => "evidence_export_sheet_surface",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub finding_kind: FindingKind,
    pub severity: FindingSeverity,
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One evidence-export truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportRow {
    pub row_id: String,
    pub lane_class: EvidenceExportLaneClass,
    pub row_class: EvidenceExportRowClass,
    pub support_class: SupportClass,
    pub wedge_class: WedgeClass,
    pub signal_slice_kind_class: SignalSliceKindClass,
    pub slice_freshness_class: SliceFreshnessClass,
    pub replay_chronology_state_class: ReplayChronologyStateClass,
    pub retention_class: RetentionClass,
    pub consumer_surface_class: ConsumerSurfaceClass,
    pub evidence_class: EvidenceClass,
    pub known_limit_class: KnownLimitClass,
    pub downgrade_automation_class: DowngradeAutomationClass,
    pub confidence_class: ConfidenceClass,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    #[serde(default)]
    pub cross_surface_evidence_lineage_attested: bool,
    pub raw_source_material_excluded: bool,
    pub secrets_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub captured_at: String,
}

impl EvidenceExportRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    pub consumer_surface: ConsumerProjectionSurface,
    pub projection_ref: String,
    pub evidence_export_packet_id_ref: String,
    pub rendered_at: String,
    pub preserves_same_packet: bool,
    pub preserves_lane_vocabulary: bool,
    pub preserves_row_class_vocabulary: bool,
    pub preserves_support_class_vocabulary: bool,
    pub preserves_wedge_vocabulary: bool,
    pub preserves_signal_slice_kind_vocabulary: bool,
    pub preserves_slice_freshness_vocabulary: bool,
    pub preserves_replay_chronology_state_vocabulary: bool,
    pub preserves_retention_class_vocabulary: bool,
    pub preserves_consumer_surface_vocabulary: bool,
    pub preserves_known_limit_vocabulary: bool,
    pub preserves_downgrade_automation_vocabulary: bool,
    pub preserves_evidence_class_vocabulary: bool,
    pub supports_json_export: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
}

impl ConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.evidence_export_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_signal_slice_kind_vocabulary
            && self.preserves_slice_freshness_vocabulary
            && self.preserves_replay_chronology_state_vocabulary
            && self.preserves_retention_class_vocabulary
            && self.preserves_consumer_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`EvidenceExportTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportTruthPacketInput {
    pub packet_id: String,
    pub workflow_or_surface_id: String,
    pub generated_at: String,
    #[serde(default)]
    pub covered_lanes: Vec<EvidenceExportLaneClass>,
    #[serde(default)]
    pub rows: Vec<EvidenceExportRow>,
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying artifact-manager / preview-runtime
/// inspector / evidence-export at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportTruthPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub workflow_or_surface_id: String,
    pub generated_at: String,
    #[serde(default)]
    pub covered_lanes: Vec<EvidenceExportLaneClass>,
    #[serde(default)]
    pub rows: Vec<EvidenceExportRow>,
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    pub promotion_state: PromotionState,
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl EvidenceExportTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: EvidenceExportTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: EVIDENCE_EXPORT_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    pub fn has_projection_for(&self, surface: ConsumerProjectionSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter()
            .map(EvidenceExportLaneClass::as_str)
            .collect()
    }

    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(EvidenceExportRowClass::as_str)
            .collect()
    }

    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    pub fn signal_slice_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.signal_slice_kind_class);
        }
        set.into_iter().map(SignalSliceKindClass::as_str).collect()
    }

    pub fn slice_freshness_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.slice_freshness_class);
        }
        set.into_iter().map(SliceFreshnessClass::as_str).collect()
    }

    pub fn replay_chronology_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.replay_chronology_state_class);
        }
        set.into_iter()
            .map(ReplayChronologyStateClass::as_str)
            .collect()
    }

    pub fn retention_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.retention_class);
        }
        set.into_iter().map(RetentionClass::as_str).collect()
    }

    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface_class);
        }
        set.into_iter().map(ConsumerSurfaceClass::as_str).collect()
    }

    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> EvidenceExportTruthSupportExport {
        EvidenceExportTruthSupportExport {
            record_kind: EVIDENCE_EXPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            evidence_export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            evidence_export_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != EVIDENCE_EXPORT_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "evidence-export packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "evidence-export packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceExportLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered evidence-export lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceExportLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers evidence-export lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw log bodies, raw trace payloads, raw test-artifact bytes, or raw command lines past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw secret values past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, SupportClass::LaunchStable)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            // wedge binding rules
            if row.row_class.requires_wedge()
                && matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a wedge_admission but has no bound wedge",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_wedge()
                && !matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds wedge {}; only wedge_admission rows may bind a wedge",
                        row.row_id,
                        row.row_class.as_str(),
                        row.wedge_class.as_str()
                    ),
                ));
            }

            // signal-slice-kind binding rules
            if row.row_class.requires_signal_slice_kind()
                && matches!(
                    row.signal_slice_kind_class,
                    SignalSliceKindClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SignalSliceKindNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a signal_slice_kind_admission but has no bound kind",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_signal_slice_kind()
                && !matches!(
                    row.signal_slice_kind_class,
                    SignalSliceKindClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SignalSliceKindNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds signal-slice kind {}; only signal_slice_kind_admission rows may bind a kind",
                        row.row_id,
                        row.row_class.as_str(),
                        row.signal_slice_kind_class.as_str()
                    ),
                ));
            }

            // slice-freshness binding rules
            if row.row_class.requires_slice_freshness()
                && matches!(
                    row.slice_freshness_class,
                    SliceFreshnessClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SliceFreshnessNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a slice_freshness_admission but has no bound freshness",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_slice_freshness()
                && !matches!(
                    row.slice_freshness_class,
                    SliceFreshnessClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SliceFreshnessNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds slice freshness {}; only slice_freshness_admission rows may bind a freshness",
                        row.row_id,
                        row.row_class.as_str(),
                        row.slice_freshness_class.as_str()
                    ),
                ));
            }

            // replay-chronology binding rules
            if row.row_class.requires_replay_chronology_state()
                && matches!(
                    row.replay_chronology_state_class,
                    ReplayChronologyStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayChronologyStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a replay_chronology_admission but has no bound state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_replay_chronology_state()
                && !matches!(
                    row.replay_chronology_state_class,
                    ReplayChronologyStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayChronologyStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds replay-chronology state {}; only replay_chronology_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.replay_chronology_state_class.as_str()
                    ),
                ));
            }

            // retention-class binding rules
            if row.row_class.requires_retention_class()
                && matches!(row.retention_class, RetentionClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RetentionClassNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a retention_class_admission but has no bound retention class",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_retention_class()
                && !matches!(row.retention_class, RetentionClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RetentionClassNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds retention class {}; only retention_class_admission rows may bind a retention class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.retention_class.as_str()
                    ),
                ));
            }

            // consumer-surface binding rules
            if row.row_class.requires_consumer_surface()
                && matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a consumer_surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_consumer_surface()
                && !matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds consumer surface {}; only consumer_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }

            // lineage admission rules
            if matches!(row.row_class, EvidenceExportRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LineageAdmissionMissingExecutionContextId,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a lineage_admission but has no bound execution_context_id",
                        row.row_id
                    ),
                ));
            }

            // cross-surface evidence lineage attestation rule
            if matches!(row.row_class, EvidenceExportRowClass::WedgeAdmission)
                && matches!(
                    row.wedge_class,
                    WedgeClass::CrossSurfaceEvidenceLineageTruth
                )
                && !row.cross_surface_evidence_lineage_attested
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CrossSurfaceEvidenceLineageNotAttested,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds cross_surface_evidence_lineage_truth but does not attest cross-surface evidence lineage",
                        row.row_id
                    ),
                ));
            }

            if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
                && matches!(row.support_class, SupportClass::LaunchStable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims launch_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        // per-lane coverage for lanes claiming launch_stable
        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, EvidenceExportRowClass::EvidenceExportQuality)
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, EvidenceExportRowClass::WedgeAdmission)
                        && row.wedge_class == wedge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWedgeCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no wedge_admission row for {}",
                            lane.as_str(),
                            wedge.as_str()
                        ),
                    ));
                }
            }

            for kind in SignalSliceKindClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EvidenceExportRowClass::SignalSliceKindAdmission
                        )
                        && row.signal_slice_kind_class == kind
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSignalSliceKindCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no signal_slice_kind_admission row for {}",
                            lane.as_str(),
                            kind.as_str()
                        ),
                    ));
                }
            }

            for freshness in SliceFreshnessClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EvidenceExportRowClass::SliceFreshnessAdmission
                        )
                        && row.slice_freshness_class == freshness
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSliceFreshnessCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no slice_freshness_admission row for {}",
                            lane.as_str(),
                            freshness.as_str()
                        ),
                    ));
                }
            }

            for state in ReplayChronologyStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EvidenceExportRowClass::ReplayChronologyAdmission
                        )
                        && row.replay_chronology_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingReplayChronologyStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no replay_chronology_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for retention in RetentionClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EvidenceExportRowClass::RetentionClassAdmission
                        )
                        && row.retention_class == retention
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingRetentionClassCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no retention_class_admission row for {}",
                            lane.as_str(),
                            retention.as_str()
                        ),
                    ));
                }
            }

            for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EvidenceExportRowClass::ConsumerSurfaceBinding
                        )
                        && row.consumer_surface_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConsumerSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no consumer_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, EvidenceExportRowClass::LineageAdmission)
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_lineage {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLineageAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                        lane.as_str()
                    ),
                ));
            }
        }

        // consumer projections
        for required_surface in ConsumerProjectionSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve evidence-export truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_wedge_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the wedge vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_signal_slice_kind_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SignalSliceKindVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the signal-slice kind vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_slice_freshness_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SliceFreshnessVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the slice-freshness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_replay_chronology_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayChronologyStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the replay-chronology state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_retention_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RetentionClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the retention-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_consumer_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the consumer-surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EvidenceClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportTruthSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub export_id: String,
    pub evidence_export_packet_id_ref: String,
    pub exported_at: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub evidence_export_packet: EvidenceExportTruthPacket,
}

impl EvidenceExportTruthSupportExport {
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == EVIDENCE_EXPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION
            && self.evidence_export_packet_id_ref == self.evidence_export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.evidence_export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum EvidenceExportTruthArtifactError {
    Packet(serde_json::Error),
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for EvidenceExportTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "evidence-export packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "evidence-export packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for EvidenceExportTruthArtifactError {}

/// Returns the checked-in stable evidence-export truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_evidence_export_truth_packet(
) -> Result<EvidenceExportTruthPacket, EvidenceExportTruthArtifactError> {
    let packet: EvidenceExportTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json"
    )))
    .map_err(EvidenceExportTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(EvidenceExportTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        EVIDENCE_EXPORT_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        EVIDENCE_EXPORT_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: EvidenceExportLaneClass) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: EvidenceExportRowClass::EvidenceExportQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        wedge: WedgeClass,
    ) -> EvidenceExportRow {
        let parity_attested = matches!(wedge, WedgeClass::CrossSurfaceEvidenceLineageTruth);
        let automation = match wedge {
            WedgeClass::ArtifactChronologyReplayTruth => {
                DowngradeAutomationClass::AutoNarrowOnReplayChronologyGap
            }
            WedgeClass::SignalSliceIdentityTruth => {
                DowngradeAutomationClass::AutoNarrowOnSignalSliceIdentityGap
            }
            WedgeClass::EvidenceExportReviewTruth => {
                DowngradeAutomationClass::AutoNarrowOnEvidenceExportReviewGap
            }
            WedgeClass::CrossSurfaceEvidenceLineageTruth => {
                DowngradeAutomationClass::AutoNarrowOnCrossSurfaceEvidenceLineageDrift
            }
            WedgeClass::NotApplicable => DowngradeAutomationClass::None,
        };
        EvidenceExportRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: automation,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#{}", doc_ref(), automation.as_str())),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: parity_attested,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn signal_slice_kind_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        kind: SignalSliceKindClass,
    ) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:signal_slice_kind:{}", kind.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::SignalSliceKindAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: kind,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnSignalSliceKindGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_signal_slice_kind_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn slice_freshness_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        freshness: SliceFreshnessClass,
    ) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:slice_freshness:{}", freshness.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::SliceFreshnessAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: freshness,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnSliceFreshnessGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_slice_freshness_gap", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn replay_chronology_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        state: ReplayChronologyStateClass,
    ) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:replay_chronology:{}", state.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::ReplayChronologyAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: state,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnReplayChronologyGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_replay_chronology_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn retention_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        retention: RetentionClass,
    ) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:retention:{}", retention.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::RetentionClassAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: retention,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnRetentionClassGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_retention_class_gap", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn consumer_surface_row(
        prefix: &str,
        lane: EvidenceExportLaneClass,
        surface: ConsumerSurfaceClass,
    ) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:consumer_surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: EvidenceExportRowClass::ConsumerSurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_consumer_surface_gap", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: EvidenceExportLaneClass) -> EvidenceExportRow {
        EvidenceExportRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: EvidenceExportRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            signal_slice_kind_class: SignalSliceKindClass::NotApplicable,
            slice_freshness_class: SliceFreshnessClass::NotApplicable,
            replay_chronology_state_class: ReplayChronologyStateClass::NotApplicable,
            retention_class: RetentionClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!(
                "exec:m4:artifact_manager:{prefix}:lineage"
            )),
            cross_surface_evidence_lineage_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerProjectionSurface) -> ConsumerProjection {
        ConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            evidence_export_packet_id_ref:
                "packet:m4:stabilize_the_artifact_manager_preview_runtime_inspectors_and".to_owned(),
            rendered_at: "2026-05-27T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_signal_slice_kind_vocabulary: true,
            preserves_slice_freshness_vocabulary: true,
            preserves_replay_chronology_state_vocabulary: true,
            preserves_retention_class_vocabulary: true,
            preserves_consumer_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: EvidenceExportLaneClass, prefix: &str) -> Vec<EvidenceExportRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for kind in SignalSliceKindClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(signal_slice_kind_row(prefix, lane, kind));
        }
        for freshness in SliceFreshnessClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(slice_freshness_row(prefix, lane, freshness));
        }
        for state in ReplayChronologyStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(replay_chronology_row(prefix, lane, state));
        }
        for retention in RetentionClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(retention_row(prefix, lane, retention));
        }
        for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(consumer_surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> EvidenceExportTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            EvidenceExportLaneClass::ArtifactManagerLane,
            "artifact_manager",
        ));
        rows.extend(lane_rows(
            EvidenceExportLaneClass::PreviewRuntimeInspectorLane,
            "preview_runtime_inspector",
        ));
        rows.extend(lane_rows(
            EvidenceExportLaneClass::SignalSliceLane,
            "signal_slice",
        ));
        rows.extend(lane_rows(
            EvidenceExportLaneClass::EvidenceExportLane,
            "evidence_export",
        ));
        EvidenceExportTruthPacketInput {
            packet_id: "packet:m4:stabilize_the_artifact_manager_preview_runtime_inspectors_and"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.stabilize_the_artifact_manager_preview_runtime_inspectors_and"
                    .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: EvidenceExportLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerProjectionSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            EvidenceExportLaneClass::ArtifactManagerLane.as_str(),
            "artifact_manager_lane"
        );
        assert_eq!(
            EvidenceExportLaneClass::EvidenceExportLane.as_str(),
            "evidence_export_lane"
        );
        assert_eq!(
            EvidenceExportRowClass::EvidenceExportQuality.as_str(),
            "evidence_export_quality"
        );
        assert_eq!(
            EvidenceExportRowClass::LineageAdmission.as_str(),
            "lineage_admission"
        );
        assert_eq!(
            WedgeClass::ArtifactChronologyReplayTruth.as_str(),
            "artifact_chronology_replay_truth"
        );
        assert_eq!(
            WedgeClass::CrossSurfaceEvidenceLineageTruth.as_str(),
            "cross_surface_evidence_lineage_truth"
        );
        assert_eq!(SignalSliceKindClass::LogsSlice.as_str(), "logs_slice");
        assert_eq!(
            SignalSliceKindClass::TestArtifactSlice.as_str(),
            "test_artifact_slice"
        );
        assert_eq!(SliceFreshnessClass::LiveStream.as_str(), "live_stream");
        assert_eq!(SliceFreshnessClass::ExportedCopy.as_str(), "exported_copy");
        assert_eq!(ReplayChronologyStateClass::Recorded.as_str(), "recorded");
        assert_eq!(
            ReplayChronologyStateClass::RestartWithRecordingAvailable.as_str(),
            "restart_with_recording_available"
        );
        assert_eq!(
            RetentionClass::SessionOnlyRetention.as_str(),
            "session_only_retention"
        );
        assert_eq!(
            RetentionClass::ImportedExternalRetention.as_str(),
            "imported_external_retention"
        );
        assert_eq!(
            ConsumerSurfaceClass::ArtifactManagerSurface.as_str(),
            "artifact_manager_surface"
        );
        assert_eq!(
            ConsumerSurfaceClass::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            FindingKind::CrossSurfaceEvidenceLineageNotAttested.as_str(),
            "cross_surface_evidence_lineage_not_attested"
        );
        assert_eq!(
            FindingKind::MissingSliceFreshnessCoverage.as_str(),
            "missing_slice_freshness_coverage"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = EvidenceExportTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:stabilize_the_artifact_manager_preview_runtime_inspectors_and",
                "2026-05-27T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_signal_slice_kind_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                EvidenceExportRowClass::SignalSliceKindAdmission
            ) && row.signal_slice_kind_class == SignalSliceKindClass::TracesSlice
                && row.lane_class == EvidenceExportLaneClass::SignalSliceLane)
        });
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingSignalSliceKindCoverage
        }));
    }

    #[test]
    fn missing_slice_freshness_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                EvidenceExportRowClass::SliceFreshnessAdmission
            ) && row.slice_freshness_class == SliceFreshnessClass::ExportedCopy
                && row.lane_class == EvidenceExportLaneClass::PreviewRuntimeInspectorLane)
        });
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingSliceFreshnessCoverage));
    }

    #[test]
    fn missing_replay_chronology_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                EvidenceExportRowClass::ReplayChronologyAdmission
            ) && row.replay_chronology_state_class
                == ReplayChronologyStateClass::RestartWithRecordingAvailable
                && row.lane_class == EvidenceExportLaneClass::ArtifactManagerLane)
        });
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingReplayChronologyStateCoverage
        }));
    }

    #[test]
    fn cross_surface_evidence_lineage_without_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, EvidenceExportRowClass::WedgeAdmission)
                && row.wedge_class == WedgeClass::CrossSurfaceEvidenceLineageTruth
                && row.lane_class == EvidenceExportLaneClass::EvidenceExportLane
            {
                row.cross_surface_evidence_lineage_attested = false;
                break;
            }
        }
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::CrossSurfaceEvidenceLineageNotAttested
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, EvidenceExportRowClass::LineageAdmission)
                && row.lane_class == EvidenceExportLaneClass::ArtifactManagerLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerProjectionSurface::ConformanceDashboard
        });
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_slice_freshness_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerProjectionSurface::HelpAbout {
                projection.preserves_slice_freshness_vocabulary = false;
            }
        }
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::SliceFreshnessVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = EvidenceExportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
