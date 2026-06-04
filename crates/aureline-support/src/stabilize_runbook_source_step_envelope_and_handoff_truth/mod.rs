//! Stable runbook source, executable-step, deviation, and handoff truth.
//!
//! This module defines the support/export contract for governed runbook
//! execution. It keeps advisory prose separate from executable step envelopes,
//! binds mutating steps to the shared action-envelope path, and preserves
//! browser or vendor-console handoffs as explicit control-plane exits.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the task packet.
pub const RUNBOOK_HANDOFF_TRUTH_PACKET_RECORD_KIND: &str = "runbook_handoff_truth_packet";

/// Stable record-kind tag for a source descriptor.
pub const RUNBOOK_SOURCE_DESCRIPTOR_RECORD_KIND: &str = "runbook_source_descriptor";

/// Stable record-kind tag for an executable step envelope.
pub const RUNBOOK_STEP_ENVELOPE_RECORD_KIND: &str = "runbook_step_envelope";

/// Stable record-kind tag for an execution record.
pub const RUNBOOK_STEP_EXECUTION_RECORD_KIND: &str = "runbook_step_execution_record";

/// Stable record-kind tag for a deviation note.
pub const RUNBOOK_DEVIATION_NOTE_RECORD_KIND: &str = "runbook_deviation_note";

/// Stable record-kind tag for an external handoff bundle.
pub const RUNBOOK_HANDOFF_BUNDLE_RECORD_KIND: &str = "runbook_external_handoff_bundle";

/// Stable record-kind tag for local checklist completion records.
pub const RUNBOOK_LOCAL_FOLLOW_UP_RECORD_KIND: &str = "runbook_local_follow_up";

/// Integer schema version for the runbook handoff truth packet.
pub const RUNBOOK_HANDOFF_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the step-envelope boundary schema.
pub const RUNBOOK_STEP_ENVELOPE_SCHEMA_REF: &str =
    "schemas/support/runbook-step-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const RUNBOOK_HANDOFF_TRUTH_DOC_REF: &str =
    "docs/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth.md";

/// Repo-relative path of the human-readable support artifact.
pub const RUNBOOK_HANDOFF_TRUTH_ARTIFACT_REF: &str =
    "artifacts/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const RUNBOOK_HANDOFF_TRUTH_FIXTURE_DIR: &str =
    "fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth";

/// Repo-relative path of the fixture manifest.
pub const RUNBOOK_HANDOFF_TRUTH_FIXTURE_MANIFEST_REF: &str =
    "fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/manifest.yaml";

const FIXTURE_SOURCES: &[(&str, &str)] = &[
    (
        "fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/managed_catalog_execution_with_deviation.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/managed_catalog_execution_with_deviation.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/browser_vendor_console_handoff.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/browser_vendor_console_handoff.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/local_checklist_handoff_bundle.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/local_checklist_handoff_bundle.yaml"
        )),
    ),
];

/// Source class for runbook guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceClass {
    /// Versioned guidance checked into a repository or workspace.
    RepoLocalWorkspaceLocal,
    /// Mirrored documentation pack with reviewed freshness metadata.
    MirroredDocsPack,
    /// Managed catalog entry controlled by an administrator.
    ManagedCatalog,
    /// Browser-only vendor documentation.
    BrowserOnlyVendorDocumentation,
}

impl RunbookSourceClass {
    /// Required source classes for stable operational guidance coverage.
    pub const REQUIRED: [Self; 4] = [
        Self::RepoLocalWorkspaceLocal,
        Self::MirroredDocsPack,
        Self::ManagedCatalog,
        Self::BrowserOnlyVendorDocumentation,
    ];

    /// Returns true when the source cannot claim executable authority.
    pub const fn is_browser_only(self) -> bool {
        matches!(self, Self::BrowserOnlyVendorDocumentation)
    }
}

/// Authority posture a source may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritativePosture {
    /// Authoritative for in-product execution when other gates pass.
    Authoritative,
    /// Reference-only guidance with no execution authority.
    ReferenceOnly,
    /// Authority delegated through managed administration policy.
    ManagedDelegated,
    /// Authority is unverifiable and must be downgraded.
    UnverifiedDowngraded,
}

/// Freshness state for source guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// The source is current for the declared compatibility window.
    Current,
    /// The source is within a bounded grace window.
    WithinGrace,
    /// The source is stale and cannot drive mutation without review.
    StaleRequiresReview,
    /// The source is browser-only reference material.
    BrowserOnlyReference,
}

/// Export rights for source material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRightClass {
    /// Metadata may be exported.
    MetadataOnly,
    /// Redacted source excerpts may be exported by policy.
    RedactedSourceAllowed,
    /// Source may be referenced but not embedded.
    ReferenceOnly,
    /// Export is forbidden.
    ExportForbidden,
}

