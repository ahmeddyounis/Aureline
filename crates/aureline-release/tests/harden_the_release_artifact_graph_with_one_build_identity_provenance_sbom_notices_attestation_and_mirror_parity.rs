//! Protected tests binding the typed release artifact graph artifact to the
//! checked-in JSON and the frozen CI validation capture.
//!
//! The positive case is the frozen, checked-in artifact; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict
//! and summary; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a row which fails to narrow, a current row with an
//! active gap reason, or a publication verdict that disagrees with the firing
//! rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity::{
    current_harden_release_artifact_graph, ArtifactFamilyGapReason, ArtifactFamilyState,
    HardenReleaseArtifactGraph, HardenReleaseArtifactGraphViolation,
    PublicationDecision,
    HARDEN_RELEASE_ARTIFACT_GRAPH_RECORD_KIND, HARDEN_RELEASE_ARTIFACT_GRAPH_SCHEMA_VERSION,
};

fn artifact() -> HardenReleaseArtifactGraph {
    current_harden_release_artifact_graph().expect("checked-in artifact parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_artifact_parses_and_validates() {
    let artifact = artifact();
    assert_eq!(
        artifact.schema_version,
        HARDEN_RELEASE_ARTIFACT_GRAPH_SCHEMA_VERSION
    );
    assert_eq!(
        artifact.record_kind,
        HARDEN_RELEASE_ARTIFACT_GRAPH_RECORD_KIND
    );
    let violations = artifact.validate();
    assert!(
        violations.is_empty(),
        "checked-in artifact must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_holding_and_narrowed_families() {
    let artifact = artifact();
    assert!(
        !artifact.rows_holding_claim().is_empty(),
        "artifact must hold at least one claim"
    );
    assert!(
        !artifact.rows_narrowed().is_empty(),
        "artifact must narrow at least one claim"
    );
}

#[test]
fn every_family_kind_is_present() {
    use aureline_release::harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity::ArtifactFamilyKind;

    let artifact = artifact();
    let present: std::collections::BTreeSet<ArtifactFamilyKind> =
        artifact.rows.iter().map(|r| r.family_kind).collect();
    for kind in ArtifactFamilyKind::ALL {
        assert!(
            present.contains(&kind),
            "missing family kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn publication_decision_matches_computed() {
    let artifact = artifact();
    assert_eq!(
        artifact.publication.decision,
        artifact.computed_publication_decision()
    );
    assert_eq!(
        artifact.publication.blocking_rule_ids,
        artifact.computed_blocking_rule_ids()
    );
    assert_eq!(
        artifact.publication.blocking_row_ids,
        artifact.computed_blocking_row_ids()
    );
}

#[test]
fn narrowed_row_that_does_not_narrow_fails() {
    let mut artifact = artifact();
    let row = artifact
        .rows
        .iter_mut()
        .find(|r| r.family_state == ArtifactFamilyState::NarrowedStale)
        .expect("artifact has a narrowed-stale row");
    row.effective_label = row.claim_label.clone();
    artifact.summary = artifact.computed_summary();
    artifact.publication.decision = artifact.computed_publication_decision();
    artifact.publication.blocking_rule_ids = artifact.computed_blocking_rule_ids();
    artifact.publication.blocking_row_ids = artifact.computed_blocking_row_ids();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenReleaseArtifactGraphViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a narrowing state must drop the effective label below the claim label"
    );
}

#[test]
fn current_row_with_active_gap_fails() {
    let mut artifact = artifact();
    let row = artifact
        .rows
        .iter_mut()
        .find(|r| r.family_state == ArtifactFamilyState::Current)
        .expect("artifact has a current row");
    row.active_gap_reasons
        .push(ArtifactFamilyGapReason::PacketFreshnessBreached);
    artifact.summary = artifact.computed_summary();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenReleaseArtifactGraphViolation::HeldRowWithActiveGap { .. }
        )),
        "a current row may not carry an active gap reason"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut artifact = artifact();
    artifact.publication.decision = PublicationDecision::Proceed;

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            HardenReleaseArtifactGraphViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m4/harden-the-release-artifact-graph-with-one-build-identity-provenance-sbom-notices-attestation-and-mirror-parity",
    );
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
        let candidate: HardenReleaseArtifactGraph =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
