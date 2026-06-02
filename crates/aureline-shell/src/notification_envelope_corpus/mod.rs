//! Durable-notification routing corpus, drift drills, badge-integrity probes,
//! overlay-parity proofs, and route/outcome support export.
//!
//! This is the comprehensive proof corpus for the beta attention lane. Where
//! the seeded corpus in [`crate::attention_router::corpus`] is a representative
//! cross-surface coverage set, this module proves that *every beta job or alert
//! family that claims durable attention truth* routes correctly through the one
//! governed [`AttentionRouter`] under look-away, lock-screen, companion fanout,
//! presentation/follow, focus, screen-reader, quiet-hours, and stale-target
//! conditions.
//!
//! It composes the existing notification primitives rather than minting a
//! second routing engine:
//!
//! - [`crate::attention_router::AttentionRouter`] mints every
//!   [`NotificationRouteOutcome`] from truth, so a corpus case cannot drift
//!   from what ships.
//! - [`crate::notifications::router::NotificationRouter`] plus
//!   [`crate::notifications::quiet_hours::DurableBadgeProjection`] back the
//!   badge-integrity probes, so badge counts in the corpus are the same deduped
//!   counts the shell paints.
//!
//! What the corpus proves, machine-checked:
//!
//! - **Family coverage.** One worked routing case per beta attention family
//!   (indexing, restore, install/update/download, AI approvals, provider sync,
//!   policy change, remote reconnect, managed alert, and classroom/presentation
//!   overlays) with its expected route and actionability outcome.
//! - **Exact-target reopen.** Every resolved surface keeps the single reopen
//!   target; stale or missing targets reopen a truthful placeholder or a
//!   revalidation requirement — never a generic home view.
//! - **Drift drills.** A wrong-target reopen, a lock-screen leak, a badge
//!   inflation, and a quiet-hours drift are each constructed as an adversarial
//!   regression and shown to fail the conformance lane, producing an actionable
//!   structured diff.
//! - **Overlay parity.** Presentation, follow, focus, and screen-reader
//!   postures dim audience-visible surfaces but never change the resolution of
//!   the claimed durable rows or the reopen target.
//! - **Export reconstruction.** A support export reconstructs class, route,
//!   suppression, and resolution from stable enums alone — never from toast
//!   copy or badge text.
//!
//! The corpus, drills, probes, support export, and the three rendered artifacts
//! are minted by the headless inspector
//! [`crate::bin`] `aureline_shell_notification_envelope_corpus` and replayed by
//! `crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::attention_router::{
    ActiveWindowState, AttentionRouter, ChannelResolutionClass, CompanionAvailability,
    CompanionHandoffClass, NotificationRouteOutcome, PresentationFollowState, ScreenReaderPosture,
    SupportSurfaceResolution,
};
use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass,
    PrivacyPayloadClass, QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind,
    SeverityClass, SourceSubsystem, StableAction, SuppressionState,
};
use crate::notifications::quiet_hours::{DurableBadgeProjection, QuietHoursPosture};
use crate::notifications::router::NotificationRouter;

pub use crate::attention_router::context::ChannelContext;

/// Schema version exported by every notification-envelope-corpus record.
pub const NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref carried by every record so shell rows, headless CLI
/// rows, support-export rows, and the artifacts pivot to the same case id.
pub const NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF: &str =
    "shell:notification_envelope_corpus:v1";

pub const NOTIFICATION_ENVELOPE_CORPUS_PACKET_RECORD_KIND: &str =
    "shell_notification_envelope_corpus_packet_record";
pub const NOTIFICATION_ENVELOPE_CORPUS_CASE_RECORD_KIND: &str =
    "shell_notification_envelope_corpus_case_record";
pub const NOTIFICATION_ROUTE_DRIFT_DRILL_RECORD_KIND: &str =
    "shell_notification_route_drift_drill_record";
pub const NOTIFICATION_BADGE_INTEGRITY_PROBE_RECORD_KIND: &str =
    "shell_notification_badge_integrity_probe_record";
pub const NOTIFICATION_OVERLAY_PARITY_RECORD_KIND: &str =
    "shell_notification_overlay_parity_record";
pub const NOTIFICATION_ROUTE_OUTCOME_EXPORT_RECORD_KIND: &str =
    "shell_notification_route_outcome_export_record";
pub const NOTIFICATION_ROUTE_OUTCOME_EXPORT_ROW_RECORD_KIND: &str =
    "shell_notification_route_outcome_export_row_record";

pub const NOTIFICATION_ENVELOPE_CORPUS_PACKET_ID: &str =
    "shell:notification_envelope_corpus:packet:001";
pub const NOTIFICATION_ENVELOPE_CORPUS_GENERATED_AT: &str = "2026-05-20T00:00:00Z";
pub const NOTIFICATION_ROUTE_OUTCOME_EXPORT_ID: &str =
    "support-export:notification-route-outcome:001";

// ---------------------------------------------------------------------------
// Vocabulary enums
// ---------------------------------------------------------------------------

/// A beta job or alert family that claims durable attention truth. Each family
/// has at least one worked routing case in the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaAttentionFamily {
    Indexing,
    Restore,
    InstallUpdateDownload,
    AiApproval,
    ProviderSync,
    PolicyChange,
    RemoteReconnect,
    ManagedAlert,
    ClassroomPresentationOverlay,
}

impl BetaAttentionFamily {
    /// Stable token recorded in records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexing => "indexing",
            Self::Restore => "restore",
            Self::InstallUpdateDownload => "install_update_download",
            Self::AiApproval => "ai_approval",
            Self::ProviderSync => "provider_sync",
            Self::PolicyChange => "policy_change",
            Self::RemoteReconnect => "remote_reconnect",
            Self::ManagedAlert => "managed_alert",
            Self::ClassroomPresentationOverlay => "classroom_presentation_overlay",
        }
    }

    /// Every beta attention family that claims durable attention truth.
    pub fn all() -> [BetaAttentionFamily; 9] {
        [
            Self::Indexing,
            Self::Restore,
            Self::InstallUpdateDownload,
            Self::AiApproval,
            Self::ProviderSync,
            Self::PolicyChange,
            Self::RemoteReconnect,
            Self::ManagedAlert,
            Self::ClassroomPresentationOverlay,
        ]
    }
}

/// How a case's reopen target resolves. Proves a stale or missing target
/// reopens a truthful placeholder or revalidation requirement, never a generic
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenProofClass {
    /// The reopen target resolves to a concrete canonical identity.
    ExactTarget,
    /// The target is unavailable; the reopen announces a truthful placeholder.
    TruthfulPlaceholder,
    /// The target requires revalidation before it can be reopened.
    DeniedRequiresRevalidation,
}

impl ReopenProofClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTarget => "exact_target",
            Self::TruthfulPlaceholder => "truthful_placeholder",
            Self::DeniedRequiresRevalidation => "denied_requires_revalidation",
        }
    }
}

/// An adversarial route regression the conformance lane must reject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteDriftViolation {
    /// A surface reopens a target other than the outcome's single reopen target.
    WrongTargetReopen,
    /// A lock-screen surface renders when the payload class forbids it.
    LockScreenLeakage,
    /// The badge count inflates from duplicate deliveries.
    BadgeInflation,
    /// A quiet-hours-held surface is upgraded back to delivered.
    QuietHoursDrift,
}

impl RouteDriftViolation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongTargetReopen => "wrong_target_reopen",
            Self::LockScreenLeakage => "lock_screen_leakage",
            Self::BadgeInflation => "badge_inflation",
            Self::QuietHoursDrift => "quiet_hours_drift",
        }
    }

    pub fn all() -> [RouteDriftViolation; 4] {
        [
            Self::WrongTargetReopen,
            Self::LockScreenLeakage,
            Self::BadgeInflation,
            Self::QuietHoursDrift,
        ]
    }
}

/// An attention-suppressing posture overlaid on a baseline route. Proves the
/// overlay dims audience-visible surfaces without changing the claimed durable
/// rows or the reopen target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayPosture {
    Presenting,
    FollowingPresenter,
    FocusMode,
    ScreenReaderActive,
}

impl OverlayPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presenting => "presenting",
            Self::FollowingPresenter => "following_presenter",
            Self::FocusMode => "focus_mode",
            Self::ScreenReaderActive => "screen_reader_active",
        }
    }

    pub fn all() -> [OverlayPosture; 4] {
        [
            Self::Presenting,
            Self::FollowingPresenter,
            Self::FocusMode,
            Self::ScreenReaderActive,
        ]
    }
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// One worked routing case: a family, the scenario, the live posture, the
/// resolved outcome, and the reopen proof class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelopeCorpusCase {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub attention_family: BetaAttentionFamily,
    pub scenario_label: String,
    pub emissions: u32,
    pub active_window_state: ActiveWindowState,
    pub screen_reader_posture: ScreenReaderPosture,
    pub companion_availability: CompanionAvailability,
    pub presentation_follow_state: PresentationFollowState,
    pub reopen_proof: ReopenProofClass,
    pub conformant: bool,
    pub outcome: NotificationRouteOutcome,
}

/// A structured before/after diff for a drift drill. The lane catching the
/// regression is the proof; this is the actionable artifact a reviewer reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDriftDiff {
    pub diff_field: String,
    pub conforming_observation: String,
    pub regressed_observation: String,
}

/// One drift drill: an adversarial regression of a real outcome that the
/// conformance lane must reject with a stable reason token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDriftDrill {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub drill_id: String,
    pub violation_class: RouteDriftViolation,
    pub drill_label: String,
    pub baseline_case_id: String,
    pub diff: RouteDriftDiff,
    /// The conforming outcome passes the conformance lane clean.
    pub conforming_passes_clean: bool,
    /// The regressed outcome fails the conformance lane.
    pub lane_rejects_regression: bool,
    /// The stable reason tokens the lane produces for the regression.
    pub rejection_reason_tokens: Vec<String>,
}

/// One badge-integrity probe: routes the same canonical event N times through
/// the dedupe core and proves the durable badge count never inflates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeIntegrityProbe {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub probe_id: String,
    pub attention_family: BetaAttentionFamily,
    pub raw_emissions: u32,
    pub durable_badge_count: u32,
    pub os_app_icon_badge_visible: bool,
    pub lock_screen_summary_visible: bool,
    pub privacy_safe_summary_label: String,
    /// True when N raw emissions collapse to a single durable item — the lane
    /// prevented inflation.
    pub inflation_prevented: bool,
}

