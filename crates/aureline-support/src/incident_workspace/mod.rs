//! Governed incident-workspace runbook packets for beta escalations.
//!
//! This module joins the incident header, environment scope, evidence
//! timeline, runbook packet, action ledger, console handoff metadata, and
//! redacted export bundle into one durable object. The object is
//! metadata-safe by construction: it stores stable refs, closed-vocabulary
//! authority states, and omission markers instead of raw console sessions,
//! command bodies, provider URLs, private content, or secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for beta incident-workspace runbook packets.
pub const INCIDENT_WORKSPACE_RUNBOOK_RECORD_KIND: &str = "incident_workspace_runbook_beta_record";

/// Frozen schema version for beta incident-workspace runbook packets.
pub const INCIDENT_WORKSPACE_RUNBOOK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative schema path for the action ledger and workspace packet.
pub const INCIDENT_ACTION_LEDGER_SCHEMA_REF: &str =
    "schemas/support/incident_action_ledger.schema.json";

/// Repo-relative schema path for support runbook packets.
pub const SUPPORT_RUNBOOK_PACKET_SCHEMA_REF: &str = "schemas/support/runbook_packet.schema.json";

/// Repo-relative reviewer doc path.
pub const INCIDENT_WORKSPACE_RUNBOOK_DOC_REF: &str =
    "docs/support/m3/incident_workspace_runbook_beta.md";

/// Repo-relative protected fixture directory.
pub const INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR: &str =
    "fixtures/support/m3/runbook_packet_and_handoff";

/// Repo-relative protected fixture manifest.
pub const INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_MANIFEST_REF: &str =
    "fixtures/support/m3/runbook_packet_and_handoff/manifest.yaml";

const FIXTURE_SOURCES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/runbook_packet_and_handoff/current_runbook_with_approved_mitigation.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/current_runbook_with_approved_mitigation.yaml"
        )),
    ),
    (
        "fixtures/support/m3/runbook_packet_and_handoff/missing_evidence_fail_closed.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/missing_evidence_fail_closed.yaml"
        )),
    ),
    (
        "fixtures/support/m3/runbook_packet_and_handoff/stale_runbook_version_blocked.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/stale_runbook_version_blocked.yaml"
        )),
    ),
    (
        "fixtures/support/m3/runbook_packet_and_handoff/blocked_approval_mitigation.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/blocked_approval_mitigation.yaml"
        )),
    ),
    (
        "fixtures/support/m3/runbook_packet_and_handoff/browser_only_vendor_docs_handoff.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/browser_only_vendor_docs_handoff.yaml"
        )),
    ),
    (
        "fixtures/support/m3/runbook_packet_and_handoff/redacted_export_bundle.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/runbook_packet_and_handoff/redacted_export_bundle.yaml"
        )),
    ),
];

/// Closed runbook source-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceClass {
    /// Versioned runbook checked into the workspace or repo.
    RepoLocalRunbook,
    /// Reviewed docs pack with signer and freshness metadata.
    ReviewedDocsPack,
    /// Managed enterprise knowledge source with auditable versioning.
    EnterpriseKnowledgeStore,
    /// Vendor documentation reachable only through a browser.
    BrowserOnlyVendorDocs,
    /// Support-authored packet prepared for a specific case.
    SupportAuthoredPacket,
    /// Imported replay packet from a previous incident or export.
    ImportedReplayPacket,
}

impl RunbookSourceClass {
    /// Returns true when the source may not claim executable authority.
    pub const fn is_reference_only(self) -> bool {
        matches!(self, Self::BrowserOnlyVendorDocs)
    }
}

/// Closed runbook step-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepClass {
    /// Read-only evidence gathering.
    Observe,
    /// Read-only validation of scope, health, or expected behavior.
    Verify,
    /// Protected target mutation or mitigation.
    Mitigate,
    /// Rollback, restore, or compensating action.
    Rollback,
    /// External or internal communication step.
    Communicate,
}

impl RunbookStepClass {
    /// Returns true for steps that can change protected state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }
}

/// Closed target selector scope vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetSelectorScope {
    /// Local workspace target.
    LocalWorkspace,
    /// Runtime target selected by execution context.
    RuntimeTarget,
    /// Environment-scoped target such as region or namespace.
    EnvironmentScope,
    /// Service or resource-level target.
    ServiceResource,
    /// External browser or console target.
    BrowserConsoleExternal,
    /// Selector could not be resolved and requires review.
    UnresolvedRequiresReview,
}

impl TargetSelectorScope {
    /// Returns true when the target identity is not resolved in-product.
    pub const fn requires_external_handoff(self) -> bool {
        matches!(
            self,
            Self::BrowserConsoleExternal | Self::UnresolvedRequiresReview
        )
    }
}

/// Runbook document freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFreshnessState {
    /// The runbook version is current.
    ExactCurrent,
    /// The runbook is within an accepted grace window.
    WarmWithinGrace,
    /// The runbook is stale and requires review before mutation.
    StaleRequiresReview,
    /// A newer runbook supersedes this packet.
    SupersededRequiresUpgrade,
    /// The required runbook source is missing.
    MissingRequiresBlock,
    /// The source is browser-only reference material.
    BrowserOnlyReference,
}

impl DocsFreshnessState {
    /// Returns true when live mutation can be considered after other gates pass.
    pub const fn allows_live_mutation(self) -> bool {
        matches!(self, Self::ExactCurrent | Self::WarmWithinGrace)
    }
}

/// Approval requirement posture for a runbook step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirementClass {
    /// No approval required.
    NoApprovalRequired,
    /// Runtime approval ticket required.
    RuntimeApprovalTicket,
    /// Two-person approval required.
    TwoPersonApproval,
    /// Policy grant required.
    PolicyGrant,
    /// Break-glass approval required.
    BreakGlass,
    /// Action is forbidden by policy.
    ApprovalForbidden,
}

/// Current approval state for a runbook step or ledger entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalState {
    /// The step never required approval.
    NotRequired,
    /// A current approval or grant is present.
    Current,
    /// Approval is pending.
    Pending,
    /// Approval is blocked.
    Blocked,
    /// Approval expired.
    Expired,
    /// Approval was revoked.
    Revoked,
    /// Approval was required but missing.
    Missing,
    /// Policy forbids approval for this action.
    Forbidden,
}

