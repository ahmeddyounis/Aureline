//! Alpha notification taxonomy protected walk.
//!
//! This integration test exercises the shell notification contract across
//! quiet-hours routing, OS payload projection, badge/action reconciliation,
//! repeated-failure dedupe, exact reopen, and suppression-audit export.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use aureline_shell::notifications::{
    BadgeClass, DedupeKeyScheme, ExternalNotificationPayload, FanoutSurfaceClass,
    NotificationActionRequest, NotificationAttentionState, NotificationBadgeReconciliation,
    NotificationEnvelope, NotificationLifecycleActionKind, NotificationRouter,
    NotificationSuppressionAuditReport, NotificationSuppressionExplanationClass, PrivacyClass,
    PrivacyPayloadClass, QuietHoursMode, QuietHoursPosture, RedactionClass, ReopenTarget,
    ReopenTargetKind, SeverityClass, SourceSubsystem, StableAction, SuppressionState,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct NotificationTaxonomyAlphaFixture {
    #[serde(rename = "__fixture__")]
    fixture_metadata: FixtureMetadata,
    expected_os_payload: ExternalNotificationPayload,
    expected_badge_reconciliation: NotificationBadgeReconciliation,
    expected_action_states: Vec<NotificationAttentionState>,
    expected_suppression_audit: NotificationSuppressionAuditReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FixtureMetadata {
    name: String,
    scenario: String,
    contract_sections: Vec<String>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace member dir must have a parent")
        .parent()
        .expect("crate dir parent must have a parent")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root()
        .join("fixtures")
        .join("ux")
        .join("quiet_hours_cases")
        .join("desktop_notification_badge_reopen_audit.json")
}

fn audit_artifact_path() -> PathBuf {
    repo_root()
        .join("artifacts")
        .join("notifications")
        .join("notification_suppression_audit_alpha.yaml")
}

fn bless_enabled() -> bool {
    std::env::var("BLESS_NOTIFICATION_TAXONOMY_ALPHA")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn assert_or_bless(actual: NotificationTaxonomyAlphaFixture) {
    if bless_enabled() {
        let path = fixture_path();
        let json = serde_json::to_string_pretty(&actual).expect("fixture must serialize");
        std::fs::write(&path, format!("{json}\n"))
            .unwrap_or_else(|err| panic!("write fixture {}: {err}", path.display()));

        let artifact_path = audit_artifact_path();
        if let Some(parent) = artifact_path.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|err| panic!("create {}: {err}", parent.display()));
        }
        let yaml = serde_yaml::to_string(&actual.expected_suppression_audit).expect("audit yaml");
        std::fs::write(&artifact_path, yaml)
            .unwrap_or_else(|err| panic!("write artifact {}: {err}", artifact_path.display()));
        return;
    }

    let path = fixture_path();
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    let expected: NotificationTaxonomyAlphaFixture = serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));
    if expected != actual {
        let expected_json = serde_json::to_string_pretty(&expected).unwrap();
        let actual_json = serde_json::to_string_pretty(&actual).unwrap();
        panic!(
            "notification taxonomy fixture mismatch.\n--- expected ---\n{expected_json}\n\n--- actual ---\n{actual_json}\n\nRe-bless with BLESS_NOTIFICATION_TAXONOMY_ALPHA=1"
        );
    }

    let artifact_path = audit_artifact_path();
    let raw_artifact = std::fs::read_to_string(&artifact_path)
        .unwrap_or_else(|err| panic!("read artifact {}: {err}", artifact_path.display()));
    let artifact: NotificationSuppressionAuditReport = serde_yaml::from_str(&raw_artifact)
        .unwrap_or_else(|err| panic!("parse artifact {}: {err}", artifact_path.display()));
    assert_eq!(artifact, actual.expected_suppression_audit);
}

