//! Cross-surface event reuse: one shared execution history that the task center,
//! test trees, coverage/flaky/snapshot intelligence, pipeline overlays, notebook
//! runs, incident runbooks, and the CLI/headless and support exports all read,
//! plus the reopen / export / rerun-review / evidence-link flows that point every
//! surface back to the same authoritative event objects.
//!
//! The canonical event envelope and first-consumer bus
//! ([`crate::m5_task_event_envelope_bus`]) land one [`TaskEventRecord`] family and
//! prove the first set of emitters and exporters read it instead of rendered
//! logs. This module closes the reuse contract those records exist for: it proves
//! that every *major* M5 execution consumer reuses that one history — with stable
//! event/trace identity and provenance preserved — rather than forking a private,
//! incompatible session history per surface. It deliberately reuses the canonical
//! [`TaskEventRecord`] verbatim (its shared history is literally the
//! first-consumers record history) and the same source-kind, confidence, and
//! provenance vocabulary; it adds exactly two things beyond the envelope: a
//! [`ConsumerBinding`] per claimed consumer surface and a [`CrossSurfaceFlow`] per
//! reopen / export / rerun-review / evidence-link hop.
//!
//! Four invariants keep the reuse model trustworthy:
//!
//! - **One shared history, not one per surface.** Every claimed consumer binds a
//!   projection that reads the shared canonical objects, never reconstructs its
//!   own history from rendered logs, and only references trace ids that exist in
//!   the shared history.
//! - **Stable ids and provenance survive every surface boundary.** A consumer
//!   that rewrites event/trace ids, drops provenance, or hides source/confidence
//!   blocks stable, and a cross-surface flow that fails to preserve ids or
//!   provenance across the hop blocks stable.
//! - **Reopen and export flows resolve to the same authoritative object.** Every
//!   flow names an authoritative event/trace pair, and that pair must resolve to
//!   exactly one event in the shared history whose trace agrees — so opening,
//!   exporting, reviewing a rerun, or linking evidence from any surface lands on
//!   the same authoritative truth.
//! - **Every required surface and flow kind is present.** A missing consumer
//!   binding or a missing flow kind blocks stable, so the reuse contract cannot
//!   silently shrink.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/cross-surface-event-reuse.md`](../../../docs/m5/cross-surface-event-reuse.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/cross-surface-event-reuse.schema.json`](../../../schemas/tooling/cross-surface-event-reuse.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    BuildTestEventSourceKind, BuildTestInteropFindingSeverity, BuildTestInteropPromotionState,
};
use crate::m5_task_event_adapter_policy::canonical_priority_rank;
use crate::m5_task_event_envelope_bus::{
    current_stable_task_event_first_consumers_input, TaskEventRecord,
};

/// Stable record-kind tag for [`CrossSurfaceEventReusePacket`].
pub const CROSS_SURFACE_EVENT_REUSE_RECORD_KIND: &str = "m5_cross_surface_event_reuse_packet";

/// Stable record-kind tag for [`CrossSurfaceEventReuseSupportExport`].
pub const CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_cross_surface_event_reuse_support_export";

/// Stable record-kind tag for [`CrossSurfaceEvidenceJoinView`].
pub const CROSS_SURFACE_EVENT_REUSE_EVIDENCE_JOIN_RECORD_KIND: &str =
    "m5_cross_surface_event_reuse_evidence_join";

/// Stable record-kind tag for [`CrossSurfaceCliHeadlessView`].
pub const CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_cross_surface_event_reuse_cli_headless";

/// Integer schema version for the cross-surface event-reuse packet.
pub const CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the cross-surface event-reuse boundary schema.
pub const CROSS_SURFACE_EVENT_REUSE_SCHEMA_REF: &str =
    "schemas/tooling/cross-surface-event-reuse.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const CROSS_SURFACE_EVENT_REUSE_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const CROSS_SURFACE_EVENT_REUSE_DOC_REF: &str = "docs/m5/cross-surface-event-reuse.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const CROSS_SURFACE_EVENT_REUSE_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the first-consumers packet whose history this packet reuses.
pub const CROSS_SURFACE_EVENT_REUSE_FIRST_CONSUMERS_PACKET_REF: &str =
    "artifacts/m5/tooling/event-envelope-first-consumers/packet.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR: &str = "fixtures/tooling/m5/consumer-parity";

/// Repo-relative path of the checked-in packet artifact.
pub const CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/cross-surface-event-reuse/packet.json";

/// Stable packet id minted by the seed.
pub const CROSS_SURFACE_EVENT_REUSE_ID: &str = "tooling:m5:cross-surface-event-reuse:v1";

/// Stable support-export id minted by the seed inspector.
pub const CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:cross-surface-event-reuse";

/// Stable AI evidence join id minted by the seed inspector.
pub const CROSS_SURFACE_EVENT_REUSE_AI_EVIDENCE_ID: &str =
    "ai-evidence:tooling:m5:cross-surface-event-reuse";

/// Stable incident packet join id minted by the seed inspector.
pub const CROSS_SURFACE_EVENT_REUSE_INCIDENT_PACKET_ID: &str =
    "incident:tooling:m5:cross-surface-event-reuse";

