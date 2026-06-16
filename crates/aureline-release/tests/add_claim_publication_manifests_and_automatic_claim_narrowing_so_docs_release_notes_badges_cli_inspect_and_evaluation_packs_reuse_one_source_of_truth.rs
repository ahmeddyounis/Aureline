//! Protected tests binding the typed M5 claim-publication manifest register to
//! the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the
//! published/narrowed counts, and the destination/report-freshness counts; the
//! negative cases mutate a parsed copy and the checked-in fixtures to prove that a
//! claim that over-claims its row, a published claim with an active gap, a
//! destination whose copy drifted from the manifest, a destination that hides its
//! freshness, a manifest that drops a required destination, and a promotion
//! verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::{
    current_m5_claim_publication_manifests, M5ClaimDestination, M5ClaimNarrowingReason,
    M5ClaimManifestState, M5ClaimPublicationRegister, M5ClaimPublicationViolation,
    M5ClaimReportState, M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND,
    M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION,
};
use aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    current_m5_qualification_and_skew_matrix, FamilyKind,
};
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth_validation_capture.json"
));

fn register() -> M5ClaimPublicationRegister {
    current_m5_claim_publication_manifests().expect("checked-in register parses into the model")
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
        M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_kind_and_required_destination() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.manifests_for_kind(kind).is_empty(),
            "family kind {} must have at least one manifest",
            kind.as_str()
        );
    }
    for m in &r.manifests {
        let driven: std::collections::BTreeSet<M5ClaimDestination> =
            m.destinations.iter().map(|d| d.destination).collect();
        for destination in M5ClaimDestination::REQUIRED {
            assert!(
                driven.contains(&destination),
                "manifest {} must drive required destination {}",
                m.entry_id,
                destination.as_str()
            );
        }
    }
}

#[test]
fn every_manifest_joins_a_real_qualification_row() {
    let r = register();
    let matrix = current_m5_qualification_and_skew_matrix().expect("matrix parses");
    for m in &r.manifests {
        let row = matrix.row(&m.qualification_row_ref).unwrap_or_else(|| {
            panic!(
                "manifest {} joins qualification row {} which must exist in the matrix",
                m.entry_id, m.qualification_row_ref
            )
        });
        assert_eq!(
            row.family_ref, m.family_ref,
            "manifest {} family must match the joined row",
            m.entry_id
        );
        assert_eq!(
            row.published_label, m.row_published_label,
            "manifest {} must inherit the joined row's published label",
            m.entry_id
        );
        assert!(
            m.published_label.rank() <= m.row_published_label.rank(),
            "manifest {} claim must never exceed its row",
            m.entry_id
        );
    }
}

