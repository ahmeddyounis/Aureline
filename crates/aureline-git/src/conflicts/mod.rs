//! Conflict and external-change handoff records for editor and Git surfaces.
//!
//! This module joins two existing alpha contracts: VFS compare-before-write
//! records and Git status conflict rows. It produces one packet that keeps the
//! divergence source, path identity, recovery posture, and safe next actions
//! visible in both editor and source-control surfaces.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use aureline_vfs::{ExternalChangeCompareRecord, ExternalChangeResolutionAction, SaveOutcome};
use serde::{Deserialize, Serialize};

use crate::status::{
    ConsumerProjectionBundle, GitChange, GitServiceState, GitStatusBackend, GitStatusRequest,
    GitStatusService, RepositoryIdentity, SystemGitStatusBackend,
};

/// Stable record-kind tag for [`GitConflictHandoffPacket`].
pub const GIT_CONFLICT_HANDOFF_PACKET_RECORD_KIND: &str = "git_conflict_handoff_packet";

/// Stable record-kind tag for [`GitConflictSurfaceRecord`].
pub const GIT_CONFLICT_SURFACE_RECORD_KIND: &str = "git_conflict_surface_record";

/// Stable record-kind tag for [`GitConflictSupportExportRecord`].
pub const GIT_CONFLICT_SUPPORT_EXPORT_RECORD_KIND: &str = "git_conflict_support_export_record";

const GIT_CONFLICT_HANDOFF_SCHEMA_VERSION: u32 = 1;
const GIT_CONFLICT_SURFACE_SCHEMA_VERSION: u32 = 1;
const GIT_CONFLICT_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Source that made the editor/Git handoff necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitConflictDivergenceSource {
    /// Git reported an unresolved merge, rebase, cherry-pick, revert, or stash conflict.
    GitMergeConflict,
    /// VFS compare-before-write detected that the canonical target changed externally.
    VfsExternalChange,
    /// Git conflict state and VFS external-change state both apply to the same path.
    GitAndVfsDiverged,
    /// No conflict was found for the selected path.
    NoConflictDetected,
}

impl GitConflictDivergenceSource {
    /// Returns the stable string vocabulary for this divergence source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitMergeConflict => "git_merge_conflict",
            Self::VfsExternalChange => "vfs_external_change",
            Self::GitAndVfsDiverged => "git_and_vfs_diverged",
            Self::NoConflictDetected => "no_conflict_detected",
        }
    }

    /// Returns a reviewer-facing source label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GitMergeConflict => "Git merge conflict",
            Self::VfsExternalChange => "External filesystem change",
            Self::GitAndVfsDiverged => "Git and filesystem divergence",
            Self::NoConflictDetected => "No conflict detected",
        }
    }
}

/// Surface that renders a conflict handoff projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitConflictSurfaceKind {
    /// Editor buffer, tab, save, or inline conflict surface.
    Editor,
    /// Source-control, change-list, history, or review surface.
    Git,
}

impl GitConflictSurfaceKind {
    /// Returns the stable string vocabulary for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Git => "git",
        }
    }
}

/// Visibility state rendered by an editor or Git surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitConflictSurfaceState {
    /// A merge conflict is visible and blocks normal mutation actions.
    ConflictVisible,
    /// External-change comparison is visible and blocks silent overwrite.
    CompareRequiredVisible,
    /// The selected path has no conflict state.
    NoConflict,
    /// The producer could not prove current conflict state.
    Degraded,
}

impl GitConflictSurfaceState {
    /// Returns the stable string vocabulary for this surface state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConflictVisible => "conflict_visible",
            Self::CompareRequiredVisible => "compare_required_visible",
            Self::NoConflict => "no_conflict",
            Self::Degraded => "degraded",
        }
    }

    /// Returns true when the state must remain visible in the surface.
    pub const fn is_visible_conflict_state(self) -> bool {
        matches!(self, Self::ConflictVisible | Self::CompareRequiredVisible)
    }
}

/// Safe next action exposed by conflict handoff surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitConflictSafeAction {
    /// Open a compare view without mutating the target.
    OpenCompare,
    /// Open the merge or conflict-resolution surface.
    OpenMergeResolver,
    /// Open the Git status or source-control detail surface.
    OpenGitStatus,
    /// Abort the in-progress Git history operation when Git supports it.
    AbortGitOperation,
    /// Restore a named checkpoint.
    RestoreCheckpoint,
    /// Reload the externally changed bytes after review.
    ReloadExternal,
    /// Save the local buffer to a different target.
    SaveAs,
    /// Re-run identity and generation-token comparison.
    Recompare,
    /// Open alias and canonical-path details.
    OpenAliasDetails,
    /// Leave the conflict visible without applying a destructive choice.
    Postpone,
    /// Close the handoff without mutating durable state.
    Cancel,
}

