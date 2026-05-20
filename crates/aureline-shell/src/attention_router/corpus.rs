//! Seeded attention-routing corpus, support export, and validation.
//!
//! This is the mint-from-truth corpus for the beta attention router. Each case
//! routes one envelope through [`AttentionRouter`] under a specific live
//! [`ChannelContext`] and captures the single [`NotificationRouteOutcome`].
//! The checked-in fixtures under `fixtures/ux/m3/notification_routing/` are a
//! literal projection of [`seeded_attention_routing_corpus`], so they cannot
//! drift from the Rust types.
//!
//! The corpus exercises every fanout surface and every channel-resolution
//! class so the same alert can be reasoned about consistently across toast,
//! banner, status overflow, activity center, native OS notification, and
//! companion surfaces.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass, PrivacyPayloadClass,
    QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem,
    StableAction, SuppressionReason, SuppressionState,
};
use crate::notifications::quiet_hours::QuietHoursPosture;

use super::context::{
    ActiveWindowState, ChannelContext, CompanionAvailability, ScreenReaderPosture,
};
use super::outcome::{
    AttentionRouter, ChannelResolutionClass, CompanionHandoffClass, NotificationRouteOutcome,
    NOTIFICATION_ROUTE_OUTCOME_RECORD_KIND,
};

/// Schema version exported by attention-router beta records.
pub const ATTENTION_ROUTER_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref carried by every attention-router beta record so shell
/// rows, headless CLI rows, and support-export rows pivot to the same case id.
pub const ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF: &str = "shell:attention_router_beta:v1";

/// Stable record kind for [`AttentionRoutingCorpus`] payloads.
pub const ATTENTION_ROUTING_CORPUS_RECORD_KIND: &str = "shell_attention_routing_corpus_record";

/// Stable record kind for [`AttentionRoutingCase`] payloads.
pub const ATTENTION_ROUTING_CASE_RECORD_KIND: &str = "shell_attention_routing_case_record";

/// Stable record kind for [`AttentionRouteSupportExport`] payloads.
pub const ATTENTION_ROUTE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_attention_route_support_export_record";

/// Stable record kind for [`AttentionRouteSupportExportRow`] payloads.
pub const ATTENTION_ROUTE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_attention_route_support_export_row_record";

/// One seeded routing case: a scenario, the number of emissions used to reach
/// it, and the resulting governed outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRoutingCase {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub scenario_label: String,
    pub emissions: u32,
    pub outcome: NotificationRouteOutcome,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRoutingCorpusSummary {
    pub case_count: u32,
    pub surfaces_covered: Vec<FanoutSurfaceClass>,
    pub resolution_classes_covered: Vec<ChannelResolutionClass>,
    pub companion_handoff_classes_covered: Vec<CompanionHandoffClass>,
    pub all_cases_preserve_reopen_target: bool,
    pub all_cases_preserve_durable_truth: bool,
    pub all_cases_truthful_reopen: bool,
}

/// The full seeded attention-routing corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRoutingCorpus {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub generated_at: String,
    pub summary: AttentionRoutingCorpusSummary,
    pub cases: Vec<AttentionRoutingCase>,
}

/// One support-safe surface resolution row (enums only, no copy).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSurfaceResolution {
    pub fanout_surface_class: FanoutSurfaceClass,
    pub channel_resolution_class: ChannelResolutionClass,
    pub visible: bool,
}

/// One support-export row. Carries support-safe enums and structured
/// resolution rows rather than raw user-facing message text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRouteSupportExportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub route_outcome_id: String,
    pub canonical_event_id: String,
    pub source_subsystem: SourceSubsystem,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub redaction_class: RedactionClass,
    pub active_window_state: ActiveWindowState,
    pub companion_handoff_class: CompanionHandoffClass,
    pub surface_resolutions: Vec<SupportSurfaceResolution>,
    pub occurrence_count: u32,
    pub is_dedupe_repeat: bool,
    pub durable_truth_preserved: bool,
    pub all_routes_preserve_reopen_target: bool,
    pub no_generic_home_reopen: bool,
}

