//! Beta-grade notification privacy, quiet-hours, badge, and cross-client
//! dedupe projection for claimed attention surfaces.
//!
//! This module promotes the existing notification primitives
//! ([`super::envelope`], [`super::router`], [`super::quiet_hours`],
//! [`super::external`], [`super::actions`], [`super::audit`]) to a
//! page-level beta projection a reviewer can inspect on every claimed
//! attention row.
//!
//! It does NOT mint a parallel notification vocabulary. Each
//! [`NotificationPrivacyBetaRow`] is a typed cross-reference to:
//!
//! - the envelope's `privacy_class` / `privacy_payload_class` /
//!   `redaction_class`,
//! - the envelope's `dedupe_key_scheme` and (where applicable) grouped
//!   burst lineage,
//! - the badge class the durable activity row contributes to
//!   ([`super::actions::BadgeClass`]),
//! - the quiet-hours posture that holds or bypasses the attention
//!   surface ([`super::quiet_hours::QuietHoursPosture`]),
//! - the closed [`super::external::ForbiddenShortcutActionClass`] set
//!   the OS / lock-screen / companion payload refuses to complete.
//!
//! The beta promise pinned by this projection is the four acceptance
//! gates from the M3 task:
//!
//! 1. **Stable classes.** Every row promises a stable
//!    `(privacy_class, privacy_payload_class, severity_class,
//!    badge_class, dedupe_key_scheme)` tuple — never a per-surface ad
//!    hoc routing decision.
//! 2. **Lock-screen privacy.** Sensitive details never reach the
//!    lock-screen / OS payload unless the privacy class explicitly
//!    permits it. The validator rejects any row whose payload class is
//!    strictly more permissive than the privacy class allows.
//! 3. **Repeated-failure coalescing.** Retry storms collapse into one
//!    durable item: the row carries an `expected_coalescing_posture`
//!    block (whose `dedupe_key_scheme` must be `grouped_burst_id` or
//!    `subsystem_plus_object_plus_phase`) and a deduped occurrence
//!    count from the live router state.
//! 4. **Cross-client dedupe.** Companion / remote-agent / managed-admin
//!    fanout collapses under `cross_client_canonical_event_id` — the
//!    row enumerates the sibling client scopes that share the canonical
//!    event id so a desktop dismissal does not strand a companion row.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::actions::BadgeClass;
use super::envelope::{
    ClientScope, DedupeKeyScheme, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass,
    PrivacyPayloadClass, QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind,
    SeverityClass, SourceSubsystem, StableAction, StaleOrUndeliveredReasonClass, SuppressionState,
    NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
};
use super::external::{ExternalNotificationPayload, ForbiddenShortcutActionClass};
use super::quiet_hours::{DurableBadgeProjection, QuietHoursPosture};
use super::router::{NotificationRouter, RoutedNotification};

/// Beta schema version exported with every record.
pub const NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta record.
pub const NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF: &str =
    "shell:notification_privacy_beta:v1";

/// Stable record kind for [`NotificationPrivacyBetaPage`] payloads.
pub const NOTIFICATION_PRIVACY_BETA_PAGE_RECORD_KIND: &str =
    "shell_notification_privacy_beta_page_record";

/// Stable record kind for [`NotificationPrivacyBetaRow`] payloads.
pub const NOTIFICATION_PRIVACY_BETA_ROW_RECORD_KIND: &str =
    "shell_notification_privacy_beta_row_record";

/// Stable record kind for [`NotificationPrivacyBetaBadge`] payloads.
pub const NOTIFICATION_PRIVACY_BETA_BADGE_RECORD_KIND: &str =
    "shell_notification_privacy_beta_badge_record";

/// Stable record kind for [`NotificationPrivacyBetaSupportExportRow`].
pub const NOTIFICATION_PRIVACY_BETA_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_notification_privacy_beta_support_export_row_record";

/// Stable record kind for [`NotificationPrivacyBetaSupportExport`].
pub const NOTIFICATION_PRIVACY_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_notification_privacy_beta_support_export_record";

/// Coarse attention-row class. One class per claimed beta scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPrivacyBetaRowClass {
    /// First emission delivered cleanly to durable, status, and toast.
    DeliveredSummarySafe,
    /// Repeated retry burst on one pipeline coalesces into one durable
    /// row; the badge increments by deduped object, not by raw event.
    CoalescedRepeatedFailure,
    /// Lock-screen-safe payload renders generic-only labels; in-product
    /// reopen routes through the canonical object.
    LockScreenSafeGenericPayload,
    /// Workspace-trust review payload renders scoped labels; reopen
    /// routes through the in-product trust review canvas.
    LockScreenSafeScopedPayload,
    /// Security-critical payload denies lock-screen render outright;
    /// in-product surfaces still light up and the badge still counts.
    LockScreenForbiddenSecurityCritical,
    /// Quiet-hours-user holds attention surfaces; durable truth and OS
    /// app-icon badge are honored by the posture.
    QuietHoursHeld,
    /// Admin-suppression denies attention surfaces outright; durable
    /// truth is preserved.
    AdminPolicySuppressed,
    /// Critical-severity event always interrupts; no quiet-hours mode
    /// may suppress it.
    CriticalSafetyEscalation,
    /// Companion / remote-agent siblings collapse under
    /// `cross_client_canonical_event_id`; a desktop dismissal does not
    /// strand the companion row.
    CompanionCrossClientFanout,
    /// OS / lock-screen / companion payload refuses to complete any
    /// forbidden shortcut action class; mutation routes through the
    /// in-product review path.
    ForbiddenShortcutBypassRefused,
}

impl NotificationPrivacyBetaRowClass {
    /// Stable token recorded in fixtures, exports, and projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeliveredSummarySafe => "delivered_summary_safe",
            Self::CoalescedRepeatedFailure => "coalesced_repeated_failure",
            Self::LockScreenSafeGenericPayload => "lock_screen_safe_generic_payload",
            Self::LockScreenSafeScopedPayload => "lock_screen_safe_scoped_payload",
            Self::LockScreenForbiddenSecurityCritical => {
                "lock_screen_forbidden_security_critical"
            }
            Self::QuietHoursHeld => "quiet_hours_held",
            Self::AdminPolicySuppressed => "admin_policy_suppressed",
            Self::CriticalSafetyEscalation => "critical_safety_escalation",
            Self::CompanionCrossClientFanout => "companion_cross_client_fanout",
            Self::ForbiddenShortcutBypassRefused => "forbidden_shortcut_bypass_refused",
        }
    }
}

/// Lock-screen / OS / companion payload posture promised by the row.
///
/// The beta validator binds this enum to the envelope's
/// `privacy_payload_class` so the row, the badge, and the support-export
/// row cannot drift on lock-screen safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockScreenPosture {
    /// Generic category-class labels only. The default lock-screen
    /// payload class — never names a workspace, object, actor, raw
    /// path, raw URL, or any privileged token.
    GenericSummaryOnly,
    /// Scoped labels (workspace, session, next-action) — never object
    /// identity, actor real name, or diff excerpt.
    ScopedWorkspaceSafe,
    /// In-product only; the lock-screen surface is denied. The durable
    /// row remains the truth source.
    InProductOnly,
    /// Metadata-only mirror; raw bodies, paths, URLs are stripped.
    RedactedMetadataOnly,
    /// Lock-screen render is forbidden outright by the privacy class.
    PolicyForbiddenOnLockScreen,
}

impl LockScreenPosture {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GenericSummaryOnly => "generic_summary_only",
            Self::ScopedWorkspaceSafe => "scoped_workspace_safe",
            Self::InProductOnly => "in_product_only",
            Self::RedactedMetadataOnly => "redacted_metadata_only",
            Self::PolicyForbiddenOnLockScreen => "policy_forbidden_on_lock_screen",
        }
    }
}

impl LockScreenPosture {
    /// Project the row's lock-screen posture from the envelope's payload
    /// class.
    pub const fn from_payload_class(payload: PrivacyPayloadClass) -> Self {
        match payload {
            PrivacyPayloadClass::LockScreenSafeGeneric => Self::GenericSummaryOnly,
            PrivacyPayloadClass::LockScreenSafeScoped => Self::ScopedWorkspaceSafe,
            PrivacyPayloadClass::InProductOnly => Self::InProductOnly,
            PrivacyPayloadClass::RedactedMetadataOnly => Self::RedactedMetadataOnly,
            PrivacyPayloadClass::PolicyForbiddenOnLockScreen => Self::PolicyForbiddenOnLockScreen,
        }
    }
}

/// Quiet-hours posture observed at routing time.
///
/// `bypassed_by_critical_severity` is the audited bypass: it MUST be
/// `true` only when the envelope's severity is `Critical` and at least
/// one non-`mode_none` mode was active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaQuietHoursPosture {
    /// Stable list of active modes recorded at routing time. Empty when
    /// no quiet mode applied.
    pub active_modes: Vec<QuietHoursMode>,
    /// True when the active posture held any attention surface for this
    /// envelope (toast / banner / OS notification / lock-screen
    /// summary / companion push).
    pub holds_attention_surfaces: bool,
    /// True when at least one durable surface (`durable_job_row`,
    /// `status_item`, `status_strip`, `activity_center_digest_card`)
    /// still delivered. Durable truth is non-negotiable.
    pub durable_truth_preserved: bool,
    /// True when the posture suppressed the OS app-icon badge.
    /// In-product badge still renders either way.
    pub os_app_icon_badge_suppressed: bool,
    /// True when the lock-screen summary surface was denied or held.
    pub lock_screen_summary_suppressed: bool,
    /// True when a critical-safety severity bypassed the active modes.
    pub bypassed_by_critical_severity: bool,
}

