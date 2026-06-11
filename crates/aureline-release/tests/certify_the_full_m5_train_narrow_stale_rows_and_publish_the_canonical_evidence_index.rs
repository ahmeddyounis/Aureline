//! Protected tests binding the typed publication-pack register to the
//! checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the
//! readiness-cell counts, and the packet-freshness counts; the negative cases
//! mutate a parsed copy and the checked-in fixtures to prove that a surface that
//! fails to narrow, a held surface with an active gap, a held surface that does
//! not disclose its redaction posture, a held surface without verified downgrade
//! automation, a surface missing a dimension, and a promotion verdict that
//! disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index::{
    current_m5_cert_train_register, NarrowingReason, M5CertTrainRegister,
    M5CertTrainViolation, ReadinessDimension, SurfaceKind, SurfaceState,
    M5_CERT_TRAIN_RECORD_KIND, M5_CERT_TRAIN_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index_validation_capture.json"
));

fn register() -> M5CertTrainRegister {
    current_m5_cert_train_register().expect("checked-in register parses into the model")
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
    assert_eq!(r.schema_version, M5_CERT_TRAIN_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_CERT_TRAIN_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_surface_kind_and_dimension() {
    let r = register();
    for kind in SurfaceKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "surface kind {} must have at least one surface",
            kind.as_str()
        );
    }
    for row in &r.rows {
        for dimension in ReadinessDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "surface {} must cover dimension {}",
                row.entry_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let r = register();
    assert!(!r.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &r.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking surface"
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
    let computed = r.computed_summary();
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        r.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_certified"].as_u64().unwrap() as usize,
        r.rows_published_stable().len(),
        "capture certified count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        r.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["entries_with_automation_gap"].as_u64().unwrap() as usize,
        computed.entries_with_automation_gap,
        "capture automation-gap count must match the model"
    );
    assert_eq!(
        summary["entries_redaction_undisclosed"].as_u64().unwrap() as usize,
        computed.entries_redaction_undisclosed,
        "capture redaction-undisclosed count must match the model"
    );
    assert_eq!(
        summary["packets_missing"].as_u64().unwrap() as usize,
        computed.packets_missing,
        "capture missing-packet count must match the model"
    );
    assert_eq!(
        summary["total_readiness_cells"].as_u64().unwrap() as usize,
        computed.total_readiness_cells,
        "capture total-cell count must match the model"
    );
    assert_eq!(
        summary["cells_pass"].as_u64().unwrap() as usize,
        computed.cells_pass,
        "capture pass-cell count must match the model"
    );
    assert_eq!(
        summary["cells_missing"].as_u64().unwrap() as usize,
        computed.cells_missing,
        "capture missing-cell count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        computed.rules_firing,
        "capture firing-rule count must match the model"
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
fn register_narrows_a_release_blocking_surface() {
    let r = register();
    let narrowed = r
        .rows
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking surface under a still-stable claim"
    );
}

#[test]
fn register_shows_a_surface_on_waiver() {
    let r = register();
    let on_waiver = r
        .rows
        .iter()
        .find(|row| row.surface_state == SurfaceState::OnWaiver)
        .expect("the register must show a surface on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn register_narrows_a_surface_for_undefined_downgrade_automation() {
    let r = register();
    let narrowed = r.rows.iter().find(|row| {
        !row.publishes_stable()
            && row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one surface whose downgrade automation is undefined"
    );
}

#[test]
fn register_narrows_a_surface_without_redaction_disclosure() {
    let r = register();
    let undisclosed = r
        .rows
        .iter()
        .find(|row| !row.disclosure.redaction_disclosed);
    let undisclosed =
        undisclosed.expect("the register must show a surface without redaction disclosure");
    assert!(
        !undisclosed.publishes_stable(),
        "a surface that does not disclose its redaction posture must not publish a Stable claim"
    );
}

#[test]
fn narrowing_surface_that_does_not_narrow_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.holds_label() && row.claim_label == StableClaimLevel::Stable)
        .expect("register has a narrowed surface under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5CertTrainViolation::PublishedLabelNotNarrowed { .. })),
        "a surface that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_surface_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed surface");
    row.active_narrowing_reasons
        .push(NarrowingReason::ReadinessDimensionFailed);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5CertTrainViolation::HeldWithActiveGap { .. })),
        "a backed surface may not carry an active narrowing reason"
    );
}

#[test]
fn backed_surface_without_redaction_disclosure_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed surface");
    row.disclosure.redaction_disclosed = false;
    r.summary = r.computed_summary();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5CertTrainViolation::HeldWithoutRedactionDisclosure { .. }
        )),
        "a backed surface must disclose its redaction posture"
    );
}

#[test]
fn backed_surface_on_a_breached_packet_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed surface");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5CertTrainViolation::HeldOnStalePacket { .. })),
        "a backed surface may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn backed_surface_without_verified_downgrade_automation_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed surface");
    row.downgrade_automation.rollback_verified = false;
    r.summary = r.computed_summary();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5CertTrainViolation::HeldWithoutDowngradeAutomation { .. }
        )),
        "a backed surface must ride defined+verified downgrade automation"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5CertTrainViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index",
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
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: M5CertTrainRegister =
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
