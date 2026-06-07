//! Stable build/test event interoperability packet and conformance checks.
//!
//! This module freezes the runtime contract that joins native adapters, BSP,
//! Bazel BEP/BES, structured imports, and heuristic parser fallbacks into one
//! replayable build/test event envelope. The packet is intentionally metadata
//! oriented: raw payload bodies stay behind retained refs with redaction and
//! replay posture, while every consumer projection keeps `source_kind`,
//! `confidence`, raw-payload refs, and provenance visible.
//!
//! The reviewer-facing contract lives at
//! [`/docs/runtime/m4/build-test-event-interoperability.md`](../../../docs/runtime/m4/build-test-event-interoperability.md)
//! and the machine-readable boundary schema lives at
//! [`/schemas/runtime/build-test-event-envelope.schema.json`](../../../schemas/runtime/build-test-event-envelope.schema.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`BuildTestEventInteroperabilityPacket`].
pub const BUILD_TEST_EVENT_INTEROPERABILITY_RECORD_KIND: &str =
    "build_test_event_interoperability_packet";

/// Stable record-kind tag for [`BuildTestEventInteroperabilitySupportExport`].
pub const BUILD_TEST_EVENT_INTEROPERABILITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "build_test_event_interoperability_support_export";

/// Integer schema version for the build/test event interoperability packet.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_REF: &str =
    "schemas/runtime/build-test-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_DOC_REF: &str =
    "docs/runtime/m4/build-test-event-interoperability.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/build-test-event-interoperability.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/build-test-event-interoperability";

/// Repo-relative path of the checked-in stable packet.
pub const BUILD_TEST_EVENT_INTEROPERABILITY_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/build_test_event_interoperability_packet.json";

/// Source class for normalized build/test events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTestEventSourceKind {
    /// First-party Aureline runtime or adapter event.
    Native,
    /// Build Server Protocol event.
    Bsp,
    /// Bazel Build Event Protocol or Build Event Service event.
    BazelBep,
    /// Structured imported output such as JUnit, SARIF, JSON, or coverage.
    StructuredOutput,
    /// Fallback parser over unstructured output.
    HeuristicParser,
}

impl BuildTestEventSourceKind {
    /// Every required source kind in stable declaration order.
    pub const ALL: [Self; 5] = [
        Self::Native,
        Self::Bsp,
        Self::BazelBep,
        Self::StructuredOutput,
        Self::HeuristicParser,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bsp => "bsp",
            Self::BazelBep => "bazel-bep",
            Self::StructuredOutput => "structured-output",
            Self::HeuristicParser => "heuristic-parser",
        }
    }

    /// Returns true when the source is a fallback parser.
    pub const fn is_heuristic(self) -> bool {
        matches!(self, Self::HeuristicParser)
    }
}

/// Confidence level on a normalized build/test event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTestEventConfidence {
    /// High-confidence native or negotiated protocol truth.
    High,
    /// Structured truth with a bounded translation layer.
    MediumHigh,
    /// Useful partial structured or imported truth.
    Medium,
    /// Low-confidence fallback or partial heuristic truth.
    Low,
}

impl BuildTestEventConfidence {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::MediumHigh => "medium-high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    const fn overclaims_for(self, source_kind: BuildTestEventSourceKind) -> bool {
        matches!(source_kind, BuildTestEventSourceKind::HeuristicParser)
            && !matches!(self, Self::Low)
    }
}

/// Canonical lifecycle event kind shared by build, task, test, and debug lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BuildTestEventKind {
    /// Work was accepted and queued.
    TaskQueued,
    /// The target graph for the run became available.
    TargetGraphReady,
    /// Work started.
    TaskStarted,
    /// Progress changed.
    ProgressUpdated,
    /// A diagnostic or problem was emitted.
    DiagnosticEmitted,
    /// A test case started.
    TestCaseStarted,
    /// A test case finished.
    TestCaseFinished,
    /// An artifact was published.
    ArtifactPublished,
    /// Work finished.
    TaskFinished,
}

