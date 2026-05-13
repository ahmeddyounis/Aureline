//! Hot-set planning for first-useful search and navigation.
//!
//! The hot-set planner owns the alpha contract that decides which files and
//! symbols should be indexed before the rest of the workspace. It consumes
//! already-known user and workspace signals, emits a bounded plan, and keeps
//! every selected target explainable so quick open can be useful without
//! claiming full-workspace coverage.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Default maximum number of hot navigation targets materialized per plan.
pub const DEFAULT_MAX_HOT_SET_TARGETS: usize = 128;

/// Search readiness states shared with the search-result truth schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchReadinessState {
    /// No indexing has started for the requested scope.
    NotIndexed,
    /// A bounded hot set is ready while broader indexing continues.
    HotSetReady,
    /// Some rows are indexed, but the declared scope is incomplete.
    PartialIndex,
    /// Warm cached or warmed rows are available, but not full readiness.
    WarmIndex,
    /// The requested scope is fully indexed.
    FullyIndexed,
    /// A stale index is being served with freshness disclosure.
    StaleIndex,
    /// Index data exists, but a rebuild is in progress.
    Reindexing,
    /// The indexer cannot currently serve this scope.
    IndexUnavailable,
}

impl SearchReadinessState {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotIndexed => "not_indexed",
            Self::HotSetReady => "hot_set_ready",
            Self::PartialIndex => "partial_index",
            Self::WarmIndex => "warm_index",
            Self::FullyIndexed => "fully_indexed",
            Self::StaleIndex => "stale_index",
            Self::Reindexing => "reindexing",
            Self::IndexUnavailable => "index_unavailable",
        }
    }

    /// Short user-visible readiness label for shell and support snapshots.
    pub const fn banner_label(self) -> &'static str {
        match self {
            Self::NotIndexed => "Not indexed",
            Self::HotSetReady => "Hot set ready",
            Self::PartialIndex => "Partial index",
            Self::WarmIndex => "Warm index",
            Self::FullyIndexed => "Fully indexed",
            Self::StaleIndex => "Stale index",
            Self::Reindexing => "Reindexing",
            Self::IndexUnavailable => "Index unavailable",
        }
    }

    /// True when this state must carry partial-truth disclosure.
    pub const fn requires_partial_truth(self) -> bool {
        !matches!(self, Self::FullyIndexed)
    }
}

/// Hot-set signal classes accepted by the scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotSetInputClass {
    /// A file currently open in an editor.
    OpenFile,
    /// A file recently edited by the user.
    RecentEdit,
    /// A file changed in the current worktree or review context.
    ChangedFile,
    /// A build, test, debug, or run target active in the current workflow.
    ActiveTarget,
    /// A test file near an active source file or target.
    NearbyTest,
    /// A file reached through imports from an active source file.
    ImportNeighborhood,
    /// A file reached through dependency or package relationships.
    DependencyNeighborhood,
    /// A file near active diagnostics.
    DiagnosticNeighborhood,
    /// A user-pinned path or symbol.
    UserPinned,
    /// A path or symbol restored from the previous session.
    RestoredSession,
}

impl HotSetInputClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFile => "open_file",
            Self::RecentEdit => "recent_edit",
            Self::ChangedFile => "changed_file",
            Self::ActiveTarget => "active_target",
            Self::NearbyTest => "nearby_test",
            Self::ImportNeighborhood => "import_neighborhood",
            Self::DependencyNeighborhood => "dependency_neighborhood",
            Self::DiagnosticNeighborhood => "diagnostic_neighborhood",
            Self::UserPinned => "user_pinned",
            Self::RestoredSession => "restored_session",
        }
    }

    const fn rank(self) -> u8 {
        match self {
            Self::OpenFile => 0,
            Self::UserPinned => 1,
            Self::RecentEdit => 2,
            Self::ChangedFile => 3,
            Self::ActiveTarget => 4,
            Self::DiagnosticNeighborhood => 5,
            Self::NearbyTest => 6,
            Self::ImportNeighborhood => 7,
            Self::DependencyNeighborhood => 8,
            Self::RestoredSession => 9,
        }
    }
}

