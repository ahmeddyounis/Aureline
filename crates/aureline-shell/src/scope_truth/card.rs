//! Shell-facing scope-truth chip card projection.
//!
//! The card is the serializable record the chrome consumes when it renders
//! a scope-truth chip on an open or search surface. It reuses the
//! canonical [`aureline_workspace::ScopeClass`] vocabulary, the canonical
//! [`aureline_workspace::WorksetArtifactRecord::project_chip`] projection,
//! and the [`super::counts::ScopeCountsRecord`] disclosure block; it never
//! mints surface-only chip vocabulary.

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    ChipAction, ChipPresentationState, ChipSurfaceClass, HiddenResultCountClass,
    ScopeClass as WorkspaceScopeClass, ScopeTruthChipRecord, WorksetArtifactRecord,
};

use super::counts::ScopeCountsRecord;

/// Stable record-kind tag carried in serialized scope-truth chip cards.
pub const SCOPE_TRUTH_CHIP_RECORD_KIND: &str = "scope_truth_chip_card";
/// Schema version for the [`ScopeTruthChipCard`] payload shape.
pub const SCOPE_TRUTH_CHIP_SCHEMA_VERSION: u32 = 1;

/// Shell surface emitting a scope-truth chip card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeTruthSurfaceClass {
    Explorer,
    QuickOpen,
    SearchShell,
    DocsBrowser,
    OpenFlowSheet,
    SupportPacket,
}

impl ScopeTruthSurfaceClass {
    /// Stable token used in records, fixtures, and shell snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explorer => "explorer",
            Self::QuickOpen => "quick_open",
            Self::SearchShell => "search_shell",
            Self::DocsBrowser => "docs_browser",
            Self::OpenFlowSheet => "open_flow_sheet",
            Self::SupportPacket => "support_packet",
        }
    }

    /// Resolve to the canonical workspace [`ChipSurfaceClass`] used when
    /// the scope-truth chip needs to project through the workspace
    /// [`WorksetArtifactRecord::project_chip`] entry point.
    pub const fn to_workspace_chip_surface_class(self) -> ChipSurfaceClass {
        match self {
            Self::Explorer => ChipSurfaceClass::ScopeBanner,
            Self::QuickOpen => ChipSurfaceClass::ScopeBanner,
            Self::SearchShell => ChipSurfaceClass::SearchResultGroupHeader,
            Self::DocsBrowser => ChipSurfaceClass::ScopeBanner,
            Self::OpenFlowSheet => ChipSurfaceClass::OpenFlowTrustCard,
            Self::SupportPacket => ChipSurfaceClass::SupportPacketHeader,
        }
    }
}

/// Shell-facing scope-truth chip card.
///
/// Surfaces render this struct directly. They MUST quote the labels and
/// tokens verbatim — the chrome does not re-derive scope, presentation,
/// or count vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeTruthChipCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub surface_class_token: String,
    pub scope_class_token: String,
    pub chip_label: String,
    pub presentation_state_token: String,
    /// Workset id when a workset artifact narrows scope; `None` when the
    /// scope is the bare `current_repo` or `full_workspace` family without
    /// a workset artifact in play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    /// Workset name (e.g. "Hot path") when a workset artifact narrows
    /// scope. The chrome quotes this verbatim — no truncation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_name: Option<String>,
    /// Number of root memberships in the active scope. `None` when the
    /// scope is computed without a workset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_count: Option<u32>,
    /// Number of declared member refs in the active scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,
    /// True when the active scope is narrower than the workspace OR the
    /// active scope's readiness is below ready. The chrome SHOULD render a
    /// `Partial` cue alongside this chip when the flag is true.
    pub partial_scope: bool,
    /// Note from the workset artifact's readiness metadata that explains
    /// the partial index (e.g. "Backend folders are excluded").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_note: Option<String>,
    /// Hidden-result count vocabulary (`partial_index`, `policy_hidden`,
    /// `outside_scope_roots`, ...). Surfaces use this to label the count
    /// rather than minting their own taxonomy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_result_count_class: Option<String>,
    /// Hidden-result count value when the workset artifact knows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_result_count: Option<u64>,
    /// Reserved for cross-repo / outside-scope row markers. The active
    /// scope chip never sets this true.
    pub outside_current_scope_marker_visible: bool,
    /// Typed actions the chip MUST offer. The chrome quotes these tokens
    /// directly when wiring chip menus and keyboard accelerators.
    pub offered_action_tokens: Vec<String>,
    /// Visible / loaded / all-matching count disclosure.
    pub counts: ScopeCountsRecord,
    pub emitted_at: String,
}

