//! Stable profiler and tracing-hooks hardening truth packet for the M4 stable lane.
//!
//! This module pins how profiler, trace, replay-capability, and comparison
//! surfaces serialize one canonical truth across the profiler/tracing lane so
//! that flamegraph, timeline, call-tree, regression-summary, replay-controls,
//! and profile-session surfaces all read one attributable execution-context
//! object. Surfaces MUST emit the typed profile-session descriptor,
//! trace-bundle manifest, capture-mode label, mapping-quality state, and
//! baseline/comparison key. Surfaces MUST NOT imply M5-class capture, replay,
//! or regression depth on M4.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `profiler_quality` row cannot prove:
//!
//! - the eight profiler wedges (`profile_session_descriptor`,
//!   `trace_bundle_manifest`, `capture_mode_label`, `mapping_quality_state`,
//!   `baseline_comparison_key`, `replay_capability_descriptor`,
//!   `reverse_step_disabled_reason`, `export_redaction_summary`) each have a
//!   structured `wedge_admission` row,
//! - the six capture-state classes (`live`, `cached`, `imported`, `stale`,
//!   `not_recorded`, `disabled_with_reason`) each have a structured
//!   `capture_state_admission` row so users never mistake imported or stale
//!   evidence for live runtime truth,
//! - the four origin classes (`local_origin`, `remote_origin`,
//!   `ci_artifact_origin`, `imported_bundle_origin`) each have a structured
//!   `origin_class_admission` row so evidence provenance stays explicit,
//! - the two build-mode classes (`debug_mode`, `release_mode`) each have a
//!   structured `build_mode_admission` row,
//! - the two run-class classes (`warm_run`, `cold_run`) each have a structured
//!   `run_class_admission` row,
//! - the three confounder classes (`hardware_class`, `power_state`,
//!   `thermal_state`) each have a structured `confounder_admission` row so
//!   regression comparability is never implied when confounders are undeclared,
//! - the five replay-state classes (`supported`, `limited`, `record_only`,
//!   `profile_only`, `import_view_only`) each have a structured
//!   `replay_state_admission` row so replay capability is never overstated,
//! - the six profiler surfaces (`flamegraph_surface`, `timeline_surface`,
//!   `call_tree_surface`, `regression_summary_surface`, `replay_controls_surface`,
//!   `profile_session_surface`) each have a `surface_binding` row attesting they
//!   preserve the capture-state, origin, build-mode, run-class, confounder, and
//!   replay-state vocabularies they must preserve,
//! - one stable `execution_context_id` threads through every emitted profile
//!   session, trace bundle, comparison packet, and support export via a
//!   `lineage_admission` row.
//!
//! Every row binds closed `profiler_lane_class`, `profiler_row_class`,
//! `support_class`, `wedge_class`, `capture_state_class`, `origin_class`,
//! `build_mode_class`, `run_class_class`, `confounder_class`,
//! `replay_state_class`, `profiler_surface_class`, `evidence_class`,
//! `known_limit_class`, `downgrade_automation_class`, and `confidence_class`
//! vocabularies plus an `evidence_refs` array and a `disclosure_ref` whenever
//! the row is narrowed below launch-stable, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw trace payloads, raw
//! profile bodies, raw flamegraph nodes, raw memory snapshots, raw command
//! lines, raw process environment bytes, secrets, or ambient credentials past
//! the boundary. A row that claims `launch_stable` while leaving its known
//! limit, downgrade automation, or evidence class unbound is refused; the
//! validator narrows below launch-stable instead of inheriting an adjacent
//! certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ProfilerTruthPacket`].
pub const PROFILER_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_the_stable_profiler_and_tracing_hooks_needed_truth_stable_packet";

/// Stable record-kind tag for [`ProfilerTruthSupportExport`].
pub const PROFILER_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_the_stable_profiler_and_tracing_hooks_needed_truth_support_export";

/// Integer schema version for the profiler truth packet.
pub const PROFILER_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROFILER_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_the_stable_profiler_and_tracing_hooks_needed_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PROFILER_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-the-stable-profiler-and-tracing-hooks-needed.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PROFILER_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-the-stable-profiler-and-tracing-hooks-needed.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PROFILER_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_the_stable_profiler_and_tracing_hooks_needed";

/// Repo-relative path of the checked-in stable packet.
pub const PROFILER_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_the_stable_profiler_and_tracing_hooks_needed_truth_packet.json";


/// Closed profiler lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilerLaneClass {
    /// Local-host profiler lane.
    LocalLane,
    /// Remote / helper profiler lane.
    RemoteHelperLane,
    /// Container-attached profiler lane.
    ContainerLane,
    /// CI-import profiler lane.
    CiImportLane,
}

impl ProfilerLaneClass {
    /// Every required profiler lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::CiImportLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::CiImportLane => "ci_import_lane",
        }
    }
}


