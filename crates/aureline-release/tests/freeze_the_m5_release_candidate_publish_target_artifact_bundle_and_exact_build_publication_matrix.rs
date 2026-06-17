//! Protected tests binding the typed M5 publication matrix to the checked-in
//! artifact and the negative fixtures.
//!
//! The positive case is the checked-in matrix; the negative cases load the
//! checked-in fixtures and prove that a duplicate row id and a backed row that
//! lost its owner sign-off both fail validation, so a stale or broken row is
//! never silently published.

use std::path::{Path, PathBuf};

use aureline_release::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::{
    current_m5_publication_matrix, M5ArtifactFamilyKind, M5PublicationMatrix,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::PromotionDecision;

fn matrix() -> M5PublicationMatrix {
    current_m5_publication_matrix().expect("checked-in matrix parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(
        m.schema_version,
        FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(
        m.record_kind,
        FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND
    );
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_kind() {
    let m = matrix();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !m.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_artifact() {
    let m = matrix();
    assert!(!m.release_blocking_artifact_refs.is_empty());
    let covered: Vec<&str> = m
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.artifact_ref.as_str())
        .collect();
    for declared in &m.release_blocking_artifact_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn matrix_narrows_at_least_one_family() {
    let m = matrix();
    assert!(
        !m.rows_narrowed().is_empty(),
        "the matrix must narrow at least one artifact family below the cutline"
    );
}

#[test]
fn release_blocking_families_publish_with_intact_exact_build() {
    let m = matrix();
    for row in m.release_blocking_rows() {
        assert!(
            row.publishes_stable(),
            "release-blocking family {} must publish at or above the cutline",
            row.entry_id
        );
        assert!(
            row.exact_build.linkage_intact(),
            "release-blocking family {} must have intact exact-build linkage",
            row.entry_id
        );
        assert!(
            row.rollback_revocation.revocable,
            "release-blocking family {} must be revocable",
            row.entry_id
        );
    }
}

#[test]
fn export_projection_is_publication_decision_consistent() {
    let m = matrix();
    let projection = m.support_export_projection();
    assert_eq!(projection.rows.len(), m.rows.len());
    assert_eq!(
        projection.publication_decision,
        m.computed_publication_decision()
    );
    assert_eq!(m.publication.decision, PromotionDecision::Proceed);
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix",
    );
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
        let candidate: M5PublicationMatrix =
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