/// Navigation target shape accepted by the hot-set planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotSetTargetKind {
    /// A file target addressable by workspace-relative path.
    File,
    /// A symbol target addressable by a symbol ref and containing file.
    Symbol,
}

impl HotSetTargetKind {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
        }
    }
}

/// One target proposed for hot-set indexing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HotSetTarget {
    /// Target kind: file or symbol.
    pub target_kind: HotSetTargetKind,
    /// Workspace-relative file path that contains or is the target.
    pub relative_path: String,
    /// Stable symbol ref for symbol targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_ref: Option<String>,
    /// Display label used only in export/debug views.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
}

impl HotSetTarget {
    /// Returns the normalized workspace-relative path for this target.
    pub fn normalized_relative_path(&self) -> String {
        normalize_relative_path(&self.relative_path)
    }

    fn key(&self) -> HotSetTargetKey {
        HotSetTargetKey {
            target_kind: self.target_kind,
            relative_path: self.normalized_relative_path(),
            symbol_ref: self
                .symbol_ref
                .as_deref()
                .map(str::trim)
                .map(str::to_string),
        }
    }
}

/// One input signal that explains why a target belongs in the hot set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetCandidate {
    /// File or symbol target being prioritized.
    pub target: HotSetTarget,
    /// Signal class that made the target hot.
    pub input_class: HotSetInputClass,
    /// Opaque ref to the source event, row, target, or restored state.
    pub input_ref: String,
    /// Human-reviewable reason for the priority decision.
    pub priority_reason: String,
    /// Project graph nodes this input is known to cover.
    #[serde(default)]
    pub project_node_refs: Vec<String>,
    /// Target project nodes associated with the input.
    #[serde(default)]
    pub target_project_node_refs: Vec<String>,
}

/// Inputs required to produce one bounded hot-set plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetPlanInputs {
    /// Workspace identity for the plan.
    pub workspace_id: String,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
    /// Planner version recorded in exported snapshots.
    pub planner_version: String,
    /// Whether the indexer can currently serve any rows.
    #[serde(default = "default_index_available")]
    pub index_available: bool,
    /// Whether full indexing has already completed for the declared scope.
    #[serde(default)]
    pub full_index_complete: bool,
    /// Workspace-relative files currently known from catalog or scan state.
    #[serde(default)]
    pub discovered_files: Vec<String>,
    /// Candidate hot-set signals from the active user and workspace context.
    #[serde(default)]
    pub candidates: Vec<HotSetCandidate>,
    /// Maximum target count for this bounded plan.
    #[serde(default = "default_max_hot_set_targets")]
    pub max_hot_targets: usize,
}

impl HotSetPlanInputs {
    /// Returns the effective target cap, with zero treated as the default.
    pub const fn effective_max_hot_targets(&self) -> usize {
        if self.max_hot_targets == 0 {
            DEFAULT_MAX_HOT_SET_TARGETS
        } else {
            self.max_hot_targets
        }
    }
}

/// One selected target in a hot-set plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetPlanEntry {
    /// Stable order inside the bounded hot-set plan.
    pub rank: usize,
    /// Target selected for hot-set indexing.
    pub target: HotSetTarget,
    /// Input classes that justify this target.
    pub input_classes: Vec<HotSetInputClass>,
    /// Human-reviewable reasons collected from contributing inputs.
    pub priority_reasons: Vec<String>,
    /// Project graph nodes covered by this target.
    pub project_node_refs: Vec<String>,
    /// Target project nodes associated with this target.
    pub target_project_node_refs: Vec<String>,
}

/// Partial-truth causes emitted by the hot-set scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotSetPartialTruthCause {
    /// Only the hot set is ready; broader index coverage is not complete.
    HotSetOnly,
    /// Background indexing is still running or queued.
    IndexingInProgress,
    /// A stale index was served while fresh materialization catches up.
    StaleIndexServed,
}

