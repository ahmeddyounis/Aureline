//! Log, metric, trace, incident-timeline, and runbook execution evidence contract.
//!
//! This module defines the metadata-only packet that stable observability and
//! incident surfaces use to preserve slice identity, timezone-aware chronology,
//! freshness honesty, and runbook execution provenance across UI, CLI/headless,
//! support export, and incident export consumers.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`OperationalEvidenceContractPacket`].
pub const OPERATIONAL_EVIDENCE_CONTRACT_RECORD_KIND: &str =
    "log_metric_slice_and_incident_timeline_contract_packet";

/// Stable record-kind tag for [`OperationalEvidenceSupportExport`].
pub const OPERATIONAL_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "log_metric_slice_and_incident_timeline_contract_support_export";

/// Integer schema version for the operational evidence contract.
pub const OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF: &str =
    "schemas/runtime/log-metric-slice-and-incident-timeline-contract.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const OPERATIONAL_EVIDENCE_CONTRACT_DOC_REF: &str =
    "docs/runtime/m4/log-metric-slice-and-incident-timeline-contract.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const OPERATIONAL_EVIDENCE_CONTRACT_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/log-metric-slice-and-incident-timeline-contract.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/log-metric-slice-and-incident-timeline-contract";

/// Closed signal vocabulary for slices that can be pinned to incidents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    /// Textual log or event evidence.
    Log,
    /// Metric sample, aggregate, threshold, or comparison evidence.
    Metric,
    /// Trace, span-set, or distributed request correlation evidence.
    Trace,
}

impl SignalKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Metric => "metric",
            Self::Trace => "trace",
        }
    }
}

/// Evidence freshness states that stable surfaces must not collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessState {
    /// Current live runtime or provider stream.
    Live,
    /// Live-ish stream that is delayed or still draining buffers.
    Buffering,
    /// Cached snapshot produced by Aureline or a trusted helper.
    Cached,
    /// Imported artifact or external packet, not live runtime truth.
    Imported,
    /// Evidence known to be outside its current freshness window.
    Stale,
    /// Evidence is incomplete because a provider, scope, or query was limited.
    Partial,
    /// Evidence source is unavailable for live verification.
    Offline,
    /// Evidence was clipped, sampled, or truncated.
    Truncated,
    /// Exported copy that must not be treated as live or mutable.
    ExportedCopy,
}

impl EvidenceFreshnessState {
    /// Every freshness state a stable observability or incident consumer must expose.
    pub const REQUIRED_FOR_STABLE_SURFACES: [Self; 9] = [
        Self::Live,
        Self::Buffering,
        Self::Cached,
        Self::Imported,
        Self::Stale,
        Self::Partial,
        Self::Offline,
        Self::Truncated,
        Self::ExportedCopy,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Buffering => "buffering",
            Self::Cached => "cached",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Offline => "offline",
            Self::Truncated => "truncated",
            Self::ExportedCopy => "exported_copy",
        }
    }
}

/// Sampling or clipping posture for a signal slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SamplePosture {
    /// Full evidence for the declared query and time window.
    Complete,
    /// Sampled evidence with an explicit sampling posture.
    Sampled,
    /// Downsampled metric or trace evidence.
    Downsampled,
    /// Partial result caused by provider, time, permission, or scope limits.
    Partial,
    /// Truncated evidence with a visible truncation state.
    Truncated,
}

impl SamplePosture {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Sampled => "sampled",
            Self::Downsampled => "downsampled",
            Self::Partial => "partial",
            Self::Truncated => "truncated",
        }
    }
}

/// Export and redaction posture attached to evidence records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRedactionClass {
    /// Metadata can be embedded in support or incident exports.
    MetadataEmbedded,
    /// Raw payload remains by reference only.
    ByReferenceOnly,
    /// Payload has been redacted according to policy.
    RedactedPayload,
    /// Evidence is omitted and an explicit reason is required.
    OmittedWithReason,
    /// Exported copy with a declared redaction profile.
    ExportedCopy,
}

impl ExportRedactionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataEmbedded => "metadata_embedded",
            Self::ByReferenceOnly => "by_reference_only",
            Self::RedactedPayload => "redacted_payload",
            Self::OmittedWithReason => "omitted_with_reason",
            Self::ExportedCopy => "exported_copy",
        }
    }
}

/// Promotion posture for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEvidencePromotionState {
    /// All claimed stable surfaces preserve the contract.
    Stable,
    /// The packet is usable only below the stable claim.
    NarrowedBelowStable,
    /// The packet blocks stable promotion.
    BlocksStable,
}

impl OperationalEvidencePromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Claimed support class for a consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEvidenceSupportClass {
    /// Consumer claims stable support for the contract.
    Stable,
    /// Consumer is intentionally inspect-only.
    InspectOnly,
    /// Consumer is handoff-only and cannot imply live parity.
    HandoffOnly,
    /// Consumer is below stable because required evidence is missing.
    Narrowed,
}

impl OperationalEvidenceSupportClass {
    /// True when the consumer makes a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::InspectOnly => "inspect_only",
            Self::HandoffOnly => "handoff_only",
            Self::Narrowed => "narrowed",
        }
    }
}

/// Surface vocabulary for packet consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEvidenceConsumerSurface {
    /// Product observability UI.
    ObservabilityUi,
    /// Incident workspace.
    IncidentWorkspace,
    /// Runbook execution pane.
    RunbookExecution,
    /// CLI or headless packet output.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Incident or compliance evidence export.
    IncidentExport,
}

impl OperationalEvidenceConsumerSurface {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservabilityUi => "observability_ui",
            Self::IncidentWorkspace => "incident_workspace",
            Self::RunbookExecution => "runbook_execution",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::IncidentExport => "incident_export",
        }
    }
}

/// Runbook action class used by step packets and timeline rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookActionClass {
    /// Observational step.
    Observe,
    /// Verification-only step.
    Verify,
    /// Mutating mitigation step.
    Mitigate,
    /// Rollback step.
    Rollback,
    /// Communication or status-update step.
    Communicate,
}

