//! Protected tests binding the typed M5 claim-scope export-packet register to the
//! checked-in artifact, the frozen CI validation capture, the upstream qualification
//! matrix and public claim-publication manifest it joins, and the negative fixtures.
//!
//! The positive case is the checked-in register; the join check proves every export
//! row reuses a real qualification row and public claim entry and is never greener
//! than them; the capture cross-check proves the typed model and the CI gate agree on
//! the promotion verdict, the published/narrowed counts, and the evidence/audience
//! counts; the negative cases mutate a parsed copy and the checked-in fixtures to
//! prove that a row that over-claims its public label, a published row with an active
//! gap, an audience whose copy drifted from the row, an audience that hides its
//! freshness, a shiproom audience that drops its reopen ref, a row that loses its
//! retest-needed reason, a row that drops a required audience, and a promotion verdict
//! that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::current_m5_claim_publication_manifests;
use aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    current_m5_qualification_and_skew_matrix, FamilyKind,
};
use aureline_release::implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth::{
    current_m5_claim_scope_export_packets, ClaimScopeAudience, ClaimScopeExportRegister,
    ClaimScopeReason, ClaimScopeRowState, ClaimScopeViolation, ScopeEvidenceKind,
    M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND, M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth_validation_capture.json"
));

fn register() -> ClaimScopeExportRegister {
    current_m5_claim_scope_export_packets().expect("checked-in register parses into the model")
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
        M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_and_required_audience() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
    for row in &r.rows {
        let driven: std::collections::BTreeSet<ClaimScopeAudience> =
            row.audiences.iter().map(|a| a.audience).collect();
        for audience in ClaimScopeAudience::REQUIRED {
            assert!(
                driven.contains(&audience),
                "row {} must drive required audience {}",
                row.entry_id,
                audience.as_str()
            );
        }
    }
}

#[test]
fn every_row_reuses_a_real_qualification_row_and_public_claim() {
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
            row.published_label.rank() <= row.source_published_label.rank(),
            "row {} may never publish greener than its public claim",
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
fn keeps_row_level_truth_and_reopen_refs() {
    let r = register();
    // The export keeps distinct per-row states (never one global flag) and the active
    // stale/retest reasons, and every row carries the reopen refs.
    let states: std::collections::BTreeSet<ClaimScopeRowState> =
        r.rows.iter().map(|row| row.export_state).collect();
    assert!(states.contains(&ClaimScopeRowState::Published));
    assert!(states.contains(&ClaimScopeRowState::NarrowedRetestPending));
    assert!(states.contains(&ClaimScopeRowState::NarrowedStale));
    assert!(states.contains(&ClaimScopeRowState::NarrowedRowDowngraded));

    let reasons: std::collections::BTreeSet<ClaimScopeReason> = r
        .rows
        .iter()
        .flat_map(|row| row.active_scope_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ClaimScopeReason::RowDowngraded));
    assert!(reasons.contains(&ClaimScopeReason::RetestPending));
    assert!(reasons.contains(&ClaimScopeReason::QualificationStale));
    assert!(reasons.contains(&ClaimScopeReason::EvidenceStale));

    for row in &r.rows {
        let kinds: std::collections::BTreeSet<ScopeEvidenceKind> =
            row.evidence_refs.iter().map(|e| e.kind).collect();
        assert!(kinds.contains(&ScopeEvidenceKind::QualificationRow));
        assert!(kinds.contains(&ScopeEvidenceKind::ClaimManifest));
        // A shiproom rendering must always expose the reopen refs.
        let shiproom = row
            .audiences
            .iter()
            .find(|a| a.audience == ClaimScopeAudience::Shiproom)
            .expect("a shiproom rendering exists");
        assert!(shiproom.reopens_authoritative_row);
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
        summary["rows_published"].as_u64().unwrap() as usize,
        r.rows_published().len()
    );
    assert_eq!(
        summary["rows_narrowed"].as_u64().unwrap() as usize,
        r.rows_narrowed().len()
    );
    assert_eq!(
        summary["total_evidence_refs"].as_u64().unwrap() as usize,
        computed.total_evidence_refs
    );
    assert_eq!(
        summary["evidence_stale"].as_u64().unwrap() as usize,
        computed.evidence_stale
    );
    assert_eq!(
        summary["total_audiences"].as_u64().unwrap() as usize,
        computed.total_audiences
    );
    assert_eq!(
        summary["audiences_reasons_disclosed"].as_u64().unwrap() as usize,
        computed.audiences_reasons_disclosed
    );
    assert_eq!(
        summary["total_active_scope_reasons"].as_u64().unwrap() as usize,
        computed.total_active_scope_reasons
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
fn register_narrows_a_row_under_a_still_stable_public_claim() {
    let r = register();
    let narrowed = r
        .rows
        .iter()
        .find(|row| row.release_blocking && row.source_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable public claim"
    );
}

#[test]
fn export_layer_failure_holds_promotion_but_inherited_narrowing_does_not() {
    let r = register();
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
    // The companion row only inherits an upstream qualification narrowing.
    let companion = r
        .row("claim-scope-companion-handoff")
        .expect("companion row");
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
    // The remote-helper row rides a still-Stable public claim with stale export evidence.
    let remote = r
        .row("claim-scope-remote-helper-skew")
        .expect("remote helper row");
    assert!(remote.has_active_reason(ClaimScopeReason::EvidenceStale));
    assert!(r.computed_blocking_claim_ids().contains(&remote.entry_id));
}

#[test]
fn row_over_claiming_the_public_label_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.source_published_label.is_at_or_above_cutline())
        .expect("register has a row reusing a below-cutline public claim");
    row.published_label = StableClaimLevel::Stable;
    for a in &mut row.audiences {
        a.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_claim_ids();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::RowLabelExceedsSource { .. })),
        "a row may not publish greener than the public claim it reuses"
    );
}

