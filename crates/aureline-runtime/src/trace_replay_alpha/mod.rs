//! Alpha profile, trace-bundle, replay-capability, and comparison contracts.
//!
//! This module owns the bounded runtime-evidence objects shared by profiler,
//! trace, replay, comparison, support/export, and shell inspection surfaces.
//! The baseline is intentionally narrow: it can describe captured or imported
//! profiling evidence, immutable trace bundles, and replay capability truth
//! without claiming general-purpose live record/replay support.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ProfileSessionAlpha`] records.
pub const PROFILE_SESSION_ALPHA_RECORD_KIND: &str = "profile_session_alpha_record";

/// Stable record-kind tag for [`TraceBundleAlphaManifest`] records.
pub const TRACE_BUNDLE_ALPHA_RECORD_KIND: &str = "trace_bundle_alpha_manifest";

/// Stable record-kind tag for [`ReplayCapabilityAlphaDescriptor`] records.
pub const REPLAY_CAPABILITY_ALPHA_RECORD_KIND: &str = "replay_capability_alpha_descriptor";

/// Stable record-kind tag for [`ComparisonClassAlphaPacket`] records.
pub const COMPARISON_CLASS_ALPHA_RECORD_KIND: &str = "runtime_evidence_comparison_alpha_packet";

/// Stable record-kind tag for [`RuntimeEvidenceSupportExport`] records.
pub const RUNTIME_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND: &str = "runtime_evidence_support_export";

/// Stable record-kind tag for [`RuntimeEvidenceAlphaPacket`] records.
pub const RUNTIME_EVIDENCE_ALPHA_PACKET_RECORD_KIND: &str = "runtime_evidence_alpha_packet";

/// Frozen schema version for the runtime-evidence alpha records in this module.
pub const RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable support-pack item id for runtime trace/profile evidence rows.
pub const SUPPORT_ITEM_RUNTIME_TRACES: &str = "support.item.runtime_traces";

/// Capture modes the alpha profile-session descriptor can claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    /// CPU samples captured by periodic sampling.
    CpuSample,
    /// CPU trace captured by instrumentation.
    InstrumentedCpuProfile,
    /// Wall-time trace over a bounded time window.
    WallTimeTrace,
    /// Memory samples captured over a bounded time window.
    MemorySample,
    /// Allocation snapshot or heap profile.
    AllocationSnapshot,
    /// Replay recording metadata without live replay control.
    ReplayRecording,
    /// Profile data imported from a bundle, provider, or external tool.
    ImportedProfile,
}

impl CaptureMode {
    /// Stable token used in manifests and UI projections.
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

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CpuSample => "Sampled CPU profile",
            Self::InstrumentedCpuProfile => "Instrumented CPU profile",
            Self::WallTimeTrace => "Wall-time trace",
            Self::MemorySample => "Memory sample",
            Self::AllocationSnapshot => "Allocation snapshot",
            Self::ReplayRecording => "Replay recording",
            Self::ImportedProfile => "Imported profile",
        }
    }
}

/// Source class for a captured or imported profile session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSource {
    /// Captured from the local target.
    LocalCapture,
    /// Captured from a remote or managed target.
    RemoteCapture,
    /// Imported from CI artifacts.
    CiArtifact,
    /// Imported from a trace/support bundle.
    ImportedBundle,
    /// Provided by an external provider integration.
    ProviderSupplied,
    /// Provided by an extension integration.
    ExtensionSupplied,
}

impl CaptureSource {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCapture => "local_capture",
            Self::RemoteCapture => "remote_capture",
            Self::CiArtifact => "ci_artifact",
            Self::ImportedBundle => "imported_bundle",
            Self::ProviderSupplied => "provider_supplied",
            Self::ExtensionSupplied => "extension_supplied",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalCapture => "Captured locally",
            Self::RemoteCapture => "Captured remotely",
            Self::CiArtifact => "CI artifact",
            Self::ImportedBundle => "Imported bundle",
            Self::ProviderSupplied => "Provider-supplied",
            Self::ExtensionSupplied => "Extension-supplied",
        }
    }
}

/// Runtime cost class disclosed before profile or replay claims are trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverheadClass {
    /// No meaningful overhead is expected for the inspected view.
    Negligible,
    /// Low overhead suitable for most local profiling.
    Low,
    /// Moderate overhead that can alter timing-sensitive workloads.
    Moderate,
    /// High overhead that makes regression comparison advisory.
    High,
    /// The capture source did not provide an overhead estimate.
    Unknown,
}

impl OverheadClass {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Negligible => "negligible",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Unknown => "unknown",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Negligible => "Negligible overhead",
            Self::Low => "Low overhead",
            Self::Moderate => "Moderate overhead",
            Self::High => "High overhead",
            Self::Unknown => "Overhead unknown",
        }
    }
}

/// Symbol and source-map fidelity state for captured runtime evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingQualityState {
    /// Symbols and source mappings exactly match the captured build.
    ExactSymbolsAndSources,
    /// Symbols are exact, but some source mappings are missing.
    ExactSymbolsPartialSources,
    /// Symbols resolve approximately but are not exact.
    ApproximateSymbols,
    /// Symbols resolve with partial source maps.
    SymbolizedWithPartialSourceMaps,
    /// Only raw addresses or module ranges are available.
    RawAddressesOnly,
    /// Source maps exist but do not match the captured build.
    StaleSourceMaps,
    /// Mapping cannot resolve to symbols or sources.
    Unresolved,
}

impl MappingQualityState {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSymbolsAndSources => "exact_symbols_and_sources",
            Self::ExactSymbolsPartialSources => "exact_symbols_partial_sources",
            Self::ApproximateSymbols => "approximate_symbols",
            Self::SymbolizedWithPartialSourceMaps => "symbolized_with_partial_source_maps",
            Self::RawAddressesOnly => "raw_addresses_only",
            Self::StaleSourceMaps => "stale_source_maps",
            Self::Unresolved => "unresolved",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactSymbolsAndSources => "Exact symbols and sources",
            Self::ExactSymbolsPartialSources => "Exact symbols, partial sources",
            Self::ApproximateSymbols => "Approximate symbols",
            Self::SymbolizedWithPartialSourceMaps => "Partial source maps",
            Self::RawAddressesOnly => "Raw addresses only",
            Self::StaleSourceMaps => "Stale source maps",
            Self::Unresolved => "Unresolved mapping",
        }
    }
}