/// Cross-client fanout posture observed on the routed event.
///
/// Sibling scopes share the envelope's `canonical_event_id`. The
/// validator confirms each declared sibling appears at most once and
/// that `dedupe_key_scheme = cross_client_canonical_event_id` is in
/// effect when more than one scope is enumerated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossClientFanoutPosture {
    /// Originating client scope (always `desktop_product` for shell
    /// rows).
    pub originating_scope: ClientScope,
    /// Sibling client scopes that share the same canonical event id.
    pub sibling_scopes: Vec<ClientScope>,
    /// True when the row's dedupe scheme is the cross-client variant.
    pub cross_client_dedupe_in_effect: bool,
    /// True when companion / remote-agent payloads carry only the
    /// privacy posture declared on the lineage (never widened).
    pub payload_class_not_widened_across_clients: bool,
    /// True when cross-client dismissal collapses (so a desktop
    /// acknowledge does not strand a companion row).
    pub cross_client_dismissal_collapses: bool,
}

impl CrossClientFanoutPosture {
    /// Build a desktop-only posture (no siblings; cross-client dedupe
    /// not in effect for this row).
    pub fn desktop_only() -> Self {
        Self {
            originating_scope: ClientScope::DesktopProduct,
            sibling_scopes: Vec::new(),
            cross_client_dedupe_in_effect: false,
            payload_class_not_widened_across_clients: true,
            cross_client_dismissal_collapses: true,
        }
    }
}

/// Expected dedupe / coalescing posture for the row.
///
/// The validator enforces this against the envelope's `dedupe_key_scheme`
/// so a retry storm cannot accidentally route under
/// `canonical_event_id` and inflate the badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedCoalescingPosture {
    /// Expected dedupe scheme.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Expected badge class the row contributes to.
    pub badge_class: BadgeClass,
    /// True when this row represents a retry burst that must collapse
    /// onto one durable item. When `true`, the validator rejects a
    /// `canonical_event_id` scheme.
    pub coalesces_retry_burst: bool,
    /// True when occurrence-count growth never causes per-toast
    /// duplicates: the chrome MUST increment the row's
    /// `occurrence_count` instead of spawning a new toast.
    pub repeats_collapse_to_single_toast: bool,
}

/// Forbidden-shortcut posture promised by every OS / lock-screen /
/// companion payload tied to the row.
///
/// Beta forbids any row that does not enumerate the closed thirteen
/// forbidden shortcut classes — the chrome cannot silently grow its
/// list of mutation classes per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForbiddenShortcutPosture {
    /// Closed thirteen forbidden classes. The validator rejects any
    /// row whose list is missing a class.
    pub forbidden_classes: Vec<ForbiddenShortcutActionClass>,
    /// True when the payload only allows an exact in-product reopen
    /// (never a shortcut replay that completes a mutation).
    pub exact_reopen_only: bool,
    /// True when the payload routes mutating activations through an
    /// in-product review path or approval workflow.
    pub mutation_routes_through_review_path: bool,
}

impl ForbiddenShortcutPosture {
    /// Build the closed-thirteen posture; the chrome reads this list
    /// verbatim.
    pub fn closed_thirteen() -> Self {
        use ForbiddenShortcutActionClass as Class;
        Self {
            forbidden_classes: vec![
                Class::DestructivePublishOrApply,
                Class::SecretOrCredentialReveal,
                Class::IrreversibleHighBlast,
                Class::BypassReviewSheet,
                Class::BypassApprovalWorkflow,
                Class::CrossWorkspaceMutation,
                Class::DirectMutationFromLockScreen,
                Class::DirectMutationFromCompanionPush,
                Class::DirectMutationFromDockOrTaskbar,
                Class::DirectMutationFromSystemTray,
                Class::PolicyOverrideFromOsShortcut,
                Class::TrustStateChangeFromOsShortcut,
                Class::ProviderGrantChangeFromOsShortcut,
            ],
            exact_reopen_only: true,
            mutation_routes_through_review_path: true,
        }
    }
}

/// Row-aligned badge mirror for one beta attention row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaBadge {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable row id this badge mirrors.
    pub row_id: String,
    /// Mirrored badge class.
    pub badge_class: BadgeClass,
    /// Mirrored severity class.
    pub severity_class: SeverityClass,
    /// Mirrored privacy class.
    pub privacy_class: PrivacyClass,
    /// True when the badge contributes to the active attention count.
    pub counts_toward_attention: bool,
    /// True when the badge mirrors a held / suppressed count.
    pub counts_toward_held_or_suppressed: bool,
    /// True when the OS app-icon mirror is visible. False under modes
    /// that suppress the dock / taskbar badge.
    pub os_app_icon_visible: bool,
    /// Privacy-safe badge label (category + count only).
    pub privacy_safe_summary_label: String,
}

/// Support-export projection row.
///
/// Raw private material (paths, URLs, raw provider payloads, prompt
/// text, secret material, customer identifiers) never crosses this
/// boundary — only typed refs and privacy-safe labels do.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaSupportExportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable row id this export quotes.
    pub row_id: String,
    /// Stable canonical event id.
    pub canonical_event_id: String,
    /// Stable notification envelope id.
    pub notification_envelope_id: String,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Row class.
    pub row_class: NotificationPrivacyBetaRowClass,
    /// Privacy class.
    pub privacy_class: PrivacyClass,
    /// Lock-screen posture.
    pub lock_screen_posture: LockScreenPosture,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Badge class.
    pub badge_class: BadgeClass,
    /// Dedupe scheme.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Occurrence count observed at export time.
    pub occurrence_count: u32,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Stable support-pack item id.
    pub support_pack_item_id: String,
}

/// One beta attention row.
///
/// The row composes the four acceptance gates: stable classes, lock-
/// screen privacy, coalesced retries, and cross-client dedupe. Every
/// referenced primitive (envelope ids, dedupe scheme, badge class,
/// quiet-hours mode set, lock-screen posture, forbidden-shortcut
/// classes) is sourced from the existing notification submodules — no
/// parallel vocabulary is minted here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Stable row id consumed by chrome and headless inspector.
    pub row_id: String,
    /// Row class.
    pub row_class: NotificationPrivacyBetaRowClass,
    /// Stable canonical event id.
    pub canonical_event_id: String,
    /// Stable notification envelope id.
    pub notification_envelope_id: String,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Privacy class.
    pub privacy_class: PrivacyClass,
    /// Privacy payload class (verbatim from the envelope).
    pub privacy_payload_class: PrivacyPayloadClass,
    /// Lock-screen posture derived from the payload class.
    pub lock_screen_posture: LockScreenPosture,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Badge class the durable row contributes to.
    pub badge_class: BadgeClass,
    /// Dedupe scheme (verbatim from the envelope).
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Stable dedupe-key ref.
    pub dedupe_key_ref: String,
    /// Reopen target the row preserves on every routed surface.
    pub reopen_target: ReopenTarget,
    /// Quiet-hours posture observed at routing time.
    pub quiet_hours_posture: BetaQuietHoursPosture,
    /// Cross-client fanout posture.
    pub cross_client_posture: CrossClientFanoutPosture,
    /// Expected coalescing posture.
    pub expected_coalescing_posture: ExpectedCoalescingPosture,
    /// Forbidden-shortcut posture promised by the payload.
    pub forbidden_shortcut_posture: ForbiddenShortcutPosture,
    /// Occurrence count observed at routing time.
    pub occurrence_count: u32,
    /// True when the row represents a dedupe repeat of a previously
    /// routed canonical event (subsequent retries collapse here).
    pub is_dedupe_repeat: bool,
    /// True when raw private material was excluded from every external
    /// payload tied to this row.
    pub raw_private_material_excluded: bool,
    /// Reviewer-facing narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Aggregate summary banner for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaSummary {
    /// Number of rows on the page.
    pub row_count: usize,
    /// Rows whose lock-screen posture is generic-only.
    pub lock_screen_generic_row_count: usize,
    /// Rows whose lock-screen posture is scoped.
    pub lock_screen_scoped_row_count: usize,
    /// Rows whose lock-screen posture forbids lock-screen rendering.
    pub lock_screen_forbidden_row_count: usize,
    /// Rows whose lock-screen posture is in-product or redacted only.
    pub in_product_only_row_count: usize,
    /// Rows whose dedupe scheme coalesces retry bursts.
    pub coalescing_row_count: usize,
    /// Rows whose cross-client dedupe scheme is in effect.
    pub cross_client_row_count: usize,
    /// Rows whose quiet-hours posture suppressed an attention surface.
    pub quiet_hours_suppressed_row_count: usize,
    /// Rows whose severity bypassed an active hold mode (critical).
    pub critical_safety_bypass_row_count: usize,
    /// Row classes present on the page (sorted).
    pub row_classes_present: Vec<NotificationPrivacyBetaRowClass>,
    /// Source subsystems present on the page (sorted).
    pub source_subsystems_present: Vec<SourceSubsystem>,
    /// Dedupe schemes present on the page (sorted).
    pub dedupe_schemes_present: Vec<DedupeKeyScheme>,
}

impl NotificationPrivacyBetaSummary {
    fn from_rows(rows: &[NotificationPrivacyBetaRow]) -> Self {
        let mut summary = Self {
            row_count: rows.len(),
            ..Self::default()
        };
        let mut row_classes: BTreeSet<NotificationPrivacyBetaRowClass> = BTreeSet::new();
        let mut subsystems: BTreeSet<SourceSubsystemSortable> = BTreeSet::new();
        let mut schemes: BTreeSet<DedupeKeySchemeSortable> = BTreeSet::new();
        for row in rows {
            match row.lock_screen_posture {
                LockScreenPosture::GenericSummaryOnly => summary.lock_screen_generic_row_count += 1,
                LockScreenPosture::ScopedWorkspaceSafe => summary.lock_screen_scoped_row_count += 1,
                LockScreenPosture::PolicyForbiddenOnLockScreen => {
                    summary.lock_screen_forbidden_row_count += 1
                }
                LockScreenPosture::InProductOnly | LockScreenPosture::RedactedMetadataOnly => {
                    summary.in_product_only_row_count += 1
                }
            }
            if row.expected_coalescing_posture.coalesces_retry_burst {
                summary.coalescing_row_count += 1;
            }
            if row.cross_client_posture.cross_client_dedupe_in_effect {
                summary.cross_client_row_count += 1;
            }
            if row.quiet_hours_posture.holds_attention_surfaces {
                summary.quiet_hours_suppressed_row_count += 1;
            }
            if row.quiet_hours_posture.bypassed_by_critical_severity {
                summary.critical_safety_bypass_row_count += 1;
            }
            row_classes.insert(row.row_class);
            subsystems.insert(SourceSubsystemSortable(row.source_subsystem));
            schemes.insert(DedupeKeySchemeSortable(row.dedupe_key_scheme));
        }
        summary.row_classes_present = row_classes.into_iter().collect();
        summary.source_subsystems_present = subsystems.into_iter().map(|s| s.0).collect();
        summary.dedupe_schemes_present = schemes.into_iter().map(|s| s.0).collect();
        summary
    }
}

