//! First-useful indexing scheduler for quick open and navigation.
//!
//! The scheduler is the first runtime consumer of the hot-set planner. It
//! converts a hot-set plan into a lexical index snapshot that quick open can
//! query immediately, while the full index remains background work with
//! explicit readiness and partial-truth disclosure.

use serde::{Deserialize, Serialize};

use aureline_reactive_state::ReadinessLabel;
use aureline_workspace::{WorkspaceLifecycleState, WorkspaceReadinessInputs};

use crate::hot_set::{
    HotSetCandidate, HotSetExplanation, HotSetPartialTruthCause, HotSetPlan, HotSetPlanInputs,
    HotSetPlanner, SearchReadinessState, DEFAULT_MAX_HOT_SET_TARGETS,
};
use crate::lexical::index::{LexicalIndexState, PartialTruthCause};
use crate::lexical::query::LexicalQuery;
use crate::lexical::scope::ScopeClass;
use crate::lexical::shell::{LexicalShell, LexicalShellSnapshot};
use crate::lexical::ReadinessClass;
use crate::scope::{WorkspaceSearchScope, WorkspaceSearchScopeMetadata};

/// Inputs for one first-useful indexing scheduler pass.
#[derive(Debug, Clone)]
pub struct IndexSchedulerInputs {
    /// Workspace lifecycle and watcher readiness inputs.
    pub readiness_inputs: WorkspaceReadinessInputs,
    /// Latest reactive readiness label for the workspace.
    pub readiness_label: ReadinessLabel,
    /// Workspace-relative files currently known from catalog or scan state.
    pub discovered_files: Vec<String>,
    /// Candidate hot-set signals from the active user and workspace context.
    pub hot_set_candidates: Vec<HotSetCandidate>,
    /// Whether the full lexical/search index is complete for this scope.
    pub full_index_complete: bool,
    /// Optional active workset or slice scope.
    pub scope: Option<WorkspaceSearchScope>,
    /// Version token recorded on the hot-set plan.
    pub planner_version: String,
    /// Maximum hot-set target count for this pass.
    pub max_hot_targets: usize,
}

impl IndexSchedulerInputs {
    /// Builds fixture-friendly inputs from a lifecycle state.
    pub fn for_fixture(
        workspace_id: impl Into<String>,
        observed_at: impl Into<String>,
        lifecycle_state: WorkspaceLifecycleState,
        readiness_label: ReadinessLabel,
        discovered_files: Vec<String>,
        hot_set_candidates: Vec<HotSetCandidate>,
        full_index_complete: bool,
    ) -> Self {
        let readiness_inputs = WorkspaceReadinessInputs {
            workspace_id: workspace_id.into(),
            lifecycle_state_token: lifecycle_state.as_str(),
            watcher_health_token: match lifecycle_state {
                WorkspaceLifecycleState::Ready => Some("healthy"),
                WorkspaceLifecycleState::PartiallyReady => Some("warming"),
                WorkspaceLifecycleState::Degraded => Some("degraded"),
                _ => None,
            },
            hot_index_ready: full_index_complete,
            command_graph_ready: matches!(lifecycle_state, WorkspaceLifecycleState::Ready),
            observed_at: observed_at.into(),
        };
        Self {
            readiness_inputs,
            readiness_label,
            discovered_files,
            hot_set_candidates,
            full_index_complete,
            scope: None,
            planner_version: "hot-set-planner/alpha".to_string(),
            max_hot_targets: DEFAULT_MAX_HOT_SET_TARGETS,
        }
    }
}

/// Result of a scheduler pass.
#[derive(Debug, Clone)]
pub struct IndexSchedulerOutput {
    /// Explainable hot-set plan.
    pub plan: HotSetPlan,
    /// Lexical index snapshot consumed by quick open/search shell.
    pub lexical_index: LexicalIndexState,
    /// Exportable scheduler evidence snapshot.
    pub navigation_snapshot: FirstUsefulNavigationSnapshot,
}

/// Exportable proof that first-useful navigation is available before full indexing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstUsefulNavigationSnapshot {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha snapshot.
    pub schema_version: u32,
    /// Workspace identity for the scheduler pass.
    pub workspace_id: String,
    /// Stable scope identity active for first-useful navigation.
    pub stable_scope_id: String,
    /// Scope-class token active for first-useful navigation.
    pub scope_class: String,
    /// Sparse/full scope mode active for first-useful navigation.
    pub scope_mode: String,
    /// User-visible scope label active for first-useful navigation.
    pub scope_label: String,
    /// Canonical workset/slice metadata when the scheduler was bound to an artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_metadata: Option<WorkspaceSearchScopeMetadata>,
    /// Search-readiness token projected from the hot-set plan.
    pub readiness_state: String,
    /// User-visible readiness label.
    pub readiness_banner: String,
    /// Whether the full index was complete when the snapshot was emitted.
    pub full_index_complete: bool,
    /// Paths available for first-useful quick-open/file navigation.
    pub first_useful_paths: Vec<String>,
    /// Partial-truth cause tokens attached to this snapshot.
    pub partial_truth_causes: Vec<String>,
    /// Fallback reason token when hot-set targeting was not available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Explanations for selected hot-set targets.
    pub hot_set_explanations: Vec<HotSetExplanation>,
    /// True when edit input would wait on index warm-up.
    pub edit_blocked_by_index_warmup: bool,
    /// True when quick open would wait for full indexing.
    pub quick_open_blocked_by_full_index: bool,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
}

