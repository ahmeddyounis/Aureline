//! Protected tests binding the typed M5 qualification-row badge binding register
//! to the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the
//! published/narrowed counts, and the artifact-freshness counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that a badge
//! that over-claims its row, a published badge with an active gap, a badge that
//! hides its freshness, a binding that drops a product-truth surface, and a
//! promotion verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family::{
    current_m5_qualification_badge_bindings, ArtifactState, BadgeSurface, BindingNarrowingReason,
    BindingState, QualificationBadgeBindingRegister, QualificationBadgeBindingViolation,
    BIND_M5_QUALIFICATION_BADGE_BINDINGS_RECORD_KIND,
    BIND_M5_QUALIFICATION_BADGE_BINDINGS_SCHEMA_VERSION,
};
use aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    current_m5_qualification_and_skew_matrix, FamilyKind,
};
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family_validation_capture.json"
));

fn register() -> QualificationBadgeBindingRegister {
    current_m5_qualification_badge_bindings().expect("checked-in register parses into the model")
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
        BIND_M5_QUALIFICATION_BADGE_BINDINGS_SCHEMA_VERSION
    );
    assert_eq!(
        r.record_kind,
        BIND_M5_QUALIFICATION_BADGE_BINDINGS_RECORD_KIND
    );
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_kind_and_truth_surface() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.bindings_for_kind(kind).is_empty(),
            "family kind {} must have at least one binding",
            kind.as_str()
        );
    }
    for b in &r.bindings {
        for surface in BadgeSurface::TRUTH_SURFACES {
            assert!(
                b.surfaces.contains(&surface),
                "binding {} must render the badge on truth surface {}",
                b.entry_id,
                surface.as_str()
            );
        }
    }
}

#[test]
fn every_binding_joins_a_real_qualification_row() {
    let r = register();
    let matrix = current_m5_qualification_and_skew_matrix().expect("matrix parses");
    for b in &r.bindings {
        let row = matrix.row(&b.qualification_row_ref).unwrap_or_else(|| {
            panic!(
                "binding {} joins qualification row {} which must exist in the matrix",
                b.entry_id, b.qualification_row_ref
            )
        });
        // The binding's claim and family must agree with the qualification row it
        // joins, and the badge may never exceed the row's published label.
        assert_eq!(
            row.family_ref, b.family_ref,
            "binding {} family must match the joined row",
            b.entry_id
        );
        assert_eq!(
            row.published_label, b.row_published_label,
            "binding {} must inherit the joined row's published label",
            b.entry_id
        );
        assert!(
            b.published_label.rank() <= b.row_published_label.rank(),
            "binding {} badge must never exceed its row",
            b.entry_id
        );
    }
}

#[test]
fn exercises_narrowing_and_artifact_vocabulary() {
    let r = register();
    let states: std::collections::BTreeSet<BindingState> =
        r.bindings.iter().map(|b| b.binding_state).collect();
    assert!(states.contains(&BindingState::Published));
    assert!(states.contains(&BindingState::NarrowedStale));
    assert!(states.contains(&BindingState::NarrowedRowDowngraded));

    let reasons: std::collections::BTreeSet<BindingNarrowingReason> = r
        .bindings
        .iter()
        .flat_map(|b| b.active_narrowing_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&BindingNarrowingReason::EvidenceStale));
    assert!(reasons.contains(&BindingNarrowingReason::EvaluationPackStale));
    assert!(reasons.contains(&BindingNarrowingReason::CompatibilityReportStale));

    let stale_eval = r
        .bindings
        .iter()
        .any(|b| b.evaluation_pack.state == ArtifactState::Stale);
    let stale_report = r
        .bindings
        .iter()
        .any(|b| b.compatibility_report.state == ArtifactState::Stale);
    assert!(stale_eval, "a binding must exercise a stale evaluation pack");
    assert!(stale_report, "a binding must exercise a stale compatibility report");

    // At least one badge must carry caveats that travel with it.
    assert!(r.bindings.iter().any(|b| !b.badge.caveat_summary.is_empty()));
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
        summary["total_bindings"].as_u64().unwrap() as usize,
        r.bindings.len()
    );
    assert_eq!(
        summary["bindings_published"].as_u64().unwrap() as usize,
        r.bindings_published().len()
    );
    assert_eq!(
        summary["bindings_narrowed"].as_u64().unwrap() as usize,
        r.bindings_narrowed().len()
    );
    assert_eq!(
        summary["badges_freshness_disclosed"].as_u64().unwrap() as usize,
        computed.badges_freshness_disclosed
    );
    assert_eq!(
        summary["evaluation_packs_stale"].as_u64().unwrap() as usize,
        computed.evaluation_packs_stale
    );
    assert_eq!(
        summary["compatibility_reports_stale"].as_u64().unwrap() as usize,
        computed.compatibility_reports_stale
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        computed.rules_firing
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(captured_decision, r.promotion.decision.as_str());
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());

    let captured_rules: Vec<&str> = capture["promotion"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rules, r.computed_blocking_rule_ids());

    let captured_claims: Vec<&str> = capture["promotion"]["blocking_claim_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_claims, r.computed_blocking_claim_ids());

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
fn register_narrows_a_release_blocking_badge() {
    let r = register();
    let narrowed = r
        .bindings
        .iter()
        .find(|b| b.release_blocking && b.claim_holds_stable() && !b.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking badge under a still-stable claim"
    );
}

#[test]
fn binding_layer_failure_holds_promotion_but_inherited_narrowing_does_not() {
    let r = register();
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
    // The companion badge only inherits an upstream row narrowing -> not a blocker.
    let companion = r.binding("m5-badge-companion").expect("companion binding");
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
    // The toolchain badge has a binding-layer evidence failure -> a blocker.
    let toolchain = r.binding("m5-badge-toolchain").expect("toolchain binding");
    assert!(toolchain.has_active_reason(BindingNarrowingReason::EvidenceStale));
    assert!(r
        .computed_blocking_claim_ids()
        .contains(&toolchain.entry_id));
}

#[test]
fn badge_over_claiming_the_row_fails() {
    let mut r = register();
    let b = r
        .bindings
        .iter_mut()
        .find(|b| !b.row_published_label.is_at_or_above_cutline())
        .expect("register has a binding inheriting a narrowed row");
    b.published_label = StableClaimLevel::Stable;
    b.badge.badge_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_claim_ids();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            QualificationBadgeBindingViolation::BadgePublishedWiderThanRow { .. }
        )),
        "a badge may not advertise wider than the qualification row it binds"
    );
}

#[test]
fn published_badge_with_active_gap_fails() {
    let mut r = register();
    let b = r
        .bindings
        .iter_mut()
        .find(|b| b.publishes_stable())
        .expect("register has a published binding");
    b.active_narrowing_reasons
        .push(BindingNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            QualificationBadgeBindingViolation::HeldWithActiveGap { .. }
        )),
        "a published badge may not carry an active narrowing reason"
    );
}

#[test]
fn badge_that_hides_freshness_fails() {
    let mut r = register();
    r.bindings[0].badge.freshness_disclosed = false;
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            QualificationBadgeBindingViolation::FreshnessNotDisclosed { .. }
        )),
        "evidence freshness must be visible wherever a support-class badge appears"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            QualificationBadgeBindingViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a binding-layer stop rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-qualification-row-badge-bindings");
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
        let candidate: QualificationBadgeBindingRegister =
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
