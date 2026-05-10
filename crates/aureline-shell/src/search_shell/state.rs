//! Live search-shell state and shell-side render projection.

use serde::{Deserialize, Serialize};

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    LexicalIndexInputs, LexicalIndexState, LexicalQuery, LexicalShell, LexicalShellSnapshot,
    LineageHintRecord, ReadinessClass, ScopeClass,
};
use aureline_workspace::{
    ScopeClass as WorkspaceScopeClass, WorkspaceLifecycleMachine, WorkspaceReadinessInputs,
};

use crate::scope_truth::{
    project_scope_truth_chip_card, ScopeCountsInputs, ScopeCountsRecord, ScopeTruthChipCard,
    ScopeTruthSurfaceClass,
};

/// Live workspace search-shell surface state.
///
/// The struct wraps a [`LexicalShell`] (owned by the search crate) plus the
/// shell-side wiring that keeps the shell aligned with the workspace
/// lifecycle machine and the live reactive store.
#[derive(Debug, Clone)]
pub struct WorkspaceSearchSurfaceState {
    inner: LexicalShell,
    workset_name: Option<String>,
    observed_at: String,
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
        let inputs = lifecycle.readiness_inputs();
        let observed_at = inputs.observed_at.clone();
        let index = build_index(inputs, readiness_label, files);
        let shell = LexicalShell::with_empty_query(scope, label, index);
        Self {
            inner: shell,
            workset_name: workset_name.map(|s| s.to_string()),
            observed_at,
        }
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
        let inputs = lifecycle.readiness_inputs();
        self.observed_at = inputs.observed_at.clone();
        let index = build_index(inputs, readiness_label, files);
        self.inner.replace_index(index);
    }

    /// Update the active scope (e.g. after the workset switcher narrowed
    /// the view).
    pub fn set_scope(&mut self, scope_class: WorkspaceScopeClass, workset_name: Option<&str>) {
        let scope = ScopeClass::from_workspace(scope_class);
        let label = project_scope_label(scope, workset_name);
        self.inner.set_scope(scope, label);
        self.workset_name = workset_name.map(|s| s.to_string());
    }

    /// Set the active query string. Empty queries clear the result set.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.inner.set_query(LexicalQuery::new(query));
    }

    /// Borrow the underlying shell (for tests / advanced consumers).
    pub fn lexical_shell(&self) -> &LexicalShell {
        &self.inner
    }

    /// Project the scope-truth chip card the chrome renders alongside
    /// search results. The card is the canonical M01-062 disclosure for
    /// scope, partiality, and visible/loaded/all-matching counts.
    pub fn scope_truth_chip(&self) -> ScopeTruthChipCard {
        let shell = &self.inner;
        let results = shell.results();
        let workspace_scope = workspace_scope_from(shell.scope_class());
        let readiness_is_ready = matches!(results.readiness, ReadinessClass::Ready);
        let visible_in_view = results.total_rows as u64;
        let loaded_in_scope = if shell.query().is_empty() {
            None
        } else {
            // Until widened-search ships, the loaded-scope count we can
            // promise is the post-truncation rendered count. Surfaces MUST
            // NOT default `all_matching_in_workspace` to this value.
            Some(visible_in_view)
        };
        let counts = ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view,
            loaded_in_scope,
            all_matching_in_workspace: None,
            scope_covers_workspace: matches!(workspace_scope, WorkspaceScopeClass::FullWorkspace),
            readiness_is_ready,
        });
        project_scope_truth_chip_card(
            shell.workspace_id(),
            ScopeTruthSurfaceClass::SearchShell,
            workspace_scope,
            self.workset_name.as_deref(),
            counts,
            self.observed_at.clone(),
        )
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
                        generated_artifact_hint: row
                            .generated_artifact_hint
                            .as_ref()
                            .map(WorkspaceSearchSurfaceLineageHint::from_record),
                    })
                    .collect(),
            })
            .collect();

        let scope_truth_chip = self.scope_truth_chip();
        WorkspaceSearchSurfaceCard {
            workspace_id: shell.workspace_id().to_string(),
            scope_class_token: shell.scope_class().as_str().to_string(),
            scope_chip_label: shell.scope_label().to_string(),
            scope_truth_chip,
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

fn workspace_scope_from(scope: ScopeClass) -> WorkspaceScopeClass {
    match scope {
        ScopeClass::CurrentRepo => WorkspaceScopeClass::CurrentRepo,
        ScopeClass::SelectedWorkset => WorkspaceScopeClass::SelectedWorkset,
        ScopeClass::SparseSlice => WorkspaceScopeClass::SparseSlice,
        ScopeClass::FullWorkspace => WorkspaceScopeClass::FullWorkspace,
        ScopeClass::PolicyLimitedView => WorkspaceScopeClass::PolicyLimitedView,
    }
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
    /// Canonical scope-truth chip card disclosing scope class, partiality,
    /// presentation state, offered actions, and visible/loaded/all-matching
    /// counts. Surfaces MUST render this directly; the legacy
    /// `scope_chip_label` and `scope_class_token` fields are retained for
    /// reviewers that still consume the older single-string vocabulary.
    pub scope_truth_chip: ScopeTruthChipCard,
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
    /// Generated-artifact lineage hint when the row's relative path matches a
    /// rule in the workspace generated-artifact catalog. Surfaces MUST render
    /// this directly so a generated row never reads as the canonical edit
    /// target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_hint: Option<WorkspaceSearchSurfaceLineageHint>,
}

/// Lineage hint projection rendered next to a search-shell row.
///
/// The shell consumes the canonical [`LineageHintRecord`] from the workspace
/// surface and projects the user-visible labels here so the chrome doesn't
/// re-derive class / freshness vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchSurfaceLineageHint {
    pub generated_class_token: String,
    pub generated_class_label: String,
    pub badge: String,
    pub freshness_class_token: String,
    pub freshness_label: String,
    pub producer_id: String,
    pub producer_label: String,
    pub explainer: String,
    pub rule_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_canonical_relative_path: Option<String>,
}

