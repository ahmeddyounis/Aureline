//! Implemented M5 table / grid and panel-header primitives.
//!
//! The frozen [navigation / content component matrix][matrix] names the reusable navigation and
//! dense-content UI components and locks their controlled vocabulary. This module is the
//! dense-structure implement lane over that matrix: it turns the two dense-collection / header
//! components — the **table / grid** and the **panel header** — into resolvers that produce
//! export-safe, honest projections, so a user can read a grid's selection-versus-current state, its
//! sort / filter provenance, its pinned-column identity, its per-value qualification (estimated,
//! imported, stale, partial, or policy-limited), its exact / loaded / all-matching / hidden count
//! scope, and a header's active context and bounded local-action budget without the component
//! quietly presenting an estimated grid as canonical, losing a pinned column under virtualization,
//! collapsing count scopes, or letting the header become a cluttered secondary toolbar.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render table / grid headers, row-state affordances, pinned-column identity, sort / filter
//!   provenance, and selection bars with an exact-versus-loaded-versus-all-matching count
//!   vocabulary.** [`resolve_table_grid`] refuses to read as a clean, structure-legible grid when the
//!   grid identity is unstated, selection is collapsed into the current row, row focus is not
//!   visible, the current selection / a blocked row / the local actions are hover-only, the sort /
//!   filter provenance is unstated, a pinned column loses its identity under virtualization, or the
//!   count scope collapses; it degrades instead.
//! * **Show when values are estimated, imported, stale, partial, or policy-limited instead of
//!   presenting every grid as exact canonical truth.** [`resolve_table_grid`] degrades when a
//!   qualified value is presented as canonical or the value qualification cannot be resolved, and
//!   never presents a loaded subset as the exact total.
//! * **Keep panel headers naming their active context and a bounded local-action budget instead of
//!   becoming secondary toolbars or re-encoding counts in surface-local copy.**
//!   [`resolve_panel_header`] degrades when the header identity or active context is unstated, a
//!   background context reads as active, the local actions are hover-only, the header overloads into
//!   a toolbar, an overflowed action is dropped, or the header re-encodes the canonical count /
//!   selection model instead of pointing back to it.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SelectionState`] selection
//! vocabulary, the [`M5ItemStateFlag`] item-state vocabulary, the [`M5DensityVariant`] density
//! vocabulary, the [`M5LocalActionBudget`] local-action-budget vocabulary, and the
//! [`M5ActiveContextState`] active-context vocabulary — so request/data, review, governance, and
//! support surfaces can never fork their own selection, count, sort, or active-context wording. Raw
//! secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_table_grid_panel_header_controls,
    seeded_m5_table_grid_panel_header_controls_data_ui_beta_narrowed,
    seeded_m5_table_grid_panel_header_controls_review_ui_preview_narrowed,
    M5_TABLE_GRID_PANEL_HEADER_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    M5ActiveContextState, M5DensityVariant, M5ItemStateFlag, M5LocalActionBudget,
    M5NavigationContentAccessibilityRoute, M5NavigationContentComponentFamily,
    M5NavigationContentConsumerSurface, M5NavigationContentDeploymentLine,
    M5NavigationContentDowngradeTrigger, M5NavigationContentQualificationClass,
    M5NavigationContentRequiredLabel, M5SelectionState, M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
    M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_PANEL_HEADER_SCHEMA_REF,
    M5_TABLE_GRID_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TablePanelControlsPacket`].
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_RECORD_KIND: &str =
    "implement_m5_table_grid_and_panel_header_controls";

/// Schema version for M5 table / grid and panel-header controls records.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-table-grid-panel-header-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_DOC_REF: &str =
    "docs/navigation/m5_table_grid_and_panel_header_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-table-grid-panel-header-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-table-grid-panel-header-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-table-grid-panel-header-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-table-grid-panel-header-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5TablePanelConsumerSurface = M5NavigationContentConsumerSurface;

/// Controlled count scope a table or grid names, so exact, loaded, and all-matching scopes are never
/// collapsed into one vague total and hidden or out-of-scope rows are never silently dropped. Minted
/// by this lane because the frozen matrix count-scope vocabulary carries exact / loaded / all-matching
/// / hidden-by-filter / hidden-by-policy but not the **outside-current-scope** count the grid
/// current-versus-all-matching acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TablePanelScopeKind {
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
    /// The number of items outside the current scope (e.g. another partition or pane).
    OutsideCurrentScope,
    /// The count scope cannot currently be resolved.
    ScopeUnresolved,
}

impl M5TablePanelScopeKind {
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

/// Controlled sort / filter provenance — where a grid's row order and filtering came from, so a
/// default, relevance-ranked, or imported order is never presented as a user-chosen sort and a
/// filtered subset is never presented as the whole. Minted by this lane because provenance is a
/// table / grid property the frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SortFilterProvenance {
    /// The user explicitly chose this sort / filter.
    UserSorted,
    /// The grid is showing its default sort.
    DefaultSort,
    /// The order is a relevance / rank ordering.
    RelevanceRanked,
    /// The order was imported from the source and is preserved.
    ImportedOrder,
    /// An active filter narrows the rows to a subset.
    FilterApplied,
    /// The grid is explicitly unsorted / in natural order.
    Unsorted,
    /// The sort / filter provenance cannot currently be resolved.
    ProvenanceUnknown,
}

impl M5SortFilterProvenance {
    /// Every provenance, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::UserSorted,
        Self::DefaultSort,
        Self::RelevanceRanked,
        Self::ImportedOrder,
        Self::FilterApplied,
        Self::Unsorted,
        Self::ProvenanceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserSorted => "user_sorted",
            Self::DefaultSort => "default_sort",
            Self::RelevanceRanked => "relevance_ranked",
            Self::ImportedOrder => "imported_order",
            Self::FilterApplied => "filter_applied",
            Self::Unsorted => "unsorted",
            Self::ProvenanceUnknown => "provenance_unknown",
        }
    }

    /// Whether the provenance resolved to a concrete origin.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ProvenanceUnknown)
    }
}

/// Controlled pinned-column posture — whether an identity / anchor column stays pinned under
/// horizontal virtualization and overflow, so an identity column is never scrolled off and lost.
/// Minted by this lane because pinned-column identity is a table / grid property the frozen matrix
/// does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PinnedColumnState {
    /// The identity column is pinned and stays anchored.
    IdentityColumnPinned,
    /// A leading column is pinned.
    LeadingPinned,
    /// A trailing column is pinned.
    TrailingPinned,
    /// No column is pinned.
    Unpinned,
    /// The pinned column is currently scrolled into the overflow region.
    OverflowScrolled,
    /// The pinned-column posture cannot currently be resolved.
    PinUnresolved,
}

impl M5PinnedColumnState {
    /// Every pinned-column posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IdentityColumnPinned,
        Self::LeadingPinned,
        Self::TrailingPinned,
        Self::Unpinned,
        Self::OverflowScrolled,
        Self::PinUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityColumnPinned => "identity_column_pinned",
            Self::LeadingPinned => "leading_pinned",
            Self::TrailingPinned => "trailing_pinned",
            Self::Unpinned => "unpinned",
            Self::OverflowScrolled => "overflow_scrolled",
            Self::PinUnresolved => "pin_unresolved",
        }
    }

    /// Whether a column is actually pinned / anchored.
    pub const fn is_pinned(self) -> bool {
        matches!(
            self,
            Self::IdentityColumnPinned | Self::LeadingPinned | Self::TrailingPinned
        )
    }

    /// Whether the pinned-column posture resolved to a concrete answer.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::PinUnresolved)
    }
}

/// Controlled value qualification — whether a grid's values are exact canonical truth or are
/// estimated, imported, stale, partial, or policy-limited, so no qualified value is ever presented as
/// exact canonical truth. Minted by this lane because value qualification is a table / grid property
/// the frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ValueQualification {
    /// The values are exact, canonical truth.
    ExactCanonical,
    /// The values are estimated / approximate.
    Estimated,
    /// The values were imported from an external source.
    Imported,
    /// The values are stale relative to the source of truth.
    Stale,
    /// Only a partial set of values could be resolved.
    Partial,
    /// The values are limited / redacted by policy.
    PolicyLimited,
    /// The value qualification cannot currently be resolved.
    QualificationUnknown,
}

impl M5ValueQualification {
    /// Every value qualification, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExactCanonical,
        Self::Estimated,
        Self::Imported,
        Self::Stale,
        Self::Partial,
        Self::PolicyLimited,
        Self::QualificationUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCanonical => "exact_canonical",
            Self::Estimated => "estimated",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::PolicyLimited => "policy_limited",
            Self::QualificationUnknown => "qualification_unknown",
        }
    }

    /// Whether the values carry a non-canonical qualification that must be shown, never presented as
    /// exact canonical truth.
    pub const fn is_qualified(self) -> bool {
        matches!(
            self,
            Self::Estimated | Self::Imported | Self::Stale | Self::Partial | Self::PolicyLimited
        )
    }

    /// Whether the value qualification resolved to a concrete answer.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::QualificationUnknown)
    }
}

/// One mandatory rendered part a table / grid or panel header must be able to show, so no selection,
/// sort / filter, count, pinned-column, value-qualification, or local-action fact is left implicit
/// behind compact chrome or pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TablePanelAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed navigation / content disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The active context (panel header).
    ActiveContext,
    /// The selection-versus-current distinction.
    SelectionVersusCurrent,
    /// The row-focus indicator.
    RowFocus,
    /// The per-row item state.
    ItemState,
    /// The sort / filter provenance.
    SortFilterProvenance,
    /// The pinned-column identity.
    PinnedColumnIdentity,
    /// The per-value qualification (estimated / imported / stale / partial / policy-limited).
    ValueQualification,
    /// The count scope (exact / loaded / all-matching / hidden / outside).
    CountScope,
    /// The density variant.
    Density,
    /// The local-action budget / overflow.
    LocalActionBudget,
    /// The command-backed path to trace the collection scope.
    ScopeCommand,
}

impl M5TablePanelAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ActiveContext,
        Self::SelectionVersusCurrent,
        Self::RowFocus,
        Self::ItemState,
        Self::SortFilterProvenance,
        Self::PinnedColumnIdentity,
        Self::ValueQualification,
        Self::CountScope,
        Self::Density,
        Self::LocalActionBudget,
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
            Self::ActiveContext => "active_context",
            Self::SelectionVersusCurrent => "selection_versus_current",
            Self::RowFocus => "row_focus",
            Self::ItemState => "item_state",
            Self::SortFilterProvenance => "sort_filter_provenance",
            Self::PinnedColumnIdentity => "pinned_column_identity",
            Self::ValueQualification => "value_qualification",
            Self::CountScope => "count_scope",
            Self::Density => "density",
            Self::LocalActionBudget => "local_action_budget",
            Self::ScopeCommand => "scope_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to inspect the
/// collection's selection, sort / filter, counts, active context, or blocked rows behind a degraded
/// component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TablePanelNextAction {
    /// Open the command-backed scope / collection detail.
    OpenScopeDetail,
    /// Inspect the selection-versus-current and row focus.
    InspectSelectionAndFocus,
    /// Inspect the sort / filter provenance and count scopes.
    InspectSortFilterAndCounts,
    /// Inspect the active context and local actions (panel header).
    InspectActiveContextAndActions,
    /// Review a hidden, out-of-scope, or blocked row.
    ReviewHiddenOrBlocked,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
}

impl M5TablePanelNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenScopeDetail,
        Self::InspectSelectionAndFocus,
        Self::InspectSortFilterAndCounts,
        Self::InspectActiveContextAndActions,
        Self::ReviewHiddenOrBlocked,
        Self::ReviewDiagnostics,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenScopeDetail => "open_scope_detail",
            Self::InspectSelectionAndFocus => "inspect_selection_and_focus",
            Self::InspectSortFilterAndCounts => "inspect_sort_filter_and_counts",
            Self::InspectActiveContextAndActions => "inspect_active_context_and_actions",
            Self::ReviewHiddenOrBlocked => "review_hidden_or_blocked",
            Self::ReviewDiagnostics => "review_diagnostics",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TablePanelExportField {
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
    /// The selection state named by the grid.
    SelectionState,
    /// The sort / filter provenance named by the grid.
    SortFilterProvenance,
    /// The pinned-column posture named by the grid.
    PinnedColumnState,
    /// The value qualification named by the grid.
    ValueQualification,
    /// The count scope named by the grid.
    CountScope,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TablePanelExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SelectionState,
        Self::SortFilterProvenance,
        Self::PinnedColumnState,
        Self::ValueQualification,
        Self::CountScope,
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
            Self::SortFilterProvenance => "sort_filter_provenance",
            Self::PinnedColumnState => "pinned_column_state",
            Self::ValueQualification => "value_qualification",
            Self::CountScope => "count_scope",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a table / grid degraded below a clean, structure-legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous grid read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TableGridDegradeReason {
    /// The grid / current-object identity is unstated.
    GridIdentityUnstated,
    /// The selection is collapsed into the current / focused row.
    SelectionVersusCurrentCollapsed,
    /// The row-focus indicator is not visible.
    RowFocusNotVisible,
    /// The current selection can only be discovered by pointer hover.
    CurrentSelectionHoverOnly,
    /// A blocked row's state can only be discovered by pointer hover.
    BlockedStateHoverOnly,
    /// The local actions can only be discovered by pointer hover.
    LocalActionsHoverOnly,
    /// The sort / filter provenance cannot currently be resolved.
    SortFilterProvenanceUnstated,
    /// A pinned identity column lost its identity under virtualization / overflow.
    PinnedColumnIdentityLost,
    /// The pinned-column posture cannot currently be resolved.
    PinnedColumnUnresolved,
    /// A qualified (estimated / imported / stale / partial / policy-limited) value is presented as
    /// exact canonical truth.
    QualifiedValueShownAsCanonical,
    /// The value qualification cannot currently be resolved.
    ValueQualificationUnresolved,
    /// The exact, loaded, and all-matching count scopes are collapsed into one total.
    CountScopeCollapsed,
    /// The count scope cannot currently be resolved.
    CountScopeUnresolved,
    /// A loaded / virtualized subset is presented as the exact total.
    LoadedShownAsExact,
    /// A stale or partial backend is presented as a complete grid.
    StaleOrPartialShownAsComplete,
    /// No command-backed path to trace the collection scope is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TableGridDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::GridIdentityUnstated,
        Self::SelectionVersusCurrentCollapsed,
        Self::RowFocusNotVisible,
        Self::CurrentSelectionHoverOnly,
        Self::BlockedStateHoverOnly,
        Self::LocalActionsHoverOnly,
        Self::SortFilterProvenanceUnstated,
        Self::PinnedColumnIdentityLost,
        Self::PinnedColumnUnresolved,
        Self::QualifiedValueShownAsCanonical,
        Self::ValueQualificationUnresolved,
        Self::CountScopeCollapsed,
        Self::CountScopeUnresolved,
        Self::LoadedShownAsExact,
        Self::StaleOrPartialShownAsComplete,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GridIdentityUnstated => "grid_identity_unstated",
            Self::SelectionVersusCurrentCollapsed => "selection_versus_current_collapsed",
            Self::RowFocusNotVisible => "row_focus_not_visible",
            Self::CurrentSelectionHoverOnly => "current_selection_hover_only",
            Self::BlockedStateHoverOnly => "blocked_state_hover_only",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::SortFilterProvenanceUnstated => "sort_filter_provenance_unstated",
            Self::PinnedColumnIdentityLost => "pinned_column_identity_lost",
            Self::PinnedColumnUnresolved => "pinned_column_unresolved",
            Self::QualifiedValueShownAsCanonical => "qualified_value_shown_as_canonical",
            Self::ValueQualificationUnresolved => "value_qualification_unresolved",
            Self::CountScopeCollapsed => "count_scope_collapsed",
            Self::CountScopeUnresolved => "count_scope_unresolved",
            Self::LoadedShownAsExact => "loaded_shown_as_exact",
            Self::StaleOrPartialShownAsComplete => "stale_or_partial_shown_as_complete",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TablePanelNextAction {
        match self {
            Self::GridIdentityUnstated
            | Self::SortFilterProvenanceUnstated
            | Self::PinnedColumnIdentityLost
            | Self::PinnedColumnUnresolved
            | Self::QualifiedValueShownAsCanonical
            | Self::ValueQualificationUnresolved
            | Self::CountScopeCollapsed
            | Self::CountScopeUnresolved
            | Self::LoadedShownAsExact => M5TablePanelNextAction::InspectSortFilterAndCounts,
            Self::SelectionVersusCurrentCollapsed
            | Self::RowFocusNotVisible
            | Self::CurrentSelectionHoverOnly => M5TablePanelNextAction::InspectSelectionAndFocus,
            Self::BlockedStateHoverOnly
            | Self::LocalActionsHoverOnly
            | Self::StaleOrPartialShownAsComplete => M5TablePanelNextAction::ReviewHiddenOrBlocked,
            Self::ContextTracePathMissing => M5TablePanelNextAction::OpenScopeDetail,
            Self::ProofStale => M5TablePanelNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::GridIdentityUnstated
            | Self::SortFilterProvenanceUnstated
            | Self::PinnedColumnIdentityLost
            | Self::PinnedColumnUnresolved
            | Self::QualifiedValueShownAsCanonical
            | Self::ValueQualificationUnresolved => {
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
            Self::StaleOrPartialShownAsComplete | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a panel header degraded below a clean, legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelHeaderDegradeReason {
    /// The header / current-object identity is unstated.
    HeaderIdentityUnstated,
    /// The active context cannot currently be resolved.
    ActiveContextUnresolved,
    /// A background / preview context is presented as the active one.
    BackgroundContextShownAsActive,
    /// The local actions can only be discovered by pointer hover.
    LocalActionsHoverOnly,
    /// The panel header overloaded into a cluttered secondary toolbar.
    PanelHeaderOverloadedAsToolbar,
    /// An overflowed local action was silently dropped rather than routed to overflow.
    OverflowedActionDropped,
    /// The header re-encodes the canonical count / selection model in surface-local copy instead of
    /// pointing back to it.
    ReEncodesCanonicalCountsLocally,
    /// No command-backed path to trace the collection scope is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PanelHeaderDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::HeaderIdentityUnstated,
        Self::ActiveContextUnresolved,
        Self::BackgroundContextShownAsActive,
        Self::LocalActionsHoverOnly,
        Self::PanelHeaderOverloadedAsToolbar,
        Self::OverflowedActionDropped,
        Self::ReEncodesCanonicalCountsLocally,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderIdentityUnstated => "header_identity_unstated",
            Self::ActiveContextUnresolved => "active_context_unresolved",
            Self::BackgroundContextShownAsActive => "background_context_shown_as_active",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::PanelHeaderOverloadedAsToolbar => "panel_header_overloaded_as_toolbar",
            Self::OverflowedActionDropped => "overflowed_action_dropped",
            Self::ReEncodesCanonicalCountsLocally => "re_encodes_canonical_counts_locally",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TablePanelNextAction {
        match self {
            Self::HeaderIdentityUnstated
            | Self::ActiveContextUnresolved
            | Self::BackgroundContextShownAsActive
            | Self::LocalActionsHoverOnly
            | Self::PanelHeaderOverloadedAsToolbar
            | Self::OverflowedActionDropped => {
                M5TablePanelNextAction::InspectActiveContextAndActions
            }
            Self::ReEncodesCanonicalCountsLocally => {
                M5TablePanelNextAction::InspectSortFilterAndCounts
            }
            Self::ContextTracePathMissing => M5TablePanelNextAction::OpenScopeDetail,
            Self::ProofStale => M5TablePanelNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::HeaderIdentityUnstated
            | Self::ActiveContextUnresolved
            | Self::BackgroundContextShownAsActive => {
                M5NavigationContentDowngradeTrigger::ActiveContextUnstated
            }
            Self::LocalActionsHoverOnly => {
                M5NavigationContentDowngradeTrigger::LocalActionsHoverOnly
            }
            Self::PanelHeaderOverloadedAsToolbar | Self::OverflowedActionDropped => {
                M5NavigationContentDowngradeTrigger::PanelHeaderOverloaded
            }
            Self::ReEncodesCanonicalCountsLocally | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_table_grid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TableGridResolutionInput {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The current row / current-object label shown; empty means unstated.
    pub grid_label: String,
    /// The selection state of the collection.
    pub selection: M5SelectionState,
    /// True when the selection stays distinct from the current / focused row.
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
    /// The sort / filter provenance of the grid's order.
    pub sort_filter_provenance: M5SortFilterProvenance,
    /// The pinned-column posture.
    pub pinned_column: M5PinnedColumnState,
    /// True when a pinned identity column lost its identity under virtualization / overflow.
    pub pinned_column_identity_lost: bool,
    /// The value qualification of the grid's values.
    pub value_qualification: M5ValueQualification,
    /// True when a qualified value is presented as exact canonical truth.
    pub qualified_value_shown_as_canonical: bool,
    /// The count scope the visible count measures.
    pub count_scope: M5TablePanelScopeKind,
    /// True when the exact / loaded / all-matching / hidden scopes stay distinct, never collapsed.
    pub count_scopes_distinct: bool,
    /// True when a loaded / virtualized subset is presented as the exact total.
    pub loaded_shown_as_exact: bool,
    /// The density variant the grid renders at.
    pub density: M5DensityVariant,
    /// The local-action budget for the grid.
    pub local_action_budget: M5LocalActionBudget,
    /// True when the local actions can only be discovered by pointer hover.
    pub local_actions_hover_only: bool,
    /// True when the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// True when a stale or partial backend is presented as a complete grid.
    pub presents_stale_or_partial_as_complete: bool,
    /// True when a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe table / grid projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTableGrid {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The current row / current-object label named by the grid.
    pub grid_label: String,
    /// The selection-state token named by the grid.
    pub selection: String,
    /// Whether the selection stays distinct from the current / focused row.
    pub selection_versus_current_distinct: bool,
    /// Whether the row-focus indicator is visible.
    pub row_focus_visible: bool,
    /// Guardrail (MUST be `false` on a clean grid): the current selection is hover-only.
    pub current_selection_hover_only: bool,
    /// The per-row item-state token named by the grid.
    pub item_state: String,
    /// Whether a blocked row is present in the collection.
    pub has_blocked_row: bool,
    /// Guardrail (MUST be `false` on a clean grid): a blocked row's state is hover-only.
    pub blocked_state_hover_only: bool,
    /// The sort / filter-provenance token named by the grid.
    pub sort_filter_provenance: String,
    /// Whether the sort / filter provenance resolved to a concrete origin.
    pub sort_filter_provenance_resolved: bool,
    /// The pinned-column token named by the grid.
    pub pinned_column: String,
    /// Whether a column is actually pinned / anchored.
    pub pinned_column_pinned: bool,
    /// Guardrail (MUST be `false` on a clean grid): a pinned identity column lost its identity.
    pub pinned_column_identity_lost: bool,
    /// The value-qualification token named by the grid.
    pub value_qualification: String,
    /// Whether the values carry a non-canonical qualification that must be shown.
    pub value_is_qualified: bool,
    /// Guardrail (MUST be `false` on a clean grid): a qualified value reads as canonical.
    pub qualified_value_shown_as_canonical: bool,
    /// The count-scope token named by the grid.
    pub count_scope: String,
    /// Whether the count scope names hidden or out-of-scope rows.
    pub count_scope_hidden_or_outside: bool,
    /// Whether the exact / loaded / all-matching / hidden scopes stay distinct.
    pub count_scopes_distinct: bool,
    /// Guardrail (MUST be `false` on a clean grid): a loaded subset reads as the exact total.
    pub loaded_shown_as_exact: bool,
    /// The density-variant token named by the grid.
    pub density: String,
    /// The local-action-budget token named by the grid.
    pub local_action_budget: String,
    /// Guardrail (MUST be `false` on a clean grid): the local actions are hover-only.
    pub local_actions_hover_only: bool,
    /// Whether the backend is stale or only partially loaded.
    pub backend_stale_or_partial: bool,
    /// Guardrail (MUST be `false` on a clean grid): a stale / partial backend reads as complete.
    pub presents_stale_or_partial_as_complete: bool,
    /// Whether a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the grid could not read as a clean, structure-legible state.
    pub degrade_reason: Option<M5TableGridDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TablePanelNextAction,
    /// Whether the structure is legible at a glance (clean grid naming every fact).
    pub structure_legible_at_a_glance: bool,
}

impl M5ResolvedTableGrid {
    /// Whether this grid reads as a clean, structure-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_panel_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PanelHeaderResolutionInput {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The current header / current-object label shown; empty means unstated.
    pub header_label: String,
    /// The active-context state of the panel.
    pub active_context: M5ActiveContextState,
    /// True when a background / preview context is presented as the active one.
    pub background_context_shown_as_active: bool,
    /// The local-action budget for the header.
    pub local_action_budget: M5LocalActionBudget,
    /// True when the local actions can only be discovered by pointer hover.
    pub local_actions_hover_only: bool,
    /// True when the header overloaded into a cluttered secondary toolbar.
    pub becomes_secondary_toolbar: bool,
    /// True when an overflowed local action was silently dropped.
    pub overflowed_action_dropped: bool,
    /// True when the header points back to the canonical count / selection model.
    pub references_canonical_model: bool,
    /// True when the header re-encodes the canonical count / selection model in surface-local copy.
    pub re_encodes_canonical_counts_locally: bool,
    /// True when a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe panel-header projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPanelHeader {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The current header / current-object label named by the header.
    pub header_label: String,
    /// The active-context token named by the header.
    pub active_context: String,
    /// Guardrail (MUST be `false` on a clean header): a background context reads as active.
    pub background_context_shown_as_active: bool,
    /// The local-action-budget token named by the header.
    pub local_action_budget: String,
    /// Guardrail (MUST be `false` on a clean header): the local actions are hover-only.
    pub local_actions_hover_only: bool,
    /// Guardrail (MUST be `false` on a clean header): the header overloaded into a toolbar.
    pub becomes_secondary_toolbar: bool,
    /// Guardrail (MUST be `false` on a clean header): an overflowed action was dropped.
    pub overflowed_action_dropped: bool,
    /// Whether the header points back to the canonical count / selection model.
    pub references_canonical_model: bool,
    /// Guardrail (MUST be `false` on a clean header): the header re-encodes counts locally.
    pub re_encodes_canonical_counts_locally: bool,
    /// Whether a command-backed entrypoint to trace the collection scope is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the header could not read as a clean, legible state.
    pub degrade_reason: Option<M5PanelHeaderDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TablePanelNextAction,
    /// Whether the header is legible at a glance (clean header naming every fact).
    pub header_legible_at_a_glance: bool,
}

impl M5ResolvedPanelHeader {
    /// Whether this header reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TablePanelResolutionError {
    /// The grid id was empty.
    EmptyGridId,
    /// The header id was empty.
    EmptyHeaderId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TablePanelResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyGridId => "empty_grid_id",
            Self::EmptyHeaderId => "empty_header_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TablePanelResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 table-grid / panel-header resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TablePanelResolutionError {}

/// True when the selection state cannot be resolved.
fn selection_is_unresolved(state: M5SelectionState) -> bool {
    matches!(state, M5SelectionState::SelectionUnknown)
}

/// Resolves a table / grid so its structure is legible at a glance: the grid names its selection-
/// versus-current distinction and row focus (never hover-only), its per-row item state (blocked state
/// never hover-only), its sort / filter provenance (never unstated), its pinned-column identity
/// (never lost under virtualization), its value qualification (never presenting an estimated /
/// imported / stale / partial / policy-limited value as canonical), and its count scope (never
/// collapsed, and a loaded subset never presented as the exact total).
pub fn resolve_table_grid(
    input: M5TableGridResolutionInput,
) -> Result<M5ResolvedTableGrid, M5TablePanelResolutionError> {
    if input.grid_id.trim().is_empty() {
        return Err(M5TablePanelResolutionError::EmptyGridId);
    }
    if string_is_forbidden(&input.grid_id) || string_is_forbidden(&input.grid_label) {
        return Err(M5TablePanelResolutionError::ForbiddenMaterial);
    }

    let blocked_present =
        input.has_blocked_row || matches!(input.item_state, M5ItemStateFlag::Blocked);
    let value_is_qualified = input.value_qualification.is_qualified();
    let qualified_value_shown_as_canonical =
        value_is_qualified && input.qualified_value_shown_as_canonical;

    let degrade_reason = if input.grid_label.trim().is_empty() {
        Some(M5TableGridDegradeReason::GridIdentityUnstated)
    } else if selection_is_unresolved(input.selection) || !input.selection_versus_current_distinct {
        Some(M5TableGridDegradeReason::SelectionVersusCurrentCollapsed)
    } else if !input.row_focus_visible {
        Some(M5TableGridDegradeReason::RowFocusNotVisible)
    } else if input.current_selection_hover_only {
        Some(M5TableGridDegradeReason::CurrentSelectionHoverOnly)
    } else if blocked_present && input.blocked_state_hover_only {
        Some(M5TableGridDegradeReason::BlockedStateHoverOnly)
    } else if input.local_actions_hover_only {
        Some(M5TableGridDegradeReason::LocalActionsHoverOnly)
    } else if !input.sort_filter_provenance.is_resolved() {
        Some(M5TableGridDegradeReason::SortFilterProvenanceUnstated)
    } else if input.pinned_column_identity_lost {
        Some(M5TableGridDegradeReason::PinnedColumnIdentityLost)
    } else if !input.pinned_column.is_resolved() {
        Some(M5TableGridDegradeReason::PinnedColumnUnresolved)
    } else if qualified_value_shown_as_canonical {
        Some(M5TableGridDegradeReason::QualifiedValueShownAsCanonical)
    } else if !input.value_qualification.is_resolved() {
        Some(M5TableGridDegradeReason::ValueQualificationUnresolved)
    } else if !input.count_scopes_distinct {
        Some(M5TableGridDegradeReason::CountScopeCollapsed)
    } else if !input.count_scope.is_resolved() {
        Some(M5TableGridDegradeReason::CountScopeUnresolved)
    } else if input.loaded_shown_as_exact {
        Some(M5TableGridDegradeReason::LoadedShownAsExact)
    } else if input.backend_stale_or_partial && input.presents_stale_or_partial_as_complete {
        Some(M5TableGridDegradeReason::StaleOrPartialShownAsComplete)
    } else if !input.detail_command_available {
        Some(M5TableGridDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5TableGridDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TablePanelNextAction::OpenScopeDetail,
    };

    Ok(M5ResolvedTableGrid {
        grid_id: input.grid_id,
        grid_label: input.grid_label,
        selection: input.selection.as_str().to_owned(),
        selection_versus_current_distinct: input.selection_versus_current_distinct,
        row_focus_visible: input.row_focus_visible,
        current_selection_hover_only: input.current_selection_hover_only,
        item_state: input.item_state.as_str().to_owned(),
        has_blocked_row: blocked_present,
        blocked_state_hover_only: input.blocked_state_hover_only,
        sort_filter_provenance: input.sort_filter_provenance.as_str().to_owned(),
        sort_filter_provenance_resolved: input.sort_filter_provenance.is_resolved(),
        pinned_column: input.pinned_column.as_str().to_owned(),
        pinned_column_pinned: input.pinned_column.is_pinned(),
        pinned_column_identity_lost: input.pinned_column_identity_lost,
        value_qualification: input.value_qualification.as_str().to_owned(),
        value_is_qualified,
        qualified_value_shown_as_canonical,
        count_scope: input.count_scope.as_str().to_owned(),
        count_scope_hidden_or_outside: input.count_scope.is_hidden_or_outside(),
        count_scopes_distinct: input.count_scopes_distinct,
        loaded_shown_as_exact: input.loaded_shown_as_exact,
        density: input.density.as_str().to_owned(),
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        local_actions_hover_only: input.local_actions_hover_only,
        backend_stale_or_partial: input.backend_stale_or_partial,
        presents_stale_or_partial_as_complete: input.backend_stale_or_partial
            && input.presents_stale_or_partial_as_complete,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        structure_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a panel header so it is legible at a glance: the header names its identity and active
/// context (never presenting a background context as active), its bounded local-action budget (never
/// hover-only, never overloading into a secondary toolbar, never dropping an overflowed action), and
/// points back to the canonical count / selection model instead of re-encoding counts in surface-
/// local copy.
pub fn resolve_panel_header(
    input: M5PanelHeaderResolutionInput,
) -> Result<M5ResolvedPanelHeader, M5TablePanelResolutionError> {
    if input.header_id.trim().is_empty() {
        return Err(M5TablePanelResolutionError::EmptyHeaderId);
    }
    if string_is_forbidden(&input.header_id) || string_is_forbidden(&input.header_label) {
        return Err(M5TablePanelResolutionError::ForbiddenMaterial);
    }

    let degrade_reason = if input.header_label.trim().is_empty() {
        Some(M5PanelHeaderDegradeReason::HeaderIdentityUnstated)
    } else if matches!(
        input.active_context,
        M5ActiveContextState::ContextUnresolved
    ) {
        Some(M5PanelHeaderDegradeReason::ActiveContextUnresolved)
    } else if input.background_context_shown_as_active {
        Some(M5PanelHeaderDegradeReason::BackgroundContextShownAsActive)
    } else if input.local_actions_hover_only {
        Some(M5PanelHeaderDegradeReason::LocalActionsHoverOnly)
    } else if input.becomes_secondary_toolbar {
        Some(M5PanelHeaderDegradeReason::PanelHeaderOverloadedAsToolbar)
    } else if input.overflowed_action_dropped {
        Some(M5PanelHeaderDegradeReason::OverflowedActionDropped)
    } else if input.re_encodes_canonical_counts_locally || !input.references_canonical_model {
        Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    } else if !input.detail_command_available {
        Some(M5PanelHeaderDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5PanelHeaderDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TablePanelNextAction::OpenScopeDetail,
    };

    Ok(M5ResolvedPanelHeader {
        header_id: input.header_id,
        header_label: input.header_label,
        active_context: input.active_context.as_str().to_owned(),
        background_context_shown_as_active: input.background_context_shown_as_active,
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        local_actions_hover_only: input.local_actions_hover_only,
        becomes_secondary_toolbar: input.becomes_secondary_toolbar,
        overflowed_action_dropped: input.overflowed_action_dropped,
        references_canonical_model: input.references_canonical_model,
        re_encodes_canonical_counts_locally: input.re_encodes_canonical_counts_locally,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        header_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved grid and header examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5TablePanelConsumerSurface,
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
    pub anatomy_parts: Vec<M5TablePanelAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TablePanelExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    /// Resolved table / grid examples.
    pub table_grid_examples: Vec<M5ResolvedTableGrid>,
    /// Resolved panel-header examples.
    pub panel_header_examples: Vec<M5ResolvedPanelHeader>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: current selection, blocked state, or local actions are never hover-only.
    pub hides_current_selection_blocked_or_actions_behind_hover_only: bool,
    /// Hard invariant: selection-versus-current and count scopes are never collapsed.
    pub collapses_selection_versus_current_or_count_scopes: bool,
    /// Hard invariant: a qualified, stale, or partial grid is never presented as exact canonical.
    pub presents_qualified_stale_or_partial_grid_as_canonical: bool,
    /// Hard invariant: a panel header never overloads into a toolbar or re-encodes counts locally.
    pub panel_header_overloads_or_re_encodes_counts: bool,
}

impl M5TablePanelControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TablePanelAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TablePanelAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TablePanelExportField> =
            self.export_fields.iter().copied().collect();
        M5TablePanelExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_current_selection_blocked_or_actions_behind_hover_only
            && !self.collapses_selection_versus_current_or_count_scopes
            && !self.presents_qualified_stale_or_partial_grid_as_canonical
            && !self.panel_header_overloads_or_re_encodes_counts
    }

    /// True when every resolved example on this row is honest: no clean grid or header hides its
    /// current selection / blocked state / actions behind hover, collapses selection or counts, fakes
    /// a canonical grid, loses a pinned column, or lacks a trace path; and no clean header overloads
    /// into a toolbar or re-encodes counts locally.
    fn examples_are_honest(&self) -> bool {
        self.table_grid_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.current_selection_hover_only
                    || ex.blocked_state_hover_only
                    || ex.local_actions_hover_only
                    || !ex.selection_versus_current_distinct
                    || !ex.count_scopes_distinct
                    || ex.loaded_shown_as_exact
                    || !ex.sort_filter_provenance_resolved
                    || ex.pinned_column_identity_lost
                    || ex.qualified_value_shown_as_canonical
                    || ex.presents_stale_or_partial_as_complete
                    || !ex.detail_command_available))
        }) && self.panel_header_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.local_actions_hover_only
                    || ex.background_context_shown_as_active
                    || ex.becomes_secondary_toolbar
                    || ex.overflowed_action_dropped
                    || ex.re_encodes_canonical_counts_locally
                    || !ex.references_canonical_model
                    || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelVocabularySet {
    /// Selection-state tokens (bound from the frozen matrix).
    pub selection_states: Vec<String>,
    /// Item-state-flag tokens (bound from the frozen matrix).
    pub item_state_flags: Vec<String>,
    /// Density-variant tokens (bound from the frozen matrix).
    pub density_variants: Vec<String>,
    /// Local-action-budget tokens (bound from the frozen matrix).
    pub local_action_budgets: Vec<String>,
    /// Active-context-state tokens (bound from the frozen matrix).
    pub active_context_states: Vec<String>,
    /// Count-scope-kind tokens (minted by this lane).
    pub count_scope_kinds: Vec<String>,
    /// Sort / filter-provenance tokens (minted by this lane).
    pub sort_filter_provenances: Vec<String>,
    /// Pinned-column-state tokens (minted by this lane).
    pub pinned_column_states: Vec<String>,
    /// Value-qualification tokens (minted by this lane).
    pub value_qualifications: Vec<String>,
    /// Table / grid degrade-reason tokens.
    pub table_grid_degrade_reasons: Vec<String>,
    /// Panel-header degrade-reason tokens.
    pub panel_header_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TablePanelVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            selection_states: tokens(&M5SelectionState::ALL, |v| v.as_str()),
            item_state_flags: tokens(&M5ItemStateFlag::ALL, |v| v.as_str()),
            density_variants: tokens(&M5DensityVariant::ALL, |v| v.as_str()),
            local_action_budgets: tokens(&M5LocalActionBudget::ALL, |v| v.as_str()),
            active_context_states: tokens(&M5ActiveContextState::ALL, |v| v.as_str()),
            count_scope_kinds: tokens(&M5TablePanelScopeKind::ALL, |v| v.as_str()),
            sort_filter_provenances: tokens(&M5SortFilterProvenance::ALL, |v| v.as_str()),
            pinned_column_states: tokens(&M5PinnedColumnState::ALL, |v| v.as_str()),
            value_qualifications: tokens(&M5ValueQualification::ALL, |v| v.as_str()),
            table_grid_degrade_reasons: tokens(&M5TableGridDegradeReason::ALL, |v| v.as_str()),
            panel_header_degrade_reasons: tokens(&M5PanelHeaderDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TablePanelAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TablePanelNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TablePanelExportField::ALL, |v| v.as_str()),
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
pub struct M5TablePanelGovernanceReview {
    /// The grid names its selection, sort / filter provenance, and count scopes.
    pub table_names_selection_sort_filter_and_counts: bool,
    /// Value qualification is honest: an estimated / imported / stale / partial / policy-limited value
    /// never reads as exact canonical truth.
    pub table_value_qualification_honest_never_canonical_overclaim: bool,
    /// A pinned identity column stays anchored under virtualization and column overflow.
    pub table_pinned_column_identity_stable_under_virtualization: bool,
    /// A loaded / virtualized grid subset is never presented as the exact total.
    pub table_loaded_never_shown_as_exact: bool,
    /// The panel header names its active context and a bounded local-action budget.
    pub panel_header_names_active_context_and_bounded_actions: bool,
    /// The panel header never overloads into a cluttered secondary toolbar.
    pub panel_header_never_becomes_secondary_toolbar: bool,
    /// The panel header points back to one canonical count / selection model.
    pub panel_header_references_canonical_count_and_selection_model: bool,
    /// Selection is always kept distinct from the current / focused row.
    pub selection_versus_current_always_distinct: bool,
    /// The current selection, a blocked row, and the local actions are never hover-only.
    pub current_selection_blocked_and_actions_never_hover_only: bool,
    /// Exact, loaded, and all-matching count scopes are never collapsed.
    pub count_scopes_never_collapsed: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelConsumerProjection {
    /// The request / data surface consumes the shared grid sort / filter / scope vocabulary.
    pub data_surface_consumes_table_sort_filter_and_scope_vocabulary: bool,
    /// The review queue consumes the shared grid selection / scope vocabulary.
    pub review_queue_consumes_table_selection_and_scope_vocabulary: bool,
    /// Governance and support surfaces consume the same shared grid semantics.
    pub governance_and_support_consume_shared_grid_semantics: bool,
    /// Grid facts trace back to one canonical header and selection model.
    pub grid_facts_trace_to_single_header_and_selection_model: bool,
    /// Support / export reads a single canonical grid source.
    pub support_export_reads_single_grid_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TablePanelControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TablePanelControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TablePanelControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TablePanelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TablePanelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TablePanelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TablePanelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TablePanelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 table / grid and panel-header controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TablePanelControlsPacket {
    /// Record kind; must equal [`M5_TABLE_GRID_PANEL_HEADER_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TablePanelControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TablePanelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TablePanelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TablePanelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TablePanelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TablePanelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TablePanelControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5TablePanelControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TABLE_GRID_PANEL_HEADER_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5TablePanelControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TABLE_GRID_PANEL_HEADER_CONTROLS_RECORD_KIND {
            violations.push(M5TablePanelControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_VERSION {
            violations.push(M5TablePanelControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TablePanelControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TablePanelControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 table-grid / panel-header controls packet serializes"),
        ) {
            violations.push(M5TablePanelControlsViolation::RawMaterialInExport);
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
            .expect("m5 table-grid / panel-header controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,table_examples,header_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .table_grid_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.panel_header_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.table_grid_examples.len(),
                row.panel_header_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Table / Grid and Panel-Header Controls\n\n");
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
            "- Sort / filter provenances: {}\n",
            self.vocabulary_set.sort_filter_provenances.join(", ")
        ));
        out.push_str(&format!(
            "- Value qualifications: {}\n",
            self.vocabulary_set.value_qualifications.join(", ")
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
                "  - Table / grid examples: {} / panel-header examples: {}\n",
                row.table_grid_examples.len(),
                row.panel_header_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5TablePanelControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TablePanelControlsViolation>),
}

impl fmt::Display for M5TablePanelControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 table-grid / panel-header controls export parse failed: {error}"
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
                    "m5 table-grid / panel-header controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TablePanelControlsArtifactError {}

/// Validation failures emitted by [`M5TablePanelControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TablePanelControlsViolation {
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
    /// Shared sort / filter and count semantics are not proven: clean grid examples do not reuse the
    /// same sort / filter and scope vocabulary across surfaces, or no count-scope-collapse or
    /// provenance-unstated example degrades.
    SharedSortFilterAndCountSemanticsNotProven,
    /// Pinned-identity and provenance truth is not proven: no grid under virtualization keeps a pinned
    /// column stable and value qualification honest, or no pinned-column-lost / qualified-value-shown-
    /// as-canonical example degrades.
    PinnedIdentityAndProvenanceTruthNotProven,
    /// Canonical header and selection model is not proven: no re-encode / overload example degrades,
    /// or a clean header re-encodes counts or overloads into a toolbar.
    CanonicalHeaderAndSelectionModelNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TablePanelControlsViolation {
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
            Self::SharedSortFilterAndCountSemanticsNotProven => {
                "shared_sort_filter_and_count_semantics_not_proven"
            }
            Self::PinnedIdentityAndProvenanceTruthNotProven => {
                "pinned_identity_and_provenance_truth_not_proven"
            }
            Self::CanonicalHeaderAndSelectionModelNotProven => {
                "canonical_header_and_selection_model_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_table_grid_panel_header_controls_export(
) -> Result<M5TablePanelControlsPacket, M5TablePanelControlsArtifactError> {
    let packet: M5TablePanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-table-grid-panel-header-controls-proof/support_export.json"
    )))
    .map_err(M5TablePanelControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TablePanelControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_REF,
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TABLE_GRID_SCHEMA_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TablePanelControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5TablePanelControlsViolation::NoControlsRows);
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
            violations.push(M5TablePanelControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TablePanelControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TablePanelControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TABLE_GRID_SCHEMA_REF) || !refs.contains(M5_PANEL_HEADER_SCHEMA_REF) {
            violations.push(M5TablePanelControlsViolation::ComponentSchemaRefMissing);
        }
        if row.table_grid_examples.is_empty() || row.panel_header_examples.is_empty() {
            violations.push(M5TablePanelControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TablePanelControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TablePanelControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.table_names_selection_sort_filter_and_counts,
        review.table_value_qualification_honest_never_canonical_overclaim,
        review.table_pinned_column_identity_stable_under_virtualization,
        review.table_loaded_never_shown_as_exact,
        review.panel_header_names_active_context_and_bounded_actions,
        review.panel_header_never_becomes_secondary_toolbar,
        review.panel_header_references_canonical_count_and_selection_model,
        review.selection_versus_current_always_distinct,
        review.current_selection_blocked_and_actions_never_hover_only,
        review.count_scopes_never_collapsed,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TablePanelControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.data_surface_consumes_table_sort_filter_and_scope_vocabulary,
        projection.review_queue_consumes_table_selection_and_scope_vocabulary,
        projection.governance_and_support_consume_shared_grid_semantics,
        projection.grid_facts_trace_to_single_header_and_selection_model,
        projection.support_export_reads_single_grid_source,
    ] {
        if !ok {
            violations.push(M5TablePanelControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TablePanelControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TablePanelControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TablePanelControlsPacket,
    violations: &mut Vec<M5TablePanelControlsViolation>,
) {
    let grids = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.table_grid_examples.iter())
    };
    let headers = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.panel_header_examples.iter())
    };

    // AC1: request/data, review, governance, and support grid consumers reuse the same sort /
    // filter and count semantics. Clean grid examples cover at least two distinct count scopes and
    // at least two distinct sort / filter provenances, a count-scope-collapse and a provenance-
    // unstated example both degrade, and no clean example collapses scopes or hides provenance.
    let clean_scope_kinds: BTreeSet<String> = grids()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.count_scope.clone())
        .collect();
    let clean_provenances: BTreeSet<String> = grids()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.sort_filter_provenance.clone())
        .collect();
    let scope_collapse_degrades =
        grids().any(|ex| ex.degrade_reason == Some(M5TableGridDegradeReason::CountScopeCollapsed));
    let provenance_unstated_degrades = grids().any(|ex| {
        ex.degrade_reason == Some(M5TableGridDegradeReason::SortFilterProvenanceUnstated)
    });
    let no_clean_scope_collapse = grids().all(|ex| !ex.is_clean() || ex.count_scopes_distinct);
    let no_clean_provenance_hidden =
        grids().all(|ex| !ex.is_clean() || ex.sort_filter_provenance_resolved);
    if !(clean_scope_kinds.len() >= 2
        && clean_provenances.len() >= 2
        && scope_collapse_degrades
        && provenance_unstated_degrades
        && no_clean_scope_collapse
        && no_clean_provenance_hidden)
    {
        violations.push(M5TablePanelControlsViolation::SharedSortFilterAndCountSemanticsNotProven);
    }

    // AC2: pinned and identity columns stay stable under virtualization and overflow without losing
    // provenance or scope truth, and values are qualified rather than presented as exact canonical.
    // At least one clean grid pins an identity column while honestly qualifying its values, a pinned-
    // column-lost example degrades, a qualified-value-shown-as-canonical example degrades, and no
    // clean example loses a pinned column or fakes canonical values.
    let clean_pinned_qualified =
        grids().any(|ex| ex.is_clean() && ex.pinned_column_pinned && ex.value_is_qualified);
    let pinned_lost_degrades = grids()
        .any(|ex| ex.degrade_reason == Some(M5TableGridDegradeReason::PinnedColumnIdentityLost));
    let qualified_as_canonical_degrades = grids().any(|ex| {
        ex.degrade_reason == Some(M5TableGridDegradeReason::QualifiedValueShownAsCanonical)
    });
    let no_clean_pin_lost_or_faked_canonical = grids().all(|ex| {
        !(ex.is_clean()
            && (ex.pinned_column_identity_lost || ex.qualified_value_shown_as_canonical))
    });
    if !(clean_pinned_qualified
        && pinned_lost_degrades
        && qualified_as_canonical_degrades
        && no_clean_pin_lost_or_faked_canonical)
    {
        violations.push(M5TablePanelControlsViolation::PinnedIdentityAndProvenanceTruthNotProven);
    }

    // AC3: grid / export consumers point back to one canonical B132 header and selection model
    // instead of re-encoding counts in surface-local copy. At least one clean panel header references
    // the canonical model, a re-encode example and a toolbar-overload example both degrade, and no
    // clean header re-encodes counts or overloads into a toolbar.
    let clean_canonical_header = headers().any(|ex| ex.is_clean() && ex.references_canonical_model);
    let re_encode_degrades = headers().any(|ex| {
        ex.degrade_reason == Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    });
    let overload_degrades = headers().any(|ex| {
        ex.degrade_reason == Some(M5PanelHeaderDegradeReason::PanelHeaderOverloadedAsToolbar)
    });
    let no_clean_re_encode_or_overload = headers().all(|ex| {
        !(ex.is_clean()
            && (ex.re_encodes_canonical_counts_locally
                || !ex.references_canonical_model
                || ex.becomes_secondary_toolbar))
    });
    if !(clean_canonical_header
        && re_encode_degrades
        && overload_degrades
        && no_clean_re_encode_or_overload)
    {
        violations.push(M5TablePanelControlsViolation::CanonicalHeaderAndSelectionModelNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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
    M5NavigationContentComponentFamily::TableGrid,
    M5NavigationContentComponentFamily::PanelHeader,
];