impl ApprovalState {
    /// Returns true when the state grants live authority.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::NotRequired | Self::Current)
    }
}

/// Current step state shown in the incident workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepState {
    /// Step has not started.
    Planned,
    /// Step is ready after all gates pass.
    Ready,
    /// Step is waiting on approval.
    WaitingApproval,
    /// Step is blocked and must not execute.
    Blocked,
    /// Step completed with required evidence.
    Completed,
    /// Step deviated and carries a deviation note.
    Deviated,
    /// Step requires external handoff.
    HandoffRequired,
    /// Step rolled back or compensated prior work.
    RolledBack,
}

impl RunbookStepState {
    /// Returns true when a step claims live work occurred.
    pub const fn claims_execution(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::Completed | Self::Deviated | Self::RolledBack
        )
    }
}

/// Evidence output class expected by runbook steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedEvidenceClass {
    /// Signal slice ref from logs, metrics, or traces.
    SignalSliceRef,
    /// Incident timeline entry ref.
    IncidentTimelineEntryRef,
    /// Runbook step result ref.
    RunbookStepResultRef,
    /// Action ledger entry ref.
    ActionLedgerEntryRef,
    /// Approval ticket ref.
    ApprovalTicketRef,
    /// Console handoff metadata ref.
    ConsoleHandoffMetadataRef,
    /// Deviation note ref.
    DeviationNoteRef,
    /// Redacted export bundle ref.
    RedactedExportBundleRef,
}

/// Redaction state for evidence in the workspace and export bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRedactionState {
    /// Metadata can be embedded.
    MetadataEmbedded,
    /// Evidence is retained by reference only.
    ByReferenceOnly,
    /// Evidence is redacted by policy.
    RedactedByPolicy,
    /// Evidence is omitted with a declared reason.
    OmittedWithReason,
}

/// Evidence freshness state in the incident timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessState {
    /// Evidence is current.
    Live,
    /// Evidence is buffered but current enough to use.
    Buffered,
    /// Evidence is cached.
    Cached,
    /// Evidence was imported.
    Imported,
    /// Evidence is stale.
    Stale,
    /// Evidence is partial.
    Partial,
    /// Evidence is missing and must be declared.
    Missing,
    /// Evidence is unavailable offline.
    Offline,
}

/// Sensitivity class carried by evidence rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSensitivityClass {
    /// Metadata-only evidence.
    Metadata,
    /// Code-adjacent evidence.
    CodeAdjacent,
    /// Potentially high-risk operational evidence.
    PotentiallyHighRisk,
    /// Private triage-only evidence.
    PrivateTriageOnly,
}

/// Action class recorded in the incident action ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentActionClass {
    /// Observe action.
    Observe,
    /// Verify action.
    Verify,
    /// Mitigation action.
    Mitigate,
    /// Rollback action.
    Rollback,
    /// Communication action.
    Communicate,
    /// Browser or console handoff action.
    ConsoleHandoff,
    /// Redacted export bundle action.
    ExportBundle,
}

impl IncidentActionClass {
    /// Returns true for action classes that can mutate protected state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }
}

/// Outcome recorded in the incident action ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentActionOutcome {
    /// Observation completed.
    Observed,
    /// Verification completed.
    Verified,
    /// Protected action succeeded.
    Succeeded,
    /// Action was blocked before side effects.
    Blocked,
    /// Action deviated and has a note.
    Deviated,
    /// Rollback completed.
    RolledBack,
    /// Console handoff was launched.
    HandoffLaunched,
    /// Redacted export was produced.
    Exported,
    /// Action failed closed before mutation.
    FailedClosed,
}

impl IncidentActionOutcome {
    /// Returns true when the outcome claims side effects or completion.
    pub const fn claims_completed_work(self) -> bool {
        matches!(
            self,
            Self::Observed
                | Self::Verified
                | Self::Succeeded
                | Self::Deviated
                | Self::RolledBack
                | Self::HandoffLaunched
                | Self::Exported
        )
    }

    /// Returns true when the outcome records fail-closed behavior.
    pub const fn is_fail_closed(self) -> bool {
        matches!(self, Self::Blocked | Self::FailedClosed)
    }
}

/// Console handoff kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleHandoffKind {
    /// Browser-only documentation handoff.
    BrowserDocs,
    /// Vendor console handoff.
    VendorConsole,
    /// Provider-native approval flow.
    ProviderApproval,
    /// External control plane handoff.
    ExternalControlPlane,
}

/// Reason an external console handoff was required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleHandoffReasonClass {
    /// Vendor docs are reference-only and browser-owned.
    BrowserOnlyVendorDocs,
    /// Provider console is the system of record.
    ProviderConsoleOnly,
    /// In-product action is unsupported.
    InProductActionUnsupported,
    /// Policy requires external approval or review.
    PolicyRequiresExternalApproval,
    /// Target could not be verified in-product.
    TargetUnverifiable,
}

/// Drill class covered by a fixture packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentWorkspaceDrillClass {
    /// Missing evidence drill.
    MissingEvidence,
    /// Stale runbook version drill.
    StaleRunbookVersion,
    /// Blocked approval drill.
    BlockedApproval,
    /// Browser-only vendor documentation drill.
    BrowserOnlyVendorDocs,
    /// Redacted export bundle drill.
    RedactedExportBundle,
    /// Approved current runbook drill.
    ApprovedCurrentRunbook,
}

/// Required drill classes covered by the protected fixture corpus.
pub const REQUIRED_INCIDENT_WORKSPACE_DRILLS: [IncidentWorkspaceDrillClass; 5] = [
    IncidentWorkspaceDrillClass::MissingEvidence,
    IncidentWorkspaceDrillClass::StaleRunbookVersion,
    IncidentWorkspaceDrillClass::BlockedApproval,
    IncidentWorkspaceDrillClass::BrowserOnlyVendorDocs,
    IncidentWorkspaceDrillClass::RedactedExportBundle,
];

/// Returns the drill classes the corpus must cover.
pub fn required_incident_workspace_drills() -> &'static [IncidentWorkspaceDrillClass] {
    &REQUIRED_INCIDENT_WORKSPACE_DRILLS
}