fn failed_test_envelope(sequence: u32) -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version:
            aureline_shell::notifications::NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: format!("ux:notif-env:test-run:unit-failed:{sequence:02}"),
        canonical_event_id: "ux:event:test-run:unit-failed".into(),
        event_lineage_id_ref: "ux:lineage:test-run:unit-failed".into(),
        source_subsystem: SourceSubsystem::TestRunner,
        source_event_ref: "test-run:unit-failed".into(),
        actor_identity_ref: "id:actor:system:test-runner".into(),
        canonical_object_target_ref: "obj:test-run:unit:failed".into(),
        severity_class: SeverityClass::Error,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
        dedupe_key_ref: "dedupe:test-run:unit:failure-root".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::OsNotification,
        ],
        summary_label: "Test run failed".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:test-run:unit-failed".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some("obj:test-run:unit:failed".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:test-run:open-failure".into(),
            label: "Open failed test run".into(),
            command_id: "cmd:test-run.open_failure".into(),
            target_identity_ref: "obj:test-run:unit:failed".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-13T03:30:00Z".into(),
    }
}

fn security_advisory_envelope() -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version:
            aureline_shell::notifications::NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: "ux:notif-env:security:critical:alpha".into(),
        canonical_event_id: "ux:event:security:critical:alpha".into(),
        event_lineage_id_ref: "ux:lineage:security:critical:alpha".into(),
        source_subsystem: SourceSubsystem::SecretBroker,
        source_event_ref: "security:critical:alpha".into(),
        actor_identity_ref: "id:actor:system:secret-broker".into(),
        canonical_object_target_ref: "obj:security:critical:alpha".into(),
        severity_class: SeverityClass::Critical,
        privacy_class: PrivacyClass::SecurityCritical,
        privacy_payload_class: PrivacyPayloadClass::RedactedMetadataOnly,
        redaction_class: RedactionClass::InternalSupportRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: "ux:event:security:critical:alpha".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::ContextualBanner,
            FanoutSurfaceClass::OsNotification,
        ],
        summary_label: "Active credential compromise advisory".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:security:critical:alpha".into(),
            reopen_target_kind: ReopenTargetKind::ReviewContext,
            exact_target_identity_ref: Some("obj:security:critical:alpha".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![
            StableAction {
                action_id: "ux:action:security:open-review:alpha".into(),
                label: "Open security review".into(),
                command_id: "cmd:security.open_review".into(),
                target_identity_ref: "obj:security:critical:alpha".into(),
                reopen_target_kind: ReopenTargetKind::ReviewContext,
                is_destructive: false,
            },
            StableAction {
                action_id: "ux:action:security:rotate-now:alpha".into(),
                label: "Rotate credential".into(),
                command_id: "cmd:security.rotate_credential".into(),
                target_identity_ref: "obj:security:critical:alpha".into(),
                reopen_target_kind: ReopenTargetKind::ReviewContext,
                is_destructive: true,
            },
        ],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-13T03:31:00Z".into(),
    }
}

fn action_state(
    suffix: &str,
    action_kind: NotificationLifecycleActionKind,
) -> NotificationAttentionState {
    let canonical_event_id = format!("ux:event:badge:failed-run:{suffix}");
    let target = format!("obj:badge:failed-run:{suffix}");
    let mut state = NotificationAttentionState::active(&canonical_event_id, BadgeClass::FailedRuns);
    let request = NotificationActionRequest::new(
        format!("ux:action:badge:{suffix}"),
        action_kind,
        &canonical_event_id,
        BadgeClass::FailedRuns,
        target,
        "2026-05-13T03:32:00Z",
    );
    let request = match action_kind {
        NotificationLifecycleActionKind::Snooze => {
            request.with_resume_condition("Resume after quiet hours")
        }
        NotificationLifecycleActionKind::Mute => request.with_muted_class("failed_runs"),
        _ => request,
    };
    state.apply(&request);
    state
}

