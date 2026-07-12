//! Implemented M5 tree-view and list-view primitives.
//!
//! The frozen [navigation / content component matrix][matrix] names the reusable navigation and
//! dense-content UI components and locks their controlled vocabulary. This module is the
//! dense-collection implement lane over that matrix: it turns the two hierarchical / queue-like
//! collection components — the **tree view** and the **list view** — into resolvers that produce
//! export-safe, honest projections, so a user can read a collection's disclosure, selection-versus-
//! current, per-row item state, count scope, density, and local-action budget without the component
//! quietly faking a complete tree, collapsing count scopes, or hiding critical state behind
//! pointer-hover affordances.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render tree and list contracts with virtualization, keyboard-complete disclosure, a
//!   current-versus-selected distinction, and a capped inline-action budget that never hides critical
//!   state behind hover-only affordances.** [`resolve_tree_view`] and [`resolve_list_view`] refuse to
//!   read as a clean, structure-legible collection when the identity is unstated, disclosure is
//!   unresolved, a lazily-unloaded subtree is drawn as an empty leaf, selection is collapsed into the
//!   current item, row focus is not visible, or the current selection, a blocked row, or the local
//!   actions can only be discovered by pointer hover; they degrade instead.
//! * **Expose exact, loaded, hidden-by-filter, hidden-by-policy, and outside-current-scope counts
//!   wherever the row or pane meaning depends on them.** Both resolvers degrade when the exact,
//!   loaded, and all-matching scopes collapse into one vague total, when a loaded subset is presented
//!   as the exact total, or when the count scope cannot be resolved.
//! * **Keep row focus, drag / reorder posture where allowed, and cross-window or cross-pane
//!   continuity honest instead of implying capabilities the current profile does not support.** Both
//!   resolvers degrade when drag / reorder or cross-surface continuity is overclaimed, and never
//!   present a stale or partial backend as a complete collection.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5DisclosureState`] disclosure
//! vocabulary, the [`M5SelectionState`] selection vocabulary, the [`M5ItemStateFlag`] item-state
//! vocabulary, the [`M5DensityVariant`] density vocabulary, and the [`M5LocalActionBudget`]
//! local-action-budget vocabulary — so explorer, search, review-queue, provider, help, and support
//! surfaces can never fork their own disclosure, selection, count, or item-state wording. Raw secret
//! values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_tree_view_list_view_controls,
    seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed,
    seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed,
    M5_TREE_VIEW_LIST_VIEW_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    M5DensityVariant, M5DisclosureState, M5ItemStateFlag, M5LocalActionBudget,
    M5NavigationContentAccessibilityRoute, M5NavigationContentComponentFamily,
    M5NavigationContentConsumerSurface, M5NavigationContentDeploymentLine,
    M5NavigationContentDowngradeTrigger, M5NavigationContentQualificationClass,
    M5NavigationContentRequiredLabel, M5SelectionState, M5_LIST_VIEW_SCHEMA_REF,
    M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF, M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
    M5_TREE_VIEW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TreeListControlsPacket`].
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_RECORD_KIND: &str =
    "implement_m5_tree_view_and_list_view_controls";

/// Schema version for M5 tree-view / list-view controls records.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-tree-view-list-view-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_DOC_REF: &str =
    "docs/navigation/m5_tree_view_and_list_view_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-tree-view-list-view-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-tree-view-list-view-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-tree-view-list-view-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-tree-view-list-view-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5TreeListConsumerSurface = M5NavigationContentConsumerSurface;

/// Controlled count scope a tree or list names, so exact, loaded, and all-matching scopes are never
/// collapsed into one vague total and hidden or out-of-scope rows are never silently dropped. Minted
/// by this lane because the frozen matrix count-scope vocabulary carries exact / loaded / all-matching
/// / hidden-by-filter / hidden-by-policy but not the **outside-current-scope** count the tree / list
/// acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TreeListScopeKind {
    /// The exact total.
    ExactCount,
    /// Only the currently loaded / virtualized subset.
    LoadedCount,
    /// All matching items regardless of what is loaded.
    AllMatchingCount,
    /// The number of items hidden by an active filter.
    HiddenByFilter,
    /// The number of items hidden by policy.
    HiddenByPolicy,
    /// The number of items outside the current scope (e.g. another root or pane).
    OutsideCurrentScope,
    /// The count scope cannot currently be resolved.
    ScopeUnresolved,
}

impl M5TreeListScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExactCount,
        Self::LoadedCount,
        Self::AllMatchingCount,
        Self::HiddenByFilter,
        Self::HiddenByPolicy,
        Self::OutsideCurrentScope,
        Self::ScopeUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCount => "exact_count",
            Self::LoadedCount => "loaded_count",
            Self::AllMatchingCount => "all_matching_count",
            Self::HiddenByFilter => "hidden_by_filter",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::ScopeUnresolved => "scope_unresolved",
        }
    }

    /// Whether this scope names hidden or out-of-scope rows that must never be silently dropped.
    pub const fn is_hidden_or_outside(self) -> bool {
        matches!(
            self,
            Self::HiddenByFilter | Self::HiddenByPolicy | Self::OutsideCurrentScope
        )
    }

    /// Whether the count scope resolved to a concrete measure.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ScopeUnresolved)
    }
}

/// Controlled drag / reorder posture — whether a collection permits reorder and under what limit, so
/// a read-only or policy-locked collection never implies drag it does not support. Minted by this
/// lane because reorder posture is a tree / list capability the frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DragReorderPosture {
    /// Reorder is enabled across the whole collection.
    ReorderEnabled,
    /// Reorder is allowed only within the current scope / parent.
    ReorderWithinScopeOnly,
    /// Reorder is disabled by policy.
    ReorderDisabledByPolicy,
    /// The collection is read-only; reorder is not offered.
    ReorderReadOnly,
    /// Reorder is not supported by this component or backend.
    ReorderNotSupported,
    /// The reorder posture cannot currently be resolved.
    ReorderUnknown,
}

impl M5DragReorderPosture {
    /// Every drag / reorder posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReorderEnabled,
        Self::ReorderWithinScopeOnly,
        Self::ReorderDisabledByPolicy,
        Self::ReorderReadOnly,
        Self::ReorderNotSupported,
        Self::ReorderUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReorderEnabled => "reorder_enabled",
            Self::ReorderWithinScopeOnly => "reorder_within_scope_only",
            Self::ReorderDisabledByPolicy => "reorder_disabled_by_policy",
            Self::ReorderReadOnly => "reorder_read_only",
            Self::ReorderNotSupported => "reorder_not_supported",
            Self::ReorderUnknown => "reorder_unknown",
        }
    }

    /// Whether this posture actually permits a reorder gesture.
    pub const fn permits_reorder(self) -> bool {
        matches!(self, Self::ReorderEnabled | Self::ReorderWithinScopeOnly)
    }
}

/// Controlled cross-window / cross-pane continuity posture — whether selection and disclosure carry
/// across surfaces, so a single-pane collection never implies mirrored continuity it does not have.
/// Minted by this lane because cross-surface continuity is a tree / list capability the frozen matrix
/// does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CrossSurfaceContinuity {
    /// Continuity is scoped to this single pane only.
    SinglePaneOnly,
    /// Selection / disclosure is mirrored across panes.
    CrossPaneMirrored,
    /// Selection / disclosure is mirrored across windows.
    CrossWindowMirrored,
    /// Cross-surface continuity is not supported on this profile.
    ContinuityNotSupported,
    /// Continuity is claimed but cannot be verified on this profile.
    ContinuityUnverifiable,
    /// The continuity posture cannot currently be resolved.
    ContinuityUnknown,
}

impl M5CrossSurfaceContinuity {
    /// Every continuity posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SinglePaneOnly,
        Self::CrossPaneMirrored,
        Self::CrossWindowMirrored,
        Self::ContinuityNotSupported,
        Self::ContinuityUnverifiable,
        Self::ContinuityUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SinglePaneOnly => "single_pane_only",
            Self::CrossPaneMirrored => "cross_pane_mirrored",
            Self::CrossWindowMirrored => "cross_window_mirrored",
            Self::ContinuityNotSupported => "continuity_not_supported",
            Self::ContinuityUnverifiable => "continuity_unverifiable",
            Self::ContinuityUnknown => "continuity_unknown",
        }
    }

    /// Whether this posture actually mirrors continuity across surfaces.
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::CrossPaneMirrored | Self::CrossWindowMirrored)
    }

    /// Whether the continuity posture resolved to a concrete answer.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContinuityUnknown)
    }
}

/// One mandatory rendered part a tree or list must be able to show, so no disclosure, selection,
/// count, or local-action fact is left implicit behind compact chrome or pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TreeListAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed navigation / content disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The disclosure / expansion state (tree view).
    Disclosure,
    /// The selection-versus-current distinction.
    SelectionVersusCurrent,
    /// The row-focus indicator.
    RowFocus,
    /// The per-row item state.
    ItemState,
    /// The count scope (exact / loaded / hidden / outside).
    CountScope,
    /// The density variant.
    Density,
    /// The local-action budget / overflow.
    LocalActionBudget,
    /// The drag / reorder posture where allowed.
    DragReorderPosture,
    /// The cross-window / cross-pane continuity posture.
    CrossSurfaceContinuity,
    /// The virtualization truth (loaded versus complete).
    VirtualizationTruth,
    /// The command-backed path to trace the collection scope.
    ScopeCommand,
}

impl M5TreeListAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Disclosure,
        Self::SelectionVersusCurrent,
        Self::RowFocus,
        Self::ItemState,
        Self::CountScope,
        Self::Density,
        Self::LocalActionBudget,
        Self::DragReorderPosture,
        Self::CrossSurfaceContinuity,
        Self::VirtualizationTruth,
        Self::ScopeCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Disclosure => "disclosure",
            Self::SelectionVersusCurrent => "selection_versus_current",
            Self::RowFocus => "row_focus",
            Self::ItemState => "item_state",
            Self::CountScope => "count_scope",
            Self::Density => "density",
            Self::LocalActionBudget => "local_action_budget",
            Self::DragReorderPosture => "drag_reorder_posture",
            Self::CrossSurfaceContinuity => "cross_surface_continuity",
            Self::VirtualizationTruth => "virtualization_truth",
            Self::ScopeCommand => "scope_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to inspect the
/// collection's disclosure, selection, counts, or blocked rows behind a degraded component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TreeListNextAction {
    /// Open the command-backed scope / collection detail.
    OpenScopeDetail,
    /// Inspect the selection-versus-current and row focus.
    InspectSelectionAndFocus,
    /// Inspect the disclosure state and count scopes.
    InspectDisclosureAndCounts,
    /// Review a hidden, out-of-scope, or blocked row.
    ReviewHiddenOrBlocked,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5TreeListNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenScopeDetail,
        Self::InspectSelectionAndFocus,
        Self::InspectDisclosureAndCounts,
        Self::ReviewHiddenOrBlocked,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenScopeDetail => "open_scope_detail",
            Self::InspectSelectionAndFocus => "inspect_selection_and_focus",
            Self::InspectDisclosureAndCounts => "inspect_disclosure_and_counts",
            Self::ReviewHiddenOrBlocked => "review_hidden_or_blocked",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TreeListExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The navigation / content dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The selection state named by the collection.
    SelectionState,
    /// The disclosure state named by the tree.
    DisclosureState,
    /// The count scope named by the collection.
    CountScope,
    /// The density variant named by the collection.
    Density,
    /// The local-action budget named by the collection.
    LocalActionBudget,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TreeListExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SelectionState,
        Self::DisclosureState,
        Self::CountScope,
        Self::Density,
        Self::LocalActionBudget,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SelectionState => "selection_state",
            Self::DisclosureState => "disclosure_state",
            Self::CountScope => "count_scope",
            Self::Density => "density",
            Self::LocalActionBudget => "local_action_budget",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a tree view degraded below a clean, structure-legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous tree read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TreeViewDegradeReason {
    /// The node / current-object identity is unstated.
    NodeIdentityUnstated,
    /// The disclosure state cannot currently be resolved.
    DisclosureStateUnresolved,
    /// A lazily-unloaded subtree is presented as an empty leaf.
    LazySubtreeShownAsEmptyLeaf,
    /// The selection is collapsed into the current / focused item.
    SelectionVersusCurrentCollapsed,
    /// The row-focus indicator is not visible.
    RowFocusNotVisible,
    /// The current selection can only be discovered by pointer hover.
    CurrentSelectionHoverOnly,
    /// A blocked row's state can only be discovered by pointer hover.
    BlockedStateHoverOnly,
    /// The local actions can only be discovered by pointer hover.
    LocalActionsHoverOnly,
    /// The exact, loaded, and all-matching count scopes are collapsed into one total.
    CountScopeCollapsed,
    /// The count scope cannot currently be resolved.
    CountScopeUnresolved,
    /// A stale or partial backend is presented as a complete tree.
    StaleOrPartialShownAsComplete,
    /// Drag / reorder is overclaimed beyond the profile's actual posture.
    DragReorderOverclaimed,
    /// Cross-window / cross-pane continuity is overclaimed beyond the profile's actual posture.
    CrossSurfaceContinuityOverclaimed,
    /// No command-backed path to trace the collection scope is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TreeViewDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::NodeIdentityUnstated,
        Self::DisclosureStateUnresolved,
        Self::LazySubtreeShownAsEmptyLeaf,
        Self::SelectionVersusCurrentCollapsed,
        Self::RowFocusNotVisible,
        Self::CurrentSelectionHoverOnly,
        Self::BlockedStateHoverOnly,
        Self::LocalActionsHoverOnly,
        Self::CountScopeCollapsed,
        Self::CountScopeUnresolved,
        Self::StaleOrPartialShownAsComplete,
        Self::DragReorderOverclaimed,
        Self::CrossSurfaceContinuityOverclaimed,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodeIdentityUnstated => "node_identity_unstated",
            Self::DisclosureStateUnresolved => "disclosure_state_unresolved",
            Self::LazySubtreeShownAsEmptyLeaf => "lazy_subtree_shown_as_empty_leaf",
            Self::SelectionVersusCurrentCollapsed => "selection_versus_current_collapsed",
            Self::RowFocusNotVisible => "row_focus_not_visible",
            Self::CurrentSelectionHoverOnly => "current_selection_hover_only",
            Self::BlockedStateHoverOnly => "blocked_state_hover_only",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::CountScopeCollapsed => "count_scope_collapsed",
            Self::CountScopeUnresolved => "count_scope_unresolved",
            Self::StaleOrPartialShownAsComplete => "stale_or_partial_shown_as_complete",
            Self::DragReorderOverclaimed => "drag_reorder_overclaimed",
            Self::CrossSurfaceContinuityOverclaimed => "cross_surface_continuity_overclaimed",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TreeListNextAction {
        match self {
            Self::NodeIdentityUnstated
            | Self::DisclosureStateUnresolved
            | Self::LazySubtreeShownAsEmptyLeaf
            | Self::CountScopeCollapsed
            | Self::CountScopeUnresolved => M5TreeListNextAction::InspectDisclosureAndCounts,
            Self::SelectionVersusCurrentCollapsed
            | Self::RowFocusNotVisible
            | Self::CurrentSelectionHoverOnly => M5TreeListNextAction::InspectSelectionAndFocus,
            Self::BlockedStateHoverOnly
            | Self::LocalActionsHoverOnly
            | Self::StaleOrPartialShownAsComplete => M5TreeListNextAction::ReviewHiddenOrBlocked,
            Self::DragReorderOverclaimed
            | Self::CrossSurfaceContinuityOverclaimed
            | Self::ContextTracePathMissing => M5TreeListNextAction::OpenScopeDetail,
            Self::ProofStale => M5TreeListNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::NodeIdentityUnstated | Self::StaleOrPartialShownAsComplete => {
                M5NavigationContentDowngradeTrigger::HierarchyPathUnstated
            }
            Self::DisclosureStateUnresolved | Self::LazySubtreeShownAsEmptyLeaf => {
                M5NavigationContentDowngradeTrigger::DisclosureStateHidden
            }
            Self::SelectionVersusCurrentCollapsed
            | Self::RowFocusNotVisible
            | Self::CurrentSelectionHoverOnly => {
                M5NavigationContentDowngradeTrigger::SelectionVersusCurrentCollapsed
            }
            Self::BlockedStateHoverOnly => {
                M5NavigationContentDowngradeTrigger::BlockedRowsHiddenBehindEllipsis
            }
            Self::LocalActionsHoverOnly => {
                M5NavigationContentDowngradeTrigger::LocalActionsHoverOnly
            }
            Self::CountScopeCollapsed | Self::CountScopeUnresolved => {
                M5NavigationContentDowngradeTrigger::CountScopeCollapsed
            }
            Self::DragReorderOverclaimed
            | Self::CrossSurfaceContinuityOverclaimed
            | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a list view degraded below a clean, structure-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ListViewDegradeReason {
    /// The row / current-object identity is unstated.
    RowIdentityUnstated,
    /// The selection is collapsed into the current / focused item.
    SelectionVersusCurrentCollapsed,
    /// The row-focus indicator is not visible.
    RowFocusNotVisible,
    /// The current selection can only be discovered by pointer hover.
    CurrentSelectionHoverOnly,
    /// A blocked row's state can only be discovered by pointer hover.
    BlockedStateHoverOnly,
    /// The local actions can only be discovered by pointer hover.
    LocalActionsHoverOnly,
    /// The exact, loaded, and all-matching count scopes are collapsed into one total.
    CountScopeCollapsed,
    /// The count scope cannot currently be resolved.
    CountScopeUnresolved,
    /// A loaded / virtualized subset is presented as the exact total.
    LoadedShownAsExact,
    /// A stale or partial backend is presented as a complete list.
    StaleOrPartialShownAsComplete,
    /// Drag / reorder is overclaimed beyond the profile's actual posture.
    DragReorderOverclaimed,
    /// Cross-window / cross-pane continuity is overclaimed beyond the profile's actual posture.
    CrossSurfaceContinuityOverclaimed,
    /// No command-backed path to trace the collection scope is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ListViewDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::RowIdentityUnstated,
        Self::SelectionVersusCurrentCollapsed,
        Self::RowFocusNotVisible,
        Self::CurrentSelectionHoverOnly,
        Self::BlockedStateHoverOnly,
        Self::LocalActionsHoverOnly,
        Self::CountScopeCollapsed,
        Self::CountScopeUnresolved,
        Self::LoadedShownAsExact,
        Self::StaleOrPartialShownAsComplete,
        Self::DragReorderOverclaimed,
        Self::CrossSurfaceContinuityOverclaimed,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowIdentityUnstated => "row_identity_unstated",
            Self::SelectionVersusCurrentCollapsed => "selection_versus_current_collapsed",
            Self::RowFocusNotVisible => "row_focus_not_visible",
            Self::CurrentSelectionHoverOnly => "current_selection_hover_only",
            Self::BlockedStateHoverOnly => "blocked_state_hover_only",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::CountScopeCollapsed => "count_scope_collapsed",
            Self::CountScopeUnresolved => "count_scope_unresolved",
            Self::LoadedShownAsExact => "loaded_shown_as_exact",
            Self::StaleOrPartialShownAsComplete => "stale_or_partial_shown_as_complete",
            Self::DragReorderOverclaimed => "drag_reorder_overclaimed",
            Self::CrossSurfaceContinuityOverclaimed => "cross_surface_continuity_overclaimed",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TreeListNextAction {
        match self {
            Self::RowIdentityUnstated
            | Self::CountScopeCollapsed
            | Self::CountScopeUnresolved
            | Self::LoadedShownAsExact => M5TreeListNextAction::InspectDisclosureAndCounts,
            Self::SelectionVersusCurrentCollapsed
            | Self::RowFocusNotVisible
            | Self::CurrentSelectionHoverOnly => M5TreeListNextAction::InspectSelectionAndFocus,
            Self::BlockedStateHoverOnly
            | Self::LocalActionsHoverOnly
            | Self::StaleOrPartialShownAsComplete => M5TreeListNextAction::ReviewHiddenOrBlocked,
            Self::DragReorderOverclaimed
            | Self::CrossSurfaceContinuityOverclaimed
            | Self::ContextTracePathMissing => M5TreeListNextAction::OpenScopeDetail,
            Self::ProofStale => M5TreeListNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::RowIdentityUnstated => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::SelectionVersusCurrentCollapsed
            | Self::RowFocusNotVisible
            | Self::CurrentSelectionHoverOnly => {
                M5NavigationContentDowngradeTrigger::SelectionVersusCurrentCollapsed
            }
            Self::BlockedStateHoverOnly => {
                M5NavigationContentDowngradeTrigger::BlockedRowsHiddenBehindEllipsis
            }
            Self::LocalActionsHoverOnly => {
                M5NavigationContentDowngradeTrigger::LocalActionsHoverOnly
            }
            Self::CountScopeCollapsed | Self::CountScopeUnresolved | Self::LoadedShownAsExact => {
                M5NavigationContentDowngradeTrigger::CountScopeCollapsed
            }
            Self::StaleOrPartialShownAsComplete
            | Self::DragReorderOverclaimed
            | Self::CrossSurfaceContinuityOverclaimed
            | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_tree_view`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TreeViewResolutionInput {
    /// Stable identity of the tree instance.
    pub tree_id: String,
    /// The current node / current-object label shown; empty means unstated.
    pub node_label: String,
    /// The disclosure / expansion state of the current node.
    pub disclosure: M5DisclosureState,
    /// True when a lazily-unloaded subtree is drawn as an empty leaf rather than a collapsed parent.
    pub lazy_subtree_shown_as_leaf: bool,
    /// The selection state of the collection.
    pub selection: M5SelectionState,
    /// True when the selection stays distinct from the current / focused item.
    pub selection_versus_current_distinct: bool,
    /// True when the row-focus indicator is visible (never hover-only).
    pub row_focus_visible: bool,
    /// True when the current selection can only be discovered by pointer hover.
    pub current_selection_hover_only: bool,
    /// The per-row item state of the current row.
    pub item_state: M5ItemStateFlag,
    /// True when at least one blocked row is present in the collection.
    pub has_blocked_row: bool,
    /// True when a blocked row's state can only be discovered by pointer hover.
    pub blocked_state_hover_only: bool,
    /// The count scope the visible count measures.
    pub count_scope: M5TreeListScopeKind,
    /// True when the exact / loaded / all-matching / hidden scopes stay distinct, never collapsed.
    pub count_scopes_distinct: bool,
    /// The density variant the tree renders at.
    pub density: M5DensityVariant,
    /// The local-action budget for the tree.
    pub local_action_budget: M5LocalActionBudget,
    /// True when the local actions can only be discovered by pointer hover.
    pub local_actions_hover_only: bool,
    /// The drag / reorder posture.
    pub drag_reorder: M5DragReorderPosture,
    /// True when drag / reorder is claimed beyond the posture's actual capability.
    pub overclaims_drag_reorder: bool,
    /// The cross-window / cross-pane continuity posture.
    pub cross_surface_continuity: M5CrossSurfaceContinuity,
    /// True when continuity is claimed beyond the posture's actual capability.
    pub overclaims_cross_surface_continuity: bool,
    /// True when the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// True when a stale or partial backend is presented as a complete tree.
    pub presents_stale_or_partial_as_complete: bool,
    /// True when a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe tree-view projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTreeView {
    /// Stable identity of the tree instance.
    pub tree_id: String,
    /// The current node / current-object label named by the tree.
    pub node_label: String,
    /// The disclosure-state token named by the tree.
    pub disclosure: String,
    /// Guardrail (MUST be `false` on a clean tree): a lazy subtree reads as an empty leaf.
    pub lazy_subtree_shown_as_leaf: bool,
    /// The selection-state token named by the tree.
    pub selection: String,
    /// Whether the selection stays distinct from the current / focused item.
    pub selection_versus_current_distinct: bool,
    /// Whether the row-focus indicator is visible.
    pub row_focus_visible: bool,
    /// Guardrail (MUST be `false` on a clean tree): the current selection is hover-only.
    pub current_selection_hover_only: bool,
    /// The per-row item-state token named by the tree.
    pub item_state: String,
    /// Whether a blocked row is present in the collection.
    pub has_blocked_row: bool,
    /// Guardrail (MUST be `false` on a clean tree): a blocked row's state is hover-only.
    pub blocked_state_hover_only: bool,
    /// The count-scope token named by the tree.
    pub count_scope: String,
    /// Whether the count scope names hidden or out-of-scope rows.
    pub count_scope_hidden_or_outside: bool,
    /// Whether the exact / loaded / all-matching / hidden scopes stay distinct.
    pub count_scopes_distinct: bool,
    /// The density-variant token named by the tree.
    pub density: String,
    /// The local-action-budget token named by the tree.
    pub local_action_budget: String,
    /// Guardrail (MUST be `false` on a clean tree): the local actions are hover-only.
    pub local_actions_hover_only: bool,
    /// The drag / reorder-posture token named by the tree.
    pub drag_reorder: String,
    /// Whether the posture actually permits a reorder gesture.
    pub drag_reorder_permitted: bool,
    /// Guardrail (MUST be `false` on a clean tree): drag / reorder is overclaimed.
    pub overclaims_drag_reorder: bool,
    /// The cross-surface-continuity token named by the tree.
    pub cross_surface_continuity: String,
    /// Whether the posture actually mirrors continuity across surfaces.
    pub continuity_continuous: bool,
    /// Guardrail (MUST be `false` on a clean tree): cross-surface continuity is overclaimed.
    pub overclaims_cross_surface_continuity: bool,
    /// Whether the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// Guardrail (MUST be `false` on a clean tree): a stale / partial backend reads as complete.
    pub presents_stale_or_partial_as_complete: bool,
    /// Whether a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the tree could not read as a clean, structure-legible state.
    pub degrade_reason: Option<M5TreeViewDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TreeListNextAction,
    /// Whether the structure is legible at a glance (clean tree naming every fact).
    pub structure_legible_at_a_glance: bool,
}

