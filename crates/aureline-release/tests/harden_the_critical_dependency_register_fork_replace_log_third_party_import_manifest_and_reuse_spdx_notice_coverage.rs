//! Protected tests binding the typed dependency and licensing governance
//! artifact to the checked-in JSON and the frozen CI validation capture.
//!
//! The positive case is the frozen, checked-in artifact; the capture cross-check
//! proves the typed model and the CI gate agree on the publication verdict
//! and summary; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a row which fails to narrow, a current row with an
//! active gap reason, or a publication verdict that disagrees with the firing
//! rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage::{
    current_harden_critical_dependency_register, HardenCriticalDependencyRegister,
    HardenCriticalDependencyRegisterViolation, LaneGapReason, LaneState, PublicationDecision,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage_validation_capture.json"
));

fn artifact() -> HardenCriticalDependencyRegister {
    current_harden_critical_dependency_register()
        .expect("checked-in artifact parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_artifact_parses_and_validates() {
    let artifact = artifact();
    assert_eq!(
        artifact.schema_version,
        HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION
    );
    assert_eq!(
        artifact.record_kind,
        HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND
    );
    let violations = artifact.validate();
    assert!(
        violations.is_empty(),
        "checked-in artifact must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_holding_and_narrowed_rows() {
    let artifact = artifact();
    assert!(
        !artifact.rows_holding_claim().is_empty(),
        "artifact must hold at least one claim"
    );
    assert!(
        !artifact.rows_narrowed().is_empty(),
        "artifact must narrow at least one claim"
    );
}

#[test]
fn every_lane_kind_is_present() {
    use aureline_release::harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage::LaneKind;

    let artifact = artifact();
    let present: std::collections::BTreeSet<LaneKind> =
        artifact.rows.iter().map(|r| r.lane_kind).collect();
    for kind in LaneKind::ALL {
        assert!(
            present.contains(&kind),
            "missing lane kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn publication_decision_matches_computed() {
    let artifact = artifact();
    assert_eq!(
        artifact.publication.decision,
        artifact.computed_publication_decision()
    );
    assert_eq!(
        artifact.publication.blocking_rule_ids,
        artifact.computed_blocking_rule_ids()
    );
    assert_eq!(
        artifact.publication.blocking_row_ids,
        artifact.computed_blocking_row_ids()
    );
}

#[test]
fn narrowed_row_that_does_not_narrow_fails() {
    let mut artifact = artifact();
    let row = artifact
        .rows
        .iter_mut()
        .find(|r| r.lane_state == LaneState::NarrowedStale)
        .expect("artifact has a narrowed-stale row");
    row.effective_label = row.claim_label.clone();
    artifact.summary = artifact.computed_summary();
    artifact.publication.decision = artifact.computed_publication_decision();
    artifact.publication.blocking_rule_ids = artifact.computed_blocking_rule_ids();
    artifact.publication.blocking_row_ids = artifact.computed_blocking_row_ids();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenCriticalDependencyRegisterViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a narrowing state must drop the effective label below the claim label"
    );
}

#[test]
fn current_row_with_active_gap_fails() {
    let mut artifact = artifact();
    let row = artifact
        .rows
        .iter_mut()
        .find(|r| r.lane_state == LaneState::Current)
        .expect("artifact has a current row");
    row.active_gap_reasons
        .push(LaneGapReason::PacketFreshnessBreached);
    artifact.summary = artifact.computed_summary();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenCriticalDependencyRegisterViolation::HeldRowWithActiveGap { .. }
        )),
        "a current row may not carry an active gap reason"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut artifact = artifact();
    artifact.publication.decision = PublicationDecision::Proceed;

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenCriticalDependencyRegisterViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let artifact = artifact();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(artifact.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        artifact.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_holding_claim"].as_u64().unwrap() as usize,
        artifact.rows_holding_claim().len(),
        "capture holding count must match the model"
    );
    assert_eq!(
        summary["rows_narrowed"].as_u64().unwrap() as usize,
        artifact.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["rows_on_active_waiver"].as_u64().unwrap() as usize,
        artifact.computed_summary().rows_on_active_waiver,
        "capture waiver count must match the model"
    );
    assert_eq!(
        summary["total_active_gap_reasons"].as_u64().unwrap() as usize,
        artifact.computed_summary().total_active_gap_reasons,
        "capture gap-reason count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        artifact.computed_summary().rules_firing,
        "capture rules-firing count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        artifact.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        artifact.publication.decision,
        artifact.computed_publication_decision()
    );

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
    let fixtures_dir = repo_root().join(
        "fixtures/release/m4/harden-the-critical-dependency-register-fork-replace-log-third-party-import-manifest-and-reuse-spdx-notice-coverage",
    );
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: HardenCriticalDependencyRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
