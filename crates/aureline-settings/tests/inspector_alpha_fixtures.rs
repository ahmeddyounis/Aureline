//! Fixture replay for the schema-backed settings inspector alpha.

use aureline_settings::inspector::{
    EffectiveSettingInspectionRecord, SettingWritePreviewRecord, SettingsSupportExportProjection,
};

#[test]
fn inspector_alpha_fixtures_round_trip_through_shared_types() {
    let effective: EffectiveSettingInspectionRecord = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/inspector_alpha/policy_locked_effective_record.json"
    )))
    .unwrap();
    assert_eq!(effective.setting_id, "security.ai.egress_policy");
    assert_eq!(effective.lock_state, "policy_locked");
    assert!(effective.policy_lock_explanation.is_some());
    assert_eq!(
        effective.restart_state.restart_posture,
        "restart_extensions"
    );
    assert_eq!(effective.capability_availability, "available");
    assert_eq!(
        effective.last_applied_revision.as_deref(),
        Some("settings-rev:00042")
    );

    let preview: SettingWritePreviewRecord = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/inspector_alpha/scope_explicit_write_preview.json"
    )))
    .unwrap();
    assert_eq!(preview.target_scope, "workspace");
    assert!(preview.destination_preview.scope_explicit);
    assert_eq!(preview.destination_preview.scope_broadening_verdict, "none");
    assert!(preview.checkpoint_required);
    assert!(preview.approval_required);
    assert!(preview.change_summary.rollback_ready);

    let export: SettingsSupportExportProjection = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/settings/inspector_alpha/support_export_shared_contract.json"
    )))
    .unwrap();
    assert_eq!(export.policy_locked_count, 1);
    assert_eq!(export.effective_settings.len(), 1);
    assert_eq!(
        export.effective_settings[0].source_record_ref,
        effective.source_record_ref
    );
    assert_eq!(export.shared_contract_ref, effective.shared_contract_ref);
}
