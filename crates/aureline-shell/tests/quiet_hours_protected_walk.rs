//! Quiet-hours, do-not-disturb, and privacy-safe badge protected walk and
//! failure drill.
//!
//! These integration tests are the protected proof for the shell-level
//! quiet-hours posture seed. They build a [`QuietHoursPosture`], apply it
//! to typed notification envelopes before routing, route them through
//! [`NotificationRouter`], project the [`DurableBadgeProjection`], and
//! compare both the routed snapshot and the projection against checked-in
//! fixtures under `/fixtures/ux/quiet_hours_cases/*.json`.
//!
//! Set `BLESS_QUIET_HOURS_FIXTURES=1` to regenerate the fixtures from the
//! current shell output (e.g., after intentionally extending posture
//! behavior). The default test run never writes fixtures.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use aureline_shell::notifications::{
    DedupeKeyScheme, DurableBadgeProjection, FanoutReceiptState, FanoutSurfaceClass,
    NotificationEnvelope, NotificationRouter, PrivacyClass, PrivacyPayloadClass, QuietHoursMode,
    QuietHoursPosture, RedactionClass, ReopenTarget, ReopenTargetKind, RoutedNotification,
    SeverityClass, SourceSubsystem, StableAction, SuppressionReason, SuppressionState,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct QuietHoursFixture {
    #[serde(rename = "__fixture__")]
    fixture_metadata: FixtureMetadata,
    #[serde(rename = "__source__")]
    source: SourceMetadata,
    expected_routed: Vec<RoutedNotification>,
    expected_badge_projection: DurableBadgeProjection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FixtureMetadata {
    name: String,
    scenario: String,
    contract_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SourceMetadata {
    /// Modes active on the shell-level posture when the routing run
    /// captured this snapshot.
    active_posture_modes: Vec<QuietHoursMode>,
    /// Number of upstream envelope mints exercised. The protected walk
    /// uses 4 (three repeats of one indexer event collapse to one durable
    /// item, plus one security advisory). The failure drill uses 1.
    envelope_emissions: u32,
}

fn fixtures_root() -> PathBuf {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .expect("workspace member dir must have a parent")
        .parent()
        .expect("crate dir parent must have a parent")
        .join("fixtures")
        .join("ux")
        .join("quiet_hours_cases")
}

fn read_fixture(name: &str) -> QuietHoursFixture {
    let path = fixtures_root().join(name);
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read quiet-hours fixture {}: {err}",
            path.display()
        )
    });
    serde_json::from_str(&raw).unwrap_or_else(|err| {
        panic!(
            "failed to parse quiet-hours fixture {}: {err}",
            path.display()
        )
    })
}

fn write_fixture(name: &str, fixture: &QuietHoursFixture) {
    let path = fixtures_root().join(name);
    let json = serde_json::to_string_pretty(fixture).expect("fixture must serialize");
    std::fs::write(&path, format!("{json}\n")).unwrap_or_else(|err| {
        panic!(
            "failed to write quiet-hours fixture {}: {err}",
            path.display()
        )
    });
}

fn bless_enabled() -> bool {
    std::env::var("BLESS_QUIET_HOURS_FIXTURES")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn assert_or_bless(fixture_name: &str, actual: QuietHoursFixture) {
    if bless_enabled() {
        write_fixture(fixture_name, &actual);
        return;
    }
    let expected = read_fixture(fixture_name);
    if expected != actual {
        let actual_json = serde_json::to_string_pretty(&actual).unwrap();
        let expected_json = serde_json::to_string_pretty(&expected).unwrap();
        panic!(
            "quiet-hours fixture {fixture_name} mismatch.\n--- expected ---\n{expected_json}\n\n--- actual ---\n{actual_json}\n\nRe-bless with BLESS_QUIET_HOURS_FIXTURES=1"
        );
    }
}

// ---- Inline envelope builders ----

fn indexer_partial_shard_envelope() -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: "ux:notif-env:indexer:partial-shard:01".into(),
        canonical_event_id: "ux:event:indexer:partial-shard:01".into(),
        event_lineage_id_ref: "ux:lineage:indexer:partial-shard:01".into(),
        source_subsystem: SourceSubsystem::Indexer,
        source_event_ref: "indexer:partial-shard:01".into(),
        actor_identity_ref: "id:actor:system:indexer".into(),
        canonical_object_target_ref: "obj:indexer:shard:fixture-repo:01".into(),
        severity_class: SeverityClass::Warning,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: "ux:event:indexer:partial-shard:01".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::OsNotification,
        ],
        summary_label: "Indexer running on a partial shard".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:indexer:shard:01".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some("obj:indexer:shard:fixture-repo:01".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:indexer:open-shard-row:01".into(),
            label: "Open indexer activity".into(),
            command_id: "cmd:indexer.open_shard_activity".into(),
            target_identity_ref: "obj:indexer:shard:fixture-repo:01".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-10T11:00:00Z".into(),
    }
}

