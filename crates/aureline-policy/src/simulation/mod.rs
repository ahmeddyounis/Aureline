//! Policy simulation, exception lifecycle, and remembered-decision narrowing.
//!
//! The module provides the beta contract for pre-apply policy previews,
//! exception and waiver expiry, remembered-decision drift checks, and
//! metadata-safe support exports that preserve the policy state in force at
//! action time.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version exported by policy simulation beta records.
pub const POLICY_SIMULATION_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, support, admin, and fixtures.
pub const POLICY_SIMULATION_SHARED_CONTRACT_REF: &str =
    "policy:simulation_exception_memory_beta:v1";

/// Stable record kind for [`PolicySimulationBetaPage`] payloads.
pub const POLICY_SIMULATION_BETA_PAGE_RECORD_KIND: &str = "policy_simulation_beta_page_record";

/// Stable record kind for [`PolicySimulationRecord`] payloads.
pub const POLICY_SIMULATION_RECORD_KIND: &str = "policy_simulation_record";

/// Stable record kind for [`AffectedPolicySurface`] payloads.
pub const POLICY_SIMULATION_AFFECTED_SURFACE_RECORD_KIND: &str =
    "policy_simulation_affected_surface_record";

/// Stable record kind for [`ExceptionalAuthorityRecord`] payloads.
pub const POLICY_SIMULATION_EXCEPTION_RECORD_KIND: &str = "policy_exception_or_waiver_record";

/// Stable record kind for [`RememberedDecisionRecord`] payloads.
pub const POLICY_SIMULATION_REMEMBERED_DECISION_RECORD_KIND: &str =
    "policy_remembered_decision_record";

/// Stable record kind for [`PolicyStateAtActionTime`] payloads.
pub const POLICY_SIMULATION_STATE_AT_ACTION_RECORD_KIND: &str =
    "policy_state_at_action_time_record";

/// Stable record kind for [`PolicySimulationSummary`] payloads.
pub const POLICY_SIMULATION_SUMMARY_RECORD_KIND: &str = "policy_simulation_summary_record";

/// Stable record kind for [`PolicySimulationBetaDefect`] payloads.
pub const POLICY_SIMULATION_BETA_DEFECT_RECORD_KIND: &str = "policy_simulation_beta_defect_record";

/// Stable record kind for [`PolicySimulationSupportExport`] payloads.
pub const POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_simulation_support_export_record";

/// Policy change class that can be previewed before application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyChangeClass {
    /// A signed policy bundle or bundle epoch changes.
    PolicyBundleChange,
    /// A managed setting lock, unlock, or forced value changes.
    SettingsLockChange,
}

impl PolicyChangeClass {
    /// Stable token recorded on policy simulation records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBundleChange => "policy_bundle_change",
            Self::SettingsLockChange => "settings_lock_change",
        }
    }
}

/// Scope kind for policy simulations, exceptions, and remembered decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// Workspace-local scope.
    Workspace,
    /// Root or repository scope.
    Root,
    /// Workset scope.
    Workset,
    /// Session scope.
    Session,
    /// Tenant scope.
    Tenant,
    /// Fleet-wide scope.
    Fleet,
}

impl ScopeKind {
    /// Stable token recorded on policy records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Root => "root",
            Self::Workset => "workset",
            Self::Session => "session",
            Self::Tenant => "tenant",
            Self::Fleet => "fleet",
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::Session => 0,
            Self::Workset => 1,
            Self::Root => 2,
            Self::Workspace => 3,
            Self::Tenant => 4,
            Self::Fleet => 5,
        }
    }
}

/// Persona class affected by a simulated policy change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorPersonaClass {
    /// End user operating local or managed project workflows.
    EndUser,
    /// Workspace administrator or repository owner.
    WorkspaceAdmin,
    /// Organization or fleet policy administrator.
    OrganizationAdmin,
    /// Support operator consuming redacted packets.
    SupportOperator,
    /// Automation or service principal.
    AutomationPrincipal,
}

impl ActorPersonaClass {
    /// Stable token recorded on affected-surface rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EndUser => "end_user",
            Self::WorkspaceAdmin => "workspace_admin",
            Self::OrganizationAdmin => "organization_admin",
            Self::SupportOperator => "support_operator",
            Self::AutomationPrincipal => "automation_principal",
        }
    }
}

/// Action family affected by policy simulation or remembered decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionFamilyClass {
    /// Connected-provider grant, publish, or mutation.
    ConnectedProviderMutation,
    /// AI apply or generated patch mutation.
    AiApplyMutation,
    /// Settings write or managed configuration change.
    SettingsWrite,
    /// Support export or handoff packet assembly.
    SupportExport,
    /// Retention, hold, export, or deletion lifecycle action.
    RecordsLifecycle,
}

impl ActionFamilyClass {
    /// Stable token recorded on policy records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectedProviderMutation => "connected_provider_mutation",
            Self::AiApplyMutation => "ai_apply_mutation",
            Self::SettingsWrite => "settings_write",
            Self::SupportExport => "support_export",
            Self::RecordsLifecycle => "records_lifecycle",
        }
    }
}

/// Degraded mode expected when a policy change narrows a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedModeClass {
    /// The action remains fully available.
    FullAccess,
    /// The action can preview but cannot apply.
    PreviewOnly,
    /// The action is read-only.
    ReadOnly,
    /// A fresh approval or step-up is required.
    RequiresReapproval,
    /// The action is blocked by policy.
    BlockedByPolicy,
    /// The action is deferred until policy or source refresh.
    PausedUntilRefresh,
}

impl DegradedModeClass {
    /// Stable token recorded on affected-surface rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAccess => "full_access",
            Self::PreviewOnly => "preview_only",
            Self::ReadOnly => "read_only",
            Self::RequiresReapproval => "requires_reapproval",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::PausedUntilRefresh => "paused_until_refresh",
        }
    }
}

/// Protected-path implication of a policy change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedPathChangeClass {
    /// No protected path changes.
    None,
    /// Policy narrows write or mutation authority.
    WritesNarrowed,
    /// Policy blocks write or mutation authority.
    WritesDenied,
    /// Policy changes egress or route exposure posture.
    EgressNarrowed,
    /// Policy changes support export or evidence retention posture.
    EvidenceExportNarrowed,
    /// Policy locks a managed setting.
    ManagedSettingLocked,
}

impl ProtectedPathChangeClass {
    /// Stable token recorded on affected-surface rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WritesNarrowed => "writes_narrowed",
            Self::WritesDenied => "writes_denied",
            Self::EgressNarrowed => "egress_narrowed",
            Self::EvidenceExportNarrowed => "evidence_export_narrowed",
            Self::ManagedSettingLocked => "managed_setting_locked",
        }
    }
}

/// Exception or waiver kind represented by [`ExceptionalAuthorityRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExceptionKindClass {
    /// Admin-authorized policy exception.
    PolicyException,
    /// Temporary waiver for policy narrowing or rollout timing.
    PolicyWaiver,
    /// Deferred reapproval window.
    AdminReconfirmationDefer,
    /// Exception that binds a remembered decision to an expiry envelope.
    RememberedDecisionBinding,
}

impl ExceptionKindClass {
    /// Stable token recorded on exception records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyException => "policy_exception",
            Self::PolicyWaiver => "policy_waiver",
            Self::AdminReconfirmationDefer => "admin_reconfirmation_defer",
            Self::RememberedDecisionBinding => "remembered_decision_binding",
        }
    }
}

/// Lifecycle state for an exception or waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExceptionalAuthorityStatusClass {
    /// The exception is currently active.
    Active,
    /// The exception expires soon and needs visible review.
    ExpiringSoon,
    /// The exception has expired and must prompt for renewal or cleanup.
    ExpiredReapprovalRequired,
    /// The exception was superseded by a later record.
    Superseded,
    /// The exception was revoked before expiry.
    Revoked,
    /// Policy forcibly retired the exception.
    ForceRetiredByPolicy,
}

