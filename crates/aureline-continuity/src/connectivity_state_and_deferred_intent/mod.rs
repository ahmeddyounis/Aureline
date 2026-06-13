//! Connectivity state, deferred-intent admission, and reconnect reconciliation.
//!
//! The model is intentionally small enough for all networked command producers
//! to use. A command without complete queueability metadata receives a typed
//! block reason; a queued item can replay only after target, auth, policy,
//! entitlement, version, data, service-family scope, and context hash are
//! revalidated. M5 surfaces also need one shared user-facing disclosure model,
//! so the same page carries connectivity badges/cards, deferred-intent outbox
//! rows, idempotency-key receipts, reconciliation packets, and support-export
//! manifests that make no-invisible-replay auditable across managed, provider,
//! and remote actions.

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

/// Stable record-kind tag for [`ConnectivityBadge`].
pub const CONNECTIVITY_BADGE_RECORD_KIND: &str = "connectivity_badge_record";

/// Stable record-kind tag for [`ConnectivityStateCard`].
pub const CONNECTIVITY_CARD_RECORD_KIND: &str = "connectivity_state_card_record";

/// Stable record-kind tag for [`NetworkCommandDeclaration`].
pub const NETWORK_COMMAND_DECLARATION_RECORD_KIND: &str = "network_command_declaration_record";

/// Stable record-kind tag for [`DeferredIntent`].
pub const DEFERRED_INTENT_RECORD_KIND: &str = "deferred_intent_record";

/// Stable record-kind tag for [`IdempotencyKeyReceipt`].
pub const IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND: &str = "idempotency_key_receipt_record";

/// Stable record-kind tag for [`ReconciliationReviewSheet`].
pub const RECONCILIATION_REVIEW_SHEET_RECORD_KIND: &str = "reconciliation_review_sheet_record";

/// Stable record-kind tag for [`ReconciliationPacket`].
pub const RECONCILIATION_PACKET_RECORD_KIND: &str = "reconciliation_packet_record";

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

    /// Returns the reviewer-facing state label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Connected => "Connected",
            Self::Constrained => "Constrained",
            Self::OfflineLocalSafe => "Offline local-safe",
            Self::ReauthRequired => "Reauth required",
            Self::ReconciliationPending => "Reconciliation pending",
            Self::ServiceUnavailable => "Service unavailable",
        }
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

impl ServiceFamily {
    /// Returns the reviewer-facing family label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Provider => "Provider-linked actions",
            Self::ManagedWorkspace => "Managed workspace",
            Self::RequestWorkspace => "Request workspace",
            Self::Remote => "Remote actions",
            Self::Collaboration => "Collaboration",
            Self::ServiceHealth => "Service health",
            Self::Ai => "AI gateway",
            Self::Support => "Support export",
        }
    }
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

/// Badge row shown in connectivity strips, action chrome, and provider cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityBadge {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable badge id.
    pub badge_id: String,
    /// Service family named by the badge.
    pub service_family: ServiceFamily,
    /// Connectivity state named by the badge.
    pub state: ConnectivityState,
    /// Stable family label surfaces show beside the state.
    pub affected_family_label: String,
    /// Promise summary shown in the strip.
    pub promise_summary: String,
    /// Primary action label.
    pub action_label: String,
    /// Opaque action ref safe for support/export.
    pub action_ref: String,
}

/// Action surfaced on a connectivity card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityCardAction {
    /// Stable action id.
    pub action_id: String,
    /// Reviewer-facing action label.
    pub label: String,
    /// Opaque action ref safe for support/export.
    pub action_ref: String,
}

/// Expanded connectivity card shown in detail views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityStateCard {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Service family named by the card.
    pub service_family: ServiceFamily,
    /// Connectivity state named by the card.
    pub state: ConnectivityState,
    /// Headline label for the card.
    pub title_label: String,
    /// Summary explaining the boundary.
    pub summary_label: String,
    /// Promise carried forward into detail surfaces.
    pub promise_label: String,
    /// Productive local-safe capabilities that remain.
    pub what_still_works: Vec<String>,
    /// Suggested actions.
    pub suggested_actions: Vec<ConnectivityCardAction>,
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

/// Exact replay prerequisite shown on outbox rows and review sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPrerequisiteClass {
    /// Connectivity for the service family must be restored.
    ConnectivityRestored,
    /// The service family must be healthy for live replay.
    ServiceFamilyHealthy,
    /// The original target identity must still match.
    TargetIdentityCurrent,
    /// The original auth scope must still match.
    AuthScopeCurrent,
    /// The original policy epoch must still match.
    PolicyEpochCurrent,
    /// The original context hash must still match.
    ContextHashCurrent,
    /// The original data fingerprint must still match.
    DataFingerprintCurrent,
}

/// Current state of a replay prerequisite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPrerequisiteState {
    /// Recorded at queue time and waiting for reconnect revalidation.
    PendingRevalidation,
    /// Revalidated successfully.
    Satisfied,
    /// Revalidation failed and review is required.
    FailedRequiresReview,
}

/// One replay prerequisite row on a deferred intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPrerequisite {
    /// Closed prerequisite class.
    pub prerequisite_class: ReplayPrerequisiteClass,
    /// Current state of the prerequisite.
    pub state: ReplayPrerequisiteState,
    /// Reviewable requirement summary.
    pub requirement_summary: String,
    /// True when the queued and current values must match exactly.
    pub exact_match_required: bool,
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
    /// Structured data fingerprint captured at queue time.
    pub data_fingerprint: String,
    /// Opaque idempotency-key ref preserved for receipts and dedupe.
    pub idempotency_key_ref: String,
    /// Redaction-safe previewed effect summary.
    pub previewed_effect_summary: String,
    /// Exact replay prerequisites that must pass before replay.
    pub replay_prerequisites: Vec<ReplayPrerequisite>,
    /// Current queue lifecycle state.
    pub state: DeferredIntentState,
    /// Actions exposed to shell, CLI, diagnostics, and support surfaces.
    pub available_actions: Vec<DeferredIntentAction>,
    /// Sensitive-payload export posture.
    pub sensitive_payload_posture: SensitivePayloadPosture,
}

/// Receipt outcome emitted for a deferred intent's idempotency key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdempotencyKeyReceiptOutcome {
    /// The key was minted and the intent is queued locally.
    QueuedForOutbox,
    /// Replay completed under the same key.
    ReplayedUnderSameKey,
    /// Provider or owner treated the key as already applied.
    DedupedAsAlreadyApplied,
    /// Replay stayed blocked pending review or reauth.
    BlockedPendingReview,
    /// The intent was discarded without replay.
    DiscardedWithoutReplay,
}

/// Receipt proving that replay visibility stayed tied to an idempotency key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdempotencyKeyReceipt {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable receipt id.
    pub receipt_id: String,
    /// Deferred intent this receipt belongs to.
    pub intent_id: String,
    /// Opaque idempotency-key ref.
    pub idempotency_key_ref: String,
    /// Opaque dedupe-scope ref.
    pub dedupe_scope_ref: String,
    /// Receipt outcome.
    pub outcome: IdempotencyKeyReceiptOutcome,
    /// Timestamp the receipt was recorded.
    pub recorded_at: String,
    /// Reviewable summary.
    pub summary_label: String,
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
    /// Current structured data fingerprint.
    pub current_data_fingerprint: String,
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
    /// Structured data fingerprint drifted.
    Data,
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
    /// Original effect summary carried forward from queue time.
    pub previewed_effect_summary: String,
    /// Current drift summary shown to the user.
    pub current_drift_summary: String,
    /// Policy or auth delta summary shown to the user.
    pub policy_or_auth_delta_summary: String,
    /// Allowed reviewer actions.
    pub available_actions: Vec<DeferredIntentAction>,
    /// True when no replay occurs until reviewer action.
    pub replay_blocked_until_review: bool,
}

