//! Repair-transaction preview compiler, outcome journal, and support packet.
//!
//! This module is the first runtime/support consumer for
//! `/fixtures/support/repair_cases/*.yaml`. It turns the checked-in seed
//! cases into typed transaction, preview, outcome, and journal records so
//! Project Doctor and Support Center flows can show blast radius,
//! checkpoint availability, reversal class, and diagnosis lineage before any
//! apply path is admitted.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for checked-in repair seed cases.
pub const REPAIR_SEED_CASE_RECORD_KIND: &str = "repair_seed_case_record";

/// Stable record-kind tag for live repair transaction records.
pub const REPAIR_TRANSACTION_RECORD_KIND: &str = "repair_transaction_record";

/// Stable record-kind tag for repair preview records.
pub const REPAIR_PREVIEW_RECORD_KIND: &str = "repair_preview_record";

/// Stable record-kind tag for repair outcome records.
pub const REPAIR_OUTCOME_RECORD_KIND: &str = "repair_outcome_record";

/// Stable record-kind tag for repair mutation-journal entries.
pub const REPAIR_MUTATION_JOURNAL_RECORD_KIND: &str = "repair_mutation_journal_entry";

/// Stable record-kind tag for repair support packets.
pub const REPAIR_SUPPORT_PACKET_RECORD_KIND: &str = "repair_support_packet";

/// Current schema version for repair transaction records.
pub const REPAIR_TRANSACTION_SCHEMA_VERSION: u32 = 1;

/// Current schema version for repair preview records.
pub const REPAIR_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Current schema version for repair outcome records.
pub const REPAIR_OUTCOME_SCHEMA_VERSION: u32 = 1;

const CACHE_INDEX_REPAIR_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/cache_index_repair.yaml"
));
const EXTENSION_QUARANTINE_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/extension_quarantine.yaml"
));
const TOOLCHAIN_RERESOLVE_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/toolchain_reresolve.yaml"
));
const REMOTE_AGENT_ROLLBACK_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/remote_agent_rollback.yaml"
));
const POLICY_REFRESH_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/policy_refresh.yaml"
));
const ESCALATION_ONLY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/repair_cases/escalation_only_packet.yaml"
));

const REQUIRED_FORBIDDEN_ACTIONS: [ForbiddenActionClass; 5] = [
    ForbiddenActionClass::WidenWorkspaceTrust,
    ForbiddenActionClass::PublishRoute,
    ForbiddenActionClass::RunRepoHookSilently,
    ForbiddenActionClass::MutateUserAuthoredFiles,
    ForbiddenActionClass::ReadOrRotateCredentials,
];

/// Loads a repair seed case from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`RepairSeedCase`].
pub fn load_repair_seed_case(yaml: &str) -> Result<RepairSeedCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the checked-in alpha repair seed corpus.
///
/// # Errors
///
/// Returns a YAML parse error when any checked-in seed case is malformed.
pub fn current_alpha_repair_seed_cases() -> Result<Vec<RepairSeedCase>, serde_yaml::Error> {
    [
        CACHE_INDEX_REPAIR_YAML,
        EXTENSION_QUARANTINE_YAML,
        TOOLCHAIN_RERESOLVE_YAML,
        REMOTE_AGENT_ROLLBACK_YAML,
        POLICY_REFRESH_YAML,
        ESCALATION_ONLY_YAML,
    ]
    .into_iter()
    .map(load_repair_seed_case)
    .collect()
}

/// Coarse repair-class family for one transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairClassFamily {
    /// Rebuilds a disposable cache, watcher backlog, or derived artifact.
    DisposableStateRebuild,
    /// Isolates or quarantines a suspect extension or host lane.
    ExtensionIsolation,
    /// Rolls back or reinstalls an extension under explicit review.
    ExtensionRollbackReinstall,
    /// Re-resolves an execution context, toolchain, or language server handle.
    ExecutionContextReresolve,
    /// Repairs, redeploys, or rolls back a remote helper or runtime.
    RemoteRuntimeRepair,
    /// Refreshes policy, entitlement, or approval state without widening trust.
    PolicyEntitlementRefresh,
    /// Refuses local apply and prepares an escalation packet instead.
    GuidedExportEscalation,
    /// Records an observe-only finding without a local repair.
    ObserveOnlyNoRepair,
}

/// Fine-grained repair suggestion shared with Project Doctor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedRepairClass {
    /// Reports the finding without a write path.
    ObserveOnlyNoRepair,
    /// Reacquires a trust approval without widening trust.
    ReacquireTrustApproval,
    /// Restarts or reseeds a watcher.
    RestartWatcherWithReseed,
    /// Resets or rebuilds ephemeral cache state.
    ResetEphemeralCache,
    /// Installs or repairs a required toolchain component.
    InstallOrRepairToolchain,
    /// Quarantines and optionally bisects a suspect extension.
    QuarantineAndBisectExtension,
    /// Reapproves a target or route.
    ReapproveTargetOrRoute,
    /// Reattaches a helper with new approval.
    ReattachHelperWithNewApproval,
    /// Refreshes docs or mirror packs.
    RefreshDocsOrMirrorPack,
    /// Defers to an escalation packet.
    DeferToEscalationPacket,
    /// Rolls back or reinstalls an extension.
    RollbackOrReinstallExtension,
    /// Rolls back a remote runtime.
    RollbackRemoteRuntime,
    /// Refreshes a policy entitlement.
    RefreshPolicyEntitlement,
}

/// Declared reversibility of a repair transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionReversalClass {
    /// Exact reversal restores the prior state bit-for-bit.
    Exact,
    /// Compensating reversal applies a semantically equivalent inverse.
    Compensating,
    /// Regenerate reversal rebuilds disposable derived state.
    Regenerate,
    /// Manual reversal requires user-guided follow-up.
    Manual,
    /// Audit-only transactions do not mutate repair target state.
    AuditOnly,
}

impl TransactionReversalClass {
    /// Returns true when apply needs a stronger confirmation than standard review.
    pub fn requires_strong_confirmation(self) -> bool {
        matches!(self, Self::Compensating | Self::Manual)
    }

    /// Returns a reviewer-facing explanation for this reversal class.
    pub fn summary(self) -> &'static str {
        match self {
            Self::Exact => "Reversal is exact and checkpoint-backed.",
            Self::Compensating => {
                "Reversal uses a compensating action, so review requires stronger confirmation."
            }
            Self::Regenerate => "Reversal regenerates disposable state from authoritative sources.",
            Self::Manual => {
                "Reversal requires a manual recovery path, so review requires stronger confirmation."
            }
            Self::AuditOnly => "No target state is mutated; the record is audit-only.",
        }
    }
}

/// Runtime apply mode for a repair transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyModeClass {
    /// Preview only; no write is admitted.
    DryRunPreviewOnly,
    /// Apply is admitted after a checkpoint is captured.
    ApplyWithCheckpoint,
    /// Apply is admitted and rolls back to a checkpoint on failure.
    ApplyWithRollbackOnFailure,
    /// Apply emits only observe/audit records.
    ApplyObserveOnlyNoWrite,
    /// Apply is refused and an escalation path is prepared.
    ApplyRefusedEscalationOnly,
}

impl ApplyModeClass {
    /// Returns true when this apply mode needs a non-null checkpoint.
    pub fn requires_checkpoint(self) -> bool {
        matches!(
            self,
            Self::ApplyWithCheckpoint | Self::ApplyWithRollbackOnFailure
        )
    }

    /// Returns true when this apply mode cannot mutate target state.
    pub fn is_no_write(self) -> bool {
        matches!(
            self,
            Self::DryRunPreviewOnly
                | Self::ApplyObserveOnlyNoWrite
                | Self::ApplyRefusedEscalationOnly
        )
    }
}

/// Online/offline requirement for the repair apply path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnlineOfflineRequirementClass {
    /// Apply requires online runtime access.
    RequiresOnline,
    /// Apply prefers online access but can observe offline.
    PrefersOnlineSupportsOfflineObserve,
    /// Apply works as a local-only offline operation.
    SupportsOfflineLocalOnly,
    /// Apply refuses remote service access.
    RequiresOfflineLocalOnly,
}

/// Trust and policy requirement for the repair apply path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPolicyRequirementClass {
    /// The transaction does not touch trust or policy state.
    NoTrustOrPolicyChange,
    /// Current trust must remain unchanged throughout apply.
    RequiresExistingTrustUnchanged,
    /// Explicit user consent is required and trust may not widen.
    RequiresExplicitUserConsentNoTrustWiden,
    /// Admin-authored policy must remain unchanged.
    RequiresAdminAuthoredPolicyUnchanged,
    /// Managed admin consent is required.
    RequiresManagedAdminConsent,
}

/// Action boundary that a repair transaction may not cross silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForbiddenActionClass {
    /// The repair may not widen workspace trust.
    WidenWorkspaceTrust,
    /// The repair may not publish a route.
    PublishRoute,
    /// The repair may not run repository hooks silently.
    RunRepoHookSilently,
    /// The repair may not silently reinstall extensions.
    SilentExtensionReinstall,
    /// The repair may not silently rebind helpers.
    SilentHelperRebind,
    /// The repair may not mutate managed policy.
    MutateManagedPolicy,
    /// The repair may not mutate user-authored files.
    MutateUserAuthoredFiles,
    /// The repair may not read or rotate credentials.
    ReadOrRotateCredentials,
    /// The repair may not retarget without user choice.
    AutoRetargetWithoutUser,
    /// The repair may not mutate authoritative profile stores.
    MutateAuthoritativeProfileStore,
    /// The repair may not embed raw secrets in export.
    EmbedRawSecretInExport,
    /// The repair may not widen the redaction choice automatically.
    AutoWidenRedactionChoice,
}