/// Project a scope-truth chip card from a workspace [`aureline_runtime::ScopeClass`] alone.
///
/// Use this when the active scope is the bare `current_repo` or
/// `full_workspace` family with no workset artifact in play, and when
/// `workset_name` is supplied for narrowed scopes that lack a full
/// artifact (e.g., a workset stub).
pub fn project_scope_truth_chip_card(
    workspace_id: impl Into<String>,
    surface_class: ScopeTruthSurfaceClass,
    scope_class: WorkspaceScopeClass,
    workset_name: Option<&str>,
    counts: ScopeCountsRecord,
    emitted_at: impl Into<String>,
) -> ScopeTruthChipCard {
    let chip_label = build_chip_label(scope_class, workset_name);
    let presentation_state = derive_simple_presentation_state(scope_class, &counts);
    let offered_actions = simple_offered_actions(scope_class);
    let partial_scope = scope_class.is_narrowed() || !counts.readiness_is_ready;

    ScopeTruthChipCard {
        record_kind: SCOPE_TRUTH_CHIP_RECORD_KIND.to_string(),
        schema_version: SCOPE_TRUTH_CHIP_SCHEMA_VERSION,
        workspace_id: workspace_id.into(),
        surface_class_token: surface_class.as_str().to_string(),
        scope_class_token: scope_class.as_str().to_string(),
        chip_label,
        presentation_state_token: presentation_state.as_token().to_string(),
        workset_id: None,
        workset_name: workset_name.map(|s| s.to_string()),
        root_count: None,
        member_count: None,
        partial_scope,
        partial_index_note: None,
        hidden_result_count_class: None,
        hidden_result_count: None,
        outside_current_scope_marker_visible: false,
        offered_action_tokens: offered_actions
            .into_iter()
            .map(|a| chip_action_token(a).to_string())
            .collect(),
        counts,
        emitted_at: emitted_at.into(),
    }
}

/// Project a scope-truth chip card from a full
/// [`WorksetArtifactRecord`].
///
/// The artifact projection wins on every field where it disagrees with a
/// caller-supplied default — surfaces never re-mint chip labels, action
/// lists, or hidden-result classes.
pub fn project_scope_truth_chip_card_for_artifact(
    workspace_id: impl Into<String>,
    surface_class: ScopeTruthSurfaceClass,
    artifact: &WorksetArtifactRecord,
    counts: ScopeCountsRecord,
    chip_id: impl Into<String>,
    emitted_at: impl Into<String>,
) -> ScopeTruthChipCard {
    let emitted = emitted_at.into();
    let chip_record = artifact.project_chip(
        chip_id,
        surface_class.to_workspace_chip_surface_class(),
        emitted.clone(),
    );
    chip_card_from_record(
        workspace_id.into(),
        surface_class,
        artifact,
        &chip_record,
        counts,
        emitted,
    )
}

/// Project an outside-current-scope chip card for a search/result row
/// whose owning root is not in the active workset's `root_refs`.
pub fn project_outside_scope_truth_chip_card(
    workspace_id: impl Into<String>,
    surface_class: ScopeTruthSurfaceClass,
    artifact: &WorksetArtifactRecord,
    counts: ScopeCountsRecord,
    chip_id: impl Into<String>,
    emitted_at: impl Into<String>,
) -> ScopeTruthChipCard {
    let emitted = emitted_at.into();
    let chip_record = artifact.project_outside_scope_chip(
        chip_id,
        surface_class.to_workspace_chip_surface_class(),
        emitted.clone(),
    );
    let mut card = chip_card_from_record(
        workspace_id.into(),
        surface_class,
        artifact,
        &chip_record,
        counts,
        emitted,
    );
    // Force the partial-scope flag on outside-scope rows so the chrome
    // never reads them as authoritative against the active scope.
    card.partial_scope = true;
    card
}