impl BuildTestEventKind {
    /// Every required lifecycle kind in canonical order.
    pub const CANONICAL_LIFECYCLE: [Self; 9] = [
        Self::TaskQueued,
        Self::TargetGraphReady,
        Self::TaskStarted,
        Self::ProgressUpdated,
        Self::DiagnosticEmitted,
        Self::TestCaseStarted,
        Self::TestCaseFinished,
        Self::ArtifactPublished,
        Self::TaskFinished,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskQueued => "TaskQueued",
            Self::TargetGraphReady => "TargetGraphReady",
            Self::TaskStarted => "TaskStarted",
            Self::ProgressUpdated => "ProgressUpdated",
            Self::DiagnosticEmitted => "DiagnosticEmitted",
            Self::TestCaseStarted => "TestCaseStarted",
            Self::TestCaseFinished => "TestCaseFinished",
            Self::ArtifactPublished => "ArtifactPublished",
            Self::TaskFinished => "TaskFinished",
        }
    }
}

/// Payload family carried by a canonical build/test event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTestPayloadKind {
    /// Lifecycle payload.
    Lifecycle,
    /// Progress payload.
    Progress,
    /// Diagnostic payload.
    Diagnostic,
    /// Test payload.
    Test,
    /// Artifact payload.
    Artifact,
    /// Debug payload.
    Debug,
}

impl BuildTestPayloadKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Progress => "progress",
            Self::Diagnostic => "diagnostic",
            Self::Test => "test",
            Self::Artifact => "artifact",
            Self::Debug => "debug",
        }
    }
}

/// Runtime lane that claims stable build/test event interoperability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTestInteropLane {
    /// Local runtime lane.
    LocalRuntime,
    /// Remote/helper runtime lane.
    RemoteHelper,
    /// Imported provider or import replay lane.
    ImportedProvider,
}

impl BuildTestInteropLane {
    /// Every claimed stable lane.
    pub const REQUIRED: [Self; 3] = [
        Self::LocalRuntime,
        Self::RemoteHelper,
        Self::ImportedProvider,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRuntime => "local_runtime",
            Self::RemoteHelper => "remote_helper",
            Self::ImportedProvider => "imported_provider",
        }
    }
}

/// Negotiated adapter capability state for a claimed source/lane pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCapabilityState {
    /// Capability was negotiated and is usable.
    Negotiated,
    /// Capability is available in metadata but degraded.
    Degraded,
    /// Capability is not supported by this source and lane.
    Unsupported,
}

impl AdapterCapabilityState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Negotiated => "negotiated",
            Self::Degraded => "degraded",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Redaction and retention class for a raw payload reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawPayloadRetentionClass {
    /// Metadata and digest only.
    MetadataDigestOnly,
    /// Redacted payload is retained by reference.
    RedactedReference,
    /// Support-only payload exists behind an approval gate.
    SupportApprovalRequired,
}

impl RawPayloadRetentionClass {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataDigestOnly => "metadata_digest_only",
            Self::RedactedReference => "redacted_reference",
            Self::SupportApprovalRequired => "support_approval_required",
        }
    }
}

/// Consumer surface that must preserve source kind and confidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTestConsumerSurface {
    /// Problems panel and problem records.
    Problems,
    /// Output channels and output headers.
    Output,
    /// Task timeline and task headers.
    TaskTimeline,
    /// Test explorer and test result headers.
    TestExplorer,
    /// Debug headers and chronology views.
    Debug,
    /// AI explanations and evidence callouts.
    AiExplanation,
    /// Review packets.
    ReviewPacket,
    /// Release packet.
    ReleasePacket,
    /// Support export.
    SupportExport,
}

