//! Qualification packet for profiler, trace, replay, and regression surfaces.
//!
//! This module owns the runtime-facing qualification layer for promoted
//! performance tooling rows. It keeps profile-session provenance, immutable
//! trace-bundle identity, replay capability, regression comparability, and
//! export posture in one packet so flamegraphs, timelines, call trees,
//! allocation views, reverse-replay chrome, regression summaries, CLI/headless
//! packets, and support exports cannot infer Stable truth from adjacent debug
//! or artifact-viewer rows.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`PerformanceQualificationPacket`].
pub const PERFORMANCE_QUALIFICATION_RECORD_KIND: &str =
    "profiler_trace_replay_regression_qualification_packet";

/// Stable record-kind tag for [`PerformanceQualificationSupportExport`].
pub const PERFORMANCE_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "profiler_trace_replay_regression_qualification_support_export";

/// Integer schema version for this packet family.
pub const PERFORMANCE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path to the boundary schema.
pub const PERFORMANCE_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/release/profiler-trace-replay-regression-qualification.schema.json";

/// Repo-relative path to the release reviewer document.
pub const PERFORMANCE_QUALIFICATION_DOC_REF: &str =
    "docs/m4/profiler-trace-replay-regression-qualification.md";

/// Repo-relative path to the help projection.
pub const PERFORMANCE_QUALIFICATION_HELP_DOC_REF: &str =
    "docs/help/profiler-trace-replay-regression-qualification.md";

/// Repo-relative path to the release artifact.
pub const PERFORMANCE_QUALIFICATION_ARTIFACT_DOC_REF: &str =
    "artifacts/release/m4/profiler-trace-replay-regression-qualification.md";

/// Repo-relative path to the protected fixture corpus.
pub const PERFORMANCE_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/perf/m4/profiler-trace-replay-regression";

/// Lifecycle label a promoted performance row may render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceClaimLabel {
    /// Row has current packets for Stable wording.
    Stable,
    /// Row is visible but narrower than Stable.
    Preview,
    /// Row is opt-in experimental.
    Labs,
    /// Row is evidence-view-only and cannot imply live capture or replay.
    EvidenceViewOnly,
    /// Row is unsupported with an explicit reason.
    Unsupported,
}

impl PerformanceClaimLabel {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Labs => "labs",
            Self::EvidenceViewOnly => "evidence_view_only",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when this label is below the Stable cutline.
    pub const fn is_below_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Performance surface family covered by one qualification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceSurfaceKind {
    /// Profile-session summary and launch/capture surface.
    ProfileSession,
    /// Flamegraph view.
    Flamegraph,
    /// Timeline or span trace view.
    Timeline,
    /// Symbolized call-tree view.
    CallTree,
    /// Allocation or heap profile view.
    AllocationView,
    /// Imported profile or trace viewer.
    ImportedProfileViewer,
    /// Reverse-debug or replay timeline chrome.
    ReverseReplayTimeline,
    /// Regression summary or comparison card.
    RegressionSummary,
}

impl PerformanceSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProfileSession,
        Self::Flamegraph,
        Self::Timeline,
        Self::CallTree,
        Self::AllocationView,
        Self::ImportedProfileViewer,
        Self::ReverseReplayTimeline,
        Self::RegressionSummary,
    ];

    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileSession => "profile_session",
            Self::Flamegraph => "flamegraph",
            Self::Timeline => "timeline",
            Self::CallTree => "call_tree",
            Self::AllocationView => "allocation_view",
            Self::ImportedProfileViewer => "imported_profile_viewer",
            Self::ReverseReplayTimeline => "reverse_replay_timeline",
            Self::RegressionSummary => "regression_summary",
        }
    }

    const fn requires_regression_summary(self) -> bool {
        matches!(self, Self::RegressionSummary)
    }

    const fn requires_replay_chrome(self) -> bool {
        matches!(self, Self::ReverseReplayTimeline)
    }
}

/// Capture mode shown in the session strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    /// Periodic CPU samples.
    CpuSample,
    /// Instrumented CPU profile.
    InstrumentedCpuProfile,
    /// Wall-clock timeline trace.
    WallTimeTrace,
    /// Memory sample profile.
    MemorySample,
    /// Allocation or heap snapshot.
    AllocationSnapshot,
    /// Recording metadata for replay-capable backends.
    ReplayRecording,
    /// Imported profile or trace evidence.
    ImportedProfile,
}

impl CaptureMode {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuSample => "cpu_sample",
            Self::InstrumentedCpuProfile => "instrumented_cpu_profile",
            Self::WallTimeTrace => "wall_time_trace",
            Self::MemorySample => "memory_sample",
            Self::AllocationSnapshot => "allocation_snapshot",
            Self::ReplayRecording => "replay_recording",
            Self::ImportedProfile => "imported_profile",
        }
    }
}

