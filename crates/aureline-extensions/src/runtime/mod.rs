//! Extension runtime v1 beta admission contract.
//!
//! This module promotes the extension platform into its first real beta
//! contract. It binds the manifest baseline (publisher identity, host
//! contract family), the host-negotiation packet (declared and negotiated
//! capability worlds), the lifecycle and restart posture (from
//! `artifacts/extensions/extension_lifecycle_states.yaml` and
//! `artifacts/extensions/quarantine_rules.yaml`), the runtime-budget
//! evidence, and the SDK / marketplace alignment refs into one typed
//! [`RuntimeV1BetaContractRecord`] that the install / review surface, the
//! permission-inspector, the support export, and the partner packet
//! template all read instead of inventing per-surface runtime truth.
//!
//! The record is intentionally bounded:
//!
//! - it works the same for capability-bounded Wasm extensions and for
//!   separately supervised external host processes; both shapes resolve
//!   through a closed [`HostPlacementClass`] and
//!   [`HostSupervisionClass`] vocabulary, and the same lifecycle / restart
//!   / degraded-state fields fire on both;
//! - it refuses beta admission when publisher identity is opaque, when
//!   the declared-vs-effective permission diff is missing, when the host
//!   placement is unknown / unsupported, when the capability negotiation
//!   refused every declared world, when the SDK alignment is unknown, and
//!   when the lifecycle / runtime-budget state is quarantined; and
//! - it projects one [`RuntimeV1BetaSupportExportRecord`] that the first
//!   consumer (support exports, partner packets, install / review chrome,
//!   and CLI / headless lanes) reads.
//!
//! The cross-tool boundary schema is
//! [`/schemas/extensions/runtime_contract.schema.json`](../../../../schemas/extensions/runtime_contract.schema.json);
//! the reviewer-facing landing page is
//! [`/docs/extensions/m3/runtime_v1_beta.md`](../../../../docs/extensions/m3/runtime_v1_beta.md).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::{
    HostContractFamilyClass, InstallDecisionClass, InstallDecisionReasonClass, RedactionClass,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`RuntimeV1BetaContractRecord`] payloads.
pub const RUNTIME_V1_BETA_CONTRACT_RECORD_KIND: &str = "runtime_v1_beta_contract_record";

/// Record-kind tag carried on serialized [`RuntimeV1BetaSupportExportRecord`] payloads.
pub const RUNTIME_V1_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "runtime_v1_beta_support_export_record";

/// Schema version for the runtime v1 beta payloads.
///
/// Bumped on breaking payload changes. Additive enum members or optional
/// fields are additive-minor and require consumers to keep unknown-field
/// preservation at their boundary.
pub const RUNTIME_V1_BETA_SCHEMA_VERSION: u32 = 1;

/// Closed host-placement vocabulary.
///
/// Describes where the extension instance actually runs. Both Wasm
/// capability worlds and isolated external host processes resolve into
/// one closed class so support, partner, and review surfaces never need
/// to invent a "runs somewhere" label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostPlacementClass {
    /// Capability-bounded Wasm component in the editor's own process.
    WasmInProcessIsolatedWorld,
    /// Wasm runtime in a dedicated, supervised subprocess.
    WasmIsolatedSubprocess,
    /// External host binary launched as a supervised long-running process.
    ExternalHostSupervisedProcess,
    /// Short-lived helper binary launched per invocation with a kill switch.
    HelperBinaryShortLived,
    /// Remote-side component executing on an attached remote agent.
    RemoteSideComponentAttached,
    /// Compatibility bridge that translates a foreign ecosystem onto the
    /// reserved Aureline capability worlds.
    CompatibilityBridgeTranslated,
    /// Reserved terminal class for a row whose placement could not be
    /// attributed. Admitted only on a denial-drill row paired with
    /// [`RuntimeAdmissionDecisionClass::Refused`] and
    /// [`RuntimeAdmissionReasonClass::HostPlacementUnsupported`].
    UnknownPlacementClass,
}