#[test]
fn published_row_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a published row");
    row.active_scope_reasons
        .push(ClaimScopeReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::PublishedWithActiveGap { .. })),
        "a published row may not carry an active narrowing reason"
    );
}

#[test]
fn audience_copy_drift_fails() {
    let mut r = register();
    r.rows[0].audiences[0].rendered_claim_text =
        "Hand-edited shiproom copy that drifted from the public claim.".to_owned();
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::AudienceCopyDrift { .. })),
        "an audience's wording must reuse the one row, not hand-maintained copy"
    );
}

#[test]
fn audience_label_drift_fails() {
    let mut r = register();
    // A narrowed row whose audience keeps a greener label than the row must fail — a
    // narrowed row downgrades every audience.
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.publishes_stable())
        .expect("a narrowed row exists");
    row.audiences[0].rendered_label = StableClaimLevel::Stable;
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::AudienceLabelDrift { .. })),
        "a narrowed row must downgrade every audience"
    );
}

#[test]
fn shiproom_without_reopen_ref_fails() {
    let mut r = register();
    let row = &mut r.rows[0];
    let shiproom = row
        .audiences
        .iter_mut()
        .find(|a| a.audience == ClaimScopeAudience::Shiproom)
        .expect("a shiproom rendering exists");
    shiproom.reopens_authoritative_row = false;
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::ReopenRefNotDisclosed { .. })),
        "a shiproom audience must always expose the reopen refs"
    );
}

#[test]
fn missing_required_audience_fails() {
    let mut r = register();
    r.rows[0]
        .audiences
        .retain(|a| a.audience != ClaimScopeAudience::PartnerReview);
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::RequiredAudienceUncovered { .. })),
        "every row must drive support, shiproom, docs, and partner review"
    );
}

#[test]
fn losing_a_retest_reason_fails() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.has_active_reason(ClaimScopeReason::RetestPending))
        .expect("a retest-pending row exists");
    row.active_scope_reasons
        .retain(|reason| *reason != ClaimScopeReason::RetestPending);
    for a in &mut row.audiences {
        a.discloses_scope_reasons = !row.active_scope_reasons.is_empty();
    }
    r.summary = r.computed_summary();
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::RowStateWithoutReason { .. })),
        "dropping the retest reason must lose the row-level retest-needed truth and be rejected"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ClaimScopeViolation::PromotionDecisionInconsistent { .. })),
        "promotion must not proceed while an export-layer stop rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-claim-scope-export-packets");
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
        let candidate: ClaimScopeExportRegister =
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