#[test]
fn exercises_narrowing_and_report_vocabulary() {
    let r = register();
    let states: std::collections::BTreeSet<M5ClaimManifestState> =
        r.manifests.iter().map(|m| m.manifest_state).collect();
    assert!(states.contains(&M5ClaimManifestState::Published));
    assert!(states.contains(&M5ClaimManifestState::NarrowedStale));
    assert!(states.contains(&M5ClaimManifestState::NarrowedRowDowngraded));
    assert!(states.contains(&M5ClaimManifestState::NarrowedMissing));

    let reasons: std::collections::BTreeSet<M5ClaimNarrowingReason> = r
        .manifests
        .iter()
        .flat_map(|m| m.active_narrowing_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&M5ClaimNarrowingReason::EvidenceStale));
    assert!(reasons.contains(&M5ClaimNarrowingReason::ReportStale));
    assert!(reasons.contains(&M5ClaimNarrowingReason::ReportMissing));

    let stale_report = r
        .manifests
        .iter()
        .flat_map(|m| m.backing_reports())
        .any(|rep| rep.state == M5ClaimReportState::Stale);
    let missing_report = r
        .manifests
        .iter()
        .flat_map(|m| m.backing_reports())
        .any(|rep| rep.state == M5ClaimReportState::Missing);
    assert!(
        stale_report,
        "a manifest must exercise a stale backing report"
    );
    assert!(
        missing_report,
        "a manifest must exercise a missing backing report"
    );

    // At least one claim must carry scope caveats that travel into every surface.
    assert!(r
        .manifests
        .iter()
        .any(|m| !m.published_claim.scope_caveats.is_empty()));
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
        summary["total_manifests"].as_u64().unwrap() as usize,
        r.manifests.len()
    );
    assert_eq!(
        summary["manifests_published"].as_u64().unwrap() as usize,
        r.manifests_published().len()
    );
    assert_eq!(
        summary["manifests_narrowed"].as_u64().unwrap() as usize,
        r.manifests_narrowed().len()
    );
    assert_eq!(
        summary["total_destinations"].as_u64().unwrap() as usize,
        computed.total_destinations
    );
    assert_eq!(
        summary["destinations_freshness_disclosed"]
            .as_u64()
            .unwrap() as usize,
        computed.destinations_freshness_disclosed
    );
    assert_eq!(
        summary["reports_stale"].as_u64().unwrap() as usize,
        computed.reports_stale
    );
    assert_eq!(
        summary["reports_missing"].as_u64().unwrap() as usize,
        computed.reports_missing
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
fn register_narrows_a_release_blocking_claim() {
    let r = register();
    let narrowed = r
        .manifests
        .iter()
        .find(|m| m.release_blocking && m.claim_holds_stable() && !m.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking claim under a still-stable claim"
    );
}

#[test]
fn manifest_layer_failure_holds_promotion_but_inherited_narrowing_does_not() {
    let r = register();
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
    // The companion claim only inherits an upstream row narrowing -> not a blocker.
    let companion = r
        .manifest("m5-claim-companion")
        .expect("companion manifest");
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
    // The toolchain claim has a manifest-layer evidence failure -> a blocker.
    let toolchain = r
        .manifest("m5-claim-toolchain")
        .expect("toolchain manifest");
    assert!(toolchain.has_active_reason(M5ClaimNarrowingReason::EvidenceStale));
    assert!(r
        .computed_blocking_claim_ids()
        .contains(&toolchain.entry_id));
}

#[test]
fn claim_over_claiming_the_row_fails() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| !m.row_published_label.is_at_or_above_cutline())
        .expect("register has a manifest inheriting a narrowed row");
    m.published_label = StableClaimLevel::Stable;
    for d in &mut m.destinations {
        d.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_claim_ids();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5ClaimPublicationViolation::ClaimPublishedWiderThanRow { .. }
        )),
        "a claim may not publish wider than the qualification row it binds"
    );
}

#[test]
fn published_claim_with_active_gap_fails() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.publishes_stable())
        .expect("register has a published manifest");
    m.active_narrowing_reasons
        .push(M5ClaimNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5ClaimPublicationViolation::HeldWithActiveGap { .. })),
        "a published claim may not carry an active narrowing reason"
    );
}

#[test]
fn destination_copy_drift_fails() {
    let mut r = register();
    r.manifests[0].destinations[0].rendered_claim_text =
        "Hand-edited marketing copy that drifted from the manifest.".to_owned();
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5ClaimPublicationViolation::DestinationCopyDrift { .. })),
        "a destination's wording must reuse the one manifest, not hand-maintained copy"
    );
}

#[test]
fn destination_label_drift_fails() {
    let mut r = register();
    // A published manifest whose docs surface keeps a greener label than the
    // narrowed manifest must fail — stale/narrowed evidence downgrades all surfaces.
    let m = r
        .manifests
        .iter_mut()
        .find(|m| !m.publishes_stable())
        .expect("a narrowed manifest exists");
    m.destinations[0].rendered_label = StableClaimLevel::Stable;
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, M5ClaimPublicationViolation::DestinationLabelDrift { .. })),
        "a narrowed manifest must downgrade every consuming surface"
    );
}

#[test]
fn missing_required_destination_fails() {
    let mut r = register();
    r.manifests[0]
        .destinations
        .retain(|d| d.destination != M5ClaimDestination::EvaluationPack);
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5ClaimPublicationViolation::RequiredDestinationUncovered { .. }
        )),
        "every claim must drive docs, release notes, badge, CLI inspect, eval pack, and admin export"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            M5ClaimPublicationViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a manifest-layer stop rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-claim-publication-manifests");
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
        let candidate: M5ClaimPublicationRegister =
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
