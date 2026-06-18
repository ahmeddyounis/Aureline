//! Canonical task/test/debug event envelope and the first-consumer event bus
//! shared by notebook runs, the task center, test sessions, debug activity, and
//! pipeline overlays, plus the support/export and CLI/headless surfaces.
//!
//! The frozen [`crate::m5_task_event_adapter_policy`] layer fixes the rules: the
//! native-first adapter-priority ladder, the raw-payload-retention matrix, the
//! closed downgrade vocabulary, and the consumer bindings. This module is the
//! implementation those rules govern. It lands one canonical
//! [`TaskEventRecord`] — carrying a stable event identity, trace correlation,
//! producer lane, workspace and target identity, source kind, adapter-priority
//! rank, confidence, payload class, execution context, retained raw-payload
//! reference, producer provenance, and an explicit downgrade flag — and a
//! [`TaskEventFirstConsumersPacket`] that proves every claimed M5 execution
//! surface reads that record instead of re-deriving execution truth from
//! rendered logs.
//!
//! It reuses the [`crate::build_test_event_interoperability`] source-kind,
//! confidence, lifecycle, payload, retention-class, and provenance vocabulary and
//! the [`crate::m5_task_event_adapter_policy`] priority ladder, confidence
//! ceilings, and downgrade vocabulary rather than minting parallel tokens. The
//! field this layer adds beyond the policy envelope is the explicit
//! [`payload_kind`](TaskEventRecord::payload_kind), so a consumer can route an
//! event by payload class without parsing its body.
//!
//! Three invariants make the history trustworthy:
//!
//! - **No surface stays log-only.** Every emitting lane (notebook, task, test,
//!   debug, pipeline) must carry at least one canonical record, and every claimed
//!   consumer surface must bind a projection that preserves the record's source,
//!   priority rank, confidence, payload class, downgrade disclosure, raw-payload
//!   reference, and provenance.
//! - **Source and confidence are explainable without bespoke parsing.** The
//!   support/export and CLI/headless surfaces project a per-event explanation
//!   derived only from canonical fields, never from a feature-local status string.
//! - **History is stable through virtualization, replay, and export.** Records
//!   order deterministically by `(trace_id, sequence, event_id)`; a replay digest
//!   is invariant to the order records arrive in, so a virtualized window or an
//!   exported bundle reproduces the same chronology.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/task-event-envelope.md`](../../../docs/m5/task-event-envelope.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/task-event-first-consumers.schema.json`](../../../schemas/tooling/task-event-first-consumers.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    BuildTestEventConfidence, BuildTestEventKind, BuildTestEventProvenance,
    BuildTestEventSourceKind, BuildTestInteropFindingSeverity, BuildTestInteropPromotionState,
    BuildTestPayloadKind, RawPayloadRetentionClass,
};
use crate::m5_task_event_adapter_policy::{
    canonical_confidence_ceiling, canonical_priority_rank, DowngradeReason,
};

/// Stable record-kind tag for [`TaskEventFirstConsumersPacket`].
pub const TASK_EVENT_FIRST_CONSUMERS_RECORD_KIND: &str = "m5_task_event_first_consumers_packet";

/// Stable record-kind tag for [`TaskEventFirstConsumersSupportExport`].
pub const TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_task_event_first_consumers_support_export";

/// Stable record-kind tag for [`TaskEventCliHeadlessView`].
pub const TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_task_event_first_consumers_cli_headless";

/// Integer schema version for the first-consumers packet.
pub const TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const TASK_EVENT_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/tooling/task-event-first-consumers.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const TASK_EVENT_FIRST_CONSUMERS_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TASK_EVENT_FIRST_CONSUMERS_DOC_REF: &str = "docs/m5/task-event-envelope.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const TASK_EVENT_FIRST_CONSUMERS_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR: &str = "fixtures/tooling/m5/event-envelope";

/// Repo-relative path of the checked-in packet artifact.
pub const TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/event-envelope-first-consumers/packet.json";

/// Stable packet id minted by the seed.
pub const TASK_EVENT_FIRST_CONSUMERS_PACKET_ID: &str = "tooling:m5:task-event-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:task-event-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:tooling:m5:task-event-first-consumers";

/// Execution surface that emits and/or consumes the canonical task-event record.
///
/// The five emitting surfaces — notebook runs, the task center, test sessions,
/// debug activity, and pipeline overlays — both produce records and read them
/// back. The two export surfaces — support/export and CLI/headless — only
/// consume, and they must explain a record's source and confidence from the
/// canonical fields alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventSurface {
    /// Notebook run cells and kernel-backed tests.
    NotebookRun,
    /// Task center timeline and task headers.
    TaskCenter,
    /// Test explorer sessions and inline results.
    TestSession,
    /// Debug session activity and chronology.
    DebugSession,
    /// Pipeline / run-control overlays.
    Pipeline,
    /// Support and release export packets.
    SupportExport,
    /// CLI / headless stable JSON surface.
    CliHeadless,
}