/// Closed host-supervision vocabulary.
///
/// Describes how the host supervises the placement. Pairs with
/// [`HostPlacementClass`] but is kept separate so review surfaces can
/// disclose the supervision posture independently of where the instance
/// runs (e.g. an external host process supervised by a kill switch vs.
/// supervised by a long-running watchdog).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostSupervisionClass {
    /// In-process capability-bounded Wasm sandbox; failures bring down
    /// the world binding but never the editor process.
    InProcessCapabilitySandbox,
    /// Dedicated subprocess with a supervised lifetime and budget axis
    /// counters; failures route through the runtime-budget packet.
    SeparateSubprocessSupervised,
    /// Short-lived helper binary protected by a kill switch and per-
    /// invocation timeout.
    SeparateSubprocessHelperKillSwitch,
    /// Remote-agent supervision: lifetime, identity, and kill paths are
    /// owned by the attached remote agent envelope.
    RemoteAgentAttachedEnvelope,
    /// Translated supervision through a compatibility bridge profile.
    /// The bridge owns the foreign-ecosystem supervisor but never widens
    /// the reserved capability world surface.
    CompatibilityBridgeTranslatedSupervision,
    /// Reserved terminal class for a row whose supervision posture is
    /// unknown. Admitted only on a denial-drill row paired with
    /// [`RuntimeAdmissionDecisionClass::Refused`] and
    /// [`RuntimeAdmissionReasonClass::HostPlacementUnsupported`].
    UnknownSupervisionClass,
}

/// Closed runtime-lifecycle-state vocabulary mirrored from
/// `artifacts/extensions/extension_lifecycle_states.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeLifecycleStateClass {
    Discovered,
    Installed,
    Disabled,
    PendingActivation,
    Active,
    Degraded,
    Quarantined,
    Recovered,
    Removed,
    PublisherBlocked,
}

/// Closed restart-posture vocabulary mirrored from
/// `artifacts/extensions/quarantine_rules.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartPostureClass {
    NoRestartAttempted,
    OneWarmRestartUnderBudget,
    ExponentialBackoffBounded,
}

/// Closed degraded-state vocabulary projected onto review and support
/// surfaces.
///
/// Mirrors the response-class vocabulary from
/// `artifacts/extensions/quarantine_rules.yaml` plus an explicit
/// `none_nominal` state for actively-running rows and a
/// `publisher_blocked` state that pairs with the install-review denial
/// reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedStateClass {
    NoneNominal,
    ThrottledBackgroundWork,
    DisabledUntilNextSession,
    DisabledUntilUserExplicitReenable,
    QuarantinedPendingReview,
    PublisherBlocked,
}

/// Closed runtime-admission decision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAdmissionDecisionClass {
    /// The runtime admitted the contract verbatim; every declared world
    /// is in the negotiated set and the row is allowed to activate.
    Admitted,
    /// The runtime admitted the contract under a typed world narrowing;
    /// some declared worlds were dropped from the negotiated set under
    /// trust / policy / lifecycle / ABI rules.
    AdmittedNarrowed,
    /// The runtime needs a user acknowledgement before the narrowed
    /// world set is admitted.
    AwaitingUserReview,
    /// The runtime refused admission outright.
    Refused,
    /// The runtime is quarantining the row; activation MUST NOT proceed
    /// until the quarantine clears.
    Quarantined,
}

/// Closed runtime-admission reason class paired with
/// [`RuntimeAdmissionDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAdmissionReasonClass {
    /// Negotiation cleared; capability worlds, host placement, supervision,
    /// lifecycle state, and SDK / marketplace alignment all admitted.
    AdmittedAfterCapabilityNegotiation,
    /// Negotiation admitted a strict subset of declared worlds.
    AdmittedWithWorldNarrowing,
    /// A narrowed world set requires a user acknowledgement before
    /// activation may resume.
    AwaitingUserWorldAcknowledgement,
    /// The manifest install decision denied the install; runtime cannot
    /// admit.
    ManifestInstallDenied,
    /// The publisher identity is opaque or quarantined; runtime refuses
    /// beta admission.
    PublisherIdentityOpaque,
    /// The declared-vs-effective permission diff is missing from the
    /// effective-permission summary.
    PermissionDiffMissing,
    /// Effective-permission truth blocked a widening attempt.
    EffectivePermissionWideningAttempted,
    /// Capability negotiation produced no admitted world for the row.
    CapabilityWorldUnavailableOnHost,
    /// Host placement or supervision is unknown / unsupported on this host.
    HostPlacementUnsupported,
    /// Runtime budget evidence reports an active quarantine or disable.
    RuntimeBudgetQuarantineActive,
    /// Crash-loop trigger fired and the row is quarantined.
    CrashLoopQuarantineActive,
    /// SDK or marketplace metadata is out of date for this runtime
    /// contract version.
    SdkOrMarketplaceMetadataOutOfDate,
    /// The lifecycle row is in a terminal state (retired / removed /
    /// publisher_blocked) that cannot be admitted.
    LifecycleTerminalState,
}

/// Closed SDK / marketplace alignment class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdkAlignmentClass {
    /// Runtime contract version matches the published SDK release-bundle
    /// version and the marketplace metadata row.
    Aligned,
    /// Runtime contract version is newer than the published SDK release
    /// bundle; SDK consumers may not see this row yet.
    SdkLags,
    /// Runtime contract version is older than the marketplace metadata;
    /// runtime truth has not picked up the latest marketplace state.
    MarketplaceLeads,
    /// SDK release bundle and marketplace metadata disagree.
    SdkMarketplaceDrift,
    /// Alignment evidence is missing or stale; treated as a beta-blocking
    /// state.
    Unknown,
}