impl BuildTestConsumerSurface {
    /// Every required consumer surface.
    pub const REQUIRED: [Self; 9] = [
        Self::Problems,
        Self::Output,
        Self::TaskTimeline,
        Self::TestExplorer,
        Self::Debug,
        Self::AiExplanation,
        Self::ReviewPacket,
        Self::ReleasePacket,
        Self::SupportExport,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Problems => "problems",
            Self::Output => "output",
            Self::TaskTimeline => "task_timeline",
            Self::TestExplorer => "test_explorer",
            Self::Debug => "debug",
            Self::AiExplanation => "ai_explanation",
            Self::ReviewPacket => "review_packet",
            Self::ReleasePacket => "release_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Promotion state derived from interoperability validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTestInteropPromotionState {
    /// Packet certifies stable interoperability.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl BuildTestInteropPromotionState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation severity for interoperability findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTestInteropFindingSeverity {
    /// Informational finding.
    Info,
    /// Warning finding.
    Warning,
    /// Blocker finding.
    Blocker,
}

/// Closed validation finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTestInteropFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required source kind is absent from the packet.
    MissingSourceKind,
    /// Required lifecycle event is absent from the packet.
    MissingLifecycleEvent,
    /// Required capability negotiation row is absent.
    MissingCapabilityNegotiation,
    /// Raw payload reference is missing or unretained.
    RawPayloadRefMissing,
    /// Raw payload reference is not safe for replay.
    RawPayloadNotReplaySafe,
    /// Heuristic parser overclaims confidence.
    HeuristicOverclaimsConfidence,
    /// Heuristic parser is not visibly downgraded.
    HeuristicNotDowngraded,
    /// Consumer projection drops source kind, confidence, raw ref, or provenance.
    ConsumerProjectionDrift,
    /// Required consumer projection is absent.
    MissingConsumerProjection,
    /// Support or release export would expose raw private material.
    ExportRedactionUnsafe,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl BuildTestInteropFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceKind => "missing_source_kind",
            Self::MissingLifecycleEvent => "missing_lifecycle_event",
            Self::MissingCapabilityNegotiation => "missing_capability_negotiation",
            Self::RawPayloadRefMissing => "raw_payload_ref_missing",
            Self::RawPayloadNotReplaySafe => "raw_payload_not_replay_safe",
            Self::HeuristicOverclaimsConfidence => "heuristic_overclaims_confidence",
            Self::HeuristicNotDowngraded => "heuristic_not_downgraded",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ExportRedactionUnsafe => "export_redaction_unsafe",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Provenance carried by every normalized event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestEventProvenance {
    /// Producer tool name.
    pub build_tool_name: String,
    /// Producer tool version, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_tool_version: Option<String>,
    /// Adapter id.
    pub adapter_id: String,
    /// Adapter version.
    pub adapter_version: String,
    /// Workspace revision, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
}

impl BuildTestEventProvenance {
    fn is_bound(&self) -> bool {
        !self.build_tool_name.trim().is_empty()
            && !self.adapter_id.trim().is_empty()
            && !self.adapter_version.trim().is_empty()
    }
}

/// Retained raw payload reference and export posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawPayloadReference {
    /// Opaque raw payload ref.
    pub raw_payload_ref: String,
    /// Source kind that produced the payload.
    pub source_kind: BuildTestEventSourceKind,
    /// Retention class.
    pub retention_class: RawPayloadRetentionClass,
    /// Digest of the source payload.
    pub payload_digest: String,
    /// True when replay suites may resolve this reference.
    pub replay_safe: bool,
    /// True when support exports may include this reference.
    pub support_export_safe: bool,
    /// True when release corpus validation may include this reference.
    pub release_corpus_safe: bool,
    /// True when AI evidence may cite this reference.
    pub ai_evidence_safe: bool,
    /// True when raw private material is excluded from the exported packet.
    pub raw_private_material_excluded: bool,
}

impl RawPayloadReference {
    fn is_bound(&self) -> bool {
        !self.raw_payload_ref.trim().is_empty() && !self.payload_digest.trim().is_empty()
    }
}

/// Adapter capability negotiation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterCapabilityNegotiation {
    /// Claimed stable lane.
    pub lane: BuildTestInteropLane,
    /// Source kind this row covers.
    pub source_kind: BuildTestEventSourceKind,
    /// Adapter id.
    pub adapter_id: String,
    /// Capability token.
    pub capability: String,
    /// Negotiated state.
    pub state: AdapterCapabilityState,
    /// Raw handshake or capability-packet ref.
    pub capability_packet_ref: String,
    /// Human-readable downgrade reason ref when state is degraded or unsupported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason_ref: Option<String>,
}

impl AdapterCapabilityNegotiation {
    fn is_bound(&self) -> bool {
        !self.adapter_id.trim().is_empty()
            && !self.capability.trim().is_empty()
            && !self.capability_packet_ref.trim().is_empty()
    }
}

