//! Unit tests for the governed attention router.

use crate::notifications::actions::NotificationLifecycleActionKind;
use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass,
    PrivacyPayloadClass, QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind,
    SeverityClass, SourceSubsystem, StableAction, SuppressionState,
};
use crate::notifications::quiet_hours::QuietHoursPosture;

use super::context::{
    ActiveWindowState, ChannelContext, CompanionAvailability, PresentationFollowState,
    ScreenReaderPosture,
};
use super::outcome::{AttentionRouter, ChannelResolutionClass, CompanionHandoffClass};
use super::{
    seeded_attention_routing_corpus, validate_attention_routing_corpus, AttentionRouteSupportExport,
};

fn base_envelope() -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: "ux:notif-env:unit:01".into(),
        canonical_event_id: "ux:event:unit:01".into(),
        event_lineage_id_ref: "ux:lineage:unit:01".into(),
        source_subsystem: SourceSubsystem::ReviewAndDiff,
        source_event_ref: "src:unit:01".into(),
        actor_identity_ref: "id:actor:unit".into(),
        canonical_object_target_ref: "obj:unit:01".into(),
        severity_class: SeverityClass::Warning,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: "ux:event:unit:01".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::OsNotification,
            FanoutSurfaceClass::CompanionPush,
        ],
        summary_label: "Unit event".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:unit:01".into(),
            reopen_target_kind: ReopenTargetKind::CanonicalObject,
            exact_target_identity_ref: Some("obj:unit:01".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:unit:open:01".into(),
            label: "Open".into(),
            command_id: "cmd:unit.open".into(),
            target_identity_ref: "obj:unit:01".into(),
            reopen_target_kind: ReopenTargetKind::CanonicalObject,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-20T09:00:00Z".into(),
    }
}

fn resolution_for(
    outcome: &super::outcome::NotificationRouteOutcome,
    surface: FanoutSurfaceClass,
) -> ChannelResolutionClass {
    outcome
        .resolved_surface_routes
        .iter()
        .find(|route| route.fanout_surface_class == surface)
        .map(|route| route.channel_resolution_class)
        .unwrap_or_else(|| panic!("missing surface {surface:?}"))
}

#[test]
fn foreground_focus_drops_redundant_os_notification_but_keeps_in_app() {
    let mut router = AttentionRouter::new();
    let context =
        ChannelContext::foreground_focused().with_companion(CompanionAvailability::PairedAvailable);
    let outcome = router.route(&base_envelope(), &context).unwrap();

    assert_eq!(
        resolution_for(&outcome, FanoutSurfaceClass::OsNotification),
        ChannelResolutionClass::SuppressedForegroundRedundant
    );
    assert_eq!(
        resolution_for(&outcome, FanoutSurfaceClass::Toast),
        ChannelResolutionClass::DeliveredInApp
    );
    // Every resolved route keeps the single reopen target.
    assert!(outcome.all_routes_preserve_reopen_target);
    assert!(outcome.durable_truth_preserved);
    assert!(outcome.no_generic_home_reopen);
}

#[test]
fn companion_unavailable_is_not_attempted_but_stays_truth() {
    let mut router = AttentionRouter::new();
    let context = ChannelContext::foreground_focused()
        .with_window_state(ActiveWindowState::BackgroundHidden)
        .with_companion(CompanionAvailability::PairedUnavailable);
    let outcome = router.route(&base_envelope(), &context).unwrap();

    let companion = outcome
        .resolved_surface_routes
        .iter()
        .find(|r| r.fanout_surface_class == FanoutSurfaceClass::CompanionPush)
        .unwrap();
    assert_eq!(
        companion.resolved_receipt_state,
        FanoutReceiptState::NotAttemptedNoRoute
    );
    assert_eq!(
        companion.channel_resolution_class,
        ChannelResolutionClass::CompanionUnavailable
    );
    assert_eq!(
        outcome.companion_handoff.handoff_class,
        CompanionHandoffClass::SummaryFanoutUnavailable
    );
    // Background + reachable OS still delivers.
    assert_eq!(
        resolution_for(&outcome, FanoutSurfaceClass::OsNotification),
        ChannelResolutionClass::DeliveredExternalSummary
    );
}

#[test]
fn companion_policy_block_suppresses_by_policy() {
    let mut router = AttentionRouter::new();
    let context = ChannelContext::foreground_focused()
        .with_window_state(ActiveWindowState::BackgroundHidden)
        .with_companion(CompanionAvailability::PolicyBlocked);
    let outcome = router.route(&base_envelope(), &context).unwrap();
    let companion = outcome
        .resolved_surface_routes
        .iter()
        .find(|r| r.fanout_surface_class == FanoutSurfaceClass::CompanionPush)
        .unwrap();
    assert_eq!(
        companion.resolved_receipt_state,
        FanoutReceiptState::SuppressedPolicy
    );
    assert_eq!(
        outcome.companion_handoff.handoff_class,
        CompanionHandoffClass::SummaryFanoutPolicyBlocked
    );
}

