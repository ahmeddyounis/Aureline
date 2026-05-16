//! Keychain lock-state, denied projection, and secret-repair beta page.
//!
//! This module owns the first executable secret-repair contract for the
//! claimed beta rows. The
//! [`crate::secret_broker`] beta froze the per-row handle-only projection
//! vocabulary and the consumer-identity audit stream. The secret-repair beta
//! builds on that vocabulary and adds three reviewable record kinds that make
//! secret handling *recoverable* without ever silently falling back to
//! plaintext storage:
//!
//! - One [`KeychainLockStateRow`] per `(profile, backing store, affected
//!   consumer, affected target, workspace scope)` tuple. The row names the
//!   typed lock-state class, the affected consumer, the linked secret-broker
//!   beta row when one exists, and a typed [`RepairActionClass`] with a
//!   user-visible label and a remediation-path ref. Rows in a non-unlocked
//!   state MUST name a repair action; `plaintext_fallback_attempted`,
//!   `plaintext_fallback_offered`, and `raw_secret_material_present` are
//!   beta guardrails that fail closed.
//! - One [`DeniedProjectionRow`] per blocked projection request. The row names
//!   the blocked consumer, requested secret class, requested projection mode,
//!   the typed [`DenialReasonClass`], the required repair action, and a
//!   typed remediation-path label. Denial rows MAY link to the originating
//!   lock-state row when the denial is store-driven, and MAY link to the
//!   downstream secret-broker beta row when the denial is row-driven; either
//!   way the row identifies *which consumer is blocked* and *how to unblock
//!   it* so the surface never collapses a denied projection into an
//!   unexplained downstream failure.
//! - One [`SecretRepairActionEvent`] per repair attempt. The event names the
//!   originating lock-state or denied-projection row, the consumer, the
//!   target/scope, the typed [`RepairActionClass`], and a typed
//!   [`RepairOutcomeClass`]. Events with a `resolved`, `user_declined`, or
//!   `failed_permanent` outcome MUST declare a `resolved_at` timestamp;
//!   events still awaiting the user or in progress MUST leave `resolved_at`
//!   empty.
//!
//! Across all three record kinds, the beta validator surfaces typed defects
//! when a plaintext fallback is attempted, offered, or taken; when a denied
//! projection is missing a consumer, a remediation path, or a typed repair
//! action; when a lock-state row leaves a non-unlocked state without a repair
//! action; or when local editing is not preserved through the failure.
//! [`SecretRepairBetaSupportExport`] wraps the page in a redaction-safe
//! envelope that admin, support, and reviewer surfaces replay verbatim.
//!
//! Reviewer-facing landing page:
//! [`/docs/security/m3/secret_repair_beta.md`](../../../../docs/security/m3/secret_repair_beta.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub use crate::secret_broker::{
    HandleProjectionModeClass, SecretBrokerBetaProfileClass, VaultAdapterClass,
};
pub use crate::secrets::{SecretClass, SecretConsumerIdentity};

/// Beta schema version exported with every secret-repair beta record.
pub const SECRET_REPAIR_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every secret-repair beta record.
pub const SECRET_REPAIR_BETA_SHARED_CONTRACT_REF: &str = "security:secret_repair_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const SECRET_REPAIR_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/secret_repair/secret_repair_matrix.yaml";

/// Stable record kind for [`SecretRepairBetaPage`] payloads.
pub const SECRET_REPAIR_BETA_PAGE_RECORD_KIND: &str = "security_secret_repair_beta_page_record";

/// Stable record kind for [`KeychainLockStateRow`] payloads.
pub const SECRET_REPAIR_BETA_LOCK_STATE_ROW_RECORD_KIND: &str =
    "security_secret_repair_beta_lock_state_row_record";

/// Stable record kind for [`DeniedProjectionRow`] payloads.
pub const SECRET_REPAIR_BETA_DENIED_PROJECTION_ROW_RECORD_KIND: &str =
    "security_secret_repair_beta_denied_projection_row_record";

/// Stable record kind for [`SecretRepairActionEvent`] payloads.
pub const SECRET_REPAIR_BETA_REPAIR_EVENT_RECORD_KIND: &str =
    "security_secret_repair_beta_repair_event_record";

/// Stable record kind for [`SecretRepairBetaSummary`] payloads.
pub const SECRET_REPAIR_BETA_SUMMARY_RECORD_KIND: &str =
    "security_secret_repair_beta_summary_record";

/// Stable record kind for [`SecretRepairBetaDefect`] payloads.
pub const SECRET_REPAIR_BETA_DEFECT_RECORD_KIND: &str =
    "security_secret_repair_beta_defect_record";

/// Stable record kind for [`SecretRepairBetaSupportExport`] payloads.
pub const SECRET_REPAIR_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_secret_repair_beta_support_export_record";

/// Typed lock-state class observed on a backing keychain or vault.
///
/// `Unlocked` is admitted only on rows that explain a transient observation
/// (for example, a row that flipped from `Locked` back to `Unlocked` after a
/// successful repair). The validator requires every non-`Unlocked` row to
/// carry a typed repair action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeychainLockStateClass {
    /// The backing store is unlocked and resolves projections.
    Unlocked,
    /// The backing store is locked and waits for an interactive unlock.
    Locked,
    /// The backing store requires a biometric prompt to unlock.
    BiometricRequired,
    /// The backing store requires a user password or passphrase.
    UserPasswordRequired,
    /// The backing store requires a hardware token (smartcard, security key).
    HardwareTokenRequired,
    /// The backing store daemon or agent is unreachable (service down, missing
    /// socket, sandbox denial).
    DaemonUnreachable,
    /// The configured adapter cannot reach its endpoint or its configuration
    /// is invalid.
    AdapterMisconfigured,
    /// An air-gapped or signed vault snapshot is past its `valid_until`.
    VaultSnapshotExpired,
    /// A signed vault mirror cannot be reached on this profile.
    VaultMirrorOutage,
    /// Current policy forbids resolving secrets from this store.
    PolicyBlocked,
}

impl KeychainLockStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Locked => "locked",
            Self::BiometricRequired => "biometric_required",
            Self::UserPasswordRequired => "user_password_required",
            Self::HardwareTokenRequired => "hardware_token_required",
            Self::DaemonUnreachable => "daemon_unreachable",
            Self::AdapterMisconfigured => "adapter_misconfigured",
            Self::VaultSnapshotExpired => "vault_snapshot_expired",
            Self::VaultMirrorOutage => "vault_mirror_outage",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when this lock state holds projection authority closed and the
    /// row MUST declare a repair action.
    pub const fn requires_repair_action(self) -> bool {
        !matches!(self, Self::Unlocked)
    }
}

/// Typed repair action surfaced by a lock-state or denied-projection row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActionClass {
    /// No repair is necessary; the row is in a healthy state.
    NoneRequired,
    /// Prompt the OS keychain unlock dialog.
    PromptKeychainUnlock,
    /// Prompt the platform biometric flow (Touch ID, Windows Hello, etc.).
    PromptBiometric,
    /// Prompt the user for a password or passphrase.
    PromptUserPassword,
    /// Ask the user to insert or tap a hardware token.
    InsertHardwareToken,
    /// Restart the OS keychain daemon or the local credential agent.
    RestartKeychainDaemon,
    /// Restart the platform credential agent (ssh-agent, gpg-agent, etc.).
    RestartCredentialAgent,
    /// Open the settings flow to reconfigure the vault adapter.
    ReconfigureVaultAdapter,
    /// Refresh the signed vault mirror over a managed transport.
    RefreshSignedVaultMirror,
    /// Import a fresh signed vault snapshot from an out-of-band channel.
    ImportSignedVaultSnapshot,
    /// Contact the workspace admin to unblock the governing policy.
    ContactAdminToUnblockPolicy,
    /// Re-authenticate through the system browser and recover handles.
    ReauthAndRecoverHandles,
    /// Accept a visibly degraded session-only projection (explicit consent).
    AcceptVisibleDegradedSessionOnly,
}

impl RepairActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::PromptKeychainUnlock => "prompt_keychain_unlock",
            Self::PromptBiometric => "prompt_biometric",
            Self::PromptUserPassword => "prompt_user_password",
            Self::InsertHardwareToken => "insert_hardware_token",
            Self::RestartKeychainDaemon => "restart_keychain_daemon",
            Self::RestartCredentialAgent => "restart_credential_agent",
            Self::ReconfigureVaultAdapter => "reconfigure_vault_adapter",
            Self::RefreshSignedVaultMirror => "refresh_signed_vault_mirror",
            Self::ImportSignedVaultSnapshot => "import_signed_vault_snapshot",
            Self::ContactAdminToUnblockPolicy => "contact_admin_to_unblock_policy",
            Self::ReauthAndRecoverHandles => "reauth_and_recover_handles",
            Self::AcceptVisibleDegradedSessionOnly => "accept_visible_degraded_session_only",
        }
    }
}

