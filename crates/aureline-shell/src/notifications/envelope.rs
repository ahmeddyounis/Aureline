//! Typed Rust mirror of the notification envelope boundary contract.
//!
//! These structs and enums mirror the frozen schema at
//! [`/schemas/ux/notification_envelope.schema.json`] and the contract at
//! [`/docs/ux/notification_envelope_contract.md`]. Callers should treat the
//! schema as canonical: when the schema's enum vocabulary changes, this
//! module's enums must change in the same patch. The shell does not invent a
//! parallel notification vocabulary — it deserializes envelopes and routes
//! them.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Schema version of the notification envelope payload this module
/// understands. Must match
/// [`schemas/ux/notification_envelope.schema.json`].
pub const NOTIFICATION_ENVELOPE_SCHEMA_VERSION: u32 = 1;

/// Schema version of the embedded fanout receipt payload.
pub const FANOUT_RECEIPT_SCHEMA_VERSION: u32 = 1;

/// Source subsystem the notification originated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceSubsystem {
    Editor,
    Terminal,
    ReviewAndDiff,
    PaletteAndSearch,
    InstallUpdateAttach,
    AiApply,
    Collaboration,
    ProviderBearing,
    DocsHelpServiceHealth,
    SupportExport,
    BuildSystem,
    TestRunner,
    DebugSession,
    TaskRunner,
    Indexer,
    VfsSave,
    SyncMirror,
    NotebookKernel,
    RemoteAgent,
    ExtensionHost,
    WorkspaceTrust,
    PolicyResolver,
    AdminPolicy,
    SecretBroker,
    RuntimePowerManager,
    Shell,
}

/// Severity vocabulary used for routing and escalation. Stable enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityClass {
    Info,
    Success,
    Warning,
    Degraded,
    Error,
    Blocking,
    Critical,
}

/// Coarse privacy gate applied to the OS payload, companion summary, and
/// support-export posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyClass {
    SummarySafe,
    WorkspaceSensitive,
    SecurityCritical,
    ManagedSensitive,
}

/// Concrete payload redaction posture for OS, lock-screen, and companion
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyPayloadClass {
    LockScreenSafeGeneric,
    LockScreenSafeScoped,
    InProductOnly,
    RedactedMetadataOnly,
    PolicyForbiddenOnLockScreen,
}

/// Export / retention redaction posture applied to durable sinks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

/// Dedupe key scheme the router collapses repeats with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedupeKeyScheme {
    CanonicalEventId,
    CanonicalObjectTargetPlusEventClass,
    GroupedBurstId,
    SubsystemPlusObjectPlusPhase,
    CrossClientCanonicalEventId,
}

/// Typed surface set the router consumes from `recommended_surfaces[]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutSurfaceClass {
    DurableJobRow,
    StatusStrip,
    StatusItem,
    ActivityCenterDigestCard,
    Toast,
    ContextualBanner,
    OsNotification,
    LockScreenSummary,
    CompanionPush,
    DigestGroupRow,
    NotDeliveredHeld,
}

impl FanoutSurfaceClass {
    /// Stable token used in fixtures, snapshots, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableJobRow => "durable_job_row",
            Self::StatusStrip => "status_strip",
            Self::StatusItem => "status_item",
            Self::ActivityCenterDigestCard => "activity_center_digest_card",
            Self::Toast => "toast",
            Self::ContextualBanner => "contextual_banner",
            Self::OsNotification => "os_notification",
            Self::LockScreenSummary => "lock_screen_summary",
            Self::CompanionPush => "companion_push",
            Self::DigestGroupRow => "digest_group_row",
            Self::NotDeliveredHeld => "not_delivered_held",
        }
    }
}

/// Quiet-hours / focus / admin mode active when the envelope was minted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuietHoursMode {
    ModeNone,
    ModeQuietHoursUser,
    ModeDoNotDisturbUser,
    ModeFocusModeUser,
    ModePresentation,
    ModeScreenShare,
    ModePrivacyMode,
    ModeReducedAttentionPolicy,
    ModePowerSaverRuntime,
    ModeAdminSuppression,
}

/// Reason that a fanout was held, suppressed, deduped, or muted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionReason {
    QuietHoursUserPolicy,
    DoNotDisturbUserPolicy,
    FocusModeUserPolicy,
    PresentationModeActive,
    ScreenShareActive,
    PrivacyModeActive,
    AdminPolicySuppression,
    ReducedAttentionPosture,
    PowerSaverBackgroundPause,
    DedupeSameCanonicalEvent,
    DedupeSameGroupedBurst,
    ClassMutedByUser,
    ClassSnoozedByUser,
    ReleasePendingNextUnsuppressedSurface,
}

/// Client scope the receipt was issued for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    DesktopProduct,
    Cli,
    CompanionSurface,
    RemoteAgent,
    SdkOrApi,
    ManagedAdminSurface,
}

/// Reopen-target kind. Generic home-screen reopen is intentionally absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenTargetKind {
    CanonicalObject,
    CanonicalRoute,
    ReviewContext,
    DurableActivityRow,
    DigestGroup,
    PlaceholderAnnounced,
    DeniedRequiresRevalidation,
}

/// Typed fanout receipt outcome carried by routed surface deliveries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutReceiptState {
    Delivered,
    HeldQuietHours,
    SuppressedPolicy,
    DedupedCanonicalEvent,
    DedupedGroupedBurst,
    ReleasedFromHold,
    NotAttemptedNoRoute,
}

