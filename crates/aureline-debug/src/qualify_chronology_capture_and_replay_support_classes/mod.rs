//! Chronology capture and replay support class truth packet for the M4 stable lane.
//!
//! This module pins how local, remote/helper, container, and notebook-bridge
//! debug sessions serialize one canonical replay support class truth across
//! the five closed vocabularies (`replay_support_class`,
//! `chronology_capture_state_class`, `mapping_quality_badge_class`,
//! `replay_scope_class`, `inspector_state_class`) so that debugger UI,
//! timeline scrubber, reverse-step toolbar, variable inspector, call-stack
//! panel, evaluate console, support export, compare card, CLI/headless
//! inspector, evidence export, release proof index, Help/About proof card,
//! and the conformance dashboard all read one attributable source.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `chronology_quality` row cannot prove:
//!
//! - the five replay support class values (`supported`, `limited`,
//!   `view_only`, `unsupported`, `policy_blocked`) each have a structured
//!   `support_class_admission` row,
//! - the four chronology capture state values (`recorded`, `not_recorded`,
//!   `restart_with_recording_available`, `capture_unsupported`) each have a
//!   structured `capture_state_admission` row so users never mistake a live
//!   session for a recorded one,
//! - the six mapping-quality badge values (`exact`, `approximate`, `partial`,
//!   `unavailable`, `stale`, `mismatched`) each have a structured
//!   `mapping_quality_badge_admission` row so badges survive into support
//!   and evidence exports,
//! - the three replay scope values (`local_scope`, `remote_scope`,
//!   `notebook_bridge_scope`) each have a structured `replay_scope_admission`
//!   row so local-vs-remote and notebook-bridge limitations are explicit,
//! - the five inspector state values (`live`, `snapshot`, `stale`, `limited`,
//!   `unavailable`) each have a structured `inspector_state_admission` row
//!   so variable, stack, and evaluate surfaces never imply live data on
//!   replay or disconnected sessions,
//! - the four restart-with-recording posture values (`available`,
//!   `unavailable_unsupported_backend`, `unavailable_policy_blocked`,
//!   `unavailable_no_live_session`) each have a structured
//!   `restart_posture_admission` row,
//! - the eight replay surfaces (`debugger_ui_surface`,
//!   `timeline_scrubber_surface`, `reverse_step_controls_surface`,
//!   `variable_inspector_surface`, `call_stack_surface`,
//!   `evaluate_surface`, `support_export_surface`, `compare_card_surface`)
//!   each have a `replay_surface_binding` row attesting they preserve the
//!   closed vocabularies they must preserve and that replay is read-only,
//! - one stable `execution_context_id` threads through every emitted
//!   chronology row via a `lineage_admission` row.
//!
//! Every row binds closed `lane_class`, `row_class`, `support_class`,
//! `replay_support_class`, `chronology_capture_state_class`,
//! `mapping_quality_badge_class`, `replay_scope_class`,
//! `inspector_state_class`, `restart_with_recording_class`,
//! `replay_surface_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `confidence_class` vocabularies plus
//! an `evidence_refs` array and a `disclosure_ref` whenever the row is
//! narrowed below launch-stable, declares a non-`none_declared` known
//! limit, or binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw recording chunks, raw
//! stack frames, raw memory bytes, raw watch expressions, raw evaluate
//! input/output, raw command lines, raw process environment bytes, secrets,
//! or ambient credentials past the boundary. A row that claims
//! `launch_stable` while leaving its known limit, downgrade automation, or
//! evidence class unbound is refused; the validator narrows below
//! launch-stable instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ChronologyReplaySupportTruthPacket`].
pub const CHRONOLOGY_REPLAY_SUPPORT_PACKET_RECORD_KIND: &str =
    "qualify_chronology_capture_and_replay_support_classes_truth_stable_packet";

/// Stable record-kind tag for [`ChronologyReplaySupportExport`].
pub const CHRONOLOGY_REPLAY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "qualify_chronology_capture_and_replay_support_classes_truth_support_export";

/// Integer schema version for the chronology replay support truth packet.
pub const CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_REF: &str =
    "schemas/debug/chronology-replay-support.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const CHRONOLOGY_REPLAY_SUPPORT_DOC_REF: &str =
    "docs/m4/qualify-chronology-capture-and-replay-support-classes.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const CHRONOLOGY_REPLAY_SUPPORT_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/qualify-chronology-capture-and-replay-support-classes.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const CHRONOLOGY_REPLAY_SUPPORT_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/qualify_chronology_capture_and_replay_support_classes";

/// Repo-relative path of the checked-in stable packet.
pub const CHRONOLOGY_REPLAY_SUPPORT_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json";

/// Closed debug lane vocabulary. Every required lane MUST have at least one
/// `chronology_quality` row claiming `launch_stable` or a precisely
/// labeled narrowed-below-stable row with a disclosure ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugLaneClass {
    /// Local-host debug sessions.
    LocalLane,
    /// SSH and remote-agent debug sessions.
    RemoteHelperLane,
    /// Container-attached debug sessions.
    ContainerLane,
    /// Notebook-kernel debugger bridge sessions.
    NotebookBridgeLane,
}

impl DebugLaneClass {
    /// Every required debug lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::NotebookBridgeLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::NotebookBridgeLane => "notebook_bridge_lane",
        }
    }
}