impl GitConflictSafeAction {
    /// Returns the stable string vocabulary for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCompare => "open_compare",
            Self::OpenMergeResolver => "open_merge_resolver",
            Self::OpenGitStatus => "open_git_status",
            Self::AbortGitOperation => "abort_git_operation",
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::ReloadExternal => "reload_external",
            Self::SaveAs => "save_as",
            Self::Recompare => "recompare",
            Self::OpenAliasDetails => "open_alias_details",
            Self::Postpone => "postpone",
            Self::Cancel => "cancel",
        }
    }

    /// Maps VFS compare-before-write actions into Git handoff actions.
    pub const fn from_vfs_action(action: ExternalChangeResolutionAction) -> Self {
        match action {
            ExternalChangeResolutionAction::Write => Self::Cancel,
            ExternalChangeResolutionAction::Compare => Self::OpenCompare,
            ExternalChangeResolutionAction::ReloadExternal => Self::ReloadExternal,
            ExternalChangeResolutionAction::Merge => Self::OpenMergeResolver,
            ExternalChangeResolutionAction::SaveAs => Self::SaveAs,
            ExternalChangeResolutionAction::Recompare => Self::Recompare,
            ExternalChangeResolutionAction::OpenAliasDetails => Self::OpenAliasDetails,
            ExternalChangeResolutionAction::Cancel => Self::Cancel,
        }
    }
}

/// Request for a Git merge-conflict handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictHandoffRequest {
    /// Stable workspace identity copied into downstream records.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Repository-relative or absolute path selected by the caller.
    pub path: PathBuf,
    /// Optional VFS identity ref already known by the caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem_identity_ref: Option<String>,
    /// Optional checkpoint or recovery object captured before the conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Timestamp supplied by the caller for deterministic exports.
    pub requested_at: String,
}

impl GitConflictHandoffRequest {
    /// Builds a path-scoped request with a derived local workspace identity.
    pub fn for_path(root_path: impl Into<PathBuf>, path: impl Into<PathBuf>) -> Self {
        let root_path = root_path.into();
        let workspace_ref = root_path
            .file_name()
            .and_then(OsStr::to_str)
            .map(|label| format!("workspace.local.{}", sanitize_id(label)))
            .filter(|label| label != "workspace.local.")
            .unwrap_or_else(|| "workspace.local.root".to_string());
        Self {
            workspace_ref,
            root_path,
            path: path.into(),
            filesystem_identity_ref: None,
            rollback_checkpoint_ref: None,
            requested_at: "now".to_string(),
        }
    }

    /// Builds a path-scoped request with explicit identity and timestamp fields.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        path: impl Into<PathBuf>,
        requested_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            path: path.into(),
            filesystem_identity_ref: None,
            rollback_checkpoint_ref: None,
            requested_at: requested_at.into(),
        }
    }

    /// Attaches a VFS identity ref to the request.
    pub fn with_filesystem_identity_ref(
        mut self,
        filesystem_identity_ref: impl Into<String>,
    ) -> Self {
        self.filesystem_identity_ref = Some(filesystem_identity_ref.into());
        self
    }

    /// Attaches a checkpoint or recovery object ref to the request.
    pub fn with_rollback_checkpoint_ref(
        mut self,
        rollback_checkpoint_ref: impl Into<String>,
    ) -> Self {
        self.rollback_checkpoint_ref = Some(rollback_checkpoint_ref.into());
        self
    }
}

/// Input that joins a VFS external-change compare record into the Git handoff lane.
#[derive(Debug, Clone, Copy)]
pub struct GitExternalChangeHandoffInput<'a> {
    /// Stable workspace identity copied into downstream records.
    pub workspace_ref: &'a str,
    /// Root path selected by the workspace or launch wedge.
    pub repo_root: &'a Path,
    /// Repository-relative path associated with the compared file.
    pub repo_relative_path: &'a Path,
    /// Timestamp supplied by the caller for deterministic exports.
    pub generated_at: &'a str,
    /// VFS compare-before-write record produced without mutating durable bytes.
    pub compare_record: &'a ExternalChangeCompareRecord,
    /// Optional Git truth-source ref to preserve when the caller has one.
    pub git_truth_source_ref: Option<&'a str>,
    /// Optional checkpoint captured before a related mutation.
    pub rollback_checkpoint_ref: Option<&'a str>,
}

/// Path and identity join keys carried by editor and Git projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictPathIdentity {
    /// Stable path-truth ref for handoff rows.
    pub path_truth_ref: String,
    /// Shared VFS filesystem identity ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem_identity_ref: Option<String>,
    /// VFS-provided Git file identity ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_file_identity_ref: Option<String>,
    /// Repository-relative path for Git consumers.
    pub repo_relative_path: PathBuf,
    /// Original path when Git reported a rename or move conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_repo_relative_path: Option<PathBuf>,
    /// Presentation label the editor should preserve.
    pub presentation_path_label: String,
    /// Canonical object label or identity token, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_target_label: Option<String>,
}

