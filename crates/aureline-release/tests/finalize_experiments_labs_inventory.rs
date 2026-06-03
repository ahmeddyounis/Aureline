//! Fixture replay for the finalized experiments/Labs inventory.

use aureline_release::finalize_experiments_labs_inventory::{
    audit_finalize_experiments_labs_inventory_page,
    project_cli_inventory,
    project_support_export,
    seeded_finalize_experiments_labs_inventory_page,
    validate_finalize_experiments_labs_inventory_page,
    FinalizeExperimentsLabsInventoryPage, InventoryQualificationClass,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF,
};

#[test]
fn seeded_page_validates_cleanly() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    validate_finalize_experiments_labs_inventory_page(&page)
        .expect("seeded page should validate");
}

#[test]
fn seeded_page_audit_is_clean() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    let extra = audit_finalize_experiments_labs_inventory_page(&page);
    assert!(extra.is_empty(), "audit should be clean, got {extra:?}");
}

#[test]
fn fixture_round_trips() {
    let page: FinalizeExperimentsLabsInventoryPage = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/release/m4/finalize-experiments-labs-inventory/certification_page.json"
        )
    ))
    .expect("fixture parses");

    assert_eq!(
        page.schema_version,
        FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION
    );
    assert_eq!(
        page.shared_contract_ref,
        FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF
    );
    validate_finalize_experiments_labs_inventory_page(&page)
        .expect("fixture page validates");
}

#[test]
fn cli_projection_matches_page() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    let cli = project_cli_inventory(&page);
    assert_eq!(cli.rows.len(), page.rows.len());
    assert_eq!(
        cli.schema_version,
        FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION
    );
}

#[test]
fn support_export_matches_page() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    let export = project_support_export("test:export", &page);
    assert_eq!(export.rows.len(), page.rows.len());
    assert_eq!(export.defects.len(), page.defects.len());
}

#[test]
fn lifecycle_vocabulary_is_exactly_controlled_set() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    let valid = [
        "Labs",
        "Preview",
        "Beta",
        "Stable",
        "Deprecated",
        "DisabledByPolicy",
        "Retired",
    ];
    for row in &page.rows {
        assert!(
            valid.contains(&row.effective_lifecycle_state.as_str()),
            "unrecognized lifecycle {} for capability {}",
            row.effective_lifecycle_state,
            row.capability_id
        );
    }
}

#[test]
fn disabled_rows_expose_kill_switch() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    for row in &page.rows {
        if row.effective_lifecycle_state == "DisabledByPolicy" {
            assert!(
                row.kill_switch_visibility.is_some(),
                "disabled row {} must expose kill_switch_visibility",
                row.capability_id
            );
        }
    }
}

#[test]
fn every_row_has_owner_and_expiry() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    for row in &page.rows {
        assert!(
            !row.owner.trim().is_empty(),
            "row {} missing owner",
            row.capability_id
        );
        assert!(
            !row.review_or_expiry_date.trim().is_empty(),
            "row {} missing expiry",
            row.capability_id
        );
    }
}

#[test]
fn stable_rows_have_no_non_stable_dependencies() {
    let page = seeded_finalize_experiments_labs_inventory_page();
    for row in &page.rows {
        if row.qualification == InventoryQualificationClass::FinalizedStable {
            for marker in &row.dependency_markers {
                assert_eq!(
                    marker.required_lifecycle_state, "Stable",
                    "stable row {} depends on non-stable capability {} ({})",
                    row.capability_id,
                    marker.required_capability_id,
                    marker.required_lifecycle_state
                );
            }
        }
    }
}
