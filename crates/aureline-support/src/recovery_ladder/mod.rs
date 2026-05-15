//! Recovery-ladder alpha evaluator and export projections.
//!
//! The ladder consumes typed recovery evidence from shell, runtime,
//! extension, restore, and cache/index owners. It then emits one bounded
//! decision record plus metadata-only support and release projections. The
//! evaluator never deletes user-owned state and never carries ambient
//! authority; mutating follow-up work is represented as explicit reviewed
//! actions.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for protected recovery-ladder alpha fixtures.
pub const RECOVERY_LADDER_ALPHA_SCENARIO_RECORD_KIND: &str = "recovery_ladder_alpha_scenario";

/// Stable record-kind tag for one recovery-ladder decision.
pub const RECOVERY_LADDER_DECISION_RECORD_KIND: &str = "recovery_ladder_decision";

/// Stable record-kind tag for the metadata-safe support projection.
pub const RECOVERY_LADDER_SUPPORT_PACKET_RECORD_KIND: &str = "recovery_ladder_support_packet";

/// Stable record-kind tag for the metadata-safe release projection.
pub const RECOVERY_LADDER_RELEASE_PACKET_RECORD_KIND: &str = "recovery_ladder_release_packet";

/// Current schema version for recovery-ladder alpha records.
pub const RECOVERY_LADDER_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Managed-service outage class used by continuity and failover surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageClass {
    /// Local editing and recovery continue while optional service lanes degrade.
    LocalCoreContinuity,
    /// Identity, policy, tenancy, catalog, quota, or orchestration authority is impaired.
    ControlPlaneImpairment,
    /// Runtime traffic, attach streams, artifact transfer, or live IO is impaired.
    DataPlaneImpairment,
    /// The target workspace, device, remote agent, or authority identity is missing or untrusted.
    FullTargetLoss,
}

impl OutageClass {
    /// All outage classes in the protected backup, restore, and failover taxonomy.
    pub const ALL: [Self; 4] = [
        Self::LocalCoreContinuity,
        Self::ControlPlaneImpairment,
        Self::DataPlaneImpairment,
        Self::FullTargetLoss,
    ];

    /// Returns the primary plane associated with this outage class.
    pub const fn primary_plane_class(self) -> OutagePlaneClass {
        match self {
            Self::LocalCoreContinuity => OutagePlaneClass::LocalCore,
            Self::ControlPlaneImpairment => OutagePlaneClass::ControlPlane,
            Self::DataPlaneImpairment => OutagePlaneClass::DataPlane,
            Self::FullTargetLoss => OutagePlaneClass::TargetAuthority,
        }
    }

    /// Returns the stable fixture token for this outage class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreContinuity => "local_core_continuity",
            Self::ControlPlaneImpairment => "control_plane_impairment",
            Self::DataPlaneImpairment => "data_plane_impairment",
            Self::FullTargetLoss => "full_target_loss",
        }
    }
}

/// Primary plane affected by a managed-service or recovery outage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutagePlaneClass {
    /// Local shell, editor, search, Git, export, and diagnostics baseline.
    LocalCore,
    /// Identity, policy, catalog, tenancy, quota, and orchestration authority.
    ControlPlane,
    /// Interactive traffic, artifact transfer, prompt streams, presence, and remote attach IO.
    DataPlane,
    /// The workspace, device, mounted filesystem, remote agent, or route target identity.
    TargetAuthority,
}

impl OutagePlaneClass {
    /// Returns the stable fixture token for this plane class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCore => "local_core",
            Self::ControlPlane => "control_plane",
            Self::DataPlane => "data_plane",
            Self::TargetAuthority => "target_authority",
        }
    }
}

/// Loads a recovery-ladder alpha scenario from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`RecoveryLadderScenario`].
pub fn load_alpha_scenario(yaml: &str) -> Result<RecoveryLadderScenario, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Versioned recovery rung handled by the alpha ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRungClass {
    /// Minimal runtime profile entered after a crash loop or explicit request.
    SafeMode,
    /// Quarantine for one runtime or extension lane.
    RuntimeExtensionQuarantine,
    /// Workspace entry that skips restore replay while preserving restore state.
    OpenWithoutRestore,
    /// Disposable cache/index repair that rebuilds from authoritative state.
    CacheIndexRepair,
}

/// Opaque target kind narrowed by a recovery rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryTargetKind {
    /// Whole workspace session entry.
    WorkspaceSession,
    /// One supervised runtime lane such as a language host.
    RuntimeHostLane,
    /// One extension or extension-host lane.
    ExtensionLane,
    /// One disposable cache or index lane.
    CacheIndexLane,
}

/// Visible state class surfaced after the rung enters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryVisibleStateClass {
    /// Work can continue with narrowed capability.
    Degraded,
    /// The target lane is isolated pending explicit review.
    Quarantined,
    /// A reviewed repair is being applied.
    Applying,
    /// The system recovered but remains under a narrowed capability floor.
    RecoveredWithLimits,
}

