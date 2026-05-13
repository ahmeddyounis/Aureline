//! Shell change-list projections derived from canonical Git status snapshots.
//!
//! This module owns the first source-control change-list surface in the shell.
//! It consumes [`aureline_git::GitStatusSnapshot`] directly, separates staged
//! and unstaged rows, applies a bounded virtualized window per group, and
//! projects one file-state chip vocabulary for shell rows, editor tabs, and
//! review entry points.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use aureline_git::{
    ChangeKind, ConsumerProjectionBundle, GitChange, GitServiceState, GitStatusSnapshot,
};

/// Stable record-kind tag for [`GitChangeListSurfaceBundle`].
pub const GIT_CHANGE_LIST_SURFACE_BUNDLE_RECORD_KIND: &str = "git_change_list_surface_bundle";

/// Stable record-kind tag for [`GitChangeListGroup`].
pub const GIT_CHANGE_LIST_GROUP_RECORD_KIND: &str = "git_change_list_group";

/// Stable record-kind tag for [`GitFileStateChip`].
pub const GIT_FILE_STATE_CHIP_RECORD_KIND: &str = "git_file_state_chip";

const GIT_CHANGE_LIST_SURFACE_BUNDLE_SCHEMA_VERSION: u32 = 1;
const GIT_CHANGE_LIST_GROUP_SCHEMA_VERSION: u32 = 1;
const GIT_FILE_STATE_CHIP_SCHEMA_VERSION: u32 = 1;
const DEFAULT_VISIBLE_ROW_LIMIT: usize = 200;

/// Top-level change-list group rendered by the source-control surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitChangeGroupKind {
    /// Index-visible changes that can be committed from the staged group.
    Staged,
    /// Worktree-visible changes, untracked files, and unresolved conflicts.
    Unstaged,
}

impl GitChangeGroupKind {
    /// Stable group token used in records, fixtures, and command routing.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Staged => "staged",
            Self::Unstaged => "unstaged",
        }
    }

    /// Human-readable group label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Staged => "Staged changes",
            Self::Unstaged => "Unstaged changes",
        }
    }

    /// Stable command id that opens this group's review surface.
    pub const fn open_review_command_id(self) -> &'static str {
        match self {
            Self::Staged => "cmd:git.review_staged_changes",
            Self::Unstaged => "cmd:git.review_unstaged_changes",
        }
    }
}

/// File-state token shared by shell rows, editor tabs, and review entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitFileStateToken {
    /// Tracked path content changed.
    Modified,
    /// Path was added.
    Added,
    /// Path was deleted.
    Deleted,
    /// Path was renamed.
    Renamed,
    /// Path was copied.
    Copied,
    /// Path changed file type.
    TypeChanged,
    /// Path is not tracked by Git.
    Untracked,
    /// Path has unresolved conflict state.
    Conflicted,
    /// Path is ignored by Git.
    Ignored,
}

impl GitFileStateToken {
    /// Returns all vocabulary tokens in stable display order.
    pub const fn all() -> [Self; 9] {
        [
            Self::Modified,
            Self::Added,
            Self::Deleted,
            Self::Renamed,
            Self::Copied,
            Self::TypeChanged,
            Self::Untracked,
            Self::Conflicted,
            Self::Ignored,
        ]
    }

    /// Stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Renamed => "renamed",
            Self::Copied => "copied",
            Self::TypeChanged => "type_changed",
            Self::Untracked => "untracked",
            Self::Conflicted => "conflicted",
            Self::Ignored => "ignored",
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Modified => "Modified",
            Self::Added => "Added",
            Self::Deleted => "Deleted",
            Self::Renamed => "Renamed",
            Self::Copied => "Copied",
            Self::TypeChanged => "Type changed",
            Self::Untracked => "Untracked",
            Self::Conflicted => "Conflict",
            Self::Ignored => "Ignored",
        }
    }

    /// Design-system tone token for the chip.
    pub const fn tone_token(self) -> &'static str {
        match self {
            Self::Modified => "accent",
            Self::Added => "positive",
            Self::Deleted => "danger",
            Self::Renamed | Self::Copied | Self::TypeChanged => "attention",
            Self::Untracked => "neutral",
            Self::Conflicted => "critical",
            Self::Ignored => "muted",
        }
    }
}