/// Closed step-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepClass {
    /// Read-only evidence gathering.
    Observe,
    /// Read-only validation.
    Verify,
    /// Protected mitigation or state change.
    Mitigate,
    /// Rollback or compensating state change.
    Rollback,
    /// Communication or stakeholder update.
    Communicate,
}

impl StepClass {
    /// Required step classes for stable runbook coverage.
    pub const REQUIRED: [Self; 5] = [
        Self::Observe,
        Self::Verify,
        Self::Mitigate,
        Self::Rollback,
        Self::Communicate,
    ];

    /// Returns true when a step can mutate protected state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }
}

/// Target-selector scope for a runbook step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetSelectorScope {
    /// Local workspace target.
    LocalWorkspace,
    /// Runtime target selected by execution context.
    RuntimeTarget,
    /// Provider-owned service or object.
    ProviderObject,
    /// Browser-only documentation or console target.
    BrowserVendorConsole,
}

impl TargetSelectorScope {
    /// Returns true when execution leaves the in-product target model.
    pub const fn requires_external_handoff(self) -> bool {
        matches!(self, Self::BrowserVendorConsole)
    }
}

/// Approval posture for a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirementClass {
    /// No approval is required for the read-only step.
    NoApprovalRequired,
    /// Runtime approval ticket is required.
    RuntimeApprovalTicket,
    /// Managed policy approval is required.
    ManagedPolicyApproval,
    /// Approval is forbidden because the step is reference-only.
    ApprovalForbidden,
}

/// Destination class for a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    /// Execution remains in Aureline.
    InProduct,
    /// The step opens browser-only documentation.
    BrowserDocumentation,
    /// The step pivots to a vendor console.
    VendorConsole,
    /// The step pivots to an external provider control plane.
    ProviderControlPlane,
}

impl DestinationClass {
    /// Returns true when the destination is outside Aureline execution.
    pub const fn is_external(self) -> bool {
        !matches!(self, Self::InProduct)
    }
}

/// Execution state for a runbook step result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepResultState {
    /// Step has rendered only a preview.
    PreviewOnly,
    /// Step was approved but has not executed.
    Approved,
    /// Step executed through the declared envelope.
    Executed,
    /// Step requires an explicit external handoff.
    HandoffRequired,
    /// Operator departed from the prescribed step sequence.
    Deviated,
}

impl StepResultState {
    /// Required step result states for UI, CLI, and export parity.
    pub const REQUIRED: [Self; 5] = [
        Self::PreviewOnly,
        Self::Approved,
        Self::Executed,
        Self::HandoffRequired,
        Self::Deviated,
    ];
}

/// Ownership class for follow-up objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderObjectOwnershipClass {
    /// The object is local to Aureline.
    AurelineLocal,
    /// The object is owned by an external provider.
    ProviderOwned,
    /// The object is a mirrored reference.
    MirroredReference,
}

/// Mutation meaning of local checklist completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCompletionState {
    /// Local completion only; no provider object changed.
    LocalChecklistOnly,
    /// A reviewed in-product command changed state.
    ReviewedCommandExecuted,
    /// External provider mutation was reported through a handoff return.
    ExternalProviderMutationReported,
}

/// One runbook source descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceDescriptor {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable source descriptor id.
    pub source_id: String,
    /// Source class.
    pub source_class: RunbookSourceClass,
    /// Authority posture.
    pub authoritative_posture: AuthoritativePosture,
    /// Signer or source owner reference.
    pub signer_or_source_ref: String,
    /// Freshness state.
    pub freshness_state: FreshnessState,
    /// Approver-policy reference.
    pub approver_policy_ref: String,
    /// Export rights.
    pub export_right: ExportRightClass,
}

/// Expected evidence output for a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedEvidenceOutput {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Evidence class token.
    pub evidence_class: String,
    /// Export-safe evidence ref.
    pub export_ref: String,
}

/// Shared action-envelope linkage required for mutating steps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedActionEnvelope {
    /// Shared command or action envelope reference.
    pub action_envelope_ref: String,
    /// Preview hash reference produced before approval.
    pub preview_hash_ref: String,
    /// Approval reference used by the shared approval system.
    pub approval_ref: String,
    /// Audit event reference.
    pub audit_ref: String,
}

/// Explicit handoff metadata for browser or provider-console pivots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHandoffRef {
    /// Stable handoff reference.
    pub handoff_ref: String,
    /// Destination class.
    pub destination_class: DestinationClass,
    /// Reason this handoff is required.
    pub reason: String,
    /// Return anchor or follow-up note reference.
    pub return_anchor_ref: Option<String>,
}

