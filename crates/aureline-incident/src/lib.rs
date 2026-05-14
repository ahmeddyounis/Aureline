//! Incident workspace and runbook packet alpha support projection.
//!
//! This crate owns the first runtime-shaped incident workspace packet for
//! local alpha triage. It keeps the workspace read-mostly, records evidence
//! and missing spans as first-class facts, consumes the existing crash trail
//! and support runbook packet contracts, and projects one redacted support
//! bundle preview through `aureline-support` instead of minting a parallel
//! export format.

#![doc(html_root_url = "https://docs.rs/aureline-incident/0.0.0")]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_crash::{CrashIncidentTrail, SymbolicationState};
use aureline_support::bundle::{
    ActionPolicySourceContext, ActionReconstructionSeed, ActionabilityImpactClass, ActorClass,
    BuildIdentity, DiagnosticDataClass, ExactBuildCapture, HighRiskContentClass, PolicyContext,
    PreviewItemSeed, ReleaseChannelClass, SizeEstimate, SupportBundlePreview,
    SupportBundlePreviewBuilder, SupportBundlePreviewError,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried on every incident workspace alpha packet.
pub const INCIDENT_WORKSPACE_PACKET_RECORD_KIND: &str = "incident_workspace_alpha_packet";

/// Stable record-kind tag carried on every incident runbook packet summary.
pub const INCIDENT_RUNBOOK_PACKET_RECORD_KIND: &str = "incident_runbook_packet_alpha_record";

/// Schema version for the incident workspace alpha packet.
pub const INCIDENT_WORKSPACE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Schema version for the incident runbook packet summary.
pub const INCIDENT_RUNBOOK_PACKET_SCHEMA_VERSION: u32 = 1;

/// Support-pack item id for the incident workspace summary row.
pub const SUPPORT_ITEM_INCIDENT_WORKSPACE_SUMMARY: &str = "support.item.incident.workspace_summary";

/// Support-pack item id for explicit missing-span disclosure.
pub const SUPPORT_ITEM_INCIDENT_MISSING_SPANS: &str = "support.item.incident.missing_spans";

/// Default support runbook packet schema path consumed by this crate.
pub const SUPPORT_RUNBOOK_PACKET_SCHEMA_REF: &str = "schemas/support/runbook_packet.schema.json";

/// Policy context captured when the incident workspace packet was built.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentPolicyContext {
    /// Opaque policy epoch ref active for incident collection.
    pub policy_epoch_ref: String,
    /// Workspace trust state token active for incident collection.
    pub trust_state: String,
    /// Execution-context ref active for incident collection.
    pub execution_context_id: String,
}

impl IncidentPolicyContext {
    /// Returns a local trusted policy context suitable for deterministic fixtures.
    pub fn local_fixture() -> Self {
        Self {
            policy_epoch_ref: "policy-epoch:local-default:1".into(),
            trust_state: "trusted".into(),
            execution_context_id: "execution-context:local-workspace".into(),
        }
    }
}

/// Hosted or provider lane availability at packet generation time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderLaneState {
    /// Hosted or provider lanes are available.
    Available,
    /// No hosted or provider lane is configured for this workspace.
    NotConfigured,
    /// Hosted or provider lanes are partially degraded.
    Degraded,
    /// Hosted or provider lanes are unavailable.
    Unavailable,
}

impl ProviderLaneState {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::NotConfigured => "not_configured",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when local continuity is required for this state.
    pub const fn requires_local_continuity(self) -> bool {
        matches!(
            self,
            Self::NotConfigured | Self::Degraded | Self::Unavailable
        )
    }
}

/// Local continuity posture for the incident workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityState {
    /// Local diagnosis, preview, and export remain available.
    LocalDiagnosticsAvailable,
    /// The workspace is an imported read-only replay.
    ReadOnlyReplay,
    /// The workspace cannot proceed until required local evidence is restored.
    BlockedAwaitingEvidence,
}

impl LocalContinuityState {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDiagnosticsAvailable => "local_diagnostics_available",
            Self::ReadOnlyReplay => "read_only_replay",
            Self::BlockedAwaitingEvidence => "blocked_awaiting_evidence",
        }
    }

    /// Returns true when the workspace can still be inspected or exported locally.
    pub const fn permits_local_review(self) -> bool {
        matches!(self, Self::LocalDiagnosticsAvailable | Self::ReadOnlyReplay)
    }
}

/// Evidence classes an incident workspace can attach without embedding raw bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentEvidenceKind {
    /// Redacted or by-reference log slice.
    LogSlice,
    /// Crash incident-trail ref.
    CrashReference,
    /// Task, command, or invocation history summary.
    TaskHistory,
    /// Support bundle manifest or preview ref.
    SupportBundle,
    /// Runbook packet ref.
    RunbookPacket,
    /// Missing-span disclosure row.
    MissingSpanDisclosure,
    /// Incident workspace summary row.
    WorkspaceSummary,
}

impl IncidentEvidenceKind {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogSlice => "log_slice",
            Self::CrashReference => "crash_reference",
            Self::TaskHistory => "task_history",
            Self::SupportBundle => "support_bundle",
            Self::RunbookPacket => "runbook_packet",
            Self::MissingSpanDisclosure => "missing_span_disclosure",
            Self::WorkspaceSummary => "workspace_summary",
        }
    }

    /// Returns the support artifact-kind class for this evidence kind.
    pub const fn artifact_kind_class(self) -> &'static str {
        match self {
            Self::LogSlice => "incident_log_slice_ref",
            Self::CrashReference => "crash_incident_trail_alpha_record",
            Self::TaskHistory => "incident_task_history_summary",
            Self::SupportBundle => "support_bundle_manifest_ref",
            Self::RunbookPacket => "support_runbook_packet_record",
            Self::MissingSpanDisclosure => "missing_span_disclosure",
            Self::WorkspaceSummary => "incident_workspace_alpha_packet",
        }
    }
}