impl RunbookActionClass {
    /// True when this action class can mutate a live target.
    pub const fn requires_approval_for_stable_completion(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Verify => "verify",
            Self::Mitigate => "mitigate",
            Self::Rollback => "rollback",
            Self::Communicate => "communicate",
        }
    }
}

/// Runbook execution outcome used by stable records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepStatus {
    /// Step has not started.
    Planned,
    /// Step is waiting on an approval or external gate.
    WaitingApproval,
    /// Step completed without mutation.
    Verified,
    /// Step executed as approved.
    Executed,
    /// Step deviated from the packet.
    Deviated,
    /// Step requires an explicit external console handoff.
    HandoffRequired,
    /// Step failed closed.
    FailedClosed,
    /// Step was rolled back.
    RolledBack,
}

impl RunbookStepStatus {
    /// True when the status requires a deviation note reference.
    pub const fn requires_deviation_note(self) -> bool {
        matches!(self, Self::Deviated)
    }

    /// True when the status requires external handoff references.
    pub const fn requires_handoff_ref(self) -> bool {
        matches!(self, Self::HandoffRequired)
    }

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::WaitingApproval => "waiting_approval",
            Self::Verified => "verified",
            Self::Executed => "executed",
            Self::Deviated => "deviated",
            Self::HandoffRequired => "handoff_required",
            Self::FailedClosed => "failed_closed",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// Timeline source/link class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineLinkClass {
    /// Link to a log, metric, or trace slice.
    SignalSlice,
    /// Link to a runtime run, request, or job.
    Run,
    /// Link to an artifact or evidence bundle.
    Artifact,
    /// Link to a provider alert, event, or console object.
    ProviderEvent,
    /// Link to a repair or mitigation transaction.
    Repair,
    /// Link to a runbook step execution.
    RunbookStep,
    /// Link to an approval ticket.
    Approval,
}

impl TimelineLinkClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignalSlice => "signal_slice",
            Self::Run => "run",
            Self::Artifact => "artifact",
            Self::ProviderEvent => "provider_event",
            Self::Repair => "repair",
            Self::RunbookStep => "runbook_step",
            Self::Approval => "approval",
        }
    }
}

/// Severity for a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEvidenceFindingSeverity {
    /// Informational finding.
    Info,
    /// Non-blocking warning.
    Warning,
    /// Finding blocks stable promotion.
    Blocker,
}

impl OperationalEvidenceFindingSeverity {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocker => "blocker",
        }
    }
}

/// Finding kinds emitted by the contract validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEvidenceFindingKind {
    /// Packet record kind is not the stable contract kind.
    WrongRecordKind,
    /// Packet schema version is not supported.
    WrongSchemaVersion,
    /// Required identity or timestamp field is empty.
    MissingIdentity,
    /// A signal slice is missing source identity, target scope, or query/window identity.
    SliceIdentityIncomplete,
    /// A signal slice has no linked incident timeline entry.
    SliceTimelineLinkMissing,
    /// Timeline event lacks timezone-aware chronology fields.
    TimelineChronologyIncomplete,
    /// Timeline event IDs are duplicated.
    DuplicateTimelineEvent,
    /// Timeline event source/evidence links are missing.
    TimelineProvenanceMissing,
    /// Timeline ordering does not match event time order.
    TimelineOutOfOrder,
    /// Runbook step execution is missing required provenance.
    RunbookExecutionProvenanceMissing,
    /// Mutating runbook step is missing approval evidence.
    RunbookMutatingStepApprovalMissing,
    /// Runbook deviation lacks a deviation note.
    RunbookDeviationNoteMissing,
    /// External console handoff lacks a handoff reference.
    RunbookHandoffRefMissing,
    /// Consumer projection does not preserve all required freshness states.
    FreshnessVocabularyCollapsed,
    /// Consumer projection does not preserve slice, chronology, or runbook provenance.
    ConsumerProjectionContinuityMissing,
    /// Export bundle omits embedded/by-reference manifest or omission summary.
    ExportContinuityMissing,
}

impl OperationalEvidenceFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::SliceIdentityIncomplete => "slice_identity_incomplete",
            Self::SliceTimelineLinkMissing => "slice_timeline_link_missing",
            Self::TimelineChronologyIncomplete => "timeline_chronology_incomplete",
            Self::DuplicateTimelineEvent => "duplicate_timeline_event",
            Self::TimelineProvenanceMissing => "timeline_provenance_missing",
            Self::TimelineOutOfOrder => "timeline_out_of_order",
            Self::RunbookExecutionProvenanceMissing => "runbook_execution_provenance_missing",
            Self::RunbookMutatingStepApprovalMissing => "runbook_mutating_step_approval_missing",
            Self::RunbookDeviationNoteMissing => "runbook_deviation_note_missing",
            Self::RunbookHandoffRefMissing => "runbook_handoff_ref_missing",
            Self::FreshnessVocabularyCollapsed => "freshness_vocabulary_collapsed",
            Self::ConsumerProjectionContinuityMissing => "consumer_projection_continuity_missing",
            Self::ExportContinuityMissing => "export_continuity_missing",
        }
    }
}

/// Source identity for a log, metric, or trace slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalSourceIdentity {
    /// Stable source identifier.
    pub source_id: String,
    /// Source class such as local process, remote agent, managed provider, mirror, or import.
    pub source_class: String,
    /// Backend or provider name.
    pub backend_ref: String,
    /// Account, project, region, namespace, or service scope.
    pub provider_scope_ref: String,
}

impl SignalSourceIdentity {
    fn is_complete(&self) -> bool {
        all_non_empty([
            &self.source_id,
            &self.source_class,
            &self.backend_ref,
            &self.provider_scope_ref,
        ])
    }
}

/// Target scope that a slice, timeline row, or runbook execution affects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetScope {
    /// Stable target-context identifier.
    pub target_context_ref: String,
    /// Human-reviewable environment, resource, service, or workspace scope.
    pub scope_label: String,
    /// Code, run, or deployment reference when available.
    pub code_or_run_ref: Option<String>,
}

impl TargetScope {
    fn is_complete(&self) -> bool {
        all_non_empty([&self.target_context_ref, &self.scope_label])
    }
}

