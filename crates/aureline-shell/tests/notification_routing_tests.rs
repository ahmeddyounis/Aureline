//! Notification routing protected walk and failure drill.
//!
//! These integration tests are the protected proof for the toast/banner/
//! status routing seed. They consume the typed notification envelope from
//! `/schemas/ux/notification_envelope.schema.json` (mirrored in
//! `aureline_shell::notifications::envelope`), route it through
//! [`NotificationRouter`], and compare the routed surface set against
//! checked-in fixtures under
//! `/fixtures/ux/notification_routing_cases/*.json`.
//!
//! Set `BLESS_NOTIFICATION_ROUTING_FIXTURES=1` to regenerate the fixtures
//! from the current router output (e.g., after intentionally extending
//! routing behavior). The default test run never writes fixtures.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use aureline_shell::notifications::{
    DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope,
    NotificationRouter, NotificationSurfaceSnapshot, PrivacyClass, PrivacyPayloadClass,
    QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind, RoutedNotification,
    SeverityClass, SourceSubsystem, StableAction, SuppressionReason, SuppressionState,
};

/// A routing fixture wraps the routed-notification snapshot the router
/// emits, plus a `__source__` block linking back to the upstream envelope
/// fixture and the emission count exercised.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RoutingFixture {
    #[serde(rename = "__fixture__")]
    fixture_metadata: FixtureMetadata,
    #[serde(rename = "__source__")]
    source: SourceMetadata,
    expected_routed: RoutedNotification,
    expected_surface_snapshot: NotificationSurfaceSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FixtureMetadata {
    name: String,
    scenario: String,
    contract_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SourceMetadata {
    /// Path to the upstream envelope fixture, relative to the repository
    /// root. Reviewers can join this fixture back to the envelope it was
    /// routed from.
    envelope_fixture_ref: String,
    /// Number of router.route() calls used to produce
    /// `expected_routed`. The protected walk uses 1; the failure drill
    /// uses 4 to exercise the dedupe path.
    emissions: u32,
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
        .join("notification_routing_cases")
}

fn read_fixture(name: &str) -> RoutingFixture {
    let path = fixtures_root().join(name);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read routing fixture {}: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("failed to parse routing fixture {}: {err}", path.display()))
}

fn write_fixture(name: &str, fixture: &RoutingFixture) {
    let path = fixtures_root().join(name);
    let json = serde_json::to_string_pretty(fixture).expect("fixture must serialize");
    std::fs::write(&path, format!("{json}\n"))
        .unwrap_or_else(|err| panic!("failed to write routing fixture {}: {err}", path.display()));
}

fn bless_enabled() -> bool {
    std::env::var("BLESS_NOTIFICATION_ROUTING_FIXTURES")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Compare the actual routed snapshot to the fixture. When
/// BLESS_NOTIFICATION_ROUTING_FIXTURES=1 is set and the snapshots
/// disagree, overwrite the fixture instead of asserting.
fn assert_or_bless(fixture_name: &str, actual: RoutingFixture) {
    if bless_enabled() {
        write_fixture(fixture_name, &actual);
        return;
    }
    let expected = read_fixture(fixture_name);
    if expected != actual {
        let actual_json = serde_json::to_string_pretty(&actual).unwrap();
        let expected_json = serde_json::to_string_pretty(&expected).unwrap();
        panic!(
            "routing fixture {fixture_name} mismatch.\n--- expected ---\n{expected_json}\n\n--- actual ---\n{actual_json}\n\nRe-bless with BLESS_NOTIFICATION_ROUTING_FIXTURES=1"
        );
    }
}

/// Sanity-check: the routing fixture's `__source__` envelope reference
/// must resolve to a real envelope fixture and parse as the boundary
/// schema's `notification_envelope_record`. This prevents the routing
/// fixture from drifting away from the envelope it claims to be routed
/// from.
fn assert_source_envelope_resolvable(source: &SourceMetadata) {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = crate_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace must have a root");
    let envelope_path = repo_root.join(&source.envelope_fixture_ref);
    let raw = std::fs::read_to_string(&envelope_path).unwrap_or_else(|err| {
        panic!(
            "routing fixture references missing envelope fixture {}: {err}",
            envelope_path.display()
        )
    });
    let value: Value = serde_json::from_str(&raw).unwrap_or_else(|err| {
        panic!(
            "envelope fixture {} did not parse as JSON: {err}",
            envelope_path.display()
        )
    });
    assert_eq!(
        value
            .get("record_kind")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        "notification_envelope_record",
        "source envelope fixture must be a notification_envelope_record"
    );
}

// ---- Inline envelope builders (mirror the upstream envelope fixtures) ----

fn terminal_recovery_envelope() -> NotificationEnvelope {
    // Mirrors /fixtures/ux/notification_envelope_cases/recovery_after_terminal_reconnect.json.
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: "ux:notif-env:terminal:recovered:01".into(),
        canonical_event_id: "ux:event:terminal:recovered:01".into(),
        event_lineage_id_ref: "ux:lineage:terminal:recovered:01".into(),
        source_subsystem: SourceSubsystem::Terminal,
        source_event_ref: "terminal:session:recovered:01".into(),
        actor_identity_ref: "id:actor:system:terminal-host".into(),
        canonical_object_target_ref: "obj:terminal:session:01".into(),
        severity_class: SeverityClass::Success,
        privacy_class: PrivacyClass::SummarySafe,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeScoped,
        redaction_class: RedactionClass::MetadataSafeDefault,
        dedupe_key_scheme: DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
        dedupe_key_ref: "dedupe:terminal:session:01:recovered".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
        ],
        summary_label: "Terminal session reconnected".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:terminal:session:01".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some("obj:terminal:session:01".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![
            StableAction {
                action_id: "ux:action:terminal:open-session:01".into(),
                label: "Open terminal".into(),
                command_id: "cmd:terminal.open_session".into(),
                target_identity_ref: "obj:terminal:session:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            },
            StableAction {
                action_id: "ux:action:terminal:open-history:01".into(),
                label: "Open recovery history".into(),
                command_id: "cmd:terminal.open_session_history".into(),
                target_identity_ref: "obj:terminal:session:01:history".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            },
        ],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-10T10:30:00Z".into(),
    }
}