/// Typed reason class for a stale or undelivered fanout receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleOrUndeliveredReasonClass {
    None,
    HeldByQuietHours,
    SuppressedByPolicy,
    DedupedOnCanonicalEvent,
    DedupedOnGroupedBurst,
    NoRouteForSurfaceOrTier,
    StaleBeforeDelivery,
    ExpiredBeforeDelivery,
    TransportFailed,
    Unknown,
}

/// Reopen-target block carried by every envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTarget {
    pub reopen_target_ref: String,
    pub reopen_target_kind: ReopenTargetKind,
    pub exact_target_identity_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_announcement_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revalidation_required_reason_label: Option<String>,
}

impl ReopenTarget {
    /// True when this reopen target resolves to a concrete canonical object,
    /// route, review context, durable row, or digest group.
    pub fn resolves_to_exact_target(&self) -> bool {
        match self.reopen_target_kind {
            ReopenTargetKind::CanonicalObject
            | ReopenTargetKind::CanonicalRoute
            | ReopenTargetKind::ReviewContext
            | ReopenTargetKind::DurableActivityRow
            | ReopenTargetKind::DigestGroup => self.exact_target_identity_ref.is_some(),
            ReopenTargetKind::PlaceholderAnnounced
            | ReopenTargetKind::DeniedRequiresRevalidation => false,
        }
    }
}

/// Stable command-backed action carried by the envelope. Surfaces bind
/// behavior to `action_id` + `command_id`, never to `label`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableAction {
    pub action_id: String,
    pub label: String,
    pub command_id: String,
    pub target_identity_ref: String,
    pub reopen_target_kind: ReopenTargetKind,
    #[serde(default)]
    pub is_destructive: bool,
}

/// Quiet-hours / suppression facts captured at envelope mint time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionState {
    pub active_modes_at_mint: Vec<QuietHoursMode>,
    pub suppression_reasons: Vec<SuppressionReason>,
    #[serde(default)]
    pub suppressed: bool,
}

impl SuppressionState {
    /// True when at least one non-`mode_none` mode is active.
    pub fn has_active_quiet_mode(&self) -> bool {
        self.active_modes_at_mint
            .iter()
            .any(|mode| !matches!(mode, QuietHoursMode::ModeNone))
    }

    /// Returns the modes set without the trivial `mode_none` baseline.
    pub fn non_trivial_modes(&self) -> HashSet<QuietHoursMode> {
        self.active_modes_at_mint
            .iter()
            .copied()
            .filter(|mode| !matches!(mode, QuietHoursMode::ModeNone))
            .collect()
    }
}

/// Stale / undelivered reason payload embedded in fanout receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleOrUndeliveredReason {
    pub reason_class: StaleOrUndeliveredReasonClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_label: Option<String>,
}

impl StaleOrUndeliveredReason {
    pub const fn none() -> Self {
        Self {
            reason_class: StaleOrUndeliveredReasonClass::None,
            reason_label: None,
        }
    }
}

/// One fanout receipt. Held, suppressed, deduped, and no-route outcomes still
/// emit a receipt — silent drops are non-conforming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutReceipt {
    pub record_kind: String,
    pub fanout_receipt_schema_version: u32,
    pub fanout_receipt_id: String,
    pub source_notification_envelope_id_ref: String,
    pub canonical_event_id: String,
    pub event_lineage_id_ref: String,
    pub fanout_surface_class: FanoutSurfaceClass,
    pub client_scope: ClientScope,
    pub receipt_state: FanoutReceiptState,
    pub stale_or_undelivered_reason: StaleOrUndeliveredReason,
    pub dedupe_key_scheme: DedupeKeyScheme,
    pub delivery_envelope_ref: String,
    pub reopen_target_ref: String,
    pub redaction_class: RedactionClass,
    #[serde(default)]
    pub suppression_reasons: Vec<SuppressionReason>,
    pub minted_at: String,
}

/// One typed notification envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelope {
    pub record_kind: String,
    pub notification_envelope_schema_version: u32,
    pub notification_envelope_id: String,
    pub canonical_event_id: String,
    pub event_lineage_id_ref: String,
    pub source_subsystem: SourceSubsystem,
    pub source_event_ref: String,
    pub actor_identity_ref: String,
    pub canonical_object_target_ref: String,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub privacy_payload_class: PrivacyPayloadClass,
    pub redaction_class: RedactionClass,
    pub dedupe_key_scheme: DedupeKeyScheme,
    pub dedupe_key_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_burst_id_ref: Option<String>,
    pub recommended_surfaces: Vec<FanoutSurfaceClass>,
    pub summary_label: String,
    pub reopen_target: ReopenTarget,
    pub actions: Vec<StableAction>,
    pub suppression_state: SuppressionState,
    #[serde(default)]
    pub fanout_receipts: Vec<FanoutReceipt>,
    pub minted_at: String,
}

impl NotificationEnvelope {
    /// Stable join key used by the router to dedupe repeats.
    pub fn dedupe_join_key(&self) -> String {
        match self.dedupe_key_scheme {
            DedupeKeyScheme::GroupedBurstId => self
                .grouped_burst_id_ref
                .clone()
                .unwrap_or_else(|| self.dedupe_key_ref.clone()),
            _ => self.dedupe_key_ref.clone(),
        }
    }
}
