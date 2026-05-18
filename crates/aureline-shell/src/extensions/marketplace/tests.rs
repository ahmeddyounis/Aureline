//! Unit coverage for shell marketplace truth page projection.

use aureline_extensions::{
    CatalogRegistrySourceClass, ClientScopeClass, LockfileImpactClass,
    MarketplaceCompatibilityLabelClass, MarketplaceFactGridSurfaceClass,
    MarketplaceSupportChipClass, MarketplaceTrustChipClass, ScriptRiskClass,
};

use super::*;

#[test]
fn seeded_marketplace_truth_page_validates() {
    let page = seeded_marketplace_truth_page();

    validate_marketplace_truth_page(&page).expect("seeded marketplace truth page must validate");
    assert_eq!(
        page.extension_bridge_matrix_id,
        "extension_bridge_matrix:m3.beta"
    );
    assert_eq!(
        page.extension_compatibility_report_ref,
        "artifacts/compat/m3/extension_compatibility_report.md"
    );
    assert_eq!(page.rows.len(), page.support_rows.len());
    assert_eq!(page.rows.len(), page.fact_grids.len());
    assert_eq!(page.fact_grids.len(), page.fact_grid_support_rows.len());
    assert_eq!(
        page.controlled_badge_vocabulary,
        MarketplaceTruthBadgeClass::required_acceptance_states().to_vec()
    );
}

#[test]
fn public_beta_row_uses_generated_report_support_not_catalog_copy() {
    let page = seeded_marketplace_truth_page();
    let row = page
        .rows
        .iter()
        .find(|row| row.row_id.ends_with(":public-beta"))
        .expect("seed row exists");

    assert_eq!(
        row.support_chips,
        vec![MarketplaceSupportChipClass::Experimental]
    );
    assert_eq!(
        row.compatibility_label_class,
        MarketplaceCompatibilityLabelClass::Limited
    );
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Beta));
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Mirrored));
    assert!(row
        .support_summary
        .contains("current generated compatibility report"));
    assert_eq!(
        row.extension_bridge_matrix_row_id,
        "extension_bridge_row:wasm_component_native_beta"
    );
    assert!(row.bridge_summary.contains("runtime window"));
}

#[test]
fn bridge_backed_row_quotes_bridge_matrix_limits() {
    let page = seeded_marketplace_truth_page();
    let row = page
        .rows
        .iter()
        .find(|row| row.row_id.ends_with(":mirror-retest-pending"))
        .expect("bridge-backed seed row exists");

    assert_eq!(
        row.extension_bridge_matrix_row_id,
        "extension_bridge_row:vscode_api_bridge_beta"
    );
    assert_eq!(
        row.extension_bridge_state_class,
        aureline_extensions::ExtensionBridgeStateClass::Bridge
    );
    assert!(!row.bridge_known_limits.is_empty());
}

#[test]
fn seeded_rows_cover_preinstall_trust_states() {
    let page = seeded_marketplace_truth_page();
    for badge in [
        MarketplaceTruthBadgeClass::Preview,
        MarketplaceTruthBadgeClass::Beta,
        MarketplaceTruthBadgeClass::Limited,
        MarketplaceTruthBadgeClass::Revoked,
        MarketplaceTruthBadgeClass::Mirrored,
        MarketplaceTruthBadgeClass::RetestPending,
    ] {
        assert!(
            page.rows
                .iter()
                .any(|row| row.lifecycle_badges.contains(&badge)),
            "seeded rows should cover {:?}",
            badge
        );
    }
    assert_eq!(page.summary.row_count, 6);
    assert!(page.summary.blocked_install_or_update_count >= 2);
    assert!(page.rows.iter().any(|row| row
        .trust_chips
        .contains(&MarketplaceTrustChipClass::OfflineBundle)));
    assert!(page.rows.iter().any(|row| row
        .trust_chips
        .contains(&MarketplaceTrustChipClass::LocalArchive)));
}

#[test]
fn support_export_rows_quote_source_rows() {
    let page = seeded_marketplace_truth_page();

    for row in &page.rows {
        let export = page
            .support_rows
            .iter()
            .find(|export| export.row_ref == row.row_id)
            .expect("support row exists for marketplace row");
        assert_eq!(export.lifecycle_badges, row.lifecycle_badges);
        assert_eq!(
            export.compatibility_label_class,
            row.compatibility_label_class
        );
        assert_eq!(export.support_chips, row.support_chips);
        assert_eq!(export.trust_chips, row.trust_chips);
        assert_eq!(export.install_review_ref, row.install_review_ref);
        assert_eq!(
            export.extension_bridge_matrix_row_id,
            row.extension_bridge_matrix_row_id
        );
    }
}