/// Render the chip card as a small set of human-readable lines for log
/// surfaces, support exports, and review sheets.
pub fn render_scope_truth_chip_lines(card: &ScopeTruthChipCard) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[{state}] {label}",
        state = card.presentation_state_token,
        label = card.chip_label,
    ));
    let scope_summary = match (card.root_count, card.member_count) {
        (Some(roots), Some(members)) if roots != members => {
            format!(
                "scope: {scope}   roots: {roots}   members: {members}",
                scope = card.scope_class_token,
                roots = roots,
                members = members,
            )
        }
        (Some(roots), _) => format!(
            "scope: {scope}   roots: {roots}",
            scope = card.scope_class_token,
            roots = roots,
        ),
        _ => format!("scope: {}", card.scope_class_token),
    };
    lines.push(scope_summary);
    let mut counts_line = format!("counts: visible={}", card.counts.visible_in_view);
    counts_line.push_str(&format!(
        "  loaded_in_scope={}",
        card.counts
            .loaded_in_scope
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string()),
    ));
    counts_line.push_str(&format!(
        "  all_matching={}",
        card.counts
            .all_matching_in_workspace
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string()),
    ));
    counts_line.push_str(&format!("  class={}", card.counts.counts_class_token));
    lines.push(counts_line);
    if let Some(note) = card.partial_index_note.as_deref() {
        lines.push(format!("note: {note}"));
    }
    if let Some(class) = card.hidden_result_count_class.as_deref() {
        let value = card
            .hidden_result_count
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string());
        lines.push(format!("hidden: {value} ({class})"));
    }
    if !card.offered_action_tokens.is_empty() {
        lines.push(format!(
            "actions: {}",
            card.offered_action_tokens.join(", ")
        ));
    }
    if card.outside_current_scope_marker_visible {
        lines.push("marker: outside_current_scope".to_string());
    }
    lines
}

fn chip_card_from_record(
    workspace_id: String,
    surface_class: ScopeTruthSurfaceClass,
    artifact: &WorksetArtifactRecord,
    chip_record: &ScopeTruthChipRecord,
    counts: ScopeCountsRecord,
    emitted_at: String,
) -> ScopeTruthChipCard {
    let partial_scope = artifact.is_narrowed_scope()
        || artifact.has_partial_member_truth()
        || !counts.readiness_is_ready
        || chip_record.chip_presentation_state == ChipPresentationState::ActivePartial
        || chip_record.chip_presentation_state == ChipPresentationState::ActivePolicyLimited;
    let workset_name = if matches!(
        artifact.scope_class,
        WorkspaceScopeClass::CurrentRepo | WorkspaceScopeClass::FullWorkspace
    ) {
        None
    } else {
        Some(artifact.workset_name.clone())
    };
    let partial_index_note = artifact.readiness.partial_index_note.clone();
    let hidden_summary = chip_record.hidden_result_summary.as_ref();
    // Surfaces only render a hidden-result class when something is actually
    // hidden. The canonical `NoneKnown` value MUST collapse back to `None`
    // here so the chrome doesn't render a "0 (none_known)" badge on a
    // ready, full-coverage chip.
    let (hidden_class, hidden_count) = match hidden_summary {
        Some(summary) if summary.count_class != HiddenResultCountClass::NoneKnown => (
            Some(hidden_count_class_token(summary.count_class).to_string()),
            summary.count,
        ),
        _ => (None, None),
    };

    let resolved_hidden_class = hidden_class.or_else(|| {
        // Even without a hidden_summary we surface the partial-index class
        // when the artifact carries a partial-index note, so the chrome
        // can label "why" the count is reduced.
        partial_index_class_for(artifact)
    });
    let resolved_hidden_count = match (resolved_hidden_class.is_some(), hidden_count) {
        // Surfaces only quote a hidden count when there is also a hidden
        // class to label it; otherwise a `0` count would be ambiguous next
        // to "Outside current scope" or a fully ready full-workspace chip.
        (true, Some(count)) if count > 0 => Some(count),
        (true, None) => artifact
            .readiness
            .hidden_result_count
            .filter(|count| *count > 0),
        _ => None,
    };

    ScopeTruthChipCard {
        record_kind: SCOPE_TRUTH_CHIP_RECORD_KIND.to_string(),
        schema_version: SCOPE_TRUTH_CHIP_SCHEMA_VERSION,
        workspace_id,
        surface_class_token: surface_class.as_str().to_string(),
        scope_class_token: artifact.scope_class.as_str().to_string(),
        chip_label: chip_record.chip_label.clone(),
        presentation_state_token: presentation_state_token(chip_record.chip_presentation_state)
            .to_string(),
        workset_id: Some(artifact.workset_id.clone()),
        workset_name,
        root_count: chip_record.root_count,
        member_count: chip_record.member_count,
        partial_scope,
        partial_index_note,
        hidden_result_count_class: resolved_hidden_class,
        hidden_result_count: resolved_hidden_count,
        outside_current_scope_marker_visible: chip_record.outside_current_scope_marker_visible,
        offered_action_tokens: chip_record
            .offered_actions
            .iter()
            .copied()
            .map(|a| chip_action_token(a).to_string())
            .collect(),
        counts,
        emitted_at,
    }
}