/// State class a repair may mutate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactedStateClass {
    /// Disposable derived cache state.
    DisposableDerivedCache,
    /// Watcher backlog state.
    WatcherBacklogState,
    /// Docs-pack mirror snapshot state.
    DocsPackMirrorSnapshot,
    /// Execution-context handle state.
    ExecutionContextHandle,
    /// Language-server session handle state.
    LanguageServerSessionHandle,
    /// Extension quarantine state.
    ExtensionQuarantineState,
    /// Extension install-set state.
    ExtensionInstallSet,
    /// Remote helper session handle state.
    RemoteHelperSessionHandle,
    /// Remote agent runtime handle state.
    RemoteAgentRuntimeHandle,
    /// Policy entitlement handle state.
    PolicyEntitlementHandle,
    /// Trust approval ticket state.
    TrustApprovalTicket,
    /// Support export store artifact state.
    SupportExportStoreExport,
    /// Doctor audit log entry state.
    DoctorAuditLogEntry,
}

/// State class a repair must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Open buffer selections.
    OpenBufferSelection,
    /// Durable workspace index records.
    DurableWorkspaceIndexes,
    /// Workspace trust store.
    WorkspaceTrustStore,
    /// Credential store.
    CredentialStore,
    /// Remote helper authorization.
    RemoteHelperAuthorization,
    /// Managed policy overrides.
    ManagedPolicyOverrides,
    /// Session restore store.
    SessionRestoreStore,
    /// Support export store.
    SupportExportStore,
    /// Docs-pack authoring state.
    DocsPackAuthoring,
    /// Installed extension marketplace state.
    InstalledExtensionMarketplaceState,
    /// Managed workspace control plane.
    ManagedWorkspaceControlPlane,
}

/// Capability class narrowed while a repair is in flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LostCapabilityClass {
    /// Extension auto-activation is paused.
    ExtensionAutoActivation,
    /// Extension host launch is paused.
    ExtensionHostLaunch,
    /// Session restore auto-reopen is paused.
    SessionRestoreAutoReopen,
    /// Remote helper attach is paused.
    RemoteHelperAttach,
    /// Live docs-pack fetch is paused.
    DocsPackLiveFetch,
    /// Background rebuild is paused.
    BackgroundRebuild,
    /// AI runtime access is paused.
    AiRuntimeAccess,
    /// Telemetry upload is paused.
    TelemetryUpload,
    /// Managed control-plane writes are paused.
    ManagedControlPlaneWrites,
    /// Workspace trust widening is paused.
    WorkspaceTrustWidening,
}

/// Escalation trigger for repair review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationTriggerClass {
    /// The repair was re-entered within its budget.
    RepeatedEntryWithinBudget,
    /// The user denied the next rung.
    UserDeniedNextRung,
    /// Exact undo is unavailable.
    ExactUndoUnavailable,
    /// Policy forced a narrower posture than requested.
    PolicyForcedNarrowerThanUserRequest,
    /// A lost capability blocks the current task.
    LostCapabilityBlocksCurrentTask,
    /// Policy denied a rung exit reason.
    RungExitReasonDeniedByPolicy,
    /// Diagnosis remains insufficient.
    DiagnosisRemainsInsufficient,
    /// No safe local repair path is available.
    NoLocalRepairPathAvailable,
    /// Online runtime is unavailable.
    OnlineRuntimeUnavailable,
    /// The idempotency key collided.
    IdempotencyKeyCollision,
    /// Trust state is below the required floor.
    TrustStateBelowFloor,
    /// Managed policy changed mid-transaction.
    ManagedPolicyChangedMidTransaction,
}

/// Default redaction choice for escalation and handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultRedactionChoiceClass {
    /// Metadata-only default.
    MetadataOnlyDefault,
    /// Selected references only.
    SelectedReferenceOnly,
    /// Support bundle by reference.
    SupportBundleByReference,
    /// Manual review required.
    ManualReviewRequired,
    /// Broadened capture requires opt-in.
    BroadenedCaptureOptIn,
}

/// Preview state for a repair preview record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewStateClass {
    /// Preview completed and is pending reviewer authorization.
    DryRunCompletePendingReview,
    /// Preview completed and all required confirmation is attached.
    DryRunSafeApplyAuthorized,
    /// Preview was blocked by policy.
    DryRunBlockedByPolicy,
    /// Preview refused because apply would widen trust.
    DryRunRefusedWidensTrust,
    /// Preview refused because apply would publish a route.
    DryRunRefusedPublishesRoute,
    /// Preview refused because apply would run a repository hook silently.
    DryRunRefusedRunsRepoHookSilently,
    /// Preview refused because apply would mutate user files.
    DryRunRefusedMutatesUserFiles,
    /// Preview refused because apply would read or rotate credentials.
    DryRunRefusedReadsOrRotatesCredentials,
    /// Preview refused because apply would retarget automatically.
    DryRunRefusedAutoRetarget,
    /// Preview refused local apply and prepared escalation.
    EscalationOnlyNoPreviewApply,
}

/// Preview blocker class for repair preview admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewBlockerClass {
    /// Finding evidence is missing.
    MissingFindingEvidence,
    /// Finding confidence is below the repair floor.
    FindingConfidenceInsufficient,
    /// No safe local repair exists.
    NoSafeLocalRepairAvailable,
    /// Policy blocks the repair.
    PolicyBlocksAction,
    /// Required online runtime is unavailable.
    OnlineRuntimeUnavailable,
    /// Trust state is below the repair floor.
    TrustStateBelowFloor,
    /// Idempotency key collided.
    IdempotencyKeyCollision,
    /// Checkpoint capture is unavailable.
    CheckpointCaptureUnavailable,
    /// Rollback target is unavailable.
    RollbackTargetUnavailable,
    /// Managed admin consent is required.
    ManagedAdminConsentRequired,
    /// Managed policy changed during preview.
    ManagedPolicyChangedDuringPreview,
}

/// Checkpoint class proposed by a repair preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointClass {
    /// Session-scoped checkpoint captured before apply.
    EphemeralPreApply,
    /// Durable checkpoint captured before apply.
    DurablePreApply,
    /// No checkpoint is needed because the repair is observe-only.
    NoCheckpointObserveOnly,
    /// No checkpoint exists because the repair is escalation-only.
    NoCheckpointEscalationOnly,
    /// Checkpoint capture was refused.
    CheckpointCaptureRefused,
}

/// Confirmation class required to authorize a preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationClass {
    /// Standard preview review is sufficient.
    StandardReview,
    /// Strong confirmation is required before apply.
    StrongConfirmationRequired,
    /// No apply path exists for this preview.
    NoApplyEscalationOnly,
}

/// Outcome class emitted after preview, refusal, or apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeClass {
    /// Preview completed but no apply ran.
    PreviewOnlyNoApply,
    /// Apply succeeded and recovered the target.
    AppliedSuccessRecovered,
    /// Apply partially recovered with typed unknowns.
    AppliedPartialRecoveredWithTypedUnknowns,
    /// Apply failed and restored a checkpoint.
    AppliedFailedRolledBack,
    /// Apply failed and ran compensating reversal.
    AppliedFailedCompensated,
    /// Apply failed with no safe reversal and exported evidence only.
    AppliedFailedNoReversalExportOnly,
    /// Apply did not run and escalation was prepared.
    EscalatedNoApply,
    /// Apply was refused because it would widen trust.
    RefusedPreApplyWidensTrust,
    /// Apply was refused because it would publish a route.
    RefusedPreApplyPublishesRoute,
    /// Apply was refused because it would run a repository hook silently.
    RefusedPreApplyRunsRepoHookSilently,
    /// Apply was refused because it would silently rebind a helper.
    RefusedPreApplySilentHelperRebind,
    /// Apply was refused because it would silently reinstall an extension.
    RefusedPreApplySilentExtensionReinstall,
    /// Apply was refused because it would mutate user files.
    RefusedPreApplyMutatesUserFiles,
    /// Apply was refused because it would read or rotate credentials.
    RefusedPreApplyReadsOrRotatesCredentials,
    /// Apply was refused because it would violate managed policy.
    RefusedPreApplyManagedPolicyViolation,
    /// Apply was refused because the idempotency key collided.
    RefusedPreApplyIdempotencyKeyCollision,
    /// Apply was refused because a runtime requirement was unmet.
    RefusedPreApplyRuntimeRequirementUnmet,
}

/// Source surface that initiated a repair transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairSourceSurfaceClass {
    /// Project Doctor initiated the repair.
    ProjectDoctor,
    /// Support Center initiated the repair.
    SupportCenter,
    /// Headless CLI initiated the repair.
    HeadlessCli,
    /// Recovery ladder initiated the repair.
    RecoveryLadder,
    /// Runbook execution initiated the repair.
    Runbook,
}

/// Actor class captured in the repair journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActorClass {
    /// Local user approved the repair.
    User,
    /// Managed admin approved the repair.
    ManagedAdmin,
    /// Support operator initiated the repair.
    SupportOperator,
    /// Project Doctor proposed the repair.
    ProjectDoctor,
    /// Automation proposed the repair.
    Automation,
}

/// Source class captured in the repair journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairJournalSourceClass {
    /// Project Doctor repair path.
    DoctorRepair,
    /// Support-guided repair path.
    SupportGuidedRepair,
    /// Recovery-ladder repair path.
    RecoveryLadderRepair,
    /// Runbook repair path.
    RunbookRepair,
}

/// Runtime gates that a transaction or preview must satisfy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeRequirements {
    /// Online/offline requirement.
    pub online_offline_requirement_class: OnlineOfflineRequirementClass,
    /// Trust and policy requirement.
    pub trust_policy_requirement_class: TrustPolicyRequirementClass,
    /// Whether active user consent is required.
    pub requires_active_user_consent: bool,
    /// Whether active admin consent is required.
    pub requires_active_admin_consent: bool,
}

/// Escalation route carried by a repair transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscalationRoute {
    /// Conditions that require escalation.
    pub escalation_required_when: Vec<EscalationTriggerClass>,
    /// Default handoff packet template.
    pub default_handoff_packet_template_ref: String,
    /// Default redaction choice.
    pub default_redaction_choice_class: DefaultRedactionChoiceClass,
}

/// Seed-case linkage refs to other support artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSeedLinkageRefs {
    /// Scenario row ref in the support scenario matrix.
    pub scenario_row_ref: String,
    /// Recovery action id, if the repair wraps a recovery action.
    #[serde(default)]
    pub recovery_action_id: Option<String>,
    /// Support-bundle fixture ref.
    pub support_bundle_case_ref: String,
    /// Object handoff or escalation case ref.
    pub object_handoff_case_ref: String,
    /// Project Doctor finding ref.
    pub project_doctor_finding_ref: String,
    /// Checkpoint ref, if the repair is checkpointed.
    #[serde(default)]
    pub checkpoint_ref: Option<String>,
    /// Preview record ref.
    pub preview_record_ref: String,
    /// Outcome record ref.
    pub outcome_record_ref: String,
}

