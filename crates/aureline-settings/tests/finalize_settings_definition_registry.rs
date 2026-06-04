//! Fixture replay for the finalized settings-definition registry.

use aureline_settings::finalize_settings_definition_registry::{
    audit_finalize_settings_definition_registry_page, project_cli_inventory,
    project_support_export, seeded_finalize_settings_definition_registry_page,
    validate_finalize_settings_definition_registry_page, FinalizeSettingsDefinitionRegistryPage,
    InspectSurfaceClass, RegistryQualificationClass,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF,
};

#[test]
fn seeded_page_validates_cleanly() {
    let page = seeded_finalize_settings_definition_registry_page();
    validate_finalize_settings_definition_registry_page(&page)
        .expect("seeded page should validate");
}

#[test]
fn seeded_page_audit_is_clean() {
    let page = seeded_finalize_settings_definition_registry_page();
    let extra = audit_finalize_settings_definition_registry_page(&page);
    assert!(extra.is_empty(), "audit should be clean, got {extra:?}");
}

#[test]
fn fixture_round_trips() {
    let page: FinalizeSettingsDefinitionRegistryPage = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/m4/finalize-settings-definition-registry/certification_page.json"
    )))
    .expect("fixture parses");

    assert_eq!(
        page.schema_version,
        FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION
    );
    assert_eq!(
        page.shared_contract_ref,
        FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF
    );
    validate_finalize_settings_definition_registry_page(&page).expect("fixture page validates");
}

#[test]
fn cli_projection_matches_page() {
    let page = seeded_finalize_settings_definition_registry_page();
    let cli = project_cli_inventory(&page);
    assert_eq!(cli.rows.len(), page.rows.len());
    assert_eq!(
        cli.schema_version,
        FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION
    );
}

#[test]
fn support_export_matches_page() {
    let page = seeded_finalize_settings_definition_registry_page();
    let export = project_support_export("test:export", &page);
    assert_eq!(export.rows.len(), page.rows.len());
    assert_eq!(export.defects.len(), page.defects.len());
}

#[test]
fn every_surface_class_renders_parity() {
    let page = seeded_finalize_settings_definition_registry_page();
    for row in &page.rows {
        for surface in &row.surface_parity {
            let valid_surfaces: Vec<&str> = InspectSurfaceClass::ALL
                .iter()
                .map(|s| s.as_str())
                .collect();
            assert!(
                valid_surfaces.contains(&surface.surface.as_str()),
                "unknown surface {} for setting {}",
                surface.surface,
                row.setting_id
            );
        }
    }
}

#[test]
fn stable_rows_have_no_hidden_dependencies() {
    let page = seeded_finalize_settings_definition_registry_page();
    for row in &page.rows {
        if row.qualification == RegistryQualificationClass::FinalizedStable {
            assert!(
                row.dependency_markers.is_empty(),
                "stable row {} must not have dependency markers",
                row.setting_id
            );
        }
    }
}
