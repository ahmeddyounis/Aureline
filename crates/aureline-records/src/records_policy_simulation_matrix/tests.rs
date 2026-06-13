use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::*;

fn matrix() -> RecordsPolicySimulationMatrix {
    current_records_policy_matrix().expect("checked-in matrix parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let matrix = matrix();
    assert_eq!(
        matrix.schema_version,
        RECORDS_POLICY_SIMULATION_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(
        matrix.record_kind,
        RECORDS_POLICY_SIMULATION_MATRIX_RECORD_KIND
    );
    assert!(
        matrix.validate().is_empty(),
        "checked-in matrix must validate cleanly: {:?}",
        matrix.validate()
    );
}

#[test]
fn matrix_covers_every_governed_family_and_consumer_surface() {
    let matrix = matrix();
    for family in GovernedArtifactFamily::ALL {
        assert!(
            matrix.row_for_family(family).is_some(),
            "family {} must be covered",
            family.as_str()
        );
    }
    let bound_surfaces: BTreeSet<ConsumerSurfaceClass> = matrix
        .consumer_bindings
        .iter()
        .map(|binding| binding.surface_class)
        .collect();
    for surface in ConsumerSurfaceClass::ALL {
        assert!(
            bound_surfaces.contains(&surface),
            "surface {:?} must be bound",
            surface
        );
    }
}

#[test]
fn cli_headless_help_and_release_projections_cover_every_row() {
    let matrix = matrix();
    assert_eq!(matrix.cli_headless_projection().len(), matrix.rows.len());
    assert_eq!(matrix.help_docs_projection().len(), matrix.rows.len());
    assert_eq!(
        matrix.release_evidence_projection().len(),
        matrix.rows.len()
    );
}

#[test]
fn local_only_row_cannot_claim_managed_delete() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.authority_boundary == AuthorityBoundaryClass::LocalOnly)
        .expect("seeded matrix contains a local-only row");
    row.claims_managed_delete = true;
    assert!(matrix.validate().iter().any(|violation| matches!(
        violation,
        RecordsPolicyMatrixViolation::ManagedControlOverclaimed { control, .. }
            if control == "managed_delete"
    )));
}

#[test]
fn remembered_decision_row_requires_all_reapproval_triggers() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| {
            row.policy_contract
                .remembered_decision_revalidation_required
        })
        .expect("seeded matrix contains a remembered-decision row");
    row.policy_contract.required_reapproval_trigger_tokens.pop();
    assert!(matrix.validate().iter().any(|violation| matches!(
        violation,
        RecordsPolicyMatrixViolation::ReapprovalTriggerMissing { .. }
    )));
}

#[test]
fn checked_in_negative_fixtures_are_rejected() {
    let fixtures_dir = repo_root().join("fixtures/governance/records_policy_simulation_matrix");
    let manifest = std::fs::read_to_string(fixtures_dir.join("cases.yaml"))
        .expect("fixture manifest is readable");
    let cases: serde_yaml::Value =
        serde_yaml::from_str(&manifest).expect("fixture manifest parses");
    let cases = cases["cases"].as_sequence().expect("cases is a sequence");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case file is present");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: RecordsPolicySimulationMatrix =
            serde_yaml::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected"
        );
    }
}