/// Reviewer-facing explanation fields shared by transactions and seeds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairExplanationFields {
    /// Summary of the work and state that remains preserved.
    pub preserved_work_summary: String,
    /// Summary of the state classes that change.
    pub change_summary: String,
    /// Summary of capabilities narrowed during repair.
    pub capability_disablement_summary: String,
    /// Summary of escalation conditions.
    pub escalation_summary: String,
    /// Next step shown to the reviewer.
    pub user_facing_next_step: String,
}

/// A checked-in repair seed case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSeedCase {
    /// Repair transaction schema version.
    pub repair_transaction_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable seed case id.
    pub case_id: String,
    /// Reviewer-facing scenario summary.
    pub scenario_summary: String,
    /// Stable repair transaction id.
    pub repair_transaction_id: String,
    /// Repair-class family.
    pub repair_class_family: RepairClassFamily,
    /// Suggested repair class.
    pub suggested_repair_class: SuggestedRepairClass,
    /// Declared reversal class.
    pub transaction_reversal_class: TransactionReversalClass,
    /// Apply mode.
    pub apply_mode_class: ApplyModeClass,
    /// Initiating Project Doctor finding codes.
    pub initiating_finding_codes: Vec<String>,
    /// State classes that may be impacted.
    pub impacted_state_classes: Vec<ImpactedStateClass>,
    /// State classes that must remain preserved.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Capabilities narrowed while repair is active.
    pub lost_capability_classes: Vec<LostCapabilityClass>,
    /// Runtime requirements.
    pub runtime_requirements: RuntimeRequirements,
    /// Forbidden action assertions.
    pub forbidden_action_assertions: Vec<ForbiddenActionClass>,
    /// Whether a checkpoint ref is required.
    pub checkpoint_ref_required: bool,
    /// Stable idempotency key.
    pub idempotency_key: String,
    /// Preview artifact ref.
    pub preview_artifact_ref: String,
    /// Outcome artifact ref.
    pub outcome_artifact_ref: String,
    /// Escalation route.
    pub escalation_route: EscalationRoute,
    /// Links to neighboring support artifacts.
    pub linkage_refs: RepairSeedLinkageRefs,
    /// Default redaction choice.
    pub default_redaction_choice_class: DefaultRedactionChoiceClass,
    /// Reviewer-facing explanations.
    pub explanation_fields: RepairExplanationFields,
    /// Seed notes.
    pub notes: String,
    /// Timestamp when the seed was emitted.
    pub emitted_at: String,
}

impl RepairSeedCase {
    /// Validates the seed case against runtime repair invariants.
    pub fn validate(&self) -> Vec<RepairViolation> {
        let mut violations = Vec::new();
        validate_common(
            &mut violations,
            &self.case_id,
            self.repair_transaction_schema_version,
            &self.record_kind,
            REPAIR_SEED_CASE_RECORD_KIND,
            CommonRepairFields {
                repair_class_family: self.repair_class_family,
                transaction_reversal_class: self.transaction_reversal_class,
                apply_mode_class: self.apply_mode_class,
                impacted_state_classes: &self.impacted_state_classes,
                preserved_state_classes: &self.preserved_state_classes,
                forbidden_action_assertions: &self.forbidden_action_assertions,
                checkpoint_ref_required: self.checkpoint_ref_required,
                checkpoint_ref: self.linkage_refs.checkpoint_ref.as_deref(),
                runtime_requirements: &self.runtime_requirements,
                escalation_route: &self.escalation_route,
            },
        );

        if self.initiating_finding_codes.is_empty() {
            push_violation(
                &mut violations,
                "repair.initiating_findings_empty",
                &self.case_id,
                "repair seed must cite at least one Project Doctor finding",
            );
        }
        for finding in &self.initiating_finding_codes {
            if !finding.starts_with("doctor.finding.") {
                push_violation(
                    &mut violations,
                    "repair.initiating_finding_not_doctor_code",
                    &self.case_id,
                    format!("initiating finding must use doctor.finding.*: {finding}"),
                );
            }
        }
        if self.preview_artifact_ref != self.linkage_refs.preview_record_ref {
            push_violation(
                &mut violations,
                "repair.preview_linkage_mismatch",
                &self.case_id,
                "preview_artifact_ref must match linkage_refs.preview_record_ref",
            );
        }
        if self.outcome_artifact_ref != self.linkage_refs.outcome_record_ref {
            push_violation(
                &mut violations,
                "repair.outcome_linkage_mismatch",
                &self.case_id,
                "outcome_artifact_ref must match linkage_refs.outcome_record_ref",
            );
        }
        violations
    }
}

/// Governance schema refs carried by generated repair records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairGovernanceBindings {
    /// Evidence packet header schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evidence_packet_header_schema_ref: Option<String>,
    /// Repair transaction schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub repair_transaction_schema_ref: Option<String>,
    /// Repair preview schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub repair_preview_schema_ref: Option<String>,
    /// Repair outcome schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub repair_outcome_schema_ref: Option<String>,
    /// Support bundle schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub support_bundle_schema_ref: Option<String>,
    /// Object handoff packet schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_handoff_packet_schema_ref: Option<String>,
    /// Support packet index schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub support_packet_index_schema_ref: Option<String>,
    /// Recovery action schema ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub recovery_action_schema_ref: Option<String>,
    /// Evidence id conventions ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evidence_id_conventions_ref: Option<String>,
    /// Record class registry ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub record_class_registry_ref: Option<String>,
    /// Project Doctor packet ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub project_doctor_packet_ref: Option<String>,
    /// Recovery ladder packet ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub recovery_ladder_packet_ref: Option<String>,
    /// Scenario matrix ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scenario_matrix_ref: Option<String>,
    /// Repair transaction contract ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub repair_transaction_contract_ref: Option<String>,
}

impl RepairGovernanceBindings {
    /// Returns governance bindings for transaction records.
    pub fn transaction() -> Self {
        Self {
            evidence_packet_header_schema_ref: Some(
                "schemas/governance/evidence_packet_header.schema.json".into(),
            ),
            repair_transaction_schema_ref: None,
            repair_preview_schema_ref: Some("schemas/support/repair_preview.schema.json".into()),
            repair_outcome_schema_ref: Some("schemas/support/repair_outcome.schema.json".into()),
            support_bundle_schema_ref: Some("schemas/support/support_bundle.schema.json".into()),
            object_handoff_packet_schema_ref: Some(
                "schemas/support/object_handoff_packet.schema.json".into(),
            ),
            support_packet_index_schema_ref: Some(
                "schemas/support/support_packet_index.schema.json".into(),
            ),
            recovery_action_schema_ref: Some("schemas/support/recovery_action.schema.json".into()),
            evidence_id_conventions_ref: Some("docs/governance/evidence_id_conventions.md".into()),
            record_class_registry_ref: Some(
                "artifacts/governance/record_class_registry.yaml".into(),
            ),
            project_doctor_packet_ref: Some("docs/support/project_doctor_packet.md".into()),
            recovery_ladder_packet_ref: Some("docs/support/recovery_ladder_packet.md".into()),
            scenario_matrix_ref: Some("fixtures/support/scenario_matrix.yaml".into()),
            repair_transaction_contract_ref: None,
        }
    }

    /// Returns governance bindings for preview records.
    pub fn preview() -> Self {
        Self {
            evidence_packet_header_schema_ref: None,
            repair_transaction_schema_ref: Some(
                "schemas/support/repair_transaction.schema.json".into(),
            ),
            repair_preview_schema_ref: None,
            repair_outcome_schema_ref: Some("schemas/support/repair_outcome.schema.json".into()),
            support_bundle_schema_ref: Some("schemas/support/support_bundle.schema.json".into()),
            object_handoff_packet_schema_ref: Some(
                "schemas/support/object_handoff_packet.schema.json".into(),
            ),
            support_packet_index_schema_ref: None,
            recovery_action_schema_ref: Some("schemas/support/recovery_action.schema.json".into()),
            evidence_id_conventions_ref: Some("docs/governance/evidence_id_conventions.md".into()),
            record_class_registry_ref: None,
            project_doctor_packet_ref: None,
            recovery_ladder_packet_ref: None,
            scenario_matrix_ref: None,
            repair_transaction_contract_ref: Some(
                "docs/support/repair_transaction_contract.md".into(),
            ),
        }
    }

    /// Returns governance bindings for outcome records.
    pub fn outcome() -> Self {
        Self {
            evidence_packet_header_schema_ref: None,
            repair_transaction_schema_ref: Some(
                "schemas/support/repair_transaction.schema.json".into(),
            ),
            repair_preview_schema_ref: Some("schemas/support/repair_preview.schema.json".into()),
            repair_outcome_schema_ref: None,
            support_bundle_schema_ref: Some("schemas/support/support_bundle.schema.json".into()),
            object_handoff_packet_schema_ref: Some(
                "schemas/support/object_handoff_packet.schema.json".into(),
            ),
            support_packet_index_schema_ref: None,
            recovery_action_schema_ref: Some("schemas/support/recovery_action.schema.json".into()),
            evidence_id_conventions_ref: Some("docs/governance/evidence_id_conventions.md".into()),
            record_class_registry_ref: None,
            project_doctor_packet_ref: None,
            recovery_ladder_packet_ref: None,
            scenario_matrix_ref: None,
            repair_transaction_contract_ref: Some(
                "docs/support/repair_transaction_contract.md".into(),
            ),
        }
    }
}

/// Linkage class for repair transaction records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkageRequirementClass {
    /// Link to a support bundle record.
    SupportBundleRecord,
    /// Link to an object handoff packet record.
    ObjectHandoffPacketRecord,
    /// Link to a Project Doctor finding.
    ProjectDoctorFinding,
    /// Link to a recovery action record.
    RecoveryActionRecord,
    /// Link to a repair preview record.
    RepairPreviewRecord,
    /// Link to a repair outcome record.
    RepairOutcomeRecord,
    /// Link to a checkpoint ref.
    CheckpointRef,
    /// Link to a crash envelope ref.
    CrashEnvelopeRef,
    /// Link to a known-limit ref.
    KnownLimitRef,
    /// Link to a scenario row ref.
    ScenarioRowRef,
}

