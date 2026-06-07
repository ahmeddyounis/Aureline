//! Stable task-event, problem, output-channel, and execution-evidence bundle.
//!
//! This module materializes the contract described by the runtime M4 execution
//! evidence lane: task events, problem records, output channels, output chunks,
//! evidence objects, and export bundles share one serializable truth model.
//! Problems, output tabs, timeline entries, CLI/headless output, review, AI
//! explanations, and support exports consume these records instead of scraping
//! rendered pane text or flattening provider-specific output.
//!
//! The reviewer-facing contract lives at
//! [`/docs/runtime/m4/stabilize-problem-records-output-channels-and-execution-evidence.md`](../../../docs/runtime/m4/stabilize-problem-records-output-channels-and-execution-evidence.md)
//! and the machine-readable boundary schema lives at
//! [`/schemas/runtime/execution-evidence-bundle.schema.json`](../../../schemas/runtime/execution-evidence-bundle.schema.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ExecutionEvidenceBundle`].
pub const EXECUTION_EVIDENCE_BUNDLE_RECORD_KIND: &str = "execution_evidence_bundle";

/// Stable record-kind tag for [`ExecutionEvidenceSupportExport`].
pub const EXECUTION_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "execution_evidence_bundle_support_export";

/// Integer schema version for the execution-evidence bundle.
pub const EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EXECUTION_EVIDENCE_BUNDLE_SCHEMA_REF: &str =
    "schemas/runtime/execution-evidence-bundle.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const EXECUTION_EVIDENCE_BUNDLE_DOC_REF: &str =
    "docs/runtime/m4/stabilize-problem-records-output-channels-and-execution-evidence.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const EXECUTION_EVIDENCE_BUNDLE_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-problem-records-output-channels-and-execution-evidence.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const EXECUTION_EVIDENCE_BUNDLE_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_problem_records_output_channels_and_execution_evidence";

/// Repo-relative path of the checked-in stable packet.
pub const EXECUTION_EVIDENCE_BUNDLE_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_problem_records_output_channels_and_execution_evidence_truth_packet.json";

/// Source class for task events, problems, output channels, and evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionSourceKind {
    /// Native Aureline adapter or local runtime producer.
    Native,
    /// Build Server Protocol producer.
    Bsp,
    /// Bazel Build Event Protocol producer.
    BazelBep,
    /// Structured imported tool output such as JUnit, SARIF, or tool JSON.
    StructuredOutput,
    /// Heuristic parser or problem matcher over raw output.
    HeuristicParser,
    /// Imported provider evidence such as CI annotations or mirrored logs.
    ImportedProvider,
}

impl ExecutionSourceKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bsp => "bsp",
            Self::BazelBep => "bazel_bep",
            Self::StructuredOutput => "structured_output",
            Self::HeuristicParser => "heuristic_parser",
            Self::ImportedProvider => "imported_provider",
        }
    }

    const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedProvider)
    }

    const fn is_heuristic(self) -> bool {
        matches!(self, Self::HeuristicParser)
    }
}

/// Confidence class preserved by task-event, problem, and parser records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceConfidenceClass {
    /// High confidence from a native or structured source.
    High,
    /// Medium-high confidence from a structured but translated source.
    MediumHigh,
    /// Medium confidence from partial structured evidence.
    Medium,
    /// Low confidence from partial or stale evidence.
    Low,
    /// Best-effort confidence from heuristic output parsing.
    HeuristicBestEffort,
    /// Unknown confidence, which never certifies stable.
    Unknown,
}

impl EvidenceConfidenceClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::MediumHigh => "medium_high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::HeuristicBestEffort => "heuristic_best_effort",
            Self::Unknown => "unknown",
        }
    }

    const fn is_bound(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Canonical task-event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTaskEventKind {
    /// Task or run was queued.
    TaskQueued,
    /// Task or run started.
    TaskStarted,
    /// Progress update was emitted.
    ProgressUpdated,
    /// Output was appended.
    OutputAppended,
    /// Diagnostic or problem was emitted.
    DiagnosticEmitted,
    /// Test case started.
    TestCaseStarted,
    /// Test case finished.
    TestCaseFinished,
    /// Artifact was published.
    ArtifactPublished,
    /// Task or run finished.
    TaskFinished,
    /// Debug event was emitted.
    DebugEvent,
}

impl EvidenceTaskEventKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskQueued => "task_queued",
            Self::TaskStarted => "task_started",
            Self::ProgressUpdated => "progress_updated",
            Self::OutputAppended => "output_appended",
            Self::DiagnosticEmitted => "diagnostic_emitted",
            Self::TestCaseStarted => "test_case_started",
            Self::TestCaseFinished => "test_case_finished",
            Self::ArtifactPublished => "artifact_published",
            Self::TaskFinished => "task_finished",
            Self::DebugEvent => "debug_event",
        }
    }
}

/// Problem severity independent of source confidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSeverity {
    /// Informational problem.
    Info,
    /// Warning problem.
    Warning,
    /// Error problem.
    Error,
    /// Fatal problem.
    Fatal,
}

impl ProblemSeverity {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }
}

/// Freshness state preserved when evidence is projected into other surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessState {
    /// Current local runtime truth.
    CurrentLocal,
    /// Current remote/helper runtime truth.
    CurrentRemote,
    /// Imported provider evidence whose provider freshness is preserved.
    ImportedProvider,
    /// Stale evidence retained for explanation only.
    Stale,
    /// Replayed evidence from a support or review bundle.
    Replayed,
    /// Unknown freshness, which never certifies stable.
    Unknown,
}

impl EvidenceFreshnessState {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocal => "current_local",
            Self::CurrentRemote => "current_remote",
            Self::ImportedProvider => "imported_provider",
            Self::Stale => "stale",
            Self::Replayed => "replayed",
            Self::Unknown => "unknown",
        }
    }

    const fn is_bound(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Trust state for output channels and execution evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputTrustState {
    /// Local runtime truth.
    LocalRuntimeTruth,
    /// Remote/helper runtime truth.
    RemoteRuntimeTruth,
    /// Imported provider truth, distinct from local state.
    ImportedProviderTruth,
    /// Heuristic fallback truth.
    HeuristicFallback,
    /// Stale projection.
    StaleProjection,
    /// Unknown trust state, which never certifies stable.
    Unknown,
}

impl OutputTrustState {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRuntimeTruth => "local_runtime_truth",
            Self::RemoteRuntimeTruth => "remote_runtime_truth",
            Self::ImportedProviderTruth => "imported_provider_truth",
            Self::HeuristicFallback => "heuristic_fallback",
            Self::StaleProjection => "stale_projection",
            Self::Unknown => "unknown",
        }
    }

    const fn is_bound(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Output retention class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputRetentionClass {
    /// Retained for the run lifecycle.
    RunScoped,
    /// Pinned for support or review.
    PinnedForSupport,
    /// Metadata-only retention.
    MetadataOnly,
    /// Explicitly redacted retention.
    Redacted,
    /// Unknown retention, which never certifies stable.
    Unknown,
}

impl OutputRetentionClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunScoped => "run_scoped",
            Self::PinnedForSupport => "pinned_for_support",
            Self::MetadataOnly => "metadata_only",
            Self::Redacted => "redacted",
            Self::Unknown => "unknown",
        }
    }

    const fn is_bound(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Stable canonical output-channel names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalOutputChannelName {
    /// Task output.
    Task,
    /// Test output.
    Test,
    /// Debug output.
    Debug,
    /// Notebook output.
    Notebook,
    /// Imported provider output.
    ProviderImported,
    /// AI/tool output.
    AiTool,
    /// Extension output.
    Extension,
}

impl CanonicalOutputChannelName {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Notebook => "notebook",
            Self::ProviderImported => "provider_imported",
            Self::AiTool => "ai_tool",
            Self::Extension => "extension",
        }
    }
}