/// Typed denial reason carried on a denied-projection row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialReasonClass {
    /// The backing keychain or vault is locked.
    BackingStoreLocked,
    /// The backing store is unreachable (daemon down, adapter misconfigured).
    BackingStoreUnavailable,
    /// A managed-authority vault entry lacks a verified signature posture.
    BackingStoreSignatureMissing,
    /// The underlying secret-broker handle expired.
    HandleExpired,
    /// The underlying secret-broker handle was revoked.
    HandleRevoked,
    /// The secret-broker row's lifecycle state holds authority closed.
    LifecycleStateFailedClosed,
    /// The governing policy forbids the projection.
    PolicyBlocked,
    /// The caller requested a plaintext serialisation.
    PlaintextProjectionRequested,
    /// The caller attempted to reuse a stale snapshot or handle.
    StaleSnapshot,
    /// The caller asked the broker to fall back to a public endpoint.
    PublicEndpointFallbackRequested,
    /// A required approval ticket was missing.
    MissingApproval,
}

impl DenialReasonClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackingStoreLocked => "backing_store_locked",
            Self::BackingStoreUnavailable => "backing_store_unavailable",
            Self::BackingStoreSignatureMissing => "backing_store_signature_missing",
            Self::HandleExpired => "handle_expired",
            Self::HandleRevoked => "handle_revoked",
            Self::LifecycleStateFailedClosed => "lifecycle_state_failed_closed",
            Self::PolicyBlocked => "policy_blocked",
            Self::PlaintextProjectionRequested => "plaintext_projection_requested",
            Self::StaleSnapshot => "stale_snapshot",
            Self::PublicEndpointFallbackRequested => "public_endpoint_fallback_requested",
            Self::MissingApproval => "missing_approval",
        }
    }

    /// True when this denial reason implies a backing-store lock-state row
    /// should be linked from the denial.
    pub const fn implies_store_lock(self) -> bool {
        matches!(
            self,
            Self::BackingStoreLocked
                | Self::BackingStoreUnavailable
                | Self::BackingStoreSignatureMissing
        )
    }
}

/// Typed outcome of a secret-repair attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairOutcomeClass {
    /// Waiting for the user to take the named action.
    AwaitingUser,
    /// Repair action accepted; still completing.
    InProgress,
    /// Repair succeeded; the row is now resolvable.
    Resolved,
    /// The user explicitly declined the repair action.
    UserDeclined,
    /// Repair failed but is expected to succeed on retry.
    FailedTransient,
    /// Repair failed and will not succeed without a different action.
    FailedPermanent,
}

impl RepairOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingUser => "awaiting_user",
            Self::InProgress => "in_progress",
            Self::Resolved => "resolved",
            Self::UserDeclined => "user_declined",
            Self::FailedTransient => "failed_transient",
            Self::FailedPermanent => "failed_permanent",
        }
    }

    /// True when this outcome is terminal and MUST declare a `resolved_at`
    /// timestamp.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Resolved | Self::UserDeclined | Self::FailedPermanent
        )
    }

    /// True when this outcome is still open and MUST leave `resolved_at`
    /// empty.
    pub const fn is_open(self) -> bool {
        matches!(self, Self::AwaitingUser | Self::InProgress)
    }
}

/// One claimed keychain lock-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeychainLockStateRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id safe for UI, logs, and support export.
    pub keychain_lock_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: SecretBrokerBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Backing store the row describes.
    pub backing_store: VaultAdapterClass,
    /// Stable token for [`Self::backing_store`].
    pub backing_store_token: String,
    /// Reviewable adapter label safe for support export.
    pub adapter_label: String,
    /// Opaque store-entry alias safe for support export.
    pub store_entry_alias_ref: String,
    /// Typed lock state observed.
    pub lock_state: KeychainLockStateClass,
    /// Stable token for [`Self::lock_state`].
    pub lock_state_token: String,
    /// Export-safe explanation of the observation.
    pub lock_state_note: String,
    /// Timestamp at which the lock state was observed.
    pub lock_state_observed_at: String,
    /// Consumer affected by the lock state.
    pub affected_consumer: SecretConsumerIdentity,
    /// Opaque target ref the consumer was trying to reach.
    pub affected_target_ref: String,
    /// Opaque workspace scope ref.
    pub affected_workspace_scope_ref: String,
    /// Optional back-reference into the secret-broker beta page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_secret_broker_row_ref: Option<String>,
    /// Repair action the row offers.
    pub repair_action: RepairActionClass,
    /// Stable token for [`Self::repair_action`].
    pub repair_action_token: String,
    /// User-visible repair action label.
    pub repair_action_label: String,
    /// Export-safe explanation of the repair action.
    pub repair_action_detail: String,
    /// Stable opaque remediation-path ref for support and admin replay.
    pub remediation_path_ref: String,
    /// Beta guardrail: raw secret material is not present on the row.
    pub raw_secret_material_present: bool,
    /// Beta guardrail: plaintext fallback was not attempted.
    pub plaintext_fallback_attempted: bool,
    /// Beta guardrail: plaintext fallback was not offered.
    pub plaintext_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this failure mode.
    pub local_editing_preserved: bool,
}

/// One denied-projection row identifying the blocked consumer and required
/// remediation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedProjectionRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id safe for UI, logs, and support export.
    pub denied_projection_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: SecretBrokerBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Consumer that was blocked.
    pub blocked_consumer: SecretConsumerIdentity,
    /// Opaque target ref the consumer was trying to reach.
    pub blocked_target_ref: String,
    /// Opaque workspace scope ref.
    pub blocked_workspace_scope_ref: String,
    /// Secret class the consumer requested.
    pub requested_secret_class: SecretClass,
    /// Stable token for [`Self::requested_secret_class`].
    pub requested_secret_class_token: String,
    /// Projection mode the consumer requested.
    pub requested_projection_mode: HandleProjectionModeClass,
    /// Stable token for [`Self::requested_projection_mode`].
    pub requested_projection_mode_token: String,
    /// Typed denial reason.
    pub denial_reason: DenialReasonClass,
    /// Stable token for [`Self::denial_reason`].
    pub denial_reason_token: String,
    /// Export-safe denial note.
    pub denial_note: String,
    /// Optional link to the originating lock-state row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_lock_state_row_ref: Option<String>,
    /// Optional link to the downstream secret-broker beta row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_secret_broker_row_ref: Option<String>,
    /// Required repair action.
    pub required_repair_action: RepairActionClass,
    /// Stable token for [`Self::required_repair_action`].
    pub required_repair_action_token: String,
    /// User-visible remediation-path label.
    pub remediation_path_label: String,
    /// Stable opaque remediation-path ref for support and admin replay.
    pub remediation_path_ref: String,
    /// Timestamp at which the denial was observed.
    pub observed_at: String,
    /// Beta guardrail: raw secret material is not present on the row.
    pub raw_secret_material_present: bool,
    /// Beta guardrail: plaintext fallback was not offered.
    pub plaintext_fallback_offered: bool,
    /// Beta guardrail: undeclared public-endpoint fallback was not offered.
    pub public_endpoint_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this failure mode.
    pub local_editing_preserved: bool,
}

/// One secret-repair attempt event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRepairActionEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub repair_event_id: String,
    /// Profile under which the event is inspected.
    pub profile: SecretBrokerBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Optional link to the originating lock-state row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keychain_lock_row_ref: Option<String>,
    /// Optional link to the originating denied-projection row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denied_projection_row_ref: Option<String>,
    /// Consumer that the repair attempt covers.
    pub consumer: SecretConsumerIdentity,
    /// Opaque target ref.
    pub target_ref: String,
    /// Opaque workspace scope ref.
    pub workspace_scope_ref: String,
    /// Repair action attempted.
    pub repair_action: RepairActionClass,
    /// Stable token for [`Self::repair_action`].
    pub repair_action_token: String,
    /// Typed outcome of the attempt.
    pub outcome: RepairOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Export-safe note describing the attempt outcome.
    pub repair_note: String,
    /// Timestamp at which the repair attempt was requested.
    pub requested_at: String,
    /// Timestamp at which the repair attempt resolved, when terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Beta guardrail: raw secret material is not present on the event.
    pub raw_secret_material_present: bool,
    /// Beta guardrail: no plaintext fallback was taken.
    pub plaintext_fallback_taken: bool,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretRepairBetaDefectKind {
    /// Record claims raw secret material is present.
    RawSecretMaterialPresent,
    /// Lock-state row attempted a plaintext fallback.
    PlaintextFallbackAttempted,
    /// Row offered a plaintext fallback to the user.
    PlaintextFallbackOffered,
    /// Repair event records that a plaintext fallback was taken.
    PlaintextFallbackTaken,
    /// Row offered an undeclared public-endpoint fallback.
    PublicEndpointFallbackOffered,
    /// Local editing was not preserved through the failure mode.
    LocalEditingNotPreserved,
    /// A non-unlocked lock-state row is missing a repair action.
    RepairActionMissing,
    /// A lock-state row in `Unlocked` state names a repair action other than
    /// `none_required`.
    UnexpectedRepairActionOnUnlocked,
    /// A denied-projection row is missing a typed remediation path.
    RemediationPathMissing,
    /// A denied-projection row is missing a blocked-consumer identity.
    BlockedConsumerMissing,
    /// A denied-projection row implies a store lock but does not link a
    /// lock-state row.
    LinkedLockStateMissing,
    /// A row links a lock-state ref that is not present on the page.
    LinkedLockStateRefUnknown,
    /// A row links a secret-broker row ref or denied-projection row ref that
    /// is not present.
    LinkedRowRefUnknown,
    /// A repair event has neither a lock-state nor a denied-projection link.
    RepairEventUnlinked,
    /// A repair event's outcome is terminal but no `resolved_at` is set.
    TerminalRepairOutcomeMissingResolvedAt,
    /// A repair event is still open but declared a `resolved_at`.
    OpenRepairOutcomeUnexpectedResolvedAt,
    /// One of the four required beta profiles has no claimed row.
    ProfileCoverageMissing,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `backing_store_token` did not match `backing_store`.
    BackingStoreTokenDrift,
    /// `lock_state_token` did not match `lock_state`.
    LockStateTokenDrift,
    /// `repair_action_token` did not match `repair_action`.
    RepairActionTokenDrift,
    /// `denial_reason_token` did not match `denial_reason`.
    DenialReasonTokenDrift,
    /// `requested_secret_class_token` did not match `requested_secret_class`.
    SecretClassTokenDrift,
    /// `requested_projection_mode_token` did not match
    /// `requested_projection_mode`.
    ProjectionModeTokenDrift,
    /// `outcome_token` did not match `outcome`.
    OutcomeTokenDrift,
}