/// Closed row-class vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyRowClass {
    /// The lane's headline chronology qualification row.
    ChronologyQuality,
    /// A row admitting one of the required replay support class values.
    SupportClassAdmission,
    /// A row admitting one of the required chronology capture state values.
    CaptureStateAdmission,
    /// A row admitting one of the required mapping-quality badge values.
    MappingQualityBadgeAdmission,
    /// A row admitting one of the required replay scope values.
    ReplayScopeAdmission,
    /// A row admitting one of the required inspector state values.
    InspectorStateAdmission,
    /// A row binding one replay surface.
    ReplaySurfaceBinding,
    /// A row admitting one of the required restart-with-recording posture values.
    RestartPostureAdmission,
    /// A row binding the stable `execution_context_id` lineage.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl ChronologyRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChronologyQuality => "chronology_quality",
            Self::SupportClassAdmission => "support_class_admission",
            Self::CaptureStateAdmission => "capture_state_admission",
            Self::MappingQualityBadgeAdmission => "mapping_quality_badge_admission",
            Self::ReplayScopeAdmission => "replay_scope_admission",
            Self::InspectorStateAdmission => "inspector_state_admission",
            Self::ReplaySurfaceBinding => "replay_surface_binding",
            Self::RestartPostureAdmission => "restart_posture_admission",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    pub const fn requires_replay_support_class(self) -> bool {
        matches!(self, Self::SupportClassAdmission)
    }

    pub const fn requires_capture_state(self) -> bool {
        matches!(self, Self::CaptureStateAdmission)
    }

    pub const fn requires_mapping_quality_badge(self) -> bool {
        matches!(self, Self::MappingQualityBadgeAdmission)
    }

    pub const fn requires_replay_scope(self) -> bool {
        matches!(self, Self::ReplayScopeAdmission)
    }

    pub const fn requires_inspector_state(self) -> bool {
        matches!(self, Self::InspectorStateAdmission)
    }

    pub const fn requires_restart_posture(self) -> bool {
        matches!(self, Self::RestartPostureAdmission)
    }

    pub const fn requires_replay_surface(self) -> bool {
        matches!(self, Self::ReplaySurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a row.
///
/// `launch_stable` is the M4 grade. `launch_stable_below`,
/// `beta_grade_only`, `preview_only`, and `unsupported` are the precise
/// narrowed labels; each narrowed row MUST surface a non-null
/// `disclosure_ref`. `support_unbound` never qualifies for stable promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable.
    LaunchStableBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
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

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed replay support class vocabulary.
///
/// This is the per-lane, per-session capability declaration. Debugger UI,
/// support exports, and release reviewers MUST read this value instead of
/// inferring capability from adjacent debug rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySupportClass {
    /// Full reverse-step and reverse-continue set backed by a recorded session.
    Supported,
    /// Subset of reverse-step supported (e.g., snapshot-only, no reverse_continue).
    Limited,
    /// Imported or exported recordings may be inspected but no backward stepping.
    ViewOnly,
    /// The runtime or toolchain cannot host any form of reverse-step or replay.
    Unsupported,
    /// The capability exists but is disabled by organizational or user policy.
    PolicyBlocked,
    /// The row is not bound to a replay support class.
    NotApplicable,
}

impl ReplaySupportClass {
    /// Every required replay support class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Supported,
        Self::Limited,
        Self::ViewOnly,
        Self::Unsupported,
        Self::PolicyBlocked,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::ViewOnly => "view_only",
            Self::Unsupported => "unsupported",
            Self::PolicyBlocked => "policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed chronology capture state vocabulary.
///
/// User-visible capture truth. Every debugger UI banner, support export
/// header, and release evidence row MUST cite one of these values instead
/// of deriving capture state from generic run metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyCaptureStateClass {
    /// A valid recording is attached and replay is possible.
    Recorded,
    /// The session is live with no recording engaged.
    NotRecorded,
    /// The session can be restarted with recording enabled.
    RestartWithRecordingAvailable,
    /// The current backend or runtime cannot produce a chronology recording.
    CaptureUnsupported,
    /// The row is not bound to a capture state.
    NotApplicable,
}

impl ChronologyCaptureStateClass {
    /// Every required capture state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Recorded,
        Self::NotRecorded,
        Self::RestartWithRecordingAvailable,
        Self::CaptureUnsupported,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
            Self::NotRecorded => "not_recorded",
            Self::RestartWithRecordingAvailable => "restart_with_recording_available",
            Self::CaptureUnsupported => "capture_unsupported",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed mapping-quality badge vocabulary.
///
/// Survives into support exports, compare cards, and release evidence.
/// Surfaces MUST NOT flatten these into generic "unavailable" copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingQualityBadgeClass {
    /// Source / symbol mapping is exact and verified against build ID.
    Exact,
    /// Mapping is inferred but not verified field-for-field.
    Approximate,
    /// Only a subset of frames or symbols are mapped.
    Partial,
    /// Symbol or source-map data is absent.
    Unavailable,
    /// Mapping data is older than the current build.
    Stale,
    /// The build ID on the recording does not match the loaded artifact.
    Mismatched,
    /// The row is not bound to a mapping-quality badge.
    NotApplicable,
}

impl MappingQualityBadgeClass {
    /// Every required mapping-quality badge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Exact,
        Self::Approximate,
        Self::Partial,
        Self::Unavailable,
        Self::Stale,
        Self::Mismatched,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
            Self::Stale => "stale",
            Self::Mismatched => "mismatched",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed replay scope vocabulary.
///
/// Surfaces the local-vs-remote and notebook-bridge distinction that
/// determines which reverse-step verbs and which export postures are
/// available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayScopeClass {
    /// Recording and replay run on the local host.
    LocalScope,
    /// Recording or replay runs through a remote agent.
    RemoteScope,
    /// Replay is available only via a notebook-kernel bridge, which may
    /// impose additional limitations such as no reverse_continue.
    NotebookBridgeScope,
    /// The row is not bound to a replay scope.
    NotApplicable,
}

impl ReplayScopeClass {
    /// Every required replay scope for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::LocalScope,
        Self::RemoteScope,
        Self::NotebookBridgeScope,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalScope => "local_scope",
            Self::RemoteScope => "remote_scope",
            Self::NotebookBridgeScope => "notebook_bridge_scope",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed inspector state vocabulary for variable, stack, and evaluate
/// surfaces during replay and disconnected sessions.
///
/// Surfaces MUST distinguish these five states instead of collapsing them
/// to "unavailable" or omitting state labels silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorStateClass {
    /// Value reflects the current replay frame.
    Live,
    /// Value reflects a captured snapshot from the recording.
    Snapshot,
    /// Value is older than the current replay position.
    Stale,
    /// Value is partially available.
    Limited,
    /// Value cannot be retrieved from the recording.
    Unavailable,
    /// The row is not bound to an inspector state.
    NotApplicable,
}

impl InspectorStateClass {
    /// Every required inspector state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Live,
        Self::Snapshot,
        Self::Stale,
        Self::Limited,
        Self::Unavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Snapshot => "snapshot",
            Self::Stale => "stale",
            Self::Limited => "limited",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed restart-with-recording posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartWithRecordingClass {
    /// The action is available and enabled for the current session.
    Available,
    /// The current backend does not support recording.
    UnavailableUnsupportedBackend,
    /// Recording is disabled by policy.
    UnavailablePolicyBlocked,
    /// There is no live session to restart.
    UnavailableNoLiveSession,
    /// The row is not bound to a restart posture.
    NotApplicable,
}

impl RestartWithRecordingClass {
    /// Every required restart posture for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Available,
        Self::UnavailableUnsupportedBackend,
        Self::UnavailablePolicyBlocked,
        Self::UnavailableNoLiveSession,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::UnavailableUnsupportedBackend => "unavailable_unsupported_backend",
            Self::UnavailablePolicyBlocked => "unavailable_policy_blocked",
            Self::UnavailableNoLiveSession => "unavailable_no_live_session",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed replay surface vocabulary.
///
/// Every surface must bind typed vocabulary from this packet instead of
/// minting local copies or silently hiding controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySurfaceClass {
    /// Main debugger UI panel.
    DebuggerUiSurface,
    /// Timeline scrubber widget.
    TimelineScrubberSurface,
    /// Reverse-step controls toolbar.
    ReverseStepControlsSurface,
    /// Variable inspector panel.
    VariableInspectorSurface,
    /// Call-stack panel.
    CallStackSurface,
    /// Evaluate console.
    EvaluateSurface,
    /// Support export bundle.
    SupportExportSurface,
    /// Compare card surface.
    CompareCardSurface,
    /// The row is not bound to a replay surface.
    NotApplicable,
}

impl ReplaySurfaceClass {
    /// Every required replay surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 8] = [
        Self::DebuggerUiSurface,
        Self::TimelineScrubberSurface,
        Self::ReverseStepControlsSurface,
        Self::VariableInspectorSurface,
        Self::CallStackSurface,
        Self::EvaluateSurface,
        Self::SupportExportSurface,
        Self::CompareCardSurface,
    ];

    /// True when the surface must attest inspector-state preservation.
    pub const fn requires_inspector_state_attestation(self) -> bool {
        matches!(
            self,
            Self::VariableInspectorSurface
                | Self::CallStackSurface
                | Self::EvaluateSurface
                | Self::SupportExportSurface
        )
    }

    /// True when the surface must attest mapping-quality preservation.
    pub const fn requires_mapping_quality_attestation(self) -> bool {
        matches!(
            self,
            Self::CallStackSurface
                | Self::VariableInspectorSurface
                | Self::EvaluateSurface
                | Self::SupportExportSurface
                | Self::CompareCardSurface
        )
    }

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebuggerUiSurface => "debugger_ui_surface",
            Self::TimelineScrubberSurface => "timeline_scrubber_surface",
            Self::ReverseStepControlsSurface => "reverse_step_controls_surface",
            Self::VariableInspectorSurface => "variable_inspector_surface",
            Self::CallStackSurface => "call_stack_surface",
            Self::EvaluateSurface => "evaluate_surface",
            Self::SupportExportSurface => "support_export_surface",
            Self::CompareCardSurface => "compare_card_surface",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// Release-evidence review.
    ReleaseEvidenceReview,
    /// Conformance suite evidence.
    ConformanceSuiteEvidence,
    /// Automated functional evidence.
    AutomatedFunctionalEvidence,
    /// Design QA evidence.
    DesignQaEvidence,
    /// Failure / recovery drill evidence.
    FailureRecoveryDrillEvidence,
    /// No evidence is bound; never qualifies launch_stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when the evidence class can support a launch_stable row.
    pub const fn qualifies_for_stable(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit is registered for the row.
    NoneDeclared,
    /// Notebook-bridge lanes support only limited replay verbs.
    NotebookBridgeReplayLimited,
    /// Remote-scope sessions cannot support reverse-step on some backends.
    RemoteScopeReverseStepLimited,
    /// Container capture produces partial history.
    ContainerCapturePartial,
    /// Recording is disabled by policy.
    PolicyBlockedNoRecording,
    /// Import or view-only lane has no reverse-step capability.
    ImportViewOnlyNoReverseStep,
    /// No capture backend ships for this runtime.
    CaptureUnsupportedNoBackend,
    /// Stale mapping data prevents reverse stepping.
    StaleMappingReverseStepDisabled,
    /// Build-ID mismatch blocks replay.
    MismatchedBuildIdReplayBlocked,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::NotebookBridgeReplayLimited => "notebook_bridge_replay_limited",
            Self::RemoteScopeReverseStepLimited => "remote_scope_reverse_step_limited",
            Self::ContainerCapturePartial => "container_capture_partial",
            Self::PolicyBlockedNoRecording => "policy_blocked_no_recording",
            Self::ImportViewOnlyNoReverseStep => "import_view_only_no_reverse_step",
            Self::CaptureUnsupportedNoBackend => "capture_unsupported_no_backend",
            Self::StaleMappingReverseStepDisabled => "stale_mapping_reverse_step_disabled",
            Self::MismatchedBuildIdReplayBlocked => "mismatched_build_id_replay_blocked",
        }
    }

    /// True when the known limit must pair with a disclosure ref.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared)
    }
}

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is bound.
    None,
    /// Block on missing evidence.
    AutoBlockOnMissingEvidence,
    /// Auto-narrow when support class coverage is incomplete.
    AutoNarrowOnSupportClassGap,
    /// Auto-narrow when capture state coverage is incomplete.
    AutoNarrowOnCaptureStateGap,
    /// Auto-narrow when mapping-quality badge coverage is incomplete.
    AutoNarrowOnMappingQualityGap,
    /// Auto-narrow when inspector state coverage is incomplete.
    AutoNarrowOnInspectorStateGap,
    /// Auto-narrow when replay surface binding is incomplete.
    AutoNarrowOnReplaySurfaceGap,
    /// Auto-narrow when restart-with-recording posture coverage is incomplete.
    AutoNarrowOnRestartPostureGap,
    /// Auto-narrow when lineage admission is missing.
    AutoNarrowOnMissingLineage,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::AutoNarrowOnSupportClassGap => "auto_narrow_on_support_class_gap",
            Self::AutoNarrowOnCaptureStateGap => "auto_narrow_on_capture_state_gap",
            Self::AutoNarrowOnMappingQualityGap => "auto_narrow_on_mapping_quality_gap",
            Self::AutoNarrowOnInspectorStateGap => "auto_narrow_on_inspector_state_gap",
            Self::AutoNarrowOnReplaySurfaceGap => "auto_narrow_on_replay_surface_gap",
            Self::AutoNarrowOnRestartPostureGap => "auto_narrow_on_restart_posture_gap",
            Self::AutoNarrowOnMissingLineage => "auto_narrow_on_missing_lineage",
        }
    }
}