impl ExceptionalAuthorityStatusClass {
    /// Stable token recorded on exception records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::ExpiredReapprovalRequired => "expired_reapproval_required",
            Self::Superseded => "superseded",
            Self::Revoked => "revoked",
            Self::ForceRetiredByPolicy => "force_retired_by_policy",
        }
    }
}

/// Renewal route for an exception or waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenewalPathClass {
    /// Re-ask the original user.
    RepromptUser,
    /// Require administrator reconfirmation.
    RequireAdminReconfirmation,
    /// Require the original author to confirm.
    RequireOriginalAuthor,
    /// No renewal is allowed.
    NoRenewalAllowed,
}

impl RenewalPathClass {
    /// Stable token recorded on exception records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepromptUser => "reprompt_user",
            Self::RequireAdminReconfirmation => "require_admin_reconfirmation",
            Self::RequireOriginalAuthor => "require_original_author",
            Self::NoRenewalAllowed => "no_renewal_allowed",
        }
    }
}

/// Revocation route for an exception or waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationPathClass {
    /// Owner can revoke through admin review.
    OwnerAdminReview,
    /// Policy epoch change revokes the record.
    PolicyEpochChange,
    /// Security lane revokes the record.
    SecurityLane,
    /// Support operator escalates revocation.
    SupportEscalation,
}

impl RevocationPathClass {
    /// Stable token recorded on exception records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerAdminReview => "owner_admin_review",
            Self::PolicyEpochChange => "policy_epoch_change",
            Self::SecurityLane => "security_lane",
            Self::SupportEscalation => "support_escalation",
        }
    }
}

/// Remembered-decision state after drift and expiry checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryStateClass {
    /// The remembered decision is active for its exact bindings.
    Active,
    /// The remembered decision carries forward only to a narrower scope.
    Narrowed,
    /// The decision expired by time horizon.
    Expired,
    /// A fresh approval is required before reuse.
    RequiresReapproval,
    /// Policy or authority drift retired the decision.
    ForceRetiredByPolicy,
    /// A newer remembered decision superseded this record.
    Superseded,
}

impl MemoryStateClass {
    /// Stable token recorded on remembered-decision rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Narrowed => "narrowed",
            Self::Expired => "expired",
            Self::RequiresReapproval => "requires_reapproval",
            Self::ForceRetiredByPolicy => "force_retired_by_policy",
            Self::Superseded => "superseded",
        }
    }
}

/// Drift reason that invalidates or narrows a remembered decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedDecisionDriftReason {
    /// The actor no longer matches the remembered binding.
    ActorChanged,
    /// The target object identity changed.
    ObjectChanged,
    /// The action family changed.
    ActionFamilyChanged,
    /// The environment binding changed.
    EnvironmentChanged,
    /// The policy epoch changed.
    PolicyEpochChanged,
    /// The target version changed.
    TargetVersionChanged,
    /// The authority epoch changed.
    AuthorityDrift,
    /// The time horizon elapsed.
    ExpiryElapsed,
}

impl RememberedDecisionDriftReason {
    /// Stable token recorded on remembered-decision rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorChanged => "actor_changed",
            Self::ObjectChanged => "object_changed",
            Self::ActionFamilyChanged => "action_family_changed",
            Self::EnvironmentChanged => "environment_changed",
            Self::PolicyEpochChanged => "policy_epoch_changed",
            Self::TargetVersionChanged => "target_version_changed",
            Self::AuthorityDrift => "authority_drift",
            Self::ExpiryElapsed => "expiry_elapsed",
        }
    }
}

/// Dashboard bucket used for exception and remembered-decision projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardBucketClass {
    /// The row is healthy.
    Healthy,
    /// The row expires soon.
    ExpiringSoon,
    /// The row is future-effective.
    FutureEffectivePending,
    /// The row carried forward only after narrowing.
    NarrowedCarryForward,
    /// The row expired and a prompt is due.
    ExpiredRepromptDue,
    /// The row is blocked by a hold.
    BlockedByHold,
    /// The row was force-retired by policy.
    ForceRetiredByPolicy,
    /// The row is superseded but needs cleanup.
    SupersededPendingCleanup,
    /// Drift was detected.
    DriftDetected,
}

impl DashboardBucketClass {
    /// Stable token recorded on dashboard-facing rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::ExpiringSoon => "expiring_soon",
            Self::FutureEffectivePending => "future_effective_pending",
            Self::NarrowedCarryForward => "narrowed_carry_forward",
            Self::ExpiredRepromptDue => "expired_reprompt_due",
            Self::BlockedByHold => "blocked_by_hold",
            Self::ForceRetiredByPolicy => "force_retired_by_policy",
            Self::SupersededPendingCleanup => "superseded_pending_cleanup",
            Self::DriftDetected => "drift_detected",
        }
    }
}

/// Typed validation defect for policy simulation beta records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySimulationBetaDefectKind {
    /// A required change class has no simulation.
    ChangeClassCoverageMissing,
    /// A simulation has no affected surfaces.
    SimulationWithoutAffectedSurfaces,
    /// An affected surface dropped persona truth.
    MissingAffectedPersona,
    /// An affected surface dropped action truth.
    MissingAffectedAction,
    /// An affected surface dropped degraded-mode truth.
    MissingDegradedMode,
    /// An affected surface dropped protected-path truth.
    MissingProtectedPathChange,
    /// An exception or waiver has no owner.
    ExceptionMissingOwner,
    /// An exception or waiver has no scope.
    ExceptionMissingScope,
    /// An exception or waiver has no expiry horizon.
    ExceptionMissingExpiry,
    /// An exception or waiver has no evidence lineage.
    ExceptionMissingEvidenceTrail,
    /// An exception or waiver has no renewal path.
    ExceptionMissingRenewalPath,
    /// An exception or waiver has no revocation path.
    ExceptionMissingRevocationPath,
    /// A remembered decision is missing actor binding.
    RememberedDecisionMissingActorBinding,
    /// A remembered decision is missing object binding.
    RememberedDecisionMissingObjectBinding,
    /// A remembered decision is missing environment binding.
    RememberedDecisionMissingEnvironmentBinding,
    /// A remembered decision is missing time horizon.
    RememberedDecisionMissingTimeHorizon,
    /// Drifted remembered memory did not expose an invalidation reason.
    RememberedDecisionDriftUnexplained,
    /// Carry-forward scope is broader than the simulation result.
    RememberedDecisionCarryForwardTooBroad,
    /// Support export lacks action-time policy state.
    SupportExportMissingActionTimePolicyState,
    /// Current-only policy truth would overwrite historical action truth.
    SupportExportHistoricalTruthDropped,
    /// A record would expose raw private material.
    RawPrivateMaterialExposed,
    /// Page summary does not match the audited record set.
    SummaryMismatch,
}

impl PolicySimulationBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeClassCoverageMissing => "change_class_coverage_missing",
            Self::SimulationWithoutAffectedSurfaces => "simulation_without_affected_surfaces",
            Self::MissingAffectedPersona => "missing_affected_persona",
            Self::MissingAffectedAction => "missing_affected_action",
            Self::MissingDegradedMode => "missing_degraded_mode",
            Self::MissingProtectedPathChange => "missing_protected_path_change",
            Self::ExceptionMissingOwner => "exception_missing_owner",
            Self::ExceptionMissingScope => "exception_missing_scope",
            Self::ExceptionMissingExpiry => "exception_missing_expiry",
            Self::ExceptionMissingEvidenceTrail => "exception_missing_evidence_trail",
            Self::ExceptionMissingRenewalPath => "exception_missing_renewal_path",
            Self::ExceptionMissingRevocationPath => "exception_missing_revocation_path",
            Self::RememberedDecisionMissingActorBinding => {
                "remembered_decision_missing_actor_binding"
            }
            Self::RememberedDecisionMissingObjectBinding => {
                "remembered_decision_missing_object_binding"
            }
            Self::RememberedDecisionMissingEnvironmentBinding => {
                "remembered_decision_missing_environment_binding"
            }
            Self::RememberedDecisionMissingTimeHorizon => {
                "remembered_decision_missing_time_horizon"
            }
            Self::RememberedDecisionDriftUnexplained => "remembered_decision_drift_unexplained",
            Self::RememberedDecisionCarryForwardTooBroad => {
                "remembered_decision_carry_forward_too_broad"
            }
            Self::SupportExportMissingActionTimePolicyState => {
                "support_export_missing_action_time_policy_state"
            }
            Self::SupportExportHistoricalTruthDropped => "support_export_historical_truth_dropped",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::SummaryMismatch => "summary_mismatch",
        }
    }
}