impl M5ResolvedTreeView {
    /// Whether this tree reads as a clean, structure-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_list_view`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ListViewResolutionInput {
    /// Stable identity of the list instance.
    pub list_id: String,
    /// The current row / current-object label shown; empty means unstated.
    pub row_label: String,
    /// The selection state of the collection.
    pub selection: M5SelectionState,
    /// True when the selection stays distinct from the current / focused item.
    pub selection_versus_current_distinct: bool,
    /// True when the row-focus indicator is visible (never hover-only).
    pub row_focus_visible: bool,
    /// True when the current selection can only be discovered by pointer hover.
    pub current_selection_hover_only: bool,
    /// The per-row item state of the current row.
    pub item_state: M5ItemStateFlag,
    /// True when at least one blocked row is present in the collection.
    pub has_blocked_row: bool,
    /// True when a blocked row's state can only be discovered by pointer hover.
    pub blocked_state_hover_only: bool,
    /// The count scope the visible count measures.
    pub count_scope: M5TreeListScopeKind,
    /// True when the exact / loaded / all-matching / hidden scopes stay distinct, never collapsed.
    pub count_scopes_distinct: bool,
    /// True when a loaded / virtualized subset is presented as the exact total.
    pub loaded_shown_as_exact: bool,
    /// The density variant the list renders at.
    pub density: M5DensityVariant,
    /// The local-action budget for the list.
    pub local_action_budget: M5LocalActionBudget,
    /// True when the local actions can only be discovered by pointer hover.
    pub local_actions_hover_only: bool,
    /// The drag / reorder posture.
    pub drag_reorder: M5DragReorderPosture,
    /// True when drag / reorder is claimed beyond the posture's actual capability.
    pub overclaims_drag_reorder: bool,
    /// The cross-window / cross-pane continuity posture.
    pub cross_surface_continuity: M5CrossSurfaceContinuity,
    /// True when continuity is claimed beyond the posture's actual capability.
    pub overclaims_cross_surface_continuity: bool,
    /// True when the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// True when a stale or partial backend is presented as a complete list.
    pub presents_stale_or_partial_as_complete: bool,
    /// True when a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe list-view projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedListView {
    /// Stable identity of the list instance.
    pub list_id: String,
    /// The current row / current-object label named by the list.
    pub row_label: String,
    /// The selection-state token named by the list.
    pub selection: String,
    /// Whether the selection stays distinct from the current / focused item.
    pub selection_versus_current_distinct: bool,
    /// Whether the row-focus indicator is visible.
    pub row_focus_visible: bool,
    /// Guardrail (MUST be `false` on a clean list): the current selection is hover-only.
    pub current_selection_hover_only: bool,
    /// The per-row item-state token named by the list.
    pub item_state: String,
    /// Whether a blocked row is present in the collection.
    pub has_blocked_row: bool,
    /// Guardrail (MUST be `false` on a clean list): a blocked row's state is hover-only.
    pub blocked_state_hover_only: bool,
    /// The count-scope token named by the list.
    pub count_scope: String,
    /// Whether the count scope names hidden or out-of-scope rows.
    pub count_scope_hidden_or_outside: bool,
    /// Whether the exact / loaded / all-matching / hidden scopes stay distinct.
    pub count_scopes_distinct: bool,
    /// Guardrail (MUST be `false` on a clean list): a loaded subset reads as the exact total.
    pub loaded_shown_as_exact: bool,
    /// The density-variant token named by the list.
    pub density: String,
    /// The local-action-budget token named by the list.
    pub local_action_budget: String,
    /// Guardrail (MUST be `false` on a clean list): the local actions are hover-only.
    pub local_actions_hover_only: bool,
    /// The drag / reorder-posture token named by the list.
    pub drag_reorder: String,
    /// Whether the posture actually permits a reorder gesture.
    pub drag_reorder_permitted: bool,
    /// Guardrail (MUST be `false` on a clean list): drag / reorder is overclaimed.
    pub overclaims_drag_reorder: bool,
    /// The cross-surface-continuity token named by the list.
    pub cross_surface_continuity: String,
    /// Whether the posture actually mirrors continuity across surfaces.
    pub continuity_continuous: bool,
    /// Guardrail (MUST be `false` on a clean list): cross-surface continuity is overclaimed.
    pub overclaims_cross_surface_continuity: bool,
    /// Whether the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// Guardrail (MUST be `false` on a clean list): a stale / partial backend reads as complete.
    pub presents_stale_or_partial_as_complete: bool,
    /// Whether a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the list could not read as a clean, structure-legible state.
    pub degrade_reason: Option<M5ListViewDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TreeListNextAction,
    /// Whether the structure is legible at a glance (clean list naming every fact).
    pub structure_legible_at_a_glance: bool,
}

