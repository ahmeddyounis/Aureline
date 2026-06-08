//! Protected tests binding the typed stable proof index to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the frozen, checked-in index; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict, the
//! launch-blocking coverage counts, and the packet-freshness counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that a row which
//! fails to narrow, a proven row riding a breached packet, a proof backed wider than
//! its public claim's ceiling, and a publication verdict that disagrees with the
//! firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stable_proof_index::{
    current_stable_proof_index, GapReason, ProofState, StableProofIndex, StableProofIndexViolation,
    STABLE_PROOF_INDEX_RECORD_KIND, STABLE_PROOF_INDEX_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_proof_index_validation_capture.json"
));

fn index() -> StableProofIndex {
    current_stable_proof_index().expect("checked-in stable proof index parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_index_parses_and_validates() {
    let index = index();
    assert_eq!(index.schema_version, STABLE_PROOF_INDEX_SCHEMA_VERSION);
    assert_eq!(index.record_kind, STABLE_PROOF_INDEX_RECORD_KIND);
    let violations = index.validate();
    assert!(
        violations.is_empty(),
        "checked-in index must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_declared_launch_blocking_requirement() {
    let index = index();
    assert!(!index.launch_blocking_requirement_refs.is_empty());
    let covered: Vec<&str> = index
        .launch_blocking_rows()
        .into_iter()
        .map(|row| row.requirement_ref.as_str())
        .collect();
    for declared in &index.launch_blocking_requirement_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering launch-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let index = index();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(index.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_requirements"].as_u64().unwrap() as usize,
        index.rows.len(),
        "capture requirement count must match the model"
    );
    assert_eq!(
        summary["requirements_proven_stable"].as_u64().unwrap() as usize,
        index.rows_proven_stable().len(),
        "capture proven count must match the model"
    );
    assert_eq!(
        summary["launch_blocking_total"].as_u64().unwrap() as usize,
        index.computed_summary().launch_blocking_total,
        "capture launch-blocking total must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        index.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        index.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        index.publication.decision,
        index.computed_publication_decision()
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
fn index_proves_requirements_without_narrowing() {
    let index = index();
    assert!(
        index.rows_narrowed().is_empty(),
        "clean proof index must not narrow a requirement"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut index = index();
    let row = index
        .rows
        .iter_mut()
        .find(|row| row.holds_proof())
        .expect("index has a proven row");
    row.index_state = ProofState::UnprovenStale;
    row.active_gap_reasons
        .push(GapReason::ProofPacketFreshnessBreached);
    row.proven_label = StableClaimLevel::Stable;
    index.summary = index.computed_summary();
    index.publication.decision = index.computed_publication_decision();
    index.publication.blocking_rule_ids = index.computed_blocking_rule_ids();
    index.publication.blocking_proof_ids = index.computed_blocking_proof_ids();

    assert!(
        index
            .validate()
            .iter()
            .any(|v| matches!(v, StableProofIndexViolation::ProvenLabelNotNarrowed { .. })),
        "a requirement that is not proven must narrow below the cutline"
    );
}

#[test]
fn proven_row_on_a_breached_packet_fails() {
    let mut index = index();
    let row = index
        .rows
        .iter_mut()
        .find(|row| row.holds_proof())
        .expect("index has a proven row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    index.summary = index.computed_summary();

    assert!(
        index
            .validate()
            .iter()
            .any(|v| matches!(v, StableProofIndexViolation::HeldOnStalePacket { .. })),
        "a proven row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_decision_mismatch_fails() {
    let mut index = index();
    index.publication.decision = PromotionDecision::Hold;

    assert!(
        index.validate().iter().any(|v| matches!(
            v,
            StableProofIndexViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication decision must agree with computed rules"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stable_proof_index");
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
        let candidate: StableProofIndex =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
