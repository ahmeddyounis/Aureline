//! Fixture replay for marketplace truth rows.
//!
//! Fixtures live under `fixtures/ux/m3/marketplace_truth/` and are generated
//! by the `aureline_shell_marketplace_truth` headless inspector.

use aureline_extensions::{
    CatalogRegistrySourceClass, ClientScopeClass, MarketplaceCompatibilityLabelClass,
    MarketplaceFactGridRecord, MarketplaceFactGridSupportExportRecord,
    MarketplaceFactGridSurfaceClass, MarketplaceSupportChipClass, MarketplaceTruthBadgeClass,
    MarketplaceTruthRowRecord, MarketplaceTruthSupportExportRecord,
};
use aureline_shell::extensions::marketplace::{
    seeded_marketplace_truth_page, validate_marketplace_truth_page, MarketplaceTruthPageRecord,
    MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION, MARKETPLACE_TRUTH_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m3/marketplace_truth"
);
const EXTENSION_FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/extensions/m3/fact_grid_and_install_review"
);

fn load_from<T: serde::de::DeserializeOwned>(dir: &str, filename: &str) -> T {
    let path = format!("{}/{}", dir, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    load_from(FIXTURE_DIR, filename)
}

#[test]
fn fixtures_round_trip_through_shared_types() {
    let rows: Vec<MarketplaceTruthRowRecord> = load("rows.json");
    assert_eq!(rows.len(), 6);
    for row in &rows {
        assert_eq!(row.marketplace_truth_schema_version, 1);
        assert_eq!(
            row.compatibility_label_source_class,
            aureline_extensions::MarketplaceCompatibilityLabelSourceClass::GeneratedCompatibilityReport
        );
        assert!(
            !row.install_review_ref.trim().is_empty(),
            "{} must cite install review",
            row.row_id
        );
        assert!(
            row.extension_bridge_matrix_id
                .starts_with("extension_bridge_matrix:"),
            "{} must cite bridge matrix",
            row.row_id
        );
        assert!(
            row.extension_bridge_matrix_row_id
                .starts_with("extension_bridge_row:"),
            "{} must cite bridge matrix row",
            row.row_id
        );
        assert!(
            !row.support_chips.is_empty(),
            "{} must render support chips",
            row.row_id
        );
        assert!(
            !row.trust_chips.is_empty(),
            "{} must render trust chips",
            row.row_id
        );
    }

    let public_beta = rows
        .iter()
        .find(|row| row.row_id.ends_with(":public-beta"))
        .expect("public beta row exists");
    assert_eq!(
        public_beta.support_chips,
        vec![MarketplaceSupportChipClass::Experimental]
    );
    assert_eq!(
        public_beta.compatibility_label_class,
        MarketplaceCompatibilityLabelClass::Limited
    );
    assert_eq!(
        public_beta.catalog_rendered_compatibility_label,
        aureline_extensions::CompatibilityLabel::Exact
    );
    assert_eq!(
        public_beta.extension_bridge_matrix_row_id,
        "extension_bridge_row:wasm_component_native_beta"
    );

    let retest = rows
        .iter()
        .find(|row| row.row_id.ends_with(":mirror-retest-pending"))
        .expect("retest row exists");
    assert!(retest.blocks_install_or_update);
    assert!(retest
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::RetestPending));
    assert_eq!(
        retest.extension_bridge_matrix_row_id,
        "extension_bridge_row:vscode_api_bridge_beta"
    );
    assert!(!retest.bridge_known_limits.is_empty());

    let support_rows: Vec<MarketplaceTruthSupportExportRecord> = load("support_rows.json");
    assert_eq!(support_rows.len(), rows.len());
    for row in &rows {
        let export = support_rows
            .iter()
            .find(|export| export.row_ref == row.row_id)
            .expect("support row exists");
        assert_eq!(export.lifecycle_badges, row.lifecycle_badges);
        assert_eq!(
            export.compatibility_label_class,
            row.compatibility_label_class
        );
        assert_eq!(export.support_chips, row.support_chips);
        assert_eq!(export.trust_chips, row.trust_chips);
        assert_eq!(
            export.extension_bridge_matrix_row_id,
            row.extension_bridge_matrix_row_id
        );
    }

    let fact_grids: Vec<MarketplaceFactGridRecord> = load("fact_grids.json");
    assert_eq!(fact_grids.len(), rows.len());
    for grid in &fact_grids {
        assert_eq!(grid.marketplace_fact_grid_schema_version, 1);
        assert!(!grid.client_scope_summary.trim().is_empty());
        assert!(!grid.manifest_changes.is_empty());
        assert_eq!(
            grid.permission_delta_count,
            grid.permission_delta_entries.len()
        );
        assert!(!grid.lockfile_impact.summary.trim().is_empty());
    }
    assert!(fact_grids
        .iter()
        .any(|grid| { grid.client_scope_class == ClientScopeClass::DesktopPlusBrowserCompanion }));
    assert!(fact_grids
        .iter()
        .any(|grid| grid.client_scope_class == ClientScopeClass::BrowserCompanion));
    assert!(fact_grids
        .iter()
        .any(|grid| grid.registry_source_class == CatalogRegistrySourceClass::OfflineBundle));
    assert!(fact_grids
        .iter()
        .any(|grid| grid.registry_source_class == CatalogRegistrySourceClass::LocalArchive));
    assert!(fact_grids
        .iter()
        .any(|grid| grid.surface_class == MarketplaceFactGridSurfaceClass::OfflineRegistryRow));
    assert!(fact_grids
        .iter()
        .any(|grid| grid.surface_class == MarketplaceFactGridSurfaceClass::ManualImportReview));

    let fact_grid_support_rows: Vec<MarketplaceFactGridSupportExportRecord> =
        load("fact_grid_support_rows.json");
    assert_eq!(fact_grid_support_rows.len(), fact_grids.len());
    for grid in &fact_grids {
        let export = fact_grid_support_rows
            .iter()
            .find(|export| export.fact_grid_ref == grid.fact_grid_id)
            .expect("fact-grid support row exists");
        assert_eq!(export.client_scope_class, grid.client_scope_class);
        assert_eq!(export.registry_source_class, grid.registry_source_class);
        assert_eq!(
            export.compatibility_label_class,
            grid.compatibility_label_class
        );
        assert_eq!(
            export.lockfile_impact_class,
            grid.lockfile_impact.impact_class
        );
    }

    let page: MarketplaceTruthPageRecord = load("page.json");
    assert_eq!(page.schema_version, MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        MARKETPLACE_TRUTH_SHARED_CONTRACT_REF
    );
    assert_eq!(
        page.extension_bridge_matrix_id,
        "extension_bridge_matrix:m3.beta"
    );
    assert_eq!(
        page.controlled_badge_vocabulary,
        MarketplaceTruthBadgeClass::required_acceptance_states().to_vec()
    );
    validate_marketplace_truth_page(&page).expect("page fixture must validate");
}

#[test]
fn extension_fixture_corpus_matches_shell_fact_grids() {
    let extension_grids: Vec<MarketplaceFactGridRecord> =
        load_from(EXTENSION_FIXTURE_DIR, "fact_grids.json");
    let extension_exports: Vec<MarketplaceFactGridSupportExportRecord> =
        load_from(EXTENSION_FIXTURE_DIR, "support_exports.json");
    let page = seeded_marketplace_truth_page();

    assert_eq!(extension_grids, page.fact_grids);
    assert_eq!(extension_exports, page.fact_grid_support_rows);
}

#[test]
fn page_fixture_matches_seeded_builder() {
    let from_file: MarketplaceTruthPageRecord = load("page.json");
    let from_code = seeded_marketplace_truth_page();
    assert_eq!(
        from_file, from_code,
        "fixture must be a literal projection of the seeded marketplace truth page"
    );
}