/// Export and privacy data class for runtime evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeEvidenceDataClass {
    /// Metadata, ids, hashes, and summary counters only.
    MetadataOnly,
    /// Redacted derived metrics or summaries.
    RedactedSummary,
    /// Environment-adjacent target/toolchain metadata.
    EnvironmentAdjacent,
    /// Raw traces, memory, arguments, or code-adjacent payloads.
    HighRiskRawCapture,
}

impl RuntimeEvidenceDataClass {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::RedactedSummary => "redacted_summary",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::HighRiskRawCapture => "high_risk_raw_capture",
        }
    }
}

/// Redaction posture for profile and trace evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceRedactionMode {
    /// Only ids, hashes, counters, and manifests are included.
    MetadataAndHashesOnly,
    /// Derived metrics are included after redaction.
    RedactedSummary,
    /// Raw payload exists only by local retained reference.
    LocalOnlyRawRetained,
    /// Raw payload export requires explicit review.
    RawPayloadReviewRequired,
    /// Active policy withholds the raw payload.
    WithheldByPolicy,
}

impl TraceRedactionMode {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "metadata_and_hashes_only",
            Self::RedactedSummary => "redacted_summary",
            Self::LocalOnlyRawRetained => "local_only_raw_retained",
            Self::RawPayloadReviewRequired => "raw_payload_review_required",
            Self::WithheldByPolicy => "withheld_by_policy",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "Metadata and hashes only",
            Self::RedactedSummary => "Redacted summary",
            Self::LocalOnlyRawRetained => "Raw payload retained locally",
            Self::RawPayloadReviewRequired => "Raw payload requires review",
            Self::WithheldByPolicy => "Withheld by policy",
        }
    }
}

/// Retention posture for trace bundles and derived evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceRetentionClass {
    /// Rotates from the local device after the declared window.
    #[serde(rename = "local_rotation_7_days")]
    LocalRotationSevenDays,
    /// Pinned local evidence retained for a review or support case.
    PinnedLocalEvidence,
    /// Only the manifest is embedded in support/export packets.
    SupportBundleManifestOnly,
    /// Managed archive retention applies.
    ManagedArchive,
    /// The artifact is not retained after the session.
    NotRetainedEphemeral,
}

impl TraceRetentionClass {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRotationSevenDays => "local_rotation_7_days",
            Self::PinnedLocalEvidence => "pinned_local_evidence",
            Self::SupportBundleManifestOnly => "support_bundle_manifest_only",
            Self::ManagedArchive => "managed_archive",
            Self::NotRetainedEphemeral => "not_retained_ephemeral",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalRotationSevenDays => "Local rotation, 7 days",
            Self::PinnedLocalEvidence => "Pinned local evidence",
            Self::SupportBundleManifestOnly => "Manifest only in support export",
            Self::ManagedArchive => "Managed archive",
            Self::NotRetainedEphemeral => "Not retained",
        }
    }
}

/// Metric families that may appear in an alpha trace bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceMetricFamily {
    /// CPU sample or execution-time metrics.
    Cpu,
    /// Wall-clock timing metrics.
    WallTime,
    /// Task, test, build, or runtime event markers.
    RuntimeEvents,
    /// Memory or allocation metrics.
    Memory,
    /// I/O or wait-state metrics.
    Io,
}

impl TraceMetricFamily {
    /// Stable token used in manifests and UI projections.
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

/// Derived view classes produced from an immutable raw trace bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedViewKind {
    /// Folded flamegraph view.
    Flamegraph,
    /// Symbolized call-tree view.
    CallTree,
    /// Timeline view.
    Timeline,
    /// Regression or comparison summary view.
    RegressionSummary,
    /// Advisory AI summary tied back to trace nodes.
    AdvisorySummary,
}

impl DerivedViewKind {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flamegraph => "flamegraph",
            Self::CallTree => "call_tree",
            Self::Timeline => "timeline",
            Self::RegressionSummary => "regression_summary",
            Self::AdvisorySummary => "advisory_summary",
        }
    }
}

/// Digest algorithms accepted by runtime-evidence manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestAlgorithm {
    /// SHA-256 digest.
    Sha256,
    /// BLAKE3 digest.
    Blake3,
}

/// Current replay-lane capability class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLaneState {
    /// Replay and reverse stepping are supported for the declared range.
    Supported,
    /// Replay is available with explicit limitations.
    Limited,
    /// Recording is supported, but replay is not.
    RecordOnly,
    /// Profiling is supported, but record/replay is not.
    ProfileOnly,
    /// Imported recordings can be inspected but not controlled live.
    ImportViewOnly,
}

impl ReplayLaneState {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::RecordOnly => "record_only",
            Self::ProfileOnly => "profile_only",
            Self::ImportViewOnly => "import_view_only",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "Replay supported",
            Self::Limited => "Replay limited",
            Self::RecordOnly => "Record only",
            Self::ProfileOnly => "Profile only",
            Self::ImportViewOnly => "Import/view-only",
        }
    }

    /// True when the lane can claim live reverse execution.
    pub const fn claims_live_replay(self) -> bool {
        matches!(self, Self::Supported | Self::Limited)
    }
}

/// Per-feature support class inside a replay-capability descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFeatureState {
    /// Feature is supported for the declared backend range.
    Supported,
    /// Feature is partially supported and must name caveats.
    Limited,
    /// Feature is visible but unavailable for this lane.
    Unsupported,
}