/// Source class for captured or imported evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSourceClass {
    /// Captured from a local process or run configuration.
    Local,
    /// Captured from a remote/helper target.
    Remote,
    /// Captured from a container target.
    Container,
    /// Imported from CI artifacts.
    CiArtifact,
    /// Imported from a trace or support bundle.
    ImportedBundle,
}

impl CaptureSourceClass {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::CiArtifact => "ci_artifact",
            Self::ImportedBundle => "imported_bundle",
        }
    }
}

/// Freshness and origin state shown anywhere a metric appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Current live capture.
    Live,
    /// Imported evidence.
    Imported,
    /// Cached local evidence.
    Cached,
    /// Stale prior result.
    Stale,
    /// No recording exists for this lane.
    NotRecorded,
}

impl EvidenceState {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Imported => "imported",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::NotRecorded => "not_recorded",
        }
    }
}

/// Symbol and source-map mapping quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingQualityState {
    /// Symbols and source maps exactly match the captured build.
    Exact,
    /// Symbols or source maps are approximate.
    Approximate,
    /// Some frames map while others remain unresolved.
    Partial,
    /// Mapping came from an imported packet and is not live-resolved.
    Imported,
    /// Mapping is stale relative to the captured build.
    Stale,
    /// Mapping is unavailable.
    Unavailable,
}

impl MappingQualityState {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Partial => "partial",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when source-level recommendations need degraded wording.
    pub const fn requires_degraded_source_claim(self) -> bool {
        !matches!(self, Self::Exact)
    }
}

/// Replay lane state for a backend/runtime combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDegradationState {
    /// Replay and reverse-step are supported for the declared range.
    Supported,
    /// Replay is supported with named caveats.
    Limited,
    /// Recording is possible but reverse replay is not.
    RecordOnly,
    /// Profiling is possible but record/replay is not.
    ProfileOnly,
    /// Imported captures can be viewed only.
    ImportViewOnly,
    /// Backend/runtime combination is unsupported.
    Unsupported,
}

impl ReplayDegradationState {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::RecordOnly => "record_only",
            Self::ProfileOnly => "profile_only",
            Self::ImportViewOnly => "import_view_only",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when live reverse controls may be enabled.
    pub const fn allows_reverse_controls(self) -> bool {
        matches!(self, Self::Supported | Self::Limited)
    }

    const fn requires_disabled_reason(self) -> bool {
        !matches!(self, Self::Supported)
    }
}

/// Support state for a single replay feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFeatureState {
    /// Feature is supported.
    Supported,
    /// Feature is partially supported with caveats.
    Limited,
    /// Feature is unavailable.
    Unsupported,
}

impl ReplayFeatureState {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Redaction mode for profile and trace evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionMode {
    /// Only ids, hashes, and counters leave the machine by default.
    MetadataAndHashesOnly,
    /// Redacted summaries may be exported.
    RedactedSummary,
    /// Raw material is retained locally only.
    LocalOnlyRawRetained,
    /// Raw material requires explicit review.
    RawReviewRequired,
    /// Policy withholds the payload.
    WithheldByPolicy,
}

impl RedactionMode {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "metadata_and_hashes_only",
            Self::RedactedSummary => "redacted_summary",
            Self::LocalOnlyRawRetained => "local_only_raw_retained",
            Self::RawReviewRequired => "raw_review_required",
            Self::WithheldByPolicy => "withheld_by_policy",
        }
    }
}

/// Retention class for raw bundles and derived views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Ephemeral local cache.
    EphemeralLocal,
    /// Rotating local evidence cache.
    RotatingLocalEvidence,
    /// Pinned local evidence.
    PinnedLocalEvidence,
    /// Manifest-only support export.
    SupportManifestOnly,
    /// Managed archive governed by policy.
    ManagedArchive,
}

impl RetentionClass {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralLocal => "ephemeral_local",
            Self::RotatingLocalEvidence => "rotating_local_evidence",
            Self::PinnedLocalEvidence => "pinned_local_evidence",
            Self::SupportManifestOnly => "support_manifest_only",
            Self::ManagedArchive => "managed_archive",
        }
    }
}

/// Metric family carried by a trace bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricFamily {
    /// CPU or execution-time metrics.
    Cpu,
    /// Wall-clock timing metrics.
    WallTime,
    /// Runtime task/span events.
    RuntimeEvents,
    /// Memory or allocation metrics.
    Memory,
    /// I/O or wait-state metrics.
    Io,
}

impl MetricFamily {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::WallTime => "wall_time",
            Self::RuntimeEvents => "runtime_events",
            Self::Memory => "memory",
            Self::Io => "io",
        }
    }
}