/// Closed confidence vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence; fully bound evidence.
    HighConfidence,
    /// Medium confidence; partially bound evidence.
    MediumConfidence,
    /// Low confidence; weak or pending evidence.
    LowConfidence,
    /// No confidence is bound; never qualifies launch_stable.
    ConfidenceUnbound,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
            Self::ConfidenceUnbound => "confidence_unbound",
        }
    }

    /// True when the confidence class can support a launch_stable row.
    pub const fn qualifies_for_stable(self) -> bool {
        !matches!(self, Self::ConfidenceUnbound)
    }
}

/// Closed promotion state vocabulary for the packet as a whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// All lanes satisfy the launch-stable invariants; zero validation findings.
    Stable,
    /// One or more lanes have findings that must be resolved before stable.
    BlocksStable,
}

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Finding must be resolved before stable promotion.
    BlocksStable,
    /// Finding is advisory only.
    Advisory,
}

/// Closed finding-kind vocabulary for validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// The quality row's evidence_class is EvidenceUnbound.
    MissingEvidenceClass,
    /// A launch_stable row carries SupportUnbound.
    LaunchStableWithUnboundBinding,
    /// Not every required replay support class has a support_class_admission row.
    MissingReplaySupportClassCoverage,
    /// Not every required capture state has a capture_state_admission row.
    MissingCaptureStateCoverage,
    /// Not every required mapping-quality badge has a mapping_quality_badge_admission row.
    MissingMappingQualityBadgeCoverage,
    /// Not every required replay scope has a replay_scope_admission row.
    MissingReplayScopeCoverage,
    /// Not every required inspector state has an inspector_state_admission row.
    MissingInspectorStateCoverage,
    /// Not every required restart posture has a restart_posture_admission row.
    MissingRestartPostureCoverage,
    /// Not every required replay surface has a replay_surface_binding row.
    MissingReplaySurfaceCoverage,
    /// A replay_surface_binding row for a surface that requires inspector-state attestation
    /// does not set attests_inspector_state_preserved = true.
    ReplaySurfaceMissingInspectorStateAttestation,
    /// A replay_surface_binding row for a surface that requires mapping-quality attestation
    /// does not set attests_mapping_quality_preserved = true.
    ReplaySurfaceMissingMappingQualityAttestation,
    /// A replay_surface_binding row that does not attest replay_read_only = true.
    ReplaySurfaceMissingReadOnlyAttestation,
    /// A narrowed or unsupported row is missing its required disclosure_ref.
    NarrowedRowMissingDisclosureRef,
    /// A known-limit row with a non-none_declared class is missing its disclosure_ref.
    KnownLimitMissingDisclosureRef,
    /// The lineage_admission row for a launch_stable lane has no execution_context_id_binding.
    LineageAdmissionMissingExecutionContextId,
    /// A required consumer projection is missing from consumer_projections.
    MissingConsumerProjection,
    /// A consumer projection does not preserve the replay_support vocabulary.
    ReplaySupportVocabularyCollapsed,
    /// A consumer projection does not preserve the inspector_state vocabulary.
    InspectorStateVocabularyCollapsed,
    /// A consumer projection does not preserve the mapping_quality vocabulary.
    MappingQualityVocabularyCollapsed,
    /// A consumer projection does not preserve the capture_state vocabulary.
    CaptureStateVocabularyCollapsed,
    /// A consumer projection does not preserve the restart_posture vocabulary.
    RestartPostureVocabularyCollapsed,
    /// Raw source material is present in a row that claims exclusion.
    RawSourceMaterialPresent,
}

