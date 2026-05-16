//! Unit and fixture coverage for extension inspector parity.

use super::{
    seeded_extension_inspector_page, seeded_extension_inspector_support_export,
    validate_extension_inspector_page, validate_extension_inspector_support_export,
    ExtensionCapabilityDispositionClass, ExtensionInspectorPage, ExtensionInspectorSupportExport,
    ExtensionSettingRedactionClass,
};

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!(
        "{}/../../fixtures/ux/m3/extension_inspectors/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    );
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn seeded_page_covers_permission_and_setting_acceptance() {
    let page = seeded_extension_inspector_page();
    validate_extension_inspector_page(&page).expect("seeded page must validate");

    assert_eq!(page.permission_inspector.summary.granted_count, 2);
    assert_eq!(page.permission_inspector.summary.policy_locked_count, 2);
    assert_eq!(page.permission_inspector.summary.denied_count, 1);
    assert!(page
        .permission_inspector
        .rows
        .iter()
        .any(|row| { row.disposition_class == ExtensionCapabilityDispositionClass::Granted }));
    assert!(page
        .permission_inspector
        .rows
        .iter()
        .any(|row| { row.disposition_class == ExtensionCapabilityDispositionClass::PolicyLocked }));
    assert!(page
        .permission_inspector
        .rows
        .iter()
        .any(|row| { row.disposition_class == ExtensionCapabilityDispositionClass::Denied }));
    assert_eq!(
        page.permission_inspector.host_placement_class,
        aureline_extensions::HostPlacementClass::WasmIsolatedSubprocess
    );
    assert_eq!(
        page.permission_inspector.lifecycle_state_class,
        aureline_extensions::RuntimeLifecycleStateClass::Active
    );

    assert_eq!(page.settings_inspector.summary.row_count, 3);
    assert_eq!(page.settings_inspector.summary.policy_locked_count, 1);
    assert_eq!(page.settings_inspector.summary.redacted_value_count, 1);
    for row in &page.settings_inspector.rows {
        assert!(!row.source_chain.is_empty());
        assert!(!row.diff_rows.is_empty());
        assert!(!row.raw_secret_value_exported);
    }
    assert!(page
        .settings_inspector
        .rows
        .iter()
        .any(|row| row.redaction_class == ExtensionSettingRedactionClass::SecretValueRedacted));
}

#[test]
fn support_export_replays_same_truth_without_raw_secret_values() {
    let page = seeded_extension_inspector_page();
    let export = seeded_extension_inspector_support_export();
    validate_extension_inspector_support_export(&export, &page)
        .expect("support export must validate against page");

    assert_eq!(
        export.permission_rows.len(),
        page.permission_inspector.rows.len()
    );
    assert_eq!(
        export.setting_rows.len(),
        page.settings_inspector.rows.len()
    );
    assert!(!export.raw_secret_values_exported);
    for row in &export.setting_rows {
        assert!(!row.raw_secret_value_exported);
        assert!(!row.source_chain.is_empty());
        assert!(!row.diff_rows.is_empty());
    }
    assert_eq!(
        export.parity_fingerprint.permission_summary,
        page.permission_inspector.summary
    );
    assert_eq!(
        export.parity_fingerprint.settings_summary,
        page.settings_inspector.summary
    );
}

#[test]
fn page_fixture_matches_seeded_builder() {
    let fixture: ExtensionInspectorPage = load("page.json");
    let seeded = seeded_extension_inspector_page();
    assert_eq!(fixture, seeded);
    validate_extension_inspector_page(&fixture).expect("fixture page must validate");
}

#[test]
fn support_export_fixture_matches_seeded_builder() {
    let page = seeded_extension_inspector_page();
    let fixture: ExtensionInspectorSupportExport = load("support_export.json");
    let seeded = seeded_extension_inspector_support_export();
    assert_eq!(fixture, seeded);
    validate_extension_inspector_support_export(&fixture, &page)
        .expect("fixture support export must validate");
}