/// Requirement strength for a repair linkage binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkageRequirement {
    /// Required for every record.
    Required,
    /// Required when the source artifact exists.
    RequiredWhenApplicable,
    /// Advisory linkage.
    Advisory,
}

/// Typed linkage binding carried by a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkageBinding {
    /// Linkage class.
    pub linkage_class: LinkageRequirementClass,
    /// Requirement strength.
    pub requirement: LinkageRequirement,
    /// Field path on the transaction.
    pub field_path: String,
    /// Reviewer-facing notes.
    pub notes: String,
}

/// A live repair transaction record compiled from a seed case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairTransactionRecord {
    /// Repair transaction schema version.
    pub repair_transaction_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable repair transaction id.
    pub repair_transaction_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Governance schema bindings.
    pub governance_bindings: RepairGovernanceBindings,
    /// Repair-class family.
    pub repair_class_family: RepairClassFamily,
    /// Suggested repair class.
    pub suggested_repair_class: SuggestedRepairClass,
    /// Initiating Project Doctor finding codes.
    pub initiating_finding_codes: Vec<String>,
    /// State classes that may be impacted.
    pub impacted_state_classes: Vec<ImpactedStateClass>,
    /// State classes that must remain preserved.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Capabilities narrowed while repair is active.
    pub lost_capability_classes: Vec<LostCapabilityClass>,
    /// Runtime requirements.
    pub runtime_requirements: RuntimeRequirements,
    /// Forbidden action assertions.
    pub forbidden_action_assertions: Vec<ForbiddenActionClass>,
    /// Checkpoint ref used before apply.
    #[serde(default)]
    pub checkpoint_ref: Option<String>,
    /// Stable idempotency key.
    pub idempotency_key: String,
    /// Declared reversal class.
    pub transaction_reversal_class: TransactionReversalClass,
    /// Preview artifact ref.
    pub preview_artifact_ref: String,
    /// Apply mode.
    pub apply_mode_class: ApplyModeClass,
    /// Escalation route.
    pub escalation_route: EscalationRoute,
    /// Linkage bindings.
    pub linkage_bindings: Vec<LinkageBinding>,
    /// Default redaction choice.
    pub default_redaction_choice_class: DefaultRedactionChoiceClass,
    /// Reviewer-facing explanations.
    pub explanation_fields: RepairExplanationFields,
    /// Timestamp when the transaction was emitted.
    pub emitted_at: String,
}

impl RepairTransactionRecord {
    /// Validates the transaction record against runtime repair invariants.
    pub fn validate(&self) -> Vec<RepairViolation> {
        let mut violations = Vec::new();
        validate_common(
            &mut violations,
            &self.repair_transaction_id,
            self.repair_transaction_schema_version,
            &self.record_kind,
            REPAIR_TRANSACTION_RECORD_KIND,
            CommonRepairFields {
                repair_class_family: self.repair_class_family,
                transaction_reversal_class: self.transaction_reversal_class,
                apply_mode_class: self.apply_mode_class,
                impacted_state_classes: &self.impacted_state_classes,
                preserved_state_classes: &self.preserved_state_classes,
                forbidden_action_assertions: &self.forbidden_action_assertions,
                checkpoint_ref_required: self.apply_mode_class.requires_checkpoint(),
                checkpoint_ref: self.checkpoint_ref.as_deref(),
                runtime_requirements: &self.runtime_requirements,
                escalation_route: &self.escalation_route,
            },
        );

        if self.initiating_finding_codes.is_empty() {
            push_violation(
                &mut violations,
                "repair.transaction_initiating_findings_empty",
                &self.repair_transaction_id,
                "repair transaction must cite at least one Project Doctor finding",
            );
        }
        for finding in &self.initiating_finding_codes {
            if !finding.starts_with("doctor.finding.") {
                push_violation(
                    &mut violations,
                    "repair.transaction_initiating_finding_not_doctor_code",
                    &self.repair_transaction_id,
                    format!("initiating finding must use doctor.finding.*: {finding}"),
                );
            }
        }
        if self.preview_artifact_ref.is_empty() {
            push_violation(
                &mut violations,
                "repair.transaction_preview_ref_empty",
                &self.repair_transaction_id,
                "repair transaction must own a preview artifact ref",
            );
        }
        violations
    }

    /// Compiles a repair preview from this transaction.
    pub fn compile_preview(&self, request: RepairPreviewRequest) -> RepairPreviewRecord {
        let confirmation_requirement =
            RepairConfirmationRequirement::for_transaction(self, &request);
        let preview_blockers = preview_blockers(self, &request);
        let preview_state_class =
            preview_state_class(self, &request, &preview_blockers, &confirmation_requirement);
        let checkpoint_proposal = checkpoint_proposal(self, &request, preview_state_class);

        RepairPreviewRecord {
            repair_preview_schema_version: REPAIR_PREVIEW_SCHEMA_VERSION,
            record_kind: REPAIR_PREVIEW_RECORD_KIND.to_owned(),
            preview_id: self.preview_artifact_ref.clone(),
            repair_transaction_ref: self.repair_transaction_id.clone(),
            preview_state_class,
            claimed_reversal_class: self.transaction_reversal_class,
            checkpoint_proposal,
            runtime_requirements: self.runtime_requirements.clone(),
            forbidden_action_assertions: self.forbidden_action_assertions.clone(),
            impacted_change_rows: impacted_change_rows(self, preview_state_class),
            preserved_assertion_rows: preserved_assertion_rows(self),
            lost_capability_classes: self.lost_capability_classes.clone(),
            preview_blockers,
            idempotency_key: self.idempotency_key.clone(),
            confirmation_requirement,
            explanation_fields: preview_explanation_fields(self),
            governance_bindings: RepairGovernanceBindings::preview(),
            emitted_at: request.emitted_at,
        }
    }

    /// Emits an outcome from a preview without applying any hidden mutation.
    pub fn outcome_from_preview(
        &self,
        preview: &RepairPreviewRecord,
        request: RepairOutcomeRequest,
    ) -> RepairOutcomeRecord {
        let (outcome_class, violations, escalation_packet_ref) =
            outcome_class_from_preview(preview, self);
        let applied_change_rows = match outcome_class {
            OutcomeClass::AppliedSuccessRecovered
            | OutcomeClass::AppliedPartialRecoveredWithTypedUnknowns
            | OutcomeClass::AppliedFailedCompensated
            | OutcomeClass::AppliedFailedNoReversalExportOnly => applied_change_rows(self),
            _ => Vec::new(),
        };
        let checkpoint_used_ref = if matches!(
            outcome_class,
            OutcomeClass::AppliedSuccessRecovered
                | OutcomeClass::AppliedPartialRecoveredWithTypedUnknowns
                | OutcomeClass::AppliedFailedRolledBack
                | OutcomeClass::AppliedFailedCompensated
        ) {
            self.checkpoint_ref.clone()
        } else {
            None
        };

        RepairOutcomeRecord {
            repair_outcome_schema_version: REPAIR_OUTCOME_SCHEMA_VERSION,
            record_kind: REPAIR_OUTCOME_RECORD_KIND.to_owned(),
            outcome_id: outcome_id_for(self),
            repair_transaction_ref: self.repair_transaction_id.clone(),
            repair_preview_ref: preview.preview_id.clone(),
            outcome_class,
            applied_change_rows,
            preserved_state_classes_observed: self.preserved_state_classes.clone(),
            checkpoint_used_ref,
            reversal_executed_class: None,
            failure_reason_class: None,
            remaining_unknowns: Vec::new(),
            forbidden_action_assertions_held: violations.is_empty(),
            forbidden_action_violations: violations,
            runtime_requirements_held: RuntimeRequirementsHeld {
                online_state_held: request.online_state_held,
                trust_state_held: request.trust_state_held,
                user_consent_held: request.user_consent_held,
                admin_consent_held: request.admin_consent_held,
                idempotency_key_held: request.idempotency_key_held,
            },
            escalation_packet_ref,
            journal_lineage: RepairJournalLineage {
                actor_lineage: request.actor_lineage,
                source_lineage: request.source_lineage,
                initiating_diagnosis_refs: self.initiating_finding_codes.clone(),
            },
            explanation_fields: outcome_explanation_fields(self, outcome_class),
            governance_bindings: RepairGovernanceBindings::outcome(),
            emitted_at: request.emitted_at,
        }
    }

    /// Builds a mutation-journal entry linked to the given outcome.
    pub fn journal_entry(
        &self,
        preview: &RepairPreviewRecord,
        outcome: &RepairOutcomeRecord,
        emitted_at: impl Into<String>,
    ) -> RepairMutationJournalEntry {
        RepairMutationJournalEntry {
            repair_transaction_schema_version: REPAIR_TRANSACTION_SCHEMA_VERSION,
            record_kind: REPAIR_MUTATION_JOURNAL_RECORD_KIND.to_owned(),
            journal_entry_id: journal_entry_id_for(self),
            repair_transaction_ref: self.repair_transaction_id.clone(),
            repair_preview_ref: preview.preview_id.clone(),
            repair_outcome_ref: outcome.outcome_id.clone(),
            initiating_diagnosis_refs: self.initiating_finding_codes.clone(),
            actor_lineage: outcome.journal_lineage.actor_lineage.clone(),
            source_lineage: outcome.journal_lineage.source_lineage.clone(),
            checkpoint_refs: self.checkpoint_ref.iter().cloned().collect(),
            reversal_class: self.transaction_reversal_class,
            mutation_scope: RepairMutationScope {
                impacted_state_classes: self.impacted_state_classes.clone(),
                preserved_state_classes: self.preserved_state_classes.clone(),
                lost_capability_classes: self.lost_capability_classes.clone(),
            },
            side_effect_summary: self.explanation_fields.change_summary.clone(),
            emitted_at: emitted_at.into(),
        }
    }
}

