//! Extension incident communication records for advisory, emergency
//! disable, quarantine, and revocation flows.
//!
//! This module owns the beta communication baseline for ecosystem
//! incidents. One [`ExtensionIncidentCommunicationRecord`] ties an
//! advisory ID, affected extension identity, registry / mirror trust
//! state, forced action, blocked operation set, and rollback / recovery
//! guidance together. The first consuming surface is
//! [`ExtensionIncidentSupportExportRecord`], which preserves the same
//! incident identifier and lifecycle state for support, CLI / headless,
//! and docs/help consumers.
//!
//! The cross-tool boundary schema is
//! [`/schemas/extensions/revocation_and_emergency_disable.schema.json`](../../../../schemas/extensions/revocation_and_emergency_disable.schema.json);
//! the reviewer-facing landing page is
//! [`/docs/extensions/m3/revocation_and_emergency_disable_beta.md`](../../../../docs/extensions/m3/revocation_and_emergency_disable_beta.md);
//! the checked fixtures live under
//! [`/fixtures/extensions/m3/revocation_and_emergency_disable/`](../../../../fixtures/extensions/m3/revocation_and_emergency_disable/).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::{RedactionClass, SummaryFreshnessClass};
use crate::review_alpha::RevocationStateClass;

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`ExtensionIncidentCommunicationRecord`] payloads.
pub const EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND: &str =
    "extension_incident_communication_record";

/// Record-kind tag carried on serialized [`ExtensionIncidentSupportExportRecord`] payloads.
pub const EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_incident_support_export_record";

/// Schema version for extension incident communication payloads.
pub const EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION: u32 = 1;

/// Incident action requested by registry, runtime, policy, or admin authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentActionClass {
    /// Publish an advisory without changing local extension enablement.
    AdvisoryOnly,
    /// Disable the affected extension while preserving installed state and recovery metadata.
    Disable,
    /// Quarantine the extension or host pending explicit review or policy clearance.
    Quarantine,
    /// Revoke the affected artifact or publisher-backed install state.
    Revoke,
}

/// Lifecycle state emitted by incident packets and copied into support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentLifecycleStateClass {
    /// Advisory is active, but no forced disable or revocation is in effect.
    AdvisoryActive,
    /// Extension is disabled and cannot activate until the action clears.
    Disabled,
    /// Extension is quarantined and remains visible for review and recovery.
    Quarantined,
    /// Extension artifact or publisher-backed state is revoked.
    Revoked,
    /// Local install is no longer exposed, but incident history remains visible.
    MitigatedLocally,
    /// Incident is resolved while retained for audit and support history.
    ResolvedRetained,
}

/// Severity vocabulary shared with advisory and emergency-notice surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorySeverityClass {
    /// Immediate action is required to prevent severe compromise or data loss.
    Critical,
    /// Serious risk where same-day mitigation is expected.
    High,
    /// Material risk with bounded mitigation or exposure.
    Moderate,
    /// Low impact advisory retained for awareness and history.
    Low,
    /// Operational emergency such as a kill switch, channel freeze, or trust-root event.
    OperationalEmergency,
}

/// User-facing reason code rendered in notices, CLI, docs/help, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentReasonCode {
    /// Publisher or artifact signing material is compromised or superseded.
    SigningKeyCompromised,
    /// Artifact provenance or supply-chain evidence failed after publication.
    ArtifactSupplyChainCompromise,
    /// Malware or active abuse was confirmed by review or scanner evidence.
    ConfirmedMalware,
    /// Extension attempted capability use beyond declared or admitted permission.
    PermissionAbuse,
    /// Extension or host repeatedly violated crash-loop, CPU, memory, or egress budgets.
    CrashLoopOrResourceAbuse,
    /// Registry moderation engaged an emergency safety action.
    RegistryModerationEmergency,
    /// Mirror promotion or mirror continuity failed for the affected artifact.
    MirrorContinuityBroken,
    /// Admin policy engaged a fleet or workspace emergency disable.
    AdminPolicyEmergencyDisable,
    /// Publisher requested a withdrawal that affects installed rows.
    PublisherWithdrawal,
}

/// Source family for incident evidence and emergency action metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentSourceClass {
    /// Public Aureline extension registry.
    PublicRegistry,
    /// Approved enterprise or private mirror.
    ApprovedMirror,
    /// Private extension registry.
    PrivateRegistry,
    /// Sealed offline or air-gap bundle import.
    OfflineBundle,
    /// Admin policy pack or fleet policy decision.
    AdminPolicyPack,
    /// Signed emergency-disable bundle.
    EmergencyDisableBundle,
    /// Runtime supervisor or extension-host isolation lane.
    RuntimeSupervisor,
    /// Explicit local user choice.
    UserChoice,
}

