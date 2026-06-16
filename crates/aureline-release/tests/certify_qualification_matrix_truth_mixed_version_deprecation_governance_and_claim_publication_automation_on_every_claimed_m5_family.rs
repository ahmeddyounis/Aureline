//! Protected tests binding the typed M5 per-family certification register to the
//! checked-in artifact, the frozen CI validation capture, the upstream qualification
//! matrix and public claim-publication manifest it certifies, and the negative fixtures.
//!
//! The positive case is the checked-in register; the join check proves every
//! certification packet reuses a real qualification row and public claim entry at
//! parity and is never greener than them; the capture cross-check proves the typed
//! model and the CI gate agree on the promotion verdict, the certified/narrowed counts,
//! and the pillar counts; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a family that over-claims its public label, a certified family
//! with an active gap, a family missing a required governance pillar, a family whose
//! pillar ref drifted, a family that loses its retest-needed reason, and a promotion
//! verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::current_m5_claim_publication_manifests;
use aureline_release::certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family::{
    current_m5_family_certification, CertificationPillarKind, CertificationReason,
    CertificationState, CertificationViolation, M5FamilyCertificationRegister,
    M5_FAMILY_CERTIFICATION_RECORD_KIND, M5_FAMILY_CERTIFICATION_SCHEMA_VERSION,
};
use aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    current_m5_qualification_and_skew_matrix, FamilyKind,
};
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family_validation_capture.json"
));

