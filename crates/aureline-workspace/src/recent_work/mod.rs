// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Recent-work registry and entry vocabulary.
//!
//! The recent-work registry is the canonical source for "resume / reopen"
//! suggestions across shell entry surfaces. It stores typed target identity and
//! availability state so a missing local folder or disconnected remote can
//! never render as an ordinary reachable local open.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const MAX_RECENT_WORK_REGISTRY_BYTES: u64 = 2 * 1024 * 1024;
const MAX_RECENT_WORK_ENTRIES: usize = 4_096;
const MAX_RECOVERY_ACTIONS_PER_ENTRY: usize = 64;
const MAX_RECOVERY_CHECKPOINTS_PER_ENTRY: usize = 128;
const MAX_ID_BYTES: usize = 1_024;
const MAX_LABEL_BYTES: usize = 4_096;
const MAX_REFERENCE_BYTES: usize = 8_192;
static RECENT_WORK_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

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
    /// Returns the default on-disk location for recent-work continuity state.
    pub fn default_store_path() -> PathBuf {
        crate::state_paths::recent_work_root().join("recent_work_registry.json")
    }

    /// Loads the registry from `path` when present, otherwise returns an empty registry.
    pub fn load_or_default(path: impl AsRef<Path>) -> Result<Self, RecentWorkRegistryError> {
        let path = path.as_ref();
        let metadata = match std::fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Self::empty()),
            Err(err) => return Err(RecentWorkRegistryError::Read(err)),
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work registry is not a regular file",
            ));
        }
        if metadata.len() > MAX_RECENT_WORK_REGISTRY_BYTES {
            return Err(RecentWorkRegistryError::TooLarge);
        }
        ensure_relative_path_has_no_symlink_ancestor(path)?;

        let mut file = File::open(path).map_err(RecentWorkRegistryError::Read)?;
        let opened = file.metadata().map_err(RecentWorkRegistryError::Read)?;
        if !opened.is_file() || opened.len() > MAX_RECENT_WORK_REGISTRY_BYTES {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work registry changed during open",
            ));
        }
        let mut payload = Vec::with_capacity(opened.len() as usize);
        Read::by_ref(&mut file)
            .take(MAX_RECENT_WORK_REGISTRY_BYTES + 1)
            .read_to_end(&mut payload)
            .map_err(RecentWorkRegistryError::Read)?;
        if payload.len() as u64 > MAX_RECENT_WORK_REGISTRY_BYTES {
            return Err(RecentWorkRegistryError::TooLarge);
        }
        let after = std::fs::symlink_metadata(path).map_err(RecentWorkRegistryError::Read)?;
        if after.file_type().is_symlink()
            || !after.is_file()
            || !same_open_file_identity(&opened, &after)
        {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work registry changed during read",
            ));
        }

        let registry: Self =
            serde_json::from_slice(&payload).map_err(RecentWorkRegistryError::Parse)?;
        registry.validate()?;
        Ok(registry)
    }

    /// Atomically writes a bounded registry to `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), RecentWorkRegistryError> {
        let path = path.as_ref();
        self.validate()?;
        let mut payload =
            serde_json::to_vec_pretty(self).map_err(RecentWorkRegistryError::Parse)?;
        payload.push(b'\n');
        if payload.len() as u64 > MAX_RECENT_WORK_REGISTRY_BYTES {
            return Err(RecentWorkRegistryError::TooLarge);
        }

        let parent = prepare_registry_parent(path)?;
        reject_unsafe_registry_target(path)?;
        let (temporary_path, mut temporary) = create_registry_temporary(&parent, path)?;
        let result = (|| -> Result<(), RecentWorkRegistryError> {
            temporary
                .write_all(&payload)
                .map_err(RecentWorkRegistryError::Write)?;
            temporary
                .sync_all()
                .map_err(RecentWorkRegistryError::Write)?;
            drop(temporary);
            reject_unsafe_registry_target(path)?;
            replace_registry_file(&temporary_path, path)?;
            sync_registry_directory(&parent)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary_path);
        }
        result
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

    fn empty() -> Self {
        Self {
            record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
            recent_work_registry_schema_version: 1,
            updated_at: "mono:0000:00:00:00.0000".to_string(),
            entries: Vec::new(),
        }
    }

    fn validate(&self) -> Result<(), RecentWorkRegistryError> {
        if self.recent_work_registry_schema_version != 1 {
            return Err(RecentWorkRegistryError::InvalidData(
                "unsupported recent-work registry schema version",
            ));
        }
        if self.entries.len() > MAX_RECENT_WORK_ENTRIES {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work registry entry limit exceeded",
            ));
        }
        if !bounded_nonempty(&self.updated_at, MAX_REFERENCE_BYTES) {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work registry timestamp is invalid",
            ));
        }

        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            validate_recent_work_entry(entry)?;
            if !ids.insert(entry.recent_work_id.as_str()) {
                return Err(RecentWorkRegistryError::InvalidData(
                    "recent-work registry contains duplicate ids",
                ));
            }
        }
        Ok(())
    }
}

