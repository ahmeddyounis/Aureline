//! Workset/slice scope projection consumed by the search and quick-open
//! shells.
//!
//! [`WorkspaceSearchScope`] is the canonical struct surfaces project from
//! either:
//!
//! - a full [`WorksetArtifactRecord`] (the primary path on the protected
//!   walk: opened workspace + active workset), or
//! - a bare [`crate::lexical::ScopeClass`] when the workspace is in a
//!   pre-workset state (Start Center landing on `current_repo` /
//!   `full_workspace`).
//!
//! Surfaces consume the projection through the methods on this struct and
//! through [`WorkspaceSearchScope::filter_files`]; they MUST NOT reach into
//! the workset record to re-derive chip labels, presentation states, or
//! pattern semantics.

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    ChipPresentationState as WorkspaceChipPresentationState, ScopeClass as WorkspaceScopeClass,
    WorksetArtifactRecord,
};

use crate::lexical::ScopeClass;

use super::filter::{
    glob_matches_relative_path, ScopeFilterOutcome, ScopePatternKind, ScopePatternRecord,
};

/// Stable presentation-state vocabulary projected onto search-shell and
/// quick-open chips. Mirrors [`WorkspaceChipPresentationState`] but with the
/// `OutsideCurrentScope` value omitted (that case is owned by the row-marker
/// chip path, not by the active scope projection).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePresentationState {
    ActiveNarrowSafe,
    ActivePartial,
    ActivePolicyLimited,
    ActiveWidened,
}

impl ScopePresentationState {
    /// Stable token used in records, fixtures, and snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveNarrowSafe => "active_narrow_safe",
            Self::ActivePartial => "active_partial",
            Self::ActivePolicyLimited => "active_policy_limited",
            Self::ActiveWidened => "active_widened",
        }
    }

    /// True when the presentation MUST surface a partiality cue alongside
    /// the chip.
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::ActivePartial | Self::ActivePolicyLimited)
    }

    fn from_workspace(state: WorkspaceChipPresentationState) -> Self {
        match state {
            WorkspaceChipPresentationState::ActiveNarrowSafe => Self::ActiveNarrowSafe,
            WorkspaceChipPresentationState::ActivePartial => Self::ActivePartial,
            WorkspaceChipPresentationState::ActivePolicyLimited => Self::ActivePolicyLimited,
            WorkspaceChipPresentationState::ActiveWidened => Self::ActiveWidened,
            // The active scope chip never carries the outside-scope state;
            // collapse it to the next-strongest partial cue so a misuse
            // upstream still produces a partial chip rather than an
            // authoritative one.
            WorkspaceChipPresentationState::OutsideCurrentScope => Self::ActivePartial,
        }
    }
}

/// Workset/slice scope the search shell projects onto query results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchScope {
    workspace_id: String,
    scope_class: ScopeClass,
    workset_id: Option<String>,
    workset_name: Option<String>,
    chip_label: String,
    presentation_state: ScopePresentationState,
    root_refs: Vec<String>,
    patterns: Vec<ScopePatternRecord>,
    partial_index_note: Option<String>,
    is_policy_limited: bool,
    has_partial_member_truth: bool,
}

impl WorkspaceSearchScope {
    /// Construct a scope projection for the bare full-workspace case (no
    /// workset artifact in play). The chrome treats this as the
    /// authoritative-by-default scope.
    pub fn for_full_workspace(workspace_id: impl Into<String>) -> Self {
        let scope_class = ScopeClass::FullWorkspace;
        Self {
            workspace_id: workspace_id.into(),
            scope_class,
            workset_id: None,
            workset_name: None,
            chip_label: scope_class.chip_label_family().to_string(),
            presentation_state: ScopePresentationState::ActiveNarrowSafe,
            root_refs: Vec::new(),
            patterns: Vec::new(),
            partial_index_note: None,
            is_policy_limited: false,
            has_partial_member_truth: false,
        }
    }

    /// Construct a scope projection for a narrowed scope class without a
    /// full workset artifact (e.g. a workset stub during the transition
    /// from Start Center to live shell). The chip label uses the supplied
    /// `workset_name` when the scope class supports it; pattern filtering
    /// is a no-op until a full artifact lands.
    pub fn for_workset_stub(
        workspace_id: impl Into<String>,
        scope_class: ScopeClass,
        workset_name: Option<&str>,
    ) -> Self {
        let chip_label = build_chip_label(scope_class, workset_name);
        let presentation_state = match scope_class {
            ScopeClass::PolicyLimitedView => ScopePresentationState::ActivePolicyLimited,
            _ => ScopePresentationState::ActiveNarrowSafe,
        };
        let workset_name = match scope_class {
            ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => None,
            _ => workset_name
                .map(|s| s.to_string())
                .filter(|s| !s.trim().is_empty()),
        };
        Self {
            workspace_id: workspace_id.into(),
            scope_class,
            workset_id: None,
            workset_name,
            chip_label,
            presentation_state,
            root_refs: Vec::new(),
            patterns: Vec::new(),
            partial_index_note: None,
            is_policy_limited: matches!(scope_class, ScopeClass::PolicyLimitedView),
            has_partial_member_truth: false,
        }
    }

