//! Fixture replay for the beta experiments / flags / Labs governance UI
//! projection.

use aureline_settings::{
    build_default_labs_governance_beta_page, project_labs_governance_beta_cli,
    project_labs_governance_beta_support_export, validate_labs_governance_beta_page,
    validate_labs_governance_beta_support_export, LabsGovernanceBetaCliProjection,
    LabsGovernanceBetaPage, LabsGovernanceBetaSupportExport, LabsGovernanceBetaValidationError,
    LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF,
};

const PAGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/experiments_labs_governance_beta/page.json"
));

const CLI_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/experiments_labs_governance_beta/cli_projection.json"
));

const SUPPORT_EXPORT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/experiments_labs_governance_beta/support_export.json"
));

const DRILL_HIDDEN_EXPERIMENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/experiments_labs_governance_beta/drill_hidden_experiment_on_stable_surface.json"
));

const DRILL_MISSING_OWNER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/experiments_labs_governance_beta/drill_missing_alignment_field.json"
));

#[test]
fn fixture_page_matches_seeded_projection() {
    let fixture: LabsGovernanceBetaPage =
        serde_json::from_str(PAGE_FIXTURE).expect("page fixture parses");
    let seeded = build_default_labs_governance_beta_page().expect("seeded page builds");
    assert_eq!(fixture, seeded, "checked-in page fixture drifted from the seed");
    validate_labs_governance_beta_page(&fixture).expect("checked-in page validates");
}

#[test]
fn fixture_cli_and_support_export_share_lifecycle_tokens() {
    let page: LabsGovernanceBetaPage =
        serde_json::from_str(PAGE_FIXTURE).expect("page fixture parses");
    let cli_fixture: LabsGovernanceBetaCliProjection =
        serde_json::from_str(CLI_FIXTURE).expect("cli fixture parses");
    let export_fixture: LabsGovernanceBetaSupportExport =
        serde_json::from_str(SUPPORT_EXPORT_FIXTURE).expect("export fixture parses");

    assert_eq!(cli_fixture.shared_contract_ref, LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF);
    assert_eq!(
        export_fixture.shared_contract_ref,
        LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF
    );

    let projected_cli = project_labs_governance_beta_cli(&page);
    assert_eq!(projected_cli, cli_fixture);

    let projected_export = project_labs_governance_beta_support_export(
        export_fixture.export_id.clone(),
        export_fixture.generated_at.clone(),
        page,
    );
    assert_eq!(projected_export, export_fixture);

    validate_labs_governance_beta_support_export(&projected_export)
        .expect("support-export round-trip validates");
}

#[test]
fn drill_hidden_experiment_on_stable_surface_is_flagged() {
    let page: LabsGovernanceBetaPage =
        serde_json::from_str(DRILL_HIDDEN_EXPERIMENT).expect("drill parses");
    let errors = validate_labs_governance_beta_page(&page).expect_err("drill must flag errors");
    assert!(errors.iter().any(|e| matches!(
        e,
        LabsGovernanceBetaValidationError::VisibleMarkerMissing { capability_id }
            if capability_id == "shell.labs.wedge_inspector_overlay"
    )));
    assert!(errors.iter().any(|e| matches!(
        e,
        LabsGovernanceBetaValidationError::StableHostRendersHiddenExperiment {
            capability_id,
            host_surface,
        } if capability_id == "shell.labs.wedge_inspector_overlay"
            && host_surface == "settings_root"
    )));
}

#[test]
fn drill_missing_alignment_field_is_flagged() {
    let page: LabsGovernanceBetaPage =
        serde_json::from_str(DRILL_MISSING_OWNER).expect("drill parses");
    let errors = validate_labs_governance_beta_page(&page).expect_err("drill must flag errors");
    assert!(errors.iter().any(|e| matches!(
        e,
        LabsGovernanceBetaValidationError::AlignmentFieldMissing { capability_id, field }
            if capability_id == "settings.sync_conflict_review" && field == "owner"
    )));
}

#[test]
fn fixture_page_covers_required_lifecycle_states() {
    let page: LabsGovernanceBetaPage =
        serde_json::from_str(PAGE_FIXTURE).expect("page fixture parses");
    for state in [
        "Labs",
        "Preview",
        "Beta",
        "Stable",
        "Deprecated",
        "DisabledByPolicy",
        "Retired",
    ] {
        assert!(
            page.lifecycle_counts.contains_key(state),
            "fixture page missing lifecycle state {state}"
        );
    }
}
