//! Unit tests for the maintenance-window builder: the operational-phase to
//! matrix-state mapping, the computed effective state, the blocked-write and
//! local-safe continuity truth, the changed-boundary disclosure, and the
//! computed review-before-replay rule.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = maintenance_window_set();
    set.validate()
        .expect("canonical maintenance-window set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(maintenance_window_set(), maintenance_window_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = maintenance_window_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_window_binds_a_canonical_matrix_surface() {
    let set = maintenance_window_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    for w in &set.windows {
        assert_eq!(w.surface, w.kind.surface());
        assert_eq!(w.surface_id, w.surface.surface_id());
        assert!(matrix.surface(w.surface).is_some());
        assert!(w.kind.permits_phase(w.phase));
    }
}

#[test]
fn every_operational_phase_is_exercised() {
    let set = maintenance_window_set();
    for phase in OperationalPhaseClass::ALL {
        assert!(
            set.windows.iter().any(|w| w.phase == phase),
            "fixture must exercise the {} phase",
            phase.as_str()
        );
    }
}

#[test]
fn both_matrix_surfaces_are_exercised() {
    let set = maintenance_window_set();
    assert!(set
        .windows
        .iter()
        .any(|w| w.surface == OperatorSurfaceClass::MaintenanceNotice));
    assert!(set
        .windows
        .iter()
        .any(|w| w.surface == OperatorSurfaceClass::FailoverNotice));
}

#[test]
fn phase_matrix_state_mapping_is_total() {
    use OperatorStateClass as S;
    assert_eq!(
        OperationalPhaseClass::Scheduled.matrix_state(),
        S::ScheduledWindow
    );
    assert_eq!(
        OperationalPhaseClass::ReadOnly.matrix_state(),
        S::ReadOnlyWindow
    );
    assert_eq!(OperationalPhaseClass::Drain.matrix_state(), S::DrainWindow);
    assert_eq!(
        OperationalPhaseClass::Migration.matrix_state(),
        S::MigrationInProgress
    );
    assert_eq!(
        OperationalPhaseClass::Failover.matrix_state(),
        S::FailoverInProgress
    );
    assert_eq!(
        OperationalPhaseClass::Reconciling.matrix_state(),
        S::Reconciling
    );
    assert_eq!(OperationalPhaseClass::Resolved.matrix_state(), S::Clear);
}