/// Input controls for compiling a repair preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairPreviewRequest {
    /// Timestamp assigned to the preview.
    pub emitted_at: String,
    /// Whether required online runtime is available.
    pub online_runtime_available: bool,
    /// Whether current trust state satisfies the preview.
    pub trust_state_satisfies: bool,
    /// Whether required finding evidence is present.
    pub finding_evidence_present: bool,
    /// Whether finding confidence meets the repair floor.
    pub finding_confidence_sufficient: bool,
    /// Whether checkpoint capture is available.
    pub checkpoint_capture_available: bool,
    /// Whether the idempotency key is available.
    pub idempotency_key_available: bool,
    /// Whether managed policy stayed current during preview.
    pub managed_policy_current: bool,
    /// Whether user consent is attached.
    pub active_user_consent: bool,
    /// Whether admin consent is attached.
    pub active_admin_consent: bool,
    /// Whether strong confirmation is attached.
    pub strong_confirmation_ack: bool,
    /// Forbidden actions detected by dry-run.
    pub forbidden_action_violations: Vec<ForbiddenActionClass>,
}

impl RepairPreviewRequest {
    /// Returns a review-only request that has evidence but no apply consent.
    pub fn review_only(emitted_at: impl Into<String>) -> Self {
        Self {
            emitted_at: emitted_at.into(),
            online_runtime_available: true,
            trust_state_satisfies: true,
            finding_evidence_present: true,
            finding_confidence_sufficient: true,
            checkpoint_capture_available: true,
            idempotency_key_available: true,
            managed_policy_current: true,
            active_user_consent: false,
            active_admin_consent: false,
            strong_confirmation_ack: false,
            forbidden_action_violations: Vec::new(),
        }
    }

    /// Returns a request with all required review gates already attached.
    pub fn authorized_for(
        transaction: &RepairTransactionRecord,
        emitted_at: impl Into<String>,
    ) -> Self {
        Self {
            emitted_at: emitted_at.into(),
            online_runtime_available: true,
            trust_state_satisfies: true,
            finding_evidence_present: true,
            finding_confidence_sufficient: true,
            checkpoint_capture_available: true,
            idempotency_key_available: true,
            managed_policy_current: true,
            active_user_consent: transaction
                .runtime_requirements
                .requires_active_user_consent,
            active_admin_consent: transaction
                .runtime_requirements
                .requires_active_admin_consent,
            strong_confirmation_ack: transaction
                .transaction_reversal_class
                .requires_strong_confirmation(),
            forbidden_action_violations: Vec::new(),
        }
    }
}

/// Confirmation requirement surfaced by a preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairConfirmationRequirement {
    /// Confirmation class required before apply.
    pub confirmation_class: ConfirmationClass,
    /// Whether stronger confirmation is required.
    pub stronger_confirmation_required: bool,
    /// Acknowledgement token required by the UI or CLI.
    #[serde(default)]
    pub acknowledgement_token: Option<String>,
    /// Reviewer-facing reason.
    pub reason_summary: String,
    /// Reversal classes this requirement applies to.
    pub applies_to_reversal_classes: Vec<TransactionReversalClass>,
}

impl RepairConfirmationRequirement {
    /// Builds the confirmation requirement for a transaction.
    pub fn for_transaction(
        transaction: &RepairTransactionRecord,
        _request: &RepairPreviewRequest,
    ) -> Self {
        if transaction.apply_mode_class == ApplyModeClass::ApplyRefusedEscalationOnly {
            return Self {
                confirmation_class: ConfirmationClass::NoApplyEscalationOnly,
                stronger_confirmation_required: false,
                acknowledgement_token: None,
                reason_summary:
                    "No local apply is available; review routes to an escalation packet.".into(),
                applies_to_reversal_classes: vec![TransactionReversalClass::AuditOnly],
            };
        }

        if transaction
            .transaction_reversal_class
            .requires_strong_confirmation()
        {
            return Self {
                confirmation_class: ConfirmationClass::StrongConfirmationRequired,
                stronger_confirmation_required: true,
                acknowledgement_token: Some("confirm-non-exact-repair".into()),
                reason_summary: transaction.transaction_reversal_class.summary().into(),
                applies_to_reversal_classes: vec![transaction.transaction_reversal_class],
            };
        }

        Self {
            confirmation_class: ConfirmationClass::StandardReview,
            stronger_confirmation_required: false,
            acknowledgement_token: None,
            reason_summary: transaction.transaction_reversal_class.summary().into(),
            applies_to_reversal_classes: vec![transaction.transaction_reversal_class],
        }
    }
}

/// Checkpoint proposal shown by a preview before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointProposal {
    /// Checkpoint class.
    pub checkpoint_class: CheckpointClass,
    /// Checkpoint ref when available.
    #[serde(default)]
    pub checkpoint_ref: Option<String>,
    /// Reviewer-facing capture summary.
    pub capture_summary: String,
    /// State classes scoped by the checkpoint.
    pub scope_state_classes: Vec<ImpactedStateClass>,
}

/// One impacted state row in a repair preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedChangeRow {
    /// Impacted state class.
    pub impacted_state_class: ImpactedStateClass,
    /// Reviewer-facing change summary.
    pub change_summary: String,
}

/// One preserved state assertion row in a repair preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedAssertionRow {
    /// Preserved state class.
    pub preserved_state_class: PreservedStateClass,
    /// Reviewer-facing preservation summary.
    pub preservation_summary: String,
}

/// Reviewer-facing explanation fields for repair previews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewExplanationFields {
    /// Summary of the work and state that remains preserved.
    pub preserved_work_summary: String,
    /// Summary of the state classes that change.
    pub change_summary: String,
    /// Summary of checkpoint behavior.
    pub checkpoint_summary: String,
    /// Summary of reversal behavior.
    pub reversal_summary: String,
    /// Summary of escalation behavior.
    pub escalation_summary: String,
    /// Next step shown to the reviewer.
    pub user_facing_next_step: String,
}

/// A repair preview record compiled before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewRecord {
    /// Repair preview schema version.
    pub repair_preview_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable preview id.
    pub preview_id: String,
    /// Transaction this preview belongs to.
    pub repair_transaction_ref: String,
    /// Preview state.
    pub preview_state_class: PreviewStateClass,
    /// Reversal class claimed by apply.
    pub claimed_reversal_class: TransactionReversalClass,
    /// Checkpoint proposal.
    pub checkpoint_proposal: CheckpointProposal,
    /// Runtime requirements.
    pub runtime_requirements: RuntimeRequirements,
    /// Forbidden action assertions.
    pub forbidden_action_assertions: Vec<ForbiddenActionClass>,
    /// Impacted change rows.
    pub impacted_change_rows: Vec<ImpactedChangeRow>,
    /// Preserved state assertion rows.
    pub preserved_assertion_rows: Vec<PreservedAssertionRow>,
    /// Lost capability classes.
    pub lost_capability_classes: Vec<LostCapabilityClass>,
    /// Preview blockers.
    pub preview_blockers: Vec<PreviewBlockerClass>,
    /// Idempotency key.
    pub idempotency_key: String,
    /// Confirmation requirement before apply.
    pub confirmation_requirement: RepairConfirmationRequirement,
    /// Reviewer-facing explanations.
    pub explanation_fields: RepairPreviewExplanationFields,
    /// Governance schema bindings.
    pub governance_bindings: RepairGovernanceBindings,
    /// Timestamp when the preview was emitted.
    pub emitted_at: String,
}

impl RepairPreviewRecord {
    /// Returns true when the preview is authorized for apply.
    pub fn authorizes_apply(&self) -> bool {
        self.preview_state_class == PreviewStateClass::DryRunSafeApplyAuthorized
            && self.preview_blockers.is_empty()
    }

    /// Returns true when the preview shows at least one impacted and one
    /// preserved state class.
    pub fn exposes_blast_radius(&self) -> bool {
        !self.preserved_assertion_rows.is_empty()
            && (self.preview_state_class == PreviewStateClass::EscalationOnlyNoPreviewApply
                || !self.impacted_change_rows.is_empty())
    }
}

/// Actor lineage carried by repair outcome and journal records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairActorLineage {
    /// Actor class.
    pub actor_class: RepairActorClass,
    /// Opaque actor ref safe for support export.
    pub actor_ref: String,
    /// Source surface that initiated review.
    pub source_surface_class: RepairSourceSurfaceClass,
    /// Approval ref if consent was required.
    #[serde(default)]
    pub approval_ref: Option<String>,
}

impl RepairActorLineage {
    /// Creates local user actor lineage for support-center review.
    pub fn local_user_review(approval_ref: impl Into<String>) -> Self {
        Self {
            actor_class: RepairActorClass::User,
            actor_ref: "actor:user:local".into(),
            source_surface_class: RepairSourceSurfaceClass::SupportCenter,
            approval_ref: Some(approval_ref.into()),
        }
    }
}

/// Source lineage carried by repair outcome and journal records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSourceLineage {
    /// Journal source class.
    pub source_class: RepairJournalSourceClass,
    /// Primary diagnosis ref.
    pub diagnosis_ref: String,
    /// Scenario row ref.
    pub scenario_row_ref: String,
    /// Recovery action id, if present.
    #[serde(default)]
    pub recovery_action_id: Option<String>,
}

impl RepairSourceLineage {
    /// Builds source lineage from a repair transaction and seed linkage.
    pub fn from_seed(seed: &RepairSeedCase) -> Self {
        Self {
            source_class: RepairJournalSourceClass::DoctorRepair,
            diagnosis_ref: seed.linkage_refs.project_doctor_finding_ref.clone(),
            scenario_row_ref: seed.linkage_refs.scenario_row_ref.clone(),
            recovery_action_id: seed.linkage_refs.recovery_action_id.clone(),
        }
    }
}

/// Outcome request carrying actor/source lineage and held runtime gates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairOutcomeRequest {
    /// Timestamp assigned to the outcome.
    pub emitted_at: String,
    /// Actor lineage.
    pub actor_lineage: RepairActorLineage,
    /// Source lineage.
    pub source_lineage: RepairSourceLineage,
    /// Whether online state held.
    pub online_state_held: bool,
    /// Whether trust state held.
    pub trust_state_held: bool,
    /// Whether user consent held.
    pub user_consent_held: bool,
    /// Whether admin consent held.
    pub admin_consent_held: bool,
    /// Whether the idempotency key held.
    pub idempotency_key_held: bool,
}