#[test]
fn fact_grids_cover_client_scope_and_workspace_change_truth() {
    let page = seeded_marketplace_truth_page();

    for scope in [
        ClientScopeClass::Desktop,
        ClientScopeClass::BrowserCompanion,
        ClientScopeClass::DesktopPlusBrowserCompanion,
        ClientScopeClass::HeadlessOnly,
    ] {
        assert!(
            page.fact_grids
                .iter()
                .any(|grid| grid.client_scope_class == scope),
            "seeded fact grids should cover {:?}",
            scope
        );
    }

    for source in [
        CatalogRegistrySourceClass::PublicRegistry,
        CatalogRegistrySourceClass::ApprovedMirror,
        CatalogRegistrySourceClass::PrivateRegistry,
        CatalogRegistrySourceClass::OfflineBundle,
        CatalogRegistrySourceClass::LocalArchive,
    ] {
        assert!(
            page.fact_grids
                .iter()
                .any(|grid| grid.registry_source_class == source),
            "seeded fact grids should cover {:?}",
            source
        );
    }

    for surface in [
        MarketplaceFactGridSurfaceClass::ResultRow,
        MarketplaceFactGridSurfaceClass::ManualImportReview,
        MarketplaceFactGridSurfaceClass::OfflineRegistryRow,
    ] {
        assert!(
            page.fact_grids
                .iter()
                .any(|grid| grid.surface_class == surface),
            "seeded fact grids should cover {:?}",
            surface
        );
    }

    let public_grid = page
        .fact_grids
        .iter()
        .find(|grid| grid.fact_grid_id.ends_with(":public-beta"))
        .expect("public fact grid exists");
    assert_eq!(
        public_grid.client_scope_class,
        ClientScopeClass::DesktopPlusBrowserCompanion
    );
    assert_eq!(
        public_grid.script_risk.script_risk_class,
        ScriptRiskClass::NoScriptsOrNativeBuild
    );
    assert_eq!(
        public_grid.lockfile_impact.impact_class,
        LockfileImpactClass::LockfileChurnExpected
    );
    assert_eq!(
        public_grid.permission_delta_count,
        public_grid.permission_delta_entries.len()
    );
    assert!(!public_grid.manifest_changes.is_empty());

    let bridge_grid = page
        .fact_grids
        .iter()
        .find(|grid| grid.fact_grid_id.ends_with(":mirror-retest-pending"))
        .expect("bridge-backed fact grid exists");
    assert_eq!(
        bridge_grid.script_risk.script_risk_class,
        ScriptRiskClass::NativeBuildRequired
    );
    assert!(bridge_grid.blocks_install_or_update);

    let manual_grid = page
        .fact_grids
        .iter()
        .find(|grid| grid.fact_grid_id.ends_with(":manual-import"))
        .expect("manual-import fact grid exists");
    assert_eq!(
        manual_grid.surface_class,
        MarketplaceFactGridSurfaceClass::ManualImportReview
    );
    assert_eq!(
        manual_grid.registry_source_class,
        CatalogRegistrySourceClass::LocalArchive
    );
    assert_eq!(
        manual_grid.script_risk.script_risk_class,
        ScriptRiskClass::UnknownScriptRiskBlocked
    );
    assert_eq!(
        manual_grid.lockfile_impact.impact_class,
        LockfileImpactClass::RegenerateAndReview
    );
    assert!(manual_grid.blocks_install_or_update);
}

#[test]
fn fact_grid_support_exports_quote_source_grids() {
    let page = seeded_marketplace_truth_page();

    for grid in &page.fact_grids {
        let export = page
            .fact_grid_support_rows
            .iter()
            .find(|export| export.fact_grid_ref == grid.fact_grid_id)
            .expect("support row exists for fact grid");
        assert_eq!(export.client_scope_class, grid.client_scope_class);
        assert_eq!(export.registry_source_class, grid.registry_source_class);
        assert_eq!(
            export.compatibility_label_class,
            grid.compatibility_label_class
        );
        assert_eq!(export.script_risk_class, grid.script_risk.script_risk_class);
        assert_eq!(
            export.lockfile_impact_class,
            grid.lockfile_impact.impact_class
        );
        assert_eq!(export.permission_delta_count, grid.permission_delta_count);
    }
}

#[test]
fn validator_detects_fact_grid_support_export_drift() {
    let mut page = seeded_marketplace_truth_page();
    page.fact_grid_support_rows[0].permission_delta_count += 1;

    let errors =
        validate_marketplace_truth_page(&page).expect_err("fact-grid drift must fail validation");
    assert!(errors.iter().any(|error| matches!(
        error,
        MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift { field, .. }
            if field == "permission_delta_count"
    )));
}

#[test]
fn validator_detects_support_export_drift() {
    let mut page = seeded_marketplace_truth_page();
    page.support_rows[0].support_chips.clear();

    let errors = validate_marketplace_truth_page(&page).expect_err("drift must fail validation");
    assert!(errors.iter().any(|error| matches!(
        error,
        MarketplaceTruthPageValidationError::SupportExportParityDrift { field, .. }
            if field == "support_chips"
    )));
}

#[test]
fn validator_detects_bridge_support_export_drift() {
    let mut page = seeded_marketplace_truth_page();
    let export = page
        .support_rows
        .iter_mut()
        .find(|export| export.row_ref.ends_with(":mirror-retest-pending"))
        .expect("bridge-backed support export exists");
    export.bridge_known_limits.clear();

    let errors =
        validate_marketplace_truth_page(&page).expect_err("bridge drift must fail validation");
    assert!(errors.iter().any(|error| matches!(
        error,
        MarketplaceTruthPageValidationError::SupportExportParityDrift { field, .. }
            if field == "bridge_known_limits"
    )));
}
