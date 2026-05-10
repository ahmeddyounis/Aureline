//! Lexical index state and readiness projection.
//!
//! The index is intentionally simple: it owns a list of workspace-relative
//! file paths plus a [`ReadinessClass`] derived from the upstream workspace
//! lifecycle and watcher posture. The shell asks the index for its current
//! readiness and partial-truth causes; it does not invent its own loading
//! vocabulary.
//!
//! Readiness derivation is total and matches the
//! [`aureline_reactive_state::ReadinessLabel`] taxonomy: the projection is
//! built from one of the published workspace-readiness frames, not from a
//! local watcher snapshot. This keeps the search shell honest if the live
//! reactive store later starts publishing the same readiness from a
//! different producer.

use serde::{Deserialize, Serialize};

use aureline_reactive_state::ReadinessLabel;
use aureline_workspace::{WorkspaceLifecycleState, WorkspaceReadinessInputs};

use crate::scope::WorkspaceSearchScope;

/// Stable readiness vocabulary surfaced to the search shell.
///
/// Tokens are a strict subset of the upstream `ReadinessLabel` set with one
/// extra `Warming` token to disambiguate the "we have not produced any rows
/// yet" case from "we have rows but the index is still scanning".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessClass {
    /// Index is fully populated and the workspace is ready.
    Ready,
    /// Index has not produced rows yet (scan in progress, pre-rows).
    Warming,
    /// Index has rows but the upstream workspace is still partial / warming.
    Partial,
    /// Index is from a cached snapshot; not yet refreshed against truth.
    Stale,
    /// Index is unavailable (workspace closed, watcher down, etc.).
    Unavailable,
    /// Subject is outside the current scope (not a failure).
    OutOfScope,
}

impl ReadinessClass {
    /// Stable token used in records, fixtures, and shell snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Short banner suitable for a search-shell scope/readiness chip.
    pub const fn banner_label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Warming => "Warming",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Unavailable => "Unavailable",
            Self::OutOfScope => "Out of scope",
        }
    }

    /// Project from the upstream readiness label and an `index_has_rows`
    /// boolean. The boolean disambiguates `warming` (no rows yet) from
    /// `partial` (rows present but coverage is incomplete) when the
    /// upstream label is `Partial`.
    pub fn project(label: ReadinessLabel, index_has_rows: bool) -> Self {
        match label {
            ReadinessLabel::Exact => Self::Ready,
            ReadinessLabel::Imported => Self::Stale,
            ReadinessLabel::Heuristic => Self::Partial,
            ReadinessLabel::Stale => Self::Stale,
            ReadinessLabel::Partial => {
                if index_has_rows {
                    Self::Partial
                } else {
                    Self::Warming
                }
            }
            ReadinessLabel::Unavailable => Self::Unavailable,
            ReadinessLabel::OutOfScope => Self::OutOfScope,
        }
    }

    /// True when the surface should label the result set as not-yet
    /// authoritative (i.e. anything other than `ready`).
    pub const fn is_partial(self) -> bool {
        !matches!(self, Self::Ready)
    }
}

/// Stable vocabulary describing why a result set is not authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialTruthCause {
    /// Workspace is still warming up (lifecycle = `partially_ready`).
    WorkspaceWarming,
    /// Watcher is degraded or has fallen back to polling.
    WatcherDegraded,
    /// Watcher reports no health signal; we cannot promise live freshness.
    WatcherUnknown,
    /// Hot index gate has not yet completed.
    HotIndexNotReady,
    /// Lexical index has scanned but holds zero rows yet.
    NoRowsScannedYet,
    /// Workspace lifecycle is closed / closing.
    WorkspaceClosed,
}

impl PartialTruthCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceWarming => "workspace_warming",
            Self::WatcherDegraded => "watcher_degraded",
            Self::WatcherUnknown => "watcher_unknown",
            Self::HotIndexNotReady => "hot_index_not_ready",
            Self::NoRowsScannedYet => "no_rows_scanned_yet",
            Self::WorkspaceClosed => "workspace_closed",
        }
    }
}