fn indexer_partial_shard_envelope() -> NotificationEnvelope {
    // Mirrors /fixtures/ux/notification_envelope_cases/duplicate_dedupe_envelope.json.
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
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
        ],
        summary_label: "Indexer running on a partial shard".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:indexer:shard:01".into(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some("obj:indexer:shard:fixture-repo:01".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![
            StableAction {
                action_id: "ux:action:indexer:open-shard-row:01".into(),
                label: "Open indexer activity".into(),
                command_id: "cmd:indexer.open_shard_activity".into(),
                target_identity_ref: "obj:indexer:shard:fixture-repo:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            },
            StableAction {
                action_id: "ux:action:indexer:retry-shard:01".into(),
                label: "Retry shard".into(),
                command_id: "cmd:indexer.retry_shard".into(),
                target_identity_ref: "obj:indexer:shard:fixture-repo:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            },
        ],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-10T10:15:00Z".into(),
    }
}

fn quiet_hours_review_envelope() -> NotificationEnvelope {
    // Mirrors /fixtures/ux/notification_envelope_cases/held_quiet_hours_companion_fanout.json
    // but routes the full {durable, toast, companion} surface set so the
    // failure-drill predicate "attention held while durable truth still
    // delivers" is exercised end to end.
    NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: 1,
        notification_envelope_id: "ux:notif-env:review-request:quiet-hours:01".into(),
        canonical_event_id: "ux:event:review-request:01".into(),
        event_lineage_id_ref: "ux:lineage:review-request:01".into(),
        source_subsystem: SourceSubsystem::ReviewAndDiff,
        source_event_ref: "review:request:01".into(),
        actor_identity_ref: "id:actor:user:collaborator".into(),
        canonical_object_target_ref: "obj:review:pr-1942".into(),
        severity_class: SeverityClass::Warning,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
        dedupe_key_ref: "dedupe:review:pr-1942:request".into(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::CompanionPush,
        ],
        summary_label: "New review request".into(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:review:pr-1942:01".into(),
            reopen_target_kind: ReopenTargetKind::CanonicalObject,
            exact_target_identity_ref: Some("obj:review:pr-1942".into()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:review:open:pr-1942:01".into(),
            label: "Open review".into(),
            command_id: "cmd:review.open".into(),
            target_identity_ref: "obj:review:pr-1942".into(),
            reopen_target_kind: ReopenTargetKind::CanonicalObject,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeQuietHoursUser],
            suppression_reasons: vec![SuppressionReason::QuietHoursUserPolicy],
            suppressed: true,
        },
        fanout_receipts: vec![],
        minted_at: "2026-05-10T10:45:00Z".into(),
    }
}

// ---- Tests: protected walk + failure drill ----

#[test]
fn protected_walk_routes_terminal_recovery_across_durable_status_and_toast() {
    let envelope = terminal_recovery_envelope();
    let mut router = NotificationRouter::new();
    let routed = router.route(&envelope).expect("routing must succeed");

    // Behavior assertions — these MUST hold regardless of fixture state so a
    // failed bless still leaves the contract intact.
    assert_eq!(routed.surface_routes.len(), 3);
    assert!(routed.has_visible_surface());
    assert!(routed.all_routes_preserve_reopen_target());
    for route in &routed.surface_routes {
        assert_eq!(route.receipt_state, FanoutReceiptState::Delivered);
        assert_eq!(
            route.reopen_target_ref,
            envelope.reopen_target.reopen_target_ref
        );
    }

    let snapshot = NotificationSurfaceSnapshot::project(&routed);
    let actual = RoutingFixture {
        fixture_metadata: FixtureMetadata {
            name: "protected_walk_cross_surface_routes".into(),
            scenario: "Terminal session reconnects on the same opaque session id. The router emits delivered receipts on durable_job_row, status_item, and toast — all carrying the same reopen target ref so the activity row, the status item, and the toast lead to the same canonical terminal session.".into(),
            contract_sections: vec![
                "Required envelope anatomy".into(),
                "Stable ids and localized copy".into(),
                "Fanout receipts".into(),
            ],
        },
        source: SourceMetadata {
            envelope_fixture_ref:
                "fixtures/ux/notification_envelope_cases/recovery_after_terminal_reconnect.json"
                    .into(),
            emissions: 1,
        },
        expected_routed: routed,
        expected_surface_snapshot: snapshot,
    };
    assert_source_envelope_resolvable(&actual.source);
    assert_or_bless("protected_walk_cross_surface_routes.json", actual);
}