impl SecretRepairBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSecretMaterialPresent => "raw_secret_material_present",
            Self::PlaintextFallbackAttempted => "plaintext_fallback_attempted",
            Self::PlaintextFallbackOffered => "plaintext_fallback_offered",
            Self::PlaintextFallbackTaken => "plaintext_fallback_taken",
            Self::PublicEndpointFallbackOffered => "public_endpoint_fallback_offered",
            Self::LocalEditingNotPreserved => "local_editing_not_preserved",
            Self::RepairActionMissing => "repair_action_missing",
            Self::UnexpectedRepairActionOnUnlocked => "unexpected_repair_action_on_unlocked",
            Self::RemediationPathMissing => "remediation_path_missing",
            Self::BlockedConsumerMissing => "blocked_consumer_missing",
            Self::LinkedLockStateMissing => "linked_lock_state_missing",
            Self::LinkedLockStateRefUnknown => "linked_lock_state_ref_unknown",
            Self::LinkedRowRefUnknown => "linked_row_ref_unknown",
            Self::RepairEventUnlinked => "repair_event_unlinked",
            Self::TerminalRepairOutcomeMissingResolvedAt => {
                "terminal_repair_outcome_missing_resolved_at"
            }
            Self::OpenRepairOutcomeUnexpectedResolvedAt => {
                "open_repair_outcome_unexpected_resolved_at"
            }
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::BackingStoreTokenDrift => "backing_store_token_drift",
            Self::LockStateTokenDrift => "lock_state_token_drift",
            Self::RepairActionTokenDrift => "repair_action_token_drift",
            Self::DenialReasonTokenDrift => "denial_reason_token_drift",
            Self::SecretClassTokenDrift => "secret_class_token_drift",
            Self::ProjectionModeTokenDrift => "projection_mode_token_drift",
            Self::OutcomeTokenDrift => "outcome_token_drift",
        }
    }
}