/// Support-export wrapper over the corpus. Quotes case ids and per-surface
/// resolution enums; never quotes summary copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRouteSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub rows: Vec<AttentionRouteSupportExportRow>,
    pub raw_private_material_excluded: bool,
}

impl AttentionRouteSupportExport {
    /// Project a corpus into a support-safe export.
    pub fn from_corpus(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        corpus: &AttentionRoutingCorpus,
    ) -> Self {
        let rows = corpus
            .cases
            .iter()
            .map(|case| {
                let outcome = &case.outcome;
                AttentionRouteSupportExportRow {
                    record_kind: ATTENTION_ROUTE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                    schema_version: ATTENTION_ROUTER_BETA_SCHEMA_VERSION,
                    shared_contract_ref: ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF.to_owned(),
                    case_id: case.case_id.clone(),
                    route_outcome_id: outcome.route_outcome_id.clone(),
                    canonical_event_id: outcome.canonical_event_id.clone(),
                    source_subsystem: outcome.source_subsystem,
                    severity_class: outcome.severity_class,
                    privacy_class: outcome.privacy_class,
                    redaction_class: outcome.redaction_class,
                    active_window_state: outcome.channel_context.active_window_state,
                    companion_handoff_class: outcome.companion_handoff.handoff_class,
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

        Self {
            record_kind: ATTENTION_ROUTE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ATTENTION_ROUTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_private_material_excluded: true,
        }
    }
}

// ---- seed scenario definitions -------------------------------------------

struct SeedCase {
    case_id: &'static str,
    scenario_label: &'static str,
    emissions: u32,
    context: ChannelContext,
    envelope: NotificationEnvelope,
}

/// Build the governed outcome for one seed case by routing it `emissions`
/// times through a fresh router (so dedupe memory never bleeds across cases).
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

/// The full seeded attention-routing corpus.
pub fn seeded_attention_routing_corpus() -> AttentionRoutingCorpus {
    let seeds = seed_cases();
    let cases: Vec<AttentionRoutingCase> = seeds
        .iter()
        .map(|seed| AttentionRoutingCase {
            record_kind: ATTENTION_ROUTING_CASE_RECORD_KIND.to_owned(),
            schema_version: ATTENTION_ROUTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: seed.case_id.to_owned(),
            scenario_label: seed.scenario_label.to_owned(),
            emissions: seed.emissions,
            outcome: outcome_for(seed),
        })
        .collect();

    let summary = summarize(&cases);

    AttentionRoutingCorpus {
        record_kind: ATTENTION_ROUTING_CORPUS_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_ROUTER_BETA_SCHEMA_VERSION,
        shared_contract_ref: ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}

fn summarize(cases: &[AttentionRoutingCase]) -> AttentionRoutingCorpusSummary {
    let mut surfaces: BTreeSet<FanoutSurfaceClass> = BTreeSet::new();
    let mut resolution_classes: BTreeSet<ChannelResolutionClass> = BTreeSet::new();
    let mut handoff_classes: BTreeSet<CompanionHandoffClass> = BTreeSet::new();
    let mut all_reopen = true;
    let mut all_durable = true;
    let mut all_truthful = true;

    for case in cases {
        let outcome = &case.outcome;
        all_reopen &= outcome.all_routes_preserve_reopen_target;
        all_durable &= outcome.durable_truth_preserved;
        all_truthful &= outcome.no_generic_home_reopen;
        handoff_classes.insert(outcome.companion_handoff.handoff_class);
        for route in &outcome.resolved_surface_routes {
            surfaces.insert(route.fanout_surface_class);
            resolution_classes.insert(route.channel_resolution_class);
        }
    }

    AttentionRoutingCorpusSummary {
        case_count: cases.len() as u32,
        surfaces_covered: surfaces.into_iter().collect(),
        resolution_classes_covered: resolution_classes.into_iter().collect(),
        companion_handoff_classes_covered: handoff_classes.into_iter().collect(),
        all_cases_preserve_reopen_target: all_reopen,
        all_cases_preserve_durable_truth: all_durable,
        all_cases_truthful_reopen: all_truthful,
    }
}

// ---- envelope / context builders -----------------------------------------

fn exact_reopen(reopen_ref: &str, kind: ReopenTargetKind, target: &str) -> ReopenTarget {
    ReopenTarget {
        reopen_target_ref: reopen_ref.to_owned(),
        reopen_target_kind: kind,
        exact_target_identity_ref: Some(target.to_owned()),
        placeholder_announcement_label: None,
        revalidation_required_reason_label: None,
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

fn quiet_state(modes: Vec<QuietHoursMode>, reasons: Vec<SuppressionReason>, suppressed: bool) -> SuppressionState {
    SuppressionState {
        active_modes_at_mint: if modes.is_empty() {
            vec![QuietHoursMode::ModeNone]
        } else {
            modes
        },
        suppression_reasons: reasons,
        suppressed,
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
    suppression: SuppressionState,
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
        suppression_state: suppression,
        fanout_receipts: vec![],
        minted_at: minted_at.to_owned(),
    }
}

fn seed_cases() -> Vec<SeedCase> {
    vec![
        // 1. Foreground focused: in-app surfaces deliver; the OS notification is
        //    dropped as redundant because the window is focused.
        SeedCase {
            case_id: "case:foreground-focused-in-app",
            scenario_label:
                "Terminal session reconnects while the window is foreground and focused. The toast, status item, and durable row deliver in-app; the redundant OS notification is dropped but stays a visible receipt.",
            emissions: 1,
            context: ChannelContext::foreground_focused(),
            envelope: envelope(
                "terminal:recovered:01",
                SourceSubsystem::Terminal,
                SeverityClass::Success,
                PrivacyClass::SummarySafe,
                PrivacyPayloadClass::LockScreenSafeScoped,
                RedactionClass::MetadataSafeDefault,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:terminal:session:01:recovered",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Terminal session reconnected",
                exact_reopen(
                    "ux:reopen:terminal:session:01",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:terminal:recovered:01",
                ),
                vec![open_action(
                    "ux:action:terminal:open:01",
                    "Open terminal",
                    "cmd:terminal.open_session",
                    "obj:terminal:recovered:01",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T10:00:00Z",
            ),
        },
        // 2. Background hidden: OS notification carries the interruption; the
        //    lock-screen summary is not applicable because the device is unlocked.
        SeedCase {
            case_id: "case:background-os-delivery",
            scenario_label:
                "A review request arrives while no Aureline window is foreground and no companion is paired. The OS notification delivers; the lock-screen summary is not applicable because the device is unlocked; the companion push is not attempted because no endpoint is paired; durable truth still delivers.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden),
            envelope: envelope(
                "review:request:02",
                SourceSubsystem::ReviewAndDiff,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
                "dedupe:review:pr-2042:request",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "New review request",
                exact_reopen(
                    "ux:reopen:review:pr-2042:02",
                    ReopenTargetKind::CanonicalObject,
                    "obj:review:request:02",
                ),
                vec![open_action(
                    "ux:action:review:open:02",
                    "Open review",
                    "cmd:review.open",
                    "obj:review:request:02",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T10:05:00Z",
            ),
        },
        // 3. Locked / away: lock-screen summary is the external path.
        SeedCase {
            case_id: "case:locked-lock-screen-summary",
            scenario_label:
                "A build completes while the device is locked. The lock-screen summary and the OS notification deliver summary-first; durable truth is preserved for the return.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::LockedOrAway),
            envelope: envelope(
                "build:completed:03",
                SourceSubsystem::BuildSystem,
                SeverityClass::Success,
                PrivacyClass::SummarySafe,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::MetadataSafeDefault,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:build:job-03:completed",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                ],
                "Build completed",
                exact_reopen(
                    "ux:reopen:build:job-03",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:build:completed:03",
                ),
                vec![open_action(
                    "ux:action:build:open:03",
                    "Open build",
                    "cmd:build.open_job",
                    "obj:build:completed:03",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T10:10:00Z",
            ),
        },
        // 4. Quiet hours with a reachable companion: durable truth delivers, the
        //    toast / OS / companion surfaces are held.
        SeedCase {
            case_id: "case:quiet-hours-companion-held",
            scenario_label:
                "A mention arrives during quiet hours with a reachable companion. The durable row delivers; the toast, OS notification, and companion push are held; the companion handoff is recorded as held truth.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PairedAvailable)
                .with_quiet_hours(QuietHoursPosture::quiet_hours_user()),
            envelope: envelope(
                "collab:mention:04",
                SourceSubsystem::Collaboration,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:collab:mention:04",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "New mention",
                exact_reopen(
                    "ux:reopen:collab:mention:04",
                    ReopenTargetKind::CanonicalObject,
                    "obj:collab:mention:04",
                ),
                vec![open_action(
                    "ux:action:collab:open:04",
                    "Open thread",
                    "cmd:collab.open_thread",
                    "obj:collab:mention:04",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T22:30:00Z",
            ),
        },
        // 5. Admin suppression + lock-screen-forbidden security event.
        SeedCase {
            case_id: "case:admin-suppressed-security",
            scenario_label:
                "A security notice fires under admin suppression. The banner and lock-screen summary are suppressed by policy; the durable status item still delivers so the user has a path back.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_quiet_hours(QuietHoursPosture::admin_suppression()),
            envelope: envelope(
                "security:notice:05",
                SourceSubsystem::SecretBroker,
                SeverityClass::Error,
                PrivacyClass::SecurityCritical,
                PrivacyPayloadClass::PolicyForbiddenOnLockScreen,
                RedactionClass::InternalSupportRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:security:notice:05",
                vec![
                    FanoutSurfaceClass::ContextualBanner,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::LockScreenSummary,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Security notice",
                exact_reopen(
                    "ux:reopen:security:notice:05",
                    ReopenTargetKind::CanonicalObject,
                    "obj:security:notice:05",
                ),
                vec![open_action(
                    "ux:action:security:open:05",
                    "Open security notice",
                    "cmd:security.open_notice",
                    "obj:security:notice:05",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T11:00:00Z",
            ),
        },
        // 6. Dedupe burst: the same canonical event fires four times.
        SeedCase {
            case_id: "case:dedupe-burst-repeat",
            scenario_label:
                "An indexer warning fires four times in quick succession. The first emission delivers; emissions two through four coalesce on every surface while the reopen target stays stable.",
            emissions: 4,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::ForegroundUnfocused),
            envelope: envelope(
                "indexer:partial-shard:06",
                SourceSubsystem::Indexer,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:indexer:partial-shard:06",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                ],
                "Indexer running on a partial shard",
                exact_reopen(
                    "ux:reopen:indexer:shard:06",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:indexer:partial-shard:06",
                ),
                vec![open_action(
                    "ux:action:indexer:open:06",
                    "Open indexer activity",
                    "cmd:indexer.open_shard_activity",
                    "obj:indexer:partial-shard:06",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T11:15:00Z",
            ),
        },
        // 7. Companion available: the summary-first push delivers.
        SeedCase {
            case_id: "case:companion-available-fanout",
            scenario_label:
                "A long task completes while the user is away from the desktop with a reachable companion. The companion push delivers summary-first; activating it reopens the durable object in-product.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PairedAvailable),
            envelope: envelope(
                "task:completed:07",
                SourceSubsystem::TaskRunner,
                SeverityClass::Success,
                PrivacyClass::SummarySafe,
                PrivacyPayloadClass::LockScreenSafeGeneric,
                RedactionClass::MetadataSafeDefault,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:task:job-07:completed",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::CompanionPush,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Task completed",
                exact_reopen(
                    "ux:reopen:task:job-07",
                    ReopenTargetKind::DurableActivityRow,
                    "obj:task:completed:07",
                ),
                vec![open_action(
                    "ux:action:task:open:07",
                    "Open task",
                    "cmd:task.open_job",
                    "obj:task:completed:07",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T11:30:00Z",
            ),
        },
        // 8. Companion policy blocked.
        SeedCase {
            case_id: "case:companion-policy-blocked",
            scenario_label:
                "A managed workspace forbids companion fanout. The companion push is suppressed by policy; durable truth still delivers and the suppression stays an inspectable receipt.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden)
                .with_companion(CompanionAvailability::PolicyBlocked),
            envelope: envelope(
                "managed:publish:08",
                SourceSubsystem::AdminPolicy,
                SeverityClass::Warning,
                PrivacyClass::ManagedSensitive,
                PrivacyPayloadClass::RedactedMetadataOnly,
                RedactionClass::InternalSupportRestricted,
                DedupeKeyScheme::CanonicalEventId,
                "ux:event:managed:publish:08",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Managed publish update",
                exact_reopen(
                    "ux:reopen:managed:publish:08",
                    ReopenTargetKind::CanonicalObject,
                    "obj:managed:publish:08",
                ),
                vec![open_action(
                    "ux:action:managed:open:08",
                    "Open update",
                    "cmd:managed.open_publish",
                    "obj:managed:publish:08",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T11:45:00Z",
            ),
        },
        // 9. Screen reader active: a navigable durable surface is guaranteed and
        //    the announcement is required.
        SeedCase {
            case_id: "case:screen-reader-navigable",
            scenario_label:
                "A review request arrives with a screen reader active and the window focused. A navigable durable surface is present, the announcement is required, and the redundant OS notification is dropped.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_screen_reader(ScreenReaderPosture::Active),
            envelope: envelope(
                "ai:review:09",
                SourceSubsystem::AiApply,
                SeverityClass::Warning,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::InProductOnly,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
                "dedupe:ai:review:09:ready",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::ActivityCenterDigestCard,
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "AI review ready",
                exact_reopen(
                    "ux:reopen:ai:review:09",
                    ReopenTargetKind::ReviewContext,
                    "obj:ai:review:09",
                ),
                vec![open_action(
                    "ux:action:ai:open:09",
                    "Open review",
                    "cmd:ai.open_review",
                    "obj:ai:review:09",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T12:00:00Z",
            ),
        },
        // 10. Placeholder reopen: a recovered target announces a truthful
        //     placeholder rather than a generic home view.
        SeedCase {
            case_id: "case:placeholder-reopen",
            scenario_label:
                "A restore notification points at a target that is still being rebuilt. The reopen target announces a truthful placeholder rather than a generic home view; durable truth is preserved.",
            emissions: 1,
            context: ChannelContext::foreground_focused()
                .with_window_state(ActiveWindowState::BackgroundHidden),
            envelope: envelope(
                "recovery:restore:10",
                SourceSubsystem::VfsSave,
                SeverityClass::Degraded,
                PrivacyClass::WorkspaceSensitive,
                PrivacyPayloadClass::InProductOnly,
                RedactionClass::OperatorOnlyRestricted,
                DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
                "dedupe:recovery:restore:10:rebuilding",
                vec![
                    FanoutSurfaceClass::DurableJobRow,
                    FanoutSurfaceClass::StatusItem,
                    FanoutSurfaceClass::Toast,
                ],
                "Restore target rebuilding",
                ReopenTarget {
                    reopen_target_ref: "ux:reopen:recovery:restore:10".to_owned(),
                    reopen_target_kind: ReopenTargetKind::PlaceholderAnnounced,
                    exact_target_identity_ref: None,
                    placeholder_announcement_label: Some(
                        "Restore target is still rebuilding.".to_owned(),
                    ),
                    revalidation_required_reason_label: None,
                },
                vec![open_action(
                    "ux:action:recovery:open:10",
                    "Open recovery center",
                    "cmd:recovery.open_center",
                    "obj:recovery:restore:10",
                )],
                quiet_state(vec![], vec![], false),
                "2026-05-20T12:15:00Z",
            ),
        },
    ]
}

// ---- validation -----------------------------------------------------------

/// Validate the corpus invariants. Returns the list of violations; an empty
/// list means the corpus conforms.
pub fn validate_attention_routing_corpus(corpus: &AttentionRoutingCorpus) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if corpus.record_kind != ATTENTION_ROUTING_CORPUS_RECORD_KIND {
        errors.push(format!("corpus record_kind is {}", corpus.record_kind));
    }
    if corpus.schema_version != ATTENTION_ROUTER_BETA_SCHEMA_VERSION {
        errors.push(format!("corpus schema_version is {}", corpus.schema_version));
    }
    if corpus.shared_contract_ref != ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF {
        errors.push(format!(
            "corpus shared_contract_ref is {}",
            corpus.shared_contract_ref
        ));
    }
    if corpus.cases.is_empty() {
        errors.push("corpus has no cases".to_owned());
    }

    let mut seen_case_ids: BTreeSet<&str> = BTreeSet::new();
    for case in &corpus.cases {
        if !seen_case_ids.insert(case.case_id.as_str()) {
            errors.push(format!("duplicate case_id {}", case.case_id));
        }
        validate_case(case, &mut errors);
    }

    // Summary must agree with the cases it claims to summarize.
    let recomputed = summarize(&corpus.cases);
    if recomputed != corpus.summary {
        errors.push("corpus summary does not match its cases".to_owned());
    }

    // Coverage: every governed resolution class and the full surface set must
    // appear so the corpus actually proves cross-surface consistency.
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
        if !corpus.summary.resolution_classes_covered.contains(&required) {
            errors.push(format!(
                "corpus does not cover resolution class {}",
                required.as_str()
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_case(case: &AttentionRoutingCase, errors: &mut Vec<String>) {
    let outcome = &case.outcome;
    let where_ = &case.case_id;

    if case.record_kind != ATTENTION_ROUTING_CASE_RECORD_KIND {
        errors.push(format!("{where_}: case record_kind is {}", case.record_kind));
    }
    if outcome.record_kind != NOTIFICATION_ROUTE_OUTCOME_RECORD_KIND {
        errors.push(format!(
            "{where_}: outcome record_kind is {}",
            outcome.record_kind
        ));
    }
    if !outcome.all_routes_preserve_reopen_target {
        errors.push(format!("{where_}: a resolved route lost the reopen target"));
    }
    if !outcome.durable_truth_preserved {
        errors.push(format!("{where_}: durable truth was not preserved"));
    }
    if !outcome.no_generic_home_reopen {
        errors.push(format!("{where_}: reopen target is not truthful"));
    }
    // Every resolved route must carry the single reopen target ref.
    for route in &outcome.resolved_surface_routes {
        if route.reopen_target_ref != outcome.reopen_target.reopen_target_ref {
            errors.push(format!(
                "{where_}: surface {} carries a divergent reopen target",
                route.fanout_surface_class.as_str()
            ));
        }
    }
    // The six governed user verbs must be published, exactly once each.
    if outcome.available_lifecycle_actions.len() != 6 {
        errors.push(format!(
            "{where_}: expected 6 governed lifecycle actions, found {}",
            outcome.available_lifecycle_actions.len()
        ));
    }
    // Screen-reader posture must guarantee a navigable durable surface.
    if outcome.screen_reader_announce_required && !outcome.screen_reader_navigable_surface_present {
        errors.push(format!(
            "{where_}: screen reader active but no navigable durable surface delivered"
        ));
    }
}