/// Actor identity safe for policy, support, and admin packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorRef {
    /// Stable opaque actor id.
    pub stable_id: String,
    /// Export-safe actor role.
    pub role: String,
}

/// Typed object identity used by simulations and remembered decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectRef {
    /// Subject kind token.
    pub subject_kind: String,
    /// Stable opaque subject id.
    pub subject_id: String,
}

/// Scope identity used by simulations, exceptions, and memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRef {
    /// Typed scope kind.
    pub scope_kind: ScopeKind,
    /// Stable token for [`Self::scope_kind`].
    pub scope_kind_token: String,
    /// Opaque scope id.
    pub scope_id: String,
}

impl ScopeRef {
    /// Builds a scope ref with the stable token populated.
    pub fn new(scope_kind: ScopeKind, scope_id: impl Into<String>) -> Self {
        Self {
            scope_kind,
            scope_kind_token: scope_kind.as_str().to_owned(),
            scope_id: scope_id.into(),
        }
    }

    fn is_equal_or_narrower_than(&self, other: &Self) -> bool {
        self.scope_kind.rank() <= other.scope_kind.rank()
    }
}

/// Time horizon used for expiry countdowns and reapproval prompts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeHorizon {
    /// UTC instant where the authority was declared.
    pub declared_at: String,
    /// UTC expiry instant.
    pub expires_at: String,
    /// UTC instant when UI should start surfacing reapproval.
    pub reapproval_prompt_at: String,
}

/// Environment binding for a remembered decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentBinding {
    /// Stable environment id.
    pub environment_id: String,
    /// Workspace ref in force when remembered.
    pub workspace_ref: String,
    /// Deployment or profile token.
    pub profile_token: String,
    /// Trust-state token.
    pub trust_state: String,
    /// Network posture token.
    pub network_posture: String,
}

/// Policy source snapshot used before, after, and at action time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContextSnapshot {
    /// Policy epoch that made the decision.
    pub policy_epoch: String,
    /// Active bundle ref.
    pub bundle_ref: String,
    /// Active bundle version.
    pub bundle_version: String,
    /// Source class token.
    pub source_class: String,
    /// UTC instant when this policy became effective for the scope.
    pub effective_at: String,
}

/// Input contract for one pre-apply simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationRequest {
    /// Stable simulation id.
    pub simulation_id: String,
    /// Change class being previewed.
    pub change_class: PolicyChangeClass,
    /// Stable token for [`Self::change_class`].
    pub change_class_token: String,
    /// Baseline policy or settings ref.
    pub baseline_ref: String,
    /// Proposed policy or settings ref.
    pub proposed_ref: String,
    /// Scope under preview.
    pub scope: ScopeRef,
    /// Actor requesting the preview.
    pub requested_by: ActorRef,
    /// UTC instant when the preview was evaluated.
    pub evaluated_at: String,
    /// UTC instant when the proposed change would become effective.
    pub effective_from: String,
}

/// One surface, command family, and degraded mode affected by a preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedPolicySurface {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable affected-surface id.
    pub affected_surface_id: String,
    /// Affected surface reference.
    pub surface_ref: SubjectRef,
    /// Persona affected by the change.
    pub affected_persona: ActorPersonaClass,
    /// Stable token for [`Self::affected_persona`].
    pub affected_persona_token: String,
    /// Action family affected by the change.
    pub action_family: ActionFamilyClass,
    /// Stable token for [`Self::action_family`].
    pub action_family_token: String,
    /// Commands affected on this surface.
    pub command_ids: Vec<String>,
    /// Degraded mode after the proposed change.
    pub degraded_mode: DegradedModeClass,
    /// Stable token for [`Self::degraded_mode`].
    pub degraded_mode_token: String,
    /// Protected-path change implied by the policy change.
    pub protected_path_change: ProtectedPathChangeClass,
    /// Stable token for [`Self::protected_path_change`].
    pub protected_path_change_token: String,
    /// User-visible consequence summary.
    pub consequence: String,
    /// Policy state before the change.
    pub policy_state_before: PolicyContextSnapshot,
    /// Policy state after the change.
    pub policy_state_after: PolicyContextSnapshot,
}

/// Preview record for one policy bundle or settings-lock change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Input object that describes the proposed change.
    pub request: PolicySimulationRequest,
    /// Affected surfaces and commands.
    pub affected_surfaces: Vec<AffectedPolicySurface>,
    /// Exception or waiver refs overlapping the preview.
    pub exception_preview_refs: Vec<String>,
    /// Remembered-decision refs overlapping the preview.
    pub remembered_decision_preview_refs: Vec<String>,
    /// Audit-export fields that must be preserved if the change lands.
    pub audit_export_required_fields: Vec<String>,
}

/// Exception or waiver record with owner, scope, evidence, expiry, and paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExceptionalAuthorityRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable exception or waiver id.
    pub exception_id: String,
    /// Exception kind.
    pub exception_kind: ExceptionKindClass,
    /// Stable token for [`Self::exception_kind`].
    pub exception_kind_token: String,
    /// Owner responsible for renewal or closure.
    pub owner: ActorRef,
    /// Scope where the exception applies.
    pub scope: ScopeRef,
    /// Evidence refs that justify or audit the exception.
    pub evidence_trail_refs: Vec<String>,
    /// Bounded time horizon.
    pub time_horizon: TimeHorizon,
    /// Renewal path after expiry.
    pub renewal_path: RenewalPathClass,
    /// Stable token for [`Self::renewal_path`].
    pub renewal_path_token: String,
    /// Revocation path before expiry.
    pub revocation_path: RevocationPathClass,
    /// Stable token for [`Self::revocation_path`].
    pub revocation_path_token: String,
    /// Current lifecycle status.
    pub status: ExceptionalAuthorityStatusClass,
    /// Stable token for [`Self::status`].
    pub status_token: String,
    /// Dashboard bucket derived from status and expiry.
    pub dashboard_bucket: DashboardBucketClass,
    /// Stable token for [`Self::dashboard_bucket`].
    pub dashboard_bucket_token: String,
    /// Audit event refs for mint, use, renewal, revoke, or expiry.
    pub audit_lineage_refs: Vec<String>,
    /// Simulation refs that previewed this exception.
    pub linked_simulation_refs: Vec<String>,
    /// True when raw justification text is excluded.
    pub raw_justification_excluded: bool,
}

/// Remembered decision narrowed to actor, object, action, environment, and time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedDecisionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable remembered-decision id.
    pub remembered_decision_id: String,
    /// Actor binding.
    pub bound_actor: ActorRef,
    /// Object binding.
    pub object: SubjectRef,
    /// Action-family binding.
    pub action_family: ActionFamilyClass,
    /// Stable token for [`Self::action_family`].
    pub action_family_token: String,
    /// Environment binding.
    pub environment: EnvironmentBinding,
    /// Policy epoch this memory is bound to.
    pub policy_epoch: String,
    /// Target version this memory is bound to.
    pub target_version_ref: String,
    /// Authority epoch this memory is bound to.
    pub authority_epoch_ref: String,
    /// Time horizon after which reapproval applies.
    pub time_horizon: TimeHorizon,
    /// Current memory state.
    pub memory_state: MemoryStateClass,
    /// Stable token for [`Self::memory_state`].
    pub memory_state_token: String,
    /// Optional narrowed carry-forward scope.
    pub carry_forward_scope: Option<ScopeRef>,
    /// Drift reasons detected during revalidation.
    pub invalidation_reasons: Vec<RememberedDecisionDriftReason>,
    /// Stable tokens for [`Self::invalidation_reasons`].
    pub invalidation_reason_tokens: Vec<String>,
    /// Dashboard bucket derived from state and drift.
    pub dashboard_bucket: DashboardBucketClass,
    /// Stable token for [`Self::dashboard_bucket`].
    pub dashboard_bucket_token: String,
    /// Prompt or escalation path after expiry.
    pub reapproval_prompt: String,
    /// Related exception or waiver, if the memory uses one.
    pub related_exception_ref: Option<String>,
    /// Audit refs for mint, replay, expiry, and reprompt.
    pub audit_lineage_refs: Vec<String>,
}