/// Time window with explicit timezone semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimeWindow {
    /// Inclusive start timestamp.
    pub started_at: String,
    /// Exclusive end timestamp.
    pub ended_at: String,
    /// IANA timezone or fixed-offset label used by the source view.
    pub timezone: String,
}

impl EvidenceTimeWindow {
    fn is_complete(&self) -> bool {
        all_non_empty([&self.started_at, &self.ended_at, &self.timezone])
    }
}

/// Typed log, metric, or trace slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalSlice {
    /// Stable slice identifier used by reopen/export links.
    pub slice_id: String,
    /// Signal family represented by this slice.
    pub signal_kind: SignalKind,
    /// Source identity for the evidence.
    pub source_identity: SignalSourceIdentity,
    /// Target scope the evidence describes.
    pub target_scope: TargetScope,
    /// Hash or stable reference for the query/filter expression.
    pub query_or_filter_hash: String,
    /// Time window and timezone for the slice.
    pub time_window: EvidenceTimeWindow,
    /// Freshness state visible to consumers.
    pub freshness_state: EvidenceFreshnessState,
    /// Sampling, partiality, or truncation posture.
    pub sample_posture: SamplePosture,
    /// Export/redaction class for this slice.
    pub export_redaction_class: ExportRedactionClass,
    /// Collection timestamp for this materialized slice record.
    pub collected_at: String,
    /// Incident timeline entries that cite this slice.
    pub linked_incident_timeline_refs: Vec<String>,
}

impl SignalSlice {
    fn validate(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        if !all_non_empty([
            &self.slice_id,
            &self.query_or_filter_hash,
            &self.collected_at,
        ]) || !self.source_identity.is_complete()
            || !self.target_scope.is_complete()
            || !self.time_window.is_complete()
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::SliceIdentityIncomplete,
                self.slice_id.clone(),
                "signal slices require source identity, target scope, query/filter hash, time window, freshness, sample posture, and collection time",
            ));
        }
        if self.linked_incident_timeline_refs.is_empty() {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::SliceTimelineLinkMissing,
                self.slice_id.clone(),
                "signal slices must link to at least one incident timeline entry before a stable surface can reopen/export them",
            ));
        }
    }
}

/// Actor lineage preserved on incident timeline and runbook records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineage {
    /// Stable actor identifier or redacted actor reference.
    pub actor_ref: String,
    /// Role class such as driver, observer, approver, automation, or provider.
    pub actor_role: String,
    /// Optional parent actor or automation origin.
    pub delegated_from_ref: Option<String>,
}

impl ActorLineage {
    fn is_complete(&self) -> bool {
        all_non_empty([&self.actor_ref, &self.actor_role])
    }
}

/// Link from a timeline row to a source object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineLink {
    /// Link class for the target object.
    pub link_class: TimelineLinkClass,
    /// Stable target reference.
    pub target_ref: String,
}

impl TimelineLink {
    fn is_complete(&self) -> bool {
        !self.target_ref.trim().is_empty()
    }
}

/// Append-only incident timeline entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentTimelineEntry {
    /// Stable event identifier.
    pub event_id: String,
    /// Incident identifier this event belongs to.
    pub incident_ref: String,
    /// Event timestamp.
    pub event_time: String,
    /// Timezone used to render and compare this event.
    pub timezone: String,
    /// Monotonic sequence number within the incident timeline.
    pub sequence_number: u64,
    /// Actor lineage for the event.
    pub actor_lineage: ActorLineage,
    /// Event action class.
    pub action_class: String,
    /// Affected target scope.
    pub affected_scope: TargetScope,
    /// Outcome label for this event.
    pub outcome: String,
    /// Source and evidence links supporting the event.
    pub links: Vec<TimelineLink>,
}

impl IncidentTimelineEntry {
    fn validate(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        if !all_non_empty([
            &self.event_id,
            &self.incident_ref,
            &self.event_time,
            &self.timezone,
            &self.action_class,
            &self.outcome,
        ]) || !self.actor_lineage.is_complete()
            || !self.affected_scope.is_complete()
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::TimelineChronologyIncomplete,
                self.event_id.clone(),
                "incident timeline entries require event id, actor lineage, timezone-aware event time, action class, affected scope, and outcome",
            ));
        }
        if self.links.is_empty() || self.links.iter().any(|link| !link.is_complete()) {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::TimelineProvenanceMissing,
                self.event_id.clone(),
                "incident timeline entries must preserve source/evidence links instead of relying on notes",
            ));
        }
    }
}

/// Runbook packet identity used by step executions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookPacket {
    /// Stable runbook identifier.
    pub runbook_id: String,
    /// Version or digest for the runbook source.
    pub version: String,
    /// Source class for authority and handoff decisions.
    pub source_class: String,
    /// Ordered step identifiers.
    pub step_ids: Vec<String>,
    /// Required approval references or policy gates.
    pub required_approvals: Vec<String>,
    /// Expected evidence output classes or refs.
    pub expected_evidence_outputs: Vec<String>,
    /// Target selector rules used by the runbook.
    pub target_selector_rules: Vec<String>,
    /// Deviation policy reference.
    pub deviation_policy_ref: String,
}

impl RunbookPacket {
    fn is_complete(&self) -> bool {
        all_non_empty([
            &self.runbook_id,
            &self.version,
            &self.source_class,
            &self.deviation_policy_ref,
        ]) && !self.step_ids.is_empty()
            && !self.expected_evidence_outputs.is_empty()
            && !self.target_selector_rules.is_empty()
    }
}