/// Closed profiler row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilerRowClass {
    /// The lane's headline profiler qualification row.
    ProfilerQuality,
    /// A row admitting one of the required profiler wedges.
    WedgeAdmission,
    /// A row admitting one capture-state class.
    CaptureStateAdmission,
    /// A row admitting one origin class.
    OriginClassAdmission,
    /// A row admitting one build-mode class.
    BuildModeAdmission,
    /// A row admitting one run-class class.
    RunClassAdmission,
    /// A row admitting one confounder class.
    ConfounderAdmission,
    /// A row admitting one replay-state class.
    ReplayStateAdmission,
    /// A row binding one profiler surface.
    SurfaceBinding,
    /// A row binding the stable `execution_context_id` lineage.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl ProfilerRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfilerQuality => "profiler_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::CaptureStateAdmission => "capture_state_admission",
            Self::OriginClassAdmission => "origin_class_admission",
            Self::BuildModeAdmission => "build_mode_admission",
            Self::RunClassAdmission => "run_class_admission",
            Self::ConfounderAdmission => "confounder_admission",
            Self::ReplayStateAdmission => "replay_state_admission",
            Self::SurfaceBinding => "surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    pub const fn requires_capture_state(self) -> bool {
        matches!(self, Self::CaptureStateAdmission)
    }

    pub const fn requires_origin_class(self) -> bool {
        matches!(self, Self::OriginClassAdmission)
    }

    pub const fn requires_build_mode(self) -> bool {
        matches!(self, Self::BuildModeAdmission)
    }

    pub const fn requires_run_class(self) -> bool {
        matches!(self, Self::RunClassAdmission)
    }

    pub const fn requires_confounder(self) -> bool {
        matches!(self, Self::ConfounderAdmission)
    }

    pub const fn requires_replay_state(self) -> bool {
        matches!(self, Self::ReplayStateAdmission)
    }

    pub const fn requires_profiler_surface(self) -> bool {
        matches!(self, Self::SurfaceBinding)
    }
}


/// Closed support-class vocabulary applied to a profiler row.
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


