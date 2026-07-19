use super::*;
use std::collections::BTreeSet;

fn proof() -> M5PipelineDependencyFindingComponentProof {
    current_m5_pipeline_dependency_finding_component_proof()
        .expect("canonical component proof loads and validates")
}

fn cloned() -> M5PipelineDependencyFindingComponentProof {
    proof()
}

#[test]
fn checked_in_component_proof_validates_clean() {
    let proof = proof();
    assert_eq!(
        proof.record_kind,
        M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_RECORD_KIND
    );
    assert_eq!(
        proof.schema_version,
        M5_PIPELINE_DEPENDENCY_FINDING_COMPONENT_PROOF_SCHEMA_VERSION
    );
    assert!(proof.validate().is_empty(), "{:?}", proof.validate());
}

#[test]
fn all_component_families_have_required_consumers() {
    let proof = proof();
    for family in ComponentFamily::ALL {
        let row = proof.family_row(family).expect("family row exists");
        assert_eq!(row.fixture_ref, family.fixture_ref());
        assert!(has_all(
            &row.consumer_surfaces,
            family.required_consumers().iter().copied()
        ));
        assert!(!row.controlled_label_coverage.is_empty());
        assert!(!row.degraded_state_coverage.is_empty());
    }
}

#[test]
fn consumer_projections_preserve_stable_component_identity_per_family() {
    let proof = proof();
    let projections = proof
        .consumer_projections()
        .expect("component fixtures produce projections");

    for family in ComponentFamily::ALL {
        let ids: BTreeSet<&str> = projections
            .iter()
            .filter(|projection| projection.family == family)
            .map(|projection| projection.component_id.as_str())
            .collect();
        assert_eq!(ids.len(), 1, "{family:?} drifted ids: {ids:?}");
    }
}

#[test]
fn companion_consumers_narrow_actions_explicitly() {
    let proof = proof();
    let projections = proof
        .consumer_projections()
        .expect("component fixtures produce projections");
    let companion: Vec<&ComponentConsumerProjection> = projections
        .iter()
        .filter(|projection| projection.consumer_surface == "companion_client")
        .collect();
    assert!(
        companion.len() >= 4,
        "expected pipeline/dependency/manifest/security companion projections"
    );

    let pipeline = companion
        .iter()
        .find(|projection| projection.family == ComponentFamily::PipelineRunRow)
        .expect("pipeline companion projection");
    assert_eq!(pipeline.action_or_authority, "provider_owned");
    assert_eq!(
        pipeline.limited_action_note,
        "provider_owned_and_stale_base_requires_reapproval"
    );

    let manifest = companion
        .iter()
        .find(|projection| projection.family == ComponentFamily::ManifestDiffCard)
        .expect("manifest companion projection");
    assert_eq!(manifest.action_or_authority, "inspect_only");
    assert!(manifest.limited_action_note.contains("narrows"));
}

#[test]
fn proof_requires_manifest_diff_companion_consumer() {
    let mut proof = cloned();
    let row = proof
        .component_families
        .iter_mut()
        .find(|row| row.family == ComponentFamily::ManifestDiffCard)
        .expect("manifest row exists");
    row.consumer_surfaces
        .retain(|surface| surface != "companion_client");
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::ConsumerSurfaceMissing(
                ComponentFamily::ManifestDiffCard
            )),
        "{:?}",
        proof.validate()
    );
}

#[test]
fn incomplete_promotion_gate_fails_validation() {
    let mut proof = cloned();
    proof
        .promotion_gate
        .first_consumers_can_reference_one_baseline = false;
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::PromotionGateIncomplete),
        "{:?}",
        proof.validate()
    );
}

#[test]
fn stale_proof_without_auto_narrowing_fails_validation() {
    let mut proof = cloned();
    proof.proof_freshness.proof_fresh = false;
    proof.proof_freshness.auto_narrow_on_stale = false;
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::ProofFreshnessInvalid),
        "{:?}",
        proof.validate()
    );
}

#[test]
fn missing_parity_check_fails_validation() {
    let mut proof = cloned();
    proof.parity_checks.pop();
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::ParityChecksIncomplete),
        "{:?}",
        proof.validate()
    );
}

#[test]
fn incomplete_consumer_certification_fails_validation() {
    let mut proof = cloned();
    proof.consumer_certifications[0].parity_check_refs.pop();
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::ConsumerCertificationsIncomplete),
        "{:?}",
        proof.validate()
    );
}

#[test]
fn changed_suppression_vocabulary_fails_validation() {
    let mut proof = cloned();
    proof.suppression_vocabulary.pop();
    assert!(
        proof
            .validate()
            .contains(&ComponentProofViolation::SuppressionVocabularyMismatch),
        "{:?}",
        proof.validate()
    );
}