/// One approval requirement attached to a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequirement {
    /// Stable approval requirement id.
    pub approval_requirement_id: String,
    /// Required approval class.
    pub requirement_class: ApprovalRequirementClass,
    /// Current approval state.
    pub current_state: ApprovalState,
    /// Current approval ticket ref, when present.
    #[serde(default)]
    pub approval_ticket_ref: Option<String>,
    /// Preview hash that the approval was granted against.
    #[serde(default)]
    pub preview_hash_ref: Option<String>,
    /// Whether this approval gates mutation.
    pub required_for_mutation: bool,
}

impl ApprovalRequirement {
    fn grants_live_mutation(&self) -> bool {
        self.current_state == ApprovalState::Current
            && self.approval_ticket_ref.as_deref().is_some_and(non_empty)
            && self.preview_hash_ref.as_deref().is_some_and(non_empty)
    }
}

/// One expected evidence output from a runbook step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedEvidenceOutput {
    /// Stable evidence-output id.
    pub evidence_output_id: String,
    /// Evidence class required.
    pub evidence_class: ExpectedEvidenceClass,
    /// Whether this output gates completion claims.
    pub required_for_completion: bool,
    /// Redaction posture for this output.
    pub redaction_state: EvidenceRedactionState,
}

/// Deviation-note policy for a runbook or step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNotePolicy {
    /// Whether deviation notes are supported.
    pub deviation_notes_supported: bool,
    /// Whether a deviation note is required when execution deviates.
    pub deviation_note_required_for_deviation: bool,
    /// Follow-up gate required after a deviation.
    #[serde(default)]
    pub follow_up_gate_ref: Option<String>,
}

/// Durable deviation note metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNote {
    /// Stable deviation note id.
    pub deviation_note_id: String,
    /// Actor that recorded the deviation.
    pub actor_ref: String,
    /// Reason class for the deviation.
    pub reason_class: String,
    /// Redaction-safe summary.
    pub summary: String,
    /// Evidence refs that justify or explain the deviation.
    pub evidence_refs: Vec<String>,
    /// UTC timestamp when the note was recorded.
    pub created_at: String,
}

/// Target selector rule declared by the runbook packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetSelectorRule {
    /// Stable target selector rule id.
    pub target_selector_rule_id: String,
    /// Target selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Resolved target identity, when available.
    #[serde(default)]
    pub target_identity_ref: Option<String>,
    /// Reviewer-safe selector summary.
    pub selector_summary: String,
}

/// One runbook step in the incident workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStep {
    /// Stable step id.
    pub step_id: String,
    /// Zero-based step ordinal.
    pub ordinal: u32,
    /// Reviewer-facing title.
    pub title: String,
    /// Step class.
    pub step_class: RunbookStepClass,
    /// Runbook source class for this step.
    pub source_class: RunbookSourceClass,
    /// Target selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Resolved target identity, when available.
    #[serde(default)]
    pub target_identity_ref: Option<String>,
    /// Approval requirement for this step.
    pub approval_requirement: ApprovalRequirement,
    /// Expected evidence outputs.
    pub expected_evidence_outputs: Vec<ExpectedEvidenceOutput>,
    /// Current step state.
    pub current_state: RunbookStepState,
    /// Evidence refs already attached to the step.
    pub evidence_refs: Vec<String>,
    /// Step-level deviation-note policy.
    pub deviation_note_policy: DeviationNotePolicy,
    /// Deviation note, when the step deviated.
    #[serde(default)]
    pub deviation_note: Option<DeviationNote>,
    /// Whether a browser or console handoff is required.
    pub handoff_required: bool,
    /// Console handoff ref, when required.
    #[serde(default)]
    pub console_handoff_ref: Option<String>,
}

/// Versioned runbook packet attached to an incident workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookPacket {
    /// Stable runbook packet id.
    pub runbook_packet_id: String,
    /// Stable runbook id.
    pub runbook_id: String,
    /// Packet version string.
    pub packet_version: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Runbook source class.
    pub source_class: RunbookSourceClass,
    /// Source document or catalog ref.
    pub source_ref: String,
    /// Freshness state for the source.
    pub docs_freshness_state: DocsFreshnessState,
    /// Runbook-level approval requirements.
    pub required_approvals: Vec<ApprovalRequirement>,
    /// Target selector rules.
    pub target_selector_rules: Vec<TargetSelectorRule>,
    /// Runbook-level expected evidence outputs.
    pub expected_evidence_outputs: Vec<ExpectedEvidenceOutput>,
    /// Ordered runbook steps.
    pub steps: Vec<RunbookStep>,
    /// Runbook-level deviation-note policy.
    pub deviation_policy: DeviationNotePolicy,
    /// Whether action ledger entries are required for execution.
    pub action_ledger_required: bool,
}

/// One action-ledger entry for an incident workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentActionLedgerEntry {
    /// Stable ledger entry id.
    pub action_id: String,
    /// Runbook id this action joins to.
    pub runbook_id_ref: String,
    /// Runbook packet version this action joins to.
    pub runbook_packet_version_ref: String,
    /// Step id this action joins to.
    #[serde(default)]
    pub step_id_ref: Option<String>,
    /// Step class for step-linked actions.
    pub step_class: RunbookStepClass,
    /// Action class.
    pub action_class: IncidentActionClass,
    /// Actor that initiated or recorded the action.
    pub actor_ref: String,
    /// Target selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Resolved target identity, when available.
    #[serde(default)]
    pub target_identity_ref: Option<String>,
    /// Preview hash ref used for mutation approval.
    #[serde(default)]
    pub preview_hash_ref: Option<String>,
    /// Approval ticket ref used for mutation approval.
    #[serde(default)]
    pub approval_ticket_ref: Option<String>,
    /// Current approval state.
    pub approval_state: ApprovalState,
    /// Sandbox profile or action-envelope ref.
    #[serde(default)]
    pub sandbox_profile_ref: Option<String>,
    /// Evidence refs attached to the action.
    pub evidence_refs: Vec<String>,
    /// Raw command or provider request ref, never the raw payload.
    #[serde(default)]
    pub raw_request_ref: Option<String>,
    /// Console handoff ref, when action launched a handoff.
    #[serde(default)]
    pub console_handoff_ref: Option<String>,
    /// Deviation note, when action deviated.
    #[serde(default)]
    pub deviation_note: Option<DeviationNote>,
    /// Action outcome.
    pub outcome: IncidentActionOutcome,
    /// UTC action timestamp.
    pub occurred_at: String,
    /// Whether the row is exportable as metadata.
    pub exportable_as_metadata: bool,
    /// Redaction class applied to this row.
    pub redaction_class: String,
}