/// Surface that renders a file-state chip from the shared vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitFileStateSurface {
    /// Source-control shell change-list row.
    ShellChangeList,
    /// Open editor tab or tab overflow row.
    EditorTab,
    /// Review workspace entry point.
    ReviewEntry,
}

impl GitFileStateSurface {
    /// Stable surface token used in chip records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChangeList => "shell_change_list",
            Self::EditorTab => "editor_tab",
            Self::ReviewEntry => "review_entry",
        }
    }

    /// Human-readable surface label for support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShellChangeList => "Shell change list",
            Self::EditorTab => "Editor tab",
            Self::ReviewEntry => "Review entry",
        }
    }
}

/// File-state chip projected for one shell, editor, or review surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitFileStateChip {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable chip identity.
    pub chip_id: String,
    /// Surface that renders this chip.
    pub surface_token: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Stable file-state token shared by every surface.
    pub state_token: String,
    /// Human-readable chip label.
    pub label: String,
    /// Design-system tone token.
    pub tone_token: String,
    /// Tooltip text naming the local Git authority.
    pub tooltip: String,
}

impl GitFileStateChip {
    /// Materializes a chip for `surface` from one shared file-state token.
    pub fn from_token(surface: GitFileStateSurface, token: GitFileStateToken) -> Self {
        Self {
            record_kind: GIT_FILE_STATE_CHIP_RECORD_KIND.to_string(),
            schema_version: GIT_FILE_STATE_CHIP_SCHEMA_VERSION,
            chip_id: format!(
                "chip.git.file_state.{}.{}",
                surface.as_str(),
                token.as_str()
            ),
            surface_token: surface.as_str().to_string(),
            surface_label: surface.label().to_string(),
            state_token: token.as_str().to_string(),
            label: token.label().to_string(),
            tone_token: token.tone_token().to_string(),
            tooltip: format!("{} from local Git status", token.label()),
        }
    }
}

/// Cross-surface chip set for one file-state token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitFileStateChipSet {
    /// Shell change-list chip.
    pub shell: GitFileStateChip,
    /// Editor tab chip.
    pub editor_tab: GitFileStateChip,
    /// Review entry-point chip.
    pub review_entry: GitFileStateChip,
}

impl GitFileStateChipSet {
    /// Materializes shell, editor-tab, and review-entry chips from one token.
    pub fn from_token(token: GitFileStateToken) -> Self {
        Self {
            shell: GitFileStateChip::from_token(GitFileStateSurface::ShellChangeList, token),
            editor_tab: GitFileStateChip::from_token(GitFileStateSurface::EditorTab, token),
            review_entry: GitFileStateChip::from_token(GitFileStateSurface::ReviewEntry, token),
        }
    }

    /// Returns true when all surfaces carry the same file-state token.
    pub fn uses_single_state_token(&self) -> bool {
        self.shell.state_token == self.editor_tab.state_token
            && self.shell.state_token == self.review_entry.state_token
    }
}

/// Requested virtualized window for staged and unstaged groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChangeListViewport {
    /// Maximum number of rows to materialize per group.
    pub row_limit: usize,
    /// Offset into the staged group.
    pub staged_offset: usize,
    /// Offset into the unstaged group.
    pub unstaged_offset: usize,
}

impl Default for GitChangeListViewport {
    fn default() -> Self {
        Self {
            row_limit: DEFAULT_VISIBLE_ROW_LIMIT,
            staged_offset: 0,
            unstaged_offset: 0,
        }
    }
}

impl GitChangeListViewport {
    /// Builds a viewport with one row limit and independent group offsets.
    pub const fn new(row_limit: usize, staged_offset: usize, unstaged_offset: usize) -> Self {
        Self {
            row_limit,
            staged_offset,
            unstaged_offset,
        }
    }