/// Canonical build/test event envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestEventEnvelope {
    /// Unique stable identifier within the task session.
    pub event_id: String,
    /// Workspace or workset identity.
    pub workspace_id: String,
    /// Build target, task, test suite, or debug-configuration identity.
    pub target_id: String,
    /// Source kind.
    pub source_kind: BuildTestEventSourceKind,
    /// Confidence.
    pub confidence: BuildTestEventConfidence,
    /// Event time in the producing execution context.
    pub timestamp: String,
    /// Resolved environment/toolchain/runtime context.
    pub execution_context_id: String,
    /// Payload family.
    pub payload_kind: BuildTestPayloadKind,
    /// Pointer to the original adapter payload for support and replay.
    pub raw_payload_ref: String,
    /// Producer provenance.
    pub provenance: BuildTestEventProvenance,
    /// Canonical lifecycle kind.
    pub event_kind: BuildTestEventKind,
    /// True when this event is visibly downgraded on every consumer surface.
    pub downgraded: bool,
    /// Stable lane that emitted or imported the event.
    pub lane: BuildTestInteropLane,
    /// Optional parent event refs.
    #[serde(default)]
    pub parent_event_refs: Vec<String>,
    /// Optional consumer/evidence correlation refs.
    #[serde(default)]
    pub correlation_refs: Vec<String>,
}

impl BuildTestEventEnvelope {
    fn is_bound(&self) -> bool {
        !self.event_id.trim().is_empty()
            && !self.workspace_id.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && !self.timestamp.trim().is_empty()
            && !self.execution_context_id.trim().is_empty()
            && !self.raw_payload_ref.trim().is_empty()
            && self.provenance.is_bound()
    }
}

/// Consumer projection proving source and confidence survive a surface hop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestConsumerProjection {
    /// Consumer surface.
    pub consumer_surface: BuildTestConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// True when event ids remain stable.
    pub preserves_event_id: bool,
    /// True when source kind remains visible.
    pub preserves_source_kind: bool,
    /// True when confidence remains visible.
    pub preserves_confidence: bool,
    /// True when raw payload refs remain inspectable.
    pub preserves_raw_payload_ref: bool,
    /// True when provenance remains inspectable.
    pub preserves_provenance: bool,
    /// True when heuristic/imported rows stay visibly downgraded.
    pub preserves_downgrade_disclosure: bool,
    /// True when exported JSON excludes raw private material.
    pub raw_private_material_excluded: bool,
}

impl BuildTestConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && !self.projection_ref.trim().is_empty()
            && self.preserves_event_id
            && self.preserves_source_kind
            && self.preserves_confidence
            && self.preserves_raw_payload_ref
            && self.preserves_provenance
            && self.preserves_downgrade_disclosure
            && self.raw_private_material_excluded
    }
}

/// Replay/export parity block for the event packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayExportParity {
    /// Replay packet ref.
    pub replay_packet_ref: String,
    /// Support export ref.
    pub support_export_ref: String,
    /// Release corpus ref.
    pub release_corpus_ref: String,
    /// AI evidence packet ref.
    pub ai_evidence_packet_ref: String,
    /// True when replay reads event envelopes rather than localized output.
    pub replay_uses_canonical_envelopes: bool,
    /// True when support export carries raw payload refs rather than bodies.
    pub support_export_retains_raw_payload_refs: bool,
    /// True when release corpus validates adapter drift and payload loss.
    pub release_corpus_validates_drift: bool,
    /// True when private raw material is excluded from exported packets.
    pub raw_private_material_excluded: bool,
}

impl ReplayExportParity {
    fn preserves_truth(&self) -> bool {
        !self.replay_packet_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
            && !self.release_corpus_ref.trim().is_empty()
            && !self.ai_evidence_packet_ref.trim().is_empty()
            && self.replay_uses_canonical_envelopes
            && self.support_export_retains_raw_payload_refs
            && self.release_corpus_validates_drift
            && self.raw_private_material_excluded
    }
}

