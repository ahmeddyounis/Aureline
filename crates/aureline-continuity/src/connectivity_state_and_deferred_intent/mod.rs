//! Connectivity state, deferred-intent admission, and reconnect reconciliation.
//!
//! The model is intentionally small enough for all networked command producers
//! to use. A command without complete queueability metadata receives a typed
//! block reason; a queued item can replay only after target, auth, policy,
//! entitlement, version, service-family scope, and context hash are revalidated.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for connectivity continuity records.
pub const CONNECTIVITY_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Shared continuity contract reference.
pub const CONNECTIVITY_CONTINUITY_SHARED_CONTRACT_REF: &str =
    "continuity:connectivity-state-deferred-intent:v1";

/// JSON schema reference for the continuity page.
pub const CONNECTIVITY_CONTINUITY_SCHEMA_REF: &str =
    "schemas/continuity/deferred-intent-and-reconciliation.schema.json";

/// Documentation reference for the continuity contract.
pub const CONNECTIVITY_CONTINUITY_DOC_REF: &str =
    "docs/continuity/m4/connectivity-state-and-deferred-intent.md";

/// Evidence artifact reference for the continuity contract.
pub const CONNECTIVITY_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/continuity/m4/connectivity-state-and-deferred-intent.md";

/// Stable record-kind tag for [`ConnectivityContinuityPage`].
pub const CONNECTIVITY_CONTINUITY_PAGE_RECORD_KIND: &str = "connectivity_continuity_page_record";

/// Stable record-kind tag for [`NetworkCommandDeclaration`].
pub const NETWORK_COMMAND_DECLARATION_RECORD_KIND: &str = "network_command_declaration_record";

/// Stable record-kind tag for [`DeferredIntent`].
pub const DEFERRED_INTENT_RECORD_KIND: &str = "deferred_intent_record";

/// Stable record-kind tag for [`ReconciliationReviewSheet`].
pub const RECONCILIATION_REVIEW_SHEET_RECORD_KIND: &str = "reconciliation_review_sheet_record";

/// Stable record-kind tag for [`ConnectivityContinuityDefect`].
pub const CONNECTIVITY_CONTINUITY_DEFECT_RECORD_KIND: &str =
    "connectivity_continuity_defect_record";

/// Stable record-kind tag for [`SupportExportPacket`].
pub const SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "connectivity_support_export_packet_record";

/// Connectivity posture shared by shell, CLI, service-health, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectivityState {
    /// Transport, auth, policy, and provider health are current.
    Connected,
    /// The service is reachable with elevated latency, intermittent failures, or backoff.
    Constrained,
    /// Managed endpoints are unreachable while local state and signed caches remain usable.
    OfflineLocalSafe,
    /// Fresh auth, token, or policy is required before managed actions can proceed.
    ReauthRequired,
    /// Connectivity returned but queued or diverged state must be revalidated first.
    ReconciliationPending,
    /// A specific service family is unavailable or disabled.
    ServiceUnavailable,
}

impl ConnectivityState {
    /// Returns true when the state preserves local-core workflows by default.
    pub fn preserves_local_core(self) -> bool {
        matches!(
            self,
            Self::Connected
                | Self::Constrained
                | Self::OfflineLocalSafe
                | Self::ReauthRequired
                | Self::ReconciliationPending
                | Self::ServiceUnavailable
        )
    }
}

/// Service family scoped by a connectivity state or deferred intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFamily {
    /// Provider-linked VCS, issue, CI, review, or artifact services.
    Provider,
    /// Managed workspace control plane or shared service.
    ManagedWorkspace,
    /// Request workspace execution or request-history service.
    RequestWorkspace,
    /// Remote attach, tunnel, or helper control plane.
    Remote,
    /// Collaboration presence, sharing, and authority-control service.
    Collaboration,
    /// Service-health cards, notices, and maintenance windows.
    ServiceHealth,
    /// AI provider or managed model service.
    Ai,
    /// Support, diagnostics, or export service.
    Support,
}

/// User promise shown with a connectivity state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafePromise {
    /// Connectivity state being described.
    pub state: ConnectivityState,
    /// Service family this promise applies to.
    pub service_family: ServiceFamily,
    /// True when local editing remains available.
    pub local_editing_available: bool,
    /// True when local search remains available.
    pub local_search_available: bool,
    /// True when local Git operations except push remain available.
    pub local_git_available: bool,
    /// True when local tasks remain available.
    pub local_tasks_available: bool,
    /// True when cached inspection remains available with freshness labels.
    pub cached_inspection_available: bool,
    /// Label a surface must show for stale or cached state.
    pub stale_label_semantics: StaleLabelSemantics,
    /// Optional deployment profile policy that narrows local-safe behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_policy_ref: Option<String>,
}