impl HotSetPartialTruthCause {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HotSetOnly => "hot_set_only",
            Self::IndexingInProgress => "indexing_in_progress",
            Self::StaleIndexServed => "stale_index_served",
        }
    }
}

/// Reason the scheduler fell back from hot-set targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotSetFallbackReason {
    /// No hot inputs were available, so known catalog paths are used.
    NoHotInputs,
    /// No known path can be served yet.
    NoKnownPaths,
    /// The indexer is unavailable.
    IndexUnavailable,
}

impl HotSetFallbackReason {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoHotInputs => "no_hot_inputs",
            Self::NoKnownPaths => "no_known_paths",
            Self::IndexUnavailable => "index_unavailable",
        }
    }
}

/// Fallback path when no hot target can be materialized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetFallback {
    /// Machine-readable fallback reason.
    pub reason: HotSetFallbackReason,
    /// Short reviewable explanation of the fallback.
    pub summary: String,
    /// Safe next action for a user or support replay.
    pub next_action: String,
    /// Paths available through the fallback, if any.
    pub candidate_paths: Vec<String>,
}

/// Foreground responsiveness guarantees projected by the scheduler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetResponsiveness {
    /// True when edit input would wait on index warm-up.
    pub edit_blocked_by_index_warmup: bool,
    /// True when quick open would wait for the full index.
    pub quick_open_blocked_by_full_index: bool,
    /// True when full indexing is explicitly deferred behind hot-set work.
    pub full_index_deferred: bool,
}

/// Exportable explanation for why a target is hot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetExplanation {
    /// Target explained by this record.
    pub target: HotSetTarget,
    /// Readiness state at the time the explanation was emitted.
    pub readiness_state: SearchReadinessState,
    /// Input classes that contributed to this target.
    pub input_classes: Vec<HotSetInputClass>,
    /// Human-reviewable reasons collected from input signals.
    pub priority_reasons: Vec<String>,
}

/// Bounded hot-set plan consumed by the first-useful navigation scheduler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotSetPlan {
    /// Workspace identity for the plan.
    pub workspace_id: String,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
    /// Planner version recorded in exported snapshots.
    pub planner_version: String,
    /// Readiness state for this plan.
    pub readiness_state: SearchReadinessState,
    /// Whether full indexing has already completed for the declared scope.
    pub full_index_complete: bool,
    /// Selected hot-set targets.
    pub entries: Vec<HotSetPlanEntry>,
    /// Known files not selected into the hot set.
    pub deferred_cold_paths: Vec<String>,
    /// Fallback used when no hot target can be materialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<HotSetFallback>,
    /// Partial-truth causes carried by non-full readiness states.
    pub partial_truth_causes: Vec<HotSetPartialTruthCause>,
    /// Foreground responsiveness projection for protected paths.
    pub responsiveness: HotSetResponsiveness,
}

impl HotSetPlan {
    /// Returns the stable readiness token for this plan.
    pub const fn readiness_token(&self) -> &'static str {
        self.readiness_state.as_str()
    }

    /// Returns selected entries in scheduler rank order.
    pub fn entries(&self) -> &[HotSetPlanEntry] {
        &self.entries
    }

    /// Returns unique file paths that make quick-open useful immediately.
    pub fn first_useful_file_paths(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut paths = Vec::new();
        for entry in &self.entries {
            let path = entry.target.normalized_relative_path();
            if seen.insert(path.clone()) {
                paths.push(path);
            }
        }
        paths
    }

    /// Returns stable partial-truth cause tokens.
    pub fn partial_truth_cause_tokens(&self) -> Vec<&'static str> {
        self.partial_truth_causes
            .iter()
            .map(|cause| cause.as_str())
            .collect()
    }

    /// Returns an explanation if the path was selected into the hot set.
    pub fn why_path_is_hot(&self, relative_path: &str) -> Option<HotSetExplanation> {
        let normalized = normalize_relative_path(relative_path);
        let matching: Vec<&HotSetPlanEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.target.normalized_relative_path() == normalized)
            .collect();
        explanation_from_entries(matching, self.readiness_state)
    }

    /// Returns an explanation if the symbol was selected into the hot set.
    pub fn why_symbol_is_hot(&self, symbol_ref: &str) -> Option<HotSetExplanation> {
        let normalized = symbol_ref.trim();
        let matching: Vec<&HotSetPlanEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.target.symbol_ref.as_deref() == Some(normalized))
            .collect();
        explanation_from_entries(matching, self.readiness_state)
    }

    /// Returns one explanation per selected entry.
    pub fn explanations(&self) -> Vec<HotSetExplanation> {
        self.entries
            .iter()
            .map(|entry| HotSetExplanation {
                target: entry.target.clone(),
                readiness_state: self.readiness_state,
                input_classes: entry.input_classes.clone(),
                priority_reasons: entry.priority_reasons.clone(),
            })
            .collect()
    }
}