/// Confounder family surfaced on a regression summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionConfounderKind {
    /// Local versus remote execution changed.
    LocalRemote,
    /// Debug versus release build changed.
    DebugRelease,
    /// Warm versus cold run changed.
    WarmCold,
    /// Hardware class changed.
    HardwareClass,
    /// Power state changed.
    PowerState,
}

impl RegressionConfounderKind {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRemote => "local_remote",
            Self::DebugRelease => "debug_release",
            Self::WarmCold => "warm_cold",
            Self::HardwareClass => "hardware_class",
            Self::PowerState => "power_state",
        }
    }
}

/// Threshold or waiver state attached to a regression card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdWaiverState {
    /// Threshold is active and unwaived.
    ThresholdActive,
    /// Threshold failed.
    ThresholdExceeded,
    /// Waiver is active and visible.
    WaiverActive,
    /// Waiver has expired.
    WaiverExpired,
    /// Threshold is advisory only.
    AdvisoryOnly,
}

impl ThresholdWaiverState {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThresholdActive => "threshold_active",
            Self::ThresholdExceeded => "threshold_exceeded",
            Self::WaiverActive => "waiver_active",
            Self::WaiverExpired => "waiver_expired",
            Self::AdvisoryOnly => "advisory_only",
        }
    }
}

/// Cross-surface projection a qualification row must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionSurface {
    /// Product UI projection.
    ProductUi,
    /// CLI or headless output projection.
    CliHeadless,
    /// Support export projection.
    SupportExport,
    /// Help and docs projection.
    HelpDocs,
    /// Release or shiproom proof projection.
    ReleaseEvidence,
}

impl ProjectionSurface {
    /// Every projection required for Stable rows.
    pub const REQUIRED: [Self; 5] = [
        Self::ProductUi,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpDocs,
        Self::ReleaseEvidence,
    ];

    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
            Self::ReleaseEvidence => "release_evidence",
        }
    }
}

/// Capture start/end and duration for a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureWindow {
    /// Capture start timestamp.
    pub started_at: String,
    /// Capture end timestamp.
    pub ended_at: String,
    /// Capture duration in milliseconds.
    pub duration_ms: u64,
}

/// Exact build and runtime identity attached to a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRuntimeIdentity {
    /// Exact build identity ref.
    pub exact_build_identity_ref: String,
    /// Build id or artifact digest.
    pub build_id: String,
    /// Commit or source revision ref.
    pub commit_ref: String,
    /// Runtime family and version summary.
    pub runtime_identity: String,
    /// Toolchain identity ref.
    pub toolchain_identity_ref: String,
}

/// Target process or launch configuration for a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentity {
    /// Target kind such as `process`, `task`, or `run_configuration`.
    pub target_kind: String,
    /// Stable target ref.
    pub target_ref: String,
    /// Reviewer-facing target label.
    pub target_label: String,
}

/// Typed profile-session descriptor required before any chart claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionDescriptor {
    /// Stable profile-session id.
    pub session_id: String,
    /// Capture mode.
    pub capture_mode: CaptureMode,
    /// Capture source class.
    pub capture_source: CaptureSourceClass,
    /// Execution context id shared across evidence.
    pub execution_context_id: String,
    /// Build and runtime identity.
    pub build_runtime: BuildRuntimeIdentity,
    /// Target process or configuration.
    pub target: TargetIdentity,
    /// Capture window.
    pub capture_window: CaptureWindow,
    /// Capture overhead class label.
    pub overhead_class: String,
    /// Mapping quality state.
    pub mapping_quality: MappingQualityState,
    /// Export data class label.
    pub data_class: String,
    /// Export posture summary.
    pub export_posture: String,
}

impl ProfileSessionDescriptor {
    /// Returns true when the descriptor names one execution context and build.
    pub fn has_required_identity(&self) -> bool {
        !self.session_id.trim().is_empty()
            && !self.execution_context_id.trim().is_empty()
            && !self
                .build_runtime
                .exact_build_identity_ref
                .trim()
                .is_empty()
            && !self.build_runtime.build_id.trim().is_empty()
            && !self.target.target_ref.trim().is_empty()
            && self.capture_window.duration_ms > 0
            && !self.data_class.trim().is_empty()
            && !self.export_posture.trim().is_empty()
    }
}

/// Symbol and source-map refs used by a trace bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingReferenceSet {
    /// Symbol manifest ref.
    pub symbol_manifest_ref: String,
    /// Source-map manifest ref.
    pub source_map_manifest_ref: String,
    /// Mapping quality state.
    pub mapping_quality: MappingQualityState,
}