fn validate_recent_work_entry(
    entry: &RecentWorkEntryRecord,
) -> Result<(), RecentWorkRegistryError> {
    if entry.entry_and_restore_schema_version != 1 {
        return Err(RecentWorkRegistryError::InvalidData(
            "unsupported recent-work entry schema version",
        ));
    }
    if !bounded_nonempty(&entry.recent_work_id, MAX_ID_BYTES)
        || !bounded_nonempty(&entry.presentation_label, MAX_LABEL_BYTES)
        || !bounded_nonempty(&entry.last_opened_at, MAX_REFERENCE_BYTES)
    {
        return Err(RecentWorkRegistryError::InvalidData(
            "recent-work entry identity or label is invalid",
        ));
    }
    for value in [
        entry.presentation_subtitle.as_deref(),
        entry.filesystem_identity_ref.as_deref(),
        entry.remote_target_descriptor_ref.as_deref(),
        entry.artifact_descriptor_ref.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        if value.is_empty() || value.len() > MAX_REFERENCE_BYTES {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work entry reference is invalid",
            ));
        }
    }
    if entry.safe_recovery_actions.len() > MAX_RECOVERY_ACTIONS_PER_ENTRY {
        return Err(RecentWorkRegistryError::InvalidData(
            "recent-work recovery action limit exceeded",
        ));
    }
    if let Some(checkpoints) = entry.recovery_checkpoint_refs.as_deref() {
        if checkpoints.len() > MAX_RECOVERY_CHECKPOINTS_PER_ENTRY {
            return Err(RecentWorkRegistryError::InvalidData(
                "recent-work checkpoint limit exceeded",
            ));
        }
        for checkpoint in checkpoints {
            if !bounded_nonempty(&checkpoint.recovery_class, MAX_ID_BYTES)
                || !bounded_nonempty(&checkpoint.checkpoint_ref, MAX_REFERENCE_BYTES)
            {
                return Err(RecentWorkRegistryError::InvalidData(
                    "recent-work checkpoint reference is invalid",
                ));
            }
        }
    }
    Ok(())
}

fn bounded_nonempty(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes
}

fn ensure_relative_path_has_no_symlink_ancestor(
    path: &Path,
) -> Result<(), RecentWorkRegistryError> {
    if path.is_absolute() {
        return Ok(());
    }
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut current = PathBuf::new();
    for component in parent.components() {
        let component = match component {
            Component::CurDir => continue,
            Component::Normal(component) => component,
            _ => {
                return Err(RecentWorkRegistryError::InvalidData(
                    "recent-work registry path contains a non-normal component",
                ))
            }
        };
        current.push(component);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(RecentWorkRegistryError::InvalidData(
                    "recent-work registry path contains a symlink",
                ))
            }
            Ok(metadata) if !metadata.is_dir() => {
                return Err(RecentWorkRegistryError::InvalidData(
                    "recent-work registry parent is not a directory",
                ))
            }
            Ok(_) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => break,
            Err(err) => return Err(RecentWorkRegistryError::Read(err)),
        }
    }
    Ok(())
}