/// Specific ladder state emitted by the alpha evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderStateClass {
    /// Safe-mode runtime profile is active.
    SafeMode,
    /// One runtime or extension lane is quarantined.
    RuntimeOrExtensionQuarantined,
    /// Workspace opened with restore replay disabled.
    OpenedWithoutRestore,
    /// Cache/index repair was admitted against disposable state only.
    CacheIndexRepairApplied,
}

/// Entry reason that made the ladder available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEntryReasonClass {
    /// Startup failures repeated inside the configured crash-loop window.
    CrashLoopDetected,
    /// Entry is already degraded and needs a bounded recovery posture.
    DegradedEntryRequested,
    /// A supervised runtime lane exceeded its restart budget.
    RuntimeRestartBudgetExceeded,
    /// A supervised extension lane exceeded its restart budget.
    ExtensionRestartBudgetExceeded,
    /// Restore replay is unsafe or user-declined for this launch.
    RestoreReplayUnsafe,
    /// Disposable cache or index integrity failed.
    CacheIndexIntegrityFailure,
    /// User explicitly chose a recovery rung.
    ExplicitUserChoice,
}

/// State class protected or touched by a recovery rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll state for open buffers.
    OpenBufferSelection,
    /// Durable index state that must not be deleted as collateral.
    DurableWorkspaceIndexes,
    /// Workspace trust state.
    WorkspaceTrustStore,
    /// Credential handles and stores.
    CredentialStore,
    /// Session restore records.
    SessionRestoreStore,
    /// Support export records and staging state.
    SupportExportStore,
    /// Disposable cache or index shards that may be regenerated.
    DisposableCacheIndex,
}

/// Capability class narrowed while a rung is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCapabilityClass {
    /// Third-party extension auto-activation.
    ExtensionAutoActivation,
    /// Extension host launch.
    ExtensionHostLaunch,
    /// Session restore auto-reopen and replay.
    SessionRestoreAutoReopen,
    /// Remote helper auto-attach or reattach.
    RemoteHelperAttach,
    /// AI runtime access.
    AiRuntimeAccess,
    /// Optional background rebuild or indexing work.
    BackgroundRebuild,
    /// Live docs-pack fetches.
    DocsPackLiveFetch,
    /// Telemetry upload.
    TelemetryUpload,
}

/// Concrete change made or staged by a rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryChangeClass {
    /// Third-party extension activation is disabled.
    DisableThirdPartyExtensionActivation,
    /// Heavy optional background services are disabled.
    DisableHeavyBackgroundServices,
    /// Remote or shared-session auto-reattach is disabled.
    DisableRemoteAutoReattach,
    /// Restore replay is disabled for this entry.
    DisableRestoreReplay,
    /// One runtime or extension lane is quarantined.
    QuarantineTargetLane,
    /// Disposable index shards are disposed.
    DisposeDisposableIndexShards,
    /// Index rebuild is scheduled from authoritative state.
    ScheduleIndexRebuild,
    /// Session restore records are kept read-only.
    KeepSessionRestoreStoreReadOnly,
}

/// Quarantine reason class that support and release packets may quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReasonClass {
    /// Crash loop exceeded the configured strike budget.
    CrashLoopBudgetExceeded,
    /// Runtime lane exceeded its restart budget.
    RuntimeRestartBudgetExceeded,
    /// Extension lane exceeded its restart budget.
    ExtensionRestartBudgetExceeded,
    /// Compatibility regression was detected.
    CompatibilityRegression,
    /// Signed policy or emergency action forced quarantine.
    PolicyForcedQuarantine,
}

/// Release visibility for a quarantine decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReleaseVisibilityClass {
    /// Visible only in local support/export packets.
    SupportOnly,
    /// Visible in release evidence packets.
    ReleaseEvidence,
    /// Visible as a public known-limit or advisory-backed row.
    PublicKnownLimit,
}

/// Action class used to leave a rung or alter a quarantine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Exit the safe-mode runtime profile after review.
    ExitSafeMode,
    /// Open the workspace with restore enabled again.
    ReopenWithRestoreEnabled,
    /// Clear an active quarantine record without enabling the lane.
    ClearQuarantine,
    /// Re-enable or retry the quarantined lane after review.
    ReenableQuarantinedLane,
    /// Return to the full runtime profile.
    ReturnToFullMode,
    /// Rebuild a cache or index from authoritative state.
    RebuildCacheIndex,
    /// Export diagnostics for support or release review.
    ExportDiagnostics,
}

/// Fuller runtime posture a return path aims to restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullerModeClass {
    /// Full mode with normal runtime services enabled.
    FullMode,
    /// Restore-enabled workspace entry.
    RestoreEnabledEntry,
    /// Lane re-enabled under normal admission checks.
    LaneReadmitted,
    /// Rebuilt cache/index service running normally.
    RebuiltIndexReady,
}

/// Diagnostic data class carried by ladder evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEvidenceDataClass {
    /// Metadata-only evidence.
    Metadata,
    /// Environment-adjacent evidence such as version or fault-domain refs.
    EnvironmentAdjacent,
    /// Code-adjacent evidence that is forbidden in alpha projections.
    CodeAdjacent,
    /// Secret-bearing evidence that is forbidden in alpha projections.
    SecretBearing,
}