/// Immutable trace-bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleManifest {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Profile session that produced or imported the bundle.
    pub profile_session_ref: String,
    /// Execution context id shared with the profile descriptor.
    pub execution_context_id: String,
    /// Chunk refs for the immutable raw bundle.
    pub chunk_refs: Vec<String>,
    /// Metric families carried by the bundle.
    pub metric_families: Vec<MetricFamily>,
    /// Symbol/source-map refs.
    pub mapping_refs: MappingReferenceSet,
    /// Redaction mode.
    pub redaction_mode: RedactionMode,
    /// Retention class.
    pub retention_class: RetentionClass,
    /// Derived views kept separate from the raw bundle.
    pub derived_view_refs: Vec<String>,
    /// True when the raw bundle is immutable.
    pub immutable: bool,
}

impl TraceBundleManifest {
    /// Returns true when the bundle cannot blur raw and derived artifacts.
    pub fn is_complete_immutable_manifest(&self) -> bool {
        self.immutable
            && !self.bundle_id.trim().is_empty()
            && !self.profile_session_ref.trim().is_empty()
            && !self.execution_context_id.trim().is_empty()
            && !self.chunk_refs.is_empty()
            && !self.metric_families.is_empty()
            && !self.mapping_refs.symbol_manifest_ref.trim().is_empty()
            && !self.mapping_refs.source_map_manifest_ref.trim().is_empty()
            && !self.derived_view_refs.is_empty()
    }
}

/// Replay feature support matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySupportMatrix {
    /// Reverse-step support.
    pub reverse_step: ReplayFeatureState,
    /// Frame inspection support.
    pub frame_inspection: ReplayFeatureState,
    /// Timeline scrubbing support.
    pub timeline_scrub: ReplayFeatureState,
}

/// Replay-capability descriptor for the current backend/runtime family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCapabilityDescriptor {
    /// Backend family.
    pub backend_family: String,
    /// Backend version.
    pub backend_version: String,
    /// Supported runtime range.
    pub supported_runtime_range: String,
    /// Supported toolchain range.
    pub supported_toolchain_range: String,
    /// Replay feature matrix.
    pub support_matrix: ReplaySupportMatrix,
    /// Current degradation state.
    pub degradation_state: ReplayDegradationState,
    /// Determinism caveats shown in UI and exports.
    pub determinism_caveats: Vec<String>,
    /// Disabled reason when replay or reverse controls are not enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// Restart or import guidance shown with disabled chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}

impl ReplayCapabilityDescriptor {
    /// Returns true when unsupported replay states carry exact user guidance.
    pub fn has_required_degradation_guidance(&self) -> bool {
        if self.backend_family.trim().is_empty()
            || self.backend_version.trim().is_empty()
            || self.supported_runtime_range.trim().is_empty()
            || self.supported_toolchain_range.trim().is_empty()
        {
            return false;
        }

        if self.degradation_state.requires_disabled_reason() {
            self.disabled_reason
                .as_deref()
                .is_some_and(|reason| !reason.trim().is_empty())
                && self
                    .guidance
                    .as_deref()
                    .is_some_and(|guidance| !guidance.trim().is_empty())
        } else {
            true
        }
    }
}

/// Session strip shown before any performance chart or recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStrip {
    /// True when the strip appears before chart content.
    pub visible_before_chart: bool,
    /// Workload label.
    pub workload: String,
    /// Build/runtime label.
    pub build_runtime: String,
    /// Capture mode label.
    pub capture_mode: CaptureMode,
    /// Mapping quality label.
    pub mapping_quality: MappingQualityState,
    /// Live/imported/cached/stale class.
    pub evidence_state: EvidenceState,
}

impl SessionStrip {
    /// Returns true when the strip mirrors profile-session identity.
    pub fn aligns_with(&self, descriptor: &ProfileSessionDescriptor) -> bool {
        self.visible_before_chart
            && !self.workload.trim().is_empty()
            && !self.build_runtime.trim().is_empty()
            && self.capture_mode == descriptor.capture_mode
            && self.mapping_quality == descriptor.mapping_quality
    }
}

/// One confounder badge shown on a regression summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionConfounder {
    /// Confounder kind.
    pub kind: RegressionConfounderKind,
    /// Reviewer-facing label.
    pub label: String,
}

/// Regression-comparison packet projected by summary cards and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionComparisonPacket {
    /// Baseline source ref or label.
    pub baseline_source: String,
    /// Baseline age in days.
    pub baseline_age_days: u32,
    /// Comparison key.
    pub comparison_key: String,
    /// Observed delta label.
    pub observed_delta: String,
    /// Threshold or waiver state.
    pub threshold_or_waiver_state: ThresholdWaiverState,
    /// Confounders visible on the summary.
    pub confounders: Vec<RegressionConfounder>,
    /// Action ref to open the trace.
    pub open_trace_action_ref: String,
    /// Action ref to open review annotation.
    pub open_review_action_ref: String,
    /// True when unlike comparisons are blocked from plain pass/fail rendering.
    pub unlike_comparison_blocks_pass_fail: bool,
}

