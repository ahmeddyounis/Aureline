//! Fixture replay for the beta settings repair, reset, import, and
//! migration review projection.
//!
//! The fixtures live under
//! `fixtures/config/m3/settings_repair_and_reset/` and are generated
//! by the `aureline_settings_inspect` CLI bin so the checked-in JSON
//! stays a literal projection of the resolver and the write-intent
//! pipeline (no hand-written drift).

use aureline_settings::repair_review::{
    SettingsRepairPlan, SettingsRepairReviewSheet, SettingsRepairSupportExport,
    SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF,
};

const REPAIR_FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/m3/settings_repair_and_reset"
);

fn load_plan(name: &str) -> SettingsRepairPlan {
    let path = format!("{REPAIR_FIXTURE_DIR}/{name}");
    let text = std::fs::read_to_string(&path).expect("fixture file present");
    serde_json::from_str(&text).expect("fixture parses as SettingsRepairPlan")
}

#[test]
fn reset_current_value_fixture_is_scope_explicit_and_ready() {
    let plan = load_plan("plan_reset_current_value.json");
    assert_eq!(plan.shared_contract_ref, SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF);
    assert_eq!(plan.action_class, "reset_current_value");
    assert_eq!(plan.target_scope, "workspace");
    assert_eq!(plan.target_scope_class, "workspace");
    assert_eq!(plan.verdict, "ready_to_apply");
    assert!(plan.write_intents.iter().all(|row| row.selected_by_user));
    assert!(plan.blocked_write_reasons.is_empty());
    assert!(!plan.hidden_reset_guard.would_broaden_scope);
    assert!(!plan.hidden_reset_guard.would_touch_adjacent_settings);
    assert!(plan.rollback_action_ref.is_some());
}

#[test]
fn reset_section_fixture_requires_checkpoint_before_apply() {
    let plan = load_plan("plan_reset_section_awaiting_checkpoint.json");
    assert!(plan.checkpoint_required);
    assert_eq!(plan.verdict, "awaiting_checkpoint");
    assert!(plan
        .blocked_write_reasons
        .iter()
        .any(|reason| reason.code_token() == "checkpoint_missing"));

    let plan_ok = load_plan("plan_reset_section_checkpointed.json");
    assert!(plan_ok.checkpoint_required);
    assert!(plan_ok.checkpoint_ref.is_some());
    assert_eq!(plan_ok.verdict, "ready_to_apply");
    assert!(plan_ok.rollback_action_ref.is_some());
}

#[test]
fn repair_drift_fixture_is_scope_explicit_single_value() {
    let plan = load_plan("plan_repair_drift.json");
    assert_eq!(plan.action_class, "repair_drift");
    assert_eq!(plan.target_scope, "user_global");
    assert_eq!(plan.verdict, "ready_to_apply");
    assert_eq!(plan.write_intents.len(), 1);
}

#[test]
fn reapply_imported_profile_fragment_fixture_preserves_checkpoint() {
    let plan = load_plan("plan_reapply_imported_profile_fragment.json");
    assert_eq!(plan.action_class, "reapply_imported_profile_fragment");
    assert_eq!(plan.target_scope_class, "profile");
    assert!(plan.checkpoint_required);
    let fragment = plan
        .imported_profile_fragment
        .as_ref()
        .expect("fragment ref present");
    assert!(fragment.profile_id.starts_with("profile:"));
    assert!(fragment.fragment_id.starts_with("fragment:"));
    assert_eq!(plan.verdict, "ready_to_apply");
}

#[test]
fn revert_migration_step_fixture_carries_migration_ref_and_checkpoint() {
    let plan = load_plan("plan_revert_migration_step.json");
    assert_eq!(plan.action_class, "revert_migration_step");
    assert!(plan.checkpoint_required);
    let step = plan.migration_step.as_ref().expect("migration step present");
    assert_eq!(step.transform_class, "narrow_enum");
    assert!(step.rollback_supported);
    assert_eq!(plan.verdict, "ready_to_apply");
    assert!(plan.rollback_action_ref.is_some());
}

#[test]
fn adjacent_refused_fixture_blocks_hidden_resets() {
    let plan = load_plan("plan_adjacent_refused.json");
    assert!(plan.hidden_reset_guard.would_touch_adjacent_settings);
    assert!(plan
        .hidden_reset_guard
        .refused_setting_ids
        .iter()
        .any(|id| id == "editor.format_on_save"));
    assert!(plan
        .blocked_write_reasons
        .iter()
        .any(|reason| reason.code_token() == "adjacent_setting_refused"));
    assert_eq!(plan.verdict, "denied");
    assert!(plan.rollback_action_ref.is_none());
}

#[test]
fn policy_owned_target_scope_fixture_refuses_repair() {
    let plan = load_plan("plan_policy_owned_refused.json");
    assert_eq!(plan.target_scope_class, "policy_owned");
    assert!(plan.hidden_reset_guard.would_broaden_scope);
    assert!(plan
        .locked_classes
        .iter()
        .any(|class| class == "policy_owned_class"));
    assert_eq!(plan.verdict, "denied");
}

#[test]
fn review_sheet_fixture_wraps_plan_truth() {
    let path = format!("{REPAIR_FIXTURE_DIR}/review_sheet_reset_current_value.json");
    let text = std::fs::read_to_string(&path).expect("review sheet fixture present");
    let sheet: SettingsRepairReviewSheet =
        serde_json::from_str(&text).expect("fixture parses as review sheet");
    assert_eq!(
        sheet.shared_contract_ref,
        SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF
    );
    assert!(sheet.source_plan_ref.contains(&sheet.plan.plan_id));
    assert!(sheet.headline.to_lowercase().contains("reset"));
    assert_eq!(sheet.plan.action_class, "reset_current_value");
    assert_eq!(sheet.plan.verdict, "ready_to_apply");
}

#[test]
fn support_export_fixture_records_decisions_and_denials() {
    let path = format!("{REPAIR_FIXTURE_DIR}/support_export.json");
    let text = std::fs::read_to_string(&path).expect("support export fixture present");
    let export: SettingsRepairSupportExport =
        serde_json::from_str(&text).expect("fixture parses as support export");
    assert_eq!(
        export.shared_contract_ref,
        SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF
    );
    assert!(!export.plans.is_empty());
    assert!(export.accepted_plan_count >= 1);
    assert!(export.declined_plan_count >= 1);
    assert!(export.denied_plan_count >= 1);

    let action_classes: Vec<&str> = export
        .plans
        .iter()
        .map(|plan| plan.action_class.as_str())
        .collect();
    for expected in [
        "reset_current_value",
        "reset_section",
        "reapply_imported_profile_fragment",
        "revert_migration_step",
    ] {
        assert!(
            action_classes.iter().any(|class| *class == expected),
            "support export should include {expected}"
        );
    }
}