/// Closed profiler wedge vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Profile-session descriptor wedge.
    ProfileSessionDescriptor,
    /// Trace-bundle manifest wedge.
    TraceBundleManifest,
    /// Capture-mode label wedge.
    CaptureModeLabel,
    /// Mapping-quality state wedge.
    MappingQualityState,
    /// Baseline/comparison key wedge.
    BaselineComparisonKey,
    /// Replay-capability descriptor wedge.
    ReplayCapabilityDescriptor,
    /// Reverse-step disabled-reason wedge.
    ReverseStepDisabledReason,
    /// Export/redaction summary wedge.
    ExportRedactionSummary,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 8] = [
        Self::ProfileSessionDescriptor,
        Self::TraceBundleManifest,
        Self::CaptureModeLabel,
        Self::MappingQualityState,
        Self::BaselineComparisonKey,
        Self::ReplayCapabilityDescriptor,
        Self::ReverseStepDisabledReason,
        Self::ExportRedactionSummary,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileSessionDescriptor => "profile_session_descriptor",
            Self::TraceBundleManifest => "trace_bundle_manifest",
            Self::CaptureModeLabel => "capture_mode_label",
            Self::MappingQualityState => "mapping_quality_state",
            Self::BaselineComparisonKey => "baseline_comparison_key",
            Self::ReplayCapabilityDescriptor => "replay_capability_descriptor",
            Self::ReverseStepDisabledReason => "reverse_step_disabled_reason",
            Self::ExportRedactionSummary => "export_redaction_summary",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed capture-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureStateClass {
    /// Live capture state.
    Live,
    /// Cached capture state.
    Cached,
    /// Imported capture state.
    Imported,
    /// Stale capture state.
    Stale,
    /// Not recorded capture state.
    NotRecorded,
    /// Disabled with reason capture state.
    DisabledWithReason,
    /// The row is not bound to a capture state.
    NotApplicable,
}

impl CaptureStateClass {
    /// Every required capture state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Live,
        Self::Cached,
        Self::Imported,
        Self::Stale,
        Self::NotRecorded,
        Self::DisabledWithReason,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::NotRecorded => "not_recorded",
            Self::DisabledWithReason => "disabled_with_reason",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed origin-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    /// Local origin.
    LocalOrigin,
    /// Remote origin.
    RemoteOrigin,
    /// CI artifact origin.
    CiArtifactOrigin,
    /// Imported bundle origin.
    ImportedBundleOrigin,
    /// The row is not bound to an origin class.
    NotApplicable,
}

impl OriginClass {
    /// Every required origin class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::LocalOrigin,
        Self::RemoteOrigin,
        Self::CiArtifactOrigin,
        Self::ImportedBundleOrigin,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOrigin => "local_origin",
            Self::RemoteOrigin => "remote_origin",
            Self::CiArtifactOrigin => "ci_artifact_origin",
            Self::ImportedBundleOrigin => "imported_bundle_origin",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed build-mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildModeClass {
    /// Debug build mode.
    DebugMode,
    /// Release build mode.
    ReleaseMode,
    /// The row is not bound to a build mode.
    NotApplicable,
}

impl BuildModeClass {
    /// Every required build mode for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 2] = [
        Self::DebugMode,
        Self::ReleaseMode,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebugMode => "debug_mode",
            Self::ReleaseMode => "release_mode",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed run-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunClassClass {
    /// Warm run class.
    WarmRun,
    /// Cold run class.
    ColdRun,
    /// The row is not bound to a run class.
    NotApplicable,
}

impl RunClassClass {
    /// Every required run class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 2] = [
        Self::WarmRun,
        Self::ColdRun,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmRun => "warm_run",
            Self::ColdRun => "cold_run",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed confounder vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfounderClass {
    /// Hardware class confounder.
    HardwareClass,
    /// Power state confounder.
    PowerState,
    /// Thermal state confounder.
    ThermalState,
    /// The row is not bound to a confounder.
    NotApplicable,
}

impl ConfounderClass {
    /// Every required confounder for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::HardwareClass,
        Self::PowerState,
        Self::ThermalState,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardwareClass => "hardware_class",
            Self::PowerState => "power_state",
            Self::ThermalState => "thermal_state",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed replay-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayStateClass {
    /// Replay is supported.
    Supported,
    /// Replay is limited.
    Limited,
    /// Recording only.
    RecordOnly,
    /// Profile only.
    ProfileOnly,
    /// Import/view only.
    ImportViewOnly,
    /// The row is not bound to a replay state.
    NotApplicable,
}

impl ReplayStateClass {
    /// Every required replay state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Supported,
        Self::Limited,
        Self::RecordOnly,
        Self::ProfileOnly,
        Self::ImportViewOnly,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::RecordOnly => "record_only",
            Self::ProfileOnly => "profile_only",
            Self::ImportViewOnly => "import_view_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}


/// Closed profiler-surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilerSurfaceClass {
    /// Flamegraph surface.
    FlamegraphSurface,
    /// Timeline surface.
    TimelineSurface,
    /// Call-tree surface.
    CallTreeSurface,
    /// Regression summary surface.
    RegressionSummarySurface,
    /// Replay controls surface.
    ReplayControlsSurface,
    /// Profile session surface.
    ProfileSessionSurface,
    /// The row is not bound to a surface.
    NotApplicable,
}

impl ProfilerSurfaceClass {
    /// Every required profiler surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::FlamegraphSurface,
        Self::TimelineSurface,
        Self::CallTreeSurface,
        Self::RegressionSummarySurface,
        Self::ReplayControlsSurface,
        Self::ProfileSessionSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FlamegraphSurface => "flamegraph_surface",
            Self::TimelineSurface => "timeline_surface",
            Self::CallTreeSurface => "call_tree_surface",
            Self::RegressionSummarySurface => "regression_summary_surface",
            Self::ReplayControlsSurface => "replay_controls_surface",
            Self::ProfileSessionSurface => "profile_session_surface",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this surface MUST attest that it preserves the
    /// capture-state vocabulary.
    pub const fn requires_capture_state_attestation(self) -> bool {
        matches!(
            self,
            Self::FlamegraphSurface
                | Self::TimelineSurface
                | Self::CallTreeSurface
                | Self::RegressionSummarySurface
                | Self::ReplayControlsSurface
                | Self::ProfileSessionSurface
        )
    }

    /// True when this surface MUST attest that it preserves the
    /// replay-state vocabulary.
    pub const fn requires_replay_state_attestation(self) -> bool {
        matches!(
            self,
            Self::RegressionSummarySurface | Self::ReplayControlsSurface | Self::ProfileSessionSurface
        )
    }
}


/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by an automated functional / unit suite.
    AutomatedFunctionalEvidence,
    /// The row is backed by a conformance / interoperability suite.
    ConformanceSuiteEvidence,
    /// The row is backed by a failure / recovery drill.
    FailureRecoveryDrillEvidence,
    /// The row is backed by design-QA / UX validation.
    DesignQaEvidence,
    /// The row is backed by release-evidence review.
    ReleaseEvidenceReview,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a benchmark / fitness-function capture.
    BenchmarkEvidence,
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}


/// Closed known-limit vocabulary attached to a profiler row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the container subset.
    ContainerSubsetOnly,
    /// The lane only certifies the CI-import subset.
    CiImportSubsetOnly,
    /// The lane only certifies a subset of the profiler wedges.
    WedgeSubsetOnly,
    /// The lane only certifies a subset of the capture states.
    CaptureStateSubsetOnly,
    /// The lane only certifies a subset of the origin classes.
    OriginClassSubsetOnly,
    /// The lane only certifies a subset of the build modes.
    BuildModeSubsetOnly,
    /// The lane only certifies a subset of the run classes.
    RunClassSubsetOnly,
    /// The lane only certifies a subset of the confounders.
    ConfounderSubsetOnly,
    /// The lane only certifies a subset of the replay states.
    ReplayStateSubsetOnly,
    /// The lane only certifies a subset of the profiler surfaces.
    ProfilerSurfaceSubsetOnly,
    /// Richer replay, reverse-debug, and chronology surfaces are explicitly
    /// outside the M4 stable contract.
    RicherReplayOutsideM4,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::ContainerSubsetOnly => "container_subset_only",
            Self::CiImportSubsetOnly => "ci_import_subset_only",
            Self::WedgeSubsetOnly => "wedge_subset_only",
            Self::CaptureStateSubsetOnly => "capture_state_subset_only",
            Self::OriginClassSubsetOnly => "origin_class_subset_only",
            Self::BuildModeSubsetOnly => "build_mode_subset_only",
            Self::RunClassSubsetOnly => "run_class_subset_only",
            Self::ConfounderSubsetOnly => "confounder_subset_only",
            Self::ReplayStateSubsetOnly => "replay_state_subset_only",
            Self::ProfilerSurfaceSubsetOnly => "profiler_surface_subset_only",
            Self::RicherReplayOutsideM4 => "richer_replay_outside_m4",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}


/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when a required capture-state admission is missing.
    AutoNarrowOnCaptureStateGap,
    /// Automatically narrow when a required origin-class admission is missing.
    AutoNarrowOnOriginClassGap,
    /// Automatically narrow when a required build-mode admission is missing.
    AutoNarrowOnBuildModeGap,
    /// Automatically narrow when a required run-class admission is missing.
    AutoNarrowOnRunClassGap,
    /// Automatically narrow when a required confounder admission is missing.
    AutoNarrowOnConfounderGap,
    /// Automatically narrow when a required replay-state admission is missing.
    AutoNarrowOnReplayStateGap,
    /// Automatically narrow when a required surface binding is missing.
    AutoNarrowOnProfilerSurfaceGap,
    /// Automatically narrow when the lineage object breaks.
    AutoNarrowOnLineageBreak,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnCaptureStateGap => "auto_narrow_on_capture_state_gap",
            Self::AutoNarrowOnOriginClassGap => "auto_narrow_on_origin_class_gap",
            Self::AutoNarrowOnBuildModeGap => "auto_narrow_on_build_mode_gap",
            Self::AutoNarrowOnRunClassGap => "auto_narrow_on_run_class_gap",
            Self::AutoNarrowOnConfounderGap => "auto_narrow_on_confounder_gap",
            Self::AutoNarrowOnReplayStateGap => "auto_narrow_on_replay_state_gap",
            Self::AutoNarrowOnProfilerSurfaceGap => "auto_narrow_on_profiler_surface_gap",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}


/// Closed confidence-class vocabulary for a profiler row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
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
    /// Packet certifies a stable claim.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet has a blocker finding.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
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
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}


/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    WrongRecordKind,
    WrongSchemaVersion,
    MissingIdentity,
    MissingProfilerLaneCoverage,
    MissingWedgeCoverage,
    MissingCaptureStateCoverage,
    MissingOriginClassCoverage,
    MissingBuildModeCoverage,
    MissingRunClassCoverage,
    MissingConfounderCoverage,
    MissingReplayStateCoverage,
    MissingProfilerSurfaceCoverage,
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
    CaptureStateNotApplicable,
    CaptureStateNotPermittedOnRowClass,
    OriginClassNotApplicable,
    OriginClassNotPermittedOnRowClass,
    BuildModeNotApplicable,
    BuildModeNotPermittedOnRowClass,
    RunClassNotApplicable,
    RunClassNotPermittedOnRowClass,
    ConfounderNotApplicable,
    ConfounderNotPermittedOnRowClass,
    ReplayStateNotApplicable,
    ReplayStateNotPermittedOnRowClass,
    ProfilerSurfaceNotApplicable,
    ProfilerSurfaceNotPermittedOnRowClass,
    ProfilerSurfaceMissingCaptureStateAttestation,
    ProfilerSurfaceMissingReplayStateAttestation,
    LineageAdmissionMissingExecutionContextId,
    RawSourceMaterialPresent,
    SecretsPresent,
    AmbientAuthorityPresent,
    MissingConsumerProjection,
    ConsumerProjectionDrift,
    LaneVocabularyCollapsed,
    RowClassVocabularyCollapsed,
    SupportClassVocabularyCollapsed,
    WedgeVocabularyCollapsed,
    CaptureStateVocabularyCollapsed,
    OriginClassVocabularyCollapsed,
    BuildModeVocabularyCollapsed,
    RunClassVocabularyCollapsed,
    ConfounderVocabularyCollapsed,
    ReplayStateVocabularyCollapsed,
    ProfilerSurfaceVocabularyCollapsed,
    KnownLimitVocabularyCollapsed,
    DowngradeAutomationVocabularyCollapsed,
    EvidenceClassVocabularyCollapsed,
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingProfilerLaneCoverage => "missing_profiler_lane_coverage",
            Self::MissingWedgeCoverage => "missing_wedge_coverage",
            Self::MissingCaptureStateCoverage => "missing_capture_state_coverage",
            Self::MissingOriginClassCoverage => "missing_origin_class_coverage",
            Self::MissingBuildModeCoverage => "missing_build_mode_coverage",
            Self::MissingRunClassCoverage => "missing_run_class_coverage",
            Self::MissingConfounderCoverage => "missing_confounder_coverage",
            Self::MissingReplayStateCoverage => "missing_replay_state_coverage",
            Self::MissingProfilerSurfaceCoverage => "missing_profiler_surface_coverage",
            Self::MissingLineageAdmission => "missing_lineage_admission",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => "downgrade_automation_missing_disclosure_ref",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::CaptureStateNotApplicable => "capture_state_not_applicable",
            Self::CaptureStateNotPermittedOnRowClass => "capture_state_not_permitted_on_row_class",
            Self::OriginClassNotApplicable => "origin_class_not_applicable",
            Self::OriginClassNotPermittedOnRowClass => "origin_class_not_permitted_on_row_class",
            Self::BuildModeNotApplicable => "build_mode_not_applicable",
            Self::BuildModeNotPermittedOnRowClass => "build_mode_not_permitted_on_row_class",
            Self::RunClassNotApplicable => "run_class_not_applicable",
            Self::RunClassNotPermittedOnRowClass => "run_class_not_permitted_on_row_class",
            Self::ConfounderNotApplicable => "confounder_not_applicable",
            Self::ConfounderNotPermittedOnRowClass => "confounder_not_permitted_on_row_class",
            Self::ReplayStateNotApplicable => "replay_state_not_applicable",
            Self::ReplayStateNotPermittedOnRowClass => "replay_state_not_permitted_on_row_class",
            Self::ProfilerSurfaceNotApplicable => "profiler_surface_not_applicable",
            Self::ProfilerSurfaceNotPermittedOnRowClass => "profiler_surface_not_permitted_on_row_class",
            Self::ProfilerSurfaceMissingCaptureStateAttestation => "profiler_surface_missing_capture_state_attestation",
            Self::ProfilerSurfaceMissingReplayStateAttestation => "profiler_surface_missing_replay_state_attestation",
            Self::LineageAdmissionMissingExecutionContextId => "lineage_admission_missing_execution_context_id",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::CaptureStateVocabularyCollapsed => "capture_state_vocabulary_collapsed",
            Self::OriginClassVocabularyCollapsed => "origin_class_vocabulary_collapsed",
            Self::BuildModeVocabularyCollapsed => "build_mode_vocabulary_collapsed",
            Self::RunClassVocabularyCollapsed => "run_class_vocabulary_collapsed",
            Self::ConfounderVocabularyCollapsed => "confounder_vocabulary_collapsed",
            Self::ReplayStateVocabularyCollapsed => "replay_state_vocabulary_collapsed",
            Self::ProfilerSurfaceVocabularyCollapsed => "profiler_surface_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => "downgrade_automation_vocabulary_collapsed",
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}


