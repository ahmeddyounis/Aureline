//! Recent-work placeholder projections shared by Start Center and switcher.

use std::path::Path;

use aureline_shell::restore::placeholders::WorkspaceSwitchRecoveryAction;
use aureline_shell::start_center::{build_recent_work_rows, StartCenterRecentWorkPrivacyMode};
use aureline_shell::workspace_switcher::{build_switcher_rows, WorkspaceSwitcherEntryClass};
use aureline_workspace::{
    RecentWorkEntryRecord, RecentWorkFailureState, RecentWorkRegistry,
    RecentWorkRegistryRecordKind, SafeRecoveryAction, TargetKind,
};

fn fixture_registry() -> RecentWorkRegistry {
    let cases_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/missing_root_cases");
    let mut entries = Vec::new();
    for file in [
        "missing_local_root.json",
        "moved_local_root.json",
        "reconnect_ssh_workspace.json",
        "inspect_only_handoff.json",
    ] {
        let payload = std::fs::read_to_string(cases_dir.join(file))
            .unwrap_or_else(|err| panic!("{file}: {err}"));
        let entry: RecentWorkEntryRecord =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("{file}: {err}"));
        entries.push(entry);
    }
    RecentWorkRegistry {
        record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
        recent_work_registry_schema_version: 1,
        updated_at: "mono:test".to_string(),
        entries,
    }
}

#[test]
fn switcher_rows_render_placeholders_with_return_paths() {
    let registry = fixture_registry();
    let rows = build_switcher_rows(&registry, "");
    assert_eq!(rows.len(), 4);

    let missing = rows
        .iter()
        .find(|row| row.failure_state == RecentWorkFailureState::MissingPath)
        .expect("missing path row");
    assert_eq!(missing.target_kind, TargetKind::LocalRepoRoot);
    assert_eq!(missing.target_kind_label, "Repository");
    assert!(missing.placeholder_card.is_some());
    assert!(missing
        .safe_recovery_actions
        .contains(&SafeRecoveryAction::LocateMissingTarget));
    assert!(missing
        .safe_recovery_actions
        .contains(&SafeRecoveryAction::OpenWithoutRestore));
    assert!(missing
        .safe_recovery_actions
        .contains(&SafeRecoveryAction::RemoveFromRecents));
    assert!(missing
        .switch_failure_actions
        .contains(&WorkspaceSwitchRecoveryAction::CancelSwitch));
    assert!(missing
        .switch_failure_actions
        .contains(&WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace));

    let remote = rows
        .iter()
        .find(|row| row.failure_state == RecentWorkFailureState::ReconnectRequired)
        .expect("reconnect row");
    assert_eq!(remote.target_kind_label, "SSH");
    assert!(remote
        .entry_classes
        .contains(&WorkspaceSwitcherEntryClass::Remote));
    assert!(remote
        .entry_classes
        .contains(&WorkspaceSwitcherEntryClass::Pinned));
    assert!(remote
        .entry_classes
        .contains(&WorkspaceSwitcherEntryClass::Recent));
    assert!(remote
        .safe_recovery_actions
        .contains(&SafeRecoveryAction::Reconnect));

    let inspect = rows
        .iter()
        .find(|row| row.failure_state == RecentWorkFailureState::InspectOnly)
        .expect("inspect-only row");
    assert!(inspect
        .safe_recovery_actions
        .contains(&SafeRecoveryAction::OpenReadOnlyCachedView));
}

#[test]
fn start_center_reuses_failure_taxonomy_and_privacy_keeps_entry_paths() {
    let registry = fixture_registry();
    let start_rows = build_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::Default);
    let switcher_rows = build_switcher_rows(&registry, "");

    assert_eq!(start_rows.rows.len(), switcher_rows.len());
    for start_row in &start_rows.rows {
        let switcher_row = switcher_rows
            .iter()
            .find(|row| row.recent_work_id == start_row.recent_work_id)
            .expect("matching switcher row");
        assert_eq!(start_row.failure_state, switcher_row.failure_state);
        assert_eq!(start_row.target_kind, switcher_row.target_kind);
        assert_eq!(start_row.trust_state, switcher_row.trust_state);
        assert_eq!(
            start_row.restore_availability,
            switcher_row.restore_availability
        );
        assert!(start_row.placeholder_card.is_some());
    }

    let hidden =
        build_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::HideRecentWork);
    assert!(hidden.rows.is_empty());
    assert!(hidden.metadata_hidden);
    assert!(hidden.local_open_still_available);
    assert!(hidden.workspace_open_still_available);
    assert!(hidden.restore_local_state_still_available);
    assert!(hidden.clear_recent_work_available);
}