impl ReplayFeatureState {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Local, remote, CI, or imported comparison source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonSourceClass {
    /// Both captures are local to the current device class.
    Local,
    /// At least one capture came from a remote runner.
    Remote,
    /// At least one capture came from a containerized target.
    Container,
    /// At least one capture came from CI.
    Ci,
    /// The comparison is imported read-only evidence.
    ImportedReadOnly,
}

impl ComparisonSourceClass {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Ci => "ci",
            Self::ImportedReadOnly => "imported_read_only",
        }
    }
}

/// Comparison-class outcome for a current capture and baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonClass {
    /// Captures are comparable on the declared dimensions.
    LikeForLike,
    /// Captures are comparable only with named confounders.
    ComparableWithDeclaredConfounders,
    /// Captures remain inspectable but must not be treated as equivalent.
    InspectableNotEquivalent,
    /// Imported evidence is view-only and not comparable as a regression.
    ImportViewOnlyNotComparable,
    /// Comparison quality is unknown and requires review.
    UnknownRequiresReview,
}

impl ComparisonClass {
    /// Stable token used in manifests and UI projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LikeForLike => "like_for_like",
            Self::ComparableWithDeclaredConfounders => "comparable_with_declared_confounders",
            Self::InspectableNotEquivalent => "inspectable_not_equivalent",
            Self::ImportViewOnlyNotComparable => "import_view_only_not_comparable",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Reviewer-facing label used by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LikeForLike => "Like-for-like",
            Self::ComparableWithDeclaredConfounders => "Comparable with confounders",
            Self::InspectableNotEquivalent => "Inspectable, not equivalent",
            Self::ImportViewOnlyNotComparable => "Import/view-only, not comparable",
            Self::UnknownRequiresReview => "Comparison requires review",
        }
    }
}

/// Capture mode and source for one profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCaptureDescriptor {
    /// Capture mode exposed by the adapter or import pipeline.
    pub capture_mode: CaptureMode,
    /// Source class for the profile session.
    pub capture_source: CaptureSource,
    /// Stable source ref such as a run, CI artifact, or imported bundle id.
    pub capture_source_ref: String,
    /// Stable state token for live, cached, imported, or stale evidence.
    pub evidence_state: String,
}

/// Exact build and runtime identity attached to a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRuntimeIdentity {
    /// Exact-build identity ref shared with support and release evidence.
    pub exact_build_identity_ref: String,
    /// Build id or artifact digest for the captured process.
    pub build_id: String,
    /// Commit or source revision ref for the captured build.
    pub commit_ref: String,
    /// Runtime identity such as `rustc`, `node`, or VM/runtime version.
    pub runtime_identity: String,
    /// Toolchain identity ref used to resolve comparable runs.
    pub toolchain_identity_ref: String,
    /// Symbol-manifest ref used for frame resolution.
    pub symbol_manifest_ref: String,
    /// Source-map or source-index manifest ref used for source jumps.
    pub source_map_manifest_ref: String,
    /// Workspace or workset identity ref active during capture.
    pub workspace_ref: String,
}

/// Target process or configuration described by a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileTargetIdentity {
    /// Target kind, for example `run_configuration` or `process`.
    pub target_kind: String,
    /// Stable target identity ref.
    pub target_ref: String,
    /// Reviewer-facing target label.
    pub target_label: String,
    /// Optional process identity ref; never a raw process command line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_ref: Option<String>,
    /// Optional run/debug configuration ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_config_ref: Option<String>,
}

/// Capture start/end and duration for one profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureWindow {
    /// Capture start timestamp.
    pub started_at: String,
    /// Capture end timestamp.
    pub ended_at: String,
    /// Capture duration in milliseconds.
    pub duration_ms: u64,
}

/// Mapping quality summary for profile or trace evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingQualitySummary {
    /// Controlled mapping-quality state.
    pub state: MappingQualityState,
    /// Symbol manifest ref used by the resolver.
    pub symbol_manifest_ref: String,
    /// Source-map manifest ref used by the resolver.
    pub source_map_manifest_ref: String,
    /// Number of frames that mapped exactly.
    pub exact_frame_count: u32,
    /// Number of frames that mapped approximately.
    pub approximate_frame_count: u32,
    /// Number of frames left unresolved.
    pub unresolved_frame_count: u32,
    /// Short redaction-safe mapping note.
    pub note: String,
}

/// Data handling posture attached to runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidenceDataPosture {
    /// Export data class for the evidence row.
    pub data_class: RuntimeEvidenceDataClass,
    /// Redaction mode for manifests and support exports.
    pub redaction_mode: TraceRedactionMode,
    /// Support-pack item id used by support bundle previews.
    pub support_pack_item_id: String,
    /// Raw-payload policy sentence for export review.
    pub raw_payload_policy: String,
}

/// Default export policy for a profile session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileExportPolicy {
    /// Default profile export scope.
    pub default_scope: String,
    /// Raw payload export posture.
    pub raw_payload_export: String,
    /// Support-bundle binding ref for this profile evidence.
    pub support_bundle_binding_ref: String,
    /// Retention class for the profile evidence.
    pub retention_class: TraceRetentionClass,
}

/// Profile-session descriptor for alpha runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionAlpha {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this descriptor.
    pub schema_version: u32,
    /// Stable profile-session id.
    pub profile_session_id: String,
    /// Capture mode/source details.
    pub capture: ProfileCaptureDescriptor,
    /// Execution context id shared by the capture and bundle.
    pub execution_context_id: String,
    /// Exact build and runtime identity for the captured target.
    pub exact_build_identity: BuildRuntimeIdentity,
    /// Captured target process or run configuration.
    pub target: ProfileTargetIdentity,
    /// Capture time range.
    pub capture_window: CaptureWindow,
    /// Capture overhead class.
    pub overhead_class: OverheadClass,
    /// Symbol/source-map fidelity state.
    pub mapping_quality: MappingQualitySummary,
    /// Data handling and support/export posture.
    pub data_posture: RuntimeEvidenceDataPosture,
    /// Trace bundle manifest ref associated with this profile.
    pub trace_bundle_ref: String,
    /// Comparison packet ref associated with this profile.
    pub comparison_packet_ref: String,
    /// Export policy for support and review surfaces.
    pub export_policy: ProfileExportPolicy,
}