/// Availability state for one evidence attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAvailability {
    /// Evidence metadata was captured in the packet.
    Captured,
    /// Evidence is available by opaque reference only.
    ByReference,
    /// Evidence is present only as a redacted summary.
    Redacted,
    /// Evidence is retained on the local machine and not exported.
    LocalOnly,
    /// Evidence was expected but unavailable.
    Missing,
}

impl EvidenceAvailability {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::ByReference => "by_reference",
            Self::Redacted => "redacted",
            Self::LocalOnly => "local_only",
            Self::Missing => "missing",
        }
    }

    /// Returns true when the evidence can contribute to diagnosis.
    pub const fn contributes_to_diagnosis(self) -> bool {
        matches!(self, Self::Captured | Self::ByReference | Self::Redacted)
    }
}

/// Action reconstruction context attached to a task-history evidence row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentActionContext {
    /// Command Aureline believed it was running.
    pub command_id: String,
    /// Descriptor or schema ref for the command.
    pub command_descriptor_ref: String,
    /// Invocation-session id for the command attempt.
    pub invocation_session_id: String,
    /// Target identity ref or typed target token.
    pub target_identity_ref: String,
    /// Optional route-truth packet ref.
    pub action_route_packet_ref: Option<String>,
    /// Origin class from the route taxonomy.
    pub action_origin_class: String,
    /// Target class from the route taxonomy.
    pub action_target_class: String,
    /// Route class from the route taxonomy.
    pub action_route_class: String,
    /// Exposure class from the route taxonomy.
    pub action_exposure_class: String,
    /// Policy source that governed the action.
    pub policy_source: ActionPolicySourceContext,
    /// Redaction-safe route summary.
    pub route_summary: String,
    /// Optional reviewed-enforcement ref.
    pub reviewed_enforcement_ref: Option<String>,
    /// Redaction class applied to the reconstruction context.
    pub redaction_class: String,
}

impl IncidentActionContext {
    fn to_support_seed(&self, support_pack_item_id: &str) -> ActionReconstructionSeed {
        ActionReconstructionSeed {
            support_pack_item_id: support_pack_item_id.to_owned(),
            command_id: self.command_id.clone(),
            command_descriptor_ref: self.command_descriptor_ref.clone(),
            invocation_session_id: self.invocation_session_id.clone(),
            target_identity_ref: self.target_identity_ref.clone(),
            action_route_packet_ref: self.action_route_packet_ref.clone(),
            action_origin_class: self.action_origin_class.clone(),
            action_target_class: self.action_target_class.clone(),
            action_route_class: self.action_route_class.clone(),
            action_exposure_class: self.action_exposure_class.clone(),
            policy_source: self.policy_source.clone(),
            route_summary: self.route_summary.clone(),
            reviewed_enforcement_ref: self.reviewed_enforcement_ref.clone(),
            redaction_class: self.redaction_class.clone(),
        }
    }
}

/// One evidence attachment retained by an incident workspace packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentEvidenceAttachment {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Stable support-pack item id used in redacted exports.
    pub support_pack_item_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Evidence kind.
    pub evidence_kind: IncidentEvidenceKind,
    /// Opaque source refs backing this attachment.
    pub source_refs: Vec<String>,
    /// Diagnostic data class used by the support redaction profile.
    pub data_class: DiagnosticDataClass,
    /// Required high-risk subtype when `data_class` is high risk.
    pub high_risk_content_class: HighRiskContentClass,
    /// Availability state for this evidence.
    pub availability: EvidenceAvailability,
    /// Whether first diagnosis depends on this evidence.
    pub required_for_first_diagnosis: bool,
    /// Optional byte estimate for the preview row.
    pub estimated_bytes: Option<u64>,
    /// Optional command and route reconstruction context.
    #[serde(default)]
    pub action_context: Option<IncidentActionContext>,
    /// Redaction-safe notes.
    pub notes: String,
}

impl IncidentEvidenceAttachment {
    /// Creates a redacted or by-reference log slice attachment.
    pub fn log_slice(
        evidence_id: impl Into<String>,
        source_ref: impl Into<String>,
        estimated_bytes: Option<u64>,
    ) -> Self {
        let evidence_id = evidence_id.into();
        Self {
            support_pack_item_id: support_item_id("incident.log", &evidence_id),
            evidence_id,
            title: "Incident log slice".into(),
            evidence_kind: IncidentEvidenceKind::LogSlice,
            source_refs: vec![source_ref.into()],
            data_class: DiagnosticDataClass::CodeAdjacent,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            availability: EvidenceAvailability::Redacted,
            required_for_first_diagnosis: true,
            estimated_bytes,
            action_context: None,
            notes: "Log slice is attached as a redacted or by-reference row; raw log bodies are not embedded.".into(),
        }
    }

    /// Creates a task-history attachment with command reconstruction context.
    pub fn task_history(
        evidence_id: impl Into<String>,
        source_ref: impl Into<String>,
        action_context: IncidentActionContext,
    ) -> Self {
        let evidence_id = evidence_id.into();
        Self {
            support_pack_item_id: support_item_id("incident.task_history", &evidence_id),
            evidence_id,
            title: "Task and command history".into(),
            evidence_kind: IncidentEvidenceKind::TaskHistory,
            source_refs: vec![source_ref.into()],
            data_class: DiagnosticDataClass::EnvironmentAdjacent,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            availability: EvidenceAvailability::ByReference,
            required_for_first_diagnosis: true,
            estimated_bytes: Some(4096),
            action_context: Some(action_context),
            notes: "Task history carries command, target, route, and policy refs without raw command lines.".into(),
        }
    }