/// Consumer surface that must inherit the profiler packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerProjectionSurface {
    FlamegraphSurface,
    TimelineSurface,
    CallTreeSurface,
    RegressionSummarySurface,
    ReplayControlsSurface,
    ProfileSessionSurface,
    CliHeadless,
    EvidenceExport,
    SupportExport,
    ReleaseProofIndex,
    HelpAbout,
    ConformanceDashboard,
}

impl ConsumerProjectionSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 12] = [
        Self::FlamegraphSurface,
        Self::TimelineSurface,
        Self::CallTreeSurface,
        Self::RegressionSummarySurface,
        Self::ReplayControlsSurface,
        Self::ProfileSessionSurface,
        Self::CliHeadless,
        Self::EvidenceExport,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FlamegraphSurface => "flamegraph_surface",
            Self::TimelineSurface => "timeline_surface",
            Self::CallTreeSurface => "call_tree_surface",
            Self::RegressionSummarySurface => "regression_summary_surface",
            Self::ReplayControlsSurface => "replay_controls_surface",
            Self::ProfileSessionSurface => "profile_session_surface",
            Self::CliHeadless => "cli_headless",
            Self::EvidenceExport => "evidence_export",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}


/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
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


/// One profiler truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilerRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Profiler lane this row certifies.
    pub lane_class: ProfilerLaneClass,
    /// Row class.
    pub row_class: ProfilerRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Capture state bound by the row (or `not_applicable`).
    pub capture_state_class: CaptureStateClass,
    /// Origin class bound by the row (or `not_applicable`).
    pub origin_class: OriginClass,
    /// Build mode bound by the row (or `not_applicable`).
    pub build_mode_class: BuildModeClass,
    /// Run class bound by the row (or `not_applicable`).
    pub run_class_class: RunClassClass,
    /// Confounder bound by the row (or `not_applicable`).
    pub confounder_class: ConfounderClass,
    /// Replay state bound by the row (or `not_applicable`).
    pub replay_state_class: ReplayStateClass,
    /// Profiler surface bound by the row (or `not_applicable`).
    pub profiler_surface_class: ProfilerSurfaceClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For surface_binding rows, true when the surface preserves the
    /// capture-state vocabulary verbatim.
    #[serde(default)]
    pub attests_capture_state_preserved: bool,
    /// For surface_binding rows, true when the surface preserves the
    /// replay-state vocabulary verbatim.
    #[serde(default)]
    pub attests_replay_state_preserved: bool,
    /// True when raw trace payloads, raw profile bodies, raw
    /// flamegraph nodes, raw memory snapshots, raw command lines, or
    /// raw process environment bytes are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl ProfilerRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}