/// Current context used to revalidate a remembered decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedDecisionDriftSnapshot {
    /// Actor attempting to reuse the decision.
    pub actor: ActorRef,
    /// Object being acted upon.
    pub object: SubjectRef,
    /// Action family being attempted.
    pub action_family: ActionFamilyClass,
    /// Current environment.
    pub environment: EnvironmentBinding,
    /// Current policy epoch.
    pub policy_epoch: String,
    /// Current target version.
    pub target_version_ref: String,
    /// Current authority epoch.
    pub authority_epoch_ref: String,
    /// UTC instant when revalidation happens.
    pub evaluated_at: String,
}

/// Policy state preserved for a support or admin packet at action time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyStateAtActionTime {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable action id.
    pub action_id: String,
    /// Action family captured by this record.
    pub action_family: ActionFamilyClass,
    /// Stable token for [`Self::action_family`].
    pub action_family_token: String,
    /// Actor who performed the action.
    pub actor: ActorRef,
    /// Target object.
    pub object: SubjectRef,
    /// Policy context that actually applied at action time.
    pub policy_context_at_action_time: PolicyContextSnapshot,
    /// Current policy context when the export was assembled.
    pub policy_context_at_export_time: PolicyContextSnapshot,
    /// Decision refs used at action time.
    pub decision_refs: Vec<String>,
    /// Exception refs used at action time.
    pub exception_refs: Vec<String>,
    /// Source chain refs used to reconstruct the decision.
    pub source_chain_refs: Vec<String>,
    /// Audit event refs for issue, use, denial, revoke, or expiry.
    pub audit_event_refs: Vec<String>,
    /// True when historical truth is preserved rather than current-only truth.
    pub preserves_historical_truth: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Aggregate summary for a policy simulation beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of simulations.
    pub simulation_count: usize,
    /// Number of affected surfaces.
    pub affected_surface_count: usize,
    /// Number of exception or waiver records.
    pub exception_count: usize,
    /// Number of remembered-decision records.
    pub remembered_decision_count: usize,
    /// Number of action-time policy snapshots.
    pub action_time_policy_state_count: usize,
    /// Change-class tokens present.
    pub change_classes_present: Vec<String>,
    /// Persona tokens present.
    pub affected_personas_present: Vec<String>,
    /// Action-family tokens present.
    pub action_families_present: Vec<String>,
    /// Degraded-mode tokens present.
    pub degraded_modes_present: Vec<String>,
    /// Protected-path change tokens present.
    pub protected_path_changes_present: Vec<String>,
    /// Number of exceptions expiring soon.
    pub expiring_exception_count: usize,
    /// Number of remembered decisions that require reapproval or expiry handling.
    pub remembered_decisions_requiring_reapproval_count: usize,
    /// Number of rows with drift detected.
    pub drift_detected_count: usize,
    /// Number of validation defects.
    pub defect_count: usize,
    /// Defect counts by kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl PolicySimulationSummary {
    /// Builds a summary from simulations, exceptions, memory, snapshots, and defects.
    pub fn from_records(
        simulations: &[PolicySimulationRecord],
        exceptions: &[ExceptionalAuthorityRecord],
        remembered_decisions: &[RememberedDecisionRecord],
        action_time_policy_states: &[PolicyStateAtActionTime],
        defects: &[PolicySimulationBetaDefect],
    ) -> Self {
        let mut affected_surface_count = 0;
        let mut personas = BTreeSet::new();
        let mut action_families = BTreeSet::new();
        let mut degraded_modes = BTreeSet::new();
        let mut protected_paths = BTreeSet::new();
        let change_classes_present: BTreeSet<String> = simulations
            .iter()
            .map(|simulation| simulation.request.change_class_token.clone())
            .collect();

        for surface in simulations
            .iter()
            .flat_map(|simulation| simulation.affected_surfaces.iter())
        {
            affected_surface_count += 1;
            personas.insert(surface.affected_persona_token.clone());
            action_families.insert(surface.action_family_token.clone());
            degraded_modes.insert(surface.degraded_mode_token.clone());
            protected_paths.insert(surface.protected_path_change_token.clone());
        }

        let expiring_exception_count = exceptions
            .iter()
            .filter(|record| {
                record.dashboard_bucket == DashboardBucketClass::ExpiringSoon
                    || record.status == ExceptionalAuthorityStatusClass::ExpiringSoon
            })
            .count();
        let remembered_decisions_requiring_reapproval_count = remembered_decisions
            .iter()
            .filter(|record| {
                matches!(
                    record.memory_state,
                    MemoryStateClass::Expired
                        | MemoryStateClass::RequiresReapproval
                        | MemoryStateClass::ForceRetiredByPolicy
                )
            })
            .count();
        let drift_detected_count = remembered_decisions
            .iter()
            .filter(|record| !record.invalidation_reasons.is_empty())
            .count();

        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: POLICY_SIMULATION_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: POLICY_SIMULATION_SUMMARY_RECORD_KIND.to_owned(),
            simulation_count: simulations.len(),
            affected_surface_count,
            exception_count: exceptions.len(),
            remembered_decision_count: remembered_decisions.len(),
            action_time_policy_state_count: action_time_policy_states.len(),
            change_classes_present: change_classes_present.into_iter().collect(),
            affected_personas_present: personas.into_iter().collect(),
            action_families_present: action_families.into_iter().collect(),
            degraded_modes_present: degraded_modes.into_iter().collect(),
            protected_path_changes_present: protected_paths.into_iter().collect(),
            expiring_exception_count,
            remembered_decisions_requiring_reapproval_count,
            drift_detected_count,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by shell, support, admin, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source verification packet or contract ref.
    pub verification_packet_ref: String,
    /// Pre-apply simulations.
    pub simulations: Vec<PolicySimulationRecord>,
    /// Exception and waiver state records.
    pub exceptions: Vec<ExceptionalAuthorityRecord>,
    /// Remembered-decision records.
    pub remembered_decisions: Vec<RememberedDecisionRecord>,
    /// Historical policy snapshots for support and admin exports.
    pub action_time_policy_states: Vec<PolicyStateAtActionTime>,
    /// Typed validation defects.
    pub defects: Vec<PolicySimulationBetaDefect>,
    /// Aggregate summary.
    pub summary: PolicySimulationSummary,
}

/// Support-export wrapper for policy simulation beta records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub exported_at: String,
    /// Exported policy page.
    pub page: PolicySimulationBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// Action-time policy snapshots exported.
    pub action_time_policy_states: Vec<PolicyStateAtActionTime>,
    /// True when historical action-time policy truth is present.
    pub preserves_historical_truth: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl PolicySimulationSupportExport {
    /// Builds a metadata-safe support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: PolicySimulationBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        let action_time_policy_states = page.action_time_policy_states.clone();
        let preserves_historical_truth = !action_time_policy_states.is_empty()
            && action_time_policy_states
                .iter()
                .all(|snapshot| snapshot.preserves_historical_truth);

        Self {
            record_kind: POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            action_time_policy_states,
            preserves_historical_truth,
            raw_private_material_excluded: true,
        }
    }
}