/// Final typed disposition captured after reconciliation or explicit review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationDisposition {
    /// The queued effect replayed exactly once under the preserved key.
    Replayed,
    /// Replay is blocked pending review, reauth, or drift repair.
    Blocked,
    /// Replay is narrowed to a safer local or inspect-only posture.
    Narrowed,
    /// Replay was discarded and will not run.
    Discarded,
}

/// User-visible packet emitted for every replayed, blocked, narrowed, or discarded outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deferred intent under reconciliation.
    pub intent_id: String,
    /// Service family resolved by the packet.
    pub service_family: ServiceFamily,
    /// Connectivity state active when the packet was minted.
    pub connectivity_state: ConnectivityState,
    /// Final disposition.
    pub disposition: ReconciliationDisposition,
    /// Drift dimensions that influenced the outcome.
    pub drift_dimensions: Vec<DriftDimension>,
    /// Detailed drift snapshots safe for export.
    pub drift_snapshots: Vec<DriftRevalidationSnapshot>,
    /// Reviewable outcome summary.
    pub summary_label: String,
    /// Matching idempotency receipt when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_receipt_ref: Option<String>,
    /// Matching review sheet when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_review_ref: Option<String>,
    /// Opaque support-manifest ref carried into export surfaces.
    pub support_manifest_ref: String,
    /// True when the outcome is disclosed explicitly and never replayed invisibly.
    pub outcome_disclosed_to_user: bool,
}

/// Support-export row narrating one deferred intent's current or final state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportOutcomeRow {
    /// Stable row id.
    pub row_id: String,
    /// Deferred intent named by the row.
    pub intent_id: String,
    /// Service family named by the row.
    pub service_family: ServiceFamily,
    /// Intent state rendered by the row.
    pub intent_state: DeferredIntentState,
    /// Final disposition when reconciliation already ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_disposition: Option<ReconciliationDisposition>,
    /// Reviewable summary.
    pub summary_label: String,
    /// Actor label carried into support/export.
    pub actor_label: String,
    /// Stable target ref carried into support/export.
    pub target_ref: String,
    /// Matching receipt ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_receipt_ref: Option<String>,
    /// Matching reconciliation packet ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_packet_ref: Option<String>,
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
    /// Connectivity strip badges.
    pub connectivity_badges: Vec<ConnectivityBadge>,
    /// Connectivity detail cards.
    pub connectivity_cards: Vec<ConnectivityStateCard>,
    /// Networked command declarations.
    pub command_declarations: Vec<NetworkCommandDeclaration>,
    /// Deferred intents.
    pub deferred_intents: Vec<DeferredIntent>,
    /// Idempotency-key receipts for queued or reconciled intents.
    pub idempotency_receipts: Vec<IdempotencyKeyReceipt>,
    /// Reconciliation review sheets.
    pub reconciliation_reviews: Vec<ReconciliationReviewSheet>,
    /// Reconciliation packets narrating replayed, blocked, narrowed, or discarded outcomes.
    pub reconciliation_packets: Vec<ReconciliationPacket>,
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
    /// Connectivity badge ids included in the export.
    pub badge_refs: Vec<String>,
    /// Deferred intent ids included in the export.
    pub deferred_intent_refs: Vec<String>,
    /// Review sheet ids included in the export.
    pub reconciliation_review_refs: Vec<String>,
    /// Reconciliation packet ids included in the export.
    pub reconciliation_packet_refs: Vec<String>,
    /// Idempotency receipt ids included in the export.
    pub idempotency_receipt_refs: Vec<String>,
    /// Export rows narrating every queued or reconciled item.
    pub outcome_rows: Vec<SupportExportOutcomeRow>,
    /// True when raw sensitive payloads are excluded by default.
    pub raw_sensitive_payloads_excluded: bool,
}