impl RepairOutcomeRequest {
    /// Builds a successful outcome request from the seed lineage.
    pub fn from_seed(seed: &RepairSeedCase, emitted_at: impl Into<String>) -> Self {
        Self {
            emitted_at: emitted_at.into(),
            actor_lineage: RepairActorLineage::local_user_review(format!(
                "approval:{}",
                seed.idempotency_key
            )),
            source_lineage: RepairSourceLineage::from_seed(seed),
            online_state_held: true,
            trust_state_held: true,
            user_consent_held: seed.runtime_requirements.requires_active_user_consent,
            admin_consent_held: seed.runtime_requirements.requires_active_admin_consent,
            idempotency_key_held: true,
        }
    }
}

/// Runtime requirement checks observed by an outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeRequirementsHeld {
    /// Online state held.
    pub online_state_held: bool,
    /// Trust state held.
    pub trust_state_held: bool,
    /// User consent held.
    pub user_consent_held: bool,
    /// Admin consent held.
    pub admin_consent_held: bool,
    /// Idempotency key held.
    pub idempotency_key_held: bool,
}

/// One applied change row in a repair outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedChangeRow {
    /// Impacted state class.
    pub impacted_state_class: ImpactedStateClass,
    /// Reviewer-facing change summary.
    pub change_summary: String,
}

/// Journal lineage embedded in a repair outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairJournalLineage {
    /// Actor lineage.
    pub actor_lineage: RepairActorLineage,
    /// Source lineage.
    pub source_lineage: RepairSourceLineage,
    /// Initiating diagnosis refs.
    pub initiating_diagnosis_refs: Vec<String>,
}

/// Reviewer-facing explanation fields for repair outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairOutcomeExplanationFields {
    /// Post-apply state summary.
    pub post_apply_state_summary: String,
    /// Preserved state summary.
    pub preserved_state_summary: String,
    /// Reversal summary.
    pub reversal_summary: String,
    /// Escalation summary.
    pub escalation_summary: String,
    /// Next step shown to the reviewer.
    pub user_facing_next_step: String,
}

/// A repair outcome record emitted after preview, refusal, or apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairOutcomeRecord {
    /// Repair outcome schema version.
    pub repair_outcome_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable outcome id.
    pub outcome_id: String,
    /// Transaction this outcome belongs to.
    pub repair_transaction_ref: String,
    /// Preview this outcome belongs to.
    pub repair_preview_ref: String,
    /// Outcome class.
    pub outcome_class: OutcomeClass,
    /// Applied change rows.
    pub applied_change_rows: Vec<AppliedChangeRow>,
    /// Preserved state classes observed as untouched.
    pub preserved_state_classes_observed: Vec<PreservedStateClass>,
    /// Checkpoint used by apply or rollback.
    #[serde(default)]
    pub checkpoint_used_ref: Option<String>,
    /// Reversal class that executed on failure.
    #[serde(default)]
    pub reversal_executed_class: Option<TransactionReversalClass>,
    /// Failure reason class when apply failed.
    #[serde(default)]
    pub failure_reason_class: Option<String>,
    /// Typed unknowns that remain after apply.
    pub remaining_unknowns: Vec<String>,
    /// Whether forbidden action assertions held.
    pub forbidden_action_assertions_held: bool,
    /// Forbidden actions detected as violations.
    pub forbidden_action_violations: Vec<ForbiddenActionClass>,
    /// Runtime requirements observed during apply.
    pub runtime_requirements_held: RuntimeRequirementsHeld,
    /// Escalation packet ref when routed.
    #[serde(default)]
    pub escalation_packet_ref: Option<String>,
    /// Actor/source lineage for the journal.
    pub journal_lineage: RepairJournalLineage,
    /// Reviewer-facing explanations.
    pub explanation_fields: RepairOutcomeExplanationFields,
    /// Governance schema bindings.
    pub governance_bindings: RepairGovernanceBindings,
    /// Timestamp when the outcome was emitted.
    pub emitted_at: String,
}

/// Mutation scope captured by the repair journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairMutationScope {
    /// State classes impacted by the transaction.
    pub impacted_state_classes: Vec<ImpactedStateClass>,
    /// State classes preserved by the transaction.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Capabilities narrowed by the transaction.
    pub lost_capability_classes: Vec<LostCapabilityClass>,
}

/// Mutation-journal entry for a repair outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairMutationJournalEntry {
    /// Repair transaction schema version.
    pub repair_transaction_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable journal entry id.
    pub journal_entry_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Repair preview ref.
    pub repair_preview_ref: String,
    /// Repair outcome ref.
    pub repair_outcome_ref: String,
    /// Initiating diagnosis refs.
    pub initiating_diagnosis_refs: Vec<String>,
    /// Actor lineage.
    pub actor_lineage: RepairActorLineage,
    /// Source lineage.
    pub source_lineage: RepairSourceLineage,
    /// Checkpoint refs used by the transaction.
    pub checkpoint_refs: Vec<String>,
    /// Declared reversal class.
    pub reversal_class: TransactionReversalClass,
    /// Mutation scope.
    pub mutation_scope: RepairMutationScope,
    /// Reviewer-facing side-effect summary.
    pub side_effect_summary: String,
    /// Timestamp when the journal entry was emitted.
    pub emitted_at: String,
}

/// One repair row exported in a support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSupportPacketRow {
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Repair preview ref.
    pub repair_preview_ref: String,
    /// Repair outcome ref.
    pub repair_outcome_ref: String,
    /// Journal entry ref.
    pub journal_entry_ref: String,
    /// Initiating diagnosis refs.
    pub initiating_diagnosis_refs: Vec<String>,
    /// Checkpoint refs.
    pub checkpoint_refs: Vec<String>,
    /// Reversal class.
    pub reversal_class: TransactionReversalClass,
    /// Whether strong confirmation was required.
    pub strong_confirmation_required: bool,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Metadata-safe support packet for repair transactions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable support packet id.
    pub packet_id: String,
    /// Generated timestamp.
    pub generated_at: String,
    /// Repair rows.
    pub rows: Vec<RepairSupportPacketRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl RepairSupportPacket {
    /// Returns true when the packet is safe for metadata-only export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .rows
                .iter()
                .all(|row| row.raw_private_material_excluded)
    }
}

/// Repair alpha compiler and support packet builder.
#[derive(Debug, Default, Clone, Copy)]
pub struct RepairAlpha;

impl RepairAlpha {
    /// Creates a repair alpha compiler.
    pub fn new() -> Self {
        Self
    }

    /// Compiles a transaction from a seed case.
    ///
    /// # Errors
    ///
    /// Returns [`RepairCompileError`] when the seed violates contract
    /// invariants.
    pub fn transaction_from_seed(
        &self,
        seed: &RepairSeedCase,
    ) -> Result<RepairTransactionRecord, RepairCompileError> {
        let violations = seed.validate();
        if !violations.is_empty() {
            return Err(RepairCompileError { violations });
        }

        Ok(RepairTransactionRecord {
            repair_transaction_schema_version: REPAIR_TRANSACTION_SCHEMA_VERSION,
            record_kind: REPAIR_TRANSACTION_RECORD_KIND.to_owned(),
            repair_transaction_id: seed.repair_transaction_id.clone(),
            title: title_from_case(seed),
            summary: seed.scenario_summary.clone(),
            governance_bindings: RepairGovernanceBindings::transaction(),
            repair_class_family: seed.repair_class_family,
            suggested_repair_class: seed.suggested_repair_class,
            initiating_finding_codes: seed.initiating_finding_codes.clone(),
            impacted_state_classes: seed.impacted_state_classes.clone(),
            preserved_state_classes: seed.preserved_state_classes.clone(),
            lost_capability_classes: seed.lost_capability_classes.clone(),
            runtime_requirements: seed.runtime_requirements.clone(),
            forbidden_action_assertions: seed.forbidden_action_assertions.clone(),
            checkpoint_ref: seed.linkage_refs.checkpoint_ref.clone(),
            idempotency_key: seed.idempotency_key.clone(),
            transaction_reversal_class: seed.transaction_reversal_class,
            preview_artifact_ref: seed.preview_artifact_ref.clone(),
            apply_mode_class: seed.apply_mode_class,
            escalation_route: seed.escalation_route.clone(),
            linkage_bindings: linkage_bindings(seed),
            default_redaction_choice_class: seed.default_redaction_choice_class,
            explanation_fields: seed.explanation_fields.clone(),
            emitted_at: seed.emitted_at.clone(),
        })
    }

    /// Builds a metadata-safe support packet from journaled repair records.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: &[JournaledRepairRecord],
    ) -> RepairSupportPacket {
        RepairSupportPacket {
            record_kind: REPAIR_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            rows: records
                .iter()
                .map(|record| RepairSupportPacketRow {
                    repair_transaction_ref: record.transaction.repair_transaction_id.clone(),
                    repair_preview_ref: record.preview.preview_id.clone(),
                    repair_outcome_ref: record.outcome.outcome_id.clone(),
                    journal_entry_ref: record.journal.journal_entry_id.clone(),
                    initiating_diagnosis_refs: record.journal.initiating_diagnosis_refs.clone(),
                    checkpoint_refs: record.journal.checkpoint_refs.clone(),
                    reversal_class: record.transaction.transaction_reversal_class,
                    strong_confirmation_required: record
                        .preview
                        .confirmation_requirement
                        .stronger_confirmation_required,
                    raw_private_material_excluded: true,
                })
                .collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }
}

/// Group of transaction, preview, outcome, and journal records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournaledRepairRecord {
    /// Repair transaction record.
    pub transaction: RepairTransactionRecord,
    /// Repair preview record.
    pub preview: RepairPreviewRecord,
    /// Repair outcome record.
    pub outcome: RepairOutcomeRecord,
    /// Repair mutation journal entry.
    pub journal: RepairMutationJournalEntry,
}

/// One repair contract violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairViolation {
    /// Stable check id.
    pub check_id: String,
    /// Target record or field ref.
    pub target_ref: String,
    /// Violation message.
    pub message: String,
}