fn security_advisory_envelope() -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: "ux:notif-env:security:advisory:01".into(),
        canonical_event_id: "ux:event:security:advisory:01".into(),
        event_lineage_id_ref: "ux:lineage:security:advisory:01".into(),
        source_subsystem: SourceSubsystem::SecretBroker,
        source_event_ref: "security:advisory:01".into(),
        actor_identity_ref: "id:actor:system:secret-broker".into(),
        canonical_object_target_ref: "obj:security:advisory:01".into(),
        severity_class: SeverityClass::Critical,
        privacy_class: PrivacyClass::SecurityCritical,
        privacy_payload_class: PrivacyPayloadClass::RedactedMetadataOnly,
        redaction_class: RedactionClass::InternalSupportRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: "ux:event:security:advisory:01".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::ContextualBanner,
            FanoutSurfaceClass::OsNotification,
        ],
        summary_label: "Active credential compromise advisory".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:security:advisory:01".into(),
            reopen_target_kind: ReopenTargetKind::ReviewContext,
            exact_target_identity_ref: Some("obj:security:advisory:01".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:security:open-review:01".into(),
            label: "Open security review".into(),
            command_id: "cmd:security.open_review".into(),
            target_identity_ref: "obj:security:advisory:01".into(),
            reopen_target_kind: ReopenTargetKind::ReviewContext,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-10T11:05:00Z".into(),
    }
}

// ---- Tests ----