// Sort wrappers so the summary projects subsystems / schemes in stable
// token order. The underlying enums don't derive `Ord`, so we sort by
// their canonical string token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceSubsystemSortable(SourceSubsystem);
impl Ord for SourceSubsystemSortable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        source_subsystem_token(self.0).cmp(source_subsystem_token(other.0))
    }
}
impl PartialOrd for SourceSubsystemSortable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DedupeKeySchemeSortable(DedupeKeyScheme);
impl Ord for DedupeKeySchemeSortable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        dedupe_scheme_token(self.0).cmp(dedupe_scheme_token(other.0))
    }
}
impl PartialOrd for DedupeKeySchemeSortable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const fn source_subsystem_token(s: SourceSubsystem) -> &'static str {
    match s {
        SourceSubsystem::Editor => "editor",
        SourceSubsystem::Terminal => "terminal",
        SourceSubsystem::ReviewAndDiff => "review_and_diff",
        SourceSubsystem::PaletteAndSearch => "palette_and_search",
        SourceSubsystem::InstallUpdateAttach => "install_update_attach",
        SourceSubsystem::AiApply => "ai_apply",
        SourceSubsystem::Collaboration => "collaboration",
        SourceSubsystem::ProviderBearing => "provider_bearing",
        SourceSubsystem::DocsHelpServiceHealth => "docs_help_service_health",
        SourceSubsystem::SupportExport => "support_export",
        SourceSubsystem::BuildSystem => "build_system",
        SourceSubsystem::TestRunner => "test_runner",
        SourceSubsystem::DebugSession => "debug_session",
        SourceSubsystem::TaskRunner => "task_runner",
        SourceSubsystem::Indexer => "indexer",
        SourceSubsystem::VfsSave => "vfs_save",
        SourceSubsystem::SyncMirror => "sync_mirror",
        SourceSubsystem::NotebookKernel => "notebook_kernel",
        SourceSubsystem::RemoteAgent => "remote_agent",
        SourceSubsystem::ExtensionHost => "extension_host",
        SourceSubsystem::WorkspaceTrust => "workspace_trust",
        SourceSubsystem::PolicyResolver => "policy_resolver",
        SourceSubsystem::AdminPolicy => "admin_policy",
        SourceSubsystem::SecretBroker => "secret_broker",
        SourceSubsystem::RuntimePowerManager => "runtime_power_manager",
        SourceSubsystem::Shell => "shell",
    }
}

const fn client_scope_token(s: ClientScope) -> &'static str {
    match s {
        ClientScope::DesktopProduct => "desktop_product",
        ClientScope::Cli => "cli",
        ClientScope::CompanionSurface => "companion_surface",
        ClientScope::RemoteAgent => "remote_agent",
        ClientScope::SdkOrApi => "sdk_or_api",
        ClientScope::ManagedAdminSurface => "managed_admin_surface",
    }
}

const fn dedupe_scheme_token(s: DedupeKeyScheme) -> &'static str {
    match s {
        DedupeKeyScheme::CanonicalEventId => "canonical_event_id",
        DedupeKeyScheme::CanonicalObjectTargetPlusEventClass => {
            "canonical_object_target_plus_event_class"
        }
        DedupeKeyScheme::GroupedBurstId => "grouped_burst_id",
        DedupeKeyScheme::SubsystemPlusObjectPlusPhase => "subsystem_plus_object_plus_phase",
        DedupeKeyScheme::CrossClientCanonicalEventId => "cross_client_canonical_event_id",
    }
}

/// Top-level beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Reviewer-facing page label.
    pub page_label: String,
    /// Aggregate summary banner.
    pub summary: NotificationPrivacyBetaSummary,
    /// Durable badge projection across every row (deduped by
    /// canonical event id).
    pub badge_projection: DurableBadgeProjection,
    /// Rows on the page.
    pub rows: Vec<NotificationPrivacyBetaRow>,
    /// Badge mirror for each row.
    pub badges: Vec<NotificationPrivacyBetaBadge>,
}

impl NotificationPrivacyBetaPage {
    /// Build a page from its rows and badges. The badge projection is
    /// computed from the supplied routed-notification slice; the slice
    /// is the only mint-from-truth path for the durable badge counts.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        rows: Vec<NotificationPrivacyBetaRow>,
        badges: Vec<NotificationPrivacyBetaBadge>,
        badge_projection: DurableBadgeProjection,
    ) -> Self {
        let summary = NotificationPrivacyBetaSummary::from_rows(&rows);
        Self {
            record_kind: NOTIFICATION_PRIVACY_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION,
            shared_contract_ref: NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            summary,
            badge_projection,
            rows,
            badges,
        }
    }

    /// True when the page exercises every required beta row class.
    pub fn covers_required_row_classes(&self) -> bool {
        for required in [
            NotificationPrivacyBetaRowClass::DeliveredSummarySafe,
            NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure,
            NotificationPrivacyBetaRowClass::LockScreenSafeGenericPayload,
            NotificationPrivacyBetaRowClass::LockScreenSafeScopedPayload,
            NotificationPrivacyBetaRowClass::LockScreenForbiddenSecurityCritical,
            NotificationPrivacyBetaRowClass::QuietHoursHeld,
            NotificationPrivacyBetaRowClass::AdminPolicySuppressed,
            NotificationPrivacyBetaRowClass::CriticalSafetyEscalation,
            NotificationPrivacyBetaRowClass::CompanionCrossClientFanout,
            NotificationPrivacyBetaRowClass::ForbiddenShortcutBypassRefused,
        ] {
            if !self.summary.row_classes_present.contains(&required) {
                return false;
            }
        }
        true
    }
}

/// Support-export wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPrivacyBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Embedded beta page.
    pub page: NotificationPrivacyBetaPage,
    /// Per-row export rows in stable page order.
    pub rows: Vec<NotificationPrivacyBetaSupportExportRow>,
    /// Case ids of the rows quoted by the export, in stable page order.
    pub case_ids: Vec<String>,
    /// True when no raw private material crosses the export boundary.
    pub raw_private_material_excluded: bool,
}

impl NotificationPrivacyBetaSupportExport {
    /// Build a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: NotificationPrivacyBetaPage,
    ) -> Self {
        let rows: Vec<NotificationPrivacyBetaSupportExportRow> = page
            .rows
            .iter()
            .map(|row| NotificationPrivacyBetaSupportExportRow {
                record_kind: NOTIFICATION_PRIVACY_BETA_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                schema_version: NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION,
                shared_contract_ref: NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                canonical_event_id: row.canonical_event_id.clone(),
                notification_envelope_id: row.notification_envelope_id.clone(),
                source_subsystem: row.source_subsystem,
                row_class: row.row_class,
                privacy_class: row.privacy_class,
                lock_screen_posture: row.lock_screen_posture,
                redaction_class: row.redaction_class,
                severity_class: row.severity_class,
                badge_class: row.badge_class,
                dedupe_key_scheme: row.dedupe_key_scheme,
                occurrence_count: row.occurrence_count,
                raw_private_material_excluded: true,
                support_pack_item_id: format!(
                    "support.item.notification_privacy.beta.{}",
                    row.row_id
                ),
            })
            .collect();
        let case_ids: Vec<String> = page.rows.iter().map(|row| row.case_id.clone()).collect();
        Self {
            record_kind: NOTIFICATION_PRIVACY_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION,
            shared_contract_ref: NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            rows,
            case_ids,
            raw_private_material_excluded: true,
        }
    }
}

/// Validation error raised when the beta page fails an acceptance gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationPrivacyBetaValidationError {
    /// Privacy class and payload class disagree (e.g., security_critical
    /// shipping a generic lock-screen payload).
    PrivacyPostureDrift {
        /// Row id.
        row_id: String,
        /// Reason label.
        reason: String,
    },
    /// Lock-screen posture does not match the envelope's payload class.
    LockScreenPostureDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// A retry-coalescing row picked an incompatible dedupe scheme.
    CoalescingSchemeIncompatible {
        /// Row id.
        row_id: String,
        /// Reason label.
        reason: String,
    },
    /// Badge class disagreed with the source subsystem's expected
    /// badge class for the row class.
    BadgeClassDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// A cross-client row carried more than one scope but did not adopt
    /// the cross-client dedupe scheme.
    CrossClientDedupeMissing {
        /// Row id.
        row_id: String,
    },
    /// Cross-client siblings repeated a scope or included the
    /// originating scope.
    CrossClientSiblingsInvalid {
        /// Row id.
        row_id: String,
        /// Reason label.
        reason: String,
    },
    /// Critical-severity row admitted a hold (durable truth was lost
    /// or attention surfaces were suppressed when they MUST interrupt).
    CriticalSafetyHoldAdmitted {
        /// Row id.
        row_id: String,
    },
    /// Forbidden-shortcut list is missing one of the closed thirteen
    /// classes.
    ForbiddenShortcutListIncomplete {
        /// Row id.
        row_id: String,
        /// Missing class token.
        missing_class: String,
    },
    /// Badge mirror is missing a row that exists on the page.
    BadgeMissingForRow {
        /// Row id.
        row_id: String,
    },
    /// Badge mirror drifted from the row on a parity field.
    BadgeRowParityDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// Support-export row is missing for a page row.
    SupportExportMissingForRow {
        /// Row id.
        row_id: String,
    },
    /// Support-export row drifted from the page row on a parity field.
    SupportExportParityDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// The page is missing one of the required beta row classes.
    RowClassCoverageIncomplete {
        /// Missing row class token.
        missing_row_class: String,
    },
    /// Durable badge projection drifted from the deduped row count.
    BadgeProjectionDrift {
        /// Reason label.
        reason: String,
    },
}

