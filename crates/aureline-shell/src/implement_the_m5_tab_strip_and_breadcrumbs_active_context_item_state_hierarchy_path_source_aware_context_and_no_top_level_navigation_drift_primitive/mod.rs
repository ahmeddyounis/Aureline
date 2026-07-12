//! Implemented M5 tab-strip and breadcrumbs primitives.
//!
//! The frozen [navigation / content component matrix][matrix] names the reusable navigation and
//! dense-content UI components and locks their controlled vocabulary. This module is the first
//! implement lane over that matrix: it turns the two active-context / local-structure components —
//! the **tab strip** and the **breadcrumb trail** — into resolvers that produce export-safe, honest
//! projections, so a user can read which context is active and trace the local path / ancestry to
//! the current object without either component quietly acting like top-level workflow navigation.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render tabs with preview, pinned, modified, read-only, blocked, shared, and reopened state
//!   using one controlled vocabulary and no-color-only semantics.** [`resolve_tab_strip`] refuses to
//!   read as a clean, context-legible strip when the active context is unstated or unresolved, the
//!   tab reads as top-level workflow navigation, a surface-local badge is invented for a shared
//!   context, an item state is unresolved or encoded by color alone, a blocked tab is hidden behind
//!   an ambiguous ellipsis, or no command-backed path to trace the context is reachable; it degrades
//!   instead.
//! * **Render breadcrumbs with stable root/path or symbol ancestry, truncation or overflow
//!   behavior, and source-aware path context that stays honest under partial, stale, or
//!   policy-limited hierarchy.** [`resolve_breadcrumbs`] degrades when the leaf identity or ancestry
//!   kind is unstated, the path is unresolved, the trail reads as top-level navigation, missing
//!   scope collapses into an ambiguous ellipsis, a partial or stale hierarchy is presented as a
//!   complete path, or the path is not explicit in both compact and expanded views.
//! * **Prevent tabs or breadcrumbs from quietly acting like top-level navigation.** The packet
//!   proves, by resolved examples, that a masquerade tab and a masquerade breadcrumb both degrade,
//!   that no clean example masquerades, and that a user can trace current context and local ancestry
//!   through one canonical component contract and its command-backed detail entrypoints.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5NavigationContentDisposition`] navigation / content-disposition vocabulary, the
//! [`M5ActiveContextState`] active-context vocabulary, the [`M5HierarchyPathState`] hierarchy / path
//! vocabulary, and the [`M5LocalActionBudget`] local-action-budget vocabulary — so shell, explorer,
//! search, review, help, and support surfaces can never fork their own active-context, hierarchy, or
//! item-state wording or invent surface-local badges for the same context. Raw secret values and
//! private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_tab_strip_breadcrumbs_controls,
    seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed,
    seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed,
    M5_TAB_STRIP_BREADCRUMBS_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    M5ActiveContextState, M5HierarchyPathState, M5LocalActionBudget,
    M5NavigationContentAccessibilityRoute, M5NavigationContentComponentFamily,
    M5NavigationContentConsumerSurface, M5NavigationContentDeploymentLine,
    M5NavigationContentDisposition, M5NavigationContentDowngradeTrigger,
    M5NavigationContentQualificationClass, M5NavigationContentRequiredLabel,
    M5_BREADCRUMBS_SCHEMA_REF, M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
    M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_TAB_STRIP_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TabStripBreadcrumbsControlsPacket`].
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_RECORD_KIND: &str =
    "implement_m5_tab_strip_and_breadcrumbs_controls";

/// Schema version for M5 tab-strip / breadcrumbs controls records.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-tab-strip-breadcrumbs-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_DOC_REF: &str =
    "docs/navigation/m5_tab_strip_and_breadcrumbs_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-tab-strip-breadcrumbs-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5TabBreadcrumbsConsumerSurface = M5NavigationContentConsumerSurface;

/// Controlled per-tab item state a tab strip names with no-color-only semantics. Minted by this lane
/// because the frozen matrix item-state vocabulary carries pinned / preview / modified / read-only /
/// blocked but not the **shared** and **reopened** states the tab-strip acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TabItemState {
    /// The tab is pinned open.
    Pinned,
    /// The tab is a single-click preview, not yet pinned.
    Preview,
    /// The tab has unsaved modifications.
    Modified,
    /// The tab is read-only.
    ReadOnly,
    /// The tab is blocked (by policy, error, or precondition).
    Blocked,
    /// The tab is shared / co-edited with another actor.
    Shared,
    /// The tab was reopened from a closed / recovered context.
    Reopened,
    /// The tab item state cannot currently be resolved.
    StateUnknown,
}

impl M5TabItemState {
    /// Every tab item state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Pinned,
        Self::Preview,
        Self::Modified,
        Self::ReadOnly,
        Self::Blocked,
        Self::Shared,
        Self::Reopened,
        Self::StateUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Preview => "preview",
            Self::Modified => "modified",
            Self::ReadOnly => "read_only",
            Self::Blocked => "blocked",
            Self::Shared => "shared",
            Self::Reopened => "reopened",
            Self::StateUnknown => "state_unknown",
        }
    }

    /// Whether this state names a shared / co-edited tab.
    pub const fn is_shared(self) -> bool {
        matches!(self, Self::Shared)
    }

    /// Whether this state names a reopened / recovered tab.
    pub const fn is_reopened(self) -> bool {
        matches!(self, Self::Reopened)
    }

    /// Whether this state names a blocked tab that must never hide behind an ambiguous ellipsis.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Controlled breadcrumb ancestry kind — what the trail's ancestry actually represents, so a
/// symbol ancestry is never presented with the same weight as a filesystem path or a search scope.
/// Minted by this lane because the frozen matrix carries hierarchy / path *state* but not the kind
/// of ancestry a breadcrumb walks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreadcrumbAncestryKind {
    /// A filesystem / repository path.
    FilePath,
    /// A symbol / declaration ancestry (module → type → member).
    SymbolAncestry,
    /// A logical root-relative container path.
    LogicalRoot,
    /// A search / query scope path.
    SearchScope,
    /// A mixed ancestry combining path and symbol segments.
    MixedAncestry,
    /// The ancestry kind cannot currently be resolved.
    AncestryUnknown,
}

impl M5BreadcrumbAncestryKind {
    /// Every ancestry kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FilePath,
        Self::SymbolAncestry,
        Self::LogicalRoot,
        Self::SearchScope,
        Self::MixedAncestry,
        Self::AncestryUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilePath => "file_path",
            Self::SymbolAncestry => "symbol_ancestry",
            Self::LogicalRoot => "logical_root",
            Self::SearchScope => "search_scope",
            Self::MixedAncestry => "mixed_ancestry",
            Self::AncestryUnknown => "ancestry_unknown",
        }
    }

    /// Whether the ancestry kind is source-aware (resolved to a concrete kind).
    pub const fn is_source_aware(self) -> bool {
        !matches!(self, Self::AncestryUnknown)
    }
}

/// One mandatory rendered part a tab strip or breadcrumb trail must be able to show, so no
/// active-context or local-ancestry fact is left implicit behind compact chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TabBreadcrumbsAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed navigation / content disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The active-context state (tab strip).
    ActiveContext,
    /// The per-tab item state (tab strip).
    ItemState,
    /// The local-action budget / overflow (tab strip).
    LocalActionBudget,
    /// The hierarchy / path state (breadcrumbs).
    HierarchyPath,
    /// The ancestry kind (breadcrumbs).
    AncestryKind,
    /// The truncation / overflow behavior (breadcrumbs).
    TruncationOverflow,
    /// The source-aware path context (breadcrumbs).
    SourceContext,
    /// The partial / stale hierarchy disclosure (breadcrumbs).
    PartialOrStaleDisclosure,
    /// The command-backed path to trace the current context / ancestry (both components).
    ContextCommand,
}

impl M5TabBreadcrumbsAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ActiveContext,
        Self::ItemState,
        Self::LocalActionBudget,
        Self::HierarchyPath,
        Self::AncestryKind,
        Self::TruncationOverflow,
        Self::SourceContext,
        Self::PartialOrStaleDisclosure,
        Self::ContextCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ActiveContext => "active_context",
            Self::ItemState => "item_state",
            Self::LocalActionBudget => "local_action_budget",
            Self::HierarchyPath => "hierarchy_path",
            Self::AncestryKind => "ancestry_kind",
            Self::TruncationOverflow => "truncation_overflow",
            Self::SourceContext => "source_context",
            Self::PartialOrStaleDisclosure => "partial_or_stale_disclosure",
            Self::ContextCommand => "context_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to trace the
/// active context or local ancestry behind a degraded navigation component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TabBreadcrumbsNextAction {
    /// Open the command-backed context / ancestry detail.
    OpenContextDetail,
    /// Inspect the active-context state behind the tab.
    InspectActiveContext,
    /// Inspect the hierarchy / path behind the breadcrumb.
    InspectHierarchyPath,
    /// Review a hidden or blocked item / scope.
    ReviewHiddenOrBlocked,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5TabBreadcrumbsNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenContextDetail,
        Self::InspectActiveContext,
        Self::InspectHierarchyPath,
        Self::ReviewHiddenOrBlocked,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenContextDetail => "open_context_detail",
            Self::InspectActiveContext => "inspect_active_context",
            Self::InspectHierarchyPath => "inspect_hierarchy_path",
            Self::ReviewHiddenOrBlocked => "review_hidden_or_blocked",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TabBreadcrumbsExportField {
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
    /// The active-context state named by the tab strip.
    ActiveContext,
    /// The per-tab item state named by the tab strip.
    ItemState,
    /// The hierarchy / path state named by the breadcrumbs.
    HierarchyPath,
    /// The ancestry kind named by the breadcrumbs.
    AncestryKind,
    /// The local-action budget named by the tab strip.
    LocalActionBudget,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TabBreadcrumbsExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ActiveContext,
        Self::ItemState,
        Self::HierarchyPath,
        Self::AncestryKind,
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
            Self::ActiveContext => "active_context",
            Self::ItemState => "item_state",
            Self::HierarchyPath => "hierarchy_path",
            Self::AncestryKind => "ancestry_kind",
            Self::LocalActionBudget => "local_action_budget",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a tab strip degraded below a clean, context-legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous strip read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TabStripDegradeReason {
    /// The active context label is unstated; a user cannot tell which context is active.
    ActiveContextUnstated,
    /// The active context cannot currently be resolved.
    ActiveContextUnresolved,
    /// The tab strip presents itself as top-level workflow navigation.
    TabsMasqueradeAsTopLevelNavigation,
    /// A surface-local badge was invented for a context the shared grammar already names.
    SurfaceLocalBadgeInvented,
    /// The per-tab item state cannot currently be resolved.
    ItemStateUnresolved,
    /// The item state is encoded by color / hover alone rather than named.
    ItemStateHiddenBehindColorOnly,
    /// A blocked tab is hidden behind an ambiguous ellipsis.
    BlockedTabHiddenBehindEllipsis,
    /// No command-backed path to trace the active context is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TabStripDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ActiveContextUnstated,
        Self::ActiveContextUnresolved,
        Self::TabsMasqueradeAsTopLevelNavigation,
        Self::SurfaceLocalBadgeInvented,
        Self::ItemStateUnresolved,
        Self::ItemStateHiddenBehindColorOnly,
        Self::BlockedTabHiddenBehindEllipsis,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveContextUnstated => "active_context_unstated",
            Self::ActiveContextUnresolved => "active_context_unresolved",
            Self::TabsMasqueradeAsTopLevelNavigation => "tabs_masquerade_as_top_level_navigation",
            Self::SurfaceLocalBadgeInvented => "surface_local_badge_invented",
            Self::ItemStateUnresolved => "item_state_unresolved",
            Self::ItemStateHiddenBehindColorOnly => "item_state_hidden_behind_color_only",
            Self::BlockedTabHiddenBehindEllipsis => "blocked_tab_hidden_behind_ellipsis",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TabBreadcrumbsNextAction {
        match self {
            Self::ActiveContextUnstated | Self::ActiveContextUnresolved => {
                M5TabBreadcrumbsNextAction::InspectActiveContext
            }
            Self::TabsMasqueradeAsTopLevelNavigation
            | Self::SurfaceLocalBadgeInvented
            | Self::ContextTracePathMissing => M5TabBreadcrumbsNextAction::OpenContextDetail,
            Self::ItemStateUnresolved
            | Self::ItemStateHiddenBehindColorOnly
            | Self::BlockedTabHiddenBehindEllipsis => {
                M5TabBreadcrumbsNextAction::ReviewHiddenOrBlocked
            }
            Self::ProofStale => M5TabBreadcrumbsNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::ActiveContextUnstated | Self::ActiveContextUnresolved => {
                M5NavigationContentDowngradeTrigger::ActiveContextUnstated
            }
            Self::TabsMasqueradeAsTopLevelNavigation => {
                M5NavigationContentDowngradeTrigger::TabsMasqueradeAsWorkflowNav
            }
            Self::BlockedTabHiddenBehindEllipsis => {
                M5NavigationContentDowngradeTrigger::BlockedRowsHiddenBehindEllipsis
            }
            Self::SurfaceLocalBadgeInvented
            | Self::ItemStateUnresolved
            | Self::ItemStateHiddenBehindColorOnly
            | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a breadcrumb trail degraded below a clean, fully-explicit state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreadcrumbsDegradeReason {
    /// The leaf / current-object identity is unstated.
    LeafIdentityUnstated,
    /// The ancestry kind cannot currently be resolved.
    AncestryKindUnresolved,
    /// The hierarchy / path cannot currently be resolved.
    HierarchyPathUnresolved,
    /// The breadcrumb trail presents itself as top-level workflow navigation.
    BreadcrumbsMasqueradeAsTopLevelNavigation,
    /// Missing scope is collapsed into an ambiguous ellipsis.
    MissingScopeCollapsedIntoEllipsis,
    /// A partial or stale hierarchy is presented as a complete path.
    PartialOrStaleShownAsComplete,
    /// The path is not explicit in both compact and expanded views.
    PathNotExplicitAcrossViews,
    /// No command-backed path to trace the ancestry is reachable.
    AncestryTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BreadcrumbsDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LeafIdentityUnstated,
        Self::AncestryKindUnresolved,
        Self::HierarchyPathUnresolved,
        Self::BreadcrumbsMasqueradeAsTopLevelNavigation,
        Self::MissingScopeCollapsedIntoEllipsis,
        Self::PartialOrStaleShownAsComplete,
        Self::PathNotExplicitAcrossViews,
        Self::AncestryTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeafIdentityUnstated => "leaf_identity_unstated",
            Self::AncestryKindUnresolved => "ancestry_kind_unresolved",
            Self::HierarchyPathUnresolved => "hierarchy_path_unresolved",
            Self::BreadcrumbsMasqueradeAsTopLevelNavigation => {
                "breadcrumbs_masquerade_as_top_level_navigation"
            }
            Self::MissingScopeCollapsedIntoEllipsis => "missing_scope_collapsed_into_ellipsis",
            Self::PartialOrStaleShownAsComplete => "partial_or_stale_shown_as_complete",
            Self::PathNotExplicitAcrossViews => "path_not_explicit_across_views",
            Self::AncestryTracePathMissing => "ancestry_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TabBreadcrumbsNextAction {
        match self {
            Self::LeafIdentityUnstated
            | Self::AncestryKindUnresolved
            | Self::HierarchyPathUnresolved
            | Self::PathNotExplicitAcrossViews => M5TabBreadcrumbsNextAction::InspectHierarchyPath,
            Self::BreadcrumbsMasqueradeAsTopLevelNavigation | Self::AncestryTracePathMissing => {
                M5TabBreadcrumbsNextAction::OpenContextDetail
            }
            Self::MissingScopeCollapsedIntoEllipsis | Self::PartialOrStaleShownAsComplete => {
                M5TabBreadcrumbsNextAction::ReviewHiddenOrBlocked
            }
            Self::ProofStale => M5TabBreadcrumbsNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::LeafIdentityUnstated
            | Self::AncestryKindUnresolved
            | Self::HierarchyPathUnresolved
            | Self::PathNotExplicitAcrossViews => {
                M5NavigationContentDowngradeTrigger::HierarchyPathUnstated
            }
            Self::BreadcrumbsMasqueradeAsTopLevelNavigation => {
                M5NavigationContentDowngradeTrigger::TabsMasqueradeAsWorkflowNav
            }
            Self::MissingScopeCollapsedIntoEllipsis | Self::PartialOrStaleShownAsComplete => {
                M5NavigationContentDowngradeTrigger::BlockedRowsHiddenBehindEllipsis
            }
            Self::AncestryTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// True when the active-context state is one that cannot be resolved.
fn active_context_is_unresolved(state: M5ActiveContextState) -> bool {
    matches!(state, M5ActiveContextState::ContextUnresolved)
}

/// True when the hierarchy / path state is a partial or stale one that must never read as complete.
fn path_is_partial_or_stale(state: M5HierarchyPathState) -> bool {
    matches!(
        state,
        M5HierarchyPathState::StaleHierarchy | M5HierarchyPathState::PartialHierarchy
    )
}

/// True when the hierarchy / path state cannot be resolved.
fn path_is_unresolved(state: M5HierarchyPathState) -> bool {
    matches!(state, M5HierarchyPathState::PathUnresolved)
}

/// Input to [`resolve_tab_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TabStripResolutionInput {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The active-context label (name of the current context) shown; empty means unstated.
    pub active_context_label: String,
    /// The active-context state.
    pub active_context: M5ActiveContextState,
    /// The per-tab item state of the shown / active tab.
    pub item_state: M5TabItemState,
    /// True when the item state is stated non-color-only (name / icon-with-label, never color alone).
    pub item_state_stated: bool,
    /// The local-action budget for the strip.
    pub local_action_budget: M5LocalActionBudget,
    /// True when at least one blocked tab is present in the strip.
    pub has_blocked_tab: bool,
    /// True when a present blocked tab is stated visibly, never hidden behind an ambiguous ellipsis.
    pub blocked_tab_stated: bool,
    /// True when the strip reads as top-level workflow navigation rather than an active-context
    /// primitive.
    pub reads_as_top_level_workflow_navigation: bool,
    /// True when the strip invents a surface-local badge for a context the shared grammar names.
    pub invents_surface_local_badge: bool,
    /// True when a command-backed entrypoint to trace the active context is reachable, never
    /// menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe tab strip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTabStrip {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The active-context label named by the strip.
    pub active_context_label: String,
    /// The active-context token named by the strip.
    pub active_context: String,
    /// The per-tab item-state token named by the strip.
    pub item_state: String,
    /// Whether the item state names a shared / co-edited tab.
    pub item_state_shared: bool,
    /// Whether the item state names a reopened / recovered tab.
    pub item_state_reopened: bool,
    /// The local-action-budget token named by the strip.
    pub local_action_budget: String,
    /// Whether a blocked tab is present in the strip.
    pub has_blocked_tab: bool,
    /// Guardrail (MUST be `false` on a clean strip): the strip reads as top-level navigation.
    pub presents_as_top_level_navigation: bool,
    /// Guardrail (MUST be `false` on a clean strip): a surface-local badge was invented.
    pub invents_surface_local_badge: bool,
    /// Whether a command-backed entrypoint to trace the active context is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the strip could not read as a clean, context-legible state.
    pub degrade_reason: Option<M5TabStripDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TabBreadcrumbsNextAction,
    /// Whether the active context is legible at a glance (clean strip naming every fact).
    pub context_legible_at_a_glance: bool,
}