/// Render class for one output chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputChunkRenderClass {
    /// Plain text output.
    PlainText,
    /// Structured JSON output.
    Json,
    /// Structured diagnostic output.
    Diagnostic,
    /// Test report output.
    TestReport,
    /// Redacted output.
    Redacted,
}

impl OutputChunkRenderClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::Json => "json",
            Self::Diagnostic => "diagnostic",
            Self::TestReport => "test_report",
            Self::Redacted => "redacted",
        }
    }
}

/// Qualifier that changes review risk or export interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceQualifier {
    /// Generated artifact or finding.
    Generated,
    /// External or provider-owned artifact.
    External,
    /// Test-only finding or artifact.
    TestOnly,
    /// Read/write capable action or artifact.
    ReadWrite,
    /// Scope-limited evidence.
    ScopeLimited,
}

impl EvidenceQualifier {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::External => "external",
            Self::TestOnly => "test_only",
            Self::ReadWrite => "read_write",
            Self::ScopeLimited => "scope_limited",
        }
    }
}

/// Execution evidence kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionEvidenceKind {
    /// Problem evidence.
    Problem,
    /// Output evidence.
    Output,
    /// Artifact evidence.
    Artifact,
    /// Review annotation evidence.
    ReviewAnnotation,
    /// Coverage evidence.
    Coverage,
    /// Test result evidence.
    TestResult,
}

impl ExecutionEvidenceKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Problem => "problem",
            Self::Output => "output",
            Self::Artifact => "artifact",
            Self::ReviewAnnotation => "review_annotation",
            Self::Coverage => "coverage",
            Self::TestResult => "test_result",
        }
    }
}

/// Consumer surface that must preserve the bundle semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceConsumerSurface {
    /// Problems panel.
    ProblemsPanel,
    /// Editor decorations.
    EditorDecorations,
    /// Output tabs.
    OutputTabs,
    /// Timeline entries.
    Timeline,
    /// CLI/headless output.
    CliHeadless,
    /// Review surface.
    Review,
    /// AI explanation surface.
    AiExplanation,
    /// Support export.
    SupportExport,
    /// Release packet.
    ReleasePacket,
}

impl EvidenceConsumerSurface {
    /// Every required consumer surface for the stable evidence bundle.
    pub const REQUIRED: [Self; 9] = [
        Self::ProblemsPanel,
        Self::EditorDecorations,
        Self::OutputTabs,
        Self::Timeline,
        Self::CliHeadless,
        Self::Review,
        Self::AiExplanation,
        Self::SupportExport,
        Self::ReleasePacket,
    ];

    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsPanel => "problems_panel",
            Self::EditorDecorations => "editor_decorations",
            Self::OutputTabs => "output_tabs",
            Self::Timeline => "timeline",
            Self::CliHeadless => "cli_headless",
            Self::Review => "review",
            Self::AiExplanation => "ai_explanation",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
        }
    }
}

/// Promotion state derived from bundle validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePromotionState {
    /// Bundle certifies stable execution evidence.
    Stable,
    /// Bundle narrows below stable.
    NarrowedBelowStable,
    /// Bundle blocks stable publication.
    BlocksStable,
}

impl EvidencePromotionState {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFindingSeverity {
    /// Informational finding.
    Info,
    /// Warning finding.
    Warning,
    /// Blocker finding.
    Blocker,
}

/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity is empty.
    MissingIdentity,
    /// Task-event coverage is missing.
    MissingTaskEvent,
    /// Problem coverage is missing.
    MissingProblemRecord,
    /// Output-channel coverage is missing.
    MissingOutputChannel,
    /// Execution-evidence object coverage is missing.
    MissingEvidenceObject,
    /// Confidence class is missing or unknown.
    MissingConfidence,
    /// Heuristic parser evidence lacks a raw-output backlink.
    HeuristicMissingRawOutputBacklink,
    /// Heuristic parser evidence claims too much confidence.
    HeuristicOverclaimsConfidence,
    /// Problem record lacks source run, task event, or static-analysis origin.
    ProblemMissingOriginRef,
    /// Problem record lacks required editor/timeline/rerun correlation.
    ProblemMissingProjectionRef,
    /// Problem record lacks quick-fix refs.
    ProblemMissingQuickFixRefs,
    /// Imported provider evidence is flattened into local truth.
    ImportedEvidenceFlattenedIntoLocalTruth,
    /// Imported provider evidence lacks provider backlink.
    ImportedEvidenceMissingProviderBacklink,
    /// Output channel has no chunks.
    OutputChannelMissingChunks,
    /// Output channel is not virtualization-safe.
    OutputChannelNotVirtualizationSafe,
    /// Output chunk is not append-friendly or searchable.
    OutputChunkNotAppendFriendly,
    /// Output channel lacks trust, retention, execution context, or run linkage.
    OutputChannelMissingStableDescriptor,
    /// Evidence object lacks reopen lineage.
    EvidenceMissingReopenLineage,
    /// Evidence object references an unknown task event, problem, or output chunk.
    EvidenceDanglingRef,
    /// Evidence object lacks freshness state.
    EvidenceMissingFreshness,
    /// Bundle lacks redaction profile.
    BundleMissingRedactionProfile,
    /// Bundle lacks reopen lineage.
    BundleMissingReopenLineage,
    /// Required consumer projection is missing.
    MissingConsumerProjection,
    /// Consumer projection drops stable object semantics.
    ConsumerProjectionDrift,
    /// Raw private material is admitted past the bundle boundary.
    RawPrivateMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl EvidenceFindingKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingTaskEvent => "missing_task_event",
            Self::MissingProblemRecord => "missing_problem_record",
            Self::MissingOutputChannel => "missing_output_channel",
            Self::MissingEvidenceObject => "missing_evidence_object",
            Self::MissingConfidence => "missing_confidence",
            Self::HeuristicMissingRawOutputBacklink => "heuristic_missing_raw_output_backlink",
            Self::HeuristicOverclaimsConfidence => "heuristic_overclaims_confidence",
            Self::ProblemMissingOriginRef => "problem_missing_origin_ref",
            Self::ProblemMissingProjectionRef => "problem_missing_projection_ref",
            Self::ProblemMissingQuickFixRefs => "problem_missing_quick_fix_refs",
            Self::ImportedEvidenceFlattenedIntoLocalTruth => {
                "imported_evidence_flattened_into_local_truth"
            }
            Self::ImportedEvidenceMissingProviderBacklink => {
                "imported_evidence_missing_provider_backlink"
            }
            Self::OutputChannelMissingChunks => "output_channel_missing_chunks",
            Self::OutputChannelNotVirtualizationSafe => "output_channel_not_virtualization_safe",
            Self::OutputChunkNotAppendFriendly => "output_chunk_not_append_friendly",
            Self::OutputChannelMissingStableDescriptor => {
                "output_channel_missing_stable_descriptor"
            }
            Self::EvidenceMissingReopenLineage => "evidence_missing_reopen_lineage",
            Self::EvidenceDanglingRef => "evidence_dangling_ref",
            Self::EvidenceMissingFreshness => "evidence_missing_freshness",
            Self::BundleMissingRedactionProfile => "bundle_missing_redaction_profile",
            Self::BundleMissingReopenLineage => "bundle_missing_reopen_lineage",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RawPrivateMaterialPresent => "raw_private_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// File/span or structured problem location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemLocation {
    /// File URI or structured target ref.
    pub uri: String,
    /// Optional 1-based start line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u32>,
    /// Optional 1-based start column.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    /// Optional 1-based end line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// Optional 1-based end column.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
    /// Optional structured location ref when no file span is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_location_ref: Option<String>,
}

