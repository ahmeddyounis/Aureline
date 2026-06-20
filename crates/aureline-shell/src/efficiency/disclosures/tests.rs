//! Tests proving per-surface disclosures derive from the canonical objects, keep
//! protected paths visible, and never disagree with the snapshot they project.

use super::*;
use crate::efficiency::governance::M5_EFFICIENCY_GOVERNANCE_MATRIX_REF;
use crate::efficiency::surfaces::seeded_efficiency_state_snapshots;
use crate::efficiency::EfficiencyState;

#[test]
fn nominal_posture_discloses_nothing() {
    let disclosures = EfficiencySurfaceDisclosures::for_state(
        "ws:nominal",
        EfficiencyState::Nominal,
        &[EfficiencyPressureSource::AcPower],
        0,
        "2026-06-20T14:00:00Z",
    );
    assert!(!disclosures.has_disclosures());
    assert!(!disclosures.behavior_changed);
    // Every surface is listed as unaffected, so no permanent banner is shown.
    assert_eq!(
        disclosures.unaffected_surface_tokens.len(),
        DisclosureSurface::ALL.len()
    );
}

#[test]
fn every_pressured_posture_discloses_all_six_surfaces() {
    for case in seeded_efficiency_disclosure_cases() {
        let disclosures = &case.disclosures;
        assert!(
            disclosures.has_disclosures(),
            "{} disclosed nothing",
            case.case_id
        );
        // Under battery/thermal/policy/protect-core/recovery every named surface
        // changes behavior, so all six surfaces carry a disclosure.
        assert_eq!(
            disclosures.disclosures.len(),
            DisclosureSurface::ALL.len(),
            "{} did not cover every surface",
            case.case_id
        );
        assert!(disclosures.unaffected_surface_tokens.is_empty());
    }
}

#[test]
fn disclosures_keep_protected_paths_visible_and_are_not_toast_only() {
    for case in seeded_efficiency_disclosure_cases() {
        assert!(
            case.disclosures.preserves_protected_path_truth(),
            "{} narrowed a protected path or used toast-only truth",
            case.case_id
        );
        for disclosure in &case.disclosures.disclosures {
            // What still works and what is delayed are always both stated, so the
            // user never has to infer a surface is broken.
            assert!(!disclosure.still_works_now.is_empty());
            assert!(!disclosure.what_is_delayed.is_empty());
            assert!(disclosure.is_degraded_not_error);
            // The disclosure is durable inline truth, not a dismissible toast, and
            // never sits in the typing hot path.
            assert!(!disclosure.placement.toast_only);
            assert!(!disclosure.placement.in_typing_hot_path);
            assert!(disclosure.placement.persistent_while_active);
            // Inspect is always offered.
            assert_eq!(disclosure.inspect.command_id, EFFICIENCY_INSPECT_COMMAND_ID);
        }
        // The protected edit/search/save/review interactions are preserved.
        for protected in ["typing", "save", "local_navigation"] {
            assert!(case
                .disclosures
                .protected_interactions_preserved
                .contains(&protected.to_owned()));
        }
        assert!(case.disclosures.durability_preserved);
    }
}

#[test]
fn overrides_are_explicit_and_policy_aware() {
    let by_case = |case_id: &str| {
        seeded_efficiency_disclosure_cases()
            .into_iter()
            .find(|case| case.case_id == case_id)
            .unwrap_or_else(|| panic!("case {case_id} exists"))
    };

    // OS battery saver: user-controllable, so a session override is offered.
    let battery = by_case("battery-saver");
    let indexing = battery
        .disclosures
        .disclosure_for("paused_indexing")
        .expect("indexing disclosed");
    assert!(indexing.override_affordance.override_allowed);
    assert!(indexing.override_affordance.override_label.is_some());
    assert_eq!(
        indexing.override_affordance.posture,
        "user_override_session_only"
    );

    // Policy cap: never silently collapsed into "battery saver"; override blocked.
    let policy = by_case("policy-cap");
    let uploads = policy
        .disclosures
        .disclosure_for("optional_uploads")
        .expect("uploads disclosed");
    assert!(!uploads.override_affordance.override_allowed);
    assert_eq!(uploads.override_affordance.posture, "policy_blocked");
    assert!(uploads.override_affordance.policy_blocked_ref.is_some());

    // Critical-battery protect-core: not overridable.
    let critical = by_case("critical-battery");
    let ai = critical
        .disclosures
        .disclosure_for("ai_warmups")
        .expect("ai warmups disclosed");
    assert!(!ai.override_affordance.override_allowed);
    assert_eq!(ai.override_affordance.posture, "not_overridable");
}

#[test]
fn freshness_class_marks_degraded_not_error() {
    let recovery = seeded_efficiency_disclosure_cases()
        .into_iter()
        .find(|case| case.case_id == "recovery")
        .expect("recovery case exists");
    assert_eq!(recovery.disclosures.recovery_state, "staged_resume");
    for disclosure in &recovery.disclosures.disclosures {
        assert_eq!(disclosure.action, "staged_resume");
        assert_eq!(disclosure.freshness_class, "resuming");
    }
}

#[test]
fn disclosures_agree_with_the_canonical_snapshot_and_support_export() {
    // The disclosure set re-derived from a snapshot equals the seeded case for
    // the same workspace, and its action/visible-state tokens match the
    // snapshot's affected subsystems wherever the families overlap. That keeps
    // the disclosure copy and support/export packets consistent.
    let cases = seeded_efficiency_disclosure_cases();
    for snapshot in seeded_efficiency_state_snapshots() {
        let case = cases
            .iter()
            .find(|case| case.workspace_id == snapshot.workspace_id)
            .unwrap_or_else(|| panic!("disclosure case for {} exists", snapshot.workspace_id));

        let projected = EfficiencySurfaceDisclosures::from_snapshot(&snapshot);
        assert_eq!(
            projected, case.disclosures,
            "disclosures drifted from the snapshot for {}",
            snapshot.workspace_id
        );

        // Shared state, cause, override posture, and governance binding.
        assert_eq!(projected.active_state, snapshot.active_state);
        assert_eq!(projected.source_of_change, snapshot.pressure_sources);
        assert_eq!(projected.override_posture, snapshot.override_posture);
        assert_eq!(projected.recovery_state, snapshot.recovery_state);
        assert_eq!(
            projected.governance.matrix_ref,
            M5_EFFICIENCY_GOVERNANCE_MATRIX_REF
        );

        // Wherever a disclosed surface's governing family appears in the
        // snapshot, the disclosed action and visible state match exactly.
        for affected in &snapshot.affected_subsystems {
            for disclosure in &projected.disclosures {
                if disclosure.governing_subsystem_token == affected.subsystem_token {
                    assert_eq!(
                        disclosure.action, affected.action,
                        "action disagreed for {} in {}",
                        affected.subsystem_token, snapshot.workspace_id
                    );
                    assert_eq!(
                        disclosure.visible_state, affected.visible_state,
                        "visible state disagreed for {} in {}",
                        affected.subsystem_token, snapshot.workspace_id
                    );
                }
            }
        }
    }
}
