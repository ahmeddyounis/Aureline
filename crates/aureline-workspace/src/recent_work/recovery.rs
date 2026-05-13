//! Recovery taxonomy for recent-work entries.
//!
//! This module owns the shared failure-state vocabulary used by Start Center,
//! workspace switcher, restore placeholders, and support exports when a recent
//! workspace target cannot be opened as an ordinary live target.

use serde::{Deserialize, Serialize};

use super::{RecentWorkEntryRecord, RecentWorkTargetState, SafeRecoveryAction, TargetKind};

/// Coarse unavailable-target state shown before a recent-work entry is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentWorkFailureState {
    /// The target can be opened normally.
    Ready,
    /// The captured local path or mount is no longer reachable.
    MissingPath,
    /// The root appears to have moved away from the stored identity.
    MovedRoot,
    /// A remote, container, or managed target requires reconnect or reauth.
    ReconnectRequired,
    /// Only cached metadata, evidence, or compare-only inspection is safe.
    InspectOnly,
    /// Policy, quarantine, or an external lock blocks normal activation.
    Blocked,
    /// The row cannot prove a stronger state.
    Unknown,
}

impl RecentWorkFailureState {
    /// Returns the stable string vocabulary used in fixtures and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::MissingPath => "missing_path",
            Self::MovedRoot => "moved_root",
            Self::ReconnectRequired => "reconnect_required",
            Self::InspectOnly => "inspect_only",
            Self::Blocked => "blocked",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true when the entry needs a placeholder before ordinary open.
    pub const fn requires_placeholder(self) -> bool {
        !matches!(self, Self::Ready)
    }
}

/// Returns the shared failure-state classification for `entry`.
pub fn classify_recent_work_failure(entry: &RecentWorkEntryRecord) -> RecentWorkFailureState {
    match entry.target_state {
        RecentWorkTargetState::Reachable => {
            if is_inspect_only_entry(entry) {
                RecentWorkFailureState::InspectOnly
            } else {
                RecentWorkFailureState::Ready
            }
        }
        RecentWorkTargetState::MissingTarget => RecentWorkFailureState::MissingPath,
        RecentWorkTargetState::MovedTargetDetected => RecentWorkFailureState::MovedRoot,
        RecentWorkTargetState::RemoteUnreachable | RecentWorkTargetState::AuthorityExpired => {
            RecentWorkFailureState::ReconnectRequired
        }
        RecentWorkTargetState::StaleMetadata | RecentWorkTargetState::ModeDowngraded => {
            RecentWorkFailureState::InspectOnly
        }
        RecentWorkTargetState::LockedByOtherInstance
        | RecentWorkTargetState::PolicyBlocked
        | RecentWorkTargetState::Quarantined => RecentWorkFailureState::Blocked,
        RecentWorkTargetState::Unknown => RecentWorkFailureState::Unknown,
    }
}

/// Returns a normalized action list for placeholder and switcher recovery.
///
/// The returned list preserves the entry's authored actions and appends the
/// minimum non-destructive recovery actions required for the classified state.
pub fn normalized_recent_work_recovery_actions(
    entry: &RecentWorkEntryRecord,
) -> Vec<SafeRecoveryAction> {
    let mut actions = entry.safe_recovery_actions.clone();
    match classify_recent_work_failure(entry) {
        RecentWorkFailureState::Ready => {
            ensure_action(&mut actions, SafeRecoveryAction::Open);
        }
        RecentWorkFailureState::MissingPath => {
            ensure_action(&mut actions, SafeRecoveryAction::LocateMissingTarget);
            ensure_action(&mut actions, SafeRecoveryAction::OpenWithoutRestore);
            ensure_action(&mut actions, SafeRecoveryAction::RemoveFromRecents);
        }
        RecentWorkFailureState::MovedRoot => {
            ensure_action(&mut actions, SafeRecoveryAction::LocateMissingTarget);
            ensure_action(&mut actions, SafeRecoveryAction::OpenWithoutRestore);
            ensure_action(&mut actions, SafeRecoveryAction::RemoveFromRecents);
        }
        RecentWorkFailureState::ReconnectRequired => {
            if entry.target_state == RecentWorkTargetState::AuthorityExpired {
                ensure_action(&mut actions, SafeRecoveryAction::Reauth);
            } else {
                ensure_action(&mut actions, SafeRecoveryAction::Reconnect);
            }
            ensure_action(&mut actions, SafeRecoveryAction::RetryLater);
            ensure_action(&mut actions, SafeRecoveryAction::OpenWithoutRestore);
            ensure_action(&mut actions, SafeRecoveryAction::RemoveFromRecents);
        }
        RecentWorkFailureState::InspectOnly => {
            ensure_action(&mut actions, SafeRecoveryAction::OpenReadOnlyCachedView);
            ensure_action(&mut actions, SafeRecoveryAction::OpenWithoutRestore);
            ensure_action(&mut actions, SafeRecoveryAction::RemoveFromRecents);
        }
        RecentWorkFailureState::Blocked | RecentWorkFailureState::Unknown => {
            ensure_action(&mut actions, SafeRecoveryAction::OpenRestricted);
            ensure_action(&mut actions, SafeRecoveryAction::RetryLater);
            ensure_action(&mut actions, SafeRecoveryAction::RemoveFromRecents);
        }
    }
    normalize_pin_action(entry, &mut actions);
    actions
}

/// Mutates an entry so its recovery action list matches the shared taxonomy.
pub fn normalize_recent_work_entry_recovery_actions(entry: &mut RecentWorkEntryRecord) {
    entry.safe_recovery_actions = normalized_recent_work_recovery_actions(entry);
}

