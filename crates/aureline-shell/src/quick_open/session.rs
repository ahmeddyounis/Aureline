//! Quick-open query session, source-state truth, and result projection.
//!
//! See the module docs in `super` for the honesty contract. This file owns:
//!
//! - the closed [`QuickOpenSourceClass`] / [`QuickOpenSourceState`]
//!   vocabularies the chrome consumes;
//! - the [`QuickOpenQuerySession`] state object that tracks query, scope,
//!   held modifiers, and per-source readiness without forking truth from the
//!   command registry / lexical shell / recents store;
//! - a serializable [`QuickOpenSnapshot`] used in fixtures and support
//!   bundles so rendered truth is byte-replayable.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use aureline_search::lexical::ResultRow as LexicalResultRow;
use aureline_search::{
    LexicalShell, ScopeClass as SearchScopeClass, WorkspaceSearchScope,
    WorkspaceSearchScopeMetadata,
};
use aureline_workspace::{ScopeClass as WorkspaceScopeClass, WorksetArtifactRecord};

/// Maximum recent-target rows surfaced in the recents lane. Keeps the lane
/// useful as an accelerator without crowding out commands and files.
pub const RECENTS_LANE_CAP: usize = 6;

/// Maximum command rows surfaced in the commands lane.
pub const COMMANDS_LANE_CAP: usize = 8;

/// Maximum lexical-file rows surfaced across the filename and path lanes.
pub const LEXICAL_LANE_CAP: usize = 12;

/// Closed vocabulary for which lane produced a quick-open row.
///
/// The token vocabulary deliberately matches
/// [`aureline_search::SourceClass`] for the lexical lanes so the chrome and
/// the search-shell snapshot stay aligned. Recent-target and command lanes
/// add their own tokens — quick open does not relabel a lexical hit as
/// semantic just because a richer surface is wired up alongside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickOpenSourceClass {
    /// Locally tracked recent navigation target (file/buffer/place).
    RecentTarget,
    /// Command registry entry, projected through the canonical command id.
    Command,
    /// Lexical filename match (basename hit on a workspace file).
    LexicalFilename,
    /// Lexical path match (substring hit on a workspace-relative path).
    LexicalPath,
}

impl QuickOpenSourceClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentTarget => "recent_target",
            Self::Command => "command",
            Self::LexicalFilename => "lexical_filename",
            Self::LexicalPath => "lexical_path",
        }
    }

    /// Short attribution badge surfaces render next to a row to make the
    /// match lane explicit.
    pub const fn badge(self) -> &'static str {
        match self {
            Self::RecentTarget => "recent",
            Self::Command => "command",
            Self::LexicalFilename => "filename",
            Self::LexicalPath => "path",
        }
    }

    /// Human-readable label for a lane header.
    pub const fn lane_label(self) -> &'static str {
        match self {
            Self::RecentTarget => "Recent",
            Self::Command => "Commands",
            Self::LexicalFilename => "Filenames",
            Self::LexicalPath => "Paths",
        }
    }
}

/// Closed vocabulary describing the readiness of one quick-open source.
///
/// The taxonomy is a strict subset of the upstream readiness vocabularies:
/// recents are always [`Self::Ready`] when any rows are tracked, commands are
/// [`Self::Ready`] when the registry surface is live (or [`Self::Unavailable`]
/// when it is not), and the lexical lane mirrors
/// [`aureline_search::ReadinessClass`] (with `out_of_scope` collapsed into
/// [`Self::Unavailable`] for quick-open's purposes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickOpenSourceState {
    /// Source has not been activated for the current session/query.
    NotRequested,
    /// Source is warming and has not produced any rows yet.
    Warming,
    /// Source has rows but coverage is still incomplete.
    Partial,
    /// Source is ready (or fully populated) for the current query.
    Ready,
    /// Source cannot answer right now (unavailable, blocked, or out of
    /// scope). The chrome MUST still render a section header so the user can
    /// see which lane went dark.
    Unavailable,
}

impl QuickOpenSourceState {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Ready => "ready",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when the source is still resolving and the chrome should
    /// surface a partiality cue.
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Warming | Self::Partial)
    }

    /// Project from the upstream lexical readiness class.
    pub fn from_lexical(readiness: aureline_search::ReadinessClass) -> Self {
        use aureline_search::ReadinessClass;
        match readiness {
            ReadinessClass::Ready => Self::Ready,
            ReadinessClass::Warming => Self::Warming,
            ReadinessClass::Partial => Self::Partial,
            ReadinessClass::Stale => Self::Partial,
            ReadinessClass::Unavailable | ReadinessClass::OutOfScope => Self::Unavailable,
        }
    }
}

/// Closed vocabulary for the rendered row kind.
///
/// The kind drives default action enablement (e.g., commands run through the
/// command runtime; files open in the editor; recent places jump to a saved
/// route).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickOpenRowKind {
    /// Row resolves to a recent navigation target.
    RecentTarget,
    /// Row resolves to a command id.
    Command,
    /// Row resolves to a workspace file by relative path.
    File,
}