impl RegressionComparisonPacket {
    /// Returns true when a regression card keeps comparability truth visible.
    pub fn preserves_comparability_truth(&self) -> bool {
        !self.baseline_source.trim().is_empty()
            && !self.comparison_key.trim().is_empty()
            && !self.observed_delta.trim().is_empty()
            && !self.confounders.is_empty()
            && !self.open_trace_action_ref.trim().is_empty()
            && !self.open_review_action_ref.trim().is_empty()
            && self.unlike_comparison_blocks_pass_fail
    }
}

/// Reverse/replay chrome posture for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReverseReplayChrome {
    /// True when the lane itself is present in the product.
    pub lane_present: bool,
    /// True when chrome is shown instead of disappearing silently.
    pub chrome_visible: bool,
    /// True when reverse controls are enabled.
    pub controls_enabled: bool,
    /// Disabled reason shown to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// Restart or import guidance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}

impl ReverseReplayChrome {
    /// Returns true when unsupported chrome degrades with exact reasons.
    pub fn has_honest_disabled_state(&self, replay: &ReplayCapabilityDescriptor) -> bool {
        if !self.lane_present {
            return true;
        }

        if replay.degradation_state.allows_reverse_controls() {
            self.chrome_visible && self.controls_enabled
        } else {
            self.chrome_visible
                && !self.controls_enabled
                && self
                    .disabled_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty())
                && self
                    .guidance
                    .as_deref()
                    .is_some_and(|guidance| !guidance.trim().is_empty())
        }
    }
}

/// Export and share defaults for a performance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportReviewPosture {
    /// Manifest output is the default export.
    pub manifest_export_default: bool,
    /// Summary output is the default share path.
    pub summary_export_default: bool,
    /// Raw trace export requires explicit review.
    pub raw_trace_requires_review: bool,
    /// Memory payload export requires explicit review.
    pub memory_payload_requires_review: bool,
    /// Argument export requires explicit review.
    pub arguments_require_review: bool,
    /// Environment-fragment export requires explicit review.
    pub environment_fragments_require_review: bool,
}

impl ExportReviewPosture {
    /// Returns true when raw or sensitive material cannot leave by default.
    pub fn is_safe_by_default(&self) -> bool {
        self.manifest_export_default
            && self.summary_export_default
            && self.raw_trace_requires_review
            && self.memory_payload_requires_review
            && self.arguments_require_review
            && self.environment_fragments_require_review
    }
}

/// One performance qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: PerformanceSurfaceKind,
    /// True when the row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Label the row is allowed to claim.
    pub claim_label: PerformanceClaimLabel,
    /// Current family packet ref.
    pub family_packet_ref: String,
    /// Profile-session descriptor.
    pub profile_session: ProfileSessionDescriptor,
    /// Trace-bundle manifest.
    pub trace_bundle: TraceBundleManifest,
    /// Replay-capability descriptor.
    pub replay_capability: ReplayCapabilityDescriptor,
    /// Session strip projected before charts.
    pub session_strip: SessionStrip,
    /// Optional regression comparison packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regression_comparison: Option<RegressionComparisonPacket>,
    /// Optional reverse/replay chrome posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverse_replay_chrome: Option<ReverseReplayChrome>,
    /// Export and share defaults.
    pub export_review: ExportReviewPosture,
    /// Projection surfaces preserving the row.
    #[serde(default)]
    pub projections: Vec<ProjectionSurface>,
    /// Evidence refs for this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Disclosure ref required for narrowed rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
}

impl PerformanceQualificationRow {
    /// Returns true when all required projections preserve this row.
    pub fn has_required_projections(&self) -> bool {
        ProjectionSurface::REQUIRED
            .iter()
            .all(|projection| self.projections.contains(projection))
    }

    /// Returns true when a row has the packets required for Stable wording.
    pub fn has_current_truth_packets(&self) -> bool {
        !self.family_packet_ref.trim().is_empty()
            && self.profile_session.has_required_identity()
            && self.trace_bundle.is_complete_immutable_manifest()
            && self.replay_capability.has_required_degradation_guidance()
            && self.session_strip.aligns_with(&self.profile_session)
            && self.export_review.is_safe_by_default()
            && !self.evidence_refs.is_empty()
    }
}

/// Constructor input for [`PerformanceQualificationPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Timestamp when the packet was generated.
    pub generated_at: String,
    /// Rows covered by the packet.
    pub rows: Vec<PerformanceQualificationRow>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Packet qualifying promoted performance tooling rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Timestamp when the packet was generated.
    pub generated_at: String,
    /// Rows covered by the packet.
    pub rows: Vec<PerformanceQualificationRow>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived validation findings.
    pub validation_findings: Vec<PerformanceQualificationFinding>,
}

