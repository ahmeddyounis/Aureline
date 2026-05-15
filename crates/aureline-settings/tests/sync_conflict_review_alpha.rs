//! Settings-sync conflict-review alpha coverage.

use aureline_settings::inspector::conflict::{
    inspect_sync_conflict, project_sync_conflict_review_surface, SyncConflictDevice,
    SyncConflictResolutionPath, SyncConflictReviewRequest,
};
use aureline_settings::{
    AliasDirection, EffectiveSettingsResolver, LifecycleLabel, PolicyConstraint, PreviewClass,
    RedactionClass, SchemaRegistry, ScopeOverlay, SensitivityClass, SettingAlias,
    SettingDefinition, SettingScope, SettingValue, SettingValueType,
};

fn device(id: &str) -> SyncConflictDevice {
    SyncConflictDevice::new(id)
}

fn seeded_resolver() -> EffectiveSettingsResolver {
    EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog())
}

#[test]
fn identical_settings_do_not_create_conflict_packet() {
    let mut resolver = seeded_resolver();
    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
    user.set_value("editor.format_on_save", SettingValue::Boolean(true));
    resolver.set_overlay(user).unwrap();

    let packet = inspect_sync_conflict(
        &resolver,
        SyncConflictReviewRequest {
            setting_id: "editor.format_on_save".to_owned(),
            current_device: device("dev-current-0001"),
            conflicting_device: device("dev-remote-0002"),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Boolean(true),
        },
    )
    .unwrap();

    assert!(packet.is_none());
}

#[test]
fn conflicting_scope_recommends_one_side_accept() {
    let resolver = seeded_resolver();

    let packet = inspect_sync_conflict(
        &resolver,
        SyncConflictReviewRequest {
            setting_id: "editor.format_on_save".to_owned(),
            current_device: device("dev-current-0001"),
            conflicting_device: device("dev-remote-0002"),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Boolean(true),
        },
    )
    .unwrap()
    .expect("different synced value should create a packet");

    assert_eq!(packet.current_device.device_id, "dev-current-0001");
    assert_eq!(packet.conflicting_device.device_id, "dev-remote-0002");
    assert_eq!(packet.conflicting_scope, "user_global");
    assert_eq!(packet.lock_state.as_str(), "inherited");
    assert_eq!(
        packet.recommended_resolution_path,
        SyncConflictResolutionPath::KeepSynced
    );
    assert!(packet.can_auto_resolve());
    assert!(packet
        .offered_resolution_paths
        .contains(&SyncConflictResolutionPath::KeepSynced));
}

#[test]
fn policy_locked_conflict_forces_merge_review_and_cannot_auto_resolve() {
    let mut resolver = seeded_resolver();
    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
    user.set_value(
        "security.ai.egress_policy",
        SettingValue::String("any_hosted_provider".to_owned()),
    );
    resolver.set_overlay(user).unwrap();

    let mut policy =
        ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
    policy.set_policy_constraint(
        "security.ai.egress_policy",
        PolicyConstraint::SingleValue {
            value: SettingValue::String("approved_hosted_providers_only".to_owned()),
        },
    );
    resolver.set_overlay(policy).unwrap();

    let packet = inspect_sync_conflict(
        &resolver,
        SyncConflictReviewRequest {
            setting_id: "security.ai.egress_policy".to_owned(),
            current_device: device("dev-current-0001"),
            conflicting_device: device("dev-remote-0002"),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("any_hosted_provider".to_owned()),
        },
    )
    .unwrap()
    .expect("policy ceiling should force review");

    assert_eq!(packet.lock_state.as_str(), "policy_locked");
    assert_eq!(
        packet.recommended_resolution_path,
        SyncConflictResolutionPath::MergePreview
    );
    assert!(!packet.can_auto_resolve());
    assert!(!packet
        .offered_resolution_paths
        .contains(&SyncConflictResolutionPath::KeepSynced));
    assert!(packet.policy_lock_ref.is_some());

    let surface = project_sync_conflict_review_surface(&packet);
    assert_eq!(surface.lock_state, "policy_locked");
    assert_eq!(surface.recommended_resolution_path, "merge_preview");
    assert!(!surface.auto_resolvable);
    let keep_synced = surface
        .action_rows
        .iter()
        .find(|row| row.resolution_path == SyncConflictResolutionPath::KeepSynced)
        .unwrap();
    assert!(!keep_synced.available);
    assert_eq!(
        keep_synced.disabled_reason.as_deref(),
        Some("policy_locked")
    );
}

#[test]
fn credential_conflicts_surface_handles_without_raw_values() {
    let mut registry = SchemaRegistry::new();
    registry
        .register(SettingDefinition {
            setting_id: "auth.provider_token".to_owned(),
            value_type: SettingValueType::String,
            default_value: SettingValue::String("cred:default-provider-token".to_owned()),
            allowed_scopes: vec![SettingScope::BuiltInDefault, SettingScope::UserGlobal],
            restart_posture: aureline_settings::RestartPosture::NoRestart,
            lifecycle_label: LifecycleLabel::Stable,
            preview_class: PreviewClass::SafeApply,
            redaction_class: RedactionClass::RedactToClassLabel,
            sensitivity_class: SensitivityClass::CredentialReference,
            alias_set: vec![SettingAlias {
                from_id: "auth.legacy_provider_token".to_owned(),
                since_version: "0.0.0-alpha".to_owned(),
                deprecated_in_version: None,
                removal_target_version: None,
                alias_direction: AliasDirection::RedirectToCanonical,
            }],
            migration_table: Vec::new(),
            capability_dependencies: Vec::new(),
            help_doc_ref: None,
            evidence_refs: Vec::new(),
            decision_row_ref: None,
            since_version: None,
            description: None,
            change_guidance: None,
            is_machine_specific: false,
            is_synced_by_default: true,
            is_policy_narrowable: false,
            summary: "Provider token broker alias.".to_owned(),
        })
        .unwrap();
    let mut resolver = EffectiveSettingsResolver::new(registry);
    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
    user.set_value(
        "auth.provider_token",
        SettingValue::String("cred:local-provider-token".to_owned()),
    );
    resolver.set_overlay(user).unwrap();

    let packet = inspect_sync_conflict(
        &resolver,
        SyncConflictReviewRequest {
            setting_id: "auth.legacy_provider_token".to_owned(),
            current_device: device("dev-current-0001"),
            conflicting_device: device("dev-remote-0002"),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("raw-secret-token-material".to_owned()),
        },
    )
    .unwrap()
    .expect("credential handle drift should create a packet");
    let json = serde_json::to_string(&packet).unwrap();

    assert_eq!(packet.setting_id, "auth.provider_token");
    assert_eq!(
        packet.current_value.value_preview_kind,
        "credential_alias_only"
    );
    assert_eq!(
        packet.conflicting_value.credential_alias_ref.as_deref(),
        Some("credential_alias_ref_redacted")
    );
    assert!(!json.contains("raw-secret-token-material"));
}