#[test]
fn protected_walk_quiet_hours_holds_attention_but_preserves_durable_truth_and_critical_safety() {
    // Posture: scheduled user quiet hours.
    let posture = QuietHoursPosture::quiet_hours_user();
    let mut router = NotificationRouter::new();

    // Three repeat emissions of the indexer warning → collapses to one
    // durable item, three deduped routes on the next two.
    let mut routed_snapshots: Vec<RoutedNotification> = Vec::new();
    for _ in 0..3 {
        let mut env = indexer_partial_shard_envelope();
        let changed = posture.apply_to_envelope(&mut env);
        assert!(
            changed,
            "posture must mark the warning envelope as suppressed"
        );
        assert!(env.suppression_state.suppressed);
        let r = router.route(&env).expect("indexer routing must succeed");
        // Toast and OS notification MUST be held on every repeat; durable
        // truth is preserved (delivered on the first emission, deduped on
        // repeats but still on the same reopen target ref).
        let by_surface: std::collections::HashMap<_, _> = r
            .surface_routes
            .iter()
            .map(|sr| (sr.fanout_surface_class, sr.receipt_state))
            .collect();
        assert!(matches!(
            by_surface.get(&FanoutSurfaceClass::Toast),
            Some(&FanoutReceiptState::HeldQuietHours)
                | Some(&FanoutReceiptState::DedupedCanonicalEvent)
        ));
        assert!(matches!(
            by_surface.get(&FanoutSurfaceClass::OsNotification),
            Some(&FanoutReceiptState::HeldQuietHours)
                | Some(&FanoutReceiptState::DedupedCanonicalEvent)
        ));
        routed_snapshots.push(r);
    }
    // The latest snapshot must be a dedupe repeat (occurrence_count = 3),
    // proving the badge will not inflate from raw emissions.
    let latest = routed_snapshots.last().unwrap();
    assert_eq!(latest.occurrence_count, 3);
    assert!(latest.is_dedupe_repeat);

    // Critical-safety security advisory: bypasses quiet hours.
    let mut crit_env = security_advisory_envelope();
    let changed = posture.apply_to_envelope(&mut crit_env);
    // Even though the posture is active, the suppression flag MUST stay
    // false for critical severity. The modes are recorded for audit but
    // suppression reasons are empty.
    assert!(changed); // active_modes_at_mint changed from [mode_none]
    assert!(
        !crit_env.suppression_state.suppressed,
        "critical-safety must never be held"
    );
    assert!(crit_env.suppression_state.suppression_reasons.is_empty());
    assert!(crit_env
        .suppression_state
        .active_modes_at_mint
        .contains(&QuietHoursMode::ModeQuietHoursUser));

    let crit_routed = router
        .route(&crit_env)
        .expect("security advisory routing must succeed");
    let crit_by_surface: std::collections::HashMap<_, _> = crit_routed
        .surface_routes
        .iter()
        .map(|sr| (sr.fanout_surface_class, sr.receipt_state))
        .collect();
    // OS notification + banner deliver because critical-safety bypasses
    // the hold; durable row delivers as always.
    assert_eq!(
        crit_by_surface.get(&FanoutSurfaceClass::DurableJobRow),
        Some(&FanoutReceiptState::Delivered)
    );
    assert_eq!(
        crit_by_surface.get(&FanoutSurfaceClass::ContextualBanner),
        Some(&FanoutReceiptState::Delivered)
    );
    assert_eq!(
        crit_by_surface.get(&FanoutSurfaceClass::OsNotification),
        Some(&FanoutReceiptState::Delivered)
    );
    routed_snapshots.push(crit_routed);

    // Badge projection: 4 raw emissions, 2 deduped durable items
    // (1 indexer warning + 1 security advisory), 1 critical, OS app-icon
    // badge suppressed under quiet-hours-user, lock-screen suppressed too.
    let projection = DurableBadgeProjection::from_routed(&routed_snapshots, &posture);
    assert_eq!(projection.durable_count, 2);
    assert_eq!(projection.severity_counts.warning, 1);
    assert_eq!(projection.severity_counts.critical, 1);
    assert_eq!(projection.critical_safety_count, 1);
    assert_eq!(projection.held_under_posture_count, 1);
    assert!(!projection.os_app_icon_badge_visible);
    assert!(!projection.lock_screen_summary_visible);
    assert_eq!(
        projection.privacy_safe_summary_label,
        "2 background items, 1 critical"
    );
    // Privacy-safe label MUST NOT name the canonical object or the actor.
    assert!(!projection.privacy_safe_summary_label.contains("indexer"));
    assert!(!projection.privacy_safe_summary_label.contains("advisory"));

    let actual = QuietHoursFixture {
        fixture_metadata: FixtureMetadata {
            name: "protected_walk_quiet_hours_holds_attention".into(),
            scenario: "Quiet hours are scheduled. Three repeat indexer warnings emit (only one durable row appears in the projection), and one security advisory emits at critical severity. The router holds the toast and the OS notification on the warning, delivers the durable_job_row on every emission, and bypasses the hold for the critical advisory. The badge projection counts deduped durable items only, exposes a privacy-safe summary label, and refuses the OS app-icon badge under the active mode.".into(),
            contract_sections: vec![
                "Truth invariants".into(),
                "Quiet-hours policy matrix".into(),
                "Privacy-safe payload classes".into(),
                "Critical-safety bypass".into(),
            ],
        },
        source: SourceMetadata {
            active_posture_modes: vec![QuietHoursMode::ModeQuietHoursUser],
            envelope_emissions: 4,
        },
        expected_routed: routed_snapshots,
        expected_badge_projection: projection,
    };
    assert_or_bless("protected_walk_quiet_hours_holds_attention.json", actual);
}

