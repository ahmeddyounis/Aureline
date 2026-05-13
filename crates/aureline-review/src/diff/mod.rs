//! Diff-view packets for local Git review surfaces.
//!
//! The diff module consumes public change-list row facts and explicit fixture
//! hunk bodies. It keeps local Git compare-target truth, path identity, syntax
//! labels, suspicious-text warnings, and copy representation choices together
//! so shell, review, support, and reopen paths do not need private state.

use std::collections::BTreeSet;
use std::path::PathBuf;

use aureline_content_safety::{
    detect_suspicious_content, escape_for_safe_inspection, BodyPosture, RepresentationActionId,
    RepresentationClass, SuspiciousContentClass, SuspiciousFinding,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DiffOpenTarget`].
pub const DIFF_OPEN_TARGET_RECORD_KIND: &str = "diff_open_target";

/// Stable record-kind tag for [`DiffViewSurfacePacket`].
pub const DIFF_VIEW_SURFACE_PACKET_RECORD_KIND: &str = "diff_view_surface_packet";

/// Stable record-kind tag for [`DiffClosedSessionRecord`].
pub const DIFF_CLOSED_SESSION_RECORD_KIND: &str = "diff_closed_session_record";

/// Stable record-kind tag for [`DiffReopenProjection`].
pub const DIFF_REOPEN_PROJECTION_RECORD_KIND: &str = "diff_reopen_projection";

const DIFF_OPEN_TARGET_SCHEMA_VERSION: u32 = 1;
const DIFF_VIEW_SURFACE_SCHEMA_VERSION: u32 = 1;
const DIFF_CLOSED_SESSION_SCHEMA_VERSION: u32 = 1;
const DIFF_REOPEN_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Diff compare target selected by the invoking change-list group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffCompareTargetKind {
    /// Compare HEAD or base revision to the index side of a staged change.
    Staged,
    /// Compare the index side to the worktree side of an unstaged change.
    Unstaged,
    /// Compare a known base revision to the current worktree.
    WorkingTree,
}

impl DiffCompareTargetKind {
    /// Stable token used in records and command arguments.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Staged => "staged",
            Self::Unstaged => "unstaged",
            Self::WorkingTree => "working_tree",
        }
    }

    /// Human-readable compare target label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Staged => "Staged change",
            Self::Unstaged => "Unstaged change",
            Self::WorkingTree => "Working tree change",
        }
    }

    fn from_group_token(token: &str) -> Self {
        match token {
            "staged" => Self::Staged,
            "unstaged" => Self::Unstaged,
            _ => Self::WorkingTree,
        }
    }

    fn command_id(self) -> &'static str {
        match self {
            Self::Staged => "cmd:git.diff.open_staged",
            Self::Unstaged => "cmd:git.diff.open_unstaged",
            Self::WorkingTree => "cmd:git.diff.open_working_tree",
        }
    }

    fn base_label(self) -> &'static str {
        match self {
            Self::Staged => "HEAD",
            Self::Unstaged => "index",
            Self::WorkingTree => "base",
        }
    }

    fn head_label(self) -> &'static str {
        match self {
            Self::Staged => "index",
            Self::Unstaged | Self::WorkingTree => "worktree",
        }
    }
}

/// Unified or side-by-side diff layout selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffViewMode {
    /// One column with inline addition, deletion, and context rows.
    Unified,
    /// Two synchronized panes for base and head rows.
    SideBySide,
}

impl DiffViewMode {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unified => "unified",
            Self::SideBySide => "side_by_side",
        }
    }
}

/// Syntax or structure class available for a diffed file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffSyntaxClass {
    /// Rust source file.
    Rust,
    /// TypeScript source file.
    TypeScript,
    /// JavaScript source file.
    JavaScript,
    /// JSON or JSON-like structured document.
    Json,
    /// YAML structured document.
    Yaml,
    /// TOML structured document.
    Toml,
    /// Markdown document.
    Markdown,
    /// Plain text with no known syntax projection.
    PlainText,
}

impl DiffSyntaxClass {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
            Self::Markdown => "markdown",
            Self::PlainText => "plain_text",
        }
    }

    /// Human-readable renderer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Rust => "Rust syntax",
            Self::TypeScript => "TypeScript syntax",
            Self::JavaScript => "JavaScript syntax",
            Self::Json => "JSON structure",
            Self::Yaml => "YAML structure",
            Self::Toml => "TOML structure",
            Self::Markdown => "Markdown structure",
            Self::PlainText => "Plain text",
        }
    }

    fn from_path_and_language(path: &std::path::Path, language_id: Option<&str>) -> Self {
        if let Some(language_id) = language_id {
            if let Some(class) = Self::from_token(language_id) {
                return class;
            }
        }
        path.extension()
            .and_then(|extension| extension.to_str())
            .and_then(Self::from_token)
            .unwrap_or(Self::PlainText)
    }

    fn from_token(token: &str) -> Option<Self> {
        match token.to_ascii_lowercase().as_str() {
            "rs" | "rust" => Some(Self::Rust),
            "ts" | "tsx" | "typescript" => Some(Self::TypeScript),
            "js" | "jsx" | "javascript" => Some(Self::JavaScript),
            "json" | "jsonc" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            "md" | "markdown" => Some(Self::Markdown),
            "txt" | "text" | "plain_text" => Some(Self::PlainText),
            _ => None,
        }
    }

    const fn has_structure(self) -> bool {
        matches!(self, Self::Json | Self::Yaml | Self::Toml | Self::Markdown)
    }
}

/// Diff row kind inside one hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffLineKind {
    /// Unchanged context row.
    Context,
    /// Added row on the head side.
    Addition,
    /// Deleted row on the base side.
    Deletion,
}

impl DiffLineKind {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::Addition => "addition",
            Self::Deletion => "deletion",
        }
    }

    /// Single-character marker used by unified diff rendering.
    pub const fn marker(self) -> &'static str {
        match self {
            Self::Context => " ",
            Self::Addition => "+",
            Self::Deletion => "-",
        }
    }
}

/// Copy representation offered by a diff row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffCopyRepresentation {
    /// Exact source text for the selected line.
    RawSource,
    /// Plain text without syntax overlay or diff chrome.
    PlainText,
    /// Rendered line plus hunk/path context for review notes.
    RenderedContext,
    /// Source text with suspicious codepoints escaped.
    EscapedSource,
}

impl DiffCopyRepresentation {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSource => "raw_source",
            Self::PlainText => "plain_text",
            Self::RenderedContext => "rendered_context",
            Self::EscapedSource => "escaped_source",
        }
    }

    fn transfer_class(self) -> &'static str {
        match self {
            Self::RawSource | Self::PlainText => RepresentationClass::Raw.as_str(),
            Self::RenderedContext => RepresentationClass::Rendered.as_str(),
            Self::EscapedSource => RepresentationClass::Escaped.as_str(),
        }
    }

    fn body_posture(self) -> &'static str {
        match self {
            Self::RawSource | Self::PlainText => BodyPosture::ExactSourceBytes.as_str(),
            Self::RenderedContext => BodyPosture::RenderedView.as_str(),
            Self::EscapedSource => BodyPosture::EscapedSourceText.as_str(),
        }
    }
}

/// Public diff-open target emitted by a source-control change-list row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffOpenTarget {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Public command id invoked by shell, palette, or automation.
    pub command_id: String,
    /// Surface ref opened by the command.
    pub opens_surface_ref: String,
    /// Change-list row or equivalent public launcher ref.
    pub launch_source_ref: String,
    /// Workspace identity copied from the source status snapshot.
    pub workspace_ref: String,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Group token that selected the compare target.
    pub group_token: String,
    /// Path reported by Git relative to the repository root.
    pub path: PathBuf,
    /// Original path for rename or copy rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<PathBuf>,
    /// Two-character Git status code from the source snapshot.
    pub status_code: String,
    /// Shared file-state token from the change-list projection.
    pub file_state_token: String,
    /// Compare target kind selected from the group token.
    pub compare_target_kind: DiffCompareTargetKind,
    /// Human-readable compare target label.
    pub compare_target_label: String,
    /// Exact target label shown on rows and reopen records.
    pub exact_target_label: String,
    /// Path-truth ref that later diff rows quote.
    pub path_truth_ref: String,
}

impl DiffOpenTarget {
    /// Builds a diff-open target from public change-list row fields.
    #[allow(clippy::too_many_arguments)]
    pub fn from_change_list_row_parts(
        workspace_ref: &str,
        truth_source_ref: &str,
        row_ref: &str,
        group_token: &str,
        path: impl Into<PathBuf>,
        original_path: Option<PathBuf>,
        status_code: &str,
        file_state_token: &str,
    ) -> Self {
        let path = path.into();
        let compare_target_kind = DiffCompareTargetKind::from_group_token(group_token);
        let path_label = path.to_string_lossy();
        let path_id = sanitize_id(&path_label);
        let workspace_id = sanitize_id(workspace_ref);
        let opens_surface_ref = format!(
            "surface.git.diff.{}.{}.{}",
            workspace_id,
            compare_target_kind.as_str(),
            path_id
        );
        let path_truth_ref = format!("path.truth.git.diff.{workspace_id}.{path_id}");
        let exact_target_label = format!(
            "{} · {} to {} · {}",
            compare_target_kind.label(),
            compare_target_kind.base_label(),
            compare_target_kind.head_label(),
            path_label
        );

        Self {
            record_kind: DIFF_OPEN_TARGET_RECORD_KIND.to_string(),
            schema_version: DIFF_OPEN_TARGET_SCHEMA_VERSION,
            command_id: compare_target_kind.command_id().to_string(),
            opens_surface_ref,
            launch_source_ref: row_ref.to_string(),
            workspace_ref: workspace_ref.to_string(),
            truth_source_ref: truth_source_ref.to_string(),
            group_token: group_token.to_string(),
            path,
            original_path,
            status_code: status_code.to_string(),
            file_state_token: file_state_token.to_string(),
            compare_target_kind,
            compare_target_label: compare_target_kind.label().to_string(),
            exact_target_label,
            path_truth_ref,
        }
    }
}

/// Exact path identity shown by the diff viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffPathTruth {
    /// Stable path-truth ref quoted by every protected row.
    pub path_truth_ref: String,
    /// Path label the user selected or saw in the change list.
    pub presentation_path: PathBuf,
    /// Git repository-relative path for the diff target.
    pub repo_relative_path: PathBuf,
    /// Resolved mutable target path in the local worktree.
    pub canonical_target_path: PathBuf,
    /// Original repository-relative path for rename/copy rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<PathBuf>,
    /// Repository root used to resolve the mutable target.
    pub repo_root: PathBuf,
    /// Logical root or workspace root identity.
    pub logical_root_ref: String,
    /// Local worktree identity.
    pub worktree_ref: String,
    /// Visible path label repeated on protected rows.
    pub path_label: String,
    /// User-facing target location label.
    pub target_location_label: String,
    /// Whether the UI can offer a reveal-on-host action.
    pub reveal_on_host_available: bool,
}

impl DiffPathTruth {
    fn from_open_target(
        open_target: &DiffOpenTarget,
        repo_root: PathBuf,
        logical_root_ref: String,
        worktree_ref: String,
    ) -> Self {
        let canonical_target_path = if open_target.path.is_absolute() {
            open_target.path.clone()
        } else {
            repo_root.join(&open_target.path)
        };
        let path_label = open_target.path.to_string_lossy().to_string();
        let target_location_label = canonical_target_path.to_string_lossy().to_string();

        Self {
            path_truth_ref: open_target.path_truth_ref.clone(),
            presentation_path: open_target.path.clone(),
            repo_relative_path: open_target.path.clone(),
            canonical_target_path,
            original_path: open_target.original_path.clone(),
            repo_root,
            logical_root_ref,
            worktree_ref,
            path_label,
            target_location_label,
            reveal_on_host_available: true,
        }
    }
}

/// Exact Git compare target shown by the diff viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffCompareTarget {
    /// Stable compare-target ref quoted by every protected row.
    pub compare_target_ref: String,
    /// Compare target token.
    pub target_kind_token: String,
    /// Base side label.
    pub base_label: String,
    /// Head side label.
    pub head_label: String,
    /// Optional base revision ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_revision_ref: Option<String>,
    /// Optional head revision ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_ref: Option<String>,
    /// Exact target label shown on protected rows.
    pub exact_target_label: String,
    /// Local diff authority token.
    pub local_diff_authority: String,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
}

impl DiffCompareTarget {
    fn from_open_target(open_target: &DiffOpenTarget) -> Self {
        let path_label = open_target.path.to_string_lossy();
        let compare_target_ref = format!(
            "git.diff.target.{}.{}.{}",
            sanitize_id(&open_target.workspace_ref),
            open_target.compare_target_kind.as_str(),
            sanitize_id(&path_label)
        );
        Self {
            compare_target_ref,
            target_kind_token: open_target.compare_target_kind.as_str().to_string(),
            base_label: open_target.compare_target_kind.base_label().to_string(),
            head_label: open_target.compare_target_kind.head_label().to_string(),
            base_revision_ref: None,
            head_revision_ref: None,
            exact_target_label: open_target.exact_target_label.clone(),
            local_diff_authority: "authoritative_local_git".to_string(),
            truth_source_ref: open_target.truth_source_ref.clone(),
        }
    }
}

/// Syntax or structure projection for a diffed file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffSyntaxProjection {
    /// Stable syntax projection ref.
    pub syntax_ref: String,
    /// Syntax class token.
    pub syntax_class_token: String,
    /// Human-readable syntax or structure label.
    pub syntax_label: String,
    /// Language id selected by the caller or path extension.
    pub language_id: String,
    /// Renderer posture for this alpha projection.
    pub renderer_kind: String,
    /// Whether structure labels are available for this file class.
    pub structure_available: bool,
    /// Whether syntax rendering is safe to show as a raw-text overlay.
    pub syntax_safe_rendering: bool,
}

impl DiffSyntaxProjection {
    fn from_path(path: &std::path::Path, language_id: Option<&str>, workspace_ref: &str) -> Self {
        let class = DiffSyntaxClass::from_path_and_language(path, language_id);
        Self {
            syntax_ref: format!(
                "git.diff.syntax.{}.{}",
                sanitize_id(workspace_ref),
                sanitize_id(&path.to_string_lossy())
            ),
            syntax_class_token: class.as_str().to_string(),
            syntax_label: class.label().to_string(),
            language_id: language_id
                .map(str::to_string)
                .unwrap_or_else(|| class.as_str().to_string()),
            renderer_kind: if class.has_structure() {
                "structure_aware_alpha".to_string()
            } else {
                "syntax_overlay_alpha".to_string()
            },
            structure_available: class.has_structure(),
            syntax_safe_rendering: true,
        }
    }
}

/// Input row used to materialize a diff line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffLineInput {
    /// Kind of row in a unified diff hunk.
    pub line_kind: DiffLineKind,
    /// Old-side line number when the row exists on the base side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_line_number: Option<u32>,
    /// New-side line number when the row exists on the head side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_line_number: Option<u32>,
    /// Exact source text without the unified diff marker.
    pub raw_text: String,
}

/// Input hunk used to materialize a diff hunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffHunkInput {
    /// Unified diff hunk header.
    pub hunk_header: String,
    /// First old-side line covered by the hunk.
    pub old_start: u32,
    /// Number of old-side lines covered by the hunk.
    pub old_lines: u32,
    /// First new-side line covered by the hunk.
    pub new_start: u32,
    /// Number of new-side lines covered by the hunk.
    pub new_lines: u32,
    /// Lines in hunk order.
    pub lines: Vec<DiffLineInput>,
}

/// Input file diff used by tests, CLI mirrors, and later Git backends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFileInput {
    /// Workspace identity copied from the source status snapshot.
    pub workspace_ref: String,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Repository root used to resolve path truth.
    pub repo_root: PathBuf,
    /// Logical root or workspace root identity.
    pub logical_root_ref: String,
    /// Local worktree identity.
    pub worktree_ref: String,
    /// Group token that selected the compare target.
    pub group_token: String,
    /// Path reported by Git relative to the repository root.
    pub path: PathBuf,
    /// Original path for rename or copy rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<PathBuf>,
    /// Two-character Git status code from the source snapshot.
    pub status_code: String,
    /// Language id when known before extension fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    /// Preferred diff view mode.
    pub view_mode: DiffViewMode,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Hunks in file order.
    pub hunks: Vec<DiffHunkInput>,
}

/// Suspicious-text cue attached to a diff line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffSuspiciousCue {
    /// Stable warning id on the diff row.
    pub warning_id: String,
    /// Detector finding id projected by the shared detector.
    pub detector_finding_id: String,
    /// Suspicious-content class token.
    pub content_class_token: String,
    /// Diff line row that owns this warning.
    pub line_ref: String,
    /// Continuity ref used by copy/export/reopen paths.
    pub continuity_ref: String,
    /// Warning label shown on the row.
    pub warning_label: String,
    /// Exact snippet matched by the detector.
    pub raw_snippet: String,
    /// Escaped snippet used by the safe copy path.
    pub escaped_snippet: String,
    /// Codepoints observed by the detector.
    pub matched_codepoints: Vec<u32>,
    /// Actions that must remain reachable from the warning.
    pub reveal_action_ids: Vec<String>,
}

/// Copy action attached to a diff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffCopyAction {
    /// Stable action id.
    pub action_id: String,
    /// Stable action ref unique to this row.
    pub action_ref: String,
    /// User-facing label naming the representation.
    pub label: String,
    /// Local diff copy representation token.
    pub representation_token: String,
    /// Shared representation policy class token.
    pub transfer_policy_representation_class: String,
    /// Shared representation policy body posture.
    pub body_posture: String,
    /// Line or hunk-context scope token.
    pub scope_token: String,
    /// Whether this action includes path, target, or hunk context.
    pub includes_context: bool,
    /// Whether this is the safe path for suspicious text.
    pub safe_representation_path: bool,
    /// Warning ids preserved by this transfer action.
    pub attached_warning_refs: Vec<String>,
    /// Fields the UI must display near this action.
    pub required_visible_fields: Vec<String>,
}

impl DiffCopyAction {
    fn line_action(
        row_ref: &str,
        action_id: &str,
        label: &str,
        representation: DiffCopyRepresentation,
        includes_context: bool,
        safe_representation_path: bool,
        warning_refs: Vec<String>,
    ) -> Self {
        Self {
            action_id: action_id.to_string(),
            action_ref: format!("git.diff.copy.{}.{}", sanitize_id(row_ref), action_id),
            label: label.to_string(),
            representation_token: representation.as_str().to_string(),
            transfer_policy_representation_class: representation.transfer_class().to_string(),
            body_posture: representation.body_posture().to_string(),
            scope_token: if includes_context {
                "hunk_context"
            } else {
                "line"
            }
            .to_string(),
            includes_context,
            safe_representation_path,
            attached_warning_refs: warning_refs,
            required_visible_fields: vec![
                "representation_label".to_string(),
                "path_label".to_string(),
                "target_label".to_string(),
                "warning_state".to_string(),
            ],
        }
    }
}

/// One rendered line in a diff hunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffLineView {
    /// Stable row ref.
    pub row_ref: String,
    /// Zero-based row index inside the hunk.
    pub hunk_row_index: usize,
    /// Diff row kind token.
    pub line_kind: DiffLineKind,
    /// Unified diff marker.
    pub marker: String,
    /// Old-side line number when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_line_number: Option<u32>,
    /// New-side line number when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_line_number: Option<u32>,
    /// Exact source text without the diff marker.
    pub raw_text: String,
    /// Text shown in the syntax-safe diff renderer.
    pub rendered_text: String,
    /// Escaped safe-inspection text used by `copy_escaped`.
    pub escaped_text: String,
    /// Syntax class token copied from the file projection.
    pub syntax_class_token: String,
    /// Optional structure label inferred from the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure_label: Option<String>,
    /// Visible path label repeated on protected rows.
    pub path_label: String,
    /// Visible compare-target label repeated on protected rows.
    pub target_label: String,
    /// Path-truth ref quoted by this row.
    pub path_truth_ref: String,
    /// Compare-target ref quoted by this row.
    pub compare_target_ref: String,
    /// Suspicious-text cues attached to this row.
    pub suspicious_cues: Vec<DiffSuspiciousCue>,
    /// Representation-labeled copy actions.
    pub copy_actions: Vec<DiffCopyAction>,
}

impl DiffLineView {
    /// Returns true when path and compare target truth are visible.
    pub fn has_visible_path_and_target_truth(&self) -> bool {
        !self.path_label.trim().is_empty()
            && !self.target_label.trim().is_empty()
            && !self.path_truth_ref.trim().is_empty()
            && !self.compare_target_ref.trim().is_empty()
    }

    /// Returns true when suspicious rows expose raw and escaped copy paths.
    pub fn suspicious_safe_copy_is_visible(&self) -> bool {
        if self.suspicious_cues.is_empty() {
            return true;
        }
        let action_ids: BTreeSet<_> = self
            .copy_actions
            .iter()
            .map(|action| action.action_id.as_str())
            .collect();
        action_ids.contains(RepresentationActionId::CopyRaw.as_str())
            && action_ids.contains(RepresentationActionId::CopyEscaped.as_str())
            && self
                .copy_actions
                .iter()
                .any(|action| action.safe_representation_path)
    }

    /// Returns the local copy representation tokens visible on the row.
    pub fn copy_representation_tokens(&self) -> BTreeSet<&str> {
        self.copy_actions
            .iter()
            .map(|action| action.representation_token.as_str())
            .collect()
    }
}

/// One rendered hunk in a diff surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffHunkView {
    /// Stable hunk ref.
    pub hunk_ref: String,
    /// Unified diff hunk header.
    pub hunk_header: String,
    /// Sticky header combining path and compare target.
    pub sticky_header_label: String,
    /// First old-side line covered by the hunk.
    pub old_start: u32,
    /// Number of old-side lines covered by the hunk.
    pub old_lines: u32,
    /// First new-side line covered by the hunk.
    pub new_start: u32,
    /// Number of new-side lines covered by the hunk.
    pub new_lines: u32,
    /// Syntax or structure label visible on the hunk.
    pub syntax_label: String,
    /// Lines in hunk order.
    pub rows: Vec<DiffLineView>,
}

/// Scroll anchor saved when a diff closes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffScrollAnchor {
    /// First visible row when the diff closed.
    pub first_visible_row_ref: String,
    /// Zero-based pixel or row offset within the first visible row.
    pub scroll_offset: u32,
}

/// Diff surface packet consumed by shell/review/support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffViewSurfacePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity copied from the source status snapshot.
    pub workspace_ref: String,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Surface ref opened by the public diff command.
    pub diff_surface_ref: String,
    /// Change-list row or equivalent public launcher ref.
    pub launch_source_ref: String,
    /// Selected diff layout mode.
    pub view_mode: DiffViewMode,
    /// Exact path truth for the diff target.
    pub path_truth: DiffPathTruth,
    /// Exact compare target for the diff.
    pub compare_target: DiffCompareTarget,
    /// Syntax or structure projection for the diffed file.
    pub syntax: DiffSyntaxProjection,
    /// Hunk rows in file order.
    pub hunks: Vec<DiffHunkView>,
    /// Whether any row contains suspicious text.
    pub suspicious_content_present: bool,
    /// Whether safe escaped copy must stay visible.
    pub safe_copy_required: bool,
}

impl DiffViewSurfacePacket {
    /// Builds a diff-view packet from an explicit open target and hunk input.
    pub fn from_file_input(open_target: DiffOpenTarget, input: DiffFileInput) -> Self {
        let path_truth = DiffPathTruth::from_open_target(
            &open_target,
            input.repo_root.clone(),
            input.logical_root_ref.clone(),
            input.worktree_ref.clone(),
        );
        let compare_target = DiffCompareTarget::from_open_target(&open_target);
        let syntax = DiffSyntaxProjection::from_path(
            &input.path,
            input.language_id.as_deref(),
            &input.workspace_ref,
        );
        let hunks = materialize_hunks(&input, &path_truth, &compare_target, &syntax);
        let suspicious_content_present = hunks
            .iter()
            .flat_map(|hunk| hunk.rows.iter())
            .any(|row| !row.suspicious_cues.is_empty());

        Self {
            record_kind: DIFF_VIEW_SURFACE_PACKET_RECORD_KIND.to_string(),
            schema_version: DIFF_VIEW_SURFACE_SCHEMA_VERSION,
            generated_at: input.generated_at,
            workspace_ref: input.workspace_ref,
            truth_source_ref: input.truth_source_ref,
            diff_surface_ref: open_target.opens_surface_ref,
            launch_source_ref: open_target.launch_source_ref,
            view_mode: input.view_mode,
            path_truth,
            compare_target,
            syntax,
            hunks,
            suspicious_content_present,
            safe_copy_required: suspicious_content_present,
        }
    }

    /// Returns true when every row shows path and compare target truth.
    pub fn all_rows_have_exact_path_and_target_truth(&self) -> bool {
        self.hunks.iter().all(|hunk| {
            hunk.rows
                .iter()
                .all(DiffLineView::has_visible_path_and_target_truth)
        })
    }

    /// Returns true when suspicious rows expose the safe copy path.
    pub fn all_suspicious_rows_offer_safe_copy(&self) -> bool {
        self.hunks.iter().all(|hunk| {
            hunk.rows
                .iter()
                .all(DiffLineView::suspicious_safe_copy_is_visible)
        })
    }

    /// Returns true when every row exposes raw, plain-text, and context copy.
    pub fn all_rows_offer_raw_plain_and_context_copy(&self) -> bool {
        self.hunks.iter().all(|hunk| {
            hunk.rows.iter().all(|row| {
                let tokens = row.copy_representation_tokens();
                tokens.contains(DiffCopyRepresentation::RawSource.as_str())
                    && tokens.contains(DiffCopyRepresentation::PlainText.as_str())
                    && tokens.contains(DiffCopyRepresentation::RenderedContext.as_str())
            })
        })
    }

    /// Returns distinct suspicious-content class tokens visible in the diff.
    pub fn suspicious_class_tokens(&self) -> Vec<String> {
        self.hunks
            .iter()
            .flat_map(|hunk| hunk.rows.iter())
            .flat_map(|row| row.suspicious_cues.iter())
            .map(|cue| cue.content_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Returns the hunk ref at `index`.
    pub fn hunk_ref_at(&self, index: usize) -> Option<&str> {
        self.hunks.get(index).map(|hunk| hunk.hunk_ref.as_str())
    }

    /// Returns the first row ref in the packet.
    pub fn first_row_ref(&self) -> Option<&str> {
        self.hunks
            .iter()
            .flat_map(|hunk| hunk.rows.iter())
            .next()
            .map(|row| row.row_ref.as_str())
    }

    /// Captures enough closed-session state to reopen the same diff target.
    pub fn close_for_reopen(
        &self,
        scroll_anchor: DiffScrollAnchor,
        selected_hunk_ref: Option<String>,
        selected_row_ref: Option<String>,
        closed_at: impl Into<String>,
    ) -> DiffClosedSessionRecord {
        DiffClosedSessionRecord {
            record_kind: DIFF_CLOSED_SESSION_RECORD_KIND.to_string(),
            schema_version: DIFF_CLOSED_SESSION_SCHEMA_VERSION,
            closed_session_ref: format!("git.diff.closed.{}", sanitize_id(&self.diff_surface_ref)),
            closed_at: closed_at.into(),
            reopen_command_id: "cmd:git.diff.reopen_closed".to_string(),
            diff_surface_ref: self.diff_surface_ref.clone(),
            workspace_ref: self.workspace_ref.clone(),
            path_truth_ref: self.path_truth.path_truth_ref.clone(),
            compare_target_ref: self.compare_target.compare_target_ref.clone(),
            compare_target: self.compare_target.clone(),
            scroll_anchor,
            selected_hunk_ref,
            selected_row_ref,
            launch_source_ref: self.launch_source_ref.clone(),
        }
    }
}

/// Closed diff-session state used by reopen-closed-diff commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffClosedSessionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable closed-session ref.
    pub closed_session_ref: String,
    /// Timestamp captured when the diff closed.
    pub closed_at: String,
    /// Public command id that reopens the closed diff.
    pub reopen_command_id: String,
    /// Surface ref that should be reopened.
    pub diff_surface_ref: String,
    /// Workspace identity copied from the source diff packet.
    pub workspace_ref: String,
    /// Path-truth ref that must be restored.
    pub path_truth_ref: String,
    /// Compare-target ref that must be restored.
    pub compare_target_ref: String,
    /// Compare target captured at close time.
    pub compare_target: DiffCompareTarget,
    /// Scroll anchor captured at close time.
    pub scroll_anchor: DiffScrollAnchor,
    /// Selected hunk ref captured at close time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_hunk_ref: Option<String>,
    /// Selected row ref captured at close time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_row_ref: Option<String>,
    /// Change-list row or equivalent public launcher ref.
    pub launch_source_ref: String,
}

impl DiffClosedSessionRecord {
    /// Projects the reopen result without falling back to a generic file open.
    pub fn reopen(&self) -> DiffReopenProjection {
        DiffReopenProjection {
            record_kind: DIFF_REOPEN_PROJECTION_RECORD_KIND.to_string(),
            schema_version: DIFF_REOPEN_PROJECTION_SCHEMA_VERSION,
            reopened_surface_ref: self.diff_surface_ref.clone(),
            reopened_from_closed_session_ref: self.closed_session_ref.clone(),
            restored_workspace_ref: self.workspace_ref.clone(),
            restored_path_truth_ref: self.path_truth_ref.clone(),
            restored_compare_target_ref: self.compare_target_ref.clone(),
            restored_compare_target_label: self.compare_target.exact_target_label.clone(),
            restored_scroll_anchor: self.scroll_anchor.clone(),
            restored_selected_hunk_ref: self.selected_hunk_ref.clone(),
            restored_selected_row_ref: self.selected_row_ref.clone(),
            fallback_open_file_used: false,
            continuity_label: "closed diff reopened with compare target and scroll anchor"
                .to_string(),
        }
    }
}

/// Reopen projection produced from a closed diff-session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffReopenProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Surface ref reopened by the command.
    pub reopened_surface_ref: String,
    /// Closed-session ref used as the source.
    pub reopened_from_closed_session_ref: String,
    /// Workspace identity restored from the closed session.
    pub restored_workspace_ref: String,
    /// Path-truth ref restored from the closed session.
    pub restored_path_truth_ref: String,
    /// Compare-target ref restored from the closed session.
    pub restored_compare_target_ref: String,
    /// Compare-target label restored from the closed session.
    pub restored_compare_target_label: String,
    /// Scroll anchor restored from the closed session.
    pub restored_scroll_anchor: DiffScrollAnchor,
    /// Selected hunk ref restored from the closed session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_selected_hunk_ref: Option<String>,
    /// Selected row ref restored from the closed session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_selected_row_ref: Option<String>,
    /// False when the diff reopens as a diff rather than generic file open.
    pub fallback_open_file_used: bool,
    /// Human-readable continuity note.
    pub continuity_label: String,
}

fn materialize_hunks(
    input: &DiffFileInput,
    path_truth: &DiffPathTruth,
    compare_target: &DiffCompareTarget,
    syntax: &DiffSyntaxProjection,
) -> Vec<DiffHunkView> {
    input
        .hunks
        .iter()
        .enumerate()
        .map(|(hunk_index, hunk)| {
            let hunk_ref = format!(
                "git.diff.hunk.{}.{}.{}",
                sanitize_id(&input.workspace_ref),
                sanitize_id(&input.path.to_string_lossy()),
                hunk_index
            );
            let rows = hunk
                .lines
                .iter()
                .enumerate()
                .map(|(row_index, line)| {
                    materialize_line(
                        input,
                        path_truth,
                        compare_target,
                        syntax,
                        &hunk_ref,
                        row_index,
                        line,
                    )
                })
                .collect();
            DiffHunkView {
                hunk_ref,
                hunk_header: hunk.hunk_header.clone(),
                sticky_header_label: format!(
                    "{} · {}",
                    path_truth.path_label, compare_target.exact_target_label
                ),
                old_start: hunk.old_start,
                old_lines: hunk.old_lines,
                new_start: hunk.new_start,
                new_lines: hunk.new_lines,
                syntax_label: syntax.syntax_label.clone(),
                rows,
            }
        })
        .collect()
}

fn materialize_line(
    input: &DiffFileInput,
    path_truth: &DiffPathTruth,
    compare_target: &DiffCompareTarget,
    syntax: &DiffSyntaxProjection,
    hunk_ref: &str,
    row_index: usize,
    line: &DiffLineInput,
) -> DiffLineView {
    let row_ref = format!("{}.row.{}.{}", hunk_ref, row_index, line.line_kind.as_str());
    let cues = suspicious_cues_for_line(&row_ref, &input.workspace_ref, &line.raw_text);
    let warning_refs = cues
        .iter()
        .map(|cue| cue.warning_id.clone())
        .collect::<Vec<_>>();
    let copy_actions = copy_actions_for_line(&row_ref, !cues.is_empty(), warning_refs);

    DiffLineView {
        row_ref,
        hunk_row_index: row_index,
        line_kind: line.line_kind,
        marker: line.line_kind.marker().to_string(),
        old_line_number: line.old_line_number,
        new_line_number: line.new_line_number,
        raw_text: line.raw_text.clone(),
        rendered_text: format!("{}{}", line.line_kind.marker(), line.raw_text),
        escaped_text: escape_for_safe_inspection(&line.raw_text),
        syntax_class_token: syntax.syntax_class_token.clone(),
        structure_label: structure_label_for_line(&syntax.syntax_class_token, &line.raw_text),
        path_label: path_truth.path_label.clone(),
        target_label: compare_target.exact_target_label.clone(),
        path_truth_ref: path_truth.path_truth_ref.clone(),
        compare_target_ref: compare_target.compare_target_ref.clone(),
        suspicious_cues: cues,
        copy_actions,
    }
}

fn suspicious_cues_for_line(
    row_ref: &str,
    workspace_ref: &str,
    text: &str,
) -> Vec<DiffSuspiciousCue> {
    let detection = detect_suspicious_content(text);
    detection
        .findings
        .iter()
        .map(|finding| suspicious_cue_for_finding(row_ref, workspace_ref, text, finding))
        .collect()
}

fn suspicious_cue_for_finding(
    row_ref: &str,
    workspace_ref: &str,
    text: &str,
    finding: &SuspiciousFinding,
) -> DiffSuspiciousCue {
    let raw_snippet = text
        .chars()
        .skip(finding.char_offset)
        .take(finding.length_chars)
        .collect::<String>();
    let escaped_snippet = escape_for_safe_inspection(&raw_snippet);
    let class_token = finding.class.as_str().to_string();
    DiffSuspiciousCue {
        warning_id: format!(
            "warning:git-diff:{}:{}",
            sanitize_id(row_ref),
            sanitize_id(&finding.finding_id)
        ),
        detector_finding_id: finding.finding_id.clone(),
        content_class_token: class_token,
        line_ref: row_ref.to_string(),
        continuity_ref: format!(
            "suspicious.git.diff.{}.{}.{}",
            sanitize_id(workspace_ref),
            sanitize_id(row_ref),
            sanitize_id(&finding.finding_id)
        ),
        warning_label: warning_label_for(finding.class).to_string(),
        raw_snippet,
        escaped_snippet,
        matched_codepoints: finding.matched_codepoints.clone(),
        reveal_action_ids: vec![
            "reveal_raw_source".to_string(),
            "reveal_escaped_source".to_string(),
            "inspect_codepoints".to_string(),
        ],
    }
}

fn copy_actions_for_line(
    row_ref: &str,
    suspicious: bool,
    warning_refs: Vec<String>,
) -> Vec<DiffCopyAction> {
    let mut actions = vec![
        DiffCopyAction::line_action(
            row_ref,
            RepresentationActionId::CopyRaw.as_str(),
            "Copy raw line",
            DiffCopyRepresentation::RawSource,
            false,
            false,
            warning_refs.clone(),
        ),
        DiffCopyAction::line_action(
            row_ref,
            "copy_plain_text",
            "Copy plain text line",
            DiffCopyRepresentation::PlainText,
            false,
            false,
            warning_refs.clone(),
        ),
        DiffCopyAction::line_action(
            row_ref,
            "copy_rendered_context",
            "Copy rendered hunk context",
            DiffCopyRepresentation::RenderedContext,
            true,
            false,
            warning_refs.clone(),
        ),
    ];
    if suspicious {
        actions.push(DiffCopyAction::line_action(
            row_ref,
            RepresentationActionId::CopyEscaped.as_str(),
            "Copy escaped line",
            DiffCopyRepresentation::EscapedSource,
            false,
            true,
            warning_refs,
        ));
    }
    actions
}

fn warning_label_for(class: SuspiciousContentClass) -> &'static str {
    match class {
        SuspiciousContentClass::BidiControl => "Bidi control present",
        SuspiciousContentClass::InvisibleFormatting => "Invisible formatting present",
        SuspiciousContentClass::MixedScriptConfusable => "Mixed-script confusable present",
        SuspiciousContentClass::WholeScriptConfusable => "Whole-script confusable present",
        SuspiciousContentClass::RawRenderedDivergence => "Raw/rendered divergence present",
    }
}

fn structure_label_for_line(syntax_token: &str, raw_text: &str) -> Option<String> {
    let trimmed = raw_text.trim_start();
    match syntax_token {
        "rust" if trimmed.starts_with("fn ") || trimmed.contains(" fn ") => {
            Some("rust.function".to_string())
        }
        "rust" if trimmed.starts_with("struct ") || trimmed.contains(" struct ") => {
            Some("rust.struct".to_string())
        }
        "typescript" | "javascript" if trimmed.starts_with("function ") => {
            Some("js.function".to_string())
        }
        "json" if trimmed.starts_with('"') && trimmed.contains(':') => Some("json.key".to_string()),
        "yaml" if !trimmed.starts_with('#') && trimmed.contains(':') => {
            Some("yaml.key".to_string())
        }
        "toml" if !trimmed.starts_with('#') && trimmed.contains('=') => {
            Some("toml.key".to_string())
        }
        "markdown" if trimmed.starts_with('#') => Some("markdown.heading".to_string()),
        _ => None,
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

    fn open_target(group: &str, path: &str) -> DiffOpenTarget {
        DiffOpenTarget::from_change_list_row_parts(
            "workspace.unit.diff",
            "git.status.snapshot.unit",
            "git.change.row.unit",
            group,
            PathBuf::from(path),
            None,
            ".M",
            "modified",
        )
    }

    fn input(path: &str, raw_text: &str) -> DiffFileInput {
        DiffFileInput {
            workspace_ref: "workspace.unit.diff".to_string(),
            truth_source_ref: "git.status.snapshot.unit".to_string(),
            repo_root: PathBuf::from("/workspace/unit"),
            logical_root_ref: "root.local.unit".to_string(),
            worktree_ref: "worktree.local.unit".to_string(),
            group_token: "unstaged".to_string(),
            path: PathBuf::from(path),
            original_path: None,
            status_code: ".M".to_string(),
            language_id: None,
            view_mode: DiffViewMode::Unified,
            generated_at: "mono:unit".to_string(),
            hunks: vec![DiffHunkInput {
                hunk_header: "@@ -1,1 +1,1 @@".to_string(),
                old_start: 1,
                old_lines: 1,
                new_start: 1,
                new_lines: 1,
                lines: vec![DiffLineInput {
                    line_kind: DiffLineKind::Addition,
                    old_line_number: None,
                    new_line_number: Some(1),
                    raw_text: raw_text.to_string(),
                }],
            }],
        }
    }

    #[test]
    fn materializes_syntax_path_truth_and_safe_copy() {
        let packet = DiffViewSurfacePacket::from_file_input(
            open_target("unstaged", "src/lib.rs"),
            input("src/lib.rs", "let admin\u{202E} = true;"),
        );

        assert_eq!(packet.syntax.syntax_class_token, "rust");
        assert!(packet.all_rows_have_exact_path_and_target_truth());
        assert!(packet.all_rows_offer_raw_plain_and_context_copy());
        assert!(packet.all_suspicious_rows_offer_safe_copy());
        assert_eq!(packet.suspicious_class_tokens(), vec!["bidi_control"]);
    }

    #[test]
    fn reopens_closed_diff_without_generic_file_fallback() {
        let packet = DiffViewSurfacePacket::from_file_input(
            open_target("staged", "src/lib.rs"),
            input("src/lib.rs", "pub fn demo() {}"),
        );
        let hunk_ref = packet.hunk_ref_at(0).expect("hunk").to_string();
        let row_ref = packet.first_row_ref().expect("row").to_string();
        let closed = packet.close_for_reopen(
            DiffScrollAnchor {
                first_visible_row_ref: row_ref.clone(),
                scroll_offset: 12,
            },
            Some(hunk_ref.clone()),
            Some(row_ref.clone()),
            "mono:closed",
        );
        let reopened = closed.reopen();

        assert!(!reopened.fallback_open_file_used);
        assert_eq!(
            reopened.restored_compare_target_ref,
            packet.compare_target.compare_target_ref
        );
        assert_eq!(
            reopened.restored_path_truth_ref,
            packet.path_truth.path_truth_ref
        );
        assert_eq!(reopened.restored_selected_hunk_ref, Some(hunk_ref));
        assert_eq!(
            reopened.restored_scroll_anchor.first_visible_row_ref,
            row_ref
        );
    }
}