/// Typed validation defect for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: PolicySimulationBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id.
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl PolicySimulationBetaDefect {
    fn new(
        defect_kind: PolicySimulationBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: POLICY_SIMULATION_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Revalidates remembered memory against the current actor, target, policy, and authority.
pub fn revalidate_remembered_decision(
    remembered: &RememberedDecisionRecord,
    snapshot: &RememberedDecisionDriftSnapshot,
) -> RememberedDecisionRecord {
    let mut invalidation_reasons = Vec::new();

    if remembered.bound_actor != snapshot.actor {
        invalidation_reasons.push(RememberedDecisionDriftReason::ActorChanged);
    }
    if remembered.object != snapshot.object {
        invalidation_reasons.push(RememberedDecisionDriftReason::ObjectChanged);
    }
    if remembered.action_family != snapshot.action_family {
        invalidation_reasons.push(RememberedDecisionDriftReason::ActionFamilyChanged);
    }
    if remembered.environment != snapshot.environment {
        invalidation_reasons.push(RememberedDecisionDriftReason::EnvironmentChanged);
    }
    if remembered.policy_epoch != snapshot.policy_epoch {
        invalidation_reasons.push(RememberedDecisionDriftReason::PolicyEpochChanged);
    }
    if remembered.target_version_ref != snapshot.target_version_ref {
        invalidation_reasons.push(RememberedDecisionDriftReason::TargetVersionChanged);
    }
    if remembered.authority_epoch_ref != snapshot.authority_epoch_ref {
        invalidation_reasons.push(RememberedDecisionDriftReason::AuthorityDrift);
    }
    if remembered.time_horizon.expires_at.as_str() <= snapshot.evaluated_at.as_str() {
        invalidation_reasons.push(RememberedDecisionDriftReason::ExpiryElapsed);
    }

    let mut result = remembered.clone();
    result.invalidation_reasons = invalidation_reasons;
    result.invalidation_reason_tokens = result
        .invalidation_reasons
        .iter()
        .map(|reason| reason.as_str().to_owned())
        .collect();

    if result
        .invalidation_reasons
        .contains(&RememberedDecisionDriftReason::ExpiryElapsed)
    {
        result.memory_state = MemoryStateClass::Expired;
        result.dashboard_bucket = DashboardBucketClass::ExpiredRepromptDue;
    } else if result.invalidation_reasons.iter().any(|reason| {
        matches!(
            reason,
            RememberedDecisionDriftReason::PolicyEpochChanged
                | RememberedDecisionDriftReason::AuthorityDrift
        )
    }) {
        result.memory_state = MemoryStateClass::ForceRetiredByPolicy;
        result.dashboard_bucket = DashboardBucketClass::ForceRetiredByPolicy;
    } else if result.invalidation_reasons.is_empty() {
        result.memory_state = MemoryStateClass::Active;
        result.dashboard_bucket = DashboardBucketClass::Healthy;
    } else {
        result.memory_state = MemoryStateClass::RequiresReapproval;
        result.dashboard_bucket = DashboardBucketClass::DriftDetected;
    }

    result.memory_state_token = result.memory_state.as_str().to_owned();
    result.dashboard_bucket_token = result.dashboard_bucket.as_str().to_owned();
    result
}

/// Builds a policy simulation record from a request and audited projections.
pub fn simulate_policy_change(
    request: PolicySimulationRequest,
    affected_surfaces: Vec<AffectedPolicySurface>,
    exception_preview_refs: Vec<String>,
    remembered_decision_preview_refs: Vec<String>,
) -> PolicySimulationRecord {
    PolicySimulationRecord {
        record_kind: POLICY_SIMULATION_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        request,
        affected_surfaces,
        exception_preview_refs,
        remembered_decision_preview_refs,
        audit_export_required_fields: vec![
            "policy_epoch_at_action_time".to_owned(),
            "policy_bundle_ref_at_action_time".to_owned(),
            "actor_ref".to_owned(),
            "object_ref".to_owned(),
            "action_family".to_owned(),
            "exception_refs".to_owned(),
            "remembered_decision_refs".to_owned(),
        ],
    }
}

/// Builds the seeded beta page covering policy changes, settings locks, exceptions, memory, and support truth.
pub fn seeded_policy_simulation_beta_page() -> PolicySimulationBetaPage {
    let base_policy = policy_context(
        "policy_epoch.enterprise.beta.2026_05_01",
        "policy-bundle:enterprise-beta:2026.05.0",
        "enterprise_policy@2026.05.0",
        "signed_local_admin_bundle",
        "2026-05-01T00:00:00Z",
    );
    let proposed_policy = policy_context(
        "policy_epoch.enterprise.beta.2026_05_17",
        "policy-bundle:enterprise-beta:2026.05.1",
        "enterprise_policy@2026.05.1",
        "signed_local_admin_bundle",
        "2026-05-17T18:00:00Z",
    );
    let settings_policy = policy_context(
        "policy_epoch.enterprise.beta.2026_05_17.settings",
        "policy-bundle:enterprise-settings:2026.05.1",
        "enterprise_settings@2026.05.1",
        "signed_local_admin_bundle",
        "2026-05-17T18:10:00Z",
    );

    let workspace_scope = ScopeRef::new(ScopeKind::Workspace, "workspace:aureline-beta");
    let root_scope = ScopeRef::new(ScopeKind::Root, "root:provider-alpha");
    let admin_actor = ActorRef {
        stable_id: "actor:policy-admin".to_owned(),
        role: "organization_admin".to_owned(),
    };

    let exceptions = vec![
        exception_record(
            "exception:connected-provider:reapproval-window",
            ExceptionKindClass::PolicyException,
            admin_actor.clone(),
            workspace_scope.clone(),
            "2026-05-17T17:00:00Z",
            "2026-05-24T17:00:00Z",
            "2026-05-22T17:00:00Z",
            ExceptionalAuthorityStatusClass::ExpiringSoon,
            DashboardBucketClass::ExpiringSoon,
            vec![
                "audit:exception:minted:connected-provider".to_owned(),
                "evidence:policy-simulation:connected-provider-deny".to_owned(),
            ],
        ),
        exception_record(
            "waiver:settings-lock:ai-provider",
            ExceptionKindClass::AdminReconfirmationDefer,
            admin_actor.clone(),
            workspace_scope.clone(),
            "2026-05-15T12:00:00Z",
            "2026-05-20T12:00:00Z",
            "2026-05-18T12:00:00Z",
            ExceptionalAuthorityStatusClass::Active,
            DashboardBucketClass::Healthy,
            vec![
                "audit:waiver:minted:ai-provider-settings-lock".to_owned(),
                "evidence:settings-lock:ai-provider-mirror-only".to_owned(),
            ],
        ),
    ];

    let remembered_allow = remembered_decision_record(
        "memory:connected-provider:workspace-allow",
        admin_actor.clone(),
        SubjectRef {
            subject_kind: "connected_provider_record".to_owned(),
            subject_id: "provider:alpha".to_owned(),
        },
        ActionFamilyClass::ConnectedProviderMutation,
        environment("env:desktop:managed-beta", "workspace:aureline-beta"),
        "policy_epoch.enterprise.beta.2026_05_01".to_owned(),
        "provider-alpha@1.6.0".to_owned(),
        "authority_epoch.shell.2026_05_01".to_owned(),
        TimeHorizon {
            declared_at: "2026-05-01T10:00:00Z".to_owned(),
            expires_at: "2026-05-21T10:00:00Z".to_owned(),
            reapproval_prompt_at: "2026-05-19T10:00:00Z".to_owned(),
        },
        MemoryStateClass::Active,
        None,
        Vec::new(),
        DashboardBucketClass::Healthy,
        Some("exception:connected-provider:reapproval-window".to_owned()),
    );

    let drifted_memory = revalidate_remembered_decision(
        &remembered_allow,
        &RememberedDecisionDriftSnapshot {
            actor: admin_actor.clone(),
            object: SubjectRef {
                subject_kind: "connected_provider_record".to_owned(),
                subject_id: "provider:alpha".to_owned(),
            },
            action_family: ActionFamilyClass::ConnectedProviderMutation,
            environment: environment("env:desktop:managed-beta", "workspace:aureline-beta"),
            policy_epoch: proposed_policy.policy_epoch.clone(),
            target_version_ref: "provider-alpha@1.7.0".to_owned(),
            authority_epoch_ref: "authority_epoch.shell.2026_05_17".to_owned(),
            evaluated_at: "2026-05-17T18:05:00Z".to_owned(),
        },
    );

    let narrowed_memory = remembered_decision_record(
        "memory:connected-provider:root-carry-forward",
        admin_actor.clone(),
        SubjectRef {
            subject_kind: "connected_provider_record".to_owned(),
            subject_id: "provider:alpha-root".to_owned(),
        },
        ActionFamilyClass::ConnectedProviderMutation,
        environment("env:desktop:managed-beta", "workspace:aureline-beta"),
        proposed_policy.policy_epoch.clone(),
        "provider-alpha@1.6.0".to_owned(),
        "authority_epoch.shell.2026_05_17".to_owned(),
        TimeHorizon {
            declared_at: "2026-05-17T18:10:00Z".to_owned(),
            expires_at: "2026-05-18T18:10:00Z".to_owned(),
            reapproval_prompt_at: "2026-05-18T12:10:00Z".to_owned(),
        },
        MemoryStateClass::Narrowed,
        Some(root_scope.clone()),
        Vec::new(),
        DashboardBucketClass::NarrowedCarryForward,
        Some("exception:connected-provider:reapproval-window".to_owned()),
    );

    let simulations = vec![
        simulate_policy_change(
            PolicySimulationRequest {
                simulation_id: "simulation:policy-bundle:connected-provider-deny".to_owned(),
                change_class: PolicyChangeClass::PolicyBundleChange,
                change_class_token: PolicyChangeClass::PolicyBundleChange.as_str().to_owned(),
                baseline_ref: base_policy.bundle_ref.clone(),
                proposed_ref: proposed_policy.bundle_ref.clone(),
                scope: workspace_scope.clone(),
                requested_by: admin_actor.clone(),
                evaluated_at: "2026-05-17T17:45:00Z".to_owned(),
                effective_from: "2026-05-17T18:00:00Z".to_owned(),
            },
            vec![
                affected_surface(
                    "affected:provider-publish",
                    "surface:connected-provider:publish",
                    ActorPersonaClass::EndUser,
                    ActionFamilyClass::ConnectedProviderMutation,
                    vec!["provider.publish".to_owned(), "provider.sync.apply".to_owned()],
                    DegradedModeClass::BlockedByPolicy,
                    ProtectedPathChangeClass::WritesDenied,
                    "Provider mutations would be denied until a fresh approval is minted under the new policy epoch.",
                    base_policy.clone(),
                    proposed_policy.clone(),
                ),
                affected_surface(
                    "affected:support-export",
                    "surface:support:policy-handoff",
                    ActorPersonaClass::SupportOperator,
                    ActionFamilyClass::SupportExport,
                    vec!["support.export.policy_state".to_owned()],
                    DegradedModeClass::ReadOnly,
                    ProtectedPathChangeClass::EvidenceExportNarrowed,
                    "Support exports would include the old and new policy epochs without raw bundle bodies.",
                    base_policy.clone(),
                    proposed_policy.clone(),
                ),
            ],
            vec!["exception:connected-provider:reapproval-window".to_owned()],
            vec![drifted_memory.remembered_decision_id.clone()],
        ),
        simulate_policy_change(
            PolicySimulationRequest {
                simulation_id: "simulation:settings-lock:ai-provider-mirror-only".to_owned(),
                change_class: PolicyChangeClass::SettingsLockChange,
                change_class_token: PolicyChangeClass::SettingsLockChange.as_str().to_owned(),
                baseline_ref: "settings:ai-provider-route:user-selectable".to_owned(),
                proposed_ref: "settings:ai-provider-route:locked-mirror-only".to_owned(),
                scope: workspace_scope.clone(),
                requested_by: admin_actor.clone(),
                evaluated_at: "2026-05-17T18:02:00Z".to_owned(),
                effective_from: "2026-05-17T18:10:00Z".to_owned(),
            },
            vec![affected_surface(
                "affected:ai-apply-settings-lock",
                "surface:ai:apply",
                ActorPersonaClass::WorkspaceAdmin,
                ActionFamilyClass::SettingsWrite,
                vec!["settings.ai_provider.write".to_owned(), "ai.apply".to_owned()],
                DegradedModeClass::PreviewOnly,
                ProtectedPathChangeClass::ManagedSettingLocked,
                "AI apply remains previewable, but provider-route changes require admin review.",
                proposed_policy.clone(),
                settings_policy.clone(),
            )],
            vec!["waiver:settings-lock:ai-provider".to_owned()],
            vec![narrowed_memory.remembered_decision_id.clone()],
        ),
    ];

    let remembered_decisions = vec![drifted_memory, narrowed_memory];
    let action_time_policy_states = vec![PolicyStateAtActionTime {
        record_kind: POLICY_SIMULATION_STATE_AT_ACTION_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        action_id: "action:provider-publish:2026-05-16".to_owned(),
        action_family: ActionFamilyClass::ConnectedProviderMutation,
        action_family_token: ActionFamilyClass::ConnectedProviderMutation
            .as_str()
            .to_owned(),
        actor: admin_actor,
        object: SubjectRef {
            subject_kind: "connected_provider_record".to_owned(),
            subject_id: "provider:alpha".to_owned(),
        },
        policy_context_at_action_time: base_policy,
        policy_context_at_export_time: proposed_policy,
        decision_refs: vec!["memory:connected-provider:workspace-allow".to_owned()],
        exception_refs: vec!["exception:connected-provider:reapproval-window".to_owned()],
        source_chain_refs: vec![
            "embedded_product_defaults".to_owned(),
            "signed_local_admin_bundle".to_owned(),
            "user_workspace_configuration".to_owned(),
        ],
        audit_event_refs: vec![
            "audit:authority-ticket:issued:provider-publish".to_owned(),
            "audit:policy-decision:provider-publish".to_owned(),
        ],
        preserves_historical_truth: true,
        raw_private_material_excluded: true,
    }];

    let mut page = PolicySimulationBetaPage {
        record_kind: POLICY_SIMULATION_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        verification_packet_ref: "docs/verification/policy_simulation_packet.md".to_owned(),
        simulations,
        exceptions,
        remembered_decisions,
        action_time_policy_states,
        defects: Vec::new(),
        summary: PolicySimulationSummary::from_records(&[], &[], &[], &[], &[]),
    };
    page.defects = audit_policy_simulation_beta_page(&page);
    page.summary = PolicySimulationSummary::from_records(
        &page.simulations,
        &page.exceptions,
        &page.remembered_decisions,
        &page.action_time_policy_states,
        &page.defects,
    );
    page
}

/// Validates a policy simulation page and returns typed defects on failure.
pub fn validate_policy_simulation_beta_page(
    page: &PolicySimulationBetaPage,
) -> Result<(), Vec<PolicySimulationBetaDefect>> {
    let defects = audit_policy_simulation_beta_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes validation defects for a policy simulation beta page.
pub fn audit_policy_simulation_beta_page(
    page: &PolicySimulationBetaPage,
) -> Vec<PolicySimulationBetaDefect> {
    let mut defects = Vec::new();

    let observed_change_classes: BTreeSet<PolicyChangeClass> = page
        .simulations
        .iter()
        .map(|simulation| simulation.request.change_class)
        .collect();
    for required in [
        PolicyChangeClass::PolicyBundleChange,
        PolicyChangeClass::SettingsLockChange,
    ] {
        if !observed_change_classes.contains(&required) {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ChangeClassCoverageMissing,
                "page",
                "simulations.request.change_class",
                format!("missing {} simulation coverage", required.as_str()),
            ));
        }
    }

    for simulation in &page.simulations {
        if simulation.request.change_class_token != simulation.request.change_class.as_str() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ChangeClassCoverageMissing,
                simulation.request.simulation_id.clone(),
                "request.change_class_token",
                "change_class_token must match change_class",
            ));
        }
        if simulation.affected_surfaces.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::SimulationWithoutAffectedSurfaces,
                simulation.request.simulation_id.clone(),
                "affected_surfaces",
                "simulation must list affected personas, actions, degraded modes, and protected paths",
            ));
        }
        for surface in &simulation.affected_surfaces {
            if surface.affected_persona_token != surface.affected_persona.as_str() {
                defects.push(PolicySimulationBetaDefect::new(
                    PolicySimulationBetaDefectKind::MissingAffectedPersona,
                    surface.affected_surface_id.clone(),
                    "affected_persona_token",
                    "affected_persona_token must match affected_persona",
                ));
            }
            if surface.action_family_token != surface.action_family.as_str()
                || surface.command_ids.is_empty()
            {
                defects.push(PolicySimulationBetaDefect::new(
                    PolicySimulationBetaDefectKind::MissingAffectedAction,
                    surface.affected_surface_id.clone(),
                    "action_family_token",
                    "affected surface must name action family and command ids",
                ));
            }
            if surface.degraded_mode_token != surface.degraded_mode.as_str() {
                defects.push(PolicySimulationBetaDefect::new(
                    PolicySimulationBetaDefectKind::MissingDegradedMode,
                    surface.affected_surface_id.clone(),
                    "degraded_mode_token",
                    "degraded_mode_token must match degraded_mode",
                ));
            }
            if surface.protected_path_change_token != surface.protected_path_change.as_str() {
                defects.push(PolicySimulationBetaDefect::new(
                    PolicySimulationBetaDefectKind::MissingProtectedPathChange,
                    surface.affected_surface_id.clone(),
                    "protected_path_change_token",
                    "protected_path_change_token must match protected_path_change",
                ));
            }
        }
    }

    for exception in &page.exceptions {
        if exception.owner.stable_id.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingOwner,
                exception.exception_id.clone(),
                "owner.stable_id",
                "exception or waiver must expose an owner",
            ));
        }
        if exception.scope.scope_id.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingScope,
                exception.exception_id.clone(),
                "scope.scope_id",
                "exception or waiver must expose scope",
            ));
        }
        if exception.time_horizon.expires_at.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingExpiry,
                exception.exception_id.clone(),
                "time_horizon.expires_at",
                "exception or waiver must include an explicit expiry",
            ));
        }
        if exception.evidence_trail_refs.is_empty() || exception.audit_lineage_refs.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingEvidenceTrail,
                exception.exception_id.clone(),
                "evidence_trail_refs",
                "exception or waiver must expose evidence and audit lineage",
            ));
        }
        if exception.renewal_path_token != exception.renewal_path.as_str() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingRenewalPath,
                exception.exception_id.clone(),
                "renewal_path_token",
                "renewal_path_token must match renewal_path",
            ));
        }
        if exception.revocation_path_token != exception.revocation_path.as_str() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::ExceptionMissingRevocationPath,
                exception.exception_id.clone(),
                "revocation_path_token",
                "revocation_path_token must match revocation_path",
            ));
        }
        if !exception.raw_justification_excluded {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RawPrivateMaterialExposed,
                exception.exception_id.clone(),
                "raw_justification_excluded",
                "exception records must not expose raw private justification text",
            ));
        }
    }

    for memory in &page.remembered_decisions {
        if memory.bound_actor.stable_id.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionMissingActorBinding,
                memory.remembered_decision_id.clone(),
                "bound_actor.stable_id",
                "remembered decisions must be actor-bound",
            ));
        }
        if memory.object.subject_kind.is_empty() || memory.object.subject_id.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionMissingObjectBinding,
                memory.remembered_decision_id.clone(),
                "object",
                "remembered decisions must be object-bound",
            ));
        }
        if memory.environment.environment_id.is_empty()
            || memory.environment.workspace_ref.is_empty()
            || memory.environment.profile_token.is_empty()
        {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionMissingEnvironmentBinding,
                memory.remembered_decision_id.clone(),
                "environment",
                "remembered decisions must be environment-bound",
            ));
        }
        if memory.time_horizon.expires_at.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionMissingTimeHorizon,
                memory.remembered_decision_id.clone(),
                "time_horizon.expires_at",
                "remembered decisions must expose an expiry horizon",
            ));
        }
        if memory.memory_state_token != memory.memory_state.as_str() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionDriftUnexplained,
                memory.remembered_decision_id.clone(),
                "memory_state_token",
                "memory_state_token must match memory_state",
            ));
        }
        let needs_drift_reason = matches!(
            memory.memory_state,
            MemoryStateClass::Expired
                | MemoryStateClass::RequiresReapproval
                | MemoryStateClass::ForceRetiredByPolicy
        );
        if needs_drift_reason && memory.invalidation_reasons.is_empty() {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RememberedDecisionDriftUnexplained,
                memory.remembered_decision_id.clone(),
                "invalidation_reasons",
                "drifted or expired remembered decisions must name invalidation reasons",
            ));
        }
        if let Some(carry_forward_scope) = &memory.carry_forward_scope {
            let original_scope = ScopeRef::new(ScopeKind::Workspace, "workspace:aureline-beta");
            if !carry_forward_scope.is_equal_or_narrower_than(&original_scope) {
                defects.push(PolicySimulationBetaDefect::new(
                    PolicySimulationBetaDefectKind::RememberedDecisionCarryForwardTooBroad,
                    memory.remembered_decision_id.clone(),
                    "carry_forward_scope",
                    "remembered decision carry-forward must be equal to or narrower than the remembered result",
                ));
            }
        }
    }

    if page.action_time_policy_states.is_empty() {
        defects.push(PolicySimulationBetaDefect::new(
            PolicySimulationBetaDefectKind::SupportExportMissingActionTimePolicyState,
            "page",
            "action_time_policy_states",
            "support/admin packets must preserve policy state that applied at action time",
        ));
    }
    for snapshot in &page.action_time_policy_states {
        if !snapshot.preserves_historical_truth {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::SupportExportHistoricalTruthDropped,
                snapshot.action_id.clone(),
                "preserves_historical_truth",
                "support export must preserve historical policy truth, not only current truth",
            ));
        }
        if !snapshot.raw_private_material_excluded {
            defects.push(PolicySimulationBetaDefect::new(
                PolicySimulationBetaDefectKind::RawPrivateMaterialExposed,
                snapshot.action_id.clone(),
                "raw_private_material_excluded",
                "action-time policy snapshots must be metadata-safe",
            ));
        }
    }

    defects
}