    fn limit(self) -> usize {
        self.row_limit.max(1)
    }

    fn offset_for(self, group: GitChangeGroupKind) -> usize {
        match group {
            GitChangeGroupKind::Staged => self.staged_offset,
            GitChangeGroupKind::Unstaged => self.unstaged_offset,
        }
    }
}

/// Virtualization metadata for one rendered change-list group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChangeListVirtualizationState {
    /// Requested row limit after applying the minimum of one row.
    pub row_limit: usize,
    /// Requested starting offset clamped to the group bounds.
    pub offset: usize,
    /// Total row count in the group.
    pub total_rows: usize,
    /// Number of rows materialized in this window.
    pub visible_rows: usize,
    /// Number of rows hidden before this window.
    pub hidden_before_count: usize,
    /// Number of rows hidden after this window.
    pub hidden_after_count: usize,
    /// True when at least one group row is outside the materialized window.
    pub virtualized: bool,
    /// True when the entire group fits in the materialized window.
    pub complete_window: bool,
    /// Stable anchor used by scroll restoration and support exports.
    pub window_anchor_ref: String,
}

impl GitChangeListVirtualizationState {
    fn from_counts(
        workspace_ref: &str,
        group: GitChangeGroupKind,
        total_rows: usize,
        offset: usize,
        row_limit: usize,
        visible_rows: usize,
    ) -> Self {
        let clamped_offset = offset.min(total_rows);
        let hidden_after_count = total_rows.saturating_sub(clamped_offset + visible_rows);
        let virtualized = clamped_offset > 0 || hidden_after_count > 0;
        Self {
            row_limit,
            offset: clamped_offset,
            total_rows,
            visible_rows,
            hidden_before_count: clamped_offset,
            hidden_after_count,
            virtualized,
            complete_window: !virtualized,
            window_anchor_ref: format!(
                "git.change_list.window.{}.{}.{}",
                sanitize_id(workspace_ref),
                group.as_str(),
                clamped_offset
            ),
        }
    }
}

/// One visible row in a staged or unstaged change-list group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChangeListRow {
    /// Stable row identity.
    pub row_ref: String,
    /// Zero-based position in the full group, not only the visible window.
    pub group_position: usize,
    /// Group token copied from [`GitChangeGroupKind`].
    pub group_token: String,
    /// File-state token shared by every chip in [`Self::chips`].
    pub file_state_token: String,
    /// Path reported by Git relative to the repository root.
    pub path: PathBuf,
    /// Display-safe path label.
    pub display_path: String,
    /// Original path for rename or copy rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<PathBuf>,
    /// Two-character Git status code from the source snapshot.
    pub status_code: String,
    /// Cross-surface chips projected from one file-state vocabulary.
    pub chips: GitFileStateChipSet,
    /// Command id that opens a local diff for this path and group.
    pub opens_diff_command_id: String,
    /// Review entry ref that quotes the same row state.
    pub review_entry_ref: String,
    /// Editor tab chip ref that quotes the same row state.
    pub editor_tab_chip_ref: String,
}

/// One staged or unstaged group in the source-control change list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChangeListGroup {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable group token.
    pub group_token: String,
    /// Human-readable group header.
    pub header_label: String,
    /// Command id that opens a review surface scoped to this group.
    pub open_review_command_id: String,
    /// Total row count before virtualization.
    pub total_count: usize,
    /// Number of rows materialized in this group.
    pub visible_count: usize,
    /// Virtualization metadata for this group.
    pub virtualization: GitChangeListVirtualizationState,
    /// Visible rows in stable path order.
    pub rows: Vec<GitChangeListRow>,
}

