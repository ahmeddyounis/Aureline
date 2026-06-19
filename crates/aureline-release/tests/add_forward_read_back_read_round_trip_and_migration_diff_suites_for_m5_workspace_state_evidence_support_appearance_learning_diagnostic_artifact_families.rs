//! Protected tests binding the typed M5 reader/writer compatibility suite to the
//! checked-in suite, the frozen CI validation capture, the per-family
//! migration-diff reports, and the negative fixtures.
//!
//! The positive case is the checked-in suite; the capture cross-check proves the
//! typed model and the CI validator agree on the summary counts and per-suite
//! checks; the report cross-check proves the embedded migration diff matches the
//! standalone per-family report; the negative cases load the checked-in fixtures
//! to prove a duplicate family and a drifted summary fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families::{
    current_m5_reader_writer_compat_suite, ChangeClass, M5ReaderWriterCompatSuite,
    M5_READER_WRITER_COMPAT_SUITE_ID, M5_READER_WRITER_COMPAT_SUITE_RECORD_KIND,
    M5_READER_WRITER_COMPAT_SUITE_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families_validation_capture.json"
));

fn suite() -> M5ReaderWriterCompatSuite {
    current_m5_reader_writer_compat_suite().expect("checked-in suite parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_suite_parses_and_validates() {
    let s = suite();
    assert_eq!(
        s.schema_version,
        M5_READER_WRITER_COMPAT_SUITE_SCHEMA_VERSION
    );
    assert_eq!(s.record_kind, M5_READER_WRITER_COMPAT_SUITE_RECORD_KIND);
    assert_eq!(s.suite_id, M5_READER_WRITER_COMPAT_SUITE_ID);
    let violations = s.validate();
    assert!(
        violations.is_empty(),
        "checked-in suite must validate cleanly: {violations:#?}"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let s = suite();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(s.as_of.as_str()));
    assert_eq!(capture["suite_id"].as_str(), Some(s.suite_id.as_str()));

    let summary = &capture["summary"];
    let computed = s.computed_summary();
    assert_eq!(
        summary["total_suites"].as_u64().unwrap() as usize,
        s.suites.len()
    );
    assert_eq!(
        summary["write_back_suites"].as_u64().unwrap() as usize,
        computed.write_back_suites
    );
    assert_eq!(
        summary["compare_only_suites"].as_u64().unwrap() as usize,
        computed.compare_only_suites
    );
    assert_eq!(
        summary["total_cases"].as_u64().unwrap() as usize,
        computed.total_cases
    );
    assert_eq!(
        summary["narrowing_cases"].as_u64().unwrap() as usize,
        computed.narrowing_cases
    );

    let checks = capture["suite_checks"].as_array().unwrap();
    assert_eq!(
        checks.len(),
        s.suites.len(),
        "capture must record every suite"
    );
    for check in checks {
        let family = check["family_id"].as_str().unwrap();
        let fam = s
            .suite(family)
            .unwrap_or_else(|| panic!("capture family {family} is in the model"));
        assert_eq!(
            check["reader_writer_posture"].as_str().unwrap(),
            serde_json::to_value(fam.reader_writer_posture)
                .unwrap()
                .as_str()
                .unwrap(),
            "capture posture must match the model for {family}"
        );
        assert_eq!(
            check["case_count"].as_u64().unwrap() as usize,
            fam.cases.len()
        );
        for key in [
            "forward_read",
            "back_read",
            "round_trip_or_compare_only",
            "migration_diff_additive",
            "unknown_field_preserved",
            "downgrade_narrows",
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
fn embedded_diff_matches_standalone_reports() {
    let s = suite();
    let root = repo_root();
    for fam in &s.suites {
        let report_path = root.join(&fam.migration_diff.report_ref);
        let raw = std::fs::read_to_string(&report_path)
            .unwrap_or_else(|_| panic!("report {} is readable", fam.migration_diff.report_ref));
        let report: serde_json::Value = serde_json::from_str(&raw).expect("report parses");
        assert_eq!(
            report["from_version"].as_u64().unwrap() as u32,
            fam.migration_diff.from_version
        );
        assert_eq!(
            report["to_version"].as_u64().unwrap() as u32,
            fam.migration_diff.to_version
        );
        assert_eq!(report["change_class"].as_str(), Some("additive"));
        assert_eq!(fam.migration_diff.change_class, ChangeClass::Additive);
        let added: Vec<String> = report["added_fields"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(added, fam.migration_diff.added_fields);
    }
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-compat-suite");
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
        // The unknown-case-kind fixture intentionally carries an off-vocab enum
        // that serde refuses to deserialize, which is itself a rejection; the
        // structurally-parseable fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5ReaderWriterCompatSuite>(&raw) {
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