/// Browser or console handoff metadata for an incident workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleHandoffMetadata {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Handoff kind.
    pub handoff_kind: ConsoleHandoffKind,
    /// Class of the destination URI, not the raw URI.
    pub target_console_uri_class: String,
    /// Target context ref preserved into the handoff.
    pub target_context_ref: String,
    /// Actor that launched or prepared the handoff.
    pub actor_ref: String,
    /// Incident workspace id preserved into the handoff.
    pub incident_workspace_id: String,
    /// Originating runbook step id.
    pub originating_step_id: String,
    /// Reason the handoff was required.
    pub reason_class: ConsoleHandoffReasonClass,
    /// Source class behind the handoff.
    pub source_class: RunbookSourceClass,
    /// Evidence refs preserved into the handoff.
    pub evidence_refs: Vec<String>,
    /// Return anchor ref, when the external surface can return context.
    #[serde(default)]
    pub return_anchor_ref: Option<String>,
    /// Approval ticket or handoff approval ref, when present.
    #[serde(default)]
    pub approval_ticket_ref: Option<String>,
    /// Whether Aureline supports this mutation in-product.
    pub in_product_mutation_supported: bool,
    /// Whether the external control plane is authoritative.
    pub external_control_plane_authoritative: bool,
    /// Whether the handoff row is exportable as metadata.
    pub exportable_as_metadata: bool,
}

/// Incident header shown by the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentHeader {
    /// Stable incident id.
    pub incident_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Severity token.
    pub severity: String,
    /// Declared owner ref.
    pub owner_ref: String,
    /// UTC start time.
    pub started_at: String,
    /// Current incident state.
    pub current_state: String,
}

/// Environment scope shown by the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentScope {
    /// Stable environment scope id.
    pub environment_scope_id: String,
    /// Service or resource owner ref.
    pub service_ref: String,
    /// Deployment profile class.
    pub deployment_profile_class: String,
    /// Target context ref.
    pub target_context_ref: String,
    /// Trust posture token.
    pub trust_posture: String,
    /// Write posture token.
    pub write_posture: String,
}

/// One evidence row in the workspace timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelineEntry {
    /// Stable timeline entry id.
    pub entry_id: String,
    /// Evidence class token.
    pub evidence_class: String,
    /// Evidence source class.
    pub source_class: RunbookSourceClass,
    /// Source ref or backend ref.
    pub source_ref: String,
    /// UTC event or capture time.
    pub occurred_at: String,
    /// Freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// Sensitivity class.
    pub sensitivity_class: EvidenceSensitivityClass,
    /// Target context ref.
    pub target_context_ref: String,
    /// Whether the evidence is redacted by default.
    pub redacted_by_default: bool,
    /// Whether any omission or missing state is declared explicitly.
    pub omission_declared: bool,
}

/// Redacted export bundle attached to the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentExportBundle {
    /// Stable export bundle id.
    pub bundle_id: String,
    /// Active redaction profile ref.
    pub redaction_profile_ref: String,
    /// Manifest digest ref.
    pub manifest_digest_ref: String,
    /// Included evidence refs.
    pub included_evidence_refs: Vec<String>,
    /// Omitted evidence refs with declared reasons elsewhere in the packet.
    pub omitted_evidence_refs: Vec<String>,
    /// Action ledger entry refs included in the export.
    pub action_ledger_entry_refs: Vec<String>,
    /// Console handoff refs included in the export.
    pub console_handoff_refs: Vec<String>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether secrets are excluded.
    pub secrets_excluded: bool,
    /// Whether the export can reconstruct runbook lineage.
    pub can_reconstruct_runbook_lineage: bool,
}

/// Privacy baseline for incident workspace packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspacePrivacyBaseline {
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether raw console sessions are excluded.
    pub raw_console_sessions_excluded: bool,
    /// Whether raw provider URLs are excluded.
    pub raw_provider_urls_excluded: bool,
    /// Whether secrets are excluded.
    pub secrets_excluded: bool,
}

/// Canonical references pinned by a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceReferences {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Action ledger schema ref.
    pub schema_ref: String,
    /// Support runbook schema ref.
    pub runbook_schema_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// One durable incident workspace object with runbook and handoff truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceRunbookPacket {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable workspace packet id.
    pub workspace_packet_id: String,
    /// Incident header.
    pub incident_header: IncidentHeader,
    /// Environment scope.
    pub environment_scope: EnvironmentScope,
    /// Drill classes this packet covers.
    pub drill_classes: Vec<IncidentWorkspaceDrillClass>,
    /// Evidence timeline rows.
    pub evidence_timeline: Vec<EvidenceTimelineEntry>,
    /// Runbook packet.
    pub runbook_packet: RunbookPacket,
    /// Action ledger entries.
    pub action_ledger: Vec<IncidentActionLedgerEntry>,
    /// Browser or console handoffs.
    pub console_handoffs: Vec<ConsoleHandoffMetadata>,
    /// Export bundle.
    pub export_bundle: IncidentExportBundle,
    /// Privacy baseline.
    pub privacy_baseline: IncidentWorkspacePrivacyBaseline,
    /// Canonical references.
    pub references: IncidentWorkspaceReferences,
    /// UTC packet creation time.
    pub emitted_at: String,
}

impl IncidentWorkspaceRunbookPacket {
    /// Returns true when the export bundle covers every action and handoff row.
    pub fn export_reconstructs_action_lineage(&self) -> bool {
        let exported_actions: BTreeSet<_> = self
            .export_bundle
            .action_ledger_entry_refs
            .iter()
            .map(String::as_str)
            .collect();
        let exported_handoffs: BTreeSet<_> = self
            .export_bundle
            .console_handoff_refs
            .iter()
            .map(String::as_str)
            .collect();
        self.action_ledger
            .iter()
            .all(|entry| exported_actions.contains(entry.action_id.as_str()))
            && self
                .console_handoffs
                .iter()
                .all(|handoff| exported_handoffs.contains(handoff.handoff_id.as_str()))
            && self.export_bundle.can_reconstruct_runbook_lineage
    }

    /// Returns true when the packet contains a console handoff with the id.
    pub fn has_console_handoff(&self, handoff_id: &str) -> bool {
        self.console_handoffs
            .iter()
            .any(|handoff| handoff.handoff_id == handoff_id)
    }