impl FindingKind {
    /// Stable token used in support exports and conformance dashboards.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::MissingReplaySupportClassCoverage => "missing_replay_support_class_coverage",
            Self::MissingCaptureStateCoverage => "missing_capture_state_coverage",
            Self::MissingMappingQualityBadgeCoverage => "missing_mapping_quality_badge_coverage",
            Self::MissingReplayScopeCoverage => "missing_replay_scope_coverage",
            Self::MissingInspectorStateCoverage => "missing_inspector_state_coverage",
            Self::MissingRestartPostureCoverage => "missing_restart_posture_coverage",
            Self::MissingReplaySurfaceCoverage => "missing_replay_surface_coverage",
            Self::ReplaySurfaceMissingInspectorStateAttestation => {
                "replay_surface_missing_inspector_state_attestation"
            }
            Self::ReplaySurfaceMissingMappingQualityAttestation => {
                "replay_surface_missing_mapping_quality_attestation"
            }
            Self::ReplaySurfaceMissingReadOnlyAttestation => {
                "replay_surface_missing_read_only_attestation"
            }
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ReplaySupportVocabularyCollapsed => "replay_support_vocabulary_collapsed",
            Self::InspectorStateVocabularyCollapsed => "inspector_state_vocabulary_collapsed",
            Self::MappingQualityVocabularyCollapsed => "mapping_quality_vocabulary_collapsed",
            Self::CaptureStateVocabularyCollapsed => "capture_state_vocabulary_collapsed",
            Self::RestartPostureVocabularyCollapsed => "restart_posture_vocabulary_collapsed",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
        }
    }
}

/// Closed consumer-surface vocabulary.
///
/// Every listed surface MUST be present in `consumer_projections` and MUST
/// preserve all closed vocabularies verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Main debugger UI.
    DebuggerUi,
    /// Timeline scrubber widget.
    TimelineScrubber,
    /// Reverse-step toolbar.
    ReverseStepToolbar,
    /// Variable inspector panel.
    VariableInspector,
    /// Call-stack panel.
    CallStackPanel,
    /// Evaluate console.
    EvaluateConsole,
    /// Support export bundle.
    SupportExport,
    /// Compare card surface.
    CompareCard,
    /// CLI / headless inspector output.
    CliHeadless,
    /// Evidence export.
    EvidenceExport,
    /// Release proof index.
    ReleaseProofIndex,
    /// Help / About proof card.
    HelpAbout,
    /// Conformance dashboard.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface for the stable packet.
    pub const REQUIRED: [Self; 13] = [
        Self::DebuggerUi,
        Self::TimelineScrubber,
        Self::ReverseStepToolbar,
        Self::VariableInspector,
        Self::CallStackPanel,
        Self::EvaluateConsole,
        Self::SupportExport,
        Self::CompareCard,
        Self::CliHeadless,
        Self::EvidenceExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebuggerUi => "debugger_ui",
            Self::TimelineScrubber => "timeline_scrubber",
            Self::ReverseStepToolbar => "reverse_step_toolbar",
            Self::VariableInspector => "variable_inspector",
            Self::CallStackPanel => "call_stack_panel",
            Self::EvaluateConsole => "evaluate_console",
            Self::SupportExport => "support_export",
            Self::CompareCard => "compare_card",
            Self::CliHeadless => "cli_headless",
            Self::EvidenceExport => "evidence_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding produced during packet materialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// The row id where the finding was raised, or the packet id for packet-level findings.
    pub row_id: String,
    /// The lane where the finding was raised, or `None` for packet-level findings.
    pub lane_class: Option<DebugLaneClass>,
    /// The finding kind.
    pub finding_kind: FindingKind,
    /// The finding severity.
    pub severity: FindingSeverity,
    /// Human-readable explanation (redaction-safe; no raw paths, raw bytes, or secrets).
    pub explanation: String,
}

/// One row in the chronology replay support class truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyReplaySupportRow {
    /// Stable, unique row identifier.
    pub row_id: String,
    /// The debug lane this row belongs to.
    pub lane_class: DebugLaneClass,
    /// The row class.
    pub row_class: ChronologyRowClass,
    /// The promotion grade for this row.
    pub support_class: SupportClass,
    /// The replay support class admitted by this row, or `None` when not applicable.
    pub replay_support_class: Option<ReplaySupportClass>,
    /// The capture state admitted by this row, or `None` when not applicable.
    pub capture_state_class: Option<ChronologyCaptureStateClass>,
    /// The mapping-quality badge admitted by this row, or `None` when not applicable.
    pub mapping_quality_badge_class: Option<MappingQualityBadgeClass>,
    /// The replay scope admitted by this row, or `None` when not applicable.
    pub replay_scope_class: Option<ReplayScopeClass>,
    /// The inspector state admitted by this row, or `None` when not applicable.
    pub inspector_state_class: Option<InspectorStateClass>,
    /// The restart-with-recording posture admitted by this row, or `None` when not applicable.
    pub restart_with_recording_class: Option<RestartWithRecordingClass>,
    /// The replay surface bound by this row, or `None` when not applicable.
    pub replay_surface_class: Option<ReplaySurfaceClass>,
    /// The evidence class for this row.
    pub evidence_class: EvidenceClass,
    /// The known-limit class for this row.
    pub known_limit_class: KnownLimitClass,
    /// The downgrade-automation class for this row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence in this row's evidence.
    pub confidence_class: ConfidenceClass,
    /// Evidence references (repo-relative paths or opaque ids).
    pub evidence_refs: Vec<String>,
    /// Disclosure ref required for narrowed, unsupported, or known-limit rows.
    pub disclosure_ref: Option<String>,
    /// Stable execution_context_id bound by a `lineage_admission` row.
    pub execution_context_id_binding: Option<String>,
    /// True when the replay surface preserves the closed inspector_state vocabulary.
    pub attests_inspector_state_preserved: bool,
    /// True when the replay surface preserves the closed mapping_quality_badge vocabulary.
    pub attests_mapping_quality_preserved: bool,
    /// True when the row attests replay is read-only.
    pub attests_replay_read_only: bool,
    /// True when raw recording chunks, raw stack frames, raw memory bytes, raw command
    /// lines, and raw env bytes are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets, tokens, and signing material are excluded.
    pub secrets_excluded: bool,
    /// True when ambient host authority is excluded.
    pub ambient_authority_excluded: bool,
    /// RFC 3339 UTC timestamp when this row was captured.
    pub captured_at: String,
}