impl std::fmt::Display for NotificationPrivacyBetaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrivacyPostureDrift { row_id, reason } => write!(
                f,
                "row {row_id} privacy posture drifted: {reason}"
            ),
            Self::LockScreenPostureDrift { row_id, field } => write!(
                f,
                "row {row_id} lock-screen posture drifted on field {field}"
            ),
            Self::CoalescingSchemeIncompatible { row_id, reason } => write!(
                f,
                "row {row_id} coalescing scheme incompatible: {reason}"
            ),
            Self::BadgeClassDrift { row_id, field } => write!(
                f,
                "row {row_id} badge class drifted on field {field}"
            ),
            Self::CrossClientDedupeMissing { row_id } => write!(
                f,
                "row {row_id} declared sibling scopes but is not using cross_client_canonical_event_id"
            ),
            Self::CrossClientSiblingsInvalid { row_id, reason } => write!(
                f,
                "row {row_id} cross-client siblings invalid: {reason}"
            ),
            Self::CriticalSafetyHoldAdmitted { row_id } => write!(
                f,
                "row {row_id} declared critical severity but admitted a hold or lost durable truth"
            ),
            Self::ForbiddenShortcutListIncomplete {
                row_id,
                missing_class,
            } => write!(
                f,
                "row {row_id} forbidden-shortcut list is missing class {missing_class}"
            ),
            Self::BadgeMissingForRow { row_id } => write!(
                f,
                "row {row_id} has no badge mirror on the page"
            ),
            Self::BadgeRowParityDrift { row_id, field } => write!(
                f,
                "badge for row {row_id} drifted from the row on field {field}"
            ),
            Self::SupportExportMissingForRow { row_id } => write!(
                f,
                "row {row_id} has no support-export row in the wrapper"
            ),
            Self::SupportExportParityDrift { row_id, field } => write!(
                f,
                "support-export row for {row_id} drifted from the page row on field {field}"
            ),
            Self::RowClassCoverageIncomplete { missing_row_class } => write!(
                f,
                "beta page did not cover required row class {missing_row_class}"
            ),
            Self::BadgeProjectionDrift { reason } => write!(
                f,
                "durable badge projection drifted: {reason}"
            ),
        }
    }
}

impl std::error::Error for NotificationPrivacyBetaValidationError {}

/// Required closed-thirteen list of forbidden shortcut classes. The
/// validator rejects any row whose list is missing a class.
const REQUIRED_FORBIDDEN_SHORTCUT_CLASSES: &[ForbiddenShortcutActionClass] = &[
    ForbiddenShortcutActionClass::DestructivePublishOrApply,
    ForbiddenShortcutActionClass::SecretOrCredentialReveal,
    ForbiddenShortcutActionClass::IrreversibleHighBlast,
    ForbiddenShortcutActionClass::BypassReviewSheet,
    ForbiddenShortcutActionClass::BypassApprovalWorkflow,
    ForbiddenShortcutActionClass::CrossWorkspaceMutation,
    ForbiddenShortcutActionClass::DirectMutationFromLockScreen,
    ForbiddenShortcutActionClass::DirectMutationFromCompanionPush,
    ForbiddenShortcutActionClass::DirectMutationFromDockOrTaskbar,
    ForbiddenShortcutActionClass::DirectMutationFromSystemTray,
    ForbiddenShortcutActionClass::PolicyOverrideFromOsShortcut,
    ForbiddenShortcutActionClass::TrustStateChangeFromOsShortcut,
    ForbiddenShortcutActionClass::ProviderGrantChangeFromOsShortcut,
];

/// Validate the beta page against the M3 acceptance gates.
pub fn validate_notification_privacy_beta_page(
    page: &NotificationPrivacyBetaPage,
) -> Result<(), Vec<NotificationPrivacyBetaValidationError>> {
    let mut errors: Vec<NotificationPrivacyBetaValidationError> = Vec::new();

    for row in &page.rows {
        // Privacy posture: payload class must be consistent with
        // privacy class. We enforce the same matrix as the router's
        // own validator.
        if !privacy_posture_consistent(row.privacy_class, row.privacy_payload_class) {
            errors.push(
                NotificationPrivacyBetaValidationError::PrivacyPostureDrift {
                    row_id: row.row_id.clone(),
                    reason: format!(
                        "privacy_class={} but privacy_payload_class={}",
                        row.privacy_class.as_token(),
                        row.privacy_payload_class.as_token()
                    ),
                },
            );
        }

        // Lock-screen posture mirror must agree with the payload class.
        let projected = LockScreenPosture::from_payload_class(row.privacy_payload_class);
        if projected != row.lock_screen_posture {
            errors.push(
                NotificationPrivacyBetaValidationError::LockScreenPostureDrift {
                    row_id: row.row_id.clone(),
                    field: "lock_screen_posture".to_owned(),
                },
            );
        }

        // Coalescing scheme: retry-coalescing rows reject
        // canonical_event_id (it would inflate the badge per delivery).
        if row.expected_coalescing_posture.coalesces_retry_burst
            && matches!(
                row.dedupe_key_scheme,
                DedupeKeyScheme::CanonicalEventId
                    | DedupeKeyScheme::CanonicalObjectTargetPlusEventClass
            )
        {
            errors.push(
                NotificationPrivacyBetaValidationError::CoalescingSchemeIncompatible {
                    row_id: row.row_id.clone(),
                    reason: format!(
                        "retry coalescing requires grouped_burst_id or subsystem_plus_object_plus_phase, got {}",
                        dedupe_scheme_token(row.dedupe_key_scheme)
                    ),
                },
            );
        }
        if row.expected_coalescing_posture.dedupe_key_scheme != row.dedupe_key_scheme {
            errors.push(
                NotificationPrivacyBetaValidationError::CoalescingSchemeIncompatible {
                    row_id: row.row_id.clone(),
                    reason: "expected_coalescing_posture.dedupe_key_scheme differs from row dedupe scheme"
                        .to_owned(),
                },
            );
        }
        if row.expected_coalescing_posture.badge_class != row.badge_class {
            errors.push(NotificationPrivacyBetaValidationError::BadgeClassDrift {
                row_id: row.row_id.clone(),
                field: "expected_coalescing_posture.badge_class".to_owned(),
            });
        }

        // Cross-client posture: more than one declared sibling demands
        // the cross-client dedupe scheme.
        if !row.cross_client_posture.sibling_scopes.is_empty()
            && !row.cross_client_posture.cross_client_dedupe_in_effect
        {
            errors.push(NotificationPrivacyBetaValidationError::CrossClientDedupeMissing {
                row_id: row.row_id.clone(),
            });
        }
        if row.cross_client_posture.cross_client_dedupe_in_effect
            && !matches!(
                row.dedupe_key_scheme,
                DedupeKeyScheme::CrossClientCanonicalEventId
            )
        {
            errors.push(NotificationPrivacyBetaValidationError::CrossClientDedupeMissing {
                row_id: row.row_id.clone(),
            });
        }
        let mut seen_scope_tokens: BTreeSet<&'static str> = BTreeSet::new();
        for scope in &row.cross_client_posture.sibling_scopes {
            if *scope == row.cross_client_posture.originating_scope {
                errors.push(
                    NotificationPrivacyBetaValidationError::CrossClientSiblingsInvalid {
                        row_id: row.row_id.clone(),
                        reason: "originating scope appears as its own sibling".to_owned(),
                    },
                );
            }
            if !seen_scope_tokens.insert(client_scope_token(*scope)) {
                errors.push(
                    NotificationPrivacyBetaValidationError::CrossClientSiblingsInvalid {
                        row_id: row.row_id.clone(),
                        reason: "sibling scope appears more than once".to_owned(),
                    },
                );
            }
        }

        // Critical-severity rows can never admit a hold.
        if matches!(row.severity_class, SeverityClass::Critical) {
            if row.quiet_hours_posture.holds_attention_surfaces
                || !row.quiet_hours_posture.durable_truth_preserved
            {
                errors.push(
                    NotificationPrivacyBetaValidationError::CriticalSafetyHoldAdmitted {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        } else if row.quiet_hours_posture.bypassed_by_critical_severity {
            // Non-critical rows must not claim a critical-safety
            // bypass.
            errors.push(NotificationPrivacyBetaValidationError::PrivacyPostureDrift {
                row_id: row.row_id.clone(),
                reason: "bypassed_by_critical_severity=true but severity is not critical".to_owned(),
            });
        }

        // Forbidden-shortcut list must enumerate every required class.
        for required in REQUIRED_FORBIDDEN_SHORTCUT_CLASSES {
            if !row
                .forbidden_shortcut_posture
                .forbidden_classes
                .contains(required)
            {
                errors.push(
                    NotificationPrivacyBetaValidationError::ForbiddenShortcutListIncomplete {
                        row_id: row.row_id.clone(),
                        missing_class: required.as_str().to_owned(),
                    },
                );
            }
        }

        // Badge mirror parity.
        let badge = page.badges.iter().find(|b| b.row_id == row.row_id);
        match badge {
            None => errors.push(NotificationPrivacyBetaValidationError::BadgeMissingForRow {
                row_id: row.row_id.clone(),
            }),
            Some(badge) => {
                if badge.badge_class != row.badge_class {
                    errors.push(NotificationPrivacyBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "badge_class".to_owned(),
                    });
                }
                if badge.severity_class != row.severity_class {
                    errors.push(NotificationPrivacyBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "severity_class".to_owned(),
                    });
                }
                if badge.privacy_class != row.privacy_class {
                    errors.push(NotificationPrivacyBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "privacy_class".to_owned(),
                    });
                }
                let expected_os_visible = !row.quiet_hours_posture.os_app_icon_badge_suppressed;
                if badge.os_app_icon_visible != expected_os_visible {
                    errors.push(NotificationPrivacyBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "os_app_icon_visible".to_owned(),
                    });
                }
            }
        }
    }

    // Row-class coverage.
    for required in [
        NotificationPrivacyBetaRowClass::DeliveredSummarySafe,
        NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure,
        NotificationPrivacyBetaRowClass::LockScreenSafeGenericPayload,
        NotificationPrivacyBetaRowClass::LockScreenSafeScopedPayload,
        NotificationPrivacyBetaRowClass::LockScreenForbiddenSecurityCritical,
        NotificationPrivacyBetaRowClass::QuietHoursHeld,
        NotificationPrivacyBetaRowClass::AdminPolicySuppressed,
        NotificationPrivacyBetaRowClass::CriticalSafetyEscalation,
        NotificationPrivacyBetaRowClass::CompanionCrossClientFanout,
        NotificationPrivacyBetaRowClass::ForbiddenShortcutBypassRefused,
    ] {
        if !page.summary.row_classes_present.contains(&required) {
            errors.push(
                NotificationPrivacyBetaValidationError::RowClassCoverageIncomplete {
                    missing_row_class: required.as_str().to_owned(),
                },
            );
        }
    }

    // Durable badge projection must equal the number of distinct
    // canonical event ids represented by the rows. (Repeats collapse
    // by canonical event id, so the projection's `durable_count`
    // cannot exceed the distinct event count.)
    let distinct_events: BTreeSet<&str> = page
        .rows
        .iter()
        .map(|r| r.canonical_event_id.as_str())
        .collect();
    if (page.badge_projection.durable_count as usize) > distinct_events.len() {
        errors.push(NotificationPrivacyBetaValidationError::BadgeProjectionDrift {
            reason: format!(
                "durable_count={} exceeds distinct canonical events {}",
                page.badge_projection.durable_count,
                distinct_events.len()
            ),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate the support-export wrapper for parity with its embedded
/// page.
pub fn validate_notification_privacy_beta_support_export(
    export: &NotificationPrivacyBetaSupportExport,
) -> Result<(), Vec<NotificationPrivacyBetaValidationError>> {
    let mut errors: Vec<NotificationPrivacyBetaValidationError> = Vec::new();
    for row in &export.page.rows {
        let export_row = export.rows.iter().find(|r| r.row_id == row.row_id);
        match export_row {
            None => errors.push(
                NotificationPrivacyBetaValidationError::SupportExportMissingForRow {
                    row_id: row.row_id.clone(),
                },
            ),
            Some(export_row) => {
                if export_row.row_class != row.row_class {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "row_class".to_owned(),
                        },
                    );
                }
                if export_row.privacy_class != row.privacy_class {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "privacy_class".to_owned(),
                        },
                    );
                }
                if export_row.lock_screen_posture != row.lock_screen_posture {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "lock_screen_posture".to_owned(),
                        },
                    );
                }
                if export_row.redaction_class != row.redaction_class {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "redaction_class".to_owned(),
                        },
                    );
                }
                if export_row.badge_class != row.badge_class {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "badge_class".to_owned(),
                        },
                    );
                }
                if export_row.dedupe_key_scheme != row.dedupe_key_scheme {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "dedupe_key_scheme".to_owned(),
                        },
                    );
                }
                if export_row.canonical_event_id != row.canonical_event_id {
                    errors.push(
                        NotificationPrivacyBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "canonical_event_id".to_owned(),
                        },
                    );
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