/// One overlay-parity proof: the same envelope routed under a baseline and an
/// overlay posture, proving the claimed durable rows and reopen target match.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayParityProof {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub parity_id: String,
    pub attention_family: BetaAttentionFamily,
    pub overlay_posture: OverlayPosture,
    pub durable_resolution_matches_baseline: bool,
    pub reopen_target_matches_baseline: bool,
    pub baseline_audience_visible: bool,
    pub overlay_audience_visible: bool,
}

/// One support-export row. Carries support-safe enums and per-surface
/// resolutions only — never the summary copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRouteOutcomeExportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub attention_family: BetaAttentionFamily,
    pub route_outcome_id: String,
    pub canonical_event_id: String,
    pub source_subsystem: SourceSubsystem,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub privacy_payload_class: PrivacyPayloadClass,
    pub redaction_class: RedactionClass,
    pub active_window_state: ActiveWindowState,
    pub presentation_follow_state: PresentationFollowState,
    pub companion_handoff_class: CompanionHandoffClass,
    pub reopen_proof: ReopenProofClass,
    pub surface_resolutions: Vec<SupportSurfaceResolution>,
    pub occurrence_count: u32,
    pub is_dedupe_repeat: bool,
    pub durable_truth_preserved: bool,
    pub all_routes_preserve_reopen_target: bool,
    pub no_generic_home_reopen: bool,
}

/// Support-export wrapper over the corpus. Quotes class, route, suppression,
/// and resolution through enums; never quotes summary copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRouteOutcomeExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub rows: Vec<NotificationRouteOutcomeExportRow>,
    pub raw_user_facing_copy_excluded: bool,
}

/// Aggregate coverage and verdict summary for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelopeCorpusCoverage {
    pub case_count: u32,
    pub families_covered: Vec<BetaAttentionFamily>,
    pub surfaces_covered: Vec<FanoutSurfaceClass>,
    pub resolution_classes_covered: Vec<ChannelResolutionClass>,
    pub companion_handoff_classes_covered: Vec<CompanionHandoffClass>,
    pub reopen_proof_classes_covered: Vec<ReopenProofClass>,
    pub drift_violations_covered: Vec<RouteDriftViolation>,
    pub overlay_postures_covered: Vec<OverlayPosture>,
    pub all_families_present: bool,
    pub all_cases_conform: bool,
    pub all_cases_preserve_reopen_target: bool,
    pub all_cases_preserve_durable_truth: bool,
    pub all_cases_truthful_reopen: bool,
    pub all_drills_reject_regression: bool,
    pub all_overlays_preserve_durable_semantics: bool,
    pub badge_never_inflates: bool,
}

/// The full notification-envelope-corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelopeCorpusPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub packet_id: String,
    pub generated_at: String,
    pub coverage: NotificationEnvelopeCorpusCoverage,
    pub cases: Vec<NotificationEnvelopeCorpusCase>,
    pub drift_drills: Vec<RouteDriftDrill>,
    pub badge_probes: Vec<BadgeIntegrityProbe>,
    pub overlay_parity: Vec<OverlayParityProof>,
    pub support_export: NotificationRouteOutcomeExport,
}

// ---------------------------------------------------------------------------
// Envelope / context builders
// ---------------------------------------------------------------------------

fn exact_reopen(reopen_ref: &str, kind: ReopenTargetKind, target: &str) -> ReopenTarget {
    ReopenTarget {
        reopen_target_ref: reopen_ref.to_owned(),
        reopen_target_kind: kind,
        exact_target_identity_ref: Some(target.to_owned()),
        placeholder_announcement_label: None,
        revalidation_required_reason_label: None,
    }
}

fn placeholder_reopen(reopen_ref: &str, announcement: &str) -> ReopenTarget {
    ReopenTarget {
        reopen_target_ref: reopen_ref.to_owned(),
        reopen_target_kind: ReopenTargetKind::PlaceholderAnnounced,
        exact_target_identity_ref: None,
        placeholder_announcement_label: Some(announcement.to_owned()),
        revalidation_required_reason_label: None,
    }
}

fn revalidation_reopen(reopen_ref: &str, reason: &str) -> ReopenTarget {
    ReopenTarget {
        reopen_target_ref: reopen_ref.to_owned(),
        reopen_target_kind: ReopenTargetKind::DeniedRequiresRevalidation,
        exact_target_identity_ref: None,
        placeholder_announcement_label: None,
        revalidation_required_reason_label: Some(reason.to_owned()),
    }
}

fn open_action(action_id: &str, label: &str, command_id: &str, target: &str) -> StableAction {
    StableAction {
        action_id: action_id.to_owned(),
        label: label.to_owned(),
        command_id: command_id.to_owned(),
        target_identity_ref: target.to_owned(),
        reopen_target_kind: ReopenTargetKind::DurableActivityRow,
        is_destructive: false,
    }
}

fn no_suppression() -> SuppressionState {
    SuppressionState {
        active_modes_at_mint: vec![QuietHoursMode::ModeNone],
        suppression_reasons: vec![],
        suppressed: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn envelope(
    id: &str,
    source: SourceSubsystem,
    severity: SeverityClass,
    privacy: PrivacyClass,
    payload: PrivacyPayloadClass,
    redaction: RedactionClass,
    dedupe_scheme: DedupeKeyScheme,
    dedupe_ref: &str,
    surfaces: Vec<FanoutSurfaceClass>,
    summary: &str,
    reopen: ReopenTarget,
    actions: Vec<StableAction>,
    minted_at: &str,
) -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".to_owned(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: format!("ux:notif-env:{id}"),
        canonical_event_id: format!("ux:event:{id}"),
        event_lineage_id_ref: format!("ux:lineage:{id}"),
        source_subsystem: source,
        source_event_ref: format!("src:{id}"),
        actor_identity_ref: "id:actor:system:attention-router".to_owned(),
        canonical_object_target_ref: format!("obj:{id}"),
        severity_class: severity,
        privacy_class: privacy,
        privacy_payload_class: payload,
        redaction_class: redaction,
        dedupe_key_scheme: dedupe_scheme,
        dedupe_key_ref: dedupe_ref.to_owned(),
        grouped_burst_id_ref: None,
        recommended_surfaces: surfaces,
        summary_label: summary.to_owned(),
        reopen_target: reopen,
        actions,
        suppression_state: no_suppression(),
        fanout_receipts: vec![],
        minted_at: minted_at.to_owned(),
    }
}

struct SeedCase {
    case_id: &'static str,
    family: BetaAttentionFamily,
    scenario_label: &'static str,
    emissions: u32,
    context: ChannelContext,
    envelope: NotificationEnvelope,
}

fn reopen_proof_for(reopen: &ReopenTarget) -> ReopenProofClass {
    if reopen.resolves_to_exact_target() {
        ReopenProofClass::ExactTarget
    } else {
        match reopen.reopen_target_kind {
            ReopenTargetKind::DeniedRequiresRevalidation => {
                ReopenProofClass::DeniedRequiresRevalidation
            }
            _ => ReopenProofClass::TruthfulPlaceholder,
        }
    }
}

/// Route a seed case `emissions` times through a fresh router so dedupe memory
/// never bleeds across cases.
fn outcome_for(seed: &SeedCase) -> NotificationRouteOutcome {
    let mut router = AttentionRouter::new();
    let mut outcome = router
        .route(&seed.envelope, &seed.context)
        .expect("seed envelope must route");
    for _ in 1..seed.emissions {
        outcome = router
            .route(&seed.envelope, &seed.context)
            .expect("seed envelope must route");
    }
    outcome
}