/// Redaction posture for one evidence reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRedactionClass {
    /// Metadata-safe default redaction.
    MetadataSafeDefault,
    /// Opt-in support evidence.
    OptInOnly,
    /// Prohibited from support or release projections.
    Prohibited,
}

/// Bounded target affected by a recovery rung.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryTarget {
    /// Opaque target identifier safe for support and release packets.
    pub target_id: String,
    /// Target kind.
    pub target_kind: RecoveryTargetKind,
    /// Opaque lane reference safe for export.
    pub lane_ref: String,
    /// Reviewer-facing label that excludes raw paths and private content.
    pub display_label: String,
}

/// Entry context consumed by the ladder evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryEntryContext {
    /// Entry reason class.
    pub entry_reason_class: RecoveryEntryReasonClass,
    /// Fault-domain or restore/cache lane reference.
    pub fault_domain_ref: String,
    /// Restart strikes observed in the active window.
    pub strike_count: u32,
    /// Automatic restart budget for the window.
    pub strike_budget: u32,
    /// Hidden restarts already attempted before the explicit rung.
    pub hidden_restart_attempts: u32,
    /// Last failure reason safe for support and release output.
    pub last_failure_reason_class: String,
    /// Project Doctor finding that justified the recovery rung.
    pub doctor_finding_ref: String,
}

/// Explicit action surfaced to return to a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryActionRef {
    /// Stable action identifier.
    pub action_id: String,
    /// Action class.
    pub action_class: RecoveryActionClass,
    /// Whether review is required before execution.
    pub requires_review: bool,
    /// Redaction-safe summary of the action.
    pub summary: String,
}

/// Return path from a recovery rung toward a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReturnPath {
    /// Fuller mode targeted by the return path.
    pub fuller_mode_class: FullerModeClass,
    /// Action used to leave the rung.
    pub return_action: RecoveryActionRef,
    /// Conditions that must hold before restoring fuller mode.
    pub restore_conditions: Vec<String>,
}

/// Mutation envelope declared by a recovery rung.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryMutationScope {
    /// Concrete changes made or staged by the rung.
    pub changes: Vec<RecoveryChangeClass>,
    /// State classes preserved by the rung.
    pub preserved_state_classes: Vec<RecoveryStateClass>,
    /// Capabilities disabled or narrowed by the rung.
    pub disabled_capability_classes: Vec<RecoveryCapabilityClass>,
    /// Disposable state classes the rung may regenerate.
    pub disposable_state_classes: Vec<RecoveryStateClass>,
    /// Whether any user-owned state is deleted.
    pub user_owned_state_deleted: bool,
    /// Whether any durable non-disposable state is deleted.
    pub durable_state_deleted: bool,
}

/// Redaction-safe evidence reference used by ladder decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence kind or source role.
    pub evidence_kind: String,
    /// Diagnostic data class.
    pub data_class: RecoveryEvidenceDataClass,
    /// Redaction class.
    pub redaction_class: RecoveryRedactionClass,
    /// Reviewable summary without raw private content.
    pub summary: String,
}

/// Quarantine declaration required when a runtime or extension lane is isolated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryQuarantineSpec {
    /// Opaque quarantined lane reference.
    pub lane_ref: String,
    /// Owner responsible for the quarantine state.
    pub owner_ref: String,
    /// Reason class for the quarantine.
    pub reason_class: QuarantineReasonClass,
    /// UTC timestamp after which the quarantine must block until reviewed.
    pub expires_at: String,
    /// Release visibility for the quarantine.
    pub release_visibility: QuarantineReleaseVisibilityClass,
    /// Conditions required before the lane may be restored.
    pub restore_conditions: Vec<String>,
    /// Explicit action that clears the quarantine record.
    pub clear_action: RecoveryActionRef,
    /// Explicit action that re-enables or retries the lane.
    pub reenable_action: RecoveryActionRef,
    /// Evidence refs that justify the quarantine.
    pub evidence_refs: Vec<String>,
}

impl RecoveryQuarantineSpec {
    /// Returns true when this quarantine has expired at `as_of`.
    pub fn is_expired_at(&self, as_of: &str) -> bool {
        !self.expires_at.is_empty() && self.expires_at.as_str() <= as_of
    }
}

/// Expected decision fields embedded in protected fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedRecoveryDecision {
    /// Expected ladder state.
    pub ladder_state_class: RecoveryLadderStateClass,
    /// Expected visible state.
    pub visible_state_class: RecoveryVisibleStateClass,
}

/// One protected recovery-ladder fixture or runtime input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderScenario {
    /// Scenario schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scenario identifier.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Capture timestamp used for expiry checks.
    pub captured_at: String,
    /// Support packet ref that consumes this decision.
    pub support_packet_ref: String,
    /// Release packet ref that consumes this decision, when applicable.
    pub release_packet_ref: Option<String>,
    /// Entry context.
    pub entry: RecoveryEntryContext,
    /// Target affected by the rung.
    pub target: RecoveryTarget,
    /// Requested recovery rung.
    pub requested_rung: RecoveryRungClass,
    /// Mutation and preservation scope.
    pub mutation: RecoveryMutationScope,
    /// Return path toward fuller mode.
    pub return_path: RecoveryReturnPath,
    /// Quarantine details for runtime or extension quarantine rungs.
    pub quarantine: Option<RecoveryQuarantineSpec>,
    /// Redaction-safe evidence refs.
    pub evidence: Vec<RecoveryEvidenceRef>,
    /// Expected decision for protected fixtures.
    pub expected: ExpectedRecoveryDecision,
}