/// Attributable runbook step execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepExecution {
    /// Stable step execution identifier.
    pub step_execution_id: String,
    /// Runbook identifier.
    pub runbook_ref: String,
    /// Runbook version reference.
    pub runbook_version_ref: String,
    /// Step identifier from the runbook packet.
    pub step_ref: String,
    /// Optional incident identifier.
    pub incident_ref: Option<String>,
    /// Target scope affected by the step.
    pub target_scope: TargetScope,
    /// Actor executing or recording the step.
    pub actor_lineage: ActorLineage,
    /// Runbook action class.
    pub action_class: RunbookActionClass,
    /// Current execution status.
    pub status: RunbookStepStatus,
    /// Approval reference when required or present.
    pub approval_ref: Option<String>,
    /// Start timestamp.
    pub started_at: String,
    /// End timestamp, if complete.
    pub ended_at: Option<String>,
    /// Durable deviation note reference when execution deviates.
    pub deviation_note_ref: Option<String>,
    /// External console or browser handoff refs.
    pub external_console_handoff_refs: Vec<String>,
    /// Rollback, repair, or backout references.
    pub rollback_refs: Vec<String>,
    /// Evidence records emitted or consumed by this step.
    pub evidence_refs: Vec<String>,
    /// Export continuity refs for support, incident, or compliance packets.
    pub export_continuity_refs: Vec<String>,
}

impl RunbookStepExecution {
    fn validate(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        if !all_non_empty([
            &self.step_execution_id,
            &self.runbook_ref,
            &self.runbook_version_ref,
            &self.step_ref,
            &self.started_at,
        ]) || !self.target_scope.is_complete()
            || !self.actor_lineage.is_complete()
            || self.evidence_refs.is_empty()
            || self.export_continuity_refs.is_empty()
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::RunbookExecutionProvenanceMissing,
                self.step_execution_id.clone(),
                "runbook step executions require target, actor, status, evidence refs, and export continuity refs",
            ));
        }
        if self.action_class.requires_approval_for_stable_completion()
            && self.approval_ref.is_none()
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::RunbookMutatingStepApprovalMissing,
                self.step_execution_id.clone(),
                "mutating runbook steps require an approval ref before stable completion",
            ));
        }
        if self.status.requires_deviation_note() && self.deviation_note_ref.is_none() {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::RunbookDeviationNoteMissing,
                self.step_execution_id.clone(),
                "deviated runbook steps require a durable deviation note ref",
            ));
        }
        if self.status.requires_handoff_ref() && self.external_console_handoff_refs.is_empty() {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::RunbookHandoffRefMissing,
                self.step_execution_id.clone(),
                "external-console runbook handoffs require stable handoff refs",
            ));
        }
    }
}

/// Export bundle manifest for incident or support evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceBundle {
    /// Stable bundle identifier.
    pub bundle_id: String,
    /// Optional incident identifier.
    pub incident_ref: Option<String>,
    /// Redaction profile applied to the bundle.
    pub redaction_profile: String,
    /// Manifest of embedded evidence refs.
    pub embedded_evidence_refs: Vec<String>,
    /// Manifest of by-reference evidence refs.
    pub by_reference_evidence_refs: Vec<String>,
    /// Explicit omission summaries for missing or redacted items.
    pub omission_summary: Vec<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Destination class for handoff/export.
    pub destination_class: String,
}

impl OperationalEvidenceBundle {
    fn validate(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        if !all_non_empty([
            &self.bundle_id,
            &self.redaction_profile,
            &self.created_at,
            &self.destination_class,
        ]) || (self.embedded_evidence_refs.is_empty()
            && self.by_reference_evidence_refs.is_empty())
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::ExportContinuityMissing,
                self.bundle_id.clone(),
                "export bundles require redaction profile, embedded/by-reference manifest, destination, and explicit omission summary",
            ));
        }
    }
}

/// Consumer projection that claims how a surface preserves the contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceConsumerProjection {
    /// Surface class.
    pub surface: OperationalEvidenceConsumerSurface,
    /// Claimed support class.
    pub support_class: OperationalEvidenceSupportClass,
    /// Freshness states rendered by this consumer.
    pub exposed_freshness_states: Vec<EvidenceFreshnessState>,
    /// Whether the surface preserves exact slice identifiers and reopen refs.
    pub preserves_slice_identity: bool,
    /// Whether the surface preserves timezone-aware timeline chronology.
    pub preserves_timezone_chronology: bool,
    /// Whether the surface preserves runbook execution provenance.
    pub preserves_runbook_execution_provenance: bool,
    /// Whether the surface preserves export/redaction continuity.
    pub preserves_export_continuity: bool,
    /// Packet or artifact refs proving the projection.
    pub projection_refs: Vec<String>,
}

impl OperationalEvidenceConsumerProjection {
    fn validate(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        if self.support_class.is_stable() {
            let observed: BTreeSet<EvidenceFreshnessState> =
                self.exposed_freshness_states.iter().copied().collect();
            for required in EvidenceFreshnessState::REQUIRED_FOR_STABLE_SURFACES {
                if !observed.contains(&required) {
                    findings.push(OperationalEvidenceValidationFinding::blocker(
                        OperationalEvidenceFindingKind::FreshnessVocabularyCollapsed,
                        self.surface.as_str(),
                        "stable observability and incident consumers must expose live, buffering, cached, imported, stale, partial, offline, truncated, and exported-copy states",
                    ));
                    break;
                }
            }
            if !self.preserves_slice_identity
                || !self.preserves_timezone_chronology
                || !self.preserves_runbook_execution_provenance
                || !self.preserves_export_continuity
                || self.projection_refs.is_empty()
            {
                findings.push(OperationalEvidenceValidationFinding::blocker(
                    OperationalEvidenceFindingKind::ConsumerProjectionContinuityMissing,
                    self.surface.as_str(),
                    "stable consumers must preserve slice identity, timezone chronology, runbook provenance, export continuity, and projection refs",
                ));
            }
        }
    }
}

/// Validation finding emitted by the operational evidence contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceValidationFinding {
    /// Finding severity.
    pub severity: OperationalEvidenceFindingSeverity,
    /// Finding kind.
    pub finding_kind: OperationalEvidenceFindingKind,
    /// Subject identifier.
    pub subject_ref: String,
    /// Human-reviewable finding message.
    pub message: String,
}

