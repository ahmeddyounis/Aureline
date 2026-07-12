//! Frozen M5 tab-strip, breadcrumbs, tree-view, list-view, table/grid, and panel-header
//! navigation/content component matrix.
//!
//! This module locks Aureline's reusable navigation and dense-content UI components into one
//! export-safe packet. Every claimed M5 surface that still ships its own tab set, breadcrumb trail,
//! explorer tree, result list, dense table/grid, or panel header — across the shell, explorer,
//! search, review, request/data, help, and support surfaces — is named once here and constrained by
//! the same active-context, hierarchy/path, disclosure, selection-versus-current, item-state
//! (pinned/preview/modified/read-only/blocked), count-scope (exact/loaded/all-matching/hidden),
//! density, and local-action-budget vocabulary regardless of the surface family that renders it.
//!
//! The matrix does not re-architect the search index, the collection query engine, or any data
//! backend — it is the shared navigation/content-honesty component contract layered on top of them.
//! The controlled vocabularies are frozen in one self-describing
//! [`M5NavigationContentVocabularySet`] rather than minted per surface. The single controlled
//! navigation/content-disposition vocabulary consumers bind to — preview, pinned, modified,
//! read-only, blocked, exact-count, loaded-count, all-matching-count, hidden-by-filter,
//! hidden-by-policy, overflowed-local-action, and stale-or-partial-hierarchy — keeps tabs from
//! masquerading as top-level workflow navigation, keeps counts and blocked rows from hiding behind
//! ambiguous ellipses, keeps tree/list/table actions from being hover-only, keeps panel headers from
//! becoming cluttered secondary toolbars, and keeps exact, loaded, and all-matching scopes from
//! collapsing into one vague total. Raw secret values and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_navigation_content_component_matrix,
    seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed,
    seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed,
    M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5NavigationContentComponentMatrixPacket`].
pub const M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix";

/// Schema version for M5 navigation-content component-matrix records.
pub const M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined navigation-content component-matrix schema.
pub const M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-navigation-content-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF: &str =
    "docs/navigation/m5_navigation_content_components_contract.md";

/// Repo-relative path of the tab-strip canonical component schema.
pub const M5_TAB_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-tab-strip.schema.json";

/// Repo-relative path of the breadcrumbs canonical component schema.
pub const M5_BREADCRUMBS_SCHEMA_REF: &str = "schemas/ui/m5-breadcrumbs.schema.json";

/// Repo-relative path of the tree-view canonical component schema.
pub const M5_TREE_VIEW_SCHEMA_REF: &str = "schemas/ui/m5-tree-view.schema.json";

/// Repo-relative path of the list-view canonical component schema.
pub const M5_LIST_VIEW_SCHEMA_REF: &str = "schemas/ui/m5-list-view.schema.json";

/// Repo-relative path of the table/grid canonical component schema.
pub const M5_TABLE_GRID_SCHEMA_REF: &str = "schemas/ui/m5-table-grid.schema.json";

/// Repo-relative path of the panel-header canonical component schema.
pub const M5_PANEL_HEADER_SCHEMA_REF: &str = "schemas/ui/m5-panel-header.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_NAVIGATION_CONTENT_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-navigation-content-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NAVIGATION_CONTENT_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-navigation-content-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_NAVIGATION_CONTENT_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-navigation-content-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_NAVIGATION_CONTENT_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-navigation-content-component-matrix.md";

/// One of the six governed navigation / content component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentComponentFamily {
    /// A tab strip naming the active context, per-tab item state, and overflow for a set of open
    /// contexts.
    TabStrip,
    /// A breadcrumb trail naming the hierarchy / path to the current object, including truncated,
    /// stale, or partial hierarchy.
    Breadcrumbs,
    /// A tree view naming hierarchy, disclosure state, selection-versus-current, item state, counts,
    /// density, and local actions.
    TreeView,
    /// A list view naming selection-versus-current, item state, exact / loaded / all-matching /
    /// hidden counts, density, and local actions.
    ListView,
    /// A table / grid naming selection, counts, density, item state, and local actions across a
    /// dense structure.
    TableGrid,
    /// A panel header naming the active context and a bounded local-action budget without becoming a
    /// secondary toolbar.
    PanelHeader,
}

impl M5NavigationContentComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TabStrip,
        Self::Breadcrumbs,
        Self::TreeView,
        Self::ListView,
        Self::TableGrid,
        Self::PanelHeader,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabStrip => "tab_strip",
            Self::Breadcrumbs => "breadcrumbs",
            Self::TreeView => "tree_view",
            Self::ListView => "list_view",
            Self::TableGrid => "table_grid",
            Self::PanelHeader => "panel_header",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating this
    /// component's navigation / content truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::TabStrip => M5_TAB_STRIP_SCHEMA_REF,
            Self::Breadcrumbs => M5_BREADCRUMBS_SCHEMA_REF,
            Self::TreeView => M5_TREE_VIEW_SCHEMA_REF,
            Self::ListView => M5_LIST_VIEW_SCHEMA_REF,
            Self::TableGrid => M5_TABLE_GRID_SCHEMA_REF,
            Self::PanelHeader => M5_PANEL_HEADER_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled active-context state.
    pub const fn declares_active_context(self) -> bool {
        matches!(self, Self::TabStrip | Self::PanelHeader)
    }

    /// `true` when this family must name a controlled hierarchy / path state.
    pub const fn declares_hierarchy_path(self) -> bool {
        matches!(self, Self::Breadcrumbs | Self::TreeView)
    }

    /// `true` when this family must name a controlled disclosure state.
    pub const fn declares_disclosure(self) -> bool {
        matches!(self, Self::TreeView)
    }

    /// `true` when this family must name a controlled selection state.
    pub const fn declares_selection(self) -> bool {
        matches!(self, Self::TreeView | Self::ListView | Self::TableGrid)
    }

    /// `true` when this family must name a controlled count scope.
    pub const fn declares_count_scope(self) -> bool {
        matches!(self, Self::TreeView | Self::ListView | Self::TableGrid)
    }

    /// `true` when this family must name a controlled item-state flag.
    pub const fn declares_item_state(self) -> bool {
        matches!(
            self,
            Self::TabStrip | Self::TreeView | Self::ListView | Self::TableGrid
        )
    }

    /// `true` when this family must name a controlled density variant.
    pub const fn declares_density(self) -> bool {
        matches!(self, Self::TreeView | Self::ListView | Self::TableGrid)
    }

    /// `true` when this family must name a controlled local-action budget.
    pub const fn declares_local_action_budget(self) -> bool {
        matches!(
            self,
            Self::TabStrip | Self::TreeView | Self::ListView | Self::TableGrid | Self::PanelHeader
        )
    }
}

/// The single controlled navigation / content-disposition vocabulary every shell, explorer, search,
/// review, request/data, help, or support consumer binds to. These are the exact acceptance-criteria
/// tokens that keep tabs from masquerading as workflow navigation, keep counts and blocked rows from
/// hiding behind ambiguous ellipses, keep local actions from being hover-only, and keep exact,
/// loaded, and all-matching scopes distinct. No navigation or content surface invents a parallel
/// word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentDisposition {
    /// The item is a preview (single-click, not yet pinned open).
    Preview,
    /// The item is pinned open.
    Pinned,
    /// The item has unsaved modifications.
    Modified,
    /// The item is read-only.
    ReadOnly,
    /// The item is blocked (by policy, error, or precondition).
    Blocked,
    /// A count reflects the exact total.
    ExactCount,
    /// A count reflects only the loaded subset.
    LoadedCount,
    /// A count reflects all matching items regardless of what is loaded.
    AllMatchingCount,
    /// Items are hidden by an active filter.
    HiddenByFilter,
    /// Items are hidden by policy.
    HiddenByPolicy,
    /// A local action was pushed into an overflow menu.
    OverflowedLocalAction,
    /// The hierarchy shown is stale or partial.
    StaleOrPartialHierarchy,
}

impl M5NavigationContentDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Preview,
        Self::Pinned,
        Self::Modified,
        Self::ReadOnly,
        Self::Blocked,
        Self::ExactCount,
        Self::LoadedCount,
        Self::AllMatchingCount,
        Self::HiddenByFilter,
        Self::HiddenByPolicy,
        Self::OverflowedLocalAction,
        Self::StaleOrPartialHierarchy,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Pinned => "pinned",
            Self::Modified => "modified",
            Self::ReadOnly => "read_only",
            Self::Blocked => "blocked",
            Self::ExactCount => "exact_count",
            Self::LoadedCount => "loaded_count",
            Self::AllMatchingCount => "all_matching_count",
            Self::HiddenByFilter => "hidden_by_filter",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::OverflowedLocalAction => "overflowed_local_action",
            Self::StaleOrPartialHierarchy => "stale_or_partial_hierarchy",
        }
    }

    /// Whether this disposition names a count scope that must never be collapsed with the others.
    pub const fn is_count_scope(self) -> bool {
        matches!(
            self,
            Self::ExactCount | Self::LoadedCount | Self::AllMatchingCount
        )
    }
}

/// Controlled active-context state — which context is current versus merely open, so a background or
/// preview context is never presented as the active one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActiveContextState {
    /// The current, focused context.
    ActiveCurrent,
    /// The active context, pinned open.
    ActivePinned,
    /// The active context, still a preview.
    ActivePreview,
    /// An open background context.
    BackgroundOpen,
    /// An open background context with unsaved modifications.
    BackgroundModified,
    /// The active context cannot currently be resolved.
    ContextUnresolved,
}

impl M5ActiveContextState {
    /// Every active-context state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActiveCurrent,
        Self::ActivePinned,
        Self::ActivePreview,
        Self::BackgroundOpen,
        Self::BackgroundModified,
        Self::ContextUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveCurrent => "active_current",
            Self::ActivePinned => "active_pinned",
            Self::ActivePreview => "active_preview",
            Self::BackgroundOpen => "background_open",
            Self::BackgroundModified => "background_modified",
            Self::ContextUnresolved => "context_unresolved",
        }
    }
}

/// Controlled hierarchy / path state — how the path to the current object is shown, so a truncated,
/// stale, or partial hierarchy is never presented as a complete path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HierarchyPathState {
    /// The full path from root is shown.
    FullPathShown,
    /// The path is shown relative to a pinned root.
    RootRelative,
    /// The middle of the path is truncated for width.
    TruncatedMiddle,
    /// The shown hierarchy is stale relative to the source of truth.
    StaleHierarchy,
    /// Only a partial hierarchy could be resolved.
    PartialHierarchy,
    /// The path cannot currently be resolved.
    PathUnresolved,
}

impl M5HierarchyPathState {
    /// Every hierarchy / path state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullPathShown,
        Self::RootRelative,
        Self::TruncatedMiddle,
        Self::StaleHierarchy,
        Self::PartialHierarchy,
        Self::PathUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullPathShown => "full_path_shown",
            Self::RootRelative => "root_relative",
            Self::TruncatedMiddle => "truncated_middle",
            Self::StaleHierarchy => "stale_hierarchy",
            Self::PartialHierarchy => "partial_hierarchy",
            Self::PathUnresolved => "path_unresolved",
        }
    }
}

/// Controlled disclosure state — whether a node is expanded, collapsed, or partially loaded, so a
/// lazily-unloaded subtree is never presented as an empty leaf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisclosureState {
    /// Fully expanded.
    Expanded,
    /// Collapsed.
    Collapsed,
    /// Partially expanded (some children loaded).
    PartiallyExpanded,
    /// A leaf node with no children.
    LeafNoChildren,
    /// Children exist but are not yet loaded.
    LazyUnloaded,
    /// The disclosure state cannot currently be resolved.
    DisclosureUnknown,
}

impl M5DisclosureState {
    /// Every disclosure state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Expanded,
        Self::Collapsed,
        Self::PartiallyExpanded,
        Self::LeafNoChildren,
        Self::LazyUnloaded,
        Self::DisclosureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Expanded => "expanded",
            Self::Collapsed => "collapsed",
            Self::PartiallyExpanded => "partially_expanded",
            Self::LeafNoChildren => "leaf_no_children",
            Self::LazyUnloaded => "lazy_unloaded",
            Self::DisclosureUnknown => "disclosure_unknown",
        }
    }
}

/// Controlled selection state — selection versus the current / focused item, so a multi-selection is
/// never confused with the single current row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionState {
    /// A single item is selected.
    SingleSelected,
    /// Multiple items are selected.
    MultiSelected,
    /// The current / focused item is not part of the selection.
    CurrentNotSelected,
    /// The current item is also selected.
    SelectedAndCurrent,
    /// Nothing is selected.
    NoneSelected,
    /// The selection state cannot currently be resolved.
    SelectionUnknown,
}

impl M5SelectionState {
    /// Every selection state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleSelected,
        Self::MultiSelected,
        Self::CurrentNotSelected,
        Self::SelectedAndCurrent,
        Self::NoneSelected,
        Self::SelectionUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSelected => "single_selected",
            Self::MultiSelected => "multi_selected",
            Self::CurrentNotSelected => "current_not_selected",
            Self::SelectedAndCurrent => "selected_and_current",
            Self::NoneSelected => "none_selected",
            Self::SelectionUnknown => "selection_unknown",
        }
    }
}

/// Controlled count scope — what a shown count actually measures, so exact, loaded, and all-matching
/// scopes are never collapsed into one vague total and hidden rows are never silently dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CountScope {
    /// The exact total.
    ExactCount,
    /// Only the currently loaded subset.
    LoadedCount,
    /// All matching items regardless of what is loaded.
    AllMatchingCount,
    /// The number of items hidden by an active filter.
    HiddenByFilterCount,
    /// The number of items hidden by policy.
    HiddenByPolicyCount,
    /// The count cannot currently be resolved.
    CountUnresolved,
}

impl M5CountScope {
    /// Every count scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactCount,
        Self::LoadedCount,
        Self::AllMatchingCount,
        Self::HiddenByFilterCount,
        Self::HiddenByPolicyCount,
        Self::CountUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCount => "exact_count",
            Self::LoadedCount => "loaded_count",
            Self::AllMatchingCount => "all_matching_count",
            Self::HiddenByFilterCount => "hidden_by_filter_count",
            Self::HiddenByPolicyCount => "hidden_by_policy_count",
            Self::CountUnresolved => "count_unresolved",
        }
    }
}

/// Controlled item-state flag — the pinned / preview / modified / read-only / blocked state of a
/// single tab, row, or node, so item state is never encoded by color or hover alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ItemStateFlag {
    /// The item is pinned.
    Pinned,
    /// The item is a preview.
    Preview,
    /// The item has unsaved modifications.
    Modified,
    /// The item is read-only.
    ReadOnly,
    /// The item is blocked.
    Blocked,
    /// The item state cannot currently be resolved.
    StateUnknown,
}

impl M5ItemStateFlag {
    /// Every item-state flag, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Pinned,
        Self::Preview,
        Self::Modified,
        Self::ReadOnly,
        Self::Blocked,
        Self::StateUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Preview => "preview",
            Self::Modified => "modified",
            Self::ReadOnly => "read_only",
            Self::Blocked => "blocked",
            Self::StateUnknown => "state_unknown",
        }
    }
}

/// Controlled density variant — the density a dense-content component renders at, so a condensed or
/// overflowed layout is never mistaken for a complete comfortable one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityVariant {
    /// Comfortable density.
    Comfortable,
    /// Compact density.
    Compact,
    /// Dense density.
    Dense,
    /// Condensed with overflow affordances.
    CondensedOverflow,
    /// A single-line collapsed layout.
    SingleLine,
    /// The density cannot currently be resolved.
    DensityUnknown,
}

impl M5DensityVariant {
    /// Every density variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Comfortable,
        Self::Compact,
        Self::Dense,
        Self::CondensedOverflow,
        Self::SingleLine,
        Self::DensityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
            Self::Dense => "dense",
            Self::CondensedOverflow => "condensed_overflow",
            Self::SingleLine => "single_line",
            Self::DensityUnknown => "density_unknown",
        }
    }
}

/// Controlled local-action budget — how many pane-local actions a component exposes before spilling
/// into overflow, so an overflowed action is never dropped and a header never becomes a toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalActionBudget {
    /// No local actions.
    NoLocalActions,
    /// Local actions within the visible budget.
    WithinBudget,
    /// A primary action plus an overflow menu.
    PrimaryPlusOverflow,
    /// Actions spilled into an overflow menu.
    OverflowedMenu,
    /// All actions overflowed off the visible surface.
    AllOverflowed,
    /// The local-action budget cannot currently be resolved.
    BudgetUnknown,
}

impl M5LocalActionBudget {
    /// Every local-action budget, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoLocalActions,
        Self::WithinBudget,
        Self::PrimaryPlusOverflow,
        Self::OverflowedMenu,
        Self::AllOverflowed,
        Self::BudgetUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLocalActions => "no_local_actions",
            Self::WithinBudget => "within_budget",
            Self::PrimaryPlusOverflow => "primary_plus_overflow",
            Self::OverflowedMenu => "overflowed_menu",
            Self::AllOverflowed => "all_overflowed",
            Self::BudgetUnknown => "budget_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a navigation-content component. No component may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentSurfaceFamily {
    /// The shell workspace.
    ShellWorkspace,
    /// The explorer.
    Explorer,
    /// The search-results surface.
    SearchResults,
    /// The review surface.
    Review,
    /// The request / data surface.
    RequestData,
    /// The help center.
    HelpCenter,
    /// The support export.
    SupportExport,
}

impl M5NavigationContentSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ShellWorkspace,
        Self::Explorer,
        Self::SearchResults,
        Self::Review,
        Self::RequestData,
        Self::HelpCenter,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellWorkspace => "shell_workspace",
            Self::Explorer => "explorer",
            Self::SearchResults => "search_results",
            Self::Review => "review",
            Self::RequestData => "request_data",
            Self::HelpCenter => "help_center",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's active-context,
/// hierarchy, count, selection, or overflow truth never silently narrows or widens between deployment
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5NavigationContentDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The explorer UI.
    ExplorerUi,
    /// The search UI.
    SearchUi,
    /// The review UI.
    ReviewUi,
    /// The request / data UI.
    DataUi,
    /// The help UI.
    HelpUi,
    /// The AI context surface.
    AiContextUi,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5NavigationContentConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::ExplorerUi,
        Self::SearchUi,
        Self::ReviewUi,
        Self::DataUi,
        Self::HelpUi,
        Self::AiContextUi,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::ExplorerUi => "explorer_ui",
            Self::SearchUi => "search_ui",
            Self::ReviewUi => "review_ui",
            Self::DataUi => "data_ui",
            Self::HelpUi => "help_ui",
            Self::AiContextUi => "ai_context_ui",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no navigation or content truth is
/// hover-only, pointer-only, motion-only, or visually encoded alone. Records the keyboard,
/// screen-reader, high-zoom, reduced-motion, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never menu-only.
    SupportPacketPresent,
}

impl M5NavigationContentAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a navigation-content component has degraded below its qualified state. Required on every
/// row so a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The hierarchy / path signal is unavailable.
    HierarchySignalUnavailable,
    /// The count signal is unavailable.
    CountSignalUnavailable,
    /// The selection state is unavailable.
    SelectionStateUnavailable,
    /// The local-action / overflow state is unavailable.
    OverflowStateUnavailable,
    /// The active-context signal is unavailable.
    ActiveContextUnavailable,
}

impl M5NavigationContentDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::HierarchySignalUnavailable,
        Self::CountSignalUnavailable,
        Self::SelectionStateUnavailable,
        Self::OverflowStateUnavailable,
        Self::ActiveContextUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::HierarchySignalUnavailable => "hierarchy_signal_unavailable",
            Self::CountSignalUnavailable => "count_signal_unavailable",
            Self::SelectionStateUnavailable => "selection_state_unavailable",
            Self::OverflowStateUnavailable => "overflow_state_unavailable",
            Self::ActiveContextUnavailable => "active_context_unavailable",
        }
    }
}

/// Mandatory label a claimed navigation-content component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria ambiguity
/// about active context / hierarchy, count / scope, and selection / item state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The active context and hierarchy / path behind the component.
    ActiveContextAndHierarchy,
    /// The count and its scope (exact / loaded / all-matching / hidden) behind the component.
    CountAndScope,
    /// The selection-versus-current and item state behind the component.
    SelectionAndItemState,
}

impl M5NavigationContentRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ActiveContextAndHierarchy,
        Self::CountAndScope,
        Self::SelectionAndItemState,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ActiveContextAndHierarchy => "active_context_and_hierarchy",
            Self::CountAndScope => "count_and_scope",
            Self::SelectionAndItemState => "selection_and_item_state",
        }
    }
}

/// Qualification class for an M5 navigation-content component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5NavigationContentQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a navigation-content component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavigationContentDowngradeTrigger {
    /// A tab strip presented itself as top-level workflow navigation.
    TabsMasqueradeAsWorkflowNav,
    /// A component left its active context unstated.
    ActiveContextUnstated,
    /// A component left its hierarchy / path unstated.
    HierarchyPathUnstated,
    /// A component hid its disclosure state.
    DisclosureStateHidden,
    /// A component collapsed selection versus the current item.
    SelectionVersusCurrentCollapsed,
    /// A component collapsed exact, loaded, and all-matching count scopes.
    CountScopeCollapsed,
    /// A component hid blocked rows behind an ambiguous ellipsis.
    BlockedRowsHiddenBehindEllipsis,
    /// A component omitted the hidden-by-policy count.
    HiddenByPolicyCountOmitted,
    /// A component made local actions hover-only.
    LocalActionsHoverOnly,
    /// A panel header became an overloaded secondary toolbar.
    PanelHeaderOverloaded,
    /// Generic chrome wording concealed navigation or content truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5NavigationContentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TabsMasqueradeAsWorkflowNav,
        Self::ActiveContextUnstated,
        Self::HierarchyPathUnstated,
        Self::DisclosureStateHidden,
        Self::SelectionVersusCurrentCollapsed,
        Self::CountScopeCollapsed,
        Self::BlockedRowsHiddenBehindEllipsis,
        Self::HiddenByPolicyCountOmitted,
        Self::LocalActionsHoverOnly,
        Self::PanelHeaderOverloaded,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabsMasqueradeAsWorkflowNav => "tabs_masquerade_as_workflow_nav",
            Self::ActiveContextUnstated => "active_context_unstated",
            Self::HierarchyPathUnstated => "hierarchy_path_unstated",
            Self::DisclosureStateHidden => "disclosure_state_hidden",
            Self::SelectionVersusCurrentCollapsed => "selection_versus_current_collapsed",
            Self::CountScopeCollapsed => "count_scope_collapsed",
            Self::BlockedRowsHiddenBehindEllipsis => "blocked_rows_hidden_behind_ellipsis",
            Self::HiddenByPolicyCountOmitted => "hidden_by_policy_count_omitted",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::PanelHeaderOverloaded => "panel_header_overloaded",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed navigation-content component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentComponentRow {
    /// Governed component family.
    pub component_family: M5NavigationContentComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5NavigationContentQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5NavigationContentSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5NavigationContentDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5NavigationContentRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5NavigationContentRequiredLabel>,
    /// Navigation / content dispositions this component can carry (the frozen AC vocabulary;
    /// required on every component).
    pub dispositions: Vec<M5NavigationContentDisposition>,
    /// Active-context states this component names (active-context-bearing families only).
    pub active_context_states: Vec<M5ActiveContextState>,
    /// Hierarchy / path states this component names (hierarchy-bearing families only).
    pub hierarchy_path_states: Vec<M5HierarchyPathState>,
    /// Disclosure states this component names (disclosure-bearing families only).
    pub disclosure_states: Vec<M5DisclosureState>,
    /// Selection states this component names (selection-bearing families only).
    pub selection_states: Vec<M5SelectionState>,
    /// Count scopes this component names (count-bearing families only).
    pub count_scopes: Vec<M5CountScope>,
    /// Item-state flags this component names (item-state-bearing families only).
    pub item_state_flags: Vec<M5ItemStateFlag>,
    /// Density variants this component names (density-bearing families only).
    pub density_variants: Vec<M5DensityVariant>,
    /// Local-action budgets this component names (action-bearing families only).
    pub local_action_budgets: Vec<M5LocalActionBudget>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5NavigationContentDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5NavigationContentAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5NavigationContentConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this tab component never presents itself as top-level workflow navigation.
    /// MUST be `false`.
    pub tabs_masquerade_as_top_level_workflow_navigation: bool,
    /// Hard invariant: this component never hides counts or blocked rows behind an ambiguous
    /// ellipsis. MUST be `false`.
    pub hides_counts_or_blocked_rows_behind_ambiguous_ellipsis: bool,
    /// Hard invariant: this component never makes tree / list / table actions hover-only. MUST be
    /// `false`.
    pub makes_tree_list_or_table_actions_hover_only: bool,
    /// Hard invariant: this panel header never becomes a cluttered secondary toolbar. MUST be
    /// `false`.
    pub panel_header_becomes_cluttered_secondary_toolbar: bool,
    /// Hard invariant: this component never collapses exact, loaded, and all-matching scopes into one
    /// vague total. MUST be `false`.
    pub collapses_exact_loaded_and_all_matching_scopes_into_one_total: bool,
}

impl M5NavigationContentComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NavigationContentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NavigationContentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.tabs_masquerade_as_top_level_workflow_navigation
            && !self.hides_counts_or_blocked_rows_behind_ambiguous_ellipsis
            && !self.makes_tree_list_or_table_actions_hover_only
            && !self.panel_header_becomes_cluttered_secondary_toolbar
            && !self.collapses_exact_loaded_and_all_matching_scopes_into_one_total
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Navigation / content-disposition tokens.
    pub dispositions: Vec<String>,
    /// Active-context-state tokens.
    pub active_context_states: Vec<String>,
    /// Hierarchy / path-state tokens.
    pub hierarchy_path_states: Vec<String>,
    /// Disclosure-state tokens.
    pub disclosure_states: Vec<String>,
    /// Selection-state tokens.
    pub selection_states: Vec<String>,
    /// Count-scope tokens.
    pub count_scopes: Vec<String>,
    /// Item-state-flag tokens.
    pub item_state_flags: Vec<String>,
    /// Density-variant tokens.
    pub density_variants: Vec<String>,
    /// Local-action-budget tokens.
    pub local_action_budgets: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5NavigationContentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5NavigationContentComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5NavigationContentDisposition::ALL, |v| v.as_str()),
            active_context_states: tokens(&M5ActiveContextState::ALL, |v| v.as_str()),
            hierarchy_path_states: tokens(&M5HierarchyPathState::ALL, |v| v.as_str()),
            disclosure_states: tokens(&M5DisclosureState::ALL, |v| v.as_str()),
            selection_states: tokens(&M5SelectionState::ALL, |v| v.as_str()),
            count_scopes: tokens(&M5CountScope::ALL, |v| v.as_str()),
            item_state_flags: tokens(&M5ItemStateFlag::ALL, |v| v.as_str()),
            density_variants: tokens(&M5DensityVariant::ALL, |v| v.as_str()),
            local_action_budgets: tokens(&M5LocalActionBudget::ALL, |v| v.as_str()),
            surface_families: tokens(&M5NavigationContentSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5NavigationContentDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5NavigationContentConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5NavigationContentAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5NavigationContentDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5NavigationContentRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5NavigationContentDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5NavigationContentGovernanceReview {
    /// The tab strip shows the active context and overflow.
    pub tab_strip_shows_active_context_and_overflow: bool,
    /// The breadcrumbs show the full or honestly truncated hierarchy.
    pub breadcrumbs_show_full_or_truncated_hierarchy: bool,
    /// The tree view shows disclosure state and selection versus current.
    pub tree_view_shows_disclosure_and_selection: bool,
    /// The list view shows count scopes and hidden scopes.
    pub list_view_shows_counts_and_hidden_scopes: bool,
    /// The table / grid shows count scopes and density.
    pub table_grid_shows_counts_and_density: bool,
    /// The panel header shows the active context and a bounded action set.
    pub panel_header_shows_context_and_bounded_actions: bool,
    /// Tabs never masquerade as top-level workflow navigation.
    pub tabs_never_masquerade_as_workflow_navigation: bool,
    /// Exact, loaded, and all-matching counts are never collapsed into one total.
    pub counts_never_collapsed_into_one_total: bool,
    /// Blocked rows are never hidden behind an ambiguous ellipsis.
    pub blocked_rows_never_hidden_behind_ellipsis: bool,
    /// Hidden-by-filter and hidden-by-policy counts stay distinct.
    pub hidden_by_filter_and_policy_always_distinct: bool,
    /// Tree / list / table actions are never hover-only.
    pub local_actions_never_hover_only: bool,
    /// Panel headers never become secondary toolbars.
    pub panel_headers_never_become_secondary_toolbars: bool,
    /// Stale or partial hierarchy is always named.
    pub stale_or_partial_hierarchy_always_named: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel navigation / content vocabulary.
    pub later_rows_cannot_invent_parallel_navigation_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentConsumerProjection {
    /// Shell surfaces consume the shared active-context vocabulary.
    pub shell_surfaces_consume_active_context_vocabulary: bool,
    /// The explorer consumes the shared hierarchy and disclosure vocabulary.
    pub explorer_consumes_hierarchy_and_disclosure_vocabulary: bool,
    /// Search consumes the shared count-scope vocabulary.
    pub search_consumes_count_scope_vocabulary: bool,
    /// Review consumes the shared selection and item-state vocabulary.
    pub review_consumes_selection_and_item_state_vocabulary: bool,
    /// Help consumes the shared navigation vocabulary.
    pub help_consumes_navigation_vocabulary: bool,
    /// Support / export reads a single canonical navigation / content source.
    pub support_export_reads_single_navigation_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the navigation-content component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting navigation / content component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5NavigationContentComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NavigationContentComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5NavigationContentComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NavigationContentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NavigationContentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NavigationContentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NavigationContentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NavigationContentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 navigation-content component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NavigationContentComponentMatrixPacket {
    /// Record kind; must equal [`M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5NavigationContentComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NavigationContentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NavigationContentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NavigationContentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NavigationContentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NavigationContentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5NavigationContentComponentMatrixPacket {
    /// Builds an M5 navigation-content component matrix packet from stable-lane input.
    pub fn new(input: M5NavigationContentComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 navigation-content component matrix invariants.
    pub fn validate(&self) -> Vec<M5NavigationContentComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5NavigationContentComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5NavigationContentComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5NavigationContentComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 navigation-content component matrix serializes"),
        ) {
            violations.push(M5NavigationContentComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 navigation-content component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Tab-Strip, Breadcrumbs, Tree-View, List-View, Table/Grid, and Panel-Header Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Navigation / content dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Count scopes: {}\n",
            self.vocabulary_set.count_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 navigation-content matrix export.
#[derive(Debug)]
pub enum M5NavigationContentComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5NavigationContentComponentMatrixViolation>),
}

impl fmt::Display for M5NavigationContentComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 navigation-content component matrix export parse failed: {error}"
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
                    "m5 navigation-content component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5NavigationContentComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5NavigationContentComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5NavigationContentComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no navigation / content dispositions.
    DispositionMissing,
    /// An active-context-bearing component declares no active-context states.
    ActiveContextMissing,
    /// A hierarchy-bearing component declares no hierarchy / path states.
    HierarchyPathMissing,
    /// A disclosure-bearing component declares no disclosure states.
    DisclosureMissing,
    /// A selection-bearing component declares no selection states.
    SelectionMissing,
    /// A count-bearing component declares no count scopes.
    CountScopeMissing,
    /// An item-state-bearing component declares no item-state flags.
    ItemStateMissing,
    /// A density-bearing component declares no density variants.
    DensityMissing,
    /// An action-bearing component declares no local-action budgets.
    LocalActionBudgetMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (tabs masquerade as workflow navigation, hides counts or
    /// blocked rows behind an ellipsis, makes actions hover-only, overloads a panel header, or
    /// collapses exact / loaded / all-matching scopes).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5NavigationContentComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::ActiveContextMissing => "active_context_missing",
            Self::HierarchyPathMissing => "hierarchy_path_missing",
            Self::DisclosureMissing => "disclosure_missing",
            Self::SelectionMissing => "selection_missing",
            Self::CountScopeMissing => "count_scope_missing",
            Self::ItemStateMissing => "item_state_missing",
            Self::DensityMissing => "density_missing",
            Self::LocalActionBudgetMissing => "local_action_budget_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 navigation-content matrix export.
pub fn current_stable_m5_navigation_content_component_matrix_export(
) -> Result<M5NavigationContentComponentMatrixPacket, M5NavigationContentComponentMatrixArtifactError>
{
    let packet: M5NavigationContentComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-navigation-content-proof/support_export.json"
        )))
        .map_err(M5NavigationContentComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5NavigationContentComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TAB_STRIP_SCHEMA_REF,
        M5_BREADCRUMBS_SCHEMA_REF,
        M5_TREE_VIEW_SCHEMA_REF,
        M5_LIST_VIEW_SCHEMA_REF,
        M5_TABLE_GRID_SCHEMA_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5NavigationContentComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5NavigationContentComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    let present: BTreeSet<M5NavigationContentComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5NavigationContentComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5NavigationContentComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5NavigationContentComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5NavigationContentComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5NavigationContentComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_active_context() && row.active_context_states.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::ActiveContextMissing);
        }
        if family.declares_hierarchy_path() && row.hierarchy_path_states.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::HierarchyPathMissing);
        }
        if family.declares_disclosure() && row.disclosure_states.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DisclosureMissing);
        }
        if family.declares_selection() && row.selection_states.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::SelectionMissing);
        }
        if family.declares_count_scope() && row.count_scopes.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::CountScopeMissing);
        }
        if family.declares_item_state() && row.item_state_flags.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::ItemStateMissing);
        }
        if family.declares_density() && row.density_variants.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DensityMissing);
        }
        if family.declares_local_action_budget() && row.local_action_budgets.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::LocalActionBudgetMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5NavigationContentComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5NavigationContentComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tab_strip_shows_active_context_and_overflow,
        review.breadcrumbs_show_full_or_truncated_hierarchy,
        review.tree_view_shows_disclosure_and_selection,
        review.list_view_shows_counts_and_hidden_scopes,
        review.table_grid_shows_counts_and_density,
        review.panel_header_shows_context_and_bounded_actions,
        review.tabs_never_masquerade_as_workflow_navigation,
        review.counts_never_collapsed_into_one_total,
        review.blocked_rows_never_hidden_behind_ellipsis,
        review.hidden_by_filter_and_policy_always_distinct,
        review.local_actions_never_hover_only,
        review.panel_headers_never_become_secondary_toolbars,
        review.stale_or_partial_hierarchy_always_named,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_navigation_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5NavigationContentComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_active_context_vocabulary,
        projection.explorer_consumes_hierarchy_and_disclosure_vocabulary,
        projection.search_consumes_count_scope_vocabulary,
        projection.review_consumes_selection_and_item_state_vocabulary,
        projection.help_consumes_navigation_vocabulary,
        projection.support_export_reads_single_navigation_source,
    ] {
        if !ok {
            violations
                .push(M5NavigationContentComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5NavigationContentComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5NavigationContentComponentMatrixPacket,
    violations: &mut Vec<M5NavigationContentComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5NavigationContentComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses navigation / content words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