/// Evaluated recovery-ladder decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderDecision {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Decision schema version.
    pub schema_version: u32,
    /// Stable decision identifier.
    pub decision_id: String,
    /// Source scenario id.
    pub scenario_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Rung that was entered.
    pub rung_class: RecoveryRungClass,
    /// Ladder state after entry.
    pub ladder_state_class: RecoveryLadderStateClass,
    /// Visible state after entry.
    pub visible_state_class: RecoveryVisibleStateClass,
    /// Target affected by the decision.
    pub target: RecoveryTarget,
    /// Last failure reason safe for export.
    pub last_failure_reason_class: String,
    /// Project Doctor finding that justified the recovery rung.
    pub doctor_finding_ref: String,
    /// Whether hidden restart attempts were stopped by the ladder.
    pub hidden_restart_suppressed: bool,
    /// Mutation and preservation scope.
    pub mutation: RecoveryMutationScope,
    /// Return path toward fuller mode.
    pub return_path: RecoveryReturnPath,
    /// Quarantine record emitted by the decision, when present.
    pub quarantine: Option<RecoveryQuarantineRecord>,
    /// Evidence refs cited by the decision.
    pub evidence: Vec<RecoveryEvidenceRef>,
    /// Support packet ref that consumes this decision.
    pub support_packet_ref: String,
    /// Release packet ref that consumes this decision.
    pub release_packet_ref: Option<String>,
}

/// Active quarantine record emitted by a recovery-ladder decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryQuarantineRecord {
    /// Opaque quarantined lane reference.
    pub lane_ref: String,
    /// Owner responsible for the quarantine state.
    pub owner_ref: String,
    /// Reason class for the quarantine.
    pub reason_class: QuarantineReasonClass,
    /// Time when the quarantine was created.
    pub created_at: String,
    /// UTC timestamp after which review is required before continuation.
    pub expires_at: String,
    /// Release visibility for the quarantine.
    pub release_visibility: QuarantineReleaseVisibilityClass,
    /// Conditions required before the lane may be restored.
    pub restore_conditions: Vec<String>,
    /// Explicit action that clears the quarantine record.
    pub clear_action: RecoveryActionRef,
    /// Explicit action that re-enables or retries the lane.
    pub reenable_action: RecoveryActionRef,
    /// Evidence refs that justify the quarantine.
    pub evidence_refs: Vec<String>,
}

/// Metadata-safe support packet projection for ladder decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Decision rows in the packet.
    pub rows: Vec<RecoveryLadderSupportRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl RecoveryLadderSupportPacket {
    /// Returns true when the support packet remains metadata-only and actionable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .rows
                .iter()
                .all(RecoveryLadderSupportRow::is_export_safe)
    }
}

/// One metadata-safe support row for a ladder decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderSupportRow {
    /// Decision id.
    pub decision_id: String,
    /// Rung class.
    pub rung_class: RecoveryRungClass,
    /// Visible state after entry.
    pub visible_state_class: RecoveryVisibleStateClass,
    /// Target kind.
    pub target_kind: RecoveryTargetKind,
    /// Opaque lane ref.
    pub lane_ref: String,
    /// Last failure reason class.
    pub last_failure_reason_class: String,
    /// Project Doctor finding that justified the recovery rung.
    pub doctor_finding_ref: String,
    /// Changed classes.
    pub changed_classes: Vec<RecoveryChangeClass>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<RecoveryStateClass>,
    /// Disabled capability classes.
    pub disabled_capability_classes: Vec<RecoveryCapabilityClass>,
    /// Return action toward fuller mode.
    pub return_action: RecoveryActionRef,
    /// Quarantine summary, when present.
    pub quarantine: Option<RecoveryQuarantineSummary>,
    /// Evidence refs cited by support.
    pub evidence_refs: Vec<String>,
}

impl RecoveryLadderSupportRow {
    /// Returns true when this row can be included in a support export.
    pub fn is_export_safe(&self) -> bool {
        !self.evidence_refs.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self
                .preserved_state_classes
                .contains(&RecoveryStateClass::UserAuthoredFiles)
            && self
                .quarantine
                .as_ref()
                .map_or(true, RecoveryQuarantineSummary::is_export_safe)
    }
}

/// Metadata-safe quarantine summary shared by support and release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryQuarantineSummary {
    /// Opaque quarantined lane reference.
    pub lane_ref: String,
    /// Owner responsible for the quarantine.
    pub owner_ref: String,
    /// Reason class for the quarantine.
    pub reason_class: QuarantineReasonClass,
    /// UTC timestamp after which review is required.
    pub expires_at: String,
    /// Release visibility for the quarantine.
    pub release_visibility: QuarantineReleaseVisibilityClass,
    /// Conditions required before fuller mode can be restored.
    pub restore_conditions: Vec<String>,
    /// Explicit clear action id.
    pub clear_action_id: String,
    /// Explicit re-enable action id.
    pub reenable_action_id: String,
    /// Evidence refs that justify the quarantine.
    pub evidence_refs: Vec<String>,
}

