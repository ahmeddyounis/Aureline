//! Unit coverage for marketplace truth-row projections.

use serde::Deserialize;

use super::{
    project_marketplace_truth_row, project_marketplace_truth_support_export,
    validate_marketplace_truth_row, validate_marketplace_truth_support_export,
    CompatibilityReportSnapshot, MarketplaceCompatibilityLabelClass, MarketplaceSupportChipClass,
    MarketplaceTruthBadgeClass, MarketplaceTruthRowInput,
};
use crate::compatibility_matrix::{current_extension_bridge_matrix, ExtensionBridgeMatrix};
use crate::registry::{
    evaluate_catalog_descriptor, CatalogDescriptorInput, CatalogDescriptorRecord,
};

#[derive(Debug, Deserialize)]
struct CatalogFixture {
    input: CatalogDescriptorInput,
}

fn compatibility_report() -> CompatibilityReportSnapshot {
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

fn project_row(
    row_suffix: &str,
    catalog: &CatalogDescriptorRecord,
) -> super::MarketplaceTruthRowRecord {
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: &format!("marketplace_truth_row:{row_suffix}"),
        catalog,
        compatibility_report: &report,
        compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
        extension_bridge_matrix: &bridge_matrix,
        extension_bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
        install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
        generated_at: "2026-05-16T19:00:00Z",
    })
    .expect("marketplace row must project")
}

#[test]
fn controlled_badge_vocabulary_covers_claimed_marketplace_states() {
    let tokens: Vec<&str> = MarketplaceTruthBadgeClass::required_acceptance_states()
        .iter()
        .map(|badge| badge.as_str())
        .collect();

    assert_eq!(
        tokens,
        vec![
            "preview",
            "beta",
            "stable",
            "deprecated",
            "limited",
            "revoked",
            "mirrored",
            "retest_pending"
        ]
    );
}

#[test]
fn generated_report_narrows_catalog_exact_label_on_beta_marketplace_row() {
    let catalog = catalog_record();
    let row = project_row("dev.aureline.samples/wasm-notes:beta", &catalog);

    assert!(validate_marketplace_truth_row(&row).is_empty());
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
        .contains(&MarketplaceTruthBadgeClass::Limited));
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Mirrored));
    assert_eq!(
        row.catalog_rendered_compatibility_label,
        crate::install_review::CompatibilityLabel::Exact
    );
    assert_eq!(row.report_effective_support_class, "experimental");
    assert_eq!(
        row.extension_bridge_matrix_row_id,
        "extension_bridge_row:wasm_component_native_beta"
    );
    assert_eq!(
        row.runtime_compatibility_window_id,
        "runtime_window:extension_host_v1_beta.wasm_component"
    );
    assert!(row
        .support_summary
        .contains("compat_row:extension_host.sdk_wit_permission_window"));

    let export = project_marketplace_truth_support_export(
        &row,
        "marketplace_truth_support_export:dev.aureline.samples/wasm-notes:beta",
    );
    assert!(validate_marketplace_truth_support_export(&export).is_empty());
    assert_eq!(export.row_ref, row.row_id);
    assert_eq!(
        export.compatibility_label_class,
        row.compatibility_label_class
    );
}

#[test]
fn support_export_refuses_bridge_row_without_limits() {
    let catalog = evaluated_catalog_fixture("limited_compatibility_catalog");
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    let row = project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: "marketplace_truth_row:dev.aureline.samples/wasm-notes:bridge-limits",
        catalog: &catalog,
        compatibility_report: &report,
        compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
        extension_bridge_matrix: &bridge_matrix,
        extension_bridge_matrix_row_ref: "extension_bridge_row:vscode_api_bridge_beta",
        install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:limited",
        generated_at: "2026-05-16T19:00:00Z",
    })
    .expect("bridge-backed row must project");
    let mut export = project_marketplace_truth_support_export(
        &row,
        "marketplace_truth_support_export:dev.aureline.samples/wasm-notes:bridge-limits",
    );
    export.bridge_known_limits.clear();

    let findings = validate_marketplace_truth_support_export(&export);
    assert!(findings
        .iter()
        .any(|finding| { finding.check_id == "marketplace_truth_export.bridge_limits_missing" }));
}

#[test]
fn mirror_pending_reverify_renders_retest_pending_before_install_review() {
    let catalog = evaluated_catalog_fixture("limited_compatibility_catalog");
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    let row = project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: "marketplace_truth_row:dev.aureline.samples/wasm-notes:retest",
        catalog: &catalog,
        compatibility_report: &report,
        compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
        extension_bridge_matrix: &bridge_matrix,
        extension_bridge_matrix_row_ref: "extension_bridge_row:vscode_api_bridge_beta",
        install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:limited",
        generated_at: "2026-05-16T19:00:00Z",
    })
    .expect("bridge-backed row must project");

    assert!(validate_marketplace_truth_row(&row).is_empty());
    assert!(row.blocks_install_or_update);
    assert_eq!(
        row.compatibility_label_class,
        MarketplaceCompatibilityLabelClass::RetestPending
    );
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::RetestPending));
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Mirrored));
    assert_eq!(
        row.extension_bridge_matrix_row_id,
        "extension_bridge_row:vscode_api_bridge_beta"
    );
    assert!(!row.bridge_known_limits.is_empty());
}

#[test]
fn revoked_catalog_row_remains_visible_but_blocks_install_update() {
    let catalog = evaluated_catalog_fixture("revoked_catalog_refused");
    let row = project_row("dev.aureline.samples/wasm-notes:revoked", &catalog);

    assert!(validate_marketplace_truth_row(&row).is_empty());
    assert!(row.blocks_install_or_update);
    assert_eq!(
        row.compatibility_label_class,
        MarketplaceCompatibilityLabelClass::Unsupported
    );
    assert!(row
        .lifecycle_badges
        .contains(&MarketplaceTruthBadgeClass::Revoked));
}

#[test]
fn missing_generated_report_row_refuses_projection() {
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    let catalog = catalog_record();
    let finding = project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: "marketplace_truth_row:missing-report-row",
        catalog: &catalog,
        compatibility_report: &report,
        compatibility_report_row_ref: "compat_row:missing",
        extension_bridge_matrix: &bridge_matrix,
        extension_bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
        install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
        generated_at: "2026-05-16T19:00:00Z",
    })
    .expect_err("missing report row must refuse projection");

    assert_eq!(
        finding.check_id,
        "marketplace_truth.compatibility_report_row_missing"
    );
}

#[test]
fn missing_bridge_matrix_row_refuses_projection() {
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    let catalog = catalog_record();
    let finding = project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: "marketplace_truth_row:missing-bridge-row",
        catalog: &catalog,
        compatibility_report: &report,
        compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
        extension_bridge_matrix: &bridge_matrix,
        extension_bridge_matrix_row_ref: "extension_bridge_row:missing",
        install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
        generated_at: "2026-05-16T19:00:00Z",
    })
    .expect_err("missing bridge row must refuse projection");

    assert_eq!(
        finding.check_id,
        "marketplace_truth.extension_bridge_matrix_row_missing"
    );
}