/// Combined scheduler plus quick-open snapshot for the first consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledQuickOpenSnapshot {
    /// Scheduler evidence snapshot.
    pub scheduler: FirstUsefulNavigationSnapshot,
    /// Search-shell snapshot produced from the scheduled lexical index.
    pub shell: LexicalShellSnapshot,
}

/// Stateless alpha scheduler for first-useful navigation.
#[derive(Debug, Default, Clone, Copy)]
pub struct IndexSchedulerAlpha;

impl IndexSchedulerAlpha {
    /// Builds a hot-set plan and the lexical index snapshot quick open consumes.
    pub fn schedule(inputs: IndexSchedulerInputs) -> IndexSchedulerOutput {
        let workspace_id = inputs.readiness_inputs.workspace_id.clone();
        let observed_at = inputs.readiness_inputs.observed_at.clone();
        let index_available = inputs.readiness_label != ReadinessLabel::Unavailable
            && !matches!(
                inputs.readiness_inputs.lifecycle_state_token,
                "closed" | "closing"
            );
        let plan = HotSetPlanner::plan(HotSetPlanInputs {
            workspace_id: workspace_id.clone(),
            observed_at: observed_at.clone(),
            planner_version: inputs.planner_version,
            index_available,
            full_index_complete: inputs.full_index_complete,
            discovered_files: inputs.discovered_files.clone(),
            candidates: inputs.hot_set_candidates,
            max_hot_targets: inputs.max_hot_targets,
        });

        let lexical_files = lexical_files_for_plan(&plan, &inputs.discovered_files);
        let readiness = lexical_readiness_for_plan(plan.readiness_state);
        let causes = lexical_causes_for_plan(&plan, &inputs.readiness_inputs);
        let navigation_snapshot =
            FirstUsefulNavigationSnapshot::from_plan(&plan, inputs.scope.as_ref());
        let lexical_index = LexicalIndexState::from_scheduled_files(
            workspace_id,
            observed_at,
            lexical_files,
            readiness,
            causes,
            inputs.scope,
            inputs.discovered_files.len() as u64,
        );

        IndexSchedulerOutput {
            plan,
            lexical_index,
            navigation_snapshot,
        }
    }

    /// Builds the first quick-open snapshot from a scheduled hot-set index.
    pub fn quick_open_snapshot(
        inputs: IndexSchedulerInputs,
        scope_class: ScopeClass,
        scope_label: impl Into<String>,
        query: LexicalQuery,
        observed_at: impl Into<String>,
    ) -> ScheduledQuickOpenSnapshot {
        let output = Self::schedule(inputs);
        let shell = LexicalShell::new(scope_class, scope_label, output.lexical_index, query);
        ScheduledQuickOpenSnapshot {
            scheduler: output.navigation_snapshot,
            shell: shell.export_snapshot(observed_at),
        }
    }
}

impl FirstUsefulNavigationSnapshot {
    /// Stable record-kind tag carried in serialized snapshots.
    pub const RECORD_KIND: &'static str = "first_useful_navigation_snapshot";

    /// Builds an exportable scheduler snapshot from a hot-set plan.
    pub fn from_plan(plan: &HotSetPlan, scope: Option<&WorkspaceSearchScope>) -> Self {
        let scope_metadata = scope.map(WorkspaceSearchScope::project_metadata);
        let stable_scope_id = scope_metadata
            .as_ref()
            .map(|metadata| metadata.stable_scope_id.clone())
            .unwrap_or_else(|| {
                format!(
                    "scope:{}:full_workspace",
                    plan.workspace_id.replace(':', "_")
                )
            });
        let scope_class = scope_metadata
            .as_ref()
            .map(|metadata| metadata.scope_class_token.clone())
            .unwrap_or_else(|| "full_workspace".to_string());
        let scope_mode = scope_metadata
            .as_ref()
            .map(|metadata| metadata.scope_mode_token.clone())
            .unwrap_or_else(|| "full".to_string());
        let scope_label = scope_metadata
            .as_ref()
            .map(|metadata| metadata.chip_label.clone())
            .unwrap_or_else(|| "Full workspace".to_string());
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: 1,
            workspace_id: plan.workspace_id.clone(),
            stable_scope_id,
            scope_class,
            scope_mode,
            scope_label,
            scope_metadata,
            readiness_state: plan.readiness_state.as_str().to_string(),
            readiness_banner: plan.readiness_state.banner_label().to_string(),
            full_index_complete: plan.full_index_complete,
            first_useful_paths: first_useful_paths_for_snapshot(plan),
            partial_truth_causes: plan
                .partial_truth_cause_tokens()
                .into_iter()
                .map(str::to_string)
                .collect(),
            fallback_reason: plan
                .fallback
                .as_ref()
                .map(|fallback| fallback.reason.as_str().to_string()),
            hot_set_explanations: plan.explanations(),
            edit_blocked_by_index_warmup: plan.responsiveness.edit_blocked_by_index_warmup,
            quick_open_blocked_by_full_index: plan.responsiveness.quick_open_blocked_by_full_index,
            observed_at: plan.observed_at.clone(),
        }
    }
}