    /// Creates a support-bundle manifest attachment.
    pub fn support_bundle(
        support_bundle_id: impl Into<String>,
        manifest_ref: impl Into<String>,
        preview_snapshot_ref: impl Into<String>,
    ) -> Self {
        let support_bundle_id = support_bundle_id.into();
        let manifest_ref = manifest_ref.into();
        Self {
            support_pack_item_id: support_item_id("incident.support_bundle", &support_bundle_id),
            evidence_id: format!("evidence:{support_bundle_id}"),
            title: "Linked support bundle".into(),
            evidence_kind: IncidentEvidenceKind::SupportBundle,
            source_refs: vec![manifest_ref, preview_snapshot_ref.into()],
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            availability: EvidenceAvailability::ByReference,
            required_for_first_diagnosis: true,
            estimated_bytes: Some(2048),
            action_context: None,
            notes: "Support bundle id and manifest refs are attached by reference.".into(),
        }
    }

    /// Creates a crash incident-trail attachment.
    pub fn crash_reference(trail: &CrashIncidentTrail) -> Self {
        let mut source_refs = vec![
            trail.incident_trail_id.clone(),
            trail.crash_envelope_ref.clone(),
            trail.crash_dump_ref.clone(),
        ];
        if let Some(report_ref) = &trail.symbolication_report_ref {
            source_refs.push(report_ref.clone());
        }
        if let Some(manifest_ref) = &trail.support_bundle_linkage.support_bundle_manifest_ref {
            source_refs.push(manifest_ref.clone());
        }

        Self {
            support_pack_item_id: support_item_id("incident.crash", &trail.incident_trail_id),
            evidence_id: format!("evidence:{}", trail.incident_trail_id),
            title: "Crash incident trail".into(),
            evidence_kind: IncidentEvidenceKind::CrashReference,
            source_refs,
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            availability: EvidenceAvailability::ByReference,
            required_for_first_diagnosis: true,
            estimated_bytes: Some(8192),
            action_context: None,
            notes: "Crash trail is attached as exact-build and symbolication metadata; raw dumps stay out of the packet.".into(),
        }
    }
}

/// Missing-span categories the workspace can disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingSpanKind {
    /// A trace window was expected but unavailable.
    TraceWindow,
    /// A log window was expected but unavailable.
    LogWindow,
    /// A task-history segment was expected but unavailable.
    TaskHistory,
    /// A crash symbolication report was expected but unavailable.
    SymbolicationReport,
    /// A provider callback or handoff event was expected but unavailable.
    ProviderCallback,
    /// A support bundle manifest join was expected but unavailable.
    SupportBundleManifest,
}

impl MissingSpanKind {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TraceWindow => "trace_window",
            Self::LogWindow => "log_window",
            Self::TaskHistory => "task_history",
            Self::SymbolicationReport => "symbolication_report",
            Self::ProviderCallback => "provider_callback",
            Self::SupportBundleManifest => "support_bundle_manifest",
        }
    }
}

/// Typed reason explaining why a span is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingSpanReasonClass {
    /// Source expired before the packet was generated.
    SourceExpired,
    /// Active policy or redaction rules withheld the span.
    RedactedByPolicy,
    /// Hosted or provider lane was unavailable.
    ProviderLaneUnavailable,
    /// The capture point did not collect this span.
    NotCollected,
    /// Symbolication was unavailable.
    SymbolicationUnavailable,
    /// The missing reason itself requires review.
    UnknownRequiresReview,
}

impl MissingSpanReasonClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceExpired => "source_expired",
            Self::RedactedByPolicy => "redacted_by_policy",
            Self::ProviderLaneUnavailable => "provider_lane_unavailable",
            Self::NotCollected => "not_collected",
            Self::SymbolicationUnavailable => "symbolication_unavailable",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// How much a missing span affects incident progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingSpanImpactClass {
    /// The workspace remains usable.
    DoesNotBlockWorkspace,
    /// First actionable diagnosis is weaker without this span.
    WeakensFirstDiagnosis,
    /// The runbook cannot claim completion until this span appears or is waived.
    BlocksCompletionClaim,
}

impl MissingSpanImpactClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoesNotBlockWorkspace => "does_not_block_workspace",
            Self::WeakensFirstDiagnosis => "weakens_first_diagnosis",
            Self::BlocksCompletionClaim => "blocks_completion_claim",
        }
    }
}

/// One explicit gap in the incident chronology or evidence graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingSpan {
    /// Stable missing-span id.
    pub missing_span_id: String,
    /// Missing-span kind.
    pub span_kind: MissingSpanKind,
    /// Typed reason for the gap.
    pub reason_class: MissingSpanReasonClass,
    /// Whether this gap affects first diagnosis.
    pub required_for_first_diagnosis: bool,
    /// Progress impact caused by this gap.
    pub impact_class: MissingSpanImpactClass,
    /// Opaque refs that would have supplied the span.
    pub expected_source_refs: Vec<String>,
    /// Redaction-safe reviewer summary.
    pub reviewer_summary: String,
}