trait PrivacyClassToken {
    fn as_token(self) -> &'static str;
}

impl PrivacyClassToken for PrivacyClass {
    fn as_token(self) -> &'static str {
        match self {
            PrivacyClass::SummarySafe => "summary_safe",
            PrivacyClass::WorkspaceSensitive => "workspace_sensitive",
            PrivacyClass::SecurityCritical => "security_critical",
            PrivacyClass::ManagedSensitive => "managed_sensitive",
        }
    }
}

trait PrivacyPayloadClassToken {
    fn as_token(self) -> &'static str;
}

impl PrivacyPayloadClassToken for PrivacyPayloadClass {
    fn as_token(self) -> &'static str {
        match self {
            PrivacyPayloadClass::LockScreenSafeGeneric => "lock_screen_safe_generic",
            PrivacyPayloadClass::LockScreenSafeScoped => "lock_screen_safe_scoped",
            PrivacyPayloadClass::InProductOnly => "in_product_only",
            PrivacyPayloadClass::RedactedMetadataOnly => "redacted_metadata_only",
            PrivacyPayloadClass::PolicyForbiddenOnLockScreen => "policy_forbidden_on_lock_screen",
        }
    }
}

fn privacy_posture_consistent(class: PrivacyClass, payload: PrivacyPayloadClass) -> bool {
    match (class, payload) {
        (
            PrivacyClass::SummarySafe,
            PrivacyPayloadClass::LockScreenSafeGeneric
            | PrivacyPayloadClass::LockScreenSafeScoped
            | PrivacyPayloadClass::InProductOnly
            | PrivacyPayloadClass::RedactedMetadataOnly,
        ) => true,
        (
            PrivacyClass::WorkspaceSensitive,
            PrivacyPayloadClass::LockScreenSafeGeneric
            | PrivacyPayloadClass::LockScreenSafeScoped
            | PrivacyPayloadClass::InProductOnly
            | PrivacyPayloadClass::RedactedMetadataOnly,
        ) => true,
        (
            PrivacyClass::SecurityCritical,
            PrivacyPayloadClass::RedactedMetadataOnly
            | PrivacyPayloadClass::PolicyForbiddenOnLockScreen
            | PrivacyPayloadClass::InProductOnly,
        ) => true,
        (
            PrivacyClass::ManagedSensitive,
            PrivacyPayloadClass::RedactedMetadataOnly
            | PrivacyPayloadClass::PolicyForbiddenOnLockScreen
            | PrivacyPayloadClass::InProductOnly,
        ) => true,
        _ => false,
    }
}

// =====================================================================
// Seed fixture builder
// =====================================================================

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed is the only mint-from-truth path for the
/// JSON checked in under `fixtures/ux/m3/notification_privacy/`.
pub fn seeded_notification_privacy_beta_page() -> NotificationPrivacyBetaPage {
    let scenarios = SeedScenarios::build();

    let mut rows: Vec<NotificationPrivacyBetaRow> = Vec::new();
    let mut routed_for_badges: Vec<RoutedNotification> = Vec::new();
    let mut router = NotificationRouter::new();

    for scenario in scenarios.into_iter() {
        let mut envelope = scenario.envelope;
        // Apply the scenario's shell posture before routing — this is
        // how the live shell narrows the envelope's suppression state.
        scenario.posture.apply_to_envelope(&mut envelope);

        // Route once for the badge projection (the row records the
        // observed occurrence_count from the first emission).
        let first = router.route(&envelope).expect("seed envelope must route");
        // If the scenario simulates retry coalescing, route again so
        // the row observes a dedupe repeat. The badge stays at one
        // durable item.
        let final_routed = if scenario.row_class
            == NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure
        {
            router.route(&envelope).expect("retry routing must succeed")
        } else {
            first.clone()
        };

        let mut posture_snapshot = BetaQuietHoursPosture {
            active_modes: scenario.posture.active_modes_sorted(),
            holds_attention_surfaces: envelope.suppression_state.suppressed,
            durable_truth_preserved: final_routed.surface_routes.iter().any(|route| {
                matches!(
                    route.fanout_surface_class,
                    FanoutSurfaceClass::DurableJobRow
                        | FanoutSurfaceClass::ActivityCenterDigestCard
                        | FanoutSurfaceClass::DigestGroupRow
                        | FanoutSurfaceClass::StatusItem
                        | FanoutSurfaceClass::StatusStrip
                ) && matches!(
                    route.stale_or_undelivered_reason.reason_class,
                    StaleOrUndeliveredReasonClass::None
                )
            }),
            os_app_icon_badge_suppressed: scenario.posture.suppresses_os_app_icon_badge(),
            lock_screen_summary_suppressed: scenario.posture.suppresses_lock_screen_summary(),
            bypassed_by_critical_severity: matches!(envelope.severity_class, SeverityClass::Critical)
                && scenario.posture.has_active_quiet_mode(),
        };
        // Critical severity always interrupts; the apply step has
        // already cleared `suppressed`, so the snapshot must reflect it.
        if matches!(envelope.severity_class, SeverityClass::Critical) {
            posture_snapshot.holds_attention_surfaces = false;
        }

        let row = NotificationPrivacyBetaRow {
            record_kind: NOTIFICATION_PRIVACY_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION,
            shared_contract_ref: NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: scenario.case_id.to_owned(),
            row_id: scenario.row_id.to_owned(),
            row_class: scenario.row_class,
            canonical_event_id: envelope.canonical_event_id.clone(),
            notification_envelope_id: envelope.notification_envelope_id.clone(),
            source_subsystem: envelope.source_subsystem,
            severity_class: envelope.severity_class,
            privacy_class: envelope.privacy_class,
            privacy_payload_class: envelope.privacy_payload_class,
            lock_screen_posture: LockScreenPosture::from_payload_class(envelope.privacy_payload_class),
            redaction_class: envelope.redaction_class,
            badge_class: scenario.badge_class,
            dedupe_key_scheme: envelope.dedupe_key_scheme,
            dedupe_key_ref: envelope.dedupe_key_ref.clone(),
            reopen_target: envelope.reopen_target.clone(),
            quiet_hours_posture: posture_snapshot,
            cross_client_posture: scenario.cross_client_posture,
            expected_coalescing_posture: ExpectedCoalescingPosture {
                dedupe_key_scheme: envelope.dedupe_key_scheme,
                badge_class: scenario.badge_class,
                coalesces_retry_burst: scenario.coalesces_retry_burst,
                repeats_collapse_to_single_toast: true,
            },
            forbidden_shortcut_posture: ForbiddenShortcutPosture::closed_thirteen(),
            occurrence_count: final_routed.occurrence_count,
            is_dedupe_repeat: final_routed.is_dedupe_repeat,
            raw_private_material_excluded: true,
            narrative: scenario.narrative.to_owned(),
        };
        rows.push(row);
        routed_for_badges.push(final_routed);
    }

    // Build the durable badge projection from the routed slice. Empty
    // posture (no quiet mode active) so the projection's
    // `os_app_icon_badge_visible` reflects the global default; the
    // per-row badge mirror records the per-row suppression state.
    let badge_projection =
        DurableBadgeProjection::from_routed(&routed_for_badges, &QuietHoursPosture::none());

    let badges: Vec<NotificationPrivacyBetaBadge> = rows
        .iter()
        .map(|row| make_badge_for_row(row))
        .collect();

    NotificationPrivacyBetaPage::new(
        "shell:notification-privacy:beta:page:default",
        "Notification privacy, quiet-hours, badge semantics, and cross-client dedupe (beta)",
        rows,
        badges,
        badge_projection,
    )
}