fn policy_context(
    policy_epoch: &str,
    bundle_ref: &str,
    bundle_version: &str,
    source_class: &str,
    effective_at: &str,
) -> PolicyContextSnapshot {
    PolicyContextSnapshot {
        policy_epoch: policy_epoch.to_owned(),
        bundle_ref: bundle_ref.to_owned(),
        bundle_version: bundle_version.to_owned(),
        source_class: source_class.to_owned(),
        effective_at: effective_at.to_owned(),
    }
}

fn environment(environment_id: &str, workspace_ref: &str) -> EnvironmentBinding {
    EnvironmentBinding {
        environment_id: environment_id.to_owned(),
        workspace_ref: workspace_ref.to_owned(),
        profile_token: "enterprise_managed".to_owned(),
        trust_state: "trusted_managed_workspace".to_owned(),
        network_posture: "mirror_or_managed_only".to_owned(),
    }
}

fn affected_surface(
    affected_surface_id: &str,
    surface_id: &str,
    affected_persona: ActorPersonaClass,
    action_family: ActionFamilyClass,
    command_ids: Vec<String>,
    degraded_mode: DegradedModeClass,
    protected_path_change: ProtectedPathChangeClass,
    consequence: &str,
    policy_state_before: PolicyContextSnapshot,
    policy_state_after: PolicyContextSnapshot,
) -> AffectedPolicySurface {
    AffectedPolicySurface {
        record_kind: POLICY_SIMULATION_AFFECTED_SURFACE_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        affected_surface_id: affected_surface_id.to_owned(),
        surface_ref: SubjectRef {
            subject_kind: "product_surface".to_owned(),
            subject_id: surface_id.to_owned(),
        },
        affected_persona,
        affected_persona_token: affected_persona.as_str().to_owned(),
        action_family,
        action_family_token: action_family.as_str().to_owned(),
        command_ids,
        degraded_mode,
        degraded_mode_token: degraded_mode.as_str().to_owned(),
        protected_path_change,
        protected_path_change_token: protected_path_change.as_str().to_owned(),
        consequence: consequence.to_owned(),
        policy_state_before,
        policy_state_after,
    }
}

