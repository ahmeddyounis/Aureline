//! Fixture-replay for the maintenance & failover continuity-notice drill
//! corpus. The drills live under
//! `fixtures/ops/m3/maintenance_and_failover_notices/` and are minted by the
//! `aureline_shell_continuity_notices_corpus` emitter so the checked-in JSON
//! stays a literal projection of the in-code corpus.
//!
//! What this guards:
//!
//! - Each scenario's view on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_continuity_notices_corpus -- emit-fixtures \
//!     fixtures/ops/m3/maintenance_and_failover_notices
//!   ```
//!
//! - The pinned `category`, `effective_freshness`, `honesty_marker_present`,
//!   preserved-intent count, changed-axis count, and boundary-unresolved value
//!   per scenario match the surface's render.
//! - The no-silent-current ban holds: a notice is `current` only while it is
//!   declared active and its last refresh is current; otherwise it downgrades,
//!   names a reason, lights the honesty marker, and carries a stale label.
//! - Queued publish-later / local-draft work carries a canonical queue ref and
//!   is marked preserved, kept separate from successful hosted mutations.
//! - A changed / unknown boundary axis carries a canonical current ref and stays
//!   visible; the display-copy "no lie" invariants all stay false.

use aureline_shell::continuity_notices::continuity_notice_corpus;
use aureline_shell::continuity_notices::model::{
    is_canonical_object_ref, BoundaryAxisStateClass, ContinuityNoticeView, EffectiveFreshnessClass,
    WriteContinuityPostureClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ops/m3/maintenance_and_failover_notices",
);

