//! Protected tests binding the typed M5 depth-train automation register to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the lane-kind
//! coverage counts, the packet-freshness counts, and the evidence-expiry counts;
//! the negative cases mutate a parsed copy and the checked-in fixtures to prove
//! that a lane which fails to narrow, a held row with an active gap, a row carried
//! wider than its public claim's ceiling, and a promotion verdict that disagrees
//! with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains::{
    current_m5_depth_train_automation_register, AutomationGapReason, AutomationState,
    M5DepthTrainAutomationRegister, M5DepthTrainAutomationViolation,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains_validation_capture.json"
));

fn register() -> M5DepthTrainAutomationRegister {
    current_m5_depth_train_automation_register().expect("checked-in register parses into the model")
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
    assert_eq!(
        r.schema_version,
        GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION
    );
    assert_eq!(
        r.record_kind,
        GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND
    );
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_lane_kind() {
    let r = register();
    use aureline_release::freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph::M5LaneKind;
    for kind in M5LaneKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "lane kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let r = register();
    assert!(!r.release_blocking_lane_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &r.release_blocking_lane_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
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

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        r.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_holding_stable"].as_u64().unwrap() as usize,
        r.rows_published_stable().len(),
        "capture holding count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        r.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        r.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );
    assert_eq!(
        summary["entries_expired_evidence"].as_u64().unwrap() as usize,
        r.computed_summary().entries_expired_evidence,
        "capture expired-evidence count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        r.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());

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
fn register_narrows_a_release_blocking_row() {
    let r = register();
    let narrowed = r
        .rows
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn register_shows_a_row_on_waiver() {
    let r = register();
    let on_waiver = r
        .rows
        .iter()
        .find(|row| row.automation_state == AutomationState::OnWaiver)
        .expect("the register must show a row on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| {
            row.automation_state == AutomationState::Incomplete
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has an incomplete row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed row");
    row.active_gap_reasons.push(AutomationGapReason::ProofPacketMissing);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5DepthTrainAutomationViolation::HeldWithActiveGap { .. })),
        "a backed row may not carry an active gap reason"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5DepthTrainAutomationViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains",
    );
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: M5DepthTrainAutomationRegister =
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