/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilerConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerProjectionSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Profiler packet id consumed by the projection.
    pub profiler_truth_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the wedge vocabulary is preserved verbatim.
    pub preserves_wedge_vocabulary: bool,
    /// True when the capture-state vocabulary is preserved verbatim.
    pub preserves_capture_state_vocabulary: bool,
    /// True when the origin-class vocabulary is preserved verbatim.
    pub preserves_origin_class_vocabulary: bool,
    /// True when the build-mode vocabulary is preserved verbatim.
    pub preserves_build_mode_vocabulary: bool,
    /// True when the run-class vocabulary is preserved verbatim.
    pub preserves_run_class_vocabulary: bool,
    /// True when the confounder vocabulary is preserved verbatim.
    pub preserves_confounder_vocabulary: bool,
    /// True when the replay-state vocabulary is preserved verbatim.
    pub preserves_replay_state_vocabulary: bool,
    /// True when the profiler-surface vocabulary is preserved verbatim.
    pub preserves_profiler_surface_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl ProfilerConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.profiler_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_capture_state_vocabulary
            && self.preserves_origin_class_vocabulary
            && self.preserves_build_mode_vocabulary
            && self.preserves_run_class_vocabulary
            && self.preserves_confounder_vocabulary
            && self.preserves_replay_state_vocabulary
            && self.preserves_profiler_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}


/// Constructor input for [`ProfilerTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilerTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Profiler lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ProfilerLaneClass>,
    /// Profiler rows.
    #[serde(default)]
    pub rows: Vec<ProfilerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ProfilerConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}