fn seed_cases() -> Vec<SeedCase> {
    vec![
        // Indexing, foreground focused: in-app surfaces deliver; the redundant
        // OS notification is dropped while the window is focused.
        SeedCase {
            case_id: "case:indexing-foreground-focused",
            family: BetaAttentionFamily::Indexing,
            scenario_label:
                "Indexing finishes a partial-shard pass while the window is foreground and focused. The durable row, status item, and toast deliver in-app; the redundant OS notification is dropped but stays a visible receipt.",
            emissions: 1,
            context: ChannelContext::foreground_focused(),
            envelope: envelope(
                "indexing:shard:01",
                SourceSubsystem::Indexer,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:indexing:shard:01:partial",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Indexing on a partial shard",
                exact_reopen(
                    "ux:reopen:indexing:shard:01",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:indexing:shard:01",
                ),
                vec![open_action(
                    "ux:action:indexing:open:01",
                    "Open indexer activity",
                    "cmd:indexer.open_shard_activity",
                    "obj:indexing:shard:01",
                )],
                "2026-05-20T10:00:00Z",
            ),
        },
        // Indexing under presentation: audience-visible surfaces are held while
        // durable truth keeps flowing.
        SeedCase {
            case_id: "case:indexing-presenting-overlay",
            family: BetaAttentionFamily::Indexing,
            scenario_label:
                "Indexing reports progress while the user is presenting. The toast and OS notification are suppressed as audience-visible surfaces; the durable row and status item still deliver.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::ForegroundUnfocused)
                .with_presentation(PresentationFollowState::Presenting),
            envelope: envelope(
                "indexing:progress:02",
                SourceSubsystem::Indexer,
                SeverityClass::Info,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:indexing:progress:02",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Indexing progress",
                exact_reopen(
                    "ux:reopen:indexing:progress:02",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:indexing:progress:02",
                ),
                vec![open_action(
                    "ux:action:indexing:open:02",
                    "Open indexer activity",
                    "cmd:indexer.open_shard_activity",
                    "obj:indexing:progress:02",
                )],
                "2026-05-20T10:05:00Z",
            ),
        },
        // Restore against a target still rebuilding: a truthful placeholder
        // reopen rather than a generic home view.
        SeedCase {
            case_id: "case:restore-placeholder-reopen",
            family: BetaAttentionFamily::Restore,
            scenario_label:
                "A restore completes while its target object is still being rebuilt and no window is foreground. The reopen announces a truthful placeholder; the durable row, status item, and toast deliver.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden),
            envelope: envelope(
                "restore:rebuilding:03",
                SourceSubsystem::VfsSave,
                SeverityClass::Degraded,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::InProductOnly,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:restore:rebuilding:03",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                ],
                "Restore target rebuilding",
                placeholder_reopen(
                    "ux:reopen:restore:rebuilding:03",
                    "Restore target is still rebuilding.",
                ),
                vec![open_action(
                    "ux:action:restore:open:03",
                    "Open recovery center",
                    "cmd:recovery.open_center",
                    "obj:restore:rebuilding:03",
                )],
                "2026-05-20T10:10:00Z",
            ),
        },
        // Install/update/download while the device is locked: the lock-screen
        // summary and OS notification deliver summary-first.
        SeedCase {
            case_id: "case:install-update-download-locked",
            family: BetaAttentionFamily::InstallUpdateDownload,
            scenario_label:
                "A download-and-install update finishes while the device is locked. The lock-screen summary and OS notification deliver summary-first; durable truth is preserved for the return.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::LockedOrAway),
            envelope: envelope(
                "install:update:04",
                SourceSubsystem::InstallUpdateAttach,
                SeverityClass::Success,
                PrivacyClass::SummarySafe,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::MetadataSafeDefault,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:install:update:04:complete",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                ],
                "Update installed",
                exact_reopen(
                    "ux:reopen:install:update:04",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:install:update:04",
                ),
                vec![open_action(
                    "ux:action:install:open:04",
                    "Open update",
                    "cmd:install.open_update",
                    "obj:install:update:04",
                )],
                "2026-05-20T10:15:00Z",
            ),
        },
        // AI approval ready with a screen reader active: a navigable durable
        // surface is guaranteed and the announcement is required.
        SeedCase {
            case_id: "case:ai-approval-screen-reader",
            family: BetaAttentionFamily::AiApproval,
            scenario_label:
                "An AI apply request needs approval with a screen reader active and the window focused. A navigable durable surface is present, the announcement is required, and the redundant OS notification is dropped.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_screen_reader(ScreenReaderPosture::Active),
            envelope: envelope(
                "ai:approval:05",
                SourceSubsystem::AiApply,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::InProductOnly,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
                "dedupe:ai:approval:05:ready",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::ActivityCenterDigestCard,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "AI change awaiting approval",
                exact_reopen(
                    "ux:reopen:ai:approval:05",
                    ReopenTargetKind::ReviewContext,
                    "obj:ai:approval:05",
                ),
                vec![open_action(
                    "ux:action:ai:open:05",
                    "Open approval",
                    "cmd:ai.open_review",
                    "obj:ai:approval:05",
                )],
                "2026-05-20T10:20:00Z",
            ),
        },
        // Provider sync whose token expired: a revalidation-required reopen and
        // an unreachable companion that stays a visible receipt.
        SeedCase {
            case_id: "case:provider-sync-revalidation",
            family: BetaAttentionFamily::ProviderSync,
            scenario_label:
                "A provider sync stalls because its token expired while no window is foreground and no companion is paired. The reopen requires revalidation; the OS notification delivers; the companion push is not attempted but stays a visible receipt.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::Unpaired),
            envelope: envelope(
                "provider:sync:06",
                SourceSubsystem::ProviderBearing,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::InProductOnly,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
                "dedupe:provider:sync:06:stalled",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Provider sync needs re-auth",
                revalidation_reopen(
                    "ux:reopen:provider:sync:06",
                    "Provider session expired; re-authentication required.",
                ),
                vec![open_action(
                    "ux:action:provider:open:06",
                    "Open provider settings",
                    "cmd:provider.open_settings",
                    "obj:provider:sync:06",
                )],
                "2026-05-20T10:25:00Z",
            ),
        },
        // Policy change under admin suppression: attention surfaces are
        // suppressed by policy; the durable status item still delivers.
        SeedCase {
            case_id: "case:policy-change-admin-suppressed",
            family: BetaAttentionFamily::PolicyChange,
            scenario_label:
                "A managed policy change fires under admin suppression while no window is foreground. The banner, OS notification, and lock-screen summary are suppressed by policy; the durable status item still delivers so the user has a path back.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_quiet_hours(QuietHoursPosture::admin_suppression()),
            envelope: envelope(
                "policy:change:07",
                SourceSubsystem::AdminPolicy,
                SeverityClass::Error,
                PrivacyClass::SecurityCritical,
                PrivacyPayloadClass::PolicyForbiddenOnLockScreen,
                RedactionClass::InternalSupportRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:policy:change:07",
                vec![
                    FanoutSurfaceClass::ContextualBanner,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                ],
                "Workspace policy changed",
                exact_reopen(
                    "ux:reopen:policy:change:07",
                    ReopenTargetKind::CanonicalObject,
                    "obj:policy:change:07",
                ),
                vec![open_action(
                    "ux:action:policy:open:07",
                    "Open policy change",
                    "cmd:policy.open_change",
                    "obj:policy:change:07",
                )],
                "2026-05-20T10:30:00Z",
            ),
        },
        // Remote reconnect with a reachable companion and an unlocked device:
        // the companion push delivers; the lock-screen summary is not
        // applicable.
        SeedCase {
            case_id: "case:remote-reconnect-companion-fanout",
            family: BetaAttentionFamily::RemoteReconnect,
            scenario_label:
                "A remote agent reconnects while the user is away from the desktop with a reachable companion and an unlocked device. The companion push and OS notification deliver summary-first; the lock-screen summary is not applicable because the device is unlocked.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PairedAvailable),
            envelope: envelope(
                "remote:reconnect:08",
                SourceSubsystem::RemoteAgent,
                SeverityClass::Success,
                PrivacyClass::SummarySafe,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::MetadataSafeDefault,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:remote:reconnect:08",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Remote session reconnected",
                exact_reopen(
                    "ux:reopen:remote:reconnect:08",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:remote:reconnect:08",
                ),
                vec![open_action(
                    "ux:action:remote:open:08",
                    "Open remote session",
                    "cmd:remote.open_session",
                    "obj:remote:reconnect:08",
                )],
                "2026-05-20T10:35:00Z",
            ),
        },
        // Managed alert under a companion policy block: the companion push is
        // suppressed by policy; durable truth still delivers.
        SeedCase {
            case_id: "case:managed-alert-companion-blocked",
            family: BetaAttentionFamily::ManagedAlert,
            scenario_label:
                "A managed-workspace alert fires where policy forbids companion fanout. The companion push is suppressed by policy; durable truth still delivers and the suppression stays an inspectable receipt.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PolicyBlocked),
            envelope: envelope(
                "managed:alert:09",
                SourceSubsystem::AdminPolicy,
                SeverityClass::Warning,
                PrivacyClass::ManagedSensitive,
                PrivacyPayloadClass::RedactedMetadataOnly,
                RedactionClass::InternalSupportRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:managed:alert:09",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Managed workspace alert",
                exact_reopen(
                    "ux:reopen:managed:alert:09",
                    ReopenTargetKind::CanonicalObject,
                    "obj:managed:alert:09",
                ),
                vec![open_action(
                    "ux:action:managed:open:09",
                    "Open alert",
                    "cmd:managed.open_alert",
                    "obj:managed:alert:09",
                )],
                "2026-05-20T10:40:00Z",
            ),
        },
        // Classroom / presentation overlay following a presenter: audience and
        // companion surfaces are held; durable rows still deliver.
        SeedCase {
            case_id: "case:classroom-presentation-overlay",
            family: BetaAttentionFamily::ClassroomPresentationOverlay,
            scenario_label:
                "A classroom session follows a presenter with a reachable companion. The toast, OS notification, and companion push are suppressed while the audience-visible overlay is active; the durable row and status item still deliver.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::ForegroundUnfocused)
                .with_companion(CompanionAvailability::PairedAvailable)
                .with_presentation(PresentationFollowState::FollowingPresenter),
            envelope: envelope(
                "classroom:overlay:10",
                SourceSubsystem::Collaboration,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:classroom:overlay:10",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Classroom session update",
                exact_reopen(
                    "ux:reopen:classroom:overlay:10",
                    ReopenTargetKind::CanonicalObject,
                    "obj:classroom:overlay:10",
                ),
                vec![open_action(
                    "ux:action:classroom:open:10",
                    "Open session",
                    "cmd:collab.open_session",
                    "obj:classroom:overlay:10",
                )],
                "2026-05-20T10:45:00Z",
            ),
        },
        // Indexing burst: the same canonical event fires four times. Repeats
        // coalesce on every surface while the reopen target stays stable.
        SeedCase {
            case_id: "case:indexing-dedupe-burst",
            family: BetaAttentionFamily::Indexing,
            scenario_label:
                "An indexer warning fires four times in quick succession while the window is foreground but unfocused. The first emission delivers; emissions two through four coalesce on every surface while the reopen target stays stable.",
            emissions: 4,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::ForegroundUnfocused),
            envelope: envelope(
                "indexing:burst:11",
                SourceSubsystem::Indexer,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:indexing:burst:11",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                ],
                "Indexer retrying a shard",
                exact_reopen(
                    "ux:reopen:indexing:burst:11",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:indexing:burst:11",
                ),
                vec![open_action(
                    "ux:action:indexing:open:11",
                    "Open indexer activity",
                    "cmd:indexer.open_shard_activity",
                    "obj:indexing:burst:11",
                )],
                "2026-05-20T10:50:00Z",
            ),
        },
        // Provider sync during quiet hours with a reachable companion: durable
        // truth delivers while the toast, OS, and companion surfaces are held.
        SeedCase {
            case_id: "case:provider-sync-quiet-hours-held",
            family: BetaAttentionFamily::ProviderSync,
            scenario_label:
                "A provider sync completes during quiet hours with a reachable companion and no window foreground. The durable row delivers; the toast, OS notification, and companion push are held; the companion handoff is recorded as held truth.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PairedAvailable)
                .with_quiet_hours(QuietHoursPosture::quiet_hours_user()),
            envelope: envelope(
                "provider:sync:12",
                SourceSubsystem::ProviderBearing,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:provider:sync:12",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Provider sync completed",
                exact_reopen(
                    "ux:reopen:provider:sync:12",
                    ReopenTargetKind::CanonicalObject,
                    "obj:provider:sync:12",
                ),
                vec![open_action(
                    "ux:action:provider:open:12",
                    "Open provider sync",
                    "cmd:provider.open_sync",
                    "obj:provider:sync:12",
                )],
                "2026-05-20T22:30:00Z",
            ),
        },
    ]
}

// ---------------------------------------------------------------------------
// Conformance lane (the regression gate)
// ---------------------------------------------------------------------------

fn is_durable_truth_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::DurableJobRow
            | FanoutSurfaceClass::StatusItem
            | FanoutSurfaceClass::StatusStrip
            | FanoutSurfaceClass::ActivityCenterDigestCard
            | FanoutSurfaceClass::DigestGroupRow
    )
}

fn is_navigable_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::DurableJobRow
            | FanoutSurfaceClass::StatusItem
            | FanoutSurfaceClass::ActivityCenterDigestCard
            | FanoutSurfaceClass::DigestGroupRow
    )
}