    /// Construct a scope projection for the bare current-repo case (no
    /// workset artifact in play). Always reads as narrowed because the
    /// workspace may carry more than one root.
    pub fn for_current_repo(workspace_id: impl Into<String>) -> Self {
        let scope_class = ScopeClass::CurrentRepo;
        Self {
            workspace_id: workspace_id.into(),
            scope_class,
            workset_id: None,
            workset_name: None,
            chip_label: scope_class.chip_label_family().to_string(),
            presentation_state: ScopePresentationState::ActiveNarrowSafe,
            root_refs: Vec::new(),
            patterns: Vec::new(),
            partial_index_note: None,
            is_policy_limited: false,
            has_partial_member_truth: false,
        }
    }

    /// Project a [`WorkspaceSearchScope`] from a workset artifact. The
    /// projection mirrors the chip vocabulary already exposed by the
    /// workspace crate; the search shell does not re-derive chip labels.
    pub fn from_workset_artifact(
        workspace_id: impl Into<String>,
        artifact: &WorksetArtifactRecord,
    ) -> Self {
        let workspace_id = workspace_id.into();
        let scope_class = ScopeClass::from_workspace(artifact.scope_class);
        let chip_label = build_chip_label(scope_class, Some(&artifact.workset_name));
        let chip_record = artifact.project_chip(
            "search_scope_chip",
            aureline_workspace::ChipSurfaceClass::SearchResultGroupHeader,
            "mono:scope".to_string(),
        );
        let presentation_state =
            ScopePresentationState::from_workspace(chip_record.chip_presentation_state);
        let patterns = artifact
            .patterns
            .iter()
            .map(ScopePatternRecord::from_workspace)
            .collect();
        let root_refs = artifact.root_refs.clone();
        let is_policy_limited =
            matches!(artifact.scope_class, WorkspaceScopeClass::PolicyLimitedView);
        let workset_name = if matches!(
            artifact.scope_class,
            WorkspaceScopeClass::CurrentRepo | WorkspaceScopeClass::FullWorkspace
        ) {
            None
        } else {
            Some(artifact.workset_name.clone())
        };
        Self {
            workspace_id,
            scope_class,
            workset_id: Some(artifact.workset_id.clone()),
            workset_name,
            chip_label,
            presentation_state,
            root_refs,
            patterns,
            partial_index_note: artifact.readiness.partial_index_note.clone(),
            is_policy_limited,
            has_partial_member_truth: artifact.has_partial_member_truth(),
        }
    }

    /// Workspace identity bound to this scope.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Active scope class.
    pub const fn scope_class(&self) -> ScopeClass {
        self.scope_class
    }

    /// Workset id when a workset narrows scope.
    pub fn workset_id(&self) -> Option<&str> {
        self.workset_id.as_deref()
    }

    /// Workset name when a workset narrows scope.
    pub fn workset_name(&self) -> Option<&str> {
        self.workset_name.as_deref()
    }

    /// Resolved chip label (e.g. `Selected workset · Hot path`).
    pub fn chip_label(&self) -> &str {
        &self.chip_label
    }

    /// Active chip presentation state.
    pub const fn presentation_state(&self) -> ScopePresentationState {
        self.presentation_state
    }

    /// Root references the active scope spans.
    pub fn root_refs(&self) -> &[String] {
        &self.root_refs
    }

    /// Active include / exclude pattern set.
    pub fn patterns(&self) -> &[ScopePatternRecord] {
        &self.patterns
    }

    /// Optional partial-index note attached to the active scope.
    pub fn partial_index_note(&self) -> Option<&str> {
        self.partial_index_note.as_deref()
    }

    /// True when the active scope hides files from the search-shell view.
    pub fn is_narrowed(&self) -> bool {
        self.scope_class.is_narrowed()
    }

    /// True when the active scope is a policy-limited view.
    pub const fn is_policy_limited(&self) -> bool {
        self.is_policy_limited
    }