/// Inputs supplied by the extension host to build a runtime v1 beta
/// admission contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeV1BetaContractInput {
    /// Stable runtime-contract id.
    pub contract_id: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version declared by the manifest baseline.
    pub extension_version: String,
    /// Ref to the manifest-baseline record consumed by this contract.
    pub manifest_baseline_ref: String,
    /// Install-decision class consumed from the manifest baseline lane.
    pub manifest_install_decision_class: InstallDecisionClass,
    /// Install-decision reason class consumed from the manifest baseline lane.
    pub manifest_install_decision_reason_class: InstallDecisionReasonClass,
    /// Host contract family declared by the manifest baseline.
    pub host_contract_family_class: HostContractFamilyClass,
    /// Closed host-placement class for this contract.
    pub host_placement_class: HostPlacementClass,
    /// Closed host-supervision class for this contract.
    pub host_supervision_class: HostSupervisionClass,
    /// Ref to the host-negotiation packet.
    pub host_negotiation_packet_ref: String,
    /// Opaque refs to the capability worlds the manifest declared.
    pub declared_capability_world_refs: Vec<String>,
    /// Opaque refs to the capability worlds the runtime admitted for this session.
    pub negotiated_capability_world_refs: Vec<String>,
    /// Opaque refs to the worlds declared but dropped from the negotiated set.
    pub narrowed_capability_world_refs: Vec<String>,
    /// Whether at least one declared world carries a typed narrowing
    /// reason or unsupported-world decision on the negotiation packet.
    pub narrowing_reasons_recorded: bool,
    /// Ref to the effective-permission summary consumed from the
    /// manifest baseline lane.
    pub effective_permission_summary_ref: String,
    /// Whether the declared-vs-effective permission diff was emitted by
    /// the effective-permission summary.
    pub effective_permission_diff_present: bool,
    /// Number of widening attempts the effective-permission summary blocked.
    pub effective_permission_widening_attempted_blocked_count: u32,
    /// Current lifecycle state class.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    /// Current restart posture class.
    pub restart_posture_class: RestartPostureClass,
    /// Restart attempts recorded so far in the current host session.
    pub restart_attempt_count: u32,
    /// Current degraded-state class.
    pub degraded_state_class: DegradedStateClass,
    /// Opaque ref to the runtime-budget summary for this contract.
    pub runtime_budget_summary_ref: String,
    /// Whether the runtime-budget evidence reports an active
    /// quarantine, disable, or crash-loop trip on any axis.
    pub runtime_budget_quarantine_active: bool,
    /// Whether the runtime-budget evidence reports an active crash-loop trip.
    pub runtime_budget_crash_loop_active: bool,
    /// Opaque ref to the SDK release bundle the runtime is aligned with.
    pub sdk_release_bundle_ref: String,
    /// Opaque ref to the marketplace metadata row the runtime is aligned with.
    pub marketplace_metadata_ref: String,
    /// SDK / marketplace alignment class.
    pub sdk_alignment_class: SdkAlignmentClass,
    /// Audit event refs emitted while building this contract.
    pub audit_event_refs: Vec<String>,
    /// Decision timestamp (ISO 8601 UTC).
    pub decided_at: String,
}

