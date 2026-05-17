//! Recent-work registry and entry vocabulary.
//!
//! The recent-work registry is the canonical source for "resume / reopen"
//! suggestions across shell entry surfaces. It stores typed target identity and
//! availability state so a missing local folder or disconnected remote can
//! never render as an ordinary reachable local open.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod recovery;

pub use recovery::{
    classify_recent_work_failure, is_remote_backed_target,
    normalize_recent_work_entry_recovery_actions, normalized_recent_work_recovery_actions,
    open_minimal_recovery_action, removes_recent_work_metadata_only, RecentWorkFailureState,
};

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

    /// Returns the compact label used by Start Center and switcher rows.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::LocalFile => "File",
            Self::LocalFolder => "Folder",
            Self::LocalRepoRoot => "Repository",
            Self::WorkspaceManifest => "Workspace",
            Self::WorksetManifest => "Workset",
            Self::RemoteRepository => "Remote repository",
            Self::SshWorkspace => "SSH",
            Self::ContainerWorkspace => "Container",
            Self::DevcontainerWorkspace => "Dev container",
            Self::ManagedCloudWorkspace => "Cloud workspace",
            Self::PortableStatePackage => "Portable state",
            Self::HandoffPacket => "Handoff packet",
            Self::CompetitorConfigRoot => "Imported config",
            Self::TemplateOrPrebuildSnapshot => "Template",
            Self::ReviewOrWorkItemDeepLink => "Deep link",
            Self::RecoveryCheckpoint => "Recovery checkpoint",
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

impl RestoreAvailability {
    /// Returns the stable string vocabulary for this restore availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
            Self::None => "none",
        }
    }
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

    /// Returns the compact label shared by Start Center and switcher rows.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::OpenInNewWindow => "Open in new window",
            Self::OpenRestricted => "Open restricted",
            Self::LocateMissingTarget => "Locate",
            Self::Reconnect => "Reconnect",
            Self::Reauth => "Reauthorize",
            Self::OpenReadOnlyCachedView => "Open read-only cached view",
            Self::RetryLater => "Retry later",
            Self::CompareBeforeRestore => "Compare before restore",
            Self::OpenWithoutRestore => "Open anyway",
            Self::Unpin => "Unpin",
            Self::Pin => "Pin",
            Self::RemoveFromRecents => "Remove from list",
            Self::RevealInExplorer => "Reveal",
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

/// Section a recent-work entry belongs to after pinned/recent partitioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentWorkListSection {
    /// User-pinned entries, still ordered by the registry's recent order.
    Pinned,
    /// Non-pinned entries ordered by the registry's recent order.
    Recent,
}

impl RecentWorkListSection {
    /// Returns the stable string vocabulary for this section.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Recent => "recent",
        }
    }
}

/// One searchable row shared by Start Center, `Open Recent`, and switchers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWorkListRow {
    /// Section used to render a visible pinned versus recent distinction.
    pub section: RecentWorkListSection,
    /// Stable upstream recent-work entry id.
    pub recent_work_id: String,
    /// Project, workspace, or target label.
    pub presentation_label: String,
    /// Redaction-aware path, host, provider, or target subtitle.
    pub presentation_subtitle: Option<String>,
    /// Canonical target kind from the workspace entry model.
    pub target_kind: TargetKind,
    /// Compact target-kind label used by shell rows.
    pub target_kind_label: String,
    /// Raw target state captured for this entry.
    pub target_state: RecentWorkTargetState,
    /// Portability posture for this entry.
    pub portability_class: PortabilityClass,
    /// Shared unavailable-target classification.
    pub failure_state: RecentWorkFailureState,
    /// Workspace trust posture shown before activation.
    pub trust_state: TrustState,
    /// Restore availability shown before activation.
    pub restore_availability: RestoreAvailability,
    /// Stable last-opened timestamp from the registry.
    pub last_opened_at: String,
    /// Whether this row is pinned.
    pub pinned: bool,
    /// Normalized actions available before activation.
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    /// Filesystem identity reference for local targets, when present.
    pub filesystem_identity_ref: Option<String>,
    /// Remote target descriptor reference for remote-backed targets, when present.
    pub remote_target_descriptor_ref: Option<String>,
    /// Artifact descriptor reference for import, handoff, or package targets.
    pub artifact_descriptor_ref: Option<String>,
    /// Recovery checkpoints tied to this row.
    pub recovery_checkpoint_refs: Option<Vec<RecoveryCheckpointRef>>,
    /// Lowercase indexed terms used by keyboard-first search.
    pub searchable_terms: Vec<String>,
}

