//! Freeze gate for the canonical editor-assist matrix.
//!
//! The checked-in fixture `fixtures/editor/m5-editor-assist/canonical_matrix.json`
//! is the published matrix. This gate rebuilds the matrix in code and asserts it
//! equals the fixture byte-for-byte after a serialize round-trip, so the in-code
//! matrix cannot drift from the published artifact without failing CI. It also
//! re-proves every frozen invariant, support-export safety, full surface ×
//! channel coverage, and that every enum token is catalogued.

use std::path::{Path, PathBuf};

use aureline_editor::{
    editor_assist_matrix, editor_assist_matrix_lines, AssistChannelClass, EditorAssistMatrix,
    EditorSurfaceClass, MicroSurfaceKind, M5_EDITOR_ASSIST_RECORD_KIND,
    M5_EDITOR_ASSIST_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-editor-assist/canonical_matrix.json")
}

fn load_fixture() -> EditorAssistMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = editor_assist_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code editor-assist matrix drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_editor_assist`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_EDITOR_ASSIST_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_EDITOR_ASSIST_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: EditorAssistMatrix =
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
fn matrix_covers_every_surface_and_channel() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.surface_profiles.len(),
        EditorSurfaceClass::ALL.len(),
        "every claimed surface must appear"
    );
    for surface in EditorSurfaceClass::ALL {
        let profile = fixture
            .surface_profile(surface)
            .unwrap_or_else(|| panic!("missing surface {}", surface.as_str()));
        for channel in AssistChannelClass::ALL {
            assert!(
                profile.cell(channel).is_some(),
                "surface {} is missing channel {}",
                surface.as_str(),
                channel.as_str()
            );
        }
    }
}

#[test]
fn matrix_covers_constrained_and_accessibility_sensitive_surfaces() {
    let fixture = load_fixture();
    // The acceptance set: notebook, request/SQL, docs-code, generated/protected,
    // partial-index, large-file. Each must be present and constrained.
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ] {
        let profile = fixture
            .surface_profile(surface)
            .unwrap_or_else(|| panic!("missing surface {}", surface.as_str()));
        assert!(
            profile.is_constrained,
            "surface {} must be marked constrained",
            surface.as_str()
        );
    }
}

#[test]
fn every_micro_surface_kind_has_identity_and_export_minimum() {
    let fixture = load_fixture();
    for kind in MicroSurfaceKind::ALL {
        assert!(
            fixture
                .identity_contracts
                .iter()
                .any(|contract| contract.kind == kind),
            "missing identity contract for {}",
            kind.as_str()
        );
        assert!(
            fixture
                .support_export_minimums
                .iter()
                .any(|minimum| minimum.record_kind == kind.export_record_kind()),
            "missing export minimum for {}",
            kind.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = editor_assist_matrix_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Editor-assist matrix")));
    assert!(lines.iter().any(|line| line.contains("Surface matrix:")));
    // Every channel token surfaces in the projection.
    for channel in AssistChannelClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(channel.as_str())),
            "projection must mention channel {}",
            channel.as_str()
        );
    }
}
