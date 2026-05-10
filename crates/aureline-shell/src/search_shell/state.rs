//! Live search-shell state and shell-side render projection.

use serde::{Deserialize, Serialize};

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    LexicalIndexInputs, LexicalIndexState, LexicalQuery, LexicalShell, LexicalShellSnapshot,
    ScopeClass,
};
use aureline_workspace::{
    ScopeClass as WorkspaceScopeClass, WorkspaceLifecycleMachine, WorkspaceReadinessInputs,
};

/// Live workspace search-shell surface state.
///
/// The struct wraps a [`LexicalShell`] (owned by the search crate) plus the
/// shell-side wiring that keeps the shell aligned with the workspace
/// lifecycle machine and the live reactive store.
#[derive(Debug, Clone)]
pub struct WorkspaceSearchSurfaceState {
    inner: LexicalShell,
}

impl WorkspaceSearchSurfaceState {
    /// Construct a new search-shell surface for the given workspace.
    ///
    /// `workset_name` is the workset chip suffix shown when the active
    /// scope is narrower than the full workspace (e.g.
    /// `Selected workset · Hot path`).
    pub fn open(
        lifecycle: &WorkspaceLifecycleMachine,
        readiness_label: ReadinessLabel,
        scope_class: WorkspaceScopeClass,
        workset_name: Option<&str>,
        files: Vec<String>,
    ) -> Self {
        let scope = ScopeClass::from_workspace(scope_class);
        let label = project_scope_label(scope, workset_name);
        let index = build_index(lifecycle.readiness_inputs(), readiness_label, files);
        let shell = LexicalShell::with_empty_query(scope, label, index);
        Self { inner: shell }
    }

    /// Project the latest readiness inputs and label into a fresh index.
    /// Call after the workspace lifecycle observes a transition or after
    /// the reactive store republishes a workspace-readiness frame.
    pub fn refresh_index(
        &mut self,
        lifecycle: &WorkspaceLifecycleMachine,
        readiness_label: ReadinessLabel,
        files: Vec<String>,
    ) {
        let index = build_index(lifecycle.readiness_inputs(), readiness_label, files);
        self.inner.replace_index(index);
    }

    /// Update the active scope (e.g. after the workset switcher narrowed
    /// the view).
    pub fn set_scope(&mut self, scope_class: WorkspaceScopeClass, workset_name: Option<&str>) {
        let scope = ScopeClass::from_workspace(scope_class);
        let label = project_scope_label(scope, workset_name);
        self.inner.set_scope(scope, label);
    }

    /// Set the active query string. Empty queries clear the result set.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.inner.set_query(LexicalQuery::new(query));
    }

    /// Borrow the underlying shell (for tests / advanced consumers).
    pub fn lexical_shell(&self) -> &LexicalShell {
        &self.inner
    }

    /// Materialize a render-ready card projection for the shell chrome.
    pub fn render_card(&self) -> WorkspaceSearchSurfaceCard {
        let shell = &self.inner;
        let results = shell.results();
        let rows: Vec<WorkspaceSearchSurfaceCardRow> = results
            .groups
            .iter()
            .map(|group| WorkspaceSearchSurfaceCardRow {
                source_class_token: group.source_class.as_str().to_string(),
                lane_label: group.label.clone(),
                lane_badge: group.source_class.badge().to_string(),
                items: group
                    .items
                    .iter()
                    .map(|row| WorkspaceSearchSurfaceCardItem {
                        relative_path: row.relative_path.clone(),
                        match_kind_token: row.match_kind.as_str().to_string(),
                    })
                    .collect(),
            })
            .collect();

        WorkspaceSearchSurfaceCard {
            workspace_id: shell.workspace_id().to_string(),
            scope_class_token: shell.scope_class().as_str().to_string(),
            scope_chip_label: shell.scope_label().to_string(),
            readiness_class_token: results.readiness.as_str().to_string(),
            readiness_banner: results.readiness.banner_label().to_string(),
            partial_truth_causes: results.partial_truth_causes.clone(),
            query: shell.query().query.clone(),
            rows,
            total_rows: results.total_rows,
            // The card carries the source-class taxonomy explicitly so the
            // chrome cannot relabel a lexical row as semantic just because
            // a future surface renders alongside it.
            available_source_classes: vec!["lexical_filename".to_string(), "lexical_path".to_string()],
        }
    }

    /// Export a portable snapshot of the shell state. Surfaces use this to
    /// attach a search-shell snapshot to a support bundle without scraping
    /// the rendered chrome.
    pub fn export_snapshot(&self, observed_at: impl Into<String>) -> LexicalShellSnapshot {
        self.inner.export_snapshot(observed_at)
    }
}

fn build_index(
    readiness_inputs: WorkspaceReadinessInputs,
    readiness_label: ReadinessLabel,
    files: Vec<String>,
) -> LexicalIndexState {
    LexicalIndexState::from_inputs(LexicalIndexInputs {
        readiness_inputs,
        readiness_label,
        files,
    })
}