/// Inputs the shell hands the index when it builds or refreshes itself.
///
/// The lifecycle inputs come from
/// [`aureline_workspace::WorkspaceLifecycleMachine::readiness_inputs`]; the
/// reactive label is the latest projection from the workspace-readiness
/// producer. The index never inspects the filesystem or watcher directly:
/// callers feed the rows in, the shell projects readiness.
#[derive(Debug, Clone)]
pub struct LexicalIndexInputs {
    /// Workspace identity and lifecycle vocabulary.
    pub readiness_inputs: WorkspaceReadinessInputs,
    /// Latest readiness label projected by the live reactive store.
    pub readiness_label: ReadinessLabel,
    /// Workspace-relative file paths. Order is preserved in result groups.
    pub files: Vec<String>,
    /// Optional active workset/slice scope. When `Some`, the constructor
    /// filters [`Self::files`] through the scope's include/exclude pattern
    /// set before the index dedups them, and the index keeps the scope
    /// projection so snapshots can carry the chip label and pattern
    /// fingerprint that produced the visible row set.
    #[doc(hidden)]
    pub scope: Option<WorkspaceSearchScope>,
}

/// One snapshot of the lexical index, ready to answer queries.
#[derive(Debug, Clone)]
pub struct LexicalIndexState {
    workspace_id: String,
    files: Vec<String>,
    readiness: ReadinessClass,
    causes: Vec<PartialTruthCause>,
    observed_at: String,
    scope: Option<WorkspaceSearchScope>,
    all_workspace_count: u64,
    out_of_scope_count: u64,
}

impl LexicalIndexState {
    /// Build a new index state from the supplied inputs. The constructor
    /// derives the readiness class and the partial-truth causes; callers
    /// should not invent their own `is_warming` flag.
    pub fn from_inputs(inputs: LexicalIndexInputs) -> Self {
        let LexicalIndexInputs {
            readiness_inputs,
            readiness_label,
            files,
            scope,
        } = inputs;

        let all_workspace_count = files.len() as u64;
        let mut filtered_files = files;
        let mut out_of_scope_count: u64 = 0;
        if let Some(scope_ref) = scope.as_ref() {
            let outcome = scope_ref.filter_files(filtered_files);
            out_of_scope_count = outcome.all_workspace_count - outcome.in_scope_count;
            filtered_files = outcome.in_scope;
        }
        let mut sorted_files = filtered_files;
        sorted_files.sort();
        sorted_files.dedup();

        let has_rows = !sorted_files.is_empty();
        let readiness = ReadinessClass::project(readiness_label, has_rows);

        let mut causes: Vec<PartialTruthCause> = Vec::new();
        if !readiness_inputs.hot_index_ready {
            causes.push(PartialTruthCause::HotIndexNotReady);
        }
        match readiness_inputs.watcher_health_token {
            None => causes.push(PartialTruthCause::WatcherUnknown),
            Some("healthy") => {}
            Some("warming") => {}
            Some(_) => causes.push(PartialTruthCause::WatcherDegraded),
        }
        match readiness_inputs.lifecycle_state_token {
            "partially_ready" => causes.push(PartialTruthCause::WorkspaceWarming),
            "closed" | "closing" => causes.push(PartialTruthCause::WorkspaceClosed),
            _ => {}
        }
        if !has_rows && readiness != ReadinessClass::Unavailable {
            causes.push(PartialTruthCause::NoRowsScannedYet);
        }
        causes.sort_by_key(|c| c.as_str());
        causes.dedup_by_key(|c| c.as_str());

        Self {
            workspace_id: readiness_inputs.workspace_id,
            files: sorted_files,
            readiness,
            causes,
            observed_at: readiness_inputs.observed_at,
            scope,
            all_workspace_count,
            out_of_scope_count,
        }
    }

