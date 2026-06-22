//! Freeze gate for the M5 response-pane set.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-response-panes/canonical_response_panes.json` is the published
//! set. This gate rebuilds the set in code and asserts it equals the fixture after
//! a serialize round-trip, so the service-ownership, runbook-response, and
//! continuity contract cannot drift from the published artifact without failing
//! CI. It also re-proves support-export safety, surface binding, the computed
//! no-silent-green strip state, the mutating-step preview/approval admission, the
//! local-outage continuity truth, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_response_panes::{
    compute_step_execution, response_pane_lines, response_pane_set, ResponsePaneSet,
    StepExecutionClass, M5_RESPONSE_PANES_MATRIX_RECORD_KIND, M5_RESPONSE_PANES_RECORD_KIND,
    M5_RESPONSE_PANES_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-response-panes/canonical_response_panes.json")
}

fn load_fixture() -> ResponsePaneSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = response_pane_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code response-pane set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_response_panes`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_RESPONSE_PANES_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_RESPONSE_PANES_SCHEMA_REF);
    assert_eq!(
        fixture.matrix_record_kind,
        M5_RESPONSE_PANES_MATRIX_RECORD_KIND
    );
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ResponsePaneSet =
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
fn fixture_has_owners_and_oncall_on_every_strip() {
    let fixture = load_fixture();
    assert!(!fixture.service_strips.is_empty());
    for strip in &fixture.service_strips {
        assert!(strip.object_ref.starts_with("aureline://"));
        assert_eq!(strip.open_detail_ref, strip.object_ref);
        assert!(!strip.primary_owner.is_empty());
        assert!(!strip.on_call_lane.is_empty());
        assert!(!strip.decision_right.is_empty());
        assert!(!strip.escalation.routes_to_ref.is_empty());
    }
}

#[test]
fn fixture_mutating_steps_are_gated() {
    let fixture = load_fixture();
    let mut saw_mutating = false;
    for pane in &fixture.runbook_panes {
        for step in &pane.steps {
            assert_eq!(
                step.execution,
                compute_step_execution(
                    step.intent,
                    step.boundary,
                    step.approval_gate,
                    step.approval_state,
                    step.boundary_state,
                    step.live_target_present,
                )
            );
            if step.intent.is_mutating() {
                saw_mutating = true;
                assert_ne!(step.execution, StepExecutionClass::RunLocal);
                assert!(step.dry_run_available);
                assert!(!step.rollback_note.is_empty());
            }
        }
    }
    assert!(saw_mutating, "fixture must exercise mutating steps");
}

#[test]
fn fixture_continuity_keeps_local_work() {
    let fixture = load_fixture();
    assert!(!fixture.continuity_views.is_empty());
    for view in &fixture.continuity_views {
        assert!(view.object_ref.starts_with("aureline://"));
        assert!(!view.local_capabilities.is_empty());
        if view.blocks_managed_writes() {
            assert!(view.publish_later_capture);
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = response_pane_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator response panes")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Local-outage continuity views")));
}
