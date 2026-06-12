use super::*;

#[test]
fn seeded_summary_covers_all_rollout_rows() {
    let summary = seeded_m5_rollout_governance_render_summary();
    assert_eq!(summary.row_count, summary.rows.len());
    assert!(summary.active_kill_switch_row_count >= 1);
    assert_eq!(summary.settings_inspector_visible_row_count, summary.row_count);
    assert_eq!(summary.help_about_visible_row_count, summary.row_count);
    assert_eq!(summary.diagnostics_visible_row_count, summary.row_count);
    assert_eq!(summary.support_export_visible_row_count, summary.row_count);
}

#[test]
fn seeded_summary_preserves_projection_refs_for_help_and_settings() {
    let summary = seeded_m5_rollout_governance_render_summary();
    let row = summary
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:sync.push_workspace_state")
        .expect("sync row must exist");
    assert_eq!(row.effective_state_label, "DisabledByPolicy");
    assert_eq!(
        row.active_kill_switch_source.as_deref(),
        Some("admin_policy_ceiling")
    );
    assert!(row.settings_projection_ref.contains("settings_row"));
    assert!(row.help_about_projection_ref.contains("help_about"));
    assert!(row.diagnostics_projection_ref.contains("diagnostics"));
}
