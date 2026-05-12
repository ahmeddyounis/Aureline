//! Command palette query-session state and grouped results.
//!
//! This module owns the live, runtime palette session: query text, selected row,
//! provider readiness, and the grouped result materialization that drives the
//! shell command palette surface.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::{Duration, Instant};

use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use aureline_input::text_input::{ImeComposition, ImeEvent, TextInputAction, TextInputSession};
use aureline_vfs::{
    VfsChangeKind, WatcherEvent, WatcherHealth, WatcherService, WatcherServiceOptions,
};
use serde::Serialize;

static PALETTE_SESSION_SEQ: AtomicUsize = AtomicUsize::new(1);

fn next_session_seq() -> usize {
    PALETTE_SESSION_SEQ.fetch_add(1, Ordering::Relaxed)
}

fn mint_palette_session_id() -> String {
    format!("palette:session:{:02}", next_session_seq())
}

fn normalize_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn is_query_effectively_empty(query: &str) -> bool {
    normalize_query(query).is_empty()
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack.to_ascii_lowercase().contains(needle)
}

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Identifies the source/provider that contributed palette results.
pub enum PaletteProviderClass {
    /// Locally tracked recently invoked commands.
    RecentHistory,
    /// Synchronous title/ID/shortcut matching over the command registry.
    LexicalCommandIndex,
    /// Deferred semantic supplement layer (streamed after lexical results).
    SemanticCommandIndex,
    /// Deferred file path index for workspace-local navigation.
    FileIndex,
    /// Keybinding resolver input used for literal shortcut matching.
    KeybindingResolver,
}

impl PaletteProviderClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentHistory => "recent_history",
            Self::LexicalCommandIndex => "lexical_command_index",
            Self::SemanticCommandIndex => "semantic_command_index",
            Self::FileIndex => "file_index",
            Self::KeybindingResolver => "keybinding_resolver",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Provider readiness state exposed in grouped palette results.
pub enum PaletteProviderStateClass {
    /// Provider has not been activated for the current session/query.
    NotRequested,
    /// Provider is warming up and has not yet produced results.
    Warming,
    /// Provider is ready to answer queries but is not actively streaming.
    Ready,
    /// Provider is currently producing incremental results.
    Streaming,
    /// Provider is active but may be incomplete or missing incremental frames.
    Partial,
    /// Provider data may be out of date relative to the current workspace state.
    Stale,
    /// Provider is blocked by policy or trust posture.
    PolicyBlocked,
    /// Provider cannot be used in the current environment.
    Unavailable,
    /// Provider has completed producing results for the current query.
    Complete,
}

impl PaletteProviderStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Warming => "warming",
            Self::Ready => "ready",
            Self::Streaming => "streaming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
            Self::Complete => "complete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Attribution badge describing why a palette item ranked/appeared.
pub enum PaletteRankingSourceClass {
    /// Item originated from local recent history.
    RecentHistory,
    /// Query matched the canonical command id.
    ExactCommandId,
    /// Query matched a literal key sequence (shortcut).
    KeySequenceMatch,
    /// Query matched the command title.
    TitleSubstring,
    /// Query matched the command summary/description.
    SummarySubstring,
    /// Item arrived from a semantic supplement stage.
    SemanticSupplement,
}

