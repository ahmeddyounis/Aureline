//! Protected tests binding the typed M5 private evaluation/pilot evidence-pack
//! register to the checked-in artifact, the frozen CI validation capture, the
//! public claim-publication manifest it reuses, and the negative fixtures.
//!
//! The positive case is the checked-in register; the join check proves every pack
//! reuses a real public claim-publication manifest entry and is never greener than
//! it; the capture cross-check proves the typed model and the CI gate agree on the
//! promotion verdict, the published/narrowed counts, and the mirror/known-issue
//! counts; the negative cases mutate a parsed copy and the checked-in fixtures to
//! prove that a pack that over-claims its public label, a published pack with an
//! active gap, a destination whose copy drifted from the pack, a destination that
//! hides its freshness, a pack that drops a required destination, and a promotion
//! verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::current_m5_claim_publication_manifests;
use aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::FamilyKind;
use aureline_release::ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes::{
    current_m5_evaluation_pilot_packs, EvalPackDestination, EvalPackLaneKind,
    EvalPackNarrowingReason, EvalPackRegister, EvalPackState, EvalPackViolation,
    M5_EVALUATION_PILOT_PACKS_RECORD_KIND, M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes_validation_capture.json"
));

fn register() -> EvalPackRegister {
    current_m5_evaluation_pilot_packs().expect("checked-in register parses into the model")
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
    assert_eq!(r.schema_version, M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_EVALUATION_PILOT_PACKS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_lane_family_and_required_destination() {
    let r = register();
    for lane in EvalPackLaneKind::ALL {
        assert!(
            !r.packs_for_lane(lane).is_empty(),
            "lane kind {} must have at least one pack",
            lane.as_str()
        );
    }
    for kind in FamilyKind::ALL {
        assert!(
            !r.packs_for_kind(kind).is_empty(),
            "family kind {} must have at least one pack",
            kind.as_str()
        );
    }
    for p in &r.packs {
        let driven: std::collections::BTreeSet<EvalPackDestination> =
            p.destinations.iter().map(|d| d.destination).collect();
        for destination in EvalPackDestination::REQUIRED {
            assert!(
                driven.contains(&destination),
                "pack {} must drive required destination {}",
                p.entry_id,
                destination.as_str()
            );
        }
    }
}

#[test]
fn every_pack_reuses_a_real_public_claim_and_is_never_greener() {
    let r = register();
    let claims = current_m5_claim_publication_manifests().expect("claim manifest parses");
    for p in &r.packs {
        let claim = claims
            .manifest(&p.claim_manifest_entry_ref)
            .unwrap_or_else(|| {
                panic!(
                    "pack {} reuses claim {} which must exist in the claim manifest",
                    p.entry_id, p.claim_manifest_entry_ref
                )
            });
        assert_eq!(
            claim.family_kind, p.family_kind,
            "pack {} family must match the reused public claim",
            p.entry_id
        );
        assert_eq!(
            claim.family_ref, p.family_ref,
            "pack {} family ref must match the reused public claim",
            p.entry_id
        );
        assert_eq!(
            claim.published_label, p.public_claim_label,
            "pack {} must mirror the public claim's published label",
            p.entry_id
        );
        assert_eq!(
            claim.published_claim.support_class, p.public_support_class,
            "pack {} must mirror the public claim's support class",
            p.entry_id
        );
        assert_eq!(
            claim.published_claim.claim_text, p.public_claim_text,
            "pack {} must mirror the public claim text verbatim",
            p.entry_id
        );
        assert!(
            p.pack_published_label.rank() <= p.public_claim_label.rank(),
            "pack {} may never publish greener than its public claim",
            p.entry_id
        );
        assert!(
            !p.over_claims_public(),
            "pack {} may never over-claim the public label or support class",
            p.entry_id
        );
    }
}

#[test]
fn exercises_narrowing_and_mirror_vocabulary() {
    let r = register();
    let states: std::collections::BTreeSet<EvalPackState> =
        r.packs.iter().map(|p| p.pack_state).collect();
    assert!(states.contains(&EvalPackState::Published));
    assert!(states.contains(&EvalPackState::NarrowedPublicClaim));
    assert!(states.contains(&EvalPackState::NarrowedStale));
    assert!(states.contains(&EvalPackState::NarrowedMissing));

    let reasons: std::collections::BTreeSet<EvalPackNarrowingReason> = r
        .packs
        .iter()
        .flat_map(|p| p.active_narrowing_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&EvalPackNarrowingReason::PublicClaimNarrowed));
    assert!(reasons.contains(&EvalPackNarrowingReason::MirrorStale));
    assert!(reasons.contains(&EvalPackNarrowingReason::MirrorMissing));

    // At least one pack must carry a bundle, a known-issues delta, and a support
    // contact that travel into every partner surface.
    assert!(r.packs.iter().all(|p| !p.bundle_id.trim().is_empty()));
    assert!(r.packs.iter().all(|p| !p.mirror_refs.is_empty()));
    assert!(r.packs.iter().any(|p| !p.known_issues_delta.is_empty()));
    assert!(r.packs.iter().all(|p| !p.support_contacts.is_empty()));
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
        summary["total_packs"].as_u64().unwrap() as usize,
        r.packs.len()
    );
    assert_eq!(
        summary["packs_published"].as_u64().unwrap() as usize,
        r.packs_published().len()
    );
    assert_eq!(
        summary["packs_narrowed"].as_u64().unwrap() as usize,
        r.packs_narrowed().len()
    );
    assert_eq!(
        summary["total_mirror_refs"].as_u64().unwrap() as usize,
        computed.total_mirror_refs
    );
    assert_eq!(
        summary["mirrors_stale"].as_u64().unwrap() as usize,
        computed.mirrors_stale
    );
    assert_eq!(
        summary["mirrors_missing"].as_u64().unwrap() as usize,
        computed.mirrors_missing
    );
    assert_eq!(
        summary["total_known_issues"].as_u64().unwrap() as usize,
        computed.total_known_issues
    );
    assert_eq!(
        summary["destinations_known_issues_disclosed"]
            .as_u64()
            .unwrap() as usize,
        computed.destinations_known_issues_disclosed
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
fn register_narrows_a_pack_under_a_still_stable_public_claim() {
    let r = register();
    let narrowed = r
        .packs
        .iter()
        .find(|p| p.release_blocking && p.public_claim_holds_stable() && !p.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one pack under a still-stable public claim"
    );
}

#[test]
fn pack_layer_failure_holds_promotion_but_inherited_narrowing_does_not() {
    let r = register();
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
    // The companion partner pack only inherits an upstream public-claim narrowing.
    let companion = r
        .pack("eval-pack-companion-ecosystem-partner")
        .expect("companion pack");
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
    // The notebook pilot pack rides a still-Stable public claim with a stale mirror.
    let pilot = r
        .pack("eval-pack-notebook-enterprise-pilot")
        .expect("notebook pilot pack");
    assert!(pilot.has_active_reason(EvalPackNarrowingReason::MirrorStale));
    assert!(r.computed_blocking_claim_ids().contains(&pilot.entry_id));
}

#[test]
fn pack_over_claiming_the_public_label_fails() {
    let mut r = register();
    let p = r
        .packs
        .iter_mut()
        .find(|p| !p.public_claim_label.is_at_or_above_cutline())
        .expect("register has a pack reusing a below-cutline public claim");
    p.pack_published_label = StableClaimLevel::Stable;
    for d in &mut p.destinations {
        d.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_claim_ids();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::PackLabelExceedsPublicClaim { .. })),
        "a pack may not publish greener than the public claim it reuses"
    );
}