/// Stateless alpha planner for hot-set navigation targets.
#[derive(Debug, Default, Clone, Copy)]
pub struct HotSetPlanner;

impl HotSetPlanner {
    /// Builds a deterministic bounded hot-set plan from the supplied inputs.
    pub fn plan(inputs: HotSetPlanInputs) -> HotSetPlan {
        let max_hot_targets = inputs.effective_max_hot_targets();
        let discovered_files = sorted_unique_paths(inputs.discovered_files);
        let mut grouped: BTreeMap<HotSetTargetKey, HotSetEntryAccumulator> = BTreeMap::new();

        if inputs.index_available {
            for candidate in inputs.candidates {
                let normalized_path = candidate.target.normalized_relative_path();
                if normalized_path.is_empty() {
                    continue;
                }
                let mut target = candidate.target.clone();
                target.relative_path = normalized_path;
                let key = target.key();
                let entry = grouped
                    .entry(key)
                    .or_insert_with(|| HotSetEntryAccumulator::new(target));
                entry.add_candidate(candidate);
            }
        }

        let mut accumulated: Vec<HotSetEntryAccumulator> = grouped.into_values().collect();
        accumulated.sort_by(|a, b| {
            a.best_rank
                .cmp(&b.best_rank)
                .then_with(|| {
                    a.target
                        .normalized_relative_path()
                        .cmp(&b.target.normalized_relative_path())
                })
                .then_with(|| {
                    a.target
                        .target_kind
                        .as_str()
                        .cmp(b.target.target_kind.as_str())
                })
                .then_with(|| a.target.symbol_ref.cmp(&b.target.symbol_ref))
        });
        accumulated.truncate(max_hot_targets);

        let entries: Vec<HotSetPlanEntry> = accumulated
            .into_iter()
            .enumerate()
            .map(|(rank, entry)| entry.into_plan_entry(rank))
            .collect();

        let selected_paths: BTreeSet<String> = entries
            .iter()
            .map(|entry| entry.target.normalized_relative_path())
            .collect();
        let deferred_cold_paths: Vec<String> = discovered_files
            .iter()
            .filter(|path| !selected_paths.contains(*path))
            .cloned()
            .collect();

        let (readiness_state, fallback) = readiness_and_fallback(
            inputs.index_available,
            inputs.full_index_complete,
            entries.is_empty(),
            &discovered_files,
            max_hot_targets,
        );
        let partial_truth_causes = partial_causes_for(readiness_state);

        HotSetPlan {
            workspace_id: inputs.workspace_id,
            observed_at: inputs.observed_at,
            planner_version: inputs.planner_version,
            readiness_state,
            full_index_complete: inputs.full_index_complete,
            entries,
            deferred_cold_paths,
            fallback,
            partial_truth_causes,
            responsiveness: HotSetResponsiveness {
                edit_blocked_by_index_warmup: false,
                quick_open_blocked_by_full_index: false,
                full_index_deferred: !inputs.full_index_complete,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct HotSetTargetKey {
    target_kind: HotSetTargetKind,
    relative_path: String,
    symbol_ref: Option<String>,
}

#[derive(Debug)]
struct HotSetEntryAccumulator {
    target: HotSetTarget,
    input_classes: BTreeSet<HotSetInputClassForSort>,
    priority_reasons: BTreeSet<String>,
    project_node_refs: BTreeSet<String>,
    target_project_node_refs: BTreeSet<String>,
    best_rank: u8,
}

impl HotSetEntryAccumulator {
    fn new(target: HotSetTarget) -> Self {
        Self {
            target,
            input_classes: BTreeSet::new(),
            priority_reasons: BTreeSet::new(),
            project_node_refs: BTreeSet::new(),
            target_project_node_refs: BTreeSet::new(),
            best_rank: u8::MAX,
        }
    }

    fn add_candidate(&mut self, candidate: HotSetCandidate) {
        self.best_rank = self.best_rank.min(candidate.input_class.rank());
        self.input_classes
            .insert(HotSetInputClassForSort(candidate.input_class));
        let reason = candidate.priority_reason.trim();
        if !reason.is_empty() {
            self.priority_reasons.insert(reason.to_string());
        }
        self.project_node_refs.extend(
            candidate
                .project_node_refs
                .into_iter()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        );
        self.target_project_node_refs.extend(
            candidate
                .target_project_node_refs
                .into_iter()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        );
    }

    fn into_plan_entry(self, rank: usize) -> HotSetPlanEntry {
        HotSetPlanEntry {
            rank,
            target: self.target,
            input_classes: self
                .input_classes
                .into_iter()
                .map(|class| class.0)
                .collect(),
            priority_reasons: self.priority_reasons.into_iter().collect(),
            project_node_refs: self.project_node_refs.into_iter().collect(),
            target_project_node_refs: self.target_project_node_refs.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HotSetInputClassForSort(HotSetInputClass);

impl PartialOrd for HotSetInputClassForSort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HotSetInputClassForSort {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .rank()
            .cmp(&other.0.rank())
            .then_with(|| self.0.as_str().cmp(other.0.as_str()))
    }
}

fn readiness_and_fallback(
    index_available: bool,
    full_index_complete: bool,
    hot_entries_empty: bool,
    discovered_files: &[String],
    max_paths: usize,
) -> (SearchReadinessState, Option<HotSetFallback>) {
    if !index_available {
        return (
            SearchReadinessState::IndexUnavailable,
            Some(HotSetFallback {
                reason: HotSetFallbackReason::IndexUnavailable,
                summary: "The indexer is unavailable, so the hot set cannot be materialized."
                    .to_string(),
                next_action: "repair_or_rebuild_index".to_string(),
                candidate_paths: Vec::new(),
            }),
        );
    }
    if full_index_complete {
        return (SearchReadinessState::FullyIndexed, None);
    }
    if !hot_entries_empty {
        return (SearchReadinessState::HotSetReady, None);
    }
    if discovered_files.is_empty() {
        return (
            SearchReadinessState::NotIndexed,
            Some(HotSetFallback {
                reason: HotSetFallbackReason::NoKnownPaths,
                summary: "No hot-set input or known path is available yet.".to_string(),
                next_action: "wait_for_warmup".to_string(),
                candidate_paths: Vec::new(),
            }),
        );
    }

    (
        SearchReadinessState::PartialIndex,
        Some(HotSetFallback {
            reason: HotSetFallbackReason::NoHotInputs,
            summary: "No hot-set input matched, so known workspace paths are used until hot inputs arrive."
                .to_string(),
            next_action: "keep_typing_or_wait_for_warmup".to_string(),
            candidate_paths: discovered_files.iter().take(max_paths).cloned().collect(),
        }),
    )
}

fn partial_causes_for(readiness: SearchReadinessState) -> Vec<HotSetPartialTruthCause> {
    match readiness {
        SearchReadinessState::FullyIndexed => Vec::new(),
        SearchReadinessState::HotSetReady => vec![
            HotSetPartialTruthCause::HotSetOnly,
            HotSetPartialTruthCause::IndexingInProgress,
        ],
        SearchReadinessState::StaleIndex => vec![HotSetPartialTruthCause::StaleIndexServed],
        _ => vec![HotSetPartialTruthCause::IndexingInProgress],
    }
}

fn explanation_from_entries(
    entries: Vec<&HotSetPlanEntry>,
    readiness_state: SearchReadinessState,
) -> Option<HotSetExplanation> {
    let first = entries.first()?;
    let target = first.target.clone();
    let mut input_classes = BTreeSet::new();
    let mut priority_reasons = BTreeSet::new();
    for entry in &entries {
        input_classes.extend(
            entry
                .input_classes
                .iter()
                .copied()
                .map(HotSetInputClassForSort),
        );
        priority_reasons.extend(entry.priority_reasons.iter().cloned());
    }
    Some(HotSetExplanation {
        target,
        readiness_state,
        input_classes: input_classes.into_iter().map(|class| class.0).collect(),
        priority_reasons: priority_reasons.into_iter().collect(),
    })
}

fn sorted_unique_paths(paths: Vec<String>) -> Vec<String> {
    let mut paths: Vec<String> = paths
        .into_iter()
        .map(|path| normalize_relative_path(&path))
        .filter(|path| !path.is_empty())
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

fn normalize_relative_path(path: &str) -> String {
    let mut normalized = path.trim().replace('\\', "/");
    while let Some(stripped) = normalized.strip_prefix("./") {
        normalized = stripped.to_string();
    }
    normalized
}

const fn default_index_available() -> bool {
    true
}

const fn default_max_hot_set_targets() -> usize {
    DEFAULT_MAX_HOT_SET_TARGETS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file_candidate(path: &str, input_class: HotSetInputClass) -> HotSetCandidate {
        HotSetCandidate {
            target: HotSetTarget {
                target_kind: HotSetTargetKind::File,
                relative_path: path.to_string(),
                symbol_ref: None,
                display_label: None,
            },
            input_class,
            input_ref: format!("input:{path}"),
            priority_reason: format!("reason for {path}"),
            project_node_refs: Vec::new(),
            target_project_node_refs: Vec::new(),
        }
    }

    #[test]
    fn open_files_rank_before_dependency_neighborhood() {
        let plan = HotSetPlanner::plan(HotSetPlanInputs {
            workspace_id: "ws-test".to_string(),
            observed_at: "mono:1".to_string(),
            planner_version: "test".to_string(),
            index_available: true,
            full_index_complete: false,
            discovered_files: Vec::new(),
            candidates: vec![
                file_candidate(
                    "src/dependency.rs",
                    HotSetInputClass::DependencyNeighborhood,
                ),
                file_candidate("src/main.rs", HotSetInputClass::OpenFile),
            ],
            max_hot_targets: DEFAULT_MAX_HOT_SET_TARGETS,
        });
        assert_eq!(plan.readiness_state, SearchReadinessState::HotSetReady);
        assert_eq!(plan.entries[0].target.relative_path, "src/main.rs");
        assert_eq!(plan.entries[1].target.relative_path, "src/dependency.rs");
    }

    #[test]
    fn empty_inputs_fall_back_to_known_paths() {
        let plan = HotSetPlanner::plan(HotSetPlanInputs {
            workspace_id: "ws-test".to_string(),
            observed_at: "mono:1".to_string(),
            planner_version: "test".to_string(),
            index_available: true,
            full_index_complete: false,
            discovered_files: vec!["src/main.rs".to_string()],
            candidates: Vec::new(),
            max_hot_targets: DEFAULT_MAX_HOT_SET_TARGETS,
        });
        let fallback = plan.fallback.expect("fallback must be present");
        assert_eq!(plan.readiness_state, SearchReadinessState::PartialIndex);
        assert_eq!(fallback.reason, HotSetFallbackReason::NoHotInputs);
        assert_eq!(fallback.candidate_paths, vec!["src/main.rs"]);
    }
}