fn load_view(filename: &str) -> ContinuityNoticeView {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in continuity_notice_corpus() {
        let on_disk = load_view(scenario.fixture_filename);
        let in_code = scenario.view();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_continuity_notices_corpus -- emit-fixtures fixtures/ops/m3/maintenance_and_failover_notices`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        assert_eq!(
            view.category, scenario.expected_category,
            "{} category",
            scenario.scenario_id,
        );
        assert_eq!(
            view.effective_freshness, scenario.expected_effective_freshness,
            "{} effective_freshness",
            scenario.scenario_id,
        );
        assert_eq!(
            view.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id,
        );
        assert_eq!(
            view.summary_counts.preserved_intent_count, scenario.expected_preserved_intent_count,
            "{} preserved_intent_count",
            scenario.scenario_id,
        );
        assert_eq!(
            view.summary_counts.changed_boundary_axis_count,
            scenario.expected_changed_boundary_axis_count,
            "{} changed_boundary_axis_count",
            scenario.scenario_id,
        );
        assert_eq!(
            view.boundary_change_unresolved, scenario.expected_boundary_change_unresolved,
            "{} boundary_change_unresolved",
            scenario.scenario_id,
        );
    }
}

#[test]
fn no_notice_reads_as_current_when_downgraded() {
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        if view.effective_freshness == EffectiveFreshnessClass::Current {
            assert!(
                !view.freshness_downgraded,
                "{}: current notice cannot be downgraded",
                scenario.scenario_id,
            );
            assert!(
                view.downgrade_reasons.is_empty(),
                "{}: current notice cannot carry downgrade reasons",
                scenario.scenario_id,
            );
            assert_eq!(
                view.lifecycle.freshness_class.as_str(),
                "active_current",
                "{}: only an active_current lifecycle may read as current",
                scenario.scenario_id,
            );
            assert!(
                view.display_copy.stale_label.is_none(),
                "{}: current notice cannot carry a stale label",
                scenario.scenario_id,
            );
        } else {
            assert!(
                view.freshness_downgraded,
                "{}: non-current notice must be marked downgraded",
                scenario.scenario_id,
            );
            assert!(
                !view.downgrade_reasons.is_empty(),
                "{}: downgraded notice must name a reason",
                scenario.scenario_id,
            );
            assert!(
                view.honesty_marker_present,
                "{}: downgraded notice must light the honesty marker",
                scenario.scenario_id,
            );
            assert!(
                view.display_copy.stale_label.is_some(),
                "{}: downgraded notice must carry a stale label",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn queued_work_is_preserved_and_separated_from_hosted_mutations() {
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        // No hosted mutation may also appear as a blocked/queued write.
        let succeeded_actions: std::collections::BTreeSet<_> = view
            .succeeded_hosted_mutations
            .iter()
            .map(|m| m.action_class)
            .collect();
        let mut preserved = 0u32;
        for w in &view.blocked_writes {
            if w.continuity_posture.is_preserved() {
                preserved += 1;
                assert!(
                    w.intent_preserved,
                    "{}: preserved posture must mark intent_preserved ({})",
                    scenario.scenario_id,
                    w.action_class.as_str(),
                );
                let queue_ref = w.queue_or_intent_ref.as_deref().unwrap_or_else(|| {
                    panic!(
                        "{}: preserved write needs a queue ref",
                        scenario.scenario_id
                    )
                });
                assert!(
                    is_canonical_object_ref(queue_ref),
                    "{}: queue ref {queue_ref:?} is not canonical",
                    scenario.scenario_id,
                );
            }
            assert!(
                !succeeded_actions.contains(&w.action_class),
                "{}: action {} appears both queued/blocked and succeeded",
                scenario.scenario_id,
                w.action_class.as_str(),
            );
        }
        assert_eq!(
            preserved, view.summary_counts.preserved_intent_count,
            "{}: preserved-intent count must match the preserved rows",
            scenario.scenario_id,
        );
        for m in &view.succeeded_hosted_mutations {
            assert!(
                is_canonical_object_ref(&m.result_ref),
                "{}: hosted mutation result_ref {:?} is not canonical",
                scenario.scenario_id,
                m.result_ref,
            );
        }
    }
}

#[test]
fn changed_boundary_is_visible_and_canonical() {
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        let mut changed = 0u32;
        for axis in &view.boundary_change.axes {
            if axis.axis_state_class.is_meaningful_change() {
                let current = axis.current_ref.as_deref().unwrap_or_else(|| {
                    panic!("{}: changed axis needs a current_ref", scenario.scenario_id)
                });
                assert!(
                    is_canonical_object_ref(current),
                    "{}: changed axis current_ref {current:?} is not canonical",
                    scenario.scenario_id,
                );
                if axis.axis_state_class == BoundaryAxisStateClass::Changed {
                    changed += 1;
                }
            }
        }
        assert_eq!(
            changed, view.summary_counts.changed_boundary_axis_count,
            "{}: changed-axis count drift",
            scenario.scenario_id,
        );
        if view.boundary_change.boundary_change_required {
            assert!(
                !view.display_copy.boundary_change_hidden,
                "{}: a required boundary change cannot be hidden",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn refs_and_no_lie_invariants_hold() {
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        assert!(
            is_canonical_object_ref(&view.history_ref),
            "{}: history_ref is not canonical",
            scenario.scenario_id,
        );
        assert!(
            is_canonical_object_ref(&view.support_export_ref),
            "{}: support_export_ref is not canonical",
            scenario.scenario_id,
        );
        let dc = &view.display_copy;
        assert!(
            !dc.all_work_broken_implied,
            "{}: all_work_broken_implied",
            scenario.scenario_id
        );
        assert!(
            !dc.incident_language_for_planned_used,
            "{}: incident_language_for_planned_used",
            scenario.scenario_id,
        );
        assert!(
            !dc.generic_degraded_banner_used,
            "{}: generic_degraded_banner_used",
            scenario.scenario_id
        );
        assert!(
            !dc.queued_and_succeeded_collapsed,
            "{}: queued_and_succeeded_collapsed",
            scenario.scenario_id,
        );
        assert!(
            !dc.stale_presented_as_current,
            "{}: stale_presented_as_current",
            scenario.scenario_id
        );
        assert!(
            !dc.boundary_change_hidden,
            "{}: boundary_change_hidden",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_every_category_and_freshness_and_posture() {
    let mut categories = std::collections::BTreeSet::new();
    let mut effective = std::collections::BTreeSet::new();
    let mut postures = std::collections::BTreeSet::new();
    let mut downgrades = std::collections::BTreeSet::new();
    let mut axis_states = std::collections::BTreeSet::new();
    for scenario in continuity_notice_corpus() {
        let view = load_view(scenario.fixture_filename);
        categories.insert(view.category.as_str());
        effective.insert(view.effective_freshness.as_str());
        for w in &view.blocked_writes {
            postures.insert(w.continuity_posture.as_str());
        }
        for r in &view.downgrade_reasons {
            downgrades.insert(r.as_str());
        }
        for a in &view.boundary_change.axes {
            axis_states.insert(a.axis_state_class.as_str());
        }
    }
    for expected in ["maintenance", "drain", "failover", "tenant_migration"] {
        assert!(
            categories.contains(expected),
            "category {expected} not exercised"
        );
    }
    for expected in [
        "current",
        "refresh_stale",
        "superseded_stale",
        "completed_historical",
        "imported_historical",
    ] {
        assert!(
            effective.contains(expected),
            "effective freshness {expected} not exercised"
        );
    }
    for expected in [
        WriteContinuityPostureClass::QueuedPublishLater.as_str(),
        WriteContinuityPostureClass::LocalDraftPreserved.as_str(),
        WriteContinuityPostureClass::RetryableWhenConnected.as_str(),
        WriteContinuityPostureClass::DrainingExistingOnly.as_str(),
        WriteContinuityPostureClass::BlockedPendingReconnect.as_str(),
        WriteContinuityPostureClass::BlockedPendingBoundaryRecheck.as_str(),
        WriteContinuityPostureClass::BlockedNoSafeRetry.as_str(),
        WriteContinuityPostureClass::RequiresManualRerun.as_str(),
    ] {
        assert!(
            postures.contains(expected),
            "write posture {expected} not exercised"
        );
    }
    for expected in [
        "refresh_expired",
        "notice_superseded",
        "window_completed",
        "imported_offline",
    ] {
        assert!(
            downgrades.contains(expected),
            "downgrade reason {expected} not exercised"
        );
    }
    for expected in [
        "unchanged",
        "changed",
        "unknown_recheck_required",
        "not_applicable",
    ] {
        assert!(
            axis_states.contains(expected),
            "boundary axis state {expected} not exercised"
        );
    }
}

#[test]
fn fixture_directory_has_no_unexpected_files() {
    let scenario_files: std::collections::BTreeSet<String> = continuity_notice_corpus()
        .iter()
        .map(|s| s.fixture_filename.to_owned())
        .collect();
    let mut on_disk = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(FIXTURE_DIR).expect("read fixture dir") {
        let entry = entry.expect("dir entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            on_disk.insert(name);
        }
    }
    assert_eq!(
        scenario_files, on_disk,
        "fixture directory drifted from the corpus; re-emit fixtures",
    );
}
