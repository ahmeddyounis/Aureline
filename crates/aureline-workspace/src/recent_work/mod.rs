//! Recent-work registry and entry vocabulary.
//!
//! The recent-work registry is the canonical source for "resume / reopen"
//! suggestions across shell entry surfaces. It stores typed target identity and
//! availability state so a missing local folder or disconnected remote can
//! never render as an ordinary reachable local open.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Schema version for `entry_and_restore_result.schema.json`.
pub type EntryAndRestoreSchemaVersion = u32;

/// Identifies the `recent_work_entry_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentWorkEntryRecordKind {
    /// `recent_work_entry_record`
    RecentWorkEntryRecord,
}

/// Re-exported target kind from the workspace entry / restore object model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    LocalFile,
    LocalFolder,
    LocalRepoRoot,
    WorkspaceManifest,
    WorksetManifest,
    RemoteRepository,
    SshWorkspace,
    ContainerWorkspace,
    DevcontainerWorkspace,
    ManagedCloudWorkspace,
    PortableStatePackage,
    HandoffPacket,
    CompetitorConfigRoot,
    TemplateOrPrebuildSnapshot,
    ReviewOrWorkItemDeepLink,
    RecoveryCheckpoint,
}

impl TargetKind {
    /// Returns the stable string vocabulary for this target kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::LocalFolder => "local_folder",
            Self::LocalRepoRoot => "local_repo_root",
            Self::WorkspaceManifest => "workspace_manifest",
            Self::WorksetManifest => "workset_manifest",
            Self::RemoteRepository => "remote_repository",
            Self::SshWorkspace => "ssh_workspace",
            Self::ContainerWorkspace => "container_workspace",
            Self::DevcontainerWorkspace => "devcontainer_workspace",
            Self::ManagedCloudWorkspace => "managed_cloud_workspace",
            Self::PortableStatePackage => "portable_state_package",
            Self::HandoffPacket => "handoff_packet",
            Self::CompetitorConfigRoot => "competitor_config_root",
            Self::TemplateOrPrebuildSnapshot => "template_or_prebuild_snapshot",
            Self::ReviewOrWorkItemDeepLink => "review_or_work_item_deep_link",
            Self::RecoveryCheckpoint => "recovery_checkpoint",
        }
    }
}

/// State of the captured recent-work target at the time the row is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentWorkTargetState {
    Reachable,
    StaleMetadata,
    MissingTarget,
    MovedTargetDetected,
    RemoteUnreachable,
    AuthorityExpired,
    LockedByOtherInstance,
    PolicyBlocked,
    Quarantined,
    ModeDowngraded,
    Unknown,
}

impl RecentWorkTargetState {
    /// Returns the stable string vocabulary for this target state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::StaleMetadata => "stale_metadata",
            Self::MissingTarget => "missing_target",
            Self::MovedTargetDetected => "moved_target_detected",
            Self::RemoteUnreachable => "remote_unreachable",
            Self::AuthorityExpired => "authority_expired",
            Self::LockedByOtherInstance => "locked_by_other_instance",
            Self::PolicyBlocked => "policy_blocked",
            Self::Quarantined => "quarantined",
            Self::ModeDowngraded => "mode_downgraded",
            Self::Unknown => "unknown",
        }
    }
}

/// Portability posture for a recent-work entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityClass {
    LocalOnly,
    Synced,
    Imported,
    ProviderLinked,
    Stale,
}

/// Workspace trust posture a recent-work entry advertises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Restricted,
    PendingEvaluation,
}

impl TrustState {
    /// Returns the stable string vocabulary for this trust state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }
}

/// How much of a prior session can be restored for a recent-work row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreAvailability {
    Exact,
    Compatible,
    LayoutOnly,
    EvidenceOnly,
    None,
}

/// Safe recovery actions exposed for a recent-work entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeRecoveryAction {
    Open,
    OpenInNewWindow,
    OpenRestricted,
    LocateMissingTarget,
    Reconnect,
    Reauth,
    OpenReadOnlyCachedView,
    RetryLater,
    CompareBeforeRestore,
    OpenWithoutRestore,
    Unpin,
    Pin,
    RemoveFromRecents,
    RevealInExplorer,
}

impl SafeRecoveryAction {
    /// Returns the stable string vocabulary for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::OpenInNewWindow => "open_in_new_window",
            Self::OpenRestricted => "open_restricted",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::Reconnect => "reconnect",
            Self::Reauth => "reauth",
            Self::OpenReadOnlyCachedView => "open_read_only_cached_view",
            Self::RetryLater => "retry_later",
            Self::CompareBeforeRestore => "compare_before_restore",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::Unpin => "unpin",
            Self::Pin => "pin",
            Self::RemoveFromRecents => "remove_from_recents",
            Self::RevealInExplorer => "reveal_in_explorer",
        }
    }
}