/// Source-control change-list surface projected from one Git status snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChangeListSurfaceBundle {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity copied from the source snapshot.
    pub workspace_ref: String,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Coarse Git service-state token copied from the source snapshot.
    pub service_state_token: String,
    /// True when the source Git snapshot was degraded or unavailable.
    pub current_claim_narrowed: bool,
    /// Shell source-control surface ref.
    pub shell_surface_ref: String,
    /// Editor-tab projection surface ref.
    pub editor_tabs_surface_ref: String,
    /// Review-entry projection surface ref.
    pub review_entry_surface_ref: String,
    /// Shared file-state vocabulary in stable token order.
    pub file_state_vocabulary: Vec<GitFileStateChipSet>,
    /// Staged and unstaged groups in stable order.
    pub groups: Vec<GitChangeListGroup>,
}

impl GitChangeListSurfaceBundle {
    /// Materializes the shell change-list surface from a canonical snapshot.
    pub fn from_snapshot(
        generated_at: impl Into<String>,
        snapshot: &GitStatusSnapshot,
        viewport: GitChangeListViewport,
    ) -> Self {
        let generated_at = generated_at.into();
        let truth_source_ref =
            ConsumerProjectionBundle::from_snapshot(generated_at.clone(), snapshot)
                .truth_source_ref;
        let groups = [GitChangeGroupKind::Staged, GitChangeGroupKind::Unstaged]
            .into_iter()
            .map(|group| materialize_group(snapshot, group, viewport))
            .collect();

        Self {
            record_kind: GIT_CHANGE_LIST_SURFACE_BUNDLE_RECORD_KIND.to_string(),
            schema_version: GIT_CHANGE_LIST_SURFACE_BUNDLE_SCHEMA_VERSION,
            generated_at,
            workspace_ref: snapshot.workspace_ref.clone(),
            truth_source_ref,
            service_state_token: snapshot.service_state.as_str().to_string(),
            current_claim_narrowed: snapshot.service_state != GitServiceState::Current,
            shell_surface_ref: format!(
                "surface.git.change_list.shell.{}",
                sanitize_id(&snapshot.workspace_ref)
            ),
            editor_tabs_surface_ref: format!(
                "surface.git.change_list.editor_tabs.{}",
                sanitize_id(&snapshot.workspace_ref)
            ),
            review_entry_surface_ref: format!(
                "surface.git.change_list.review.{}",
                sanitize_id(&snapshot.workspace_ref)
            ),
            file_state_vocabulary: GitFileStateToken::all()
                .into_iter()
                .map(GitFileStateChipSet::from_token)
                .collect(),
            groups,
        }
    }

    /// Returns the materialized group for `kind`.
    pub fn group(&self, kind: GitChangeGroupKind) -> Option<&GitChangeListGroup> {
        self.groups
            .iter()
            .find(|group| group.group_token == kind.as_str())
    }

    /// True when every visible row and vocabulary entry reuse one chip token.
    pub fn has_shared_file_state_vocabulary(&self) -> bool {
        self.file_state_vocabulary
            .iter()
            .all(GitFileStateChipSet::uses_single_state_token)
            && self.groups.iter().all(|group| {
                group
                    .rows
                    .iter()
                    .all(|row| row.chips.uses_single_state_token())
            })
    }
}

fn materialize_group(
    snapshot: &GitStatusSnapshot,
    group: GitChangeGroupKind,
    viewport: GitChangeListViewport,
) -> GitChangeListGroup {
    let mut rows: Vec<_> = snapshot
        .changes
        .iter()
        .filter_map(|change| row_from_change(snapshot, group, change))
        .collect();
    rows.sort_by(|left, right| {
        left.display_path
            .cmp(&right.display_path)
            .then(left.file_state_token.cmp(&right.file_state_token))
            .then(left.status_code.cmp(&right.status_code))
    });
    for (position, row) in rows.iter_mut().enumerate() {
        row.group_position = position;
    }

    let total_count = rows.len();
    let row_limit = viewport.limit();
    let offset = viewport.offset_for(group).min(total_count);
    let rows: Vec<_> = rows.into_iter().skip(offset).take(row_limit).collect();
    let visible_count = rows.len();
    let virtualization = GitChangeListVirtualizationState::from_counts(
        &snapshot.workspace_ref,
        group,
        total_count,
        offset,
        row_limit,
        visible_count,
    );

    GitChangeListGroup {
        record_kind: GIT_CHANGE_LIST_GROUP_RECORD_KIND.to_string(),
        schema_version: GIT_CHANGE_LIST_GROUP_SCHEMA_VERSION,
        group_token: group.as_str().to_string(),
        header_label: group.label().to_string(),
        open_review_command_id: group.open_review_command_id().to_string(),
        total_count,
        visible_count,
        virtualization,
        rows,
    }
}