fn route_is_visible(state: FanoutReceiptState) -> bool {
    matches!(
        state,
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
    )
}

fn route_preserves_durable_path(surface: FanoutSurfaceClass, state: FanoutReceiptState) -> bool {
    is_durable_truth_surface(surface)
        && matches!(
            state,
            FanoutReceiptState::Delivered
                | FanoutReceiptState::ReleasedFromHold
                | FanoutReceiptState::DedupedCanonicalEvent
                | FanoutReceiptState::DedupedGroupedBurst
        )
}

fn reopen_is_truthful(reopen: &ReopenTarget) -> bool {
    reopen.resolves_to_exact_target()
        || matches!(
            reopen.reopen_target_kind,
            ReopenTargetKind::PlaceholderAnnounced | ReopenTargetKind::DeniedRequiresRevalidation
        )
}

/// The conformance lane. Recomputes every invariant from the resolved routes
/// (it does **not** trust the stored proof booleans), so an adversarial drill
/// that tampers a route is genuinely caught. Returns the stable reason tokens
/// for any violation; an empty list means the outcome conforms.
pub fn route_outcome_violations(outcome: &NotificationRouteOutcome) -> Vec<String> {
    let mut tokens = BTreeSet::new();

    let canonical_ref = &outcome.reopen_target.reopen_target_ref;
    for route in &outcome.resolved_surface_routes {
        if &route.reopen_target_ref != canonical_ref {
            tokens.insert("reopen_target_divergence".to_owned());
        }
        // A core decision of held / suppressed / deduped / no-route must never
        // be upgraded back to a delivered surface by live-channel resolution.
        let core_withheld = matches!(
            route.core_receipt_state,
            FanoutReceiptState::HeldQuietHours
                | FanoutReceiptState::SuppressedPolicy
                | FanoutReceiptState::DedupedCanonicalEvent
                | FanoutReceiptState::DedupedGroupedBurst
                | FanoutReceiptState::NotAttemptedNoRoute
        );
        if core_withheld && route_is_visible(route.resolved_receipt_state) {
            tokens.insert("held_surface_upgraded_to_delivered".to_owned());
        }
    }

    if !reopen_is_truthful(&outcome.reopen_target) {
        tokens.insert("generic_home_reopen".to_owned());
    }

    let durable_preserved = outcome.resolved_surface_routes.iter().any(|route| {
        route_preserves_durable_path(route.fanout_surface_class, route.resolved_receipt_state)
    });
    if !durable_preserved {
        tokens.insert("durable_truth_lost".to_owned());
    }

    // Lock-screen leakage: a payload class that forbids lock-screen rendering
    // must never leave a visible lock-screen summary.
    if matches!(
        outcome.privacy_payload_class,
        PrivacyPayloadClass::PolicyForbiddenOnLockScreen
    ) {
        for route in &outcome.resolved_surface_routes {
            if matches!(
                route.fanout_surface_class,
                FanoutSurfaceClass::LockScreenSummary
            ) && route_is_visible(route.resolved_receipt_state)
            {
                tokens.insert("lock_screen_leak".to_owned());
            }
        }
    }

    // Screen-reader posture must keep a navigable durable surface in view.
    if outcome.screen_reader_announce_required {
        let navigable = outcome.resolved_surface_routes.iter().any(|route| {
            is_navigable_surface(route.fanout_surface_class)
                && route_is_visible(route.resolved_receipt_state)
        });
        if !navigable {
            tokens.insert("screen_reader_no_navigable_surface".to_owned());
        }
    }

    tokens.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Drift drills
// ---------------------------------------------------------------------------

fn find_outcome<'a>(
    cases: &'a [NotificationEnvelopeCorpusCase],
    case_id: &str,
) -> &'a NotificationRouteOutcome {
    &cases
        .iter()
        .find(|c| c.case_id == case_id)
        .unwrap_or_else(|| panic!("drill references unknown case {case_id}"))
        .outcome
}

fn drift_drill(
    drill_id: &str,
    violation: RouteDriftViolation,
    drill_label: &str,
    baseline_case_id: &str,
    conforming: &NotificationRouteOutcome,
    regressed: NotificationRouteOutcome,
    diff: RouteDriftDiff,
) -> RouteDriftDrill {
    let conforming_passes_clean = route_outcome_violations(conforming).is_empty();
    let rejection_reason_tokens = route_outcome_violations(&regressed);
    RouteDriftDrill {
        record_kind: NOTIFICATION_ROUTE_DRIFT_DRILL_RECORD_KIND.to_owned(),
        schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        drill_id: drill_id.to_owned(),
        violation_class: violation,
        drill_label: drill_label.to_owned(),
        baseline_case_id: baseline_case_id.to_owned(),
        diff,
        conforming_passes_clean,
        lane_rejects_regression: !rejection_reason_tokens.is_empty(),
        rejection_reason_tokens,
    }
}

fn build_drift_drills(
    cases: &[NotificationEnvelopeCorpusCase],
    badge_probes: &[BadgeIntegrityProbe],
) -> Vec<RouteDriftDrill> {
    let mut drills = Vec::new();

    // 1. Wrong-target reopen: repoint one surface at a generic home target.
    {
        let base = find_outcome(cases, "case:indexing-foreground-focused");
        let mut regressed = base.clone();
        if let Some(route) = regressed.resolved_surface_routes.first_mut() {
            route.reopen_target_ref = "ux:reopen:generic-home".to_owned();
        }
        drills.push(drift_drill(
            "drill:wrong-target-reopen",
            RouteDriftViolation::WrongTargetReopen,
            "A surface is repointed at a generic home view instead of the outcome's single reopen target.",
            "case:indexing-foreground-focused",
            base,
            regressed,
            RouteDriftDiff {
                diff_field: "resolved_surface_routes[].reopen_target_ref".to_owned(),
                conforming_observation: base.reopen_target.reopen_target_ref.clone(),
                regressed_observation: "ux:reopen:generic-home".to_owned(),
            },
        ));
    }

    // 2. Lock-screen leakage: force a forbidden lock-screen summary visible.
    {
        let base = find_outcome(cases, "case:policy-change-admin-suppressed");
        let mut regressed = base.clone();
        for route in &mut regressed.resolved_surface_routes {
            if matches!(
                route.fanout_surface_class,
                FanoutSurfaceClass::LockScreenSummary
            ) {
                route.resolved_receipt_state = FanoutReceiptState::Delivered;
                route.visible = true;
            }
        }
        drills.push(drift_drill(
            "drill:lock-screen-leakage",
            RouteDriftViolation::LockScreenLeakage,
            "A lock-screen summary is forced visible even though the payload class forbids lock-screen rendering.",
            "case:policy-change-admin-suppressed",
            base,
            regressed,
            RouteDriftDiff {
                diff_field: "lock_screen_summary.resolved_receipt_state".to_owned(),
                conforming_observation: "suppressed_policy".to_owned(),
                regressed_observation: "delivered".to_owned(),
            },
        ));
    }

    // 3. Badge inflation: the deduped durable count vs the raw emission count.
    {
        let probe = badge_probes
            .iter()
            .find(|p| p.attention_family == BetaAttentionFamily::Indexing)
            .expect("indexing badge probe present");
        // The conforming "outcome" the lane checks is the deduped burst case;
        // the regression is the raw, un-deduped emission count.
        let base = find_outcome(cases, "case:indexing-dedupe-burst");
        let mut regressed = base.clone();
        // Simulate the inflation regression: a repeat that should have deduped
        // is instead re-delivered on the toast surface.
        for route in &mut regressed.resolved_surface_routes {
            if matches!(route.fanout_surface_class, FanoutSurfaceClass::Toast) {
                route.resolved_receipt_state = FanoutReceiptState::Delivered;
                route.visible = true;
            }
        }
        drills.push(drift_drill(
            "drill:badge-inflation",
            RouteDriftViolation::BadgeInflation,
            "A deduped repeat is re-delivered, which would inflate the badge above the single deduped durable item.",
            "case:indexing-dedupe-burst",
            base,
            regressed,
            RouteDriftDiff {
                diff_field: "durable_badge_count".to_owned(),
                conforming_observation: format!("durable_count={}", probe.durable_badge_count),
                regressed_observation: format!("raw_emissions={}", probe.raw_emissions),
            },
        ));
    }

    // 4. Quiet-hours drift: a held attention surface is upgraded to delivered.
    {
        let base = find_outcome(cases, "case:provider-sync-quiet-hours-held");
        let mut regressed = base.clone();
        for route in &mut regressed.resolved_surface_routes {
            if matches!(route.fanout_surface_class, FanoutSurfaceClass::Toast) {
                route.resolved_receipt_state = FanoutReceiptState::Delivered;
                route.visible = true;
            }
        }
        drills.push(drift_drill(
            "drill:quiet-hours-drift",
            RouteDriftViolation::QuietHoursDrift,
            "A toast held by quiet hours is upgraded back to delivered, breaking the quiet-hours hold.",
            "case:provider-sync-quiet-hours-held",
            base,
            regressed,
            RouteDriftDiff {
                diff_field: "toast.resolved_receipt_state".to_owned(),
                conforming_observation: "held_quiet_hours".to_owned(),
                regressed_observation: "delivered".to_owned(),
            },
        ));
    }

    drills
}

// ---------------------------------------------------------------------------
// Badge-integrity probes
// ---------------------------------------------------------------------------

fn badge_probe(
    probe_id: &str,
    family: BetaAttentionFamily,
    raw_emissions: u32,
    envelope: &NotificationEnvelope,
    posture: &QuietHoursPosture,
) -> BadgeIntegrityProbe {
    let mut router = NotificationRouter::new();
    let mut routed = Vec::new();
    for _ in 0..raw_emissions {
        routed.push(router.route(envelope).expect("badge probe envelope routes"));
    }
    let projection = DurableBadgeProjection::from_routed(&routed, posture);
    BadgeIntegrityProbe {
        record_kind: NOTIFICATION_BADGE_INTEGRITY_PROBE_RECORD_KIND.to_owned(),
        schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        probe_id: probe_id.to_owned(),
        attention_family: family,
        raw_emissions,
        durable_badge_count: projection.durable_count,
        os_app_icon_badge_visible: projection.os_app_icon_badge_visible,
        lock_screen_summary_visible: projection.lock_screen_summary_visible,
        privacy_safe_summary_label: projection.privacy_safe_summary_label,
        inflation_prevented: raw_emissions > 1 && projection.durable_count == 1,
    }
}

