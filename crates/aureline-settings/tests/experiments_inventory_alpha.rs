//! Fixture replay for the experiments inventory alpha.

use aureline_settings::experiments::{
    inspect_default_inventory, project_cli_inventory, project_support_export,
    ArtifactDependencyWarning, ExperimentsInventoryCliProjection,
    ExperimentsInventorySupportExportProjection,
};

#[test]
fn default_inventory_projects_acceptance_states() {
    let inspection = inspect_default_inventory().expect("inventory should inspect");

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
            inspection.lifecycle_counts.contains_key(state),
            "missing lifecycle state {state}"
        );
    }

    let managed = inspection
        .rows
        .iter()
        .find(|row| row.capability_id == "alpha.managed_cloud_daily_driver")
        .expect("managed disabled row");
    assert_eq!(managed.effective_lifecycle_state, "DisabledByPolicy");
    assert_eq!(
        managed
            .winning_disable_source
            .as_ref()
            .map(|source| source.source_class.as_str()),
        Some("admin_policy_ceiling")
    );
    assert!(
        managed
            .winning_disable_source
            .as_ref()
            .expect("source")
            .preserve_user_data
    );
}

#[test]
fn support_export_and_cli_projection_share_inventory_contract() {
    let inspection = inspect_default_inventory().expect("inventory should inspect");
    let cli = project_cli_inventory(&inspection);
    let export = project_support_export("support-export:experiments:test", &inspection);

    assert_eq!(
        cli.source_inventory_ref,
        "artifacts/governance/experiments_inventory_alpha.yaml"
    );
    assert_eq!(export.shared_contract_ref, inspection.shared_contract_ref);
    assert_eq!(
        export.artifact_dependency_warnings.len(),
        inspection.artifact_dependency_marker_count
    );
    assert!(export.artifact_dependency_warnings.iter().any(|warning| {
        warning.artifact_class == "workspace_manifest"
            && warning.required_lifecycle_state == "Preview"
    }));
}

#[test]
fn fixtures_round_trip_through_shared_types() {
    let warning: ArtifactDependencyWarning = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/experiments_inventory_alpha/saved_artifact_dependency_warning.json"
    )))
    .unwrap();
    assert_eq!(warning.required_lifecycle_state, "Retired");
    assert!(warning.warning.contains("refuse to reactivate"));

    let support: ExperimentsInventorySupportExportProjection =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/settings/experiments_inventory_alpha/support_export_shared_contract.json"
        )))
        .unwrap();
    assert_eq!(support.disabled_or_retired_count, 1);
    assert_eq!(
        support.rows[0]
            .winning_disable_source
            .as_ref()
            .map(|source| source.source_class.as_str()),
        Some("admin_policy_ceiling")
    );

    let cli: ExperimentsInventoryCliProjection = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/experiments_inventory_alpha/cli_projection_acceptance.json"
    )))
    .unwrap();
    assert_eq!(cli.fields.get("row_count").map(String::as_str), Some("7"));
    assert!(cli.rows.iter().any(|row| {
        row.capability_id == "settings.sync_conflict_review"
            && row.effective_lifecycle_state == "Beta"
    }));
}