impl M5ResolvedTabStrip {
    /// Whether this strip reads as a clean, context-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_breadcrumbs`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BreadcrumbsResolutionInput {
    /// Stable identity of the trail instance.
    pub trail_id: String,
    /// The leaf / current-object identity shown; empty means unstated.
    pub leaf_label: String,
    /// The ancestry kind the trail walks.
    pub ancestry_kind: M5BreadcrumbAncestryKind,
    /// The hierarchy / path state.
    pub hierarchy_path: M5HierarchyPathState,
    /// True when the path stays explicit in both compact and expanded views (and in the export).
    pub path_explicit_in_compact_and_expanded: bool,
    /// True when the trail collapses missing scope into an ambiguous ellipsis.
    pub collapses_missing_scope_into_ellipsis: bool,
    /// True when a partial or stale hierarchy is presented as a complete path.
    pub presents_partial_or_stale_as_complete: bool,
    /// True when the trail reads as top-level workflow navigation rather than a local-structure
    /// primitive.
    pub reads_as_top_level_workflow_navigation: bool,
    /// True when a command-backed entrypoint to trace the ancestry is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe breadcrumb trail projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBreadcrumbs {
    /// Stable identity of the trail instance.
    pub trail_id: String,
    /// The leaf / current-object identity named by the trail.
    pub leaf_label: String,
    /// The ancestry-kind token named by the trail.
    pub ancestry_kind: String,
    /// Whether the ancestry is source-aware (resolved to a concrete kind).
    pub source_aware: bool,
    /// The hierarchy / path token named by the trail.
    pub hierarchy_path: String,
    /// Whether the hierarchy is partial or stale (and therefore must not read as complete).
    pub path_is_partial_or_stale: bool,
    /// Whether the path stays explicit in both compact and expanded views.
    pub path_explicit_in_compact_and_expanded: bool,
    /// Guardrail (MUST be `false` on a clean trail): missing scope collapses into an ambiguous
    /// ellipsis.
    pub collapses_missing_scope_into_ellipsis: bool,
    /// Guardrail (MUST be `false` on a clean trail): a partial / stale hierarchy reads as complete.
    pub presents_partial_or_stale_as_complete: bool,
    /// Guardrail (MUST be `false` on a clean trail): the trail reads as top-level navigation.
    pub presents_as_top_level_navigation: bool,
    /// Whether a command-backed entrypoint to trace the ancestry is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the trail could not read as a clean, fully-explicit state.
    pub degrade_reason: Option<M5BreadcrumbsDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TabBreadcrumbsNextAction,
    /// Whether the path is explicit end-to-end (clean trail naming every fact).
    pub path_explicit: bool,
}