impl OperationalEvidenceValidationFinding {
    fn blocker(
        finding_kind: OperationalEvidenceFindingKind,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: OperationalEvidenceFindingSeverity::Blocker,
            finding_kind,
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Input packet before validation derives promotion state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceContractPacketInput {
    /// Stable packet identifier.
    pub packet_id: String,
    /// Surface or workflow identifier.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Signal slices covered by the packet.
    pub signal_slices: Vec<SignalSlice>,
    /// Incident timeline entries covered by the packet.
    pub incident_timeline: Vec<IncidentTimelineEntry>,
    /// Runbook packets covered by the packet.
    pub runbook_packets: Vec<RunbookPacket>,
    /// Runbook step executions covered by the packet.
    pub runbook_step_executions: Vec<RunbookStepExecution>,
    /// Export bundles covered by the packet.
    pub evidence_bundles: Vec<OperationalEvidenceBundle>,
    /// Consumer projections covered by the packet.
    pub consumer_projections: Vec<OperationalEvidenceConsumerProjection>,
    /// Source contracts used to derive the packet.
    pub source_contract_refs: Vec<String>,
}

/// Materialized operational evidence contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceContractPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Surface or workflow identifier.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Signal slices covered by the packet.
    pub signal_slices: Vec<SignalSlice>,
    /// Incident timeline entries covered by the packet.
    pub incident_timeline: Vec<IncidentTimelineEntry>,
    /// Runbook packets covered by the packet.
    pub runbook_packets: Vec<RunbookPacket>,
    /// Runbook step executions covered by the packet.
    pub runbook_step_executions: Vec<RunbookStepExecution>,
    /// Export bundles covered by the packet.
    pub evidence_bundles: Vec<OperationalEvidenceBundle>,
    /// Consumer projections covered by the packet.
    pub consumer_projections: Vec<OperationalEvidenceConsumerProjection>,
    /// Source contracts used to derive the packet.
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: OperationalEvidencePromotionState,
    /// Validation findings emitted during materialization.
    pub validation_findings: Vec<OperationalEvidenceValidationFinding>,
}

impl OperationalEvidenceContractPacket {
    /// Materializes and validates an operational evidence packet.
    pub fn materialize(input: OperationalEvidenceContractPacketInput) -> Self {
        let mut packet = Self {
            record_kind: OPERATIONAL_EVIDENCE_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            signal_slices: input.signal_slices,
            incident_timeline: input.incident_timeline,
            runbook_packets: input.runbook_packets,
            runbook_step_executions: input.runbook_step_executions,
            evidence_bundles: input.evidence_bundles,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: OperationalEvidencePromotionState::Stable,
            validation_findings: Vec::new(),
        };
        packet.validation_findings = packet.validate();
        packet.promotion_state = if packet
            .validation_findings
            .iter()
            .any(|finding| finding.severity == OperationalEvidenceFindingSeverity::Blocker)
        {
            OperationalEvidencePromotionState::BlocksStable
        } else {
            OperationalEvidencePromotionState::Stable
        };
        packet
    }

    /// Validates a materialized packet and returns findings.
    pub fn validate(&self) -> Vec<OperationalEvidenceValidationFinding> {
        let mut findings = Vec::new();
        if self.record_kind != OPERATIONAL_EVIDENCE_CONTRACT_RECORD_KIND {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::WrongRecordKind,
                self.packet_id.clone(),
                "packet record_kind must match the stable operational evidence contract",
            ));
        }
        if self.schema_version != OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_VERSION {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::WrongSchemaVersion,
                self.packet_id.clone(),
                "packet schema_version must match the stable operational evidence contract",
            ));
        }
        if !all_non_empty([
            &self.packet_id,
            &self.workflow_or_surface_id,
            &self.generated_at,
        ]) || self.source_contract_refs.is_empty()
        {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::MissingIdentity,
                self.packet_id.clone(),
                "packet identity, generated_at, workflow id, and source contract refs are required",
            ));
        }
        for slice in &self.signal_slices {
            slice.validate(&mut findings);
        }
        self.validate_timeline(&mut findings);
        for packet in &self.runbook_packets {
            if !packet.is_complete() {
                findings.push(OperationalEvidenceValidationFinding::blocker(
                    OperationalEvidenceFindingKind::RunbookExecutionProvenanceMissing,
                    packet.runbook_id.clone(),
                    "runbook packets require version, source class, step ids, expected evidence, target selector rules, and deviation policy",
                ));
            }
        }
        for step in &self.runbook_step_executions {
            step.validate(&mut findings);
        }
        for bundle in &self.evidence_bundles {
            bundle.validate(&mut findings);
        }
        for projection in &self.consumer_projections {
            projection.validate(&mut findings);
        }
        findings
    }

    fn validate_timeline(&self, findings: &mut Vec<OperationalEvidenceValidationFinding>) {
        let mut seen = BTreeSet::new();
        let mut times_by_sequence = BTreeMap::new();
        for entry in &self.incident_timeline {
            entry.validate(findings);
            if !seen.insert(entry.event_id.clone()) {
                findings.push(OperationalEvidenceValidationFinding::blocker(
                    OperationalEvidenceFindingKind::DuplicateTimelineEvent,
                    entry.event_id.clone(),
                    "incident timeline event ids must be unique",
                ));
            }
            times_by_sequence.insert(entry.sequence_number, entry.event_time.clone());
        }
        let ordered_times = times_by_sequence.values().cloned().collect::<Vec<_>>();
        if ordered_times.windows(2).any(|window| window[0] > window[1]) {
            findings.push(OperationalEvidenceValidationFinding::blocker(
                OperationalEvidenceFindingKind::TimelineOutOfOrder,
                self.packet_id.clone(),
                "incident timeline sequence order must match event_time chronology",
            ));
        }
    }

    /// Returns the unique freshness tokens represented by signal slices.
    pub fn freshness_tokens(&self) -> Vec<&'static str> {
        let set: BTreeSet<_> = self
            .signal_slices
            .iter()
            .map(|slice| slice.freshness_state)
            .collect();
        set.into_iter()
            .map(EvidenceFreshnessState::as_str)
            .collect()
    }

    /// Returns the unique signal kind tokens represented by signal slices.
    pub fn signal_kind_tokens(&self) -> Vec<&'static str> {
        let set: BTreeSet<_> = self
            .signal_slices
            .iter()
            .map(|slice| slice.signal_kind)
            .collect();
        set.into_iter().map(SignalKind::as_str).collect()
    }

    /// Returns the unique consumer surface tokens represented by projections.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let set: BTreeSet<_> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        set.into_iter()
            .map(OperationalEvidenceConsumerSurface::as_str)
            .collect()
    }

    /// Builds the metadata-only support export projection for this packet.
    pub fn support_export(
        &self,
        support_export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> OperationalEvidenceSupportExport {
        OperationalEvidenceSupportExport {
            record_kind: OPERATIONAL_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            packet_id: self.packet_id.clone(),
            exported_at: exported_at.into(),
            promotion_state: self.promotion_state,
            signal_slice_count: self.signal_slices.len(),
            incident_timeline_entry_count: self.incident_timeline.len(),
            runbook_step_execution_count: self.runbook_step_executions.len(),
            evidence_bundle_count: self.evidence_bundles.len(),
            freshness_tokens: self
                .freshness_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            support_export_safe: self.validation_findings.is_empty(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str().to_owned())
                .collect(),
        }
    }
}

