//! Protected tests binding the typed M5 admin/policy story register to the
//! checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the publication verdict, the
//! lane-kind coverage counts, the packet-freshness counts, the story-item
//! counts, and the gap-reason counts; the negative cases mutate a parsed copy
//! and the checked-in fixtures to prove that a lane which fails to narrow, a
//! held row with an active gap, a row carried wider than its public claim's
//! ceiling, and a publication verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes::{
    current_m5_admin_policy_story_register, AdminPolicyGapReason, M5AdminPolicyLaneKind,
    AdminPolicyLaneState, M5AdminPolicyRegisterViolation, M5AdminPolicyStoryRegister,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes_validation_capture.json"
));

fn register() -> M5AdminPolicyStoryRegister {
    current_m5_admin_policy_story_register().expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let reg = register();
    assert_eq!(
        reg.schema_version,
        PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION
    );
    assert_eq!(
        reg.record_kind,
        PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND
    );
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_lane_kind() {
    let reg = register();
    for kind in M5AdminPolicyLaneKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "lane kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let reg = register();
    assert!(!reg.release_blocking_lane_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_lane_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_holding_stable"].as_u64().unwrap() as usize,
        reg.rows_published_stable().len(),
        "capture holding count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        reg.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );
    assert_eq!(
        summary["story_items_stale"].as_u64().unwrap() as usize,
        reg.computed_summary().story_items_stale,
        "capture stale-story-item count must match the model"
    );
    assert_eq!(
        summary["story_items_missing"].as_u64().unwrap() as usize,
        reg.computed_summary().story_items_missing,
        "capture missing-story-item count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        reg.publication.decision,
        reg.computed_publication_decision()
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
fn register_narrows_a_release_blocking_row() {
    let reg = register();
    let narrowed = reg
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
    let reg = register();
    let on_waiver = reg
        .rows
        .iter()
        .find(|row| row.lane_state == AdminPolicyLaneState::OnWaiver)
        .expect("the register must show a row on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.lane_state == AdminPolicyLaneState::Incomplete
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has an incomplete row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.publication.decision = reg.computed_publication_decision();
    reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.publication.blocking_claim_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            M5AdminPolicyRegisterViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_with_active_gap_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed row");
    row.active_gap_reasons
        .push(AdminPolicyGapReason::ProofPacketMissing);
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, M5AdminPolicyRegisterViolation::HeldWithActiveGap { .. })),
        "a backed row may not carry an active gap reason"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, M5AdminPolicyRegisterViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut reg = register();
    reg.publication.decision = PromotionDecision::Proceed;

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            M5AdminPolicyRegisterViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root()
        .join("fixtures/release/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_violation"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: M5AdminPolicyStoryRegister =
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
