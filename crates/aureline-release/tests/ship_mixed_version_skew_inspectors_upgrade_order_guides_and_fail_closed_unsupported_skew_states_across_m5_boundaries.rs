//! Protected tests binding the typed M5 boundary skew-inspector register to the
//! checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the gate-posture
//! counts, the upgrade-step count, and the packet-freshness counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that an
//! inspector that fails to narrow, a held inspector with an active gap, a fail-closed
//! verdict whose gate still allows the action, a fail-closed verdict missing its
//! upgrade-order guide, and a promotion verdict that disagrees with the firing rules
//! all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries::{
    current_m5_boundary_skew_inspectors, BoundaryKind, BoundarySkewInspectorRegister,
    BoundarySkewInspectorViolation, DowngradeSubject, GatePosture, InspectorState, InspectorVerdict,
    NarrowingReason, SkewWindowClass, UpgradeLeadSide,
    SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND, SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries_validation_capture.json"
));

fn register() -> BoundarySkewInspectorRegister {
    current_m5_boundary_skew_inspectors().expect("checked-in register parses into the model")
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
        SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_boundary_kind_and_subject() {
    let r = register();
    for kind in BoundaryKind::ALL {
        assert!(
            !r.inspectors_for_kind(kind).is_empty(),
            "boundary kind {} must have at least one inspector",
            kind.as_str()
        );
    }
    for subject in DowngradeSubject::ALL {
        assert!(
            !r.inspectors_for_subject(subject).is_empty(),
            "downgrade subject {} must have at least one inspector",
            subject.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_boundary() {
    let r = register();
    assert!(!r.release_blocking_boundary_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_inspectors()
        .into_iter()
        .map(|row| row.boundary_ref.as_str())
        .collect();
    for declared in &r.release_blocking_boundary_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking inspector"
        );
    }
}

#[test]
fn exercises_the_skew_verdict_and_downgrade_vocabulary() {
    let r = register();
    let verdicts: std::collections::BTreeSet<InspectorVerdict> =
        r.inspectors.iter().map(|row| row.verdict).collect();
    for verdict in [
        InspectorVerdict::InsideWindow,
        InspectorVerdict::UnsupportedSkew,
        InspectorVerdict::ReconnectRequired,
        InspectorVerdict::ReinstallRequired,
        InspectorVerdict::MigrationNeeded,
        InspectorVerdict::RetestPending,
    ] {
        assert!(
            verdicts.contains(&verdict),
            "the register must exercise the {} verdict",
            verdict.as_str()
        );
    }

    let skew: std::collections::BTreeSet<SkewWindowClass> = r
        .inspectors
        .iter()
        .map(|row| row.skew_window.skew_window_class)
        .collect();
    assert!(
        skew.contains(&SkewWindowClass::UnsupportedSkew),
        "at least one inspector must exercise an unsupported skew window"
    );
    assert!(
        skew.contains(&SkewWindowClass::LockstepOnly),
        "at least one inspector must exercise a lockstep-only skew window"
    );

    // Every gated boundary records whether the mutating-or-privileged action is
    // allowed or fails closed.
    assert!(
        r.inspectors.iter().any(|row| row.action_allowed()),
        "at least one boundary must allow its gated action"
    );
    assert!(
        r.inspectors.iter().any(|row| !row.action_allowed()),
        "at least one boundary must fail closed on its gated action"
    );

    // The lead side of every upgrade-order guide is recorded; recovery from a
    // fail-closed skew verdict prescribes an actual upgrade order.
    let leads: std::collections::BTreeSet<UpgradeLeadSide> = r
        .inspectors
        .iter()
        .map(|row| row.upgrade_order_guide.lead_side)
        .collect();
    assert!(leads.contains(&UpgradeLeadSide::NoneRequired));
    assert!(leads.iter().any(|s| s.requires_upgrade()));
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
        summary["total_inspectors"].as_u64().unwrap() as usize,
        r.inspectors.len(),
        "capture inspector count must match the model"
    );
    assert_eq!(
        summary["inspectors_publishing_stable"].as_u64().unwrap() as usize,
        r.inspectors_publishing_stable().len(),
        "capture publishing-stable count must match the model"
    );
    assert_eq!(
        summary["inspectors_narrowed"].as_u64().unwrap() as usize,
        r.inspectors_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["gate_fail_closed"].as_u64().unwrap() as usize,
        computed.gate_fail_closed,
        "capture fail-closed gate count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        computed.packets_breached,
        "capture breached-packet count must match the model"
    );
    assert_eq!(
        summary["total_upgrade_steps"].as_u64().unwrap() as usize,
        computed.total_upgrade_steps,
        "capture upgrade-step count must match the model"
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

    let captured_rules: Vec<&str> = capture["promotion"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(
        captured_rules,
        r.computed_blocking_rule_ids(),
        "capture blocking rule ids must match the model"
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
fn register_narrows_a_release_blocking_boundary() {
    let r = register();
    let narrowed = r
        .inspectors
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking boundary under a still-stable claim"
    );
}

#[test]
fn register_shows_a_boundary_on_waiver() {
    let r = register();
    let on_waiver = r
        .inspectors
        .iter()
        .find(|row| row.inspector_state == InspectorState::OnWaiver)
        .expect("the register must show a boundary on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn register_shows_an_allowed_action_with_a_narrowed_claim() {
    // A boundary whose skew is inside the window allows its action, yet its support
    // claim still narrows on stale or missing evidence.
    let r = register();
    let narrowed_but_allowed = r.inspectors.iter().find(|row| {
        row.action_allowed() && row.verdict.is_inside_window() && !row.publishes_stable()
    });
    assert!(
        narrowed_but_allowed.is_some(),
        "the register must show an in-window boundary whose claim narrows on evidence"
    );
}

#[test]
fn narrowing_boundary_that_does_not_narrow_fails() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| !row.holds_label() && row.claim_label == StableClaimLevel::Stable)
        .expect("register has a narrowed inspector under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            BoundarySkewInspectorViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a boundary that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_boundary_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed inspector");
    row.active_narrowing_reasons
        .push(NarrowingReason::SkewWindowExceeded);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, BoundarySkewInspectorViolation::HeldWithActiveGap { .. })),
        "a backed boundary may not carry an active narrowing reason"
    );
}

#[test]
fn allowing_a_fail_closed_verdict_fails() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| !row.verdict.is_inside_window())
        .expect("register has a fail-closed inspector");
    row.gate_posture = GatePosture::Allow;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            BoundarySkewInspectorViolation::GatePostureIncoherent { .. }
        )),
        "a fail-closed verdict may not allow the mutating-or-privileged action"
    );
}

#[test]
fn backed_boundary_on_a_breached_packet_fails() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed inspector");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, BoundarySkewInspectorViolation::HeldOnStalePacket { .. })),
        "a backed boundary may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            BoundarySkewInspectorViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-boundary-skew-inspectors");
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
        let candidate: BoundarySkewInspectorRegister =
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