fn make_badge_for_row(row: &NotificationPrivacyBetaRow) -> NotificationPrivacyBetaBadge {
    let counts_toward_attention = !row.quiet_hours_posture.holds_attention_surfaces
        && !matches!(row.severity_class, SeverityClass::Success);
    let counts_toward_held_or_suppressed = row.quiet_hours_posture.holds_attention_surfaces;
    let os_app_icon_visible = !row.quiet_hours_posture.os_app_icon_badge_suppressed;
    let summary_label = privacy_safe_badge_label(row);
    NotificationPrivacyBetaBadge {
        record_kind: NOTIFICATION_PRIVACY_BETA_BADGE_RECORD_KIND.to_owned(),
        schema_version: NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION,
        shared_contract_ref: NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row.row_id.clone(),
        badge_class: row.badge_class,
        severity_class: row.severity_class,
        privacy_class: row.privacy_class,
        counts_toward_attention,
        counts_toward_held_or_suppressed,
        os_app_icon_visible,
        privacy_safe_summary_label: summary_label,
    }
}

fn privacy_safe_badge_label(row: &NotificationPrivacyBetaRow) -> String {
    let class_token = match row.badge_class {
        BadgeClass::NeedsReview => "review",
        BadgeClass::FailedRuns => "failed run",
        BadgeClass::Mentions => "mention",
        BadgeClass::SecurityNotices => "security notice",
        BadgeClass::SessionRequests => "session request",
        BadgeClass::OfflinePublishPending => "offline publish",
        BadgeClass::DurableRunningCount => "running item",
        BadgeClass::HeldOrSuppressedCount => "held item",
        BadgeClass::CompletionUnread => "unread completion",
    };
    if row.is_dedupe_repeat {
        format!("{} {} (coalesced)", row.occurrence_count, class_token)
    } else {
        format!("1 {}", class_token)
    }
}

// One scenario seed used by the page builder.
struct SeedScenario {
    case_id: &'static str,
    row_id: &'static str,
    row_class: NotificationPrivacyBetaRowClass,
    envelope: NotificationEnvelope,
    posture: QuietHoursPosture,
    badge_class: BadgeClass,
    coalesces_retry_burst: bool,
    cross_client_posture: CrossClientFanoutPosture,
    narrative: &'static str,
}

struct SeedScenarios;
impl SeedScenarios {
    fn build() -> Vec<SeedScenario> {
        vec![
            delivered_summary_safe(),
            coalesced_repeated_failure(),
            lock_screen_safe_generic(),
            lock_screen_safe_scoped(),
            lock_screen_forbidden_security_critical(),
            quiet_hours_held(),
            admin_policy_suppressed(),
            critical_safety_escalation(),
            companion_cross_client_fanout(),
            forbidden_shortcut_bypass_refused(),
        ]
    }
}

fn baseline_envelope(
    envelope_id: &str,
    canonical_event_id: &str,
    dedupe_key_ref: &str,
    source: SourceSubsystem,
    severity: SeverityClass,
    privacy: PrivacyClass,
    payload: PrivacyPayloadClass,
    redaction: RedactionClass,
    scheme: DedupeKeyScheme,
    surfaces: Vec<FanoutSurfaceClass>,
    summary: &str,
    reopen_ref: &str,
    target_ref: &str,
    open_command: &str,
) -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".to_owned(),
        notification_envelope_schema_version: NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: envelope_id.to_owned(),
        canonical_event_id: canonical_event_id.to_owned(),
        event_lineage_id_ref: format!("ux:lineage:{}", canonical_event_id),
        source_subsystem: source,
        source_event_ref: format!("source:{}", canonical_event_id),
        actor_identity_ref: "id:actor:system:notification-privacy-beta".to_owned(),
        canonical_object_target_ref: target_ref.to_owned(),
        severity_class: severity,
        privacy_class: privacy,
        privacy_payload_class: payload,
        redaction_class: redaction,
        dedupe_key_scheme: scheme,
        dedupe_key_ref: dedupe_key_ref.to_owned(),
        grouped_burst_id_ref: None,
        recommended_surfaces: surfaces,
        summary_label: summary.to_owned(),
        reopen_target: ReopenTarget {
            reopen_target_ref: reopen_ref.to_owned(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some(target_ref.to_owned()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: format!("ux:action:{}:open", canonical_event_id),
            label: "Open".to_owned(),
            command_id: open_command.to_owned(),
            target_identity_ref: target_ref.to_owned(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-15T10:00:00Z".to_owned(),
    }
}

fn delivered_summary_safe() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:delivered-summary-safe:01",
        row_id: "ux:notification-privacy:beta:row:delivered-summary-safe",
        row_class: NotificationPrivacyBetaRowClass::DeliveredSummarySafe,
        envelope: baseline_envelope(
            "ux:notif-env:beta:delivered-summary-safe:01",
            "ux:event:beta:delivered-summary-safe:01",
            "ux:dedupe:beta:delivered-summary-safe:01",
            SourceSubsystem::Indexer,
            SeverityClass::Success,
            PrivacyClass::SummarySafe,
            PrivacyPayloadClass::LockScreenSafeGeneric,
            RedactionClass::MetadataSafeDefault,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::Toast,
            ],
            "Indexing pass complete",
            "ux:reopen:beta:delivered-summary-safe:01",
            "obj:indexer:hot-set:01",
            "cmd:indexer.open_job_details",
        ),
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::CompletionUnread,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "First emission delivered cleanly to the durable row, the status strip, and the toast. The OS app-icon mirror is visible; lock-screen payload renders the generic category-level label only.",
    }
}

fn coalesced_repeated_failure() -> SeedScenario {
    let mut envelope = baseline_envelope(
        "ux:notif-env:beta:coalesced-retry-burst:01",
        "ux:event:beta:coalesced-retry-burst:01",
        "ux:dedupe:beta:coalesced-retry-burst:01",
        SourceSubsystem::TestRunner,
        SeverityClass::Error,
        PrivacyClass::WorkspaceSensitive,
        PrivacyPayloadClass::LockScreenSafeGeneric,
        RedactionClass::OperatorOnlyRestricted,
        DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
        vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
        ],
        "Test suite retries coalesced",
        "ux:reopen:beta:coalesced-retry-burst:01",
        "obj:test-runner:pytest-suite:01",
        "cmd:test_run.open_job_details",
    );
    envelope.grouped_burst_id_ref =
        Some("ux:burst:beta:coalesced-retry-burst:01".to_owned());
    SeedScenario {
        case_id: "shell:notification-privacy:beta:coalesced-retry-burst:01",
        row_id: "ux:notification-privacy:beta:row:coalesced-retry-burst",
        row_class: NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure,
        envelope,
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::FailedRuns,
        coalesces_retry_burst: true,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Ten retries against one pipeline collapse under subsystem_plus_object_plus_phase; the toast does not stack, the badge counts one failed-runs item, and the deduped occurrence count is recorded on the row.",
    }
}

fn lock_screen_safe_generic() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:lock-screen-safe-generic:01",
        row_id: "ux:notification-privacy:beta:row:lock-screen-safe-generic",
        row_class: NotificationPrivacyBetaRowClass::LockScreenSafeGenericPayload,
        envelope: baseline_envelope(
            "ux:notif-env:beta:lock-screen-safe-generic:01",
            "ux:event:beta:lock-screen-safe-generic:01",
            "ux:dedupe:beta:lock-screen-safe-generic:01",
            SourceSubsystem::TaskRunner,
            SeverityClass::Info,
            PrivacyClass::SummarySafe,
            PrivacyPayloadClass::LockScreenSafeGeneric,
            RedactionClass::MetadataSafeDefault,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::OsNotification,
                FanoutSurfaceClass::LockScreenSummary,
            ],
            "Background task completed",
            "ux:reopen:beta:lock-screen-safe-generic:01",
            "obj:task-runner:format-pass:01",
            "cmd:task_runner.open_job_details",
        ),
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::CompletionUnread,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Lock-screen payload renders only generic category-level labels: severity, subsystem class, and count. No workspace, object, actor, raw path, or raw URL crosses the OS sink. Reopen routes through the in-product canonical object.",
    }
}

fn lock_screen_safe_scoped() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:lock-screen-safe-scoped:01",
        row_id: "ux:notification-privacy:beta:row:lock-screen-safe-scoped",
        row_class: NotificationPrivacyBetaRowClass::LockScreenSafeScopedPayload,
        envelope: baseline_envelope(
            "ux:notif-env:beta:lock-screen-safe-scoped:01",
            "ux:event:beta:lock-screen-safe-scoped:01",
            "ux:dedupe:beta:lock-screen-safe-scoped:01",
            SourceSubsystem::ReviewAndDiff,
            SeverityClass::Warning,
            PrivacyClass::SummarySafe,
            PrivacyPayloadClass::LockScreenSafeScoped,
            RedactionClass::OperatorOnlyRestricted,
            DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::ContextualBanner,
                FanoutSurfaceClass::OsNotification,
            ],
            "Review needs attention",
            "ux:reopen:beta:lock-screen-safe-scoped:01",
            "obj:review:trust-canvas:01",
            "cmd:review.open_trust_canvas",
        ),
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::NeedsReview,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Workspace-trust review payload upgrades to scoped labels — workspace label, session label, and next-action label — but never names the diff excerpt or the actor real name. Reopen routes through the in-product trust review canvas.",
    }
}