/// Validation defect kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityContinuityDefectKind {
    /// A required connectivity state is missing.
    MissingConnectivityState,
    /// A required badge or card presentation is missing.
    MissingStatePresentation,
    /// Managed, provider, or remote service-family coverage is missing.
    MissingManagedProviderRemoteCoverage,
    /// A local-safe promise incorrectly disables local core without policy ref.
    LocalSafePromiseMissing,
    /// Command metadata is incomplete.
    CommandMetadataIncomplete,
    /// A forbidden action was admitted as queueable.
    ForbiddenActionQueueable,
    /// Deferred intent is missing lineage required for replay.
    DeferredIntentLineageIncomplete,
    /// Deferred intent is missing exact replay prerequisites.
    ReplayPrerequisitesMissing,
    /// Reconciliation review truth is missing or does not block replay.
    ReviewDoesNotBlockReplay,
    /// An idempotency receipt required for visibility is missing.
    IdempotencyReceiptMissing,
    /// A reconciliation packet required for outcome disclosure is missing.
    ReconciliationPacketMissing,
    /// A replay, block, narrowing, or discard outcome is hidden.
    HiddenReplayOutcome,
    /// Support export manifest is incomplete.
    SupportExportManifestIncomplete,
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
    push_drift(
        &mut snapshots,
        DriftDimension::Data,
        intent.data_fingerprint != revalidation.current_data_fingerprint,
        intent.data_fingerprint.clone(),
        revalidation.current_data_fingerprint.clone(),
    );

    if snapshots.is_empty() {
        ReconciliationDecision {
            outcome: ReplayOutcome::ReplayAllowed,
            drift_dimensions: vec![],
            drift_snapshots: vec![],
            reason:
                "Target, auth, policy, entitlement, version, data, scope, and context still match."
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
    let covered_states = page.covered_states();
    let badge_states = page
        .connectivity_badges
        .iter()
        .map(|badge| badge.state)
        .collect::<BTreeSet<_>>();
    let card_states = page
        .connectivity_cards
        .iter()
        .map(|card| card.state)
        .collect::<BTreeSet<_>>();

    for state in required_states() {
        if !covered_states.contains(&state) {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingConnectivityState,
                format!("{state:?}"),
                "Required connectivity state is not represented.",
            ));
        }
        if !badge_states.contains(&state) || !card_states.contains(&state) {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingStatePresentation,
                format!("{state:?}"),
                "Every required connectivity state must render both a badge and a detail card.",
            ));
        }
    }

    let family_coverage = page
        .connectivity_badges
        .iter()
        .map(|badge| badge.service_family)
        .chain(
            page.connectivity_cards
                .iter()
                .map(|card| card.service_family),
        )
        .chain(
            page.command_declarations
                .iter()
                .map(|declaration| declaration.service_family),
        )
        .collect::<BTreeSet<_>>();
    for family in [
        ServiceFamily::ManagedWorkspace,
        ServiceFamily::Provider,
        ServiceFamily::Remote,
    ] {
        if !family_coverage.contains(&family) {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingManagedProviderRemoteCoverage,
                format!("{family:?}"),
                "Managed, provider, and remote actions must all participate in M5 connectivity disclosure.",
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

    for badge in &page.connectivity_badges {
        if badge.badge_id.trim().is_empty()
            || badge.promise_summary.trim().is_empty()
            || badge.action_label.trim().is_empty()
            || badge.action_ref.trim().is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingStatePresentation,
                badge.badge_id.clone(),
                "Connectivity badges must name a promise and one explicit recovery action.",
            ));
        }
    }

    for card in &page.connectivity_cards {
        if card.card_id.trim().is_empty()
            || card.title_label.trim().is_empty()
            || card.summary_label.trim().is_empty()
            || card.promise_label.trim().is_empty()
            || card.what_still_works.is_empty()
            || card.suggested_actions.is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::MissingStatePresentation,
                card.card_id.clone(),
                "Connectivity cards must explain what still works and offer explicit next actions.",
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
            || intent.data_fingerprint.trim().is_empty()
            || intent.idempotency_key_ref.trim().is_empty()
            || intent.previewed_effect_summary.trim().is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::DeferredIntentLineageIncomplete,
                intent.intent_id.clone(),
                "Deferred intent lacks replay lineage required for safe reconciliation.",
            ));
        }
        if intent.replay_prerequisites.is_empty() {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReplayPrerequisitesMissing,
                intent.intent_id.clone(),
                "Deferred intent must retain explicit replay prerequisites.",
            ));
        }
        if intent.state != DeferredIntentState::Cancelled
            && !page
                .idempotency_receipts
                .iter()
                .any(|receipt| receipt.intent_id == intent.intent_id)
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::IdempotencyReceiptMissing,
                intent.intent_id.clone(),
                "Queued or reconciled intents must preserve an idempotency-key receipt.",
            ));
        }
        if matches!(
            intent.state,
            DeferredIntentState::Replayed
                | DeferredIntentState::Expired
                | DeferredIntentState::Dropped
        ) && !page
            .reconciliation_packets
            .iter()
            .any(|packet| packet.intent_id == intent.intent_id)
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReconciliationPacketMissing,
                intent.intent_id.clone(),
                "Replayed, expired, or dropped intents must carry a reconciliation packet.",
            ));
        }
        if intent.state == DeferredIntentState::ConflictReview
            && !page
                .reconciliation_reviews
                .iter()
                .any(|review| review.intent_id == intent.intent_id)
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReviewDoesNotBlockReplay,
                intent.intent_id.clone(),
                "Conflict-review intents must carry an explicit reconciliation review sheet.",
            ));
        }
    }

    for review in &page.reconciliation_reviews {
        if !review.replay_blocked_until_review
            || review.current_drift_summary.trim().is_empty()
            || review.policy_or_auth_delta_summary.trim().is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReviewDoesNotBlockReplay,
                review.review_id.clone(),
                "Reconciliation review sheet must block replay and explain drift and policy/auth deltas.",
            ));
        }
    }

    for packet in &page.reconciliation_packets {
        if !packet.outcome_disclosed_to_user {
            defects.push(defect(
                ConnectivityContinuityDefectKind::HiddenReplayOutcome,
                packet.packet_id.clone(),
                "Replay, block, narrowing, and discard outcomes must stay visible to the user.",
            ));
        }
        if packet.summary_label.trim().is_empty() || packet.support_manifest_ref.trim().is_empty() {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReconciliationPacketMissing,
                packet.packet_id.clone(),
                "Reconciliation packets must carry a summary and support-manifest ref.",
            ));
        }
        if matches!(
            packet.disposition,
            ReconciliationDisposition::Blocked | ReconciliationDisposition::Narrowed
        ) && packet.reconciliation_review_ref.is_none()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::ReviewDoesNotBlockReplay,
                packet.packet_id.clone(),
                "Blocked or narrowed packets must cite the reconciliation review that kept replay visible.",
            ));
        }
        if packet.disposition == ReconciliationDisposition::Replayed
            && packet.idempotency_receipt_ref.is_none()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::IdempotencyReceiptMissing,
                packet.packet_id.clone(),
                "Replayed packets must cite the matching idempotency receipt.",
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

    if page.support_export.outcome_rows.is_empty() {
        defects.push(defect(
            ConnectivityContinuityDefectKind::SupportExportManifestIncomplete,
            "support_export".to_string(),
            "Support export must narrate queued or reconciled deferred intents.",
        ));
    }

    for row in &page.support_export.outcome_rows {
        if row.summary_label.trim().is_empty()
            || row.actor_label.trim().is_empty()
            || row.target_ref.trim().is_empty()
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::SupportExportManifestIncomplete,
                row.row_id.clone(),
                "Support export rows must preserve actor, target, and summary truth.",
            ));
        }
        if !page
            .support_export
            .deferred_intent_refs
            .iter()
            .any(|intent_id| intent_id == &row.intent_id)
        {
            defects.push(defect(
                ConnectivityContinuityDefectKind::SupportExportManifestIncomplete,
                row.row_id.clone(),
                "Support export row must resolve a deferred intent ref on the manifest.",
            ));
        }
        if let Some(receipt_ref) = &row.idempotency_receipt_ref {
            if !page
                .support_export
                .idempotency_receipt_refs
                .iter()
                .any(|item| item == receipt_ref)
            {
                defects.push(defect(
                    ConnectivityContinuityDefectKind::SupportExportManifestIncomplete,
                    row.row_id.clone(),
                    "Support export row cites an idempotency receipt missing from the manifest.",
                ));
            }
        }
        if let Some(packet_ref) = &row.reconciliation_packet_ref {
            if !page
                .support_export
                .reconciliation_packet_refs
                .iter()
                .any(|item| item == packet_ref)
            {
                defects.push(defect(
                    ConnectivityContinuityDefectKind::SupportExportManifestIncomplete,
                    row.row_id.clone(),
                    "Support export row cites a reconciliation packet missing from the manifest.",
                ));
            }
        }
    }

    defects
}