/// Error returned when a repair seed cannot compile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairCompileError {
    /// Violations that blocked compilation.
    pub violations: Vec<RepairViolation>,
}

impl fmt::Display for RepairCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "repair seed failed validation with {} violation(s)",
            self.violations.len()
        )
    }
}

impl Error for RepairCompileError {}

#[derive(Clone, Copy)]
struct CommonRepairFields<'a> {
    repair_class_family: RepairClassFamily,
    transaction_reversal_class: TransactionReversalClass,
    apply_mode_class: ApplyModeClass,
    impacted_state_classes: &'a [ImpactedStateClass],
    preserved_state_classes: &'a [PreservedStateClass],
    forbidden_action_assertions: &'a [ForbiddenActionClass],
    checkpoint_ref_required: bool,
    checkpoint_ref: Option<&'a str>,
    runtime_requirements: &'a RuntimeRequirements,
    escalation_route: &'a EscalationRoute,
}

fn validate_common(
    violations: &mut Vec<RepairViolation>,
    target_ref: &str,
    schema_version: u32,
    record_kind: &str,
    expected_record_kind: &str,
    fields: CommonRepairFields<'_>,
) {
    if schema_version != REPAIR_TRANSACTION_SCHEMA_VERSION {
        push_violation(
            violations,
            "repair.schema_version",
            target_ref,
            "repair transaction schema version must be 1",
        );
    }
    if record_kind != expected_record_kind {
        push_violation(
            violations,
            "repair.record_kind",
            target_ref,
            format!("record_kind must be {expected_record_kind}"),
        );
    }
    if fields
        .preserved_state_classes
        .iter()
        .all(|state| *state != PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            violations,
            "repair.user_authored_files_not_preserved",
            target_ref,
            "user_authored_files must be listed as preserved",
        );
    }
    for required in REQUIRED_FORBIDDEN_ACTIONS {
        if !fields
            .forbidden_action_assertions
            .iter()
            .any(|action| *action == required)
        {
            push_violation(
                violations,
                "repair.missing_forbidden_action",
                target_ref,
                format!("forbidden_action_assertions must contain {required:?}"),
            );
        }
    }
    if fields.apply_mode_class.requires_checkpoint() != fields.checkpoint_ref_required {
        push_violation(
            violations,
            "repair.checkpoint_requirement_mismatch",
            target_ref,
            "checkpoint requirement must match apply mode",
        );
    }
    if fields.checkpoint_ref_required && fields.checkpoint_ref.is_none() {
        push_violation(
            violations,
            "repair.checkpoint_ref_missing",
            target_ref,
            "checkpointed apply must carry a checkpoint ref",
        );
    }
    if !fields.checkpoint_ref_required && fields.checkpoint_ref.is_some() {
        push_violation(
            violations,
            "repair.checkpoint_ref_unexpected",
            target_ref,
            "non-checkpointed apply must not carry a checkpoint ref",
        );
    }
    if fields.apply_mode_class.is_no_write() && !fields.impacted_state_classes.is_empty() {
        push_violation(
            violations,
            "repair.no_write_impacted_state_present",
            target_ref,
            "no-write apply modes must not list impacted state classes",
        );
    }
    if matches!(
        fields.repair_class_family,
        RepairClassFamily::GuidedExportEscalation | RepairClassFamily::ObserveOnlyNoRepair
    ) {
        if fields.transaction_reversal_class != TransactionReversalClass::AuditOnly {
            push_violation(
                violations,
                "repair.no_write_reversal_not_audit_only",
                target_ref,
                "guided export and observe-only repairs must be audit_only",
            );
        }
        if !fields.impacted_state_classes.is_empty() {
            push_violation(
                violations,
                "repair.no_write_impacted_state_not_empty",
                target_ref,
                "guided export and observe-only repairs must not list impacted state",
            );
        }
    }
    if fields.repair_class_family == RepairClassFamily::GuidedExportEscalation
        && !fields
            .escalation_route
            .escalation_required_when
            .contains(&EscalationTriggerClass::NoLocalRepairPathAvailable)
    {
        push_violation(
            violations,
            "repair.guided_export_missing_no_local_repair_trigger",
            target_ref,
            "guided export repairs must escalate when no local repair path exists",
        );
    }
    if fields.runtime_requirements.trust_policy_requirement_class
        == TrustPolicyRequirementClass::RequiresManagedAdminConsent
        && !fields.runtime_requirements.requires_active_admin_consent
    {
        push_violation(
            violations,
            "repair.managed_admin_consent_flag_missing",
            target_ref,
            "managed admin consent requirement must set requires_active_admin_consent",
        );
    }
}

fn preview_blockers(
    transaction: &RepairTransactionRecord,
    request: &RepairPreviewRequest,
) -> Vec<PreviewBlockerClass> {
    let mut blockers = Vec::new();
    if !request.finding_evidence_present {
        blockers.push(PreviewBlockerClass::MissingFindingEvidence);
    }
    if !request.finding_confidence_sufficient {
        blockers.push(PreviewBlockerClass::FindingConfidenceInsufficient);
    }
    if transaction
        .runtime_requirements
        .online_offline_requirement_class
        == OnlineOfflineRequirementClass::RequiresOnline
        && !request.online_runtime_available
    {
        blockers.push(PreviewBlockerClass::OnlineRuntimeUnavailable);
    }
    if !request.trust_state_satisfies {
        blockers.push(PreviewBlockerClass::TrustStateBelowFloor);
    }
    if !request.idempotency_key_available {
        blockers.push(PreviewBlockerClass::IdempotencyKeyCollision);
    }
    if transaction.apply_mode_class.requires_checkpoint() && !request.checkpoint_capture_available {
        blockers.push(PreviewBlockerClass::CheckpointCaptureUnavailable);
    }
    if !request.managed_policy_current {
        blockers.push(PreviewBlockerClass::ManagedPolicyChangedDuringPreview);
    }
    if transaction
        .runtime_requirements
        .requires_active_admin_consent
        && !request.active_admin_consent
    {
        blockers.push(PreviewBlockerClass::ManagedAdminConsentRequired);
    }
    blockers
}

fn preview_state_class(
    transaction: &RepairTransactionRecord,
    request: &RepairPreviewRequest,
    blockers: &[PreviewBlockerClass],
    confirmation: &RepairConfirmationRequirement,
) -> PreviewStateClass {
    if transaction.apply_mode_class == ApplyModeClass::ApplyRefusedEscalationOnly {
        return PreviewStateClass::EscalationOnlyNoPreviewApply;
    }
    if let Some(refused) =
        request
            .forbidden_action_violations
            .iter()
            .find_map(|action| match action {
                ForbiddenActionClass::WidenWorkspaceTrust => {
                    Some(PreviewStateClass::DryRunRefusedWidensTrust)
                }
                ForbiddenActionClass::PublishRoute => {
                    Some(PreviewStateClass::DryRunRefusedPublishesRoute)
                }
                ForbiddenActionClass::RunRepoHookSilently => {
                    Some(PreviewStateClass::DryRunRefusedRunsRepoHookSilently)
                }
                ForbiddenActionClass::MutateUserAuthoredFiles => {
                    Some(PreviewStateClass::DryRunRefusedMutatesUserFiles)
                }
                ForbiddenActionClass::ReadOrRotateCredentials => {
                    Some(PreviewStateClass::DryRunRefusedReadsOrRotatesCredentials)
                }
                ForbiddenActionClass::AutoRetargetWithoutUser => {
                    Some(PreviewStateClass::DryRunRefusedAutoRetarget)
                }
                _ => None,
            })
    {
        return refused;
    }
    if blockers.iter().any(|blocker| {
        matches!(
            blocker,
            PreviewBlockerClass::PolicyBlocksAction
                | PreviewBlockerClass::ManagedPolicyChangedDuringPreview
                | PreviewBlockerClass::ManagedAdminConsentRequired
        )
    }) {
        return PreviewStateClass::DryRunBlockedByPolicy;
    }

    let user_consent_ok = !transaction
        .runtime_requirements
        .requires_active_user_consent
        || request.active_user_consent;
    let admin_consent_ok = !transaction
        .runtime_requirements
        .requires_active_admin_consent
        || request.active_admin_consent;
    let strong_ok = !confirmation.stronger_confirmation_required || request.strong_confirmation_ack;
    if blockers.is_empty() && user_consent_ok && admin_consent_ok && strong_ok {
        PreviewStateClass::DryRunSafeApplyAuthorized
    } else {
        PreviewStateClass::DryRunCompletePendingReview
    }
}

fn checkpoint_proposal(
    transaction: &RepairTransactionRecord,
    request: &RepairPreviewRequest,
    preview_state_class: PreviewStateClass,
) -> CheckpointProposal {
    if preview_state_class == PreviewStateClass::EscalationOnlyNoPreviewApply {
        return CheckpointProposal {
            checkpoint_class: CheckpointClass::NoCheckpointEscalationOnly,
            checkpoint_ref: None,
            capture_summary:
                "No checkpoint is captured because this repair only prepares escalation.".into(),
            scope_state_classes: Vec::new(),
        };
    }
    if transaction.apply_mode_class == ApplyModeClass::ApplyObserveOnlyNoWrite {
        return CheckpointProposal {
            checkpoint_class: CheckpointClass::NoCheckpointObserveOnly,
            checkpoint_ref: None,
            capture_summary:
                "No checkpoint is needed because the repair does not write target state.".into(),
            scope_state_classes: Vec::new(),
        };
    }
    if transaction.apply_mode_class.requires_checkpoint() && !request.checkpoint_capture_available {
        return CheckpointProposal {
            checkpoint_class: CheckpointClass::CheckpointCaptureRefused,
            checkpoint_ref: None,
            capture_summary: "Checkpoint capture is unavailable; apply is not authorized.".into(),
            scope_state_classes: transaction.impacted_state_classes.clone(),
        };
    }
    if transaction.apply_mode_class.requires_checkpoint() {
        return CheckpointProposal {
            checkpoint_class: CheckpointClass::DurablePreApply,
            checkpoint_ref: transaction.checkpoint_ref.clone(),
            capture_summary: "A durable pre-apply checkpoint is available before mutation.".into(),
            scope_state_classes: transaction.impacted_state_classes.clone(),
        };
    }
    CheckpointProposal {
        checkpoint_class: CheckpointClass::NoCheckpointObserveOnly,
        checkpoint_ref: None,
        capture_summary: "No checkpoint is needed for this preview-only repair.".into(),
        scope_state_classes: Vec::new(),
    }
}