/// Offline-read posture for a networked command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineReadClass {
    /// No network or managed authority is needed.
    PureLocalCore,
    /// Cached or last-known-good inspection is allowed with freshness labels.
    CachedReadInspect,
    /// Live reachability or fresh auth is required.
    LiveRequired,
}

/// Queueability posture for a networked command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueabilityClass {
    /// The command does not need queueing because it executes locally.
    NotNeededLocal,
    /// The command may queue only as an explicit, idempotent, reviewed intent.
    ExplicitIdempotentReviewable,
    /// The command may refresh cached data later.
    ReadOnlyRefresh,
    /// The command may queue only when artifact class and policy explicitly allow it.
    PolicyBoundBackground,
    /// The command is forbidden from queueing.
    NeverQueue,
}

/// Replay-safety class for command families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySafetyClass {
    /// Bounded and idempotent once all lineage matches.
    IdempotentBounded,
    /// Read-only refresh with no external mutation.
    ReadOnlyRefresh,
    /// Local-only command.
    LocalOnly,
    /// Non-idempotent or target-sensitive command that must be rerun manually.
    ManualRerunRequired,
}

/// Shape required for idempotency keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdempotencyKeyShape {
    /// True when the key includes a stable command id.
    pub includes_command_id: bool,
    /// True when the key includes target identity.
    pub includes_target_identity: bool,
    /// True when the key includes actor identity.
    pub includes_actor_identity: bool,
    /// True when the key includes policy epoch.
    pub includes_policy_epoch: bool,
    /// True when the key includes context hash.
    pub includes_context_hash: bool,
    /// Human-readable key description safe for support export.
    pub support_summary: String,
}

impl IdempotencyKeyShape {
    /// Returns true when the key binds the required replay lineage.
    pub fn is_stable_for_replay(&self) -> bool {
        self.includes_command_id
            && self.includes_target_identity
            && self.includes_actor_identity
            && self.includes_policy_epoch
            && self.includes_context_hash
            && !self.support_summary.trim().is_empty()
    }
}

/// Expiry behavior for deferred intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryPolicy {
    /// ISO-8601 duration or explicit timestamp used by the owner.
    pub expires_after: String,
    /// True when expired items are blocked from replay.
    pub blocks_replay_after_expiry: bool,
    /// Support-safe explanation of the expiry source.
    pub rationale_summary: String,
}

/// Stale-label semantics required on cached or constrained surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleLabelSemantics {
    /// No stale label is required because the data is current.
    Current,
    /// Surface must label data with the last successful refresh time.
    LastKnownGoodTimestamp,
    /// Surface must label data as cached and inspect-only.
    CachedInspectOnly,
    /// Surface must label data as auth or policy stale.
    AuthOrPolicyStale,
    /// Surface must label data as service-family unavailable.
    ServiceFamilyUnavailable,
}

/// Team or subsystem responsible for reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationOwnerClass {
    /// Provider adapter or connected-provider registry.
    Provider,
    /// Managed workspace lifecycle owner.
    ManagedWorkspace,
    /// Request-workspace owner.
    RequestWorkspace,
    /// Remote attach or helper owner.
    Remote,
    /// Service-health continuity owner.
    ServiceHealth,
    /// Support export and diagnostics owner.
    Support,
}

/// Sensitive payload posture for export and inspect surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivePayloadPosture {
    /// No raw sensitive payload is retained.
    NoRawPayload,
    /// Raw payload exists but is withheld from default export.
    RawPayloadWithheldByDefault,
    /// Payload is redacted before export.
    RedactedPreviewOnly,
}

/// Required queueability declaration for a networked command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandQueueabilityDeclaration {
    /// Offline-read posture.
    pub offline_read_class: OfflineReadClass,
    /// Queueability posture.
    pub queueability: QueueabilityClass,
    /// Replay-safety posture.
    pub replay_safety: ReplaySafetyClass,
    /// Idempotency-key shape when queueable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key_shape: Option<IdempotencyKeyShape>,
    /// Expiry policy when queueable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_policy: Option<ExpiryPolicy>,
    /// Stale-label semantics for cached or stale state.
    pub stale_label_semantics: StaleLabelSemantics,
    /// Reconciliation owner for queued or drifted state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_owner: Option<ReconciliationOwnerClass>,
}