/// Metadata-only support export projection for an operational evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalEvidenceSupportExport {
    /// Stable support-export record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export identifier.
    pub support_export_id: String,
    /// Source packet identifier.
    pub packet_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Packet promotion state.
    pub promotion_state: OperationalEvidencePromotionState,
    /// Number of signal slices in the source packet.
    pub signal_slice_count: usize,
    /// Number of incident timeline entries in the source packet.
    pub incident_timeline_entry_count: usize,
    /// Number of runbook step executions in the source packet.
    pub runbook_step_execution_count: usize,
    /// Number of evidence bundles in the source packet.
    pub evidence_bundle_count: usize,
    /// Freshness tokens preserved from signal slices.
    pub freshness_tokens: Vec<String>,
    /// True when the packet has no validation findings.
    pub support_export_safe: bool,
    /// Finding kinds carried into the export.
    pub finding_kinds: Vec<String>,
}

/// Error returned when serializing the current packet fails.
#[derive(Debug)]
pub enum OperationalEvidenceContractArtifactError {
    /// JSON serialization failed.
    Serialize(serde_json::Error),
}

impl fmt::Display for OperationalEvidenceContractArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(f, "serialize operational evidence contract: {err}"),
        }
    }
}

impl Error for OperationalEvidenceContractArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialize(err) => Some(err),
        }
    }
}

/// Returns the current stable operational evidence contract packet input.
pub fn current_stable_operational_evidence_contract_input() -> OperationalEvidenceContractPacketInput
{
    OperationalEvidenceContractPacketInput {
        packet_id: "runtime:operational-evidence:stable:001".to_owned(),
        workflow_or_surface_id: "runtime:observability-incident-runbook-contract".to_owned(),
        generated_at: "2026-06-06T19:15:00Z".to_owned(),
        signal_slices: baseline_signal_slices(),
        incident_timeline: baseline_timeline(),
        runbook_packets: vec![baseline_runbook_packet()],
        runbook_step_executions: baseline_runbook_steps(),
        evidence_bundles: vec![baseline_evidence_bundle()],
        consumer_projections: baseline_consumer_projections(),
        source_contract_refs: vec![
            OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF.to_owned(),
            OPERATIONAL_EVIDENCE_CONTRACT_DOC_REF.to_owned(),
            "schemas/support/incident_action_ledger.schema.json".to_owned(),
            "docs/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth.md"
                .to_owned(),
        ],
    }
}

/// Returns the current stable operational evidence contract packet.
pub fn current_stable_operational_evidence_contract_packet() -> OperationalEvidenceContractPacket {
    OperationalEvidenceContractPacket::materialize(
        current_stable_operational_evidence_contract_input(),
    )
}

/// Serializes the current stable packet as pretty JSON.
pub fn current_stable_operational_evidence_contract_json(
) -> Result<String, OperationalEvidenceContractArtifactError> {
    serde_json::to_string_pretty(&current_stable_operational_evidence_contract_packet())
        .map_err(OperationalEvidenceContractArtifactError::Serialize)
}

fn baseline_signal_slices() -> Vec<SignalSlice> {
    let target_scope = TargetScope {
        target_context_ref: "target:prod:checkout-api".to_owned(),
        scope_label: "prod checkout-api us-west-2".to_owned(),
        code_or_run_ref: Some("run:deploy:checkout-api:2026-06-06".to_owned()),
    };
    vec![
        SignalSlice {
            slice_id: "slice:log:checkout-api:tail:001".to_owned(),
            signal_kind: SignalKind::Log,
            source_identity: SignalSourceIdentity {
                source_id: "source:otel:prod-logs".to_owned(),
                source_class: "managed_provider".to_owned(),
                backend_ref: "provider:otel:logs".to_owned(),
                provider_scope_ref: "account:prod:us-west-2".to_owned(),
            },
            target_scope: target_scope.clone(),
            query_or_filter_hash: "sha256:log-filter-checkout-5xx".to_owned(),
            time_window: EvidenceTimeWindow {
                started_at: "2026-06-06T18:50:00Z".to_owned(),
                ended_at: "2026-06-06T19:05:00Z".to_owned(),
                timezone: "America/Los_Angeles".to_owned(),
            },
            freshness_state: EvidenceFreshnessState::Live,
            sample_posture: SamplePosture::Complete,
            export_redaction_class: ExportRedactionClass::RedactedPayload,
            collected_at: "2026-06-06T19:05:05Z".to_owned(),
            linked_incident_timeline_refs: vec!["timeline:inc-042:001".to_owned()],
        },
        SignalSlice {
            slice_id: "slice:metric:checkout-api:latency:001".to_owned(),
            signal_kind: SignalKind::Metric,
            source_identity: SignalSourceIdentity {
                source_id: "source:otel:prod-metrics".to_owned(),
                source_class: "managed_provider".to_owned(),
                backend_ref: "provider:otel:metrics".to_owned(),
                provider_scope_ref: "account:prod:us-west-2".to_owned(),
            },
            target_scope: target_scope.clone(),
            query_or_filter_hash: "sha256:metric-p95-checkout-window".to_owned(),
            time_window: EvidenceTimeWindow {
                started_at: "2026-06-06T18:45:00Z".to_owned(),
                ended_at: "2026-06-06T19:05:00Z".to_owned(),
                timezone: "America/Los_Angeles".to_owned(),
            },
            freshness_state: EvidenceFreshnessState::Cached,
            sample_posture: SamplePosture::Downsampled,
            export_redaction_class: ExportRedactionClass::MetadataEmbedded,
            collected_at: "2026-06-06T19:05:08Z".to_owned(),
            linked_incident_timeline_refs: vec!["timeline:inc-042:002".to_owned()],
        },
        SignalSlice {
            slice_id: "slice:trace:checkout-api:span-set:001".to_owned(),
            signal_kind: SignalKind::Trace,
            source_identity: SignalSourceIdentity {
                source_id: "source:trace-import:provider-a".to_owned(),
                source_class: "imported_artifact".to_owned(),
                backend_ref: "provider:trace:external".to_owned(),
                provider_scope_ref: "case:inc-042".to_owned(),
            },
            target_scope,
            query_or_filter_hash: "sha256:trace-slowest-checkout".to_owned(),
            time_window: EvidenceTimeWindow {
                started_at: "2026-06-06T18:57:00Z".to_owned(),
                ended_at: "2026-06-06T19:00:00Z".to_owned(),
                timezone: "UTC".to_owned(),
            },
            freshness_state: EvidenceFreshnessState::Imported,
            sample_posture: SamplePosture::Sampled,
            export_redaction_class: ExportRedactionClass::ByReferenceOnly,
            collected_at: "2026-06-06T19:06:00Z".to_owned(),
            linked_incident_timeline_refs: vec!["timeline:inc-042:003".to_owned()],
        },
    ]
}