#[test]
fn failure_drill_dnd_during_sensitive_event_keeps_critical_safety_visible() {
    // Failure drill: do-not-disturb is active when a sensitive (critical-
    // safety) event arrives. The badge MUST respect privacy: no raw object
    // identity, no actor identity, no workspace label, and the OS app-icon
    // badge is suppressed under DND. The interruptibility policy MUST let
    // the critical event interrupt — DND does not gag tier_critical_safety.
    let posture = QuietHoursPosture::do_not_disturb();
    let mut router = NotificationRouter::new();
    let mut env = security_advisory_envelope();
    let changed = posture.apply_to_envelope(&mut env);
    assert!(changed);
    assert!(
        !env.suppression_state.suppressed,
        "DND must never hold critical-safety severity"
    );
    assert!(env.suppression_state.suppression_reasons.is_empty());
    assert!(env
        .suppression_state
        .active_modes_at_mint
        .contains(&QuietHoursMode::ModeDoNotDisturbUser));

    let routed = router
        .route(&env)
        .expect("security advisory routing must succeed");

    // Every surface delivers because the envelope is not suppressed.
    let by_surface: std::collections::HashMap<_, _> = routed
        .surface_routes
        .iter()
        .map(|sr| (sr.fanout_surface_class, sr.receipt_state))
        .collect();
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::DurableJobRow),
        Some(&FanoutReceiptState::Delivered)
    );
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::ContextualBanner),
        Some(&FanoutReceiptState::Delivered)
    );
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::OsNotification),
        Some(&FanoutReceiptState::Delivered)
    );

    // Reopen target preservation: every surface shares the same exact
    // identity ref, even when a sensitive event uses a redacted privacy
    // posture — the activity center, the banner, and the OS notification
    // all reopen the same security review context.
    assert!(routed.all_routes_preserve_reopen_target());
    for route in &routed.surface_routes {
        assert_eq!(route.reopen_target_ref, "ux:reopen:security:advisory:01");
    }

    let projection = DurableBadgeProjection::from_routed(std::slice::from_ref(&routed), &posture);
    assert_eq!(projection.durable_count, 1);
    assert_eq!(projection.critical_safety_count, 1);
    assert_eq!(projection.held_under_posture_count, 0);
    assert!(!projection.os_app_icon_badge_visible);
    assert!(!projection.lock_screen_summary_visible);
    assert_eq!(
        projection.privacy_safe_summary_label,
        "1 background item, 1 critical"
    );
    // Failure-drill privacy assertion: the badge label MUST NOT echo the
    // raw object identity, actor, or summary copy of the sensitive event.
    let label = &projection.privacy_safe_summary_label;
    assert!(!label.contains("security:advisory"));
    assert!(!label.contains("secret-broker"));
    assert!(!label.contains("compromise"));
    assert!(!label.contains("credential"));

    let actual = QuietHoursFixture {
        fixture_metadata: FixtureMetadata {
            name: "failure_drill_dnd_during_sensitive_event".into(),
            scenario: "Do-not-disturb is active when a sensitive (critical-safety) security advisory arrives. The shell posture MUST NOT suppress critical severity: the durable_job_row, banner, and OS notification deliver on the same reopen target ref so the user can route through the in-product review canvas. The badge projection respects privacy by exposing only category-class tokens and counts; the OS app-icon badge stays suppressed because DND policy holds it for non-critical items, but the in-product durable count and critical-safety subcount remain truthful so the activity center can still show the user there is one item to review.".into(),
            contract_sections: vec![
                "Failure drill: DND during sensitive event".into(),
                "Critical-safety bypass".into(),
                "Privacy-safe summary label".into(),
                "Reopen target preserved across surfaces".into(),
            ],
        },
        source: SourceMetadata {
            active_posture_modes: vec![QuietHoursMode::ModeDoNotDisturbUser],
            envelope_emissions: 1,
        },
        expected_routed: vec![routed],
        expected_badge_projection: projection,
    };
    assert_or_bless("failure_drill_dnd_during_sensitive_event.json", actual);
}

#[test]
fn presentation_mode_holds_warning_envelope_but_preserves_durable_row() {
    // Sanity: presentation mode (audience-visible) holds attention surfaces.
    let posture = QuietHoursPosture::presentation();
    let mut router = NotificationRouter::new();
    let mut env = indexer_partial_shard_envelope();
    posture.apply_to_envelope(&mut env);
    assert!(env.suppression_state.suppressed);
    assert!(env
        .suppression_state
        .suppression_reasons
        .contains(&SuppressionReason::PresentationModeActive));
    let routed = router.route(&env).unwrap();
    let by_surface: std::collections::HashMap<_, _> = routed
        .surface_routes
        .iter()
        .map(|sr| (sr.fanout_surface_class, sr.receipt_state))
        .collect();
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::DurableJobRow),
        Some(&FanoutReceiptState::Delivered)
    );
    // Presentation: presentation/screen-share/privacy modes go through
    // SuppressedPolicy so audience-visible surfaces are denied outright.
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::Toast),
        Some(&FanoutReceiptState::SuppressedPolicy)
    );
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::OsNotification),
        Some(&FanoutReceiptState::SuppressedPolicy)
    );
}