    /// Returns true when the packet contains the drill class.
    pub fn covers_drill(&self, drill_class: IncidentWorkspaceDrillClass) -> bool {
        self.drill_classes.contains(&drill_class)
    }
}

/// One fixture-bound corpus entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceRunbookCorpusEntry {
    /// Repo-relative fixture ref.
    pub fixture_ref: String,
    /// Parsed packet.
    pub packet: IncidentWorkspaceRunbookPacket,
}

/// Protected fixture corpus for beta incident workspace runbook packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceRunbookCorpus {
    /// Fixture-bound entries.
    pub entries: Vec<IncidentWorkspaceRunbookCorpusEntry>,
}

impl IncidentWorkspaceRunbookCorpus {
    /// Returns packets without their fixture wrappers.
    pub fn packets(&self) -> impl Iterator<Item = &IncidentWorkspaceRunbookPacket> {
        self.entries.iter().map(|entry| &entry.packet)
    }

    /// Validates the corpus and every packet inside it.
    pub fn validate(&self) -> Vec<IncidentWorkspaceViolation> {
        let mut violations = Vec::new();
        if self.entries.is_empty() {
            push_violation(
                &mut violations,
                "corpus.empty",
                INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR,
                "corpus must contain at least one incident workspace packet",
            );
            return violations;
        }

        let mut fixture_refs = BTreeSet::new();
        let mut packet_ids = BTreeSet::new();
        let mut covered_drills = BTreeSet::new();
        for entry in &self.entries {
            if !fixture_refs.insert(entry.fixture_ref.as_str()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_fixture_ref",
                    &entry.fixture_ref,
                    "fixture_ref must be unique",
                );
            }
            if !packet_ids.insert(entry.packet.workspace_packet_id.as_str()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_workspace_packet_id",
                    &entry.packet.workspace_packet_id,
                    "workspace_packet_id must be unique",
                );
            }
            covered_drills.extend(entry.packet.drill_classes.iter().copied());
            validate_packet(&mut violations, &entry.packet);
        }

        for required in required_incident_workspace_drills() {
            if !covered_drills.contains(required) {
                push_violation(
                    &mut violations,
                    "corpus.required_drill_missing",
                    INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR,
                    format!("missing required drill class {required:?}"),
                );
            }
        }

        violations
    }
}

/// Validation violation emitted by the incident workspace validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceViolation {
    /// Stable check id.
    pub check_id: String,
    /// Packet id, fixture ref, or row id that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Error returned when a checked-in incident workspace corpus cannot load.
#[derive(Debug)]
pub enum IncidentWorkspaceLoadError {
    /// One fixture failed to parse.
    Fixture {
        /// Fixture path that failed.
        fixture_ref: &'static str,
        /// YAML parser error.
        source: serde_yaml::Error,
    },
}

impl fmt::Display for IncidentWorkspaceLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture {
                fixture_ref,
                source,
            } => write!(
                f,
                "incident workspace fixture {fixture_ref} parse error: {source}"
            ),
        }
    }
}

impl Error for IncidentWorkspaceLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fixture { source, .. } => Some(source),
        }
    }
}

/// Parses one incident workspace runbook packet from YAML.
///
/// # Errors
///
/// Returns an error when the YAML does not match
/// [`IncidentWorkspaceRunbookPacket`].
pub fn load_incident_workspace_runbook_packet(
    yaml: &str,
) -> Result<IncidentWorkspaceRunbookPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the checked-in incident workspace runbook fixture corpus.
///
/// # Errors
///
/// Returns an error when any fixture fails to parse.
pub fn current_incident_workspace_runbook_corpus(
) -> Result<IncidentWorkspaceRunbookCorpus, IncidentWorkspaceLoadError> {
    let mut entries = Vec::with_capacity(FIXTURE_SOURCES.len());
    for (fixture_ref, contents) in FIXTURE_SOURCES.iter() {
        let packet = load_incident_workspace_runbook_packet(contents).map_err(|source| {
            IncidentWorkspaceLoadError::Fixture {
                fixture_ref: *fixture_ref,
                source,
            }
        })?;
        entries.push(IncidentWorkspaceRunbookCorpusEntry {
            fixture_ref: (*fixture_ref).to_owned(),
            packet,
        });
    }
    Ok(IncidentWorkspaceRunbookCorpus { entries })
}

/// Validates one incident workspace runbook packet.
pub fn validate_incident_workspace_runbook_packet(
    packet: &IncidentWorkspaceRunbookPacket,
) -> Vec<IncidentWorkspaceViolation> {
    let mut violations = Vec::new();
    validate_packet(&mut violations, packet);
    violations
}

fn validate_packet(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    packet: &IncidentWorkspaceRunbookPacket,
) {
    let target = packet.workspace_packet_id.as_str();

    if packet.schema_version != INCIDENT_WORKSPACE_RUNBOOK_SCHEMA_VERSION {
        push_violation(
            violations,
            "packet.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if packet.record_kind != INCIDENT_WORKSPACE_RUNBOOK_RECORD_KIND {
        push_violation(
            violations,
            "packet.record_kind",
            target,
            format!("record_kind must be {INCIDENT_WORKSPACE_RUNBOOK_RECORD_KIND}"),
        );
    }
    if !non_empty(&packet.workspace_packet_id) || !non_empty(&packet.emitted_at) {
        push_violation(
            violations,
            "packet.required_field_empty",
            target,
            "workspace_packet_id and emitted_at must be non-empty",
        );
    }

    validate_header(violations, target, &packet.incident_header);
    validate_environment_scope(violations, target, &packet.environment_scope);
    validate_evidence_timeline(violations, target, &packet.evidence_timeline);
    validate_runbook(violations, target, &packet.runbook_packet, packet);
    validate_handoffs(violations, target, &packet.console_handoffs, packet);
    validate_action_ledger(violations, target, &packet.action_ledger, packet);
    validate_export_bundle(violations, target, packet);
    validate_privacy_baseline(violations, target, &packet.privacy_baseline);
    validate_references(violations, target, &packet.references);
}

fn validate_header(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    header: &IncidentHeader,
) {
    for (field, value) in [
        ("incident_id", header.incident_id.as_str()),
        ("title", header.title.as_str()),
        ("severity", header.severity.as_str()),
        ("owner_ref", header.owner_ref.as_str()),
        ("started_at", header.started_at.as_str()),
        ("current_state", header.current_state.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.incident_header.empty",
                target,
                format!("incident_header.{field} must be non-empty"),
            );
        }
    }
}