/// One runtime v1 beta admission contract record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeV1BetaContractRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this beta record.
    pub runtime_v1_beta_schema_version: u32,
    /// Stable runtime-contract id.
    pub contract_id: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version declared by the manifest baseline.
    pub extension_version: String,
    /// Ref to the manifest-baseline record consumed by this contract.
    pub manifest_baseline_ref: String,
    /// Install-decision class consumed from the manifest baseline lane.
    pub manifest_install_decision_class: InstallDecisionClass,
    /// Install-decision reason class consumed from the manifest baseline lane.
    pub manifest_install_decision_reason_class: InstallDecisionReasonClass,
    /// Host contract family declared by the manifest baseline.
    pub host_contract_family_class: HostContractFamilyClass,
    /// Closed host-placement class for this contract.
    pub host_placement_class: HostPlacementClass,
    /// Closed host-supervision class for this contract.
    pub host_supervision_class: HostSupervisionClass,
    /// Ref to the host-negotiation packet.
    pub host_negotiation_packet_ref: String,
    /// Opaque refs to the capability worlds the manifest declared.
    pub declared_capability_world_refs: Vec<String>,
    /// Opaque refs to the capability worlds the runtime admitted for this session.
    pub negotiated_capability_world_refs: Vec<String>,
    /// Opaque refs to the worlds declared but dropped from the negotiated set.
    pub narrowed_capability_world_refs: Vec<String>,
    /// Whether at least one declared world carries a typed narrowing
    /// reason on the negotiation packet.
    pub narrowing_reasons_recorded: bool,
    /// Ref to the effective-permission summary consumed from the
    /// manifest baseline lane.
    pub effective_permission_summary_ref: String,
    /// Whether the declared-vs-effective permission diff was emitted by
    /// the effective-permission summary.
    pub effective_permission_diff_present: bool,
    /// Number of widening attempts the effective-permission summary blocked.
    pub effective_permission_widening_attempted_blocked_count: u32,
    /// Current lifecycle state class.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    /// Current restart posture class.
    pub restart_posture_class: RestartPostureClass,
    /// Restart attempts recorded so far in the current host session.
    pub restart_attempt_count: u32,
    /// Current degraded-state class.
    pub degraded_state_class: DegradedStateClass,
    /// Opaque ref to the runtime-budget summary for this contract.
    pub runtime_budget_summary_ref: String,
    /// Whether the runtime-budget evidence reports an active
    /// quarantine, disable, or crash-loop trip on any axis.
    pub runtime_budget_quarantine_active: bool,
    /// Whether the runtime-budget evidence reports an active crash-loop trip.
    pub runtime_budget_crash_loop_active: bool,
    /// Opaque ref to the SDK release bundle the runtime is aligned with.
    pub sdk_release_bundle_ref: String,
    /// Opaque ref to the marketplace metadata row the runtime is aligned with.
    pub marketplace_metadata_ref: String,
    /// SDK / marketplace alignment class.
    pub sdk_alignment_class: SdkAlignmentClass,
    /// Audit event refs emitted while building this contract.
    pub audit_event_refs: Vec<String>,
    /// Decision timestamp (ISO 8601 UTC).
    pub decided_at: String,
    /// Decision class emitted by [`evaluate_runtime_v1_beta_contract`].
    pub admission_decision_class: RuntimeAdmissionDecisionClass,
    /// Typed reason paired with the decision.
    pub admission_reason_class: RuntimeAdmissionReasonClass,
    /// Export-safe decision summary.
    pub admission_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// First consumer projection: a metadata-safe support / partner export
/// row that quotes the same closed tokens as the contract record without
/// leaking raw manifest, signing-key, or runtime-payload bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeV1BetaSupportExportRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this beta record.
    pub runtime_v1_beta_schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the runtime-contract record this export quotes.
    pub contract_ref: String,
    /// Extension identity rendered on the export row.
    pub extension_identity_ref: String,
    /// Extension version rendered on the export row.
    pub extension_version: String,
    /// Host contract family rendered on the export row.
    pub host_contract_family_class: HostContractFamilyClass,
    /// Host placement class rendered on the export row.
    pub host_placement_class: HostPlacementClass,
    /// Host supervision class rendered on the export row.
    pub host_supervision_class: HostSupervisionClass,
    /// Lifecycle state class rendered on the export row.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    /// Restart posture class rendered on the export row.
    pub restart_posture_class: RestartPostureClass,
    /// Restart attempt count rendered on the export row.
    pub restart_attempt_count: u32,
    /// Degraded state class rendered on the export row.
    pub degraded_state_class: DegradedStateClass,
    /// Number of declared capability worlds.
    pub declared_world_count: u32,
    /// Number of negotiated capability worlds.
    pub negotiated_world_count: u32,
    /// Number of declared worlds dropped from the negotiated set.
    pub narrowed_world_count: u32,
    /// SDK alignment class rendered on the export row.
    pub sdk_alignment_class: SdkAlignmentClass,
    /// Admission decision class rendered on the export row.
    pub admission_decision_class: RuntimeAdmissionDecisionClass,
    /// Admission reason class rendered on the export row.
    pub admission_reason_class: RuntimeAdmissionReasonClass,
    /// Whether the export blocks activation (denied or quarantined).
    pub blocks_activation: bool,
    /// Export-safe summary suitable for support / partner consumers.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by runtime v1 beta validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV1BetaFinding {
    /// Stable validation check id.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl RuntimeV1BetaFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate a runtime v1 beta admission contract.