impl TaskEventSurface {
    /// Every claimed task-event surface in stable declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotebookRun,
        Self::TaskCenter,
        Self::TestSession,
        Self::DebugSession,
        Self::Pipeline,
        Self::SupportExport,
        Self::CliHeadless,
    ];

    /// The five surfaces that emit canonical records.
    pub const EMITTING: [Self; 5] = [
        Self::NotebookRun,
        Self::TaskCenter,
        Self::TestSession,
        Self::DebugSession,
        Self::Pipeline,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRun => "notebook_run",
            Self::TaskCenter => "task_center",
            Self::TestSession => "test_session",
            Self::DebugSession => "debug_session",
            Self::Pipeline => "pipeline",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }

    /// True when the surface emits canonical records into the bus.
    pub const fn emits(self) -> bool {
        matches!(
            self,
            Self::NotebookRun
                | Self::TaskCenter
                | Self::TestSession
                | Self::DebugSession
                | Self::Pipeline
        )
    }

    /// True when the surface is an export consumer that must explain truth.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::SupportExport | Self::CliHeadless)
    }
}

/// Numeric weight used to compare confidence levels (higher is stronger).
const fn confidence_weight(confidence: BuildTestEventConfidence) -> u8 {
    match confidence {
        BuildTestEventConfidence::High => 4,
        BuildTestEventConfidence::MediumHigh => 3,
        BuildTestEventConfidence::Medium => 2,
        BuildTestEventConfidence::Low => 1,
    }
}

/// True when `confidence` exceeds the ceiling allowed for `source_kind`.
fn confidence_overclaims(
    confidence: BuildTestEventConfidence,
    source_kind: BuildTestEventSourceKind,
) -> bool {
    confidence_weight(confidence) > confidence_weight(canonical_confidence_ceiling(source_kind))
}

/// Canonical payload class for a lifecycle event kind.
///
/// The debug session lane may additionally tag its lifecycle records with
/// [`BuildTestPayloadKind::Debug`]; every other lane uses this mapping verbatim.
pub const fn canonical_payload_kind(event_kind: BuildTestEventKind) -> BuildTestPayloadKind {
    match event_kind {
        BuildTestEventKind::TaskQueued
        | BuildTestEventKind::TargetGraphReady
        | BuildTestEventKind::TaskStarted
        | BuildTestEventKind::TaskFinished => BuildTestPayloadKind::Lifecycle,
        BuildTestEventKind::ProgressUpdated => BuildTestPayloadKind::Progress,
        BuildTestEventKind::DiagnosticEmitted => BuildTestPayloadKind::Diagnostic,
        BuildTestEventKind::TestCaseStarted | BuildTestEventKind::TestCaseFinished => {
            BuildTestPayloadKind::Test
        }
        BuildTestEventKind::ArtifactPublished => BuildTestPayloadKind::Artifact,
    }
}

/// Canonical task/test/debug event envelope emitted onto the first-consumer bus.
///
/// One record rides every meaningful runtime event. Consumers group records by
/// [`trace_id`](Self::trace_id), order them by
/// [`sequence`](Self::sequence), and route them by
/// [`payload_kind`](Self::payload_kind) without re-parsing console text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventRecord {
    /// Unique stable identifier within the packet.
    pub event_id: String,
    /// Correlation id shared across records for the same run.
    pub trace_id: String,
    /// Monotonic ordering position within the record's trace.
    pub sequence: u64,
    /// Execution surface that emitted the record.
    pub producer_lane: TaskEventSurface,
    /// Workspace or workset identity.
    pub workspace_id: String,
    /// Build target, task, test suite, or debug-configuration identity.
    pub target_id: String,
    /// Canonical lifecycle kind.
    pub event_kind: BuildTestEventKind,
    /// Payload class naming the shape of the event without decoding it.
    pub payload_kind: BuildTestPayloadKind,
    /// Source kind that produced the event.
    pub source_kind: BuildTestEventSourceKind,
    /// Adapter priority rank (must match the source's canonical rank).
    pub priority_rank: u8,
    /// Confidence (at or below the source's ceiling).
    pub confidence: BuildTestEventConfidence,
    /// Capture time in the producing execution context.
    pub captured_at: String,
    /// Resolved environment/toolchain/runtime context.
    pub execution_context_id: String,
    /// Pointer to the retained raw adapter payload.
    pub raw_payload_ref: String,
    /// Retention class for the raw payload.
    pub raw_payload_retention_class: RawPayloadRetentionClass,
    /// Producer provenance.
    pub provenance: BuildTestEventProvenance,
    /// True when the emission is visibly downgraded on every consumer surface.
    pub downgraded: bool,
    /// Downgrade reason, present iff the emission is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason: Option<DowngradeReason>,
}