impl CommandQueueabilityDeclaration {
    /// Returns true when all metadata required for queue admission is present.
    pub fn has_complete_queue_metadata(&self) -> bool {
        match self.queueability {
            QueueabilityClass::ExplicitIdempotentReviewable => {
                self.replay_safety == ReplaySafetyClass::IdempotentBounded
                    && self
                        .idempotency_key_shape
                        .as_ref()
                        .is_some_and(IdempotencyKeyShape::is_stable_for_replay)
                    && self.expiry_policy.as_ref().is_some_and(|policy| {
                        policy.blocks_replay_after_expiry
                            && !policy.expires_after.trim().is_empty()
                            && !policy.rationale_summary.trim().is_empty()
                    })
                    && self.reconciliation_owner.is_some()
            }
            QueueabilityClass::ReadOnlyRefresh => {
                self.replay_safety == ReplaySafetyClass::ReadOnlyRefresh
                    && self.reconciliation_owner.is_some()
            }
            QueueabilityClass::PolicyBoundBackground => {
                self.expiry_policy.is_some() && self.reconciliation_owner.is_some()
            }
            QueueabilityClass::NotNeededLocal | QueueabilityClass::NeverQueue => true,
        }
    }
}

/// Stable declaration every networked command must publish.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCommandDeclaration {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable command id.
    pub command_id: String,
    /// Service family touched by the command.
    pub service_family: ServiceFamily,
    /// Stable action family.
    pub action_family: String,
    /// Queueability metadata.
    pub queueability: CommandQueueabilityDeclaration,
    /// Redaction-safe reason shown when missing metadata blocks the command.
    pub missing_metadata_block_reason: String,
}

/// Actor identity captured with deferred intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorIdentity {
    /// Stable actor reference.
    pub actor_ref: String,
    /// Actor class such as human, install grant, delegated token, or service.
    pub actor_class: String,
    /// Redaction-safe display label.
    pub display_label: String,
}

/// Target identity captured with deferred intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentity {
    /// Stable target reference.
    pub target_ref: String,
    /// Target class.
    pub target_class: String,
    /// Tenant or organization reference.
    pub tenant_ref: String,
    /// Region reference.
    pub region_ref: String,
    /// Endpoint reference.
    pub endpoint_ref: String,
    /// Version, branch, head, cursor, or object version reference.
    pub version_ref: String,
}

/// Auth scope captured with deferred intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthScopeSnapshot {
    /// Auth subject reference.
    pub subject_ref: String,
    /// Scope names or refs.
    pub scope_refs: Vec<String>,
    /// Session, token, policy, or entitlement epoch.
    pub auth_epoch: String,
}

/// Lifecycle state for a deferred intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredIntentState {
    /// Queued and awaiting reconnect or prerequisites.
    Queued,
    /// Replayed after successful revalidation.
    Replayed,
    /// Cancelled by user or policy.
    Cancelled,
    /// Expired before safe replay.
    Expired,
    /// Dropped because policy or owner refused it.
    Dropped,
    /// Held for reconciliation review because drift was detected.
    ConflictReview,
}

/// Action a surface can expose for deferred intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredIntentAction {
    /// Replay after successful revalidation.
    Replay,
    /// Cancel the item.
    Cancel,
    /// Export redaction-safe lineage.
    Export,
    /// Open explicit reconciliation review.
    OpenReview,
}

/// Deferred-intent object safe to inspect and export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredIntent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable intent id.
    pub intent_id: String,
    /// Command id.
    pub command_id: String,
    /// Service family touched by the intent.
    pub service_family: ServiceFamily,
    /// Target captured at queue time.
    pub target_identity: TargetIdentity,
    /// Actor captured at queue time.
    pub actor: ActorIdentity,
    /// Queue timestamp.
    pub queued_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Policy epoch captured at queue time.
    pub policy_epoch: String,
    /// Auth scope captured at queue time.
    pub auth_scope: AuthScopeSnapshot,
    /// Context hash captured at queue time.
    pub context_hash: String,
    /// Redaction-safe previewed effect summary.
    pub previewed_effect_summary: String,
    /// Current queue lifecycle state.
    pub state: DeferredIntentState,
    /// Actions exposed to shell, CLI, diagnostics, and support surfaces.
    pub available_actions: Vec<DeferredIntentAction>,
    /// Sensitive-payload export posture.
    pub sensitive_payload_posture: SensitivePayloadPosture,
}