/// Stable CLI/headless view id minted by the seed inspector.
pub const CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID: &str =
    "cli-headless:tooling:m5:cross-surface-event-reuse";

/// A major M5 execution consumer that reuses the shared event history.
///
/// These are the surfaces the cross-surface reuse contract joins. The five
/// emitting lanes from the bus appear here as consumers (task center, test trees,
/// pipeline overlays, notebook runs), alongside the surfaces that the first
/// consumer bus did not enumerate but that the reuse contract must also bind:
/// coverage/flaky/snapshot intelligence and incident runbooks. The two export
/// surfaces — CLI/headless and support — round out the set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Task center timeline and task headers.
    TaskCenter,
    /// Test explorer tree and inline results.
    TestTree,
    /// Coverage, flaky, and snapshot/golden intelligence panels.
    CoverageFlakySnapshot,
    /// Pipeline / run-control overlays.
    PipelineOverlay,
    /// Notebook run history.
    NotebookRun,
    /// Incident runbooks and timelines.
    IncidentRunbook,
    /// CLI / headless stable JSON export.
    CliHeadlessExport,
    /// Support and release export bundle.
    SupportExport,
}

impl ConsumerSurface {
    /// Every claimed consumer surface in stable declaration order.
    pub const ALL: [Self; 8] = [
        Self::TaskCenter,
        Self::TestTree,
        Self::CoverageFlakySnapshot,
        Self::PipelineOverlay,
        Self::NotebookRun,
        Self::IncidentRunbook,
        Self::CliHeadlessExport,
        Self::SupportExport,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskCenter => "task_center",
            Self::TestTree => "test_tree",
            Self::CoverageFlakySnapshot => "coverage_flaky_snapshot",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::NotebookRun => "notebook_run",
            Self::IncidentRunbook => "incident_runbook",
            Self::CliHeadlessExport => "cli_headless_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// A cross-surface flow that points one surface back to an authoritative object.
///
/// Each flow models a user action that crosses a surface boundary and must land
/// on the same authoritative event/session object regardless of where it started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossSurfaceFlowKind {
    /// Reopen the same execution history from a different surface.
    Reopen,
    /// Export the same execution history through CLI/headless or support.
    Export,
    /// Review a rerun against the same authoritative attempt.
    RerunReview,
    /// Link evidence (incident, AI, review) to the same authoritative event.
    EvidenceLink,
}

impl CrossSurfaceFlowKind {
    /// Every required cross-surface flow kind in stable declaration order.
    pub const ALL: [Self; 4] = [
        Self::Reopen,
        Self::Export,
        Self::RerunReview,
        Self::EvidenceLink,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reopen => "reopen",
            Self::Export => "export",
            Self::RerunReview => "rerun_review",
            Self::EvidenceLink => "evidence_link",
        }
    }
}

/// Evidence-join surface that explains the shared history across a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReuseEvidenceSurface {
    /// Support bundle / support export.
    SupportBundle,
    /// Incident timeline packet.
    IncidentPacket,
    /// AI evidence packet.
    AiEvidence,
}

impl ReuseEvidenceSurface {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

/// One consumer binding proving a surface reuses the shared execution history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerBinding {
    /// Consumer surface.
    pub surface: ConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Shared-history trace ids this surface reads (must exist in the history).
    #[serde(default)]
    pub bound_trace_ids: Vec<String>,
    /// True when the surface reads the shared canonical objects.
    pub reads_shared_history: bool,
    /// True when the surface reconstructs history from rendered logs (forbidden).
    pub reconstructs_from_logs: bool,
    /// True when the surface preserves stable event/trace ids (no local rewrite).
    pub preserves_stable_ids: bool,
    /// True when the surface preserves provenance.
    pub preserves_provenance: bool,
    /// True when the surface keeps source kind and confidence visible.
    pub preserves_source_and_confidence: bool,
    /// Count of shared events this surface observes (derived from bound traces).
    pub observed_event_count: usize,
}

/// One cross-surface flow pointing a surface back to an authoritative object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceFlow {
    /// Flow kind.
    pub flow_kind: CrossSurfaceFlowKind,
    /// Stable flow ref.
    pub flow_ref: String,
    /// Surface the flow starts from.
    pub origin_surface: ConsumerSurface,
    /// Surface the flow lands on.
    pub target_surface: ConsumerSurface,
    /// Authoritative trace id the flow resolves to.
    pub authoritative_trace_id: String,
    /// Authoritative event id the flow resolves to.
    pub authoritative_event_id: String,
    /// True when the same stable ids survive the hop.
    pub preserves_stable_ids: bool,
    /// True when provenance survives the hop.
    pub preserves_provenance: bool,
}