    /// True when the active scope reads as partial (partial chip state, a
    /// recorded partial-index note, partial member truth, or policy
    /// limitation). Surfaces use this flag to decide whether the chrome
    /// MUST surface a partiality cue alongside the chip.
    pub fn is_partial_scope(&self) -> bool {
        self.is_narrowed()
            || self.presentation_state.is_partial()
            || self.partial_index_note.is_some()
            || self.has_partial_member_truth
            || self.is_policy_limited
    }

    /// True when the path is in the active scope under the active include /
    /// exclude pattern set.
    pub fn matches_relative_path(&self, relative_path: &str) -> bool {
        glob_matches_relative_path(relative_path, &self.patterns)
    }

    /// Partition a workspace-relative file list into in-scope and
    /// out-of-scope buckets. The `all_workspace_count` field is the total
    /// pre-filter count surfaces use to disclose
    /// `all_matching_in_workspace`.
    pub fn filter_files<I, S>(&self, files: I) -> ScopeFilterOutcome
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut in_scope: Vec<String> = Vec::new();
        let mut out_of_scope: Vec<String> = Vec::new();
        for file in files {
            let file = file.into();
            if self.matches_relative_path(&file) {
                in_scope.push(file);
            } else {
                out_of_scope.push(file);
            }
        }
        let all_workspace_count = (in_scope.len() + out_of_scope.len()) as u64;
        let in_scope_count = in_scope.len() as u64;
        ScopeFilterOutcome {
            in_scope,
            out_of_scope,
            all_workspace_count,
            in_scope_count,
        }
    }

    /// Project a serializable metadata record. Surfaces attach this to
    /// snapshots so an exported session can be replayed without losing the
    /// scope label, partiality flag, or pattern fingerprint.
    pub fn project_metadata(&self) -> WorkspaceSearchScopeMetadata {
        let include_pattern_count = self
            .patterns
            .iter()
            .filter(|p| p.kind == ScopePatternKind::Include)
            .count() as u32;
        let exclude_pattern_count = self
            .patterns
            .iter()
            .filter(|p| p.kind == ScopePatternKind::Exclude)
            .count() as u32;
        WorkspaceSearchScopeMetadata {
            workspace_id: self.workspace_id.clone(),
            scope_class_token: self.scope_class.as_str().to_string(),
            workset_id: self.workset_id.clone(),
            workset_name: self.workset_name.clone(),
            chip_label: self.chip_label.clone(),
            presentation_state_token: self.presentation_state.as_str().to_string(),
            root_refs: self.root_refs.clone(),
            partial_scope: self.is_partial_scope(),
            partial_index_note: self.partial_index_note.clone(),
            is_policy_limited: self.is_policy_limited,
            include_pattern_count,
            exclude_pattern_count,
            patterns: self.patterns.clone(),
        }
    }
}

/// Serializable scope metadata projection used in support exports and
/// fixtures. Surfaces quote `chip_label`, `presentation_state_token`, and
/// `partial_scope` directly when they have to render a scope footer next to
/// a captured row set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSearchScopeMetadata {
    pub workspace_id: String,
    pub scope_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_name: Option<String>,
    pub chip_label: String,
    pub presentation_state_token: String,
    pub root_refs: Vec<String>,
    pub partial_scope: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_note: Option<String>,
    pub is_policy_limited: bool,
    pub include_pattern_count: u32,
    pub exclude_pattern_count: u32,
    pub patterns: Vec<ScopePatternRecord>,
}