fn exception_record(
    exception_id: &str,
    exception_kind: ExceptionKindClass,
    owner: ActorRef,
    scope: ScopeRef,
    declared_at: &str,
    expires_at: &str,
    reapproval_prompt_at: &str,
    status: ExceptionalAuthorityStatusClass,
    dashboard_bucket: DashboardBucketClass,
    evidence_trail_refs: Vec<String>,
) -> ExceptionalAuthorityRecord {
    ExceptionalAuthorityRecord {
        record_kind: POLICY_SIMULATION_EXCEPTION_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        exception_id: exception_id.to_owned(),
        exception_kind,
        exception_kind_token: exception_kind.as_str().to_owned(),
        owner,
        scope,
        evidence_trail_refs: evidence_trail_refs.clone(),
        time_horizon: TimeHorizon {
            declared_at: declared_at.to_owned(),
            expires_at: expires_at.to_owned(),
            reapproval_prompt_at: reapproval_prompt_at.to_owned(),
        },
        renewal_path: RenewalPathClass::RequireAdminReconfirmation,
        renewal_path_token: RenewalPathClass::RequireAdminReconfirmation
            .as_str()
            .to_owned(),
        revocation_path: RevocationPathClass::PolicyEpochChange,
        revocation_path_token: RevocationPathClass::PolicyEpochChange.as_str().to_owned(),
        status,
        status_token: status.as_str().to_owned(),
        dashboard_bucket,
        dashboard_bucket_token: dashboard_bucket.as_str().to_owned(),
        audit_lineage_refs: evidence_trail_refs,
        linked_simulation_refs: Vec::new(),
        raw_justification_excluded: true,
    }
}

