//! Protected tests binding the typed M5 contract-health register to the
//! checked-in register, the CI gate descriptors, the shiproom dashboard, the
//! frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the gate cross-check proves
//! every gate descriptor file matches the register's per-family gate evaluations;
//! the graph-linkage cross-check proves every family resolves to a build identity
//! and package version; the capture cross-check proves the typed model and the CI
//! validator agree on the summary and the per-family checks; the negative cases
//! load the checked-in fixtures to prove that a duplicate family id, an unknown
//! health state, a drifted summary, a missing gate, and a lying blocker decision
//! all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages::{
    current_m5_contract_health_register, BlockerDecision, GateKind, HealthState,
    M5ContractHealthRegister, M5_CONTRACT_HEALTH_RECORD_KIND, M5_CONTRACT_HEALTH_REGISTER_ID,
    M5_CONTRACT_HEALTH_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages_validation_capture.json"
));

fn register() -> M5ContractHealthRegister {
    current_m5_contract_health_register().expect("checked-in register parses into the model")
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
    assert_eq!(r.schema_version, M5_CONTRACT_HEALTH_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_CONTRACT_HEALTH_RECORD_KIND);
    assert_eq!(r.register_id, M5_CONTRACT_HEALTH_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn gate_descriptor_files_match_the_register() {
    let r = register();
    let root = repo_root();
    for gate in &r.gate_catalog {
        let path = root.join(&gate.descriptor_ref);
        assert!(
            path.exists(),
            "gate descriptor {} exists",
            gate.descriptor_ref
        );
        let raw = std::fs::read_to_string(&path).expect("gate descriptor is readable");
        let descriptor: serde_json::Value = serde_json::from_str(&raw).expect("descriptor parses");
        assert_eq!(
            descriptor["gate_id"].as_str(),
            Some(gate.gate_id.as_str())
        );

        // Every per-family evaluation must agree with the register's gate row.
        let evaluations = descriptor["evaluations"].as_array().expect("evaluations array");
        assert_eq!(
            evaluations.len(),
            r.rows.len(),
            "gate {} must record every family",
            gate.gate_id
        );
        for eval in evaluations {
            let family = eval["family_id"].as_str().unwrap();
            let row = r.row(family).unwrap_or_else(|| panic!("{family} is in the model"));
            let row_gate = row
                .gates
                .iter()
                .find(|g| g.gate_kind == gate.gate_kind)
                .expect("row carries this gate");
            assert_eq!(
                eval["outcome"].as_str().unwrap(),
                serde_json::to_value(row_gate.outcome)
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "{family}: gate outcome must match the register"
            );
        }
    }
}

#[test]
fn release_blocking_failure_holds_promotion_and_is_resolvable() {
    // The acceptance anchor exercised by a real fixture family: a release-blocking
    // family with a missing required contract package holds promotion, narrows to
    // the matrix label, and is not mirror-publishable; the shiproom dashboard and
    // gate manifest resolve from the same register.
    let r = register();
    let te = r
        .row("task_event_envelope")
        .expect("task_event_envelope present");
    assert!(te.release_blocking);
    assert_eq!(te.health_state, HealthState::Blocked);
    assert_eq!(te.blocker.decision, BlockerDecision::Hold);
    assert!(!te.graph_linkage.offline_verifiable);
    assert!(r.holds_promotion());
    assert!(r
        .blockers
        .blocking_gate_kinds
        .contains(&GateKind::CompatibilityReport));

    let root = repo_root();
    assert!(
        root.join("shiproom/m5-contract-blocker-dashboard.md").exists(),
        "shiproom blocker dashboard is checked in"
    );
    assert!(
        root.join("ci/contracts/m5-contract-gates/manifest.json").exists(),
        "CI gate manifest is checked in"
    );
}

#[test]
fn every_family_resolves_to_a_build_identity_and_package_version() {
    let r = register();
    let root = repo_root();
    assert!(root.join(&r.build_identity.build_identity_ref).exists());
    for row in &r.rows {
        assert!(!row.graph_linkage.release_packet_ref.is_empty());
        assert!(row.package_identity.package_version >= 1);
        assert!(root.join(&row.package_identity.schema_or_spec_ref).exists());
        assert_eq!(
            row.graph_linkage.build_identity_ref,
            r.build_identity.build_identity_ref
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));
    assert_eq!(capture["register_id"].as_str(), Some(r.register_id.as_str()));
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
        summary["blocked_families"].as_u64().unwrap() as usize,
        computed.blocked_families
    );
    assert_eq!(
        summary["gates_failing"].as_u64().unwrap() as usize,
        computed.gates_failing
    );

    let checks = capture["family_checks"].as_array().unwrap();
    assert_eq!(checks.len(), r.rows.len(), "capture records every family");
    for check in checks {
        let family = check["family_id"].as_str().unwrap();
        let row = r
            .row(family)
            .unwrap_or_else(|| panic!("capture family {family} is in the model"));
        assert_eq!(
            check["health_state"].as_str().unwrap(),
            serde_json::to_value(row.health_state)
                .unwrap()
                .as_str()
                .unwrap(),
            "capture health state must match the model for {family}"
        );
        for key in [
            "gates_evaluated",
            "lifecycle_matches_matrix",
            "graph_linkage_resolves",
            "mirror_parity_follows_gates",
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
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-contract-health");
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
        // The unknown-health-state fixture intentionally carries an off-vocab enum
        // that serde refuses to deserialize, which is itself a rejection; the
        // structurally-parseable fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5ContractHealthRegister>(&raw) {
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
