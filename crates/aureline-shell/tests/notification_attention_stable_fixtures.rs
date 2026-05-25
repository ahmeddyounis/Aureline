//! Fixture-replay and invariant tests for the stable durable-attention lock
//! corpus.
//!
//! The records live under
//! `fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/`
//! and are minted by the `aureline_shell_notification_attention_stable` emitter
//! so the checked-in JSON stays a literal projection of the in-code corpus,
//! which is itself a projection of the live attention router.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_notification_attention_stable -- emit-fixtures \
//!     fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet
//!   ```
//!
//! - No launch-critical row relies on toast-only truth; every row keeps a
//!   durable surface and a durable job row.
//! - Quiet-hours / admin suppression preserves the durable object, the reopen
//!   target, and the audit trail across in-app, OS, and companion surfaces.
//! - The OS / lock-screen alert is summary-first and privacy-safe by default.
//! - Badge counts derive from durable item state, not raw fanout.
//! - Acknowledge, resolve, dismiss, snooze, and mute are distinct transitions.
//! - Exact-target reopen returns to the authoritative object or a truthful
//!   placeholder, never a generic home, and never re-issues a side effect.
//! - No row over-claims: each pillar of the claim ceiling is bound to the real
//!   evidence; a row missing a pillar or on a below-Stable surface is narrowed
//!   below Stable with a named reason.
//! - The same item opens from the activity center, command palette, status bar,
//!   and a menu command, keyboard-first.
//! - Tab order, narration (which discloses the subsystem), action labels, and
//!   recovery affordances stay reachable in normal, high-contrast, and zoomed
//!   layouts.
//! - Every row stays available without an account or managed services.

use aureline_shell::notification_attention_stable::{
    attention_lock_corpus, is_canonical_object_ref, required_recovery_actions, AttentionLockRecord,
    AttentionRouteSurface, LayoutMode, StableClaimClass, REQUIRED_LIFECYCLE_VERBS,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet",
);

fn load_record(filename: &str) -> AttentionLockRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in attention_lock_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_notification_attention_stable -- emit-fixtures fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.attention_class, scenario.expected_attention_class,
            "{} attention_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.claim_class, scenario.expected_claim_class,
            "{} claim_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} qualifies_stable",
            scenario.scenario_id
        );
        assert_eq!(
            record.surface_lifecycle_marker, scenario.expected_surface_marker,
            "{} surface marker",
            scenario.scenario_id
        );
        assert_eq!(
            record.claim_ceiling.asserts_durable_attention, scenario.expected_durable_attention,
            "{} durable attention",
            scenario.scenario_id
        );
    }
}

#[test]
fn no_launch_critical_row_is_toast_only() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.interruptibility.no_toast_only_truth,
            "{} relies on toast-only truth",
            scenario.scenario_id,
        );
        assert!(
            record.durable_job.durable_surface_present,
            "{} has no durable surface",
            scenario.scenario_id,
        );
        assert!(
            record.durable_job.is_durable(),
            "{} durable job does not survive look-away / sleep-resume",
            scenario.scenario_id,
        );
    }
}

#[test]
fn quiet_hours_preserves_durable_truth() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.quiet_hours.suppression_preserves_durable_object,
            "{} suppression erased the durable object",
            scenario.scenario_id,
        );
        assert!(
            record.quiet_hours.suppression_preserves_reopen_target,
            "{} suppression erased the reopen target",
            scenario.scenario_id,
        );
        assert!(
            record.quiet_hours.suppression_audit_trail_present,
            "{} suppression has no audit trail",
            scenario.scenario_id,
        );
        assert!(
            record.quiet_hours.is_coherent(),
            "{} quiet-hours policy is not coherent",
            scenario.scenario_id,
        );
    }
}

#[test]
fn os_alerts_are_privacy_safe_by_default() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.privacy.lock_screen_safe_by_default,
            "{} lock-screen alert is not safe by default",
            scenario.scenario_id,
        );
        assert!(
            record.privacy.summary_first,
            "{} OS alert is not summary-first",
            scenario.scenario_id,
        );
        assert!(
            !record.privacy.exposes_restricted_detail,
            "{} OS alert exposes restricted detail",
            scenario.scenario_id,
        );
        assert!(
            record.privacy.is_privacy_safe(),
            "{} OS alert is not privacy-safe",
            scenario.scenario_id,
        );
    }
}

#[test]
fn badges_derive_from_durable_item_state() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.badge.derived_from_durable_item_state,
            "{} badge does not derive from durable item state",
            scenario.scenario_id,
        );
        assert!(
            record.badge.count_class_truthful(),
            "{} badge count is not class-truthful",
            scenario.scenario_id,
        );
        // The badge must never outpace the durable model: a multi-count badge
        // requires a present durable surface.
        if record.badge.active_count > 1 {
            assert!(
                record.durable_job.durable_surface_present,
                "{} badge outpaces the durable model",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn lifecycle_verbs_are_distinct() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for verb in REQUIRED_LIFECYCLE_VERBS {
            assert!(
                record
                    .lifecycle
                    .available_actions
                    .iter()
                    .any(|action| action.action_kind == verb),
                "{} missing lifecycle verb {}",
                scenario.scenario_id,
                verb.as_str(),
            );
        }
        assert!(
            record.lifecycle.verbs_distinct,
            "{} lifecycle verbs are not distinct",
            scenario.scenario_id,
        );
    }
}