impl PerformanceQualificationPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: PerformanceQualificationPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PERFORMANCE_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: PERFORMANCE_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            rows: input.rows,
            source_contract_refs: input.source_contract_refs,
            validation_findings: Vec::new(),
        };
        packet.validation_findings = packet.derived_findings(false);
        packet
    }

    /// Re-validates the packet against performance qualification invariants.
    pub fn validate(&self) -> Vec<PerformanceQualificationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when the packet has no blocker-level findings.
    pub fn is_stable_ready(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns all surface-kind tokens present in the packet.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.surface_kind);
        }
        set.into_iter()
            .map(PerformanceSurfaceKind::as_str)
            .collect()
    }

    /// Builds a metadata-only support export wrapping this packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> PerformanceQualificationSupportExport {
        PerformanceQualificationSupportExport {
            record_kind: PERFORMANCE_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PERFORMANCE_QUALIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_traces_excluded_by_default: true,
            memory_payloads_excluded_by_default: true,
            arguments_excluded_by_default: true,
            environment_fragments_excluded_by_default: true,
            packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<PerformanceQualificationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != PERFORMANCE_QUALIFICATION_RECORD_KIND {
            findings.push(PerformanceQualificationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "performance qualification packet has the wrong record kind",
            ));
        }

        if include_record_fields && self.schema_version != PERFORMANCE_QUALIFICATION_SCHEMA_VERSION
        {
            findings.push(PerformanceQualificationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "performance qualification packet has the wrong schema version",
            ));
        }

        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(PerformanceQualificationFinding::new(
                FindingKind::MissingPacketIdentity,
                FindingSeverity::Blocker,
                "packet_id and generated_at must be present",
            ));
        }

        if self.rows.is_empty() {
            findings.push(PerformanceQualificationFinding::new(
                FindingKind::MissingRows,
                FindingSeverity::Blocker,
                "at least one performance qualification row is required",
            ));
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.title.trim().is_empty() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::MissingRowIdentity,
                    FindingSeverity::Blocker,
                    "row_id and title must be present",
                ));
            }

            if !row.profile_session.has_required_identity() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::MissingProfileSessionDescriptor,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} lacks a complete profile-session descriptor",
                        row.row_id
                    ),
                ));
            }

            if row.profile_session.execution_context_id != row.trace_bundle.execution_context_id {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::ExecutionContextMismatch,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} profile session and trace bundle do not share execution_context_id",
                        row.row_id
                    ),
                ));
            }

            if row.profile_session.session_id != row.trace_bundle.profile_session_ref {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::TraceProfileSessionMismatch,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} trace bundle is not bound to the profile session",
                        row.row_id
                    ),
                ));
            }

            if !row.trace_bundle.is_complete_immutable_manifest() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::IncompleteTraceBundleManifest,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} lacks an immutable trace-bundle manifest",
                        row.row_id
                    ),
                ));
            }

            if !row.session_strip.aligns_with(&row.profile_session) {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::SessionStripMissingOrMisaligned,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} must show workload, build/runtime, capture mode, mapping quality, and evidence class before charts",
                        row.row_id
                    ),
                ));
            }

            if row.session_strip.evidence_state == EvidenceState::Live
                && !matches!(
                    row.profile_session.capture_source,
                    CaptureSourceClass::Local
                        | CaptureSourceClass::Remote
                        | CaptureSourceClass::Container
                )
            {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::ImportedEvidenceClaimsLive,
                    FindingSeverity::Blocker,
                    format!("row {} claims imported or CI evidence is live", row.row_id),
                ));
            }

            if !row.replay_capability.has_required_degradation_guidance() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::MissingReplayCapabilityDescriptor,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} lacks replay backend identity, supported ranges, or disabled guidance",
                        row.row_id
                    ),
                ));
            }

            if row.surface_kind.requires_replay_chrome() {
                match &row.reverse_replay_chrome {
                    Some(chrome)
                        if chrome.has_honest_disabled_state(&row.replay_capability) => {}
                    _ => findings.push(PerformanceQualificationFinding::new(
                        FindingKind::ReplayChromeMissingDisabledReason,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} must keep reverse/replay chrome visible with exact disabled reasons",
                            row.row_id
                        ),
                    )),
                }
            }

            if row.surface_kind.requires_regression_summary() {
                match &row.regression_comparison {
                    Some(regression) if regression.preserves_comparability_truth() => {}
                    _ => findings.push(PerformanceQualificationFinding::new(
                        FindingKind::RegressionSummaryMissingTruth,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} must surface baseline, age, comparison key, delta, threshold/waiver, confounders, and actions",
                            row.row_id
                        ),
                    )),
                }
            }

            if !row.export_review.is_safe_by_default() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::ExportReviewNotSafeByDefault,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} must default to manifest/summary export and review raw payload classes",
                        row.row_id
                    ),
                ));
            }

            if !row.has_required_projections() {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::ProjectionMissing,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} must preserve truth across UI, CLI/headless, support, Help/docs, and release evidence",
                        row.row_id
                    ),
                ));
            }

            if row.promoted_build_surface
                && row.claim_label == PerformanceClaimLabel::Stable
                && !row.has_current_truth_packets()
            {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::StableClaimMissingCurrentPackets,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} cannot claim Stable without provenance, replay, comparability, and redaction packets",
                        row.row_id
                    ),
                ));
            }

            if row.claim_label.is_below_stable()
                && row
                    .disclosure_ref
                    .as_deref()
                    .map_or(true, |disclosure| disclosure.trim().is_empty())
            {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosure,
                    FindingSeverity::Blocker,
                    format!("row {} is narrowed but lacks a disclosure ref", row.row_id),
                ));
            }

            if row
                .profile_session
                .mapping_quality
                .requires_degraded_source_claim()
                && row.claim_label == PerformanceClaimLabel::Stable
                && row
                    .disclosure_ref
                    .as_deref()
                    .map_or(true, |disclosure| disclosure.trim().is_empty())
            {
                findings.push(PerformanceQualificationFinding::new(
                    FindingKind::MappingQualityOverclaim,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} uses degraded mapping quality and needs source-fidelity disclosure",
                        row.row_id
                    ),
                ));
            }
        }

        findings
    }
}

