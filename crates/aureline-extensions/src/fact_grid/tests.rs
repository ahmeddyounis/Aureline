//! Unit coverage for marketplace fact-grid projections.

use aureline_install::InstallTopologyAlphaPacket;
use serde::Deserialize;

use super::*;
use crate::compatibility_matrix::{current_extension_bridge_matrix, ExtensionBridgeMatrix};
use crate::install_review::{
    evaluate_install_review_alpha, ActivationBudgetDisclosure, CompatibilityLabelBlock,
    InstallReviewAlphaEvaluation, InstallReviewAlphaInput, InstallReviewBoundaryTruth,
};
use crate::manifest_baseline::EffectivePermissionBaselineRecord;
use crate::registry::{
    evaluate_catalog_descriptor, CatalogDescriptorInput, CatalogDescriptorRecord,
};
use crate::review_alpha::ExtensionReviewAlphaPacketRecord;

#[derive(Debug, Deserialize)]
struct CatalogFixture {
    input: CatalogDescriptorInput,
}

#[derive(Debug, Deserialize)]
struct InstallReviewFixture {
    input: InstallReviewAlphaInput,
    extension_review: ExtensionReviewAlphaPacketRecord,
    effective_permission: EffectivePermissionBaselineRecord,
    boundary_truth: InstallReviewBoundaryTruth,
    compatibility: CompatibilityLabelBlock,
    activation_budget: ActivationBudgetDisclosure,
    install_topology_row_id: String,
}

fn compatibility_report() -> crate::marketplace_truth::CompatibilityReportSnapshot {
    serde_json::from_str(include_str!(
        "../../../../artifacts/compat/m3/compatibility_report.json"
    ))
    .expect("compatibility report fixture must deserialize")
}

fn bridge_matrix() -> ExtensionBridgeMatrix {
    current_extension_bridge_matrix().expect("bridge matrix fixture must deserialize")
}

fn catalog_record() -> CatalogDescriptorRecord {
    serde_json::from_str(include_str!(
        "../../../../artifacts/extensions/m3/registry_moderation/catalog_descriptor_record.json"
    ))
    .expect("catalog descriptor artifact must deserialize")
}

fn evaluated_catalog_fixture(name: &str) -> CatalogDescriptorRecord {
    let raw = match name {
        "staged_pending_moderation" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/staged_pending_moderation.json"
        ),
        "limited_compatibility_catalog" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/limited_compatibility_catalog.json"
        ),
        "revoked_catalog_refused" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/revoked_catalog_refused.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    let fixture: CatalogFixture = serde_json::from_str(raw).expect("fixture must deserialize");
    evaluate_catalog_descriptor(fixture.input)
}

fn install_review_packet(review_id: &str) -> InstallReviewAlphaPacketRecord {
    let raw = include_str!(
        "../../../../fixtures/extensions/install_review_alpha/native_marketplace_package_lane.json"
    );
    let mut fixture: InstallReviewFixture =
        serde_json::from_str(raw).expect("install-review fixture must deserialize");
    fixture.input.review_id = review_id.to_string();

    let topology: InstallTopologyAlphaPacket = serde_json::from_str(include_str!(
        "../../../../fixtures/install/topology_alpha/install_topology_alpha_packet.json"
    ))
    .expect("install topology fixture must deserialize");
    let row = topology
        .row_by_id(&fixture.install_topology_row_id)
        .expect("fixture must cite an install-topology row");

    evaluate_install_review_alpha(InstallReviewAlphaEvaluation {
        input: fixture.input,
        extension_review: &fixture.extension_review,
        effective_permission: &fixture.effective_permission,
        boundary_truth: fixture.boundary_truth,
        compatibility: fixture.compatibility,
        activation_budget: fixture.activation_budget,
        install_topology_row: row,
        decided_at: "2026-05-16T19:05:00Z",
    })
}

fn marketplace_row(
    row_suffix: &str,
    catalog: &CatalogDescriptorRecord,
    bridge_row_ref: &str,
    install_review_ref: &str,
) -> MarketplaceTruthRowRecord {
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    crate::marketplace_truth::project_marketplace_truth_row(
        crate::marketplace_truth::MarketplaceTruthRowInput {
            row_id: &format!("marketplace_truth_row:dev.aureline.samples/wasm-notes:{row_suffix}"),
            catalog,
            compatibility_report: &report,
            compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
            extension_bridge_matrix: &bridge_matrix,
            extension_bridge_matrix_row_ref: bridge_row_ref,
            install_review_ref,
            generated_at: "2026-05-16T19:10:00Z",
        },
    )
    .expect("marketplace row must project")
}