/// Actor class accountable for the incident action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentActorClass {
    /// Public registry operator.
    RegistryOperator,
    /// Mirror operator responsible for mirror promotion and import.
    MirrorOperator,
    /// Private registry administrator.
    PrivateRegistryAdministrator,
    /// Security responder or release responder invoking break-glass authority.
    SecurityResponder,
    /// Runtime supervisor applying deterministic host-health policy.
    RuntimeSupervisor,
    /// Workspace or organization administrator.
    WorkspaceAdmin,
    /// Verified publisher or successor publisher.
    Publisher,
    /// Local user acting on the installed extension.
    User,
}

/// Registry lane represented in an incident packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentRegistryLaneClass {
    /// Primary public registry lane.
    PrimaryRegistry,
    /// Approved mirror lane.
    ApprovedMirror,
    /// Private registry lane.
    PrivateRegistry,
    /// Offline or air-gap bundle lane.
    OfflineBundle,
}

/// Trust state for a registry, mirror, private-registry, or offline lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentTrustStateClass {
    /// Signed metadata is current for the lane.
    VerifiedCurrent,
    /// Signed metadata is verified but stale enough to narrow claims.
    VerifiedStale,
    /// Emergency or advisory metadata is waiting for mirror import.
    PendingMirrorImport,
    /// Mirror continuity is broken and must be shown directly.
    MirrorContinuityBroken,
    /// Digest or signature verification failed.
    SignatureOrDigestMismatch,
    /// No revocation or advisory snapshot is available.
    RevocationSnapshotMissing,
    /// Lane cannot identify a trustworthy source state.
    UnknownRefused,
}

/// Operation class blocked by an incident action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentBlockedOperationClass {
    /// Installing the affected extension for the first time.
    NewInstall,
    /// Updating an existing installed copy.
    Update,
    /// Automatic background update.
    AutoUpdate,
    /// Extension activation.
    Activation,
    /// Runtime execution after activation.
    Execution,
    /// Mirror import or mirror promotion.
    MirrorImport,
}

/// Recovery action shown with rollback, quarantine-clear, or support guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentRecoveryActionClass {
    /// Roll back to the declared last-known-good version.
    RollBackToLastKnownGood,
    /// Keep the current safe version pinned.
    KeepPinned,
    /// Keep the affected extension disabled until explicit reenable.
    DisableUntilReenabled,
    /// Remove the affected extension.
    RemoveExtension,
    /// Refresh mirror metadata before attempting install, update, or recovery.
    RefreshMirrorMetadata,
    /// Import a signed emergency or advisory bundle in an offline lane.
    ImportEmergencyBundle,
    /// Ask an admin, mirror operator, or security responder to clear policy.
    ConsultAdmin,
    /// Open safe mode or targeted extension recovery.
    OpenSafeMode,
    /// Open the incident packet for support or audit review.
    OpenIncidentPacket,
    /// No exact rollback is available; guidance must explain the limit.
    NoRecoveryAvailable,
}

/// Disclosure class that must render before an incident action is considered actionable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentDisclosureClass {
    /// Advisory ID and incident ID are visible and copyable.
    AdvisoryIdentity,
    /// Affected extension identity, package ID, version, and publisher are visible.
    AffectedExtension,
    /// Severity and reason code are visible.
    SeverityAndReason,
    /// Source, signer, and actor metadata are visible.
    SourceAndActor,
    /// Primary registry and mirror lane state are visible.
    RegistryAndMirrorState,
    /// Blocked operation set is visible.
    BlockedOperations,
    /// Lifecycle and revocation state are visible.
    LifecycleState,
    /// Rollback or recovery guidance is visible.
    RecoveryGuidance,
}

/// Decision emitted by the incident communication evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentDecisionClass {
    /// Advisory is published without a forced local action.
    AdvisoryPublished,
    /// Disable action is engaged and actionable.
    DisableEngaged,
    /// Quarantine action is engaged and actionable.
    QuarantineEngaged,
    /// Revocation action is engaged and actionable.
    RevocationEngaged,
    /// The incident is valid but the mirror lane must import metadata before acting.
    AwaitingMirrorImport,
    /// The packet is incomplete or unsafe to apply.
    Refused,
}