impl MissingSpan {
    /// Creates a missing-span disclosure row.
    pub fn new(
        missing_span_id: impl Into<String>,
        span_kind: MissingSpanKind,
        reason_class: MissingSpanReasonClass,
        required_for_first_diagnosis: bool,
        impact_class: MissingSpanImpactClass,
        reviewer_summary: impl Into<String>,
    ) -> Self {
        Self {
            missing_span_id: missing_span_id.into(),
            span_kind,
            reason_class,
            required_for_first_diagnosis,
            impact_class,
            expected_source_refs: Vec::new(),
            reviewer_summary: reviewer_summary.into(),
        }
    }

    /// Adds an expected source ref.
    pub fn with_expected_source_ref(mut self, source_ref: impl Into<String>) -> Self {
        self.expected_source_refs.push(source_ref.into());
        self
    }
}

/// Summary of captured versus missing spans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanCoverageSummary {
    /// Required evidence or span count.
    pub required_span_count: u32,
    /// Required spans that have usable metadata or refs.
    pub captured_required_span_count: u32,
    /// Required spans that are explicitly missing.
    pub missing_required_span_count: u32,
    /// True only when there are no missing required spans.
    pub complete_coverage_claimed: bool,
    /// Redaction-safe summary shown in packet previews.
    pub summary: String,
}

impl SpanCoverageSummary {
    fn from_parts(evidence: &[IncidentEvidenceAttachment], missing_spans: &[MissingSpan]) -> Self {
        let required_evidence_count = evidence
            .iter()
            .filter(|item| item.required_for_first_diagnosis)
            .count() as u32;
        let captured_required_span_count = evidence
            .iter()
            .filter(|item| {
                item.required_for_first_diagnosis && item.availability.contributes_to_diagnosis()
            })
            .count() as u32;
        let missing_required_span_count = evidence
            .iter()
            .filter(|item| {
                item.required_for_first_diagnosis
                    && item.availability == EvidenceAvailability::Missing
            })
            .count() as u32
            + missing_spans
                .iter()
                .filter(|span| span.required_for_first_diagnosis)
                .count() as u32;
        let required_span_count = required_evidence_count + missing_required_span_count;
        let complete_coverage_claimed = missing_required_span_count == 0;
        let summary = if complete_coverage_claimed {
            "Required incident spans are present as metadata, redacted summaries, or by-reference rows.".into()
        } else {
            format!(
                "{missing_required_span_count} required incident span(s) are missing and disclosed; diagnosis remains bounded by available evidence."
            )
        };

        Self {
            required_span_count,
            captured_required_span_count,
            missing_required_span_count,
            complete_coverage_claimed,
            summary,
        }
    }
}

/// Support-bundle link pinned by the incident workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSupportBundleLink {
    /// Support bundle id.
    pub support_bundle_id: String,
    /// Support-bundle manifest ref.
    pub manifest_ref: String,
    /// Local preview snapshot ref.
    pub preview_snapshot_ref: String,
    /// Redaction profile used by the linked bundle.
    pub redaction_profile_ref: String,
    /// Whether the linked preview can reopen without network access.
    pub can_reopen_without_network: bool,
}

/// Summary of one support runbook packet attached to an incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentRunbookPacket {
    /// Runbook summary schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Support runbook packet id.
    pub runbook_packet_id: String,
    /// Runbook id.
    pub runbook_id: String,
    /// Packet version string.
    pub packet_version: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Source document ref from the support runbook packet.
    pub source_document_ref: String,
    /// Docs freshness state from the support runbook packet.
    pub docs_freshness_state: String,
    /// Default rollback posture.
    pub default_rollback_posture: String,
    /// Number of declared steps.
    pub step_count: u32,
    /// Number of mutating or rollback steps.
    pub mutating_step_count: u32,
    /// Support runbook schema ref consumed by this summary.
    pub support_schema_ref: String,
    /// Exact-build refs copied from the incident workspace.
    pub exact_build_refs: Vec<String>,
    /// Redaction class from the support runbook packet.
    pub redaction_class: String,
    /// Policy epoch ref from the support runbook packet.
    pub policy_epoch_ref: String,
    /// Execution-context ref from the support runbook packet.
    pub execution_context_id_ref: String,
    /// True when the packet can be exported as metadata with exact-build refs.
    pub exportable_with_redaction_controls: bool,
    /// Redaction-safe notes.
    pub notes: String,
}

impl IncidentRunbookPacket {
    /// Parses the existing support runbook packet YAML into an incident summary.
    ///
    /// # Errors
    ///
    /// Returns an error when YAML parsing fails, no steps are declared, or
    /// no exact-build refs are supplied for export linkage.
    pub fn from_support_runbook_yaml(
        yaml: &str,
        exact_build_refs: Vec<String>,
    ) -> Result<Self, IncidentRunbookPacketError> {
        if exact_build_refs.is_empty() {
            return Err(IncidentRunbookPacketError::MissingExactBuildRefs);
        }

        let wire: SupportRunbookPacketWire =
            serde_yaml::from_str(yaml).map_err(IncidentRunbookPacketError::Yaml)?;
        if wire.step_contracts.is_empty() {
            return Err(IncidentRunbookPacketError::EmptyStepContracts {
                runbook_packet_id: wire.runbook_packet_id,
            });
        }

        let mutating_step_count = wire
            .step_contracts
            .iter()
            .filter(|step| matches!(step.step_class.as_str(), "mitigate" | "rollback" | "custom"))
            .count() as u32;

        Ok(Self {
            schema_version: INCIDENT_RUNBOOK_PACKET_SCHEMA_VERSION,
            record_kind: INCIDENT_RUNBOOK_PACKET_RECORD_KIND.into(),
            runbook_packet_id: wire.runbook_packet_id,
            runbook_id: wire.runbook_id,
            packet_version: wire.packet_version,
            title: wire.title,
            source_document_ref: wire.source_document.source_ref,
            docs_freshness_state: wire.source_document.docs_freshness_state,
            default_rollback_posture: wire.default_rollback_posture,
            step_count: wire.step_contracts.len() as u32,
            mutating_step_count,
            support_schema_ref: SUPPORT_RUNBOOK_PACKET_SCHEMA_REF.into(),
            exact_build_refs,
            redaction_class: wire.redaction_class,
            policy_epoch_ref: wire.policy_context.policy_epoch,
            execution_context_id_ref: wire.policy_context.execution_context_id,
            exportable_with_redaction_controls: true,
            notes: "Runbook packet summary cites the support runbook schema and exact-build refs; raw commands and provider payloads stay out.".into(),
        })
    }
}