impl WorkspaceSearchSurfaceLineageHint {
    /// Build a chrome-facing hint projection from a canonical record.
    pub fn from_record(record: &LineageHintRecord) -> Self {
        Self {
            generated_class_token: record.generated_class.as_str().to_string(),
            generated_class_label: record.generated_class.label().to_string(),
            badge: record.generated_class.badge().to_string(),
            freshness_class_token: record.freshness_class.as_str().to_string(),
            freshness_label: record.freshness_class.label().to_string(),
            producer_id: record.producer_id.clone(),
            producer_label: record.producer_label.clone(),
            explainer: record.explainer.clone(),
            rule_id: record.rule_id.clone(),
            source_canonical_relative_path: record.source_canonical_relative_path.clone(),
        }
    }
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

        // Even on a ready CurrentRepo scope, the chip flags partial_scope
        // because the scope is narrower than the workspace.
        let chip = &card.scope_truth_chip;
        assert_eq!(chip.scope_class_token, "current_repo");
        assert_eq!(chip.chip_label, "Current repo");
        assert_eq!(chip.surface_class_token, "search_shell");
        assert!(chip.partial_scope);
        assert!(chip
            .offered_action_tokens
            .iter()
            .any(|t| t == "widen_to_full_workspace"));
        assert_eq!(chip.counts.visible_in_view, 1);
        assert_eq!(chip.counts.loaded_in_scope, Some(1));
        assert!(chip.counts.all_matching_in_workspace.is_none());
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

        let chip = &card.scope_truth_chip;
        assert!(chip.partial_scope);
        assert_eq!(chip.presentation_state_token, "active_partial");
        assert!(!chip.counts.readiness_is_ready);
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

        let chip = &card.scope_truth_chip;
        assert_eq!(chip.scope_class_token, "selected_workset");
        assert_eq!(chip.chip_label, "Selected workset · Hot path");
        assert_eq!(chip.workset_name.as_deref(), Some("Hot path"));
        assert!(chip.partial_scope);
        assert!(chip
            .offered_action_tokens
            .iter()
            .any(|t| t == "widen_with_review"));
        assert!(chip
            .offered_action_tokens
            .iter()
            .any(|t| t == "open_scope_diff"));
    }

    #[test]
    fn lockfile_row_card_carries_lineage_hint() {
        let lifecycle = ready_lifecycle();
        let mut surface = WorkspaceSearchSurfaceState::open(
            &lifecycle,
            ReadinessLabel::Exact,
            WorkspaceScopeClass::CurrentRepo,
            None,
            vec![
                "Cargo.lock".to_string(),
                "Cargo.toml".to_string(),
                "src/main.rs".to_string(),
            ],
        );
        surface.set_query("cargo");

        let card = surface.render_card();
        let lockfile_item = card
            .rows
            .iter()
            .flat_map(|row| row.items.iter())
            .find(|item| item.relative_path == "Cargo.lock")
            .expect("Cargo.lock row must surface");
        let hint = lockfile_item
            .generated_artifact_hint
            .as_ref()
            .expect("lockfile row must carry lineage hint");
        assert_eq!(hint.generated_class_token, "lockfile");
        assert_eq!(hint.freshness_class_token, "derived_from_canonical");
        assert_eq!(
            hint.source_canonical_relative_path.as_deref(),
            Some("Cargo.toml")
        );
        assert_eq!(hint.rule_id, "lockfile.cargo");

        let manifest_item = card
            .rows
            .iter()
            .flat_map(|row| row.items.iter())
            .find(|item| item.relative_path == "Cargo.toml")
            .expect("Cargo.toml row must surface");
        assert!(
            manifest_item.generated_artifact_hint.is_none(),
            "Cargo.toml is the canonical source and must not carry a generated hint",
        );
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