/// Executable runbook step envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepEnvelope {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable step id.
    pub step_id: String,
    /// Step class.
    pub step_class: StepClass,
    /// Target selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Approval requirement.
    pub approval_requirement: ApprovalRequirementClass,
    /// Destination class.
    pub destination_class: DestinationClass,
    /// Expected evidence outputs.
    pub expected_evidence_outputs: Vec<ExpectedEvidenceOutput>,
    /// Shared action envelope for mutating in-product steps.
    pub shared_action_envelope: Option<SharedActionEnvelope>,
    /// External handoff reference for browser or console destinations.
    pub external_handoff: Option<ExternalHandoffRef>,
}

/// Attributable execution record emitted for a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepExecutionRecord {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable execution id.
    pub execution_id: String,
    /// Step id this execution belongs to.
    pub step_id: String,
    /// Incident timeline id for chronology joins.
    pub incident_timeline_id: String,
    /// Actor reference.
    pub actor_ref: String,
    /// Step result state.
    pub result_state: StepResultState,
    /// Approval refs attached to this execution.
    pub approval_refs: Vec<String>,
    /// Deviation note refs attached to this execution.
    pub deviation_note_refs: Vec<String>,
    /// External handoff refs attached to this execution.
    pub external_console_handoff_refs: Vec<String>,
    /// Evidence or export links produced by this execution.
    pub evidence_export_links: Vec<String>,
}

/// Durable deviation note for a departed runbook sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNote {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable deviation note id.
    pub deviation_note_id: String,
    /// Step id departed from.
    pub departed_step_id: String,
    /// Actor reference.
    pub actor_ref: String,
    /// Incident timeline id for chronology joins.
    pub incident_timeline_id: String,
    /// Export-safe reason.
    pub reason: String,
    /// Evidence refs supporting the departure.
    pub evidence_refs: Vec<String>,
}

/// Exportable bundle for an external browser or vendor-console handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHandoffBundle {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable handoff bundle id.
    pub handoff_bundle_id: String,
    /// Destination class.
    pub destination_class: DestinationClass,
    /// Reason for the handoff.
    pub reason: String,
    /// Return anchor or follow-up note reference.
    pub return_anchor_ref: Option<String>,
    /// True when no raw provider URL or secret payload is embedded.
    pub raw_provider_payload_excluded: bool,
}

/// Local follow-up or checklist completion record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalFollowUpRecord {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable follow-up id.
    pub follow_up_id: String,
    /// Object ownership class.
    pub provider_object_ownership: ProviderObjectOwnershipClass,
    /// Local completion state.
    pub local_completion_state: LocalCompletionState,
    /// True only when a reviewed command mutated the provider-owned object.
    pub provider_mutation_claimed: bool,
    /// Reviewed command reference, if a mutation actually executed.
    pub reviewed_command_ref: Option<String>,
}

/// Canonical support/export packet for stable runbook truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHandoffTruthPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Schema reference.
    pub schema_ref: String,
    /// Source descriptors.
    pub source_descriptors: Vec<RunbookSourceDescriptor>,
    /// Executable step envelopes.
    pub step_envelopes: Vec<RunbookStepEnvelope>,
    /// Step execution records.
    pub execution_records: Vec<RunbookStepExecutionRecord>,
    /// Deviation notes.
    pub deviation_notes: Vec<DeviationNote>,
    /// External handoff bundles.
    pub handoff_bundles: Vec<ExternalHandoffBundle>,
    /// Local follow-up records.
    pub local_follow_ups: Vec<LocalFollowUpRecord>,
}

