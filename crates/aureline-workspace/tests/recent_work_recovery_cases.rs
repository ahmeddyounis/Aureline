//! Protected recent-work recovery fixture checks.

use std::path::Path;

use aureline_workspace::{
    classify_recent_work_failure, normalized_recent_work_recovery_actions,
    open_minimal_recovery_action, removes_recent_work_metadata_only, RecentWorkEntryRecord,
    RecentWorkFailureState, SafeRecoveryAction,
};

#[test]
fn missing_root_cases_project_required_failure_states_and_actions() {
    let cases = [
        (
            "missing_local_root.json",
            RecentWorkFailureState::MissingPath,
            vec![
                SafeRecoveryAction::LocateMissingTarget,
                SafeRecoveryAction::OpenWithoutRestore,
                SafeRecoveryAction::RemoveFromRecents,
            ],
        ),
        (
            "moved_local_root.json",
            RecentWorkFailureState::MovedRoot,
            vec![
                SafeRecoveryAction::LocateMissingTarget,
                SafeRecoveryAction::OpenReadOnlyCachedView,
                SafeRecoveryAction::OpenWithoutRestore,
                SafeRecoveryAction::RemoveFromRecents,
            ],
        ),
        (
            "reconnect_ssh_workspace.json",
            RecentWorkFailureState::ReconnectRequired,
            vec![
                SafeRecoveryAction::Reconnect,
                SafeRecoveryAction::OpenWithoutRestore,
                SafeRecoveryAction::RemoveFromRecents,
            ],
        ),
        (
            "inspect_only_handoff.json",
            RecentWorkFailureState::InspectOnly,
            vec![
                SafeRecoveryAction::OpenReadOnlyCachedView,
                SafeRecoveryAction::CompareBeforeRestore,
                SafeRecoveryAction::OpenWithoutRestore,
                SafeRecoveryAction::RemoveFromRecents,
            ],
        ),
    ];

    let cases_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/missing_root_cases");

    for (file, expected_state, required_actions) in cases {
        let payload = std::fs::read_to_string(cases_dir.join(file))
            .unwrap_or_else(|err| panic!("{file}: {err}"));
        let entry: RecentWorkEntryRecord =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("{file}: {err}"));

        assert_eq!(
            classify_recent_work_failure(&entry),
            expected_state,
            "{file}: wrong failure state"
        );
        let actions = normalized_recent_work_recovery_actions(&entry);
        for action in required_actions {
            assert!(
                actions.contains(&action),
                "{file}: missing action {}",
                action.as_str()
            );
        }
        assert!(
            open_minimal_recovery_action(&entry).is_some(),
            "{file}: missing open minimal route"
        );
        assert!(removes_recent_work_metadata_only(
            SafeRecoveryAction::RemoveFromRecents
        ));
    }
}
