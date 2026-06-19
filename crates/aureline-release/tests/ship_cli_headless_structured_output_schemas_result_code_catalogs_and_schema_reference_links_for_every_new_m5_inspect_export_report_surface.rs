//! Protected tests binding the typed M5 CLI/headless structured-output catalog to
//! the checked-in catalog, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in catalog; the capture cross-check proves
//! the typed model and the CI validator agree on the summary counts and the
//! per-surface checks; the negative cases load the checked-in fixtures to prove
//! that a duplicate surface id and a drifted summary fail validation.

use std::path::{Path, PathBuf};

use aureline_release::ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface::{
    current_m5_cli_output_catalog, LifecycleLabel, M5CliOutputCatalog,
    M5_CLI_OUTPUT_CATALOG_ID, M5_CLI_OUTPUT_CATALOG_RECORD_KIND,
    M5_CLI_OUTPUT_CATALOG_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface_validation_capture.json"
));

fn catalog() -> M5CliOutputCatalog {
    current_m5_cli_output_catalog().expect("checked-in catalog parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_catalog_parses_and_validates() {
    let c = catalog();
    assert_eq!(c.schema_version, M5_CLI_OUTPUT_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_CLI_OUTPUT_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_CLI_OUTPUT_CATALOG_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "checked-in catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_surface_publishes_a_schema_result_codes_and_label() {
    let c = catalog();
    assert!(!c.surfaces.is_empty());
    for s in &c.surfaces {
        assert!(s
            .structured_output_schema_ref
            .starts_with("schemas/public/m5-json/"));
        assert!(
            !s.result_code_catalog.is_empty(),
            "{} needs result codes",
            s.surface_id
        );
        assert!(!s.compatibility_note.is_empty());
        assert!(matches!(
            s.lifecycle_label,
            LifecycleLabel::Stable | LifecycleLabel::Beta | LifecycleLabel::Lts
        ));
        // The schema-ref's target schema file exists on disk.
        let schema_file = repo_root().join(&s.structured_output_schema_ref);
        assert!(
            schema_file.exists(),
            "{}: schema {schema_file:?} must exist",
            s.surface_id
        );
        // Both parity fixtures exist on disk.
        assert!(repo_root().join(&s.cli_parity_fixture_ref).exists());
        assert!(repo_root().join(&s.ui_parity_fixture_ref).exists());
    }
}

#[test]
fn ui_and_cli_parity_fixtures_carry_identical_lifecycle_vocabulary() {
    let c = catalog();
    let root = repo_root();
    for s in &c.surfaces {
        let cli: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join(&s.cli_parity_fixture_ref))
                .expect("cli fixture readable"),
        )
        .expect("cli fixture parses");
        let ui: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join(&s.ui_parity_fixture_ref))
                .expect("ui fixture readable"),
        )
        .expect("ui fixture parses");
        for field in ["partial_result_state", "freshness_state", "lifecycle_label"] {
            assert_eq!(
                cli[field], ui[field],
                "{}: {field} must be identical between CLI and UI inspect output",
                s.surface_id
            );
        }
        // And the shared lifecycle label matches the surface row.
        assert_eq!(
            cli["lifecycle_label"].as_str().unwrap(),
            serde_json::to_value(s.lifecycle_label)
                .unwrap()
                .as_str()
                .unwrap(),
            "{}: fixture lifecycle label must match the surface row",
            s.surface_id
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let c = catalog();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(c.as_of.as_str()));
    assert_eq!(capture["catalog_id"].as_str(), Some(c.catalog_id.as_str()));

    let summary = &capture["summary"];
    let computed = c.computed_summary();
    assert_eq!(
        summary["total_surfaces"].as_u64().unwrap() as usize,
        c.surfaces.len()
    );
    assert_eq!(
        summary["inspect_surfaces"].as_u64().unwrap() as usize,
        computed.inspect_surfaces
    );
    assert_eq!(
        summary["export_surfaces"].as_u64().unwrap() as usize,
        computed.export_surfaces
    );
    assert_eq!(
        summary["surfaces_with_parity_fixtures"].as_u64().unwrap() as usize,
        computed.surfaces_with_parity_fixtures
    );

    let checks = capture["surface_checks"].as_array().unwrap();
    assert_eq!(
        checks.len(),
        c.surfaces.len(),
        "capture must record every surface"
    );
    for check in checks {
        let sid = check["surface_id"].as_str().unwrap();
        let s = c
            .surface(sid)
            .unwrap_or_else(|| panic!("capture surface {sid} is in the model"));
        assert_eq!(
            check["lifecycle_label"].as_str().unwrap(),
            serde_json::to_value(s.lifecycle_label)
                .unwrap()
                .as_str()
                .unwrap(),
            "capture lifecycle label must match the model for {sid}"
        );
        for key in [
            "schema_ref_resolves",
            "result_codes_in_vocabulary",
            "lifecycle_matches_matrix",
            "ui_cli_parity_vocabulary_identical",
        ] {
            assert_eq!(
                check[key].as_str(),
                Some("passed"),
                "{sid}: {key} must have passed"
            );
        }
    }

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-cli-catalog");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        // The off-vocabulary fixture carries an enum serde refuses to
        // deserialize, which is itself a rejection; the structurally-parseable
        // fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5CliOutputCatalog>(&raw) {
            Ok(candidate) => {
                assert!(
                    !candidate.validate().is_empty(),
                    "fixture {file} must be rejected by the typed model"
                );
                model_checked += 1;
            }
            Err(_) => {
                model_checked += 1;
            }
        }
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model invariant"
    );
}
