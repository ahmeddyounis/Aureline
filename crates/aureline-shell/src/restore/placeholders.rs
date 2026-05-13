//! Placeholder cards for recent-work and restore recovery surfaces.
//!
//! These cards are the shell-side projection of the recent-work failure
//! taxonomy. They preserve target identity, trust, restore, and recovery
//! actions while replacing only the broken surface.

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    classify_recent_work_failure, normalized_recent_work_recovery_actions,
    open_minimal_recovery_action, RecentWorkEntryRecord, RecentWorkFailureState,
    RecentWorkTargetState, RecoveryCheckpointRef, RestoreAvailability, SafeRecoveryAction,
    TargetKind, TrustState,
};

/// Schema version for shell recent-work placeholder cards.
pub const RECENT_WORK_PLACEHOLDER_SCHEMA_VERSION: u32 = 1;

/// Surface that owns a placeholder card instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderSurfaceClass {
    /// Start Center recent-work or pinned-work row.
    StartCenterRecentWork,
    /// In-workspace switcher row.
    WorkspaceSwitcher,
    /// Restore prompt or restore-card surface.
    RestoreCard,
}

impl PlaceholderSurfaceClass {
    /// Returns the stable string vocabulary for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterRecentWork => "start_center_recent_work",
            Self::WorkspaceSwitcher => "workspace_switcher",
            Self::RestoreCard => "restore_card",
        }
    }
}

/// Recovery route retained when a workspace switch cannot complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceSwitchRecoveryAction {
    /// Stay on or return to the currently active workspace.
    CancelSwitch,
    /// Reopen the previously active workspace from the suspended frame.
    ReopenPreviousWorkspace,
}

impl WorkspaceSwitchRecoveryAction {
    /// Returns the stable string vocabulary for switch recovery.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CancelSwitch => "cancel_switch",
            Self::ReopenPreviousWorkspace => "reopen_previous_workspace",
        }
    }
}

/// Scope affected by a placeholder cleanup action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderCleanupScope {
    /// Only the recent-work metadata row is removed.
    RecentWorkMetadataOnly,
}

impl PlaceholderCleanupScope {
    /// Returns the stable string vocabulary for cleanup scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentWorkMetadataOnly => "recent_work_metadata_only",
        }
    }
}

/// Export-safe identity preserved on a recent-work placeholder card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWorkPlaceholderIdentity {
    /// The exact target kind from the workspace object model.
    pub target_kind: TargetKind,
    /// The raw recent-work target state that produced the placeholder.
    pub target_state: RecentWorkTargetState,
    /// Trust posture visible before activation.
    pub trust_state: TrustState,
    /// Restore availability visible before activation.
    pub restore_availability: RestoreAvailability,
    /// Filesystem identity reference for local targets, when present.
    pub filesystem_identity_ref: Option<String>,
    /// Remote target descriptor reference for remote-backed targets, when present.
    pub remote_target_descriptor_ref: Option<String>,
    /// Artifact descriptor reference for import, handoff, or package targets.
    pub artifact_descriptor_ref: Option<String>,
    /// Recovery checkpoints tied to this row.
    pub recovery_checkpoint_refs: Option<Vec<RecoveryCheckpointRef>>,
}

/// Shell placeholder shown instead of an unavailable recent-work surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWorkPlaceholderCard {
    /// Discriminator for logs and fixtures.
    pub record_kind: String,
    /// Schema revision for this shell-side projection.
    pub recent_work_placeholder_schema_version: u32,
    /// Surface that rendered the placeholder.
    pub surface_class: PlaceholderSurfaceClass,
    /// Upstream recent-work entry id.
    pub recent_work_entry_ref: String,
    /// User-facing project or workspace label.
    pub presentation_label: String,
    /// Redaction-aware path, host, provider, or target subtitle.
    pub presentation_subtitle: Option<String>,
    /// Shared failure state used by Start Center and switcher surfaces.
    pub failure_state: RecentWorkFailureState,
    /// Export-safe identity retained for support and recovery.
    pub support_identity: RecentWorkPlaceholderIdentity,
    /// Safe actions offered by this placeholder.
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    /// Action that represents the row's minimal-open route, when available.
    pub open_minimal_action: Option<SafeRecoveryAction>,
    /// Return path preserved when an in-workspace switch fails.
    pub switch_recovery_actions: Vec<WorkspaceSwitchRecoveryAction>,
    /// Scope affected if the user removes the row.
    pub cleanup_scope: PlaceholderCleanupScope,
    /// Whether cleanup and recovery leave unrelated workspace state intact.
    pub preserves_unrelated_state: bool,
    /// Compact explanation rendered in rows and support exports.
    pub recovery_summary: String,
}