/// Returns the action that represents "open minimal" for this row, when present.
pub fn open_minimal_recovery_action(entry: &RecentWorkEntryRecord) -> Option<SafeRecoveryAction> {
    let actions = normalized_recent_work_recovery_actions(entry);
    [
        SafeRecoveryAction::OpenWithoutRestore,
        SafeRecoveryAction::OpenRestricted,
        SafeRecoveryAction::OpenReadOnlyCachedView,
    ]
    .into_iter()
    .find(|candidate| actions.contains(candidate))
}

/// Returns true when the action is scoped to recent-work metadata only.
pub const fn removes_recent_work_metadata_only(action: SafeRecoveryAction) -> bool {
    matches!(action, SafeRecoveryAction::RemoveFromRecents)
}

/// Returns true when a target kind depends on a remote or managed target.
pub const fn is_remote_backed_target(target_kind: TargetKind) -> bool {
    matches!(
        target_kind,
        TargetKind::RemoteRepository
            | TargetKind::SshWorkspace
            | TargetKind::ContainerWorkspace
            | TargetKind::DevcontainerWorkspace
            | TargetKind::ManagedCloudWorkspace
    )
}

fn is_inspect_only_entry(entry: &RecentWorkEntryRecord) -> bool {
    let has_live_open = entry.safe_recovery_actions.iter().any(|action| {
        matches!(
            action,
            SafeRecoveryAction::Open
                | SafeRecoveryAction::OpenInNewWindow
                | SafeRecoveryAction::OpenRestricted
        )
    });
    let has_inspect_action = entry.safe_recovery_actions.iter().any(|action| {
        matches!(
            action,
            SafeRecoveryAction::OpenReadOnlyCachedView | SafeRecoveryAction::CompareBeforeRestore
        )
    });
    has_inspect_action && !has_live_open
}

fn ensure_action(actions: &mut Vec<SafeRecoveryAction>, action: SafeRecoveryAction) {
    if !actions.contains(&action) {
        actions.push(action);
    }
}

fn normalize_pin_action(entry: &RecentWorkEntryRecord, actions: &mut Vec<SafeRecoveryAction>) {
    if entry.pinned {
        actions.retain(|action| *action != SafeRecoveryAction::Pin);
        ensure_action(actions, SafeRecoveryAction::Unpin);
    } else {
        actions.retain(|action| *action != SafeRecoveryAction::Unpin);
        ensure_action(actions, SafeRecoveryAction::Pin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PortabilityClass, RecentWorkEntryRecordKind, RestoreAvailability, TrustState};

    fn entry(
        target_kind: TargetKind,
        target_state: RecentWorkTargetState,
        actions: Vec<SafeRecoveryAction>,
    ) -> RecentWorkEntryRecord {
        RecentWorkEntryRecord {
            record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
            entry_and_restore_schema_version: 1,
            recent_work_id: "recent:test".to_string(),
            presentation_label: "workspace".to_string(),
            presentation_subtitle: Some("Local folder".to_string()),
            target_kind,
            target_state,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::LayoutOnly,
            safe_recovery_actions: actions,
            pinned: false,
            last_opened_at: "mono:test".to_string(),
            filesystem_identity_ref: Some("fs:test".to_string()),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
            recovery_checkpoint_refs: None,
        }
    }

    #[test]
    fn classifies_required_placeholder_failure_states() {
        let missing = entry(
            TargetKind::LocalRepoRoot,
            RecentWorkTargetState::MissingTarget,
            vec![SafeRecoveryAction::LocateMissingTarget],
        );
        assert_eq!(
            classify_recent_work_failure(&missing),
            RecentWorkFailureState::MissingPath
        );

        let moved = entry(
            TargetKind::LocalRepoRoot,
            RecentWorkTargetState::MovedTargetDetected,
            vec![SafeRecoveryAction::LocateMissingTarget],
        );
        assert_eq!(
            classify_recent_work_failure(&moved),
            RecentWorkFailureState::MovedRoot
        );

        let remote = entry(
            TargetKind::SshWorkspace,
            RecentWorkTargetState::RemoteUnreachable,
            vec![SafeRecoveryAction::Reconnect],
        );
        assert_eq!(
            classify_recent_work_failure(&remote),
            RecentWorkFailureState::ReconnectRequired
        );

        let inspect = entry(
            TargetKind::PortableStatePackage,
            RecentWorkTargetState::StaleMetadata,
            vec![SafeRecoveryAction::OpenReadOnlyCachedView],
        );
        assert_eq!(
            classify_recent_work_failure(&inspect),
            RecentWorkFailureState::InspectOnly
        );
    }

    #[test]
    fn normalization_adds_non_destructive_recovery_paths() {
        let missing = entry(
            TargetKind::LocalRepoRoot,
            RecentWorkTargetState::MissingTarget,
            vec![SafeRecoveryAction::LocateMissingTarget],
        );
        let actions = normalized_recent_work_recovery_actions(&missing);
        assert!(actions.contains(&SafeRecoveryAction::LocateMissingTarget));
        assert!(actions.contains(&SafeRecoveryAction::OpenWithoutRestore));
        assert!(actions.contains(&SafeRecoveryAction::RemoveFromRecents));
        assert_eq!(
            open_minimal_recovery_action(&missing),
            Some(SafeRecoveryAction::OpenWithoutRestore)
        );
        assert!(removes_recent_work_metadata_only(
            SafeRecoveryAction::RemoveFromRecents
        ));
    }
}