impl RecentWorkListRow {
    /// Projects a canonical recent-work entry into a searchable list row.
    pub fn from_entry(entry: &RecentWorkEntryRecord) -> Self {
        let failure_state = classify_recent_work_failure(entry);
        let mut searchable_terms = searchable_terms_for(entry, failure_state);
        searchable_terms.sort();
        searchable_terms.dedup();

        Self {
            section: if entry.pinned {
                RecentWorkListSection::Pinned
            } else {
                RecentWorkListSection::Recent
            },
            recent_work_id: entry.recent_work_id.clone(),
            presentation_label: entry.presentation_label.clone(),
            presentation_subtitle: entry.presentation_subtitle.clone(),
            target_kind: entry.target_kind,
            target_kind_label: entry.target_kind.surface_label().to_string(),
            target_state: entry.target_state,
            portability_class: entry.portability_class,
            failure_state,
            trust_state: entry.trust_state,
            restore_availability: entry.restore_availability,
            last_opened_at: entry.last_opened_at.clone(),
            pinned: entry.pinned,
            safe_recovery_actions: normalized_recent_work_recovery_actions(entry),
            filesystem_identity_ref: entry.filesystem_identity_ref.clone(),
            remote_target_descriptor_ref: entry.remote_target_descriptor_ref.clone(),
            artifact_descriptor_ref: entry.artifact_descriptor_ref.clone(),
            recovery_checkpoint_refs: entry.recovery_checkpoint_refs.clone(),
            searchable_terms,
        }
    }

    /// Reconstructs the canonical entry shape represented by this row.
    pub fn to_entry_record(&self) -> RecentWorkEntryRecord {
        RecentWorkEntryRecord {
            record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
            entry_and_restore_schema_version: 1,
            recent_work_id: self.recent_work_id.clone(),
            presentation_label: self.presentation_label.clone(),
            presentation_subtitle: self.presentation_subtitle.clone(),
            target_kind: self.target_kind,
            target_state: self.target_state,
            portability_class: self.portability_class,
            trust_state: self.trust_state,
            restore_availability: self.restore_availability,
            safe_recovery_actions: self.safe_recovery_actions.clone(),
            pinned: self.pinned,
            last_opened_at: self.last_opened_at.clone(),
            filesystem_identity_ref: self.filesystem_identity_ref.clone(),
            remote_target_descriptor_ref: self.remote_target_descriptor_ref.clone(),
            artifact_descriptor_ref: self.artifact_descriptor_ref.clone(),
            recovery_checkpoint_refs: self.recovery_checkpoint_refs.clone(),
        }
    }
}

/// Search result partition used by entry surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchableRecentWorkLists {
    /// Normalized lowercase query used for this projection.
    pub query: String,
    /// Pinned entries matching the query.
    pub pinned: Vec<RecentWorkListRow>,
    /// Non-pinned recent entries matching the query.
    pub recent: Vec<RecentWorkListRow>,
    /// Number of matching entries across both sections.
    pub total_matches: usize,
}

impl SearchableRecentWorkLists {
    /// Returns all rows in rendered pinned-then-recent order.
    pub fn rows(&self) -> Vec<RecentWorkListRow> {
        self.pinned
            .iter()
            .chain(self.recent.iter())
            .cloned()
            .collect()
    }
}