///
/// The function is deterministic and fails closed on every typed guardrail:
///
/// - opaque or quarantined publisher identity from the manifest decision,
/// - a missing declared-vs-effective permission diff,
/// - a widening attempt the effective-permission summary already blocked,
/// - a host placement / supervision combination the contract has not
///   reserved,
/// - a capability negotiation that admitted zero worlds, or admitted
///   fewer worlds than declared without any recorded narrowing reason,
/// - a lifecycle state that cannot be admitted (`removed`,
///   `publisher_blocked`, or `quarantined`),
/// - a runtime-budget summary that reports a quarantine, disable, or
///   crash-loop trip, and
/// - an SDK / marketplace alignment class outside of [`SdkAlignmentClass::Aligned`].
pub fn evaluate_runtime_v1_beta_contract(
    input: RuntimeV1BetaContractInput,
) -> RuntimeV1BetaContractRecord {
    let (admission_decision_class, admission_reason_class, admission_summary) =
        admission_outcome(&input);

    RuntimeV1BetaContractRecord {
        record_kind: RUNTIME_V1_BETA_CONTRACT_RECORD_KIND.to_string(),
        runtime_v1_beta_schema_version: RUNTIME_V1_BETA_SCHEMA_VERSION,
        contract_id: input.contract_id,
        extension_identity_ref: input.extension_identity_ref,
        extension_version: input.extension_version,
        manifest_baseline_ref: input.manifest_baseline_ref,
        manifest_install_decision_class: input.manifest_install_decision_class,
        manifest_install_decision_reason_class: input.manifest_install_decision_reason_class,
        host_contract_family_class: input.host_contract_family_class,
        host_placement_class: input.host_placement_class,
        host_supervision_class: input.host_supervision_class,
        host_negotiation_packet_ref: input.host_negotiation_packet_ref,
        declared_capability_world_refs: input.declared_capability_world_refs,
        negotiated_capability_world_refs: input.negotiated_capability_world_refs,
        narrowed_capability_world_refs: input.narrowed_capability_world_refs,
        narrowing_reasons_recorded: input.narrowing_reasons_recorded,
        effective_permission_summary_ref: input.effective_permission_summary_ref,
        effective_permission_diff_present: input.effective_permission_diff_present,
        effective_permission_widening_attempted_blocked_count: input
            .effective_permission_widening_attempted_blocked_count,
        lifecycle_state_class: input.lifecycle_state_class,
        restart_posture_class: input.restart_posture_class,
        restart_attempt_count: input.restart_attempt_count,
        degraded_state_class: input.degraded_state_class,
        runtime_budget_summary_ref: input.runtime_budget_summary_ref,
        runtime_budget_quarantine_active: input.runtime_budget_quarantine_active,
        runtime_budget_crash_loop_active: input.runtime_budget_crash_loop_active,
        sdk_release_bundle_ref: input.sdk_release_bundle_ref,
        marketplace_metadata_ref: input.marketplace_metadata_ref,
        sdk_alignment_class: input.sdk_alignment_class,
        audit_event_refs: input.audit_event_refs,
        decided_at: input.decided_at,
        admission_decision_class,
        admission_reason_class,
        admission_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a [`RuntimeV1BetaContractRecord`] into the first consumer
/// surface: a metadata-safe support / partner export row.
///
/// The export is intentionally minimal. It repeats only the closed
/// tokens (host placement / supervision, lifecycle, restart posture,
/// degraded state, SDK alignment, admission decision / reason) plus
/// scalar counts of declared / negotiated / narrowed worlds. It never
/// embeds raw manifests, raw signatures, raw policy bodies, raw paths,
/// raw tokens, or raw runtime payload bytes.
pub fn project_runtime_v1_beta_support_export(
    contract: &RuntimeV1BetaContractRecord,
) -> RuntimeV1BetaSupportExportRecord {
    let blocks_activation = matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused | RuntimeAdmissionDecisionClass::Quarantined
    );
    let declared_world_count = contract.declared_capability_world_refs.len() as u32;
    let negotiated_world_count = contract.negotiated_capability_world_refs.len() as u32;
    let narrowed_world_count = contract.narrowed_capability_world_refs.len() as u32;

    let export_safe_summary = format!(
        "{} Host: placement={:?} supervision={:?} family={:?}. Worlds: declared={} negotiated={} narrowed={}. Lifecycle={:?} restart={:?}(x{}) degraded={:?}. SDK alignment={:?}.",
        contract.admission_summary,
        contract.host_placement_class,
        contract.host_supervision_class,
        contract.host_contract_family_class,
        declared_world_count,
        negotiated_world_count,
        narrowed_world_count,
        contract.lifecycle_state_class,
        contract.restart_posture_class,
        contract.restart_attempt_count,
        contract.degraded_state_class,
        contract.sdk_alignment_class,
    );

    RuntimeV1BetaSupportExportRecord {
        record_kind: RUNTIME_V1_BETA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        runtime_v1_beta_schema_version: RUNTIME_V1_BETA_SCHEMA_VERSION,
        export_id: format!("runtime_v1_beta_support_export:{}", contract.contract_id),
        contract_ref: contract.contract_id.clone(),
        extension_identity_ref: contract.extension_identity_ref.clone(),
        extension_version: contract.extension_version.clone(),
        host_contract_family_class: contract.host_contract_family_class,
        host_placement_class: contract.host_placement_class,
        host_supervision_class: contract.host_supervision_class,
        lifecycle_state_class: contract.lifecycle_state_class,
        restart_posture_class: contract.restart_posture_class,
        restart_attempt_count: contract.restart_attempt_count,
        degraded_state_class: contract.degraded_state_class,
        declared_world_count,
        negotiated_world_count,
        narrowed_world_count,
        sdk_alignment_class: contract.sdk_alignment_class,
        admission_decision_class: contract.admission_decision_class,
        admission_reason_class: contract.admission_reason_class,
        blocks_activation,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a runtime v1 beta contract record.
pub fn validate_runtime_v1_beta_contract(
    contract: &RuntimeV1BetaContractRecord,
) -> Vec<RuntimeV1BetaFinding> {
    let mut findings = Vec::new();

    if contract.record_kind != RUNTIME_V1_BETA_CONTRACT_RECORD_KIND {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.record_kind_wrong",
            format!(
                "record_kind must be '{RUNTIME_V1_BETA_CONTRACT_RECORD_KIND}'; got {:?}",
                contract.record_kind
            ),
        ));
    }
    if contract.runtime_v1_beta_schema_version != RUNTIME_V1_BETA_SCHEMA_VERSION {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.schema_version_wrong",
            format!(
                "runtime_v1_beta_schema_version must be {RUNTIME_V1_BETA_SCHEMA_VERSION}; got {}",
                contract.runtime_v1_beta_schema_version
            ),
        ));
    }
    if !contract.contract_id.starts_with("runtime_v1_beta:") {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.id_unprefixed",
            "contract_id must start with 'runtime_v1_beta:'",
        ));
    }
    if contract.extension_identity_ref.trim().is_empty() {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.extension_identity_required",
            "extension_identity_ref must be a non-empty ref",
        ));
    }
    if !contract
        .manifest_baseline_ref
        .starts_with("manifest_baseline:")
    {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.manifest_baseline_ref_unprefixed",
            "manifest_baseline_ref must start with 'manifest_baseline:'",
        ));
    }
    if !host_placement_supports_family(
        contract.host_placement_class,
        contract.host_contract_family_class,
    ) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.placement_family_mismatch",
            format!(
                "host_placement_class {:?} is not reserved for host_contract_family_class {:?}",
                contract.host_placement_class, contract.host_contract_family_class
            ),
        ));
    }
    if !host_supervision_supports_placement(
        contract.host_supervision_class,
        contract.host_placement_class,
    ) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.supervision_placement_mismatch",
            format!(
                "host_supervision_class {:?} is not reserved for host_placement_class {:?}",
                contract.host_supervision_class, contract.host_placement_class
            ),
        ));
    }
    if !negotiated_is_subset(
        &contract.negotiated_capability_world_refs,
        &contract.declared_capability_world_refs,
    ) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.negotiated_widens_declared",
            "negotiated_capability_world_refs must be a subset of declared_capability_world_refs",
        ));
    }
    if narrowed_worlds_disagree_with_diff(contract) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.narrowing_diff_inconsistent",
            "narrowed_capability_world_refs is non-empty but narrowing_reasons_recorded is false",
        ));
    }
    if matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Admitted | RuntimeAdmissionDecisionClass::AdmittedNarrowed
    ) && !matches!(
        contract.lifecycle_state_class,
        RuntimeLifecycleStateClass::Installed
            | RuntimeLifecycleStateClass::PendingActivation
            | RuntimeLifecycleStateClass::Active
            | RuntimeLifecycleStateClass::Recovered
    ) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.admitted_lifecycle_not_runnable",
            format!(
                "admission_decision_class {:?} requires a runnable lifecycle_state_class; got {:?}",
                contract.admission_decision_class, contract.lifecycle_state_class
            ),
        ));
    }
    if matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Quarantined
    ) && !matches!(
        contract.lifecycle_state_class,
        RuntimeLifecycleStateClass::Quarantined
    ) {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.quarantined_lifecycle_mismatch",
            "admission_decision_class quarantined requires lifecycle_state_class quarantined",
        ));
    }
    if matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused | RuntimeAdmissionDecisionClass::Quarantined
    ) && contract.restart_attempt_count > 0
        && contract.restart_posture_class == RestartPostureClass::NoRestartAttempted
    {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.restart_posture_inconsistent",
            "restart_attempt_count > 0 contradicts restart_posture_class=no_restart_attempted",
        ));
    }
    if !contract
        .host_negotiation_packet_ref
        .starts_with("host_negotiation:")
    {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.host_negotiation_packet_ref_unprefixed",
            "host_negotiation_packet_ref must start with 'host_negotiation:'",
        ));
    }
    if !contract
        .sdk_release_bundle_ref
        .starts_with("sdk_release_bundle:")
    {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.sdk_release_bundle_ref_unprefixed",
            "sdk_release_bundle_ref must start with 'sdk_release_bundle:'",
        ));
    }
    if !contract
        .marketplace_metadata_ref
        .starts_with("marketplace_metadata:")
    {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.marketplace_metadata_ref_unprefixed",
            "marketplace_metadata_ref must start with 'marketplace_metadata:'",
        ));
    }
    if contract.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(RuntimeV1BetaFinding::new(
            "runtime_v1_beta.contract.redaction_class_must_be_metadata_safe",
            "runtime v1 beta records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    findings
}

fn admission_outcome(
    input: &RuntimeV1BetaContractInput,
) -> (
    RuntimeAdmissionDecisionClass,
    RuntimeAdmissionReasonClass,
    String,
) {
    if matches!(
        input.manifest_install_decision_class,
        InstallDecisionClass::Denied
    ) {
        let reason = match input.manifest_install_decision_reason_class {
            InstallDecisionReasonClass::PublisherAnonymous
            | InstallDecisionReasonClass::PublisherIdentityRequired
            | InstallDecisionReasonClass::PublisherQuarantined
            | InstallDecisionReasonClass::PublisherLifecycleRetired => {
                RuntimeAdmissionReasonClass::PublisherIdentityOpaque
            }
            InstallDecisionReasonClass::EffectivePermissionWideningAttempted => {
                RuntimeAdmissionReasonClass::EffectivePermissionWideningAttempted
            }
            _ => RuntimeAdmissionReasonClass::ManifestInstallDenied,
        };
        return (
            RuntimeAdmissionDecisionClass::Refused,
            reason,
            "Refused: manifest install decision denied the row; runtime v1 beta cannot admit."
                .to_string(),
        );
    }

    if !input.effective_permission_diff_present {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::PermissionDiffMissing,
            "Refused: declared-vs-effective permission diff is missing; runtime v1 beta requires the diff.".to_string(),
        );
    }
    if input.effective_permission_widening_attempted_blocked_count > 0 {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::EffectivePermissionWideningAttempted,
            "Refused: effective-permission summary blocked a widening attempt; runtime v1 beta refuses.".to_string(),
        );
    }
    if matches!(
        input.host_placement_class,
        HostPlacementClass::UnknownPlacementClass
    ) || matches!(
        input.host_supervision_class,
        HostSupervisionClass::UnknownSupervisionClass
    ) || !host_placement_supports_family(
        input.host_placement_class,
        input.host_contract_family_class,
    ) || !host_supervision_supports_placement(
        input.host_supervision_class,
        input.host_placement_class,
    ) {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::HostPlacementUnsupported,
            "Refused: host placement or supervision is unknown or not reserved for the declared host contract family.".to_string(),
        );
    }

    if matches!(
        input.lifecycle_state_class,
        RuntimeLifecycleStateClass::PublisherBlocked
    ) {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::PublisherIdentityOpaque,
            "Refused: lifecycle state publisher_blocked refuses runtime admission until the publisher block clears.".to_string(),
        );
    }
    if matches!(
        input.lifecycle_state_class,
        RuntimeLifecycleStateClass::Removed | RuntimeLifecycleStateClass::Discovered
    ) {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::LifecycleTerminalState,
            "Refused: lifecycle state is not installable on this row.".to_string(),
        );
    }

    if input.runtime_budget_crash_loop_active {
        return (
            RuntimeAdmissionDecisionClass::Quarantined,
            RuntimeAdmissionReasonClass::CrashLoopQuarantineActive,
            "Quarantined: crash-loop trip is active; runtime admission is held until the quarantine clears.".to_string(),
        );
    }
    if input.runtime_budget_quarantine_active
        || matches!(
            input.lifecycle_state_class,
            RuntimeLifecycleStateClass::Quarantined
        )
    {
        return (
            RuntimeAdmissionDecisionClass::Quarantined,
            RuntimeAdmissionReasonClass::RuntimeBudgetQuarantineActive,
            "Quarantined: runtime-budget evidence reports an active quarantine, disable, or trip."
                .to_string(),
        );
    }

    if input.declared_capability_world_refs.is_empty() {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::CapabilityWorldUnavailableOnHost,
            "Refused: manifest declared no capability worlds; runtime v1 beta requires at least one.".to_string(),
        );
    }
    if input.negotiated_capability_world_refs.is_empty() {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::CapabilityWorldUnavailableOnHost,
            "Refused: host negotiation admitted zero capability worlds for this row.".to_string(),
        );
    }
    if !negotiated_is_subset(
        &input.negotiated_capability_world_refs,
        &input.declared_capability_world_refs,
    ) {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::HostPlacementUnsupported,
            "Refused: negotiated capability worlds widened beyond the declared set; widening is forbidden.".to_string(),
        );
    }
    if !input.narrowed_capability_world_refs.is_empty() && !input.narrowing_reasons_recorded {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::PermissionDiffMissing,
            "Refused: declared worlds were narrowed but no narrowing reasons were recorded."
                .to_string(),
        );
    }

    if !matches!(input.sdk_alignment_class, SdkAlignmentClass::Aligned) {
        return (
            RuntimeAdmissionDecisionClass::Refused,
            RuntimeAdmissionReasonClass::SdkOrMarketplaceMetadataOutOfDate,
            "Refused: SDK or marketplace metadata is not aligned with the runtime v1 beta contract version.".to_string(),
        );
    }

    let narrowed = !input.narrowed_capability_world_refs.is_empty();
    if narrowed
        && matches!(
            input.lifecycle_state_class,
            RuntimeLifecycleStateClass::PendingActivation
        )
    {
        return (
            RuntimeAdmissionDecisionClass::AwaitingUserReview,
            RuntimeAdmissionReasonClass::AwaitingUserWorldAcknowledgement,
            "Awaiting user review: narrowed capability worlds require a user acknowledgement before activation resumes.".to_string(),
        );
    }
    if narrowed {
        return (
            RuntimeAdmissionDecisionClass::AdmittedNarrowed,
            RuntimeAdmissionReasonClass::AdmittedWithWorldNarrowing,
            "Admitted with world narrowing: a strict subset of declared worlds was admitted under typed narrowing reasons.".to_string(),
        );
    }

    (
        RuntimeAdmissionDecisionClass::Admitted,
        RuntimeAdmissionReasonClass::AdmittedAfterCapabilityNegotiation,
        "Admitted after capability negotiation: every declared world was admitted under the current host placement, supervision, lifecycle, and SDK alignment.".to_string(),
    )
}

