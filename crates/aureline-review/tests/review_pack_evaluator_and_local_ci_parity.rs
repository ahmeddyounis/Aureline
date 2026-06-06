//! Fixture-driven coverage for stable review-pack evaluator and replay packets.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_review_pack_stable_evaluation, ReviewPackStableEvaluationInput,
    ReviewPackStableEvaluationPacket, REVIEW_PACK_EVALUATOR_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/review/m4/review-pack-evaluator-and-local-ci-parity")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("stable review-pack evaluator fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_packet(name: &str) -> ReviewPackStableEvaluationPacket {
    let path = fixtures_dir().join(name);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    project_review_pack_stable_evaluation(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"))
}

fn load_input(name: &str) -> ReviewPackStableEvaluationInput {
    let path = fixtures_dir().join(name);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"))
}

#[test]
fn every_fixture_projects_and_replays() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "stable review-pack evaluator fixtures must exist"
    );
    for path in paths {
        let payload =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let packet = project_review_pack_stable_evaluation(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        assert_eq!(packet.schema_version, REVIEW_PACK_EVALUATOR_SCHEMA_VERSION);
        assert_eq!(
            packet.replay_packet.review_pack_digest_ref,
            packet.evaluation.review_pack_digest_ref
        );
        assert_eq!(
            packet.replay_packet.required_check_names,
            packet
                .evaluation
                .required_checks
                .iter()
                .map(|check| check.required_check_name.clone())
                .collect::<Vec<_>>()
        );
        assert!(
            packet.inspection.replay_packet_exportable,
            "fixture {path:?} must be exportable and replayable"
        );
        assert!(
            packet
                .evaluation
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "browser_companion"),
            "fixture {path:?} must wire browser companion truth"
        );
    }
}

#[test]
fn full_parity_fixture_allows_stable_claim() {
    let packet = load_packet("full_parity_replay_packet.json");
    assert_eq!(packet.evaluation.overall_verdict_class, "full_parity");
    assert!(packet.inspection.all_surfaces_bound_to_same_identity);
    assert!(!packet.inspection.any_surface_downgraded);
    assert!(packet.inspection.advisory_ownership_present);
    assert!(packet.inspection.enforced_ownership_present);
    assert!(packet.inspection.required_check_vocabulary_replayable);
    assert!(packet.inspection.stable_full_parity_claim_allowed);
}

#[test]
fn stale_ai_and_missing_provider_write_fixture_downgrades_without_hiding_intent() {
    let packet = load_packet("stale_ai_missing_provider_write_fallback.json");
    assert_eq!(
        packet.evaluation.overall_verdict_class,
        "degraded_requires_review"
    );
    assert!(packet.inspection.any_surface_downgraded);
    assert!(packet.inspection.stale_ai_findings_downgraded);
    assert!(packet.inspection.missing_provider_write_access_falls_back);
    assert!(!packet.inspection.stable_full_parity_claim_allowed);
    assert!(packet
        .replay_packet
        .divergence_labels
        .iter()
        .any(|label| label == "ai_review_outdated"));
    assert!(packet
        .replay_packet
        .divergence_labels
        .iter()
        .any(|label| label == "missing_provider_write_access"));
}

#[test]
fn digest_mismatch_and_unsupported_capability_fixture_is_explicitly_downgraded() {
    let packet = load_packet("digest_mismatch_unsupported_capability.json");
    assert_eq!(
        packet.evaluation.overall_verdict_class,
        "unsupported_capability_downgraded"
    );
    assert!(!packet.inspection.all_surfaces_bound_to_same_identity);
    assert!(packet.inspection.any_surface_downgraded);
    assert!(!packet.inspection.stable_full_parity_claim_allowed);
    assert!(packet
        .replay_packet
        .divergence_labels
        .iter()
        .any(|label| label == "review_pack_digest_mismatch"));
}

#[test]
fn rejects_false_full_parity_when_surface_is_stale() {
    let mut input = load_input("stale_ai_missing_provider_write_fallback.json");
    input.overall_verdict_class = "full_parity".to_string();
    let err = ReviewPackStableEvaluationPacket::from_input(input)
        .expect_err("stale surfaces must not retain full parity");
    assert!(err.message().contains("full_parity"));
}

#[test]
fn rejects_materially_changed_ai_finding_left_open() {
    let mut input = load_input("stale_ai_missing_provider_write_fallback.json");
    input.ai_findings[0].resolution_state = "open".to_string();
    let err = ReviewPackStableEvaluationPacket::from_input(input)
        .expect_err("changed AI finding must be outdated or rerun recommended");
    assert!(err.message().contains("AI finding"));
}

#[test]
fn rejects_missing_provider_write_access_without_fallback() {
    let mut input = load_input("stale_ai_missing_provider_write_fallback.json");
    input.publish_previews[0].local_copy_export_fallback_available = false;
    let err = ReviewPackStableEvaluationPacket::from_input(input)
        .expect_err("missing provider write access must keep fallback visible");
    assert!(err.message().contains("fallback"));
}