impl GitConflictPathIdentity {
    /// Returns true when the path has a stable join key and visible label.
    pub fn is_joinable(&self) -> bool {
        !self.path_truth_ref.trim().is_empty() && !self.presentation_path_label.trim().is_empty()
    }

    /// Returns true when VFS and Git identity refs agree, or VFS identity is absent.
    pub fn vfs_and_git_refs_align(&self) -> bool {
        match (
            self.filesystem_identity_ref.as_deref(),
            self.git_file_identity_ref.as_deref(),
        ) {
            (Some(filesystem), Some(git)) => filesystem == git,
            (Some(_), None) => false,
            _ => true,
        }
    }
}

/// Recovery posture carried by the handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictRollbackCheckpoint {
    /// Optional checkpoint or recovery object ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Source that supplied the checkpoint or recovery path.
    pub checkpoint_source: String,
    /// True when a named rollback or abort path is available.
    pub rollback_available: bool,
    /// Stable rollback class for support and review surfaces.
    pub rollback_path_class: String,
    /// Command id that opens the rollback path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_command_id: Option<String>,
    /// True when the handoff blocked before a new write committed.
    pub no_write_committed: bool,
    /// Reviewer-facing recovery detail.
    pub detail_label: String,
}

impl GitConflictRollbackCheckpoint {
    /// Returns true when the record is honest about recovery.
    pub fn is_recovery_explained(&self) -> bool {
        self.no_write_committed
            || (self.rollback_available
                && self
                    .checkpoint_ref
                    .as_ref()
                    .is_some_and(|value| !value.is_empty()))
            || !self.detail_label.trim().is_empty()
    }
}

/// Git-side state observed for the selected path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictGitStateProjection {
    /// Git status code observed at handoff time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// True when Git reported an unresolved conflict.
    pub git_conflict_reported: bool,
    /// Conflict class used by merge-resolution surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_class_value: Option<String>,
    /// Number of unresolved conflict markers or unresolved path rows.
    pub unresolved_count: u32,
    /// Navigation class for unresolved conflict state.
    pub unresolved_count_navigation_class: String,
    /// In-progress Git operation ref when detected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_operation_ref: Option<String>,
    /// Base revision ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_revision_ref: Option<String>,
    /// Current revision ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_revision_ref: Option<String>,
    /// Incoming revision ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_revision_ref: Option<String>,
}

/// VFS external-change compare projection retained by the Git handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictExternalCompareProjection {
    /// VFS compare outcome token.
    pub compare_outcome: String,
    /// Save outcome that blocked the write, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_save_outcome: Option<String>,
    /// Pinned canonical target URI from the save token.
    pub pinned_canonical_uri: String,
    /// Observed canonical target URI after re-resolving the presentation path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_canonical_uri: Option<String>,
    /// Generation-token kind captured at open.
    pub pinned_generation_token_kind: String,
    /// Generation-token value captured at open.
    pub pinned_generation_token_value: String,
    /// Generation-token kind observed at compare time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation_token_kind: Option<String>,
    /// Generation-token value observed at compare time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation_token_value: Option<String>,
    /// Diff availability token from the VFS compare record.
    pub diff_availability: String,
    /// Number of changed hunks in the compare preview.
    pub changed_hunk_count: u32,
    /// Number of changed external lines in the compare preview.
    pub external_line_change_count: u32,
    /// Number of changed local lines in the compare preview.
    pub local_line_change_count: u32,
    /// True when overwrite is blocked until review resolves the compare.
    pub silent_overwrite_forbidden: bool,
    /// Safe action tokens inherited from VFS compare-before-write.
    pub vfs_resolution_actions: Vec<String>,
}

impl GitConflictExternalCompareProjection {
    /// Builds a redaction-safe projection from a VFS compare record.
    pub fn from_compare_record(record: &ExternalChangeCompareRecord) -> Self {
        Self {
            compare_outcome: record.outcome.as_str().to_string(),
            blocking_save_outcome: record
                .blocking_save_outcome
                .map(SaveOutcome::as_str)
                .map(str::to_string),
            pinned_canonical_uri: record.pinned_canonical_uri.to_string(),
            observed_canonical_uri: record
                .observed_canonical_uri
                .as_ref()
                .map(ToString::to_string),
            pinned_generation_token_kind: record.pinned_generation_token_kind.as_str().to_string(),
            pinned_generation_token_value: record.pinned_generation_token_value.clone(),
            observed_generation_token_kind: record
                .observed_generation_token_kind
                .map(|kind| kind.as_str().to_string()),
            observed_generation_token_value: record.observed_generation_token_value.clone(),
            diff_availability: record.diff.availability.as_str().to_string(),
            changed_hunk_count: record.diff.changed_hunk_count,
            external_line_change_count: record.diff.external_line_change_count,
            local_line_change_count: record.diff.local_line_change_count,
            silent_overwrite_forbidden: record.silent_overwrite_forbidden,
            vfs_resolution_actions: record
                .resolution_actions
                .iter()
                .map(|action| action.as_str().to_string())
                .collect(),
        }
    }
}

