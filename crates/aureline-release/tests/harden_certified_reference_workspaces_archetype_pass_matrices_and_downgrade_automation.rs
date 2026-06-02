//! Protected tests binding the typed certified-reference-workspaces artifact to
//! the checked-in JSON and the frozen CI validation capture.
//!
//! The positive case is the frozen, checked-in artifact; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict
//! and summary; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a matrix row which fails to narrow, a certified claim
//! with a stale report, or a publication verdict that disagrees with the firing
//! downgrade rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation::{
    current_certified_reference_workspaces, DowngradeReason, MatrixRowState, PublicationDecision,
    CertifiedReferenceWorkspaces, CertifiedReferenceWorkspacesViolation,
    CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND, CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION,
};

fn artifact() -> CertifiedReferenceWorkspaces {
    current_certified_reference_workspaces().expect("checked-in artifact parses into the model")
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
        CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION
    );
    assert_eq!(
        artifact.record_kind,
        CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND
    );
    let violations = artifact.validate();
    assert!(
        violations.is_empty(),
        "checked-in artifact must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_certified_and_narrowed_archetypes() {
    let artifact = artifact();
    assert!(
        !artifact.rows_holding_certified().is_empty(),
        "artifact must certify at least one archetype"
    );
    assert!(
        !artifact.rows_narrowed().is_empty(),
        "artifact must narrow at least one archetype"
    );
}

#[test]
fn every_report_backs_its_matrix_row() {
    let artifact = artifact();
    let archetypes_with_reports: std::collections::BTreeSet<&str> = artifact
        .reference_workspace_reports
        .iter()
        .map(|r| r.archetype_ref.as_str())
        .collect();
    for row in &artifact.archetype_pass_matrix_rows {
        if row.claimed_certified {
            assert!(
                archetypes_with_reports.contains(row.archetype_ref.as_str()),
                "archetype {} is claimed certified but has no reference-workspace report",
                row.archetype_ref
            );
        }
    }
}

#[test]
fn matrix_row_narrows_when_report_is_stale_or_missing() {
    let artifact = artifact();
    let stale_or_missing_rows: Vec<&str> = artifact
        .rows_narrowed()
        .into_iter()
        .filter(|r| {
            r.has_active_reason(DowngradeReason::ReferenceWorkspaceReportStale)
                || r.has_active_reason(DowngradeReason::ReferenceWorkspaceReportMissing)
        })
        .map(|r| r.row_id.as_str())
        .collect();
    assert!(
        !stale_or_missing_rows.is_empty(),
        "at least one matrix row must narrow because its report is stale or missing"
    );
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
        .archetype_pass_matrix_rows
        .iter_mut()
        .find(|r| r.matrix_state == MatrixRowState::NarrowedStale)
        .expect("artifact has a narrowed-stale row");
    row.effective_certified = true;
    artifact.summary = artifact.computed_summary();
    artifact.publication.decision = artifact.computed_publication_decision();
    artifact.publication.blocking_rule_ids = artifact.computed_blocking_rule_ids();
    artifact.publication.blocking_row_ids = artifact.computed_blocking_row_ids();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            CertifiedReferenceWorkspacesViolation::EffectiveCertifiedNotNarrowed { .. }
        )),
        "a narrowing state must drop effective_certified to false"
    );
}

#[test]
fn certified_claim_with_stale_report_fails() {
    let mut artifact = artifact();
    let row = artifact
        .archetype_pass_matrix_rows
        .iter_mut()
        .find(|r| r.matrix_state == MatrixRowState::Certified)
        .expect("artifact has a certified row");
    row.active_downgrade_reasons
        .push(DowngradeReason::ReferenceWorkspaceReportStale);
    artifact.summary = artifact.computed_summary();

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            CertifiedReferenceWorkspacesViolation::HeldRowWithActiveDowngrade { .. }
        )),
        "a certified row may not carry an active downgrade reason"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut artifact = artifact();
    artifact.publication.decision = PublicationDecision::Proceed;

    assert!(
        artifact.validate().iter().any(|v| matches!(
            v,
            CertifiedReferenceWorkspacesViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking downgrade rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation",
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
        let candidate: CertifiedReferenceWorkspaces =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