impl RecoveryQuarantineSummary {
    /// Returns true when the quarantine summary is attributable and actionable.
    pub fn is_export_safe(&self) -> bool {
        !self.lane_ref.is_empty()
            && !self.owner_ref.is_empty()
            && !self.expires_at.is_empty()
            && !self.restore_conditions.is_empty()
            && !self.clear_action_id.is_empty()
            && !self.reenable_action_id.is_empty()
            && !self.evidence_refs.is_empty()
    }
}

/// Metadata-safe release projection for ladder decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderReleasePacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Release rows in the packet.
    pub rows: Vec<RecoveryLadderReleaseRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl RecoveryLadderReleasePacket {
    /// Returns true when release rows preserve evidence without private payloads.
    pub fn is_release_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .rows
                .iter()
                .all(RecoveryLadderReleaseRow::is_release_safe)
    }
}

/// One metadata-safe release row for a ladder decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderReleaseRow {
    /// Decision id.
    pub decision_id: String,
    /// Rung class.
    pub rung_class: RecoveryRungClass,
    /// Visible state class after entry.
    pub visible_state_class: RecoveryVisibleStateClass,
    /// Release visibility.
    pub release_visibility: QuarantineReleaseVisibilityClass,
    /// Target kind.
    pub target_kind: RecoveryTargetKind,
    /// Opaque lane ref.
    pub lane_ref: String,
    /// Last failure reason class.
    pub last_failure_reason_class: String,
    /// Project Doctor finding that justified the recovery rung.
    pub doctor_finding_ref: String,
    /// Quarantine summary, when present.
    pub quarantine: Option<RecoveryQuarantineSummary>,
    /// Evidence refs that justify the release row.
    pub evidence_refs: Vec<String>,
}

impl RecoveryLadderReleaseRow {
    /// Returns true when this row can be included in release evidence.
    pub fn is_release_safe(&self) -> bool {
        !self.evidence_refs.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self
                .quarantine
                .as_ref()
                .map_or(true, RecoveryQuarantineSummary::is_export_safe)
    }
}

/// Validation failure emitted by the recovery-ladder evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLadderViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when a scenario cannot enter the ladder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLadderValidationReport {
    /// Validation failures.
    pub violations: Vec<RecoveryLadderViolation>,
}

impl fmt::Display for RecoveryLadderValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} recovery-ladder violation(s)", self.violations.len())
    }
}

impl Error for RecoveryLadderValidationReport {}

/// Recovery-ladder alpha evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct RecoveryLadderAlpha;

impl RecoveryLadderAlpha {
    /// Creates a new recovery-ladder alpha evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates one typed scenario into a bounded ladder decision.
    ///
    /// # Errors
    ///
    /// Returns [`RecoveryLadderValidationReport`] when the scenario omits
    /// required state preservation, carries unsafe evidence, defines an
    /// invalid quarantine, or mismatches the expected protected outcome.
    pub fn evaluate(
        &self,
        scenario: &RecoveryLadderScenario,
    ) -> Result<RecoveryLadderDecision, RecoveryLadderValidationReport> {
        let mut violations = validate_scenario(scenario);
        let (ladder_state_class, visible_state_class) = state_for_rung(scenario.requested_rung);
        if scenario.expected.ladder_state_class != ladder_state_class {
            push_violation(
                &mut violations,
                "recovery_ladder.expected_ladder_state_mismatch",
                &scenario.scenario_id,
                "expected ladder state does not match the requested rung",
            );
        }
        if scenario.expected.visible_state_class != visible_state_class {
            push_violation(
                &mut violations,
                "recovery_ladder.expected_visible_state_mismatch",
                &scenario.scenario_id,
                "expected visible state does not match the requested rung",
            );
        }

        if !violations.is_empty() {
            return Err(RecoveryLadderValidationReport { violations });
        }

        Ok(RecoveryLadderDecision {
            record_kind: RECOVERY_LADDER_DECISION_RECORD_KIND.to_owned(),
            schema_version: RECOVERY_LADDER_ALPHA_SCHEMA_VERSION,
            decision_id: format!(
                "decision:recovery-ladder:{}",
                sanitize_ref(&scenario.scenario_id)
            ),
            scenario_id: scenario.scenario_id.clone(),
            captured_at: scenario.captured_at.clone(),
            rung_class: scenario.requested_rung,
            ladder_state_class,
            visible_state_class,
            target: scenario.target.clone(),
            last_failure_reason_class: scenario.entry.last_failure_reason_class.clone(),
            doctor_finding_ref: scenario.entry.doctor_finding_ref.clone(),
            hidden_restart_suppressed: scenario.entry.strike_count >= scenario.entry.strike_budget
                && scenario.entry.strike_budget > 0,
            mutation: scenario.mutation.clone(),
            return_path: scenario.return_path.clone(),
            quarantine: scenario
                .quarantine
                .as_ref()
                .map(|quarantine| RecoveryQuarantineRecord {
                    lane_ref: quarantine.lane_ref.clone(),
                    owner_ref: quarantine.owner_ref.clone(),
                    reason_class: quarantine.reason_class,
                    created_at: scenario.captured_at.clone(),
                    expires_at: quarantine.expires_at.clone(),
                    release_visibility: quarantine.release_visibility,
                    restore_conditions: quarantine.restore_conditions.clone(),
                    clear_action: quarantine.clear_action.clone(),
                    reenable_action: quarantine.reenable_action.clone(),
                    evidence_refs: quarantine.evidence_refs.clone(),
                }),
            evidence: scenario.evidence.clone(),
            support_packet_ref: scenario.support_packet_ref.clone(),
            release_packet_ref: scenario.release_packet_ref.clone(),
        })
    }