/// Surface-specific view of the same handoff state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictSurfaceRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable surface row ref.
    pub surface_ref: String,
    /// Surface kind that renders this projection.
    pub surface_kind: GitConflictSurfaceKind,
    /// Visible state class for this surface.
    pub state_class: GitConflictSurfaceState,
    /// Source that made the handoff necessary.
    pub divergence_source: GitConflictDivergenceSource,
    /// Shared handoff packet ref.
    pub handoff_ref: String,
    /// Shared path-truth ref.
    pub path_truth_ref: String,
    /// Shared filesystem identity ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem_identity_ref: Option<String>,
    /// Reviewer-facing path label.
    pub path_label: String,
    /// Primary safe action for the surface.
    pub primary_action: GitConflictSafeAction,
    /// Ordered safe actions this surface may expose.
    pub safe_next_actions: Vec<GitConflictSafeAction>,
    /// Source detail shown near the conflict banner or row.
    pub source_detail_label: String,
    /// Recovery detail shown before destructive choices.
    pub rollback_detail_label: String,
}

impl GitConflictSurfaceRecord {
    /// Returns true when the surface points at the shared handoff and path refs.
    pub fn shares_handoff_identity(&self, handoff_ref: &str, path_truth_ref: &str) -> bool {
        self.handoff_ref == handoff_ref
            && self.path_truth_ref == path_truth_ref
            && self.state_class.is_visible_conflict_state()
    }
}

/// Redaction-safe support/export projection for a conflict handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictSupportExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable support-export ref.
    pub support_export_ref: String,
    /// Redaction mode for this export row.
    pub redaction_mode: String,
    /// Retention class for this export row.
    pub retention_class: String,
    /// Workspace identity copied from the packet.
    pub workspace_ref: String,
    /// Handoff packet ref.
    pub handoff_ref: String,
    /// Divergence source token.
    pub divergence_source: String,
    /// Shared path-truth ref.
    pub path_truth_ref: String,
    /// Shared filesystem identity ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem_identity_ref: Option<String>,
    /// Git status code observed for the path, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_status_code: Option<String>,
    /// VFS compare outcome observed for the path, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_compare_outcome: Option<String>,
    /// Checkpoint refs available to support and recovery surfaces.
    pub checkpoint_refs: Vec<String>,
    /// Safe action tokens available without raw body export.
    pub safe_action_tokens: Vec<String>,
    /// Fields deliberately omitted from export.
    pub omitted_fields: Vec<String>,
}

/// Unified handoff packet consumed by editor, Git, CLI, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConflictHandoffPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable handoff packet ref.
    pub handoff_ref: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity copied from the request.
    pub workspace_ref: String,
    /// Repository root selected by the workspace or launch wedge.
    pub repo_root: PathBuf,
    /// Git truth-source ref when a status snapshot was available.
    pub truth_source_ref: String,
    /// Source that made the handoff necessary.
    pub divergence_source: GitConflictDivergenceSource,
    /// Reviewer-facing source label.
    pub divergence_source_label: String,
    /// Path and identity join keys.
    pub path_identity: GitConflictPathIdentity,
    /// Git-side state for the selected path.
    pub git_state: GitConflictGitStateProjection,
    /// VFS external-change compare projection, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_compare: Option<GitConflictExternalCompareProjection>,
    /// Checkpoint and recovery posture for the handoff.
    pub rollback_checkpoint: GitConflictRollbackCheckpoint,
    /// Editor surface projection.
    pub editor_surface: GitConflictSurfaceRecord,
    /// Git surface projection.
    pub git_surface: GitConflictSurfaceRecord,
    /// Redaction-safe support/export projection.
    pub support_export: GitConflictSupportExportRecord,
}

impl GitConflictHandoffPacket {
    /// Returns true when both editor and Git surfaces keep the state visible.
    pub fn conflict_state_visible_in_editor_and_git(&self) -> bool {
        self.editor_surface
            .shares_handoff_identity(&self.handoff_ref, &self.path_identity.path_truth_ref)
            && self
                .git_surface
                .shares_handoff_identity(&self.handoff_ref, &self.path_identity.path_truth_ref)
    }

    /// Returns true when identity and recovery posture are preserved.
    pub fn preserves_identity_and_recovery(&self) -> bool {
        self.path_identity.is_joinable()
            && self.path_identity.vfs_and_git_refs_align()
            && self.rollback_checkpoint.is_recovery_explained()
    }

    /// Returns the ordered safe action tokens from the editor projection.
    pub fn safe_action_tokens(&self) -> Vec<&'static str> {
        self.editor_surface
            .safe_next_actions
            .iter()
            .map(|action| action.as_str())
            .collect()
    }
}

/// Service that creates conflict handoff packets from Git and VFS state.
#[derive(Debug, Clone)]
pub struct GitConflictHandoffService<B = SystemGitStatusBackend> {
    status_service: GitStatusService<B>,
}