/// Admission outcome for a queue attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueAdmissionOutcome {
    /// Intent may be queued.
    Queued,
    /// Command executes locally and does not queue.
    ExecuteLocal,
    /// Cached read may refresh later without mutation.
    CachedReadRefresh,
    /// Command is blocked because metadata is missing.
    BlockedMissingMetadata,
    /// Command is blocked because it is never queueable.
    BlockedNeverQueue,
}

/// Queue admission decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueAdmissionDecision {
    /// Admission outcome.
    pub outcome: QueueAdmissionOutcome,
    /// Redaction-safe reason.
    pub reason: String,
}

/// Revalidation input at replay time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRevalidationInput {
    /// Current target identity.
    pub current_target_identity: TargetIdentity,
    /// Current auth scope.
    pub current_auth_scope: AuthScopeSnapshot,
    /// Current policy epoch.
    pub current_policy_epoch: String,
    /// True when entitlement still admits the action.
    pub entitlement_current: bool,
    /// Current service-family scope.
    pub current_service_family: ServiceFamily,
    /// Current context hash.
    pub current_context_hash: String,
    /// True when the command declaration is still complete.
    pub command_metadata_complete: bool,
    /// True when the intent is expired.
    pub expired: bool,
}

/// Drift dimension that forces reconciliation review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftDimension {
    /// Route or service-family scope drifted.
    Route,
    /// Policy epoch drifted.
    Policy,
    /// Auth scope or auth epoch drifted.
    Auth,
    /// Tenant or organization drifted.
    Tenant,
    /// Region drifted.
    Region,
    /// Endpoint drifted.
    Endpoint,
    /// Target identity drifted.
    Target,
    /// Version, branch, head, cursor, or object version drifted.
    Version,
    /// Entitlement no longer admits the action.
    Entitlement,
    /// Context hash drifted.
    Context,
    /// Command declaration is no longer complete.
    CommandMetadata,
    /// Intent expired before replay.
    Expiry,
}

/// Drift snapshot used by review and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftRevalidationSnapshot {
    /// Drift dimension.
    pub dimension: DriftDimension,
    /// Redaction-safe captured value.
    pub queued_value_summary: String,
    /// Redaction-safe current value.
    pub current_value_summary: String,
}

/// Reconnect replay outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayOutcome {
    /// Replay may proceed.
    ReplayAllowed,
    /// Reconciliation review sheet must open before replay.
    ReconciliationReviewRequired,
    /// Intent expired and must not replay.
    Expired,
    /// Intent is blocked because command metadata is missing.
    BlockedMissingMetadata,
}

/// Replay decision with drift details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationDecision {
    /// Replay outcome.
    pub outcome: ReplayOutcome,
    /// Drift dimensions that blocked replay.
    pub drift_dimensions: Vec<DriftDimension>,
    /// Review snapshots for drifted dimensions.
    pub drift_snapshots: Vec<DriftRevalidationSnapshot>,
    /// Redaction-safe reason.
    pub reason: String,
}

/// Explicit review sheet opened when replay would be unsafe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationReviewSheet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Deferred intent under review.
    pub intent_id: String,
    /// Current connectivity state.
    pub connectivity_state: ConnectivityState,
    /// Drift dimensions requiring review.
    pub drift_dimensions: Vec<DriftDimension>,
    /// Redaction-safe effect summary.
    pub previewed_effect_summary: String,
    /// Allowed reviewer actions.
    pub available_actions: Vec<DeferredIntentAction>,
    /// True when no replay occurs until reviewer action.
    pub replay_blocked_until_review: bool,
}

/// Top-level continuity evidence page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityContinuityPage {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract refs consumed by surfaces.
    pub contract_refs: Vec<String>,
    /// Connectivity states and their local-safe promises.
    pub local_safe_promises: Vec<LocalSafePromise>,
    /// Networked command declarations.
    pub command_declarations: Vec<NetworkCommandDeclaration>,
    /// Deferred intents.
    pub deferred_intents: Vec<DeferredIntent>,
    /// Reconciliation review sheets.
    pub reconciliation_reviews: Vec<ReconciliationReviewSheet>,
    /// Support export packet.
    pub support_export: SupportExportPacket,
}

impl ConnectivityContinuityPage {
    /// Validates that the continuity page covers the stable state and queue rules.
    pub fn validate(&self) -> Vec<ConnectivityContinuityDefect> {
        audit_connectivity_continuity_page(self)
    }