/// Errors raised when parsing a support runbook packet for incident use.
#[derive(Debug)]
pub enum IncidentRunbookPacketError {
    /// YAML did not parse into the expected support runbook shape.
    Yaml(serde_yaml::Error),
    /// The packet supplied no step contracts.
    EmptyStepContracts {
        /// Packet id that lacked step contracts.
        runbook_packet_id: String,
    },
    /// The caller did not supply exact-build refs.
    MissingExactBuildRefs,
}

impl fmt::Display for IncidentRunbookPacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "support runbook packet parse error: {err}"),
            Self::EmptyStepContracts { runbook_packet_id } => write!(
                f,
                "support runbook packet {runbook_packet_id} must declare at least one step"
            ),
            Self::MissingExactBuildRefs => {
                f.write_str("incident runbook packet export requires exact-build refs")
            }
        }
    }
}

impl Error for IncidentRunbookPacketError {}

/// Top-level incident workspace packet consumed by support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspacePacket {
    /// Incident workspace schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable incident workspace id.
    pub workspace_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Short redaction-safe summary.
    pub summary: String,
    /// RFC 3339 UTC timestamp for packet creation.
    pub generated_at: String,
    /// Exact build identity captured from support-bundle infrastructure.
    pub build_identity: BuildIdentity,
    /// Deployment profile class for the incident.
    pub deployment_profile_class: String,
    /// Provider lane state at generation time.
    pub provider_lane_state: ProviderLaneState,
    /// Local continuity state at generation time.
    pub local_continuity_state: LocalContinuityState,
    /// Policy context active at generation time.
    pub policy_context: IncidentPolicyContext,
    /// Evidence attachments.
    pub evidence_attachments: Vec<IncidentEvidenceAttachment>,
    /// Explicit missing spans.
    pub missing_spans: Vec<MissingSpan>,
    /// Span coverage rollup.
    pub span_coverage: SpanCoverageSummary,
    /// Runbook packet summaries.
    pub runbook_packets: Vec<IncidentRunbookPacket>,
    /// Linked support-bundle manifests and previews.
    pub support_bundle_links: Vec<IncidentSupportBundleLink>,
    /// Redaction-safe notes.
    pub notes: String,
}

impl IncidentWorkspacePacket {
    /// Returns true when at least one missing required span is disclosed.
    pub fn has_missing_required_spans(&self) -> bool {
        self.span_coverage.missing_required_span_count > 0
    }

    /// Returns true when a missing required span prevents a complete coverage claim.
    pub fn span_coverage_is_honest(&self) -> bool {
        self.span_coverage.complete_coverage_claimed != self.has_missing_required_spans()
    }

    /// Returns true when this workspace remains usable without hosted/provider lanes.
    pub fn stays_usable_without_provider_lane(&self) -> bool {
        self.provider_lane_state.requires_local_continuity()
            && self.local_continuity_state.permits_local_review()
    }

    /// Returns true when an evidence kind is attached.
    pub fn has_evidence_kind(&self, kind: IncidentEvidenceKind) -> bool {
        self.evidence_attachments
            .iter()
            .any(|item| item.evidence_kind == kind)
    }

    /// Builds a redacted support preview for the incident workspace packet.
    ///
    /// # Errors
    ///
    /// Returns an error when exact-build refs are missing or when the
    /// support-bundle preview builder rejects the generated rows.
    pub fn redacted_export_preview(
        &self,
        emitted_at: impl Into<String>,
    ) -> Result<SupportBundlePreview, IncidentExportError> {
        if self.build_identity.exact_build_refs.is_empty() {
            return Err(IncidentExportError::MissingExactBuildIdentity);
        }

        let emitted_at = emitted_at.into();
        let bundle_id = format!(
            "support-bundle:incident-workspace:{}",
            id_suffix(&self.workspace_id)
        );
        let mut builder = SupportBundlePreviewBuilder::new(
            bundle_id,
            format!("Incident workspace export for {}", self.title),
            emitted_at,
            self.exact_build_capture(),
        )
        .with_actor_class(ActorClass::SupportCenterPreview)
        .with_policy_context(PolicyContext {
            policy_epoch: 1,
            trust_state: trust_state_for_support(&self.policy_context.trust_state),
            policy_bundle_ref: Some(self.policy_context.policy_epoch_ref.clone()),
        })
        .with_collection_intent(
            "Preview a redacted incident workspace and runbook packet before export.",
        );

        builder.add_item(self.workspace_summary_seed());
        if !self.missing_spans.is_empty() {
            builder.add_item(self.missing_spans_seed());
        }

        for evidence in &self.evidence_attachments {
            builder.add_item(self.evidence_seed(evidence));
            if let Some(action_context) = &evidence.action_context {
                builder.add_action_reconstruction_context(
                    action_context.to_support_seed(&evidence.support_pack_item_id),
                );
            }
        }

        for runbook in &self.runbook_packets {
            builder.add_item(self.runbook_seed(runbook));
        }

        builder.build().map_err(IncidentExportError::SupportPreview)
    }

