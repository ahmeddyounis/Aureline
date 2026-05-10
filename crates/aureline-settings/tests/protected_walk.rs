//! Integration test: end-to-end protected walk for the
//! effective-settings substrate.
//!
//! Mirrors the steps documented in
//! `docs/settings/effective_settings_contract.md` so a reviewer can
//! see the substrate exercised without launching the live shell:
//!
//! 1. inspect a setting across scopes,
//! 2. attempt a locked write (failure drill),
//! 3. verify precedence, shadow chain, and lock reason are
//!    explainable, and
//! 4. round-trip the resolver state through file-based JSON.

use aureline_settings::{
    EffectiveSettingsResolver, LockReason, LockState, PolicyConstraint, SchemaRegistry,
    ScopeOverlay, SettingScope, SettingValue, ShadowRelation, WriteDenialReason, WriteIntent,
};

#[test]
fn protected_walk_inspects_locks_and_round_trips() {
    let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());

    // Step 1: lay down user_global, then workspace overlays so the
    // shadow chain has something interesting to show.
    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
    user.set_value("editor.tab_size", SettingValue::Integer(8));
    user.set_value(
        "security.ai.egress_policy",
        SettingValue::String("any_hosted_provider".into()),
    );
    resolver.set_overlay(user).unwrap();

    let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
    workspace.set_value("editor.tab_size", SettingValue::Integer(2));
    resolver.set_overlay(workspace).unwrap();

    // Inspect editor.tab_size: workspace wins, shadow chain shows
    // built_in_default and user_global as shadowed.
    let effective_tab_size = resolver.resolve("editor.tab_size").unwrap();
    assert_eq!(effective_tab_size.value, SettingValue::Integer(2));
    assert_eq!(effective_tab_size.winning_scope, SettingScope::Workspace);
    assert_eq!(effective_tab_size.lock_state, LockState::Open);
    assert_eq!(effective_tab_size.lock_reason, LockReason::None);
    assert!(effective_tab_size
        .shadow_chain
        .iter()
        .any(|e| e.scope == SettingScope::UserGlobal && e.relation == ShadowRelation::Shadowed));
    assert!(effective_tab_size
        .shadow_chain
        .iter()
        .any(|e| e.scope == SettingScope::Workspace && e.relation == ShadowRelation::Winner));

    // Step 2 + 3: introduce a policy ceiling on
    // security.ai.egress_policy, then attempt a locked write at
    // user_global. The write must be denied with the typed
    // policy_locked reason; the shadow chain must continue to
    // surface the active policy ceiling.
    let mut policy =
        ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
    policy.set_policy_constraint(
        "security.ai.egress_policy",
        PolicyConstraint::SingleValue {
            value: SettingValue::String("approved_hosted_providers_only".into()),
        },
    );
    resolver.set_overlay(policy).unwrap();

    let outcome = resolver.attempt_write(
        "security.ai.egress_policy",
        SettingScope::UserGlobal,
        SettingValue::String("any_hosted_provider".into()),
    );
    assert_eq!(outcome.verdict, WriteIntent::Denied);
    assert!(matches!(
        outcome.denial_reason,
        Some(WriteDenialReason::PolicyLocked)
    ));
    let after = outcome.effective_after.unwrap();
    assert_eq!(after.lock_state, LockState::PolicyLocked);
    assert!(after.policy_ceiling_active);
    assert!(after
        .shadow_chain
        .iter()
        .any(|e| e.scope == SettingScope::AdminPolicyNarrowing));

    // Step 4: file-based exportability. Export, re-import in a
    // fresh resolver, and prove the inspect/locked-write story is
    // identical.
    let exported = resolver.export_state();
    let payload = serde_json::to_string(&exported).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap();

    let mut restored = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
    restored.import_state(&parsed).unwrap();

    assert_eq!(
        restored.resolve("editor.tab_size").unwrap(),
        resolver.resolve("editor.tab_size").unwrap()
    );
    assert_eq!(
        restored
            .resolve("security.ai.egress_policy")
            .unwrap()
            .lock_state,
        LockState::PolicyLocked
    );
}