impl PaletteRankingSourceClass {
    pub const fn badge(self) -> &'static str {
        match self {
            Self::RecentHistory => "recent",
            Self::ExactCommandId => "id",
            Self::KeySequenceMatch => "keys",
            Self::TitleSubstring => "title",
            Self::SummarySubstring => "summary",
            Self::SemanticSupplement => "semantic",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Stable key for selecting or committing a palette item.
pub enum PaletteItemKey {
    /// Select a command by its canonical command id.
    Command { command_id: String },
    /// Select a file by its workspace-relative path.
    File { relative_path: String },
}

#[derive(Debug, Clone)]
/// Commit outcome for the selected palette row.
pub enum CommandPaletteCommit {
    /// Commit dispatches a command id into the command runtime.
    CommandId(String),
    /// Commit requests opening or navigating to a file path.
    FilePath(String),
}

#[derive(Debug, Clone)]
/// One materialized result item inside a provider group.
pub struct PaletteResultItem {
    /// Stable selection key for this item.
    pub key: PaletteItemKey,
    /// Provider that produced the item.
    pub provider: PaletteProviderClass,
    /// Provider state at the moment the item was materialized.
    pub provider_state: PaletteProviderStateClass,
    /// Attribution badges for why this item was surfaced/ranked.
    pub ranking_sources: Vec<PaletteRankingSourceClass>,
}

#[derive(Debug, Clone)]
/// One grouped result section rendered by the palette.
pub struct PaletteResultGroup {
    /// Human label for the group header.
    pub label: String,
    /// Provider class that owns the group.
    pub provider: PaletteProviderClass,
    /// Provider state surfaced for the group header.
    pub provider_state: PaletteProviderStateClass,
    /// Items in this group, ordered by their current ranking.
    pub items: Vec<PaletteResultItem>,
}

#[derive(Debug, Clone, Serialize)]
/// Structured export snapshot for a live palette session.
///
/// This record is intended for inspection and support capture: it exposes
/// canonical command identity, provider readiness, and ranking-source
/// attribution without scraping the UI render strings.
pub struct CommandPaletteSnapshot {
    pub record_kind: &'static str,
    pub schema_version: u32,
    pub palette_session_id: String,
    pub generated_at: String,
    pub query: String,
    pub selected_key: Option<CommandPaletteSnapshotSelectedKey>,
    pub providers: Vec<CommandPaletteSnapshotProvider>,
    pub groups: Vec<CommandPaletteSnapshotGroup>,
}

#[derive(Debug, Clone, Serialize)]
/// Summary line for one provider in a palette snapshot.
pub struct CommandPaletteSnapshotProvider {
    pub provider_class: String,
    pub state_class: String,
    pub visible_result_count: usize,
}

#[derive(Debug, Clone, Serialize)]
/// Snapshot representation of one palette result group.
pub struct CommandPaletteSnapshotGroup {
    pub label: String,
    pub provider_class: String,
    pub provider_state: String,
    pub items: Vec<CommandPaletteSnapshotItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
/// Snapshot representation of one palette item.
pub enum CommandPaletteSnapshotItem {
    /// A command row (canonical id + descriptor projection).
    Command {
        command_id: String,
        title: String,
        summary: String,
        dominant_side_effect_class: String,
        shortcuts: String,
        provider_class: String,
        provider_state: String,
        ranking_sources: Vec<String>,
    },
    /// A file row (workspace-relative path).
    File {
        relative_path: String,
        provider_class: String,
        provider_state: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
/// Snapshot representation of the currently selected item key.
pub enum CommandPaletteSnapshotSelectedKey {
    /// Selected command key.
    Command { command_id: String },
    /// Selected file key.
    File { relative_path: String },
}

#[derive(Debug)]
struct FileIndexWorker {
    rx: Receiver<FileIndexMessage>,
    state: PaletteProviderStateClass,
    file_paths: Vec<String>,
    last_progress_at: Instant,
    complete: bool,
    root: PathBuf,
    watcher: Option<WatcherService>,
    watcher_health: WatcherHealth,
    watcher_source: Option<String>,
    needs_rescan: bool,
    last_watcher_events: Vec<WatcherEvent>,
}

/// Readiness signals derived from the workspace file index worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceFileIndexReadiness {
    /// Latest watcher health observed for the workspace root.
    pub watcher_health: WatcherHealth,
    /// Whether the current scan has completed.
    pub hot_index_ready: bool,
}

#[derive(Debug)]
enum FileIndexMessage {
    Chunk(Vec<String>),
    Complete,
}

pub(crate) fn is_workspace_file_index_ignored_dir(name: &str) -> bool {
    if name.starts_with('.') {
        return true;
    }
    matches!(
        name,
        "target" | "node_modules" | "dist" | "build" | "out" | "artifacts"
    )
}

fn spawn_file_index_worker(root: PathBuf) -> FileIndexWorker {
    let root = root.canonicalize().unwrap_or(root);
    let (tx, rx) = std::sync::mpsc::channel::<FileIndexMessage>();
    let worker_root = root.clone();
    std::thread::Builder::new()
        .name("aureline_file_index".to_string())
        .spawn(move || scan_files(worker_root, tx))
        .ok();

    let watcher =
        WatcherService::spawn_local("root-local", root.clone(), WatcherServiceOptions::default())
            .ok();
    let watcher_health = watcher
        .as_ref()
        .map(|w| w.latest_health())
        .unwrap_or(WatcherHealth::Unavailable);

    FileIndexWorker {
        rx,
        state: PaletteProviderStateClass::Warming,
        file_paths: Vec::new(),
        last_progress_at: Instant::now(),
        complete: false,
        root,
        watcher,
        watcher_health,
        watcher_source: None,
        needs_rescan: false,
        last_watcher_events: Vec::new(),
    }
}

fn restart_file_index_scan(worker: &mut FileIndexWorker, now: Instant) {
    let (tx, rx) = std::sync::mpsc::channel::<FileIndexMessage>();
    let worker_root = worker.root.clone();
    let started = std::thread::Builder::new()
        .name("aureline_file_index".to_string())
        .spawn(move || scan_files(worker_root, tx))
        .is_ok();

    if started {
        worker.rx = rx;
        worker.file_paths.clear();
        worker.last_progress_at = now;
        worker.complete = false;
        worker.needs_rescan = false;
        worker.state = PaletteProviderStateClass::Warming;
    } else {
        worker.state = PaletteProviderStateClass::Stale;
    }
}

fn scan_files(root: PathBuf, tx: std::sync::mpsc::Sender<FileIndexMessage>) {
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(root.clone());

    let mut chunk: Vec<String> = Vec::with_capacity(256);
    let mut scanned = 0usize;

    while let Some(dir) = queue.pop_front() {
        if scanned > 20_000 {
            break;
        }
        let read_dir = match std::fs::read_dir(&dir) {
            Ok(v) => v,
            Err(_) => continue,
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(v) => v,
                Err(_) => continue,
            };
            if file_type.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if is_workspace_file_index_ignored_dir(name) {
                        continue;
                    }
                }
                queue.push_back(path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }

            let relative = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            chunk.push(relative);
            scanned += 1;
            if chunk.len() >= 256 {
                let send = std::mem::take(&mut chunk);
                if tx.send(FileIndexMessage::Chunk(send)).is_err() {
                    return;
                }
            }
        }
    }

    if !chunk.is_empty() {
        let _ = tx.send(FileIndexMessage::Chunk(chunk));
    }
    let _ = tx.send(FileIndexMessage::Complete);
}

fn apply_watcher_event(worker: &mut FileIndexWorker, event: WatcherEvent) {
    match event {
        WatcherEvent::Health(frame) => {
            worker.watcher_health = frame.watcher_health;
            worker.watcher_source = Some(frame.watcher_source.as_str().to_owned());
            worker.state = state_for_watch(
                worker.state,
                worker.complete,
                worker.needs_rescan,
                frame.watcher_health,
            );
        }
        WatcherEvent::Change(change) => {
            if change.root_id != "root-local" {
                return;
            }
            match change.kind {
                VfsChangeKind::Created { uri } => {
                    if let Some(relative) = relative_path_for_uri(&worker.root, &uri) {
                        if !worker.file_paths.iter().any(|p| p == &relative) {
                            worker.file_paths.push(relative);
                        }
                    } else {
                        worker.needs_rescan = true;
                    }
                }
                VfsChangeKind::Deleted { uri } => {
                    if let Some(relative) = relative_path_for_uri(&worker.root, &uri) {
                        worker.file_paths.retain(|p| p != &relative);
                    } else {
                        worker.needs_rescan = true;
                    }
                }
                VfsChangeKind::Renamed { from, to } => {
                    let from_rel = relative_path_for_uri(&worker.root, &from);
                    let to_rel = relative_path_for_uri(&worker.root, &to);
                    match (from_rel, to_rel) {
                        (Some(from_rel), Some(to_rel)) => {
                            worker.file_paths.retain(|p| p != &from_rel);
                            if !worker.file_paths.iter().any(|p| p == &to_rel) {
                                worker.file_paths.push(to_rel);
                            }
                        }
                        _ => {
                            worker.needs_rescan = true;
                        }
                    }
                }
                VfsChangeKind::Modified { .. } => {}
                VfsChangeKind::Rescan => {
                    worker.needs_rescan = true;
                }
            }
            worker.state = state_for_watch(
                worker.state,
                worker.complete,
                worker.needs_rescan,
                worker.watcher_health,
            );
        }
    }
}

fn state_for_watch(
    current: PaletteProviderStateClass,
    complete: bool,
    needs_rescan: bool,
    health: WatcherHealth,
) -> PaletteProviderStateClass {
    if needs_rescan {
        return PaletteProviderStateClass::Stale;
    }
    if !complete {
        return current;
    }
    match health {
        WatcherHealth::Healthy => PaletteProviderStateClass::Complete,
        WatcherHealth::Warming => PaletteProviderStateClass::Partial,
        WatcherHealth::Degraded | WatcherHealth::FallbackPolling => {
            PaletteProviderStateClass::Partial
        }
        WatcherHealth::Unavailable => PaletteProviderStateClass::Unavailable,
    }
}

fn relative_path_for_uri(root: &PathBuf, uri: &aureline_vfs::VfsUri) -> Option<String> {
    let path = uri.file_path()?;
    let relative = path.strip_prefix(root).ok()?;
    Some(relative.to_string_lossy().replace('\\', "/"))
}

/// Runtime command-palette state that owns query text, provider readiness, and
/// grouped result materialization.
#[derive(Debug)]
pub struct CommandPaletteState {
    open: bool,
    palette_session_id: String,
    opened_at: Instant,
    updated_at: Instant,

    query: String,
    text_input: TextInputSession,
    selection: usize,
    selected_key: Option<PaletteItemKey>,

    recent_command_ids: VecDeque<String>,
    visible_command_ids: Vec<String>,
    labs_enabled: bool,

    groups: Vec<PaletteResultGroup>,
    flat_item_keys: Vec<PaletteItemKey>,

    semantic_state: PaletteProviderStateClass,
    semantic_deadline: Option<Instant>,

    file_index: Option<FileIndexWorker>,
}

impl CommandPaletteState {
    pub fn new(registry: &CommandRegistry) -> Self {
        let now = Instant::now();
        let mut state = Self {
            open: false,
            palette_session_id: mint_palette_session_id(),
            opened_at: now,
            updated_at: now,
            query: String::new(),
            text_input: TextInputSession::new(),
            selection: 0,
            selected_key: None,
            recent_command_ids: VecDeque::new(),
            visible_command_ids: Vec::new(),
            labs_enabled: false,
            groups: Vec::new(),
            flat_item_keys: Vec::new(),
            semantic_state: PaletteProviderStateClass::NotRequested,
            semantic_deadline: None,
            file_index: None,
        };
        state.rebuild_visible_entries(registry);
        state
    }

    pub fn rebuild_visible_entries(&mut self, registry: &CommandRegistry) {
        self.visible_command_ids = registry
            .entries()
            .iter()
            .filter(|entry| {
                entry
                    .descriptor
                    .client_scopes
                    .iter()
                    .any(|scope| scope == "desktop_product")
                    && entry.descriptor.palette_visibility != "hidden_palette_callable_only"
                    && (entry.descriptor.palette_visibility != "developer_only"
                        || self.labs_enabled)
            })
            .map(|entry| entry.command_id().to_string())
            .collect();
    }

    /// Enables or disables Labs-only palette rows and refreshes the visible index.
    pub fn set_labs_enabled(&mut self, registry: &CommandRegistry, enabled: bool) {
        if self.labs_enabled == enabled {
            return;
        }
        self.labs_enabled = enabled;
        self.rebuild_visible_entries(registry);
        self.recompute_groups(registry, &HashMap::new());
    }

    /// Returns whether Labs-only palette rows are currently visible.
    pub const fn labs_enabled(&self) -> bool {
        self.labs_enabled
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn opened_at(&self) -> Instant {
        self.opened_at
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn ime_composition(&self) -> Option<&ImeComposition> {
        self.text_input.composition()
    }

    pub fn open(&mut self, registry: &CommandRegistry, cwd: PathBuf) {
        if self.open {
            return;
        }
        self.open = true;
        self.palette_session_id = mint_palette_session_id();
        let now = Instant::now();
        self.opened_at = now;
        self.updated_at = now;
        self.query.clear();
        self.text_input = TextInputSession::new();
        self.semantic_state = PaletteProviderStateClass::NotRequested;
        self.semantic_deadline = None;
        if self.file_index.is_none() {
            self.set_workspace_root(cwd);
        }
        self.recompute_groups(registry, &HashMap::new());
    }

    pub fn close(&mut self) {
        self.open = false;
        self.selected_key = None;
        self.selection = 0;
        self.groups.clear();
        self.flat_item_keys.clear();
        self.semantic_state = PaletteProviderStateClass::NotRequested;
        self.semantic_deadline = None;
        self.text_input.force_clear_composition();
    }

    /// Ensures the file index worker is tracking `root`.
    pub fn set_workspace_root(&mut self, root: PathBuf) {
        let now = Instant::now();
        let root = root.canonicalize().unwrap_or(root);
        match self.file_index.as_mut() {
            None => {
                self.file_index = Some(spawn_file_index_worker(root));
            }
            Some(worker) => {
                if worker.root == root {
                    return;
                }
                worker.root = root.clone();
                worker.watcher = WatcherService::spawn_local(
                    "root-local",
                    root.clone(),
                    WatcherServiceOptions::default(),
                )
                .ok();
                worker.watcher_health = worker
                    .watcher
                    .as_ref()
                    .map(|w| w.latest_health())
                    .unwrap_or(WatcherHealth::Unavailable);
                worker.watcher_source = None;
                worker.needs_rescan = false;
                worker.last_watcher_events.clear();
                restart_file_index_scan(worker, now);
            }
        }
    }

    /// Takes watcher events observed since the last tick so sibling views can
    /// consume the same VFS watcher stream without starting their own watcher.
    pub(crate) fn take_workspace_watcher_events(&mut self) -> Vec<WatcherEvent> {
        self.file_index
            .as_mut()
            .map(|worker| std::mem::take(&mut worker.last_watcher_events))
            .unwrap_or_default()
    }

    /// Returns the latest readiness signals derived from the file index worker.
    pub fn workspace_file_index_readiness(&self) -> Option<WorkspaceFileIndexReadiness> {
        let worker = self.file_index.as_ref()?;
        Some(WorkspaceFileIndexReadiness {
            watcher_health: worker.watcher_health,
            hot_index_ready: worker.complete,
        })
    }

    /// Returns the current workspace root, when a file index worker is active.
    pub fn workspace_root(&self) -> Option<&Path> {
        Some(self.file_index.as_ref()?.root.as_path())
    }

    pub fn note_command_invoked(&mut self, command_id: &str) {
        if command_id.trim().is_empty() {
            return;
        }
        if let Some(pos) = self
            .recent_command_ids
            .iter()
            .position(|row| row == command_id)
        {
            self.recent_command_ids.remove(pos);
        }
        self.recent_command_ids.push_front(command_id.to_string());
        while self.recent_command_ids.len() > 16 {
            self.recent_command_ids.pop_back();
        }
    }

    pub fn selected_entry<'a>(
        &self,
        registry: &'a CommandRegistry,
    ) -> Option<&'a CommandRegistryEntryRecord> {
        let key = self.flat_item_keys.get(self.selection)?;
        match key {
            PaletteItemKey::Command { command_id } => registry.get(command_id),
            PaletteItemKey::File { .. } => None,
        }
    }

    pub fn selected_key(&self) -> Option<&PaletteItemKey> {
        self.flat_item_keys.get(self.selection)
    }

    pub fn groups(&self) -> &[PaletteResultGroup] {
        &self.groups
    }

    pub fn export_snapshot(
        &self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) -> CommandPaletteSnapshot {
        let generated_at = aureline_commands::invocation::now_rfc3339();
        let selected_key = self.selected_key().cloned().map(|key| match key {
            PaletteItemKey::Command { command_id } => {
                CommandPaletteSnapshotSelectedKey::Command { command_id }
            }
            PaletteItemKey::File { relative_path } => {
                CommandPaletteSnapshotSelectedKey::File { relative_path }
            }
        });

        let mut providers: Vec<CommandPaletteSnapshotProvider> = Vec::new();
        providers.push(CommandPaletteSnapshotProvider {
            provider_class: PaletteProviderClass::RecentHistory.as_str().to_string(),
            state_class: PaletteProviderStateClass::Complete.as_str().to_string(),
            visible_result_count: self
                .groups
                .iter()
                .find(|g| g.provider == PaletteProviderClass::RecentHistory)
                .map(|g| g.items.len())
                .unwrap_or(0),
        });
        providers.push(CommandPaletteSnapshotProvider {
            provider_class: PaletteProviderClass::LexicalCommandIndex
                .as_str()
                .to_string(),
            state_class: PaletteProviderStateClass::Complete.as_str().to_string(),
            visible_result_count: self
                .groups
                .iter()
                .find(|g| g.provider == PaletteProviderClass::LexicalCommandIndex)
                .map(|g| g.items.len())
                .unwrap_or(0),
        });
        providers.push(CommandPaletteSnapshotProvider {
            provider_class: PaletteProviderClass::SemanticCommandIndex
                .as_str()
                .to_string(),
            state_class: self.semantic_state.as_str().to_string(),
            visible_result_count: self
                .groups
                .iter()
                .find(|g| g.provider == PaletteProviderClass::SemanticCommandIndex)
                .map(|g| g.items.len())
                .unwrap_or(0),
        });
        let file_state = self
            .file_index
            .as_ref()
            .map(|idx| idx.state)
            .unwrap_or(PaletteProviderStateClass::NotRequested);
        providers.push(CommandPaletteSnapshotProvider {
            provider_class: PaletteProviderClass::FileIndex.as_str().to_string(),
            state_class: file_state.as_str().to_string(),
            visible_result_count: self
                .groups
                .iter()
                .find(|g| g.provider == PaletteProviderClass::FileIndex)
                .map(|g| g.items.len())
                .unwrap_or(0),
        });
        providers.push(CommandPaletteSnapshotProvider {
            provider_class: PaletteProviderClass::KeybindingResolver
                .as_str()
                .to_string(),
            state_class: PaletteProviderStateClass::Ready.as_str().to_string(),
            visible_result_count: 0,
        });

        let groups = self
            .groups
            .iter()
            .map(|group| CommandPaletteSnapshotGroup {
                label: group.label.clone(),
                provider_class: group.provider.as_str().to_string(),
                provider_state: group.provider_state.as_str().to_string(),
                items: group
                    .items
                    .iter()
                    .filter_map(|item| match &item.key {
                        PaletteItemKey::Command { command_id } => {
                            let entry = registry.get(command_id)?;
                            Some(CommandPaletteSnapshotItem::Command {
                                command_id: command_id.clone(),
                                title: entry.title.clone(),
                                summary: entry.summary.clone(),
                                dominant_side_effect_class: entry
                                    .dominant_side_effect_class
                                    .clone(),
                                shortcuts: shortcuts_by_command_id
                                    .get(command_id)
                                    .map(|seqs| seqs.join(", "))
                                    .unwrap_or_else(|| "unbound".to_string()),
                                provider_class: item.provider.as_str().to_string(),
                                provider_state: item.provider_state.as_str().to_string(),
                                ranking_sources: item
                                    .ranking_sources
                                    .iter()
                                    .map(|src| src.badge().to_string())
                                    .collect(),
                            })
                        }
                        PaletteItemKey::File { relative_path } => {
                            Some(CommandPaletteSnapshotItem::File {
                                relative_path: relative_path.clone(),
                                provider_class: item.provider.as_str().to_string(),
                                provider_state: item.provider_state.as_str().to_string(),
                            })
                        }
                    })
                    .collect(),
            })
            .collect();

        CommandPaletteSnapshot {
            record_kind: "command_palette_snapshot_record",
            schema_version: 1,
            palette_session_id: self.palette_session_id.clone(),
            generated_at,
            query: self.query.clone(),
            selected_key,
            providers,
            groups,
        }
    }

    pub fn write_snapshot_log(
        &self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) {
        if !self.open {
            return;
        }

        let snapshot = self.export_snapshot(registry, shortcuts_by_command_id);
        let root = std::path::PathBuf::from(".logs").join("palette_sessions");
        if std::fs::create_dir_all(&root).is_err() {
            return;
        }

        let filename = format!(
            "{}.palette_session.json",
            sanitize_filename(&snapshot.palette_session_id)
        );
        let Ok(json) = serde_json::to_string_pretty(&snapshot) else {
            return;
        };
        let _ = std::fs::write(root.join(filename), json);
    }

    pub fn handle_backspace(
        &mut self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) -> bool {
        if !self.open || self.query.is_empty() {
            return false;
        }
        self.query.pop();
        self.on_query_changed(registry, shortcuts_by_command_id);
        true
    }

    pub fn handle_text_input(
        &mut self,
        ch: char,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) -> bool {
        if !self.open {
            return false;
        }
        if ch.is_control() {
            return false;
        }
        self.query.push(ch);
        self.on_query_changed(registry, shortcuts_by_command_id);
        true
    }

    pub fn handle_ime_event(
        &mut self,
        event: ImeEvent,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) -> bool {
        if !self.open {
            return false;
        }

        let Some(action) = self.text_input.handle_ime_event(event) else {
            return false;
        };

        match action {
            TextInputAction::InsertText { text } => {
                let mut changed = false;
                for ch in text.chars() {
                    changed |= self.handle_text_input(ch, registry, shortcuts_by_command_id);
                }
                changed
            }
            TextInputAction::DeleteBackward => {
                self.handle_backspace(registry, shortcuts_by_command_id)
            }
            TextInputAction::DeleteForward => false,
            TextInputAction::MoveCaret { .. } => false,
            TextInputAction::UpdateComposition { .. } | TextInputAction::ClearComposition => true,
        }
    }

    pub fn handle_arrow_up(&mut self) -> bool {
        if !self.open || self.flat_item_keys.is_empty() {
            return false;
        }
        self.selection =
            (self.selection + self.flat_item_keys.len() - 1) % self.flat_item_keys.len();
        self.selected_key = self.flat_item_keys.get(self.selection).cloned();
        true
    }

    pub fn handle_arrow_down(&mut self) -> bool {
        if !self.open || self.flat_item_keys.is_empty() {
            return false;
        }
        self.selection = (self.selection + 1) % self.flat_item_keys.len();
        self.selected_key = self.flat_item_keys.get(self.selection).cloned();
        true
    }

    pub fn commit(&mut self, registry: &CommandRegistry) -> Option<CommandPaletteCommit> {
        if !self.open {
            return None;
        }
        let key = self.flat_item_keys.get(self.selection).cloned();
        self.close();
        match key {
            Some(PaletteItemKey::Command { command_id }) => registry
                .get(&command_id)
                .map(|_| CommandPaletteCommit::CommandId(command_id)),
            Some(PaletteItemKey::File { relative_path }) => {
                Some(CommandPaletteCommit::FilePath(relative_path))
            }
            None => None,
        }
    }

    pub fn tick(
        &mut self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
        now: Instant,
    ) -> bool {
        let mut changed = false;

        if let Some(file_index) = self.file_index.as_mut() {
            loop {
                match file_index.rx.try_recv() {
                    Ok(msg) => {
                        changed = true;
                        file_index.last_progress_at = now;
                        match msg {
                            FileIndexMessage::Chunk(mut chunk) => {
                                file_index.state = PaletteProviderStateClass::Streaming;
                                file_index.file_paths.append(&mut chunk);
                            }
                            FileIndexMessage::Complete => {
                                file_index.state = PaletteProviderStateClass::Complete;
                                file_index.complete = true;
                                break;
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        file_index.state = PaletteProviderStateClass::Unavailable;
                        file_index.complete = true;
                        changed = true;
                        break;
                    }
                }
            }

            if let Some(watcher) = file_index.watcher.as_ref() {
                let mut pending: Vec<WatcherEvent> = Vec::new();
                while let Some(event) = watcher.try_recv() {
                    pending.push(event);
                }
                if !pending.is_empty() {
                    changed = true;
                    file_index
                        .last_watcher_events
                        .extend(pending.iter().cloned());
                    for event in pending {
                        apply_watcher_event(file_index, event);
                    }
                }
            }

            if !file_index.complete && file_index.state == PaletteProviderStateClass::Warming {
                if now.duration_since(file_index.last_progress_at) > Duration::from_secs(2) {
                    file_index.state = PaletteProviderStateClass::Partial;
                    changed = true;
                }
            }

            if file_index.needs_rescan && file_index.complete {
                restart_file_index_scan(file_index, now);
                changed = true;
            }
        }

        if !self.open {
            return changed;
        }

        if let Some(deadline) = self.semantic_deadline {
            if now >= deadline {
                changed = true;
                match self.semantic_state {
                    PaletteProviderStateClass::Warming => {
                        self.semantic_state = PaletteProviderStateClass::Streaming;
                        self.semantic_deadline = Some(now + Duration::from_millis(250));
                    }
                    PaletteProviderStateClass::Streaming => {
                        self.semantic_state = PaletteProviderStateClass::Complete;
                        self.semantic_deadline = None;
                    }
                    _ => {
                        self.semantic_deadline = None;
                    }
                }
            }
        }

        if changed {
            self.updated_at = now;
            self.recompute_groups(registry, shortcuts_by_command_id);
        }

        changed
    }

    pub fn next_wake_deadline(&self, now: Instant) -> Option<Instant> {
        if !self.open {
            return None;
        }
        let mut deadline = self.semantic_deadline;

        if let Some(file_index) = self.file_index.as_ref() {
            let poll = if !file_index.complete {
                now + Duration::from_millis(50)
            } else if file_index.watcher.is_some() {
                now + Duration::from_millis(200)
            } else {
                now
            };
            if poll != now {
                deadline = match deadline {
                    Some(existing) => Some(existing.min(poll)),
                    None => Some(poll),
                };
            }
        }

        deadline
    }

    fn on_query_changed(
        &mut self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) {
        let now = Instant::now();
        self.updated_at = now;
        if is_query_effectively_empty(&self.query) {
            self.semantic_state = PaletteProviderStateClass::NotRequested;
            self.semantic_deadline = None;
        } else {
            self.semantic_state = PaletteProviderStateClass::Warming;
            self.semantic_deadline = Some(now + Duration::from_millis(250));
        }
        self.recompute_groups(registry, shortcuts_by_command_id);
    }

    fn recompute_groups(
        &mut self,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    ) {
        let normalized = normalize_query(&self.query);
        let mut next_groups: Vec<PaletteResultGroup> = Vec::new();
        let mut next_flat: Vec<PaletteItemKey> = Vec::new();

        let mut recent_items = Vec::new();
        for command_id in self.recent_command_ids.iter() {
            let Some(entry) = registry.get(command_id) else {
                continue;
            };
            if !normalized.is_empty()
                && !contains_case_insensitive(&entry.title, &normalized)
                && !contains_case_insensitive(entry.command_id(), &normalized)
            {
                continue;
            }
            let item = PaletteResultItem {
                key: PaletteItemKey::Command {
                    command_id: command_id.clone(),
                },
                provider: PaletteProviderClass::RecentHistory,
                provider_state: PaletteProviderStateClass::Complete,
                ranking_sources: vec![PaletteRankingSourceClass::RecentHistory],
            };
            recent_items.push(item);
        }
        if !recent_items.is_empty() {
            for item in &recent_items {
                next_flat.push(item.key.clone());
            }
            next_groups.push(PaletteResultGroup {
                label: "Recent".to_string(),
                provider: PaletteProviderClass::RecentHistory,
                provider_state: PaletteProviderStateClass::Complete,
                items: recent_items,
            });
        }

        let mut command_matches: Vec<(i32, PaletteResultItem)> = Vec::new();
        for command_id in self.visible_command_ids.iter() {
            let Some(entry) = registry.get(command_id) else {
                continue;
            };
            let shortcuts = shortcuts_by_command_id
                .get(command_id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            let (score, sources) = match_command(entry, &normalized, shortcuts);
            if score.is_none() {
                continue;
            }
            let item = PaletteResultItem {
                key: PaletteItemKey::Command {
                    command_id: command_id.clone(),
                },
                provider: PaletteProviderClass::LexicalCommandIndex,
                provider_state: PaletteProviderStateClass::Complete,
                ranking_sources: sources,
            };
            command_matches.push((score.unwrap_or(1000), item));
        }
        command_matches.sort_by_key(|(score, _)| *score);

        let mut lexical_items: Vec<PaletteResultItem> = Vec::new();
        for (_, item) in command_matches.into_iter().take(12) {
            if next_flat.iter().any(|key| key == &item.key) {
                continue;
            }
            lexical_items.push(item);
        }
        if !lexical_items.is_empty() {
            for item in &lexical_items {
                next_flat.push(item.key.clone());
            }
            next_groups.push(PaletteResultGroup {
                label: "Commands".to_string(),
                provider: PaletteProviderClass::LexicalCommandIndex,
                provider_state: PaletteProviderStateClass::Complete,
                items: lexical_items,
            });
        }

        if !normalized.is_empty() && self.semantic_state != PaletteProviderStateClass::NotRequested
        {
            let mut semantic_items: Vec<PaletteResultItem> = Vec::new();
            if self.semantic_state == PaletteProviderStateClass::Streaming
                || self.semantic_state == PaletteProviderStateClass::Complete
            {
                for command_id in self.visible_command_ids.iter() {
                    if next_flat
                        .iter()
                        .any(|key| matches!(key, PaletteItemKey::Command { command_id: id } if id == command_id))
                    {
                        continue;
                    }
                    let Some(entry) = registry.get(command_id) else {
                        continue;
                    };
                    if semantic_match(entry, &normalized) {
                        semantic_items.push(PaletteResultItem {
                            key: PaletteItemKey::Command {
                                command_id: command_id.clone(),
                            },
                            provider: PaletteProviderClass::SemanticCommandIndex,
                            provider_state: self.semantic_state,
                            ranking_sources: vec![PaletteRankingSourceClass::SemanticSupplement],
                        });
                    }
                    if semantic_items.len() >= 6 {
                        break;
                    }
                }
            }

            for item in &semantic_items {
                next_flat.push(item.key.clone());
            }
            next_groups.push(PaletteResultGroup {
                label: "Semantic".to_string(),
                provider: PaletteProviderClass::SemanticCommandIndex,
                provider_state: self.semantic_state,
                items: semantic_items,
            });
        }

        let file_state = self
            .file_index
            .as_ref()
            .map(|idx| idx.state)
            .unwrap_or(PaletteProviderStateClass::NotRequested);
        let file_paths = self
            .file_index
            .as_ref()
            .map(|idx| idx.file_paths.as_slice())
            .unwrap_or(&[]);
        let file_items = materialize_file_results(file_paths, &normalized, file_state);
        if file_state != PaletteProviderStateClass::NotRequested {
            for item in &file_items {
                next_flat.push(item.key.clone());
            }
            let file_label = self
                .file_index
                .as_ref()
                .and_then(|idx| idx.watcher.as_ref().map(|_| idx.watcher_health))
                .map(|health| format!("Files (watcher: {})", health.as_str()))
                .unwrap_or_else(|| "Files".to_string());
            next_groups.push(PaletteResultGroup {
                label: file_label,
                provider: PaletteProviderClass::FileIndex,
                provider_state: file_state,
                items: file_items,
            });
        }

        self.groups = next_groups;
        self.flat_item_keys = next_flat;

        if self.flat_item_keys.is_empty() {
            self.selection = 0;
            self.selected_key = None;
            return;
        }

        if let Some(selected_key) = self.selected_key.clone() {
            if let Some(idx) = self
                .flat_item_keys
                .iter()
                .position(|key| key == &selected_key)
            {
                self.selection = idx;
                return;
            }
        }

        self.selection = self
            .selection
            .min(self.flat_item_keys.len().saturating_sub(1));
        self.selected_key = self.flat_item_keys.get(self.selection).cloned();
    }
}

fn match_command(
    entry: &CommandRegistryEntryRecord,
    normalized_query: &str,
    shortcuts: &[String],
) -> (Option<i32>, Vec<PaletteRankingSourceClass>) {
    if normalized_query.is_empty() {
        return (Some(100), vec![PaletteRankingSourceClass::TitleSubstring]);
    }

    let mut score = None;
    let mut sources = Vec::new();

    if entry.command_id().eq_ignore_ascii_case(normalized_query) {
        score = Some(0);
        sources.push(PaletteRankingSourceClass::ExactCommandId);
    } else if contains_case_insensitive(entry.command_id(), normalized_query) {
        score = Some(5);
        sources.push(PaletteRankingSourceClass::ExactCommandId);
    }

    if contains_case_insensitive(&entry.title, normalized_query) {
        score = Some(score.unwrap_or(20).min(20));
        sources.push(PaletteRankingSourceClass::TitleSubstring);
    } else if contains_case_insensitive(&entry.summary, normalized_query) {
        score = Some(score.unwrap_or(40).min(40));
        sources.push(PaletteRankingSourceClass::SummarySubstring);
    }

    if !shortcuts.is_empty() {
        let query_no_space = normalized_query.replace(' ', "");
        if shortcuts
            .iter()
            .any(|seq| seq.eq_ignore_ascii_case(&query_no_space))
        {
            score = Some(score.unwrap_or(10).min(10));
            sources.push(PaletteRankingSourceClass::KeySequenceMatch);
        } else if shortcuts
            .iter()
            .any(|seq| contains_case_insensitive(seq, &query_no_space))
        {
            score = Some(score.unwrap_or(30).min(30));
            sources.push(PaletteRankingSourceClass::KeySequenceMatch);
        }
    }

    (score, sources)
}

fn semantic_match(entry: &CommandRegistryEntryRecord, normalized_query: &str) -> bool {
    if normalized_query.len() < 3 {
        return false;
    }
    let parts: Vec<&str> = normalized_query.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }
    let title = entry.title.to_ascii_lowercase();
    let summary = entry.summary.to_ascii_lowercase();
    parts
        .iter()
        .any(|part| title.contains(part) || summary.contains(part))
}

fn materialize_file_results(
    file_paths: &[String],
    normalized_query: &str,
    provider_state: PaletteProviderStateClass,
) -> Vec<PaletteResultItem> {
    if file_paths.is_empty() && provider_state != PaletteProviderStateClass::Streaming {
        return Vec::new();
    }
    let mut out = Vec::new();
    if normalized_query.is_empty() {
        for path in file_paths.iter().take(6) {
            out.push(PaletteResultItem {
                key: PaletteItemKey::File {
                    relative_path: path.clone(),
                },
                provider: PaletteProviderClass::FileIndex,
                provider_state,
                ranking_sources: vec![],
            });
        }
        return out;
    }

    for path in file_paths.iter() {
        if contains_case_insensitive(path, normalized_query) {
            out.push(PaletteResultItem {
                key: PaletteItemKey::File {
                    relative_path: path.clone(),
                },
                provider: PaletteProviderClass::FileIndex,
                provider_state,
                ranking_sources: vec![],
            });
        }
        if out.len() >= 8 {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_commands::registry::seeded_registry;

    #[test]
    fn command_id_search_matches() {
        let registry = seeded_registry();
        let mut palette = CommandPaletteState::new(registry);
        palette.open(registry, PathBuf::from("."));

        let mut shortcuts: HashMap<String, Vec<String>> = HashMap::new();
        shortcuts.insert(
            "cmd:workspace.open_folder".to_string(),
            vec!["Cmd+O".to_string()],
        );

        palette.handle_text_input('c', registry, &shortcuts);
        palette.handle_text_input('m', registry, &shortcuts);
        palette.handle_text_input('d', registry, &shortcuts);
        palette.handle_text_input(':', registry, &shortcuts);
        palette.handle_text_input('w', registry, &shortcuts);

        let selected = palette
            .selected_entry(registry)
            .expect("must select an entry");
        assert!(
            selected.command_id().starts_with("cmd:"),
            "selected command should have stable id"
        );
    }

    #[test]
    fn semantic_stream_preserves_selection_key() {
        let registry = seeded_registry();
        let mut palette = CommandPaletteState::new(registry);
        palette.open(registry, PathBuf::from("."));

        let shortcuts: HashMap<String, Vec<String>> = HashMap::new();
        for ch in "open".chars() {
            palette.handle_text_input(ch, registry, &shortcuts);
        }
        palette.handle_arrow_down();
        let selected_key = palette.selected_key().cloned();

        let now = Instant::now() + Duration::from_millis(300);
        let _ = palette.tick(registry, &shortcuts, now);
        assert_eq!(palette.selected_key(), selected_key.as_ref());
    }
}