    /// Returns all connectivity states covered by the page.
    pub fn covered_states(&self) -> BTreeSet<ConnectivityState> {
        self.local_safe_promises
            .iter()
            .map(|promise| promise.state)
            .collect()
    }
}

/// Support export packet with redaction-safe continuity lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Connectivity state vocabulary used by the export.
    pub state_vocabulary: Vec<ConnectivityState>,
    /// Deferred intent ids included in the export.
    pub deferred_intent_refs: Vec<String>,
    /// Review sheet ids included in the export.
    pub reconciliation_review_refs: Vec<String>,
    /// True when raw sensitive payloads are excluded by default.
    pub raw_sensitive_payloads_excluded: bool,
}

/// Validation defect kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityContinuityDefectKind {
    /// A required connectivity state is missing.
    MissingConnectivityState,
    /// A local-safe promise incorrectly disables local core without policy ref.
    LocalSafePromiseMissing,
    /// Command metadata is incomplete.
    CommandMetadataIncomplete,
    /// A forbidden action was admitted as queueable.
    ForbiddenActionQueueable,
    /// Deferred intent is missing lineage required for replay.
    DeferredIntentLineageIncomplete,
    /// Review sheet does not block replay.
    ReviewDoesNotBlockReplay,
    /// Support export includes raw sensitive payloads.
    SupportExportUnsafe,
}

/// Validation defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityContinuityDefect {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Defect kind.
    pub defect_kind: ConnectivityContinuityDefectKind,
    /// Affected record reference.
    pub record_ref: String,
    /// Redaction-safe detail.
    pub detail: String,
}

/// Returns the queue admission decision for a command declaration.
pub fn admit_deferred_intent(declaration: &NetworkCommandDeclaration) -> QueueAdmissionDecision {
    if !declaration.queueability.has_complete_queue_metadata() {
        return QueueAdmissionDecision {
            outcome: QueueAdmissionOutcome::BlockedMissingMetadata,
            reason: declaration.missing_metadata_block_reason.clone(),
        };
    }

    match declaration.queueability.queueability {
        QueueabilityClass::NotNeededLocal => QueueAdmissionDecision {
            outcome: QueueAdmissionOutcome::ExecuteLocal,
            reason: "Local-core command executes without queueing.".to_string(),
        },
        QueueabilityClass::ReadOnlyRefresh => QueueAdmissionDecision {
            outcome: QueueAdmissionOutcome::CachedReadRefresh,
            reason: "Cached read/inspect refresh is read-only and may refresh later.".to_string(),
        },
        QueueabilityClass::ExplicitIdempotentReviewable
        | QueueabilityClass::PolicyBoundBackground => QueueAdmissionDecision {
            outcome: QueueAdmissionOutcome::Queued,
            reason: "Explicit bounded intent has complete replay lineage.".to_string(),
        },
        QueueabilityClass::NeverQueue => QueueAdmissionDecision {
            outcome: QueueAdmissionOutcome::BlockedNeverQueue,
            reason: "This command family requires live context and must be rerun manually."
                .to_string(),
        },
    }
}