/// One validation finding emitted by the interoperability validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestInteropValidationFinding {
    /// Closed finding kind.
    pub finding_kind: BuildTestInteropFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl BuildTestInteropValidationFinding {
    fn new(
        finding_kind: BuildTestInteropFindingKind,
        severity: BuildTestInteropFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`BuildTestEventInteroperabilityPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestEventInteroperabilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Canonical event envelopes.
    #[serde(default)]
    pub events: Vec<BuildTestEventEnvelope>,
    /// Retained raw payload refs.
    #[serde(default)]
    pub raw_payload_refs: Vec<RawPayloadReference>,
    /// Adapter capability negotiation rows.
    #[serde(default)]
    pub capability_negotiations: Vec<AdapterCapabilityNegotiation>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<BuildTestConsumerProjection>,
    /// Replay/export parity block.
    pub replay_export_parity: ReplayExportParity,
}

/// Stable interoperability packet tying canonical events to consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestEventInteroperabilityPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Canonical event envelopes.
    #[serde(default)]
    pub events: Vec<BuildTestEventEnvelope>,
    /// Retained raw payload refs.
    #[serde(default)]
    pub raw_payload_refs: Vec<RawPayloadReference>,
    /// Adapter capability negotiation rows.
    #[serde(default)]
    pub capability_negotiations: Vec<AdapterCapabilityNegotiation>,
    /// Consumer projections.
    #[serde(default)]
    pub consumer_projections: Vec<BuildTestConsumerProjection>,
    /// Replay/export parity block.
    pub replay_export_parity: ReplayExportParity,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<BuildTestInteropValidationFinding>,
}

impl BuildTestEventInteroperabilityPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: BuildTestEventInteroperabilityPacketInput) -> Self {
        let mut packet = Self {
            record_kind: BUILD_TEST_EVENT_INTEROPERABILITY_RECORD_KIND.to_owned(),
            schema_version: BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            events: input.events,
            raw_payload_refs: input.raw_payload_refs,
            capability_negotiations: input.capability_negotiations,
            consumer_projections: input.consumer_projections,
            replay_export_parity: input.replay_export_parity,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the stable interoperability invariants.
    pub fn validate(&self) -> Vec<BuildTestInteropValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Builds an export-safe support packet that carries the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> BuildTestEventInteroperabilitySupportExport {
        BuildTestEventInteroperabilitySupportExport {
            record_kind: BUILD_TEST_EVENT_INTEROPERABILITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            raw_private_material_excluded: self.replay_export_parity.raw_private_material_excluded,
            packet: self.clone(),
        }
    }

    /// Returns the unique source-kind tokens present in event envelopes.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.events {
            set.insert(event.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the unique canonical lifecycle tokens present in event envelopes.
    pub fn event_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.events {
            set.insert(event.event_kind);
        }
        set.into_iter().map(BuildTestEventKind::as_str).collect()
    }

    /// Returns the consumer-surface tokens present in projection rows.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for projection in &self.consumer_projections {
            set.insert(projection.consumer_surface);
        }
        set.into_iter()
            .map(BuildTestConsumerSurface::as_str)
            .collect()
    }

    fn has_projection_for(&self, surface: BuildTestConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<BuildTestInteropValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != BUILD_TEST_EVENT_INTEROPERABILITY_RECORD_KIND
        {
            findings.push(BuildTestInteropValidationFinding::new(
                BuildTestInteropFindingKind::WrongRecordKind,
                BuildTestInteropFindingSeverity::Blocker,
                "build/test event interoperability packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION
        {
            findings.push(BuildTestInteropValidationFinding::new(
                BuildTestInteropFindingKind::WrongSchemaVersion,
                BuildTestInteropFindingSeverity::Blocker,
                "build/test event interoperability packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(BuildTestInteropValidationFinding::new(
                BuildTestInteropFindingKind::MissingIdentity,
                BuildTestInteropFindingSeverity::Blocker,
                "packet id and timestamp are required",
            ));
        }

        let event_kinds: BTreeSet<_> = self.events.iter().map(|event| event.event_kind).collect();
        let source_kinds: BTreeSet<_> = self.events.iter().map(|event| event.source_kind).collect();
        let raw_refs: BTreeSet<&str> = self
            .raw_payload_refs
            .iter()
            .map(|raw| raw.raw_payload_ref.as_str())
            .collect();

        for source_kind in BuildTestEventSourceKind::ALL {
            if !source_kinds.contains(&source_kind) {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::MissingSourceKind,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!("source kind {} is missing", source_kind.as_str()),
                ));
            }
        }

        for event_kind in BuildTestEventKind::CANONICAL_LIFECYCLE {
            if !event_kinds.contains(&event_kind) {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::MissingLifecycleEvent,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!(
                        "canonical lifecycle event {} is missing",
                        event_kind.as_str()
                    ),
                ));
            }
        }

        for event in &self.events {
            if !event.is_bound() {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::MissingIdentity,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!("event {} has incomplete identity", event.event_id),
                ));
            }
            if !raw_refs.contains(event.raw_payload_ref.as_str()) {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::RawPayloadRefMissing,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!("event {} lacks a retained raw payload ref", event.event_id),
                ));
            }
            if event.confidence.overclaims_for(event.source_kind) {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::HeuristicOverclaimsConfidence,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!("heuristic event {} overclaims confidence", event.event_id),
                ));
            }
            if event.source_kind.is_heuristic() && !event.downgraded {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::HeuristicNotDowngraded,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!(
                        "heuristic event {} lacks downgrade disclosure",
                        event.event_id
                    ),
                ));
            }
        }

        for raw_ref in &self.raw_payload_refs {
            if !raw_ref.is_bound() {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::RawPayloadRefMissing,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!("raw payload ref {} is incomplete", raw_ref.raw_payload_ref),
                ));
            }
            if !raw_ref.replay_safe
                || !raw_ref.support_export_safe
                || !raw_ref.release_corpus_safe
                || !raw_ref.ai_evidence_safe
            {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::RawPayloadNotReplaySafe,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!(
                        "raw payload ref {} is not replay/export safe",
                        raw_ref.raw_payload_ref
                    ),
                ));
            }
            if !raw_ref.raw_private_material_excluded {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::ExportRedactionUnsafe,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!(
                        "raw payload ref {} admits private material",
                        raw_ref.raw_payload_ref
                    ),
                ));
            }
        }

        for lane in BuildTestInteropLane::REQUIRED {
            for source_kind in BuildTestEventSourceKind::ALL {
                let covered = self.capability_negotiations.iter().any(|row| {
                    row.lane == lane
                        && row.source_kind == source_kind
                        && row.is_bound()
                        && !matches!(row.state, AdapterCapabilityState::Unsupported)
                });
                if !covered {
                    findings.push(BuildTestInteropValidationFinding::new(
                        BuildTestInteropFindingKind::MissingCapabilityNegotiation,
                        BuildTestInteropFindingSeverity::Blocker,
                        format!(
                            "{} lacks capability negotiation for {}",
                            lane.as_str(),
                            source_kind.as_str()
                        ),
                    ));
                }
            }
        }

        for required_surface in BuildTestConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::MissingConsumerProjection,
                    BuildTestInteropFindingSeverity::Blocker,
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
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::ConsumerProjectionDrift,
                    BuildTestInteropFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve event interoperability truth",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if !self.replay_export_parity.preserves_truth() {
            findings.push(BuildTestInteropValidationFinding::new(
                BuildTestInteropFindingKind::ExportRedactionUnsafe,
                BuildTestInteropFindingSeverity::Blocker,
                "replay/export parity does not preserve event refs with redacted raw payload posture",
            ));
        }

        if include_record_fields {
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(BuildTestInteropValidationFinding::new(
                    BuildTestInteropFindingKind::PromotionStateMismatch,
                    BuildTestInteropFindingSeverity::Blocker,
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

/// Support-export wrapper carrying the exact interoperability packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTestEventInteroperabilitySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact packet exported.
    pub packet: BuildTestEventInteroperabilityPacket,
}

impl BuildTestEventInteroperabilitySupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == BUILD_TEST_EVENT_INTEROPERABILITY_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.raw_private_material_excluded
            && self.packet.is_stable()
    }
}

/// Builds the current stable interoperability packet input.
pub fn current_stable_build_test_event_interoperability_input(
) -> BuildTestEventInteroperabilityPacketInput {
    let packet_id = "packet:m4:build-test-event-interoperability:stable";
    let workspace_id = "workspace:checkout";
    let target_id = "target:checkout:all";
    let execution_context_id = "exec-context:local:checkout";
    let raw_payload_refs = vec![
        raw_ref("raw:native:queued", BuildTestEventSourceKind::Native),
        raw_ref("raw:bsp:target-graph", BuildTestEventSourceKind::Bsp),
        raw_ref("raw:bep:started", BuildTestEventSourceKind::BazelBep),
        raw_ref(
            "raw:structured:progress",
            BuildTestEventSourceKind::StructuredOutput,
        ),
        raw_ref(
            "raw:heuristic:diagnostic",
            BuildTestEventSourceKind::HeuristicParser,
        ),
        raw_ref("raw:native:test-started", BuildTestEventSourceKind::Native),
        raw_ref(
            "raw:structured:test-finished",
            BuildTestEventSourceKind::StructuredOutput,
        ),
        raw_ref("raw:bep:artifact", BuildTestEventSourceKind::BazelBep),
        raw_ref("raw:native:finished", BuildTestEventSourceKind::Native),
    ];
    let events = vec![
        event(
            "event:task-queued",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::Native,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:00Z",
            execution_context_id,
            BuildTestPayloadKind::Lifecycle,
            "raw:native:queued",
            "aureline-task",
            "adapter:aureline-task",
            BuildTestEventKind::TaskQueued,
            false,
            BuildTestInteropLane::LocalRuntime,
        ),
        event(
            "event:target-graph-ready",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::Bsp,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:01Z",
            "exec-context:remote-helper:checkout",
            BuildTestPayloadKind::Lifecycle,
            "raw:bsp:target-graph",
            "bsp",
            "adapter:bsp",
            BuildTestEventKind::TargetGraphReady,
            false,
            BuildTestInteropLane::RemoteHelper,
        ),
        event(
            "event:task-started",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::BazelBep,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:02Z",
            "exec-context:imported:bazel",
            BuildTestPayloadKind::Lifecycle,
            "raw:bep:started",
            "bazel",
            "adapter:bazel-bep",
            BuildTestEventKind::TaskStarted,
            false,
            BuildTestInteropLane::ImportedProvider,
        ),
        event(
            "event:progress-updated",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::StructuredOutput,
            BuildTestEventConfidence::MediumHigh,
            "2026-06-07T00:00:03Z",
            execution_context_id,
            BuildTestPayloadKind::Progress,
            "raw:structured:progress",
            "cargo-json",
            "adapter:cargo-json",
            BuildTestEventKind::ProgressUpdated,
            true,
            BuildTestInteropLane::LocalRuntime,
        ),
        event(
            "event:diagnostic-emitted",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::HeuristicParser,
            BuildTestEventConfidence::Low,
            "2026-06-07T00:00:04Z",
            execution_context_id,
            BuildTestPayloadKind::Diagnostic,
            "raw:heuristic:diagnostic",
            "stderr",
            "adapter:problem-matcher",
            BuildTestEventKind::DiagnosticEmitted,
            true,
            BuildTestInteropLane::LocalRuntime,
        ),
        event(
            "event:test-case-started",
            workspace_id,
            "target:checkout:test:total",
            BuildTestEventSourceKind::Native,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:05Z",
            execution_context_id,
            BuildTestPayloadKind::Test,
            "raw:native:test-started",
            "aureline-test",
            "adapter:aureline-test",
            BuildTestEventKind::TestCaseStarted,
            false,
            BuildTestInteropLane::LocalRuntime,
        ),
        event(
            "event:test-case-finished",
            workspace_id,
            "target:checkout:test:total",
            BuildTestEventSourceKind::StructuredOutput,
            BuildTestEventConfidence::MediumHigh,
            "2026-06-07T00:00:06Z",
            execution_context_id,
            BuildTestPayloadKind::Test,
            "raw:structured:test-finished",
            "junit",
            "adapter:junit-import",
            BuildTestEventKind::TestCaseFinished,
            true,
            BuildTestInteropLane::LocalRuntime,
        ),
        event(
            "event:artifact-published",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::BazelBep,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:07Z",
            "exec-context:imported:bazel",
            BuildTestPayloadKind::Artifact,
            "raw:bep:artifact",
            "bazel",
            "adapter:bazel-bep",
            BuildTestEventKind::ArtifactPublished,
            false,
            BuildTestInteropLane::ImportedProvider,
        ),
        event(
            "event:task-finished",
            workspace_id,
            target_id,
            BuildTestEventSourceKind::Native,
            BuildTestEventConfidence::High,
            "2026-06-07T00:00:08Z",
            execution_context_id,
            BuildTestPayloadKind::Lifecycle,
            "raw:native:finished",
            "aureline-task",
            "adapter:aureline-task",
            BuildTestEventKind::TaskFinished,
            false,
            BuildTestInteropLane::LocalRuntime,
        ),
    ];

    BuildTestEventInteroperabilityPacketInput {
        packet_id: packet_id.to_owned(),
        generated_at: "2026-06-07T00:00:09Z".to_owned(),
        events,
        raw_payload_refs,
        capability_negotiations: capability_rows(),
        consumer_projections: BuildTestConsumerSurface::REQUIRED
            .into_iter()
            .map(|surface| BuildTestConsumerProjection {
                consumer_surface: surface,
                projection_ref: format!("projection:{}:{}", packet_id, surface.as_str()),
                packet_id_ref: packet_id.to_owned(),
                preserves_event_id: true,
                preserves_source_kind: true,
                preserves_confidence: true,
                preserves_raw_payload_ref: true,
                preserves_provenance: true,
                preserves_downgrade_disclosure: true,
                raw_private_material_excluded: true,
            })
            .collect(),
        replay_export_parity: ReplayExportParity {
            replay_packet_ref: "replay:m4:build-test-event-interoperability".to_owned(),
            support_export_ref: "support-export:m4:build-test-event-interoperability".to_owned(),
            release_corpus_ref: "release-corpus:m4:build-test-event-interoperability".to_owned(),
            ai_evidence_packet_ref: "ai-evidence:m4:build-test-event-interoperability".to_owned(),
            replay_uses_canonical_envelopes: true,
            support_export_retains_raw_payload_refs: true,
            release_corpus_validates_drift: true,
            raw_private_material_excluded: true,
        },
    }
}

/// Builds the current stable interoperability packet.
pub fn current_stable_build_test_event_interoperability_packet(
) -> BuildTestEventInteroperabilityPacket {
    BuildTestEventInteroperabilityPacket::materialize(
        current_stable_build_test_event_interoperability_input(),
    )
}

fn raw_ref(
    raw_payload_ref: impl Into<String>,
    source_kind: BuildTestEventSourceKind,
) -> RawPayloadReference {
    let raw_payload_ref = raw_payload_ref.into();
    RawPayloadReference {
        payload_digest: format!("sha256:{}", raw_payload_ref.replace(':', "-")),
        raw_payload_ref,
        source_kind,
        retention_class: RawPayloadRetentionClass::MetadataDigestOnly,
        replay_safe: true,
        support_export_safe: true,
        release_corpus_safe: true,
        ai_evidence_safe: true,
        raw_private_material_excluded: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn event(
    event_id: impl Into<String>,
    workspace_id: &str,
    target_id: &str,
    source_kind: BuildTestEventSourceKind,
    confidence: BuildTestEventConfidence,
    timestamp: &str,
    execution_context_id: &str,
    payload_kind: BuildTestPayloadKind,
    raw_payload_ref: &str,
    build_tool_name: &str,
    adapter_id: &str,
    event_kind: BuildTestEventKind,
    downgraded: bool,
    lane: BuildTestInteropLane,
) -> BuildTestEventEnvelope {
    BuildTestEventEnvelope {
        event_id: event_id.into(),
        workspace_id: workspace_id.to_owned(),
        target_id: target_id.to_owned(),
        source_kind,
        confidence,
        timestamp: timestamp.to_owned(),
        execution_context_id: execution_context_id.to_owned(),
        payload_kind,
        raw_payload_ref: raw_payload_ref.to_owned(),
        provenance: BuildTestEventProvenance {
            build_tool_name: build_tool_name.to_owned(),
            build_tool_version: Some("1.0.0".to_owned()),
            adapter_id: adapter_id.to_owned(),
            adapter_version: "1.0.0".to_owned(),
            workspace_revision: Some("rev:checkout:abc123".to_owned()),
        },
        event_kind,
        downgraded,
        lane,
        parent_event_refs: Vec::new(),
        correlation_refs: vec!["trace:build-test-event-interoperability".to_owned()],
    }
}

fn capability_rows() -> Vec<AdapterCapabilityNegotiation> {
    let mut rows = Vec::new();
    for lane in BuildTestInteropLane::REQUIRED {
        for source_kind in BuildTestEventSourceKind::ALL {
            rows.push(AdapterCapabilityNegotiation {
                lane,
                source_kind,
                adapter_id: format!("adapter:{}:{}", lane.as_str(), source_kind.as_str()),
                capability: "canonical-build-test-event-envelope".to_owned(),
                state: if source_kind.is_heuristic() {
                    AdapterCapabilityState::Degraded
                } else {
                    AdapterCapabilityState::Negotiated
                },
                capability_packet_ref: format!(
                    "capability-packet:{}:{}",
                    lane.as_str(),
                    source_kind.as_str()
                ),
                downgrade_reason_ref: source_kind
                    .is_heuristic()
                    .then(|| "downgrade:heuristic-parser-fallback".to_owned()),
            });
        }
    }
    rows
}

fn promotion_state_for_findings(
    findings: &[BuildTestInteropValidationFinding],
) -> BuildTestInteropPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    {
        BuildTestInteropPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Warning)
    {
        BuildTestInteropPromotionState::NarrowedBelowStable
    } else {
        BuildTestInteropPromotionState::Stable
    }
}