impl TaskEventRecord {
    fn is_bound(&self) -> bool {
        !self.event_id.trim().is_empty()
            && !self.trace_id.trim().is_empty()
            && !self.workspace_id.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && !self.captured_at.trim().is_empty()
            && !self.execution_context_id.trim().is_empty()
            && !self.raw_payload_ref.trim().is_empty()
            && !self.provenance.build_tool_name.trim().is_empty()
            && !self.provenance.adapter_id.trim().is_empty()
            && !self.provenance.adapter_version.trim().is_empty()
    }

    fn payload_kind_consistent(&self) -> bool {
        self.payload_kind == canonical_payload_kind(self.event_kind)
            || (self.producer_lane == TaskEventSurface::DebugSession
                && self.payload_kind == BuildTestPayloadKind::Debug)
    }

    /// Support-safe one-line explanation of source and confidence derived only
    /// from canonical fields — never from rendered output.
    ///
    /// Support/export and CLI/headless surfaces present this string so a reviewer
    /// can tell where an event came from and how much to trust it without parsing
    /// a feature-local status line.
    pub fn explain(&self) -> String {
        let mut text = format!(
            "{} ({}) from {} adapter at priority {} with {} confidence; execution {} via {}; raw payload {} retained as {}",
            self.event_kind.as_str(),
            self.payload_kind.as_str(),
            self.source_kind.as_str(),
            self.priority_rank,
            self.confidence.as_str(),
            self.execution_context_id,
            self.provenance.adapter_id,
            self.raw_payload_ref,
            self.raw_payload_retention_class.as_str(),
        );
        if let Some(reason) = self.downgrade_reason {
            text.push_str(&format!(" — downgraded: {}", reason.as_str()));
        }
        text
    }

    /// Deterministic ordering key for replay and virtualization.
    fn order_key(&self) -> (&str, u64, &str) {
        (
            self.trace_id.as_str(),
            self.sequence,
            self.event_id.as_str(),
        )
    }
}

/// Projection proving a claimed surface reads the canonical record verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventSurfaceProjection {
    /// Consumer surface.
    pub surface: TaskEventSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// True when the surface reads the canonical record, not rendered output.
    pub reads_canonical_envelope: bool,
    /// True when event ids remain stable on the surface.
    pub preserves_event_id: bool,
    /// True when source kind remains visible.
    pub preserves_source_kind: bool,
    /// True when the adapter priority rank remains visible.
    pub preserves_priority_rank: bool,
    /// True when confidence remains visible.
    pub preserves_confidence: bool,
    /// True when the payload class remains visible.
    pub preserves_payload_kind: bool,
    /// True when downgraded rows stay visibly downgraded.
    pub preserves_downgrade_disclosure: bool,
    /// True when the retained raw-payload reference remains inspectable.
    pub preserves_raw_payload_ref: bool,
    /// True when provenance remains inspectable.
    pub preserves_provenance: bool,
    /// True when the surface can explain source and confidence without parsing.
    pub explains_source_and_confidence: bool,
    /// Count of canonical records this surface references (derived).
    pub observed_event_count: usize,
}

impl TaskEventSurfaceProjection {
    fn preserves_truth(&self) -> bool {
        !self.projection_ref.trim().is_empty()
            && self.reads_canonical_envelope
            && self.preserves_event_id
            && self.preserves_source_kind
            && self.preserves_priority_rank
            && self.preserves_confidence
            && self.preserves_payload_kind
            && self.preserves_downgrade_disclosure
            && self.preserves_raw_payload_ref
            && self.preserves_provenance
    }
}

/// Replay-stable summary of one trace's record history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTraceSummary {
    /// Correlation id for the trace.
    pub trace_id: String,
    /// Workspace identity of the first record in the trace.
    pub workspace_id: String,
    /// Target identity of the first record in the trace.
    pub target_id: String,
    /// Count of records in the trace.
    pub event_count: usize,
    /// Lowest sequence number observed in the trace.
    pub first_sequence: u64,
    /// Highest sequence number observed in the trace.
    pub last_sequence: u64,
    /// Distinct source-kind tokens observed, sorted.
    pub source_kinds: Vec<String>,
    /// Count of visibly downgraded records in the trace.
    pub downgraded_event_count: usize,
    /// Order-invariant digest of the trace's ordered event ids.
    pub replay_digest: String,
}