/// Projects recent work into searchable pinned and recent lists.
pub fn project_searchable_recent_work_lists(
    registry: &RecentWorkRegistry,
    query: &str,
) -> SearchableRecentWorkLists {
    let query = normalize_recent_work_query(query);
    let mut pinned = Vec::new();
    let mut recent = Vec::new();

    for entry in &registry.entries {
        let row = RecentWorkListRow::from_entry(entry);
        if !recent_work_row_matches_query(&row, &query) {
            continue;
        }
        if row.pinned {
            pinned.push(row);
        } else {
            recent.push(row);
        }
    }

    let total_matches = pinned.len() + recent.len();
    SearchableRecentWorkLists {
        query,
        pinned,
        recent,
        total_matches,
    }
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

fn normalize_recent_work_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn recent_work_row_matches_query(row: &RecentWorkListRow, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    row.searchable_terms.iter().any(|term| term.contains(query))
}

fn searchable_terms_for(
    entry: &RecentWorkEntryRecord,
    failure_state: RecentWorkFailureState,
) -> Vec<String> {
    let mut terms = vec![
        entry.presentation_label.to_ascii_lowercase(),
        entry.target_kind.as_str().to_string(),
        entry.target_kind.surface_label().to_ascii_lowercase(),
        entry.target_state.as_str().to_string(),
        entry.trust_state.as_str().to_string(),
        entry.restore_availability.as_str().to_string(),
        failure_state.as_str().to_string(),
        if entry.pinned { "pinned" } else { "recent" }.to_string(),
    ];

    if let Some(subtitle) = entry.presentation_subtitle.as_deref() {
        terms.push(subtitle.to_ascii_lowercase());
    }

    for action in normalized_recent_work_recovery_actions(entry) {
        terms.push(action.as_str().to_string());
        terms.push(action.surface_label().to_ascii_lowercase());
    }

    terms
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

    #[test]
    fn searchable_lists_split_pinned_and_recent_entries() {
        let registry = RecentWorkRegistry {
            record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
            recent_work_registry_schema_version: 1,
            updated_at: "mono:test".to_string(),
            entries: vec![
                RecentWorkEntryRecord {
                    record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
                    entry_and_restore_schema_version: 1,
                    recent_work_id: "recent:pinned".to_string(),
                    presentation_label: "platform".to_string(),
                    presentation_subtitle: Some("SSH workspace".to_string()),
                    target_kind: TargetKind::SshWorkspace,
                    target_state: RecentWorkTargetState::RemoteUnreachable,
                    portability_class: PortabilityClass::ProviderLinked,
                    trust_state: TrustState::PendingEvaluation,
                    restore_availability: RestoreAvailability::EvidenceOnly,
                    safe_recovery_actions: vec![SafeRecoveryAction::Reconnect],
                    pinned: true,
                    last_opened_at: "mono:1".to_string(),
                    filesystem_identity_ref: None,
                    remote_target_descriptor_ref: Some("remote:platform".to_string()),
                    artifact_descriptor_ref: None,
                    recovery_checkpoint_refs: None,
                },
                RecentWorkEntryRecord {
                    record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
                    entry_and_restore_schema_version: 1,
                    recent_work_id: "recent:local".to_string(),
                    presentation_label: "docs".to_string(),
                    presentation_subtitle: Some("~/Code/docs".to_string()),
                    target_kind: TargetKind::LocalFolder,
                    target_state: RecentWorkTargetState::Reachable,
                    portability_class: PortabilityClass::LocalOnly,
                    trust_state: TrustState::Trusted,
                    restore_availability: RestoreAvailability::None,
                    safe_recovery_actions: vec![SafeRecoveryAction::Open],
                    pinned: false,
                    last_opened_at: "mono:2".to_string(),
                    filesystem_identity_ref: None,
                    remote_target_descriptor_ref: None,
                    artifact_descriptor_ref: None,
                    recovery_checkpoint_refs: None,
                },
            ],
        };

        let all = project_searchable_recent_work_lists(&registry, "");
        assert_eq!(all.pinned.len(), 1);
        assert_eq!(all.recent.len(), 1);
        assert_eq!(all.total_matches, 2);

        let remote = project_searchable_recent_work_lists(&registry, "reconnect");
        assert_eq!(remote.pinned.len(), 1);
        assert!(remote.recent.is_empty());
        assert_eq!(
            remote.pinned[0].failure_state,
            RecentWorkFailureState::ReconnectRequired
        );
    }
}
