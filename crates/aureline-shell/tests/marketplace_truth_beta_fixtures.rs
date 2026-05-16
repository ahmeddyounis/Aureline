//! Fixture replay for marketplace truth rows.
//!
//! Fixtures live under `fixtures/ux/m3/marketplace_truth/` and are generated
//! by the `aureline_shell_marketplace_truth` headless inspector.

use aureline_extensions::{
    MarketplaceCompatibilityLabelClass, MarketplaceSupportChipClass, MarketplaceTruthBadgeClass,
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

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_round_trip_through_shared_types() {
    let rows: Vec<MarketplaceTruthRowRecord> = load("rows.json");
    assert_eq!(rows.len(), 4);
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

    let retest = rows
        .iter()
        .find(|row| row.row_id.ends_with(":mirror-retest-pending"))
        .expect("retest row exists");
    assert!(retest.blocks_install_or_update);
    assert!(retest
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::RetestPending));

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
    }

    let page: MarketplaceTruthPageRecord = load("page.json");
    assert_eq!(page.schema_version, MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        MARKETPLACE_TRUTH_SHARED_CONTRACT_REF
    );
    assert_eq!(
        page.controlled_badge_vocabulary,
        MarketplaceTruthBadgeClass::required_acceptance_states().to_vec()
    );
    validate_marketplace_truth_page(&page).expect("page fixture must validate");
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