/// Typed reason paired with [`ExtensionIncidentDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentDecisionReasonClass {
    /// Advisory metadata is complete and no forced action is requested.
    AdvisoryNoForcedAction,
    /// Emergency disable is engaged.
    EmergencyDisableEngaged,
    /// Quarantine is engaged.
    QuarantineEngaged,
    /// Revocation is engaged.
    RevocationEngaged,
    /// Mirror lane has not imported the advisory or revocation metadata yet.
    AwaitingMirrorImport,
    /// Incident ID or advisory ID is missing.
    RefusedIncidentIdentityMissing,
    /// Affected extension identity is missing.
    RefusedSubjectIdentityMissing,
    /// The consumer failed to render a required disclosure.
    RefusedRequiredDisclosureMissing,
    /// Registry or mirror trust state is ambiguous.
    RefusedAmbiguousTrustState,
    /// Forced action lacks blocked operation metadata.
    RefusedBlockedOperationsMissing,
    /// Forced action lacks rollback or recovery guidance.
    RefusedRecoveryGuidanceMissing,
}

/// Action offered by support, CLI/headless, docs/help, or review consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionIncidentSupportActionClass {
    /// Acknowledge an advisory that does not force a mutation.
    Acknowledge,
    /// Open the advisory detail.
    OpenAdvisory,
    /// Open recovery guidance.
    OpenRecoveryGuidance,
    /// Roll back or pin to the last-known-good version.
    RollBackOrPin,
    /// Remove or keep disabled the affected extension.
    RemoveOrDisable,
    /// Refresh mirror metadata.
    RefreshMirror,
    /// Consult an administrator or mirror operator.
    ConsultAdmin,
    /// Export a metadata-safe support packet.
    ExportSupportPacket,
}

/// Extension subject affected by an incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentSubject {
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub publisher_id: String,
    pub registry_manifest_ref: String,
    pub catalog_descriptor_ref: String,
    pub runtime_contract_ref: String,
    pub installed_state_refs: Vec<String>,
    pub affected_artifact_refs: Vec<String>,
}

/// Advisory metadata attached to an extension incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentAdvisory {
    pub advisory_id: String,
    pub severity_class: AdvisorySeverityClass,
    pub reason_code: ExtensionIncidentReasonCode,
    pub source_class: ExtensionIncidentSourceClass,
    pub source_ref: String,
    pub signer_ref: String,
    pub published_at: String,
    pub updated_at: String,
    pub disclosure_refs: Vec<String>,
}

/// Forced action metadata for an incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentAction {
    pub action_class: ExtensionIncidentActionClass,
    pub actor_class: ExtensionIncidentActorClass,
    pub effective_at: String,
    pub deadline_at: Option<String>,
    pub blocked_operations: Vec<ExtensionIncidentBlockedOperationClass>,
    pub emergency_bundle_ref: Option<String>,
    pub policy_refs: Vec<String>,
    pub audit_event_refs: Vec<String>,
}

/// Registry, mirror, private-registry, or offline lane state for an incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentRegistryLane {
    pub lane_class: ExtensionIncidentRegistryLaneClass,
    pub source_ref: String,
    pub snapshot_ref: String,
    pub trust_state_class: ExtensionIncidentTrustStateClass,
    pub freshness_class: SummaryFreshnessClass,
    pub signer_continuity_ref: String,
    pub mirror_continuity_ref: Option<String>,
    pub import_required: bool,
}

/// Rollback or recovery guidance that accompanies an incident action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentRecoveryGuidance {
    pub last_known_good_version: Option<String>,
    pub rollback_manifest_ref: Option<String>,
    pub recovery_action_classes: Vec<ExtensionIncidentRecoveryActionClass>,
    pub user_facing_guidance: String,
    pub admin_handoff_refs: Vec<String>,
    pub safe_mode_profile_ref: Option<String>,
}

/// Input supplied by registry, runtime, policy, or support lanes to create an incident packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentCommunicationInput {
    pub incident_id: String,
    pub subject: ExtensionIncidentSubject,
    pub advisory: ExtensionIncidentAdvisory,
    pub action: ExtensionIncidentAction,
    pub primary_registry_lane: ExtensionIncidentRegistryLane,
    pub mirror_lane: ExtensionIncidentRegistryLane,
    pub recovery: ExtensionIncidentRecoveryGuidance,
    pub rendered_disclosures: Vec<ExtensionIncidentDisclosureClass>,
    pub generated_at: String,
}