#[test]
fn published_pack_with_active_gap_fails() {
    let mut r = register();
    let p = r
        .packs
        .iter_mut()
        .find(|p| p.publishes_stable())
        .expect("register has a published pack");
    p.active_narrowing_reasons
        .push(EvalPackNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::PublishedWithActiveGap { .. })),
        "a published pack may not carry an active narrowing reason"
    );
}

#[test]
fn destination_copy_drift_fails() {
    let mut r = register();
    r.packs[0].destinations[0].rendered_claim_text =
        "Hand-edited pilot-only marketing copy that drifted from the public claim.".to_owned();
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::DestinationCopyDrift { .. })),
        "a destination's wording must reuse the one pack, not hand-maintained copy"
    );
}

#[test]
fn destination_label_drift_fails() {
    let mut r = register();
    // A narrowed pack whose partner surface keeps a greener label than the pack
    // must fail — a narrowed pack downgrades every partner surface.
    let p = r
        .packs
        .iter_mut()
        .find(|p| !p.publishes_stable())
        .expect("a narrowed pack exists");
    p.destinations[0].rendered_label = StableClaimLevel::Stable;
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::DestinationLabelDrift { .. })),
        "a narrowed pack must downgrade every partner surface"
    );
}

#[test]
fn missing_required_destination_fails() {
    let mut r = register();
    r.packs[0]
        .destinations
        .retain(|d| d.destination != EvalPackDestination::PilotPacket);
    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::RequiredDestinationUncovered { .. })),
        "every pack must drive the evaluation pack, pilot packet, admin export, and support export"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, EvalPackViolation::PromotionDecisionInconsistent { .. })),
        "promotion must not proceed while a pack-layer stop rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-evaluation-pilot-packs");
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
        let candidate: EvalPackRegister =
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