impl ProblemLocation {
    fn is_bound(&self) -> bool {
        !self.uri.trim().is_empty() || self.structured_location_ref.as_deref().is_some()
    }
}

/// Versioned task-event envelope linking execution source and causal refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableTaskEventEnvelope {
    /// Stable event id.
    pub event_id: String,
    /// Canonical event kind.
    pub event_kind: EvidenceTaskEventKind,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Run/session linkage.
    pub run_session_ref: String,
    /// Adapter/provider source kind.
    pub source_kind: ExecutionSourceKind,
    /// Adapter identity ref.
    pub adapter_ref: String,
    /// Optional provider identity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_ref: Option<String>,
    /// Timestamp or sequence token from the producing execution context.
    pub occurred_at_or_sequence: String,
    /// Parent event refs.
    #[serde(default)]
    pub parent_event_refs: Vec<String>,
    /// Correlation refs.
    #[serde(default)]
    pub correlation_refs: Vec<String>,
    /// Confidence class.
    pub confidence_class: EvidenceConfidenceClass,
    /// True when this event came from a fallback path.
    pub fallback_active: bool,
    /// Optional original adapter payload ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_payload_ref: Option<String>,
}

/// Stable problem row shared across Problems, editor, timeline, and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProblemRecord {
    /// Stable problem id.
    pub problem_id: String,
    /// Problem severity.
    pub severity: ProblemSeverity,
    /// Support-safe problem message.
    pub message: String,
    /// File/span or structured location.
    pub location: ProblemLocation,
    /// Source tool or subsystem ref.
    pub source_tool_ref: String,
    /// Source kind.
    pub source_kind: ExecutionSourceKind,
    /// Optional task-event ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_event_ref: Option<String>,
    /// Optional static-analysis ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_analysis_ref: Option<String>,
    /// Confidence class.
    pub confidence_class: EvidenceConfidenceClass,
    /// Quick-fix refs visible to action surfaces.
    #[serde(default)]
    pub quick_fix_refs: Vec<String>,
    /// Optional raw-output backlink required for heuristic parsing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_output_backlink: Option<String>,
    /// Optional provider evidence backlink required for provider imports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_evidence_backlink: Option<String>,
    /// Optional owning output-channel ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_channel_ref: Option<String>,
    /// Freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// Qualifiers that affect review risk.
    #[serde(default)]
    pub qualifiers: Vec<EvidenceQualifier>,
    /// Editor decoration ref.
    pub editor_decoration_ref: String,
    /// Timeline entry ref.
    pub timeline_entry_ref: String,
    /// Optional rerun action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_action_ref: Option<String>,
}

impl StableProblemRecord {
    fn has_origin_ref(&self) -> bool {
        self.task_event_ref
            .as_deref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
            || self
                .static_analysis_ref
                .as_deref()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
    }
}

/// Append-friendly output chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableOutputChunk {
    /// Stable chunk id.
    pub chunk_id: String,
    /// Inclusive starting sequence number.
    pub sequence_start: u64,
    /// Inclusive ending sequence number.
    pub sequence_end: u64,
    /// Optional byte-start offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_start: Option<u64>,
    /// Optional byte-end offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_end: Option<u64>,
    /// Timestamp or sequence token for the chunk.
    pub timestamp_or_sequence: String,
    /// Render class.
    pub render_class: OutputChunkRenderClass,
    /// Repeat-count metadata for noisy output.
    pub repeat_count: u32,
    /// Link parse refs.
    #[serde(default)]
    pub link_refs: Vec<String>,
    /// Redaction profile ref used for this chunk.
    pub redaction_profile_ref: String,
    /// True when chunk can be appended without rewriting earlier chunks.
    pub append_safe: bool,
    /// True when chunk can be searched independently under virtualization.
    pub searchable: bool,
}

/// Canonical output-channel descriptor plus append-friendly chunks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableOutputChannelDescriptor {
    /// Stable channel id.
    pub channel_id: String,
    /// Stable canonical channel name.
    pub canonical_name: CanonicalOutputChannelName,
    /// Source subsystem.
    pub source_subsystem: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Run/session linkage.
    pub run_session_ref: String,
    /// Source kind.
    pub source_kind: ExecutionSourceKind,
    /// Trust state.
    pub trust_state: OutputTrustState,
    /// Retention class.
    pub retention_class: OutputRetentionClass,
    /// Freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// True when large logs remain virtualization-safe.
    pub virtualization_safe: bool,
    /// True when a heuristic parser owns some channel projection.
    pub heuristic_parser_active: bool,
    /// Optional parser confidence label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parser_confidence: Option<EvidenceConfidenceClass>,
    /// Optional raw-output backlink.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_output_backlink: Option<String>,
    /// Optional imported provider ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_provider_ref: Option<String>,
    /// Append-friendly searchable chunks.
    #[serde(default)]
    pub chunks: Vec<StableOutputChunk>,
}

/// Execution evidence object projected into review, editor, or export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableExecutionEvidenceObject {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Evidence kind.
    pub evidence_kind: ExecutionEvidenceKind,
    /// Run or provider ref.
    pub run_or_provider_ref: String,
    /// Source kind.
    pub source_kind: ExecutionSourceKind,
    /// Artifact refs.
    #[serde(default)]
    pub artifact_refs: Vec<String>,
    /// Mapping refs.
    #[serde(default)]
    pub mapping_refs: Vec<String>,
    /// Freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// Optional baseline or comparison ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_or_comparison_ref: Option<String>,
    /// Linked problem refs.
    #[serde(default)]
    pub problem_refs: Vec<String>,
    /// Linked output chunk refs.
    #[serde(default)]
    pub output_chunk_refs: Vec<String>,
    /// Linked task-event refs.
    #[serde(default)]
    pub task_event_refs: Vec<String>,
    /// Qualifiers that affect review risk.
    #[serde(default)]
    pub qualifiers: Vec<EvidenceQualifier>,
    /// Reopen refs for run, channel, artifact, or provider detail.
    #[serde(default)]
    pub reopen_refs: Vec<String>,
    /// Trust state.
    pub trust_state: OutputTrustState,
}