    /// Builds a metadata-safe support projection from decisions.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        decisions: &[RecoveryLadderDecision],
    ) -> RecoveryLadderSupportPacket {
        RecoveryLadderSupportPacket {
            record_kind: RECOVERY_LADDER_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: RECOVERY_LADDER_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            rows: decisions
                .iter()
                .map(RecoveryLadderSupportRow::from)
                .collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    /// Builds a metadata-safe release projection from decisions.
    pub fn release_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        decisions: &[RecoveryLadderDecision],
    ) -> RecoveryLadderReleasePacket {
        RecoveryLadderReleasePacket {
            record_kind: RECOVERY_LADDER_RELEASE_PACKET_RECORD_KIND.to_owned(),
            schema_version: RECOVERY_LADDER_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            rows: decisions
                .iter()
                .map(RecoveryLadderReleaseRow::from)
                .collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }
}

impl From<&RecoveryLadderDecision> for RecoveryLadderSupportRow {
    fn from(decision: &RecoveryLadderDecision) -> Self {
        Self {
            decision_id: decision.decision_id.clone(),
            rung_class: decision.rung_class,
            visible_state_class: decision.visible_state_class,
            target_kind: decision.target.target_kind,
            lane_ref: decision.target.lane_ref.clone(),
            last_failure_reason_class: decision.last_failure_reason_class.clone(),
            doctor_finding_ref: decision.doctor_finding_ref.clone(),
            changed_classes: decision.mutation.changes.clone(),
            preserved_state_classes: decision.mutation.preserved_state_classes.clone(),
            disabled_capability_classes: decision.mutation.disabled_capability_classes.clone(),
            return_action: decision.return_path.return_action.clone(),
            quarantine: decision
                .quarantine
                .as_ref()
                .map(RecoveryQuarantineSummary::from),
            evidence_refs: decision
                .evidence
                .iter()
                .map(|evidence| evidence.evidence_ref.clone())
                .collect(),
        }
    }
}

impl From<&RecoveryLadderDecision> for RecoveryLadderReleaseRow {
    fn from(decision: &RecoveryLadderDecision) -> Self {
        Self {
            decision_id: decision.decision_id.clone(),
            rung_class: decision.rung_class,
            visible_state_class: decision.visible_state_class,
            release_visibility: decision.quarantine.as_ref().map_or(
                QuarantineReleaseVisibilityClass::SupportOnly,
                |quarantine| quarantine.release_visibility,
            ),
            target_kind: decision.target.target_kind,
            lane_ref: decision.target.lane_ref.clone(),
            last_failure_reason_class: decision.last_failure_reason_class.clone(),
            doctor_finding_ref: decision.doctor_finding_ref.clone(),
            quarantine: decision
                .quarantine
                .as_ref()
                .map(RecoveryQuarantineSummary::from),
            evidence_refs: decision
                .evidence
                .iter()
                .map(|evidence| evidence.evidence_ref.clone())
                .collect(),
        }
    }
}

impl From<&RecoveryQuarantineRecord> for RecoveryQuarantineSummary {
    fn from(quarantine: &RecoveryQuarantineRecord) -> Self {
        Self {
            lane_ref: quarantine.lane_ref.clone(),
            owner_ref: quarantine.owner_ref.clone(),
            reason_class: quarantine.reason_class,
            expires_at: quarantine.expires_at.clone(),
            release_visibility: quarantine.release_visibility,
            restore_conditions: quarantine.restore_conditions.clone(),
            clear_action_id: quarantine.clear_action.action_id.clone(),
            reenable_action_id: quarantine.reenable_action.action_id.clone(),
            evidence_refs: quarantine.evidence_refs.clone(),
        }
    }
}

fn validate_scenario(scenario: &RecoveryLadderScenario) -> Vec<RecoveryLadderViolation> {
    let mut violations = Vec::new();

    if scenario.schema_version != RECOVERY_LADDER_ALPHA_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "recovery_ladder.schema_version",
            &scenario.scenario_id,
            "scenario schema_version must be 1",
        );
    }
    if scenario.record_kind != RECOVERY_LADDER_ALPHA_SCENARIO_RECORD_KIND {
        push_violation(
            &mut violations,
            "recovery_ladder.record_kind",
            &scenario.scenario_id,
            "scenario record_kind must be recovery_ladder_alpha_scenario",
        );
    }
    if scenario.evidence.is_empty() {
        push_violation(
            &mut violations,
            "recovery_ladder.evidence_missing",
            &scenario.scenario_id,
            "scenario must cite at least one metadata-safe evidence ref",
        );
    }
    if !scenario
        .entry
        .doctor_finding_ref
        .starts_with("doctor.finding.")
    {
        push_violation(
            &mut violations,
            "recovery_ladder.doctor_finding_ref_missing",
            &scenario.scenario_id,
            "scenario must cite a Project Doctor finding ref",
        );
    }
    for evidence in &scenario.evidence {
        if matches!(
            evidence.data_class,
            RecoveryEvidenceDataClass::CodeAdjacent | RecoveryEvidenceDataClass::SecretBearing
        ) {
            push_violation(
                &mut violations,
                "recovery_ladder.evidence_private_data_class",
                &evidence.evidence_ref,
                "evidence must stay metadata or environment-adjacent",
            );
        }
        if evidence.redaction_class != RecoveryRedactionClass::MetadataSafeDefault {
            push_violation(
                &mut violations,
                "recovery_ladder.evidence_redaction_not_metadata_safe",
                &evidence.evidence_ref,
                "evidence must use metadata_safe_default redaction",
            );
        }
    }

    if scenario.mutation.user_owned_state_deleted {
        push_violation(
            &mut violations,
            "recovery_ladder.user_owned_state_deleted",
            &scenario.scenario_id,
            "recovery rungs must not delete user-owned state",
        );
    }
    if scenario.mutation.durable_state_deleted {
        push_violation(
            &mut violations,
            "recovery_ladder.durable_state_deleted",
            &scenario.scenario_id,
            "recovery rungs must not delete durable non-disposable state",
        );
    }
    if !scenario
        .mutation
        .preserved_state_classes
        .contains(&RecoveryStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "recovery_ladder.user_authored_files_must_be_preserved",
            &scenario.scenario_id,
            "user_authored_files must be preserved on every rung",
        );
    }
    if scenario.mutation.changes.is_empty() {
        push_violation(
            &mut violations,
            "recovery_ladder.changes_missing",
            &scenario.scenario_id,
            "each rung must name what it changes",
        );
    }
    if scenario.return_path.restore_conditions.is_empty()
        || scenario.return_path.return_action.action_id.is_empty()
    {
        push_violation(
            &mut violations,
            "recovery_ladder.return_path_missing",
            &scenario.scenario_id,
            "each rung must name how to return to a fuller mode",
        );
    }

    match scenario.requested_rung {
        RecoveryRungClass::SafeMode => validate_safe_mode(scenario, &mut violations),
        RecoveryRungClass::RuntimeExtensionQuarantine => {
            validate_runtime_extension_quarantine(scenario, &mut violations)
        }
        RecoveryRungClass::OpenWithoutRestore => {
            validate_open_without_restore(scenario, &mut violations)
        }
        RecoveryRungClass::CacheIndexRepair => {
            validate_cache_index_repair(scenario, &mut violations)
        }
    }

    violations
}