#[test]
fn quiet_hours_holds_attention_but_delivers_durable_truth() {
    let mut router = AttentionRouter::new();
    let context = ChannelContext::foreground_focused()
        .with_window_state(ActiveWindowState::BackgroundHidden)
        .with_companion(CompanionAvailability::PairedAvailable)
        .with_quiet_hours(QuietHoursPosture::quiet_hours_user());
    let outcome = router.route(&base_envelope(), &context).unwrap();

    let durable = outcome
        .resolved_surface_routes
        .iter()
        .find(|r| r.fanout_surface_class == FanoutSurfaceClass::DurableJobRow)
        .unwrap();
    assert_eq!(
        durable.resolved_receipt_state,
        FanoutReceiptState::Delivered
    );

    for surface in [
        FanoutSurfaceClass::Toast,
        FanoutSurfaceClass::OsNotification,
        FanoutSurfaceClass::CompanionPush,
    ] {
        let route = outcome
            .resolved_surface_routes
            .iter()
            .find(|r| r.fanout_surface_class == surface)
            .unwrap();
        assert_eq!(
            route.resolved_receipt_state,
            FanoutReceiptState::HeldQuietHours,
            "{surface:?} should be held during quiet hours"
        );
    }
    assert!(outcome.durable_truth_preserved);
}

#[test]
fn presenting_holds_audience_visible_surfaces_but_keeps_durable_truth() {
    let mut router = AttentionRouter::new();
    // Presenting folds into the effective presentation posture, which holds
    // attention-grabbing surfaces while durable truth keeps flowing.
    let context = ChannelContext::foreground_focused()
        .with_window_state(ActiveWindowState::ForegroundUnfocused)
        .with_companion(CompanionAvailability::PairedAvailable)
        .with_presentation(PresentationFollowState::Presenting);
    let outcome = router.route(&base_envelope(), &context).unwrap();

    let toast = outcome
        .resolved_surface_routes
        .iter()
        .find(|r| r.fanout_surface_class == FanoutSurfaceClass::Toast)
        .unwrap();
    // Presentation mode suppresses audience-visible surfaces by policy.
    assert_eq!(
        toast.resolved_receipt_state,
        FanoutReceiptState::SuppressedPolicy
    );
    assert_eq!(
        toast.channel_resolution_class,
        ChannelResolutionClass::SuppressedByPolicy
    );
    assert!(outcome.durable_truth_preserved);
}

#[test]
fn dedupe_repeat_collapses_without_splitting_reopen_target() {
    let mut router = AttentionRouter::new();
    let context = ChannelContext::foreground_focused();
    let env = base_envelope();
    let _ = router.route(&env, &context).unwrap();
    let outcome = router.route(&env, &context).unwrap();
    assert!(outcome.is_dedupe_repeat);
    assert_eq!(outcome.occurrence_count, 2);
    for route in &outcome.resolved_surface_routes {
        // Repeats either dedupe or stay foreground-suppressed; none upgrade.
        assert!(matches!(
            route.channel_resolution_class,
            ChannelResolutionClass::DedupedRepeat
                | ChannelResolutionClass::SuppressedForegroundRedundant
                | ChannelResolutionClass::CompanionUnavailable
        ));
    }
    assert!(outcome.all_routes_preserve_reopen_target);
}

#[test]
fn screen_reader_requires_navigable_surface_and_announce() {
    let mut router = AttentionRouter::new();
    let context =
        ChannelContext::foreground_focused().with_screen_reader(ScreenReaderPosture::Active);
    let outcome = router.route(&base_envelope(), &context).unwrap();
    assert!(outcome.screen_reader_announce_required);
    assert!(outcome.screen_reader_navigable_surface_present);
}

#[test]
fn governed_actions_include_clear_and_resolve_distinctly() {
    let mut router = AttentionRouter::new();
    let outcome = router
        .route(&base_envelope(), &ChannelContext::foreground_focused())
        .unwrap();
    let kinds: Vec<NotificationLifecycleActionKind> = outcome
        .available_lifecycle_actions
        .iter()
        .map(|a| a.action_kind)
        .collect();
    for required in [
        NotificationLifecycleActionKind::Dismiss,
        NotificationLifecycleActionKind::Snooze,
        NotificationLifecycleActionKind::Acknowledge,
        NotificationLifecycleActionKind::Mute,
        NotificationLifecycleActionKind::Clear,
        NotificationLifecycleActionKind::Resolve,
    ] {
        assert!(kinds.contains(&required), "missing action {required:?}");
    }
    // Only resolve mutates the source object.
    let resolve = outcome
        .available_lifecycle_actions
        .iter()
        .find(|a| a.action_kind == NotificationLifecycleActionKind::Resolve)
        .unwrap();
    assert!(resolve.mutates_source_object);
    let clear = outcome
        .available_lifecycle_actions
        .iter()
        .find(|a| a.action_kind == NotificationLifecycleActionKind::Clear)
        .unwrap();
    assert!(!clear.mutates_source_object);
    assert!(clear.durable_history_preserved);
}

#[test]
fn safe_action_target_is_open_only() {
    let mut router = AttentionRouter::new();
    let outcome = router
        .route(&base_envelope(), &ChannelContext::foreground_focused())
        .unwrap();
    let action = outcome.safe_action_target.expect("safe action present");
    assert!(!action.is_destructive);
    assert!(action.command_id.contains(".open"));
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_attention_routing_corpus();
    validate_attention_routing_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: super::AttentionRoutingCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    let export = AttentionRouteSupportExport::from_corpus(
        "support-export:attention-router-beta:001",
        "2026-05-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.cases.len());
}