/// Evaluated incident packet consumed by review, support, CLI, mirror, and docs/help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentCommunicationRecord {
    pub record_kind: String,
    pub extension_incident_schema_version: u32,
    pub incident_id: String,
    pub subject: ExtensionIncidentSubject,
    pub advisory: ExtensionIncidentAdvisory,
    pub action: ExtensionIncidentAction,
    pub primary_registry_lane: ExtensionIncidentRegistryLane,
    pub mirror_lane: ExtensionIncidentRegistryLane,
    pub recovery: ExtensionIncidentRecoveryGuidance,
    pub required_disclosures: Vec<ExtensionIncidentDisclosureClass>,
    pub rendered_disclosures: Vec<ExtensionIncidentDisclosureClass>,
    pub lifecycle_state_class: ExtensionIncidentLifecycleStateClass,
    pub revocation_state_class: RevocationStateClass,
    pub blocks_new_installs: bool,
    pub blocks_updates: bool,
    pub blocks_activation: bool,
    pub blocks_execution: bool,
    pub mirror_trust_unambiguous: bool,
    pub recovery_guidance_ready: bool,
    pub decision_class: ExtensionIncidentDecisionClass,
    pub reason_class: ExtensionIncidentDecisionReasonClass,
    pub decision_summary: String,
    pub generated_at: String,
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export that preserves incident and lifecycle identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionIncidentSupportExportRecord {
    pub record_kind: String,
    pub extension_incident_schema_version: u32,
    pub export_id: String,
    pub incident_ref: String,
    pub advisory_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub publisher_id: String,
    pub severity_class: AdvisorySeverityClass,
    pub reason_code: ExtensionIncidentReasonCode,
    pub source_class: ExtensionIncidentSourceClass,
    pub actor_class: ExtensionIncidentActorClass,
    pub primary_registry_trust_state_class: ExtensionIncidentTrustStateClass,
    pub mirror_trust_state_class: ExtensionIncidentTrustStateClass,
    pub lifecycle_state_class: ExtensionIncidentLifecycleStateClass,
    pub revocation_state_class: RevocationStateClass,
    pub decision_class: ExtensionIncidentDecisionClass,
    pub reason_class: ExtensionIncidentDecisionReasonClass,
    pub blocked_operations: Vec<ExtensionIncidentBlockedOperationClass>,
    pub recovery_action_classes: Vec<ExtensionIncidentRecoveryActionClass>,
    pub support_action_classes: Vec<ExtensionIncidentSupportActionClass>,
    pub blocks_install_or_update: bool,
    pub blocks_activation_or_execution: bool,
    pub mirror_trust_unambiguous: bool,
    pub recovery_guidance_ready: bool,
    pub audit_event_refs: Vec<String>,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by extension incident validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionIncidentFinding {
    /// Stable validation check id.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl ExtensionIncidentFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate advisory, disable, quarantine, and revocation communication into one packet.
pub fn evaluate_extension_incident_communication(
    input: ExtensionIncidentCommunicationInput,
) -> ExtensionIncidentCommunicationRecord {
    let required_disclosures = required_disclosures_for_incident_action(input.action.action_class);
    let lifecycle_state_class = lifecycle_state_for_action(input.action.action_class);
    let revocation_state_class = revocation_state_for_action(input.action.action_class);
    let blocks_new_installs = input
        .action
        .blocked_operations
        .contains(&ExtensionIncidentBlockedOperationClass::NewInstall);
    let blocks_updates = input.action.blocked_operations.iter().any(|operation| {
        matches!(
            operation,
            ExtensionIncidentBlockedOperationClass::Update
                | ExtensionIncidentBlockedOperationClass::AutoUpdate
        )
    });
    let blocks_activation = input
        .action
        .blocked_operations
        .contains(&ExtensionIncidentBlockedOperationClass::Activation);
    let blocks_execution = input
        .action
        .blocked_operations
        .contains(&ExtensionIncidentBlockedOperationClass::Execution);
    let mirror_trust_unambiguous = lane_trust_unambiguous(&input.primary_registry_lane)
        && lane_trust_unambiguous(&input.mirror_lane);
    let recovery_guidance_ready =
        recovery_guidance_ready(input.action.action_class, &input.recovery);

    let (decision_class, reason_class, decision_summary) = decide_incident_communication(
        &input,
        &required_disclosures,
        mirror_trust_unambiguous,
        recovery_guidance_ready,
    );

    ExtensionIncidentCommunicationRecord {
        record_kind: EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND.to_string(),
        extension_incident_schema_version: EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION,
        incident_id: input.incident_id,
        subject: input.subject,
        advisory: input.advisory,
        action: input.action,
        primary_registry_lane: input.primary_registry_lane,
        mirror_lane: input.mirror_lane,
        recovery: input.recovery,
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        lifecycle_state_class,
        revocation_state_class,
        blocks_new_installs,
        blocks_updates,
        blocks_activation,
        blocks_execution,
        mirror_trust_unambiguous,
        recovery_guidance_ready,
        decision_class,
        reason_class,
        decision_summary,
        generated_at: input.generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project an incident packet into the support/export consumer shape.
pub fn project_extension_incident_support_export(
    record: &ExtensionIncidentCommunicationRecord,
    export_id: &str,
) -> ExtensionIncidentSupportExportRecord {
    let mut support_action_classes = vec![
        ExtensionIncidentSupportActionClass::OpenAdvisory,
        ExtensionIncidentSupportActionClass::OpenRecoveryGuidance,
        ExtensionIncidentSupportActionClass::ExportSupportPacket,
    ];

    match record.decision_class {
        ExtensionIncidentDecisionClass::AdvisoryPublished => {
            support_action_classes.insert(0, ExtensionIncidentSupportActionClass::Acknowledge);
        }
        ExtensionIncidentDecisionClass::DisableEngaged
        | ExtensionIncidentDecisionClass::QuarantineEngaged
        | ExtensionIncidentDecisionClass::RevocationEngaged => {
            support_action_classes.push(ExtensionIncidentSupportActionClass::RollBackOrPin);
            support_action_classes.push(ExtensionIncidentSupportActionClass::RemoveOrDisable);
            support_action_classes.push(ExtensionIncidentSupportActionClass::ConsultAdmin);
        }
        ExtensionIncidentDecisionClass::AwaitingMirrorImport => {
            support_action_classes.push(ExtensionIncidentSupportActionClass::RefreshMirror);
            support_action_classes.push(ExtensionIncidentSupportActionClass::ConsultAdmin);
        }
        ExtensionIncidentDecisionClass::Refused => {
            support_action_classes.push(ExtensionIncidentSupportActionClass::ConsultAdmin);
        }
    }

    if record.mirror_lane.trust_state_class != ExtensionIncidentTrustStateClass::VerifiedCurrent
        && !support_action_classes.contains(&ExtensionIncidentSupportActionClass::RefreshMirror)
    {
        support_action_classes.push(ExtensionIncidentSupportActionClass::RefreshMirror);
    }

    ExtensionIncidentSupportExportRecord {
        record_kind: EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        extension_incident_schema_version: EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        incident_ref: record.incident_id.clone(),
        advisory_id: record.advisory.advisory_id.clone(),
        extension_identity: record.subject.extension_identity.clone(),
        extension_version: record.subject.extension_version.clone(),
        package_id: record.subject.package_id.clone(),
        publisher_id: record.subject.publisher_id.clone(),
        severity_class: record.advisory.severity_class,
        reason_code: record.advisory.reason_code,
        source_class: record.advisory.source_class,
        actor_class: record.action.actor_class,
        primary_registry_trust_state_class: record.primary_registry_lane.trust_state_class,
        mirror_trust_state_class: record.mirror_lane.trust_state_class,
        lifecycle_state_class: record.lifecycle_state_class,
        revocation_state_class: record.revocation_state_class,
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        blocked_operations: record.action.blocked_operations.clone(),
        recovery_action_classes: record.recovery.recovery_action_classes.clone(),
        support_action_classes,
        blocks_install_or_update: record.blocks_new_installs || record.blocks_updates,
        blocks_activation_or_execution: record.blocks_activation || record.blocks_execution,
        mirror_trust_unambiguous: record.mirror_trust_unambiguous,
        recovery_guidance_ready: record.recovery_guidance_ready,
        audit_event_refs: record.action.audit_event_refs.clone(),
        export_safe_summary: format!(
            "{} {} incident={} advisory={} lifecycle={:?} revocation={:?} primary={:?} mirror={:?}",
            record.subject.extension_identity,
            record.subject.extension_version,
            record.incident_id,
            record.advisory.advisory_id,
            record.lifecycle_state_class,
            record.revocation_state_class,
            record.primary_registry_lane.trust_state_class,
            record.mirror_lane.trust_state_class
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for an incident communication packet.
pub fn validate_extension_incident_communication_record(
    record: &ExtensionIncidentCommunicationRecord,
) -> Vec<ExtensionIncidentFinding> {
    let mut findings = Vec::new();

    if record.record_kind != EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.extension_incident_schema_version != EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.schema_version_wrong",
            format!(
                "extension_incident_schema_version must be {EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION}; got {}",
                record.extension_incident_schema_version
            ),
        ));
    }
    if !record.incident_id.starts_with("extension_incident:") {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.id_unprefixed",
            "incident_id must start with 'extension_incident:'",
        ));
    }
    if record.advisory.advisory_id.trim().is_empty() {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.advisory_id_required",
            "advisory_id must be present",
        ));
    }
    if subject_identity_missing(&record.subject) {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.subject_identity_required",
            "extension identity, package id, publisher id, registry manifest, catalog descriptor, and runtime contract refs must be present",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&record.required_disclosures, &record.rendered_disclosures)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if record.lifecycle_state_class != lifecycle_state_for_action(record.action.action_class) {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.lifecycle_state_inconsistent",
            "lifecycle_state_class must match action_class",
        ));
    }
    if record.revocation_state_class != revocation_state_for_action(record.action.action_class) {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.revocation_state_inconsistent",
            "revocation_state_class must match action_class",
        ));
    }
    if record.blocks_new_installs
        != record
            .action
            .blocked_operations
            .contains(&ExtensionIncidentBlockedOperationClass::NewInstall)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.blocks_new_installs_inconsistent",
            "blocks_new_installs must reflect blocked_operations",
        ));
    }
    if record.blocks_updates
        != record.action.blocked_operations.iter().any(|operation| {
            matches!(
                operation,
                ExtensionIncidentBlockedOperationClass::Update
                    | ExtensionIncidentBlockedOperationClass::AutoUpdate
            )
        })
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.blocks_updates_inconsistent",
            "blocks_updates must reflect update or auto_update blocked operations",
        ));
    }
    if record.blocks_activation
        != record
            .action
            .blocked_operations
            .contains(&ExtensionIncidentBlockedOperationClass::Activation)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.blocks_activation_inconsistent",
            "blocks_activation must reflect blocked_operations",
        ));
    }
    if record.blocks_execution
        != record
            .action
            .blocked_operations
            .contains(&ExtensionIncidentBlockedOperationClass::Execution)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.blocks_execution_inconsistent",
            "blocks_execution must reflect blocked_operations",
        ));
    }
    if record.mirror_trust_unambiguous
        != (lane_trust_unambiguous(&record.primary_registry_lane)
            && lane_trust_unambiguous(&record.mirror_lane))
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.mirror_trust_inconsistent",
            "mirror_trust_unambiguous must reflect primary and mirror lane trust states",
        ));
    }
    if !record.mirror_trust_unambiguous {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.trust_state_ambiguous",
            "primary and mirror lane trust states must be explicit and non-unknown",
        ));
    }
    if record.recovery_guidance_ready
        != recovery_guidance_ready(record.action.action_class, &record.recovery)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.recovery_guidance_inconsistent",
            "recovery_guidance_ready must reflect rollback or recovery guidance",
        ));
    }
    if requires_forced_action_metadata(record.action.action_class)
        && record.action.blocked_operations.is_empty()
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.blocked_operations_required",
            "disable, quarantine, and revoke actions must name blocked operations",
        ));
    }
    if requires_forced_action_metadata(record.action.action_class)
        && !record.recovery_guidance_ready
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.recovery_guidance_required",
            "disable, quarantine, and revoke actions must carry rollback or recovery guidance",
        ));
    }
    if matches!(
        record.mirror_lane.lane_class,
        ExtensionIncidentRegistryLaneClass::ApprovedMirror
    ) && record.mirror_lane.mirror_continuity_ref.is_none()
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.mirror_continuity_required",
            "approved mirror lanes must cite a mirror_continuity_ref",
        ));
    }
    if matches!(
        record.mirror_lane.trust_state_class,
        ExtensionIncidentTrustStateClass::MirrorContinuityBroken
            | ExtensionIncidentTrustStateClass::SignatureOrDigestMismatch
    ) && !record
        .action
        .blocked_operations
        .contains(&ExtensionIncidentBlockedOperationClass::MirrorImport)
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.mirror_import_block_required",
            "broken mirror continuity or signature mismatch must block mirror_import",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident.redaction_class_must_be_metadata_safe",
            "incident communication records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    findings
}