impl M5ResolvedBreadcrumbs {
    /// Whether this trail reads as a clean, fully-explicit state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TabBreadcrumbsResolutionError {
    /// The strip id was empty.
    EmptyStripId,
    /// The trail id was empty.
    EmptyTrailId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TabBreadcrumbsResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStripId => "empty_strip_id",
            Self::EmptyTrailId => "empty_trail_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TabBreadcrumbsResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 tab-strip / breadcrumbs resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TabBreadcrumbsResolutionError {}

/// Resolves a tab strip so which context is active is legible at a glance: the strip names its
/// active-context state, per-tab item state (with no-color-only semantics), and local-action budget,
/// never reads as top-level workflow navigation, never invents a surface-local badge for a shared
/// context, and never hides a blocked tab behind an ambiguous ellipsis.
pub fn resolve_tab_strip(
    input: M5TabStripResolutionInput,
) -> Result<M5ResolvedTabStrip, M5TabBreadcrumbsResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5TabBreadcrumbsResolutionError::EmptyStripId);
    }
    if string_is_forbidden(&input.strip_id) || string_is_forbidden(&input.active_context_label) {
        return Err(M5TabBreadcrumbsResolutionError::ForbiddenMaterial);
    }

    let degrade_reason = if input.active_context_label.trim().is_empty() {
        Some(M5TabStripDegradeReason::ActiveContextUnstated)
    } else if active_context_is_unresolved(input.active_context) {
        Some(M5TabStripDegradeReason::ActiveContextUnresolved)
    } else if input.reads_as_top_level_workflow_navigation {
        Some(M5TabStripDegradeReason::TabsMasqueradeAsTopLevelNavigation)
    } else if input.invents_surface_local_badge {
        Some(M5TabStripDegradeReason::SurfaceLocalBadgeInvented)
    } else if matches!(input.item_state, M5TabItemState::StateUnknown) {
        Some(M5TabStripDegradeReason::ItemStateUnresolved)
    } else if !input.item_state_stated {
        Some(M5TabStripDegradeReason::ItemStateHiddenBehindColorOnly)
    } else if (input.has_blocked_tab || input.item_state.is_blocked()) && !input.blocked_tab_stated
    {
        Some(M5TabStripDegradeReason::BlockedTabHiddenBehindEllipsis)
    } else if !input.detail_command_available {
        Some(M5TabStripDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5TabStripDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TabBreadcrumbsNextAction::OpenContextDetail,
    };

    Ok(M5ResolvedTabStrip {
        strip_id: input.strip_id,
        active_context_label: input.active_context_label,
        active_context: input.active_context.as_str().to_owned(),
        item_state: input.item_state.as_str().to_owned(),
        item_state_shared: input.item_state.is_shared(),
        item_state_reopened: input.item_state.is_reopened(),
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        has_blocked_tab: input.has_blocked_tab || input.item_state.is_blocked(),
        presents_as_top_level_navigation: input.reads_as_top_level_workflow_navigation,
        invents_surface_local_badge: input.invents_surface_local_badge,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        context_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a breadcrumb trail so local ancestry is explicit: the trail names its leaf identity,
/// ancestry kind, hierarchy / path state, and source-aware context, stays explicit in compact,
/// expanded, and exported views, never collapses missing scope into an ambiguous ellipsis, never
/// presents a partial or stale hierarchy as a complete path, and never reads as top-level
/// navigation.
pub fn resolve_breadcrumbs(
    input: M5BreadcrumbsResolutionInput,
) -> Result<M5ResolvedBreadcrumbs, M5TabBreadcrumbsResolutionError> {
    if input.trail_id.trim().is_empty() {
        return Err(M5TabBreadcrumbsResolutionError::EmptyTrailId);
    }
    if string_is_forbidden(&input.trail_id) || string_is_forbidden(&input.leaf_label) {
        return Err(M5TabBreadcrumbsResolutionError::ForbiddenMaterial);
    }

    let partial_or_stale = path_is_partial_or_stale(input.hierarchy_path);

    let degrade_reason = if input.leaf_label.trim().is_empty() {
        Some(M5BreadcrumbsDegradeReason::LeafIdentityUnstated)
    } else if matches!(
        input.ancestry_kind,
        M5BreadcrumbAncestryKind::AncestryUnknown
    ) {
        Some(M5BreadcrumbsDegradeReason::AncestryKindUnresolved)
    } else if path_is_unresolved(input.hierarchy_path) {
        Some(M5BreadcrumbsDegradeReason::HierarchyPathUnresolved)
    } else if input.reads_as_top_level_workflow_navigation {
        Some(M5BreadcrumbsDegradeReason::BreadcrumbsMasqueradeAsTopLevelNavigation)
    } else if input.collapses_missing_scope_into_ellipsis {
        Some(M5BreadcrumbsDegradeReason::MissingScopeCollapsedIntoEllipsis)
    } else if partial_or_stale && input.presents_partial_or_stale_as_complete {
        Some(M5BreadcrumbsDegradeReason::PartialOrStaleShownAsComplete)
    } else if !input.path_explicit_in_compact_and_expanded {
        Some(M5BreadcrumbsDegradeReason::PathNotExplicitAcrossViews)
    } else if !input.detail_command_available {
        Some(M5BreadcrumbsDegradeReason::AncestryTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5BreadcrumbsDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TabBreadcrumbsNextAction::OpenContextDetail,
    };

    Ok(M5ResolvedBreadcrumbs {
        trail_id: input.trail_id,
        leaf_label: input.leaf_label,
        ancestry_kind: input.ancestry_kind.as_str().to_owned(),
        source_aware: input.ancestry_kind.is_source_aware(),
        hierarchy_path: input.hierarchy_path.as_str().to_owned(),
        path_is_partial_or_stale: partial_or_stale,
        path_explicit_in_compact_and_expanded: input.path_explicit_in_compact_and_expanded,
        collapses_missing_scope_into_ellipsis: input.collapses_missing_scope_into_ellipsis,
        presents_partial_or_stale_as_complete: partial_or_stale
            && input.presents_partial_or_stale_as_complete,
        presents_as_top_level_navigation: input.reads_as_top_level_workflow_navigation,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        path_explicit: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved tab strip and breadcrumb examples it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5TabBreadcrumbsConsumerSurface,
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
    pub anatomy_parts: Vec<M5TabBreadcrumbsAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TabBreadcrumbsExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    /// Resolved tab strip examples.
    pub tab_strip_examples: Vec<M5ResolvedTabStrip>,
    /// Resolved breadcrumb trail examples.
    pub breadcrumbs_examples: Vec<M5ResolvedBreadcrumbs>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a tab strip never presents itself as top-level workflow navigation.
    pub tabs_masquerade_as_top_level_workflow_navigation: bool,
    /// Hard invariant: a breadcrumb trail never presents itself as top-level workflow navigation.
    pub breadcrumbs_masquerade_as_top_level_workflow_navigation: bool,
    /// Hard invariant: a component never invents a surface-local badge for a shared context.
    pub invents_surface_local_badges_for_shared_context: bool,
    /// Hard invariant: a component never hides a blocked tab or collapses missing breadcrumb scope
    /// behind an ambiguous ellipsis.
    pub collapses_missing_scope_or_hides_blocked_behind_ellipsis: bool,
}

impl M5TabBreadcrumbsControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TabBreadcrumbsAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TabBreadcrumbsAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TabBreadcrumbsExportField> =
            self.export_fields.iter().copied().collect();
        M5TabBreadcrumbsExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.tabs_masquerade_as_top_level_workflow_navigation
            && !self.breadcrumbs_masquerade_as_top_level_workflow_navigation
            && !self.invents_surface_local_badges_for_shared_context
            && !self.collapses_missing_scope_or_hides_blocked_behind_ellipsis
    }

    /// True when every resolved example on this row is honest: no clean tab masquerades, invents a
    /// badge, hides a blocked tab, or lacks a trace path, and no clean breadcrumb masquerades,
    /// collapses missing scope, shows a partial / stale hierarchy as complete, or lacks a trace
    /// path.
    fn examples_are_honest(&self) -> bool {
        self.tab_strip_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presents_as_top_level_navigation
                    || ex.invents_surface_local_badge
                    || !ex.detail_command_available))
        }) && self.breadcrumbs_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presents_as_top_level_navigation
                    || ex.collapses_missing_scope_into_ellipsis
                    || ex.presents_partial_or_stale_as_complete
                    || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsVocabularySet {
    /// Navigation / content-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Active-context-state tokens (bound from the frozen matrix).
    pub active_context_states: Vec<String>,
    /// Hierarchy / path-state tokens (bound from the frozen matrix).
    pub hierarchy_path_states: Vec<String>,
    /// Local-action-budget tokens (bound from the frozen matrix).
    pub local_action_budgets: Vec<String>,
    /// Tab item-state tokens (minted by this lane).
    pub tab_item_states: Vec<String>,
    /// Breadcrumb ancestry-kind tokens (minted by this lane).
    pub breadcrumb_ancestry_kinds: Vec<String>,
    /// Tab-strip degrade-reason tokens.
    pub tab_strip_degrade_reasons: Vec<String>,
    /// Breadcrumbs degrade-reason tokens.
    pub breadcrumbs_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TabBreadcrumbsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5NavigationContentDisposition::ALL, |v| v.as_str()),
            active_context_states: tokens(&M5ActiveContextState::ALL, |v| v.as_str()),
            hierarchy_path_states: tokens(&M5HierarchyPathState::ALL, |v| v.as_str()),
            local_action_budgets: tokens(&M5LocalActionBudget::ALL, |v| v.as_str()),
            tab_item_states: tokens(&M5TabItemState::ALL, |v| v.as_str()),
            breadcrumb_ancestry_kinds: tokens(&M5BreadcrumbAncestryKind::ALL, |v| v.as_str()),
            tab_strip_degrade_reasons: tokens(&M5TabStripDegradeReason::ALL, |v| v.as_str()),
            breadcrumbs_degrade_reasons: tokens(&M5BreadcrumbsDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TabBreadcrumbsAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TabBreadcrumbsNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TabBreadcrumbsExportField::ALL, |v| v.as_str()),
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
pub struct M5TabBreadcrumbsGovernanceReview {
    /// The tab strip names its active context and per-tab item state.
    pub tab_names_active_context_and_item_state: bool,
    /// The tab strip states item state with no-color-only semantics.
    pub tab_item_state_no_color_only: bool,
    /// The breadcrumb trail names its ancestry kind and hierarchy / path.
    pub breadcrumbs_name_ancestry_and_path: bool,
    /// The breadcrumb trail stays explicit in compact, expanded, and exported views.
    pub breadcrumbs_explicit_across_views: bool,
    /// Tabs never masquerade as top-level workflow navigation.
    pub tabs_never_masquerade_as_workflow_navigation: bool,
    /// Breadcrumbs never masquerade as top-level workflow navigation.
    pub breadcrumbs_never_masquerade_as_workflow_navigation: bool,
    /// A shared context never gets a surface-local badge invented for it.
    pub no_surface_local_badges_for_shared_context: bool,
    /// Missing scope and blocked tabs are never hidden behind an ambiguous ellipsis.
    pub missing_scope_and_blocked_never_hidden_behind_ellipsis: bool,
    /// A partial or stale hierarchy is never presented as a complete path.
    pub partial_or_stale_hierarchy_never_shown_as_complete: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsConsumerProjection {
    /// Shell surfaces consume the shared active-context vocabulary.
    pub shell_surfaces_consume_active_context_vocabulary: bool,
    /// The explorer consumes the shared hierarchy / ancestry vocabulary.
    pub explorer_consumes_hierarchy_and_ancestry_vocabulary: bool,
    /// Search consumes the shared active-context and hierarchy vocabulary.
    pub search_consumes_active_context_and_hierarchy_vocabulary: bool,
    /// Navigation facts trace back to one canonical component contract.
    pub navigation_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical navigation source.
    pub support_export_reads_single_navigation_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TabBreadcrumbsControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TabBreadcrumbsControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TabBreadcrumbsControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TabBreadcrumbsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TabBreadcrumbsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TabBreadcrumbsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TabBreadcrumbsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TabBreadcrumbsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 tab-strip / breadcrumbs controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TabBreadcrumbsControlsPacket {
    /// Record kind; must equal [`M5_TAB_STRIP_BREADCRUMBS_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TabBreadcrumbsControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TabBreadcrumbsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TabBreadcrumbsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TabBreadcrumbsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TabBreadcrumbsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TabBreadcrumbsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TabBreadcrumbsControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5TabBreadcrumbsControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TAB_STRIP_BREADCRUMBS_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5TabBreadcrumbsControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TAB_STRIP_BREADCRUMBS_CONTROLS_RECORD_KIND {
            violations.push(M5TabBreadcrumbsControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_VERSION {
            violations.push(M5TabBreadcrumbsControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TabBreadcrumbsControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TabBreadcrumbsControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 tab-strip / breadcrumbs controls packet serializes"),
        ) {
            violations.push(M5TabBreadcrumbsControlsViolation::RawMaterialInExport);
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
            .expect("m5 tab-strip / breadcrumbs controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,tab_examples,breadcrumb_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .tab_strip_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.breadcrumbs_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.tab_strip_examples.len(),
                row.breadcrumbs_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Tab-Strip and Breadcrumbs Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Tab item states: {}\n",
            self.vocabulary_set.tab_item_states.join(", ")
        ));
        out.push_str(&format!(
            "- Breadcrumb ancestry kinds: {}\n",
            self.vocabulary_set.breadcrumb_ancestry_kinds.join(", ")
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
                "  - Tab-strip examples: {} / breadcrumb examples: {}\n",
                row.tab_strip_examples.len(),
                row.breadcrumbs_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5TabBreadcrumbsControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TabBreadcrumbsControlsViolation>),
}

impl fmt::Display for M5TabBreadcrumbsControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 tab-strip / breadcrumbs controls export parse failed: {error}"
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
                    "m5 tab-strip / breadcrumbs controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TabBreadcrumbsControlsArtifactError {}

/// Validation failures emitted by [`M5TabBreadcrumbsControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TabBreadcrumbsControlsViolation {
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
    /// A controls row carries a dishonest clean example (masquerade, badge, ellipsis, or missing
    /// trace).
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
    /// Tab-state grammar is not proven: clean tabs do not cover the shared item-state grammar across
    /// surfaces, or no masquerade / badge-invention example degrades.
    TabStateGrammarNotProven,
    /// Breadcrumb explicitness is not proven: no clean breadcrumb stays explicit across views, or no
    /// ellipsis-collapse / partial-shown-complete example degrades.
    BreadcrumbExplicitnessNotProven,
    /// Context / ancestry traceability is not proven: no clean tab and clean breadcrumb both offer a
    /// command-backed detail entrypoint.
    ContextAndAncestryTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TabBreadcrumbsControlsViolation {
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
            Self::TabStateGrammarNotProven => "tab_state_grammar_not_proven",
            Self::BreadcrumbExplicitnessNotProven => "breadcrumb_explicitness_not_proven",
            Self::ContextAndAncestryTraceabilityNotProven => {
                "context_and_ancestry_traceability_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_tab_strip_breadcrumbs_controls_export(
) -> Result<M5TabBreadcrumbsControlsPacket, M5TabBreadcrumbsControlsArtifactError> {
    let packet: M5TabBreadcrumbsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/support_export.json"
    )))
    .map_err(M5TabBreadcrumbsControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TabBreadcrumbsControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_REF,
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TAB_STRIP_SCHEMA_REF,
        M5_BREADCRUMBS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TabBreadcrumbsControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5TabBreadcrumbsControlsViolation::NoControlsRows);
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
            violations.push(M5TabBreadcrumbsControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TabBreadcrumbsControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TabBreadcrumbsControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TAB_STRIP_SCHEMA_REF) || !refs.contains(M5_BREADCRUMBS_SCHEMA_REF) {
            violations.push(M5TabBreadcrumbsControlsViolation::ComponentSchemaRefMissing);
        }
        if row.tab_strip_examples.is_empty() || row.breadcrumbs_examples.is_empty() {
            violations.push(M5TabBreadcrumbsControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TabBreadcrumbsControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TabBreadcrumbsControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tab_names_active_context_and_item_state,
        review.tab_item_state_no_color_only,
        review.breadcrumbs_name_ancestry_and_path,
        review.breadcrumbs_explicit_across_views,
        review.tabs_never_masquerade_as_workflow_navigation,
        review.breadcrumbs_never_masquerade_as_workflow_navigation,
        review.no_surface_local_badges_for_shared_context,
        review.missing_scope_and_blocked_never_hidden_behind_ellipsis,
        review.partial_or_stale_hierarchy_never_shown_as_complete,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TabBreadcrumbsControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_active_context_vocabulary,
        projection.explorer_consumes_hierarchy_and_ancestry_vocabulary,
        projection.search_consumes_active_context_and_hierarchy_vocabulary,
        projection.navigation_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_navigation_source,
    ] {
        if !ok {
            violations.push(M5TabBreadcrumbsControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TabBreadcrumbsControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TabBreadcrumbsControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TabBreadcrumbsControlsPacket,
    violations: &mut Vec<M5TabBreadcrumbsControlsViolation>,
) {
    let tabs = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.tab_strip_examples.iter())
    };
    let trails = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.breadcrumbs_examples.iter())
    };

    // AC1: tabs across claimed M5 panes show the same state grammar and do not invent surface-local
    // badges. Clean tabs cover at least two distinct item states from the shared vocabulary, a
    // masquerade example degrades, a badge-invention example degrades, and no clean tab masquerades
    // or invents a badge.
    let clean_tab_states: BTreeSet<String> = tabs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.item_state.clone())
        .collect();
    let masquerade_tab_degrades = tabs().any(|ex| {
        ex.degrade_reason == Some(M5TabStripDegradeReason::TabsMasqueradeAsTopLevelNavigation)
    });
    let badge_tab_degrades = tabs()
        .any(|ex| ex.degrade_reason == Some(M5TabStripDegradeReason::SurfaceLocalBadgeInvented));
    let no_clean_tab_masquerade_or_badge = tabs().all(|ex| {
        !(ex.is_clean() && (ex.presents_as_top_level_navigation || ex.invents_surface_local_badge))
    });
    if !(clean_tab_states.len() >= 2
        && masquerade_tab_degrades
        && badge_tab_degrades
        && no_clean_tab_masquerade_or_badge)
    {
        violations.push(M5TabBreadcrumbsControlsViolation::TabStateGrammarNotProven);
    }

    // AC2: breadcrumb paths remain explicit in compact, expanded, and exported views without
    // collapsing missing scope into ambiguous ellipses. At least one clean breadcrumb stays explicit
    // across views, an ellipsis-collapse example degrades, a partial/stale-shown-complete example
    // degrades, and no clean breadcrumb collapses missing scope or shows partial/stale as complete.
    let explicit_clean_breadcrumb = trails()
        .any(|ex| ex.is_clean() && ex.path_explicit_in_compact_and_expanded && ex.path_explicit);
    let ellipsis_collapse_degrades = trails().any(|ex| {
        ex.degrade_reason == Some(M5BreadcrumbsDegradeReason::MissingScopeCollapsedIntoEllipsis)
    });
    let partial_shown_complete_degrades = trails().any(|ex| {
        ex.degrade_reason == Some(M5BreadcrumbsDegradeReason::PartialOrStaleShownAsComplete)
    });
    let no_clean_collapse_or_partial = trails().all(|ex| {
        !(ex.is_clean()
            && (ex.collapses_missing_scope_into_ellipsis
                || ex.presents_partial_or_stale_as_complete))
    });
    if !(explicit_clean_breadcrumb
        && ellipsis_collapse_degrades
        && partial_shown_complete_degrades
        && no_clean_collapse_or_partial)
    {
        violations.push(M5TabBreadcrumbsControlsViolation::BreadcrumbExplicitnessNotProven);
    }

    // AC3: users can trace current context and local ancestry through one canonical component
    // contract and command-backed detail entrypoints. At least one clean tab and one clean
    // breadcrumb both expose a command-backed detail entrypoint.
    let traceable_tab = tabs().any(|ex| ex.is_clean() && ex.detail_command_available);
    let traceable_breadcrumb = trails().any(|ex| ex.is_clean() && ex.detail_command_available);
    if !(traceable_tab && traceable_breadcrumb) {
        violations.push(M5TabBreadcrumbsControlsViolation::ContextAndAncestryTraceabilityNotProven);
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
    M5NavigationContentComponentFamily::TabStrip,
    M5NavigationContentComponentFamily::Breadcrumbs,
];