fn lexical_files_for_plan(plan: &HotSetPlan, discovered_files: &[String]) -> Vec<String> {
    if plan.full_index_complete {
        return discovered_files.to_vec();
    }
    let hot_paths = plan.first_useful_file_paths();
    if !hot_paths.is_empty() {
        return hot_paths;
    }
    plan.fallback
        .as_ref()
        .map(|fallback| fallback.candidate_paths.clone())
        .unwrap_or_default()
}

fn first_useful_paths_for_snapshot(plan: &HotSetPlan) -> Vec<String> {
    let hot_paths = plan.first_useful_file_paths();
    if !hot_paths.is_empty() {
        return hot_paths;
    }
    plan.fallback
        .as_ref()
        .map(|fallback| fallback.candidate_paths.clone())
        .unwrap_or_default()
}

fn lexical_readiness_for_plan(readiness: SearchReadinessState) -> ReadinessClass {
    match readiness {
        SearchReadinessState::FullyIndexed => ReadinessClass::Ready,
        SearchReadinessState::HotSetReady => ReadinessClass::HotSetReady,
        SearchReadinessState::NotIndexed => ReadinessClass::Warming,
        SearchReadinessState::PartialIndex
        | SearchReadinessState::WarmIndex
        | SearchReadinessState::Reindexing => ReadinessClass::Partial,
        SearchReadinessState::StaleIndex => ReadinessClass::Stale,
        SearchReadinessState::IndexUnavailable => ReadinessClass::Unavailable,
    }
}

fn lexical_causes_for_plan(
    plan: &HotSetPlan,
    readiness_inputs: &WorkspaceReadinessInputs,
) -> Vec<PartialTruthCause> {
    let mut causes = Vec::new();
    for cause in &plan.partial_truth_causes {
        match cause {
            HotSetPartialTruthCause::HotSetOnly => causes.push(PartialTruthCause::HotSetOnly),
            HotSetPartialTruthCause::IndexingInProgress => {
                causes.push(PartialTruthCause::IndexingInProgress);
            }
            HotSetPartialTruthCause::StaleIndexServed => {}
        }
    }
    match readiness_inputs.watcher_health_token {
        None => causes.push(PartialTruthCause::WatcherUnknown),
        Some("healthy") | Some("warming") => {}
        Some(_) => causes.push(PartialTruthCause::WatcherDegraded),
    }
    match readiness_inputs.lifecycle_state_token {
        "partially_ready" => causes.push(PartialTruthCause::WorkspaceWarming),
        "closed" | "closing" => causes.push(PartialTruthCause::WorkspaceClosed),
        _ => {}
    }
    causes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hot_set::{HotSetInputClass, HotSetTarget, HotSetTargetKind};

    fn open_file(path: &str) -> HotSetCandidate {
        HotSetCandidate {
            target: HotSetTarget {
                target_kind: HotSetTargetKind::File,
                relative_path: path.to_string(),
                symbol_ref: None,
                display_label: None,
            },
            input_class: HotSetInputClass::OpenFile,
            input_ref: format!("file:{path}"),
            priority_reason: "Open file is in the protected hot set.".to_string(),
            project_node_refs: Vec::new(),
            target_project_node_refs: Vec::new(),
        }
    }

    #[test]
    fn hot_set_index_materializes_before_full_index() {
        let inputs = IndexSchedulerInputs::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            vec!["src/main.rs".to_string(), "src/cold.rs".to_string()],
            vec![open_file("src/main.rs")],
            false,
        );
        let output = IndexSchedulerAlpha::schedule(inputs);
        assert_eq!(
            output.plan.readiness_state,
            SearchReadinessState::HotSetReady
        );
        assert_eq!(
            output.lexical_index.readiness(),
            ReadinessClass::HotSetReady
        );
        assert_eq!(output.lexical_index.files(), &["src/main.rs".to_string()]);
        assert!(!output.navigation_snapshot.quick_open_blocked_by_full_index);
        assert!(!output.navigation_snapshot.edit_blocked_by_index_warmup);
    }
}