fn impacted_change_rows(
    transaction: &RepairTransactionRecord,
    preview_state_class: PreviewStateClass,
) -> Vec<ImpactedChangeRow> {
    if matches!(
        preview_state_class,
        PreviewStateClass::EscalationOnlyNoPreviewApply
            | PreviewStateClass::DryRunRefusedWidensTrust
            | PreviewStateClass::DryRunRefusedPublishesRoute
            | PreviewStateClass::DryRunRefusedRunsRepoHookSilently
            | PreviewStateClass::DryRunRefusedMutatesUserFiles
            | PreviewStateClass::DryRunRefusedReadsOrRotatesCredentials
            | PreviewStateClass::DryRunRefusedAutoRetarget
    ) {
        return Vec::new();
    }
    transaction
        .impacted_state_classes
        .iter()
        .copied()
        .map(|impacted_state_class| ImpactedChangeRow {
            impacted_state_class,
            change_summary: transaction.explanation_fields.change_summary.clone(),
        })
        .collect()
}

fn preserved_assertion_rows(transaction: &RepairTransactionRecord) -> Vec<PreservedAssertionRow> {
    transaction
        .preserved_state_classes
        .iter()
        .copied()
        .map(|preserved_state_class| PreservedAssertionRow {
            preserved_state_class,
            preservation_summary: transaction
                .explanation_fields
                .preserved_work_summary
                .clone(),
        })
        .collect()
}

fn applied_change_rows(transaction: &RepairTransactionRecord) -> Vec<AppliedChangeRow> {
    transaction
        .impacted_state_classes
        .iter()
        .copied()
        .map(|impacted_state_class| AppliedChangeRow {
            impacted_state_class,
            change_summary: transaction.explanation_fields.change_summary.clone(),
        })
        .collect()
}

fn preview_explanation_fields(
    transaction: &RepairTransactionRecord,
) -> RepairPreviewExplanationFields {
    RepairPreviewExplanationFields {
        preserved_work_summary: transaction
            .explanation_fields
            .preserved_work_summary
            .clone(),
        change_summary: transaction.explanation_fields.change_summary.clone(),
        checkpoint_summary: match transaction.checkpoint_ref.as_deref() {
            Some(checkpoint) => format!("Checkpoint available before apply: {checkpoint}."),
            None => "No checkpoint is required for this repair path.".into(),
        },
        reversal_summary: transaction.transaction_reversal_class.summary().into(),
        escalation_summary: transaction.explanation_fields.escalation_summary.clone(),
        user_facing_next_step: transaction.explanation_fields.user_facing_next_step.clone(),
    }
}

fn outcome_explanation_fields(
    transaction: &RepairTransactionRecord,
    outcome_class: OutcomeClass,
) -> RepairOutcomeExplanationFields {
    RepairOutcomeExplanationFields {
        post_apply_state_summary: match outcome_class {
            OutcomeClass::AppliedSuccessRecovered => {
                "Repair applied and recovered the targeted state classes.".into()
            }
            OutcomeClass::EscalatedNoApply => {
                "No local repair was applied; escalation context was prepared.".into()
            }
            OutcomeClass::PreviewOnlyNoApply => {
                "Preview completed; no mutation was applied.".into()
            }
            _ => "Repair outcome recorded with typed review state.".into(),
        },
        preserved_state_summary: transaction
            .explanation_fields
            .preserved_work_summary
            .clone(),
        reversal_summary: transaction.transaction_reversal_class.summary().into(),
        escalation_summary: transaction.explanation_fields.escalation_summary.clone(),
        user_facing_next_step: transaction.explanation_fields.user_facing_next_step.clone(),
    }
}

fn outcome_class_from_preview(
    preview: &RepairPreviewRecord,
    transaction: &RepairTransactionRecord,
) -> (OutcomeClass, Vec<ForbiddenActionClass>, Option<String>) {
    match preview.preview_state_class {
        PreviewStateClass::DryRunSafeApplyAuthorized => {
            (OutcomeClass::AppliedSuccessRecovered, Vec::new(), None)
        }
        PreviewStateClass::EscalationOnlyNoPreviewApply => (
            OutcomeClass::EscalatedNoApply,
            Vec::new(),
            Some(escalation_packet_ref(transaction)),
        ),
        PreviewStateClass::DryRunRefusedWidensTrust => (
            OutcomeClass::RefusedPreApplyWidensTrust,
            vec![ForbiddenActionClass::WidenWorkspaceTrust],
            None,
        ),
        PreviewStateClass::DryRunRefusedPublishesRoute => (
            OutcomeClass::RefusedPreApplyPublishesRoute,
            vec![ForbiddenActionClass::PublishRoute],
            None,
        ),
        PreviewStateClass::DryRunRefusedRunsRepoHookSilently => (
            OutcomeClass::RefusedPreApplyRunsRepoHookSilently,
            vec![ForbiddenActionClass::RunRepoHookSilently],
            None,
        ),
        PreviewStateClass::DryRunRefusedMutatesUserFiles => (
            OutcomeClass::RefusedPreApplyMutatesUserFiles,
            vec![ForbiddenActionClass::MutateUserAuthoredFiles],
            None,
        ),
        PreviewStateClass::DryRunRefusedReadsOrRotatesCredentials => (
            OutcomeClass::RefusedPreApplyReadsOrRotatesCredentials,
            vec![ForbiddenActionClass::ReadOrRotateCredentials],
            None,
        ),
        PreviewStateClass::DryRunRefusedAutoRetarget => (
            OutcomeClass::RefusedPreApplyManagedPolicyViolation,
            vec![ForbiddenActionClass::AutoRetargetWithoutUser],
            None,
        ),
        PreviewStateClass::DryRunBlockedByPolicy => (
            OutcomeClass::RefusedPreApplyManagedPolicyViolation,
            vec![ForbiddenActionClass::MutateManagedPolicy],
            None,
        ),
        PreviewStateClass::DryRunCompletePendingReview => {
            (OutcomeClass::PreviewOnlyNoApply, Vec::new(), None)
        }
    }
}

fn title_from_case(seed: &RepairSeedCase) -> String {
    let suffix = seed
        .repair_transaction_id
        .rsplit([':', '.'])
        .next()
        .unwrap_or(seed.repair_transaction_id.as_str())
        .replace('_', " ");
    format!("Repair transaction: {suffix}")
}

fn outcome_id_for(transaction: &RepairTransactionRecord) -> String {
    transaction
        .preview_artifact_ref
        .replacen("repair_preview:", "repair_outcome:", 1)
}

fn journal_entry_id_for(transaction: &RepairTransactionRecord) -> String {
    transaction
        .repair_transaction_id
        .replacen("repair_transaction:", "repair_journal:", 1)
}

fn escalation_packet_ref(transaction: &RepairTransactionRecord) -> String {
    let suffix = transaction
        .repair_transaction_id
        .strip_prefix("repair_transaction:")
        .unwrap_or(transaction.repair_transaction_id.as_str());
    format!("object_handoff_packet:repair_escalation.{suffix}")
}

fn linkage_bindings(seed: &RepairSeedCase) -> Vec<LinkageBinding> {
    let mut bindings = vec![
        LinkageBinding {
            linkage_class: LinkageRequirementClass::ProjectDoctorFinding,
            requirement: LinkageRequirement::Required,
            field_path: "initiating_finding_codes".into(),
            notes: format!(
                "Initiating diagnosis is {}.",
                seed.linkage_refs.project_doctor_finding_ref
            ),
        },
        LinkageBinding {
            linkage_class: LinkageRequirementClass::RepairPreviewRecord,
            requirement: LinkageRequirement::Required,
            field_path: "preview_artifact_ref".into(),
            notes: format!("Preview record is {}.", seed.preview_artifact_ref),
        },
        LinkageBinding {
            linkage_class: LinkageRequirementClass::RepairOutcomeRecord,
            requirement: LinkageRequirement::Required,
            field_path: "outcome_artifact_ref".into(),
            notes: format!("Outcome record is {}.", seed.outcome_artifact_ref),
        },
        LinkageBinding {
            linkage_class: LinkageRequirementClass::SupportBundleRecord,
            requirement: LinkageRequirement::RequiredWhenApplicable,
            field_path: "linkage_refs.support_bundle_case_ref".into(),
            notes: format!(
                "Support bundle case ref is {}.",
                seed.linkage_refs.support_bundle_case_ref
            ),
        },
        LinkageBinding {
            linkage_class: LinkageRequirementClass::ObjectHandoffPacketRecord,
            requirement: LinkageRequirement::RequiredWhenApplicable,
            field_path: "linkage_refs.object_handoff_case_ref".into(),
            notes: format!(
                "Object handoff case ref is {}.",
                seed.linkage_refs.object_handoff_case_ref
            ),
        },
        LinkageBinding {
            linkage_class: LinkageRequirementClass::ScenarioRowRef,
            requirement: LinkageRequirement::Required,
            field_path: "linkage_refs.scenario_row_ref".into(),
            notes: format!(
                "Scenario row ref is {}.",
                seed.linkage_refs.scenario_row_ref
            ),
        },
    ];

    if seed.linkage_refs.recovery_action_id.is_some() {
        bindings.push(LinkageBinding {
            linkage_class: LinkageRequirementClass::RecoveryActionRecord,
            requirement: LinkageRequirement::RequiredWhenApplicable,
            field_path: "linkage_refs.recovery_action_id".into(),
            notes: "Recovery action linkage is present for this transaction.".into(),
        });
    }
    if seed.linkage_refs.checkpoint_ref.is_some() {
        bindings.push(LinkageBinding {
            linkage_class: LinkageRequirementClass::CheckpointRef,
            requirement: LinkageRequirement::RequiredWhenApplicable,
            field_path: "checkpoint_ref".into(),
            notes: "Checkpoint ref is available before apply.".into(),
        });
    }

    bindings
}

fn push_violation(
    violations: &mut Vec<RepairViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(RepairViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