fn partial_index_class_for(artifact: &WorksetArtifactRecord) -> Option<String> {
    let has_real_hiding = artifact.readiness.partial_index_note.is_some()
        || artifact
            .readiness
            .hidden_result_count
            .is_some_and(|count| count > 0);
    if !has_real_hiding {
        return None;
    }
    Some(
        hidden_count_class_token(match artifact.scope_class {
            WorkspaceScopeClass::SparseSlice => HiddenResultCountClass::PartialIndex,
            WorkspaceScopeClass::SelectedWorkset => HiddenResultCountClass::OutsideScopeRoots,
            WorkspaceScopeClass::PolicyLimitedView => HiddenResultCountClass::PolicyHidden,
            _ => HiddenResultCountClass::WarmingIndex,
        })
        .to_string(),
    )
}

fn build_chip_label(scope: WorkspaceScopeClass, workset_name: Option<&str>) -> String {
    match scope {
        WorkspaceScopeClass::CurrentRepo | WorkspaceScopeClass::FullWorkspace => {
            scope.chip_label_family().to_string()
        }
        WorkspaceScopeClass::SelectedWorkset
        | WorkspaceScopeClass::SparseSlice
        | WorkspaceScopeClass::PolicyLimitedView => match workset_name {
            Some(name) if !name.trim().is_empty() => {
                format!("{} · {}", scope.chip_label_family(), name)
            }
            _ => scope.chip_label_family().to_string(),
        },
    }
}

fn derive_simple_presentation_state(
    scope: WorkspaceScopeClass,
    counts: &ScopeCountsRecord,
) -> ChipPresentationState {
    if scope == WorkspaceScopeClass::PolicyLimitedView {
        return ChipPresentationState::ActivePolicyLimited;
    }
    if !counts.readiness_is_ready {
        return ChipPresentationState::ActivePartial;
    }
    ChipPresentationState::ActiveNarrowSafe
}

fn simple_offered_actions(scope: WorkspaceScopeClass) -> Vec<ChipAction> {
    match scope {
        WorkspaceScopeClass::CurrentRepo => vec![ChipAction::WidenToFullWorkspace],
        WorkspaceScopeClass::FullWorkspace => vec![ChipAction::NarrowToCurrentRepo],
        WorkspaceScopeClass::SelectedWorkset | WorkspaceScopeClass::SparseSlice => vec![
            ChipAction::WidenWithReview,
            ChipAction::WidenToFullWorkspace,
            ChipAction::OpenScopeDiff,
        ],
        WorkspaceScopeClass::PolicyLimitedView => vec![ChipAction::KeepCurrentScope],
    }
}

trait PresentationStateAsToken {
    fn as_token(self) -> &'static str;
}

impl PresentationStateAsToken for ChipPresentationState {
    fn as_token(self) -> &'static str {
        presentation_state_token(self)
    }
}

const fn presentation_state_token(state: ChipPresentationState) -> &'static str {
    match state {
        ChipPresentationState::ActiveNarrowSafe => "active_narrow_safe",
        ChipPresentationState::ActivePartial => "active_partial",
        ChipPresentationState::ActivePolicyLimited => "active_policy_limited",
        ChipPresentationState::ActiveWidened => "active_widened",
        ChipPresentationState::OutsideCurrentScope => "outside_current_scope",
    }
}

const fn chip_action_token(action: ChipAction) -> &'static str {
    match action {
        ChipAction::WidenToFullWorkspace => "widen_to_full_workspace",
        ChipAction::WidenWithReview => "widen_with_review",
        ChipAction::NarrowToCurrentRepo => "narrow_to_current_repo",
        ChipAction::OpenScopeDiff => "open_scope_diff",
        ChipAction::BuildMissingIndexes => "build_missing_indexes",
        ChipAction::KeepCurrentScope => "keep_current_scope",
        ChipAction::RevealHiddenResultsPolicyAdminOnly => "reveal_hidden_results_policy_admin_only",
        ChipAction::OpenInNewPane => "open_in_new_pane",
        ChipAction::CopyWorksetId => "copy_workset_id",
        ChipAction::ExportWorksetArtifact => "export_workset_artifact",
    }
}