/// Consumer projection proving a surface consumes the stable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: EvidenceConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Bundle id consumed by the projection.
    pub bundle_id_ref: String,
    /// True when stable object ids are preserved.
    pub preserves_object_refs: bool,
    /// True when source kind remains visible.
    pub preserves_source_kind: bool,
    /// True when confidence labels remain visible.
    pub preserves_confidence: bool,
    /// True when freshness and stale/imported flags remain visible.
    pub preserves_freshness: bool,
    /// True when reopen lineage remains available.
    pub preserves_reopen_lineage: bool,
    /// True when export-safe JSON is available.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl EvidenceConsumerProjection {
    fn preserves_truth_for(&self, bundle_id: &str) -> bool {
        self.bundle_id_ref == bundle_id
            && !self.projection_ref.trim().is_empty()
            && self.preserves_object_refs
            && self.preserves_source_kind
            && self.preserves_confidence
            && self.preserves_freshness
            && self.preserves_reopen_lineage
            && self.supports_json_export
            && self.raw_private_material_excluded
    }
}

/// One validation finding emitted by the bundle validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceValidationFinding {
    /// Closed finding kind.
    pub finding_kind: EvidenceFindingKind,
    /// Finding severity.
    pub severity: EvidenceFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl EvidenceValidationFinding {
    fn new(
        finding_kind: EvidenceFindingKind,
        severity: EvidenceFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`ExecutionEvidenceBundle::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEvidenceBundleInput {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Run/session linkage.
    pub run_session_ref: String,
    /// Task-event envelopes.
    #[serde(default)]
    pub task_events: Vec<StableTaskEventEnvelope>,
    /// Problem records.
    #[serde(default)]
    pub problems: Vec<StableProblemRecord>,
    /// Output-channel descriptors.
    #[serde(default)]
    pub output_channels: Vec<StableOutputChannelDescriptor>,
    /// Evidence objects.
    #[serde(default)]
    pub evidence_objects: Vec<StableExecutionEvidenceObject>,
    /// Artifact refs included in the bundle.
    #[serde(default)]
    pub artifact_refs: Vec<String>,
    /// Mapping refs included in the bundle.
    #[serde(default)]
    pub mapping_refs: Vec<String>,
    /// Redaction profile ref.
    pub redaction_profile_ref: String,
    /// Reopen lineage refs.
    #[serde(default)]
    pub reopen_lineage_refs: Vec<String>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<EvidenceConsumerProjection>,
    /// True when raw private material is excluded from the bundle.
    pub raw_private_material_excluded: bool,
}

/// Export-safe bundle linking task events, problems, output chunks, artifacts,
/// mapping refs, redaction profile, and reopen lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEvidenceBundle {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Run/session linkage.
    pub run_session_ref: String,
    /// Task-event envelopes.
    #[serde(default)]
    pub task_events: Vec<StableTaskEventEnvelope>,
    /// Problem records.
    #[serde(default)]
    pub problems: Vec<StableProblemRecord>,
    /// Output-channel descriptors.
    #[serde(default)]
    pub output_channels: Vec<StableOutputChannelDescriptor>,
    /// Evidence objects.
    #[serde(default)]
    pub evidence_objects: Vec<StableExecutionEvidenceObject>,
    /// Artifact refs included in the bundle.
    #[serde(default)]
    pub artifact_refs: Vec<String>,
    /// Mapping refs included in the bundle.
    #[serde(default)]
    pub mapping_refs: Vec<String>,
    /// Redaction profile ref.
    pub redaction_profile_ref: String,
    /// Reopen lineage refs.
    #[serde(default)]
    pub reopen_lineage_refs: Vec<String>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<EvidenceConsumerProjection>,
    /// True when raw private material is excluded from the bundle.
    pub raw_private_material_excluded: bool,
    /// Derived promotion state.
    pub promotion_state: EvidencePromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<EvidenceValidationFinding>,
}

impl ExecutionEvidenceBundle {
    /// Materializes a bundle and records derived validation findings.
    pub fn materialize(input: ExecutionEvidenceBundleInput) -> Self {
        let mut bundle = Self {
            record_kind: EXECUTION_EVIDENCE_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION,
            bundle_id: input.bundle_id,
            generated_at: input.generated_at,
            execution_context_ref: input.execution_context_ref,
            run_session_ref: input.run_session_ref,
            task_events: input.task_events,
            problems: input.problems,
            output_channels: input.output_channels,
            evidence_objects: input.evidence_objects,
            artifact_refs: input.artifact_refs,
            mapping_refs: input.mapping_refs,
            redaction_profile_ref: input.redaction_profile_ref,
            reopen_lineage_refs: input.reopen_lineage_refs,
            source_contract_refs: input.source_contract_refs,
            consumer_projections: input.consumer_projections,
            raw_private_material_excluded: input.raw_private_material_excluded,
            promotion_state: EvidencePromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = bundle.derived_findings(false);
        bundle.promotion_state = promotion_state_for_findings(&findings);
        bundle.validation_findings = findings;
        bundle
    }

    /// Re-validates the bundle against the stable execution-evidence invariants.
    pub fn validate(&self) -> Vec<EvidenceValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this bundle has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == EvidenceFindingSeverity::Blocker)
    }

    /// Builds a support export wrapping the exact bundle.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ExecutionEvidenceSupportExport {
        ExecutionEvidenceSupportExport {
            record_kind: EXECUTION_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            bundle_id_ref: self.bundle_id.clone(),
            redaction_profile_ref: self.redaction_profile_ref.clone(),
            raw_private_material_excluded: self.raw_private_material_excluded,
            bundle: self.clone(),
        }
    }

    /// Returns the unique source-kind tokens observed across bundle objects.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.task_events {
            set.insert(event.source_kind);
        }
        for problem in &self.problems {
            set.insert(problem.source_kind);
        }
        for channel in &self.output_channels {
            set.insert(channel.source_kind);
        }
        for evidence in &self.evidence_objects {
            set.insert(evidence.source_kind);
        }
        set.into_iter().map(ExecutionSourceKind::as_str).collect()
    }

    /// Returns the unique canonical output-channel name tokens.
    pub fn output_channel_name_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for channel in &self.output_channels {
            set.insert(channel.canonical_name);
        }
        set.into_iter()
            .map(CanonicalOutputChannelName::as_str)
            .collect()
    }

    /// Returns the unique consumer-surface tokens.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for projection in &self.consumer_projections {
            set.insert(projection.consumer_surface);
        }
        set.into_iter()
            .map(EvidenceConsumerSurface::as_str)
            .collect()
    }

    fn has_projection_for(&self, surface: EvidenceConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.bundle_id)
        })
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<EvidenceValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != EXECUTION_EVIDENCE_BUNDLE_RECORD_KIND {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::WrongRecordKind,
                EvidenceFindingSeverity::Blocker,
                "execution-evidence bundle has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION
        {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::WrongSchemaVersion,
                EvidenceFindingSeverity::Blocker,
                "execution-evidence bundle has the wrong schema version",
            ));
        }
        if self.bundle_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.execution_context_ref.trim().is_empty()
            || self.run_session_ref.trim().is_empty()
        {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::MissingIdentity,
                EvidenceFindingSeverity::Blocker,
                "bundle id, timestamp, execution context, and run session are required",
            ));
        }
        if self.task_events.is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::MissingTaskEvent,
                EvidenceFindingSeverity::Blocker,
                "bundle must carry at least one versioned task-event envelope",
            ));
        }
        if self.problems.is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::MissingProblemRecord,
                EvidenceFindingSeverity::Blocker,
                "bundle must carry at least one problem record",
            ));
        }
        if self.output_channels.is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::MissingOutputChannel,
                EvidenceFindingSeverity::Blocker,
                "bundle must carry at least one output-channel descriptor",
            ));
        }
        if self.evidence_objects.is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::MissingEvidenceObject,
                EvidenceFindingSeverity::Blocker,
                "bundle must carry at least one execution-evidence object",
            ));
        }
        if self.redaction_profile_ref.trim().is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::BundleMissingRedactionProfile,
                EvidenceFindingSeverity::Blocker,
                "bundle must name its redaction profile",
            ));
        }
        if self.reopen_lineage_refs.is_empty() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::BundleMissingReopenLineage,
                EvidenceFindingSeverity::Blocker,
                "bundle must preserve reopen lineage refs",
            ));
        }
        if !self.raw_private_material_excluded {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::RawPrivateMaterialPresent,
                EvidenceFindingSeverity::Blocker,
                "bundle admits raw private material past the export boundary",
            ));
        }

        let event_ids: BTreeSet<&str> = self
            .task_events
            .iter()
            .map(|event| event.event_id.as_str())
            .collect();
        let problem_ids: BTreeSet<&str> = self
            .problems
            .iter()
            .map(|problem| problem.problem_id.as_str())
            .collect();
        let channel_ids: BTreeSet<&str> = self
            .output_channels
            .iter()
            .map(|channel| channel.channel_id.as_str())
            .collect();
        let chunk_ids: BTreeSet<&str> = self
            .output_channels
            .iter()
            .flat_map(|channel| channel.chunks.iter())
            .map(|chunk| chunk.chunk_id.as_str())
            .collect();

        for event in &self.task_events {
            if event.event_id.trim().is_empty()
                || event.execution_context_ref.trim().is_empty()
                || event.run_session_ref.trim().is_empty()
                || event.adapter_ref.trim().is_empty()
                || event.occurred_at_or_sequence.trim().is_empty()
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingIdentity,
                    EvidenceFindingSeverity::Blocker,
                    format!("task event {} has incomplete identity", event.event_id),
                ));
            }
            if !event.confidence_class.is_bound() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingConfidence,
                    EvidenceFindingSeverity::Blocker,
                    format!("task event {} has unknown confidence", event.event_id),
                ));
            }
            validate_source_honesty(
                event.source_kind,
                event.confidence_class,
                event.fallback_active,
                event.raw_payload_ref.as_deref(),
                event.provider_ref.as_deref(),
                &event.event_id,
                &mut findings,
            );
        }

        for problem in &self.problems {
            if problem.problem_id.trim().is_empty()
                || problem.message.trim().is_empty()
                || problem.source_tool_ref.trim().is_empty()
                || !problem.location.is_bound()
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingIdentity,
                    EvidenceFindingSeverity::Blocker,
                    format!("problem {} has incomplete identity", problem.problem_id),
                ));
            }
            if !problem.confidence_class.is_bound() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingConfidence,
                    EvidenceFindingSeverity::Blocker,
                    format!("problem {} has unknown confidence", problem.problem_id),
                ));
            }
            if !problem.has_origin_ref() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::ProblemMissingOriginRef,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "problem {} lacks task-event or static-analysis origin",
                        problem.problem_id
                    ),
                ));
            }
            if problem.editor_decoration_ref.trim().is_empty()
                || problem.timeline_entry_ref.trim().is_empty()
                || problem
                    .rerun_action_ref
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::ProblemMissingProjectionRef,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "problem {} lacks editor, timeline, or rerun correlation",
                        problem.problem_id
                    ),
                ));
            }
            if problem.quick_fix_refs.is_empty() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::ProblemMissingQuickFixRefs,
                    EvidenceFindingSeverity::Warning,
                    format!("problem {} has no quick-fix refs", problem.problem_id),
                ));
            }
            if problem.source_kind.is_heuristic() && problem.raw_output_backlink.is_none() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::HeuristicMissingRawOutputBacklink,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "heuristic problem {} lacks raw-output backlink",
                        problem.problem_id
                    ),
                ));
            }
            if problem.source_kind.is_imported() {
                if problem.provider_evidence_backlink.is_none() {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::ImportedEvidenceMissingProviderBacklink,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "imported problem {} lacks provider evidence backlink",
                            problem.problem_id
                        ),
                    ));
                }
                if matches!(
                    problem.freshness_state,
                    EvidenceFreshnessState::CurrentLocal
                ) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::ImportedEvidenceFlattenedIntoLocalTruth,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "imported problem {} masquerades as current local state",
                            problem.problem_id
                        ),
                    ));
                }
            }
            if let Some(task_event_ref) = problem.task_event_ref.as_deref() {
                if !event_ids.contains(task_event_ref) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::EvidenceDanglingRef,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "problem {} references missing task event {}",
                            problem.problem_id, task_event_ref
                        ),
                    ));
                }
            }
            if let Some(output_channel_ref) = problem.output_channel_ref.as_deref() {
                if !channel_ids.contains(output_channel_ref) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::EvidenceDanglingRef,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "problem {} references missing output channel {}",
                            problem.problem_id, output_channel_ref
                        ),
                    ));
                }
            }
        }

        for channel in &self.output_channels {
            if channel.channel_id.trim().is_empty()
                || channel.source_subsystem.trim().is_empty()
                || channel.execution_context_ref.trim().is_empty()
                || channel.run_session_ref.trim().is_empty()
                || !channel.trust_state.is_bound()
                || !channel.retention_class.is_bound()
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::OutputChannelMissingStableDescriptor,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "output channel {} lacks stable descriptor fields",
                        channel.channel_id
                    ),
                ));
            }
            if channel.chunks.is_empty() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::OutputChannelMissingChunks,
                    EvidenceFindingSeverity::Blocker,
                    format!("output channel {} has no chunks", channel.channel_id),
                ));
            }
            if !channel.virtualization_safe {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::OutputChannelNotVirtualizationSafe,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "output channel {} is not virtualization-safe",
                        channel.channel_id
                    ),
                ));
            }
            if channel.heuristic_parser_active
                && (channel.parser_confidence.is_none() || channel.raw_output_backlink.is_none())
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::HeuristicMissingRawOutputBacklink,
                    EvidenceFindingSeverity::Blocker,
                    format!("heuristic output channel {} lacks parser confidence or raw-output backlink", channel.channel_id),
                ));
            }
            if channel.source_kind.is_imported() {
                if channel.imported_provider_ref.is_none() {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::ImportedEvidenceMissingProviderBacklink,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "imported output channel {} lacks provider ref",
                            channel.channel_id
                        ),
                    ));
                }
                if matches!(
                    channel.trust_state,
                    OutputTrustState::LocalRuntimeTruth | OutputTrustState::RemoteRuntimeTruth
                ) || matches!(
                    channel.freshness_state,
                    EvidenceFreshnessState::CurrentLocal
                ) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::ImportedEvidenceFlattenedIntoLocalTruth,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "imported output channel {} masquerades as runtime truth",
                            channel.channel_id
                        ),
                    ));
                }
            }
            for chunk in &channel.chunks {
                if chunk.chunk_id.trim().is_empty()
                    || chunk.repeat_count == 0
                    || !chunk.append_safe
                    || !chunk.searchable
                    || chunk.redaction_profile_ref.trim().is_empty()
                    || chunk.sequence_end < chunk.sequence_start
                {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::OutputChunkNotAppendFriendly,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "output chunk {} is not append-friendly/searchable",
                            chunk.chunk_id
                        ),
                    ));
                }
            }
        }

        for evidence in &self.evidence_objects {
            if evidence.evidence_id.trim().is_empty()
                || evidence.run_or_provider_ref.trim().is_empty()
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingIdentity,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "evidence object {} has incomplete identity",
                        evidence.evidence_id
                    ),
                ));
            }
            if !evidence.freshness_state.is_bound() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::EvidenceMissingFreshness,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "evidence object {} has unknown freshness",
                        evidence.evidence_id
                    ),
                ));
            }
            if evidence.reopen_refs.is_empty() {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::EvidenceMissingReopenLineage,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "evidence object {} has no reopen refs",
                        evidence.evidence_id
                    ),
                ));
            }
            if evidence.source_kind.is_imported()
                && matches!(
                    evidence.trust_state,
                    OutputTrustState::LocalRuntimeTruth | OutputTrustState::RemoteRuntimeTruth
                )
            {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::ImportedEvidenceFlattenedIntoLocalTruth,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "imported evidence object {} masquerades as runtime truth",
                        evidence.evidence_id
                    ),
                ));
            }
            for event_ref in &evidence.task_event_refs {
                if !event_ids.contains(event_ref.as_str()) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::EvidenceDanglingRef,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "evidence object {} references missing task event {}",
                            evidence.evidence_id, event_ref
                        ),
                    ));
                }
            }
            for problem_ref in &evidence.problem_refs {
                if !problem_ids.contains(problem_ref.as_str()) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::EvidenceDanglingRef,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "evidence object {} references missing problem {}",
                            evidence.evidence_id, problem_ref
                        ),
                    ));
                }
            }
            for output_chunk_ref in &evidence.output_chunk_refs {
                if !chunk_ids.contains(output_chunk_ref.as_str()) {
                    findings.push(EvidenceValidationFinding::new(
                        EvidenceFindingKind::EvidenceDanglingRef,
                        EvidenceFindingSeverity::Blocker,
                        format!(
                            "evidence object {} references missing output chunk {}",
                            evidence.evidence_id, output_chunk_ref
                        ),
                    ));
                }
            }
        }

        for required_surface in EvidenceConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::MissingConsumerProjection,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "bundle {} is missing a preserved {} projection",
                        self.bundle_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.bundle_id) {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::ConsumerProjectionDrift,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve execution evidence truth",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(EvidenceValidationFinding::new(
                    EvidenceFindingKind::PromotionStateMismatch,
                    EvidenceFindingSeverity::Blocker,
                    format!(
                        "stored promotion state {} does not match derived {}",
                        self.promotion_state.as_str(),
                        expected.as_str()
                    ),
                ));
            }
        }

        findings
    }
}

/// Support-export wrapper carrying the exact evidence bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEvidenceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Bundle id ref.
    pub bundle_id_ref: String,
    /// Redaction profile ref.
    pub redaction_profile_ref: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact bundle exported.
    pub bundle: ExecutionEvidenceBundle,
}

impl ExecutionEvidenceSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == EXECUTION_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.bundle_id_ref == self.bundle.bundle_id
            && self.raw_private_material_excluded
            && self.bundle.raw_private_material_excluded
            && self.bundle.is_stable()
    }
}

/// Builds the current stable execution-evidence bundle input.
pub fn current_stable_execution_evidence_bundle_input() -> ExecutionEvidenceBundleInput {
    let bundle_id =
        "bundle:m4:stabilize_problem_records_output_channels_and_execution_evidence:stable";
    ExecutionEvidenceBundleInput {
        bundle_id: bundle_id.to_owned(),
        generated_at: "2026-06-06T12:00:00Z".to_owned(),
        execution_context_ref: "exec-context:local:workspace:test".to_owned(),
        run_session_ref: "run-session:local:test:1".to_owned(),
        task_events: vec![
            StableTaskEventEnvelope {
                event_id: "task-event:local:test:start".to_owned(),
                event_kind: EvidenceTaskEventKind::TaskStarted,
                execution_context_ref: "exec-context:local:workspace:test".to_owned(),
                run_session_ref: "run-session:local:test:1".to_owned(),
                source_kind: ExecutionSourceKind::Native,
                adapter_ref: "adapter:pytest:native".to_owned(),
                provider_ref: None,
                occurred_at_or_sequence: "2026-06-06T12:00:01Z".to_owned(),
                parent_event_refs: Vec::new(),
                correlation_refs: vec!["trace:local:test:1".to_owned()],
                confidence_class: EvidenceConfidenceClass::High,
                fallback_active: false,
                raw_payload_ref: Some("raw:event:local:test:start".to_owned()),
            },
            StableTaskEventEnvelope {
                event_id: "task-event:provider:ci:annotation".to_owned(),
                event_kind: EvidenceTaskEventKind::DiagnosticEmitted,
                execution_context_ref: "exec-context:provider:ci:import".to_owned(),
                run_session_ref: "provider-run:ci:123".to_owned(),
                source_kind: ExecutionSourceKind::ImportedProvider,
                adapter_ref: "adapter:github-actions:annotations".to_owned(),
                provider_ref: Some("provider:github-actions:repo".to_owned()),
                occurred_at_or_sequence: "provider-seq:987".to_owned(),
                parent_event_refs: Vec::new(),
                correlation_refs: vec!["trace:provider:ci:123".to_owned()],
                confidence_class: EvidenceConfidenceClass::MediumHigh,
                fallback_active: false,
                raw_payload_ref: Some("raw:provider:ci:annotation:987".to_owned()),
            },
        ],
        problems: vec![
            StableProblemRecord {
                problem_id: "problem:local:test:assertion".to_owned(),
                severity: ProblemSeverity::Error,
                message: "assertion failed in checkout total calculation".to_owned(),
                location: ProblemLocation {
                    uri: "workspace://tests/checkout_test.rs".to_owned(),
                    start_line: Some(42),
                    start_column: Some(9),
                    end_line: Some(42),
                    end_column: Some(42),
                    structured_location_ref: None,
                },
                source_tool_ref: "tool:pytest".to_owned(),
                source_kind: ExecutionSourceKind::HeuristicParser,
                task_event_ref: Some("task-event:local:test:start".to_owned()),
                static_analysis_ref: None,
                confidence_class: EvidenceConfidenceClass::HeuristicBestEffort,
                quick_fix_refs: vec!["quick-fix:open-test".to_owned()],
                raw_output_backlink: Some("output-chunk:local:test:stderr:1".to_owned()),
                provider_evidence_backlink: None,
                output_channel_ref: Some("output-channel:local:test:stderr".to_owned()),
                freshness_state: EvidenceFreshnessState::CurrentLocal,
                qualifiers: vec![EvidenceQualifier::TestOnly],
                editor_decoration_ref: "editor-decoration:problem:local:test:assertion".to_owned(),
                timeline_entry_ref: "timeline:run-session:local:test:1:problem".to_owned(),
                rerun_action_ref: Some("rerun:run-session:local:test:1".to_owned()),
            },
            StableProblemRecord {
                problem_id: "problem:provider:lint:annotation".to_owned(),
                severity: ProblemSeverity::Warning,
                message: "provider lint annotation mapped to current branch".to_owned(),
                location: ProblemLocation {
                    uri: "workspace://src/lib.rs".to_owned(),
                    start_line: Some(12),
                    start_column: Some(1),
                    end_line: Some(12),
                    end_column: Some(32),
                    structured_location_ref: None,
                },
                source_tool_ref: "tool:provider-lint".to_owned(),
                source_kind: ExecutionSourceKind::ImportedProvider,
                task_event_ref: Some("task-event:provider:ci:annotation".to_owned()),
                static_analysis_ref: None,
                confidence_class: EvidenceConfidenceClass::MediumHigh,
                quick_fix_refs: vec!["quick-fix:open-provider-annotation".to_owned()],
                raw_output_backlink: None,
                provider_evidence_backlink: Some("provider-annotation:github:987".to_owned()),
                output_channel_ref: Some("output-channel:provider:ci:lint".to_owned()),
                freshness_state: EvidenceFreshnessState::ImportedProvider,
                qualifiers: vec![EvidenceQualifier::External, EvidenceQualifier::ScopeLimited],
                editor_decoration_ref: "editor-decoration:problem:provider:lint".to_owned(),
                timeline_entry_ref: "timeline:provider:ci:123:annotation".to_owned(),
                rerun_action_ref: Some("provider-open:ci:123".to_owned()),
            },
        ],
        output_channels: vec![
            StableOutputChannelDescriptor {
                channel_id: "output-channel:local:test:stderr".to_owned(),
                canonical_name: CanonicalOutputChannelName::Test,
                source_subsystem: "test_runner".to_owned(),
                execution_context_ref: "exec-context:local:workspace:test".to_owned(),
                run_session_ref: "run-session:local:test:1".to_owned(),
                source_kind: ExecutionSourceKind::HeuristicParser,
                trust_state: OutputTrustState::HeuristicFallback,
                retention_class: OutputRetentionClass::PinnedForSupport,
                freshness_state: EvidenceFreshnessState::CurrentLocal,
                virtualization_safe: true,
                heuristic_parser_active: true,
                parser_confidence: Some(EvidenceConfidenceClass::HeuristicBestEffort),
                raw_output_backlink: Some("raw-output:local:test:stderr".to_owned()),
                imported_provider_ref: None,
                chunks: vec![StableOutputChunk {
                    chunk_id: "output-chunk:local:test:stderr:1".to_owned(),
                    sequence_start: 1,
                    sequence_end: 24,
                    byte_start: Some(0),
                    byte_end: Some(2048),
                    timestamp_or_sequence: "2026-06-06T12:00:02Z".to_owned(),
                    render_class: OutputChunkRenderClass::PlainText,
                    repeat_count: 1,
                    link_refs: vec!["link:workspace://tests/checkout_test.rs:42".to_owned()],
                    redaction_profile_ref: "redaction-profile:metadata-safe-default".to_owned(),
                    append_safe: true,
                    searchable: true,
                }],
            },
            StableOutputChannelDescriptor {
                channel_id: "output-channel:provider:ci:lint".to_owned(),
                canonical_name: CanonicalOutputChannelName::ProviderImported,
                source_subsystem: "provider_ci".to_owned(),
                execution_context_ref: "exec-context:provider:ci:import".to_owned(),
                run_session_ref: "provider-run:ci:123".to_owned(),
                source_kind: ExecutionSourceKind::ImportedProvider,
                trust_state: OutputTrustState::ImportedProviderTruth,
                retention_class: OutputRetentionClass::MetadataOnly,
                freshness_state: EvidenceFreshnessState::ImportedProvider,
                virtualization_safe: true,
                heuristic_parser_active: false,
                parser_confidence: None,
                raw_output_backlink: None,
                imported_provider_ref: Some("provider:github-actions:repo".to_owned()),
                chunks: vec![StableOutputChunk {
                    chunk_id: "output-chunk:provider:ci:lint:1".to_owned(),
                    sequence_start: 1,
                    sequence_end: 3,
                    byte_start: None,
                    byte_end: None,
                    timestamp_or_sequence: "provider-seq:987".to_owned(),
                    render_class: OutputChunkRenderClass::Diagnostic,
                    repeat_count: 1,
                    link_refs: vec!["provider-url:annotation:987".to_owned()],
                    redaction_profile_ref: "redaction-profile:metadata-safe-default".to_owned(),
                    append_safe: true,
                    searchable: true,
                }],
            },
        ],
        evidence_objects: vec![
            StableExecutionEvidenceObject {
                evidence_id: "evidence:problem:local:test:assertion".to_owned(),
                evidence_kind: ExecutionEvidenceKind::Problem,
                run_or_provider_ref: "run-session:local:test:1".to_owned(),
                source_kind: ExecutionSourceKind::HeuristicParser,
                artifact_refs: vec!["artifact:test-report:local:1".to_owned()],
                mapping_refs: vec!["mapping:workspace:test-current".to_owned()],
                freshness_state: EvidenceFreshnessState::CurrentLocal,
                baseline_or_comparison_ref: None,
                problem_refs: vec!["problem:local:test:assertion".to_owned()],
                output_chunk_refs: vec!["output-chunk:local:test:stderr:1".to_owned()],
                task_event_refs: vec!["task-event:local:test:start".to_owned()],
                qualifiers: vec![EvidenceQualifier::TestOnly],
                reopen_refs: vec![
                    "reopen:run-session:local:test:1".to_owned(),
                    "reopen:output-channel:local:test:stderr".to_owned(),
                ],
                trust_state: OutputTrustState::HeuristicFallback,
            },
            StableExecutionEvidenceObject {
                evidence_id: "evidence:provider:lint:annotation".to_owned(),
                evidence_kind: ExecutionEvidenceKind::ReviewAnnotation,
                run_or_provider_ref: "provider-run:ci:123".to_owned(),
                source_kind: ExecutionSourceKind::ImportedProvider,
                artifact_refs: vec!["provider-artifact:lint-log:123".to_owned()],
                mapping_refs: vec!["mapping:provider:annotation-to-workspace".to_owned()],
                freshness_state: EvidenceFreshnessState::ImportedProvider,
                baseline_or_comparison_ref: Some(
                    "comparison:provider-head-to-local-head".to_owned(),
                ),
                problem_refs: vec!["problem:provider:lint:annotation".to_owned()],
                output_chunk_refs: vec!["output-chunk:provider:ci:lint:1".to_owned()],
                task_event_refs: vec!["task-event:provider:ci:annotation".to_owned()],
                qualifiers: vec![EvidenceQualifier::External, EvidenceQualifier::ScopeLimited],
                reopen_refs: vec![
                    "reopen:provider-run:ci:123".to_owned(),
                    "reopen:provider-annotation:github:987".to_owned(),
                ],
                trust_state: OutputTrustState::ImportedProviderTruth,
            },
        ],
        artifact_refs: vec![
            "artifact:test-report:local:1".to_owned(),
            "provider-artifact:lint-log:123".to_owned(),
        ],
        mapping_refs: vec![
            "mapping:workspace:test-current".to_owned(),
            "mapping:provider:annotation-to-workspace".to_owned(),
        ],
        redaction_profile_ref: "redaction-profile:metadata-safe-default".to_owned(),
        reopen_lineage_refs: vec![
            "reopen:run-session:local:test:1".to_owned(),
            "reopen:output-channel:local:test:stderr".to_owned(),
            "reopen:provider-run:ci:123".to_owned(),
        ],
        source_contract_refs: vec![
            EXECUTION_EVIDENCE_BUNDLE_DOC_REF.to_owned(),
            EXECUTION_EVIDENCE_BUNDLE_SCHEMA_REF.to_owned(),
        ],
        consumer_projections: EvidenceConsumerSurface::REQUIRED
            .into_iter()
            .map(|surface| EvidenceConsumerProjection {
                consumer_surface: surface,
                projection_ref: format!("projection:{}:stable", surface.as_str()),
                bundle_id_ref: bundle_id.to_owned(),
                preserves_object_refs: true,
                preserves_source_kind: true,
                preserves_confidence: true,
                preserves_freshness: true,
                preserves_reopen_lineage: true,
                supports_json_export: true,
                raw_private_material_excluded: true,
            })
            .collect(),
        raw_private_material_excluded: true,
    }
}

/// Builds the current stable execution-evidence bundle.
pub fn current_stable_execution_evidence_bundle() -> ExecutionEvidenceBundle {
    ExecutionEvidenceBundle::materialize(current_stable_execution_evidence_bundle_input())
}

fn validate_source_honesty(
    source_kind: ExecutionSourceKind,
    confidence_class: EvidenceConfidenceClass,
    fallback_active: bool,
    raw_payload_ref: Option<&str>,
    provider_ref: Option<&str>,
    object_id: &str,
    findings: &mut Vec<EvidenceValidationFinding>,
) {
    if source_kind.is_heuristic() {
        if !fallback_active && raw_payload_ref.is_none() {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::HeuristicMissingRawOutputBacklink,
                EvidenceFindingSeverity::Blocker,
                format!("heuristic object {object_id} lacks fallback/raw payload linkage"),
            ));
        }
        if matches!(
            confidence_class,
            EvidenceConfidenceClass::High | EvidenceConfidenceClass::MediumHigh
        ) {
            findings.push(EvidenceValidationFinding::new(
                EvidenceFindingKind::HeuristicOverclaimsConfidence,
                EvidenceFindingSeverity::Blocker,
                format!("heuristic object {object_id} overclaims confidence"),
            ));
        }
    }
    if source_kind.is_imported() && provider_ref.map(str::trim).unwrap_or("").is_empty() {
        findings.push(EvidenceValidationFinding::new(
            EvidenceFindingKind::ImportedEvidenceMissingProviderBacklink,
            EvidenceFindingSeverity::Blocker,
            format!("imported object {object_id} lacks provider ref"),
        ));
    }
}

fn promotion_state_for_findings(findings: &[EvidenceValidationFinding]) -> EvidencePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == EvidenceFindingSeverity::Blocker)
    {
        EvidencePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == EvidenceFindingSeverity::Warning)
    {
        EvidencePromotionState::NarrowedBelowStable
    } else {
        EvidencePromotionState::Stable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_bundle_materializes() {
        let bundle = current_stable_execution_evidence_bundle();
        assert_eq!(bundle.promotion_state, EvidencePromotionState::Stable);
        assert!(bundle.validation_findings.is_empty());
        assert!(bundle.is_stable());
        assert!(bundle
            .support_export("support:execution-evidence:stable", "2026-06-06T12:01:00Z")
            .is_export_safe());
    }

    #[test]
    fn heuristic_problem_without_raw_backlink_blocks() {
        let mut input = current_stable_execution_evidence_bundle_input();
        input.problems[0].raw_output_backlink = None;
        let bundle = ExecutionEvidenceBundle::materialize(input);
        assert_eq!(bundle.promotion_state, EvidencePromotionState::BlocksStable);
        assert!(bundle.validation_findings.iter().any(|finding| {
            finding.finding_kind == EvidenceFindingKind::HeuristicMissingRawOutputBacklink
        }));
    }

    #[test]
    fn imported_output_as_local_truth_blocks() {
        let mut input = current_stable_execution_evidence_bundle_input();
        input.output_channels[1].trust_state = OutputTrustState::LocalRuntimeTruth;
        let bundle = ExecutionEvidenceBundle::materialize(input);
        assert_eq!(bundle.promotion_state, EvidencePromotionState::BlocksStable);
        assert!(bundle.validation_findings.iter().any(|finding| {
            finding.finding_kind == EvidenceFindingKind::ImportedEvidenceFlattenedIntoLocalTruth
        }));
    }

    #[test]
    fn missing_consumer_projection_blocks() {
        let mut input = current_stable_execution_evidence_bundle_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != EvidenceConsumerSurface::ReleasePacket
        });
        let bundle = ExecutionEvidenceBundle::materialize(input);
        assert_eq!(bundle.promotion_state, EvidencePromotionState::BlocksStable);
        assert!(bundle
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == EvidenceFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn dangling_evidence_ref_blocks() {
        let mut input = current_stable_execution_evidence_bundle_input();
        input.evidence_objects[0].output_chunk_refs = vec!["output-chunk:missing".to_owned()];
        let bundle = ExecutionEvidenceBundle::materialize(input);
        assert_eq!(bundle.promotion_state, EvidencePromotionState::BlocksStable);
        assert!(bundle
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == EvidenceFindingKind::EvidenceDanglingRef));
    }
}