fn row_from_change(
    snapshot: &GitStatusSnapshot,
    group: GitChangeGroupKind,
    change: &GitChange,
) -> Option<GitChangeListRow> {
    let token = state_token_for_group(change, group)?;
    let display_path = change.path.to_string_lossy().to_string();
    let row_ref = format!(
        "git.change.row.{}.{}.{}.{}",
        sanitize_id(&snapshot.workspace_ref),
        group.as_str(),
        sanitize_id(&display_path),
        token.as_str()
    );
    let chips = GitFileStateChipSet::from_token(token);

    Some(GitChangeListRow {
        row_ref: row_ref.clone(),
        group_position: 0,
        group_token: group.as_str().to_string(),
        file_state_token: token.as_str().to_string(),
        path: change.path.clone(),
        display_path,
        original_path: change.original_path.clone(),
        status_code: change.status_code.clone(),
        opens_diff_command_id: format!("cmd:git.diff.open_{}", group.as_str()),
        review_entry_ref: format!("review.entry.{row_ref}"),
        editor_tab_chip_ref: chips.editor_tab.chip_id.clone(),
        chips,
    })
}

fn state_token_for_group(
    change: &GitChange,
    group: GitChangeGroupKind,
) -> Option<GitFileStateToken> {
    if change.is_conflicted {
        return (group == GitChangeGroupKind::Unstaged).then_some(GitFileStateToken::Conflicted);
    }
    match group {
        GitChangeGroupKind::Staged if change.is_staged => status_side_token(change, 0),
        GitChangeGroupKind::Unstaged
            if change.is_unstaged
                || matches!(
                    change.change_kind,
                    ChangeKind::Untracked | ChangeKind::Ignored
                ) =>
        {
            status_side_token(change, 1)
        }
        _ => None,
    }
}

fn status_side_token(change: &GitChange, side: usize) -> Option<GitFileStateToken> {
    let status_char = change.status_code.chars().nth(side).unwrap_or('.');
    match status_char {
        'M' => Some(GitFileStateToken::Modified),
        'A' => Some(GitFileStateToken::Added),
        'D' => Some(GitFileStateToken::Deleted),
        'R' => Some(GitFileStateToken::Renamed),
        'C' => Some(GitFileStateToken::Copied),
        'T' => Some(GitFileStateToken::TypeChanged),
        'U' => Some(GitFileStateToken::Conflicted),
        '?' => Some(GitFileStateToken::Untracked),
        '!' => Some(GitFileStateToken::Ignored),
        _ => token_from_change_kind(change.change_kind),
    }
}