fn prepare_registry_parent(path: &Path) -> Result<PathBuf, RecentWorkRegistryError> {
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(RecentWorkRegistryError::InvalidData(
            "recent-work registry path contains parent traversal",
        ));
    }
    ensure_relative_path_has_no_symlink_ancestor(path)?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent).map_err(RecentWorkRegistryError::CreateDir)?;
    let metadata = std::fs::symlink_metadata(parent).map_err(RecentWorkRegistryError::CreateDir)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(RecentWorkRegistryError::InvalidData(
            "recent-work registry parent is unsafe",
        ));
    }
    ensure_relative_path_has_no_symlink_ancestor(path)?;
    Ok(parent.to_path_buf())
}

fn reject_unsafe_registry_target(path: &Path) -> Result<(), RecentWorkRegistryError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => Err(
            RecentWorkRegistryError::InvalidData("recent-work registry target is unsafe"),
        ),
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(RecentWorkRegistryError::Write(err)),
    }
}

fn create_registry_temporary(
    parent: &Path,
    path: &Path,
) -> Result<(PathBuf, File), RecentWorkRegistryError> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or(RecentWorkRegistryError::InvalidData(
            "recent-work registry filename is invalid",
        ))?;
    for _ in 0..16 {
        let sequence = RECENT_WORK_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temporary_path = parent.join(format!(
            ".{file_name}.tmp-{}-{sequence}",
            std::process::id()
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&temporary_path) {
            Ok(file) => return Ok((temporary_path, file)),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(RecentWorkRegistryError::Write(err)),
        }
    }
    Err(RecentWorkRegistryError::InvalidData(
        "recent-work temporary-file namespace is exhausted",
    ))
}

#[cfg(not(windows))]
fn replace_registry_file(temporary: &Path, path: &Path) -> Result<(), RecentWorkRegistryError> {
    std::fs::rename(temporary, path).map_err(RecentWorkRegistryError::Write)
}

#[cfg(windows)]
fn replace_registry_file(temporary: &Path, path: &Path) -> Result<(), RecentWorkRegistryError> {
    match std::fs::rename(temporary, path) {
        Ok(()) => return Ok(()),
        Err(err) => match std::fs::symlink_metadata(path) {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {}
            _ => return Err(RecentWorkRegistryError::Write(err)),
        },
    }

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let sequence = RECENT_WORK_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let backup = parent.join(format!(
        ".recent_work_registry.backup-{}-{sequence}",
        std::process::id()
    ));
    std::fs::hard_link(path, &backup).map_err(RecentWorkRegistryError::Write)?;
    if let Err(err) = std::fs::remove_file(path) {
        let _ = std::fs::remove_file(&backup);
        return Err(RecentWorkRegistryError::Write(err));
    }
    if let Err(err) = std::fs::rename(temporary, path) {
        let _ = std::fs::hard_link(&backup, path);
        let _ = std::fs::remove_file(&backup);
        return Err(RecentWorkRegistryError::Write(err));
    }
    let _ = std::fs::remove_file(&backup);
    Ok(())
}

#[cfg(unix)]
fn sync_registry_directory(parent: &Path) -> Result<(), RecentWorkRegistryError> {
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(RecentWorkRegistryError::Write)
}

#[cfg(not(unix))]
fn sync_registry_directory(_parent: &Path) -> Result<(), RecentWorkRegistryError> {
    Ok(())
}

#[cfg(unix)]
fn same_open_file_identity(opened: &std::fs::Metadata, after: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    opened.dev() == after.dev() && opened.ino() == after.ino() && opened.len() == after.len()
}

#[cfg(not(unix))]
fn same_open_file_identity(opened: &std::fs::Metadata, after: &std::fs::Metadata) -> bool {
    opened.len() == after.len()
        && opened.modified().ok() == after.modified().ok()
        && opened.created().ok() == after.created().ok()
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
    InvalidData(&'static str),
    TooLarge,
}

impl std::fmt::Display for RecentWorkRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateDir(err) => write!(f, "create dir failed: {err}"),
            Self::Read(err) => write!(f, "read failed: {err}"),
            Self::Write(err) => write!(f, "write failed: {err}"),
            Self::Parse(err) => write!(f, "parse failed: {err}"),
            Self::InvalidData(reason) => write!(f, "invalid recent-work registry: {reason}"),
            Self::TooLarge => write!(f, "recent-work registry exceeds the byte limit"),
        }
    }
}

