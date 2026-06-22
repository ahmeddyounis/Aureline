//! Freeze gate for the canonical advanced-editing micro-surface model.
//!
//! The checked-in fixture `fixtures/editor/m5-advanced-editing/canonical_model.json`
//! is the published model. This gate rebuilds the model in code and asserts it
//! equals the fixture after a serialize round-trip, so the in-code model cannot
//! drift from the published artifact without failing CI. It also re-proves every
//! frozen invariant, support-export safety, the selection-semantics disclosure
//! contract, the fold-state hidden-critical-state contract, and the overview-aid
//! optionality / alignment / honest-degradation contract.

use std::path::{Path, PathBuf};

use aureline_editor::{
    advanced_editing_model, advanced_editing_model_lines, AdvancedEditingModel, EditorSurfaceClass,
    FoldRiskClass, SelectionModeClass, SelectionSemanticsClass, M5_ADVANCED_EDITING_RECORD_KIND,
    M5_ADVANCED_EDITING_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-advanced-editing/canonical_model.json")
}

fn load_fixture() -> AdvancedEditingModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = advanced_editing_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code advanced-editing model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_advanced_editing`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ADVANCED_EDITING_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ADVANCED_EDITING_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: AdvancedEditingModel =
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
fn every_surface_is_covered_once() {
    let fixture = load_fixture();
    for surface in EditorSurfaceClass::ALL {
        assert!(
            fixture.snapshot(surface).is_some(),
            "missing snapshot for {}",
            surface.as_str()
        );
    }
    assert_eq!(
        fixture.surface_snapshots.len(),
        EditorSurfaceClass::ALL.len()
    );
}

#[test]
fn selection_strips_disclose_semantics_and_explain_unsupported_ops() {
    let fixture = load_fixture();
    for strip in fixture.all_selection_strips() {
        assert!(
            strip.count_and_primary_visible(),
            "strip {} must show count and primary",
            strip.strip_id
        );
        assert!(
            strip.semantics_disclosed_when_inexact(),
            "strip {} must disclose non-exact semantics",
            strip.strip_id
        );
        assert!(
            strip.unsupported_operations_explained(),
            "strip {} must explain unsupported ops",
            strip.strip_id
        );
    }
}

#[test]
fn folds_advertise_hidden_critical_state() {
    let fixture = load_fixture();
    for fold in fixture.all_fold_summaries() {
        assert!(
            fold.risk_class_matches_counts(),
            "fold {} risk must match counts",
            fold.fold_id
        );
        assert!(
            fold.advertises_critical_state(),
            "fold {} must advertise hidden critical state",
            fold.fold_id
        );
        assert!(
            fold.keyboard_and_label_present(),
            "fold {} must stay keyboard-toggleable and labeled",
            fold.fold_id
        );
    }
}

#[test]
fn overview_aids_are_optional_aligned_and_degrade_honestly() {
    let fixture = load_fixture();
    for aid in fixture.all_overview_aids() {
        assert!(
            aid.not_sole_carrier(),
            "aid {} must not be the sole carrier of critical state",
            aid.aid_id
        );
        assert!(
            aid.aligned_with_main(),
            "aid {} must align with main-editor markers",
            aid.aid_id
        );
        assert!(
            aid.degrades_honestly(),
            "aid {} must degrade honestly",
            aid.aid_id
        );
    }
}

#[test]
fn large_file_and_blocked_surfaces_hold_their_contracts() {
    let fixture = load_fixture();
    let large = fixture
        .snapshot(EditorSurfaceClass::LargeFileRestricted)
        .expect("large");
    assert_eq!(
        large.selection_strip.mode_class,
        SelectionModeClass::SingleCaret
    );
    assert!(large.fold_summaries.is_empty());

    let generated = fixture
        .snapshot(EditorSurfaceClass::GeneratedFile)
        .expect("generated");
    assert_eq!(
        generated.selection_strip.semantics_class,
        SelectionSemanticsClass::Blocked
    );
    let gfold = generated.fold_summaries.first().expect("generated fold");
    assert_eq!(gfold.risk_class, FoldRiskClass::HiddenCritical);
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = advanced_editing_model_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Advanced-editing model")));
    assert!(lines.iter().any(|line| line.contains("Surface snapshots:")));
    for snapshot in &fixture.surface_snapshots {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(snapshot.surface_class.as_str())),
            "projection must mention surface {}",
            snapshot.surface_class.as_str()
        );
    }
}