/// Decides whether a queued intent can replay or must open reconciliation review.
pub fn replay_decision(
    intent: &DeferredIntent,
    revalidation: &ReplayRevalidationInput,
) -> ReconciliationDecision {
    if revalidation.expired {
        return ReconciliationDecision {
            outcome: ReplayOutcome::Expired,
            drift_dimensions: vec![DriftDimension::Expiry],
            drift_snapshots: vec![],
            reason: "Deferred intent expired before replay.".to_string(),
        };
    }
    if !revalidation.command_metadata_complete {
        return ReconciliationDecision {
            outcome: ReplayOutcome::BlockedMissingMetadata,
            drift_dimensions: vec![DriftDimension::CommandMetadata],
            drift_snapshots: vec![],
            reason: "Command declaration is missing replay metadata.".to_string(),
        };
    }

    let mut snapshots = Vec::new();
    push_drift(
        &mut snapshots,
        DriftDimension::Route,
        intent.service_family != revalidation.current_service_family,
        format!("{:?}", intent.service_family),
        format!("{:?}", revalidation.current_service_family),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Policy,
        intent.policy_epoch != revalidation.current_policy_epoch,
        intent.policy_epoch.clone(),
        revalidation.current_policy_epoch.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Auth,
        intent.auth_scope != revalidation.current_auth_scope,
        intent.auth_scope.auth_epoch.clone(),
        revalidation.current_auth_scope.auth_epoch.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Tenant,
        intent.target_identity.tenant_ref != revalidation.current_target_identity.tenant_ref,
        intent.target_identity.tenant_ref.clone(),
        revalidation.current_target_identity.tenant_ref.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Region,
        intent.target_identity.region_ref != revalidation.current_target_identity.region_ref,
        intent.target_identity.region_ref.clone(),
        revalidation.current_target_identity.region_ref.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Endpoint,
        intent.target_identity.endpoint_ref != revalidation.current_target_identity.endpoint_ref,
        intent.target_identity.endpoint_ref.clone(),
        revalidation.current_target_identity.endpoint_ref.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Target,
        intent.target_identity.target_ref != revalidation.current_target_identity.target_ref,
        intent.target_identity.target_ref.clone(),
        revalidation.current_target_identity.target_ref.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Version,
        intent.target_identity.version_ref != revalidation.current_target_identity.version_ref,
        intent.target_identity.version_ref.clone(),
        revalidation.current_target_identity.version_ref.clone(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Entitlement,
        !revalidation.entitlement_current,
        "entitled_at_queue_time".to_string(),
        "entitlement_not_current".to_string(),
    );
    push_drift(
        &mut snapshots,
        DriftDimension::Context,
        intent.context_hash != revalidation.current_context_hash,
        intent.context_hash.clone(),
        revalidation.current_context_hash.clone(),
    );

    if snapshots.is_empty() {
        ReconciliationDecision {
            outcome: ReplayOutcome::ReplayAllowed,
            drift_dimensions: vec![],
            drift_snapshots: vec![],
            reason: "Target, auth, policy, entitlement, version, scope, and context still match."
                .to_string(),
        }
    } else {
        ReconciliationDecision {
            outcome: ReplayOutcome::ReconciliationReviewRequired,
            drift_dimensions: snapshots
                .iter()
                .map(|snapshot| snapshot.dimension)
                .collect(),
            drift_snapshots: snapshots,
            reason: "Replay blocked until reconciliation review resolves drift.".to_string(),
        }
    }
}

fn push_drift(
    snapshots: &mut Vec<DriftRevalidationSnapshot>,
    dimension: DriftDimension,
    drifted: bool,
    queued_value_summary: String,
    current_value_summary: String,
) {
    if drifted {
        snapshots.push(DriftRevalidationSnapshot {
            dimension,
            queued_value_summary,
            current_value_summary,
        });
    }
}

/// Audits a continuity page for required states and replay-safety invariants.
pub fn audit_connectivity_continuity_page(
    page: &ConnectivityContinuityPage,
) -> Vec<ConnectivityContinuityDefect> {
    let mut defects = Vec::new();
    for state in [
        ConnectivityState::Connected,
        ConnectivityState::Constrained,
        ConnectivityState::OfflineLocalSafe,
        ConnectivityState::ReauthRequired,
        ConnectivityState::ReconciliationPending,
        ConnectivityState::ServiceUnavailable,
    ] {
        if !page.covered_states().contains(&state) {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingConnectivityState,
                format!("{state:?}"),
                "Required connectivity state is not represented.",
            ));
        }
    }

    for promise in &page.local_safe_promises {
        let local_core_disabled = !promise.local_editing_available
            || !promise.local_search_available
            || !promise.local_git_available
            || !promise.local_tasks_available
            || !promise.cached_inspection_available;
        if promise.state.preserves_local_core()
            && local_core_disabled
            && promise.deployment_policy_ref.is_none()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::LocalSafePromiseMissing,
                format!("{:?}:{:?}", promise.state, promise.service_family),
                "Local-core workflow was disabled without an explicit deployment policy.",
            ));
        }
    }

    for declaration in &page.command_declarations {
        if !declaration.queueability.has_complete_queue_metadata() {
            defects.push(defect(
                ConnectivityContinuityDefectKind::CommandMetadataIncomplete,
                declaration.command_id.clone(),
                "Networked command is missing queueability, idempotency, expiry, stale-label, or owner metadata.",
            ));
        }
        if declaration.queueability.queueability != QueueabilityClass::NeverQueue
            && matches_forbidden_action_family(&declaration.action_family)
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ForbiddenActionQueueable,
                declaration.command_id.clone(),
                "High-impact action family must fail clearly and may not queue.",
            ));
        }
    }

    for intent in &page.deferred_intents {
        if intent.command_id.trim().is_empty()
            || intent.target_identity.target_ref.trim().is_empty()
            || intent.actor.actor_ref.trim().is_empty()
            || intent.queued_at.trim().is_empty()
            || intent.expires_at.trim().is_empty()
            || intent.policy_epoch.trim().is_empty()
            || intent.auth_scope.scope_refs.is_empty()
            || intent.context_hash.trim().is_empty()
            || intent.previewed_effect_summary.trim().is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::DeferredIntentLineageIncomplete,
                intent.intent_id.clone(),
                "Deferred intent lacks replay lineage required for safe reconciliation.",
            ));
        }
    }

    for review in &page.reconciliation_reviews {
        if !review.replay_blocked_until_review {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReviewDoesNotBlockReplay,
                review.review_id.clone(),
                "Reconciliation review sheet must block replay until explicit review.",
            ));
        }
    }

    if !page.support_export.raw_sensitive_payloads_excluded {
        defects.push(defect(
            ConnectivityContinuityDefectKind::SupportExportUnsafe,
            "support_export".to_string(),
            "Support export must exclude raw sensitive payloads by default.",
        ));
    }

    defects
}