fn lock_screen_forbidden_security_critical() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:lock-screen-forbidden:01",
        row_id: "ux:notification-privacy:beta:row:lock-screen-forbidden",
        row_class: NotificationPrivacyBetaRowClass::LockScreenForbiddenSecurityCritical,
        envelope: baseline_envelope(
            "ux:notif-env:beta:lock-screen-forbidden:01",
            "ux:event:beta:lock-screen-forbidden:01",
            "ux:dedupe:beta:lock-screen-forbidden:01",
            SourceSubsystem::SecretBroker,
            SeverityClass::Blocking,
            PrivacyClass::SecurityCritical,
            PrivacyPayloadClass::PolicyForbiddenOnLockScreen,
            RedactionClass::InternalSupportRestricted,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::ContextualBanner,
                FanoutSurfaceClass::LockScreenSummary,
            ],
            "Security notice requires attention",
            "ux:reopen:beta:lock-screen-forbidden:01",
            "obj:secret-broker:credential-review:01",
            "cmd:secret_broker.open_credential_review",
        ),
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::SecurityNotices,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Security-critical envelope denies lock-screen render outright (policy_forbidden_on_lock_screen). In-product durable surfaces still light up; the badge counts the security notice; the user must reopen in-product to inspect the detail.",
    }
}

fn quiet_hours_held() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:quiet-hours-held:01",
        row_id: "ux:notification-privacy:beta:row:quiet-hours-held",
        row_class: NotificationPrivacyBetaRowClass::QuietHoursHeld,
        envelope: baseline_envelope(
            "ux:notif-env:beta:quiet-hours-held:01",
            "ux:event:beta:quiet-hours-held:01",
            "ux:dedupe:beta:quiet-hours-held:01",
            SourceSubsystem::BuildSystem,
            SeverityClass::Warning,
            PrivacyClass::WorkspaceSensitive,
            PrivacyPayloadClass::LockScreenSafeGeneric,
            RedactionClass::OperatorOnlyRestricted,
            DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::Toast,
                FanoutSurfaceClass::OsNotification,
            ],
            "Build degraded",
            "ux:reopen:beta:quiet-hours-held:01",
            "obj:build-system:degraded-cache:01",
            "cmd:build_system.open_job_details",
        ),
        posture: QuietHoursPosture::quiet_hours_user(),
        badge_class: BadgeClass::HeldOrSuppressedCount,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Quiet-hours-user holds the toast and OS notification; the durable row and status item still deliver. The OS app-icon badge mirror is suppressed by policy, but the in-product badge still renders the held-or-suppressed count.",
    }
}

fn admin_policy_suppressed() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:admin-policy-suppressed:01",
        row_id: "ux:notification-privacy:beta:row:admin-policy-suppressed",
        row_class: NotificationPrivacyBetaRowClass::AdminPolicySuppressed,
        envelope: baseline_envelope(
            "ux:notif-env:beta:admin-policy-suppressed:01",
            "ux:event:beta:admin-policy-suppressed:01",
            "ux:dedupe:beta:admin-policy-suppressed:01",
            SourceSubsystem::AdminPolicy,
            SeverityClass::Warning,
            PrivacyClass::ManagedSensitive,
            PrivacyPayloadClass::PolicyForbiddenOnLockScreen,
            RedactionClass::InternalSupportRestricted,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::ContextualBanner,
                FanoutSurfaceClass::LockScreenSummary,
            ],
            "Managed policy update pending",
            "ux:reopen:beta:admin-policy-suppressed:01",
            "obj:admin-policy:managed-rule:01",
            "cmd:admin_policy.open_managed_review",
        ),
        posture: QuietHoursPosture::admin_suppression(),
        badge_class: BadgeClass::HeldOrSuppressedCount,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Managed admin-suppression denies the banner and lock-screen surfaces outright. Durable truth survives (durable row and status item still deliver) and the suppression reason is recorded on the receipt.",
    }
}

fn critical_safety_escalation() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:critical-safety-escalation:01",
        row_id: "ux:notification-privacy:beta:row:critical-safety-escalation",
        row_class: NotificationPrivacyBetaRowClass::CriticalSafetyEscalation,
        envelope: baseline_envelope(
            "ux:notif-env:beta:critical-safety-escalation:01",
            "ux:event:beta:critical-safety-escalation:01",
            "ux:dedupe:beta:critical-safety-escalation:01",
            SourceSubsystem::SecretBroker,
            SeverityClass::Critical,
            PrivacyClass::SecurityCritical,
            PrivacyPayloadClass::RedactedMetadataOnly,
            RedactionClass::InternalSupportRestricted,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::ContextualBanner,
                FanoutSurfaceClass::OsNotification,
            ],
            "Credential rotation required",
            "ux:reopen:beta:critical-safety-escalation:01",
            "obj:secret-broker:credential-rotation:01",
            "cmd:secret_broker.open_credential_rotation",
        ),
        posture: QuietHoursPosture::quiet_hours_user(),
        badge_class: BadgeClass::SecurityNotices,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "Critical-safety severity bypasses an active quiet-hours-user posture: durable truth, status item, and OS notification all deliver, and the badge counts a security notice. The bypass is audited on the row's quiet_hours_posture.",
    }
}

fn companion_cross_client_fanout() -> SeedScenario {
    let mut envelope = baseline_envelope(
        "ux:notif-env:beta:companion-cross-client:01",
        "ux:event:beta:companion-cross-client:01",
        "ux:dedupe:beta:companion-cross-client:01",
        SourceSubsystem::Collaboration,
        SeverityClass::Warning,
        PrivacyClass::WorkspaceSensitive,
        PrivacyPayloadClass::LockScreenSafeGeneric,
        RedactionClass::OperatorOnlyRestricted,
        DedupeKeyScheme::CrossClientCanonicalEventId,
        vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::OsNotification,
            FanoutSurfaceClass::CompanionPush,
        ],
        "Companion mention awaiting reply",
        "ux:reopen:beta:companion-cross-client:01",
        "obj:collaboration:mention:01",
        "cmd:collaboration.open_mention",
    );
    envelope.grouped_burst_id_ref = None;
    SeedScenario {
        case_id: "shell:notification-privacy:beta:companion-cross-client:01",
        row_id: "ux:notification-privacy:beta:row:companion-cross-client",
        row_class: NotificationPrivacyBetaRowClass::CompanionCrossClientFanout,
        envelope,
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::Mentions,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture {
            originating_scope: ClientScope::DesktopProduct,
            sibling_scopes: vec![
                ClientScope::CompanionSurface,
                ClientScope::RemoteAgent,
                ClientScope::ManagedAdminSurface,
            ],
            cross_client_dedupe_in_effect: true,
            payload_class_not_widened_across_clients: true,
            cross_client_dismissal_collapses: true,
        },
        narrative: "Desktop, companion, remote-agent, and managed-admin clients share one canonical event id. Cross-client dedupe collapses sibling deliveries; companion payload class never widens past the lineage; a desktop dismissal collapses the companion row.",
    }
}

fn forbidden_shortcut_bypass_refused() -> SeedScenario {
    SeedScenario {
        case_id: "shell:notification-privacy:beta:forbidden-shortcut:01",
        row_id: "ux:notification-privacy:beta:row:forbidden-shortcut",
        row_class: NotificationPrivacyBetaRowClass::ForbiddenShortcutBypassRefused,
        envelope: baseline_envelope(
            "ux:notif-env:beta:forbidden-shortcut:01",
            "ux:event:beta:forbidden-shortcut:01",
            "ux:dedupe:beta:forbidden-shortcut:01",
            SourceSubsystem::ProviderBearing,
            SeverityClass::Blocking,
            PrivacyClass::ManagedSensitive,
            PrivacyPayloadClass::RedactedMetadataOnly,
            RedactionClass::InternalSupportRestricted,
            DedupeKeyScheme::CanonicalEventId,
            vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::ContextualBanner,
                FanoutSurfaceClass::OsNotification,
                FanoutSurfaceClass::CompanionPush,
            ],
            "Publish requires review",
            "ux:reopen:beta:forbidden-shortcut:01",
            "obj:provider:publish-request:01",
            "cmd:review.open_publish_review",
        ),
        posture: QuietHoursPosture::none(),
        badge_class: BadgeClass::NeedsReview,
        coalesces_retry_burst: false,
        cross_client_posture: CrossClientFanoutPosture::desktop_only(),
        narrative: "An OS notification and a companion push for a publish request enumerate every forbidden shortcut class. The payload refuses to complete a destructive_publish_or_apply shortcut and routes mutation through the in-product review sheet.",
    }
}

// =====================================================================
// External payload projection helper used by the inspector.
// =====================================================================