/// One consumer projection that must preserve all closed vocabularies verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyConsumerProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Stable reference to the packet id this projection reads from.
    pub packet_id_ref: String,
    /// True when this projection preserves the replay_support_class vocabulary.
    pub preserves_replay_support_vocabulary: bool,
    /// True when this projection preserves the capture_state_class vocabulary.
    pub preserves_capture_state_vocabulary: bool,
    /// True when this projection preserves the mapping_quality_badge_class vocabulary.
    pub preserves_mapping_quality_vocabulary: bool,
    /// True when this projection preserves the inspector_state_class vocabulary.
    pub preserves_inspector_state_vocabulary: bool,
    /// True when this projection preserves the restart_with_recording_class vocabulary.
    pub preserves_restart_posture_vocabulary: bool,
}

/// Input to [`ChronologyReplaySupportTruthPacket::materialize`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronologyReplaySupportInput {
    /// Stable, unique packet identifier.
    pub packet_id: String,
    /// Stable workflow or surface identifier.
    pub workflow_or_surface_id: String,
    /// RFC 3339 UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Rows to validate and include in the packet.
    pub rows: Vec<ChronologyReplaySupportRow>,
    /// Consumer projections to validate.
    pub consumer_projections: Vec<ChronologyConsumerProjection>,
}

/// Canonical chronology replay support class truth packet.
///
/// Produced by [`ChronologyReplaySupportTruthPacket::materialize`]; never
/// constructed directly. The packet is the single source of truth every
/// downstream surface reads for replay support class, capture state,
/// mapping-quality badge, inspector state, restart-with-recording posture,
/// and scope across all four debug lanes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronologyReplaySupportTruthPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable, unique packet identifier.
    pub packet_id: String,
    /// Stable workflow or surface identifier.
    pub workflow_or_surface_id: String,
    /// RFC 3339 UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// The lanes covered by this packet.
    pub covered_lanes: Vec<DebugLaneClass>,
    /// The validated rows in the packet.
    pub rows: Vec<ChronologyReplaySupportRow>,
    /// The consumer projections in the packet.
    pub consumer_projections: Vec<ChronologyConsumerProjection>,
    /// The overall promotion state for the packet.
    pub promotion_state: PromotionState,
    /// Validation findings; empty when `promotion_state == Stable`.
    pub validation_findings: Vec<ValidationFinding>,
}