impl std::error::Error for RecentWorkRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(label: &str) -> Self {
            for _ in 0..16 {
                let sequence = RECENT_WORK_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
                let path = std::env::temp_dir().join(format!(
                    "aureline-recent-work-{label}-{}-{sequence}",
                    std::process::id()
                ));
                match std::fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(err) => panic!("create test directory failed: {err}"),
                }
            }
            panic!("recent-work test directory namespace exhausted")
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    fn local_entry(id: &str) -> RecentWorkEntryRecord {
        RecentWorkEntryRecord {
            record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
            entry_and_restore_schema_version: 1,
            recent_work_id: id.to_string(),
            presentation_label: "docs".to_string(),
            presentation_subtitle: Some("workspace/docs".to_string()),
            target_kind: TargetKind::LocalFolder,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::None,
            safe_recovery_actions: vec![SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:2".to_string(),
            filesystem_identity_ref: Some("filesystem:docs".to_string()),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
            recovery_checkpoint_refs: None,
        }
    }

    #[test]
    fn registry_save_is_bounded_atomic_and_round_trips_updates() {
        let temp = TestDirectory::new("roundtrip");
        let path = temp.path().join("state/recent_work_registry.json");
        let mut registry = RecentWorkRegistry::empty();
        registry.updated_at = "mono:1".to_string();
        registry.upsert(local_entry("recent:docs"));
        registry.save(&path).expect("first save");

        registry.updated_at = "mono:2".to_string();
        registry.upsert(local_entry("recent:source"));
        registry.save(&path).expect("replacement save");

        assert_eq!(
            RecentWorkRegistry::load_or_default(&path).expect("load registry"),
            registry
        );
        let state_dir = path.parent().expect("state parent");
        assert!(std::fs::read_dir(state_dir)
            .expect("state entries")
            .all(|entry| !entry
                .expect("state entry")
                .file_name()
                .to_string_lossy()
                .contains(".tmp-")));
    }

    #[test]
    fn registry_load_rejects_oversized_and_wrong_schema_state() {
        let temp = TestDirectory::new("bounds");
        let oversized = temp.path().join("oversized.json");
        File::create(&oversized)
            .and_then(|file| file.set_len(MAX_RECENT_WORK_REGISTRY_BYTES + 1))
            .expect("oversized registry");
        assert!(matches!(
            RecentWorkRegistry::load_or_default(&oversized),
            Err(RecentWorkRegistryError::TooLarge)
        ));

        let wrong_schema = temp.path().join("wrong-schema.json");
        let mut registry = RecentWorkRegistry::empty();
        registry.recent_work_registry_schema_version = 99;
        std::fs::write(
            &wrong_schema,
            serde_json::to_vec(&registry).expect("serialize registry"),
        )
        .expect("write wrong schema");
        assert!(matches!(
            RecentWorkRegistry::load_or_default(&wrong_schema),
            Err(RecentWorkRegistryError::InvalidData(
                "unsupported recent-work registry schema version"
            ))
        ));
        assert!(matches!(
            registry.save(temp.path().join("must-not-write.json")),
            Err(RecentWorkRegistryError::InvalidData(
                "unsupported recent-work registry schema version"
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn registry_never_follows_a_symlink_target() {
        use std::os::unix::fs::symlink;

        let temp = TestDirectory::new("symlink");
        let outside = temp.path().join("outside.txt");
        let path = temp.path().join("recent.json");
        std::fs::write(&outside, b"PRIVATE-RECENT-WORK-SENTINEL").expect("outside file");
        symlink(&outside, &path).expect("registry symlink");

        assert!(matches!(
            RecentWorkRegistry::load_or_default(&path),
            Err(RecentWorkRegistryError::InvalidData(_))
        ));
        assert!(matches!(
            RecentWorkRegistry::empty().save(&path),
            Err(RecentWorkRegistryError::InvalidData(_))
        ));
        assert_eq!(
            std::fs::read(&outside).expect("outside bytes"),
            b"PRIVATE-RECENT-WORK-SENTINEL"
        );
    }

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