/// Project an [`ExternalNotificationPayload`] for every external
/// surface tied to a routed notification, in stable surface order. The
/// shell uses this to verify that no external payload widens beyond
/// the envelope's privacy posture.
pub fn project_external_payloads(
    routed: &RoutedNotification,
) -> Vec<ExternalNotificationPayload> {
    let mut payloads: Vec<ExternalNotificationPayload> = Vec::new();
    for surface in [
        FanoutSurfaceClass::OsNotification,
        FanoutSurfaceClass::LockScreenSummary,
        FanoutSurfaceClass::CompanionPush,
    ] {
        if let Some(payload) = ExternalNotificationPayload::project(routed, surface) {
            payloads.push(payload);
        }
    }
    payloads
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_passes_validation() {
        let page = seeded_notification_privacy_beta_page();
        validate_notification_privacy_beta_page(&page)
            .expect("seeded page must validate");
    }

    #[test]
    fn seeded_page_covers_required_row_classes() {
        let page = seeded_notification_privacy_beta_page();
        assert!(
            page.covers_required_row_classes(),
            "seeded page must exercise every required row class"
        );
    }

    #[test]
    fn seeded_summary_aggregates_match_row_count() {
        let page = seeded_notification_privacy_beta_page();
        assert_eq!(page.summary.row_count, page.rows.len());
        let coalescing = page
            .rows
            .iter()
            .filter(|r| r.expected_coalescing_posture.coalesces_retry_burst)
            .count();
        assert_eq!(page.summary.coalescing_row_count, coalescing);
        let cross_client = page
            .rows
            .iter()
            .filter(|r| r.cross_client_posture.cross_client_dedupe_in_effect)
            .count();
        assert_eq!(page.summary.cross_client_row_count, cross_client);
        let critical_bypass = page
            .rows
            .iter()
            .filter(|r| r.quiet_hours_posture.bypassed_by_critical_severity)
            .count();
        assert_eq!(page.summary.critical_safety_bypass_row_count, critical_bypass);
    }

    #[test]
    fn validation_flags_privacy_posture_drift() {
        let mut page = seeded_notification_privacy_beta_page();
        page.rows[0].privacy_class = PrivacyClass::SecurityCritical;
        // Leave payload_class as LockScreenSafeGeneric — that is the drift.
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag privacy posture drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::PrivacyPostureDrift { .. }
        )));
    }

    #[test]
    fn validation_flags_lock_screen_posture_drift() {
        let mut page = seeded_notification_privacy_beta_page();
        // Force a posture that does not match the payload class.
        page.rows[0].lock_screen_posture = LockScreenPosture::ScopedWorkspaceSafe;
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag lock-screen posture drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::LockScreenPostureDrift { .. }
        )));
    }

    #[test]
    fn validation_flags_coalescing_scheme_incompatibility() {
        let mut page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.expected_coalescing_posture.coalesces_retry_burst)
            .expect("seed has a coalescing row");
        row.dedupe_key_scheme = DedupeKeyScheme::CanonicalEventId;
        row.expected_coalescing_posture.dedupe_key_scheme = DedupeKeyScheme::CanonicalEventId;
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag coalescing scheme incompatibility");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::CoalescingSchemeIncompatible { .. }
        )));
    }

    #[test]
    fn validation_flags_cross_client_dedupe_missing() {
        let mut page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.cross_client_posture.cross_client_dedupe_in_effect)
            .expect("seed has a cross-client row");
        row.cross_client_posture.cross_client_dedupe_in_effect = false;
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag cross-client dedupe missing");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::CrossClientDedupeMissing { .. }
        )));
    }

    #[test]
    fn validation_flags_critical_safety_hold_admitted() {
        let mut page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| matches!(r.severity_class, SeverityClass::Critical))
            .expect("seed has a critical row");
        row.quiet_hours_posture.holds_attention_surfaces = true;
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag critical hold admitted");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::CriticalSafetyHoldAdmitted { .. }
        )));
    }

    #[test]
    fn validation_flags_forbidden_shortcut_list_incomplete() {
        let mut page = seeded_notification_privacy_beta_page();
        page.rows[0].forbidden_shortcut_posture.forbidden_classes.pop();
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag forbidden shortcut list incomplete");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::ForbiddenShortcutListIncomplete { .. }
        )));
    }

    #[test]
    fn validation_flags_badge_parity_drift() {
        let mut page = seeded_notification_privacy_beta_page();
        page.badges[0].badge_class = BadgeClass::OfflinePublishPending;
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag badge parity drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::BadgeRowParityDrift { field, .. } if field == "badge_class"
        )));
    }

    #[test]
    fn validation_flags_missing_badge() {
        let mut page = seeded_notification_privacy_beta_page();
        let removed_row_id = page.rows[0].row_id.clone();
        page.badges.retain(|b| b.row_id != removed_row_id);
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag missing badge");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::BadgeMissingForRow { .. }
        )));
    }

    #[test]
    fn validation_flags_row_class_coverage_gap() {
        let mut page = seeded_notification_privacy_beta_page();
        let dropped = NotificationPrivacyBetaRowClass::CompanionCrossClientFanout;
        page.rows.retain(|r| r.row_class != dropped);
        let removed_ids: Vec<String> = page
            .rows
            .iter()
            .filter(|r| r.row_class == dropped)
            .map(|r| r.row_id.clone())
            .collect();
        page.badges.retain(|b| !removed_ids.contains(&b.row_id));
        page.summary = NotificationPrivacyBetaSummary::from_rows(&page.rows);
        let errors = validate_notification_privacy_beta_page(&page)
            .expect_err("must flag missing row class");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::RowClassCoverageIncomplete { .. }
        )));
    }

    #[test]
    fn support_export_quotes_every_row_and_case_id() {
        let page = seeded_notification_privacy_beta_page();
        let export = NotificationPrivacyBetaSupportExport::from_page(
            "support-export:notification-privacy-beta:001",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            NOTIFICATION_PRIVACY_BETA_SHARED_CONTRACT_REF
        );
        assert_eq!(export.rows.len(), page.rows.len());
        assert_eq!(export.case_ids.len(), page.rows.len());
        for (page_row, export_row) in page.rows.iter().zip(export.rows.iter()) {
            assert_eq!(page_row.row_id, export_row.row_id);
            assert_eq!(page_row.row_class, export_row.row_class);
            assert_eq!(page_row.privacy_class, export_row.privacy_class);
            assert_eq!(page_row.lock_screen_posture, export_row.lock_screen_posture);
            assert_eq!(page_row.badge_class, export_row.badge_class);
            assert_eq!(page_row.dedupe_key_scheme, export_row.dedupe_key_scheme);
        }
        validate_notification_privacy_beta_support_export(&export)
            .expect("support export must validate against its embedded page");
    }

    #[test]
    fn support_export_validation_flags_drift() {
        let page = seeded_notification_privacy_beta_page();
        let mut export = NotificationPrivacyBetaSupportExport::from_page(
            "support-export:notification-privacy-beta:001",
            "2026-05-15T00:00:00Z",
            page,
        );
        export.rows[0].badge_class = BadgeClass::OfflinePublishPending;
        let errors = validate_notification_privacy_beta_support_export(&export)
            .expect_err("must flag drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            NotificationPrivacyBetaValidationError::SupportExportParityDrift { field, .. } if field == "badge_class"
        )));
    }

    #[test]
    fn coalesced_row_is_dedupe_repeat_and_occurrence_two() {
        let page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter()
            .find(|r| {
                matches!(
                    r.row_class,
                    NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure
                )
            })
            .expect("seed has the coalesced row");
        assert!(row.is_dedupe_repeat, "retry coalescing row must be a dedupe repeat");
        assert_eq!(row.occurrence_count, 2);
        assert!(matches!(
            row.dedupe_key_scheme,
            DedupeKeyScheme::SubsystemPlusObjectPlusPhase
        ));
    }

    #[test]
    fn critical_row_records_bypass_and_no_hold() {
        let page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter()
            .find(|r| matches!(r.severity_class, SeverityClass::Critical))
            .expect("seed has a critical row");
        assert!(
            row.quiet_hours_posture.bypassed_by_critical_severity,
            "critical-severity row must record a quiet-hours bypass"
        );
        assert!(
            !row.quiet_hours_posture.holds_attention_surfaces,
            "critical-severity row must never hold an attention surface"
        );
    }

    #[test]
    fn cross_client_row_has_three_sibling_scopes() {
        let page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter()
            .find(|r| {
                matches!(
                    r.row_class,
                    NotificationPrivacyBetaRowClass::CompanionCrossClientFanout
                )
            })
            .expect("seed has a cross-client row");
        assert_eq!(row.cross_client_posture.sibling_scopes.len(), 3);
        assert!(matches!(
            row.dedupe_key_scheme,
            DedupeKeyScheme::CrossClientCanonicalEventId
        ));
    }

    #[test]
    fn lock_screen_forbidden_row_uses_in_product_summary_label() {
        let page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter()
            .find(|r| {
                matches!(
                    r.row_class,
                    NotificationPrivacyBetaRowClass::LockScreenForbiddenSecurityCritical
                )
            })
            .expect("seed has a lock-screen-forbidden row");
        assert!(matches!(
            row.lock_screen_posture,
            LockScreenPosture::PolicyForbiddenOnLockScreen
        ));
    }

    #[test]
    fn quiet_hours_held_row_records_os_app_icon_suppressed() {
        let page = seeded_notification_privacy_beta_page();
        let row = page
            .rows
            .iter()
            .find(|r| matches!(r.row_class, NotificationPrivacyBetaRowClass::QuietHoursHeld))
            .expect("seed has a quiet-hours-held row");
        assert!(row.quiet_hours_posture.holds_attention_surfaces);
        assert!(row.quiet_hours_posture.os_app_icon_badge_suppressed);
        assert!(row.quiet_hours_posture.durable_truth_preserved);
    }

    #[test]
    fn page_supports_serde_round_trip() {
        let page = seeded_notification_privacy_beta_page();
        let json = serde_json::to_string(&page).expect("must serialize");
        let parsed: NotificationPrivacyBetaPage =
            serde_json::from_str(&json).expect("must round-trip");
        assert_eq!(parsed, page);
    }

    #[test]
    fn project_external_payloads_excludes_in_product_only_surfaces() {
        let mut router = NotificationRouter::new();
        let scenario = lock_screen_safe_generic();
        let mut envelope = scenario.envelope;
        scenario.posture.apply_to_envelope(&mut envelope);
        let routed = router.route(&envelope).unwrap();
        let payloads = project_external_payloads(&routed);
        for payload in &payloads {
            // No payload may quote a raw object identity or workspace
            // identifier on the lock-screen surface. The summary label
            // is the privacy-safe projection (generic / scoped).
            assert!(payload.raw_private_material_excluded);
            assert!(payload.shortcut_bypass_prohibited);
            assert!(!payload.summary_label.contains("obj:"));
        }
    }
}