/// Validate structural invariants for a support-export incident projection.
pub fn validate_extension_incident_support_export_record(
    record: &ExtensionIncidentSupportExportRecord,
) -> Vec<ExtensionIncidentFinding> {
    let mut findings = Vec::new();

    if record.record_kind != EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.extension_incident_schema_version != EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.schema_version_wrong",
            format!(
                "extension_incident_schema_version must be {EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION}; got {}",
                record.extension_incident_schema_version
            ),
        ));
    }
    if !record
        .export_id
        .starts_with("extension_incident_support_export:")
    {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.id_unprefixed",
            "export_id must start with 'extension_incident_support_export:'",
        ));
    }
    if !record.incident_ref.starts_with("extension_incident:") {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.incident_ref_unprefixed",
            "incident_ref must start with 'extension_incident:'",
        ));
    }
    if record.advisory_id.trim().is_empty() {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.advisory_id_required",
            "advisory_id must be present",
        ));
    }
    if record.export_safe_summary.trim().is_empty() {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.summary_required",
            "export_safe_summary must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ExtensionIncidentFinding::new(
            "extension_incident_support_export.redaction_class_must_be_metadata_safe",
            "support export records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    findings
}

fn required_disclosures_for_incident_action(
    _action_class: ExtensionIncidentActionClass,
) -> Vec<ExtensionIncidentDisclosureClass> {
    vec![
        ExtensionIncidentDisclosureClass::AdvisoryIdentity,
        ExtensionIncidentDisclosureClass::AffectedExtension,
        ExtensionIncidentDisclosureClass::SeverityAndReason,
        ExtensionIncidentDisclosureClass::SourceAndActor,
        ExtensionIncidentDisclosureClass::RegistryAndMirrorState,
        ExtensionIncidentDisclosureClass::BlockedOperations,
        ExtensionIncidentDisclosureClass::LifecycleState,
        ExtensionIncidentDisclosureClass::RecoveryGuidance,
    ]
}