fn actual_fixture() -> NotificationTaxonomyAlphaFixture {
    let mut router = NotificationRouter::new();
    let quiet_hours = QuietHoursPosture::quiet_hours_user();
    let do_not_disturb = QuietHoursPosture::do_not_disturb();

    let mut routed_pairs = Vec::new();
    for sequence in 1..=3 {
        let mut envelope = failed_test_envelope(sequence);
        quiet_hours.apply_to_envelope(&mut envelope);
        let routed = router.route(&envelope).expect("route failed test");
        routed_pairs.push((envelope, routed));
    }
    let latest_failure = routed_pairs.last().expect("latest failure").1.clone();
    assert_eq!(latest_failure.occurrence_count, 3);
    assert!(latest_failure.is_dedupe_repeat);

    let mut critical_envelope = security_advisory_envelope();
    do_not_disturb.apply_to_envelope(&mut critical_envelope);
    assert!(!critical_envelope.suppression_state.suppressed);
    assert!(critical_envelope
        .suppression_state
        .active_modes_at_mint
        .contains(&QuietHoursMode::ModeDoNotDisturbUser));
    let critical_routed = router
        .route(&critical_envelope)
        .expect("route critical advisory");
    let os_payload =
        ExternalNotificationPayload::project(&critical_routed, FanoutSurfaceClass::OsNotification)
            .expect("project OS payload");
    assert_eq!(os_payload.summary_label, "security notification");
    assert!(os_payload.safe_primary_action.is_some());
    assert_eq!(
        os_payload
            .safe_primary_action
            .as_ref()
            .map(|action| action.command_id.as_str()),
        Some("cmd:security.open_review")
    );
    assert!(os_payload.exact_reopen_only);
    assert!(os_payload.shortcut_bypass_prohibited);

    routed_pairs.push((critical_envelope, critical_routed));

    let action_states = vec![
        action_state("dismissed", NotificationLifecycleActionKind::Dismiss),
        action_state("acknowledged", NotificationLifecycleActionKind::Acknowledge),
        action_state("snoozed", NotificationLifecycleActionKind::Snooze),
        action_state("muted", NotificationLifecycleActionKind::Mute),
        action_state("resolved", NotificationLifecycleActionKind::Resolve),
        action_state("suppressed", NotificationLifecycleActionKind::Suppress),
    ];
    let badge_reconciliation =
        NotificationBadgeReconciliation::for_badge_class(&action_states, BadgeClass::FailedRuns);
    assert_eq!(badge_reconciliation.active_count, 1);
    assert_eq!(badge_reconciliation.held_or_suppressed_count, 2);
    assert_eq!(badge_reconciliation.acknowledged_count, 1);
    assert_eq!(badge_reconciliation.snoozed_count, 1);
    assert_eq!(badge_reconciliation.muted_count, 1);
    assert_eq!(badge_reconciliation.resolved_count, 1);
    assert!(badge_reconciliation.durable_history_preserved);

    let audit = NotificationSuppressionAuditReport::from_routed_pairs(
        "audit-report:notifications:alpha",
        "2026-05-13T03:33:00Z",
        routed_pairs
            .iter()
            .map(|(envelope, routed)| (envelope, routed)),
    );
    let classes: std::collections::HashSet<_> = audit
        .entries
        .iter()
        .map(|entry| entry.explanation_class)
        .collect();
    assert!(classes.contains(&NotificationSuppressionExplanationClass::HeldQuietHours));
    assert!(classes.contains(&NotificationSuppressionExplanationClass::DedupedCanonicalEvent));
    assert!(classes.contains(&NotificationSuppressionExplanationClass::EscalatedCriticalSafety));
    assert!(audit
        .entries
        .iter()
        .all(|entry| entry.durable_truth_preserved));

    NotificationTaxonomyAlphaFixture {
        fixture_metadata: FixtureMetadata {
            name: "desktop_notification_badge_reopen_audit".into(),
            scenario: "Quiet hours hold repeated failed test fanout while preserving one durable row; a critical security notification bypasses do-not-disturb with privacy-safe OS copy, one safe primary action, and exact reopen; badge action states reconcile dismiss, acknowledge, snooze, mute, resolve, and suppress without deleting durable history; the audit report explains shown, held, deduped, and escalated decisions.".into(),
            contract_sections: vec![
                "Typed notification envelope baseline".into(),
                "OS notification privacy and exact reopen".into(),
                "Badge/action reconciliation".into(),
                "Suppression audit reconstruction".into(),
            ],
        },
        expected_os_payload: os_payload,
        expected_badge_reconciliation: badge_reconciliation,
        expected_action_states: action_states,
        expected_suppression_audit: audit,
    }
}

#[test]
fn desktop_notification_badge_reopen_and_suppression_audit_are_canonical() {
    assert_or_bless(actual_fixture());
}