impl QuickOpenRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentTarget => "recent_target",
            Self::Command => "command",
            Self::File => "file",
        }
    }
}

/// Stable selection key for a quick-open row.
///
/// Selection survives ranking churn and partial streaming because rows are
/// re-resolved against this key each time the result set is materialized.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QuickOpenRowKey {
    /// Recent target keyed by its stable identifier (path, route, or buffer
    /// anchor — quick open does not interpret it).
    RecentTarget { recent_id: String },
    /// Command keyed by its canonical command id.
    Command { command_id: String },
    /// Workspace file keyed by its workspace-relative path.
    File { relative_path: String },
}

/// Recent-target row supplied by the consumer.
///
/// Quick open does not own the recent-work registry. It accepts a lightweight
/// projection per session so the session can remain testable without pulling
/// the full registry pipeline in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenRecentTarget {
    /// Stable identifier for the recent target. May be a workspace-relative
    /// path, route id, buffer anchor — quick open does not interpret it.
    pub recent_id: String,
    /// Primary label (e.g., basename or display title).
    pub display_label: String,
    /// Secondary label (e.g., directory or route hint).
    pub secondary_label: String,
    /// Optional workspace-relative path. When present, the session
    /// deduplicates lexical rows that match this path (recents win).
    pub relative_path: Option<String>,
    /// Stable target-kind token, e.g., `local_file`, `recent_location`. The
    /// session does not interpret it; it forwards it to the snapshot.
    pub target_kind_token: String,
}

/// Command row supplied by the consumer.
///
/// The consumer constructs this projection from
/// [`aureline_commands::CommandRegistryEntryRecord`] so the canonical command
/// identity, disabled-reason vocabulary, and invocation-preview class flow
/// across surfaces without translation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenCommandRow {
    pub command_id: String,
    pub title: String,
    pub summary: String,
    pub dominant_side_effect_class: String,
    /// Frozen invocation-preview class projected from the command descriptor
    /// (e.g., `none`, `summary`, `diff`). Surfaces MUST surface this directly
    /// rather than guessing at preview behavior.
    pub invocation_preview_class: String,
    /// `disabled_reason_code` from the registry entry's
    /// `disabled_reason_records`, when the command is currently disabled.
    /// Surfaces MUST surface this directly so the same disabled reason shows
    /// up in palette, quick open, and command diagnostics.
    pub disabled_reason_class: Option<String>,
}

impl QuickOpenCommandRow {
    /// Project a command row from a registry entry. Picks the first
    /// disabled-reason record (if any) for `disabled_reason_class`. Surfaces
    /// that need a richer disabled-reason picker should read the registry
    /// directly — the quick-open row only carries the dominant reason.
    pub fn from_registry_entry(entry: &CommandRegistryEntryRecord) -> Self {
        let disabled_reason_class = entry
            .disabled_reason_records
            .first()
            .map(|record| record.disabled_reason_code.as_str().to_string());
        Self {
            command_id: entry.command_id().to_string(),
            title: entry.title.clone(),
            summary: entry.summary.clone(),
            dominant_side_effect_class: entry.dominant_side_effect_class.clone(),
            invocation_preview_class: entry.descriptor.preview_class.clone(),
            disabled_reason_class,
        }
    }
}

/// Lexical-row projection consumed by the quick-open session.
///
/// We project [`aureline_search::lexical::ResultRow`] into a small struct so
/// the session does not need to depend on the lexical shell's internal
/// match-kind scoring — and so the session stays testable without standing
/// up a full workspace lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenLexicalRow {
    pub relative_path: String,
    pub source_class: QuickOpenSourceClass,
    pub match_kind_token: String,
}

impl QuickOpenLexicalRow {
    /// Build a lexical row from a search-shell row. The function preserves
    /// the upstream source-class vocabulary verbatim — quick open does not
    /// invent a parallel taxonomy.
    pub fn from_lexical_row(row: &LexicalResultRow) -> Self {
        let source_class = match row.source_class {
            aureline_search::SourceClass::LexicalFilename => QuickOpenSourceClass::LexicalFilename,
            aureline_search::SourceClass::LexicalPath => QuickOpenSourceClass::LexicalPath,
        };
        Self {
            relative_path: row.relative_path.clone(),
            source_class,
            match_kind_token: row.match_kind.as_str().to_string(),
        }
    }
}

/// Scope chip projected onto the quick-open session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenScopeChip {
    pub scope_class_token: String,
    pub scope_chip_label: String,
}

