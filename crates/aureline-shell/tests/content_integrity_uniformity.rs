//! Uniform content-integrity warning coverage for protected product surfaces.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use aureline_buffer::{project_editor_content_integrity, EditorContentIntegrityEvent};
use aureline_content_safety::{
    detect_suspicious_content, ContentIntegrityWarningRecord, CONTENT_INTEGRITY_WARNING_RECORD_KIND,
};
use aureline_preview::{build_risky_text_preview, RiskyTextInput};
use aureline_shell::install_review_fact_grid::project_install_review_content_integrity;
use aureline_shell::save_review::project_save_review_diff_content_integrity;
use aureline_shell::search::project_search_content_integrity;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    case_id: String,
    content: String,
    expected_surfaces: Vec<String>,
    expected_warning_classes: Vec<String>,
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("content_safety")
        .join("content_integrity_uniformity")
        .join("suspicious_source.json")
}

fn load_fixture() -> Fixture {
    let path = fixture_path();
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("invalid JSON in {}: {err}", path.display()))
}

fn class_tokens(warnings: &[ContentIntegrityWarningRecord]) -> BTreeSet<String> {
    warnings
        .iter()
        .map(|warning| warning.content_class_token.clone())
        .collect()
}

fn assert_shared_warning_shape(surface: &str, warnings: &[ContentIntegrityWarningRecord]) {
    assert!(!warnings.is_empty(), "{surface} did not emit warnings");
    assert!(
        warnings
            .iter()
            .all(|warning| warning.record_kind == CONTENT_INTEGRITY_WARNING_RECORD_KIND),
        "{surface} emitted a non-shared warning record kind: {warnings:#?}"
    );
    assert!(
        warnings
            .iter()
            .all(|warning| warning.surface_token == surface),
        "{surface} emitted warnings tagged for the wrong surface: {warnings:#?}"
    );
    assert!(
        warnings.iter().all(|warning| {
            warning
                .transfer_action_ids
                .iter()
                .any(|action| action == "copy_raw")
                && warning
                    .transfer_action_ids
                    .iter()
                    .any(|action| action == "copy_escaped")
        }),
        "{surface} did not preserve raw and escaped transfer actions"
    );
}

#[test]
fn five_named_surfaces_emit_uniform_content_integrity_warnings() {
    let fixture = load_fixture();
    let expected_classes = fixture
        .expected_warning_classes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    let editor = project_editor_content_integrity(
        &fixture.case_id,
        "buffer:src/app.ts",
        EditorContentIntegrityEvent::Open,
        fixture.content.as_bytes(),
    );

    let diff = project_save_review_diff_content_integrity(
        &fixture.case_id,
        "save-review:src/app.ts:diff",
        fixture.content.as_bytes(),
    );

    let search = project_search_content_integrity(
        &fixture.case_id,
        "search:row:src/app.ts:payload",
        &fixture.content,
    );

    let preview = build_risky_text_preview(RiskyTextInput {
        preview_id: "preview:risky-text:src/app.ts".to_string(),
        source_subject_ref: "preview:src/app.ts".to_string(),
        source_surface_family: "rich_preview".to_string(),
        trust_class: aureline_content_safety::TrustClass::RawText,
        detection: detect_suspicious_content(&fixture.content),
    });

    let package = project_install_review_content_integrity(
        &fixture.case_id,
        "package-review:extension:suspicious-source",
        &fixture.content,
    );

    let mut surfaces = BTreeMap::new();
    surfaces.insert("editor".to_string(), editor.warnings);
    surfaces.insert("diff".to_string(), diff);
    surfaces.insert("search".to_string(), search.warnings);
    surfaces.insert("preview".to_string(), preview.content_integrity_warnings);
    surfaces.insert("package".to_string(), package);

    let expected_surfaces = fixture
        .expected_surfaces
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let actual_surfaces = surfaces.keys().cloned().collect::<BTreeSet<_>>();
    assert_eq!(actual_surfaces, expected_surfaces);

    for (surface, warnings) in &surfaces {
        assert_shared_warning_shape(surface, warnings);
        assert_eq!(
            class_tokens(warnings),
            expected_classes,
            "{surface} warning classes diverged"
        );
    }
}