fn lifecycle_state_for_action(
    action_class: ExtensionIncidentActionClass,
) -> ExtensionIncidentLifecycleStateClass {
    match action_class {
        ExtensionIncidentActionClass::AdvisoryOnly => {
            ExtensionIncidentLifecycleStateClass::AdvisoryActive
        }
        ExtensionIncidentActionClass::Disable => ExtensionIncidentLifecycleStateClass::Disabled,
        ExtensionIncidentActionClass::Quarantine => {
            ExtensionIncidentLifecycleStateClass::Quarantined
        }
        ExtensionIncidentActionClass::Revoke => ExtensionIncidentLifecycleStateClass::Revoked,
    }
}

fn revocation_state_for_action(action_class: ExtensionIncidentActionClass) -> RevocationStateClass {
    match action_class {
        ExtensionIncidentActionClass::AdvisoryOnly => RevocationStateClass::NoKnownRevocation,
        ExtensionIncidentActionClass::Disable => RevocationStateClass::EmergencyDisabled,
        ExtensionIncidentActionClass::Quarantine => RevocationStateClass::Quarantined,
        ExtensionIncidentActionClass::Revoke => RevocationStateClass::Revoked,
    }
}

fn decide_incident_communication(
    input: &ExtensionIncidentCommunicationInput,
    required_disclosures: &[ExtensionIncidentDisclosureClass],
    mirror_trust_unambiguous: bool,
    recovery_guidance_ready: bool,
) -> (
    ExtensionIncidentDecisionClass,
    ExtensionIncidentDecisionReasonClass,
    String,
) {
    if input.incident_id.trim().is_empty() || input.advisory.advisory_id.trim().is_empty() {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedIncidentIdentityMissing,
            "incident communication is missing incident or advisory identity",
        );
    }
    if subject_identity_missing(&input.subject) {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedSubjectIdentityMissing,
            "incident communication is missing affected extension identity, package, publisher, or refs",
        );
    }
    if let Some(missing) =
        first_missing_disclosure(required_disclosures, &input.rendered_disclosures)
    {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedRequiredDisclosureMissing,
            format!("incident communication did not render required disclosure '{missing:?}'"),
        );
    }
    if !mirror_trust_unambiguous {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedAmbiguousTrustState,
            "incident communication cannot proceed because primary or mirror trust state is ambiguous",
        );
    }
    if requires_forced_action_metadata(input.action.action_class)
        && input.action.blocked_operations.is_empty()
    {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedBlockedOperationsMissing,
            "incident communication cannot force an action without blocked operation metadata",
        );
    }
    if requires_forced_action_metadata(input.action.action_class) && !recovery_guidance_ready {
        return refused(
            ExtensionIncidentDecisionReasonClass::RefusedRecoveryGuidanceMissing,
            "incident communication cannot force an action without rollback or recovery guidance",
        );
    }
    if input.mirror_lane.import_required
        || input.mirror_lane.trust_state_class
            == ExtensionIncidentTrustStateClass::PendingMirrorImport
    {
        return (
            ExtensionIncidentDecisionClass::AwaitingMirrorImport,
            ExtensionIncidentDecisionReasonClass::AwaitingMirrorImport,
            "Awaiting mirror import: primary incident metadata is known, but the mirror lane must import signed advisory or revocation metadata before acting."
                .to_string(),
        );
    }

    match input.action.action_class {
        ExtensionIncidentActionClass::AdvisoryOnly => (
            ExtensionIncidentDecisionClass::AdvisoryPublished,
            ExtensionIncidentDecisionReasonClass::AdvisoryNoForcedAction,
            "Advisory published: affected installs remain visible and no forced disable or revocation is active."
                .to_string(),
        ),
        ExtensionIncidentActionClass::Disable => (
            ExtensionIncidentDecisionClass::DisableEngaged,
            ExtensionIncidentDecisionReasonClass::EmergencyDisableEngaged,
            "Emergency disable engaged: affected installs remain visible, activation is blocked, and recovery guidance is attached."
                .to_string(),
        ),
        ExtensionIncidentActionClass::Quarantine => (
            ExtensionIncidentDecisionClass::QuarantineEngaged,
            ExtensionIncidentDecisionReasonClass::QuarantineEngaged,
            "Quarantine engaged: affected extension remains visible for review and cannot reactivate until recovery conditions clear."
                .to_string(),
        ),
        ExtensionIncidentActionClass::Revoke => (
            ExtensionIncidentDecisionClass::RevocationEngaged,
            ExtensionIncidentDecisionReasonClass::RevocationEngaged,
            "Revocation engaged: affected artifact is blocked for install, update, activation, and execution while rollback guidance stays attached."
                .to_string(),
        ),
    }
}

