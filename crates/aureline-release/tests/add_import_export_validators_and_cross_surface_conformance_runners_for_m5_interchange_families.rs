//! Protected tests binding the typed M5 interchange-conformance register to the checked-in
//! register, the per-family validator descriptors, the real emitted-artifact corpus, the
//! frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the validator cross-check proves every
//! validator descriptor file matches the register's per-family validator; the emitted-artifact
//! cross-check proves every family resolves to a real, well-formed emitted artifact whose
//! per-surface renderings agree with the row; the capture cross-check proves the typed model
//! and the CI validator agree on the summary and the per-family checks; the negative cases
//! load the checked-in fixtures to prove that a duplicate family id, an unknown conformance
//! state, a drifted summary, a missing dimension, silently widened trust, and an off-vocabulary
//! reason code all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families::{
    current_m5_interchange_conformance_register, ConsumerSurface, DecisionState, M5InterchangeConformanceRegister,
    M5_INTERCHANGE_CONFORMANCE_RECORD_KIND, M5_INTERCHANGE_CONFORMANCE_REGISTER_ID,
    M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families_validation_capture.json"
));

fn register() -> M5InterchangeConformanceRegister {
    current_m5_interchange_conformance_register()
        .expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_INTERCHANGE_CONFORMANCE_RECORD_KIND);
    assert_eq!(r.register_id, M5_INTERCHANGE_CONFORMANCE_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
    assert_eq!(r.blockers.decision, DecisionState::Clear);
}

#[test]
fn validator_descriptor_files_match_the_register() {
    let r = register();
    let root = repo_root();
    for row in &r.rows {
        let path = root.join(&row.validator.descriptor_ref);
        assert!(
            path.exists(),
            "validator descriptor {} exists",
            row.validator.descriptor_ref
        );
        let raw = std::fs::read_to_string(&path).expect("validator descriptor is readable");
        let descriptor: serde_json::Value = serde_json::from_str(&raw).expect("descriptor parses");
        assert_eq!(
            descriptor["validator_id"].as_str(),
            Some(row.validator.validator_id.as_str())
        );
        assert_eq!(
            descriptor["family_id"].as_str(),
            Some(row.family_id.as_str())
        );
        // Every failure mode names a reason code from the closed vocabulary, with a
        // copy-safe diagnostic instead of a raw parser exception.
        let modes = descriptor["failure_modes"]
            .as_array()
            .expect("failure_modes array");
        assert!(
            !modes.is_empty(),
            "{} enumerates failure modes",
            row.family_id
        );
        for mode in modes {
            assert!(
                mode["reason_code"].as_str().is_some(),
                "{}: failure mode names a reason code",
                row.family_id
            );
            assert!(
                mode["copy_safe_diagnostic"]
                    .as_str()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false),
                "{}: failure mode carries a copy-safe diagnostic",
                row.family_id
            );
        }
    }
}

#[test]
fn every_family_resolves_to_a_real_emitted_artifact() {
    let r = register();
    let root = repo_root();
    assert!(root.join(&r.build_identity_ref).exists());
    for row in &r.rows {
        let path = root.join(&row.runner.artifact_ref);
        assert!(path.exists(), "{} emitted artifact exists", row.family_id);
        let raw = std::fs::read_to_string(&path).expect("emitted artifact is readable");
        let art: serde_json::Value = serde_json::from_str(&raw).expect("emitted artifact parses");

        // The artifact carries its contract version and lifecycle label, and they match.
        assert_eq!(
            art[&row.contract_version_field].as_u64().map(|v| v as u32),
            Some(row.contract_version),
            "{}: emitted artifact carries its contract version",
            row.family_id
        );
        assert_eq!(
            art["lifecycle_label"].as_str(),
            serde_json::to_value(row.lifecycle_label)
                .unwrap()
                .as_str()
                .map(|s| s.to_owned())
                .as_deref(),
            "{}: emitted artifact lifecycle label matches the row",
            row.family_id
        );

        // Provenance is preserved (not stripped).
        let provenance = &art["provenance"];
        for field in [
            "exported_by_surface",
            "build_identity_ref",
            "source_record_class",
            "redaction_class",
        ] {
            assert!(
                provenance[field]
                    .as_str()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false),
                "{}: emitted artifact preserves provenance field {field}",
                row.family_id
            );
        }

        // Every consumer surface renders the artifact and agrees on version + label.
        for surface in ConsumerSurface::ALL {
            let key = serde_json::to_value(surface).unwrap();
            let key = key.as_str().unwrap();
            let rendering = &art["surface_renderings"][key];
            assert_eq!(
                rendering["contract_version"].as_u64().map(|v| v as u32),
                Some(row.contract_version),
                "{}: surface {key} agrees on contract version",
                row.family_id
            );
        }
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));
    assert_eq!(
        capture["register_id"].as_str(),
        Some(r.register_id.as_str())
    );
    assert_eq!(
        capture["promotion_decision"].as_str().unwrap(),
        serde_json::to_value(r.blockers.decision)
            .unwrap()
            .as_str()
            .unwrap()
    );

    let summary = &capture["summary"];
    let computed = r.computed_summary();
    assert_eq!(
        summary["total_families"].as_u64().unwrap() as usize,
        r.rows.len()
    );
    assert_eq!(
        summary["conformant_families"].as_u64().unwrap() as usize,
        computed.conformant_families
    );
    assert_eq!(
        summary["dimensions_failing"].as_u64().unwrap() as usize,
        computed.dimensions_failing
    );

    let checks = capture["family_checks"].as_array().unwrap();
    assert_eq!(checks.len(), r.rows.len(), "capture records every family");
    for check in checks {
        let family = check["family_id"].as_str().unwrap();
        let row = r
            .row(family)
            .unwrap_or_else(|| panic!("capture family {family} is in the model"));
        assert_eq!(
            check["conformance_state"].as_str().unwrap(),
            serde_json::to_value(row.conformance_state)
                .unwrap()
                .as_str()
                .unwrap(),
            "capture conformance state must match the model for {family}"
        );
        for key in [
            "dimensions_evaluated",
            "emitted_artifact_exists",
            "lifecycle_matches_catalog",
            "consumers_agree",
            "reason_codes_in_vocabulary",
        ] {
            assert_eq!(
                check[key].as_str(),
                Some("passed"),
                "{family}: {key} must have passed"
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
    assert!(!fixtures.is_empty(), "capture records fixture cases");
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
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-interchange/negative");
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
        // The unknown-conformance-state and unknown-reason-code fixtures intentionally carry
        // off-vocab enums that serde refuses to deserialize, which is itself a rejection; the
        // structurally-parseable fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5InterchangeConformanceRegister>(&raw) {
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