impl ChronologyReplaySupportTruthPacket {
    /// Materializes the packet by running the full validation pass over `input`.
    ///
    /// Consumers MUST read the returned `promotion_state` and
    /// `validation_findings` before acting on the packet. A packet whose
    /// `promotion_state` is `BlocksStable` MUST NOT be used as stable
    /// evidence.
    pub fn materialize(input: ChronologyReplaySupportInput) -> Self {
        let mut findings = Vec::new();

        let covered_lanes: Vec<DebugLaneClass> = {
            let mut seen = BTreeSet::new();
            input
                .rows
                .iter()
                .filter(|r| seen.insert(r.lane_class))
                .map(|r| r.lane_class)
                .collect()
        };

        for &lane in &DebugLaneClass::REQUIRED {
            let lane_rows: Vec<&ChronologyReplaySupportRow> = input
                .rows
                .iter()
                .filter(|r| r.lane_class == lane)
                .collect();

            // Quality row must have bound evidence and confidence.
            for row in lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::ChronologyQuality))
            {
                if !row.evidence_class.qualifies_for_stable() {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingEvidenceClass,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} quality row has evidence_unbound; bind a qualifying evidence class.",
                            lane.as_str()
                        ),
                    });
                }
                if !row.confidence_class.qualifies_for_stable() {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::LaunchStableWithUnboundBinding,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} quality row has confidence_unbound; bind a qualifying confidence class.",
                            lane.as_str()
                        ),
                    });
                }
                if matches!(row.support_class, SupportClass::LaunchStable)
                    && matches!(row.evidence_class, EvidenceClass::EvidenceUnbound)
                {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::LaunchStableWithUnboundBinding,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} quality row claims launch_stable with unbound evidence.",
                            lane.as_str()
                        ),
                    });
                }
            }

            // Every required replay support class must have a support_class_admission row.
            let admitted_replay_support: BTreeSet<ReplaySupportClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::SupportClassAdmission))
                .filter_map(|r| r.replay_support_class)
                .collect();
            for required in ReplaySupportClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_replay_support.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingReplaySupportClassCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing support_class_admission row for replay_support_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required capture state must have a capture_state_admission row.
            let admitted_capture_state: BTreeSet<ChronologyCaptureStateClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::CaptureStateAdmission))
                .filter_map(|r| r.capture_state_class)
                .collect();
            for required in ChronologyCaptureStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_capture_state.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingCaptureStateCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing capture_state_admission row for capture_state_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required mapping-quality badge must have a mapping_quality_badge_admission row.
            let admitted_mapping_quality: BTreeSet<MappingQualityBadgeClass> = lane_rows
                .iter()
                .filter(|r| {
                    matches!(
                        r.row_class,
                        ChronologyRowClass::MappingQualityBadgeAdmission
                    )
                })
                .filter_map(|r| r.mapping_quality_badge_class)
                .collect();
            for required in MappingQualityBadgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_mapping_quality.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingMappingQualityBadgeCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing mapping_quality_badge_admission row for mapping_quality_badge_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required replay scope must have a replay_scope_admission row.
            let admitted_replay_scope: BTreeSet<ReplayScopeClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::ReplayScopeAdmission))
                .filter_map(|r| r.replay_scope_class)
                .collect();
            for required in ReplayScopeClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_replay_scope.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingReplayScopeCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing replay_scope_admission row for replay_scope_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required inspector state must have an inspector_state_admission row.
            let admitted_inspector_state: BTreeSet<InspectorStateClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::InspectorStateAdmission))
                .filter_map(|r| r.inspector_state_class)
                .collect();
            for required in InspectorStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_inspector_state.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingInspectorStateCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing inspector_state_admission row for inspector_state_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required restart posture must have a restart_posture_admission row.
            let admitted_restart_posture: BTreeSet<RestartWithRecordingClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::RestartPostureAdmission))
                .filter_map(|r| r.restart_with_recording_class)
                .collect();
            for required in RestartWithRecordingClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !admitted_restart_posture.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingRestartPostureCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing restart_posture_admission row for restart_with_recording_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Every required replay surface must have a replay_surface_binding row.
            let bound_surfaces: BTreeSet<ReplaySurfaceClass> = lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::ReplaySurfaceBinding))
                .filter_map(|r| r.replay_surface_class)
                .collect();
            for required in ReplaySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                if !bound_surfaces.contains(&required) {
                    findings.push(ValidationFinding {
                        row_id: input.packet_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::MissingReplaySurfaceCoverage,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} is missing replay_surface_binding row for replay_surface_class = {}.",
                            lane.as_str(),
                            required.as_str()
                        ),
                    });
                }
            }

            // Surface-binding attestation checks.
            for row in lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::ReplaySurfaceBinding))
            {
                let surface = match row.replay_surface_class {
                    Some(s) => s,
                    None => continue,
                };

                if surface.requires_inspector_state_attestation()
                    && !row.attests_inspector_state_preserved
                {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::ReplaySurfaceMissingInspectorStateAttestation,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} surface {} must set attests_inspector_state_preserved = true.",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    });
                }

                if surface.requires_mapping_quality_attestation()
                    && !row.attests_mapping_quality_preserved
                {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::ReplaySurfaceMissingMappingQualityAttestation,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} surface {} must set attests_mapping_quality_preserved = true.",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    });
                }

                if !matches!(surface, ReplaySurfaceClass::SupportExportSurface)
                    && !row.attests_replay_read_only
                {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::ReplaySurfaceMissingReadOnlyAttestation,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} surface {} must set attests_replay_read_only = true.",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    });
                }
            }

            // Narrowed rows must have disclosure refs.
            for row in &lane_rows {
                if row.support_class.requires_explicit_disclosure()
                    && row.disclosure_ref.is_none()
                {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::NarrowedRowMissingDisclosureRef,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} row {} is narrowed to {} but has no disclosure_ref.",
                            lane.as_str(),
                            row.row_id,
                            row.support_class.as_str()
                        ),
                    });
                }

                if row.known_limit_class.requires_disclosure() && row.disclosure_ref.is_none() {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::KnownLimitMissingDisclosureRef,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} row {} declares known_limit = {} but has no disclosure_ref.",
                            lane.as_str(),
                            row.row_id,
                            row.known_limit_class.as_str()
                        ),
                    });
                }

                // Raw source material must be excluded.
                if !row.raw_source_material_excluded {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::RawSourceMaterialPresent,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} row {} admits raw source material past the boundary.",
                            lane.as_str(),
                            row.row_id
                        ),
                    });
                }
            }

            // Lineage admission must carry an execution_context_id_binding.
            for row in lane_rows
                .iter()
                .filter(|r| matches!(r.row_class, ChronologyRowClass::LineageAdmission))
            {
                if row.execution_context_id_binding.is_none() {
                    findings.push(ValidationFinding {
                        row_id: row.row_id.clone(),
                        lane_class: Some(lane),
                        finding_kind: FindingKind::LineageAdmissionMissingExecutionContextId,
                        severity: FindingSeverity::BlocksStable,
                        explanation: format!(
                            "Lane {} lineage_admission row has no execution_context_id_binding.",
                            lane.as_str()
                        ),
                    });
                }
            }
        }

        // Consumer projection checks.
        let present_surfaces: BTreeSet<ConsumerSurface> = input
            .consumer_projections
            .iter()
            .map(|p| p.consumer_surface)
            .collect();
        for required in ConsumerSurface::REQUIRED {
            if !present_surfaces.contains(&required) {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::MissingConsumerProjection,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} is missing from consumer_projections.",
                        required.as_str()
                    ),
                });
            }
        }
        for projection in &input.consumer_projections {
            if !projection.preserves_replay_support_vocabulary {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::ReplaySupportVocabularyCollapsed,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} does not preserve replay_support_class vocabulary.",
                        projection.consumer_surface.as_str()
                    ),
                });
            }
            if !projection.preserves_inspector_state_vocabulary {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::InspectorStateVocabularyCollapsed,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} does not preserve inspector_state_class vocabulary.",
                        projection.consumer_surface.as_str()
                    ),
                });
            }
            if !projection.preserves_mapping_quality_vocabulary {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::MappingQualityVocabularyCollapsed,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} does not preserve mapping_quality_badge_class vocabulary.",
                        projection.consumer_surface.as_str()
                    ),
                });
            }
            if !projection.preserves_capture_state_vocabulary {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::CaptureStateVocabularyCollapsed,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} does not preserve capture_state_class vocabulary.",
                        projection.consumer_surface.as_str()
                    ),
                });
            }
            if !projection.preserves_restart_posture_vocabulary {
                findings.push(ValidationFinding {
                    row_id: input.packet_id.clone(),
                    lane_class: None,
                    finding_kind: FindingKind::RestartPostureVocabularyCollapsed,
                    severity: FindingSeverity::BlocksStable,
                    explanation: format!(
                        "Consumer surface {} does not preserve restart_with_recording_class vocabulary.",
                        projection.consumer_surface.as_str()
                    ),
                });
            }
        }

        let promotion_state = if findings.is_empty() {
            PromotionState::Stable
        } else {
            PromotionState::BlocksStable
        };

        Self {
            record_kind: CHRONOLOGY_REPLAY_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
        }
    }

    /// Returns `true` when the packet is fully stable with zero findings.
    pub fn is_stable(&self) -> bool {
        matches!(self.promotion_state, PromotionState::Stable)
    }

    /// Returns a support export safe for inclusion in a support bundle.
    ///
    /// The export preserves the record kind, schema version, packet id,
    /// promotion state, and findings but excludes raw source material,
    /// raw payload bytes, secrets, and ambient authority.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> ChronologyReplaySupportExport {
        ChronologyReplaySupportExport {
            record_kind: CHRONOLOGY_REPLAY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            source_packet_id: self.packet_id.clone(),
            exported_at: exported_at.to_owned(),
            promotion_state: self.promotion_state,
            finding_count: self.validation_findings.len(),
            covered_lanes: self.covered_lanes.clone(),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
        }
    }
}

/// Support-export record for the chronology replay support packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronologyReplaySupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export identifier.
    pub export_id: String,
    /// The packet id this export was derived from.
    pub source_packet_id: String,
    /// RFC 3339 UTC timestamp when the export was produced.
    pub exported_at: String,
    /// The promotion state of the source packet.
    pub promotion_state: PromotionState,
    /// Number of validation findings in the source packet.
    pub finding_count: usize,
    /// The lanes covered by the source packet.
    pub covered_lanes: Vec<DebugLaneClass>,
    /// Always true in a well-formed export.
    pub raw_source_material_excluded: bool,
    /// Always true in a well-formed export.
    pub secrets_excluded: bool,
    /// Always true in a well-formed export.
    pub ambient_authority_excluded: bool,
}

impl ChronologyReplaySupportExport {
    /// Returns `true` when the export is safe to include in a support bundle.
    pub fn is_export_safe(&self) -> bool {
        self.raw_source_material_excluded && self.secrets_excluded && self.ambient_authority_excluded
    }
}

/// Error type for artifact loading and validation.
#[derive(Debug)]
pub enum ChronologyReplaySupportArtifactError {
    /// The artifact JSON could not be parsed.
    ParseError(String),
    /// The packet's record_kind does not match the expected constant.
    RecordKindMismatch { expected: String, found: String },
    /// The packet's schema_version does not match the expected constant.
    SchemaVersionMismatch { expected: u32, found: u32 },
    /// The materialized packet blocks stable promotion.
    BlocksStable(Vec<ValidationFinding>),
}

impl fmt::Display for ChronologyReplaySupportArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "parse error: {msg}"),
            Self::RecordKindMismatch { expected, found } => {
                write!(f, "record_kind mismatch: expected {expected}, found {found}")
            }
            Self::SchemaVersionMismatch { expected, found } => write!(
                f,
                "schema_version mismatch: expected {expected}, found {found}"
            ),
            Self::BlocksStable(findings) => write!(
                f,
                "packet blocks stable with {} finding(s)",
                findings.len()
            ),
        }
    }
}