fn baseline_timeline() -> Vec<IncidentTimelineEntry> {
    let target_scope = TargetScope {
        target_context_ref: "target:prod:checkout-api".to_owned(),
        scope_label: "prod checkout-api us-west-2".to_owned(),
        code_or_run_ref: Some("run:deploy:checkout-api:2026-06-06".to_owned()),
    };
    vec![
        IncidentTimelineEntry {
            event_id: "timeline:inc-042:001".to_owned(),
            incident_ref: "incident:checkout-latency:042".to_owned(),
            event_time: "2026-06-06T19:05:10Z".to_owned(),
            timezone: "America/Los_Angeles".to_owned(),
            sequence_number: 1,
            actor_lineage: ActorLineage {
                actor_ref: "actor:oncall:driver".to_owned(),
                actor_role: "driver".to_owned(),
                delegated_from_ref: None,
            },
            action_class: "observe_log_slice".to_owned(),
            affected_scope: target_scope.clone(),
            outcome: "observed".to_owned(),
            links: vec![
                TimelineLink {
                    link_class: TimelineLinkClass::SignalSlice,
                    target_ref: "slice:log:checkout-api:tail:001".to_owned(),
                },
                TimelineLink {
                    link_class: TimelineLinkClass::ProviderEvent,
                    target_ref: "provider-event:alert:checkout-p95".to_owned(),
                },
            ],
        },
        IncidentTimelineEntry {
            event_id: "timeline:inc-042:002".to_owned(),
            incident_ref: "incident:checkout-latency:042".to_owned(),
            event_time: "2026-06-06T19:07:00Z".to_owned(),
            timezone: "America/Los_Angeles".to_owned(),
            sequence_number: 2,
            actor_lineage: ActorLineage {
                actor_ref: "actor:oncall:approver".to_owned(),
                actor_role: "approver".to_owned(),
                delegated_from_ref: None,
            },
            action_class: "approve_mitigation".to_owned(),
            affected_scope: target_scope.clone(),
            outcome: "approved".to_owned(),
            links: vec![
                TimelineLink {
                    link_class: TimelineLinkClass::SignalSlice,
                    target_ref: "slice:metric:checkout-api:latency:001".to_owned(),
                },
                TimelineLink {
                    link_class: TimelineLinkClass::Approval,
                    target_ref: "approval:runtime:restart:checkout-api:001".to_owned(),
                },
            ],
        },
        IncidentTimelineEntry {
            event_id: "timeline:inc-042:003".to_owned(),
            incident_ref: "incident:checkout-latency:042".to_owned(),
            event_time: "2026-06-06T19:08:00Z".to_owned(),
            timezone: "UTC".to_owned(),
            sequence_number: 3,
            actor_lineage: ActorLineage {
                actor_ref: "actor:oncall:driver".to_owned(),
                actor_role: "driver".to_owned(),
                delegated_from_ref: Some("actor:oncall:approver".to_owned()),
            },
            action_class: "execute_runbook_step".to_owned(),
            affected_scope: target_scope,
            outcome: "executed".to_owned(),
            links: vec![
                TimelineLink {
                    link_class: TimelineLinkClass::SignalSlice,
                    target_ref: "slice:trace:checkout-api:span-set:001".to_owned(),
                },
                TimelineLink {
                    link_class: TimelineLinkClass::RunbookStep,
                    target_ref: "runbook-step-exec:checkout-restart:001".to_owned(),
                },
                TimelineLink {
                    link_class: TimelineLinkClass::Repair,
                    target_ref: "repair:restart:checkout-api:001".to_owned(),
                },
            ],
        },
    ]
}

fn baseline_runbook_packet() -> RunbookPacket {
    RunbookPacket {
        runbook_id: "runbook:checkout-api:latency-mitigation".to_owned(),
        version: "2026.06.06+sha256:runbook-checkout-latency".to_owned(),
        source_class: "repo_local_runbook".to_owned(),
        step_ids: vec![
            "step:observe-current-error-rate".to_owned(),
            "step:restart-checkout-api".to_owned(),
        ],
        required_approvals: vec!["approval-policy:prod-runtime-mutation".to_owned()],
        expected_evidence_outputs: vec![
            "signal_slice_ref".to_owned(),
            "incident_timeline_entry_ref".to_owned(),
            "runbook_step_result_ref".to_owned(),
        ],
        target_selector_rules: vec![
            "service=checkout-api environment=prod region=us-west-2".to_owned()
        ],
        deviation_policy_ref: "deviation-policy:incident-runbook:default".to_owned(),
    }
}