fn build_badge_probes() -> Vec<BadgeIntegrityProbe> {
    // A retry storm must collapse to one durable item with the OS badge visible.
    let indexing = envelope(
        "badge:indexing:burst",
        SourceSubsystem::Indexer,
        SeverityClass::Warning,
        PrivacyClass::WorkspaceSensitive,
        PrivacyPayloadClass::LockScreenSafeGeneric,
        RedactionClass::OperatorOnlyRestricted,
        DedupeKeyScheme::CanonicalEventId,
        "ux:event:badge:indexing:burst",
        vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::OsNotification,
        ],
        "Indexer retrying a shard",
        exact_reopen(
            "ux:reopen:badge:indexing:burst",
            ReopenTargetKind::DurableActivityRow,
            "obj:badge:indexing:burst",
        ),
        vec![open_action(
            "ux:action:badge:indexing",
            "Open indexer activity",
            "cmd:indexer.open_shard_activity",
            "obj:badge:indexing:burst",
        )],
        "2026-05-20T11:00:00Z",
    );

    // The same burst under quiet hours: still one durable item, but the OS
    // app-icon badge is suppressed by policy while in-product truth survives.
    let managed = envelope(
        "badge:managed:burst",
        SourceSubsystem::AdminPolicy,
        SeverityClass::Warning,
        PrivacyClass::ManagedSensitive,
        PrivacyPayloadClass::RedactedMetadataOnly,
        RedactionClass::InternalSupportRestricted,
        DedupeKeyScheme::CanonicalEventId,
        "ux:event:badge:managed:burst",
        vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::OsNotification,
        ],
        "Managed workspace alert",
        exact_reopen(
            "ux:reopen:badge:managed:burst",
            ReopenTargetKind::CanonicalObject,
            "obj:badge:managed:burst",
        ),
        vec![open_action(
            "ux:action:badge:managed",
            "Open alert",
            "cmd:managed.open_alert",
            "obj:badge:managed:burst",
        )],
        "2026-05-20T11:05:00Z",
    );

    vec![
        badge_probe(
            "probe:indexing-retry-storm",
            BetaAttentionFamily::Indexing,
            4,
            &indexing,
            &QuietHoursPosture::none(),
        ),
        badge_probe(
            "probe:managed-quiet-hours-badge",
            BetaAttentionFamily::ManagedAlert,
            3,
            &managed,
            &QuietHoursPosture::quiet_hours_user(),
        ),
    ]
}

// ---------------------------------------------------------------------------
// Overlay parity
// ---------------------------------------------------------------------------

fn durable_resolution_fingerprint(outcome: &NotificationRouteOutcome) -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = outcome
        .resolved_surface_routes
        .iter()
        .filter(|route| is_durable_truth_surface(route.fanout_surface_class))
        .map(|route| {
            (
                route.fanout_surface_class.as_str().to_owned(),
                route.channel_resolution_class.as_str().to_owned(),
            )
        })
        .collect();
    rows.sort();
    rows
}

fn audience_surface_visible(outcome: &NotificationRouteOutcome) -> bool {
    outcome.resolved_surface_routes.iter().any(|route| {
        matches!(
            route.fanout_surface_class,
            FanoutSurfaceClass::Toast | FanoutSurfaceClass::ContextualBanner
        ) && route.visible
    })
}

fn overlay_context(base: ChannelContext, overlay: OverlayPosture) -> ChannelContext {
    match overlay {
        OverlayPosture::Presenting => base.with_presentation(PresentationFollowState::Presenting),
        OverlayPosture::FollowingPresenter => {
            base.with_presentation(PresentationFollowState::FollowingPresenter)
        }
        OverlayPosture::FocusMode => base.with_quiet_hours(QuietHoursPosture::focus_mode()),
        OverlayPosture::ScreenReaderActive => base.with_screen_reader(ScreenReaderPosture::Active),
    }
}