/// Metadata-only support export for a performance qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Raw traces are excluded by default.
    pub raw_traces_excluded_by_default: bool,
    /// Memory payloads are excluded by default.
    pub memory_payloads_excluded_by_default: bool,
    /// Arguments are excluded by default.
    pub arguments_excluded_by_default: bool,
    /// Environment fragments are excluded by default.
    pub environment_fragments_excluded_by_default: bool,
    /// Exact packet projection.
    pub packet: PerformanceQualificationPacket,
}

impl PerformanceQualificationSupportExport {
    /// Returns true when the export excludes raw and sensitive material by default.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PERFORMANCE_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PERFORMANCE_QUALIFICATION_SCHEMA_VERSION
            && self.raw_traces_excluded_by_default
            && self.memory_payloads_excluded_by_default
            && self.arguments_excluded_by_default
            && self.environment_fragments_excluded_by_default
            && self.packet.validate().is_empty()
    }
}

/// Severity of a qualification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Finding blocks Stable qualification.
    Blocker,
    /// Finding should be reviewed but does not block the row.
    Warning,
}

/// Finding vocabulary emitted by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Packet id or timestamp is missing.
    MissingPacketIdentity,
    /// Packet has no rows.
    MissingRows,
    /// Row identity is missing.
    MissingRowIdentity,
    /// Profile-session descriptor is incomplete.
    MissingProfileSessionDescriptor,
    /// Profile and bundle execution contexts differ.
    ExecutionContextMismatch,
    /// Trace bundle does not bind to the profile session.
    TraceProfileSessionMismatch,
    /// Trace-bundle manifest is incomplete or mutable.
    IncompleteTraceBundleManifest,
    /// Session strip is missing or mismatched.
    SessionStripMissingOrMisaligned,
    /// Imported or CI evidence is labeled live.
    ImportedEvidenceClaimsLive,
    /// Replay-capability descriptor is incomplete.
    MissingReplayCapabilityDescriptor,
    /// Replay chrome is missing exact disabled state.
    ReplayChromeMissingDisabledReason,
    /// Regression summary lacks baseline/comparison truth.
    RegressionSummaryMissingTruth,
    /// Export defaults are not safe.
    ExportReviewNotSafeByDefault,
    /// Required cross-surface projection is missing.
    ProjectionMissing,
    /// Stable row lacks current truth packets.
    StableClaimMissingCurrentPackets,
    /// Narrowed row lacks a disclosure ref.
    NarrowedRowMissingDisclosure,
    /// Degraded mapping is overclaimed as exact Stable source fidelity.
    MappingQualityOverclaim,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingRows => "missing_rows",
            Self::MissingRowIdentity => "missing_row_identity",
            Self::MissingProfileSessionDescriptor => "missing_profile_session_descriptor",
            Self::ExecutionContextMismatch => "execution_context_mismatch",
            Self::TraceProfileSessionMismatch => "trace_profile_session_mismatch",
            Self::IncompleteTraceBundleManifest => "incomplete_trace_bundle_manifest",
            Self::SessionStripMissingOrMisaligned => "session_strip_missing_or_misaligned",
            Self::ImportedEvidenceClaimsLive => "imported_evidence_claims_live",
            Self::MissingReplayCapabilityDescriptor => "missing_replay_capability_descriptor",
            Self::ReplayChromeMissingDisabledReason => "replay_chrome_missing_disabled_reason",
            Self::RegressionSummaryMissingTruth => "regression_summary_missing_truth",
            Self::ExportReviewNotSafeByDefault => "export_review_not_safe_by_default",
            Self::ProjectionMissing => "projection_missing",
            Self::StableClaimMissingCurrentPackets => "stable_claim_missing_current_packets",
            Self::NarrowedRowMissingDisclosure => "narrowed_row_missing_disclosure",
            Self::MappingQualityOverclaim => "mapping_quality_overclaim",
        }
    }
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationFinding {
    /// Finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Redaction-safe message.
    pub message: String,
}