fn build_chip_label(scope: ScopeClass, workset_name: Option<&str>) -> String {
    match scope {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
            scope.chip_label_family().to_string()
        }
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            match workset_name {
                Some(name) if !name.trim().is_empty() => {
                    format!("{} · {}", scope.chip_label_family(), name)
                }
                _ => scope.chip_label_family().to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{
        MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause, PartialTruthLabel,
        PatternEntry, PatternKind, PolicyLimitation, PortabilityMetadata, ReadinessMetadata,
        ReadinessState, SourceClass as WksSourceClass, WorksetArtifactRecord,
        WorksetArtifactRecordKind, WorksetPortabilityClass,
    };

    fn fixture_artifact(scope: WorkspaceScopeClass) -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:test:hot:0".to_string(),
            workset_name: "Hot path".to_string(),
            presentation_subtitle: None,
            scope_class: scope,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            patterns: vec![
                PatternEntry {
                    pattern_kind: PatternKind::Include,
                    pattern: "apps/web/**".to_string(),
                    applies_to_root_ref: None,
                },
                PatternEntry {
                    pattern_kind: PatternKind::Exclude,
                    pattern: "apps/web/public/vendor/**".to_string(),
                    applies_to_root_ref: None,
                },
            ],
            membership_policy: MembershipPolicy::GlobPattern,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: if scope == WorkspaceScopeClass::PolicyLimitedView {
                Some(PolicyLimitation {
                    underlying_workset_ref: "wks:test:hot:0:underlying".to_string(),
                    policy_ref: "policy:test:admin".to_string(),
                    narrowing_cause: NarrowingCause::AdminPolicy,
                    visible_member_count: 1,
                    hidden_member_count: 1,
                    hidden_member_list_visible: false,
                })
            } else {
                None
            },
            portability: PortabilityMetadata {
                source_class: WksSourceClass::WorkspaceShared,
                portability_class: WorksetPortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(0),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    #[test]
    fn full_workspace_default_is_authoritative_and_keeps_every_path() {
        let scope = WorkspaceSearchScope::for_full_workspace("ws-test");
        assert_eq!(scope.scope_class(), ScopeClass::FullWorkspace);
        assert_eq!(scope.chip_label(), "Full workspace");
        assert!(!scope.is_partial_scope());
        let outcome = scope.filter_files(["src/main.rs", "tests/smoke.rs"]);
        assert_eq!(outcome.in_scope_count, 2);
        assert_eq!(outcome.all_workspace_count, 2);
        assert!(outcome.out_of_scope.is_empty());
    }

    #[test]
    fn current_repo_default_is_narrowed_and_partial() {
        let scope = WorkspaceSearchScope::for_current_repo("ws-test");
        assert_eq!(scope.chip_label(), "Current repo");
        assert!(scope.is_narrowed());
        assert!(scope.is_partial_scope());
    }

    #[test]
    fn selected_workset_chip_label_carries_workset_name() {
        let artifact = fixture_artifact(WorkspaceScopeClass::SelectedWorkset);
        let scope = WorkspaceSearchScope::from_workset_artifact("ws-test", &artifact);
        assert_eq!(scope.chip_label(), "Selected workset · Hot path");
        assert_eq!(scope.workset_id(), Some("wks:test:hot:0"));
        assert_eq!(scope.workset_name(), Some("Hot path"));
        assert!(scope.is_partial_scope());
    }

    #[test]
    fn workset_filter_drops_paths_outside_include_pattern() {
        let artifact = fixture_artifact(WorkspaceScopeClass::SelectedWorkset);
        let scope = WorkspaceSearchScope::from_workset_artifact("ws-test", &artifact);
        let outcome = scope.filter_files([
            "apps/web/main.tsx",
            "apps/api/handler.rs",
            "apps/web/public/vendor/jquery.js",
        ]);
        assert!(outcome.in_scope.contains(&"apps/web/main.tsx".to_string()));
        assert!(outcome
            .out_of_scope
            .contains(&"apps/api/handler.rs".to_string()));
        assert!(outcome
            .out_of_scope
            .contains(&"apps/web/public/vendor/jquery.js".to_string()));
        assert_eq!(outcome.in_scope_count, 1);
        assert_eq!(outcome.all_workspace_count, 3);
        assert!(outcome.is_narrowed());
    }

    #[test]
    fn policy_limited_scope_is_partial_and_locked() {
        let artifact = fixture_artifact(WorkspaceScopeClass::PolicyLimitedView);
        let scope = WorkspaceSearchScope::from_workset_artifact("ws-test", &artifact);
        assert!(scope.is_policy_limited());
        assert_eq!(
            scope.presentation_state(),
            ScopePresentationState::ActivePolicyLimited
        );
        assert!(scope.is_partial_scope());
    }

    #[test]
    fn metadata_export_records_pattern_counts_and_chip_label() {
        let artifact = fixture_artifact(WorkspaceScopeClass::SparseSlice);
        let scope = WorkspaceSearchScope::from_workset_artifact("ws-test", &artifact);
        let metadata = scope.project_metadata();
        assert_eq!(metadata.scope_class_token, "sparse_slice");
        assert_eq!(metadata.chip_label, "Sparse slice · Hot path");
        assert!(metadata.partial_scope);
        assert_eq!(metadata.include_pattern_count, 1);
        assert_eq!(metadata.exclude_pattern_count, 1);
        assert_eq!(metadata.root_refs, vec!["fs-r-0".to_string()]);
    }

    #[test]
    fn metadata_round_trips_through_serde() {
        let artifact = fixture_artifact(WorkspaceScopeClass::SelectedWorkset);
        let scope = WorkspaceSearchScope::from_workset_artifact("ws-test", &artifact);
        let metadata = scope.project_metadata();
        let json = serde_json::to_string(&metadata).expect("metadata must serialize");
        let parsed: WorkspaceSearchScopeMetadata =
            serde_json::from_str(&json).expect("metadata must round-trip");
        assert_eq!(parsed, metadata);
    }
}