fn build_overlay_parity() -> Vec<OverlayParityProof> {
    // One shared envelope routed under a baseline (no overlay) and each overlay.
    let env = envelope(
        "overlay:ai:approval",
        SourceSubsystem::AiApply,
        SeverityClass::Warning,
        PrivacyClass::WorkspaceSensitive,
        PrivacyPayloadClass::InProductOnly,
        RedactionClass::OperatorOnlyRestricted,
        DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
        "dedupe:overlay:ai:approval",
        vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::ActivityCenterDigestCard,
            FanoutSurfaceClass::Toast,
        ],
        "AI change awaiting approval",
        exact_reopen(
            "ux:reopen:overlay:ai:approval",
            ReopenTargetKind::ReviewContext,
            "obj:overlay:ai:approval",
        ),
        vec![open_action(
            "ux:action:overlay:ai",
            "Open approval",
            "cmd:ai.open_review",
            "obj:overlay:ai:approval",
        )],
        "2026-05-20T11:10:00Z",
    );

    let base_ctx = || {
        ChannelContext::foreground_focused()
            .with_window_state(ActiveWindowState::ForegroundUnfocused)
    };

    let baseline = {
        let mut router = AttentionRouter::new();
        router.route(&env, &base_ctx()).expect("baseline routes")
    };
    let baseline_fingerprint = durable_resolution_fingerprint(&baseline);
    let baseline_audience = audience_surface_visible(&baseline);
    let baseline_reopen = baseline.reopen_target.reopen_target_ref.clone();

    OverlayPosture::all()
        .into_iter()
        .map(|overlay| {
            let mut router = AttentionRouter::new();
            let routed = router
                .route(&env, &overlay_context(base_ctx(), overlay))
                .expect("overlay routes");
            OverlayParityProof {
                record_kind: NOTIFICATION_OVERLAY_PARITY_RECORD_KIND.to_owned(),
                schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
                shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
                parity_id: format!("parity:ai-approval:{}", overlay.as_str()),
                attention_family: BetaAttentionFamily::AiApproval,
                overlay_posture: overlay,
                durable_resolution_matches_baseline: durable_resolution_fingerprint(&routed)
                    == baseline_fingerprint,
                reopen_target_matches_baseline: routed.reopen_target.reopen_target_ref
                    == baseline_reopen,
                baseline_audience_visible: baseline_audience,
                overlay_audience_visible: audience_surface_visible(&routed),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

fn build_support_export(
    cases: &[NotificationEnvelopeCorpusCase],
) -> NotificationRouteOutcomeExport {
    let rows = cases
        .iter()
        .map(|case| {
            let outcome = &case.outcome;
            NotificationRouteOutcomeExportRow {
                record_kind: NOTIFICATION_ROUTE_OUTCOME_EXPORT_ROW_RECORD_KIND.to_owned(),
                schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
                shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
                case_id: case.case_id.clone(),
                attention_family: case.attention_family,
                route_outcome_id: outcome.route_outcome_id.clone(),
                canonical_event_id: outcome.canonical_event_id.clone(),
                source_subsystem: outcome.source_subsystem,
                severity_class: outcome.severity_class,
                privacy_class: outcome.privacy_class,
                privacy_payload_class: outcome.privacy_payload_class,
                redaction_class: outcome.redaction_class,
                active_window_state: outcome.channel_context.active_window_state,
                presentation_follow_state: outcome.channel_context.presentation_follow_state,
                companion_handoff_class: outcome.companion_handoff.handoff_class,
                reopen_proof: case.reopen_proof,
                surface_resolutions: outcome
                    .resolved_surface_routes
                    .iter()
                    .map(|route| SupportSurfaceResolution {
                        fanout_surface_class: route.fanout_surface_class,
                        channel_resolution_class: route.channel_resolution_class,
                        visible: route.visible,
                    })
                    .collect(),
                occurrence_count: outcome.occurrence_count,
                is_dedupe_repeat: outcome.is_dedupe_repeat,
                durable_truth_preserved: outcome.durable_truth_preserved,
                all_routes_preserve_reopen_target: outcome.all_routes_preserve_reopen_target,
                no_generic_home_reopen: outcome.no_generic_home_reopen,
            }
        })
        .collect();

    NotificationRouteOutcomeExport {
        record_kind: NOTIFICATION_ROUTE_OUTCOME_EXPORT_RECORD_KIND.to_owned(),
        schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        export_id: NOTIFICATION_ROUTE_OUTCOME_EXPORT_ID.to_owned(),
        generated_at: NOTIFICATION_ENVELOPE_CORPUS_GENERATED_AT.to_owned(),
        rows,
        raw_user_facing_copy_excluded: true,
    }
}

// ---------------------------------------------------------------------------
// Packet assembly
// ---------------------------------------------------------------------------

fn build_cases() -> Vec<NotificationEnvelopeCorpusCase> {
    seed_cases()
        .iter()
        .map(|seed| {
            let outcome = outcome_for(seed);
            let reopen_proof = reopen_proof_for(&seed.envelope.reopen_target);
            let conformant = route_outcome_violations(&outcome).is_empty();
            NotificationEnvelopeCorpusCase {
                record_kind: NOTIFICATION_ENVELOPE_CORPUS_CASE_RECORD_KIND.to_owned(),
                schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
                shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
                case_id: seed.case_id.to_owned(),
                attention_family: seed.family,
                scenario_label: seed.scenario_label.to_owned(),
                emissions: seed.emissions,
                active_window_state: outcome.channel_context.active_window_state,
                screen_reader_posture: outcome.channel_context.screen_reader_posture,
                companion_availability: outcome.channel_context.companion_availability,
                presentation_follow_state: outcome.channel_context.presentation_follow_state,
                reopen_proof,
                conformant,
                outcome,
            }
        })
        .collect()
}

fn summarize(
    cases: &[NotificationEnvelopeCorpusCase],
    drills: &[RouteDriftDrill],
    probes: &[BadgeIntegrityProbe],
    overlays: &[OverlayParityProof],
) -> NotificationEnvelopeCorpusCoverage {
    let mut families: BTreeSet<BetaAttentionFamily> = BTreeSet::new();
    let mut surfaces: BTreeSet<FanoutSurfaceClass> = BTreeSet::new();
    let mut resolution_classes: BTreeSet<ChannelResolutionClass> = BTreeSet::new();
    let mut handoff_classes: BTreeSet<CompanionHandoffClass> = BTreeSet::new();
    let mut reopen_proofs: BTreeSet<ReopenProofClass> = BTreeSet::new();
    let mut overlay_postures: BTreeSet<OverlayPosture> = BTreeSet::new();

    let mut all_conform = true;
    let mut all_reopen = true;
    let mut all_durable = true;
    let mut all_truthful = true;

    for case in cases {
        families.insert(case.attention_family);
        reopen_proofs.insert(case.reopen_proof);
        handoff_classes.insert(case.outcome.companion_handoff.handoff_class);
        all_conform &= case.conformant;
        all_reopen &= case.outcome.all_routes_preserve_reopen_target;
        all_durable &= case.outcome.durable_truth_preserved;
        all_truthful &= case.outcome.no_generic_home_reopen;
        for route in &case.outcome.resolved_surface_routes {
            surfaces.insert(route.fanout_surface_class);
            resolution_classes.insert(route.channel_resolution_class);
        }
    }

    let mut drift_violations: BTreeSet<RouteDriftViolation> = BTreeSet::new();
    let mut all_drills_reject = true;
    for drill in drills {
        drift_violations.insert(drill.violation_class);
        all_drills_reject &= drill.conforming_passes_clean && drill.lane_rejects_regression;
    }

    let badge_never_inflates = probes.iter().all(|p| p.inflation_prevented);

    let mut all_overlays_ok = true;
    for overlay in overlays {
        overlay_postures.insert(overlay.overlay_posture);
        all_overlays_ok &=
            overlay.durable_resolution_matches_baseline && overlay.reopen_target_matches_baseline;
    }

    let all_families_present = BetaAttentionFamily::all()
        .iter()
        .all(|f| families.contains(f));

    NotificationEnvelopeCorpusCoverage {
        case_count: cases.len() as u32,
        families_covered: families.into_iter().collect(),
        surfaces_covered: surfaces.into_iter().collect(),
        resolution_classes_covered: resolution_classes.into_iter().collect(),
        companion_handoff_classes_covered: handoff_classes.into_iter().collect(),
        reopen_proof_classes_covered: reopen_proofs.into_iter().collect(),
        drift_violations_covered: drift_violations.into_iter().collect(),
        overlay_postures_covered: overlay_postures.into_iter().collect(),
        all_families_present,
        all_cases_conform: all_conform,
        all_cases_preserve_reopen_target: all_reopen,
        all_cases_preserve_durable_truth: all_durable,
        all_cases_truthful_reopen: all_truthful,
        all_drills_reject_regression: all_drills_reject,
        all_overlays_preserve_durable_semantics: all_overlays_ok,
        badge_never_inflates,
    }
}

/// The full, mint-from-truth notification-envelope-corpus packet.
pub fn seeded_notification_envelope_corpus_packet() -> NotificationEnvelopeCorpusPacket {
    let cases = build_cases();
    let badge_probes = build_badge_probes();
    let drift_drills = build_drift_drills(&cases, &badge_probes);
    let overlay_parity = build_overlay_parity();
    let support_export = build_support_export(&cases);
    let coverage = summarize(&cases, &drift_drills, &badge_probes, &overlay_parity);

    NotificationEnvelopeCorpusPacket {
        record_kind: NOTIFICATION_ENVELOPE_CORPUS_PACKET_RECORD_KIND.to_owned(),
        schema_version: NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: NOTIFICATION_ENVELOPE_CORPUS_PACKET_ID.to_owned(),
        generated_at: NOTIFICATION_ENVELOPE_CORPUS_GENERATED_AT.to_owned(),
        coverage,
        cases,
        drift_drills,
        badge_probes,
        overlay_parity,
        support_export,
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validate the packet invariants. Returns the list of violations; an empty
/// list means the packet conforms.
pub fn validate_notification_envelope_corpus_packet(
    packet: &NotificationEnvelopeCorpusPacket,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if packet.record_kind != NOTIFICATION_ENVELOPE_CORPUS_PACKET_RECORD_KIND {
        errors.push(format!("packet record_kind is {}", packet.record_kind));
    }
    if packet.schema_version != NOTIFICATION_ENVELOPE_CORPUS_SCHEMA_VERSION {
        errors.push(format!(
            "packet schema_version is {}",
            packet.schema_version
        ));
    }
    if packet.shared_contract_ref != NOTIFICATION_ENVELOPE_CORPUS_SHARED_CONTRACT_REF {
        errors.push(format!(
            "packet shared_contract_ref is {}",
            packet.shared_contract_ref
        ));
    }
    if packet.cases.is_empty() {
        errors.push("packet has no cases".to_owned());
    }

    // Every family the corpus claims must have a case.
    if !packet.coverage.all_families_present {
        errors.push("packet does not cover every beta attention family".to_owned());
    }
    for family in BetaAttentionFamily::all() {
        if !packet.coverage.families_covered.contains(&family) {
            errors.push(format!("missing attention family {}", family.as_str()));
        }
    }

    // Every case must conform and re-validate against the conformance lane.
    let mut seen_case_ids: BTreeSet<&str> = BTreeSet::new();
    for case in &packet.cases {
        if !seen_case_ids.insert(case.case_id.as_str()) {
            errors.push(format!("duplicate case_id {}", case.case_id));
        }
        let violations = route_outcome_violations(&case.outcome);
        if !violations.is_empty() {
            errors.push(format!(
                "case {} violates the conformance lane: {}",
                case.case_id,
                violations.join(", ")
            ));
        }
        if !case.conformant {
            errors.push(format!("case {} is flagged non-conformant", case.case_id));
        }
        if case.reopen_proof != reopen_proof_for(&case.outcome.reopen_target) {
            errors.push(format!(
                "case {} reopen_proof disagrees with its reopen target",
                case.case_id
            ));
        }
    }

    // Required resolution-class coverage so the corpus proves cross-surface
    // consistency under every live posture.
    for required in [
        ChannelResolutionClass::DeliveredInApp,
        ChannelResolutionClass::DeliveredExternalSummary,
        ChannelResolutionClass::SuppressedForegroundRedundant,
        ChannelResolutionClass::LockScreenNotApplicable,
        ChannelResolutionClass::CompanionUnavailable,
        ChannelResolutionClass::CompanionPolicyBlocked,
        ChannelResolutionClass::HeldByQuietHoursOrFocus,
        ChannelResolutionClass::SuppressedByPolicy,
        ChannelResolutionClass::DedupedRepeat,
    ] {
        if !packet
            .coverage
            .resolution_classes_covered
            .contains(&required)
        {
            errors.push(format!(
                "packet does not cover resolution class {}",
                required.as_str()
            ));
        }
    }

    // Reopen-proof coverage: exact, placeholder, and revalidation.
    for required in [
        ReopenProofClass::ExactTarget,
        ReopenProofClass::TruthfulPlaceholder,
        ReopenProofClass::DeniedRequiresRevalidation,
    ] {
        if !packet
            .coverage
            .reopen_proof_classes_covered
            .contains(&required)
        {
            errors.push(format!(
                "packet does not cover reopen proof {}",
                required.as_str()
            ));
        }
    }

    // Every drift drill must pass clean as conforming and be rejected as
    // regressed. A drill that does not reject its regression is a hole.
    let mut seen_violations: BTreeSet<RouteDriftViolation> = BTreeSet::new();
    for drill in &packet.drift_drills {
        seen_violations.insert(drill.violation_class);
        if !drill.conforming_passes_clean {
            errors.push(format!(
                "drill {} baseline does not pass clean",
                drill.drill_id
            ));
        }
        if !drill.lane_rejects_regression {
            errors.push(format!(
                "drill {} regression was not rejected by the lane",
                drill.drill_id
            ));
        }
        if drill.rejection_reason_tokens.is_empty() {
            errors.push(format!(
                "drill {} produced no rejection reason tokens",
                drill.drill_id
            ));
        }
    }
    for required in RouteDriftViolation::all() {
        if !seen_violations.contains(&required) {
            errors.push(format!("missing drift drill {}", required.as_str()));
        }
    }

    // Badge probes must never inflate.
    if packet.badge_probes.is_empty() {
        errors.push("packet has no badge-integrity probes".to_owned());
    }
    for probe in &packet.badge_probes {
        if !probe.inflation_prevented {
            errors.push(format!(
                "badge probe {} inflated: raw={} durable={}",
                probe.probe_id, probe.raw_emissions, probe.durable_badge_count
            ));
        }
    }

    // Overlay parity must hold for every overlay posture.
    for required in OverlayPosture::all() {
        if !packet.coverage.overlay_postures_covered.contains(&required) {
            errors.push(format!("missing overlay posture {}", required.as_str()));
        }
    }
    for overlay in &packet.overlay_parity {
        if !overlay.durable_resolution_matches_baseline {
            errors.push(format!(
                "overlay {} diverged durable resolution from baseline",
                overlay.parity_id
            ));
        }
        if !overlay.reopen_target_matches_baseline {
            errors.push(format!(
                "overlay {} diverged reopen target from baseline",
                overlay.parity_id
            ));
        }
    }

    // The summary must agree with the records it claims to summarize.
    let recomputed = summarize(
        &packet.cases,
        &packet.drift_drills,
        &packet.badge_probes,
        &packet.overlay_parity,
    );
    if recomputed != packet.coverage {
        errors.push("packet coverage summary does not match its records".to_owned());
    }

    // Support export must be row-aligned and carry no user-facing copy.
    if packet.support_export.rows.len() != packet.cases.len() {
        errors.push("support export row count does not match cases".to_owned());
    }
    if !packet.support_export.raw_user_facing_copy_excluded {
        errors.push("support export does not exclude raw user-facing copy".to_owned());
    }
    if let Ok(export_json) = serde_json::to_string(&packet.support_export) {
        for case in &packet.cases {
            if export_json.contains(&case.outcome.summary_label) {
                errors.push(format!(
                    "support export leaked summary copy for {}",
                    case.case_id
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate and return a one-line summary string for the headless inspector.
pub fn validate_notification_envelope_corpus_packet_summary(
    packet: &NotificationEnvelopeCorpusPacket,
) -> Result<String, Vec<String>> {
    validate_notification_envelope_corpus_packet(packet)?;
    Ok(format!(
        "ok: {} cases across {} families, {} drift drills, {} badge probes, {} overlay parity proofs",
        packet.cases.len(),
        packet.coverage.families_covered.len(),
        packet.drift_drills.len(),
        packet.badge_probes.len(),
        packet.overlay_parity.len(),
    ))
}

// ---------------------------------------------------------------------------
// Markdown rendering (mint-from-truth artifacts)
// ---------------------------------------------------------------------------

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn join_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> String {
    let collected: Vec<String> = tokens.map(|t| format!("`{t}`")).collect();
    if collected.is_empty() {
        "(none)".to_owned()
    } else {
        collected.join(", ")
    }
}

fn surface_resolution_cell(
    outcome: &NotificationRouteOutcome,
    surface: FanoutSurfaceClass,
) -> String {
    match outcome
        .resolved_surface_routes
        .iter()
        .find(|route| route.fanout_surface_class == surface)
    {
        None => "—".to_owned(),
        Some(route) => route.channel_resolution_class.as_str().to_owned(),
    }
}

/// Renders the privacy and route audit
/// (`artifacts/ux/m3/notification_privacy_and_route_audit.md`).
pub fn render_notification_privacy_and_route_audit_markdown(
    packet: &NotificationEnvelopeCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Notification privacy and route audit\n\n");
    out.push_str(
        "Generated from the seeded corpus in `crates/aureline-shell/src/notification_envelope_corpus/mod.rs`. ",
    );
    out.push_str(
        "Every route outcome is minted from truth by the governed attention router, never copied from a screenshot or toast text.\n\n",
    );
    out.push_str("Regenerate with:\n\n```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- audit-md > \\\n  artifacts/ux/m3/notification_privacy_and_route_audit.md\n",
    );
    out.push_str("```\n\n");
    out.push_str(&format!("- Packet id: `{}`\n", packet.packet_id));
    out.push_str(&format!(
        "- Shared contract ref: `{}`\n",
        packet.shared_contract_ref
    ));
    out.push_str("- Route-outcome schema: `schemas/ux/notification_route_outcome.schema.json`\n");
    out.push_str(&format!("- Generated at: `{}`\n", packet.generated_at));
    out.push_str(&format!(
        "- Cases: {} across {} beta attention families\n",
        packet.cases.len(),
        packet.coverage.families_covered.len()
    ));
    out.push_str(&format!(
        "- All families present: {} · all cases conform: {}\n\n",
        yes_no(packet.coverage.all_families_present),
        yes_no(packet.coverage.all_cases_conform)
    ));

    out.push_str("## Surface resolution by case\n\n");
    out.push_str(
        "Cells carry the `channel_resolution_class` for each surface; `—` means the surface was not recommended.\n\n",
    );
    out.push_str(
        "| Case | Family | Window | durable_job_row | status_item | activity_center_digest_card | contextual_banner | toast | os_notification | lock_screen_summary | companion_push |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for case in &packet.cases {
        out.push_str(&format!(
            "| `{case}` | `{family}` | `{window}` | {djr} | {si} | {acdc} | {cb} | {toast} | {os} | {ls} | {cp} |\n",
            case = case.case_id,
            family = case.attention_family.as_str(),
            window = case.active_window_state.as_str(),
            djr = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::DurableJobRow),
            si = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::StatusItem),
            acdc =
                surface_resolution_cell(&case.outcome, FanoutSurfaceClass::ActivityCenterDigestCard),
            cb = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::ContextualBanner),
            toast = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::Toast),
            os = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::OsNotification),
            ls = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::LockScreenSummary),
            cp = surface_resolution_cell(&case.outcome, FanoutSurfaceClass::CompanionPush),
        ));
    }
    out.push('\n');

    out.push_str("## Reopen, durable truth, and handoff proofs\n\n");
    out.push_str(
        "| Case | Reopen proof | Companion handoff | durable_truth_preserved | reopen_target_preserved | no_generic_home_reopen | emissions |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for case in &packet.cases {
        out.push_str(&format!(
            "| `{case}` | `{reopen}` | `{handoff}` | {durable} | {pres} | {nogen} | {emit} |\n",
            case = case.case_id,
            reopen = case.reopen_proof.as_str(),
            handoff = case.outcome.companion_handoff.handoff_class.as_str(),
            durable = yes_no(case.outcome.durable_truth_preserved),
            pres = yes_no(case.outcome.all_routes_preserve_reopen_target),
            nogen = yes_no(case.outcome.no_generic_home_reopen),
            emit = case.emissions,
        ));
    }
    out.push('\n');

    out.push_str("## Drift drills (regression gate)\n\n");
    out.push_str(
        "Each drill takes a conforming outcome, constructs the named regression, and shows the conformance lane rejects it. The diff is the actionable artifact.\n\n",
    );
    out.push_str(
        "| Drill | Violation | Diff field | Conforming | Regressed | Lane rejects | Reason tokens |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for drill in &packet.drift_drills {
        out.push_str(&format!(
            "| `{id}` | `{viol}` | `{field}` | `{conf}` | `{reg}` | {rej} | {tokens} |\n",
            id = drill.drill_id,
            viol = drill.violation_class.as_str(),
            field = drill.diff.diff_field,
            conf = drill.diff.conforming_observation,
            reg = drill.diff.regressed_observation,
            rej = yes_no(drill.lane_rejects_regression),
            tokens = join_tokens(drill.rejection_reason_tokens.iter().map(String::as_str)),
        ));
    }
    out.push('\n');

    out.push_str("## Badge integrity\n\n");
    out.push_str(
        "| Probe | Family | Raw emissions | Durable count | OS badge | Lock-screen | Inflation prevented |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for probe in &packet.badge_probes {
        out.push_str(&format!(
            "| `{id}` | `{family}` | {raw} | {durable} | {os} | {ls} | {prevented} |\n",
            id = probe.probe_id,
            family = probe.attention_family.as_str(),
            raw = probe.raw_emissions,
            durable = probe.durable_badge_count,
            os = yes_no(probe.os_app_icon_badge_visible),
            ls = yes_no(probe.lock_screen_summary_visible),
            prevented = yes_no(probe.inflation_prevented),
        ));
    }
    out.push('\n');

    out.push_str("## Overlay parity\n\n");
    out.push_str(
        "Presentation, follow, focus, and screen-reader postures dim audience-visible surfaces but never change the claimed durable rows or the reopen target.\n\n",
    );
    out.push_str(
        "| Overlay | Durable rows match baseline | Reopen matches baseline | Audience visible (baseline → overlay) |\n",
    );
    out.push_str("| --- | --- | --- | --- |\n");
    for overlay in &packet.overlay_parity {
        out.push_str(&format!(
            "| `{posture}` | {durable} | {reopen} | {base} → {ov} |\n",
            posture = overlay.overlay_posture.as_str(),
            durable = yes_no(overlay.durable_resolution_matches_baseline),
            reopen = yes_no(overlay.reopen_target_matches_baseline),
            base = yes_no(overlay.baseline_audience_visible),
            ov = yes_no(overlay.overlay_audience_visible),
        ));
    }
    out.push('\n');

    out.push_str("## Results\n\n");
    out.push_str("| Rule | Result |\n| --- | --- |\n");
    out.push_str(&format!(
        "| Every beta attention family has a worked routing case | {} |\n",
        pass_fail(packet.coverage.all_families_present)
    ));
    out.push_str(&format!(
        "| Every resolved surface preserves the single reopen target | {} |\n",
        pass_fail(packet.coverage.all_cases_preserve_reopen_target)
    ));
    out.push_str(&format!(
        "| No outcome opens a generic home view | {} |\n",
        pass_fail(packet.coverage.all_cases_truthful_reopen)
    ));
    out.push_str(&format!(
        "| Durable truth preserved under hold, suppression, and dedupe | {} |\n",
        pass_fail(packet.coverage.all_cases_preserve_durable_truth)
    ));
    out.push_str(&format!(
        "| Wrong-target, lock-screen leak, badge inflation, quiet-hours drift all fail the lane | {} |\n",
        pass_fail(packet.coverage.all_drills_reject_regression)
    ));
    out.push_str(&format!(
        "| Badge never inflates under a retry storm | {} |\n",
        pass_fail(packet.coverage.badge_never_inflates)
    ));
    out.push_str(&format!(
        "| Presentation/follow/focus/screen-reader keep durable semantics | {} |\n",
        pass_fail(packet.coverage.all_overlays_preserve_durable_semantics)
    ));

    out
}

/// Renders the support route/outcome export report
/// (`artifacts/support/m3/notification_route_outcome_export_report.md`).
pub fn render_notification_route_outcome_export_report_markdown(
    packet: &NotificationEnvelopeCorpusPacket,
) -> String {
    let export = &packet.support_export;
    let mut out = String::new();
    out.push_str("# Notification route/outcome export report\n\n");
    out.push_str(
        "This report proves a support packet can reconstruct notification class, surface route, suppression, and final resolution from structured fields alone — never by scraping badge text or toast copy.\n\n",
    );
    out.push_str(
        "Minted from `crates/aureline-shell/src/notification_envelope_corpus/mod.rs` and replayed by `crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`. The export schema of record is `schemas/ux/notification_route_outcome.schema.json`.\n\n",
    );
    out.push_str("Regenerate with:\n\n```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- export-report-md > \\\n  artifacts/support/m3/notification_route_outcome_export_report.md\n",
    );
    out.push_str("```\n\n");
    out.push_str(&format!("- Export id: `{}`\n", export.export_id));
    out.push_str(&format!(
        "- Shared contract ref: `{}`\n",
        export.shared_contract_ref
    ));
    out.push_str(&format!("- Generated at: `{}`\n", export.generated_at));
    out.push_str(&format!("- Rows: {}\n", export.rows.len()));
    out.push_str(&format!(
        "- Raw user-facing copy excluded: {}\n\n",
        yes_no(export.raw_user_facing_copy_excluded)
    ));

    out.push_str("## Reconstructed route/outcome rows\n\n");
    out.push_str(
        "| Case | Family | Source | Severity | Privacy | Payload | Window | Presentation | Handoff | Reopen | Occurrences | Dedupe repeat | Durable |\n",
    );
    out.push_str(
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for row in &export.rows {
        out.push_str(&format!(
            "| `{case}` | `{family}` | `{source}` | `{sev}` | `{priv_}` | `{payload}` | `{window}` | `{pres}` | `{handoff}` | `{reopen}` | {occ} | {dedupe} | {durable} |\n",
            case = row.case_id,
            family = row.attention_family.as_str(),
            source = subsystem_token(row.source_subsystem),
            sev = severity_token(row.severity_class),
            priv_ = privacy_token(row.privacy_class),
            payload = payload_token(row.privacy_payload_class),
            window = row.active_window_state.as_str(),
            pres = row.presentation_follow_state.as_str(),
            handoff = row.companion_handoff_class.as_str(),
            reopen = row.reopen_proof.as_str(),
            occ = row.occurrence_count,
            dedupe = yes_no(row.is_dedupe_repeat),
            durable = yes_no(row.durable_truth_preserved),
        ));
    }
    out.push('\n');

    out.push_str("## Per-surface resolution (structured, no copy)\n\n");
    for row in &export.rows {
        out.push_str(&format!("### `{}`\n\n", row.case_id));
        out.push_str("| Surface | Resolution | Visible |\n| --- | --- | --- |\n");
        for res in &row.surface_resolutions {
            out.push_str(&format!(
                "| `{surface}` | `{class}` | {visible} |\n",
                surface = res.fanout_surface_class.as_str(),
                class = res.channel_resolution_class.as_str(),
                visible = yes_no(res.visible),
            ));
        }
        out.push('\n');
    }

    out.push_str("## Results\n\n");
    out.push_str("| Rule | Result |\n| --- | --- |\n");
    out.push_str(&format!(
        "| Every case reconstructs from structured fields alone | {} |\n",
        pass_fail(export.rows.len() == packet.cases.len())
    ));
    out.push_str(&format!(
        "| No raw user-facing copy in the export | {} |\n",
        pass_fail(export.raw_user_facing_copy_excluded)
    ));
    out.push_str(&format!(
        "| Route and outcome reconstructable per surface | {} |\n",
        pass_fail(
            export
                .rows
                .iter()
                .all(|r| !r.surface_resolutions.is_empty())
        )
    ));

    out
}

/// Renders the conformance doc
/// (`docs/ux/m3/notification_route_conformance.md`).
pub fn render_notification_route_conformance_markdown(
    packet: &NotificationEnvelopeCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Notification route conformance (beta)\n\n");
    out.push_str(
        "This corpus turns the governed attention router into a regression-gated proof system for durable notification routing. Every beta job or alert family that claims durable attention truth has a worked routing case that proves privacy-safe routing, exact-target reopen, quiet-hours/admin suppression, and export-safe route/outcome truth — instead of inferred behavior.\n\n",
    );
    out.push_str(
        "It is minted from `crates/aureline-shell/src/notification_envelope_corpus/mod.rs`, replayed by `crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`, and composes the route-outcome contract at `docs/ux/m3/notification_envelope_beta_contract.md`. The route-outcome schema of record is `schemas/ux/notification_route_outcome.schema.json`.\n\n",
    );

    out.push_str("## What every claimed family must prove\n\n");
    out.push_str("1. **One alert, every surface.** The same envelope resolves consistently across the durable row, status item, activity center, banner, toast, OS notification, lock-screen summary, and companion push.\n");
    out.push_str("2. **Live-channel narrowing, never widening.** Look-away drops the redundant OS toast, an unlocked device drops the lock-screen summary, an unreachable or policy-blocked companion drops the push — and no held/suppressed/deduped surface is ever upgraded back to delivered.\n");
    out.push_str("3. **Exact-target reopen.** Every surface keeps the single reopen target; a stale or missing target reopens a truthful placeholder or a revalidation requirement, never a generic home view.\n");
    out.push_str("4. **Durable truth survives the hold.** Quiet hours, admin suppression, presentation, and dedupe delay interruption while durable truth, badge integrity, and reopen semantics survive.\n");
    out.push_str("5. **Export-safe truth.** A support packet reconstructs class, route, suppression, and resolution from stable enums — never from badge text or toast copy.\n\n");

    out.push_str("## Family coverage\n\n");
    out.push_str("| Family | Cases | Surfaces resolved |\n| --- | --- | --- |\n");
    for family in BetaAttentionFamily::all() {
        let family_cases: Vec<&NotificationEnvelopeCorpusCase> = packet
            .cases
            .iter()
            .filter(|c| c.attention_family == family)
            .collect();
        let mut surfaces: BTreeSet<&str> = BTreeSet::new();
        for case in &family_cases {
            for route in &case.outcome.resolved_surface_routes {
                surfaces.insert(route.fanout_surface_class.as_str());
            }
        }
        out.push_str(&format!(
            "| `{family}` | {count} | {surfaces} |\n",
            family = family.as_str(),
            count = family_cases.len(),
            surfaces = join_tokens(surfaces.into_iter()),
        ));
    }
    out.push('\n');

    out.push_str("## Channel-resolution coverage\n\n");
    for class in &packet.coverage.resolution_classes_covered {
        out.push_str(&format!("- `{}`\n", class.as_str()));
    }
    out.push('\n');

    out.push_str("## Drift drills\n\n");
    out.push_str(
        "Each drill ships an adversarial regression that the conformance lane must reject. The case records the exact reason tokens the lane produces, so a regression that lets the behavior through fails the fixture replay.\n\n",
    );
    out.push_str("| Drill | Violation | Reason tokens |\n| --- | --- | --- |\n");
    for drill in &packet.drift_drills {
        out.push_str(&format!(
            "| `{id}` | `{viol}` | {tokens} |\n",
            id = drill.drill_id,
            viol = drill.violation_class.as_str(),
            tokens = join_tokens(drill.rejection_reason_tokens.iter().map(String::as_str)),
        ));
    }
    out.push('\n');

    out.push_str("## Conformance results\n\n");
    out.push_str("| Rule | Result |\n| --- | --- |\n");
    out.push_str(&format!(
        "| Every beta attention family covered | {} |\n",
        pass_fail(packet.coverage.all_families_present)
    ));
    out.push_str(&format!(
        "| Every case conforms to the route lane | {} |\n",
        pass_fail(packet.coverage.all_cases_conform)
    ));
    out.push_str(&format!(
        "| Reopen target preserved, no generic home | {} |\n",
        pass_fail(
            packet.coverage.all_cases_preserve_reopen_target
                && packet.coverage.all_cases_truthful_reopen
        )
    ));
    out.push_str(&format!(
        "| Drift drills all fail the lane | {} |\n",
        pass_fail(packet.coverage.all_drills_reject_regression)
    ));
    out.push_str(&format!(
        "| Badge never inflates | {} |\n",
        pass_fail(packet.coverage.badge_never_inflates)
    ));
    out.push_str(&format!(
        "| Overlays preserve durable semantics | {} |\n",
        pass_fail(packet.coverage.all_overlays_preserve_durable_semantics)
    ));
    out.push_str(&format!(
        "| Support export reconstructs from enums only | {} |\n",
        pass_fail(packet.support_export.raw_user_facing_copy_excluded)
    ));

    out
}

fn pass_fail(value: bool) -> &'static str {
    if value {
        "PASS"
    } else {
        "FAIL"
    }
}

// Token mappers mirror the schema's snake_case enum vocabulary for the export
// report so the rendered tokens match the serialized export exactly.
fn subsystem_token(source: SourceSubsystem) -> &'static str {
    match source {
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

fn severity_token(severity: SeverityClass) -> &'static str {
    match severity {
        SeverityClass::Info => "info",
        SeverityClass::Success => "success",
        SeverityClass::Warning => "warning",
        SeverityClass::Degraded => "degraded",
        SeverityClass::Error => "error",
        SeverityClass::Blocking => "blocking",
        SeverityClass::Critical => "critical",
    }
}

fn privacy_token(privacy: PrivacyClass) -> &'static str {
    match privacy {
        PrivacyClass::SummarySafe => "summary_safe",
        PrivacyClass::WorkspaceSensitive => "workspace_sensitive",
        PrivacyClass::SecurityCritical => "security_critical",
        PrivacyClass::ManagedSensitive => "managed_sensitive",
    }
}

fn payload_token(payload: PrivacyPayloadClass) -> &'static str {
    match payload {
        PrivacyPayloadClass::LockScreenSafeGeneric => "lock_screen_safe_generic",
        PrivacyPayloadClass::LockScreenSafeScoped => "lock_screen_safe_scoped",
        PrivacyPayloadClass::InProductOnly => "in_product_only",
        PrivacyPayloadClass::RedactedMetadataOnly => "redacted_metadata_only",
        PrivacyPayloadClass::PolicyForbiddenOnLockScreen => "policy_forbidden_on_lock_screen",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_notification_envelope_corpus_packet();
        validate_notification_envelope_corpus_packet(&packet).expect("seeded packet must validate");
    }

    #[test]
    fn packet_round_trips_through_json() {
        let packet = seeded_notification_envelope_corpus_packet();
        let json = serde_json::to_string(&packet).unwrap();
        let parsed: NotificationEnvelopeCorpusPacket = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, packet);
    }

    #[test]
    fn every_family_has_a_case() {
        let packet = seeded_notification_envelope_corpus_packet();
        for family in BetaAttentionFamily::all() {
            assert!(
                packet.cases.iter().any(|c| c.attention_family == family),
                "missing family {}",
                family.as_str()
            );
        }
    }

    #[test]
    fn drift_drills_reject_their_regressions() {
        let packet = seeded_notification_envelope_corpus_packet();
        assert_eq!(packet.drift_drills.len(), 4);
        for drill in &packet.drift_drills {
            assert!(
                drill.conforming_passes_clean,
                "{} baseline did not pass clean",
                drill.drill_id
            );
            assert!(
                drill.lane_rejects_regression,
                "{} regression slipped through the lane",
                drill.drill_id
            );
            assert!(!drill.rejection_reason_tokens.is_empty());
        }
    }

    #[test]
    fn badge_probes_never_inflate() {
        let packet = seeded_notification_envelope_corpus_packet();
        for probe in &packet.badge_probes {
            assert!(probe.inflation_prevented, "{} inflated", probe.probe_id);
            assert_eq!(probe.durable_badge_count, 1);
        }
    }

    #[test]
    fn overlays_preserve_durable_semantics_and_reopen() {
        let packet = seeded_notification_envelope_corpus_packet();
        for overlay in &packet.overlay_parity {
            assert!(
                overlay.durable_resolution_matches_baseline,
                "{}",
                overlay.parity_id
            );
            assert!(
                overlay.reopen_target_matches_baseline,
                "{}",
                overlay.parity_id
            );
        }
        // Presentation and follow overlays must dim audience surfaces.
        for overlay in &packet.overlay_parity {
            if matches!(
                overlay.overlay_posture,
                OverlayPosture::Presenting | OverlayPosture::FollowingPresenter
            ) {
                assert!(overlay.baseline_audience_visible);
                assert!(!overlay.overlay_audience_visible);
            }
        }
    }

    #[test]
    fn support_export_excludes_summary_copy() {
        let packet = seeded_notification_envelope_corpus_packet();
        let json = serde_json::to_string(&packet.support_export).unwrap();
        for case in &packet.cases {
            assert!(
                !json.contains(&case.outcome.summary_label),
                "export leaked summary for {}",
                case.case_id
            );
        }
    }
}