fn host_placement_supports_family(
    placement: HostPlacementClass,
    family: HostContractFamilyClass,
) -> bool {
    matches!(
        (placement, family),
        (
            HostPlacementClass::WasmInProcessIsolatedWorld,
            HostContractFamilyClass::WasmComponentModel | HostContractFamilyClass::WasmCoreModule,
        ) | (
            HostPlacementClass::WasmIsolatedSubprocess,
            HostContractFamilyClass::WasmComponentModel | HostContractFamilyClass::WasmCoreModule,
        ) | (
            HostPlacementClass::ExternalHostSupervisedProcess,
            HostContractFamilyClass::ExternalHostProcess,
        ) | (
            HostPlacementClass::HelperBinaryShortLived,
            HostContractFamilyClass::HelperBinary,
        ) | (
            HostPlacementClass::RemoteSideComponentAttached,
            HostContractFamilyClass::RemoteSideComponent,
        ) | (
            HostPlacementClass::CompatibilityBridgeTranslated,
            HostContractFamilyClass::CompatibilityBridge,
        )
    )
}

fn host_supervision_supports_placement(
    supervision: HostSupervisionClass,
    placement: HostPlacementClass,
) -> bool {
    matches!(
        (supervision, placement),
        (
            HostSupervisionClass::InProcessCapabilitySandbox,
            HostPlacementClass::WasmInProcessIsolatedWorld,
        ) | (
            HostSupervisionClass::SeparateSubprocessSupervised,
            HostPlacementClass::WasmIsolatedSubprocess
                | HostPlacementClass::ExternalHostSupervisedProcess,
        ) | (
            HostSupervisionClass::SeparateSubprocessHelperKillSwitch,
            HostPlacementClass::HelperBinaryShortLived,
        ) | (
            HostSupervisionClass::RemoteAgentAttachedEnvelope,
            HostPlacementClass::RemoteSideComponentAttached,
        ) | (
            HostSupervisionClass::CompatibilityBridgeTranslatedSupervision,
            HostPlacementClass::CompatibilityBridgeTranslated,
        )
    )
}

fn negotiated_is_subset(negotiated: &[String], declared: &[String]) -> bool {
    negotiated.iter().all(|world| declared.contains(world))
}

fn narrowed_worlds_disagree_with_diff(contract: &RuntimeV1BetaContractRecord) -> bool {
    !contract.narrowed_capability_world_refs.is_empty() && !contract.narrowing_reasons_recorded
}