impl ProfileSessionAlpha {
    /// Returns true when the profile session is tied to an exact build and context.
    pub fn has_attribution(&self) -> bool {
        !self.execution_context_id.trim().is_empty()
            && !self
                .exact_build_identity
                .exact_build_identity_ref
                .trim()
                .is_empty()
            && !self.exact_build_identity.build_id.trim().is_empty()
    }
}

/// Immutable raw bundle metadata inside a trace-bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawTraceBundle {
    /// Stable ref to the raw bundle body or retained local artifact.
    pub raw_bundle_ref: String,
    /// Raw bundle kind, such as `perfetto_trace` or `pprof_profile`.
    pub raw_bundle_kind: String,
    /// Storage class for the raw bundle body.
    pub storage_class: String,
    /// Chunk refs that compose the bundle.
    pub chunk_refs: Vec<String>,
    /// Metric families present in the bundle.
    pub metric_families: Vec<TraceMetricFamily>,
}

/// Derived trace view whose provenance stays separate from the raw bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedTraceView {
    /// Stable view id.
    pub view_id: String,
    /// Derived view kind.
    pub view_kind: DerivedViewKind,
    /// Source bundle id for the derived view.
    pub source_bundle_id: String,
    /// Provenance ref for the derivation job.
    pub provenance_ref: String,
    /// Freshness state for the derived view.
    pub freshness_class: String,
    /// Digest ref for the derived view body.
    pub digest_ref: String,
}

/// Redaction posture for a trace bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleRedaction {
    /// Active redaction mode.
    pub redaction_mode: TraceRedactionMode,
    /// Redaction profile ref that governed the bundle.
    pub redaction_profile_ref: String,
    /// Default raw-payload export posture.
    pub raw_payload_export_default: String,
    /// Support-bundle posture for this trace.
    pub support_bundle_posture: String,
}

/// Retention posture for a trace bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleRetention {
    /// Retention class for the raw bundle.
    pub retention_class: TraceRetentionClass,
    /// Retention owner or policy ref.
    pub retention_owner: String,
    /// Optional expiry timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Optional retention pin ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin_ref: Option<String>,
}

/// Digest entry for an immutable trace object or derived view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestEntry {
    /// Digest algorithm.
    pub algorithm: DigestAlgorithm,
    /// Object ref covered by the digest.
    pub object_ref: String,
    /// Digest value.
    pub digest: String,
}

/// Immutability block for a captured trace bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleImmutability {
    /// Timestamp when the bundle became immutable.
    pub frozen_at: String,
    /// Stable immutability state token.
    pub immutability_state: String,
    /// Digest ref for the raw bundle.
    pub raw_bundle_digest_ref: String,
    /// Short redaction-safe immutability note.
    pub note: String,
}

/// Trace-bundle manifest for alpha runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleAlphaManifest {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this manifest.
    pub schema_version: u32,
    /// Stable trace-bundle id.
    pub bundle_id: String,
    /// Profile session that produced or imported the trace.
    pub profile_session_ref: String,
    /// Execution context id shared with the profile session.
    pub execution_context_id: String,
    /// Exact-build identity ref shared with the profile session.
    pub exact_build_identity_ref: String,
    /// Immutable raw bundle block.
    pub raw_bundle: RawTraceBundle,
    /// Separate derived views over the raw bundle.
    pub derived_views: Vec<DerivedTraceView>,
    /// Mapping quality summary.
    pub mapping_quality: MappingQualitySummary,
    /// Redaction posture for support and export.
    pub redaction: TraceBundleRedaction,
    /// Retention posture for the raw bundle.
    pub retention: TraceBundleRetention,
    /// Digest set for raw and derived artifacts.
    pub digest_set: Vec<DigestEntry>,
    /// Immutability assertion.
    pub immutability: TraceBundleImmutability,
}

impl TraceBundleAlphaManifest {
    /// Returns true when the manifest carries the required immutable bundle markers.
    pub fn is_immutable_manifest(&self) -> bool {
        self.immutability.immutability_state == "captured_immutable"
            && !self.digest_set.is_empty()
            && !self.raw_bundle.chunk_refs.is_empty()
    }
}

/// Replay backend identity and adapter version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBackendIdentity {
    /// Backend family, such as `rr`, `perfetto`, or `none_import_viewer`.
    pub backend_family: String,
    /// Backend version or compatibility token.
    pub backend_version: String,
    /// Aureline adapter version or descriptor ref.
    pub adapter_version: String,
}

/// Runtime/toolchain range supported by a replay backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRuntimeToolchainRange {
    /// Runtime family named by the backend.
    pub runtime_family: String,
    /// Supported runtime version range.
    pub supported_runtime_range: String,
    /// Toolchain family named by the backend.
    pub toolchain_family: String,
    /// Supported toolchain version range.
    pub supported_toolchain_range: String,
}

/// Support state and reason for one replay feature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayFeatureSupport {
    /// Feature support state.
    pub state: ReplayFeatureState,
    /// Redaction-safe reason or caveat.
    pub reason: String,
}

/// Replay feature matrix for one descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySupportMatrix {
    /// Reverse stepping support.
    pub reverse_step: ReplayFeatureSupport,
    /// Frame inspection support.
    pub frame_inspection: ReplayFeatureSupport,
    /// Data or variable inspection support.
    pub data_inspection: ReplayFeatureSupport,
    /// Timeline scrubbing support.
    pub timeline_scrub: ReplayFeatureSupport,
    /// Checkpoint bookmark support.
    pub checkpoint_bookmarks: ReplayFeatureSupport,
}

/// Replay overhead and storage cost band.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayOverheadStorageBand {
    /// Runtime overhead class.
    pub overhead_class: OverheadClass,
    /// Storage growth class or bound.
    pub storage_growth_class: String,
    /// User-visible cost note.
    pub cost_note: String,
}