    fn exact_build_capture(&self) -> ExactBuildCapture {
        let mut capture = ExactBuildCapture::for_fixture(
            self.build_identity.build_id.clone(),
            self.build_identity.product_version.clone(),
            self.build_identity.release_channel_class,
        );
        for exact_ref in &self.build_identity.exact_build_refs {
            capture = capture.with_extra_exact_build_ref(exact_ref.clone());
        }
        capture
    }

    fn workspace_summary_seed(&self) -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: SUPPORT_ITEM_INCIDENT_WORKSPACE_SUMMARY.into(),
            title: "Incident workspace summary".into(),
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            bundle_section_class: "incident_workspace_truth".into(),
            artifact_kind_class: IncidentEvidenceKind::WorkspaceSummary
                .artifact_kind_class()
                .into(),
            manifest_path_ref: "incident_workspace.summary".into(),
            bundle_member_path_ref: Some("incident/workspace_summary.json".into()),
            source_refs: vec![
                self.workspace_id.clone(),
                self.policy_context.execution_context_id.clone(),
            ],
            size_estimate: SizeEstimate {
                estimated_bytes: Some(4096),
                confidence_class: "estimated".into(),
                display_label: "4 KB".into(),
                size_source_class: "incident_workspace_summary_estimate".into(),
            },
            impact_class: ActionabilityImpactClass::High,
            impact_summary:
                "Without the workspace summary, support cannot join evidence to provider and continuity state."
                    .into(),
            notes: self.span_coverage.summary.clone(),
        }
    }

    fn missing_spans_seed(&self) -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: SUPPORT_ITEM_INCIDENT_MISSING_SPANS.into(),
            title: "Missing spans and unavailable evidence".into(),
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            bundle_section_class: "incident_workspace_truth".into(),
            artifact_kind_class: IncidentEvidenceKind::MissingSpanDisclosure
                .artifact_kind_class()
                .into(),
            manifest_path_ref: "incident_workspace.missing_spans".into(),
            bundle_member_path_ref: Some("incident/missing_spans.json".into()),
            source_refs: self
                .missing_spans
                .iter()
                .map(|span| span.missing_span_id.clone())
                .collect(),
            size_estimate: SizeEstimate {
                estimated_bytes: None,
                confidence_class: "unknown_missing_span".into(),
                display_label: "unknown".into(),
                size_source_class: "missing_span_disclosure".into(),
            },
            impact_class: ActionabilityImpactClass::BlocksFirstActionableDiagnosis,
            impact_summary:
                "The packet remains usable, but missing spans limit first-diagnosis confidence."
                    .into(),
            notes: "Missing spans are exported as typed gap markers instead of empty evidence."
                .into(),
        }
    }

    fn evidence_seed(&self, evidence: &IncidentEvidenceAttachment) -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: evidence.support_pack_item_id.clone(),
            title: evidence.title.clone(),
            data_class: evidence.data_class,
            high_risk_content_class: evidence.high_risk_content_class,
            bundle_section_class: "incident_workspace_evidence".into(),
            artifact_kind_class: evidence.evidence_kind.artifact_kind_class().into(),
            manifest_path_ref: format!("incident_workspace.evidence.{}", evidence.evidence_id),
            bundle_member_path_ref: if evidence.availability == EvidenceAvailability::LocalOnly {
                None
            } else {
                Some(format!(
                    "incident/evidence/{}.json",
                    id_suffix(&evidence.evidence_id)
                ))
            },
            source_refs: if evidence.source_refs.is_empty() {
                vec![evidence.evidence_id.clone()]
            } else {
                evidence.source_refs.clone()
            },
            size_estimate: size_estimate(
                evidence.estimated_bytes,
                evidence.availability == EvidenceAvailability::Missing,
            ),
            impact_class: if evidence.required_for_first_diagnosis {
                ActionabilityImpactClass::High
            } else {
                ActionabilityImpactClass::Low
            },
            impact_summary: if evidence.required_for_first_diagnosis {
                "Evidence contributes to first actionable diagnosis.".into()
            } else {
                "Evidence is supporting context and may be omitted without blocking diagnosis."
                    .into()
            },
            notes: evidence.notes.clone(),
        }
    }

    fn runbook_seed(&self, runbook: &IncidentRunbookPacket) -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: support_item_id("incident.runbook", &runbook.runbook_packet_id),
            title: format!("Runbook packet: {}", runbook.title),
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            bundle_section_class: "incident_runbook_packets".into(),
            artifact_kind_class: IncidentEvidenceKind::RunbookPacket
                .artifact_kind_class()
                .into(),
            manifest_path_ref: format!("incident_workspace.runbook.{}", runbook.runbook_packet_id),
            bundle_member_path_ref: Some(format!(
                "incident/runbooks/{}.json",
                id_suffix(&runbook.runbook_packet_id)
            )),
            source_refs: vec![
                runbook.runbook_packet_id.clone(),
                runbook.source_document_ref.clone(),
                runbook.support_schema_ref.clone(),
            ],
            size_estimate: SizeEstimate {
                estimated_bytes: Some(6144),
                confidence_class: "estimated".into(),
                display_label: "6 KB".into(),
                size_source_class: "runbook_packet_summary_estimate".into(),
            },
            impact_class: ActionabilityImpactClass::High,
            impact_summary:
                "Runbook metadata preserves source freshness, rollback posture, and exact-build refs."
                    .into(),
            notes: runbook.notes.clone(),
        }
    }
}