impl Default for GitConflictHandoffService<SystemGitStatusBackend> {
    fn default() -> Self {
        Self::new(SystemGitStatusBackend::default())
    }
}

impl<B> GitConflictHandoffService<B> {
    /// Creates a service backed by a Git status backend.
    pub fn new(status_backend: B) -> Self {
        Self {
            status_service: GitStatusService::new(status_backend),
        }
    }

    /// Builds a handoff packet from a VFS external-change compare record.
    pub fn from_external_change_compare(
        input: GitExternalChangeHandoffInput<'_>,
    ) -> GitConflictHandoffPacket {
        let source = GitConflictDivergenceSource::VfsExternalChange;
        let filesystem_identity_ref = input
            .compare_record
            .identity_references
            .filesystem_identity_ref
            .clone();
        let git_file_identity_ref = input
            .compare_record
            .identity_references
            .git_file_identity_ref
            .clone();
        let handoff_ref = handoff_ref(input.workspace_ref, source, input.repo_relative_path);
        let path_truth_ref = format!(
            "path.truth.git.conflict.{}",
            sanitize_id(&git_file_identity_ref)
        );
        let path_identity = GitConflictPathIdentity {
            path_truth_ref,
            filesystem_identity_ref: Some(filesystem_identity_ref),
            git_file_identity_ref: Some(git_file_identity_ref),
            repo_relative_path: input.repo_relative_path.to_path_buf(),
            original_repo_relative_path: None,
            presentation_path_label: input.compare_record.presentation_uri.to_string(),
            canonical_target_label: Some(input.compare_record.pinned_canonical_uri.to_string()),
        };
        let external_compare =
            GitConflictExternalCompareProjection::from_compare_record(input.compare_record);
        let rollback_checkpoint = external_rollback_checkpoint(input);
        let safe_actions = external_safe_actions(input.compare_record);
        let state_class = if input.compare_record.review_required {
            GitConflictSurfaceState::CompareRequiredVisible
        } else {
            GitConflictSurfaceState::NoConflict
        };
        let source_detail = format!(
            "{}; compare outcome {}",
            source.label(),
            input.compare_record.outcome.as_str()
        );
        let editor_surface = surface_record(SurfaceRecordInput {
            handoff_ref: &handoff_ref,
            surface_kind: GitConflictSurfaceKind::Editor,
            state_class,
            divergence_source: source,
            path_identity: &path_identity,
            safe_next_actions: safe_actions.clone(),
            source_detail_label: source_detail.clone(),
            rollback_detail_label: rollback_checkpoint.detail_label.clone(),
        });
        let git_surface = surface_record(SurfaceRecordInput {
            handoff_ref: &handoff_ref,
            surface_kind: GitConflictSurfaceKind::Git,
            state_class,
            divergence_source: source,
            path_identity: &path_identity,
            safe_next_actions: safe_actions.clone(),
            source_detail_label: source_detail,
            rollback_detail_label: rollback_checkpoint.detail_label.clone(),
        });
        let git_state = GitConflictGitStateProjection {
            status_code: None,
            git_conflict_reported: false,
            conflict_class_value: None,
            unresolved_count: 0,
            unresolved_count_navigation_class: "external_compare_diff_navigation".to_string(),
            history_operation_ref: None,
            base_revision_ref: None,
            current_revision_ref: None,
            incoming_revision_ref: None,
        };
        let support_export = support_export_record(SupportExportInput {
            workspace_ref: input.workspace_ref,
            handoff_ref: &handoff_ref,
            divergence_source: source,
            path_identity: &path_identity,
            git_state: &git_state,
            external_compare: Some(&external_compare),
            rollback_checkpoint: &rollback_checkpoint,
            safe_actions: &safe_actions,
        });

        GitConflictHandoffPacket {
            record_kind: GIT_CONFLICT_HANDOFF_PACKET_RECORD_KIND.to_string(),
            schema_version: GIT_CONFLICT_HANDOFF_SCHEMA_VERSION,
            handoff_ref,
            generated_at: input.generated_at.to_string(),
            workspace_ref: input.workspace_ref.to_string(),
            repo_root: input.repo_root.to_path_buf(),
            truth_source_ref: input
                .git_truth_source_ref
                .map(str::to_string)
                .unwrap_or_else(|| "vfs.external_change_compare".to_string()),
            divergence_source: source,
            divergence_source_label: source.label().to_string(),
            path_identity,
            git_state,
            external_compare: Some(external_compare),
            rollback_checkpoint,
            editor_surface,
            git_surface,
            support_export,
        }
    }
}