fn token_from_change_kind(kind: ChangeKind) -> Option<GitFileStateToken> {
    match kind {
        ChangeKind::Modified => Some(GitFileStateToken::Modified),
        ChangeKind::Added => Some(GitFileStateToken::Added),
        ChangeKind::Deleted => Some(GitFileStateToken::Deleted),
        ChangeKind::TypeChanged => Some(GitFileStateToken::TypeChanged),
        ChangeKind::Renamed => Some(GitFileStateToken::Renamed),
        ChangeKind::Copied => Some(GitFileStateToken::Copied),
        ChangeKind::Untracked => Some(GitFileStateToken::Untracked),
        ChangeKind::Ignored => Some(GitFileStateToken::Ignored),
        ChangeKind::Conflict => Some(GitFileStateToken::Conflicted),
    }
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
    if out.is_empty() {
        "root".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_git::{ChangeDiscovery, ChangeSummary, HeadIdentity, RepositoryIdentity};

    fn snapshot(changes: Vec<GitChange>) -> GitStatusSnapshot {
        GitStatusSnapshot {
            record_kind: aureline_git::GIT_STATUS_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: 1,
            service_ref: "service.git.status.alpha".to_string(),
            workspace_ref: "workspace.unit.change-list".to_string(),
            requested_root: PathBuf::from("/workspace/unit"),
            repository: Some(RepositoryIdentity {
                repo_ref: "repo.local.unit".to_string(),
                worktree_ref: "worktree.local.unit".to_string(),
                repo_label: "unit".to_string(),
                repo_root: PathBuf::from("/workspace/unit"),
                git_dir: PathBuf::from("/workspace/unit/.git"),
                common_dir: PathBuf::from("/workspace/unit/.git"),
            }),
            head: HeadIdentity {
                state: aureline_git::BranchState::Attached,
                branch_label: Some("main".to_string()),
                branch_ref: Some("refs/heads/main".to_string()),
                head_oid: Some("1111111111111111111111111111111111111111".to_string()),
                head_short_oid: Some("1111111".to_string()),
                upstream: None,
                ahead: None,
                behind: None,
            },
            service_state: GitServiceState::Current,
            degraded_reason: None,
            discovery: ChangeDiscovery {
                status_available: true,
                branch_identity_available: true,
                change_list_available: true,
                current_claim_narrowed: false,
                coverage_label: "unit status".to_string(),
            },
            change_summary: ChangeSummary::from_changes(&changes),
            changes,
            consumer_refs: Vec::new(),
            observed_at: "mono:unit".to_string(),
        }
    }

    fn change(path: &str, code: &str, kind: ChangeKind, staged: bool, unstaged: bool) -> GitChange {
        GitChange {
            path: PathBuf::from(path),
            original_path: None,
            status_code: code.to_string(),
            change_kind: kind,
            is_staged: staged,
            is_unstaged: unstaged,
            is_conflicted: false,
        }
    }

    #[test]
    fn groups_dual_state_paths_without_chip_drift() {
        let snapshot = snapshot(vec![
            change("src/app.rs", "MM", ChangeKind::Modified, true, true),
            change("src/new.rs", "A.", ChangeKind::Added, true, false),
            change("docs/notes.md", "??", ChangeKind::Untracked, false, false),
        ]);
        let bundle = GitChangeListSurfaceBundle::from_snapshot(
            "mono:bundle",
            &snapshot,
            GitChangeListViewport::default(),
        );

        let staged = bundle.group(GitChangeGroupKind::Staged).expect("staged");
        let unstaged = bundle
            .group(GitChangeGroupKind::Unstaged)
            .expect("unstaged");
        assert_eq!(staged.total_count, 2);
        assert_eq!(unstaged.total_count, 2);
        assert!(bundle.has_shared_file_state_vocabulary());
        assert!(staged
            .rows
            .iter()
            .any(|row| row.display_path == "src/app.rs" && row.file_state_token == "modified"));
        assert!(unstaged
            .rows
            .iter()
            .any(|row| row.display_path == "src/app.rs" && row.file_state_token == "modified"));
    }

    #[test]
    fn viewport_materializes_bounded_windows() {
        let changes: Vec<_> = (0..250)
            .map(|index| {
                change(
                    &format!("src/file_{index:04}.rs"),
                    "M.",
                    ChangeKind::Modified,
                    true,
                    false,
                )
            })
            .collect();
        let snapshot = snapshot(changes);
        let bundle = GitChangeListSurfaceBundle::from_snapshot(
            "mono:bundle",
            &snapshot,
            GitChangeListViewport::new(25, 10, 0),
        );

        let staged = bundle.group(GitChangeGroupKind::Staged).expect("staged");
        assert_eq!(staged.total_count, 250);
        assert_eq!(staged.visible_count, 25);
        assert!(staged.virtualization.virtualized);
        assert_eq!(staged.virtualization.hidden_before_count, 10);
        assert_eq!(staged.rows[0].display_path, "src/file_0010.rs");
    }
}