/// Errors raised while projecting an incident workspace into a support preview.
#[derive(Debug)]
pub enum IncidentExportError {
    /// The incident workspace did not carry exact-build refs.
    MissingExactBuildIdentity,
    /// The support-bundle preview builder rejected the generated projection.
    SupportPreview(SupportBundlePreviewError),
}

impl fmt::Display for IncidentExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingExactBuildIdentity => {
                f.write_str("incident workspace export requires exact-build identity")
            }
            Self::SupportPreview(err) => write!(f, "support preview projection failed: {err}"),
        }
    }
}

impl Error for IncidentExportError {}

/// Builder for an [`IncidentWorkspacePacket`].
pub struct IncidentWorkspaceBuilder {
    workspace_id: String,
    title: String,
    summary: String,
    generated_at: String,
    exact_build: ExactBuildCapture,
    deployment_profile_class: String,
    provider_lane_state: ProviderLaneState,
    local_continuity_state: LocalContinuityState,
    policy_context: IncidentPolicyContext,
    evidence_attachments: Vec<IncidentEvidenceAttachment>,
    missing_spans: Vec<MissingSpan>,
    runbook_packets: Vec<IncidentRunbookPacket>,
    support_bundle_links: Vec<IncidentSupportBundleLink>,
    notes: String,
}

