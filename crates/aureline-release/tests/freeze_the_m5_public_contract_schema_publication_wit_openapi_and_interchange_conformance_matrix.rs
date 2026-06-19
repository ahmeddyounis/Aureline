//! Protected tests binding the typed M5 public-contract matrix to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in matrix; the capture cross-check proves the
//! typed model and the CI validator agree on the promotion verdict and the
//! summary counts; the negative cases mutate parsed copies and load the checked-in
//! fixtures to prove that a published family with an unpublished requirement, a
//! narrowed family that drops its gap reasons, and a duplicate family id all fail
//! validation.

use std::path::{Path, PathBuf};

use aureline_release::freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix::{
    current_m5_public_contract_matrix, ContractForm, M5PublicContractMatrix, M5_PUBLIC_CONTRACT_RECORD_KIND,
    M5_PUBLIC_CONTRACT_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix_validation_capture.json"
));

fn matrix() -> M5PublicContractMatrix {
    current_m5_public_contract_matrix().expect("checked-in matrix parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(m.schema_version, M5_PUBLIC_CONTRACT_SCHEMA_VERSION);
    assert_eq!(m.record_kind, M5_PUBLIC_CONTRACT_RECORD_KIND);
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn inventories_wit_and_openapi_contract_forms() {
    let m = matrix();
    assert!(!m.rows_for_form(ContractForm::WitWorldPackage).is_empty());
    assert!(!m.rows_for_form(ContractForm::OpenapiFamily).is_empty());
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let m = matrix();
    assert!(!m.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = m
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.family_id.as_str())
        .collect();
    for declared in &m.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let m = matrix();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(m.as_of.as_str()));

    let summary = &capture["summary"];
    let computed = m.computed_summary();
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        m.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_published"].as_u64().unwrap() as usize,
        computed.rows_published,
        "capture published count must match the model"
    );
    assert_eq!(
        summary["rows_narrowed"].as_u64().unwrap() as usize,
        computed.rows_narrowed,
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["total_required_publications"].as_u64().unwrap() as usize,
        computed.total_required_publications,
        "capture required-publication count must match the model"
    );
    assert_eq!(
        summary["total_published_publications"].as_u64().unwrap() as usize,
        computed.total_published_publications,
        "capture published-publication count must match the model"
    );
    assert_eq!(
        summary["total_active_gap_reasons"].as_u64().unwrap() as usize,
        computed.total_active_gap_reasons,
        "capture gap-reason count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        computed.rules_firing,
        "capture firing-rule count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        m.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(m.promotion.decision, m.computed_promotion_decision());

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
    let fixtures_dir = repo_root().join("fixtures/contracts/m5");
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
        let candidate: M5PublicContractMatrix =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        model_checked += 1;
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model structural invariant"
    );
}