const fn hidden_count_class_token(class: HiddenResultCountClass) -> &'static str {
    match class {
        HiddenResultCountClass::NoneKnown => "none_known",
        HiddenResultCountClass::PartialIndex => "partial_index",
        HiddenResultCountClass::OutsideScopeRoots => "outside_scope_roots",
        HiddenResultCountClass::PolicyHidden => "policy_hidden",
        HiddenResultCountClass::WarmingIndex => "warming_index",
        HiddenResultCountClass::RemoteUnreachable => "remote_unreachable",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scope_truth::counts::ScopeCountsInputs;
    use aureline_workspace::{
        MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause, PartialTruthLabel,
        PatternEntry, PatternKind, PolicyLimitation, PortabilityMetadata, ReadinessMetadata,
        ReadinessState, SourceClass, WorksetArtifactRecordKind,
        WorksetPortabilityClass as WksPortabilityClass,
    };

    fn ready_globally_authoritative_counts() -> ScopeCountsRecord {
        ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 4,
            loaded_in_scope: Some(4),
            all_matching_in_workspace: Some(4),
            scope_covers_workspace: true,
            readiness_is_ready: true,
        })
    }

    fn narrowed_partial_counts() -> ScopeCountsRecord {
        ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 8,
            loaded_in_scope: Some(8),
            all_matching_in_workspace: Some(45),
            scope_covers_workspace: false,
            readiness_is_ready: true,
        })
    }

    fn warming_counts() -> ScopeCountsRecord {
        ScopeCountsRecord::derive(ScopeCountsInputs {
            visible_in_view: 0,
            loaded_in_scope: Some(0),
            all_matching_in_workspace: None,
            scope_covers_workspace: true,
            readiness_is_ready: false,
        })
    }

    fn full_workspace_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:full".to_string(),
            workset_name: "Full payments workspace".to_string(),
            presentation_subtitle: None,
            scope_class: WorkspaceScopeClass::FullWorkspace,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string(), "fs-r-1".to_string()],
            patterns: Vec::new(),
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-0".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-a".to_string()),
                },
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-1".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-b".to_string()),
                },
            ],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::WorkspaceShared,
                portability_class: WksPortabilityClass::PortableWithRebinding,
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
            updated_at: "mono:0".to_string(),
            notes: None,
        }
    }

    fn sparse_slice_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:sparse".to_string(),
            workset_name: "Frontend slice".to_string(),
            presentation_subtitle: Some("Sparse slice".to_string()),
            scope_class: WorkspaceScopeClass::SparseSlice,
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
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::LocalOnly,
                portability_class: WksPortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: true,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Partial,
                hidden_result_count_known: true,
                hidden_result_count: Some(17_412),
                partial_index_note: Some("Backend folders excluded.".to_string()),
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    fn policy_limited_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:policy".to_string(),
            workset_name: "Restricted view".to_string(),
            presentation_subtitle: None,
            scope_class: WorkspaceScopeClass::PolicyLimitedView,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            patterns: Vec::new(),
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: Some(PolicyLimitation {
                underlying_workset_ref: "wks:policy:underlying".to_string(),
                policy_ref: "policy:test:admin".to_string(),
                narrowing_cause: NarrowingCause::AdminPolicy,
                visible_member_count: 1,
                hidden_member_count: 1,
                hidden_member_list_visible: false,
            }),
            portability: PortabilityMetadata {
                source_class: SourceClass::Managed,
                portability_class: WksPortabilityClass::ManagedProviderLocked,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: true,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(1),
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
    fn full_workspace_card_is_globally_authoritative() {
        let card = project_scope_truth_chip_card_for_artifact(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &full_workspace_artifact(),
            ready_globally_authoritative_counts(),
            "chip:full",
            "mono:1",
        );
        assert_eq!(card.scope_class_token, "full_workspace");
        assert_eq!(card.chip_label, "Full workspace");
        assert!(!card.partial_scope);
        assert_eq!(card.counts.counts_class_token, "globally_authoritative");
        assert!(card
            .offered_action_tokens
            .iter()
            .any(|t| t == "narrow_to_current_repo"));
    }

    #[test]
    fn sparse_slice_card_disclosed_partial_scope_with_hidden_class() {
        let card = project_scope_truth_chip_card_for_artifact(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &sparse_slice_artifact(),
            narrowed_partial_counts(),
            "chip:sparse",
            "mono:1",
        );
        assert_eq!(card.scope_class_token, "sparse_slice");
        assert!(card.chip_label.starts_with("Sparse slice · "));
        assert!(card.partial_scope);
        assert_eq!(card.presentation_state_token, "active_partial");
        assert_eq!(
            card.hidden_result_count_class.as_deref(),
            Some("partial_index")
        );
        assert_eq!(card.hidden_result_count, Some(17_412));
        assert_eq!(
            card.partial_index_note.as_deref(),
            Some("Backend folders excluded.")
        );
        assert_eq!(card.counts.counts_class_token, "partial_truth");
        assert!(card
            .offered_action_tokens
            .iter()
            .any(|t| t == "open_scope_diff"));
    }

    #[test]
    fn policy_limited_card_carries_admin_policy_state_and_class() {
        let card = project_scope_truth_chip_card_for_artifact(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &policy_limited_artifact(),
            ScopeCountsRecord::derive(ScopeCountsInputs {
                visible_in_view: 3,
                loaded_in_scope: Some(3),
                all_matching_in_workspace: None,
                scope_covers_workspace: false,
                readiness_is_ready: true,
            }),
            "chip:policy",
            "mono:1",
        );
        assert_eq!(card.presentation_state_token, "active_policy_limited");
        assert_eq!(card.scope_class_token, "policy_limited_view");
        assert_eq!(
            card.hidden_result_count_class.as_deref(),
            Some("policy_hidden")
        );
        assert!(card.partial_scope);
        assert!(card
            .offered_action_tokens
            .iter()
            .any(|t| t == "keep_current_scope"));
        // Managed-provider-locked artifacts MUST NOT offer export.
        assert!(!card
            .offered_action_tokens
            .iter()
            .any(|t| t == "export_workset_artifact"));
    }

    #[test]
    fn outside_scope_card_marks_outside_marker_visible_and_partial() {
        let card = project_outside_scope_truth_chip_card(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &sparse_slice_artifact(),
            ScopeCountsRecord::not_computed(false, true),
            "chip:outside",
            "mono:1",
        );
        assert!(card.outside_current_scope_marker_visible);
        assert_eq!(card.presentation_state_token, "outside_current_scope");
        assert_eq!(card.chip_label, "Outside current scope");
        assert!(card.partial_scope);
    }

    #[test]
    fn bare_scope_class_card_uses_workset_name_when_provided() {
        let card = project_scope_truth_chip_card(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            WorkspaceScopeClass::SelectedWorkset,
            Some("Hot path"),
            ScopeCountsRecord::not_computed(false, true),
            "mono:1",
        );
        assert_eq!(card.chip_label, "Selected workset · Hot path");
        assert!(card.partial_scope);
        assert_eq!(card.workset_name.as_deref(), Some("Hot path"));
        assert!(card.workset_id.is_none());
    }

    #[test]
    fn bare_full_workspace_with_warming_readiness_marks_partial() {
        let card = project_scope_truth_chip_card(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            WorkspaceScopeClass::FullWorkspace,
            None,
            warming_counts(),
            "mono:1",
        );
        assert_eq!(card.chip_label, "Full workspace");
        assert!(card.partial_scope);
        assert_eq!(card.presentation_state_token, "active_partial");
    }

    #[test]
    fn render_lines_disclose_visible_loaded_and_all_matching_counts() {
        let card = project_scope_truth_chip_card_for_artifact(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &sparse_slice_artifact(),
            narrowed_partial_counts(),
            "chip:sparse",
            "mono:1",
        );
        let lines = render_scope_truth_chip_lines(&card);
        let counts_line = lines
            .iter()
            .find(|line| line.starts_with("counts:"))
            .expect("counts line must render");
        assert!(counts_line.contains("visible=8"));
        assert!(counts_line.contains("loaded_in_scope=8"));
        assert!(counts_line.contains("all_matching=45"));
        assert!(counts_line.contains("class=partial_truth"));
    }

    #[test]
    fn card_round_trips_through_serde() {
        let card = project_scope_truth_chip_card_for_artifact(
            "ws-test",
            ScopeTruthSurfaceClass::SearchShell,
            &sparse_slice_artifact(),
            narrowed_partial_counts(),
            "chip:sparse",
            "mono:1",
        );
        let json = serde_json::to_string(&card).expect("card must serialize");
        let parsed: ScopeTruthChipCard = serde_json::from_str(&json).expect("card must round-trip");
        assert_eq!(parsed, card);
    }
}