fn remembered_decision_record(
    remembered_decision_id: &str,
    bound_actor: ActorRef,
    object: SubjectRef,
    action_family: ActionFamilyClass,
    environment: EnvironmentBinding,
    policy_epoch: String,
    target_version_ref: String,
    authority_epoch_ref: String,
    time_horizon: TimeHorizon,
    memory_state: MemoryStateClass,
    carry_forward_scope: Option<ScopeRef>,
    invalidation_reasons: Vec<RememberedDecisionDriftReason>,
    dashboard_bucket: DashboardBucketClass,
    related_exception_ref: Option<String>,
) -> RememberedDecisionRecord {
    RememberedDecisionRecord {
        record_kind: POLICY_SIMULATION_REMEMBERED_DECISION_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        remembered_decision_id: remembered_decision_id.to_owned(),
        bound_actor,
        object,
        action_family,
        action_family_token: action_family.as_str().to_owned(),
        environment,
        policy_epoch,
        target_version_ref,
        authority_epoch_ref,
        time_horizon,
        memory_state,
        memory_state_token: memory_state.as_str().to_owned(),
        carry_forward_scope,
        invalidation_reason_tokens: invalidation_reasons
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect(),
        invalidation_reasons,
        dashboard_bucket,
        dashboard_bucket_token: dashboard_bucket.as_str().to_owned(),
        reapproval_prompt: "revalidate with current actor, object, action, environment, policy epoch, target version, and authority epoch".to_owned(),
        related_exception_ref,
        audit_lineage_refs: vec![
            format!("audit:remembered-decision:minted:{remembered_decision_id}"),
            format!("audit:remembered-decision:replay:{remembered_decision_id}"),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_covers_required_change_classes() {
        let page = seeded_policy_simulation_beta_page();
        validate_policy_simulation_beta_page(&page).expect("seeded page validates");
        assert_eq!(page.summary.defect_count, 0);
        assert!(page
            .summary
            .change_classes_present
            .contains(&"policy_bundle_change".to_owned()));
        assert!(page
            .summary
            .change_classes_present
            .contains(&"settings_lock_change".to_owned()));
        assert!(page.summary.affected_surface_count >= 3);
    }

    #[test]
    fn exceptions_are_bounded_and_attributable() {
        let page = seeded_policy_simulation_beta_page();
        for exception in &page.exceptions {
            assert!(!exception.owner.stable_id.is_empty());
            assert!(!exception.scope.scope_id.is_empty());
            assert!(!exception.time_horizon.expires_at.is_empty());
            assert!(!exception.evidence_trail_refs.is_empty());
            assert_eq!(
                exception.renewal_path_token,
                exception.renewal_path.as_str()
            );
            assert_eq!(
                exception.revocation_path_token,
                exception.revocation_path.as_str()
            );
        }
    }

    #[test]
    fn remembered_decision_revalidates_on_policy_version_and_authority_drift() {
        let page = seeded_policy_simulation_beta_page();
        let drifted = page
            .remembered_decisions
            .iter()
            .find(|record| {
                record.remembered_decision_id == "memory:connected-provider:workspace-allow"
            })
            .expect("drifted memory");
        assert_eq!(drifted.memory_state, MemoryStateClass::ForceRetiredByPolicy);
        assert!(drifted
            .invalidation_reasons
            .contains(&RememberedDecisionDriftReason::PolicyEpochChanged));
        assert!(drifted
            .invalidation_reasons
            .contains(&RememberedDecisionDriftReason::AuthorityDrift));
        assert!(drifted
            .invalidation_reasons
            .contains(&RememberedDecisionDriftReason::TargetVersionChanged));
    }

    #[test]
    fn remembered_decision_carry_forward_is_narrower_than_workspace() {
        let page = seeded_policy_simulation_beta_page();
        let narrowed = page
            .remembered_decisions
            .iter()
            .find(|record| {
                record.remembered_decision_id == "memory:connected-provider:root-carry-forward"
            })
            .expect("narrowed memory");
        let carry_forward = narrowed
            .carry_forward_scope
            .as_ref()
            .expect("carry-forward scope");
        assert_eq!(narrowed.memory_state, MemoryStateClass::Narrowed);
        assert_eq!(carry_forward.scope_kind, ScopeKind::Root);
        assert_eq!(
            narrowed.dashboard_bucket,
            DashboardBucketClass::NarrowedCarryForward
        );
    }

    #[test]
    fn support_export_preserves_action_time_policy_truth() {
        let page = seeded_policy_simulation_beta_page();
        let export = PolicySimulationSupportExport::from_page(
            "support-export:policy-simulation:001",
            "2026-05-17T19:00:00Z",
            page,
        );
        assert!(export.preserves_historical_truth);
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        let snapshot = &export.action_time_policy_states[0];
        assert_ne!(
            snapshot.policy_context_at_action_time.policy_epoch,
            snapshot.policy_context_at_export_time.policy_epoch
        );
    }

    #[test]
    fn audit_flags_exception_without_expiry() {
        let mut page = seeded_policy_simulation_beta_page();
        page.exceptions[0].time_horizon.expires_at.clear();
        let defects = audit_policy_simulation_beta_page(&page);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == PolicySimulationBetaDefectKind::ExceptionMissingExpiry));
    }

    #[test]
    fn audit_flags_unexplained_drift() {
        let mut page = seeded_policy_simulation_beta_page();
        page.remembered_decisions[0].invalidation_reasons.clear();
        page.remembered_decisions[0]
            .invalidation_reason_tokens
            .clear();
        let defects = audit_policy_simulation_beta_page(&page);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == PolicySimulationBetaDefectKind::RememberedDecisionDriftUnexplained));
    }
}
