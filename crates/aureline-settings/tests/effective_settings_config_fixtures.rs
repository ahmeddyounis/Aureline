//! Fixture replay for the canonical effective-settings record and
//! scope-explicit write preview.

use aureline_settings::EffectiveSettingRecord;

#[test]
fn config_effective_settings_fixtures_round_trip() {
    let effective: EffectiveSettingRecord = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/effective_settings_shadow_chain/policy_locked_effective_record.json"
    )))
    .expect("effective record fixture parses");

    assert_eq!(effective.setting_id, "security.ai.egress_policy");
    assert_eq!(effective.lock_state, "policy_locked");
    assert_eq!(effective.resolved_scope, "admin_policy_narrowing");
    assert!(effective.shadow_chain.iter().any(|row| {
        row.scope.as_str() == "user_global" && row.relation.as_str() == "capped" && !row.winner
    }));
    assert!(effective.is_policy_locked());

    let preview: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/effective_settings_shadow_chain/scope_explicit_write_preview.json"
    )))
    .expect("write preview fixture parses");
    assert_eq!(preview["target_scope"], "workspace");
    assert_eq!(
        preview["destination_preview"]["target_artifact_ref"],
        "settings://scope/workspace/security.ai.egress_policy"
    );
    assert_eq!(
        preview["destination_preview"]["scope_broadening_verdict"],
        "none"
    );
    assert_eq!(preview["checkpoint_required"], true);
    assert_eq!(preview["approval_required"], true);
}