/// One materialized quick-open result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenResultRow {
    pub key: QuickOpenRowKey,
    pub row_kind: QuickOpenRowKind,
    pub source_class: QuickOpenSourceClass,
    pub source_state: QuickOpenSourceState,
    pub display_label: String,
    pub secondary_label: String,
    pub ranking_sources: Vec<String>,
    /// `command_id` echoed for command rows; `None` for non-command rows.
    /// Surfaces MUST quote this directly — the row never re-derives a
    /// command id from a display label.
    pub command_id: Option<String>,
    /// Disabled-reason class for command rows that are currently disabled.
    pub disabled_reason_class: Option<String>,
    /// Invocation-preview class for command rows.
    pub invocation_preview_class: Option<String>,
    /// Workspace-relative path for file rows (and recent-target rows whose
    /// recent identity is path-shaped).
    pub relative_path: Option<String>,
}

/// Live runtime state for a quick-open query session.
#[derive(Debug, Clone)]
pub struct QuickOpenQuerySession {
    open: bool,
    query: String,
    held_modifiers: BTreeSet<String>,
    scope_class: WorkspaceScopeClass,
    workset_name: Option<String>,
    workspace_id: String,
    scope: WorkspaceSearchScope,

    recents: Vec<QuickOpenRecentTarget>,
    commands: Vec<QuickOpenCommandRow>,
    lexical_rows: Vec<QuickOpenLexicalRow>,

    recents_state: QuickOpenSourceState,
    commands_state: QuickOpenSourceState,
    lexical_state: QuickOpenSourceState,
    lexical_partial_truth_causes: Vec<String>,

    rows: Vec<QuickOpenResultRow>,
}

