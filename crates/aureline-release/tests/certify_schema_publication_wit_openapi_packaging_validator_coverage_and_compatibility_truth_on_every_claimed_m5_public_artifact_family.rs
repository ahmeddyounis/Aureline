//! Protected tests binding the typed M5 public-contract certification register to the
//! checked-in register, the upstream contract-health register and publication matrix it
//! joins, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the upstream cross-check proves every
//! certified family carries the same certified label, public claim, and published label the
//! contract-health register and publication matrix carry; the capture cross-check proves the
//! typed model and the CI validator agree on the decision and the summary; the negative cases
//! load the checked-in fixtures to prove that a duplicate family id, a state/pillar
//! disagreement, a greener-than-claim label, a missing pillar, an unknown certification state,
//! a drifted summary, and a drifted promotion decision all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family::{
    current_m5_public_contract_certification_register, CertificationState, DecisionState,
    M5PublicContractCertificationRegister, M5_PUBLIC_CONTRACT_CERTIFICATION_RECORD_KIND,
    M5_PUBLIC_CONTRACT_CERTIFICATION_REGISTER_ID, M5_PUBLIC_CONTRACT_CERTIFICATION_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family_validation_capture.json"
));

fn register() -> M5PublicContractCertificationRegister {
    current_m5_public_contract_certification_register()
        .expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read_json(rel: &str) -> serde_json::Value {
    let raw = std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|_| panic!("{rel} is readable"));
    serde_json::from_str(&raw).unwrap_or_else(|_| panic!("{rel} parses"))
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_PUBLIC_CONTRACT_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_PUBLIC_CONTRACT_CERTIFICATION_RECORD_KIND);
    assert_eq!(r.register_id, M5_PUBLIC_CONTRACT_CERTIFICATION_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_certified_family_matches_the_upstream_health_register_and_matrix() {
    let r = register();
    let health = read_json("artifacts/release/m5-contract-health.json");
    let matrix = read_json("artifacts/contracts/m5-stability-lifecycle-map.json");

    let health_rows = health["rows"].as_array().expect("health rows");
    let matrix_rows = matrix["rows"].as_array().expect("matrix rows");
    assert_eq!(
        r.rows.len(),
        health_rows.len(),
        "every claimed family in the contract-health register is certified"
    );

    for row in &r.rows {
        let h = health_rows
            .iter()
            .find(|hr| hr["family_id"].as_str() == Some(row.family_id.as_str()))
            .unwrap_or_else(|| panic!("{} is in the contract-health register", row.family_id));
        let m = matrix_rows
            .iter()
            .find(|mr| mr["family_id"].as_str() == Some(row.family_id.as_str()))
            .unwrap_or_else(|| panic!("{} is in the publication matrix", row.family_id));

        // The certified label equals the contract-health lifecycle label (post-narrowing).
        assert_eq!(
            h["lifecycle_label"].as_str(),
            serde_json::to_value(row.certified_label)
                .unwrap()
                .as_str()
                .map(str::to_owned)
                .as_deref(),
            "{}: certified label matches the contract-health lifecycle label",
            row.family_id
        );
        // The public claim equals the matrix claim label.
        assert_eq!(
            m["claim_label"].as_str(),
            serde_json::to_value(row.claim_label)
                .unwrap()
                .as_str()
                .map(str::to_owned)
                .as_deref(),
            "{}: public claim matches the matrix claim label",
            row.family_id
        );
        assert_eq!(
            h["release_blocking"].as_bool(),
            Some(row.release_blocking),
            "{}: release-blocking flag matches the contract-health register",
            row.family_id
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(
        capture["register_id"].as_str(),
        Some(r.register_id.as_str())
    );
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));
    assert_eq!(
        capture["decision"].as_str().unwrap(),
        serde_json::to_value(r.promotion.decision)
            .unwrap()
            .as_str()
            .unwrap()
    );

    let summary = &capture["summary"];
    let computed = r.computed_summary();
    assert_eq!(
        summary["total_families"].as_u64().unwrap() as usize,
        computed.total_families
    );
    assert_eq!(
        summary["certified_families"].as_u64().unwrap() as usize,
        computed.certified_families
    );
    assert_eq!(
        summary["withheld_families"].as_u64().unwrap() as usize,
        computed.withheld_families
    );
    assert_eq!(
        summary["pillars_missing"].as_u64().unwrap() as usize,
        computed.pillars_missing
    );

    let blocking = capture["blocking_family_ids"]
        .as_array()
        .expect("capture lists blocking families");
    let blocking: Vec<&str> = blocking.iter().filter_map(|v| v.as_str()).collect();
    assert_eq!(
        blocking,
        r.promotion
            .blocking_family_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn promotion_holds_only_when_a_release_blocking_family_withholds() {
    let r = register();
    let withheld_blocking = r
        .rows
        .iter()
        .any(|row| row.certification_state == CertificationState::Withheld && row.release_blocking);
    assert_eq!(
        r.promotion.decision == DecisionState::Hold,
        withheld_blocking
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-public-contract-certification");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        // The unknown-certification-state fixture carries an off-vocab enum that serde refuses
        // to deserialize, which is itself a rejection; the structurally-parseable fixtures must
        // be rejected by `validate()`.
        match serde_json::from_str::<M5PublicContractCertificationRegister>(&raw) {
            Ok(candidate) => {
                assert!(
                    !candidate.validate().is_empty(),
                    "fixture {file} must be rejected by the typed model"
                );
                checked += 1;
            }
            Err(_) => {
                checked += 1;
            }
        }
    }
    assert_eq!(checked, cases.len(), "every fixture is exercised");
}