/// One recent-work entry (identity + availability + safe recovery actions).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWorkEntryRecord {
    pub record_kind: RecentWorkEntryRecordKind,
    pub entry_and_restore_schema_version: EntryAndRestoreSchemaVersion,
    pub recent_work_id: String,
    pub presentation_label: String,
    pub presentation_subtitle: Option<String>,
    pub target_kind: TargetKind,
    pub target_state: RecentWorkTargetState,
    pub portability_class: PortabilityClass,
    pub trust_state: TrustState,
    pub restore_availability: RestoreAvailability,
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    pub pinned: bool,
    pub last_opened_at: String,
    pub filesystem_identity_ref: Option<String>,
    pub remote_target_descriptor_ref: Option<String>,
    pub artifact_descriptor_ref: Option<String>,
    pub recovery_checkpoint_refs: Option<Vec<RecoveryCheckpointRef>>,
}

/// Recovery checkpoint reference exposed on a recent-work entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCheckpointRef {
    pub recovery_class: String,
    pub checkpoint_ref: String,
}

/// Identifies the registry record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentWorkRegistryRecordKind {
    /// `recent_work_registry_record`
    RecentWorkRegistryRecord,
}

/// Canonical persisted container for recent-work entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWorkRegistry {
    pub record_kind: RecentWorkRegistryRecordKind,
    pub recent_work_registry_schema_version: u32,
    pub updated_at: String,
    pub entries: Vec<RecentWorkEntryRecord>,
}

impl RecentWorkRegistry {
    /// Returns the default on-disk location under the repository-local log root.
    pub fn default_store_path() -> PathBuf {
        PathBuf::from(".logs")
            .join("recent_work")
            .join("recent_work_registry.json")
    }

    /// Loads the registry from `path` when present, otherwise returns an empty registry.
    pub fn load_or_default(path: impl AsRef<Path>) -> Result<Self, RecentWorkRegistryError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self {
                record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
                recent_work_registry_schema_version: 1,
                updated_at: "mono:0000:00:00:00.0000".to_string(),
                entries: Vec::new(),
            });
        }
        let payload = std::fs::read_to_string(path).map_err(RecentWorkRegistryError::Read)?;
        serde_json::from_str(&payload).map_err(RecentWorkRegistryError::Parse)
    }

    /// Writes the registry to `path`, creating parent directories as needed.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), RecentWorkRegistryError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(RecentWorkRegistryError::CreateDir)?;
        }
        let payload = serde_json::to_string_pretty(self).map_err(RecentWorkRegistryError::Parse)?;
        std::fs::write(path, payload).map_err(RecentWorkRegistryError::Write)?;
        Ok(())
    }

    /// Inserts or updates a recent-work entry by `recent_work_id`, moving it to the front.
    pub fn upsert(&mut self, entry: RecentWorkEntryRecord) {
        self.entries
            .retain(|row| row.recent_work_id != entry.recent_work_id);
        self.entries.insert(0, entry);
    }

    /// Removes an entry from the registry by `recent_work_id`.
    pub fn remove(&mut self, recent_work_id: &str) -> bool {
        let before = self.entries.len();
        self.entries
            .retain(|row| row.recent_work_id != recent_work_id);
        before != self.entries.len()
    }
}

/// Errors returned by recent-work registry load/save operations.
#[derive(Debug)]
pub enum RecentWorkRegistryError {
    CreateDir(std::io::Error),
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(serde_json::Error),
}

impl std::fmt::Display for RecentWorkRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateDir(err) => write!(f, "create dir failed: {err}"),
            Self::Read(err) => write!(f, "read failed: {err}"),
            Self::Write(err) => write!(f, "write failed: {err}"),
            Self::Parse(err) => write!(f, "parse failed: {err}"),
        }
    }
}

impl std::error::Error for RecentWorkRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn loads_entry_restore_example_fixture() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/workspace/entry_restore_examples/recent_work_missing_target.json",
        );
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let fixture: RecentWorkEntryRecord =
            serde_json::from_str(&payload).expect("fixture must parse");
        assert_eq!(
            fixture.record_kind,
            RecentWorkEntryRecordKind::RecentWorkEntryRecord
        );
        assert_eq!(fixture.entry_and_restore_schema_version, 1);
        assert_eq!(fixture.target_kind, TargetKind::LocalRepoRoot);
        assert_eq!(fixture.target_state, RecentWorkTargetState::MissingTarget);
        assert!(!fixture.safe_recovery_actions.is_empty());
    }
}