fn validate_environment_scope(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    scope: &EnvironmentScope,
) {
    for (field, value) in [
        ("environment_scope_id", scope.environment_scope_id.as_str()),
        ("service_ref", scope.service_ref.as_str()),
        (
            "deployment_profile_class",
            scope.deployment_profile_class.as_str(),
        ),
        ("target_context_ref", scope.target_context_ref.as_str()),
        ("trust_posture", scope.trust_posture.as_str()),
        ("write_posture", scope.write_posture.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.environment_scope.empty",
                target,
                format!("environment_scope.{field} must be non-empty"),
            );
        }
    }
}

fn validate_evidence_timeline(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    timeline: &[EvidenceTimelineEntry],
) {
    if timeline.is_empty() {
        push_violation(
            violations,
            "packet.evidence_timeline.empty",
            target,
            "incident workspace must attach at least one evidence timeline row",
        );
        return;
    }
    let mut seen = BTreeSet::new();
    for row in timeline {
        if !seen.insert(row.entry_id.as_str()) {
            push_violation(
                violations,
                "packet.evidence_timeline.duplicate_entry_id",
                target,
                format!("duplicate evidence timeline entry {}", row.entry_id),
            );
        }
        for (field, value) in [
            ("entry_id", row.entry_id.as_str()),
            ("evidence_class", row.evidence_class.as_str()),
            ("source_ref", row.source_ref.as_str()),
            ("occurred_at", row.occurred_at.as_str()),
            ("target_context_ref", row.target_context_ref.as_str()),
        ] {
            if !non_empty(value) {
                push_violation(
                    violations,
                    "packet.evidence_timeline.row_empty",
                    target,
                    format!("evidence_timeline.{field} must be non-empty"),
                );
            }
        }
        if matches!(
            row.freshness_state,
            EvidenceFreshnessState::Missing
                | EvidenceFreshnessState::Stale
                | EvidenceFreshnessState::Partial
        ) && !row.omission_declared
        {
            push_violation(
                violations,
                "packet.evidence_timeline.omission_not_declared",
                &row.entry_id,
                "missing, stale, or partial evidence must declare the omission",
            );
        }
    }
}

fn validate_runbook(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    runbook: &RunbookPacket,
    packet: &IncidentWorkspaceRunbookPacket,
) {
    for (field, value) in [
        ("runbook_packet_id", runbook.runbook_packet_id.as_str()),
        ("runbook_id", runbook.runbook_id.as_str()),
        ("packet_version", runbook.packet_version.as_str()),
        ("title", runbook.title.as_str()),
        ("source_ref", runbook.source_ref.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.runbook.empty",
                target,
                format!("runbook_packet.{field} must be non-empty"),
            );
        }
    }
    if runbook.required_approvals.is_empty() {
        push_violation(
            violations,
            "packet.runbook.required_approvals.empty",
            target,
            "runbook packet must declare approval requirements",
        );
    }
    if runbook.target_selector_rules.is_empty() {
        push_violation(
            violations,
            "packet.runbook.target_selector_rules.empty",
            target,
            "runbook packet must declare target selector rules",
        );
    }
    if runbook.expected_evidence_outputs.is_empty() {
        push_violation(
            violations,
            "packet.runbook.expected_evidence_outputs.empty",
            target,
            "runbook packet must declare expected evidence outputs",
        );
    }
    if runbook.steps.is_empty() {
        push_violation(
            violations,
            "packet.runbook.steps.empty",
            target,
            "runbook packet must contain at least one step",
        );
    }
    if !runbook.deviation_policy.deviation_notes_supported {
        push_violation(
            violations,
            "packet.runbook.deviation_policy.unsupported",
            target,
            "runbook packet must support deviation notes",
        );
    }
    if !runbook.action_ledger_required {
        push_violation(
            violations,
            "packet.runbook.action_ledger_required",
            target,
            "runbook execution must require action ledger entries",
        );
    }

    let mut step_ids = BTreeSet::new();
    for step in &runbook.steps {
        if !step_ids.insert(step.step_id.as_str()) {
            push_violation(
                violations,
                "packet.runbook.steps.duplicate_step_id",
                target,
                format!("duplicate step_id {}", step.step_id),
            );
        }
        validate_step(violations, target, runbook, step, packet);
    }
}

