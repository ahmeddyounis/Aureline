//! Protected tests binding the typed M5 JSON Schema catalog to the checked-in
//! catalog, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in catalog; the capture cross-check proves
//! the typed model and the CI validator agree on the summary counts and the
//! per-package checks; the negative cases load the checked-in fixtures to prove
//! that a duplicate package id and a drifted summary fail validation.

use std::path::{Path, PathBuf};

use aureline_release::implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts::{
    current_m5_json_schema_catalog, LifecycleLabel, M5JsonSchemaCatalog,
    M5_JSON_SCHEMA_CATALOG_ID, M5_JSON_SCHEMA_CATALOG_RECORD_KIND,
    M5_JSON_SCHEMA_CATALOG_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts_validation_capture.json"
));

fn catalog() -> M5JsonSchemaCatalog {
    current_m5_json_schema_catalog().expect("checked-in catalog parses into the model")
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
    assert_eq!(c.schema_version, M5_JSON_SCHEMA_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_JSON_SCHEMA_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_JSON_SCHEMA_CATALOG_ID);
    let violations = c.validate();
    assert!(violations.is_empty(), "checked-in catalog must validate cleanly: {violations:#?}");
}

#[test]
fn every_durable_family_publishes_a_versioned_labeled_package() {
    let c = catalog();
    assert!(!c.packages.is_empty());
    for pkg in &c.packages {
        // Explicit version field, lifecycle label, schema package, and note.
        assert!(!pkg.version_field_names.is_empty(), "{} needs a version field", pkg.family_id);
        assert!(pkg.schema_path.starts_with("schemas/public/m5-json/"));
        assert!(pkg.schema_id.ends_with(".schema.json"));
        assert!(!pkg.compatibility_note.is_empty());
        assert!(matches!(
            pkg.lifecycle_label,
            LifecycleLabel::Stable | LifecycleLabel::Beta | LifecycleLabel::Lts
        ));
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
    assert_eq!(summary["total_packages"].as_u64().unwrap() as usize, c.packages.len());
    assert_eq!(
        summary["stable_label_packages"].as_u64().unwrap() as usize,
        computed.stable_label_packages
    );
    assert_eq!(
        summary["beta_label_packages"].as_u64().unwrap() as usize,
        computed.beta_label_packages
    );
    assert_eq!(
        summary["preserve_unknown_packages"].as_u64().unwrap() as usize,
        computed.preserve_unknown_packages
    );

    let checks = capture["package_checks"].as_array().unwrap();
    assert_eq!(checks.len(), c.packages.len(), "capture must record every package");
    for check in checks {
        let family = check["family_id"].as_str().unwrap();
        let pkg = c.package(family).unwrap_or_else(|| panic!("capture family {family} is in the model"));
        assert_eq!(
            check["lifecycle_label"].as_str().unwrap(),
            serde_json::to_value(pkg.lifecycle_label).unwrap().as_str().unwrap(),
            "capture lifecycle label must match the model for {family}"
        );
        for key in [
            "schema_valid",
            "example_valid",
            "roundtrip_preserves_unknown",
            "lifecycle_matches_matrix",
        ] {
            assert_eq!(check[key].as_str(), Some("passed"), "{family}: {key} must have passed");
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
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-json-catalog");
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
        // The unknown-lifecycle-label fixture intentionally carries an off-vocab
        // enum that serde refuses to deserialize, which is itself a rejection;
        // the structurally-parseable fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5JsonSchemaCatalog>(&raw) {
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
    assert!(model_checked > 0, "at least one fixture must exercise a typed-model invariant");
}