impl<B: GitStatusBackend> GitConflictHandoffService<B> {
    /// Builds a handoff packet from the current Git status for one path.
    pub fn preview_git_conflict(
        &self,
        request: &GitConflictHandoffRequest,
    ) -> GitConflictHandoffPacket {
        let status_request = GitStatusRequest::with_observed_at(
            request.workspace_ref.clone(),
            request.root_path.clone(),
            request.requested_at.clone(),
        );
        let snapshot = self.status_service.snapshot(&status_request);
        let truth_source_ref =
            ConsumerProjectionBundle::from_snapshot(request.requested_at.clone(), &snapshot)
                .truth_source_ref;
        let repo_root = snapshot
            .repository
            .as_ref()
            .map(|repo| repo.repo_root.clone())
            .unwrap_or_else(|| request.root_path.clone());
        let repo_path = normalize_requested_path(&request.path, &repo_root);
        let change = snapshot
            .changes
            .iter()
            .find(|change| change.path == repo_path);
        let git_conflict_reported = change.is_some_and(|change| change.is_conflicted);
        let source = if git_conflict_reported {
            GitConflictDivergenceSource::GitMergeConflict
        } else {
            GitConflictDivergenceSource::NoConflictDetected
        };
        let handoff_ref = handoff_ref(&request.workspace_ref, source, &repo_path);
        let path_identity = git_path_identity(
            &request.workspace_ref,
            &repo_path,
            request.filesystem_identity_ref.clone(),
            snapshot.repository.as_ref(),
            change,
        );
        let rollback_checkpoint = git_rollback_checkpoint(
            &request.workspace_ref,
            &repo_path,
            snapshot.repository.as_ref(),
            request.rollback_checkpoint_ref.clone(),
            git_conflict_reported,
        );
        let unresolved_count = if git_conflict_reported {
            count_conflict_markers(&repo_root.join(&repo_path)).max(1)
        } else {
            0
        };
        let git_state = GitConflictGitStateProjection {
            status_code: change.map(|change| change.status_code.clone()),
            git_conflict_reported,
            conflict_class_value: git_conflict_reported
                .then(|| "plain_text_line_oriented_merge".to_string()),
            unresolved_count,
            unresolved_count_navigation_class: if git_conflict_reported {
                "count_required_with_next_unresolved_navigation"
            } else {
                "count_not_applicable_no_conflict"
            }
            .to_string(),
            history_operation_ref: snapshot
                .repository
                .as_ref()
                .and_then(detect_history_operation_ref),
            base_revision_ref: git_conflict_reported.then(|| "git.stage.base".to_string()),
            current_revision_ref: git_conflict_reported.then(|| "git.stage.current".to_string()),
            incoming_revision_ref: git_conflict_reported.then(|| "git.stage.incoming".to_string()),
        };
        let state_class = if snapshot.service_state != GitServiceState::Current {
            GitConflictSurfaceState::Degraded
        } else if git_conflict_reported {
            GitConflictSurfaceState::ConflictVisible
        } else {
            GitConflictSurfaceState::NoConflict
        };
        let safe_actions = git_safe_actions(git_conflict_reported, &rollback_checkpoint);
        let source_detail = if git_conflict_reported {
            format!(
                "{}; status {}; unresolved count {}",
                source.label(),
                git_state.status_code.as_deref().unwrap_or("unknown"),
                unresolved_count
            )
        } else {
            source.label().to_string()
        };
        let editor_surface = surface_record(SurfaceRecordInput {
            handoff_ref: &handoff_ref,
            surface_kind: GitConflictSurfaceKind::Editor,
            state_class,
            divergence_source: source,
            path_identity: &path_identity,
            safe_next_actions: safe_actions.clone(),
            source_detail_label: source_detail.clone(),
            rollback_detail_label: rollback_checkpoint.detail_label.clone(),
        });
        let git_surface = surface_record(SurfaceRecordInput {
            handoff_ref: &handoff_ref,
            surface_kind: GitConflictSurfaceKind::Git,
            state_class,
            divergence_source: source,
            path_identity: &path_identity,
            safe_next_actions: safe_actions.clone(),
            source_detail_label: source_detail,
            rollback_detail_label: rollback_checkpoint.detail_label.clone(),
        });
        let support_export = support_export_record(SupportExportInput {
            workspace_ref: &request.workspace_ref,
            handoff_ref: &handoff_ref,
            divergence_source: source,
            path_identity: &path_identity,
            git_state: &git_state,
            external_compare: None,
            rollback_checkpoint: &rollback_checkpoint,
            safe_actions: &safe_actions,
        });

        GitConflictHandoffPacket {
            record_kind: GIT_CONFLICT_HANDOFF_PACKET_RECORD_KIND.to_string(),
            schema_version: GIT_CONFLICT_HANDOFF_SCHEMA_VERSION,
            handoff_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            divergence_source: source,
            divergence_source_label: source.label().to_string(),
            path_identity,
            git_state,
            external_compare: None,
            rollback_checkpoint,
            editor_surface,
            git_surface,
            support_export,
        }
    }
}

