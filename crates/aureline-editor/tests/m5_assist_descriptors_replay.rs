//! Freeze gate for the canonical assist-descriptor model.
//!
//! The checked-in fixture
//! `fixtures/editor/m5-assist-descriptors/canonical_model.json` is the published
//! model. This gate rebuilds the model in code and asserts it equals the fixture
//! after a serialize round-trip, so the in-code model cannot drift from the
//! published artifact without failing CI. It also re-proves every frozen
//! invariant, support-export safety, full per-class catalog coverage, and the
//! cross-cutting precedence / suppression / accessibility contracts.

use std::path::{Path, PathBuf};

use aureline_editor::{
    assist_descriptor_model, assist_descriptor_model_lines, AssistDescriptorFamily,
    AssistDescriptorModel, SuppressionReason, TruthTier, VisibilityVerdict,
    M5_ASSIST_DESCRIPTORS_RECORD_KIND, M5_ASSIST_DESCRIPTORS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-assist-descriptors/canonical_model.json")
}

fn load_fixture() -> AssistDescriptorModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = assist_descriptor_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code assist-descriptor model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_assist_descriptors`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ASSIST_DESCRIPTORS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ASSIST_DESCRIPTORS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: AssistDescriptorModel =
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
fn editing_truth_is_never_suppressed_in_any_scenario() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for resolved in scenario
            .resolved
            .iter()
            .filter(|r| r.truth_tier == TruthTier::EditingTruth)
        {
            assert!(
                matches!(
                    resolved.visibility,
                    VisibilityVerdict::Rendered | VisibilityVerdict::Downgraded
                ),
                "{}::{} editing truth must never be suppressed or deferred",
                scenario.context.scenario_id,
                resolved.descriptor_id
            );
        }
    }
}

#[test]
fn every_non_rendered_resolution_explains_itself() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for resolved in &scenario.resolved {
            if resolved.visibility != VisibilityVerdict::Rendered {
                assert_ne!(
                    resolved.suppression_reason,
                    SuppressionReason::NotSuppressed,
                    "{}::{} must carry a reason",
                    scenario.context.scenario_id,
                    resolved.descriptor_id
                );
                assert!(
                    !resolved.reason_detail.trim().is_empty(),
                    "{}::{} must carry reason detail",
                    scenario.context.scenario_id,
                    resolved.descriptor_id
                );
            }
        }
    }
}

#[test]
fn actionable_and_severity_decorations_are_accessible() {
    let fixture = load_fixture();
    for descriptor in fixture
        .descriptor_catalog
        .iter()
        .filter(|d| d.family == AssistDescriptorFamily::Decoration)
        .filter(|d| d.actionability.requires_keyboard_path())
    {
        assert!(
            descriptor.accessibility.keyboard_path.is_some(),
            "{} must declare a keyboard path",
            descriptor.descriptor_id
        );
        assert!(
            !descriptor
                .accessibility
                .screen_reader_label
                .trim()
                .is_empty(),
            "{} must declare a screen-reader label",
            descriptor.descriptor_id
        );
        assert!(
            !descriptor
                .accessibility
                .non_color_differentiator
                .trim()
                .is_empty(),
            "{} must declare a non-color differentiator",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn precedence_conflicts_are_resolved_to_editing_truth() {
    let fixture = load_fixture();
    assert!(!fixture.precedence_conflicts.is_empty());
    for case in &fixture.precedence_conflicts {
        assert_eq!(case.winner_descriptor_id, case.editing_truth_descriptor_id);
        assert_eq!(case.yielded_descriptor_id, case.convenience_descriptor_id);
        assert_eq!(case.yielded_visibility, VisibilityVerdict::Deferred);
        assert_eq!(
            case.yielded_reason,
            SuppressionReason::OutrankedByEditingTruth
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = assist_descriptor_model_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Assist-descriptor model")));
    assert!(lines.iter().any(|line| line.contains("Scenarios:")));
    for scenario in &fixture.scenarios {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(&scenario.context.scenario_id)),
            "projection must mention scenario {}",
            scenario.context.scenario_id
        );
    }
}