fn validate_step(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    packet_target: &str,
    runbook: &RunbookPacket,
    step: &RunbookStep,
    packet: &IncidentWorkspaceRunbookPacket,
) {
    let target = step.step_id.as_str();
    for (field, value) in [
        ("step_id", step.step_id.as_str()),
        ("title", step.title.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.runbook.step.empty",
                packet_target,
                format!("step.{field} must be non-empty"),
            );
        }
    }
    if step.expected_evidence_outputs.is_empty() {
        push_violation(
            violations,
            "packet.runbook.step.expected_evidence_outputs.empty",
            target,
            "every runbook step must declare expected evidence outputs",
        );
    }
    for output in &step.expected_evidence_outputs {
        if !non_empty(&output.evidence_output_id) {
            push_violation(
                violations,
                "packet.runbook.step.expected_evidence_output.empty_id",
                target,
                "expected evidence output ids must be non-empty",
            );
        }
    }
    if !step.deviation_note_policy.deviation_notes_supported {
        push_violation(
            violations,
            "packet.runbook.step.deviation_policy.unsupported",
            target,
            "every step must support deviation notes",
        );
    }
    if matches!(step.current_state, RunbookStepState::Deviated) && step.deviation_note.is_none() {
        push_violation(
            violations,
            "packet.runbook.step.deviation_note_required",
            target,
            "deviated steps must attach a deviation note",
        );
    }
    if step.handoff_required || step.target_selector_scope.requires_external_handoff() {
        match step.console_handoff_ref.as_deref() {
            Some(handoff_ref) if packet.has_console_handoff(handoff_ref) => {}
            _ => push_violation(
                violations,
                "packet.runbook.step.handoff_metadata_required",
                target,
                "handoff steps must reference console handoff metadata",
            ),
        }
    }

    if !runbook.docs_freshness_state.allows_live_mutation()
        && step.step_class.is_mutating()
        && step.current_state.claims_execution()
        && !matches!(
            step.current_state,
            RunbookStepState::Blocked | RunbookStepState::HandoffRequired
        )
    {
        push_violation(
            violations,
            "packet.runbook.step.stale_runbook_blocks_mutation",
            target,
            "stale, missing, superseded, or browser-only runbooks must fail closed before mutation",
        );
    }

    if runbook.source_class.is_reference_only()
        && step.step_class.is_mutating()
        && !matches!(
            step.current_state,
            RunbookStepState::Blocked | RunbookStepState::HandoffRequired
        )
    {
        push_violation(
            violations,
            "packet.runbook.step.browser_docs_not_authority",
            target,
            "browser-only vendor docs are reference-only and cannot authorize mutation",
        );
    }

    if step.step_class.is_mutating() {
        let target_resolved = step.target_identity_ref.as_deref().is_some_and(non_empty);
        let has_evidence_requirements = step
            .expected_evidence_outputs
            .iter()
            .any(|output| output.required_for_completion);
        let authorized = step.approval_requirement.grants_live_mutation();
        let claims_mutation = step.current_state.claims_execution();

        if claims_mutation && !target_resolved {
            push_violation(
                violations,
                "packet.runbook.step.target_identity_required_for_mutation",
                target,
                "mutating steps that claim execution must carry a resolved target identity",
            );
        }
        if claims_mutation && !authorized {
            push_violation(
                violations,
                "packet.runbook.step.current_approval_required_for_mutation",
                target,
                "mutating steps that claim execution must carry current approval, ticket, and preview hash",
            );
        }
        if claims_mutation && !has_evidence_requirements {
            push_violation(
                violations,
                "packet.runbook.step.evidence_required_for_mutation",
                target,
                "mutating steps that claim execution must declare completion evidence requirements",
            );
        }
        if !authorized
            && !matches!(
                step.current_state,
                RunbookStepState::Blocked
                    | RunbookStepState::WaitingApproval
                    | RunbookStepState::Planned
                    | RunbookStepState::HandoffRequired
            )
        {
            push_violation(
                violations,
                "packet.runbook.step.fail_closed_without_approval",
                target,
                "mutating steps without current approval must remain blocked, waiting, planned, or handoff-required",
            );
        }
    }
}

fn validate_action_ledger(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    entries: &[IncidentActionLedgerEntry],
    packet: &IncidentWorkspaceRunbookPacket,
) {
    if entries.is_empty() {
        push_violation(
            violations,
            "packet.action_ledger.empty",
            target,
            "incident workspace must contain action ledger entries",
        );
        return;
    }

    let step_ids: BTreeSet<_> = packet
        .runbook_packet
        .steps
        .iter()
        .map(|step| step.step_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for entry in entries {
        if !seen.insert(entry.action_id.as_str()) {
            push_violation(
                violations,
                "packet.action_ledger.duplicate_action_id",
                target,
                format!("duplicate action id {}", entry.action_id),
            );
        }
        validate_action_ledger_entry(violations, target, entry, &step_ids, packet);
    }
}

fn validate_action_ledger_entry(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    packet_target: &str,
    entry: &IncidentActionLedgerEntry,
    step_ids: &BTreeSet<&str>,
    packet: &IncidentWorkspaceRunbookPacket,
) {
    let target = entry.action_id.as_str();
    for (field, value) in [
        ("action_id", entry.action_id.as_str()),
        ("runbook_id_ref", entry.runbook_id_ref.as_str()),
        (
            "runbook_packet_version_ref",
            entry.runbook_packet_version_ref.as_str(),
        ),
        ("actor_ref", entry.actor_ref.as_str()),
        ("occurred_at", entry.occurred_at.as_str()),
        ("redaction_class", entry.redaction_class.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.action_ledger.row_empty",
                packet_target,
                format!("action_ledger.{field} must be non-empty"),
            );
        }
    }
    if let Some(step_id) = entry.step_id_ref.as_deref() {
        if !step_ids.contains(step_id) {
            push_violation(
                violations,
                "packet.action_ledger.unknown_step_ref",
                target,
                "action ledger step_id_ref must point at a runbook step",
            );
        }
    }
    if !entry.exportable_as_metadata {
        push_violation(
            violations,
            "packet.action_ledger.exportable_as_metadata",
            target,
            "action ledger entries must be exportable as metadata",
        );
    }
    if matches!(entry.outcome, IncidentActionOutcome::Deviated) && entry.deviation_note.is_none() {
        push_violation(
            violations,
            "packet.action_ledger.deviation_note_required",
            target,
            "deviated action ledger entries must attach a deviation note",
        );
    }
    if entry.outcome.claims_completed_work() && entry.evidence_refs.is_empty() {
        push_violation(
            violations,
            "packet.action_ledger.evidence_required_for_completed_outcome",
            target,
            "completed action outcomes must attach evidence refs",
        );
    }
    if entry.outcome.is_fail_closed()
        && entry.evidence_refs.is_empty()
        && entry.deviation_note.is_none()
    {
        push_violation(
            violations,
            "packet.action_ledger.fail_closed_requires_explanation",
            target,
            "fail-closed ledger entries must attach evidence or a deviation note",
        );
    }
    if entry.console_handoff_ref.as_deref().is_some_and(non_empty)
        || entry.action_class == IncidentActionClass::ConsoleHandoff
        || entry.target_selector_scope.requires_external_handoff()
    {
        match entry.console_handoff_ref.as_deref() {
            Some(handoff_ref) if packet.has_console_handoff(handoff_ref) => {}
            _ => push_violation(
                violations,
                "packet.action_ledger.handoff_metadata_required",
                target,
                "console handoff actions must reference console handoff metadata",
            ),
        }
    }

    if entry.action_class.is_mutating() {
        let target_resolved = entry.target_identity_ref.as_deref().is_some_and(non_empty);
        let approval_current = entry.approval_state == ApprovalState::Current
            && entry.approval_ticket_ref.as_deref().is_some_and(non_empty)
            && entry.preview_hash_ref.as_deref().is_some_and(non_empty);
        let claims_success = matches!(
            entry.outcome,
            IncidentActionOutcome::Succeeded
                | IncidentActionOutcome::Deviated
                | IncidentActionOutcome::RolledBack
        );
        if claims_success && !target_resolved {
            push_violation(
                violations,
                "packet.action_ledger.target_identity_required_for_mutation",
                target,
                "successful mutating ledger entries must carry target identity",
            );
        }
        if claims_success && !approval_current {
            push_violation(
                violations,
                "packet.action_ledger.current_approval_required_for_mutation",
                target,
                "successful mutating ledger entries must carry current approval, ticket, and preview hash",
            );
        }
        if !approval_current && !entry.outcome.is_fail_closed() {
            push_violation(
                violations,
                "packet.action_ledger.fail_closed_without_approval",
                target,
                "mutating ledger entries without approval must fail closed",
            );
        }
    }
}