fn validate_safe_mode(
    scenario: &RecoveryLadderScenario,
    violations: &mut Vec<RecoveryLadderViolation>,
) {
    if scenario.target.target_kind != RecoveryTargetKind::WorkspaceSession {
        push_violation(
            violations,
            "recovery_ladder.safe_mode_target_not_workspace",
            &scenario.scenario_id,
            "safe mode must target a workspace session",
        );
    }
    for required in [
        RecoveryChangeClass::DisableThirdPartyExtensionActivation,
        RecoveryChangeClass::DisableRestoreReplay,
    ] {
        require_change(scenario, violations, required);
    }
}

fn validate_runtime_extension_quarantine(
    scenario: &RecoveryLadderScenario,
    violations: &mut Vec<RecoveryLadderViolation>,
) {
    if !matches!(
        scenario.target.target_kind,
        RecoveryTargetKind::RuntimeHostLane | RecoveryTargetKind::ExtensionLane
    ) {
        push_violation(
            violations,
            "recovery_ladder.quarantine_target_not_runtime_or_extension",
            &scenario.scenario_id,
            "runtime/extension quarantine must target one runtime or extension lane",
        );
    }
    require_change(
        scenario,
        violations,
        RecoveryChangeClass::QuarantineTargetLane,
    );

    let Some(quarantine) = &scenario.quarantine else {
        push_violation(
            violations,
            "recovery_ladder.quarantine_missing",
            &scenario.scenario_id,
            "runtime/extension quarantine must define a quarantine record",
        );
        return;
    };

    if quarantine.owner_ref.trim().is_empty() {
        push_violation(
            violations,
            "recovery_ladder.quarantine_owner_missing",
            &scenario.scenario_id,
            "quarantine owner must be explicit",
        );
    }
    if quarantine.lane_ref != scenario.target.lane_ref {
        push_violation(
            violations,
            "recovery_ladder.quarantine_lane_mismatch",
            &scenario.scenario_id,
            "quarantine lane_ref must match the target lane_ref",
        );
    }
    if quarantine.expires_at.trim().is_empty() || quarantine.is_expired_at(&scenario.captured_at) {
        push_violation(
            violations,
            "recovery_ladder.quarantine_expired",
            &scenario.scenario_id,
            "expired or missing quarantine expiry blocks the decision",
        );
    }
    if quarantine.restore_conditions.is_empty() {
        push_violation(
            violations,
            "recovery_ladder.quarantine_restore_conditions_missing",
            &scenario.scenario_id,
            "quarantine restore conditions must be explicit",
        );
    }
    if quarantine.clear_action.action_class != RecoveryActionClass::ClearQuarantine
        || quarantine.clear_action.action_id.trim().is_empty()
    {
        push_violation(
            violations,
            "recovery_ladder.quarantine_clear_action_missing",
            &scenario.scenario_id,
            "quarantine must define an explicit clear action",
        );
    }
    if quarantine.reenable_action.action_class != RecoveryActionClass::ReenableQuarantinedLane
        || quarantine.reenable_action.action_id.trim().is_empty()
    {
        push_violation(
            violations,
            "recovery_ladder.quarantine_reenable_action_missing",
            &scenario.scenario_id,
            "quarantine must define an explicit re-enable action",
        );
    }
    if quarantine.evidence_refs.is_empty() {
        push_violation(
            violations,
            "recovery_ladder.quarantine_evidence_missing",
            &scenario.scenario_id,
            "quarantine must cite evidence refs that justify it",
        );
    }
}