fn register() -> M5FamilyCertificationRegister {
    current_m5_family_certification().expect("checked-in register parses into the model")
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
    assert_eq!(r.schema_version, M5_FAMILY_CERTIFICATION_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_FAMILY_CERTIFICATION_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_and_binds_four_pillars() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
    for row in &r.rows {
        let bound: std::collections::BTreeSet<CertificationPillarKind> =
            row.pillars.iter().map(|p| p.kind).collect();
        for kind in CertificationPillarKind::REQUIRED {
            assert!(
                bound.contains(&kind),
                "row {} must bind governance pillar {}",
                row.entry_id,
                kind.as_str()
            );
        }
    }
}

#[test]
fn every_family_certifies_a_real_qualification_row_and_public_claim_at_parity() {
    let r = register();
    let matrix = current_m5_qualification_and_skew_matrix().expect("qualification matrix parses");
    let claims = current_m5_claim_publication_manifests().expect("claim manifest parses");
    for row in &r.rows {
        let qrow = matrix.row(&row.qualification_row_ref).unwrap_or_else(|| {
            panic!(
                "row {} joins qualification row {} which must exist",
                row.entry_id, row.qualification_row_ref
            )
        });
        assert_eq!(
            qrow.family_ref, row.family_ref,
            "row {} family ref must match the joined qualification row",
            row.entry_id
        );
        assert_eq!(
            qrow.published_label, row.source_published_label,
            "row {} must mirror the qualification row's published label",
            row.entry_id
        );

        let claim = claims
            .manifest(&row.claim_manifest_entry_ref)
            .unwrap_or_else(|| {
                panic!(
                    "row {} reuses claim {} which must exist in the claim manifest",
                    row.entry_id, row.claim_manifest_entry_ref
                )
            });
        assert_eq!(
            claim.family_ref, row.family_ref,
            "row {} family ref must match the reused public claim",
            row.entry_id
        );
        assert_eq!(
            claim.published_label, row.source_published_label,
            "row {} must mirror the public claim's published label",
            row.entry_id
        );
        assert_eq!(
            claim.published_claim.support_class, row.source_support_class,
            "row {} must mirror the public claim's support class",
            row.entry_id
        );
        assert_eq!(
            claim.published_claim.claim_text, row.source_claim_text,
            "row {} must mirror the public claim text verbatim",
            row.entry_id
        );
        assert!(
            row.certified_label.rank() <= row.source_published_label.rank(),
            "row {} may never certify greener than its public claim",
            row.entry_id
        );
        assert!(
            !row.over_claims_source(),
            "row {} may never over-claim the public label or support class",
            row.entry_id
        );
    }
}

#[test]
fn keeps_pillar_level_truth_and_reopen_refs() {
    let r = register();
    // The certification keeps distinct per-row states (never one global flag) and the
    // active stale/retest reasons, and every family binds the four pillars with reopen
    // refs.
    let states: std::collections::BTreeSet<CertificationState> =
        r.rows.iter().map(|row| row.certification_state).collect();
    assert!(states.contains(&CertificationState::Certified));
    assert!(states.contains(&CertificationState::NarrowedRetestPending));
    assert!(states.contains(&CertificationState::NarrowedStale));
    assert!(states.contains(&CertificationState::NarrowedRowDowngraded));

    // A pillar can be stale while the family's other pillars stay current.
    assert!(r.summary.pillars_stale >= 1);

    let reasons: std::collections::BTreeSet<CertificationReason> = r
        .rows
        .iter()
        .flat_map(|row| row.active_certification_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&CertificationReason::RowDowngraded));
    assert!(reasons.contains(&CertificationReason::RetestPending));
    assert!(reasons.contains(&CertificationReason::QualificationStale));
    assert!(reasons.contains(&CertificationReason::EvidenceStale));

    for row in &r.rows {
        assert_eq!(
            row.pillar(CertificationPillarKind::QualificationMatrix)
                .map(|p| p.pillar_ref.as_str()),
            Some(row.qualification_row_ref.as_str())
        );
        assert_eq!(
            row.pillar(CertificationPillarKind::ClaimPublication)
                .map(|p| p.pillar_ref.as_str()),
            Some(row.claim_manifest_entry_ref.as_str())
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
        summary["total_rows"].as_u64().unwrap() as usize,
        r.rows.len()
    );
    assert_eq!(
        summary["rows_certified"].as_u64().unwrap() as usize,
        r.rows_certified().len()
    );
    assert_eq!(
        summary["rows_narrowed"].as_u64().unwrap() as usize,
        r.rows_narrowed().len()
    );
    assert_eq!(
        summary["total_pillars"].as_u64().unwrap() as usize,
        computed.total_pillars
    );
    assert_eq!(
        summary["pillars_stale"].as_u64().unwrap() as usize,
        computed.pillars_stale
    );
    assert_eq!(
        summary["total_active_certification_reasons"]
            .as_u64()
            .unwrap() as usize,
        computed.total_active_certification_reasons
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
fn register_narrows_a_family_under_a_still_stable_public_claim() {
    let r = register();
    let narrowed = r
        .rows
        .iter()
        .find(|row| row.release_blocking && row.source_holds_stable() && !row.certifies_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking family under a still-stable public claim"
    );
}

#[test]
fn certification_layer_failure_holds_promotion_but_inherited_narrowing_does_not() {
    let r = register();
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
    // The companion family only inherits an upstream qualification narrowing.
    let companion = r.row("cert-companion-handoff").expect("companion row");
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
    // The remote-helper family rides a still-Stable public claim with stale certification
    // evidence.
    let remote = r.row("cert-remote-helper-skew").expect("remote helper row");
    assert!(remote.has_active_reason(CertificationReason::EvidenceStale));
    assert!(r.computed_blocking_claim_ids().contains(&remote.entry_id));
}

#[test]
fn family_over_claiming_the_public_label_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.source_published_label.is_at_or_above_cutline())
        .expect("register has a row reusing a below-cutline public claim");
    row.certified_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_claim_ids();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, CertificationViolation::RowLabelExceedsSource { .. })),
        "a family may not certify greener than the public claim it reuses"
    );
}

#[test]
fn certified_family_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.holds_certification())
        .expect("register has a certified family");
    row.active_certification_reasons
        .push(CertificationReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, CertificationViolation::CertifiedWithActiveGap { .. })),
        "a certified family may not carry an active narrowing reason"
    );
}

#[test]
fn missing_required_pillar_fails() {
    let mut r = register();
    r.rows[0]
        .pillars
        .retain(|p| p.kind != CertificationPillarKind::DiffDeprecation);
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, CertificationViolation::RequiredPillarUncovered { .. })),
        "every family must bind all four governance pillars"
    );
}

#[test]
fn losing_a_retest_reason_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.has_active_reason(CertificationReason::RetestPending))
        .expect("a retest-pending family exists");
    row.active_certification_reasons
        .retain(|reason| *reason != CertificationReason::RetestPending);
    r.summary = r.computed_summary();
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, CertificationViolation::RowStateWithoutReason { .. })),
        "dropping the retest reason must lose the row-level retest-needed truth and be rejected"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            CertificationViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a certification-layer stop rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-family-certification");
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
        let candidate: M5FamilyCertificationRegister =
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