/// Closed validation finding vocabulary for the reuse packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossSurfaceFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The packet carries no shared history.
    NoSharedHistory,
    /// Stored shared-history digest disagrees with the derived digest.
    SharedHistoryDigestDrift,
    /// A shared event has incomplete identity.
    EventIdentityIncomplete,
    /// A shared event's priority rank disagrees with its source kind.
    EventPriorityMismatch,
    /// Two shared events share an event id.
    DuplicateEventId,
    /// Two shared events in one trace share a sequence number.
    ReplaySequenceCollision,
    /// A required consumer-surface binding is absent.
    ConsumerBindingMissing,
    /// A consumer binding has no ref or no bound traces.
    ConsumerBindingMalformed,
    /// A consumer reconstructs its own history from rendered logs.
    ConsumerReconstructsFromLogs,
    /// A consumer forks a private history instead of reading the shared one.
    ConsumerForksHistory,
    /// A consumer rewrites stable event/trace ids.
    ConsumerRewritesStableIds,
    /// A consumer drops provenance.
    ConsumerDropsProvenance,
    /// A consumer hides source kind or confidence.
    ConsumerDropsSourceConfidence,
    /// A consumer binds a trace id that is not in the shared history.
    BindingTraceUnknown,
    /// A consumer's observed event count disagrees with the derivation.
    BindingCountDrift,
    /// A required cross-surface flow kind is absent.
    FlowKindMissing,
    /// A cross-surface flow has no ref.
    FlowMalformed,
    /// A flow's authoritative object is not in the shared history.
    FlowTargetMissing,
    /// A flow's authoritative trace disagrees with the resolved event's trace.
    FlowTraceMismatch,
    /// A flow names an origin or target surface with no consumer binding.
    FlowSurfaceUnbound,
    /// A flow rewrites stable ids across the surface boundary.
    FlowRewritesStableIds,
    /// A flow drops provenance across the surface boundary.
    FlowDropsProvenance,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl CrossSurfaceFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::NoSharedHistory => "no_shared_history",
            Self::SharedHistoryDigestDrift => "shared_history_digest_drift",
            Self::EventIdentityIncomplete => "event_identity_incomplete",
            Self::EventPriorityMismatch => "event_priority_mismatch",
            Self::DuplicateEventId => "duplicate_event_id",
            Self::ReplaySequenceCollision => "replay_sequence_collision",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
            Self::ConsumerBindingMalformed => "consumer_binding_malformed",
            Self::ConsumerReconstructsFromLogs => "consumer_reconstructs_from_logs",
            Self::ConsumerForksHistory => "consumer_forks_history",
            Self::ConsumerRewritesStableIds => "consumer_rewrites_stable_ids",
            Self::ConsumerDropsProvenance => "consumer_drops_provenance",
            Self::ConsumerDropsSourceConfidence => "consumer_drops_source_confidence",
            Self::BindingTraceUnknown => "binding_trace_unknown",
            Self::BindingCountDrift => "binding_count_drift",
            Self::FlowKindMissing => "flow_kind_missing",
            Self::FlowMalformed => "flow_malformed",
            Self::FlowTargetMissing => "flow_target_missing",
            Self::FlowTraceMismatch => "flow_trace_mismatch",
            Self::FlowSurfaceUnbound => "flow_surface_unbound",
            Self::FlowRewritesStableIds => "flow_rewrites_stable_ids",
            Self::FlowDropsProvenance => "flow_drops_provenance",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CrossSurfaceFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl CrossSurfaceValidationFinding {
    fn blocker(finding_kind: CrossSurfaceFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`CrossSurfaceEventReusePacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceEventReusePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// The one shared execution history (reused verbatim from the canonical bus).
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Consumer-surface bindings (observed counts derived at materialization).
    #[serde(default)]
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Cross-surface flows.
    #[serde(default)]
    pub cross_surface_flows: Vec<CrossSurfaceFlow>,
}

/// Canonical cross-surface event-reuse packet: the shared history, the consumer
/// bindings that prove every surface reuses it, and the cross-surface flows that
/// point each surface back to the same authoritative objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceEventReusePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Cross-surface reuse boundary schema ref.
    pub reuse_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// First-consumers packet whose history this packet reuses.
    pub first_consumers_packet_ref: String,
    /// The one shared execution history.
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Order-invariant digest of the shared history.
    pub shared_history_digest: String,
    /// Consumer-surface bindings.
    #[serde(default)]
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Cross-surface flows.
    #[serde(default)]
    pub cross_surface_flows: Vec<CrossSurfaceFlow>,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CrossSurfaceValidationFinding>,
}

impl CrossSurfaceEventReusePacket {
    /// Materializes a packet, deriving observed counts and the shared-history
    /// digest, then records validation findings and the derived promotion state.
    pub fn materialize(input: CrossSurfaceEventReusePacketInput) -> Self {
        let events = input.events;
        let consumer_bindings = derive_binding_counts(input.consumer_bindings, &events);
        let shared_history_digest = shared_history_digest(&events);

        let mut packet = Self {
            record_kind: CROSS_SURFACE_EVENT_REUSE_RECORD_KIND.to_owned(),
            schema_version: CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            reuse_schema_ref: CROSS_SURFACE_EVENT_REUSE_SCHEMA_REF.to_owned(),
            envelope_schema_ref: CROSS_SURFACE_EVENT_REUSE_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: CROSS_SURFACE_EVENT_REUSE_DOC_REF.to_owned(),
            policy_baseline_ref: CROSS_SURFACE_EVENT_REUSE_POLICY_BASELINE_REF.to_owned(),
            first_consumers_packet_ref: CROSS_SURFACE_EVENT_REUSE_FIRST_CONSUMERS_PACKET_REF
                .to_owned(),
            events,
            shared_history_digest,
            consumer_bindings,
            cross_surface_flows: input.cross_surface_flows,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the frozen invariants.
    pub fn validate(&self) -> Vec<CrossSurfaceValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Returns the shared history in deterministic replay order.
    pub fn replay_ordered(&self) -> Vec<&TaskEventRecord> {
        let mut ordered: Vec<&TaskEventRecord> = self.events.iter().collect();
        ordered.sort_by(|a, b| order_key(a).cmp(&order_key(b)));
        ordered
    }

    /// Returns the shared event with the given id, if any.
    pub fn event_for(&self, event_id: &str) -> Option<&TaskEventRecord> {
        self.events.iter().find(|event| event.event_id == event_id)
    }

    /// Builds an evidence join for one export/evidence surface, presenting the
    /// shared history and the cross-surface flows that point back to it.
    pub fn evidence_join(
        &self,
        surface: ReuseEvidenceSurface,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> CrossSurfaceEvidenceJoinView {
        let shared_event_rows = self
            .replay_ordered()
            .into_iter()
            .map(SharedEventRow::from_record)
            .collect();
        let flow_rows = self
            .cross_surface_flows
            .iter()
            .map(|flow| {
                CrossSurfaceFlowRow::from_flow(flow, self.event_for(&flow.authoritative_event_id))
            })
            .collect();
        CrossSurfaceEvidenceJoinView {
            record_kind: CROSS_SURFACE_EVENT_REUSE_EVIDENCE_JOIN_RECORD_KIND.to_owned(),
            schema_version: CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION,
            view_id: view_id.into(),
            surface,
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            shared_history_digest: self.shared_history_digest.clone(),
            shared_event_rows,
            flow_rows,
        }
    }

    /// Builds the CLI/headless stable view of the reuse contract.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> CrossSurfaceCliHeadlessView {
        let binding_rows = self
            .consumer_bindings
            .iter()
            .map(ConsumerBindingRow::from_binding)
            .collect();
        let flow_rows = self
            .cross_surface_flows
            .iter()
            .map(|flow| {
                CrossSurfaceFlowRow::from_flow(flow, self.event_for(&flow.authoritative_event_id))
            })
            .collect();
        CrossSurfaceCliHeadlessView {
            record_kind: CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            shared_history_digest: self.shared_history_digest.clone(),
            binding_rows,
            flow_rows,
        }
    }

    /// Builds an export-safe support bundle carrying the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> CrossSurfaceEventReuseSupportExport {
        CrossSurfaceEventReuseSupportExport {
            record_kind: CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            packet: self.clone(),
        }
    }

    /// Returns the consumer-surface tokens present in the bindings.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for binding in &self.consumer_bindings {
            set.insert(binding.surface);
        }
        set.into_iter().map(ConsumerSurface::as_str).collect()
    }

    /// Returns the flow-kind tokens present in the flows.
    pub fn flow_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for flow in &self.cross_surface_flows {
            set.insert(flow.flow_kind);
        }
        set.into_iter().map(CrossSurfaceFlowKind::as_str).collect()
    }

    /// Returns the source-kind tokens present in the shared history.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for event in &self.events {
            set.insert(event.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet {} schema_version={} promotion={} events={} bindings={} flows={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.events.len(),
            self.consumer_bindings.len(),
            self.cross_surface_flows.len(),
            self.shared_history_digest,
        ));
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} reads_shared={} from_logs={} stable_ids={} provenance={} observed={}",
                binding.surface.as_str(),
                binding.reads_shared_history,
                binding.reconstructs_from_logs,
                binding.preserves_stable_ids,
                binding.preserves_provenance,
                binding.observed_event_count,
            ));
        }
        for flow in &self.cross_surface_flows {
            lines.push(format!(
                "flow {} {}->{} event={} trace={} stable_ids={} provenance={}",
                flow.flow_kind.as_str(),
                flow.origin_surface.as_str(),
                flow.target_surface.as_str(),
                flow.authoritative_event_id,
                flow.authoritative_trace_id,
                flow.preserves_stable_ids,
                flow.preserves_provenance,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<CrossSurfaceValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != CROSS_SURFACE_EVENT_REUSE_RECORD_KIND {
            findings.push(CrossSurfaceValidationFinding::blocker(
                CrossSurfaceFindingKind::WrongRecordKind,
                "packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION
        {
            findings.push(CrossSurfaceValidationFinding::blocker(
                CrossSurfaceFindingKind::WrongSchemaVersion,
                "packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(CrossSurfaceValidationFinding::blocker(
                CrossSurfaceFindingKind::MissingIdentity,
                "packet id and timestamp are required",
            ));
        }
        for (label, value) in [
            ("reuse schema", self.reuse_schema_ref.as_str()),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
            ("policy baseline", self.policy_baseline_ref.as_str()),
            (
                "first-consumers packet",
                self.first_consumers_packet_ref.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        let shared_event_ids = self.check_events(&mut findings);
        self.check_bindings(&mut findings, &shared_event_ids, include_record_fields);
        self.check_flows(&mut findings, &shared_event_ids);

        if include_record_fields {
            let expected_digest = shared_history_digest(&self.events);
            if self.shared_history_digest != expected_digest {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::SharedHistoryDigestDrift,
                    "stored shared-history digest does not match the history",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::PromotionStateMismatch,
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

    /// Validates the shared event history and returns the set of event ids and
    /// the set of trace ids present.
    fn check_events(
        &self,
        findings: &mut Vec<CrossSurfaceValidationFinding>,
    ) -> SharedHistoryIndex {
        let mut index = SharedHistoryIndex::default();
        if self.events.is_empty() {
            findings.push(CrossSurfaceValidationFinding::blocker(
                CrossSurfaceFindingKind::NoSharedHistory,
                "packet carries no shared history",
            ));
            return index;
        }
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let mut seen_trace_seq: BTreeSet<(&str, u64)> = BTreeSet::new();
        for event in &self.events {
            if event.event_id.trim().is_empty()
                || event.trace_id.trim().is_empty()
                || event.workspace_id.trim().is_empty()
                || event.target_id.trim().is_empty()
            {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::EventIdentityIncomplete,
                    format!("shared event {} has incomplete identity", event.event_id),
                ));
            }
            if event.priority_rank != canonical_priority_rank(event.source_kind) {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::EventPriorityMismatch,
                    format!(
                        "shared event {} carries a priority rank that disagrees with {}",
                        event.event_id,
                        event.source_kind.as_str()
                    ),
                ));
            }
            if !event.event_id.trim().is_empty() && !seen_ids.insert(event.event_id.as_str()) {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::DuplicateEventId,
                    format!("shared event id {} is not unique", event.event_id),
                ));
            }
            if !seen_trace_seq.insert((event.trace_id.as_str(), event.sequence)) {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ReplaySequenceCollision,
                    format!(
                        "trace {} reuses sequence {} so replay order is ambiguous",
                        event.trace_id, event.sequence
                    ),
                ));
            }
            index
                .event_trace
                .insert(event.event_id.clone(), event.trace_id.clone());
            index.trace_ids.insert(event.trace_id.clone());
        }
        index
    }

    fn check_bindings(
        &self,
        findings: &mut Vec<CrossSurfaceValidationFinding>,
        index: &SharedHistoryIndex,
        include_record_fields: bool,
    ) {
        let present: BTreeSet<ConsumerSurface> = self
            .consumer_bindings
            .iter()
            .map(|binding| binding.surface)
            .collect();
        for surface in ConsumerSurface::ALL {
            if !present.contains(&surface) {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerBindingMissing,
                    format!("consumer binding is missing for {}", surface.as_str()),
                ));
            }
        }
        for binding in &self.consumer_bindings {
            let surface = binding.surface.as_str();
            if binding.binding_ref.trim().is_empty() || binding.bound_trace_ids.is_empty() {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerBindingMalformed,
                    format!("{surface} binding has no ref or no bound traces"),
                ));
            }
            if binding.reconstructs_from_logs {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerReconstructsFromLogs,
                    format!("{surface} reconstructs its own history from rendered logs"),
                ));
            }
            if !binding.reads_shared_history {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerForksHistory,
                    format!("{surface} forks a private history instead of the shared one"),
                ));
            }
            if !binding.preserves_stable_ids {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerRewritesStableIds,
                    format!("{surface} rewrites stable event/trace ids"),
                ));
            }
            if !binding.preserves_provenance {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerDropsProvenance,
                    format!("{surface} drops provenance"),
                ));
            }
            if !binding.preserves_source_and_confidence {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::ConsumerDropsSourceConfidence,
                    format!("{surface} hides source kind or confidence"),
                ));
            }
            for trace_id in &binding.bound_trace_ids {
                if !index.trace_ids.contains(trace_id) {
                    findings.push(CrossSurfaceValidationFinding::blocker(
                        CrossSurfaceFindingKind::BindingTraceUnknown,
                        format!("{surface} binds trace {trace_id} not in the shared history"),
                    ));
                }
            }
            if include_record_fields {
                let expected = observed_event_count(&binding.bound_trace_ids, &self.events);
                if binding.observed_event_count != expected {
                    findings.push(CrossSurfaceValidationFinding::blocker(
                        CrossSurfaceFindingKind::BindingCountDrift,
                        format!(
                            "{surface} observed count {} disagrees with the shared history ({expected})",
                            binding.observed_event_count
                        ),
                    ));
                }
            }
        }
    }

    fn check_flows(
        &self,
        findings: &mut Vec<CrossSurfaceValidationFinding>,
        index: &SharedHistoryIndex,
    ) {
        let bound: BTreeSet<ConsumerSurface> = self
            .consumer_bindings
            .iter()
            .map(|binding| binding.surface)
            .collect();
        let present: BTreeSet<CrossSurfaceFlowKind> = self
            .cross_surface_flows
            .iter()
            .map(|flow| flow.flow_kind)
            .collect();
        for kind in CrossSurfaceFlowKind::ALL {
            if !present.contains(&kind) {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::FlowKindMissing,
                    format!("cross-surface flow kind {} is missing", kind.as_str()),
                ));
            }
        }
        for flow in &self.cross_surface_flows {
            let label = flow.flow_kind.as_str();
            if flow.flow_ref.trim().is_empty() {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::FlowMalformed,
                    format!("{label} flow has no ref"),
                ));
            }
            match index.event_trace.get(&flow.authoritative_event_id) {
                None => findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::FlowTargetMissing,
                    format!(
                        "{label} flow points at {} which is not in the shared history",
                        flow.authoritative_event_id
                    ),
                )),
                Some(trace_id) if trace_id != &flow.authoritative_trace_id => {
                    findings.push(CrossSurfaceValidationFinding::blocker(
                        CrossSurfaceFindingKind::FlowTraceMismatch,
                        format!(
                            "{label} flow names trace {} but {} belongs to {}",
                            flow.authoritative_trace_id, flow.authoritative_event_id, trace_id
                        ),
                    ));
                }
                Some(_) => {}
            }
            for surface in [flow.origin_surface, flow.target_surface] {
                if !bound.contains(&surface) {
                    findings.push(CrossSurfaceValidationFinding::blocker(
                        CrossSurfaceFindingKind::FlowSurfaceUnbound,
                        format!("{label} flow names unbound surface {}", surface.as_str()),
                    ));
                }
            }
            if !flow.preserves_stable_ids {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::FlowRewritesStableIds,
                    format!("{label} flow rewrites stable ids across the surface boundary"),
                ));
            }
            if !flow.preserves_provenance {
                findings.push(CrossSurfaceValidationFinding::blocker(
                    CrossSurfaceFindingKind::FlowDropsProvenance,
                    format!("{label} flow drops provenance across the surface boundary"),
                ));
            }
        }
    }
}

/// Derived index of the shared history used by the binding and flow checks.
#[derive(Debug, Default)]
struct SharedHistoryIndex {
    /// Map from event id to its trace id.
    event_trace: BTreeMap<String, String>,
    /// Set of trace ids present in the shared history.
    trace_ids: BTreeSet<String>,
}

/// Support-export wrapper carrying the exact reuse packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceEventReuseSupportExport {
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
    pub packet: CrossSurfaceEventReusePacket,
}

impl CrossSurfaceEventReuseSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.packet.is_stable()
    }
}

/// One shared-history row in an evidence join, with provenance preserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedEventRow {
    /// Event id.
    pub event_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Ordering position within the trace.
    pub sequence: u64,
    /// Producer lane token.
    pub producer_lane: String,
    /// Source kind token.
    pub source_kind: String,
    /// Confidence token.
    pub confidence: String,
    /// True when the event is visibly downgraded.
    pub downgraded: bool,
    /// Adapter id from provenance (provenance is never flattened away).
    pub adapter_id: String,
    /// Support-safe explanation derived from canonical fields.
    pub explanation: String,
}

impl SharedEventRow {
    fn from_record(record: &TaskEventRecord) -> Self {
        Self {
            event_id: record.event_id.clone(),
            trace_id: record.trace_id.clone(),
            sequence: record.sequence,
            producer_lane: record.producer_lane.as_str().to_owned(),
            source_kind: record.source_kind.as_str().to_owned(),
            confidence: record.confidence.as_str().to_owned(),
            downgraded: record.downgraded,
            adapter_id: record.provenance.adapter_id.clone(),
            explanation: record.explain(),
        }
    }
}

/// One cross-surface flow row, with its resolution against the shared history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceFlowRow {
    /// Flow kind token.
    pub flow_kind: String,
    /// Stable flow ref.
    pub flow_ref: String,
    /// Origin surface token.
    pub origin_surface: String,
    /// Target surface token.
    pub target_surface: String,
    /// Authoritative trace id the flow resolves to.
    pub authoritative_trace_id: String,
    /// Authoritative event id the flow resolves to.
    pub authoritative_event_id: String,
    /// True when the authoritative object resolves to one shared event whose
    /// trace agrees.
    pub resolves_to_shared_object: bool,
    /// True when stable ids survive the hop.
    pub preserves_stable_ids: bool,
    /// True when provenance survives the hop.
    pub preserves_provenance: bool,
    /// Support-safe note describing where the flow lands.
    pub note: String,
}

impl CrossSurfaceFlowRow {
    fn from_flow(flow: &CrossSurfaceFlow, resolved: Option<&TaskEventRecord>) -> Self {
        let resolves_to_shared_object = resolved
            .map(|event| event.trace_id == flow.authoritative_trace_id)
            .unwrap_or(false);
        let note = if resolves_to_shared_object {
            format!(
                "{} from {} lands on shared event {} in trace {}",
                flow.flow_kind.as_str(),
                flow.origin_surface.as_str(),
                flow.authoritative_event_id,
                flow.authoritative_trace_id,
            )
        } else {
            format!(
                "{} from {} does not resolve to a shared event",
                flow.flow_kind.as_str(),
                flow.origin_surface.as_str(),
            )
        };
        Self {
            flow_kind: flow.flow_kind.as_str().to_owned(),
            flow_ref: flow.flow_ref.clone(),
            origin_surface: flow.origin_surface.as_str().to_owned(),
            target_surface: flow.target_surface.as_str().to_owned(),
            authoritative_trace_id: flow.authoritative_trace_id.clone(),
            authoritative_event_id: flow.authoritative_event_id.clone(),
            resolves_to_shared_object,
            preserves_stable_ids: flow.preserves_stable_ids,
            preserves_provenance: flow.preserves_provenance,
            note,
        }
    }
}

/// Evidence-join view for one export/evidence surface (support, incident, or AI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceEvidenceJoinView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Evidence surface this view serves.
    pub surface: ReuseEvidenceSurface,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the shared history.
    pub shared_history_digest: String,
    /// Shared-history rows in deterministic replay order.
    #[serde(default)]
    pub shared_event_rows: Vec<SharedEventRow>,
    /// Cross-surface flow rows.
    #[serde(default)]
    pub flow_rows: Vec<CrossSurfaceFlowRow>,
}

impl CrossSurfaceEvidenceJoinView {
    /// Returns true when every flow resolves to a shared object and every shared
    /// row keeps its provenance and explanation.
    pub fn explains_consistently(&self) -> bool {
        let flows_ok = self
            .flow_rows
            .iter()
            .all(|row| row.resolves_to_shared_object && row.preserves_stable_ids);
        let rows_ok = self.shared_event_rows.iter().all(|row| {
            !row.source_kind.trim().is_empty()
                && !row.adapter_id.trim().is_empty()
                && !row.explanation.trim().is_empty()
        });
        flows_ok && rows_ok
    }
}

/// One CLI/headless consumer-binding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerBindingRow {
    /// Consumer surface token.
    pub surface: String,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Shared-history trace ids this surface reads.
    #[serde(default)]
    pub bound_trace_ids: Vec<String>,
    /// Count of shared events observed.
    pub observed_event_count: usize,
    /// True when the surface reads the shared canonical objects.
    pub reads_shared_history: bool,
    /// True when the surface preserves stable event/trace ids.
    pub preserves_stable_ids: bool,
    /// True when the surface preserves provenance.
    pub preserves_provenance: bool,
    /// Support-safe explanation of the binding.
    pub explanation: String,
}

impl ConsumerBindingRow {
    fn from_binding(binding: &ConsumerBinding) -> Self {
        let explanation = format!(
            "{} reuses the shared history across {} trace(s) ({} event(s)); reads_shared={} stable_ids={} provenance={}",
            binding.surface.as_str(),
            binding.bound_trace_ids.len(),
            binding.observed_event_count,
            binding.reads_shared_history,
            binding.preserves_stable_ids,
            binding.preserves_provenance,
        );
        Self {
            surface: binding.surface.as_str().to_owned(),
            binding_ref: binding.binding_ref.clone(),
            bound_trace_ids: binding.bound_trace_ids.clone(),
            observed_event_count: binding.observed_event_count,
            reads_shared_history: binding.reads_shared_history,
            preserves_stable_ids: binding.preserves_stable_ids,
            preserves_provenance: binding.preserves_provenance,
            explanation,
        }
    }
}

/// CLI/headless stable view of the reuse contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceCliHeadlessView {
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
    /// Order-invariant digest of the shared history.
    pub shared_history_digest: String,
    /// Consumer-binding rows.
    #[serde(default)]
    pub binding_rows: Vec<ConsumerBindingRow>,
    /// Cross-surface flow rows.
    #[serde(default)]
    pub flow_rows: Vec<CrossSurfaceFlowRow>,
}

impl CrossSurfaceCliHeadlessView {
    /// Returns true when every binding row reads the shared history and explains
    /// itself.
    pub fn every_binding_reuses(&self) -> bool {
        self.binding_rows
            .iter()
            .all(|row| row.reads_shared_history && !row.explanation.trim().is_empty())
    }
}

/// Deterministic ordering key for replay.
fn order_key(record: &TaskEventRecord) -> (&str, u64, &str) {
    (
        record.trace_id.as_str(),
        record.sequence,
        record.event_id.as_str(),
    )
}

/// Count of shared events whose trace id is in `bound_trace_ids`.
fn observed_event_count(bound_trace_ids: &[String], events: &[TaskEventRecord]) -> usize {
    let bound: BTreeSet<&str> = bound_trace_ids.iter().map(String::as_str).collect();
    events
        .iter()
        .filter(|event| bound.contains(event.trace_id.as_str()))
        .count()
}

fn derive_binding_counts(
    mut bindings: Vec<ConsumerBinding>,
    events: &[TaskEventRecord],
) -> Vec<ConsumerBinding> {
    for binding in &mut bindings {
        binding.observed_event_count = observed_event_count(&binding.bound_trace_ids, events);
    }
    bindings
}

/// Order-invariant FNV-1a 64-bit digest of the shared history's ordered ids.
fn shared_history_digest(events: &[TaskEventRecord]) -> String {
    let mut ordered: Vec<&TaskEventRecord> = events.iter().collect();
    ordered.sort_by(|a, b| order_key(a).cmp(&order_key(b)));
    let ids: Vec<&str> = ordered
        .iter()
        .map(|record| record.event_id.as_str())
        .collect();
    fnv1a64(&ids)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of event ids.
fn fnv1a64(event_ids_in_order: &[&str]) -> String {
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
    findings: &[CrossSurfaceValidationFinding],
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

/// Builds the canonical stable cross-surface event-reuse packet input.
///
/// The shared history is the canonical first-consumers record history, reused
/// verbatim, so the reuse contract literally binds the same objects every other
/// lane emits and reads.
pub fn current_stable_cross_surface_event_reuse_input() -> CrossSurfaceEventReusePacketInput {
    CrossSurfaceEventReusePacketInput {
        packet_id: CROSS_SURFACE_EVENT_REUSE_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        events: current_stable_task_event_first_consumers_input().events,
        consumer_bindings: canonical_consumer_bindings(),
        cross_surface_flows: canonical_cross_surface_flows(),
    }
}

/// Materializes the canonical stable cross-surface event-reuse packet.
pub fn seeded_cross_surface_event_reuse_packet() -> CrossSurfaceEventReusePacket {
    CrossSurfaceEventReusePacket::materialize(current_stable_cross_surface_event_reuse_input())
}

/// Validates a packet and returns an `Ok(())` / findings result.
pub fn validate_cross_surface_event_reuse_packet(
    packet: &CrossSurfaceEventReusePacket,
) -> Result<(), Vec<CrossSurfaceValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Shared-history trace ids (mirroring the canonical first-consumers seed).
const TRACE_TASK: &str = "trace:task:build";
const TRACE_TEST: &str = "trace:test:suite";
const TRACE_NOTEBOOK: &str = "trace:notebook:run";
const TRACE_DEBUG: &str = "trace:debug:session";
const TRACE_PIPELINE: &str = "trace:pipeline:run";

fn binding(surface: ConsumerSurface, traces: &[&str]) -> ConsumerBinding {
    ConsumerBinding {
        surface,
        binding_ref: format!(
            "binding:tooling:m5:cross-surface-event-reuse:{}",
            surface.as_str()
        ),
        bound_trace_ids: traces.iter().map(|t| (*t).to_owned()).collect(),
        reads_shared_history: true,
        reconstructs_from_logs: false,
        preserves_stable_ids: true,
        preserves_provenance: true,
        preserves_source_and_confidence: true,
        // Overwritten by `derive_binding_counts` at materialization.
        observed_event_count: 0,
    }
}

fn canonical_consumer_bindings() -> Vec<ConsumerBinding> {
    use ConsumerSurface::{
        CliHeadlessExport, CoverageFlakySnapshot, IncidentRunbook, NotebookRun, PipelineOverlay,
        SupportExport, TaskCenter, TestTree,
    };
    let all_traces = [
        TRACE_TASK,
        TRACE_TEST,
        TRACE_NOTEBOOK,
        TRACE_DEBUG,
        TRACE_PIPELINE,
    ];
    vec![
        // The task center timeline aggregates every run in the shared history.
        binding(TaskCenter, &all_traces),
        // The test tree reads the test-session and notebook test traces.
        binding(TestTree, &[TRACE_TEST, TRACE_NOTEBOOK]),
        // Coverage/flaky/snapshot intelligence derives from the same test runs.
        binding(CoverageFlakySnapshot, &[TRACE_TEST, TRACE_NOTEBOOK]),
        // Pipeline overlays read the pipeline run trace.
        binding(PipelineOverlay, &[TRACE_PIPELINE]),
        // Notebook run history reads the notebook run trace.
        binding(NotebookRun, &[TRACE_NOTEBOOK]),
        // Incident runbooks link the failing pipeline and test runs.
        binding(IncidentRunbook, &[TRACE_PIPELINE, TRACE_TEST]),
        // The CLI/headless and support exports carry the whole shared history.
        binding(CliHeadlessExport, &all_traces),
        binding(SupportExport, &all_traces),
    ]
}

fn flow(
    flow_kind: CrossSurfaceFlowKind,
    origin_surface: ConsumerSurface,
    target_surface: ConsumerSurface,
    authoritative_trace_id: &str,
    authoritative_event_id: &str,
) -> CrossSurfaceFlow {
    CrossSurfaceFlow {
        flow_kind,
        flow_ref: format!(
            "flow:tooling:m5:cross-surface-event-reuse:{}",
            flow_kind.as_str()
        ),
        origin_surface,
        target_surface,
        authoritative_trace_id: authoritative_trace_id.to_owned(),
        authoritative_event_id: authoritative_event_id.to_owned(),
        preserves_stable_ids: true,
        preserves_provenance: true,
    }
}

fn canonical_cross_surface_flows() -> Vec<CrossSurfaceFlow> {
    use ConsumerSurface::{
        CliHeadlessExport, CoverageFlakySnapshot, IncidentRunbook, PipelineOverlay, TaskCenter,
        TestTree,
    };
    use CrossSurfaceFlowKind::{EvidenceLink, Export, Reopen, RerunReview};
    vec![
        // Reopen the same test result from the task center.
        flow(
            Reopen,
            TaskCenter,
            TestTree,
            TRACE_TEST,
            "event:test:finished",
        ),
        // Export the same pipeline artifact event through the CLI/headless surface.
        flow(
            Export,
            PipelineOverlay,
            CliHeadlessExport,
            TRACE_PIPELINE,
            "event:pipeline:artifact",
        ),
        // Review a rerun against the same authoritative test attempt.
        flow(
            RerunReview,
            CoverageFlakySnapshot,
            TestTree,
            TRACE_TEST,
            "event:test:finished",
        ),
        // Link an incident runbook to the same authoritative pipeline diagnostic.
        flow(
            EvidenceLink,
            IncidentRunbook,
            PipelineOverlay,
            TRACE_PIPELINE,
            "event:pipeline:diagnostic",
        ),
    ]
}