    /// Convenience constructor for tests / fixtures: build an index state
    /// from a synthetic lifecycle state with default trust/watcher posture.
    pub fn for_fixture(
        workspace_id: impl Into<String>,
        observed_at: impl Into<String>,
        lifecycle_state: WorkspaceLifecycleState,
        readiness_label: ReadinessLabel,
        files: Vec<String>,
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
            hot_index_ready: matches!(lifecycle_state, WorkspaceLifecycleState::Ready),
            command_graph_ready: matches!(lifecycle_state, WorkspaceLifecycleState::Ready),
            observed_at: observed_at.into(),
        };
        Self::from_inputs(LexicalIndexInputs {
            readiness_inputs,
            readiness_label,
            files,
            scope: None,
        })
    }

    /// Workspace id this index belongs to.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Workspace-relative file paths backing the index.
    pub fn files(&self) -> &[String] {
        &self.files
    }

    /// Latest readiness class.
    pub const fn readiness(&self) -> ReadinessClass {
        self.readiness
    }

    /// Causes describing why the result set is not authoritative.
    pub fn partial_truth_causes(&self) -> &[PartialTruthCause] {
        &self.causes
    }

    /// `observed_at` timestamp surfaced through the search-shell snapshot.
    pub fn observed_at(&self) -> &str {
        &self.observed_at
    }

    /// Active workset/slice scope, when any. Search-shell consumers project
    /// the chip label and the snapshot metadata from this single source of
    /// truth — they MUST NOT mint a parallel workset/slice projection.
    pub fn scope(&self) -> Option<&WorkspaceSearchScope> {
        self.scope.as_ref()
    }

    /// Total file count handed to the index before scope filtering. This is
    /// the `all_matching_in_workspace` upper bound the chrome can disclose
    /// alongside the visible / loaded counts.
    pub const fn all_workspace_count(&self) -> u64 {
        self.all_workspace_count
    }

    /// Count of paths the active scope filtered out. Surfaces use this to
    /// disclose how many files are hidden by the active workset/slice.
    pub const fn out_of_scope_count(&self) -> u64 {
        self.out_of_scope_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::WorkspaceLifecycleState;

    #[test]
    fn ready_workspace_with_files_is_ready() {
        let state = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Ready,
            ReadinessLabel::Exact,
            vec!["src/main.rs".to_string()],
        );
        assert_eq!(state.readiness(), ReadinessClass::Ready);
        assert!(state.partial_truth_causes().is_empty());
    }

    #[test]
    fn partially_ready_with_no_rows_reports_warming() {
        let state = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            Vec::new(),
        );
        assert_eq!(state.readiness(), ReadinessClass::Warming);
        assert!(state
            .partial_truth_causes()
            .contains(&PartialTruthCause::WorkspaceWarming));
        assert!(state
            .partial_truth_causes()
            .contains(&PartialTruthCause::HotIndexNotReady));
        assert!(state
            .partial_truth_causes()
            .contains(&PartialTruthCause::NoRowsScannedYet));
    }

    #[test]
    fn partially_ready_with_rows_reports_partial() {
        let state = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::PartiallyReady,
            ReadinessLabel::Partial,
            vec!["src/main.rs".to_string()],
        );
        assert_eq!(state.readiness(), ReadinessClass::Partial);
        assert!(state
            .partial_truth_causes()
            .contains(&PartialTruthCause::WorkspaceWarming));
    }

    #[test]
    fn degraded_workspace_reports_unavailable_when_label_unavailable() {
        let state = LexicalIndexState::for_fixture(
            "ws-test",
            "mono:1",
            WorkspaceLifecycleState::Degraded,
            ReadinessLabel::Unavailable,
            vec!["src/main.rs".to_string()],
        );
        assert_eq!(state.readiness(), ReadinessClass::Unavailable);
        assert!(state
            .partial_truth_causes()
            .contains(&PartialTruthCause::WatcherDegraded));
    }
}