impl PerformanceQualificationFinding {
    /// Creates a validation finding.
    pub fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            message: message.into(),
        }
    }
}

/// Artifact loading or validation error.
#[derive(Debug)]
pub enum PerformanceQualificationArtifactError {
    /// JSON parsing failed.
    Parse(serde_json::Error),
    /// Packet validation found blockers.
    Validation(Vec<PerformanceQualificationFinding>),
}

impl fmt::Display for PerformanceQualificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "failed to parse performance qualification: {error}"),
            Self::Validation(findings) => {
                write!(
                    f,
                    "performance qualification validation failed with {} finding(s)",
                    findings.len()
                )
            }
        }
    }
}

impl Error for PerformanceQualificationArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Validation(_) => None,
        }
    }
}

/// Parses and validates a checked-in performance qualification input.
pub fn performance_qualification_packet_from_json(
    json: &str,
) -> Result<PerformanceQualificationPacket, PerformanceQualificationArtifactError> {
    let input: PerformanceQualificationPacketInput =
        serde_json::from_str(json).map_err(PerformanceQualificationArtifactError::Parse)?;
    let packet = PerformanceQualificationPacket::materialize(input);
    let findings = packet.validate();
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        return Err(PerformanceQualificationArtifactError::Validation(findings));
    }
    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_JSON: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/perf/m4/profiler-trace-replay-regression/qualification_manifest.json"
    ));

    fn baseline_input() -> PerformanceQualificationPacketInput {
        serde_json::from_str(FIXTURE_JSON).expect("fixture parses")
    }

    #[test]
    fn fixture_qualification_manifest_is_stable_ready() {
        let packet = PerformanceQualificationPacket::materialize(baseline_input());

        assert!(packet.validate().is_empty());
        assert!(packet.is_stable_ready());
        assert!(packet
            .surface_tokens()
            .contains(&PerformanceSurfaceKind::Flamegraph.as_str()));
        assert!(packet
            .support_export("support:perf.qualification", "2026-06-07T00:00:00Z")
            .is_export_safe());
    }

    #[test]
    fn stable_row_without_session_strip_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].session_strip.visible_before_chart = false;
        let packet = PerformanceQualificationPacket::materialize(input);

        assert!(packet.validate().iter().any(|finding| {
            finding.finding_kind == FindingKind::SessionStripMissingOrMisaligned
        }));
    }

    #[test]
    fn imported_evidence_cannot_claim_live_capture() {
        let mut input = baseline_input();
        input.rows[0].profile_session.capture_source = CaptureSourceClass::ImportedBundle;
        input.rows[0].session_strip.evidence_state = EvidenceState::Live;
        let packet = PerformanceQualificationPacket::materialize(input);

        assert!(packet
            .validate()
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ImportedEvidenceClaimsLive));
    }

    #[test]
    fn unsupported_replay_requires_visible_disabled_chrome() {
        let mut input = baseline_input();
        let replay_row = input
            .rows
            .iter_mut()
            .find(|row| row.surface_kind == PerformanceSurfaceKind::ReverseReplayTimeline)
            .expect("replay row exists");
        replay_row.reverse_replay_chrome = None;
        let packet = PerformanceQualificationPacket::materialize(input);

        assert!(packet.validate().iter().any(|finding| {
            finding.finding_kind == FindingKind::ReplayChromeMissingDisabledReason
        }));
    }

    #[test]
    fn regression_summary_requires_confounders_and_actions() {
        let mut input = baseline_input();
        let regression_row = input
            .rows
            .iter_mut()
            .find(|row| row.surface_kind == PerformanceSurfaceKind::RegressionSummary)
            .expect("regression row exists");
        regression_row
            .regression_comparison
            .as_mut()
            .expect("regression packet exists")
            .confounders
            .clear();
        let packet = PerformanceQualificationPacket::materialize(input);

        assert!(packet
            .validate()
            .iter()
            .any(|finding| { finding.finding_kind == FindingKind::RegressionSummaryMissingTruth }));
    }

    #[test]
    fn raw_export_defaults_block_stable() {
        let mut input = baseline_input();
        input.rows[0].export_review.raw_trace_requires_review = false;
        let packet = PerformanceQualificationPacket::materialize(input);

        assert!(packet
            .validate()
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ExportReviewNotSafeByDefault));
    }
}