#[test]
fn effective_state_is_computed_for_every_window() {
    let set = maintenance_window_set();
    for w in &set.windows {
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
fn resolved_window_with_stale_refresh_downgrades_from_clear() {
    // A resolved window claims "clear"; a stale refresh must downgrade it to
    // unconfirmed so a stale all-clear never reads as a confirmed green.
    assert_eq!(
        compute_effective_state(
            OperationalPhaseClass::Resolved.matrix_state(),
            FreshnessClass::VeryStale,
            BlockerWaiverClass::None
        ),
        OperatorStateClass::Unconfirmed
    );
    // While the refresh is fresh, it stays clear.
    assert_eq!(
        compute_effective_state(
            OperationalPhaseClass::Resolved.matrix_state(),
            FreshnessClass::Fresh,
            BlockerWaiverClass::None
        ),
        OperatorStateClass::Clear
    );
}

#[test]
fn every_window_names_exact_times_and_zone() {
    let set = maintenance_window_set();
    for w in &set.windows {
        let t = &w.window_time;
        assert!(timestamp_carries_offset(&t.starts_at), "{}", w.window_id);
        assert!(timestamp_carries_offset(&t.ends_at), "{}", w.window_id);
        assert!(!t.time_zone.is_empty(), "{}", w.window_id);
        assert!(is_utc_offset(&t.utc_offset), "{}", w.window_id);
        assert!(
            offset_matches(&t.starts_at, &t.utc_offset),
            "{}",
            w.window_id
        );
        assert!(parse_rfc3339(&t.starts_at).unwrap() <= parse_rfc3339(&t.ends_at).unwrap());
    }
}

#[test]
fn blocking_windows_name_blocked_writes_with_local_alternatives() {
    let set = maintenance_window_set();
    let mut saw_blocking = false;
    for w in &set.windows {
        if w.blocks_managed_writes() {
            saw_blocking = true;
            assert!(!w.blocked_writes.is_empty(), "{}", w.window_id);
            assert!(w.publish_later_available, "{}", w.window_id);
            assert!(w.write_posture.blocks_live_writes(), "{}", w.window_id);
            for b in &w.blocked_writes {
                assert!(!b.local_alternative.is_empty(), "{}", w.window_id);
            }
        } else {
            assert_eq!(
                w.write_posture,
                WritePostureClass::WritesLive,
                "{}",
                w.window_id
            );
        }
    }
    assert!(saw_blocking, "fixture must exercise a blocking window");
}

#[test]
fn every_window_keeps_local_work() {
    let set = maintenance_window_set();
    for w in &set.windows {
        assert!(!w.local_safe_actions.is_empty(), "{}", w.window_id);
    }
}

#[test]
fn failover_and_migration_disclose_changed_boundaries() {
    let set = maintenance_window_set();
    let mut saw_changed = false;
    for w in &set.windows {
        if w.surface == OperatorSurfaceClass::FailoverNotice {
            assert!(!w.boundary_disclosure.axes.is_empty(), "{}", w.window_id);
        }
        assert_eq!(
            w.boundary_disclosure.recheck_required,
            w.boundary_disclosure.any_crossed(),
            "{}",
            w.window_id
        );
        for a in &w.boundary_disclosure.axes {
            if a.state.crossed() {
                saw_changed = true;
                assert!(!a.disclosure.is_empty(), "{}", w.window_id);
            }
        }
    }
    assert!(saw_changed, "fixture must exercise a changed boundary");
}

#[test]
fn replay_review_is_computed_for_every_window() {
    let set = maintenance_window_set();
    for w in &set.windows {
        assert_eq!(
            w.replay_review.required,
            compute_replay_review_required(
                w.queued_actions_present,
                w.boundary_disclosure.any_crossed()
            ),
            "{}",
            w.window_id
        );
        if w.replay_review.required {
            assert!(w.replay_review.trigger.requires_review(), "{}", w.window_id);
            assert!(
                !w.replay_review.reconcile_action.is_empty(),
                "{}",
                w.window_id
            );
        } else {
            assert!(
                !w.replay_review.trigger.requires_review(),
                "{}",
                w.window_id
            );
        }
    }
}

#[test]
fn queued_writes_against_unchanged_boundary_need_no_review() {
    // The read-only window has queued actions but an unchanged endpoint, so its
    // queue replays without review.
    let set = maintenance_window_set();
    let read_only = set
        .window("maintenance_window.0002")
        .expect("read-only window present");
    assert!(read_only.queued_actions_present);
    assert!(!read_only.boundary_disclosure.any_crossed());
    assert!(!read_only.replay_review.required);
}

#[test]
fn queued_writes_across_changed_boundary_require_review() {
    let set = maintenance_window_set();
    let failover = set
        .window("maintenance_window.0004")
        .expect("failover window present");
    assert!(failover.queued_actions_present);
    assert!(failover.boundary_disclosure.any_crossed());
    assert!(failover.replay_review.required);
    assert_eq!(
        failover.replay_review.trigger,
        ReplayReviewTriggerClass::ChangedRegion
    );
}

#[test]
fn compute_replay_review_required_truth_table() {
    assert!(!compute_replay_review_required(false, false));
    assert!(!compute_replay_review_required(false, true));
    assert!(!compute_replay_review_required(true, false));
    assert!(compute_replay_review_required(true, true));
}

#[test]
fn every_window_is_distinguishable_from_a_generic_outage() {
    let set = maintenance_window_set();
    for w in &set.windows {
        assert!(w.distinguishable_from_outage, "{}", w.window_id);
        assert!(!w.outage_distinction.is_empty(), "{}", w.window_id);
    }
}

#[test]
fn window_ids_are_unique() {
    let set = maintenance_window_set();
    let mut seen = std::collections::BTreeSet::new();
    for w in &set.windows {
        assert!(
            seen.insert(w.window_id.clone()),
            "duplicate {}",
            w.window_id
        );
    }
}

#[test]
fn offset_matches_treats_z_as_utc() {
    assert!(offset_matches("2026-06-22T01:00:00Z", "+00:00"));
    assert!(!offset_matches("2026-06-22T01:00:00Z", "-04:00"));
    assert!(offset_matches("2026-06-23T02:00:00-04:00", "-04:00"));
    assert!(!offset_matches("2026-06-23T02:00:00-04:00", "+00:00"));
}

#[test]
fn projection_renders_for_support() {
    let set = maintenance_window_set();
    let lines = maintenance_window_lines(&set);
    assert!(lines
        .iter()
        .any(|l| l.contains("Maintenance & failover windows")));
    assert!(lines.iter().any(|l| l.contains("review-before-replay")));
    assert!(lines.iter().any(|l| l.contains("boundary:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
}

#[test]
fn round_trips_through_json() {
    let set = maintenance_window_set();
    let json = serde_json::to_string(&set).expect("serialize");
    let back: MaintenanceWindowSet = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(set, back);
}