/// Project a scope chip label from the active scope and the workset name.
/// Mirrors `aureline_workspace::WorksetArtifactRecord::project_chip` so the
/// search shell never mints a parallel chip vocabulary.
pub fn project_scope_label(scope: ScopeClass, workset_name: Option<&str>) -> String {
    match scope {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
            scope.chip_label_family().to_string()
        }
        ScopeClass::SelectedWorkset
        | ScopeClass::SparseSlice
        | ScopeClass::PolicyLimitedView => match workset_name {
            Some(name) if !name.trim().is_empty() => {
                format!("{} · {}", scope.chip_label_family(), name)
            }
            _ => scope.chip_label_family().to_string(),
        },
    }
}

/// Render-ready projection of the search-shell surface.
///
/// The chrome consumes this struct directly; it should never inspect the
/// underlying `LexicalShell` to invent additional labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchSurfaceCard {
    pub workspace_id: String,
    pub scope_class_token: String,
    pub scope_chip_label: String,
    pub readiness_class_token: String,
    pub readiness_banner: String,
    pub partial_truth_causes: Vec<String>,
    pub query: String,
    pub rows: Vec<WorkspaceSearchSurfaceCardRow>,
    pub total_rows: usize,
    pub available_source_classes: Vec<String>,
}

/// One lane (group) in a [`WorkspaceSearchSurfaceCard`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchSurfaceCardRow {
    pub source_class_token: String,
    pub lane_label: String,
    pub lane_badge: String,
    pub items: Vec<WorkspaceSearchSurfaceCardItem>,
}

/// One row in a [`WorkspaceSearchSurfaceCardRow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchSurfaceCardItem {
    pub relative_path: String,
    pub match_kind_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::TrustState;

    fn ready_lifecycle() -> WorkspaceLifecycleMachine {
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-test", "mono:0");
        machine.open_workspace("mono:1");
        machine.resolve_trust(TrustState::Trusted, "mono:2");
        machine.mark_shell_interactive("mono:3");
        machine.update_readiness_gates(
            Some(aureline_vfs::WatcherHealth::Healthy),
            Some(true),
            Some(true),
            "mono:4",
            None,
        );
        machine
    }

    fn partial_lifecycle() -> WorkspaceLifecycleMachine {
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-test", "mono:0");
        machine.open_workspace("mono:1");
        machine.resolve_trust(TrustState::Trusted, "mono:2");
        machine.mark_shell_interactive("mono:3");
        machine.update_readiness_gates(
            Some(aureline_vfs::WatcherHealth::Warming),
            Some(false),
            Some(true),
            "mono:4",
            None,
        );
        machine
    }

    #[test]
    fn ready_workspace_renders_ready_card() {
        let lifecycle = ready_lifecycle();
        let mut surface = WorkspaceSearchSurfaceState::open(
            &lifecycle,
            ReadinessLabel::Exact,
            WorkspaceScopeClass::CurrentRepo,
            None,
            vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        );
        surface.set_query("main");

        let card = surface.render_card();
        assert_eq!(card.readiness_class_token, "ready");
        assert_eq!(card.readiness_banner, "Ready");
        assert!(card.partial_truth_causes.is_empty());
        assert_eq!(card.scope_class_token, "current_repo");
        assert_eq!(card.scope_chip_label, "Current repo");
        assert_eq!(card.total_rows, 1);
        assert_eq!(card.rows[0].source_class_token, "lexical_filename");
        assert_eq!(card.rows[0].lane_badge, "filename");
        assert!(card
            .available_source_classes
            .iter()
            .any(|s| s == "lexical_filename"));
        assert!(card
            .available_source_classes
            .iter()
            .any(|s| s == "lexical_path"));
    }

    #[test]
    fn warming_workspace_with_no_files_reports_warming_card() {
        let lifecycle = partial_lifecycle();
        let mut surface = WorkspaceSearchSurfaceState::open(
            &lifecycle,
            ReadinessLabel::Partial,
            WorkspaceScopeClass::CurrentRepo,
            None,
            Vec::new(),
        );
        surface.set_query("main");

        let card = surface.render_card();
        assert_eq!(card.readiness_class_token, "warming");
        assert!(card
            .partial_truth_causes
            .iter()
            .any(|c| c == "workspace_warming"));
        assert_eq!(card.total_rows, 0);
    }

    #[test]
    fn selected_workset_label_uses_chip_family_with_name() {
        let lifecycle = ready_lifecycle();
        let surface = WorkspaceSearchSurfaceState::open(
            &lifecycle,
            ReadinessLabel::Exact,
            WorkspaceScopeClass::SelectedWorkset,
            Some("Hot path"),
            vec!["src/main.rs".to_string()],
        );
        let card = surface.render_card();
        assert_eq!(card.scope_chip_label, "Selected workset · Hot path");
    }

    #[test]
    fn refresh_index_propagates_to_card() {
        let lifecycle_partial = partial_lifecycle();
        let mut surface = WorkspaceSearchSurfaceState::open(
            &lifecycle_partial,
            ReadinessLabel::Partial,
            WorkspaceScopeClass::CurrentRepo,
            None,
            vec!["src/main.rs".to_string()],
        );
        surface.set_query("main");
        assert_eq!(surface.render_card().readiness_class_token, "partial");

        let lifecycle_ready = ready_lifecycle();
        surface.refresh_index(
            &lifecycle_ready,
            ReadinessLabel::Exact,
            vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        );
        let card = surface.render_card();
        assert_eq!(card.readiness_class_token, "ready");
        assert_eq!(card.total_rows, 1);
    }
}