/// Runtime-owned packet certifying profiler and tracing-hook truth at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilerTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Profiler lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ProfilerLaneClass>,
    /// Profiler rows.
    #[serde(default)]
    pub rows: Vec<ProfilerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ProfilerConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ProfilerTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ProfilerTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PROFILER_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROFILER_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable profiler invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerProjectionSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter()
            .map(ProfilerLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(ProfilerRowClass::as_str).collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique capture-state tokens observed across rows.
    pub fn capture_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.capture_state_class);
        }
        set.into_iter().map(CaptureStateClass::as_str).collect()
    }

    /// Returns the unique origin-class tokens observed across rows.
    pub fn origin_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.origin_class);
        }
        set.into_iter().map(OriginClass::as_str).collect()
    }

    /// Returns the unique build-mode tokens observed across rows.
    pub fn build_mode_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.build_mode_class);
        }
        set.into_iter().map(BuildModeClass::as_str).collect()
    }

    /// Returns the unique run-class tokens observed across rows.
    pub fn run_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.run_class_class);
        }
        set.into_iter().map(RunClassClass::as_str).collect()
    }

    /// Returns the unique confounder tokens observed across rows.
    pub fn confounder_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.confounder_class);
        }
        set.into_iter().map(ConfounderClass::as_str).collect()
    }

    /// Returns the unique replay-state tokens observed across rows.
    pub fn replay_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.replay_state_class);
        }
        set.into_iter().map(ReplayStateClass::as_str).collect()
    }

    /// Returns the unique profiler-surface tokens observed across rows.
    pub fn profiler_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.profiler_surface_class);
        }
        set.into_iter().map(ProfilerSurfaceClass::as_str).collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ProfilerTruthSupportExport {
        ProfilerTruthSupportExport {
            record_kind: PROFILER_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROFILER_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            profiler_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            profiler_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != PROFILER_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "profiler truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != PROFILER_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "profiler truth packet has the wrong schema version",
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
                FindingKind::MissingProfilerLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered profiler lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingProfilerLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers profiler lane {}", lane.as_str()),
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
                        "row {} admits raw trace payloads, raw profile bodies, raw flamegraph nodes, raw memory snapshots, raw command lines, or raw env bytes past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
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

            // capture-state binding rules
            if row.row_class.requires_capture_state()
                && matches!(
                    row.capture_state_class,
                    CaptureStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CaptureStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a capture_state_admission but has no bound state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_capture_state()
                && !matches!(
                    row.capture_state_class,
                    CaptureStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CaptureStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds capture state {}; only capture_state_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.capture_state_class.as_str()
                    ),
                ));
            }

            // origin-class binding rules
            if row.row_class.requires_origin_class()
                && matches!(
                    row.origin_class,
                    OriginClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::OriginClassNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an origin_class_admission but has no bound origin",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_origin_class()
                && !matches!(
                    row.origin_class,
                    OriginClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::OriginClassNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds origin class {}; only origin_class_admission rows may bind an origin",
                        row.row_id,
                        row.row_class.as_str(),
                        row.origin_class.as_str()
                    ),
                ));
            }

            // build-mode binding rules
            if row.row_class.requires_build_mode()
                && matches!(
                    row.build_mode_class,
                    BuildModeClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::BuildModeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a build_mode_admission but has no bound build mode",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_build_mode()
                && !matches!(
                    row.build_mode_class,
                    BuildModeClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::BuildModeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds build mode {}; only build_mode_admission rows may bind a build mode",
                        row.row_id,
                        row.row_class.as_str(),
                        row.build_mode_class.as_str()
                    ),
                ));
            }

            // run-class binding rules
            if row.row_class.requires_run_class()
                && matches!(
                    row.run_class_class,
                    RunClassClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RunClassNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a run_class_admission but has no bound run class",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_run_class()
                && !matches!(
                    row.run_class_class,
                    RunClassClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RunClassNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds run class {}; only run_class_admission rows may bind a run class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.run_class_class.as_str()
                    ),
                ));
            }

            // confounder binding rules
            if row.row_class.requires_confounder()
                && matches!(
                    row.confounder_class,
                    ConfounderClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConfounderNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a confounder_admission but has no bound confounder",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_confounder()
                && !matches!(
                    row.confounder_class,
                    ConfounderClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConfounderNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds confounder {}; only confounder_admission rows may bind a confounder",
                        row.row_id,
                        row.row_class.as_str(),
                        row.confounder_class.as_str()
                    ),
                ));
            }

            // replay-state binding rules
            if row.row_class.requires_replay_state()
                && matches!(
                    row.replay_state_class,
                    ReplayStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a replay_state_admission but has no bound replay state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_replay_state()
                && !matches!(
                    row.replay_state_class,
                    ReplayStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds replay state {}; only replay_state_admission rows may bind a replay state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.replay_state_class.as_str()
                    ),
                ));
            }

            // profiler-surface binding rules
            if row.row_class.requires_profiler_surface()
                && matches!(
                    row.profiler_surface_class,
                    ProfilerSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ProfilerSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_profiler_surface()
                && !matches!(
                    row.profiler_surface_class,
                    ProfilerSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ProfilerSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds profiler surface {}; only surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.profiler_surface_class.as_str()
                    ),
                ));
            }

            if matches!(
                row.row_class,
                ProfilerRowClass::SurfaceBinding
            ) {
                if row
                    .profiler_surface_class
                    .requires_capture_state_attestation()
                    && !row.attests_capture_state_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::ProfilerSurfaceMissingCaptureStateAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds profiler surface {} but does not attest capture-state preservation",
                            row.row_id,
                            row.profiler_surface_class.as_str()
                        ),
                    ));
                }
                if row
                    .profiler_surface_class
                    .requires_replay_state_attestation()
                    && !row.attests_replay_state_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::ProfilerSurfaceMissingReplayStateAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds profiler surface {} but does not attest replay-state preservation",
                            row.row_id,
                            row.profiler_surface_class.as_str()
                        ),
                    ));
                }
            }

            if matches!(row.row_class, ProfilerRowClass::LineageAdmission)
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

            if matches!(
                row.confidence_class,
                ConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::LaunchStable)
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

        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, ProfilerRowClass::ProfilerQuality)
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, ProfilerRowClass::WedgeAdmission)
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

            for state in CaptureStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::CaptureStateAdmission
                        )
                        && row.capture_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingCaptureStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no capture_state_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for origin in OriginClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::OriginClassAdmission
                        )
                        && row.origin_class == origin
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingOriginClassCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no origin_class_admission row for {}",
                            lane.as_str(),
                            origin.as_str()
                        ),
                    ));
                }
            }

            for mode in BuildModeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::BuildModeAdmission
                        )
                        && row.build_mode_class == mode
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingBuildModeCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no build_mode_admission row for {}",
                            lane.as_str(),
                            mode.as_str()
                        ),
                    ));
                }
            }

            for run_class in RunClassClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::RunClassAdmission
                        )
                        && row.run_class_class == run_class
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingRunClassCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no run_class_admission row for {}",
                            lane.as_str(),
                            run_class.as_str()
                        ),
                    ));
                }
            }

            for confounder in ConfounderClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::ConfounderAdmission
                        )
                        && row.confounder_class == confounder
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConfounderCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no confounder_admission row for {}",
                            lane.as_str(),
                            confounder.as_str()
                        ),
                    ));
                }
            }

            for replay_state in ReplayStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::ReplayStateAdmission
                        )
                        && row.replay_state_class == replay_state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingReplayStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no replay_state_admission row for {}",
                            lane.as_str(),
                            replay_state.as_str()
                        ),
                    ));
                }
            }

            for surface in ProfilerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ProfilerRowClass::SurfaceBinding
                        )
                        && row.profiler_surface_class == surface
                        && (!surface.requires_capture_state_attestation()
                            || row.attests_capture_state_preserved)
                        && (!surface.requires_replay_state_attestation()
                            || row.attests_replay_state_preserved)
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingProfilerSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no fully-attested surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, ProfilerRowClass::LineageAdmission)
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
                        "projection {} does not preserve profiler truth",
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
            if !projection.preserves_capture_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::CaptureStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the capture-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_origin_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::OriginClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the origin-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_build_mode_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::BuildModeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the build-mode vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_run_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RunClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the run-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_confounder_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ConfounderVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the confounder vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_replay_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplayStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the replay-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_profiler_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ProfilerSurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the profiler-surface vocabulary",
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
pub struct ProfilerTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub profiler_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub profiler_truth_packet: ProfilerTruthPacket,
}