fn refused(
    reason_class: ExtensionIncidentDecisionReasonClass,
    summary: impl Into<String>,
) -> (
    ExtensionIncidentDecisionClass,
    ExtensionIncidentDecisionReasonClass,
    String,
) {
    (
        ExtensionIncidentDecisionClass::Refused,
        reason_class,
        format!("Refused: {}.", summary.into()),
    )
}

fn requires_forced_action_metadata(action_class: ExtensionIncidentActionClass) -> bool {
    !matches!(action_class, ExtensionIncidentActionClass::AdvisoryOnly)
}

fn recovery_guidance_ready(
    action_class: ExtensionIncidentActionClass,
    recovery: &ExtensionIncidentRecoveryGuidance,
) -> bool {
    if recovery.user_facing_guidance.trim().is_empty() {
        return false;
    }
    if matches!(action_class, ExtensionIncidentActionClass::AdvisoryOnly) {
        return true;
    }
    if recovery.recovery_action_classes.is_empty() {
        return false;
    }
    let rollback_requested = recovery
        .recovery_action_classes
        .contains(&ExtensionIncidentRecoveryActionClass::RollBackToLastKnownGood);
    if rollback_requested
        && (recovery
            .last_known_good_version
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
            || recovery
                .rollback_manifest_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty())
    {
        return false;
    }
    recovery.rollback_manifest_ref.is_some()
        || recovery.safe_mode_profile_ref.is_some()
        || !recovery.admin_handoff_refs.is_empty()
        || recovery
            .recovery_action_classes
            .contains(&ExtensionIncidentRecoveryActionClass::NoRecoveryAvailable)
}

fn lane_trust_unambiguous(lane: &ExtensionIncidentRegistryLane) -> bool {
    if lane.source_ref.trim().is_empty()
        || lane.snapshot_ref.trim().is_empty()
        || lane.signer_continuity_ref.trim().is_empty()
    {
        return false;
    }
    !matches!(
        lane.trust_state_class,
        ExtensionIncidentTrustStateClass::UnknownRefused
            | ExtensionIncidentTrustStateClass::RevocationSnapshotMissing
    )
}

fn subject_identity_missing(subject: &ExtensionIncidentSubject) -> bool {
    subject.extension_identity.trim().is_empty()
        || subject.extension_version.trim().is_empty()
        || subject.package_id.trim().is_empty()
        || subject.publisher_id.trim().is_empty()
        || subject.registry_manifest_ref.trim().is_empty()
        || subject.catalog_descriptor_ref.trim().is_empty()
        || subject.runtime_contract_ref.trim().is_empty()
}

fn first_missing_disclosure(
    required: &[ExtensionIncidentDisclosureClass],
    rendered: &[ExtensionIncidentDisclosureClass],
) -> Option<ExtensionIncidentDisclosureClass> {
    required
        .iter()
        .copied()
        .find(|required| !rendered.contains(required))
}