fn manifest_changes(catalog: &CatalogDescriptorRecord) -> Vec<ManifestChangeRow> {
    vec![
        ManifestChangeRow {
            change_id: format!("manifest_change:{}:manifest", catalog.package_id),
            change_class: ManifestChangeClass::Added,
            manifest_ref: catalog.registry_manifest_ref.clone(),
            field_path: "extension_manifest".to_string(),
            before_summary: None,
            after_summary: Some(format!(
                "{}@{}",
                catalog.extension_identity, catalog.extension_version
            )),
            review_required: true,
            summary: "Install review will add the extension manifest entry.".to_string(),
        },
        ManifestChangeRow {
            change_id: format!("manifest_change:{}:permission_manifest", catalog.package_id),
            change_class: ManifestChangeClass::PermissionDelta,
            manifest_ref: catalog.permission_manifest_ref.clone(),
            field_path: "permissions".to_string(),
            before_summary: Some("not installed".to_string()),
            after_summary: Some("declared permissions and effective policy delta".to_string()),
            review_required: true,
            summary: "Permission manifest delta is reviewed before commit.".to_string(),
        },
    ]
}

fn script_risk(class: ScriptRiskClass) -> ScriptRiskDisclosure {
    ScriptRiskDisclosure {
        script_risk_class: class,
        risk_source_refs: vec!["registry_manifest:script-risk-reviewed".to_string()],
        native_build_requirement_refs: Vec::new(),
        policy_block_refs: Vec::new(),
        summary: format!("Script/native-build risk is {class:?}."),
    }
}

fn lockfile_impact(class: LockfileImpactClass) -> LockfileImpact {
    LockfileImpact {
        impact_class: class,
        resolver_ref: "resolver:aureline-extension-lock:v1".to_string(),
        affected_lockfile_refs: vec!["aureline.extensions.lock".to_string()],
        generated_file_refs: Vec::new(),
        environment_factor_refs: vec!["platform:any".to_string()],
        rollback_checkpoint_ref: Some("checkpoint:extension-lock:last-known-good".to_string()),
        summary: "Extension lockfile churn is visible before commit.".to_string(),
    }
}

fn fact_grid_for(
    row_suffix: &str,
    catalog: &CatalogDescriptorRecord,
    bridge_row_ref: &str,
    client_scope_class: ClientScopeClass,
    script_risk_class: ScriptRiskClass,
) -> MarketplaceFactGridRecord {
    let install_review_ref =
        format!("install_review_alpha:dev.aureline.samples/wasm-notes:{row_suffix}");
    let row = marketplace_row(row_suffix, catalog, bridge_row_ref, &install_review_ref);
    let review = install_review_packet(&install_review_ref);

    project_marketplace_fact_grid(MarketplaceFactGridInput {
        fact_grid_id: &format!(
            "marketplace_fact_grid:dev.aureline.samples/wasm-notes:{row_suffix}"
        ),
        surface_class: MarketplaceFactGridSurfaceClass::ResultRow,
        marketplace_row: &row,
        catalog,
        install_review: &review,
        client_scope_class,
        client_scope_summary: client_scope_class.label(),
        script_risk: script_risk(script_risk_class),
        manifest_changes: manifest_changes(catalog),
        lockfile_impact: lockfile_impact(LockfileImpactClass::LockfileChurnExpected),
        generated_at: "2026-05-16T19:20:00Z",
    })
    .expect("fact grid must project")
}

#[test]
fn public_catalog_fact_grid_carries_workspace_change_truth() {
    let catalog = catalog_record();
    let grid = fact_grid_for(
        "public-beta",
        &catalog,
        "extension_bridge_row:wasm_component_native_beta",
        ClientScopeClass::DesktopPlusBrowserCompanion,
        ScriptRiskClass::NoScriptsOrNativeBuild,
    );

    assert!(validate_marketplace_fact_grid(&grid).is_empty());
    assert_eq!(
        grid.client_scope_class,
        ClientScopeClass::DesktopPlusBrowserCompanion
    );
    assert_eq!(
        grid.registry_source_class,
        CatalogRegistrySourceClass::PublicRegistry
    );
    assert_eq!(grid.manifest_changes.len(), 2);
    assert_eq!(
        grid.permission_delta_count,
        grid.permission_delta_entries.len()
    );
    assert_eq!(
        grid.lockfile_impact.impact_class,
        LockfileImpactClass::LockfileChurnExpected
    );
    assert_eq!(
        grid.runtime_cost_class,
        RuntimeCostClass::RuntimeCostLowNominal
    );
}