/// Replay export posture for support and review packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayExportPosture {
    /// Export data class.
    pub data_class: RuntimeEvidenceDataClass,
    /// Redaction mode for replay-related exports.
    pub redaction_mode: TraceRedactionMode,
    /// Retention class for replay metadata.
    pub retention_class: TraceRetentionClass,
    /// Whether raw replay snapshots are exported by default.
    pub raw_snapshots_exported_by_default: bool,
}

/// Replay-capability descriptor for alpha runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCapabilityAlphaDescriptor {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this descriptor.
    pub schema_version: u32,
    /// Stable replay descriptor id.
    pub descriptor_id: String,
    /// Replay backend identity.
    pub backend: ReplayBackendIdentity,
    /// Supported runtime/toolchain ranges.
    pub supported_ranges: Vec<ReplayRuntimeToolchainRange>,
    /// Per-feature support matrix.
    pub support_matrix: ReplaySupportMatrix,
    /// Current lane state.
    pub lane_state: ReplayLaneState,
    /// Determinism caveats that must remain visible.
    pub determinism_caveats: Vec<String>,
    /// Overhead and storage cost band.
    pub overhead_storage_band: ReplayOverheadStorageBand,
    /// Reason shown when the lane is import/view-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_view_only_reason: Option<String>,
    /// Export posture for support/replay metadata.
    pub export_posture: ReplayExportPosture,
}

impl ReplayCapabilityAlphaDescriptor {
    /// Returns true when this descriptor must not expose live replay controls.
    pub fn is_import_view_only(&self) -> bool {
        self.lane_state == ReplayLaneState::ImportViewOnly
    }
}

/// Hardware and power context for comparison-class decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwarePowerProfile {
    /// Hardware class or reference profile.
    pub hardware_class: String,
    /// CPU architecture class.
    pub cpu_arch_class: String,
    /// Power mode during capture.
    pub power_state: String,
    /// Thermal state during capture.
    pub thermal_state: String,
}

/// Runtime/toolchain context for comparison-class decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonRuntimeToolchain {
    /// Runtime family and version summary.
    pub runtime_identity: String,
    /// Toolchain identity ref.
    pub toolchain_identity_ref: String,
    /// Capture mode used by the current run.
    pub capture_mode: CaptureMode,
}

/// Variance window disclosed by a comparison packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VarianceWindow {
    /// Number of samples in the comparison set.
    pub sample_count: u32,
    /// Window label used by review surfaces.
    pub window_label: String,
    /// Relative variance in basis points.
    pub relative_variance_bps: u32,
    /// Threshold state token.
    pub threshold_state: String,
}

/// Comparison-class packet for alpha runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonClassAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Stable comparison packet id.
    pub comparison_packet_id: String,
    /// Workload id being compared.
    pub workload_id: String,
    /// Corpus or archetype for the workload.
    pub corpus_archetype: String,
    /// Local/remote/CI/imported source class.
    pub source_class: ComparisonSourceClass,
    /// Hardware and power posture.
    pub hardware_power_profile: HardwarePowerProfile,
    /// Runtime and toolchain posture.
    pub runtime_toolchain: ComparisonRuntimeToolchain,
    /// Baseline capture ref.
    pub baseline_ref: String,
    /// Current profile session ref.
    pub current_profile_session_ref: String,
    /// Sample and variance disclosure.
    pub variance_window: VarianceWindow,
    /// Comparison-class outcome.
    pub comparison_class: ComparisonClass,
    /// Confounders or caveats visible to review surfaces.
    pub confounders: Vec<String>,
}

impl ComparisonClassAlphaPacket {
    /// Returns true when consumers may treat this comparison as like-for-like.
    pub fn is_like_for_like(&self) -> bool {
        self.comparison_class == ComparisonClass::LikeForLike
    }
}

/// Support/export projection for one runtime-evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidenceSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this support projection.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Profile-session id.
    pub profile_session_id: String,
    /// Trace-bundle id.
    pub trace_bundle_id: String,
    /// Replay descriptor id.
    pub replay_descriptor_id: String,
    /// Comparison packet id.
    pub comparison_packet_id: String,
    /// Execution context id shared by the evidence.
    pub execution_context_id: String,
    /// Exact-build identity ref shared by the evidence.
    pub exact_build_identity_ref: String,
    /// Mapping quality preserved for support/export review.
    pub mapping_quality_state: MappingQualityState,
    /// Comparison class preserved for support/export review.
    pub comparison_class: ComparisonClass,
    /// Redaction mode preserved for support/export review.
    pub redaction_mode: TraceRedactionMode,
    /// Retention class preserved for support/export review.
    pub retention_class: TraceRetentionClass,
    /// Replay lane state preserved for support/export review.
    pub replay_lane_state: ReplayLaneState,
    /// Support-pack item id used by the support bundle.
    pub support_pack_item_id: String,
    /// True when raw trace/profile payloads are exported.
    pub raw_payload_exported: bool,
    /// True when the replay lane is inspect-only imported evidence.
    pub import_view_only: bool,
}

/// Complete alpha runtime-evidence packet for one profile/trace/replay lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidenceAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Canonical profile-session descriptor.
    pub profile_session: ProfileSessionAlpha,
    /// Canonical trace-bundle manifest.
    pub trace_bundle: TraceBundleAlphaManifest,
    /// Canonical replay-capability descriptor.
    pub replay_capability: ReplayCapabilityAlphaDescriptor,
    /// Canonical comparison-class packet.
    pub comparison: ComparisonClassAlphaPacket,
    /// Support/export projection for first consumers.
    pub support_export: RuntimeEvidenceSupportExport,
}