impl QuickOpenQuerySession {
    /// Construct a new closed session for the given workspace and scope. The
    /// session is not visible until [`Self::open`] is called.
    pub fn new(
        workspace_id: impl Into<String>,
        scope_class: WorkspaceScopeClass,
        workset_name: Option<String>,
    ) -> Self {
        let workspace_id = workspace_id.into();
        let scope = project_default_scope(&workspace_id, scope_class, workset_name.as_deref());
        Self {
            open: false,
            query: String::new(),
            held_modifiers: BTreeSet::new(),
            scope_class,
            workset_name,
            workspace_id,
            scope,
            recents: Vec::new(),
            commands: Vec::new(),
            lexical_rows: Vec::new(),
            recents_state: QuickOpenSourceState::NotRequested,
            commands_state: QuickOpenSourceState::NotRequested,
            lexical_state: QuickOpenSourceState::NotRequested,
            lexical_partial_truth_causes: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Construct a new closed session bound to a workset artifact. The
    /// scope's chip label, presentation state, and pattern fingerprint
    /// flow from the artifact through the canonical
    /// [`WorkspaceSearchScope`] resolver — quick open never re-derives
    /// scope vocabulary locally.
    pub fn new_with_workset_artifact(
        workspace_id: impl Into<String>,
        artifact: &WorksetArtifactRecord,
    ) -> Self {
        let workspace_id = workspace_id.into();
        let scope = WorkspaceSearchScope::from_workset_artifact(&workspace_id, artifact);
        Self {
            open: false,
            query: String::new(),
            held_modifiers: BTreeSet::new(),
            scope_class: artifact.scope_class,
            workset_name: scope.workset_name().map(|s| s.to_string()),
            workspace_id,
            scope,
            recents: Vec::new(),
            commands: Vec::new(),
            lexical_rows: Vec::new(),
            recents_state: QuickOpenSourceState::NotRequested,
            commands_state: QuickOpenSourceState::NotRequested,
            lexical_state: QuickOpenSourceState::NotRequested,
            lexical_partial_truth_causes: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Open the session. Re-materializes the row set so callers can render
    /// hot-state recents and commands immediately, even before lexical
    /// indexing finishes.
    pub fn open(&mut self) {
        self.open = true;
        self.rebuild();
    }

    /// Close the session. Held modifiers and the query string are cleared so
    /// the next open does not surface stale typing.
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.held_modifiers.clear();
        self.rows.clear();
    }

    /// Returns whether the session is currently open.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Active query string.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Replace the query and re-materialize results.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.rebuild();
    }

    /// Held modifier tokens (e.g., `command_palette_shortcut`,
    /// `files_only_filter`). The session does not interpret modifier
    /// semantics — it stores them so palette and search surfaces can converge
    /// on the same mental model.
    pub fn held_modifiers(&self) -> Vec<&str> {
        self.held_modifiers.iter().map(String::as_str).collect()
    }

    /// Replace the held-modifier set.
    pub fn set_held_modifiers<I, S>(&mut self, modifiers: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.held_modifiers = modifiers.into_iter().map(Into::into).collect();
        self.rebuild();
    }

    /// Active scope class.
    pub fn scope_class(&self) -> WorkspaceScopeClass {
        self.scope_class
    }

    /// Replace the scope class and (optional) workset name.
    pub fn set_scope(&mut self, scope_class: WorkspaceScopeClass, workset_name: Option<String>) {
        self.scope_class = scope_class;
        self.workset_name = workset_name.clone();
        self.scope =
            project_default_scope(&self.workspace_id, scope_class, workset_name.as_deref());
        self.rebuild();
    }

    /// Replace the active workset/slice scope with one projected from a
    /// workset artifact. The chip label, presentation state, and pattern
    /// fingerprint flow from the artifact so quick open and the search
    /// shell stay aligned.
    pub fn set_workset_artifact(&mut self, artifact: &WorksetArtifactRecord) {
        self.scope_class = artifact.scope_class;
        let scope = WorkspaceSearchScope::from_workset_artifact(&self.workspace_id, artifact);
        self.workset_name = scope.workset_name().map(|s| s.to_string());
        self.scope = scope;
        self.rebuild();
    }

    /// Active workset/slice scope.
    pub fn workspace_search_scope(&self) -> &WorkspaceSearchScope {
        &self.scope
    }

    /// Project the active scope's serializable metadata for support
    /// bundles, dogfood replays, and diagnostic exports.
    pub fn scope_metadata(&self) -> WorkspaceSearchScopeMetadata {
        self.scope.project_metadata()
    }

    /// Project the canonical scope chip.
    pub fn scope_chip(&self) -> QuickOpenScopeChip {
        let search_scope = SearchScopeClass::from_workspace(self.scope_class);
        let label = match search_scope {
            SearchScopeClass::CurrentRepo | SearchScopeClass::FullWorkspace => {
                search_scope.chip_label_family().to_string()
            }
            SearchScopeClass::SelectedWorkset
            | SearchScopeClass::SparseSlice
            | SearchScopeClass::PolicyLimitedView => match self.workset_name.as_deref() {
                Some(name) if !name.trim().is_empty() => {
                    format!("{} · {}", search_scope.chip_label_family(), name)
                }
                _ => search_scope.chip_label_family().to_string(),
            },
        };
        QuickOpenScopeChip {
            scope_class_token: search_scope.as_str().to_string(),
            scope_chip_label: label,
        }
    }

    /// Workspace identity bound to the session.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Replace the recent-target list. Recents are served from hot local
    /// state; their source state is [`QuickOpenSourceState::Ready`] when any
    /// targets are tracked, and [`QuickOpenSourceState::NotRequested`] when
    /// none are.
    pub fn set_recents(&mut self, recents: Vec<QuickOpenRecentTarget>) {
        self.recents_state = if recents.is_empty() {
            QuickOpenSourceState::NotRequested
        } else {
            QuickOpenSourceState::Ready
        };
        self.recents = recents;
        self.rebuild();
    }

    /// Mark the recents source as unavailable (e.g., recent-work registry
    /// blocked by policy). Existing rows are dropped.
    pub fn mark_recents_unavailable(&mut self) {
        self.recents.clear();
        self.recents_state = QuickOpenSourceState::Unavailable;
        self.rebuild();
    }

    /// Replace the command-row projection from the canonical registry.
    pub fn set_commands_from_registry(&mut self, registry: &CommandRegistry) {
        let rows: Vec<QuickOpenCommandRow> = registry
            .entries()
            .iter()
            .filter(|entry| {
                entry
                    .descriptor
                    .client_scopes
                    .iter()
                    .any(|scope| scope == "desktop_product")
                    && entry.descriptor.palette_visibility != "hidden_palette_callable_only"
            })
            .map(QuickOpenCommandRow::from_registry_entry)
            .collect();
        self.set_commands(rows);
    }

    /// Replace the command-row projection directly. Use this in tests or
    /// when the registry projection has already been computed elsewhere.
    pub fn set_commands(&mut self, commands: Vec<QuickOpenCommandRow>) {
        self.commands_state = if commands.is_empty() {
            QuickOpenSourceState::NotRequested
        } else {
            QuickOpenSourceState::Ready
        };
        self.commands = commands;
        self.rebuild();
    }

    /// Mark the commands source as unavailable.
    pub fn mark_commands_unavailable(&mut self) {
        self.commands.clear();
        self.commands_state = QuickOpenSourceState::Unavailable;
        self.rebuild();
    }

    /// Replace the lexical projection from a live [`LexicalShell`].
    ///
    /// The session does not invoke the shell — it only projects the latest
    /// materialized result set. Callers should re-run this when the shell's
    /// query / index / scope has changed.
    pub fn refresh_lexical_from_shell(&mut self, shell: &LexicalShell) {
        let results = shell.results();
        let mut rows: Vec<QuickOpenLexicalRow> = Vec::new();
        for group in &results.groups {
            for row in &group.items {
                rows.push(QuickOpenLexicalRow::from_lexical_row(row));
            }
        }
        self.lexical_rows = rows;
        self.lexical_state = QuickOpenSourceState::from_lexical(results.readiness);
        self.lexical_partial_truth_causes = results.partial_truth_causes.clone();
        self.rebuild();
    }

    /// Replace the lexical projection directly with a pre-built row set and
    /// state. Used in tests and for synthetic fixtures.
    pub fn set_lexical(
        &mut self,
        rows: Vec<QuickOpenLexicalRow>,
        state: QuickOpenSourceState,
        partial_truth_causes: Vec<String>,
    ) {
        self.lexical_rows = rows;
        self.lexical_state = state;
        self.lexical_partial_truth_causes = partial_truth_causes;
        self.rebuild();
    }

    /// State of the recents source.
    pub fn recents_state(&self) -> QuickOpenSourceState {
        self.recents_state
    }

    /// State of the commands source.
    pub fn commands_state(&self) -> QuickOpenSourceState {
        self.commands_state
    }

    /// State of the lexical source.
    pub fn lexical_state(&self) -> QuickOpenSourceState {
        self.lexical_state
    }

    /// Partial-truth causes projected from the lexical shell, when any.
    pub fn lexical_partial_truth_causes(&self) -> &[String] {
        &self.lexical_partial_truth_causes
    }

    /// Materialized result rows in display order.
    pub fn rows(&self) -> &[QuickOpenResultRow] {
        &self.rows
    }

    /// Export a serializable snapshot of the session.
    pub fn export_snapshot(&self, observed_at: impl Into<String>) -> QuickOpenSnapshot {
        let scope = self.scope_chip();
        let sources = vec![
            QuickOpenSnapshotSource {
                source_class_token: QuickOpenSourceClass::RecentTarget.as_str().to_string(),
                source_state_token: self.recents_state.as_str().to_string(),
                visible_row_count: self
                    .rows
                    .iter()
                    .filter(|r| r.source_class == QuickOpenSourceClass::RecentTarget)
                    .count(),
            },
            QuickOpenSnapshotSource {
                source_class_token: QuickOpenSourceClass::Command.as_str().to_string(),
                source_state_token: self.commands_state.as_str().to_string(),
                visible_row_count: self
                    .rows
                    .iter()
                    .filter(|r| r.source_class == QuickOpenSourceClass::Command)
                    .count(),
            },
            QuickOpenSnapshotSource {
                source_class_token: QuickOpenSourceClass::LexicalFilename.as_str().to_string(),
                source_state_token: self.lexical_state.as_str().to_string(),
                visible_row_count: self
                    .rows
                    .iter()
                    .filter(|r| r.source_class == QuickOpenSourceClass::LexicalFilename)
                    .count(),
            },
            QuickOpenSnapshotSource {
                source_class_token: QuickOpenSourceClass::LexicalPath.as_str().to_string(),
                source_state_token: self.lexical_state.as_str().to_string(),
                visible_row_count: self
                    .rows
                    .iter()
                    .filter(|r| r.source_class == QuickOpenSourceClass::LexicalPath)
                    .count(),
            },
        ];

        let rows = self
            .rows
            .iter()
            .map(|row| QuickOpenSnapshotRow {
                key: row.key.clone(),
                row_kind_token: row.row_kind.as_str().to_string(),
                source_class_token: row.source_class.as_str().to_string(),
                source_state_token: row.source_state.as_str().to_string(),
                display_label: row.display_label.clone(),
                secondary_label: row.secondary_label.clone(),
                ranking_sources: row.ranking_sources.clone(),
                command_id: row.command_id.clone(),
                disabled_reason_class: row.disabled_reason_class.clone(),
                invocation_preview_class: row.invocation_preview_class.clone(),
                relative_path: row.relative_path.clone(),
            })
            .collect();

        QuickOpenSnapshot {
            record_kind: "quick_open_query_session_snapshot".to_string(),
            schema_version: 1,
            workspace_id: self.workspace_id.clone(),
            scope_class_token: scope.scope_class_token,
            scope_chip_label: scope.scope_chip_label,
            scope_metadata: self.scope.project_metadata(),
            query: self.query.clone(),
            held_modifiers: self.held_modifiers.iter().cloned().collect(),
            sources,
            lexical_partial_truth_causes: self.lexical_partial_truth_causes.clone(),
            rows,
            available_source_classes: vec![
                QuickOpenSourceClass::RecentTarget.as_str().to_string(),
                QuickOpenSourceClass::Command.as_str().to_string(),
                QuickOpenSourceClass::LexicalFilename.as_str().to_string(),
                QuickOpenSourceClass::LexicalPath.as_str().to_string(),
            ],
            observed_at: observed_at.into(),
        }
    }

    fn rebuild(&mut self) {
        self.rows.clear();
        if !self.open {
            return;
        }

        let normalized = self.query.trim().to_ascii_lowercase();
        let mut taken_paths: BTreeSet<String> = BTreeSet::new();

        // Recents lane (always served from hot local state first).
        let mut recents_count = 0usize;
        for recent in &self.recents {
            if recents_count >= RECENTS_LANE_CAP {
                break;
            }
            if !normalized.is_empty()
                && !contains_ci(&recent.display_label, &normalized)
                && !contains_ci(&recent.secondary_label, &normalized)
                && !recent
                    .relative_path
                    .as_deref()
                    .map(|path| contains_ci(path, &normalized))
                    .unwrap_or(false)
            {
                continue;
            }
            if let Some(path) = &recent.relative_path {
                taken_paths.insert(path.clone());
            }
            self.rows.push(QuickOpenResultRow {
                key: QuickOpenRowKey::RecentTarget {
                    recent_id: recent.recent_id.clone(),
                },
                row_kind: QuickOpenRowKind::RecentTarget,
                source_class: QuickOpenSourceClass::RecentTarget,
                source_state: self.recents_state,
                display_label: recent.display_label.clone(),
                secondary_label: recent.secondary_label.clone(),
                ranking_sources: vec![
                    "recent_target".to_string(),
                    recent.target_kind_token.clone(),
                ],
                command_id: None,
                disabled_reason_class: None,
                invocation_preview_class: None,
                relative_path: recent.relative_path.clone(),
            });
            recents_count += 1;
        }

        // Commands lane (canonical command identity, always quoted by id).
        let mut commands_count = 0usize;
        for command in &self.commands {
            if commands_count >= COMMANDS_LANE_CAP {
                break;
            }
            if !normalized.is_empty()
                && !contains_ci(&command.command_id, &normalized)
                && !contains_ci(&command.title, &normalized)
                && !contains_ci(&command.summary, &normalized)
            {
                continue;
            }
            let mut ranking: Vec<String> = Vec::new();
            if normalized.is_empty() {
                ranking.push("command_listing".to_string());
            } else {
                if contains_ci(&command.command_id, &normalized) {
                    ranking.push("exact_command_id".to_string());
                }
                if contains_ci(&command.title, &normalized) {
                    ranking.push("title_substring".to_string());
                }
                if contains_ci(&command.summary, &normalized) {
                    ranking.push("summary_substring".to_string());
                }
            }
            self.rows.push(QuickOpenResultRow {
                key: QuickOpenRowKey::Command {
                    command_id: command.command_id.clone(),
                },
                row_kind: QuickOpenRowKind::Command,
                source_class: QuickOpenSourceClass::Command,
                source_state: self.commands_state,
                display_label: command.title.clone(),
                secondary_label: command.summary.clone(),
                ranking_sources: ranking,
                command_id: Some(command.command_id.clone()),
                disabled_reason_class: command.disabled_reason_class.clone(),
                invocation_preview_class: Some(command.invocation_preview_class.clone()),
                relative_path: None,
            });
            commands_count += 1;
        }

        // Lexical lane (filenames first, then paths). Recents win on
        // duplicates: lexical rows whose path is already covered by a recent
        // target are skipped so the same path never renders twice.
        let mut filename_count = 0usize;
        let mut path_count = 0usize;
        for row in &self.lexical_rows {
            if taken_paths.contains(&row.relative_path) {
                continue;
            }
            match row.source_class {
                QuickOpenSourceClass::LexicalFilename => {
                    if filename_count >= LEXICAL_LANE_CAP {
                        continue;
                    }
                    filename_count += 1;
                }
                QuickOpenSourceClass::LexicalPath => {
                    if path_count >= LEXICAL_LANE_CAP {
                        continue;
                    }
                    path_count += 1;
                }
                _ => continue,
            }
            taken_paths.insert(row.relative_path.clone());
            let display_label = row
                .relative_path
                .rsplit_once('/')
                .map(|(_, name)| name.to_string())
                .unwrap_or_else(|| row.relative_path.clone());
            self.rows.push(QuickOpenResultRow {
                key: QuickOpenRowKey::File {
                    relative_path: row.relative_path.clone(),
                },
                row_kind: QuickOpenRowKind::File,
                source_class: row.source_class,
                source_state: self.lexical_state,
                display_label,
                secondary_label: row.relative_path.clone(),
                ranking_sources: vec![row.match_kind_token.clone()],
                command_id: None,
                disabled_reason_class: None,
                invocation_preview_class: None,
                relative_path: Some(row.relative_path.clone()),
            });
        }
    }
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack.to_ascii_lowercase().contains(needle)
}

fn project_default_scope(
    workspace_id: &str,
    scope_class: WorkspaceScopeClass,
    workset_name: Option<&str>,
) -> WorkspaceSearchScope {
    match scope_class {
        WorkspaceScopeClass::FullWorkspace => {
            WorkspaceSearchScope::for_full_workspace(workspace_id)
        }
        WorkspaceScopeClass::CurrentRepo => WorkspaceSearchScope::for_current_repo(workspace_id),
        WorkspaceScopeClass::SelectedWorkset
        | WorkspaceScopeClass::SparseSlice
        | WorkspaceScopeClass::PolicyLimitedView => WorkspaceSearchScope::for_workset_stub(
            workspace_id,
            SearchScopeClass::from_workspace(scope_class),
            workset_name,
        ),
    }
}

/// Per-source summary in the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenSnapshotSource {
    pub source_class_token: String,
    pub source_state_token: String,
    pub visible_row_count: usize,
}

/// One row in the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenSnapshotRow {
    pub key: QuickOpenRowKey,
    pub row_kind_token: String,
    pub source_class_token: String,
    pub source_state_token: String,
    pub display_label: String,
    pub secondary_label: String,
    pub ranking_sources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation_preview_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

/// Serializable snapshot of a quick-open query session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickOpenSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub scope_class_token: String,
    pub scope_chip_label: String,
    /// Canonical workset/slice scope metadata projected onto the snapshot
    /// so a replayed session keeps the chip label, presentation state, and
    /// pattern fingerprint that produced the visible row set.
    pub scope_metadata: WorkspaceSearchScopeMetadata,
    pub query: String,
    pub held_modifiers: Vec<String>,
    pub sources: Vec<QuickOpenSnapshotSource>,
    pub lexical_partial_truth_causes: Vec<String>,
    pub rows: Vec<QuickOpenSnapshotRow>,
    pub available_source_classes: Vec<String>,
    pub observed_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_reactive_state::ReadinessLabel;
    use aureline_search::{LexicalIndexState, LexicalQuery, LexicalShell, ScopeClass};
    use aureline_workspace::WorkspaceLifecycleState;

