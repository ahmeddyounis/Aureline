//! Freeze gate for the canonical completion-row model.
//!
//! The checked-in fixture `fixtures/editor/m5-completion-rows/canonical_model.json`
//! is the published model. This gate rebuilds the model in code and asserts it
//! equals the fixture after a serialize round-trip, so the in-code model cannot
//! drift from the published artifact without failing CI. It also re-proves every
//! frozen invariant, support-export safety, the deterministic-versus-AI trust
//! distinction, the commit-disclosure contract, and the per-surface degraded
//! provider labeling.

use std::path::{Path, PathBuf};

use aureline_editor::{
    completion_row_model, completion_row_model_lines, AdditionalEditCue, CompletionAssistClass,
    CompletionAvailabilityClass, CompletionRowModel, M5_COMPLETION_ROWS_RECORD_KIND,
    M5_COMPLETION_ROWS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-completion-rows/canonical_model.json")
}

fn load_fixture() -> CompletionRowModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = completion_row_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code completion-row model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_completion_rows`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_COMPLETION_ROWS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_COMPLETION_ROWS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: CompletionRowModel =
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
fn ai_and_local_word_never_carry_full_semantic_trust() {
    let fixture = load_fixture();
    for row in fixture.all_rows() {
        if matches!(
            row.assist_class,
            CompletionAssistClass::AiBacked | CompletionAssistClass::LocalWord
        ) {
            assert!(
                !row.trust_weight.is_full_semantic(),
                "{} must not carry full-semantic trust weight",
                row.row_id
            );
        }
    }
}

#[test]
fn additional_edit_rows_disclose_before_commit() {
    let fixture = load_fixture();
    for row in fixture.all_rows() {
        if row.availability.is_acceptable()
            && row.additional_edit_cue.requires_pre_commit_disclosure()
        {
            assert!(
                row.commit_disclosure_required,
                "{} must disclose its additional edit before commit",
                row.row_id
            );
            assert!(
                row.additional_edit_summary.is_some() || row.preview_required,
                "{} must carry a summary or require preview",
                row.row_id
            );
        }
    }
}

#[test]
fn generated_and_dependency_effects_require_preview() {
    let fixture = load_fixture();
    for row in fixture.all_rows() {
        if row.availability.is_acceptable()
            && matches!(
                row.additional_edit_cue,
                AdditionalEditCue::GeneratedOutputEffect | AdditionalEditCue::AddsDependency
            )
        {
            assert!(
                row.preview_required,
                "{} must require preview before applying",
                row.row_id
            );
        }
    }
}

#[test]
fn degraded_surfaces_carry_visible_fallback_label() {
    let fixture = load_fixture();
    let mut degraded_seen = 0;
    for snapshot in &fixture.surface_snapshots {
        if snapshot.provider_posture.is_degraded() {
            degraded_seen += 1;
            assert!(snapshot.fallback_label_required);
            assert!(!snapshot.provider_posture_label.trim().is_empty());
            assert!(snapshot.disclosure_required);
        }
    }
    assert!(
        degraded_seen > 0,
        "at least one degraded surface is expected"
    );
}

#[test]
fn rows_mirror_their_canonical_snapshot() {
    let fixture = load_fixture();
    for snapshot in &fixture.surface_snapshots {
        assert_eq!(snapshot.canonical_snapshot.items.len(), snapshot.rows.len());
        for row in &snapshot.rows {
            let item = snapshot
                .canonical_snapshot
                .items
                .iter()
                .find(|item| item.completion_item_id == row.row_id)
                .unwrap_or_else(|| panic!("canonical item for row {}", row.row_id));
            assert_eq!(item.source, row.source);
            assert_eq!(item.label, row.primary_label);
        }
    }
}

#[test]
fn non_available_rows_are_marked() {
    let fixture = load_fixture();
    for row in fixture.all_rows() {
        if row.availability != CompletionAvailabilityClass::Available {
            assert!(row.requires_visual_distinction);
            assert!(!row.non_color_differentiator.trim().is_empty());
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = completion_row_model_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Completion-row model")));
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