impl RuntimeEvidenceAlphaPacket {
    /// Returns the deterministic import/view-only alpha baseline.
    pub fn import_view_only_baseline() -> Self {
        let generated_at = "2026-05-14T18:45:00Z".to_owned();
        let execution_context_id = "exec:20260514T184500Z:runtime-evidence-alpha:0001".to_owned();
        let exact_build_identity_ref = "build-identity:aureline:alpha:0.0.0:runtime-evidence:0001";
        let profile_session_id = "profile-session:checkout-api-hot-path:0001".to_owned();
        let trace_bundle_id = "trace-bundle:checkout-api-hot-path:0001".to_owned();
        let replay_descriptor_id = "replay-capability:import-viewer:0001".to_owned();
        let comparison_packet_id = "comparison-class:checkout-api-hot-path:0001".to_owned();

        let exact_build_identity = BuildRuntimeIdentity {
            exact_build_identity_ref: exact_build_identity_ref.to_owned(),
            build_id: "build-id:aureline:alpha:0.0.0:darwin-aarch64:debug:9f0e7d6c5b4a".into(),
            commit_ref: "commit:9f0e7d6c5b4a".into(),
            runtime_identity: "rustc 1.75.0".into(),
            toolchain_identity_ref: "toolchain:rust:1.75.0:stable".into(),
            symbol_manifest_ref: "symbols:aureline:checkout-api-hot-path:0001".into(),
            source_map_manifest_ref: "source-map:aureline:checkout-api-hot-path:0001".into(),
            workspace_ref: "workspace:reference:checkout-api".into(),
        };
        let mapping_quality = MappingQualitySummary {
            state: MappingQualityState::SymbolizedWithPartialSourceMaps,
            symbol_manifest_ref: exact_build_identity.symbol_manifest_ref.clone(),
            source_map_manifest_ref: exact_build_identity.source_map_manifest_ref.clone(),
            exact_frame_count: 14,
            approximate_frame_count: 6,
            unresolved_frame_count: 3,
            note: "Imported profile resolves most frames but local sources are newer than the captured build.".into(),
        };

        let profile_session = ProfileSessionAlpha {
            record_kind: PROFILE_SESSION_ALPHA_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            profile_session_id: profile_session_id.clone(),
            capture: ProfileCaptureDescriptor {
                capture_mode: CaptureMode::ImportedProfile,
                capture_source: CaptureSource::ImportedBundle,
                capture_source_ref: "support-bundle:design-partner:checkout-api:trace-import:0001"
                    .into(),
                evidence_state: "imported_read_only".into(),
            },
            execution_context_id: execution_context_id.clone(),
            exact_build_identity: exact_build_identity.clone(),
            target: ProfileTargetIdentity {
                target_kind: "run_configuration".into(),
                target_ref: "run-config:checkout-api:test-hot-path".into(),
                target_label: "checkout-api test hot path".into(),
                process_ref: Some("process:remote-runner:28419".into()),
                run_config_ref: Some("run-config:checkout-api:test-hot-path".into()),
            },
            capture_window: CaptureWindow {
                started_at: "2026-05-14T18:40:00Z".into(),
                ended_at: "2026-05-14T18:40:18Z".into(),
                duration_ms: 18_300,
            },
            overhead_class: OverheadClass::Moderate,
            mapping_quality: mapping_quality.clone(),
            data_posture: RuntimeEvidenceDataPosture {
                data_class: RuntimeEvidenceDataClass::HighRiskRawCapture,
                redaction_mode: TraceRedactionMode::LocalOnlyRawRetained,
                support_pack_item_id: SUPPORT_ITEM_RUNTIME_TRACES.into(),
                raw_payload_policy:
                    "Raw trace/profile payloads stay local unless explicit export review widens them."
                        .into(),
            },
            trace_bundle_ref: trace_bundle_id.clone(),
            comparison_packet_ref: comparison_packet_id.clone(),
            export_policy: ProfileExportPolicy {
                default_scope: "manifest_and_redacted_summary".into(),
                raw_payload_export: "requires_explicit_review".into(),
                support_bundle_binding_ref:
                    "support-bundle-binding:runtime-evidence:checkout-api:0001".into(),
                retention_class: TraceRetentionClass::LocalRotationSevenDays,
            },
        };

        let trace_bundle = TraceBundleAlphaManifest {
            record_kind: TRACE_BUNDLE_ALPHA_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            bundle_id: trace_bundle_id.clone(),
            profile_session_ref: profile_session_id.clone(),
            execution_context_id: execution_context_id.clone(),
            exact_build_identity_ref: exact_build_identity_ref.into(),
            raw_bundle: RawTraceBundle {
                raw_bundle_ref: "local-retained-trace:checkout-api-hot-path:0001".into(),
                raw_bundle_kind: "perfetto_trace_plus_pprof_profile".into(),
                storage_class: "local_only_copy_retained".into(),
                chunk_refs: vec![
                    "trace-chunk:checkout-api-hot-path:cpu:0001".into(),
                    "trace-chunk:checkout-api-hot-path:runtime-events:0001".into(),
                ],
                metric_families: vec![
                    TraceMetricFamily::Cpu,
                    TraceMetricFamily::WallTime,
                    TraceMetricFamily::RuntimeEvents,
                ],
            },
            derived_views: vec![
                DerivedTraceView {
                    view_id: "trace-view:checkout-api-hot-path:flamegraph:0001".into(),
                    view_kind: DerivedViewKind::Flamegraph,
                    source_bundle_id: trace_bundle_id.clone(),
                    provenance_ref: "derived-view-provenance:flamegraph:0001".into(),
                    freshness_class: "derived_current_for_bundle_digest".into(),
                    digest_ref: "digest:trace-view:flamegraph:sha256:0001".into(),
                },
                DerivedTraceView {
                    view_id: "trace-view:checkout-api-hot-path:call-tree:0001".into(),
                    view_kind: DerivedViewKind::CallTree,
                    source_bundle_id: trace_bundle_id.clone(),
                    provenance_ref: "derived-view-provenance:call-tree:0001".into(),
                    freshness_class: "derived_current_for_bundle_digest".into(),
                    digest_ref: "digest:trace-view:call-tree:sha256:0001".into(),
                },
                DerivedTraceView {
                    view_id: "trace-view:checkout-api-hot-path:timeline:0001".into(),
                    view_kind: DerivedViewKind::Timeline,
                    source_bundle_id: trace_bundle_id.clone(),
                    provenance_ref: "derived-view-provenance:timeline:0001".into(),
                    freshness_class: "derived_current_for_bundle_digest".into(),
                    digest_ref: "digest:trace-view:timeline:sha256:0001".into(),
                },
            ],
            mapping_quality: mapping_quality.clone(),
            redaction: TraceBundleRedaction {
                redaction_mode: TraceRedactionMode::LocalOnlyRawRetained,
                redaction_profile_ref: "support.redaction.local_first_default".into(),
                raw_payload_export_default: "not_exported_without_explicit_review".into(),
                support_bundle_posture: "manifest_only_raw_payload_local".into(),
            },
            retention: TraceBundleRetention {
                retention_class: TraceRetentionClass::LocalRotationSevenDays,
                retention_owner: "local_runtime_evidence_cache".into(),
                expires_at: Some("2026-05-21T18:45:00Z".into()),
                pin_ref: None,
            },
            digest_set: vec![
                DigestEntry {
                    algorithm: DigestAlgorithm::Sha256,
                    object_ref: "local-retained-trace:checkout-api-hot-path:0001".into(),
                    digest: "sha256:0d8d2f3f1a4b2c5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6".into(),
                },
                DigestEntry {
                    algorithm: DigestAlgorithm::Sha256,
                    object_ref: "trace-view:checkout-api-hot-path:flamegraph:0001".into(),
                    digest: "sha256:1d8d2f3f1a4b2c5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6".into(),
                },
                DigestEntry {
                    algorithm: DigestAlgorithm::Sha256,
                    object_ref: "trace-view:checkout-api-hot-path:timeline:0001".into(),
                    digest: "sha256:2d8d2f3f1a4b2c5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6".into(),
                },
            ],
            immutability: TraceBundleImmutability {
                frozen_at: generated_at.clone(),
                immutability_state: "captured_immutable".into(),
                raw_bundle_digest_ref: "digest:raw-trace:checkout-api-hot-path:sha256:0001"
                    .into(),
                note: "Raw bundle is immutable; flamegraph, call-tree, and timeline views are derived artifacts.".into(),
            },
        };

        let replay_capability = ReplayCapabilityAlphaDescriptor {
            record_kind: REPLAY_CAPABILITY_ALPHA_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            descriptor_id: replay_descriptor_id.clone(),
            backend: ReplayBackendIdentity {
                backend_family: "import_viewer".into(),
                backend_version: "alpha.1".into(),
                adapter_version: "aureline-runtime-evidence-alpha.1".into(),
            },
            supported_ranges: vec![ReplayRuntimeToolchainRange {
                runtime_family: "rust".into(),
                supported_runtime_range: "1.75.x".into(),
                toolchain_family: "rustup_stable".into(),
                supported_toolchain_range: "1.75.x".into(),
            }],
            support_matrix: ReplaySupportMatrix {
                reverse_step: ReplayFeatureSupport {
                    state: ReplayFeatureState::Unsupported,
                    reason: "Imported bundle lacks deterministic execution history.".into(),
                },
                frame_inspection: ReplayFeatureSupport {
                    state: ReplayFeatureState::Limited,
                    reason: "Frame inspection is available only through derived call-tree rows.".into(),
                },
                data_inspection: ReplayFeatureSupport {
                    state: ReplayFeatureState::Unsupported,
                    reason: "Raw variable and memory snapshots are not captured in this lane.".into(),
                },
                timeline_scrub: ReplayFeatureSupport {
                    state: ReplayFeatureState::Supported,
                    reason: "Timeline scrub is available over immutable imported trace events.".into(),
                },
                checkpoint_bookmarks: ReplayFeatureSupport {
                    state: ReplayFeatureState::Limited,
                    reason: "Bookmarks can annotate imported events but cannot resume execution.".into(),
                },
            },
            lane_state: ReplayLaneState::ImportViewOnly,
            determinism_caveats: vec![
                "No deterministic scheduler state is present.".into(),
                "Kernel, host, and input stream state are not replayable from the imported bundle."
                    .into(),
                "Derived flamegraph and timeline views are inspectable evidence, not a live debug session."
                    .into(),
            ],
            overhead_storage_band: ReplayOverheadStorageBand {
                overhead_class: OverheadClass::Moderate,
                storage_growth_class: "bounded_imported_bundle".into(),
                cost_note: "Storage cost is bounded by the imported trace bundle; live recording is not active.".into(),
            },
            import_view_only_reason: Some(
                "The alpha baseline can inspect imported trace/profile evidence but does not control a live replay backend."
                    .into(),
            ),
            export_posture: ReplayExportPosture {
                data_class: RuntimeEvidenceDataClass::RedactedSummary,
                redaction_mode: TraceRedactionMode::MetadataAndHashesOnly,
                retention_class: TraceRetentionClass::SupportBundleManifestOnly,
                raw_snapshots_exported_by_default: false,
            },
        };

        let comparison = ComparisonClassAlphaPacket {
            record_kind: COMPARISON_CLASS_ALPHA_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            comparison_packet_id: comparison_packet_id.clone(),
            workload_id: "workload:checkout-api:test-hot-path".into(),
            corpus_archetype: "reference_workspace_checkout_api_hot_path".into(),
            source_class: ComparisonSourceClass::ImportedReadOnly,
            hardware_power_profile: HardwarePowerProfile {
                hardware_class: "remote_linux_runner_standard".into(),
                cpu_arch_class: "x86_64".into(),
                power_state: "provider_controlled_unknown".into(),
                thermal_state: "unknown_imported".into(),
            },
            runtime_toolchain: ComparisonRuntimeToolchain {
                runtime_identity: "rustc 1.75.0".into(),
                toolchain_identity_ref: "toolchain:rust:1.75.0:stable".into(),
                capture_mode: CaptureMode::ImportedProfile,
            },
            baseline_ref: "baseline:checkout-api-hot-path:linux-release:20260514".into(),
            current_profile_session_ref: profile_session_id.clone(),
            variance_window: VarianceWindow {
                sample_count: 5,
                window_label: "five imported samples from remote runner".into(),
                relative_variance_bps: 420,
                threshold_state: "advisory_only".into(),
            },
            comparison_class: ComparisonClass::ImportViewOnlyNotComparable,
            confounders: vec![
                "Captured remotely while the current workspace is local.".into(),
                "Local sources are newer than the captured build.".into(),
                "Power and thermal state are imported as unknown.".into(),
            ],
        };

        let support_export = RuntimeEvidenceSupportExport {
            record_kind: RUNTIME_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            export_id: "runtime-evidence-support-export:checkout-api-hot-path:0001".into(),
            profile_session_id: profile_session_id.clone(),
            trace_bundle_id: trace_bundle_id.clone(),
            replay_descriptor_id: replay_descriptor_id.clone(),
            comparison_packet_id: comparison_packet_id.clone(),
            execution_context_id: execution_context_id.clone(),
            exact_build_identity_ref: exact_build_identity_ref.into(),
            mapping_quality_state: mapping_quality.state,
            comparison_class: comparison.comparison_class,
            redaction_mode: trace_bundle.redaction.redaction_mode,
            retention_class: trace_bundle.retention.retention_class,
            replay_lane_state: replay_capability.lane_state,
            support_pack_item_id: SUPPORT_ITEM_RUNTIME_TRACES.into(),
            raw_payload_exported: false,
            import_view_only: true,
        };

        Self {
            record_kind: RUNTIME_EVIDENCE_ALPHA_PACKET_RECORD_KIND.into(),
            schema_version: RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION,
            packet_id: "runtime-evidence-alpha:checkout-api-hot-path:0001".into(),
            generated_at,
            profile_session,
            trace_bundle,
            replay_capability,
            comparison,
            support_export,
        }
    }