    fn ready_lexical_shell() -> LexicalShell {
        let index = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Ready,
            ReadinessLabel::Exact,
            vec![
                "src/main.rs".to_string(),
                "src/lib.rs".to_string(),
                "tests/smoke.rs".to_string(),
            ],
        );
        LexicalShell::new(
            ScopeClass::CurrentRepo,
            "Current repo",
            index,
            LexicalQuery::new("main"),
        )
    }

    fn warming_lexical_shell() -> LexicalShell {
        let index = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            Vec::new(),
        );
        LexicalShell::new(
            ScopeClass::CurrentRepo,
            "Current repo",
            index,
            LexicalQuery::new("main"),
        )
    }

    fn sample_command_rows() -> Vec<QuickOpenCommandRow> {
        vec![
            QuickOpenCommandRow {
                command_id: "cmd:workspace.open_folder".to_string(),
                title: "Open folder".to_string(),
                summary: "Open a folder as a workspace root".to_string(),
                dominant_side_effect_class: "workspace_state".to_string(),
                invocation_preview_class: "summary".to_string(),
                disabled_reason_class: None,
            },
            QuickOpenCommandRow {
                command_id: "cmd:workspace.save_all".to_string(),
                title: "Save all".to_string(),
                summary: "Save all dirty buffers".to_string(),
                dominant_side_effect_class: "filesystem_write".to_string(),
                invocation_preview_class: "diff".to_string(),
                disabled_reason_class: Some("workspace_trust_restricted".to_string()),
            },
        ]
    }

    fn sample_recents() -> Vec<QuickOpenRecentTarget> {
        vec![
            QuickOpenRecentTarget {
                recent_id: "recent:src_main_rs".to_string(),
                display_label: "main.rs".to_string(),
                secondary_label: "src/main.rs".to_string(),
                relative_path: Some("src/main.rs".to_string()),
                target_kind_token: "local_file".to_string(),
            },
            QuickOpenRecentTarget {
                recent_id: "recent:debug_panel".to_string(),
                display_label: "Debug panel".to_string(),
                secondary_label: "recent place".to_string(),
                relative_path: None,
                target_kind_token: "recent_location".to_string(),
            },
        ]
    }

    #[test]
    fn open_session_merges_recents_commands_and_lexical() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_recents(sample_recents());
        session.set_commands(sample_command_rows());
        session.refresh_lexical_from_shell(&ready_lexical_shell());
        session.open();
        session.set_query("main");

        assert!(session.is_open());
        assert_eq!(session.recents_state(), QuickOpenSourceState::Ready);
        assert_eq!(session.commands_state(), QuickOpenSourceState::Ready);
        assert_eq!(session.lexical_state(), QuickOpenSourceState::Ready);

        let mut sources_seen: BTreeSet<QuickOpenSourceClass> = BTreeSet::new();
        for row in session.rows() {
            sources_seen.insert(row.source_class);
        }
        assert!(sources_seen.contains(&QuickOpenSourceClass::RecentTarget));
        // No lexical_filename row for src/main.rs because recents already
        // claimed that path; the deduplication keeps recents as the winner.
        assert!(!session.rows().iter().any(|r| matches!(
            &r.key,
            QuickOpenRowKey::File { relative_path } if relative_path == "src/main.rs"
        )));
    }

    #[test]
    fn lexical_warming_is_surfaced_explicitly() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_recents(sample_recents());
        session.set_commands(sample_command_rows());
        session.refresh_lexical_from_shell(&warming_lexical_shell());
        session.open();
        // Use a query that matches at least one recent and at least one
        // command so the warming lexical lane is the only source that goes
        // dark.
        session.set_query("save");

        assert_eq!(session.lexical_state(), QuickOpenSourceState::Warming);
        assert!(!session.lexical_partial_truth_causes().is_empty());

        // The lexical lane produces no rows while warming, but the per-lane
        // source state is still surfaced for the chrome.
        assert!(!session
            .rows()
            .iter()
            .any(|r| matches!(r.row_kind, QuickOpenRowKind::File)));

        // Commands stay usable while lexical warms.
        let has_command = session
            .rows()
            .iter()
            .any(|r| r.source_class == QuickOpenSourceClass::Command);
        assert!(
            has_command,
            "commands must remain usable while lexical warms"
        );

        // The snapshot exposes the warming state directly per source.
        let snapshot = session.export_snapshot("mono:warm");
        let lexical_filename_state = snapshot
            .sources
            .iter()
            .find(|s| s.source_class_token == "lexical_filename")
            .map(|s| s.source_state_token.clone())
            .unwrap_or_default();
        assert_eq!(lexical_filename_state, "warming");
    }

    #[test]
    fn rows_carry_command_id_disabled_reason_and_invocation_preview() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_commands(sample_command_rows());
        session.open();
        session.set_query("save");

        let row = session
            .rows()
            .iter()
            .find(|r| r.source_class == QuickOpenSourceClass::Command)
            .expect("command row must surface");
        assert_eq!(row.command_id.as_deref(), Some("cmd:workspace.save_all"));
        assert_eq!(
            row.disabled_reason_class.as_deref(),
            Some("workspace_trust_restricted")
        );
        assert_eq!(row.invocation_preview_class.as_deref(), Some("diff"));
    }

    #[test]
    fn empty_query_serves_recents_and_command_listing_only() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_recents(sample_recents());
        session.set_commands(sample_command_rows());
        session.refresh_lexical_from_shell(&ready_lexical_shell());
        session.open();
        // No query set; lexical lane should not surface rows on an empty
        // query because the upstream shell returns no rows.
        assert!(!session
            .rows()
            .iter()
            .any(|r| matches!(r.row_kind, QuickOpenRowKind::File)));
        assert!(session
            .rows()
            .iter()
            .any(|r| matches!(r.row_kind, QuickOpenRowKind::RecentTarget)));
    }

    #[test]
    fn unavailable_lexical_keeps_recents_and_commands() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_recents(sample_recents());
        session.set_commands(sample_command_rows());
        session.set_lexical(
            Vec::new(),
            QuickOpenSourceState::Unavailable,
            vec!["watcher_degraded".to_string()],
        );
        session.open();
        session.set_query("main");

        assert_eq!(session.lexical_state(), QuickOpenSourceState::Unavailable);
        assert!(session
            .rows()
            .iter()
            .any(|r| r.source_class == QuickOpenSourceClass::RecentTarget));
        assert!(!session
            .rows()
            .iter()
            .any(|r| matches!(r.row_kind, QuickOpenRowKind::File)));
    }

    #[test]
    fn scope_chip_uses_workset_name_when_narrowed() {
        let session = QuickOpenQuerySession::new(
            "ws-test",
            WorkspaceScopeClass::SelectedWorkset,
            Some("Hot path".to_string()),
        );
        let chip = session.scope_chip();
        assert_eq!(chip.scope_class_token, "selected_workset");
        assert_eq!(chip.scope_chip_label, "Selected workset · Hot path");
    }

    #[test]
    fn snapshot_round_trips_through_serde() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.set_recents(sample_recents());
        session.set_commands(sample_command_rows());
        session.refresh_lexical_from_shell(&ready_lexical_shell());
        session.open();
        session.set_query("lib");

        let snapshot = session.export_snapshot("mono:42");
        let json = serde_json::to_string(&snapshot).expect("snapshot must serialize");
        let parsed: QuickOpenSnapshot =
            serde_json::from_str(&json).expect("snapshot must round-trip");
        assert_eq!(parsed.workspace_id, "ws-test");
        assert_eq!(parsed.scope_class_token, "current_repo");
        assert!(parsed
            .available_source_classes
            .iter()
            .any(|s| s == "recent_target"));
    }

    #[test]
    fn close_clears_query_and_modifiers() {
        let mut session =
            QuickOpenQuerySession::new("ws-test", WorkspaceScopeClass::CurrentRepo, None);
        session.open();
        session.set_query("main");
        session.set_held_modifiers(["files_only"].iter().copied());
        assert_eq!(session.query(), "main");
        session.close();
        assert!(!session.is_open());
        assert_eq!(session.query(), "");
        assert!(session.held_modifiers().is_empty());
    }
}