#[test]
fn failure_drill_collapses_four_indexer_emissions_onto_one_durable_truth() {
    let envelope = indexer_partial_shard_envelope();
    let mut router = NotificationRouter::new();
    // Route the same canonical event four times and capture only the final
    // routed snapshot — the dedupe receipts on emissions 2..=4 are the proof
    // that repeats do not split the reopen target across surfaces.
    let _first = router.route(&envelope).expect("routing must succeed");
    let _second = router.route(&envelope).expect("routing must succeed");
    let _third = router.route(&envelope).expect("routing must succeed");
    let routed = router.route(&envelope).expect("routing must succeed");

    assert_eq!(routed.occurrence_count, 4);
    assert!(routed.is_dedupe_repeat);
    for route in &routed.surface_routes {
        assert_eq!(
            route.receipt_state,
            FanoutReceiptState::DedupedCanonicalEvent,
            "surface {:?} should dedupe on repeat",
            route.fanout_surface_class
        );
        assert_eq!(
            route.reopen_target_ref,
            envelope.reopen_target.reopen_target_ref
        );
        assert!(route
            .suppression_reasons
            .contains(&SuppressionReason::DedupeSameCanonicalEvent));
    }

    let snapshot = NotificationSurfaceSnapshot::project(&routed);
    let actual = RoutingFixture {
        fixture_metadata: FixtureMetadata {
            name: "failure_drill_dedupe_repeats".into(),
            scenario: "An indexer warning fires four times in quick succession. The first emission delivers on durable_job_row, status_item, and toast; emissions 2..=4 emit deduped_canonical_event receipts on every surface while the reopen target ref stays stable. The activity center never splits across two reopen targets and never spawns four toasts.".into(),
            contract_sections: vec![
                "Required envelope anatomy".into(),
                "Fanout receipts".into(),
                "Privacy-class rules".into(),
                "Non-conforming cases".into(),
            ],
        },
        source: SourceMetadata {
            envelope_fixture_ref:
                "fixtures/ux/notification_envelope_cases/duplicate_dedupe_envelope.json".into(),
            emissions: 4,
        },
        expected_routed: routed,
        expected_surface_snapshot: snapshot,
    };
    assert_source_envelope_resolvable(&actual.source);
    assert_or_bless("failure_drill_dedupe_repeats.json", actual);
}

#[test]
fn quiet_hours_holds_attention_surfaces_but_durable_row_still_delivers() {
    let envelope = quiet_hours_review_envelope();
    let mut router = NotificationRouter::new();
    let routed = router.route(&envelope).expect("routing must succeed");

    // The durable_job_row MUST still deliver so the user has a path back to
    // the canonical review object. The toast and companion_push MUST hold so
    // we honor quiet hours.
    let by_surface: std::collections::HashMap<FanoutSurfaceClass, FanoutReceiptState> = routed
        .surface_routes
        .iter()
        .map(|r| (r.fanout_surface_class, r.receipt_state))
        .collect();
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::DurableJobRow),
        Some(&FanoutReceiptState::Delivered)
    );
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::Toast),
        Some(&FanoutReceiptState::HeldQuietHours)
    );
    assert_eq!(
        by_surface.get(&FanoutSurfaceClass::CompanionPush),
        Some(&FanoutReceiptState::HeldQuietHours)
    );

    let snapshot = NotificationSurfaceSnapshot::project(&routed);
    let actual = RoutingFixture {
        fixture_metadata: FixtureMetadata {
            name: "quiet_hours_holds_attention_but_delivers_durable".into(),
            scenario: "A review request arrives during quiet hours. The router holds the toast and companion_push on the active quiet-hours mode but still delivers the durable_job_row so the user has a path back to the canonical review object. Held receipts preserve the same reopen target ref as the delivered receipt.".into(),
            contract_sections: vec![
                "Privacy-class rules".into(),
                "Fanout receipts".into(),
                "Non-conforming cases".into(),
            ],
        },
        source: SourceMetadata {
            envelope_fixture_ref:
                "fixtures/ux/notification_envelope_cases/held_quiet_hours_companion_fanout.json"
                    .into(),
            emissions: 1,
        },
        expected_routed: routed,
        expected_surface_snapshot: snapshot,
    };
    assert_source_envelope_resolvable(&actual.source);
    assert_or_bless(
        "quiet_hours_holds_attention_but_delivers_durable.json",
        actual,
    );
}