    /// Returns true when profile, trace, comparison, and support export share one context.
    pub fn shares_one_execution_context(&self) -> bool {
        let context = &self.profile_session.execution_context_id;
        self.trace_bundle.execution_context_id == *context
            && self.support_export.execution_context_id == *context
    }

    /// Returns true when profile, trace, and support export share one build identity.
    pub fn shares_one_exact_build_identity(&self) -> bool {
        let build = &self
            .profile_session
            .exact_build_identity
            .exact_build_identity_ref;
        self.trace_bundle.exact_build_identity_ref == *build
            && self.support_export.exact_build_identity_ref == *build
    }

    /// Returns true when the packet is an honest import/view-only replay lane.
    pub fn preserves_import_view_only_truth(&self) -> bool {
        self.replay_capability.is_import_view_only()
            && self.support_export.import_view_only
            && !self.replay_capability.lane_state.claims_live_replay()
            && !self.support_export.raw_payload_exported
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn repo_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("crate is under repo/crates")
    }

    #[test]
    fn import_view_only_baseline_preserves_context_build_and_replay_truth() {
        let packet = RuntimeEvidenceAlphaPacket::import_view_only_baseline();

        assert!(packet.profile_session.has_attribution());
        assert!(packet.trace_bundle.is_immutable_manifest());
        assert!(packet.shares_one_execution_context());
        assert!(packet.shares_one_exact_build_identity());
        assert!(packet.preserves_import_view_only_truth());
        assert_eq!(
            packet.comparison.comparison_class,
            ComparisonClass::ImportViewOnlyNotComparable
        );
        assert_eq!(
            packet.support_export.mapping_quality_state,
            MappingQualityState::SymbolizedWithPartialSourceMaps
        );
    }

