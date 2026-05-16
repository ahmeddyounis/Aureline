//! Integration test for the beta debugger and execution-context support
//! matrix.
//!
//! This test loads the checked-in
//! [`fixtures/runtime/m3/support_matrix_inputs/`](../../fixtures/runtime/m3/support_matrix_inputs/)
//! rows, asserts they match the canonical
//! [`aureline_runtime::SupportMatrixBetaManifest`] row for the same wedge,
//! and replays the support-export bundle end-to-end so migration, partner,
//! and release packets quote the same closed vocabulary the runtime emits.

use std::path::PathBuf;

use aureline_runtime::{
    SupportMatrixBetaManifest, SupportMatrixBetaSupportExport, SupportMatrixClass,
    SupportMatrixContextLane, SupportMatrixDowngradeRule, SupportMatrixWedgeId,
    SupportMatrixWedgeInput, SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND,
    SUPPORT_MATRIX_BETA_SCHEMA_VERSION, SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/m3/support_matrix_inputs")
        .join(name)
}

fn load_input(name: &str) -> SupportMatrixWedgeInput {
    let path = fixture_path(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn python_fixture_matches_canonical_row() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:test:python", "2026-05-16T00:00:00Z");
    let input = load_input("python.json");
    assert_eq!(input.wedge_id, SupportMatrixWedgeId::Python);
    let mismatches = manifest.compare_input(&input);
    assert!(
        mismatches.is_empty(),
        "python fixture must match canonical row, got mismatches: {mismatches:#?}"
    );
}

#[test]
fn typescript_javascript_fixture_matches_canonical_row() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:test:tsjs", "2026-05-16T00:00:00Z");
    let input = load_input("typescript_javascript.json");
    assert_eq!(input.wedge_id, SupportMatrixWedgeId::TypescriptJavascript);
    let mismatches = manifest.compare_input(&input);
    assert!(
        mismatches.is_empty(),
        "typescript_javascript fixture must match canonical row, got mismatches: {mismatches:#?}"
    );
}

#[test]
fn support_export_round_trips_canonical_manifest_and_input_rows() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:roundtrip", "2026-05-16T00:00:00Z");
    let inputs = vec![load_input("python.json"), load_input("typescript_javascript.json")];
    let export = SupportMatrixBetaSupportExport::new(
        "support-export:matrix-beta:1",
        "2026-05-16T00:00:01Z",
        manifest.clone(),
        inputs.clone(),
    );
    assert_eq!(
        export.record_kind,
        SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.schema_version, SUPPORT_MATRIX_BETA_SCHEMA_VERSION);
    assert_eq!(export.manifest.record_kind, SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND);
    assert_eq!(export.manifest, manifest);
    assert_eq!(export.inputs, inputs);

    // The export must JSON-serialise stably; every column-class token, lane
    // token, and downgrade-rule token must be present and verbatim.
    let json = serde_json::to_string(&export).expect("serialise export");
    for class in [
        SupportMatrixClass::Supported,
        SupportMatrixClass::Preview,
        SupportMatrixClass::Limited,
    ] {
        assert!(
            json.contains(class.as_str()),
            "export must mention class {}",
            class.as_str()
        );
    }
    for lane in SupportMatrixContextLane::ALL {
        assert!(
            json.contains(lane.as_str()),
            "export must mention lane {}",
            lane.as_str()
        );
    }
    for rule in SupportMatrixDowngradeRule::ALL {
        assert!(
            json.contains(rule.as_str()),
            "export must mention downgrade rule {}",
            rule.as_str()
        );
    }

    // Out-of-scope: the bundle must not advertise held-for-later wedges
    // (Java/Kotlin, Rust workspace, Go service) that are not in the claimed
    // wedge vocabulary.
    assert!(!json.contains("kotlin"));
    assert!(!json.contains("\"rust_workspace\""));
    assert!(!json.contains("\"go_service\""));
}

#[test]
fn missing_features_render_as_limited_or_preview_rather_than_supported() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:missing", "2026-05-16T00:00:00Z");
    let tsjs = manifest
        .row_for_wedge(SupportMatrixWedgeId::TypescriptJavascript)
        .expect("tsjs row present");
    // TS/JS has no claimed framework yet — the test column must surface as
    // `limited`, not `supported`.
    assert_eq!(tsjs.test.class_token, "limited");
    assert!(tsjs.test.claimed_framework_tokens.is_empty());
    // Protected dispatch must NOT be permitted on a preview/limited row.
    assert!(!tsjs.allows_protected_dispatch());

    // Python: the attach column is preview (capability is wired but not
    // exercised), so the row must not silently advertise full attach
    // parity.
    let python = manifest
        .row_for_wedge(SupportMatrixWedgeId::Python)
        .expect("python row present");
    assert_eq!(python.attach.class_token, "preview");
    assert!(!python.attach.class.allows_protected_dispatch());
}