struct SurfaceRecordInput<'a> {
    handoff_ref: &'a str,
    surface_kind: GitConflictSurfaceKind,
    state_class: GitConflictSurfaceState,
    divergence_source: GitConflictDivergenceSource,
    path_identity: &'a GitConflictPathIdentity,
    safe_next_actions: Vec<GitConflictSafeAction>,
    source_detail_label: String,
    rollback_detail_label: String,
}

fn surface_record(input: SurfaceRecordInput<'_>) -> GitConflictSurfaceRecord {
    GitConflictSurfaceRecord {
        record_kind: GIT_CONFLICT_SURFACE_RECORD_KIND.to_string(),
        schema_version: GIT_CONFLICT_SURFACE_SCHEMA_VERSION,
        surface_ref: format!(
            "surface.{}.{}",
            input.surface_kind.as_str(),
            sanitize_id(input.handoff_ref)
        ),
        surface_kind: input.surface_kind,
        state_class: input.state_class,
        divergence_source: input.divergence_source,
        handoff_ref: input.handoff_ref.to_string(),
        path_truth_ref: input.path_identity.path_truth_ref.clone(),
        filesystem_identity_ref: input.path_identity.filesystem_identity_ref.clone(),
        path_label: input.path_identity.presentation_path_label.clone(),
        primary_action: input
            .safe_next_actions
            .first()
            .copied()
            .unwrap_or(GitConflictSafeAction::Cancel),
        safe_next_actions: input.safe_next_actions,
        source_detail_label: input.source_detail_label,
        rollback_detail_label: input.rollback_detail_label,
    }
}

struct SupportExportInput<'a> {
    workspace_ref: &'a str,
    handoff_ref: &'a str,
    divergence_source: GitConflictDivergenceSource,
    path_identity: &'a GitConflictPathIdentity,
    git_state: &'a GitConflictGitStateProjection,
    external_compare: Option<&'a GitConflictExternalCompareProjection>,
    rollback_checkpoint: &'a GitConflictRollbackCheckpoint,
    safe_actions: &'a [GitConflictSafeAction],
}

fn support_export_record(input: SupportExportInput<'_>) -> GitConflictSupportExportRecord {
    GitConflictSupportExportRecord {
        record_kind: GIT_CONFLICT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_CONFLICT_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.export.{}", sanitize_id(input.handoff_ref)),
        redaction_mode: "metadata_only".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        workspace_ref: input.workspace_ref.to_string(),
        handoff_ref: input.handoff_ref.to_string(),
        divergence_source: input.divergence_source.as_str().to_string(),
        path_truth_ref: input.path_identity.path_truth_ref.clone(),
        filesystem_identity_ref: input.path_identity.filesystem_identity_ref.clone(),
        git_status_code: input.git_state.status_code.clone(),
        external_compare_outcome: input
            .external_compare
            .map(|compare| compare.compare_outcome.clone()),
        checkpoint_refs: input
            .rollback_checkpoint
            .checkpoint_ref
            .iter()
            .cloned()
            .collect(),
        safe_action_tokens: input
            .safe_actions
            .iter()
            .map(|action| action.as_str().to_string())
            .collect(),
        omitted_fields: vec![
            "raw_patch_body".to_string(),
            "raw_file_body".to_string(),
            "raw_absolute_path".to_string(),
        ],
    }
}

fn git_path_identity(
    workspace_ref: &str,
    repo_path: &Path,
    filesystem_identity_ref: Option<String>,
    repository: Option<&RepositoryIdentity>,
    change: Option<&GitChange>,
) -> GitConflictPathIdentity {
    let path_label = repo_path.to_string_lossy().to_string();
    let path_id = sanitize_id(&path_label);
    let workspace_id = sanitize_id(workspace_ref);
    let path_truth_ref = format!("path.truth.git.conflict.{workspace_id}.{path_id}");
    let canonical_target_label =
        repository.map(|repo| format!("{}:{}", repo.worktree_ref, repo_path.to_string_lossy()));
    GitConflictPathIdentity {
        path_truth_ref,
        git_file_identity_ref: filesystem_identity_ref.clone(),
        filesystem_identity_ref,
        repo_relative_path: repo_path.to_path_buf(),
        original_repo_relative_path: change.and_then(|change| change.original_path.clone()),
        presentation_path_label: path_label,
        canonical_target_label,
    }
}

