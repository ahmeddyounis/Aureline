//! Freeze gate for the M5 maintenance / failover / reconciliation window set.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-maintenance-windows/canonical_windows.json` is the published
//! set. This gate rebuilds the set in code and asserts it equals the fixture after
//! a serialize round-trip, so the maintenance, failover, migration, and
//! reconciliation contract cannot drift from the published artifact without
//! failing CI. It also re-proves support-export safety, surface binding, the
//! computed no-silent-green effective state, the named blocked write classes, the
//! local-safe / publish-later continuity, the changed-boundary disclosure, the
//! computed review-before-replay rule, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_maintenance_windows::{
    compute_replay_review_required, maintenance_window_lines, maintenance_window_set,
    MaintenanceWindowSet, M5_MAINTENANCE_WINDOWS_MATRIX_RECORD_KIND,
    M5_MAINTENANCE_WINDOWS_RECORD_KIND, M5_MAINTENANCE_WINDOWS_SCHEMA_REF,
};
use aureline_support::m5_operator_boards::{compute_effective_state, BlockerWaiverClass};
use aureline_support::m5_operator_surfaces::OperatorSurfaceClass;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-maintenance-windows/canonical_windows.json")
}

fn load_fixture() -> MaintenanceWindowSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = maintenance_window_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code maintenance-window set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_maintenance_windows`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_MAINTENANCE_WINDOWS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_MAINTENANCE_WINDOWS_SCHEMA_REF);
    assert_eq!(
        fixture.matrix_record_kind,
        M5_MAINTENANCE_WINDOWS_MATRIX_RECORD_KIND
    );
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: MaintenanceWindowSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn fixture_names_exact_times_and_blocked_writes() {
    let fixture = load_fixture();
    assert!(!fixture.windows.is_empty());
    for w in &fixture.windows {
        assert!(w.object_ref.starts_with("aureline://"));
        assert_eq!(w.open_detail_ref, w.object_ref);
        assert!(!w.window_time.time_zone.is_empty());
        assert!(!w.window_time.utc_offset.is_empty());
        assert!(!w.local_safe_actions.is_empty());
        if w.blocks_managed_writes() {
            assert!(!w.blocked_writes.is_empty());
            assert!(w.publish_later_available);
            for b in &w.blocked_writes {
                assert!(!b.local_alternative.is_empty());
            }
        }
    }
}

#[test]
fn fixture_effective_state_is_computed() {
    let fixture = load_fixture();
    for w in &fixture.windows {
        assert_eq!(
            w.effective_state,
            compute_effective_state(
                w.phase.matrix_state(),
                w.window_time.refresh_freshness,
                BlockerWaiverClass::None
            )
        );
    }
}

#[test]
fn fixture_failover_discloses_changed_boundaries_and_gates_replay() {
    let fixture = load_fixture();
    let mut saw_failover_change = false;
    for w in &fixture.windows {
        if w.surface == OperatorSurfaceClass::FailoverNotice {
            assert!(!w.boundary_disclosure.axes.is_empty());
        }
        assert_eq!(
            w.replay_review.required,
            compute_replay_review_required(
                w.queued_actions_present,
                w.boundary_disclosure.any_crossed()
            )
        );
        if w.boundary_disclosure.any_crossed() && w.queued_actions_present {
            saw_failover_change = true;
            assert!(w.replay_review.required);
            assert!(!w.replay_review.reconcile_action.is_empty());
        }
    }
    assert!(
        saw_failover_change,
        "fixture must exercise a changed boundary that gates replay"
    );
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = maintenance_window_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Maintenance & failover windows")));
    assert!(lines
        .iter()
        .any(|line| line.contains("review-before-replay")));
}