/// Typed validation defect for the secret-repair beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRepairBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: SecretRepairBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, event id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl SecretRepairBetaDefect {
    fn new(
        defect_kind: SecretRepairBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SECRET_REPAIR_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
            shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the secret-repair beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRepairBetaSummary {
    /// Stable record kind of the parent page.
    pub page_record_kind: String,
    /// Stable record kind of the summary.
    pub record_kind: String,
    /// Number of lock-state rows.
    pub lock_state_row_count: usize,
    /// Number of denied-projection rows.
    pub denied_projection_row_count: usize,
    /// Number of repair events.
    pub repair_event_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Backing-store tokens present across the page.
    pub backing_stores_present: Vec<String>,
    /// Lock-state tokens present across the page.
    pub lock_states_present: Vec<String>,
    /// Denial-reason tokens present across the page.
    pub denial_reasons_present: Vec<String>,
    /// Repair-action tokens present across the page.
    pub repair_actions_present: Vec<String>,
    /// Repair-outcome tokens present across the page.
    pub repair_outcomes_present: Vec<String>,
    /// Counts of repair events by outcome token.
    pub repair_events_by_outcome: BTreeMap<String, usize>,
    /// Counts of denied projections by reason token.
    pub denials_by_reason: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl SecretRepairBetaSummary {
    /// Builds the summary from rows, repair events, and defects.
    pub fn from_records(
        lock_state_rows: &[KeychainLockStateRow],
        denied_projection_rows: &[DeniedProjectionRow],
        repair_events: &[SecretRepairActionEvent],
        defects: &[SecretRepairBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut backing_stores_present: BTreeSet<String> = BTreeSet::new();
        let mut lock_states_present: BTreeSet<String> = BTreeSet::new();
        let mut denial_reasons_present: BTreeSet<String> = BTreeSet::new();
        let mut repair_actions_present: BTreeSet<String> = BTreeSet::new();
        let mut repair_outcomes_present: BTreeSet<String> = BTreeSet::new();

        for row in lock_state_rows {
            profiles_present.insert(row.profile_token.clone());
            backing_stores_present.insert(row.backing_store_token.clone());
            lock_states_present.insert(row.lock_state_token.clone());
            repair_actions_present.insert(row.repair_action_token.clone());
        }
        for row in denied_projection_rows {
            profiles_present.insert(row.profile_token.clone());
            denial_reasons_present.insert(row.denial_reason_token.clone());
            repair_actions_present.insert(row.required_repair_action_token.clone());
        }
        for event in repair_events {
            profiles_present.insert(event.profile_token.clone());
            repair_actions_present.insert(event.repair_action_token.clone());
            repair_outcomes_present.insert(event.outcome_token.clone());
        }

        let mut repair_events_by_outcome: BTreeMap<String, usize> = BTreeMap::new();
        for event in repair_events {
            *repair_events_by_outcome
                .entry(event.outcome_token.clone())
                .or_insert(0) += 1;
        }

        let mut denials_by_reason: BTreeMap<String, usize> = BTreeMap::new();
        for row in denied_projection_rows {
            *denials_by_reason
                .entry(row.denial_reason_token.clone())
                .or_insert(0) += 1;
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: SECRET_REPAIR_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: SECRET_REPAIR_BETA_SUMMARY_RECORD_KIND.to_owned(),
            lock_state_row_count: lock_state_rows.len(),
            denied_projection_row_count: denied_projection_rows.len(),
            repair_event_count: repair_events.len(),
            profiles_present: profiles_present.into_iter().collect(),
            backing_stores_present: backing_stores_present.into_iter().collect(),
            lock_states_present: lock_states_present.into_iter().collect(),
            denial_reasons_present: denial_reasons_present.into_iter().collect(),
            repair_actions_present: repair_actions_present.into_iter().collect(),
            repair_outcomes_present: repair_outcomes_present.into_iter().collect(),
            repair_events_by_outcome,
            denials_by_reason,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level secret-repair beta page consumed by admin, support, shell, and
/// reviewer fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRepairBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Claimed lock-state rows.
    pub lock_state_rows: Vec<KeychainLockStateRow>,
    /// Claimed denied-projection rows.
    pub denied_projection_rows: Vec<DeniedProjectionRow>,
    /// Repair-attempt events.
    pub repair_events: Vec<SecretRepairActionEvent>,
    /// Typed validation defects.
    pub defects: Vec<SecretRepairBetaDefect>,
    /// Aggregate summary.
    pub summary: SecretRepairBetaSummary,
}

/// Support-export wrapper for the secret-repair beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRepairBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: SecretRepairBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw secret values are excluded from the export.
    pub raw_secret_values_excluded: bool,
    /// True when consumer lineage (consumer id, target, scope, repair action,
    /// remediation path, denial reason) is preserved verbatim.
    pub consumer_lineage_preserved: bool,
    /// True when repair-action lineage (lock-state row, denied-projection row,
    /// repair event) is preserved verbatim.
    pub repair_lineage_preserved: bool,
    /// True when the export proves the no-plaintext-fallback invariant.
    pub no_plaintext_fallback_invariant: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl SecretRepairBetaSupportExport {
    /// Builds a support-export wrapper from a beta page. The wrapped page
    /// preserves consumer lineage and repair lineage verbatim; the secret-
    /// repair beta page never carries raw secret bytes, raw runtime handle
    /// ids, or plaintext fallback material, so no row content needs to be
    /// stripped during the export.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: SecretRepairBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: SECRET_REPAIR_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
            shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_secret_values_excluded: true,
            consumer_lineage_preserved: true,
            repair_lineage_preserved: true,
            no_plaintext_fallback_invariant: true,
            redaction_summary:
                "Metadata-only secret-repair beta export: lock-state, denial reason, blocked \
                 consumer, target/scope refs, repair-action lineage, and remediation-path refs \
                 are preserved; raw secret values and plaintext fallback material are excluded \
                 because the beta projection never carries them."
                    .to_owned(),
        }
    }
}

/// Validates the secret-repair beta page and returns typed defects on failure.
pub fn validate_secret_repair_beta_page(
    page: &SecretRepairBetaPage,
) -> Result<(), Vec<SecretRepairBetaDefect>> {
    let defects = audit_secret_repair_beta_page(
        &page.lock_state_rows,
        &page.denied_projection_rows,
        &page.repair_events,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a secret-repair beta page.
pub fn audit_secret_repair_beta_page(
    lock_state_rows: &[KeychainLockStateRow],
    denied_projection_rows: &[DeniedProjectionRow],
    repair_events: &[SecretRepairActionEvent],
) -> Vec<SecretRepairBetaDefect> {
    let mut defects = Vec::new();

    let lock_state_ids: BTreeSet<&str> = lock_state_rows
        .iter()
        .map(|row| row.keychain_lock_row_id.as_str())
        .collect();
    let denied_row_ids: BTreeSet<&str> = denied_projection_rows
        .iter()
        .map(|row| row.denied_projection_row_id.as_str())
        .collect();

    for row in lock_state_rows {
        if row.profile_token != row.profile.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::ProfileTokenDrift,
                row.keychain_lock_row_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if row.backing_store_token != row.backing_store.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::BackingStoreTokenDrift,
                row.keychain_lock_row_id.clone(),
                "backing_store_token",
                "backing_store_token must match backing_store",
            ));
        }
        if row.lock_state_token != row.lock_state.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::LockStateTokenDrift,
                row.keychain_lock_row_id.clone(),
                "lock_state_token",
                "lock_state_token must match lock_state",
            ));
        }
        if row.repair_action_token != row.repair_action.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RepairActionTokenDrift,
                row.keychain_lock_row_id.clone(),
                "repair_action_token",
                "repair_action_token must match repair_action",
            ));
        }

        if row.raw_secret_material_present {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RawSecretMaterialPresent,
                row.keychain_lock_row_id.clone(),
                "raw_secret_material_present",
                "claimed beta row must not carry raw secret material",
            ));
        }
        if row.plaintext_fallback_attempted {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::PlaintextFallbackAttempted,
                row.keychain_lock_row_id.clone(),
                "plaintext_fallback_attempted",
                "claimed beta row must not attempt a plaintext fallback",
            ));
        }
        if row.plaintext_fallback_offered {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::PlaintextFallbackOffered,
                row.keychain_lock_row_id.clone(),
                "plaintext_fallback_offered",
                "claimed beta row must not offer a plaintext fallback",
            ));
        }
        if !row.local_editing_preserved {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::LocalEditingNotPreserved,
                row.keychain_lock_row_id.clone(),
                "local_editing_preserved",
                "claimed beta row must preserve local editing",
            ));
        }

        match (row.lock_state.requires_repair_action(), row.repair_action) {
            (true, RepairActionClass::NoneRequired) => {
                defects.push(SecretRepairBetaDefect::new(
                    SecretRepairBetaDefectKind::RepairActionMissing,
                    row.keychain_lock_row_id.clone(),
                    "repair_action",
                    "non-unlocked lock-state row must declare a typed repair action",
                ));
            }
            (false, action) if !matches!(action, RepairActionClass::NoneRequired) => {
                defects.push(SecretRepairBetaDefect::new(
                    SecretRepairBetaDefectKind::UnexpectedRepairActionOnUnlocked,
                    row.keychain_lock_row_id.clone(),
                    "repair_action",
                    "unlocked lock-state row must declare repair_action=none_required",
                ));
            }
            _ => {}
        }

        if row.lock_state.requires_repair_action()
            && (row.repair_action_label.is_empty() || row.remediation_path_ref.is_empty())
        {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RemediationPathMissing,
                row.keychain_lock_row_id.clone(),
                "repair_action_label/remediation_path_ref",
                "non-unlocked lock-state row must declare a user-visible label and remediation \
                 path ref",
            ));
        }
    }

    for row in denied_projection_rows {
        if row.profile_token != row.profile.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::ProfileTokenDrift,
                row.denied_projection_row_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if row.requested_secret_class_token != row.requested_secret_class.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::SecretClassTokenDrift,
                row.denied_projection_row_id.clone(),
                "requested_secret_class_token",
                "requested_secret_class_token must match requested_secret_class",
            ));
        }
        if row.requested_projection_mode_token != row.requested_projection_mode.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::ProjectionModeTokenDrift,
                row.denied_projection_row_id.clone(),
                "requested_projection_mode_token",
                "requested_projection_mode_token must match requested_projection_mode",
            ));
        }
        if row.denial_reason_token != row.denial_reason.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::DenialReasonTokenDrift,
                row.denied_projection_row_id.clone(),
                "denial_reason_token",
                "denial_reason_token must match denial_reason",
            ));
        }
        if row.required_repair_action_token != row.required_repair_action.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RepairActionTokenDrift,
                row.denied_projection_row_id.clone(),
                "required_repair_action_token",
                "required_repair_action_token must match required_repair_action",
            ));
        }

        if row.raw_secret_material_present {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RawSecretMaterialPresent,
                row.denied_projection_row_id.clone(),
                "raw_secret_material_present",
                "claimed beta row must not carry raw secret material",
            ));
        }
        if row.plaintext_fallback_offered {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::PlaintextFallbackOffered,
                row.denied_projection_row_id.clone(),
                "plaintext_fallback_offered",
                "claimed beta row must not offer a plaintext fallback",
            ));
        }
        if row.public_endpoint_fallback_offered {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::PublicEndpointFallbackOffered,
                row.denied_projection_row_id.clone(),
                "public_endpoint_fallback_offered",
                "claimed beta row must not offer an undeclared public-endpoint fallback",
            ));
        }
        if !row.local_editing_preserved {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::LocalEditingNotPreserved,
                row.denied_projection_row_id.clone(),
                "local_editing_preserved",
                "claimed beta row must preserve local editing",
            ));
        }

        if row.blocked_consumer.consumer_id.is_empty()
            || row.blocked_consumer.consumer_label.is_empty()
            || row.blocked_consumer.capability_hash_ref.is_empty()
        {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::BlockedConsumerMissing,
                row.denied_projection_row_id.clone(),
                "blocked_consumer",
                "denied-projection row must identify the blocked consumer",
            ));
        }

        if row.remediation_path_label.is_empty() || row.remediation_path_ref.is_empty() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RemediationPathMissing,
                row.denied_projection_row_id.clone(),
                "remediation_path_label/remediation_path_ref",
                "denied-projection row must declare a typed remediation path",
            ));
        }

        if row.denial_reason.implies_store_lock() && row.linked_lock_state_row_ref.is_none() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::LinkedLockStateMissing,
                row.denied_projection_row_id.clone(),
                "linked_lock_state_row_ref",
                "store-driven denial must link the originating lock-state row",
            ));
        }
        if let Some(link) = &row.linked_lock_state_row_ref {
            if !lock_state_ids.contains(link.as_str()) {
                defects.push(SecretRepairBetaDefect::new(
                    SecretRepairBetaDefectKind::LinkedLockStateRefUnknown,
                    row.denied_projection_row_id.clone(),
                    "linked_lock_state_row_ref",
                    "linked lock-state row id is not present on the page",
                ));
            }
        }
    }

    for event in repair_events {
        if event.profile_token != event.profile.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::ProfileTokenDrift,
                event.repair_event_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if event.repair_action_token != event.repair_action.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RepairActionTokenDrift,
                event.repair_event_id.clone(),
                "repair_action_token",
                "repair_action_token must match repair_action",
            ));
        }
        if event.outcome_token != event.outcome.as_str() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::OutcomeTokenDrift,
                event.repair_event_id.clone(),
                "outcome_token",
                "outcome_token must match outcome",
            ));
        }
        if event.raw_secret_material_present {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::RawSecretMaterialPresent,
                event.repair_event_id.clone(),
                "raw_secret_material_present",
                "repair event must not carry raw secret material",
            ));
        }
        if event.plaintext_fallback_taken {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::PlaintextFallbackTaken,
                event.repair_event_id.clone(),
                "plaintext_fallback_taken",
                "repair event must not record a plaintext fallback",
            ));
        }

        match (
            event.keychain_lock_row_ref.as_deref(),
            event.denied_projection_row_ref.as_deref(),
        ) {
            (None, None) => {
                defects.push(SecretRepairBetaDefect::new(
                    SecretRepairBetaDefectKind::RepairEventUnlinked,
                    event.repair_event_id.clone(),
                    "keychain_lock_row_ref/denied_projection_row_ref",
                    "repair event must link a lock-state row or a denied-projection row",
                ));
            }
            (lock_ref, denied_ref) => {
                if let Some(link) = lock_ref {
                    if !lock_state_ids.contains(link) {
                        defects.push(SecretRepairBetaDefect::new(
                            SecretRepairBetaDefectKind::LinkedLockStateRefUnknown,
                            event.repair_event_id.clone(),
                            "keychain_lock_row_ref",
                            "linked lock-state row id is not present on the page",
                        ));
                    }
                }
                if let Some(link) = denied_ref {
                    if !denied_row_ids.contains(link) {
                        defects.push(SecretRepairBetaDefect::new(
                            SecretRepairBetaDefectKind::LinkedRowRefUnknown,
                            event.repair_event_id.clone(),
                            "denied_projection_row_ref",
                            "linked denied-projection row id is not present on the page",
                        ));
                    }
                }
            }
        }

        match (event.outcome.is_terminal(), event.resolved_at.as_deref()) {
            (true, None) | (true, Some("")) => {
                defects.push(SecretRepairBetaDefect::new(
                    SecretRepairBetaDefectKind::TerminalRepairOutcomeMissingResolvedAt,
                    event.repair_event_id.clone(),
                    "resolved_at",
                    "terminal repair outcome must declare a resolved_at timestamp",
                ));
            }
            _ => {}
        }
        if event.outcome.is_open() && event.resolved_at.is_some() {
            defects.push(SecretRepairBetaDefect::new(
                SecretRepairBetaDefectKind::OpenRepairOutcomeUnexpectedResolvedAt,
                event.repair_event_id.clone(),
                "resolved_at",
                "open repair outcome must not declare a resolved_at timestamp",
            ));
        }
    }

    let mut observed_profiles: BTreeSet<&str> = BTreeSet::new();
    for row in lock_state_rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in denied_projection_rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    let required_profiles: BTreeSet<&str> = SecretBrokerBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(SecretRepairBetaDefect::new(
            SecretRepairBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    defects
}

/// Builds the seeded secret-repair beta page covering connected, mirror,
/// offline, and enterprise-managed profiles.
pub fn seeded_secret_repair_beta_page() -> SecretRepairBetaPage {
    let lock_state_rows = seed_lock_state_rows();
    let denied_projection_rows = seed_denied_projection_rows();
    let repair_events = seed_repair_events();

    let defects = audit_secret_repair_beta_page(
        &lock_state_rows,
        &denied_projection_rows,
        &repair_events,
    );
    let summary = SecretRepairBetaSummary::from_records(
        &lock_state_rows,
        &denied_projection_rows,
        &repair_events,
        &defects,
    );

    SecretRepairBetaPage {
        record_kind: SECRET_REPAIR_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: SECRET_REPAIR_BETA_SOURCE_MATRIX_REF.to_owned(),
        lock_state_rows,
        denied_projection_rows,
        repair_events,
        defects,
        summary,
    }
}

fn seed_lock_state_rows() -> Vec<KeychainLockStateRow> {
    vec![
        lock_state_row(LockStateSeed {
            row_id: "secret-repair-beta:lock-state:connected:os-keychain-locked",
            display_label: "OS keychain is locked at launch",
            profile: SecretBrokerBetaProfileClass::Connected,
            backing_store: VaultAdapterClass::OsKeychain,
            adapter_label: "OS credential store",
            store_entry_alias_ref: "vault-entry:os-keychain:registry:npm:payments",
            lock_state: KeychainLockStateClass::Locked,
            lock_state_note:
                "Saved registry credential entry exists but the OS keychain has not been \
                 unlocked this session.",
            lock_state_observed_at: "2026-05-16T01:00:10Z",
            consumer_id: "consumer:package-registry:npm",
            consumer_label: "Package registry client",
            capability_hash_ref: "capability-hash:package-registry-client:beta",
            target_ref: "registry:npm:public",
            workspace_scope_ref: "workspace:local:payments",
            affected_secret_broker_row_ref:
                Some("secret-broker-beta:row:connected:registry-auth"),
            repair_action: RepairActionClass::PromptKeychainUnlock,
            repair_action_label: "Unlock keychain",
            repair_action_detail:
                "Prompt the OS keychain unlock dialog so saved registry credentials resolve. \
                 Local editing continues regardless of the unlock decision.",
            remediation_path_ref:
                "remediation-path:os-keychain-unlock:registry:npm:payments",
        }),
        lock_state_row(LockStateSeed {
            row_id:
                "secret-repair-beta:lock-state:mirror_only:vault-mirror-outage",
            display_label: "Self-hosted vault mirror cannot be reached",
            profile: SecretBrokerBetaProfileClass::MirrorOnly,
            backing_store: VaultAdapterClass::EnterpriseVaultSelfHostedMirror,
            adapter_label: "Self-hosted enterprise vault (signed mirror)",
            store_entry_alias_ref:
                "vault-entry:enterprise-vault:mirror:provider:byok-ai:tenant-001",
            lock_state: KeychainLockStateClass::VaultMirrorOutage,
            lock_state_note:
                "Mirror endpoint returned a connection failure on the mirror-only profile.",
            lock_state_observed_at: "2026-05-16T01:05:30Z",
            consumer_id: "consumer:ai-provider:byok",
            consumer_label: "BYOK AI provider client",
            capability_hash_ref: "capability-hash:ai-provider:byok:beta",
            target_ref: "provider:byok-ai:tenant-001",
            workspace_scope_ref: "workspace:remote:payments",
            affected_secret_broker_row_ref:
                Some("secret-broker-beta:row:mirror_only:provider-token"),
            repair_action: RepairActionClass::RefreshSignedVaultMirror,
            repair_action_label: "Refresh signed vault mirror",
            repair_action_detail:
                "Refresh the signed mirror so the provider handle resolves. The previous handle \
                 stays paused until a fresh signature posture is verified.",
            remediation_path_ref:
                "remediation-path:vault-mirror-refresh:provider:byok-ai:tenant-001",
        }),
        lock_state_row(LockStateSeed {
            row_id: "secret-repair-beta:lock-state:offline:vault-snapshot-expired",
            display_label: "Air-gapped vault snapshot is past valid_until",
            profile: SecretBrokerBetaProfileClass::Offline,
            backing_store: VaultAdapterClass::EnterpriseVaultAirGappedSnapshot,
            adapter_label: "Air-gapped enterprise vault snapshot",
            store_entry_alias_ref:
                "vault-entry:enterprise-vault:air-gapped:ssh:deploy:fleet:0001",
            lock_state: KeychainLockStateClass::VaultSnapshotExpired,
            lock_state_note:
                "Air-gapped vault snapshot's valid_until is past; managed authority closed.",
            lock_state_observed_at: "2026-05-16T01:30:15Z",
            consumer_id: "consumer:git:deploy",
            consumer_label: "Git deploy client",
            capability_hash_ref: "capability-hash:git:deploy:beta",
            target_ref: "remote:git:internal-fleet",
            workspace_scope_ref: "workspace:local:fleet",
            affected_secret_broker_row_ref:
                Some("secret-broker-beta:row:offline:ssh-deploy-key"),
            repair_action: RepairActionClass::ImportSignedVaultSnapshot,
            repair_action_label: "Import fresh signed vault snapshot",
            repair_action_detail:
                "Import a fresh signed air-gapped snapshot through the manual delivery path. \
                 The stale snapshot remains paused; no plaintext fallback is admitted.",
            remediation_path_ref:
                "remediation-path:air-gapped-snapshot-import:ssh:deploy:fleet:0001",
        }),
        lock_state_row(LockStateSeed {
            row_id:
                "secret-repair-beta:lock-state:enterprise_managed:policy-injector-misconfigured",
            display_label: "Managed policy injector is misconfigured",
            profile: SecretBrokerBetaProfileClass::EnterpriseManaged,
            backing_store: VaultAdapterClass::ManagedPolicyInjector,
            adapter_label: "Managed policy injector (signed file import)",
            store_entry_alias_ref: "vault-entry:policy-injector:tunnel:fleet:0001",
            lock_state: KeychainLockStateClass::AdapterMisconfigured,
            lock_state_note:
                "Managed policy injector adapter endpoint rejected the workspace-bound \
                 capability hash; admin must reconfigure.",
            lock_state_observed_at: "2026-05-16T01:40:00Z",
            consumer_id: "consumer:tunnel:remote",
            consumer_label: "Remote tunnel client",
            capability_hash_ref: "capability-hash:tunnel:remote:beta",
            target_ref: "tunnel:remote:fleet:0001",
            workspace_scope_ref: "workspace:remote:fleet",
            affected_secret_broker_row_ref:
                Some("secret-broker-beta:row:enterprise_managed:tunnel-delegate"),
            repair_action: RepairActionClass::ContactAdminToUnblockPolicy,
            repair_action_label: "Contact workspace admin",
            repair_action_detail:
                "Ask the workspace admin to reconfigure the managed policy injector. Local \
                 editing continues; no plaintext fallback is admitted.",
            remediation_path_ref:
                "remediation-path:contact-admin:policy-injector:tunnel:fleet:0001",
        }),
        lock_state_row(LockStateSeed {
            row_id: "secret-repair-beta:lock-state:connected:hardware-token-required",
            display_label: "Hardware token required for signing key",
            profile: SecretBrokerBetaProfileClass::Connected,
            backing_store: VaultAdapterClass::HsmOrKmsBacked,
            adapter_label: "Hardware security module",
            store_entry_alias_ref: "vault-entry:hsm:signing:release",
            lock_state: KeychainLockStateClass::HardwareTokenRequired,
            lock_state_note:
                "Release signing key requires a physical hardware token presence.",
            lock_state_observed_at: "2026-05-16T01:15:00Z",
            consumer_id: "consumer:release:signing",
            consumer_label: "Release signing client",
            capability_hash_ref: "capability-hash:release:signing:beta",
            target_ref: "release:signing:fleet",
            workspace_scope_ref: "workspace:local:release",
            affected_secret_broker_row_ref: None,
            repair_action: RepairActionClass::InsertHardwareToken,
            repair_action_label: "Insert and tap hardware token",
            repair_action_detail:
                "Insert the hardware token and tap to authorize the next sign-only projection. \
                 The HSM-backed key bytes never leave the device.",
            remediation_path_ref: "remediation-path:hardware-token:release:signing",
        }),
    ]
}

fn seed_denied_projection_rows() -> Vec<DeniedProjectionRow> {
    vec![
        denied_projection_row(DeniedProjectionSeed {
            row_id:
                "secret-repair-beta:denied:connected:registry-store-locked",
            display_label:
                "Registry projection blocked while OS keychain is locked",
            profile: SecretBrokerBetaProfileClass::Connected,
            blocked_consumer_id: "consumer:package-registry:npm",
            blocked_consumer_label: "Package registry client",
            blocked_capability_hash_ref: "capability-hash:package-registry-client:beta",
            blocked_target_ref: "registry:npm:public",
            blocked_workspace_scope_ref: "workspace:local:payments",
            requested_secret_class: SecretClass::PackageRegistryToken,
            requested_projection_mode: HandleProjectionModeClass::BrokerCallback,
            denial_reason: DenialReasonClass::BackingStoreLocked,
            denial_note:
                "Broker callback denied: the OS keychain is locked, so the saved registry \
                 credential cannot be resolved.",
            linked_lock_state_row_ref:
                Some("secret-repair-beta:lock-state:connected:os-keychain-locked"),
            linked_secret_broker_row_ref:
                Some("secret-broker-beta:row:connected:registry-auth"),
            required_repair_action: RepairActionClass::PromptKeychainUnlock,
            remediation_path_label: "Unlock keychain",
            remediation_path_ref:
                "remediation-path:os-keychain-unlock:registry:npm:payments",
            observed_at: "2026-05-16T01:00:20Z",
        }),
        denied_projection_row(DeniedProjectionSeed {
            row_id:
                "secret-repair-beta:denied:mirror_only:public-endpoint-fallback-refused",
            display_label:
                "Public-endpoint fallback refused on mirror-only profile",
            profile: SecretBrokerBetaProfileClass::MirrorOnly,
            blocked_consumer_id: "consumer:ai-provider:byok",
            blocked_consumer_label: "BYOK AI provider client",
            blocked_capability_hash_ref: "capability-hash:ai-provider:byok:beta",
            blocked_target_ref: "provider:byok-ai:tenant-001",
            blocked_workspace_scope_ref: "workspace:remote:payments",
            requested_secret_class: SecretClass::AiProviderToken,
            requested_projection_mode: HandleProjectionModeClass::RequestHeaderSigner,
            denial_reason: DenialReasonClass::PublicEndpointFallbackRequested,
            denial_note:
                "Caller asked the broker to fall back to a public endpoint after the signed \
                 vault mirror went down; refused on the mirror-only profile.",
            linked_lock_state_row_ref:
                Some("secret-repair-beta:lock-state:mirror_only:vault-mirror-outage"),
            linked_secret_broker_row_ref:
                Some("secret-broker-beta:row:mirror_only:provider-token"),
            required_repair_action: RepairActionClass::RefreshSignedVaultMirror,
            remediation_path_label: "Refresh signed vault mirror",
            remediation_path_ref:
                "remediation-path:vault-mirror-refresh:provider:byok-ai:tenant-001",
            observed_at: "2026-05-16T01:05:45Z",
        }),
        denied_projection_row(DeniedProjectionSeed {
            row_id: "secret-repair-beta:denied:offline:stale-snapshot-refused",
            display_label: "Stale air-gapped snapshot reuse refused",
            profile: SecretBrokerBetaProfileClass::Offline,
            blocked_consumer_id: "consumer:git:deploy",
            blocked_consumer_label: "Git deploy client",
            blocked_capability_hash_ref: "capability-hash:git:deploy:beta",
            blocked_target_ref: "remote:git:internal-fleet",
            blocked_workspace_scope_ref: "workspace:local:fleet",
            requested_secret_class: SecretClass::SshKeyMaterial,
            requested_projection_mode: HandleProjectionModeClass::SignOnly,
            denial_reason: DenialReasonClass::StaleSnapshot,
            denial_note:
                "SSH sign-only projection denied: the air-gapped snapshot is past its \
                 valid_until and managed authority is closed until a fresh signed snapshot is \
                 imported.",
            linked_lock_state_row_ref: Some(
                "secret-repair-beta:lock-state:offline:vault-snapshot-expired",
            ),
            linked_secret_broker_row_ref:
                Some("secret-broker-beta:row:offline:ssh-deploy-key"),
            required_repair_action: RepairActionClass::ImportSignedVaultSnapshot,
            remediation_path_label: "Import fresh signed vault snapshot",
            remediation_path_ref:
                "remediation-path:air-gapped-snapshot-import:ssh:deploy:fleet:0001",
            observed_at: "2026-05-16T01:30:25Z",
        }),
        denied_projection_row(DeniedProjectionSeed {
            row_id:
                "secret-repair-beta:denied:enterprise_managed:plaintext-projection-refused",
            display_label:
                "Plaintext projection request refused by managed policy",
            profile: SecretBrokerBetaProfileClass::EnterpriseManaged,
            blocked_consumer_id: "consumer:tunnel:remote",
            blocked_consumer_label: "Remote tunnel client",
            blocked_capability_hash_ref: "capability-hash:tunnel:remote:beta",
            blocked_target_ref: "tunnel:remote:fleet:0001",
            blocked_workspace_scope_ref: "workspace:remote:fleet",
            requested_secret_class: SecretClass::EphemeralOperationToken,
            requested_projection_mode: HandleProjectionModeClass::TokenExchange,
            denial_reason: DenialReasonClass::PlaintextProjectionRequested,
            denial_note:
                "Caller requested a plaintext serialisation of the delegated tunnel credential; \
                 refused on the enterprise-managed profile. Only token-exchange projection is \
                 admitted.",
            linked_lock_state_row_ref: None,
            linked_secret_broker_row_ref: Some(
                "secret-broker-beta:row:enterprise_managed:tunnel-delegate",
            ),
            required_repair_action: RepairActionClass::ContactAdminToUnblockPolicy,
            remediation_path_label:
                "Contact workspace admin to discuss capability scope",
            remediation_path_ref:
                "remediation-path:contact-admin:policy-injector:tunnel:fleet:0001",
            observed_at: "2026-05-16T01:35:00Z",
        }),
    ]
}

fn seed_repair_events() -> Vec<SecretRepairActionEvent> {
    vec![
        repair_event(RepairEventSeed {
            event_id: "secret-repair-beta:repair:connected:os-keychain-unlock-resolved",
            profile: SecretBrokerBetaProfileClass::Connected,
            keychain_lock_row_ref:
                Some("secret-repair-beta:lock-state:connected:os-keychain-locked"),
            denied_projection_row_ref: Some(
                "secret-repair-beta:denied:connected:registry-store-locked",
            ),
            consumer_id: "consumer:package-registry:npm",
            consumer_label: "Package registry client",
            capability_hash_ref: "capability-hash:package-registry-client:beta",
            target_ref: "registry:npm:public",
            workspace_scope_ref: "workspace:local:payments",
            repair_action: RepairActionClass::PromptKeychainUnlock,
            outcome: RepairOutcomeClass::Resolved,
            repair_note:
                "User unlocked the OS keychain; registry handle resumed without a plaintext \
                 fallback.",
            requested_at: "2026-05-16T01:00:25Z",
            resolved_at: Some("2026-05-16T01:00:38Z"),
        }),
        repair_event(RepairEventSeed {
            event_id:
                "secret-repair-beta:repair:mirror_only:refresh-mirror-in-progress",
            profile: SecretBrokerBetaProfileClass::MirrorOnly,
            keychain_lock_row_ref:
                Some("secret-repair-beta:lock-state:mirror_only:vault-mirror-outage"),
            denied_projection_row_ref: Some(
                "secret-repair-beta:denied:mirror_only:public-endpoint-fallback-refused",
            ),
            consumer_id: "consumer:ai-provider:byok",
            consumer_label: "BYOK AI provider client",
            capability_hash_ref: "capability-hash:ai-provider:byok:beta",
            target_ref: "provider:byok-ai:tenant-001",
            workspace_scope_ref: "workspace:remote:payments",
            repair_action: RepairActionClass::RefreshSignedVaultMirror,
            outcome: RepairOutcomeClass::InProgress,
            repair_note:
                "Mirror refresh accepted; signature verification still pending. Provider handle \
                 stays paused until verification completes.",
            requested_at: "2026-05-16T01:06:00Z",
            resolved_at: None,
        }),
        repair_event(RepairEventSeed {
            event_id: "secret-repair-beta:repair:offline:import-snapshot-awaiting",
            profile: SecretBrokerBetaProfileClass::Offline,
            keychain_lock_row_ref: Some(
                "secret-repair-beta:lock-state:offline:vault-snapshot-expired",
            ),
            denied_projection_row_ref: Some(
                "secret-repair-beta:denied:offline:stale-snapshot-refused",
            ),
            consumer_id: "consumer:git:deploy",
            consumer_label: "Git deploy client",
            capability_hash_ref: "capability-hash:git:deploy:beta",
            target_ref: "remote:git:internal-fleet",
            workspace_scope_ref: "workspace:local:fleet",
            repair_action: RepairActionClass::ImportSignedVaultSnapshot,
            outcome: RepairOutcomeClass::AwaitingUser,
            repair_note:
                "Awaiting manual delivery of a fresh signed snapshot from the air-gapped \
                 transfer channel.",
            requested_at: "2026-05-16T01:30:35Z",
            resolved_at: None,
        }),
        repair_event(RepairEventSeed {
            event_id:
                "secret-repair-beta:repair:enterprise_managed:contact-admin-declined",
            profile: SecretBrokerBetaProfileClass::EnterpriseManaged,
            keychain_lock_row_ref: Some(
                "secret-repair-beta:lock-state:enterprise_managed:policy-injector-misconfigured",
            ),
            denied_projection_row_ref: Some(
                "secret-repair-beta:denied:enterprise_managed:plaintext-projection-refused",
            ),
            consumer_id: "consumer:tunnel:remote",
            consumer_label: "Remote tunnel client",
            capability_hash_ref: "capability-hash:tunnel:remote:beta",
            target_ref: "tunnel:remote:fleet:0001",
            workspace_scope_ref: "workspace:remote:fleet",
            repair_action: RepairActionClass::ContactAdminToUnblockPolicy,
            outcome: RepairOutcomeClass::UserDeclined,
            repair_note:
                "User declined to contact admin; tunnel reuse stays paused. No plaintext \
                 fallback admitted.",
            requested_at: "2026-05-16T01:40:15Z",
            resolved_at: Some("2026-05-16T01:40:55Z"),
        }),
        repair_event(RepairEventSeed {
            event_id:
                "secret-repair-beta:repair:connected:hardware-token-failed-permanent",
            profile: SecretBrokerBetaProfileClass::Connected,
            keychain_lock_row_ref: Some(
                "secret-repair-beta:lock-state:connected:hardware-token-required",
            ),
            denied_projection_row_ref: None,
            consumer_id: "consumer:release:signing",
            consumer_label: "Release signing client",
            capability_hash_ref: "capability-hash:release:signing:beta",
            target_ref: "release:signing:fleet",
            workspace_scope_ref: "workspace:local:release",
            repair_action: RepairActionClass::InsertHardwareToken,
            outcome: RepairOutcomeClass::FailedPermanent,
            repair_note:
                "Hardware token presence check failed permanently for this session. Sign-only \
                 projection remains closed; release signing must retry with a recognized token.",
            requested_at: "2026-05-16T01:15:30Z",
            resolved_at: Some("2026-05-16T01:15:50Z"),
        }),
    ]
}

struct LockStateSeed {
    row_id: &'static str,
    display_label: &'static str,
    profile: SecretBrokerBetaProfileClass,
    backing_store: VaultAdapterClass,
    adapter_label: &'static str,
    store_entry_alias_ref: &'static str,
    lock_state: KeychainLockStateClass,
    lock_state_note: &'static str,
    lock_state_observed_at: &'static str,
    consumer_id: &'static str,
    consumer_label: &'static str,
    capability_hash_ref: &'static str,
    target_ref: &'static str,
    workspace_scope_ref: &'static str,
    affected_secret_broker_row_ref: Option<&'static str>,
    repair_action: RepairActionClass,
    repair_action_label: &'static str,
    repair_action_detail: &'static str,
    remediation_path_ref: &'static str,
}

fn lock_state_row(seed: LockStateSeed) -> KeychainLockStateRow {
    KeychainLockStateRow {
        record_kind: SECRET_REPAIR_BETA_LOCK_STATE_ROW_RECORD_KIND.to_owned(),
        schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
        keychain_lock_row_id: seed.row_id.to_owned(),
        display_label: seed.display_label.to_owned(),
        profile: seed.profile,
        profile_token: seed.profile.as_str().to_owned(),
        backing_store: seed.backing_store,
        backing_store_token: seed.backing_store.as_str().to_owned(),
        adapter_label: seed.adapter_label.to_owned(),
        store_entry_alias_ref: seed.store_entry_alias_ref.to_owned(),
        lock_state: seed.lock_state,
        lock_state_token: seed.lock_state.as_str().to_owned(),
        lock_state_note: seed.lock_state_note.to_owned(),
        lock_state_observed_at: seed.lock_state_observed_at.to_owned(),
        affected_consumer: SecretConsumerIdentity {
            consumer_id: seed.consumer_id.to_owned(),
            consumer_label: seed.consumer_label.to_owned(),
            capability_hash_ref: seed.capability_hash_ref.to_owned(),
        },
        affected_target_ref: seed.target_ref.to_owned(),
        affected_workspace_scope_ref: seed.workspace_scope_ref.to_owned(),
        affected_secret_broker_row_ref: seed.affected_secret_broker_row_ref.map(String::from),
        repair_action: seed.repair_action,
        repair_action_token: seed.repair_action.as_str().to_owned(),
        repair_action_label: seed.repair_action_label.to_owned(),
        repair_action_detail: seed.repair_action_detail.to_owned(),
        remediation_path_ref: seed.remediation_path_ref.to_owned(),
        raw_secret_material_present: false,
        plaintext_fallback_attempted: false,
        plaintext_fallback_offered: false,
        local_editing_preserved: true,
    }
}

struct DeniedProjectionSeed {
    row_id: &'static str,
    display_label: &'static str,
    profile: SecretBrokerBetaProfileClass,
    blocked_consumer_id: &'static str,
    blocked_consumer_label: &'static str,
    blocked_capability_hash_ref: &'static str,
    blocked_target_ref: &'static str,
    blocked_workspace_scope_ref: &'static str,
    requested_secret_class: SecretClass,
    requested_projection_mode: HandleProjectionModeClass,
    denial_reason: DenialReasonClass,
    denial_note: &'static str,
    linked_lock_state_row_ref: Option<&'static str>,
    linked_secret_broker_row_ref: Option<&'static str>,
    required_repair_action: RepairActionClass,
    remediation_path_label: &'static str,
    remediation_path_ref: &'static str,
    observed_at: &'static str,
}

fn denied_projection_row(seed: DeniedProjectionSeed) -> DeniedProjectionRow {
    DeniedProjectionRow {
        record_kind: SECRET_REPAIR_BETA_DENIED_PROJECTION_ROW_RECORD_KIND.to_owned(),
        schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
        denied_projection_row_id: seed.row_id.to_owned(),
        display_label: seed.display_label.to_owned(),
        profile: seed.profile,
        profile_token: seed.profile.as_str().to_owned(),
        blocked_consumer: SecretConsumerIdentity {
            consumer_id: seed.blocked_consumer_id.to_owned(),
            consumer_label: seed.blocked_consumer_label.to_owned(),
            capability_hash_ref: seed.blocked_capability_hash_ref.to_owned(),
        },
        blocked_target_ref: seed.blocked_target_ref.to_owned(),
        blocked_workspace_scope_ref: seed.blocked_workspace_scope_ref.to_owned(),
        requested_secret_class: seed.requested_secret_class,
        requested_secret_class_token: seed.requested_secret_class.as_str().to_owned(),
        requested_projection_mode: seed.requested_projection_mode,
        requested_projection_mode_token: seed.requested_projection_mode.as_str().to_owned(),
        denial_reason: seed.denial_reason,
        denial_reason_token: seed.denial_reason.as_str().to_owned(),
        denial_note: seed.denial_note.to_owned(),
        linked_lock_state_row_ref: seed.linked_lock_state_row_ref.map(String::from),
        linked_secret_broker_row_ref: seed.linked_secret_broker_row_ref.map(String::from),
        required_repair_action: seed.required_repair_action,
        required_repair_action_token: seed.required_repair_action.as_str().to_owned(),
        remediation_path_label: seed.remediation_path_label.to_owned(),
        remediation_path_ref: seed.remediation_path_ref.to_owned(),
        observed_at: seed.observed_at.to_owned(),
        raw_secret_material_present: false,
        plaintext_fallback_offered: false,
        public_endpoint_fallback_offered: false,
        local_editing_preserved: true,
    }
}

struct RepairEventSeed {
    event_id: &'static str,
    profile: SecretBrokerBetaProfileClass,
    keychain_lock_row_ref: Option<&'static str>,
    denied_projection_row_ref: Option<&'static str>,
    consumer_id: &'static str,
    consumer_label: &'static str,
    capability_hash_ref: &'static str,
    target_ref: &'static str,
    workspace_scope_ref: &'static str,
    repair_action: RepairActionClass,
    outcome: RepairOutcomeClass,
    repair_note: &'static str,
    requested_at: &'static str,
    resolved_at: Option<&'static str>,
}

fn repair_event(seed: RepairEventSeed) -> SecretRepairActionEvent {
    SecretRepairActionEvent {
        record_kind: SECRET_REPAIR_BETA_REPAIR_EVENT_RECORD_KIND.to_owned(),
        schema_version: SECRET_REPAIR_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_REPAIR_BETA_SHARED_CONTRACT_REF.to_owned(),
        repair_event_id: seed.event_id.to_owned(),
        profile: seed.profile,
        profile_token: seed.profile.as_str().to_owned(),
        keychain_lock_row_ref: seed.keychain_lock_row_ref.map(String::from),
        denied_projection_row_ref: seed.denied_projection_row_ref.map(String::from),
        consumer: SecretConsumerIdentity {
            consumer_id: seed.consumer_id.to_owned(),
            consumer_label: seed.consumer_label.to_owned(),
            capability_hash_ref: seed.capability_hash_ref.to_owned(),
        },
        target_ref: seed.target_ref.to_owned(),
        workspace_scope_ref: seed.workspace_scope_ref.to_owned(),
        repair_action: seed.repair_action,
        repair_action_token: seed.repair_action.as_str().to_owned(),
        outcome: seed.outcome,
        outcome_token: seed.outcome.as_str().to_owned(),
        repair_note: seed.repair_note.to_owned(),
        requested_at: seed.requested_at.to_owned(),
        resolved_at: seed.resolved_at.map(String::from),
        raw_secret_material_present: false,
        plaintext_fallback_taken: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_secret_repair_beta_page();
        validate_secret_repair_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        for profile in SecretBrokerBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
    }

    #[test]
    fn seeded_page_covers_lock_state_denied_and_repair_records() {
        let page = seeded_secret_repair_beta_page();
        assert!(page.lock_state_rows.len() >= 4);
        assert!(page.denied_projection_rows.len() >= 4);
        assert!(page.repair_events.len() >= 4);
        let outcomes: BTreeSet<&str> = page
            .summary
            .repair_outcomes_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(outcomes.contains("resolved"));
        assert!(outcomes.contains("awaiting_user"));
        assert!(outcomes.contains("in_progress"));
    }

    #[test]
    fn validator_flags_plaintext_fallback_attempted_on_lock_state() {
        let mut page = seeded_secret_repair_beta_page();
        page.lock_state_rows[0].plaintext_fallback_attempted = true;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretRepairBetaDefectKind::PlaintextFallbackAttempted));
    }

    #[test]
    fn validator_flags_plaintext_fallback_offered_on_denied_row() {
        let mut page = seeded_secret_repair_beta_page();
        page.denied_projection_rows[0].plaintext_fallback_offered = true;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretRepairBetaDefectKind::PlaintextFallbackOffered));
    }

    #[test]
    fn validator_flags_plaintext_fallback_taken_on_repair_event() {
        let mut page = seeded_secret_repair_beta_page();
        page.repair_events[0].plaintext_fallback_taken = true;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == SecretRepairBetaDefectKind::PlaintextFallbackTaken));
    }

    #[test]
    fn validator_flags_repair_action_missing_on_non_unlocked_row() {
        let mut page = seeded_secret_repair_beta_page();
        page.lock_state_rows[0].repair_action = RepairActionClass::NoneRequired;
        page.lock_state_rows[0].repair_action_token =
            RepairActionClass::NoneRequired.as_str().to_owned();
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == SecretRepairBetaDefectKind::RepairActionMissing));
    }

    #[test]
    fn validator_flags_missing_remediation_path_on_denied_row() {
        let mut page = seeded_secret_repair_beta_page();
        page.denied_projection_rows[0].remediation_path_ref.clear();
        page.denied_projection_rows[0]
            .remediation_path_label
            .clear();
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == SecretRepairBetaDefectKind::RemediationPathMissing
        ));
    }

    #[test]
    fn validator_flags_missing_blocked_consumer_on_denied_row() {
        let mut page = seeded_secret_repair_beta_page();
        page.denied_projection_rows[0]
            .blocked_consumer
            .consumer_id
            .clear();
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == SecretRepairBetaDefectKind::BlockedConsumerMissing
        ));
    }

    #[test]
    fn validator_flags_terminal_outcome_missing_resolved_at() {
        let mut page = seeded_secret_repair_beta_page();
        let event = page
            .repair_events
            .iter_mut()
            .find(|event| event.outcome == RepairOutcomeClass::Resolved)
            .expect("seeded resolved event");
        event.resolved_at = None;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretRepairBetaDefectKind::TerminalRepairOutcomeMissingResolvedAt));
    }

    #[test]
    fn validator_flags_open_outcome_with_unexpected_resolved_at() {
        let mut page = seeded_secret_repair_beta_page();
        let event = page
            .repair_events
            .iter_mut()
            .find(|event| event.outcome == RepairOutcomeClass::AwaitingUser)
            .expect("seeded awaiting_user event");
        event.resolved_at = Some("2026-05-16T01:31:00Z".to_owned());
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretRepairBetaDefectKind::OpenRepairOutcomeUnexpectedResolvedAt));
    }

    #[test]
    fn validator_flags_profile_coverage_missing() {
        let mut page = seeded_secret_repair_beta_page();
        page.lock_state_rows
            .retain(|row| row.profile != SecretBrokerBetaProfileClass::Offline);
        page.denied_projection_rows
            .retain(|row| row.profile != SecretBrokerBetaProfileClass::Offline);
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretRepairBetaDefectKind::ProfileCoverageMissing
            && defect.note.contains("offline")));
    }

    #[test]
    fn validator_flags_store_lock_denial_without_linked_lock_state() {
        let mut page = seeded_secret_repair_beta_page();
        let row = page
            .denied_projection_rows
            .iter_mut()
            .find(|row| row.denial_reason == DenialReasonClass::BackingStoreLocked)
            .expect("seeded store-lock denial");
        row.linked_lock_state_row_ref = None;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == SecretRepairBetaDefectKind::LinkedLockStateMissing
        ));
    }

    #[test]
    fn validator_flags_unlinked_repair_event() {
        let mut page = seeded_secret_repair_beta_page();
        page.repair_events[0].keychain_lock_row_ref = None;
        page.repair_events[0].denied_projection_row_ref = None;
        let defects = audit_secret_repair_beta_page(
            &page.lock_state_rows,
            &page.denied_projection_rows,
            &page.repair_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == SecretRepairBetaDefectKind::RepairEventUnlinked));
    }

    #[test]
    fn support_export_round_trip_preserves_lineage() {
        let page = seeded_secret_repair_beta_page();
        let export = SecretRepairBetaSupportExport::from_page(
            "secret-repair-beta:support-export:001",
            "2026-05-16T05:00:00Z",
            page,
        );
        assert!(export.raw_secret_values_excluded);
        assert!(export.consumer_lineage_preserved);
        assert!(export.repair_lineage_preserved);
        assert!(export.no_plaintext_fallback_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_secret_repair_beta_page();
        assert_eq!(page.summary.lock_state_row_count, page.lock_state_rows.len());
        assert_eq!(
            page.summary.denied_projection_row_count,
            page.denied_projection_rows.len()
        );
        assert_eq!(page.summary.repair_event_count, page.repair_events.len());
        let denial_total: usize = page.summary.denials_by_reason.values().sum();
        assert_eq!(denial_total, page.denied_projection_rows.len());
        let event_total: usize = page.summary.repair_events_by_outcome.values().sum();
        assert_eq!(event_total, page.repair_events.len());
    }
}
