//! Live lexical-search shell.
//!
//! [`LexicalShell`] is the runtime the search-shell surface in
//! `aureline-shell` consumes. It owns:
//!
//! - the latest [`LexicalIndexState`] (rows + readiness + partial-truth
//!   causes) supplied by the caller, and
//! - the latest [`LexicalQuery`] typed into the search box.
//!
//! Every state mutation produces a fresh [`LexicalSearchResults`]
//! materialization through [`super::query::run_query`]; the shell never
//! caches stale rows. A [`LexicalShellSnapshot`] export is provided so
//! support bundles can replay the same truth the live runtime saw.

use serde::{Deserialize, Serialize};

use super::index::LexicalIndexState;
use super::query::{run_query, LexicalQuery, LexicalSearchResults};
use super::scope::ScopeClass;
use super::source::SourceClass;
use crate::counts::{HiddenScopeDisclosure, ScopeWarningRecord, SearchScopeCountsRecord};
use crate::scope::WorkspaceSearchScopeMetadata;

/// Live workspace lexical-search shell state.
#[derive(Debug, Clone)]
pub struct LexicalShell {
    workspace_id: String,
    scope_class: ScopeClass,
    scope_label: String,
    index: LexicalIndexState,
    query: LexicalQuery,
    results: LexicalSearchResults,
}

impl LexicalShell {
    /// Construct a shell from an initial index, scope, and (optional) seeded
    /// query. The constructor materializes the first result snapshot so the
    /// caller can render immediately.
    pub fn new(
        scope_class: ScopeClass,
        scope_label: impl Into<String>,
        index: LexicalIndexState,
        query: LexicalQuery,
    ) -> Self {
        let workspace_id = index.workspace_id().to_string();
        let scope_label = scope_label.into();
        let results = run_query(&index, &query);
        Self {
            workspace_id,
            scope_class,
            scope_label,
            index,
            query,
            results,
        }
    }

    /// Build a shell with an empty seeded query.
    pub fn with_empty_query(
        scope_class: ScopeClass,
        scope_label: impl Into<String>,
        index: LexicalIndexState,
    ) -> Self {
        Self::new(scope_class, scope_label, index, LexicalQuery::new(""))
    }

    /// Workspace id that backs the shell.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Active scope class projected from the workspace surface.
    pub const fn scope_class(&self) -> ScopeClass {
        self.scope_class
    }

    /// Scope chip label (e.g. `Current repo · Hot path`).
    pub fn scope_label(&self) -> &str {
        &self.scope_label
    }

    /// Latest index snapshot.
    pub fn index(&self) -> &LexicalIndexState {
        &self.index
    }

    /// Active query.
    pub fn query(&self) -> &LexicalQuery {
        &self.query
    }

    /// Latest materialized result set.
    pub fn results(&self) -> &LexicalSearchResults {
        &self.results
    }

    /// Replace the index snapshot (e.g. after the watcher streamed a fresh
    /// scan) and re-materialize the result set against the current query.
    pub fn replace_index(&mut self, index: LexicalIndexState) {
        self.workspace_id = index.workspace_id().to_string();
        self.index = index;
        self.results = run_query(&self.index, &self.query);
    }

    /// Replace the active scope (e.g. after the workset switcher narrowed
    /// the view). The result set is re-materialized so the rendered scope
    /// chip and the rendered rows stay aligned.
    pub fn set_scope(&mut self, scope_class: ScopeClass, scope_label: impl Into<String>) {
        self.scope_class = scope_class;
        self.scope_label = scope_label.into();
        self.results = run_query(&self.index, &self.query);
    }

    /// Replace the query and re-materialize the result set.
    pub fn set_query(&mut self, query: LexicalQuery) {
        self.query = query;
        self.results = run_query(&self.index, &self.query);
    }

    /// Export a portable snapshot of the shell state suitable for support
    /// bundles and protected-row proof artifacts.
    pub fn export_snapshot(&self, observed_at: impl Into<String>) -> LexicalShellSnapshot {
        let groups: Vec<LexicalShellSnapshotGroup> = self
            .results
            .groups
            .iter()
            .map(|group| LexicalShellSnapshotGroup {
                source_class: group.source_class.as_str().to_string(),
                label: group.label.clone(),
                items: group
                    .items
                    .iter()
                    .map(|row| LexicalShellSnapshotItem {
                        relative_path: row.relative_path.clone(),
                        source_class: row.source_class.as_str().to_string(),
                        match_kind: row.match_kind.as_str().to_string(),
                    })
                    .collect(),
            })
            .collect();

        let scope_metadata = self.index.scope().map(|scope| scope.project_metadata());
        let stable_scope_id = scope_metadata
            .as_ref()
            .map(|metadata| metadata.stable_scope_id.clone())
            .unwrap_or_else(|| default_stable_scope_id(self.scope_class, &self.scope_label));
        let scope_mode = scope_metadata
            .as_ref()
            .map(|metadata| metadata.scope_mode_token.clone())
            .unwrap_or_else(|| default_scope_mode(self.scope_class).to_string());
        let workset_id = scope_metadata
            .as_ref()
            .and_then(|metadata| metadata.workset_id.clone());

        LexicalShellSnapshot {
            record_kind: LexicalShellSnapshot::RECORD_KIND.to_string(),
            schema_version: 1,
            workspace_id: self.workspace_id.clone(),
            scope_class: self.scope_class.as_str().to_string(),
            stable_scope_id,
            scope_mode,
            workset_id,
            scope_label: self.scope_label.clone(),
            scope_metadata,
            query: self.query.query.clone(),
            readiness_class: self.results.readiness.as_str().to_string(),
            readiness_banner: self.results.readiness.banner_label().to_string(),
            partial_truth_causes: self.results.partial_truth_causes.clone(),
            counts: self.results.counts.clone(),
            empty_state_token: self.results.empty_state.as_str().to_string(),
            empty_state_label: self.results.empty_state.label().to_string(),
            hidden_scope_disclosure: self.results.hidden_scope_disclosure.clone(),
            scope_warnings: self.results.scope_warnings.clone(),
            groups,
            total_rows: self.results.total_rows,
            observed_at: observed_at.into(),
            // The shell explicitly records the source-class vocabulary so
            // support bundles cannot silently re-attribute a lexical row to
            // a future semantic lane.
            available_source_classes: vec![
                SourceClass::LexicalFilename.as_str().to_string(),
                SourceClass::LexicalPath.as_str().to_string(),
            ],
        }
    }
}