#[test]
fn mirrored_catalog_fact_grid_preserves_same_vocabulary() {
    let catalog = evaluated_catalog_fixture("staged_pending_moderation");
    let grid = fact_grid_for(
        "mirror-preview",
        &catalog,
        "extension_bridge_row:wasm_component_native_beta",
        ClientScopeClass::Desktop,
        ScriptRiskClass::LifecycleScriptsDeclared,
    );

    assert!(validate_marketplace_fact_grid(&grid).is_empty());
    assert_eq!(
        grid.registry_source_class,
        CatalogRegistrySourceClass::ApprovedMirror
    );
    assert!(grid
        .trust_chips
        .contains(&MarketplaceTrustChipClass::ApprovedMirror));
    assert_eq!(
        grid.script_risk.script_risk_class,
        ScriptRiskClass::LifecycleScriptsDeclared
    );
    assert!(grid.workspace_change_summary.contains("Manifest changes"));
}

#[test]
fn support_export_quotes_fact_grid_truth_without_drift() {
    let catalog = catalog_record();
    let grid = fact_grid_for(
        "public-beta",
        &catalog,
        "extension_bridge_row:wasm_component_native_beta",
        ClientScopeClass::DesktopPlusBrowserCompanion,
        ScriptRiskClass::NoScriptsOrNativeBuild,
    );
    let export = project_marketplace_fact_grid_support_export(
        &grid,
        "marketplace_fact_grid_support_export:public-beta",
    );

    assert!(validate_marketplace_fact_grid_support_export(&export).is_empty());
    assert_eq!(export.fact_grid_ref, grid.fact_grid_id);
    assert_eq!(export.client_scope_class, grid.client_scope_class);
    assert_eq!(export.registry_source_class, grid.registry_source_class);
    assert_eq!(
        export.lockfile_impact_class,
        grid.lockfile_impact.impact_class
    );
    assert_eq!(export.permission_delta_count, grid.permission_delta_count);
}

#[test]
fn revoked_or_pending_reverify_state_blocks_install_update() {
    let catalog = evaluated_catalog_fixture("revoked_catalog_refused");
    let grid = fact_grid_for(
        "revoked",
        &catalog,
        "extension_bridge_row:unsupported_webview_runtime",
        ClientScopeClass::HeadlessOnly,
        ScriptRiskClass::ExternalHelperOrHost,
    );

    assert!(validate_marketplace_fact_grid(&grid).is_empty());
    assert!(grid.blocks_install_or_update);
    assert!(grid.quarantine_revocation.install_or_update_blocked);
    assert_eq!(
        grid.quarantine_revocation.revocation_state_class,
        RevocationStateClass::Revoked
    );
}

#[test]
fn unknown_lockfile_impact_must_block_mutation() {
    let catalog = catalog_record();
    let install_review_ref = "install_review_alpha:dev.aureline.samples/wasm-notes:unknown-lock";
    let row = marketplace_row(
        "unknown-lock",
        &catalog,
        "extension_bridge_row:wasm_component_native_beta",
        install_review_ref,
    );
    let review = install_review_packet(install_review_ref);
    let grid = project_marketplace_fact_grid(MarketplaceFactGridInput {
        fact_grid_id: "marketplace_fact_grid:dev.aureline.samples/wasm-notes:unknown-lock",
        surface_class: MarketplaceFactGridSurfaceClass::InstallReviewSheet,
        marketplace_row: &row,
        catalog: &catalog,
        install_review: &review,
        client_scope_class: ClientScopeClass::Desktop,
        client_scope_summary: "Desktop",
        script_risk: script_risk(ScriptRiskClass::NoScriptsOrNativeBuild),
        manifest_changes: manifest_changes(&catalog),
        lockfile_impact: LockfileImpact {
            impact_class: LockfileImpactClass::UnknownBlocked,
            resolver_ref: "resolver:aureline-extension-lock:v1".to_string(),
            affected_lockfile_refs: Vec::new(),
            generated_file_refs: Vec::new(),
            environment_factor_refs: Vec::new(),
            rollback_checkpoint_ref: None,
            summary: "Lockfile impact is unknown.".to_string(),
        },
        generated_at: "2026-05-16T19:20:00Z",
    })
    .expect("unknown impact still projects as blocked fact grid");

    assert!(grid.blocks_install_or_update);
    assert!(validate_marketplace_fact_grid(&grid).is_empty());
}