impl ProfilerTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PROFILER_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PROFILER_TRUTH_SCHEMA_VERSION
            && self.profiler_truth_packet_id_ref == self.profiler_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.profiler_truth_packet.validate().is_empty()
    }
}


/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum ProfilerTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ProfilerTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "profiler truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "profiler truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProfilerTruthArtifactError {}

/// Returns the checked-in stable profiler truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_profiler_truth_packet(
) -> Result<ProfilerTruthPacket, ProfilerTruthArtifactError> {
    let packet: ProfilerTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_the_stable_profiler_and_tracing_hooks_needed_truth_packet.json"
    )))
    .map_err(ProfilerTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ProfilerTruthArtifactError::Validation(findings))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        PROFILER_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        PROFILER_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: ProfilerLaneClass) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: ProfilerRowClass::ProfilerQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        wedge: WedgeClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn capture_state_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        state: CaptureStateClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:capture_state:{}", state.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::CaptureStateAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: state,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnCaptureStateGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_capture_state_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn origin_class_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        origin: OriginClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:origin:{}", origin.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::OriginClassAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: origin,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnOriginClassGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_origin_class_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn build_mode_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        mode: BuildModeClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:build_mode:{}", mode.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::BuildModeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: mode,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnBuildModeGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_build_mode_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn run_class_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        run_class: RunClassClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:run_class:{}", run_class.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::RunClassAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: run_class,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnRunClassGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_run_class_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn confounder_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        confounder: ConfounderClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:confounder:{}", confounder.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::ConfounderAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: confounder,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConfounderGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_confounder_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn replay_state_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        replay_state: ReplayStateClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:replay_state:{}", replay_state.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::ReplayStateAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: replay_state,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnReplayStateGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_replay_state_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn surface_row(
        prefix: &str,
        lane: ProfilerLaneClass,
        surface: ProfilerSurfaceClass,
    ) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: ProfilerRowClass::SurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnProfilerSurfaceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_profiler_surface_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_capture_state_preserved: surface.requires_capture_state_attestation(),
            attests_replay_state_preserved: surface.requires_replay_state_attestation(),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: ProfilerLaneClass) -> ProfilerRow {
        ProfilerRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: ProfilerRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            capture_state_class: CaptureStateClass::NotApplicable,
            origin_class: OriginClass::NotApplicable,
            build_mode_class: BuildModeClass::NotApplicable,
            run_class_class: RunClassClass::NotApplicable,
            confounder_class: ConfounderClass::NotApplicable,
            replay_state_class: ReplayStateClass::NotApplicable,
            profiler_surface_class: ProfilerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:profiler_lineage")),
            attests_capture_state_preserved: false,
            attests_replay_state_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerProjectionSurface) -> ProfilerConsumerProjection {
        ProfilerConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            profiler_truth_packet_id_ref:
                "packet:m4:harden_the_stable_profiler_and_tracing_hooks_needed".to_owned(),
            rendered_at: "2026-05-27T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_capture_state_vocabulary: true,
            preserves_origin_class_vocabulary: true,
            preserves_build_mode_vocabulary: true,
            preserves_run_class_vocabulary: true,
            preserves_confounder_vocabulary: true,
            preserves_replay_state_vocabulary: true,
            preserves_profiler_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: ProfilerLaneClass, prefix: &str) -> Vec<ProfilerRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for state in CaptureStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(capture_state_row(prefix, lane, state));
        }
        for origin in OriginClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(origin_class_row(prefix, lane, origin));
        }
        for mode in BuildModeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(build_mode_row(prefix, lane, mode));
        }
        for run_class in RunClassClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(run_class_row(prefix, lane, run_class));
        }
        for confounder in ConfounderClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(confounder_row(prefix, lane, confounder));
        }
        for replay_state in ReplayStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(replay_state_row(prefix, lane, replay_state));
        }
        for surface in ProfilerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> ProfilerTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(ProfilerLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            ProfilerLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(
            ProfilerLaneClass::ContainerLane,
            "container",
        ));
        rows.extend(lane_rows(
            ProfilerLaneClass::CiImportLane,
            "ci_import",
        ));
        ProfilerTruthPacketInput {
            packet_id: "packet:m4:harden_the_stable_profiler_and_tracing_hooks_needed"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_the_stable_profiler_and_tracing_hooks_needed"
                    .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: ProfilerLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(ProfilerLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            ProfilerLaneClass::CiImportLane.as_str(),
            "ci_import_lane"
        );
        assert_eq!(
            ProfilerRowClass::ProfilerQuality.as_str(),
            "profiler_quality"
        );
        assert_eq!(
            ProfilerRowClass::CaptureStateAdmission.as_str(),
            "capture_state_admission"
        );
        assert_eq!(
            ProfilerRowClass::OriginClassAdmission.as_str(),
            "origin_class_admission"
        );
        assert_eq!(
            ProfilerRowClass::BuildModeAdmission.as_str(),
            "build_mode_admission"
        );
        assert_eq!(
            ProfilerRowClass::RunClassAdmission.as_str(),
            "run_class_admission"
        );
        assert_eq!(
            ProfilerRowClass::ConfounderAdmission.as_str(),
            "confounder_admission"
        );
        assert_eq!(
            ProfilerRowClass::ReplayStateAdmission.as_str(),
            "replay_state_admission"
        );
        assert_eq!(
            ProfilerRowClass::SurfaceBinding.as_str(),
            "surface_binding"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(
            WedgeClass::ProfileSessionDescriptor.as_str(),
            "profile_session_descriptor"
        );
        assert_eq!(
            WedgeClass::ExportRedactionSummary.as_str(),
            "export_redaction_summary"
        );
        assert_eq!(CaptureStateClass::Live.as_str(), "live");
        assert_eq!(
            CaptureStateClass::DisabledWithReason.as_str(),
            "disabled_with_reason"
        );
        assert_eq!(OriginClass::LocalOrigin.as_str(), "local_origin");
        assert_eq!(
            OriginClass::ImportedBundleOrigin.as_str(),
            "imported_bundle_origin"
        );
        assert_eq!(BuildModeClass::DebugMode.as_str(), "debug_mode");
        assert_eq!(BuildModeClass::ReleaseMode.as_str(), "release_mode");
        assert_eq!(RunClassClass::WarmRun.as_str(), "warm_run");
        assert_eq!(RunClassClass::ColdRun.as_str(), "cold_run");
        assert_eq!(ConfounderClass::HardwareClass.as_str(), "hardware_class");
        assert_eq!(ConfounderClass::ThermalState.as_str(), "thermal_state");
        assert_eq!(ReplayStateClass::Supported.as_str(), "supported");
        assert_eq!(
            ReplayStateClass::ImportViewOnly.as_str(),
            "import_view_only"
        );
        assert_eq!(
            ProfilerSurfaceClass::FlamegraphSurface.as_str(),
            "flamegraph_surface"
        );
        assert_eq!(
            ProfilerSurfaceClass::ProfileSessionSurface.as_str(),
            "profile_session_surface"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            KnownLimitClass::RicherReplayOutsideM4.as_str(),
            "richer_replay_outside_m4"
        );
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ConsumerProjectionSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::ProfilerSurfaceMissingCaptureStateAttestation.as_str(),
            "profiler_surface_missing_capture_state_attestation"
        );
        assert_eq!(
            FindingKind::ProfilerSurfaceMissingReplayStateAttestation.as_str(),
            "profiler_surface_missing_replay_state_attestation"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = ProfilerTruthPacket::materialize(sample_input());
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
                "support:m4:harden_the_stable_profiler_and_tracing_hooks_needed",
                "2026-05-27T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = ProfilerTruthPacket::materialize(input);
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
    fn missing_capture_state_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::CaptureStateAdmission
            ) && row.capture_state_class == CaptureStateClass::DisabledWithReason
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingCaptureStateCoverage));
    }

    #[test]
    fn missing_origin_class_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::OriginClassAdmission
            ) && row.origin_class == OriginClass::ImportedBundleOrigin
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingOriginClassCoverage));
    }

    #[test]
    fn missing_build_mode_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::BuildModeAdmission
            ) && row.build_mode_class == BuildModeClass::ReleaseMode
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingBuildModeCoverage));
    }

    #[test]
    fn missing_run_class_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::RunClassAdmission
            ) && row.run_class_class == RunClassClass::ColdRun
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingRunClassCoverage));
    }

    #[test]
    fn missing_confounder_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::ConfounderAdmission
            ) && row.confounder_class == ConfounderClass::ThermalState
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConfounderCoverage));
    }

    #[test]
    fn missing_replay_state_for_launch_stable_blocks_stable() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                ProfilerRowClass::ReplayStateAdmission
            ) && row.replay_state_class == ReplayStateClass::ImportViewOnly
                && row.lane_class == ProfilerLaneClass::LocalLane)
        });
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingReplayStateCoverage));
    }

    #[test]
    fn surface_missing_capture_state_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                ProfilerRowClass::SurfaceBinding
            ) && row.lane_class == ProfilerLaneClass::LocalLane
                && row.profiler_surface_class == ProfilerSurfaceClass::FlamegraphSurface
            {
                row.attests_capture_state_preserved = false;
                break;
            }
        }
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ProfilerSurfaceMissingCaptureStateAttestation
        }));
    }

    #[test]
    fn surface_missing_replay_state_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                ProfilerRowClass::SurfaceBinding
            ) && row.lane_class == ProfilerLaneClass::LocalLane
                && row.profiler_surface_class == ProfilerSurfaceClass::ReplayControlsSurface
            {
                row.attests_replay_state_preserved = false;
                break;
            }
        }
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ProfilerSurfaceMissingReplayStateAttestation
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, ProfilerRowClass::LineageAdmission)
                && row.lane_class == ProfilerLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = ProfilerTruthPacket::materialize(input);
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
        let packet = ProfilerTruthPacket::materialize(input);
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
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_capture_state_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerProjectionSurface::HelpAbout {
                projection.preserves_capture_state_vocabulary = false;
            }
        }
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::CaptureStateVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = ProfilerTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