/// Closed validation finding vocabulary for the first-consumers packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventBusFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The packet carries no canonical records.
    NoCanonicalEvents,
    /// A record has incomplete identity.
    EventIdentityIncomplete,
    /// Two records share an event id.
    DuplicateEventId,
    /// A record's priority rank disagrees with its source kind.
    EventPriorityMismatch,
    /// A record claims confidence above its source ceiling.
    EventConfidenceOverclaim,
    /// A record's payload class disagrees with its event kind.
    EventPayloadKindMismatch,
    /// A record's downgrade flag and reason are inconsistent.
    EventDowngradeInconsistent,
    /// A record names a producer lane that does not emit.
    ProducerLaneNotEmitting,
    /// A claimed emitting lane carries no canonical records (would be log-only).
    LaneMissingCanonicalEvents,
    /// A required consumer-surface projection is absent.
    SurfaceProjectionMissing,
    /// A consumer-surface projection drops canonical record truth.
    SurfaceProjectionDropsTruth,
    /// An export surface cannot explain source and confidence without parsing.
    ExportCannotExplain,
    /// Two records in one trace share a sequence number.
    ReplaySequenceCollision,
    /// A stored trace summary disagrees with the derived summary.
    TraceSummaryDrift,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl EventBusFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::NoCanonicalEvents => "no_canonical_events",
            Self::EventIdentityIncomplete => "event_identity_incomplete",
            Self::DuplicateEventId => "duplicate_event_id",
            Self::EventPriorityMismatch => "event_priority_mismatch",
            Self::EventConfidenceOverclaim => "event_confidence_overclaim",
            Self::EventPayloadKindMismatch => "event_payload_kind_mismatch",
            Self::EventDowngradeInconsistent => "event_downgrade_inconsistent",
            Self::ProducerLaneNotEmitting => "producer_lane_not_emitting",
            Self::LaneMissingCanonicalEvents => "lane_missing_canonical_events",
            Self::SurfaceProjectionMissing => "surface_projection_missing",
            Self::SurfaceProjectionDropsTruth => "surface_projection_drops_truth",
            Self::ExportCannotExplain => "export_cannot_explain",
            Self::ReplaySequenceCollision => "replay_sequence_collision",
            Self::TraceSummaryDrift => "trace_summary_drift",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventBusValidationFinding {
    /// Closed finding kind.
    pub finding_kind: EventBusFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl EventBusValidationFinding {
    fn blocker(finding_kind: EventBusFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`TaskEventFirstConsumersPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventFirstConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Canonical record history.
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Consumer-surface projections.
    #[serde(default)]
    pub surface_projections: Vec<TaskEventSurfaceProjection>,
}

/// Canonical first-consumers packet: the record history plus the surface
/// projections that prove every claimed M5 surface reads it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventFirstConsumersPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// First-consumers boundary schema ref.
    pub first_consumers_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// Canonical record history.
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Derived per-trace summaries.
    #[serde(default)]
    pub trace_summaries: Vec<TaskEventTraceSummary>,
    /// Consumer-surface projections.
    #[serde(default)]
    pub surface_projections: Vec<TaskEventSurfaceProjection>,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<EventBusValidationFinding>,
}

impl TaskEventFirstConsumersPacket {
    /// Materializes a packet, deriving trace summaries and observed counts, then
    /// records validation findings and the derived promotion state.
    pub fn materialize(input: TaskEventFirstConsumersPacketInput) -> Self {
        let events = input.events;
        let trace_summaries = derive_trace_summaries(&events);
        let surface_projections = derive_projection_counts(input.surface_projections, &events);

        let mut packet = Self {
            record_kind: TASK_EVENT_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            first_consumers_schema_ref: TASK_EVENT_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            envelope_schema_ref: TASK_EVENT_FIRST_CONSUMERS_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: TASK_EVENT_FIRST_CONSUMERS_DOC_REF.to_owned(),
            policy_baseline_ref: TASK_EVENT_FIRST_CONSUMERS_POLICY_BASELINE_REF.to_owned(),
            events,
            trace_summaries,
            surface_projections,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the frozen invariants.
    pub fn validate(&self) -> Vec<EventBusValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Returns the records in deterministic replay order.
    ///
    /// The order is invariant to the order records were emitted in, so a
    /// virtualized window or an exported bundle reproduces the same chronology.
    pub fn replay_ordered(&self) -> Vec<&TaskEventRecord> {
        let mut ordered: Vec<&TaskEventRecord> = self.events.iter().collect();
        ordered.sort_by(|a, b| a.order_key().cmp(&b.order_key()));
        ordered
    }

    /// Returns a contiguous, replay-stable virtualization window for one trace.
    ///
    /// `offset` and `limit` slice the trace's ordered history; the same arguments
    /// always return the same records regardless of emission order.
    pub fn trace_window(
        &self,
        trace_id: &str,
        offset: usize,
        limit: usize,
    ) -> Vec<&TaskEventRecord> {
        self.replay_ordered()
            .into_iter()
            .filter(|record| record.trace_id == trace_id)
            .skip(offset)
            .take(limit)
            .collect()
    }

    /// Order-invariant digest of the whole packet's ordered event ids.
    pub fn replay_digest(&self) -> String {
        let ordered = self.replay_ordered();
        let ids: Vec<&str> = ordered
            .iter()
            .map(|record| record.event_id.as_str())
            .collect();
        replay_digest(&ids)
    }

    /// Builds the CLI/headless stable view consumers read without parsing logs.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> TaskEventCliHeadlessView {
        let rows = self
            .replay_ordered()
            .into_iter()
            .map(TaskEventCliHeadlessRow::from_record)
            .collect();
        TaskEventCliHeadlessView {
            record_kind: TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            replay_digest: self.replay_digest(),
            rows,
            trace_summaries: self.trace_summaries.clone(),
        }
    }

    /// Builds an export-safe support packet carrying the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> TaskEventFirstConsumersSupportExport {
        TaskEventFirstConsumersSupportExport {
            record_kind: TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            packet: self.clone(),
        }
    }

    /// Returns the surface tokens present in the projections.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for projection in &self.surface_projections {
            set.insert(projection.surface);
        }
        set.into_iter().map(TaskEventSurface::as_str).collect()
    }

    /// Returns the source-kind tokens present in the record history.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.events {
            set.insert(event.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the payload-kind tokens present in the record history.
    pub fn payload_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.events {
            set.insert(event.payload_kind);
        }
        set.into_iter().map(BuildTestPayloadKind::as_str).collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet {} schema_version={} promotion={} events={} traces={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.events.len(),
            self.trace_summaries.len(),
            self.replay_digest(),
        ));
        for event in self.replay_ordered() {
            lines.push(format!(
                "event {} trace={} seq={} lane={} kind={} payload={} source={} rank={} conf={} downgraded={}",
                event.event_id,
                event.trace_id,
                event.sequence,
                event.producer_lane.as_str(),
                event.event_kind.as_str(),
                event.payload_kind.as_str(),
                event.source_kind.as_str(),
                event.priority_rank,
                event.confidence.as_str(),
                event.downgraded,
            ));
        }
        for summary in &self.trace_summaries {
            lines.push(format!(
                "trace {} events={} sources={} downgraded={} digest={}",
                summary.trace_id,
                summary.event_count,
                summary.source_kinds.join("|"),
                summary.downgraded_event_count,
                summary.replay_digest,
            ));
        }
        for projection in &self.surface_projections {
            lines.push(format!(
                "surface {} reads_canonical={} preserves_payload={} preserves_confidence={} explains={} observed={}",
                projection.surface.as_str(),
                projection.reads_canonical_envelope,
                projection.preserves_payload_kind,
                projection.preserves_confidence,
                projection.explains_source_and_confidence,
                projection.observed_event_count,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<EventBusValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != TASK_EVENT_FIRST_CONSUMERS_RECORD_KIND {
            findings.push(EventBusValidationFinding::blocker(
                EventBusFindingKind::WrongRecordKind,
                "packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION
        {
            findings.push(EventBusValidationFinding::blocker(
                EventBusFindingKind::WrongSchemaVersion,
                "packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(EventBusValidationFinding::blocker(
                EventBusFindingKind::MissingIdentity,
                "packet id and timestamp are required",
            ));
        }
        for (label, value) in [
            (
                "first-consumers schema",
                self.first_consumers_schema_ref.as_str(),
            ),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
            ("policy baseline", self.policy_baseline_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_events(&mut findings);
        self.check_lane_coverage(&mut findings);
        self.check_surface_projections(&mut findings);

        if include_record_fields {
            let derived = derive_trace_summaries(&self.events);
            if derived != self.trace_summaries {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::TraceSummaryDrift,
                    "stored trace summaries do not match the derived summaries",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::PromotionStateMismatch,
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

    fn check_events(&self, findings: &mut Vec<EventBusValidationFinding>) {
        if self.events.is_empty() {
            findings.push(EventBusValidationFinding::blocker(
                EventBusFindingKind::NoCanonicalEvents,
                "packet carries no canonical records",
            ));
            return;
        }

        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let mut seen_trace_seq: BTreeSet<(&str, u64)> = BTreeSet::new();
        for event in &self.events {
            if !event.is_bound() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::EventIdentityIncomplete,
                    format!("record {} has incomplete identity", event.event_id),
                ));
            }
            if !event.event_id.trim().is_empty() && !seen_ids.insert(event.event_id.as_str()) {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::DuplicateEventId,
                    format!("record id {} is not unique", event.event_id),
                ));
            }
            if !event.producer_lane.emits() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::ProducerLaneNotEmitting,
                    format!(
                        "record {} names non-emitting producer lane {}",
                        event.event_id,
                        event.producer_lane.as_str()
                    ),
                ));
            }
            if event.priority_rank != canonical_priority_rank(event.source_kind) {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::EventPriorityMismatch,
                    format!(
                        "record {} carries a priority rank that disagrees with {}",
                        event.event_id,
                        event.source_kind.as_str()
                    ),
                ));
            }
            if confidence_overclaims(event.confidence, event.source_kind) {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::EventConfidenceOverclaim,
                    format!(
                        "record {} overclaims confidence for {}",
                        event.event_id,
                        event.source_kind.as_str()
                    ),
                ));
            }
            if !event.payload_kind_consistent() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::EventPayloadKindMismatch,
                    format!(
                        "record {} payload class {} disagrees with {}",
                        event.event_id,
                        event.payload_kind.as_str(),
                        event.event_kind.as_str()
                    ),
                ));
            }
            if event.downgraded != event.downgrade_reason.is_some() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::EventDowngradeInconsistent,
                    format!(
                        "record {} downgrade flag and reason disagree",
                        event.event_id
                    ),
                ));
            }
            if !seen_trace_seq.insert((event.trace_id.as_str(), event.sequence)) {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::ReplaySequenceCollision,
                    format!(
                        "trace {} reuses sequence {} so replay order is ambiguous",
                        event.trace_id, event.sequence
                    ),
                ));
            }
        }
    }

    fn check_lane_coverage(&self, findings: &mut Vec<EventBusValidationFinding>) {
        for lane in TaskEventSurface::EMITTING {
            let count = self
                .events
                .iter()
                .filter(|event| event.producer_lane == lane)
                .count();
            if count == 0 {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::LaneMissingCanonicalEvents,
                    format!(
                        "{} has no canonical records and would stay log-only",
                        lane.as_str()
                    ),
                ));
            }
        }
    }

    fn check_surface_projections(&self, findings: &mut Vec<EventBusValidationFinding>) {
        let present: BTreeSet<TaskEventSurface> = self
            .surface_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for surface in TaskEventSurface::ALL {
            if !present.contains(&surface) {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::SurfaceProjectionMissing,
                    format!("surface projection is missing for {}", surface.as_str()),
                ));
            }
        }
        for projection in &self.surface_projections {
            if !projection.preserves_truth() {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::SurfaceProjectionDropsTruth,
                    format!(
                        "{} projection drops canonical record truth",
                        projection.surface.as_str()
                    ),
                ));
            }
            if projection.surface.is_export() && !projection.explains_source_and_confidence {
                findings.push(EventBusValidationFinding::blocker(
                    EventBusFindingKind::ExportCannotExplain,
                    format!(
                        "{} export cannot explain source and confidence without parsing",
                        projection.surface.as_str()
                    ),
                ));
            }
        }
    }
}

/// Support-export wrapper carrying the exact first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventFirstConsumersSupportExport {
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
    /// Exact packet exported.
    pub packet: TaskEventFirstConsumersPacket,
}

impl TaskEventFirstConsumersSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.packet.is_stable()
    }
}

/// One CLI/headless row, carrying canonical fields plus a derived explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventCliHeadlessRow {
    /// Event id.
    pub event_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Ordering position within the trace.
    pub sequence: u64,
    /// Producer lane token.
    pub producer_lane: String,
    /// Lifecycle kind token.
    pub event_kind: String,
    /// Payload class token.
    pub payload_kind: String,
    /// Source kind token.
    pub source_kind: String,
    /// Adapter priority rank.
    pub priority_rank: u8,
    /// Confidence token.
    pub confidence: String,
    /// True when the row is visibly downgraded.
    pub downgraded: bool,
    /// Downgrade reason token, present iff downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason: Option<String>,
    /// Retained raw-payload reference.
    pub raw_payload_ref: String,
    /// Raw-payload retention class token.
    pub raw_payload_retention_class: String,
    /// Support-safe explanation derived from canonical fields.
    pub explanation: String,
}

impl TaskEventCliHeadlessRow {
    fn from_record(record: &TaskEventRecord) -> Self {
        Self {
            event_id: record.event_id.clone(),
            trace_id: record.trace_id.clone(),
            sequence: record.sequence,
            producer_lane: record.producer_lane.as_str().to_owned(),
            event_kind: record.event_kind.as_str().to_owned(),
            payload_kind: record.payload_kind.as_str().to_owned(),
            source_kind: record.source_kind.as_str().to_owned(),
            priority_rank: record.priority_rank,
            confidence: record.confidence.as_str().to_owned(),
            downgraded: record.downgraded,
            downgrade_reason: record
                .downgrade_reason
                .map(|reason| reason.as_str().to_owned()),
            raw_payload_ref: record.raw_payload_ref.clone(),
            raw_payload_retention_class: record.raw_payload_retention_class.as_str().to_owned(),
            explanation: record.explain(),
        }
    }
}

/// CLI/headless stable view of the canonical record history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventCliHeadlessView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant replay digest of the source packet.
    pub replay_digest: String,
    /// Rows in deterministic replay order.
    #[serde(default)]
    pub rows: Vec<TaskEventCliHeadlessRow>,
    /// Per-trace summaries.
    #[serde(default)]
    pub trace_summaries: Vec<TaskEventTraceSummary>,
}

impl TaskEventCliHeadlessView {
    /// Returns true when every row can explain source and confidence.
    pub fn every_row_explains(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.source_kind.trim().is_empty()
                && !row.confidence.trim().is_empty()
                && !row.explanation.trim().is_empty()
        })
    }
}

fn derive_projection_counts(
    mut projections: Vec<TaskEventSurfaceProjection>,
    events: &[TaskEventRecord],
) -> Vec<TaskEventSurfaceProjection> {
    for projection in &mut projections {
        projection.observed_event_count = if projection.surface.emits() {
            events
                .iter()
                .filter(|event| event.producer_lane == projection.surface)
                .count()
        } else {
            // Export surfaces read the whole canonical history.
            events.len()
        };
    }
    projections
}

fn derive_trace_summaries(events: &[TaskEventRecord]) -> Vec<TaskEventTraceSummary> {
    let mut by_trace: BTreeMap<&str, Vec<&TaskEventRecord>> = BTreeMap::new();
    for event in events {
        by_trace
            .entry(event.trace_id.as_str())
            .or_default()
            .push(event);
    }
    by_trace
        .into_iter()
        .map(|(trace_id, mut records)| {
            records.sort_by(|a, b| a.order_key().cmp(&b.order_key()));
            let first = records.first().expect("trace has at least one record");
            let first_sequence = records
                .iter()
                .map(|record| record.sequence)
                .min()
                .unwrap_or(0);
            let last_sequence = records
                .iter()
                .map(|record| record.sequence)
                .max()
                .unwrap_or(0);
            let mut source_kinds: BTreeSet<&str> = BTreeSet::new();
            let mut downgraded_event_count = 0usize;
            for record in &records {
                source_kinds.insert(record.source_kind.as_str());
                if record.downgraded {
                    downgraded_event_count += 1;
                }
            }
            let ids: Vec<&str> = records
                .iter()
                .map(|record| record.event_id.as_str())
                .collect();
            TaskEventTraceSummary {
                trace_id: trace_id.to_owned(),
                workspace_id: first.workspace_id.clone(),
                target_id: first.target_id.clone(),
                event_count: records.len(),
                first_sequence,
                last_sequence,
                source_kinds: source_kinds.into_iter().map(str::to_owned).collect(),
                downgraded_event_count,
                replay_digest: replay_digest(&ids),
            }
        })
        .collect()
}

/// Order-stable FNV-1a 64-bit digest of a sequence of event ids.
fn replay_digest(event_ids_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for id in event_ids_in_order {
        for byte in id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

fn promotion_state_for_findings(
    findings: &[EventBusValidationFinding],
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

/// Builds the canonical stable first-consumers packet input.
pub fn current_stable_task_event_first_consumers_input() -> TaskEventFirstConsumersPacketInput {
    TaskEventFirstConsumersPacketInput {
        packet_id: TASK_EVENT_FIRST_CONSUMERS_PACKET_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        events: canonical_events(),
        surface_projections: canonical_surface_projections(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_task_event_first_consumers_packet() -> TaskEventFirstConsumersPacket {
    TaskEventFirstConsumersPacket::materialize(current_stable_task_event_first_consumers_input())
}

/// Validates a packet and returns an `Ok(())` / findings result.
pub fn validate_task_event_first_consumers_packet(
    packet: &TaskEventFirstConsumersPacket,
) -> Result<(), Vec<EventBusValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

#[allow(clippy::too_many_arguments)]
fn record(
    event_id: &str,
    trace_id: &str,
    sequence: u64,
    producer_lane: TaskEventSurface,
    target_id: &str,
    event_kind: BuildTestEventKind,
    payload_kind: BuildTestPayloadKind,
    source_kind: BuildTestEventSourceKind,
    confidence: BuildTestEventConfidence,
    adapter_id: &str,
    downgrade_reason: Option<DowngradeReason>,
) -> TaskEventRecord {
    let raw_payload_retention_class = match source_kind {
        BuildTestEventSourceKind::BazelBep => RawPayloadRetentionClass::RedactedReference,
        _ => RawPayloadRetentionClass::MetadataDigestOnly,
    };
    TaskEventRecord {
        event_id: event_id.to_owned(),
        trace_id: trace_id.to_owned(),
        sequence,
        producer_lane,
        workspace_id: "workspace:checkout".to_owned(),
        target_id: target_id.to_owned(),
        event_kind,
        payload_kind,
        source_kind,
        priority_rank: canonical_priority_rank(source_kind),
        confidence,
        captured_at: "2026-06-17T00:00:00Z".to_owned(),
        execution_context_id: "exec-context:local:checkout".to_owned(),
        raw_payload_ref: format!("raw:{event_id}"),
        raw_payload_retention_class,
        provenance: BuildTestEventProvenance {
            build_tool_name: adapter_id.to_owned(),
            build_tool_version: Some("1.0.0".to_owned()),
            adapter_id: format!("adapter:{adapter_id}"),
            adapter_version: "1.0.0".to_owned(),
            workspace_revision: Some("rev:checkout:abc123".to_owned()),
        },
        downgraded: downgrade_reason.is_some(),
        downgrade_reason,
    }
}

fn canonical_events() -> Vec<TaskEventRecord> {
    use BuildTestEventConfidence::{High, MediumHigh};
    use BuildTestEventKind::{
        ArtifactPublished, DiagnosticEmitted, ProgressUpdated, TaskFinished, TaskQueued,
        TaskStarted, TestCaseFinished, TestCaseStarted,
    };
    use BuildTestEventSourceKind::{BazelBep, Bsp, HeuristicParser, Native, StructuredOutput};
    use BuildTestPayloadKind::{Artifact, Debug, Diagnostic, Lifecycle, Progress, Test};
    use TaskEventSurface::{DebugSession, NotebookRun, Pipeline, TaskCenter, TestSession};

    vec![
        // Task center build trace: native lifecycle and progress truth.
        record(
            "event:task:queued",
            "trace:task:build",
            1,
            TaskCenter,
            "target:checkout:build",
            TaskQueued,
            Lifecycle,
            Native,
            High,
            "aureline-task",
            None,
        ),
        record(
            "event:task:started",
            "trace:task:build",
            2,
            TaskCenter,
            "target:checkout:build",
            TaskStarted,
            Lifecycle,
            Native,
            High,
            "aureline-task",
            None,
        ),
        record(
            "event:task:progress",
            "trace:task:build",
            3,
            TaskCenter,
            "target:checkout:build",
            ProgressUpdated,
            Progress,
            Native,
            High,
            "aureline-task",
            None,
        ),
        record(
            "event:task:finished",
            "trace:task:build",
            4,
            TaskCenter,
            "target:checkout:build",
            TaskFinished,
            Lifecycle,
            Native,
            High,
            "aureline-task",
            None,
        ),
        // Test session trace: native start, BSP-reported finish.
        record(
            "event:test:started",
            "trace:test:suite",
            1,
            TestSession,
            "target:checkout:test",
            TestCaseStarted,
            Test,
            Native,
            High,
            "aureline-test",
            None,
        ),
        record(
            "event:test:finished",
            "trace:test:suite",
            2,
            TestSession,
            "target:checkout:test",
            TestCaseFinished,
            Test,
            Bsp,
            High,
            "bsp",
            None,
        ),
        // Notebook run trace: native lifecycle plus an imported structured result.
        record(
            "event:notebook:started",
            "trace:notebook:run",
            1,
            NotebookRun,
            "target:checkout:notebook",
            TaskStarted,
            Lifecycle,
            Native,
            High,
            "aureline-notebook",
            None,
        ),
        record(
            "event:notebook:test",
            "trace:notebook:run",
            2,
            NotebookRun,
            "target:checkout:notebook",
            TestCaseFinished,
            Test,
            StructuredOutput,
            MediumHigh,
            "junit-import",
            None,
        ),
        record(
            "event:notebook:finished",
            "trace:notebook:run",
            3,
            NotebookRun,
            "target:checkout:notebook",
            TaskFinished,
            Lifecycle,
            Native,
            High,
            "aureline-notebook",
            None,
        ),
        // Debug session trace: lifecycle records tagged as debug payloads.
        record(
            "event:debug:started",
            "trace:debug:session",
            1,
            DebugSession,
            "target:checkout:debug",
            TaskStarted,
            Debug,
            Native,
            High,
            "aureline-debug",
            None,
        ),
        record(
            "event:debug:finished",
            "trace:debug:session",
            2,
            DebugSession,
            "target:checkout:debug",
            TaskFinished,
            Debug,
            Native,
            High,
            "aureline-debug",
            None,
        ),
        // Pipeline trace: native diagnostic truth, a heuristic shadow, and a
        // Bazel BEP artifact carried by reference.
        record(
            "event:pipeline:diagnostic",
            "trace:pipeline:run",
            1,
            Pipeline,
            "target:checkout:pipeline",
            DiagnosticEmitted,
            Diagnostic,
            Native,
            High,
            "aureline-pipeline",
            None,
        ),
        record(
            "event:pipeline:diagnostic-shadow",
            "trace:pipeline:run",
            2,
            Pipeline,
            "target:checkout:pipeline",
            DiagnosticEmitted,
            Diagnostic,
            HeuristicParser,
            BuildTestEventConfidence::Low,
            "problem-matcher",
            Some(DowngradeReason::HeuristicFallback),
        ),
        record(
            "event:pipeline:artifact",
            "trace:pipeline:run",
            3,
            Pipeline,
            "target:checkout:pipeline",
            ArtifactPublished,
            Artifact,
            BazelBep,
            High,
            "bazel-bep",
            None,
        ),
    ]
}

fn canonical_surface_projections() -> Vec<TaskEventSurfaceProjection> {
    TaskEventSurface::ALL
        .into_iter()
        .map(|surface| TaskEventSurfaceProjection {
            surface,
            projection_ref: format!(
                "projection:tooling:m5:task-event-first-consumers:{}",
                surface.as_str()
            ),
            reads_canonical_envelope: true,
            preserves_event_id: true,
            preserves_source_kind: true,
            preserves_priority_rank: true,
            preserves_confidence: true,
            preserves_payload_kind: true,
            preserves_downgrade_disclosure: true,
            preserves_raw_payload_ref: true,
            preserves_provenance: true,
            // Every surface can explain truth; export surfaces are required to.
            explains_source_and_confidence: true,
            // Overwritten by `derive_projection_counts` at materialization.
            observed_event_count: 0,
        })
        .collect()
}