fn required_states() -> [ConnectivityState; 6] {
    [
        ConnectivityState::Connected,
        ConnectivityState::Constrained,
        ConnectivityState::OfflineLocalSafe,
        ConnectivityState::ReauthRequired,
        ConnectivityState::ReconciliationPending,
        ConnectivityState::ServiceUnavailable,
    ]
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

/// Returns a seeded continuity page covering stable state, queue, review, receipt, and export rules.
pub fn seeded_connectivity_continuity_page() -> ConnectivityContinuityPage {
    let states = required_states().to_vec();
    let local_safe_promises = vec![
        LocalSafePromise {
            state: ConnectivityState::Connected,
            service_family: ServiceFamily::Provider,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::Current,
            deployment_policy_ref: None,
        },
        LocalSafePromise {
            state: ConnectivityState::Constrained,
            service_family: ServiceFamily::ManagedWorkspace,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::LastKnownGoodTimestamp,
            deployment_policy_ref: None,
        },
        LocalSafePromise {
            state: ConnectivityState::OfflineLocalSafe,
            service_family: ServiceFamily::Remote,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::CachedInspectOnly,
            deployment_policy_ref: None,
        },
        LocalSafePromise {
            state: ConnectivityState::ReauthRequired,
            service_family: ServiceFamily::Provider,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::AuthOrPolicyStale,
            deployment_policy_ref: None,
        },
        LocalSafePromise {
            state: ConnectivityState::ReconciliationPending,
            service_family: ServiceFamily::ManagedWorkspace,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::LastKnownGoodTimestamp,
            deployment_policy_ref: None,
        },
        LocalSafePromise {
            state: ConnectivityState::ServiceUnavailable,
            service_family: ServiceFamily::Remote,
            local_editing_available: true,
            local_search_available: true,
            local_git_available: true,
            local_tasks_available: true,
            cached_inspection_available: true,
            stale_label_semantics: StaleLabelSemantics::ServiceFamilyUnavailable,
            deployment_policy_ref: None,
        },
    ];

    let connectivity_badges = vec![
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:provider:connected".to_string(),
            service_family: ServiceFamily::Provider,
            state: ConnectivityState::Connected,
            affected_family_label: ServiceFamily::Provider.label().to_string(),
            promise_summary: "Provider-backed review and publish flows are current.".to_string(),
            action_label: "Open deferred intents".to_string(),
            action_ref: "action:continuity:provider:open_outbox".to_string(),
        },
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:managed:constrained".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            state: ConnectivityState::Constrained,
            affected_family_label: ServiceFamily::ManagedWorkspace.label().to_string(),
            promise_summary: "Managed workspace writes may slow or narrow, but local work stays responsive."
                .to_string(),
            action_label: "Open managed continuity details".to_string(),
            action_ref: "action:continuity:managed:open_details".to_string(),
        },
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:remote:offline-local-safe".to_string(),
            service_family: ServiceFamily::Remote,
            state: ConnectivityState::OfflineLocalSafe,
            affected_family_label: ServiceFamily::Remote.label().to_string(),
            promise_summary: "Local editing and cached remote context remain available while live remote control is offline."
                .to_string(),
            action_label: "Open remote continuity details".to_string(),
            action_ref: "action:continuity:remote:open_details".to_string(),
        },
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:provider:reauth-required".to_string(),
            service_family: ServiceFamily::Provider,
            state: ConnectivityState::ReauthRequired,
            affected_family_label: ServiceFamily::Provider.label().to_string(),
            promise_summary: "Local drafts stay available; provider writes pause until the session is renewed."
                .to_string(),
            action_label: "Reauthenticate".to_string(),
            action_ref: "action:continuity:provider:reauth".to_string(),
        },
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:managed:reconciliation-pending".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            state: ConnectivityState::ReconciliationPending,
            affected_family_label: ServiceFamily::ManagedWorkspace.label().to_string(),
            promise_summary: "Nothing replays invisibly; affected managed writes wait for reconcile review."
                .to_string(),
            action_label: "Open reconcile review".to_string(),
            action_ref: "action:continuity:managed:open_reconcile".to_string(),
        },
        ConnectivityBadge {
            record_kind: CONNECTIVITY_BADGE_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            badge_id: "badge:remote:service-unavailable".to_string(),
            service_family: ServiceFamily::Remote,
            state: ConnectivityState::ServiceUnavailable,
            affected_family_label: ServiceFamily::Remote.label().to_string(),
            promise_summary: "Only remote execution degrades; local work stays available.".to_string(),
            action_label: "Retry remote health".to_string(),
            action_ref: "action:continuity:remote:retry_health".to_string(),
        },
    ];

    let connectivity_cards = vec![
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:provider:connected".to_string(),
            service_family: ServiceFamily::Provider,
            state: ConnectivityState::Connected,
            title_label: "Connected provider state".to_string(),
            summary_label: "Provider transport, auth, and policy are current for this action family."
                .to_string(),
            promise_label: "Full claimed provider-backed behavior remains available.".to_string(),
            what_still_works: vec![
                "Review comment publish".to_string(),
                "Deferred draft replay".to_string(),
                "Support export with current route facts".to_string(),
            ],
            suggested_actions: vec![ConnectivityCardAction {
                action_id: "action.card.provider.connected.outbox".to_string(),
                label: "Open deferred intents".to_string(),
                action_ref: "action:continuity:provider:open_outbox".to_string(),
            }],
        },
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:managed:constrained".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            state: ConnectivityState::Constrained,
            title_label: "Managed workspace constrained".to_string(),
            summary_label: "Latency and refresh pressure rose above the normal floor, so managed writes may narrow visibly."
                .to_string(),
            promise_label: "Editing, search, Git, tasks, and cached inspection remain available."
                .to_string(),
            what_still_works: vec![
                "Local editing against the current workspace".to_string(),
                "Cached managed metadata inspection".to_string(),
                "Deferred replay review".to_string(),
            ],
            suggested_actions: vec![
                ConnectivityCardAction {
                    action_id: "action.card.managed.constrained.details".to_string(),
                    label: "Open managed continuity details".to_string(),
                    action_ref: "action:continuity:managed:open_details".to_string(),
                },
                ConnectivityCardAction {
                    action_id: "action.card.managed.constrained.export".to_string(),
                    label: "Export admin handoff".to_string(),
                    action_ref: "action:continuity:managed:export_handoff".to_string(),
                },
            ],
        },
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:remote:offline-local-safe".to_string(),
            service_family: ServiceFamily::Remote,
            state: ConnectivityState::OfflineLocalSafe,
            title_label: "Remote control offline, local work safe".to_string(),
            summary_label: "No route to the remote endpoint exists, but the local workspace and signed caches remain usable."
                .to_string(),
            promise_label: "Local editing, search, Git, tasks, and cached remote inspection remain available."
                .to_string(),
            what_still_works: vec![
                "Local code edits".to_string(),
                "Cached remote run history".to_string(),
                "Support export without live remote credentials".to_string(),
            ],
            suggested_actions: vec![
                ConnectivityCardAction {
                    action_id: "action.card.remote.offline.details".to_string(),
                    label: "Open remote continuity details".to_string(),
                    action_ref: "action:continuity:remote:open_details".to_string(),
                },
                ConnectivityCardAction {
                    action_id: "action.card.remote.offline.export".to_string(),
                    label: "Export remote handoff".to_string(),
                    action_ref: "action:continuity:remote:export_handoff".to_string(),
                },
            ],
        },
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:provider:reauth-required".to_string(),
            service_family: ServiceFamily::Provider,
            state: ConnectivityState::ReauthRequired,
            title_label: "Provider reauthentication required".to_string(),
            summary_label: "The provider session no longer admits managed writes on the current scope."
                .to_string(),
            promise_label: "Local files, local drafts, and cached review context remain available.".to_string(),
            what_still_works: vec![
                "Continue editing the local draft".to_string(),
                "Inspect queued provider intent details".to_string(),
                "Export redaction-safe reconcile evidence".to_string(),
            ],
            suggested_actions: vec![
                ConnectivityCardAction {
                    action_id: "action.card.provider.reauth".to_string(),
                    label: "Reauthenticate".to_string(),
                    action_ref: "action:continuity:provider:reauth".to_string(),
                },
                ConnectivityCardAction {
                    action_id: "action.card.provider.cancel".to_string(),
                    label: "Cancel queued writes".to_string(),
                    action_ref: "action:continuity:provider:cancel_outbox".to_string(),
                },
            ],
        },
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:managed:reconciliation-pending".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            state: ConnectivityState::ReconciliationPending,
            title_label: "Managed reconcile review required".to_string(),
            summary_label: "Connectivity returned, but at least one queued managed action drifted and cannot replay as-is."
                .to_string(),
            promise_label: "Nothing replays invisibly; affected actions wait for review, expiry, or safe replay."
                .to_string(),
            what_still_works: vec![
                "Local editing against the workspace".to_string(),
                "Inspect queued managed actions".to_string(),
                "Export reconcile packet without raw secrets".to_string(),
            ],
            suggested_actions: vec![
                ConnectivityCardAction {
                    action_id: "action.card.managed.reconcile".to_string(),
                    label: "Open reconcile review".to_string(),
                    action_ref: "action:continuity:managed:open_reconcile".to_string(),
                },
                ConnectivityCardAction {
                    action_id: "action.card.managed.local_only".to_string(),
                    label: "Continue local-only".to_string(),
                    action_ref: "action:continuity:managed:continue_local_only".to_string(),
                },
            ],
        },
        ConnectivityStateCard {
            record_kind: CONNECTIVITY_CARD_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            card_id: "card:remote:service-unavailable".to_string(),
            service_family: ServiceFamily::Remote,
            state: ConnectivityState::ServiceUnavailable,
            title_label: "Remote service unavailable".to_string(),
            summary_label: "The remote helper or control plane is unavailable for this action family."
                .to_string(),
            promise_label: "Only remote execution degrades; unrelated local workflows remain usable.".to_string(),
            what_still_works: vec![
                "Local editing and tasks".to_string(),
                "Cached remote target inspection".to_string(),
                "Export support packet for remote recovery".to_string(),
            ],
            suggested_actions: vec![
                ConnectivityCardAction {
                    action_id: "action.card.remote.retry".to_string(),
                    label: "Retry remote health".to_string(),
                    action_ref: "action:continuity:remote:retry_health".to_string(),
                },
                ConnectivityCardAction {
                    action_id: "action.card.remote.inspect".to_string(),
                    label: "Inspect route details".to_string(),
                    action_ref: "action:continuity:remote:inspect_route".to_string(),
                },
            ],
        },
    ];

    let replayable_shape = IdempotencyKeyShape {
        includes_command_id: true,
        includes_target_identity: true,
        includes_actor_identity: true,
        includes_policy_epoch: true,
        includes_context_hash: true,
        support_summary: "command+target+actor+policy+context".to_string(),
    };

    let provider_expiry = ExpiryPolicy {
        expires_after: "PT72H".to_string(),
        blocks_replay_after_expiry: true,
        rationale_summary: "Provider review drafts expire after three days unless refreshed."
            .to_string(),
    };
    let managed_expiry = ExpiryPolicy {
        expires_after: "PT24H".to_string(),
        blocks_replay_after_expiry: true,
        rationale_summary:
            "Managed label changes expire after one day if policy, auth, or target drift."
                .to_string(),
    };

    let command_declarations = vec![
        NetworkCommandDeclaration {
            record_kind: NETWORK_COMMAND_DECLARATION_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            command_id: "cmd:provider.review_comment.save_draft".to_string(),
            service_family: ServiceFamily::Provider,
            action_family: "explicit_idempotent_managed_write".to_string(),
            queueability: CommandQueueabilityDeclaration {
                offline_read_class: OfflineReadClass::LiveRequired,
                queueability: QueueabilityClass::ExplicitIdempotentReviewable,
                replay_safety: ReplaySafetyClass::IdempotentBounded,
                idempotency_key_shape: Some(replayable_shape.clone()),
                expiry_policy: Some(provider_expiry),
                stale_label_semantics: StaleLabelSemantics::CachedInspectOnly,
                reconciliation_owner: Some(ReconciliationOwnerClass::Provider),
            },
            missing_metadata_block_reason: "Deferred provider writes require idempotency, expiry, stale-label, and reconciliation owner metadata.".to_string(),
        },
        NetworkCommandDeclaration {
            record_kind: NETWORK_COMMAND_DECLARATION_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            command_id: "cmd:managed.workspace.label.apply".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            action_family: "explicit_idempotent_managed_write".to_string(),
            queueability: CommandQueueabilityDeclaration {
                offline_read_class: OfflineReadClass::LiveRequired,
                queueability: QueueabilityClass::ExplicitIdempotentReviewable,
                replay_safety: ReplaySafetyClass::IdempotentBounded,
                idempotency_key_shape: Some(replayable_shape),
                expiry_policy: Some(managed_expiry),
                stale_label_semantics: StaleLabelSemantics::AuthOrPolicyStale,
                reconciliation_owner: Some(ReconciliationOwnerClass::ManagedWorkspace),
            },
            missing_metadata_block_reason: "Managed writes require idempotency, expiry, stale-label, and reconciliation owner metadata.".to_string(),
        },
        NetworkCommandDeclaration {
            record_kind: NETWORK_COMMAND_DECLARATION_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            command_id: "cmd:remote.execution.dispatch".to_string(),
            service_family: ServiceFamily::Remote,
            action_family: "remote_execution".to_string(),
            queueability: CommandQueueabilityDeclaration {
                offline_read_class: OfflineReadClass::LiveRequired,
                queueability: QueueabilityClass::NeverQueue,
                replay_safety: ReplaySafetyClass::ManualRerunRequired,
                idempotency_key_shape: None,
                expiry_policy: None,
                stale_label_semantics: StaleLabelSemantics::ServiceFamilyUnavailable,
                reconciliation_owner: None,
            },
            missing_metadata_block_reason: "Remote execution requires a live target witness and must be rerun manually after reconnect.".to_string(),
        },
        NetworkCommandDeclaration {
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
        },
    ];

    let provider_actor = ActorIdentity {
        actor_ref: "actor:user:alice".to_string(),
        actor_class: "human_account".to_string(),
        display_label: "Alice".to_string(),
    };
    let managed_actor = ActorIdentity {
        actor_ref: "actor:user:bob".to_string(),
        actor_class: "human_account".to_string(),
        display_label: "Bob".to_string(),
    };
    let provider_auth = AuthScopeSnapshot {
        subject_ref: "subject:alice".to_string(),
        scope_refs: vec!["pull_request:write".to_string()],
        auth_epoch: "auth-epoch-17".to_string(),
    };
    let managed_auth = AuthScopeSnapshot {
        subject_ref: "subject:bob".to_string(),
        scope_refs: vec!["managed_label:write".to_string()],
        auth_epoch: "auth-epoch-44".to_string(),
    };

    let provider_prerequisites_pending = vec![
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ConnectivityRestored,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Provider route returned after the original queue window."
                .to_string(),
            exact_match_required: false,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::TargetIdentityCurrent,
            state: ReplayPrerequisiteState::FailedRequiresReview,
            requirement_summary: "Replay requires the same PR anchor and target object identity."
                .to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::AuthScopeCurrent,
            state: ReplayPrerequisiteState::FailedRequiresReview,
            requirement_summary:
                "Replay requires the same provider write scope and account mapping.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::PolicyEpochCurrent,
            state: ReplayPrerequisiteState::FailedRequiresReview,
            requirement_summary: "Replay requires the same effective policy epoch.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ContextHashCurrent,
            state: ReplayPrerequisiteState::PendingRevalidation,
            requirement_summary: "Replay requires the same command context hash.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::DataFingerprintCurrent,
            state: ReplayPrerequisiteState::FailedRequiresReview,
            requirement_summary:
                "Replay requires the same provider-side data fingerprint for the comment anchor."
                    .to_string(),
            exact_match_required: true,
        },
    ];
    let managed_prerequisites_ok = vec![
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ConnectivityRestored,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Managed control-plane connectivity is current.".to_string(),
            exact_match_required: false,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ServiceFamilyHealthy,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary:
                "Managed workspace service health remains within the replay floor.".to_string(),
            exact_match_required: false,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::TargetIdentityCurrent,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary:
                "Replay requires the same managed workspace identity and target label set."
                    .to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::AuthScopeCurrent,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Replay requires the same managed write scope.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::PolicyEpochCurrent,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Replay requires the same managed policy epoch.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ContextHashCurrent,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Replay requires the same command context hash.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::DataFingerprintCurrent,
            state: ReplayPrerequisiteState::Satisfied,
            requirement_summary: "Replay requires the same target data fingerprint.".to_string(),
            exact_match_required: true,
        },
    ];
    let managed_prerequisites_queued = vec![
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::ConnectivityRestored,
            state: ReplayPrerequisiteState::PendingRevalidation,
            requirement_summary: "Managed connectivity must return before replay.".to_string(),
            exact_match_required: false,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::AuthScopeCurrent,
            state: ReplayPrerequisiteState::PendingRevalidation,
            requirement_summary: "Replay requires current auth scope after reauth.".to_string(),
            exact_match_required: true,
        },
        ReplayPrerequisite {
            prerequisite_class: ReplayPrerequisiteClass::PolicyEpochCurrent,
            state: ReplayPrerequisiteState::PendingRevalidation,
            requirement_summary: "Replay requires the same managed policy epoch.".to_string(),
            exact_match_required: true,
        },
    ];

    let deferred_intents = vec![
        DeferredIntent {
            record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            intent_id: "intent:provider-review-comment:01".to_string(),
            command_id: "cmd:provider.review_comment.save_draft".to_string(),
            service_family: ServiceFamily::Provider,
            target_identity: TargetIdentity {
                target_ref: "provider:github:pr:42:comment-thread:7".to_string(),
                target_class: "pull_request_comment_thread".to_string(),
                tenant_ref: "org:acme".to_string(),
                region_ref: "region:us".to_string(),
                endpoint_ref: "endpoint:github.com".to_string(),
                version_ref: "head:abc123".to_string(),
            },
            actor: provider_actor.clone(),
            queued_at: "2026-06-04T18:00:00Z".to_string(),
            expires_at: "2026-06-07T18:00:00Z".to_string(),
            policy_epoch: "policy-epoch-42".to_string(),
            auth_scope: provider_auth.clone(),
            context_hash: "sha256:provider-review-context".to_string(),
            data_fingerprint: "sha256:provider-review-anchor".to_string(),
            idempotency_key_ref: "idem:provider-review-comment:01".to_string(),
            previewed_effect_summary: "Post one provider review comment to PR 42 line anchor 7."
                .to_string(),
            replay_prerequisites: provider_prerequisites_pending.clone(),
            state: DeferredIntentState::ConflictReview,
            available_actions: vec![
                DeferredIntentAction::OpenReview,
                DeferredIntentAction::Cancel,
                DeferredIntentAction::Export,
            ],
            sensitive_payload_posture: SensitivePayloadPosture::RedactedPreviewOnly,
        },
        DeferredIntent {
            record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            intent_id: "intent:managed-label-apply:02".to_string(),
            command_id: "cmd:managed.workspace.label.apply".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            target_identity: TargetIdentity {
                target_ref: "managed:workspace:billing-api:label:owner".to_string(),
                target_class: "managed_workspace_label".to_string(),
                tenant_ref: "tenant:acme".to_string(),
                region_ref: "region:us-west-2".to_string(),
                endpoint_ref: "endpoint:managed-control".to_string(),
                version_ref: "workspace-rev:18".to_string(),
            },
            actor: managed_actor.clone(),
            queued_at: "2026-06-05T09:00:00Z".to_string(),
            expires_at: "2026-06-06T09:00:00Z".to_string(),
            policy_epoch: "policy-epoch-91".to_string(),
            auth_scope: managed_auth.clone(),
            context_hash: "sha256:managed-label-context".to_string(),
            data_fingerprint: "sha256:managed-label-target".to_string(),
            idempotency_key_ref: "idem:managed-label-apply:02".to_string(),
            previewed_effect_summary: "Apply the queued owner label to the managed workspace."
                .to_string(),
            replay_prerequisites: managed_prerequisites_ok.clone(),
            state: DeferredIntentState::Replayed,
            available_actions: vec![DeferredIntentAction::Export],
            sensitive_payload_posture: SensitivePayloadPosture::NoRawPayload,
        },
        DeferredIntent {
            record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            intent_id: "intent:provider-publish-later:03".to_string(),
            command_id: "cmd:provider.review_comment.save_draft".to_string(),
            service_family: ServiceFamily::Provider,
            target_identity: TargetIdentity {
                target_ref: "provider:github:pr:77:draft-review".to_string(),
                target_class: "pull_request_draft_review".to_string(),
                tenant_ref: "org:acme".to_string(),
                region_ref: "region:us".to_string(),
                endpoint_ref: "endpoint:github.enterprise.example".to_string(),
                version_ref: "head:release-4".to_string(),
            },
            actor: provider_actor.clone(),
            queued_at: "2026-06-05T14:20:00Z".to_string(),
            expires_at: "2026-06-08T14:20:00Z".to_string(),
            policy_epoch: "policy-epoch-43".to_string(),
            auth_scope: provider_auth.clone(),
            context_hash: "sha256:provider-publish-context".to_string(),
            data_fingerprint: "sha256:provider-review-draft".to_string(),
            idempotency_key_ref: "idem:provider-publish-later:03".to_string(),
            previewed_effect_summary: "Publish the saved review draft for PR 77 after reconnect."
                .to_string(),
            replay_prerequisites: provider_prerequisites_pending,
            state: DeferredIntentState::ConflictReview,
            available_actions: vec![
                DeferredIntentAction::OpenReview,
                DeferredIntentAction::Cancel,
                DeferredIntentAction::Export,
            ],
            sensitive_payload_posture: SensitivePayloadPosture::RedactedPreviewOnly,
        },
        DeferredIntent {
            record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            intent_id: "intent:managed-label-expired:04".to_string(),
            command_id: "cmd:managed.workspace.label.apply".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            target_identity: TargetIdentity {
                target_ref: "managed:workspace:payments-api:label:owner".to_string(),
                target_class: "managed_workspace_label".to_string(),
                tenant_ref: "tenant:acme".to_string(),
                region_ref: "region:eu-west-1".to_string(),
                endpoint_ref: "endpoint:managed-control-eu".to_string(),
                version_ref: "workspace-rev:44".to_string(),
            },
            actor: managed_actor.clone(),
            queued_at: "2026-06-03T02:15:00Z".to_string(),
            expires_at: "2026-06-04T02:15:00Z".to_string(),
            policy_epoch: "policy-epoch-88".to_string(),
            auth_scope: managed_auth.clone(),
            context_hash: "sha256:managed-expired-context".to_string(),
            data_fingerprint: "sha256:managed-expired-target".to_string(),
            idempotency_key_ref: "idem:managed-label-expired:04".to_string(),
            previewed_effect_summary: "Apply the queued owner label to the payments workspace."
                .to_string(),
            replay_prerequisites: managed_prerequisites_ok.clone(),
            state: DeferredIntentState::Dropped,
            available_actions: vec![DeferredIntentAction::Export],
            sensitive_payload_posture: SensitivePayloadPosture::NoRawPayload,
        },
        DeferredIntent {
            record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            intent_id: "intent:managed-label-queued:05".to_string(),
            command_id: "cmd:managed.workspace.label.apply".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            target_identity: TargetIdentity {
                target_ref: "managed:workspace:search-api:label:tier".to_string(),
                target_class: "managed_workspace_label".to_string(),
                tenant_ref: "tenant:acme".to_string(),
                region_ref: "region:us-east-1".to_string(),
                endpoint_ref: "endpoint:managed-control-use1".to_string(),
                version_ref: "workspace-rev:52".to_string(),
            },
            actor: managed_actor,
            queued_at: "2026-06-06T07:10:00Z".to_string(),
            expires_at: "2026-06-07T07:10:00Z".to_string(),
            policy_epoch: "policy-epoch-92".to_string(),
            auth_scope: managed_auth,
            context_hash: "sha256:managed-queued-context".to_string(),
            data_fingerprint: "sha256:managed-queued-target".to_string(),
            idempotency_key_ref: "idem:managed-label-queued:05".to_string(),
            previewed_effect_summary:
                "Queue the tier label update until managed connectivity and auth recover."
                    .to_string(),
            replay_prerequisites: managed_prerequisites_queued,
            state: DeferredIntentState::Queued,
            available_actions: vec![DeferredIntentAction::Cancel, DeferredIntentAction::Export],
            sensitive_payload_posture: SensitivePayloadPosture::NoRawPayload,
        },
    ];

    let idempotency_receipts = vec![
        IdempotencyKeyReceipt {
            record_kind: IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            receipt_id: "receipt:provider-review-comment:01".to_string(),
            intent_id: "intent:provider-review-comment:01".to_string(),
            idempotency_key_ref: "idem:provider-review-comment:01".to_string(),
            dedupe_scope_ref: "dedupe:provider:pull-request-review".to_string(),
            outcome: IdempotencyKeyReceiptOutcome::BlockedPendingReview,
            recorded_at: "2026-06-06T10:02:00Z".to_string(),
            summary_label: "Replay stayed blocked because the provider anchor, auth scope, policy epoch, and data fingerprint drifted."
                .to_string(),
        },
        IdempotencyKeyReceipt {
            record_kind: IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            receipt_id: "receipt:managed-label-apply:02".to_string(),
            intent_id: "intent:managed-label-apply:02".to_string(),
            idempotency_key_ref: "idem:managed-label-apply:02".to_string(),
            dedupe_scope_ref: "dedupe:managed:workspace-label".to_string(),
            outcome: IdempotencyKeyReceiptOutcome::ReplayedUnderSameKey,
            recorded_at: "2026-06-05T09:08:00Z".to_string(),
            summary_label: "Managed label replay completed exactly once under the preserved idempotency key."
                .to_string(),
        },
        IdempotencyKeyReceipt {
            record_kind: IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            receipt_id: "receipt:provider-publish-later:03".to_string(),
            intent_id: "intent:provider-publish-later:03".to_string(),
            idempotency_key_ref: "idem:provider-publish-later:03".to_string(),
            dedupe_scope_ref: "dedupe:provider:draft-review".to_string(),
            outcome: IdempotencyKeyReceiptOutcome::BlockedPendingReview,
            recorded_at: "2026-06-06T12:40:00Z".to_string(),
            summary_label: "Replay was narrowed to local-draft continuity because the provider boundary changed."
                .to_string(),
        },
        IdempotencyKeyReceipt {
            record_kind: IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            receipt_id: "receipt:managed-label-expired:04".to_string(),
            intent_id: "intent:managed-label-expired:04".to_string(),
            idempotency_key_ref: "idem:managed-label-expired:04".to_string(),
            dedupe_scope_ref: "dedupe:managed:workspace-label".to_string(),
            outcome: IdempotencyKeyReceiptOutcome::DiscardedWithoutReplay,
            recorded_at: "2026-06-04T02:20:00Z".to_string(),
            summary_label: "The managed label intent expired before replay and was discarded without mutation."
                .to_string(),
        },
        IdempotencyKeyReceipt {
            record_kind: IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            receipt_id: "receipt:managed-label-queued:05".to_string(),
            intent_id: "intent:managed-label-queued:05".to_string(),
            idempotency_key_ref: "idem:managed-label-queued:05".to_string(),
            dedupe_scope_ref: "dedupe:managed:workspace-label".to_string(),
            outcome: IdempotencyKeyReceiptOutcome::QueuedForOutbox,
            recorded_at: "2026-06-06T07:10:00Z".to_string(),
            summary_label: "The managed label intent is queued locally with its idempotency key preserved for later replay."
                .to_string(),
        },
    ];

    let reconciliation_reviews = vec![
        ReconciliationReviewSheet {
            record_kind: RECONCILIATION_REVIEW_SHEET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            review_id: "review:provider-review-comment:01".to_string(),
            intent_id: "intent:provider-review-comment:01".to_string(),
            connectivity_state: ConnectivityState::ReconciliationPending,
            drift_dimensions: vec![
                DriftDimension::Policy,
                DriftDimension::Auth,
                DriftDimension::Target,
                DriftDimension::Data,
            ],
            previewed_effect_summary: "Post one provider review comment to PR 42 line anchor 7.".to_string(),
            current_drift_summary: "The provider diff changed and the original line anchor moved."
                .to_string(),
            policy_or_auth_delta_summary: "Provider session reauth is required and the policy epoch advanced."
                .to_string(),
            available_actions: vec![
                DeferredIntentAction::OpenReview,
                DeferredIntentAction::Cancel,
                DeferredIntentAction::Export,
            ],
            replay_blocked_until_review: true,
        },
        ReconciliationReviewSheet {
            record_kind: RECONCILIATION_REVIEW_SHEET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            review_id: "review:provider-publish-later:03".to_string(),
            intent_id: "intent:provider-publish-later:03".to_string(),
            connectivity_state: ConnectivityState::ReconciliationPending,
            drift_dimensions: vec![
                DriftDimension::Route,
                DriftDimension::Endpoint,
                DriftDimension::Target,
            ],
            previewed_effect_summary: "Publish the saved review draft for PR 77 after reconnect."
                .to_string(),
            current_drift_summary: "The provider boundary now resolves to a different endpoint and draft target."
                .to_string(),
            policy_or_auth_delta_summary: "No fresh auth block exists, but the changed provider boundary requires manual review."
                .to_string(),
            available_actions: vec![
                DeferredIntentAction::OpenReview,
                DeferredIntentAction::Cancel,
                DeferredIntentAction::Export,
            ],
            replay_blocked_until_review: true,
        },
    ];

    let reconciliation_packets = vec![
        ReconciliationPacket {
            record_kind: RECONCILIATION_PACKET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            packet_id: "packet:provider-review-comment:01".to_string(),
            intent_id: "intent:provider-review-comment:01".to_string(),
            service_family: ServiceFamily::Provider,
            connectivity_state: ConnectivityState::ReconciliationPending,
            disposition: ReconciliationDisposition::Blocked,
            drift_dimensions: vec![
                DriftDimension::Policy,
                DriftDimension::Auth,
                DriftDimension::Target,
                DriftDimension::Data,
            ],
            drift_snapshots: vec![
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Policy,
                    queued_value_summary: "policy-epoch-42".to_string(),
                    current_value_summary: "policy-epoch-43".to_string(),
                },
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Auth,
                    queued_value_summary: "pull_request:write via auth-epoch-17".to_string(),
                    current_value_summary: "reauth-required on auth-epoch-18".to_string(),
                },
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Target,
                    queued_value_summary: "provider:github:pr:42:comment-thread:7".to_string(),
                    current_value_summary: "provider:github:pr:42:comment-thread:9".to_string(),
                },
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Data,
                    queued_value_summary: "sha256:provider-review-anchor".to_string(),
                    current_value_summary: "sha256:provider-review-anchor:new".to_string(),
                },
            ],
            summary_label: "Queued provider comment stayed blocked and entered reconcile review instead of replaying invisibly."
                .to_string(),
            idempotency_receipt_ref: Some("receipt:provider-review-comment:01".to_string()),
            reconciliation_review_ref: Some("review:provider-review-comment:01".to_string()),
            support_manifest_ref: "support-manifest:connectivity:001".to_string(),
            outcome_disclosed_to_user: true,
        },
        ReconciliationPacket {
            record_kind: RECONCILIATION_PACKET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            packet_id: "packet:managed-label-apply:02".to_string(),
            intent_id: "intent:managed-label-apply:02".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            connectivity_state: ConnectivityState::Connected,
            disposition: ReconciliationDisposition::Replayed,
            drift_dimensions: vec![],
            drift_snapshots: vec![],
            summary_label: "Managed label replayed once under the preserved idempotency key after prerequisites revalidated cleanly."
                .to_string(),
            idempotency_receipt_ref: Some("receipt:managed-label-apply:02".to_string()),
            reconciliation_review_ref: None,
            support_manifest_ref: "support-manifest:connectivity:001".to_string(),
            outcome_disclosed_to_user: true,
        },
        ReconciliationPacket {
            record_kind: RECONCILIATION_PACKET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            packet_id: "packet:provider-publish-later:03".to_string(),
            intent_id: "intent:provider-publish-later:03".to_string(),
            service_family: ServiceFamily::Provider,
            connectivity_state: ConnectivityState::ReconciliationPending,
            disposition: ReconciliationDisposition::Narrowed,
            drift_dimensions: vec![
                DriftDimension::Route,
                DriftDimension::Endpoint,
                DriftDimension::Target,
            ],
            drift_snapshots: vec![
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Route,
                    queued_value_summary: "provider route: github.com".to_string(),
                    current_value_summary: "provider route: github.enterprise.example".to_string(),
                },
                DriftRevalidationSnapshot {
                    dimension: DriftDimension::Target,
                    queued_value_summary: "provider:github:pr:77:draft-review".to_string(),
                    current_value_summary: "provider:github-enterprise:pr:77:draft-review".to_string(),
                },
            ],
            summary_label: "Queued publish was narrowed to local-draft continuity because the provider boundary changed."
                .to_string(),
            idempotency_receipt_ref: Some("receipt:provider-publish-later:03".to_string()),
            reconciliation_review_ref: Some("review:provider-publish-later:03".to_string()),
            support_manifest_ref: "support-manifest:connectivity:001".to_string(),
            outcome_disclosed_to_user: true,
        },
        ReconciliationPacket {
            record_kind: RECONCILIATION_PACKET_RECORD_KIND.to_string(),
            schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
            packet_id: "packet:managed-label-expired:04".to_string(),
            intent_id: "intent:managed-label-expired:04".to_string(),
            service_family: ServiceFamily::ManagedWorkspace,
            connectivity_state: ConnectivityState::ReauthRequired,
            disposition: ReconciliationDisposition::Discarded,
            drift_dimensions: vec![DriftDimension::Expiry],
            drift_snapshots: vec![],
            summary_label: "Queued managed label expired before replay and was discarded with explicit disclosure."
                .to_string(),
            idempotency_receipt_ref: Some("receipt:managed-label-expired:04".to_string()),
            reconciliation_review_ref: None,
            support_manifest_ref: "support-manifest:connectivity:001".to_string(),
            outcome_disclosed_to_user: true,
        },
    ];

    let support_export = SupportExportPacket {
        record_kind: SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: CONNECTIVITY_CONTINUITY_SCHEMA_VERSION,
        state_vocabulary: states.clone(),
        badge_refs: connectivity_badges
            .iter()
            .map(|badge| badge.badge_id.clone())
            .collect(),
        deferred_intent_refs: deferred_intents
            .iter()
            .map(|intent| intent.intent_id.clone())
            .collect(),
        reconciliation_review_refs: reconciliation_reviews
            .iter()
            .map(|review| review.review_id.clone())
            .collect(),
        reconciliation_packet_refs: reconciliation_packets
            .iter()
            .map(|packet| packet.packet_id.clone())
            .collect(),
        idempotency_receipt_refs: idempotency_receipts
            .iter()
            .map(|receipt| receipt.receipt_id.clone())
            .collect(),
        outcome_rows: vec![
            SupportExportOutcomeRow {
                row_id: "support-row:provider-review-comment:01".to_string(),
                intent_id: "intent:provider-review-comment:01".to_string(),
                service_family: ServiceFamily::Provider,
                intent_state: DeferredIntentState::ConflictReview,
                reconciliation_disposition: Some(ReconciliationDisposition::Blocked),
                summary_label: "Provider comment replay blocked pending reconcile review after target, auth, policy, and data drift."
                    .to_string(),
                actor_label: "Alice".to_string(),
                target_ref: "provider:github:pr:42:comment-thread:7".to_string(),
                idempotency_receipt_ref: Some("receipt:provider-review-comment:01".to_string()),
                reconciliation_packet_ref: Some("packet:provider-review-comment:01".to_string()),
            },
            SupportExportOutcomeRow {
                row_id: "support-row:managed-label-apply:02".to_string(),
                intent_id: "intent:managed-label-apply:02".to_string(),
                service_family: ServiceFamily::ManagedWorkspace,
                intent_state: DeferredIntentState::Replayed,
                reconciliation_disposition: Some(ReconciliationDisposition::Replayed),
                summary_label: "Managed label replay completed once under the preserved idempotency key."
                    .to_string(),
                actor_label: "Bob".to_string(),
                target_ref: "managed:workspace:billing-api:label:owner".to_string(),
                idempotency_receipt_ref: Some("receipt:managed-label-apply:02".to_string()),
                reconciliation_packet_ref: Some("packet:managed-label-apply:02".to_string()),
            },
            SupportExportOutcomeRow {
                row_id: "support-row:provider-publish-later:03".to_string(),
                intent_id: "intent:provider-publish-later:03".to_string(),
                service_family: ServiceFamily::Provider,
                intent_state: DeferredIntentState::ConflictReview,
                reconciliation_disposition: Some(ReconciliationDisposition::Narrowed),
                summary_label: "Queued provider publish was narrowed to a local draft because the provider boundary changed."
                    .to_string(),
                actor_label: "Alice".to_string(),
                target_ref: "provider:github:pr:77:draft-review".to_string(),
                idempotency_receipt_ref: Some("receipt:provider-publish-later:03".to_string()),
                reconciliation_packet_ref: Some("packet:provider-publish-later:03".to_string()),
            },
            SupportExportOutcomeRow {
                row_id: "support-row:managed-label-expired:04".to_string(),
                intent_id: "intent:managed-label-expired:04".to_string(),
                service_family: ServiceFamily::ManagedWorkspace,
                intent_state: DeferredIntentState::Dropped,
                reconciliation_disposition: Some(ReconciliationDisposition::Discarded),
                summary_label: "Queued managed label expired before replay and was discarded without mutation."
                    .to_string(),
                actor_label: "Bob".to_string(),
                target_ref: "managed:workspace:payments-api:label:owner".to_string(),
                idempotency_receipt_ref: Some("receipt:managed-label-expired:04".to_string()),
                reconciliation_packet_ref: Some("packet:managed-label-expired:04".to_string()),
            },
            SupportExportOutcomeRow {
                row_id: "support-row:managed-label-queued:05".to_string(),
                intent_id: "intent:managed-label-queued:05".to_string(),
                service_family: ServiceFamily::ManagedWorkspace,
                intent_state: DeferredIntentState::Queued,
                reconciliation_disposition: None,
                summary_label: "Queued managed label remains in the outbox awaiting reconnect and reauth; no replay occurred yet."
                    .to_string(),
                actor_label: "Bob".to_string(),
                target_ref: "managed:workspace:search-api:label:tier".to_string(),
                idempotency_receipt_ref: Some("receipt:managed-label-queued:05".to_string()),
                reconciliation_packet_ref: None,
            },
        ],
        raw_sensitive_payloads_excluded: true,
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
        connectivity_badges,
        connectivity_cards,
        command_declarations,
        deferred_intents,
        idempotency_receipts,
        reconciliation_reviews,
        reconciliation_packets,
        support_export,
    }
}