/// Exportable snapshot record of a [`LexicalShell`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalShellSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub scope_class: String,
    pub stable_scope_id: String,
    pub scope_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    pub scope_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_metadata: Option<WorkspaceSearchScopeMetadata>,
    pub query: String,
    pub readiness_class: String,
    pub readiness_banner: String,
    pub partial_truth_causes: Vec<String>,
    pub counts: SearchScopeCountsRecord,
    pub empty_state_token: String,
    pub empty_state_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_scope_disclosure: Option<HiddenScopeDisclosure>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope_warnings: Vec<ScopeWarningRecord>,
    pub groups: Vec<LexicalShellSnapshotGroup>,
    pub total_rows: usize,
    pub observed_at: String,
    pub available_source_classes: Vec<String>,
}

impl LexicalShellSnapshot {
    /// Stable record-kind tag carried in serialized snapshots.
    pub const RECORD_KIND: &'static str = "workspace_search_shell_snapshot";
}

/// Snapshot representation of one search-shell result group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalShellSnapshotGroup {
    pub source_class: String,
    pub label: String,
    pub items: Vec<LexicalShellSnapshotItem>,
}

/// Snapshot representation of one search-shell row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalShellSnapshotItem {
    pub relative_path: String,
    pub source_class: String,
    pub match_kind: String,
}

fn default_stable_scope_id(scope_class: ScopeClass, scope_label: &str) -> String {
    format!(
        "scope:{}:{}",
        scope_class.as_str(),
        crate::stable_query_hash(scope_label)
    )
}

fn default_scope_mode(scope_class: ScopeClass) -> &'static str {
    match scope_class {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => "full",
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            "sparse"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexical::index::{LexicalIndexState, ReadinessClass};
    use aureline_reactive_state::ReadinessLabel;
    use aureline_workspace::WorkspaceLifecycleState;

    fn ready_index() -> LexicalIndexState {
        LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Ready,
            ReadinessLabel::Exact,
            vec![
                "src/main.rs".to_string(),
                "src/lib.rs".to_string(),
                "tests/smoke.rs".to_string(),
            ],
        )
    }

    #[test]
    fn shell_materializes_results_on_construction() {
        let shell = LexicalShell::new(
            ScopeClass::CurrentRepo,
            "Current repo",
            ready_index(),
            LexicalQuery::new("main"),
        );
        assert_eq!(shell.results().readiness, ReadinessClass::Ready);
        assert_eq!(shell.results().total_rows, 1);
    }

    #[test]
    fn updating_query_re_runs_against_current_index() {
        let mut shell =
            LexicalShell::with_empty_query(ScopeClass::CurrentRepo, "Current repo", ready_index());
        assert_eq!(shell.results().total_rows, 0);
        shell.set_query(LexicalQuery::new("smoke"));
        assert_eq!(shell.results().total_rows, 1);
        assert_eq!(
            shell.results().groups[0].items[0].relative_path,
            "tests/smoke.rs"
        );
    }

    #[test]
    fn snapshot_round_trips_through_serde() {
        let shell = LexicalShell::new(
            ScopeClass::CurrentRepo,
            "Current repo",
            ready_index(),
            LexicalQuery::new("lib"),
        );
        let snapshot = shell.export_snapshot("mono:42");
        let json = serde_json::to_string(&snapshot).expect("snapshot must serialize");
        let parsed: LexicalShellSnapshot =
            serde_json::from_str(&json).expect("snapshot must round-trip");
        assert_eq!(parsed.workspace_id, "ws-test");
        assert_eq!(parsed.readiness_class, "ready");
        assert_eq!(parsed.scope_class, "current_repo");
        assert_eq!(parsed.scope_mode, "full");
        assert!(parsed.stable_scope_id.starts_with("scope:current_repo:"));
    }

    #[test]
    fn warming_index_keeps_partial_causes_in_results() {
        let warming = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            vec!["src/main.rs".to_string()],
        );
        let shell = LexicalShell::new(
            ScopeClass::CurrentRepo,
            "Current repo",
            warming,
            LexicalQuery::new("main"),
        );
        assert_eq!(shell.results().readiness, ReadinessClass::Partial);
        assert!(!shell.results().partial_truth_causes.is_empty());
    }
}