fn validate_open_without_restore(
    scenario: &RecoveryLadderScenario,
    violations: &mut Vec<RecoveryLadderViolation>,
) {
    if scenario.target.target_kind != RecoveryTargetKind::WorkspaceSession {
        push_violation(
            violations,
            "recovery_ladder.open_without_restore_target_not_workspace",
            &scenario.scenario_id,
            "open without restore must target a workspace session",
        );
    }
    require_change(
        scenario,
        violations,
        RecoveryChangeClass::DisableRestoreReplay,
    );
    if !scenario
        .mutation
        .preserved_state_classes
        .contains(&RecoveryStateClass::SessionRestoreStore)
    {
        push_violation(
            violations,
            "recovery_ladder.session_restore_store_must_be_preserved",
            &scenario.scenario_id,
            "open without restore must preserve the restore store",
        );
    }
    if !scenario
        .mutation
        .disabled_capability_classes
        .contains(&RecoveryCapabilityClass::SessionRestoreAutoReopen)
    {
        push_violation(
            violations,
            "recovery_ladder.restore_auto_reopen_not_disabled",
            &scenario.scenario_id,
            "open without restore must disable restore auto-reopen",
        );
    }
}

fn validate_cache_index_repair(
    scenario: &RecoveryLadderScenario,
    violations: &mut Vec<RecoveryLadderViolation>,
) {
    if scenario.target.target_kind != RecoveryTargetKind::CacheIndexLane {
        push_violation(
            violations,
            "recovery_ladder.cache_index_target_not_cache_lane",
            &scenario.scenario_id,
            "cache/index repair must target a cache or index lane",
        );
    }
    for required in [
        RecoveryChangeClass::DisposeDisposableIndexShards,
        RecoveryChangeClass::ScheduleIndexRebuild,
    ] {
        require_change(scenario, violations, required);
    }
    if !scenario
        .mutation
        .disposable_state_classes
        .contains(&RecoveryStateClass::DisposableCacheIndex)
    {
        push_violation(
            violations,
            "recovery_ladder.cache_index_disposable_scope_missing",
            &scenario.scenario_id,
            "cache/index repair must limit mutation to disposable cache/index state",
        );
    }
}

fn require_change(
    scenario: &RecoveryLadderScenario,
    violations: &mut Vec<RecoveryLadderViolation>,
    change: RecoveryChangeClass,
) {
    if !scenario.mutation.changes.contains(&change) {
        push_violation(
            violations,
            "recovery_ladder.required_change_missing",
            &scenario.scenario_id,
            format!("rung is missing required change {change:?}"),
        );
    }
}

fn state_for_rung(
    rung: RecoveryRungClass,
) -> (RecoveryLadderStateClass, RecoveryVisibleStateClass) {
    match rung {
        RecoveryRungClass::SafeMode => (
            RecoveryLadderStateClass::SafeMode,
            RecoveryVisibleStateClass::Degraded,
        ),
        RecoveryRungClass::RuntimeExtensionQuarantine => (
            RecoveryLadderStateClass::RuntimeOrExtensionQuarantined,
            RecoveryVisibleStateClass::Quarantined,
        ),
        RecoveryRungClass::OpenWithoutRestore => (
            RecoveryLadderStateClass::OpenedWithoutRestore,
            RecoveryVisibleStateClass::RecoveredWithLimits,
        ),
        RecoveryRungClass::CacheIndexRepair => (
            RecoveryLadderStateClass::CacheIndexRepairApplied,
            RecoveryVisibleStateClass::Applying,
        ),
    }
}

fn push_violation(
    violations: &mut Vec<RecoveryLadderViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(RecoveryLadderViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

fn sanitize_ref(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