impl Error for ChronologyReplaySupportArtifactError {}

/// Returns the current stable chronology replay support class truth packet.
///
/// Loads and validates the checked-in artifact at
/// [`CHRONOLOGY_REPLAY_SUPPORT_PACKET_ARTIFACT_REF`] and returns the
/// materialized packet. Returns
/// [`ChronologyReplaySupportArtifactError`] if the file cannot be parsed,
/// the record kind or schema version is wrong, or the packet blocks stable.
///
/// # Errors
///
/// Returns an error when the artifact file is missing, malformed, or the
/// materialized packet is not stable.
pub fn current_stable_chronology_replay_support_truth_packet(
    artifact_json: &str,
) -> Result<ChronologyReplaySupportTruthPacket, ChronologyReplaySupportArtifactError> {
    let packet: ChronologyReplaySupportTruthPacket = serde_json::from_str(artifact_json)
        .map_err(|e| ChronologyReplaySupportArtifactError::ParseError(e.to_string()))?;

    if packet.record_kind != CHRONOLOGY_REPLAY_SUPPORT_PACKET_RECORD_KIND {
        return Err(ChronologyReplaySupportArtifactError::RecordKindMismatch {
            expected: CHRONOLOGY_REPLAY_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            found: packet.record_kind,
        });
    }
    if packet.schema_version != CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_VERSION {
        return Err(ChronologyReplaySupportArtifactError::SchemaVersionMismatch {
            expected: CHRONOLOGY_REPLAY_SUPPORT_SCHEMA_VERSION,
            found: packet.schema_version,
        });
    }
    if !packet.is_stable() {
        return Err(ChronologyReplaySupportArtifactError::BlocksStable(
            packet.validation_findings.clone(),
        ));
    }

    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_row(
        row_id: &str,
        lane: DebugLaneClass,
        row_class: ChronologyRowClass,
        replay_support: Option<ReplaySupportClass>,
        capture_state: Option<ChronologyCaptureStateClass>,
        mapping_quality: Option<MappingQualityBadgeClass>,
        replay_scope: Option<ReplayScopeClass>,
        inspector_state: Option<InspectorStateClass>,
        restart_posture: Option<RestartWithRecordingClass>,
        replay_surface: Option<ReplaySurfaceClass>,
        exec_ctx: Option<&str>,
    ) -> ChronologyReplaySupportRow {
        let requires_inspector = replay_surface
            .map(|s| s.requires_inspector_state_attestation())
            .unwrap_or(false);
        let requires_mapping = replay_surface
            .map(|s| s.requires_mapping_quality_attestation())
            .unwrap_or(false);
        ChronologyReplaySupportRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            replay_support_class: replay_support,
            capture_state_class: capture_state,
            mapping_quality_badge_class: mapping_quality,
            replay_scope_class: replay_scope,
            inspector_state_class: inspector_state,
            restart_with_recording_class: restart_posture,
            replay_surface_class: replay_surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![CHRONOLOGY_REPLAY_SUPPORT_FIXTURE_DIR.to_owned()],
            disclosure_ref: None,
            execution_context_id_binding: exec_ctx.map(str::to_owned),
            attests_inspector_state_preserved: requires_inspector,
            attests_mapping_quality_preserved: requires_mapping,
            attests_replay_read_only: matches!(
                row_class,
                ChronologyRowClass::ReplaySurfaceBinding
            ) && !matches!(
                replay_surface,
                Some(ReplaySurfaceClass::SupportExportSurface)
            ),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-06-03T12:00:00Z".to_owned(),
        }
    }

    fn make_quality_row(lane: DebugLaneClass) -> ChronologyReplaySupportRow {
        let row_id = format!("row:{}:quality", lane.as_str());
        ChronologyReplaySupportRow {
            row_id,
            lane_class: lane,
            row_class: ChronologyRowClass::ChronologyQuality,
            support_class: SupportClass::LaunchStable,
            replay_support_class: None,
            capture_state_class: None,
            mapping_quality_badge_class: None,
            replay_scope_class: None,
            inspector_state_class: None,
            restart_with_recording_class: None,
            replay_surface_class: None,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![
                CHRONOLOGY_REPLAY_SUPPORT_DOC_REF.to_owned(),
                CHRONOLOGY_REPLAY_SUPPORT_FIXTURE_DIR.to_owned(),
            ],
            disclosure_ref: None,
            execution_context_id_binding: None,
            attests_inspector_state_preserved: false,
            attests_mapping_quality_preserved: false,
            attests_replay_read_only: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-06-03T12:00:00Z".to_owned(),
        }
    }

    fn sample_rows_for_lane(lane: DebugLaneClass) -> Vec<ChronologyReplaySupportRow> {
        let lane_str = lane.as_str();
        let mut rows = vec![make_quality_row(lane)];

        // Support class admissions.
        for sc in ReplaySupportClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:support:{}", sc.as_str()),
                lane,
                ChronologyRowClass::SupportClassAdmission,
                Some(sc),
                None, None, None, None, None, None, None,
            ));
        }
        // Capture state admissions.
        for cs in ChronologyCaptureStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:capture:{}", cs.as_str()),
                lane,
                ChronologyRowClass::CaptureStateAdmission,
                None,
                Some(cs),
                None, None, None, None, None, None,
            ));
        }
        // Mapping quality admissions.
        for mq in MappingQualityBadgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:mapping:{}", mq.as_str()),
                lane,
                ChronologyRowClass::MappingQualityBadgeAdmission,
                None, None,
                Some(mq),
                None, None, None, None, None,
            ));
        }
        // Replay scope admissions.
        for rs in ReplayScopeClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:scope:{}", rs.as_str()),
                lane,
                ChronologyRowClass::ReplayScopeAdmission,
                None, None, None,
                Some(rs),
                None, None, None, None,
            ));
        }
        // Inspector state admissions.
        for is in InspectorStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:inspector:{}", is.as_str()),
                lane,
                ChronologyRowClass::InspectorStateAdmission,
                None, None, None, None,
                Some(is),
                None, None, None,
            ));
        }
        // Restart posture admissions.
        for rp in RestartWithRecordingClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:restart:{}", rp.as_str()),
                lane,
                ChronologyRowClass::RestartPostureAdmission,
                None, None, None, None, None,
                Some(rp),
                None, None,
            ));
        }
        // Replay surface bindings.
        for surface in ReplaySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(make_row(
                &format!("row:{lane_str}:surface:{}", surface.as_str()),
                lane,
                ChronologyRowClass::ReplaySurfaceBinding,
                None, None, None, None, None, None,
                Some(surface),
                None,
            ));
        }
        // Lineage admission.
        rows.push(make_row(
            &format!("row:{lane_str}:lineage"),
            lane,
            ChronologyRowClass::LineageAdmission,
            None, None, None, None, None, None, None,
            Some(&format!("exec:m4:{lane_str}:chronology_replay_support_lineage")),
        ));

        rows
    }

    fn sample_input() -> ChronologyReplaySupportInput {
        let mut rows = Vec::new();
        for lane in DebugLaneClass::REQUIRED {
            rows.extend(sample_rows_for_lane(lane));
        }

        let packet_id =
            "packet:m4:qualify_chronology_capture_and_replay_support_classes:stable".to_owned();
        let consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .map(|&s| ChronologyConsumerProjection {
                consumer_surface: s,
                packet_id_ref: packet_id.clone(),
                preserves_replay_support_vocabulary: true,
                preserves_capture_state_vocabulary: true,
                preserves_mapping_quality_vocabulary: true,
                preserves_inspector_state_vocabulary: true,
                preserves_restart_posture_vocabulary: true,
            })
            .collect();

        ChronologyReplaySupportInput {
            packet_id,
            workflow_or_surface_id:
                "workflow.debug.qualify_chronology_capture_and_replay_support_classes.stable"
                    .to_owned(),
            generated_at: "2026-06-03T12:00:00Z".to_owned(),
            rows,
            consumer_projections,
        }
    }

    #[test]
    fn all_as_str_are_stable_tokens() {
        assert_eq!(DebugLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(DebugLaneClass::NotebookBridgeLane.as_str(), "notebook_bridge_lane");
        assert_eq!(ReplaySupportClass::Supported.as_str(), "supported");
        assert_eq!(ReplaySupportClass::PolicyBlocked.as_str(), "policy_blocked");
        assert_eq!(ChronologyCaptureStateClass::Recorded.as_str(), "recorded");
        assert_eq!(
            ChronologyCaptureStateClass::RestartWithRecordingAvailable.as_str(),
            "restart_with_recording_available"
        );
        assert_eq!(MappingQualityBadgeClass::Exact.as_str(), "exact");
        assert_eq!(MappingQualityBadgeClass::Mismatched.as_str(), "mismatched");
        assert_eq!(ReplayScopeClass::NotebookBridgeScope.as_str(), "notebook_bridge_scope");
        assert_eq!(InspectorStateClass::Snapshot.as_str(), "snapshot");
        assert_eq!(RestartWithRecordingClass::Available.as_str(), "available");
        assert_eq!(
            RestartWithRecordingClass::UnavailableUnsupportedBackend.as_str(),
            "unavailable_unsupported_backend"
        );
        assert_eq!(FindingKind::MissingEvidenceClass.as_str(), "missing_evidence_class");
        assert_eq!(
            FindingKind::ReplaySurfaceMissingReadOnlyAttestation.as_str(),
            "replay_surface_missing_read_only_attestation"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = ChronologyReplaySupportTruthPacket::materialize(sample_input());
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
                "support:m4:qualify_chronology_capture_and_replay_support_classes",
                "2026-06-03T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn unbound_evidence_blocks_stable() {
        let mut input = sample_input();
        if let Some(row) = input
            .rows
            .iter_mut()
            .find(|r| matches!(r.row_class, ChronologyRowClass::ChronologyQuality)
                && r.lane_class == DebugLaneClass::LocalLane)
        {
            row.evidence_class = EvidenceClass::EvidenceUnbound;
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingEvidenceClass));
    }

    #[test]
    fn missing_replay_support_class_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|r| {
            !(matches!(r.row_class, ChronologyRowClass::SupportClassAdmission)
                && r.replay_support_class == Some(ReplaySupportClass::PolicyBlocked)
                && r.lane_class == DebugLaneClass::LocalLane)
        });
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingReplaySupportClassCoverage));
    }

    #[test]
    fn missing_capture_state_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|r| {
            !(matches!(r.row_class, ChronologyRowClass::CaptureStateAdmission)
                && r.capture_state_class
                    == Some(ChronologyCaptureStateClass::RestartWithRecordingAvailable)
                && r.lane_class == DebugLaneClass::LocalLane)
        });
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingCaptureStateCoverage));
    }

    #[test]
    fn missing_mapping_quality_badge_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|r| {
            !(matches!(r.row_class, ChronologyRowClass::MappingQualityBadgeAdmission)
                && r.mapping_quality_badge_class == Some(MappingQualityBadgeClass::Mismatched)
                && r.lane_class == DebugLaneClass::LocalLane)
        });
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingMappingQualityBadgeCoverage));
    }

    #[test]
    fn missing_inspector_state_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|r| {
            !(matches!(r.row_class, ChronologyRowClass::InspectorStateAdmission)
                && r.inspector_state_class == Some(InspectorStateClass::Unavailable)
                && r.lane_class == DebugLaneClass::LocalLane)
        });
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingInspectorStateCoverage));
    }

    #[test]
    fn missing_restart_posture_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|r| {
            !(matches!(r.row_class, ChronologyRowClass::RestartPostureAdmission)
                && r.restart_with_recording_class
                    == Some(RestartWithRecordingClass::UnavailablePolicyBlocked)
                && r.lane_class == DebugLaneClass::LocalLane)
        });
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingRestartPostureCoverage));
    }

    #[test]
    fn replay_surface_missing_inspector_state_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, ChronologyRowClass::ReplaySurfaceBinding)
                && row.lane_class == DebugLaneClass::LocalLane
                && row.replay_surface_class == Some(ReplaySurfaceClass::VariableInspectorSurface)
            {
                row.attests_inspector_state_preserved = false;
                break;
            }
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|f| {
            f.finding_kind == FindingKind::ReplaySurfaceMissingInspectorStateAttestation
        }));
    }

    #[test]
    fn replay_surface_missing_mapping_quality_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, ChronologyRowClass::ReplaySurfaceBinding)
                && row.lane_class == DebugLaneClass::LocalLane
                && row.replay_surface_class == Some(ReplaySurfaceClass::CallStackSurface)
            {
                row.attests_mapping_quality_preserved = false;
                break;
            }
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|f| {
            f.finding_kind == FindingKind::ReplaySurfaceMissingMappingQualityAttestation
        }));
    }

    #[test]
    fn replay_surface_missing_read_only_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, ChronologyRowClass::ReplaySurfaceBinding)
                && row.lane_class == DebugLaneClass::LocalLane
                && row.replay_surface_class == Some(ReplaySurfaceClass::TimelineScrubberSurface)
            {
                row.attests_replay_read_only = false;
                break;
            }
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|f| {
            f.finding_kind == FindingKind::ReplaySurfaceMissingReadOnlyAttestation
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        if let Some(row) = input
            .rows
            .iter_mut()
            .find(|r| matches!(r.row_class, ChronologyRowClass::ChronologyQuality)
                && r.lane_class == DebugLaneClass::LocalLane)
        {
            row.support_class = SupportClass::LaunchStableBelow;
            row.disclosure_ref = None;
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        if let Some(row) = input
            .rows
            .iter_mut()
            .find(|r| matches!(r.row_class, ChronologyRowClass::LineageAdmission)
                && r.lane_class == DebugLaneClass::LocalLane)
        {
            row.execution_context_id_binding = None;
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|f| {
            f.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn missing_consumer_projection_blocks() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|p| p.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_inspector_state_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_inspector_state_vocabulary = false;
            }
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::InspectorStateVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks() {
        let mut input = sample_input();
        if let Some(row) = input.rows.first_mut() {
            row.raw_source_material_excluded = false;
        }
        let packet = ChronologyReplaySupportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