impl IncidentWorkspaceBuilder {
    /// Creates a new incident workspace builder.
    pub fn new(
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        generated_at: impl Into<String>,
        exact_build: ExactBuildCapture,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            title: title.into(),
            summary: summary.into(),
            generated_at: generated_at.into(),
            exact_build,
            deployment_profile_class: "individual_local".into(),
            provider_lane_state: ProviderLaneState::NotConfigured,
            local_continuity_state: LocalContinuityState::LocalDiagnosticsAvailable,
            policy_context: IncidentPolicyContext::local_fixture(),
            evidence_attachments: Vec::new(),
            missing_spans: Vec::new(),
            runbook_packets: Vec::new(),
            support_bundle_links: Vec::new(),
            notes: "Incident workspace packet is read-mostly; mutating actions stay in runbook and action-ledger refs.".into(),
        }
    }

    /// Sets the deployment profile class.
    pub fn with_deployment_profile_class(
        mut self,
        deployment_profile_class: impl Into<String>,
    ) -> Self {
        self.deployment_profile_class = deployment_profile_class.into();
        self
    }

    /// Sets provider lane state.
    pub fn with_provider_lane_state(mut self, provider_lane_state: ProviderLaneState) -> Self {
        self.provider_lane_state = provider_lane_state;
        self
    }

    /// Sets local continuity state.
    pub fn with_local_continuity_state(
        mut self,
        local_continuity_state: LocalContinuityState,
    ) -> Self {
        self.local_continuity_state = local_continuity_state;
        self
    }

    /// Sets policy context.
    pub fn with_policy_context(mut self, policy_context: IncidentPolicyContext) -> Self {
        self.policy_context = policy_context;
        self
    }

    /// Adds one evidence attachment.
    pub fn add_evidence(&mut self, evidence: IncidentEvidenceAttachment) -> &mut Self {
        self.evidence_attachments.push(evidence);
        self
    }

    /// Adds one missing-span marker.
    pub fn add_missing_span(&mut self, missing_span: MissingSpan) -> &mut Self {
        self.missing_spans.push(missing_span);
        self
    }

    /// Adds one support runbook packet summary.
    pub fn add_runbook_packet(&mut self, mut runbook: IncidentRunbookPacket) -> &mut Self {
        if runbook.exact_build_refs.is_empty() {
            runbook.exact_build_refs = self.exact_build.exact_build_refs.clone();
        }
        self.runbook_packets.push(runbook);
        self
    }

    /// Attaches a crash incident trail and records missing crash joins honestly.
    pub fn attach_crash_trail(&mut self, trail: &CrashIncidentTrail) -> &mut Self {
        self.add_evidence(IncidentEvidenceAttachment::crash_reference(trail));
        if trail.symbolication_state == SymbolicationState::Missing {
            self.add_missing_span(
                MissingSpan::new(
                    format!("missing-span:{}:symbolication", trail.incident_trail_id),
                    MissingSpanKind::SymbolicationReport,
                    MissingSpanReasonClass::SymbolicationUnavailable,
                    true,
                    MissingSpanImpactClass::WeakensFirstDiagnosis,
                    "No symbolication report was available for the crash trail.",
                )
                .with_expected_source_ref(trail.crash_envelope_ref.clone()),
            );
        }
        if trail
            .support_bundle_linkage
            .support_bundle_manifest_ref
            .is_none()
        {
            self.add_missing_span(
                MissingSpan::new(
                    format!(
                        "missing-span:{}:support-bundle-manifest",
                        trail.incident_trail_id
                    ),
                    MissingSpanKind::SupportBundleManifest,
                    MissingSpanReasonClass::NotCollected,
                    true,
                    MissingSpanImpactClass::WeakensFirstDiagnosis,
                    "Crash trail did not have a support-bundle manifest ref.",
                )
                .with_expected_source_ref(trail.support_bundle_linkage.support_bundle_ref.clone()),
            );
        }
        self
    }

    /// Attaches a support-bundle preview and records its manifest refs.
    pub fn attach_support_bundle_preview(&mut self, preview: &SupportBundlePreview) -> &mut Self {
        let link = IncidentSupportBundleLink {
            support_bundle_id: preview.manifest.support_bundle_id.clone(),
            manifest_ref: preview.manifest.manifest_id.clone(),
            preview_snapshot_ref: preview.preview_snapshot_ref.clone(),
            redaction_profile_ref: preview
                .manifest
                .collection_context
                .active_redaction_profile_ref
                .clone(),
            can_reopen_without_network: preview
                .manifest
                .reopen_after_export_path
                .can_reopen_without_network,
        };
        self.add_evidence(IncidentEvidenceAttachment::support_bundle(
            link.support_bundle_id.clone(),
            link.manifest_ref.clone(),
            link.preview_snapshot_ref.clone(),
        ));
        self.support_bundle_links.push(link);
        self
    }

    /// Builds the incident workspace packet.
    pub fn build(self) -> IncidentWorkspacePacket {
        let span_coverage =
            SpanCoverageSummary::from_parts(&self.evidence_attachments, &self.missing_spans);

        IncidentWorkspacePacket {
            schema_version: INCIDENT_WORKSPACE_PACKET_SCHEMA_VERSION,
            record_kind: INCIDENT_WORKSPACE_PACKET_RECORD_KIND.into(),
            workspace_id: self.workspace_id,
            title: self.title,
            summary: self.summary,
            generated_at: self.generated_at,
            build_identity: self.exact_build.to_build_identity(),
            deployment_profile_class: self.deployment_profile_class,
            provider_lane_state: self.provider_lane_state,
            local_continuity_state: self.local_continuity_state,
            policy_context: self.policy_context,
            evidence_attachments: self.evidence_attachments,
            missing_spans: self.missing_spans,
            span_coverage,
            runbook_packets: self.runbook_packets,
            support_bundle_links: self.support_bundle_links,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SupportRunbookPacketWire {
    runbook_packet_id: String,
    runbook_id: String,
    packet_version: String,
    title: String,
    source_document: SupportRunbookSourceDocumentWire,
    default_rollback_posture: String,
    step_contracts: Vec<SupportRunbookStepWire>,
    policy_context: SupportRunbookPolicyContextWire,
    redaction_class: String,
}

#[derive(Debug, Deserialize)]
struct SupportRunbookSourceDocumentWire {
    source_ref: String,
    docs_freshness_state: String,
}

#[derive(Debug, Deserialize)]
struct SupportRunbookStepWire {
    step_class: String,
}

#[derive(Debug, Deserialize)]
struct SupportRunbookPolicyContextWire {
    policy_epoch: String,
    execution_context_id: String,
}

fn trust_state_for_support(token: &str) -> aureline_support::bundle::TrustState {
    match token {
        "untrusted" => aureline_support::bundle::TrustState::Untrusted,
        "restricted" => aureline_support::bundle::TrustState::Restricted,
        "managed_admin" => aureline_support::bundle::TrustState::ManagedAdmin,
        _ => aureline_support::bundle::TrustState::Trusted,
    }
}

fn support_item_id(prefix: &str, raw_id: &str) -> String {
    format!("support.item.{}.{}", prefix, id_suffix(raw_id))
}

fn id_suffix(raw_id: &str) -> String {
    let mut suffix = String::with_capacity(raw_id.len());
    let mut last_was_dot = false;
    for ch in raw_id.chars() {
        let next = if ch.is_ascii_alphanumeric() {
            last_was_dot = false;
            ch.to_ascii_lowercase()
        } else if last_was_dot {
            continue;
        } else {
            last_was_dot = true;
            '.'
        };
        suffix.push(next);
    }
    let suffix = suffix.trim_matches('.');
    if suffix.is_empty() {
        "item".into()
    } else {
        suffix.into()
    }
}

fn size_estimate(estimated_bytes: Option<u64>, missing: bool) -> SizeEstimate {
    SizeEstimate {
        estimated_bytes,
        confidence_class: if missing {
            "unknown_missing_span".into()
        } else if estimated_bytes.is_some() {
            "estimated".into()
        } else {
            "unknown".into()
        },
        display_label: estimated_bytes
            .map(|bytes| format!("{bytes} bytes"))
            .unwrap_or_else(|| "unknown".into()),
        size_source_class: if missing {
            "missing_span_disclosure".into()
        } else {
            "collector_estimate".into()
        },
    }
}

/// Returns unique evidence kinds in stable order.
pub fn evidence_kinds(evidence: &[IncidentEvidenceAttachment]) -> Vec<IncidentEvidenceKind> {
    let set = evidence
        .iter()
        .map(|item| item.evidence_kind)
        .collect::<BTreeSet<_>>();
    set.into_iter().collect()
}

/// Returns a deterministic exact-build capture for incident tests and fixtures.
pub fn fixture_exact_build_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a",
        "0.8.0-alpha.1",
        ReleaseChannelClass::Preview,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_suffix_collapses_unsafe_characters() {
        assert_eq!(
            id_suffix("crash:path/With Spaces"),
            "crash.path.with.spaces"
        );
        assert_eq!(id_suffix("///"), "item");
    }

    #[test]
    fn missing_span_coverage_disables_complete_claim() {
        let evidence = vec![IncidentEvidenceAttachment::log_slice(
            "log:renderer:window",
            "log-slice:renderer:window",
            Some(2048),
        )];
        let spans = vec![MissingSpan::new(
            "missing-span:trace:renderer",
            MissingSpanKind::TraceWindow,
            MissingSpanReasonClass::SourceExpired,
            true,
            MissingSpanImpactClass::WeakensFirstDiagnosis,
            "Trace window expired before export.",
        )];
        let coverage = SpanCoverageSummary::from_parts(&evidence, &spans);

        assert_eq!(coverage.missing_required_span_count, 1);
        assert!(!coverage.complete_coverage_claimed);
    }
}