impl RunbookHandoffTruthPacket {
    /// Validates the packet against stable runbook handoff truth.
    pub fn validate(&self) -> Vec<RunbookTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != RUNBOOK_HANDOFF_TRUTH_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "packet.record_kind",
                &self.packet_id,
                "packet record_kind mismatch",
            );
        }
        if self.schema_version != RUNBOOK_HANDOFF_TRUTH_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "packet.schema_version",
                &self.packet_id,
                "packet schema_version must be 1",
            );
        }

        for source in &self.source_descriptors {
            if source.record_kind != RUNBOOK_SOURCE_DESCRIPTOR_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "source.record_kind",
                    &source.source_id,
                    "source descriptor record_kind mismatch",
                );
            }
            if source.source_class.is_browser_only()
                && source.authoritative_posture != AuthoritativePosture::ReferenceOnly
            {
                push_violation(
                    &mut violations,
                    "source.browser_only.authority",
                    &source.source_id,
                    "browser-only vendor documentation must be reference-only",
                );
            }
        }
        for execution in &self.execution_records {
            if execution.record_kind != RUNBOOK_STEP_EXECUTION_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "execution.record_kind",
                    &execution.execution_id,
                    "execution record_kind mismatch",
                );
            }
            if execution.result_state == StepResultState::Deviated
                && execution.deviation_note_refs.is_empty()
            {
                push_violation(
                    &mut violations,
                    "execution.deviated.missing_note",
                    &execution.execution_id,
                    "deviated execution must link a deviation note",
                );
            }
        }
        for step in &self.step_envelopes {
            if step.record_kind != RUNBOOK_STEP_ENVELOPE_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "step.record_kind",
                    &step.step_id,
                    "step envelope record_kind mismatch",
                );
            }
            if step.expected_evidence_outputs.is_empty() {
                push_violation(
                    &mut violations,
                    "step.evidence.empty",
                    &step.step_id,
                    "step must declare expected evidence outputs",
                );
            }
            if step.step_class.is_mutating()
                && step.destination_class == DestinationClass::InProduct
                && step.shared_action_envelope.is_none()
            {
                push_violation(
                    &mut violations,
                    "step.mutation.missing_shared_action_envelope",
                    &step.step_id,
                    "mutating in-product step must reuse the shared action envelope",
                );
            }
            if (step.destination_class.is_external()
                || step.target_selector_scope.requires_external_handoff())
                && step.external_handoff.is_none()
            {
                push_violation(
                    &mut violations,
                    "step.external_handoff.missing",
                    &step.step_id,
                    "external step must carry explicit browser or console handoff metadata",
                );
            }
        }

        for note in &self.deviation_notes {
            if note.record_kind != RUNBOOK_DEVIATION_NOTE_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "deviation.record_kind",
                    &note.deviation_note_id,
                    "deviation note record_kind mismatch",
                );
            }
        }

        for bundle in &self.handoff_bundles {
            if bundle.record_kind != RUNBOOK_HANDOFF_BUNDLE_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "handoff.record_kind",
                    &bundle.handoff_bundle_id,
                    "handoff bundle record_kind mismatch",
                );
            }
            if !bundle.raw_provider_payload_excluded {
                push_violation(
                    &mut violations,
                    "handoff.raw_provider_payload",
                    &bundle.handoff_bundle_id,
                    "handoff bundle must exclude raw provider payloads",
                );
            }
        }

        for follow_up in &self.local_follow_ups {
            if follow_up.record_kind != RUNBOOK_LOCAL_FOLLOW_UP_RECORD_KIND {
                push_violation(
                    &mut violations,
                    "follow_up.record_kind",
                    &follow_up.follow_up_id,
                    "local follow-up record_kind mismatch",
                );
            }
            if follow_up.provider_object_ownership == ProviderObjectOwnershipClass::ProviderOwned
                && follow_up.local_completion_state == LocalCompletionState::LocalChecklistOnly
                && follow_up.provider_mutation_claimed
            {
                push_violation(
                    &mut violations,
                    "follow_up.local_completion_overclaims_provider_mutation",
                    &follow_up.follow_up_id,
                    "local checklist completion cannot imply provider-owned object mutation",
                );
            }
            if follow_up.provider_mutation_claimed && follow_up.reviewed_command_ref.is_none() {
                push_violation(
                    &mut violations,
                    "follow_up.provider_mutation_without_reviewed_command",
                    &follow_up.follow_up_id,
                    "provider mutation claims must link a reviewed command",
                );
            }
        }

        violations
    }
}

/// One validation issue found in the stable runbook truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookTruthViolation {
    /// Stable validation check id.
    pub check_id: String,
    /// Artifact or row reference associated with the violation.
    pub ref_id: String,
    /// Copy-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<RunbookTruthViolation>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    violations.push(RunbookTruthViolation {
        check_id: check_id.to_owned(),
        ref_id: ref_id.to_owned(),
        message: message.to_owned(),
    });
}

/// Loads a [`RunbookHandoffTruthPacket`] from YAML.
///
/// # Errors
///
/// Returns a YAML deserialization error when the input does not match the
/// packet shape.
pub fn load_runbook_handoff_truth_packet(
    yaml: &str,
) -> Result<RunbookHandoffTruthPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the embedded fixture corpus as runbook handoff truth packets.
///
/// # Errors
///
/// Returns a YAML deserialization error when any embedded fixture fails to
/// parse.
pub fn current_runbook_handoff_truth_corpus(
) -> Result<Vec<RunbookHandoffTruthPacket>, serde_yaml::Error> {
    FIXTURE_SOURCES
        .iter()
        .map(|(_, yaml)| serde_yaml::from_str(yaml))
        .collect()
}