#[test]
fn reopen_is_deterministic_and_side_effect_free() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.reopen.resolves_to_exact_target
                || record.reopen.degrades_to_truthful_placeholder,
            "{} reopen is neither exact nor a truthful placeholder",
            scenario.scenario_id,
        );
        assert!(
            record.reopen.no_generic_home_reopen,
            "{} reopen could land on a generic home",
            scenario.scenario_id,
        );
        assert!(
            record.reopen.all_routes_preserve_reopen_target,
            "{} routes diverge on the reopen target",
            scenario.scenario_id,
        );
        assert!(
            record.reopen.no_side_effects_from_notification_surface,
            "{} could re-issue a side effect from a notification surface",
            scenario.scenario_id,
        );
        assert!(
            record.reopen.is_deterministic(),
            "{} reopen is not deterministic",
            scenario.scenario_id,
        );
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        if ceiling.asserts_durable_attention {
            assert!(
                record.durable_job.is_durable() && record.interruptibility.holds(),
                "{} claims durable attention it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_quiet_hours_coherent {
            assert!(
                record.quiet_hours.is_coherent(),
                "{} claims coherent quiet hours it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_privacy_safe {
            assert!(
                record.privacy.is_privacy_safe(),
                "{} claims a privacy-safe alert it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_badge_count_class_truthful {
            assert!(
                record.badge.count_class_truthful(),
                "{} claims class-truthful badge counts it cannot prove",
                scenario.scenario_id,
            );
        }
        if ceiling.asserts_exact_target_reopen {
            assert!(
                record.reopen.is_deterministic(),
                "{} claims deterministic reopen it cannot prove",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = attention_lock_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable row");
    assert!(narrowed >= 1, "matrix must include a narrowed row");
}

#[test]
fn narrowed_rows_drop_below_cutline_and_name_a_reason() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let qualification = &record.stable_qualification;
        if qualification.claim_class == StableClaimClass::Stable {
            assert!(
                qualification.qualifies_stable && qualification.narrowing_reasons.is_empty(),
                "{} Stable row carries a narrowing reason",
                scenario.scenario_id,
            );
        } else {
            assert!(
                !qualification.qualifies_stable,
                "{} narrowed row still qualifies",
                scenario.scenario_id,
            );
            assert!(
                !qualification.claim_class.at_or_above_cutline(),
                "{} narrowed claim sits at or above the cutline",
                scenario.scenario_id,
            );
            assert!(
                !qualification.narrowing_reasons.is_empty(),
                "{} narrowed without a named reason",
                scenario.scenario_id,
            );
            assert!(
                record.honesty_marker_present,
                "{} hides the honesty marker despite being narrowed",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(
            record.durable_job.cancelable,
            record.durable_job.retriable,
            record.durable_job.resolvable,
        ) {
            assert!(
                route_ids.contains(&required.as_str()),
                "{} missing recovery route {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for route in &record.recovery_routes {
            assert!(
                route.keyboard_reachable,
                "{} recovery route {} not keyboard reachable",
                scenario.scenario_id,
                route.action_id,
            );
        }
    }
}

#[test]
fn surfaces_share_one_model() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.surfaces.parity_holds,
            "{} surfaces disagree",
            scenario.scenario_id,
        );
        let route_ids: Vec<String> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.clone())
            .collect();
        assert_eq!(
            record.surfaces.recovery_action_ids, route_ids,
            "{} surface recovery ids drift from recovery routes",
            scenario.scenario_id,
        );
        for required in ["os_notification", "companion_push", "support_export"] {
            assert!(
                record
                    .surfaces
                    .reopen_surfaces
                    .iter()
                    .any(|surface| surface == required),
                "{} dropped reopen surface {}",
                scenario.scenario_id,
                required,
            );
        }
    }
}

#[test]
fn routes_reach_every_surface_keyboard_first() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AttentionRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|route| route.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing route surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable,
                "{} route {} not keyboard reachable",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                route.activates_same_item,
                "{} route {} activates a different item",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                is_canonical_object_ref(&route.route_ref),
                "{} route {} ref {:?} not canonical",
                scenario.scenario_id,
                required.as_str(),
                route.route_ref,
            );
        }
    }
}

#[test]
fn accessibility_holds_in_every_layout() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action labels drift from recovery routes",
            scenario.scenario_id,
        );
        for (label, route) in record
            .accessibility
            .action_labels
            .iter()
            .zip(record.recovery_routes.iter())
        {
            assert_eq!(
                label, &route.action_label,
                "{} action label drift",
                scenario.scenario_id
            );
        }
        for required in LayoutMode::REQUIRED {
            let mode = record
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                mode.row_narration_available && mode.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn rows_stay_available_without_account_or_managed_services() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.available_without_account,
            "{} hidden without an account",
            scenario.scenario_id,
        );
        assert!(
            record.available_without_managed_services,
            "{} hidden without managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn minted_refs_are_canonical_durable_objects() {
    for scenario in attention_lock_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for (label, value) in [
            ("durable_job.durable_object_ref", &record.durable_job.durable_object_ref),
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
        ] {
            assert!(
                is_canonical_object_ref(value),
                "{} {label} {value:?} not canonical",
                scenario.scenario_id,
            );
        }
        for value in record
            .evidence_refs
            .iter()
            .chain(record.narrative_refs.iter())
        {
            assert!(
                is_canonical_object_ref(value),
                "{} ref {value:?} not canonical",
                scenario.scenario_id,
            );
        }
    }
}