    #[test]
    fn fixture_round_trips_through_runtime_contract() {
        let fixture = repo_root()
            .join("fixtures")
            .join("runtime")
            .join("trace_bundle_alpha")
            .join("runtime_evidence_import_view_only.json");
        let payload = fs::read_to_string(&fixture)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", fixture.display()));
        let packet: RuntimeEvidenceAlphaPacket = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", fixture.display()));

        assert_eq!(
            packet.record_kind,
            RUNTIME_EVIDENCE_ALPHA_PACKET_RECORD_KIND
        );
        assert!(packet.shares_one_execution_context());
        assert!(packet.shares_one_exact_build_identity());
        assert!(packet.preserves_import_view_only_truth());
        assert!(!packet.comparison.is_like_for_like());
    }

    #[test]
    fn checked_in_replay_capability_descriptor_matches_runtime_vocabulary() {
        let artifact = repo_root()
            .join("artifacts")
            .join("runtime")
            .join("replay_capability_alpha.yaml");
        let payload = fs::read_to_string(&artifact)
            .unwrap_or_else(|err| panic!("read artifact {}: {err}", artifact.display()));
        let descriptor: ReplayCapabilityAlphaDescriptor = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse artifact {}: {err}", artifact.display()));

        assert_eq!(descriptor.lane_state, ReplayLaneState::ImportViewOnly);
        assert!(descriptor.is_import_view_only());
        assert_eq!(
            descriptor.support_matrix.reverse_step.state,
            ReplayFeatureState::Unsupported
        );
        assert!(!descriptor.export_posture.raw_snapshots_exported_by_default);
    }

    #[test]
    fn checked_in_comparison_class_packet_discloses_import_view_only_limits() {
        let artifact = repo_root()
            .join("artifacts")
            .join("runtime")
            .join("comparison_class_alpha.yaml");
        let payload = fs::read_to_string(&artifact)
            .unwrap_or_else(|err| panic!("read artifact {}: {err}", artifact.display()));
        let comparison: ComparisonClassAlphaPacket = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse artifact {}: {err}", artifact.display()));

        assert_eq!(
            comparison.comparison_class,
            ComparisonClass::ImportViewOnlyNotComparable
        );
        assert!(!comparison.is_like_for_like());
        assert_eq!(comparison.variance_window.sample_count, 5);
        assert!(!comparison.confounders.is_empty());
    }
}