fn baseline_runbook_steps() -> Vec<RunbookStepExecution> {
    vec![RunbookStepExecution {
        step_execution_id: "runbook-step-exec:checkout-restart:001".to_owned(),
        runbook_ref: "runbook:checkout-api:latency-mitigation".to_owned(),
        runbook_version_ref: "2026.06.06+sha256:runbook-checkout-latency".to_owned(),
        step_ref: "step:restart-checkout-api".to_owned(),
        incident_ref: Some("incident:checkout-latency:042".to_owned()),
        target_scope: TargetScope {
            target_context_ref: "target:prod:checkout-api".to_owned(),
            scope_label: "prod checkout-api us-west-2".to_owned(),
            code_or_run_ref: Some("run:deploy:checkout-api:2026-06-06".to_owned()),
        },
        actor_lineage: ActorLineage {
            actor_ref: "actor:oncall:driver".to_owned(),
            actor_role: "driver".to_owned(),
            delegated_from_ref: Some("actor:oncall:approver".to_owned()),
        },
        action_class: RunbookActionClass::Mitigate,
        status: RunbookStepStatus::Executed,
        approval_ref: Some("approval:runtime:restart:checkout-api:001".to_owned()),
        started_at: "2026-06-06T19:07:30Z".to_owned(),
        ended_at: Some("2026-06-06T19:08:00Z".to_owned()),
        deviation_note_ref: None,
        external_console_handoff_refs: Vec::new(),
        rollback_refs: vec!["rollback:checkout-api:restart:001".to_owned()],
        evidence_refs: vec![
            "slice:log:checkout-api:tail:001".to_owned(),
            "timeline:inc-042:003".to_owned(),
        ],
        export_continuity_refs: vec!["bundle:incident:checkout-latency:042".to_owned()],
    }]
}

fn baseline_evidence_bundle() -> OperationalEvidenceBundle {
    OperationalEvidenceBundle {
        bundle_id: "bundle:incident:checkout-latency:042".to_owned(),
        incident_ref: Some("incident:checkout-latency:042".to_owned()),
        redaction_profile: "support-default-redaction".to_owned(),
        embedded_evidence_refs: vec![
            "timeline:inc-042:001".to_owned(),
            "timeline:inc-042:002".to_owned(),
            "timeline:inc-042:003".to_owned(),
        ],
        by_reference_evidence_refs: vec![
            "slice:log:checkout-api:tail:001".to_owned(),
            "slice:trace:checkout-api:span-set:001".to_owned(),
        ],
        omission_summary: vec!["raw provider console session omitted by policy".to_owned()],
        created_at: "2026-06-06T19:10:00Z".to_owned(),
        destination_class: "support_handoff".to_owned(),
    }
}

fn baseline_consumer_projections() -> Vec<OperationalEvidenceConsumerProjection> {
    let states = EvidenceFreshnessState::REQUIRED_FOR_STABLE_SURFACES.to_vec();
    [
        OperationalEvidenceConsumerSurface::ObservabilityUi,
        OperationalEvidenceConsumerSurface::IncidentWorkspace,
        OperationalEvidenceConsumerSurface::RunbookExecution,
        OperationalEvidenceConsumerSurface::CliHeadless,
        OperationalEvidenceConsumerSurface::SupportExport,
        OperationalEvidenceConsumerSurface::IncidentExport,
    ]
    .into_iter()
    .map(|surface| OperationalEvidenceConsumerProjection {
        surface,
        support_class: OperationalEvidenceSupportClass::Stable,
        exposed_freshness_states: states.clone(),
        preserves_slice_identity: true,
        preserves_timezone_chronology: true,
        preserves_runbook_execution_provenance: true,
        preserves_export_continuity: true,
        projection_refs: vec![format!("projection:{}", surface.as_str())],
    })
    .collect()
}

fn all_non_empty<'a>(values: impl IntoIterator<Item = &'a String>) -> bool {
    values.into_iter().all(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_packet_is_stable_and_export_safe() {
        let packet = current_stable_operational_evidence_contract_packet();
        assert_eq!(
            packet.promotion_state,
            OperationalEvidencePromotionState::Stable
        );
        assert!(packet.validation_findings.is_empty());
        assert_eq!(packet.signal_kind_tokens(), vec!["log", "metric", "trace"]);
        assert_eq!(
            packet.consumer_surface_tokens(),
            vec![
                "observability_ui",
                "incident_workspace",
                "runbook_execution",
                "cli_headless",
                "support_export",
                "incident_export"
            ]
        );
        assert!(
            packet
                .support_export("support:operational-evidence:001", "2026-06-06T19:11:00Z")
                .support_export_safe
        );
    }

    #[test]
    fn stable_projection_that_collapses_freshness_blocks_stable() {
        let mut input = current_stable_operational_evidence_contract_input();
        input.consumer_projections[0].exposed_freshness_states =
            vec![EvidenceFreshnessState::Live, EvidenceFreshnessState::Cached];
        let packet = OperationalEvidenceContractPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            OperationalEvidencePromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == OperationalEvidenceFindingKind::FreshnessVocabularyCollapsed
        }));
    }

    #[test]
    fn timeline_without_timezone_blocks_stable() {
        let mut input = current_stable_operational_evidence_contract_input();
        input.incident_timeline[0].timezone.clear();
        let packet = OperationalEvidenceContractPacket::materialize(input);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == OperationalEvidenceFindingKind::TimelineChronologyIncomplete
        }));
    }

    #[test]
    fn mutating_runbook_step_without_approval_blocks_stable() {
        let mut input = current_stable_operational_evidence_contract_input();
        input.runbook_step_executions[0].approval_ref = None;
        let packet = OperationalEvidenceContractPacket::materialize(input);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind
                == OperationalEvidenceFindingKind::RunbookMutatingStepApprovalMissing
        }));
    }
}