/// Builds a placeholder card for unavailable recent work.
pub fn recent_work_placeholder_card(
    entry: &RecentWorkEntryRecord,
    surface_class: PlaceholderSurfaceClass,
) -> Option<RecentWorkPlaceholderCard> {
    let failure_state = classify_recent_work_failure(entry);
    if !failure_state.requires_placeholder() {
        return None;
    }

    let safe_recovery_actions = normalized_recent_work_recovery_actions(entry);
    Some(RecentWorkPlaceholderCard {
        record_kind: "recent_work_placeholder_card".to_string(),
        recent_work_placeholder_schema_version: RECENT_WORK_PLACEHOLDER_SCHEMA_VERSION,
        surface_class,
        recent_work_entry_ref: entry.recent_work_id.clone(),
        presentation_label: entry.presentation_label.clone(),
        presentation_subtitle: entry.presentation_subtitle.clone(),
        failure_state,
        support_identity: RecentWorkPlaceholderIdentity {
            target_kind: entry.target_kind,
            target_state: entry.target_state,
            trust_state: entry.trust_state,
            restore_availability: entry.restore_availability,
            filesystem_identity_ref: entry.filesystem_identity_ref.clone(),
            remote_target_descriptor_ref: entry.remote_target_descriptor_ref.clone(),
            artifact_descriptor_ref: entry.artifact_descriptor_ref.clone(),
            recovery_checkpoint_refs: entry.recovery_checkpoint_refs.clone(),
        },
        open_minimal_action: open_minimal_recovery_action(entry),
        switch_recovery_actions: switch_recovery_actions_for(surface_class),
        cleanup_scope: PlaceholderCleanupScope::RecentWorkMetadataOnly,
        preserves_unrelated_state: true,
        recovery_summary: recovery_summary(entry, failure_state),
        safe_recovery_actions,
    })
}

fn switch_recovery_actions_for(
    surface_class: PlaceholderSurfaceClass,
) -> Vec<WorkspaceSwitchRecoveryAction> {
    if surface_class == PlaceholderSurfaceClass::WorkspaceSwitcher {
        vec![
            WorkspaceSwitchRecoveryAction::CancelSwitch,
            WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace,
        ]
    } else {
        Vec::new()
    }
}

fn recovery_summary(entry: &RecentWorkEntryRecord, state: RecentWorkFailureState) -> String {
    match state {
        RecentWorkFailureState::Ready => "Target is available.".to_string(),
        RecentWorkFailureState::MissingPath => format!(
            "{} is missing at the last known {} target. Locate it, open minimal, or remove only the recent-work metadata.",
            entry.presentation_label,
            entry.target_kind.surface_label()
        ),
        RecentWorkFailureState::MovedRoot => format!(
            "{} appears to have moved. Cached identity is preserved until a new location is selected.",
            entry.presentation_label
        ),
        RecentWorkFailureState::ReconnectRequired => format!(
            "{} requires reconnect or reauthorization before write-capable restore.",
            entry.presentation_label
        ),
        RecentWorkFailureState::InspectOnly => format!(
            "{} can be inspected from cached or evidence state; writes wait for revalidation.",
            entry.presentation_label
        ),
        RecentWorkFailureState::Blocked => format!(
            "{} is blocked by policy, quarantine, or another owner. Recovery stays scoped to this recent-work row.",
            entry.presentation_label
        ),
        RecentWorkFailureState::Unknown => format!(
            "{} has unknown target state. Open restricted or remove only recent-work metadata.",
            entry.presentation_label
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{
        PortabilityClass, RecentWorkEntryRecordKind, RecentWorkTargetState, TargetKind,
    };

    fn missing_entry() -> RecentWorkEntryRecord {
        RecentWorkEntryRecord {
            record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
            entry_and_restore_schema_version: 1,
            recent_work_id: "recent:missing".to_string(),
            presentation_label: "payments".to_string(),
            presentation_subtitle: Some("Local repository".to_string()),
            target_kind: TargetKind::LocalRepoRoot,
            target_state: RecentWorkTargetState::MissingTarget,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::LayoutOnly,
            safe_recovery_actions: vec![SafeRecoveryAction::LocateMissingTarget],
            pinned: false,
            last_opened_at: "mono:test".to_string(),
            filesystem_identity_ref: Some("fs:payments".to_string()),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
            recovery_checkpoint_refs: None,
        }
    }

    #[test]
    fn missing_target_placeholder_preserves_identity_and_return_path() {
        let card = recent_work_placeholder_card(
            &missing_entry(),
            PlaceholderSurfaceClass::WorkspaceSwitcher,
        )
        .expect("placeholder");

        assert_eq!(card.failure_state, RecentWorkFailureState::MissingPath);
        assert_eq!(card.support_identity.target_kind, TargetKind::LocalRepoRoot);
        assert_eq!(
            card.open_minimal_action,
            Some(SafeRecoveryAction::OpenWithoutRestore)
        );
        assert!(card
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::LocateMissingTarget));
        assert!(card
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::RemoveFromRecents));
        assert!(card
            .switch_recovery_actions
            .contains(&WorkspaceSwitchRecoveryAction::CancelSwitch));
        assert!(card
            .switch_recovery_actions
            .contains(&WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace));
        assert!(card.preserves_unrelated_state);
    }
}
