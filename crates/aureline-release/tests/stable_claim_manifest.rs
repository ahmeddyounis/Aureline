//! Protected tests binding the typed stable claim manifest to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the frozen, checked-in manifest; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict,
//! summary, and packet-freshness counts; the negative cases mutate a parsed copy
//! and the checked-in fixtures to prove that an entry which fails to narrow, a
//! published label riding a breached packet, and a publication verdict that
//! disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::{
    current_stable_claim_manifest, FreshnessSloState, ManifestState, NarrowingReason,
    StableClaimManifest, StableClaimManifestViolation, STABLE_CLAIM_MANIFEST_RECORD_KIND,
    STABLE_CLAIM_MANIFEST_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::PromotionDecision;

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_claim_manifest_validation_capture.json"
));

fn manifest() -> StableClaimManifest {
    current_stable_claim_manifest().expect("checked-in stable claim manifest parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_manifest_parses_and_validates() {
    let manifest = manifest();
    assert_eq!(
        manifest.schema_version,
        STABLE_CLAIM_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(manifest.record_kind, STABLE_CLAIM_MANIFEST_RECORD_KIND);
    let violations = manifest.validate();
    assert!(
        violations.is_empty(),
        "checked-in manifest must validate cleanly: {violations:#?}"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let manifest = manifest();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(manifest.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        manifest.entries.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_published_stable"].as_u64().unwrap() as usize,
        manifest.entries_published_stable().len(),
        "capture published count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        manifest.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        manifest.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        manifest.publication.decision,
        manifest.computed_publication_decision()
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
fn freshness_automation_narrows_a_breached_packet() {
    let manifest = manifest();
    let narrowed_on_breach = manifest.entries.iter().find(|entry| {
        entry.proof_packet.slo_state == FreshnessSloState::Breached
            && !entry.publishes_stable()
            && entry.has_active_reason(NarrowingReason::ProofPacketFreshnessBreached)
    });
    assert!(
        narrowed_on_breach.is_some(),
        "the packet-freshness automation must narrow at least one breached-packet label"
    );
}

#[test]
fn narrowing_entry_that_does_not_narrow_fails() {
    let mut manifest = manifest();
    let entry = manifest
        .entries
        .iter_mut()
        .find(|entry| entry.manifest_state == ManifestState::NarrowedUnqualified)
        .expect("manifest has a narrowed-unqualified entry");
    entry.published_label = entry.claimed_label;
    manifest.summary = manifest.computed_summary();
    manifest.publication.decision = manifest.computed_publication_decision();
    manifest.publication.blocking_rule_ids = manifest.computed_blocking_rule_ids();
    manifest.publication.blocking_entry_ids = manifest.computed_blocking_entry_ids();

    assert!(
        manifest.validate().iter().any(|v| matches!(
            v,
            StableClaimManifestViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a subject lacking qualification must narrow below the cutline"
    );
}

#[test]
fn published_label_on_a_breached_packet_fails() {
    let mut manifest = manifest();
    let entry = manifest
        .entries
        .iter_mut()
        .find(|entry| entry.holds_label())
        .expect("manifest has a held entry");
    entry.proof_packet.slo_state = FreshnessSloState::Breached;
    manifest.summary = manifest.computed_summary();

    assert!(
        manifest
            .validate()
            .iter()
            .any(|v| matches!(v, StableClaimManifestViolation::HeldOnStalePacket { .. })),
        "a published label may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut manifest = manifest();
    manifest.publication.decision = PromotionDecision::Proceed;

    assert!(
        manifest.validate().iter().any(|v| matches!(
            v,
            StableClaimManifestViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn stripping_a_rule_for_an_active_reason_fails() {
    let mut manifest = manifest();
    manifest
        .publication_rules
        .retain(|rule| rule.trigger_reason != NarrowingReason::ProofPacketFreshnessBreached);
    manifest.summary = manifest.computed_summary();
    manifest.publication.decision = manifest.computed_publication_decision();
    manifest.publication.blocking_rule_ids = manifest.computed_blocking_rule_ids();
    manifest.publication.blocking_entry_ids = manifest.computed_blocking_entry_ids();

    assert!(
        manifest
            .validate()
            .contains(&StableClaimManifestViolation::NarrowingReasonWithoutRule {
                reason: NarrowingReason::ProofPacketFreshnessBreached,
            }),
        "every narrowing reason must keep a rule watching for it"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stable_claim_manifest");
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
        let candidate: StableClaimManifest =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