fn matches_forbidden_action_family(action_family: &str) -> bool {
    matches!(
        action_family,
        "terminal_input"
            | "remote_execution"
            | "git_push"
            | "destructive_api_mutation"
            | "publish_deploy"
            | "collaboration_control"
            | "unbounded_ai_job"
    )
}

fn defect(
    defect_kind: ConnectivityContinuityDefectKind,
    record_ref: String,
    detail: &str,
) -> ConnectivityContinuityDefect {
    ConnectivityContinuityDefect {
        record_kind: CONNECTIVITY_CONTINUITY_DEFECT_RECORD_KIND.to_string(),
        defect_kind,
        record_ref,
        detail: detail.to_string(),
    }
}

/// Validates a continuity page and returns true when no defects were found.
pub fn validate_connectivity_continuity_page(page: &ConnectivityContinuityPage) -> bool {
    audit_connectivity_continuity_page(page).is_empty()
}

/// Returns a seeded continuity page covering stable state, queue, review, and export rules.
pub fn seeded_connectivity_continuity_page() -> ConnectivityContinuityPage {
    let states = vec![
        ConnectivityState::Connected,
        ConnectivityState::Constrained,
        ConnectivityState::OfflineLocalSafe,
        ConnectivityState::ReauthRequired,
        ConnectivityState::ReconciliationPending,
        ConnectivityState::ServiceUnavailable,
    ];
    let local_safe_promises = states
        .iter()
        .map(|state| LocalSafePromise {
            state: *state,
            service_family: if *state == ConnectivityState::ServiceUnavailable {
                ServiceFamily::Provider
            } else {
                ServiceFamily::ManagedWorkspace
            },
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: match state {
                ConnectivityState::Connected => StaleLabelSemantics::Current,
                ConnectivityState::Constrained => StaleLabelSemantics::LastKnownGoodTimestamp,
                ConnectivityState::OfflineLocalSafe => StaleLabelSemantics::CachedInspectOnly,
                ConnectivityState::ReauthRequired => StaleLabelSemantics::AuthOrPolicyStale,
                ConnectivityState::ReconciliationPending => {
                    StaleLabelSemantics::LastKnownGoodTimestamp
                }
                ConnectivityState::ServiceUnavailable => {
                    StaleLabelSemantics::ServiceFamilyUnavailable
                }
            },
            deployment_policy_ref: None,
        })
        .collect::<Vec<_>>();

    let replayable_shape = IdempotencyKeyShape {
        includes_command_id: true,
        includes_target_identity: true,
        includes_actor_identity: true,
        includes_policy_epoch: true,
        includes_context_hash: true,
        support_summary: "command+target+actor+policy+context".to_string(),
    };
    let expiry_policy = ExpiryPolicy {
        expires_after: "PT72H".to_string(),
        blocks_replay_after_expiry: true,
        rationale_summary: "Provider review drafts expire after three days unless refreshed."
            .to_string(),
    };

    let queueable = NetworkCommandDeclaration {
        record_kind: NETWORK_COMMAND_DECLARATION_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        command_id: "cmd:provider.review_comment.save_draft".to_string(),
        service_family: ServiceFamily::Provider,
        action_family: "explicit_idempotent_managed_write".to_string(),
        queueability: CommandQueueabilityDeclaration {
            offline_read_class: OfflineReadClass::LiveRequired,
            queueability: QueueabilityClass::ExplicitIdempotentReviewable,
            replay_safety: ReplaySafetyClass::IdempotentBounded,
            idempotency_key_shape: Some(replayable_shape),
            expiry_policy: Some(expiry_policy),
            stale_label_semantics: StaleLabelSemantics::CachedInspectOnly,
            reconciliation_owner: Some(ReconciliationOwnerClass::Provider),
        },
        missing_metadata_block_reason: "Deferred provider writes require idempotency, expiry, stale-label, and reconciliation owner metadata.".to_string(),
    };
    let forbidden = NetworkCommandDeclaration {
        record_kind: NETWORK_COMMAND_DECLARATION_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        command_id: "cmd:git.push".to_string(),
        service_family: ServiceFamily::Provider,
        action_family: "git_push".to_string(),
        queueability: CommandQueueabilityDeclaration {
            offline_read_class: OfflineReadClass::LiveRequired,
            queueability: QueueabilityClass::NeverQueue,
            replay_safety: ReplaySafetyClass::ManualRerunRequired,
            idempotency_key_shape: None,
            expiry_policy: None,
            stale_label_semantics: StaleLabelSemantics::ServiceFamilyUnavailable,
            reconciliation_owner: None,
        },
        missing_metadata_block_reason:
            "Git push requires live target context and fresh auth; rerun manually after reconnect."
                .to_string(),
    };
    let target_identity = TargetIdentity {
        target_ref: "provider:github:pr:42:comment-thread:7".to_string(),
        target_class: "pull_request_comment_thread".to_string(),
        tenant_ref: "org:acme".to_string(),
        region_ref: "region:us".to_string(),
        endpoint_ref: "endpoint:github.com".to_string(),
        version_ref: "head:abc123".to_string(),
    };
    let actor = ActorIdentity {
        actor_ref: "actor:user:alice".to_string(),
        actor_class: "human_account".to_string(),
        display_label: "Alice".to_string(),
    };
    let auth_scope = AuthScopeSnapshot {
        subject_ref: "subject:alice".to_string(),
        scope_refs: vec!["pull_request:write".to_string()],
        auth_epoch: "auth-epoch-17".to_string(),
    };
    let intent = DeferredIntent {
        record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        intent_id: "intent:provider-review-comment-1".to_string(),
        command_id: queueable.command_id.clone(),
        service_family: ServiceFamily::Provider,
        target_identity,
        actor,
        queued_at: "2026-06-04T18:00:00Z".to_string(),
        expires_at: "2026-06-07T18:00:00Z".to_string(),
        policy_epoch: "policy-epoch-42".to_string(),
        auth_scope,
        context_hash: "sha256:queued-context".to_string(),
        previewed_effect_summary: "Would save a provider review comment draft on PR 42."
            .to_string(),
        state: DeferredIntentState::Queued,
        available_actions: vec![
            DeferredIntentAction::Replay,
            DeferredIntentAction::Cancel,
            DeferredIntentAction::Export,
        ],
        sensitive_payload_posture: SensitivePayloadPosture::RedactedPreviewOnly,
    };
    let review = ReconciliationReviewSheet {
        record_kind: RECONCILIATION_REVIEW_SHEET_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        review_id: "review:provider-review-comment-1".to_string(),
        intent_id: intent.intent_id.clone(),
        connectivity_state: ConnectivityState::ReconciliationPending,
        drift_dimensions: vec![DriftDimension::Target, DriftDimension::Policy],
        previewed_effect_summary: intent.previewed_effect_summary.clone(),
        available_actions: vec![
            DeferredIntentAction::OpenReview,
            DeferredIntentAction::Cancel,
            DeferredIntentAction::Export,
        ],
        replay_blocked_until_review: true,
    };

    ConnectivityContinuityPage {
        record_kind: CONNECTIVITY_CONTINUITY_PAGE_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        contract_refs: vec![
            CONNECTIVITY_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            CONNECTIVITY_CONTINUITY_SCHEMA_REF.to_string(),
            CONNECTIVITY_CONTINUITY_DOC_REF.to_string(),
        ],
        local_safe_promises,
        command_declarations: vec![queueable, forbidden],
        deferred_intents: vec![intent],
        reconciliation_reviews: vec![review],
        support_export: SupportExportPacket {
            record_kind: SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            state_vocabulary: states,
            deferred_intent_refs: vec!["intent:provider-review-comment-1".to_string()],
            reconciliation_review_refs: vec!["review:provider-review-comment-1".to_string()],
            raw_sensitive_payloads_excluded: true,
        },
    }
}
