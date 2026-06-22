//! Freeze gate for the M5 action-plan set.
//!
//! The checked-in fixture `fixtures/ops/m5-action-plans/canonical_action_plans.json`
//! is the published action-plan set. This gate rebuilds the set in code and asserts
//! it equals the fixture after a serialize round-trip, so the action-plan contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full plan coverage, canonical object linkage, the
//! controlled item vocabulary, the no-implicit-external-resolution rule, the
//! scope/boundary truth, progress and handoff parity, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_action_plans::{
    action_plan_lines, action_plan_set, export_plan, ActionPlanSet, ExternalMutationState,
    ItemLocalState, PlanClass, PlanItemClass, SharePosture, M5_ACTION_PLANS_MATRIX_REF,
    M5_ACTION_PLANS_RECORD_KIND, M5_ACTION_PLANS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-action-plans/canonical_action_plans.json")
}

fn load_fixture() -> ActionPlanSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = action_plan_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code action-plan set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_action_plans`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ACTION_PLANS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ACTION_PLANS_SCHEMA_REF);
    assert_eq!(fixture.matrix_ref, M5_ACTION_PLANS_MATRIX_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ActionPlanSet =
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
fn set_covers_every_plan() {
    let fixture = load_fixture();
    assert_eq!(fixture.plans.len(), PlanClass::ALL.len());
    for plan in PlanClass::ALL {
        let entry = fixture.plan(plan).expect("plan present");
        assert!(!entry.items.is_empty());
        assert_eq!(entry.surface_id, plan.surface().surface_id());
    }
}

#[test]
fn fixture_proves_the_controlled_vocabulary() {
    let fixture = load_fixture();
    let items: Vec<_> = fixture.plans.iter().flat_map(|p| p.items.iter()).collect();
    for class in PlanItemClass::ALL {
        assert!(items.iter().any(|i| i.item_class == class));
    }
    for state in ExternalMutationState::ALL {
        assert!(items.iter().any(|i| i.external_mutation_state == state));
    }
    for local in ItemLocalState::ALL {
        assert!(items.iter().any(|i| i.local_state == local));
    }
    for posture in SharePosture::ALL {
        assert!(fixture.plans.iter().any(|p| p.share_posture == posture));
    }
}

#[test]
fn fixture_local_checkoff_never_resolves_external() {
    let fixture = load_fixture();
    let items: Vec<_> = fixture.plans.iter().flat_map(|p| p.items.iter()).collect();
    for item in &items {
        if item.resolves_external_object {
            assert_eq!(
                item.external_mutation_state,
                ExternalMutationState::ExecutedConfirmed
            );
        }
    }
    // A locally-done item leaves its external object unresolved.
    assert!(items.iter().any(|i| {
        i.local_state == ItemLocalState::DoneLocal
            && i.external_link.is_external()
            && !i.resolves_external_object
    }));
    // At least one plan checks off more locally than it resolves externally.
    assert!(fixture
        .plans
        .iter()
        .any(|p| p.progress.done_local > p.progress.externally_resolved));
}

#[test]
fn fixture_handoff_parity_holds() {
    let fixture = load_fixture();
    for plan in &fixture.plans {
        let exported = export_plan(plan);
        assert_eq!(
            exported,
            plan.handoff,
            "{} frozen handoff must equal re-exporting it",
            plan.plan.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = action_plan_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator action plans")));
    for plan in PlanClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(plan.as_str())),
            "projection must mention plan {}",
            plan.as_str()
        );
    }
}