fn git_rollback_checkpoint(
    workspace_ref: &str,
    repo_path: &Path,
    repository: Option<&RepositoryIdentity>,
    explicit_checkpoint_ref: Option<String>,
    git_conflict_reported: bool,
) -> GitConflictRollbackCheckpoint {
    if !git_conflict_reported {
        return GitConflictRollbackCheckpoint {
            checkpoint_ref: None,
            checkpoint_source: "none".to_string(),
            rollback_available: false,
            rollback_path_class: "no_conflict_no_recovery_needed".to_string(),
            restore_command_id: None,
            no_write_committed: true,
            detail_label: "No conflict mutation has been admitted for this path.".to_string(),
        };
    }

    let operation_ref = repository
        .and_then(detect_history_operation_ref)
        .unwrap_or_else(|| "git.operation.conflict".to_string());
    let checkpoint_ref = explicit_checkpoint_ref.unwrap_or_else(|| {
        format!(
            "git.recovery.{}.{}.abort",
            sanitize_id(workspace_ref),
            sanitize_id(&repo_path.to_string_lossy())
        )
    });
    let restore_command_id = if operation_ref.contains("merge") {
        "cmd:git.merge.abort"
    } else if operation_ref.contains("rebase") {
        "cmd:git.rebase.abort"
    } else if operation_ref.contains("cherry-pick") {
        "cmd:git.cherry_pick.abort"
    } else if operation_ref.contains("revert") {
        "cmd:git.revert.abort"
    } else {
        "cmd:git.operation.abort"
    };

    GitConflictRollbackCheckpoint {
        checkpoint_ref: Some(checkpoint_ref),
        checkpoint_source: operation_ref,
        rollback_available: true,
        rollback_path_class: "abort_or_reflog_recovery".to_string(),
        restore_command_id: Some(restore_command_id.to_string()),
        no_write_committed: false,
        detail_label: "Git can abort the in-progress operation or use reflog recovery.".to_string(),
    }
}

fn external_rollback_checkpoint(
    input: GitExternalChangeHandoffInput<'_>,
) -> GitConflictRollbackCheckpoint {
    let checkpoint_ref = input.rollback_checkpoint_ref.map(str::to_string);
    let rollback_available = checkpoint_ref.is_some();
    GitConflictRollbackCheckpoint {
        checkpoint_ref,
        checkpoint_source: "vfs_compare_before_write".to_string(),
        rollback_available,
        rollback_path_class: if rollback_available {
            "restore_from_checkpoint"
        } else {
            "no_write_committed_cancel_or_save_as"
        }
        .to_string(),
        restore_command_id: rollback_available
            .then(|| "cmd:history.restore_checkpoint".to_string()),
        no_write_committed: true,
        detail_label: if rollback_available {
            "A checkpoint is available; no overwrite has been committed.".to_string()
        } else {
            "No overwrite has been committed; cancel, merge, reload, or save as.".to_string()
        },
    }
}

fn git_safe_actions(
    git_conflict_reported: bool,
    checkpoint: &GitConflictRollbackCheckpoint,
) -> Vec<GitConflictSafeAction> {
    if !git_conflict_reported {
        return vec![
            GitConflictSafeAction::OpenGitStatus,
            GitConflictSafeAction::Cancel,
        ];
    }
    let mut actions = vec![
        GitConflictSafeAction::OpenMergeResolver,
        GitConflictSafeAction::OpenCompare,
        GitConflictSafeAction::OpenGitStatus,
    ];
    if checkpoint.rollback_available {
        push_unique(&mut actions, GitConflictSafeAction::AbortGitOperation);
        push_unique(&mut actions, GitConflictSafeAction::RestoreCheckpoint);
    }
    push_unique(&mut actions, GitConflictSafeAction::Postpone);
    actions
}

fn external_safe_actions(record: &ExternalChangeCompareRecord) -> Vec<GitConflictSafeAction> {
    let mut actions = Vec::new();
    for action in &record.resolution_actions {
        push_unique(
            &mut actions,
            GitConflictSafeAction::from_vfs_action(*action),
        );
    }
    if record.review_required {
        push_unique(&mut actions, GitConflictSafeAction::Postpone);
    }
    actions
}

fn push_unique(actions: &mut Vec<GitConflictSafeAction>, action: GitConflictSafeAction) {
    if !actions.contains(&action) {
        actions.push(action);
    }
}

fn detect_history_operation_ref(repository: &RepositoryIdentity) -> Option<String> {
    let candidates = [
        ("MERGE_HEAD", "git.operation.merge"),
        ("REBASE_HEAD", "git.operation.rebase"),
        ("CHERRY_PICK_HEAD", "git.operation.cherry-pick"),
        ("REVERT_HEAD", "git.operation.revert"),
    ];
    candidates
        .iter()
        .find(|(file, _)| repository.git_dir.join(file).exists())
        .map(|(_, operation)| (*operation).to_string())
}

fn count_conflict_markers(path: &Path) -> u32 {
    let Ok(text) = std::fs::read_to_string(path) else {
        return 0;
    };
    text.lines()
        .filter(|line| line.starts_with("<<<<<<< "))
        .count() as u32
}

fn normalize_requested_path(path: &Path, repo_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.strip_prefix(repo_root)
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| path.to_path_buf())
    } else {
        path.to_path_buf()
    }
}

fn handoff_ref(workspace_ref: &str, source: GitConflictDivergenceSource, path: &Path) -> String {
    format!(
        "git.conflict.handoff.{}.{}.{}",
        sanitize_id(workspace_ref),
        source.as_str(),
        sanitize_id(&path.to_string_lossy())
    )
}

fn sanitize_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}