fn validate_handoffs(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    handoffs: &[ConsoleHandoffMetadata],
    packet: &IncidentWorkspaceRunbookPacket,
) {
    let mut seen = BTreeSet::new();
    for handoff in handoffs {
        if !seen.insert(handoff.handoff_id.as_str()) {
            push_violation(
                violations,
                "packet.console_handoffs.duplicate_handoff_id",
                target,
                format!("duplicate handoff id {}", handoff.handoff_id),
            );
        }
        for (field, value) in [
            ("handoff_id", handoff.handoff_id.as_str()),
            (
                "target_console_uri_class",
                handoff.target_console_uri_class.as_str(),
            ),
            ("target_context_ref", handoff.target_context_ref.as_str()),
            ("actor_ref", handoff.actor_ref.as_str()),
            (
                "incident_workspace_id",
                handoff.incident_workspace_id.as_str(),
            ),
            ("originating_step_id", handoff.originating_step_id.as_str()),
        ] {
            if !non_empty(value) {
                push_violation(
                    violations,
                    "packet.console_handoffs.row_empty",
                    target,
                    format!("console_handoff.{field} must be non-empty"),
                );
            }
        }
        if handoff.incident_workspace_id != packet.incident_header.incident_id {
            push_violation(
                violations,
                "packet.console_handoffs.incident_ref_mismatch",
                &handoff.handoff_id,
                "console handoff must preserve the incident id",
            );
        }
        if handoff.evidence_refs.is_empty() {
            push_violation(
                violations,
                "packet.console_handoffs.evidence_refs.empty",
                &handoff.handoff_id,
                "console handoff metadata must preserve evidence refs",
            );
        }
        if !handoff.exportable_as_metadata {
            push_violation(
                violations,
                "packet.console_handoffs.exportable_as_metadata",
                &handoff.handoff_id,
                "console handoff metadata must be exportable as metadata",
            );
        }
        if handoff.external_control_plane_authoritative && handoff.in_product_mutation_supported {
            push_violation(
                violations,
                "packet.console_handoffs.overclaims_in_product_parity",
                &handoff.handoff_id,
                "authoritative external control-plane handoffs must not claim in-product mutation parity",
            );
        }
        if handoff.source_class.is_reference_only() && !handoff.external_control_plane_authoritative
        {
            push_violation(
                violations,
                "packet.console_handoffs.browser_docs_boundary",
                &handoff.handoff_id,
                "browser-only vendor docs handoffs must preserve the external authority boundary",
            );
        }
    }
}

fn validate_export_bundle(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    packet: &IncidentWorkspaceRunbookPacket,
) {
    let bundle = &packet.export_bundle;
    for (field, value) in [
        ("bundle_id", bundle.bundle_id.as_str()),
        (
            "redaction_profile_ref",
            bundle.redaction_profile_ref.as_str(),
        ),
        ("manifest_digest_ref", bundle.manifest_digest_ref.as_str()),
    ] {
        if !non_empty(value) {
            push_violation(
                violations,
                "packet.export_bundle.empty",
                target,
                format!("export_bundle.{field} must be non-empty"),
            );
        }
    }
    if !bundle.raw_private_material_excluded || !bundle.secrets_excluded {
        push_violation(
            violations,
            "packet.export_bundle.privacy_baseline",
            target,
            "export bundle must exclude raw private material and secrets",
        );
    }
    if !bundle.can_reconstruct_runbook_lineage {
        push_violation(
            violations,
            "packet.export_bundle.can_reconstruct_runbook_lineage",
            target,
            "export bundle must reconstruct runbook lineage",
        );
    }
    if !packet.export_reconstructs_action_lineage() {
        push_violation(
            violations,
            "packet.export_bundle.action_lineage_incomplete",
            target,
            "export bundle must include every action ledger and console handoff ref",
        );
    }
}

fn validate_privacy_baseline(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    baseline: &IncidentWorkspacePrivacyBaseline,
) {
    if !baseline.raw_private_material_excluded
        || !baseline.ambient_authority_excluded
        || !baseline.raw_console_sessions_excluded
        || !baseline.raw_provider_urls_excluded
        || !baseline.secrets_excluded
    {
        push_violation(
            violations,
            "packet.privacy_baseline",
            target,
            "privacy baseline must exclude raw private material, ambient authority, raw console sessions, raw provider URLs, and secrets",
        );
    }
}

fn validate_references(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    target: &str,
    references: &IncidentWorkspaceReferences,
) {
    if references.doc_ref != INCIDENT_WORKSPACE_RUNBOOK_DOC_REF {
        push_violation(
            violations,
            "packet.references.doc_ref",
            target,
            format!("doc_ref must pin {INCIDENT_WORKSPACE_RUNBOOK_DOC_REF}"),
        );
    }
    if references.schema_ref != INCIDENT_ACTION_LEDGER_SCHEMA_REF {
        push_violation(
            violations,
            "packet.references.schema_ref",
            target,
            format!("schema_ref must pin {INCIDENT_ACTION_LEDGER_SCHEMA_REF}"),
        );
    }
    if references.runbook_schema_ref != SUPPORT_RUNBOOK_PACKET_SCHEMA_REF {
        push_violation(
            violations,
            "packet.references.runbook_schema_ref",
            target,
            format!("runbook_schema_ref must pin {SUPPORT_RUNBOOK_PACKET_SCHEMA_REF}"),
        );
    }
    if references.fixture_manifest_ref != INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_MANIFEST_REF {
        push_violation(
            violations,
            "packet.references.fixture_manifest_ref",
            target,
            format!(
                "fixture_manifest_ref must pin {INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_MANIFEST_REF}"
            ),
        );
    }
}

fn push_violation(
    violations: &mut Vec<IncidentWorkspaceViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(IncidentWorkspaceViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}