impl M5ResolvedListView {
    /// Whether this list reads as a clean, structure-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TreeListResolutionError {
    /// The tree id was empty.
    EmptyTreeId,
    /// The list id was empty.
    EmptyListId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TreeListResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyTreeId => "empty_tree_id",
            Self::EmptyListId => "empty_list_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TreeListResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 tree-view / list-view resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TreeListResolutionError {}

/// True when the selection state cannot be resolved.
fn selection_is_unresolved(state: M5SelectionState) -> bool {
    matches!(state, M5SelectionState::SelectionUnknown)
}

/// Resolves a tree view so its structure is legible at a glance: the tree names its disclosure state
/// (never faking a lazy subtree as an empty leaf), its selection-versus-current distinction and row
/// focus (never hover-only), its per-row item state (blocked state never hover-only), its count scope
/// (never collapsed), and its density and local-action budget, and keeps drag / reorder posture and
/// cross-surface continuity honest instead of overclaiming them.
pub fn resolve_tree_view(
    input: M5TreeViewResolutionInput,
) -> Result<M5ResolvedTreeView, M5TreeListResolutionError> {
    if input.tree_id.trim().is_empty() {
        return Err(M5TreeListResolutionError::EmptyTreeId);
    }
    if string_is_forbidden(&input.tree_id) || string_is_forbidden(&input.node_label) {
        return Err(M5TreeListResolutionError::ForbiddenMaterial);
    }

    let blocked_present =
        input.has_blocked_row || matches!(input.item_state, M5ItemStateFlag::Blocked);

    let degrade_reason = if input.node_label.trim().is_empty() {
        Some(M5TreeViewDegradeReason::NodeIdentityUnstated)
    } else if matches!(input.disclosure, M5DisclosureState::DisclosureUnknown) {
        Some(M5TreeViewDegradeReason::DisclosureStateUnresolved)
    } else if input.lazy_subtree_shown_as_leaf {
        Some(M5TreeViewDegradeReason::LazySubtreeShownAsEmptyLeaf)
    } else if selection_is_unresolved(input.selection) || !input.selection_versus_current_distinct {
        Some(M5TreeViewDegradeReason::SelectionVersusCurrentCollapsed)
    } else if !input.row_focus_visible {
        Some(M5TreeViewDegradeReason::RowFocusNotVisible)
    } else if input.current_selection_hover_only {
        Some(M5TreeViewDegradeReason::CurrentSelectionHoverOnly)
    } else if blocked_present && input.blocked_state_hover_only {
        Some(M5TreeViewDegradeReason::BlockedStateHoverOnly)
    } else if input.local_actions_hover_only {
        Some(M5TreeViewDegradeReason::LocalActionsHoverOnly)
    } else if !input.count_scopes_distinct {
        Some(M5TreeViewDegradeReason::CountScopeCollapsed)
    } else if !input.count_scope.is_resolved() {
        Some(M5TreeViewDegradeReason::CountScopeUnresolved)
    } else if input.backend_stale_or_partial && input.presents_stale_or_partial_as_complete {
        Some(M5TreeViewDegradeReason::StaleOrPartialShownAsComplete)
    } else if input.overclaims_drag_reorder {
        Some(M5TreeViewDegradeReason::DragReorderOverclaimed)
    } else if input.overclaims_cross_surface_continuity {
        Some(M5TreeViewDegradeReason::CrossSurfaceContinuityOverclaimed)
    } else if !input.detail_command_available {
        Some(M5TreeViewDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5TreeViewDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TreeListNextAction::OpenScopeDetail,
    };

    Ok(M5ResolvedTreeView {
        tree_id: input.tree_id,
        node_label: input.node_label,
        disclosure: input.disclosure.as_str().to_owned(),
        lazy_subtree_shown_as_leaf: input.lazy_subtree_shown_as_leaf,
        selection: input.selection.as_str().to_owned(),
        selection_versus_current_distinct: input.selection_versus_current_distinct,
        row_focus_visible: input.row_focus_visible,
        current_selection_hover_only: input.current_selection_hover_only,
        item_state: input.item_state.as_str().to_owned(),
        has_blocked_row: blocked_present,
        blocked_state_hover_only: input.blocked_state_hover_only,
        count_scope: input.count_scope.as_str().to_owned(),
        count_scope_hidden_or_outside: input.count_scope.is_hidden_or_outside(),
        count_scopes_distinct: input.count_scopes_distinct,
        density: input.density.as_str().to_owned(),
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        local_actions_hover_only: input.local_actions_hover_only,
        drag_reorder: input.drag_reorder.as_str().to_owned(),
        drag_reorder_permitted: input.drag_reorder.permits_reorder(),
        overclaims_drag_reorder: input.overclaims_drag_reorder,
        cross_surface_continuity: input.cross_surface_continuity.as_str().to_owned(),
        continuity_continuous: input.cross_surface_continuity.is_continuous(),
        overclaims_cross_surface_continuity: input.overclaims_cross_surface_continuity,
        backend_stale_or_partial: input.backend_stale_or_partial,
        presents_stale_or_partial_as_complete: input.backend_stale_or_partial
            && input.presents_stale_or_partial_as_complete,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        structure_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a list view so its structure is legible at a glance: the list names its selection-versus-
/// current distinction and row focus (never hover-only), its per-row item state (blocked state never
/// hover-only), its count scope (never collapsed, and a loaded subset never presented as the exact
/// total), and its density and local-action budget, and keeps drag / reorder posture and cross-
/// surface continuity honest instead of overclaiming them.
pub fn resolve_list_view(
    input: M5ListViewResolutionInput,
) -> Result<M5ResolvedListView, M5TreeListResolutionError> {
    if input.list_id.trim().is_empty() {
        return Err(M5TreeListResolutionError::EmptyListId);
    }
    if string_is_forbidden(&input.list_id) || string_is_forbidden(&input.row_label) {
        return Err(M5TreeListResolutionError::ForbiddenMaterial);
    }

    let blocked_present =
        input.has_blocked_row || matches!(input.item_state, M5ItemStateFlag::Blocked);

    let degrade_reason = if input.row_label.trim().is_empty() {
        Some(M5ListViewDegradeReason::RowIdentityUnstated)
    } else if selection_is_unresolved(input.selection) || !input.selection_versus_current_distinct {
        Some(M5ListViewDegradeReason::SelectionVersusCurrentCollapsed)
    } else if !input.row_focus_visible {
        Some(M5ListViewDegradeReason::RowFocusNotVisible)
    } else if input.current_selection_hover_only {
        Some(M5ListViewDegradeReason::CurrentSelectionHoverOnly)
    } else if blocked_present && input.blocked_state_hover_only {
        Some(M5ListViewDegradeReason::BlockedStateHoverOnly)
    } else if input.local_actions_hover_only {
        Some(M5ListViewDegradeReason::LocalActionsHoverOnly)
    } else if !input.count_scopes_distinct {
        Some(M5ListViewDegradeReason::CountScopeCollapsed)
    } else if !input.count_scope.is_resolved() {
        Some(M5ListViewDegradeReason::CountScopeUnresolved)
    } else if input.loaded_shown_as_exact {
        Some(M5ListViewDegradeReason::LoadedShownAsExact)
    } else if input.backend_stale_or_partial && input.presents_stale_or_partial_as_complete {
        Some(M5ListViewDegradeReason::StaleOrPartialShownAsComplete)
    } else if input.overclaims_drag_reorder {
        Some(M5ListViewDegradeReason::DragReorderOverclaimed)
    } else if input.overclaims_cross_surface_continuity {
        Some(M5ListViewDegradeReason::CrossSurfaceContinuityOverclaimed)
    } else if !input.detail_command_available {
        Some(M5ListViewDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5ListViewDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TreeListNextAction::OpenScopeDetail,
    };

    Ok(M5ResolvedListView {
        list_id: input.list_id,
        row_label: input.row_label,
        selection: input.selection.as_str().to_owned(),
        selection_versus_current_distinct: input.selection_versus_current_distinct,
        row_focus_visible: input.row_focus_visible,
        current_selection_hover_only: input.current_selection_hover_only,
        item_state: input.item_state.as_str().to_owned(),
        has_blocked_row: blocked_present,
        blocked_state_hover_only: input.blocked_state_hover_only,
        count_scope: input.count_scope.as_str().to_owned(),
        count_scope_hidden_or_outside: input.count_scope.is_hidden_or_outside(),
        count_scopes_distinct: input.count_scopes_distinct,
        loaded_shown_as_exact: input.loaded_shown_as_exact,
        density: input.density.as_str().to_owned(),
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        local_actions_hover_only: input.local_actions_hover_only,
        drag_reorder: input.drag_reorder.as_str().to_owned(),
        drag_reorder_permitted: input.drag_reorder.permits_reorder(),
        overclaims_drag_reorder: input.overclaims_drag_reorder,
        cross_surface_continuity: input.cross_surface_continuity.as_str().to_owned(),
        continuity_continuous: input.cross_surface_continuity.is_continuous(),
        overclaims_cross_surface_continuity: input.overclaims_cross_surface_continuity,
        backend_stale_or_partial: input.backend_stale_or_partial,
        presents_stale_or_partial_as_complete: input.backend_stale_or_partial
            && input.presents_stale_or_partial_as_complete,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        structure_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved tree and list examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5TreeListConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5NavigationContentQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5NavigationContentDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5NavigationContentRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5NavigationContentAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5TreeListAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TreeListExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    /// Resolved tree-view examples.
    pub tree_view_examples: Vec<M5ResolvedTreeView>,
    /// Resolved list-view examples.
    pub list_view_examples: Vec<M5ResolvedListView>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: current selection, blocked state, or local actions are never hover-only.
    pub hides_current_selection_blocked_or_actions_behind_hover_only: bool,
    /// Hard invariant: selection-versus-current and count scopes are never collapsed.
    pub collapses_selection_versus_current_or_count_scopes: bool,
    /// Hard invariant: a stale, partial, or lazy collection is never presented as complete.
    pub presents_stale_partial_or_lazy_collection_as_complete: bool,
    /// Hard invariant: drag / reorder or cross-surface continuity is never overclaimed.
    pub overclaims_drag_reorder_or_cross_surface_continuity: bool,
}

impl M5TreeListControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TreeListAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5TreeListAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TreeListExportField> = self.export_fields.iter().copied().collect();
        M5TreeListExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_current_selection_blocked_or_actions_behind_hover_only
            && !self.collapses_selection_versus_current_or_count_scopes
            && !self.presents_stale_partial_or_lazy_collection_as_complete
            && !self.overclaims_drag_reorder_or_cross_surface_continuity
    }

    /// True when every resolved example on this row is honest: no clean tree or list hides its
    /// current selection / blocked state / actions behind hover, collapses selection or counts,
    /// fakes a complete collection, overclaims drag or continuity, or lacks a trace path.
    fn examples_are_honest(&self) -> bool {
        self.tree_view_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.current_selection_hover_only
                    || ex.blocked_state_hover_only
                    || ex.local_actions_hover_only
                    || !ex.selection_versus_current_distinct
                    || !ex.count_scopes_distinct
                    || ex.lazy_subtree_shown_as_leaf
                    || ex.presents_stale_or_partial_as_complete
                    || ex.overclaims_drag_reorder
                    || ex.overclaims_cross_surface_continuity
                    || !ex.detail_command_available))
        }) && self.list_view_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.current_selection_hover_only
                    || ex.blocked_state_hover_only
                    || ex.local_actions_hover_only
                    || !ex.selection_versus_current_distinct
                    || !ex.count_scopes_distinct
                    || ex.loaded_shown_as_exact
                    || ex.presents_stale_or_partial_as_complete
                    || ex.overclaims_drag_reorder
                    || ex.overclaims_cross_surface_continuity
                    || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListVocabularySet {
    /// Disclosure-state tokens (bound from the frozen matrix).
    pub disclosure_states: Vec<String>,
    /// Selection-state tokens (bound from the frozen matrix).
    pub selection_states: Vec<String>,
    /// Item-state-flag tokens (bound from the frozen matrix).
    pub item_state_flags: Vec<String>,
    /// Density-variant tokens (bound from the frozen matrix).
    pub density_variants: Vec<String>,
    /// Local-action-budget tokens (bound from the frozen matrix).
    pub local_action_budgets: Vec<String>,
    /// Count-scope-kind tokens (minted by this lane).
    pub count_scope_kinds: Vec<String>,
    /// Drag / reorder-posture tokens (minted by this lane).
    pub drag_reorder_postures: Vec<String>,
    /// Cross-surface-continuity tokens (minted by this lane).
    pub cross_surface_continuities: Vec<String>,
    /// Tree-view degrade-reason tokens.
    pub tree_view_degrade_reasons: Vec<String>,
    /// List-view degrade-reason tokens.
    pub list_view_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TreeListVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            disclosure_states: tokens(&M5DisclosureState::ALL, |v| v.as_str()),
            selection_states: tokens(&M5SelectionState::ALL, |v| v.as_str()),
            item_state_flags: tokens(&M5ItemStateFlag::ALL, |v| v.as_str()),
            density_variants: tokens(&M5DensityVariant::ALL, |v| v.as_str()),
            local_action_budgets: tokens(&M5LocalActionBudget::ALL, |v| v.as_str()),
            count_scope_kinds: tokens(&M5TreeListScopeKind::ALL, |v| v.as_str()),
            drag_reorder_postures: tokens(&M5DragReorderPosture::ALL, |v| v.as_str()),
            cross_surface_continuities: tokens(&M5CrossSurfaceContinuity::ALL, |v| v.as_str()),
            tree_view_degrade_reasons: tokens(&M5TreeViewDegradeReason::ALL, |v| v.as_str()),
            list_view_degrade_reasons: tokens(&M5ListViewDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TreeListAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TreeListNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TreeListExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5NavigationContentConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListGovernanceReview {
    /// The tree names its disclosure, selection-versus-current, and count scopes.
    pub tree_names_disclosure_selection_and_counts: bool,
    /// Tree virtualization is honest: a lazy subtree never reads as complete.
    pub tree_virtualization_honest_never_fakes_complete: bool,
    /// The list names its selection-versus-current, count scopes, and density.
    pub list_names_selection_counts_and_density: bool,
    /// A loaded / virtualized list subset is never presented as the exact total.
    pub list_loaded_never_shown_as_exact: bool,
    /// Selection is always kept distinct from the current / focused item.
    pub selection_versus_current_always_distinct: bool,
    /// The current selection, a blocked row, and the local actions are never hover-only.
    pub current_selection_blocked_and_actions_never_hover_only: bool,
    /// Exact, loaded, and all-matching count scopes are never collapsed.
    pub count_scopes_never_collapsed: bool,
    /// Drag / reorder posture stays honest where allowed.
    pub drag_reorder_posture_honest_where_allowed: bool,
    /// Cross-window / cross-pane continuity is never overclaimed.
    pub cross_surface_continuity_never_overclaimed: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListConsumerProjection {
    /// The explorer consumes the shared tree disclosure / scope vocabulary.
    pub explorer_consumes_tree_disclosure_and_scope_vocabulary: bool,
    /// The review queue consumes the shared list selection / scope vocabulary.
    pub review_queue_consumes_list_selection_and_scope_vocabulary: bool,
    /// Search and provider surfaces consume the same shared row semantics.
    pub search_and_provider_consume_shared_row_semantics: bool,
    /// Collection facts trace back to one canonical component contract.
    pub collection_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical collection source.
    pub support_export_reads_single_collection_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TreeListControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TreeListControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TreeListControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TreeListVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TreeListGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TreeListConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TreeListProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TreeListReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 tree-view / list-view controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TreeListControlsPacket {
    /// Record kind; must equal [`M5_TREE_VIEW_LIST_VIEW_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TreeListControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TreeListVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TreeListGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TreeListConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TreeListProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TreeListReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TreeListControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5TreeListControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TREE_VIEW_LIST_VIEW_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5TreeListControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TREE_VIEW_LIST_VIEW_CONTROLS_RECORD_KIND {
            violations.push(M5TreeListControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_VERSION {
            violations.push(M5TreeListControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TreeListControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TreeListControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 tree-view / list-view controls packet serializes"),
        ) {
            violations.push(M5TreeListControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 tree-view / list-view controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,tree_examples,list_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .tree_view_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.list_view_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.tree_view_examples.len(),
                row.list_view_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Tree-View and List-View Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Count scope kinds: {}\n",
            self.vocabulary_set.count_scope_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Drag / reorder postures: {}\n",
            self.vocabulary_set.drag_reorder_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Tree-view examples: {} / list-view examples: {}\n",
                row.tree_view_examples.len(),
                row.list_view_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5TreeListControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TreeListControlsViolation>),
}

impl fmt::Display for M5TreeListControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 tree-view / list-view controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 tree-view / list-view controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TreeListControlsArtifactError {}

/// Validation failures emitted by [`M5TreeListControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TreeListControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (hover-only, collapsed, faked, or
    /// overclaimed).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Shared row semantics are not proven: clean tree and list examples do not reuse the same scope
    /// vocabulary across surfaces, or no count-scope-collapse example degrades.
    SharedRowSemanticsNotProven,
    /// Selection and disclosure truth is not proven: no partial / lazy example stays honest, or no
    /// lazy-shown-as-leaf / stale-shown-complete example degrades.
    SelectionAndDisclosureTruthNotProven,
    /// Hover-free discovery is not proven: no current-selection / blocked / local-action hover-only
    /// example degrades, or a clean example hides one behind hover.
    NoHoverOnlyDiscoveryNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TreeListControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::SharedRowSemanticsNotProven => "shared_row_semantics_not_proven",
            Self::SelectionAndDisclosureTruthNotProven => {
                "selection_and_disclosure_truth_not_proven"
            }
            Self::NoHoverOnlyDiscoveryNotProven => "no_hover_only_discovery_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_tree_view_list_view_controls_export(
) -> Result<M5TreeListControlsPacket, M5TreeListControlsArtifactError> {
    let packet: M5TreeListControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-tree-view-list-view-controls-proof/support_export.json"
    )))
    .map_err(M5TreeListControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TreeListControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_REF,
        M5_TREE_VIEW_LIST_VIEW_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TREE_VIEW_SCHEMA_REF,
        M5_LIST_VIEW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TreeListControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5TreeListControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5TreeListControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TreeListControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TreeListControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TREE_VIEW_SCHEMA_REF) || !refs.contains(M5_LIST_VIEW_SCHEMA_REF) {
            violations.push(M5TreeListControlsViolation::ComponentSchemaRefMissing);
        }
        if row.tree_view_examples.is_empty() || row.list_view_examples.is_empty() {
            violations.push(M5TreeListControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TreeListControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TreeListControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tree_names_disclosure_selection_and_counts,
        review.tree_virtualization_honest_never_fakes_complete,
        review.list_names_selection_counts_and_density,
        review.list_loaded_never_shown_as_exact,
        review.selection_versus_current_always_distinct,
        review.current_selection_blocked_and_actions_never_hover_only,
        review.count_scopes_never_collapsed,
        review.drag_reorder_posture_honest_where_allowed,
        review.cross_surface_continuity_never_overclaimed,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TreeListControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.explorer_consumes_tree_disclosure_and_scope_vocabulary,
        projection.review_queue_consumes_list_selection_and_scope_vocabulary,
        projection.search_and_provider_consume_shared_row_semantics,
        projection.collection_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_collection_source,
    ] {
        if !ok {
            violations.push(M5TreeListControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TreeListControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TreeListControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TreeListControlsPacket,
    violations: &mut Vec<M5TreeListControlsViolation>,
) {
    let trees = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.tree_view_examples.iter())
    };
    let lists = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.list_view_examples.iter())
    };

    // AC1: explorer, search, review-queue, provider, and support-facing tree / list consumers reuse
    // the same row semantics and scope vocabulary. Clean tree and list examples both exist, together
    // cover at least two distinct scope kinds from the shared vocabulary, a count-scope-collapse
    // example degrades on the tree side and the list side, and no clean example collapses scopes.
    let clean_scope_kinds: BTreeSet<String> = trees()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.count_scope.clone())
        .chain(
            lists()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.count_scope.clone()),
        )
        .collect();
    let has_clean_tree = trees().any(|ex| ex.is_clean());
    let has_clean_list = lists().any(|ex| ex.is_clean());
    let tree_scope_collapse_degrades =
        trees().any(|ex| ex.degrade_reason == Some(M5TreeViewDegradeReason::CountScopeCollapsed));
    let list_scope_collapse_degrades =
        lists().any(|ex| ex.degrade_reason == Some(M5ListViewDegradeReason::CountScopeCollapsed));
    let no_clean_scope_collapse = trees().all(|ex| !ex.is_clean() || ex.count_scopes_distinct)
        && lists().all(|ex| !ex.is_clean() || ex.count_scopes_distinct);
    if !(has_clean_tree
        && has_clean_list
        && clean_scope_kinds.len() >= 2
        && tree_scope_collapse_degrades
        && list_scope_collapse_degrades
        && no_clean_scope_collapse)
    {
        violations.push(M5TreeListControlsViolation::SharedRowSemanticsNotProven);
    }

    // AC2: deep nesting, compact layouts, and stale or partial backends preserve selection and
    // disclosure truth rather than faking a complete tree. At least one clean tree honestly discloses
    // a lazy / partial backend, a lazy-shown-as-leaf example degrades, a stale-shown-complete example
    // degrades, and no clean example fakes a complete tree.
    let honest_partial_tree = trees().any(|ex| {
        ex.is_clean() && ex.backend_stale_or_partial && !ex.presents_stale_or_partial_as_complete
    });
    let lazy_leaf_degrades = trees()
        .any(|ex| ex.degrade_reason == Some(M5TreeViewDegradeReason::LazySubtreeShownAsEmptyLeaf));
    let stale_complete_degrades = trees().any(|ex| {
        ex.degrade_reason == Some(M5TreeViewDegradeReason::StaleOrPartialShownAsComplete)
    }) || lists().any(|ex| {
        ex.degrade_reason == Some(M5ListViewDegradeReason::StaleOrPartialShownAsComplete)
    });
    let no_clean_fakes_complete = trees().all(|ex| {
        !(ex.is_clean()
            && (ex.lazy_subtree_shown_as_leaf || ex.presents_stale_or_partial_as_complete))
    }) && lists()
        .all(|ex| !(ex.is_clean() && ex.presents_stale_or_partial_as_complete));
    if !(honest_partial_tree
        && lazy_leaf_degrades
        && stale_complete_degrades
        && no_clean_fakes_complete)
    {
        violations.push(M5TreeListControlsViolation::SelectionAndDisclosureTruthNotProven);
    }

    // AC3: no claimed M5 tree / list surface requires pointer hover to discover the current
    // selection, blocked state, or available local actions. A current-selection, a blocked-state, and
    // a local-action hover-only example each degrade, and no clean example hides one behind hover.
    let current_hover_degrades = trees()
        .any(|ex| ex.degrade_reason == Some(M5TreeViewDegradeReason::CurrentSelectionHoverOnly))
        || lists().any(|ex| {
            ex.degrade_reason == Some(M5ListViewDegradeReason::CurrentSelectionHoverOnly)
        });
    let blocked_hover_degrades = trees()
        .any(|ex| ex.degrade_reason == Some(M5TreeViewDegradeReason::BlockedStateHoverOnly))
        || lists()
            .any(|ex| ex.degrade_reason == Some(M5ListViewDegradeReason::BlockedStateHoverOnly));
    let actions_hover_degrades = trees()
        .any(|ex| ex.degrade_reason == Some(M5TreeViewDegradeReason::LocalActionsHoverOnly))
        || lists()
            .any(|ex| ex.degrade_reason == Some(M5ListViewDegradeReason::LocalActionsHoverOnly));
    let no_clean_hover_only = trees().all(|ex| {
        !(ex.is_clean()
            && (ex.current_selection_hover_only
                || ex.blocked_state_hover_only
                || ex.local_actions_hover_only))
    }) && lists().all(|ex| {
        !(ex.is_clean()
            && (ex.current_selection_hover_only
                || ex.blocked_state_hover_only
                || ex.local_actions_hover_only))
    });
    if !(current_hover_degrades
        && blocked_hover_degrades
        && actions_hover_degrades
        && no_clean_hover_only)
    {
        violations.push(M5TreeListControlsViolation::NoHoverOnlyDiscoveryNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5NavigationContentComponentFamily; 2] = [
    M5NavigationContentComponentFamily::TreeView,
    M5NavigationContentComponentFamily::ListView,
];
