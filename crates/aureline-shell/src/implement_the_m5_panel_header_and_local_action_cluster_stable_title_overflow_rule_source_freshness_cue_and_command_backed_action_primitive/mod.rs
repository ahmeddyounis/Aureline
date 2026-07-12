//! Implemented M5 panel-header and local-action-cluster primitives.
//!
//! The frozen [navigation / content component matrix][matrix] names the reusable navigation and
//! content UI components and locks their controlled vocabulary. This module is the **panel-header /
//! local-action-cluster implement lane** over that matrix: it turns the panel header — and the
//! bounded local-action cluster that lives inside it — into resolvers that produce export-safe,
//! honest projections, so a user can read a pane's stable title, its active context, its
//! source / freshness truth (cached, partial, stale, remote, or provider-owned), and its bounded
//! local-action budget (structured menus / overflow, keyboard-reachable, never persistent clutter,
//! never silently dropped, never re-instantiating a different surface under compaction) without the
//! header quietly overstating readiness or a compacted header masquerading as a different pane.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render panel headers with stable title slots, local-action clusters, overflow rules,
//!   source / freshness badges where relevant, and command-backed reveal / refresh / detail
//!   affordances.** [`resolve_panel_header`] refuses to read as a clean, low-noise header when the
//!   title is unstated or unstable, the active context is unresolved or a background context reads as
//!   active, the source / freshness is unresolved, a cached / partial / stale / remote /
//!   provider-owned pane hides its freshness cue at the boundary, the header re-encodes the canonical
//!   count / selection model in surface-local copy, or the refresh / detail commands are missing; it
//!   degrades instead.
//! * **Keep advanced actions in structured menus or overflow rather than persistent clutter, while
//!   preserving discoverability and keyboard access.** [`resolve_local_action_cluster`] degrades when
//!   the actions are hover-only, keyboard access is lost, advanced actions are kept as persistent
//!   clutter instead of an overflow / structured menu, the action placement is unresolved, or an
//!   overflowed action is silently dropped rather than routed to overflow.
//! * **Prevent headers from overstating readiness, and keep panel identity and action semantics
//!   stable under compaction and responsive collapse.** [`resolve_panel_header`] degrades when a
//!   qualified pane is presented as current / ready; [`resolve_local_action_cluster`] degrades when
//!   compaction re-instantiates a different surface or loses the panel identity or its action
//!   semantics.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ActiveContextState`]
//! active-context vocabulary and the [`M5LocalActionBudget`] local-action-budget vocabulary — so
//! shell, explorer, search, review, request/data, help, and support surfaces can never fork their own
//! active-context or action wording. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_panel_header_local_action_cluster_controls,
    seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed,
    seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed,
    M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    M5ActiveContextState, M5LocalActionBudget, M5NavigationContentAccessibilityRoute,
    M5NavigationContentComponentFamily, M5NavigationContentConsumerSurface,
    M5NavigationContentDeploymentLine, M5NavigationContentDowngradeTrigger,
    M5NavigationContentQualificationClass, M5NavigationContentRequiredLabel,
    M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF, M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
    M5_PANEL_HEADER_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5PanelControlsPacket`].
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_RECORD_KIND: &str =
    "implement_m5_panel_header_and_local_action_cluster_controls";

/// Schema version for M5 panel-header and local-action-cluster controls records.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-panel-header-local-action-cluster-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_DOC_REF: &str =
    "docs/navigation/m5_panel_header_and_local_action_cluster_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-panel-header-local-action-cluster-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-panel-header-local-action-cluster-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-panel-header-local-action-cluster-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-panel-header-local-action-cluster-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5PanelConsumerSurface = M5NavigationContentConsumerSurface;

/// Controlled source / freshness posture a panel header names at the pane boundary, so a cached,
/// partial, stale, remote, or provider-owned pane never masquerades as current, first-party, live
/// content. Minted by this lane because source / freshness at the pane boundary is a panel-header
/// property the frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelSourceFreshness {
    /// The pane owns current, live, first-party content.
    Current,
    /// The pane shows cached content.
    Cached,
    /// Only a partial view of the content could be loaded.
    Partial,
    /// The content is stale relative to the source of truth.
    Stale,
    /// The content is owned by a remote source.
    Remote,
    /// The content is owned by a third-party provider.
    ProviderOwned,
    /// The source / freshness cannot currently be resolved.
    FreshnessUnknown,
}

impl M5PanelSourceFreshness {
    /// Every source / freshness posture, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Current,
        Self::Cached,
        Self::Partial,
        Self::Stale,
        Self::Remote,
        Self::ProviderOwned,
        Self::FreshnessUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Cached => "cached",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Remote => "remote",
            Self::ProviderOwned => "provider_owned",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether the pane carries a non-current qualification that must be labelled directly at the
    /// pane boundary, never presented as current, live, first-party content.
    pub const fn is_qualified(self) -> bool {
        matches!(
            self,
            Self::Cached | Self::Partial | Self::Stale | Self::Remote | Self::ProviderOwned
        )
    }

    /// Whether the source / freshness resolved to a concrete posture.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::FreshnessUnknown)
    }
}

/// Controlled placement of a local-action cluster's actions, so advanced actions are kept in
/// structured menus or overflow rather than persistent clutter. Minted by this lane because action
/// placement is a panel-header property the frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelActionPlacement {
    /// A small number of primary actions rendered inline.
    InlinePrimary,
    /// Advanced actions grouped into a structured menu.
    StructuredMenu,
    /// Advanced actions grouped into an overflow menu.
    OverflowMenu,
    /// A primary action inline plus the rest in overflow.
    MixedPrimaryOverflow,
    /// No local actions on this cluster.
    NoActions,
    /// The action placement cannot currently be resolved.
    PlacementUnknown,
}

impl M5PanelActionPlacement {
    /// Every action placement, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InlinePrimary,
        Self::StructuredMenu,
        Self::OverflowMenu,
        Self::MixedPrimaryOverflow,
        Self::NoActions,
        Self::PlacementUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlinePrimary => "inline_primary",
            Self::StructuredMenu => "structured_menu",
            Self::OverflowMenu => "overflow_menu",
            Self::MixedPrimaryOverflow => "mixed_primary_overflow",
            Self::NoActions => "no_actions",
            Self::PlacementUnknown => "placement_unknown",
        }
    }

    /// Whether the action placement resolved to a concrete answer.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::PlacementUnknown)
    }
}

/// Controlled compaction / responsive-collapse mode of a panel header, so a compacted or collapsed
/// header preserves the same panel identity and action semantics instead of re-instantiating a
/// different surface. Minted by this lane because compaction mode is a panel-header property the
/// frozen matrix does not enumerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelCompactionMode {
    /// The full header, no compaction.
    FullHeader,
    /// A compacted header with tightened spacing.
    CompactHeader,
    /// A header whose actions collapsed into an overflow menu.
    CollapsedToOverflow,
    /// A responsively reflowed header.
    ResponsiveReflow,
    /// A minimized rail representation of the header.
    MinimizedRail,
    /// The compaction mode cannot currently be resolved.
    CompactionUnknown,
}

impl M5PanelCompactionMode {
    /// Every compaction mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullHeader,
        Self::CompactHeader,
        Self::CollapsedToOverflow,
        Self::ResponsiveReflow,
        Self::MinimizedRail,
        Self::CompactionUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullHeader => "full_header",
            Self::CompactHeader => "compact_header",
            Self::CollapsedToOverflow => "collapsed_to_overflow",
            Self::ResponsiveReflow => "responsive_reflow",
            Self::MinimizedRail => "minimized_rail",
            Self::CompactionUnknown => "compaction_unknown",
        }
    }

    /// Whether this mode is an actively compacted / collapsed representation (not the full header),
    /// where panel identity and action semantics must be explicitly preserved.
    pub const fn is_compacted(self) -> bool {
        matches!(
            self,
            Self::CompactHeader
                | Self::CollapsedToOverflow
                | Self::ResponsiveReflow
                | Self::MinimizedRail
        )
    }

    /// Whether the compaction mode resolved to a concrete answer.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::CompactionUnknown)
    }
}

/// One mandatory rendered part a panel header or local-action cluster must be able to show, so no
/// title, active-context, freshness, action-placement, overflow, or compaction fact is left implicit
/// behind compact chrome or pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed navigation / content disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The stable title slot.
    TitleSlot,
    /// The active context named by the header.
    ActiveContext,
    /// The source / freshness cue at the pane boundary.
    SourceFreshnessCue,
    /// The bounded local-action budget.
    LocalActionBudget,
    /// The action placement (inline / structured menu / overflow).
    ActionPlacement,
    /// The overflow menu that keeps spilled actions reachable.
    OverflowMenu,
    /// The compaction / responsive-collapse mode.
    CompactionMode,
    /// The command-backed refresh affordance.
    RefreshCommand,
    /// The command-backed reveal / detail affordance.
    DetailCommand,
    /// The reference back to the canonical count / selection model.
    CanonicalModelReference,
}

impl M5PanelAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::TitleSlot,
        Self::ActiveContext,
        Self::SourceFreshnessCue,
        Self::LocalActionBudget,
        Self::ActionPlacement,
        Self::OverflowMenu,
        Self::CompactionMode,
        Self::RefreshCommand,
        Self::DetailCommand,
        Self::CanonicalModelReference,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::TitleSlot => "title_slot",
            Self::ActiveContext => "active_context",
            Self::SourceFreshnessCue => "source_freshness_cue",
            Self::LocalActionBudget => "local_action_budget",
            Self::ActionPlacement => "action_placement",
            Self::OverflowMenu => "overflow_menu",
            Self::CompactionMode => "compaction_mode",
            Self::RefreshCommand => "refresh_command",
            Self::DetailCommand => "detail_command",
            Self::CanonicalModelReference => "canonical_model_reference",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to inspect the pane's
/// title, active context, freshness, actions, or compaction behind a degraded component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelNextAction {
    /// Open the command-backed pane / scope detail.
    OpenPanelDetail,
    /// Refresh the pane content through the command-backed refresh.
    RefreshPanelContent,
    /// Inspect the active context and source / freshness.
    InspectActiveContextAndFreshness,
    /// Inspect the local actions and overflow.
    InspectActionsAndOverflow,
    /// Review the compaction mode or panel identity.
    ReviewCompactionOrIdentity,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
}

impl M5PanelNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenPanelDetail,
        Self::RefreshPanelContent,
        Self::InspectActiveContextAndFreshness,
        Self::InspectActionsAndOverflow,
        Self::ReviewCompactionOrIdentity,
        Self::ReviewDiagnostics,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenPanelDetail => "open_panel_detail",
            Self::RefreshPanelContent => "refresh_panel_content",
            Self::InspectActiveContextAndFreshness => "inspect_active_context_and_freshness",
            Self::InspectActionsAndOverflow => "inspect_actions_and_overflow",
            Self::ReviewCompactionOrIdentity => "review_compaction_or_identity",
            Self::ReviewDiagnostics => "review_diagnostics",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelExportField {
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
    /// The active-context state named by the header.
    ActiveContextState,
    /// The source / freshness named by the header.
    SourceFreshness,
    /// The action placement named by the cluster.
    ActionPlacement,
    /// The local-action budget named by the cluster.
    LocalActionBudget,
    /// The compaction mode named by the cluster.
    CompactionMode,
    /// The accountable owner role.
    OwnerRole,
}

impl M5PanelExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ActiveContextState,
        Self::SourceFreshness,
        Self::ActionPlacement,
        Self::LocalActionBudget,
        Self::CompactionMode,
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
            Self::ActiveContextState => "active_context_state",
            Self::SourceFreshness => "source_freshness",
            Self::ActionPlacement => "action_placement",
            Self::LocalActionBudget => "local_action_budget",
            Self::CompactionMode => "compaction_mode",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a panel header degraded below a clean, low-noise state. The degrade-first ladder returns one
/// of these instead of ever letting an ambiguous header read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PanelHeaderDegradeReason {
    /// The header title / current-object identity is unstated.
    HeaderTitleUnstated,
    /// The title slot is not stable (swapped for a transient status string).
    TitleSlotUnstable,
    /// The active context cannot currently be resolved.
    ActiveContextUnresolved,
    /// A background / preview context is presented as the active one.
    BackgroundContextShownAsActive,
    /// The source / freshness cannot currently be resolved.
    SourceFreshnessUnresolved,
    /// A cached / partial / stale / remote / provider-owned pane hides its freshness cue at the
    /// boundary.
    FreshnessCueMissing,
    /// A qualified (cached / partial / stale / remote / provider-owned) pane is presented as current /
    /// ready.
    ReadinessOverstated,
    /// The header re-encodes the canonical count / selection model in surface-local copy instead of
    /// pointing back to it.
    ReEncodesCanonicalCountsLocally,
    /// No command-backed refresh affordance is reachable.
    RefreshCommandMissing,
    /// No command-backed path to reveal / trace the pane detail is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PanelHeaderDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::HeaderTitleUnstated,
        Self::TitleSlotUnstable,
        Self::ActiveContextUnresolved,
        Self::BackgroundContextShownAsActive,
        Self::SourceFreshnessUnresolved,
        Self::FreshnessCueMissing,
        Self::ReadinessOverstated,
        Self::ReEncodesCanonicalCountsLocally,
        Self::RefreshCommandMissing,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderTitleUnstated => "header_title_unstated",
            Self::TitleSlotUnstable => "title_slot_unstable",
            Self::ActiveContextUnresolved => "active_context_unresolved",
            Self::BackgroundContextShownAsActive => "background_context_shown_as_active",
            Self::SourceFreshnessUnresolved => "source_freshness_unresolved",
            Self::FreshnessCueMissing => "freshness_cue_missing",
            Self::ReadinessOverstated => "readiness_overstated",
            Self::ReEncodesCanonicalCountsLocally => "re_encodes_canonical_counts_locally",
            Self::RefreshCommandMissing => "refresh_command_missing",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PanelNextAction {
        match self {
            Self::HeaderTitleUnstated | Self::TitleSlotUnstable => {
                M5PanelNextAction::ReviewCompactionOrIdentity
            }
            Self::ActiveContextUnresolved
            | Self::BackgroundContextShownAsActive
            | Self::SourceFreshnessUnresolved
            | Self::FreshnessCueMissing
            | Self::ReadinessOverstated => M5PanelNextAction::InspectActiveContextAndFreshness,
            Self::ReEncodesCanonicalCountsLocally => M5PanelNextAction::InspectActionsAndOverflow,
            Self::RefreshCommandMissing => M5PanelNextAction::RefreshPanelContent,
            Self::ContextTracePathMissing => M5PanelNextAction::OpenPanelDetail,
            Self::ProofStale => M5PanelNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::ActiveContextUnresolved | Self::BackgroundContextShownAsActive => {
                M5NavigationContentDowngradeTrigger::ActiveContextUnstated
            }
            Self::HeaderTitleUnstated
            | Self::TitleSlotUnstable
            | Self::SourceFreshnessUnresolved
            | Self::FreshnessCueMissing
            | Self::ReadinessOverstated
            | Self::ReEncodesCanonicalCountsLocally
            | Self::RefreshCommandMissing
            | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a local-action cluster degraded below a clean, discoverable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalActionClusterDegradeReason {
    /// The cluster / current-object identity is unstated.
    ClusterIdentityUnstated,
    /// The local-action budget cannot currently be resolved.
    ActionBudgetUnresolved,
    /// The local actions can only be discovered by pointer hover.
    LocalActionsHoverOnly,
    /// Keyboard access to the actions (including the overflow menu) is lost.
    KeyboardAccessMissing,
    /// Advanced actions are kept as persistent clutter instead of a structured / overflow menu.
    AdvancedActionsPersistentClutter,
    /// The action placement cannot currently be resolved.
    ActionPlacementUnresolved,
    /// An overflowed local action was silently dropped rather than routed to overflow.
    OverflowedActionDropped,
    /// Compaction / responsive collapse re-instantiated a different surface.
    CompactionReinstantiatesSurface,
    /// Compaction / responsive collapse lost the panel identity.
    CompactionLosesPanelIdentity,
    /// Compaction / responsive collapse lost the action semantics.
    CompactionLosesActionSemantics,
    /// No command-backed path to reveal / trace the pane detail is reachable.
    ContextTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5LocalActionClusterDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ClusterIdentityUnstated,
        Self::ActionBudgetUnresolved,
        Self::LocalActionsHoverOnly,
        Self::KeyboardAccessMissing,
        Self::AdvancedActionsPersistentClutter,
        Self::ActionPlacementUnresolved,
        Self::OverflowedActionDropped,
        Self::CompactionReinstantiatesSurface,
        Self::CompactionLosesPanelIdentity,
        Self::CompactionLosesActionSemantics,
        Self::ContextTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClusterIdentityUnstated => "cluster_identity_unstated",
            Self::ActionBudgetUnresolved => "action_budget_unresolved",
            Self::LocalActionsHoverOnly => "local_actions_hover_only",
            Self::KeyboardAccessMissing => "keyboard_access_missing",
            Self::AdvancedActionsPersistentClutter => "advanced_actions_persistent_clutter",
            Self::ActionPlacementUnresolved => "action_placement_unresolved",
            Self::OverflowedActionDropped => "overflowed_action_dropped",
            Self::CompactionReinstantiatesSurface => "compaction_reinstantiates_surface",
            Self::CompactionLosesPanelIdentity => "compaction_loses_panel_identity",
            Self::CompactionLosesActionSemantics => "compaction_loses_action_semantics",
            Self::ContextTracePathMissing => "context_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PanelNextAction {
        match self {
            Self::ClusterIdentityUnstated => M5PanelNextAction::ReviewCompactionOrIdentity,
            Self::ActionBudgetUnresolved
            | Self::LocalActionsHoverOnly
            | Self::KeyboardAccessMissing
            | Self::AdvancedActionsPersistentClutter
            | Self::ActionPlacementUnresolved
            | Self::OverflowedActionDropped => M5PanelNextAction::InspectActionsAndOverflow,
            Self::CompactionReinstantiatesSurface
            | Self::CompactionLosesPanelIdentity
            | Self::CompactionLosesActionSemantics => M5PanelNextAction::ReviewCompactionOrIdentity,
            Self::ContextTracePathMissing => M5PanelNextAction::OpenPanelDetail,
            Self::ProofStale => M5PanelNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            Self::LocalActionsHoverOnly | Self::KeyboardAccessMissing => {
                M5NavigationContentDowngradeTrigger::LocalActionsHoverOnly
            }
            Self::AdvancedActionsPersistentClutter | Self::OverflowedActionDropped => {
                M5NavigationContentDowngradeTrigger::PanelHeaderOverloaded
            }
            Self::ClusterIdentityUnstated
            | Self::ActionBudgetUnresolved
            | Self::ActionPlacementUnresolved
            | Self::CompactionReinstantiatesSurface
            | Self::CompactionLosesPanelIdentity
            | Self::CompactionLosesActionSemantics
            | Self::ContextTracePathMissing => {
                M5NavigationContentDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_panel_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PanelHeaderResolutionInput {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The current header / title-slot label shown; empty means unstated.
    pub header_label: String,
    /// True when the title slot stays stable (never swapped for a transient status string).
    pub title_slot_stable: bool,
    /// The active-context state of the panel.
    pub active_context: M5ActiveContextState,
    /// True when a background / preview context is presented as the active one.
    pub background_context_shown_as_active: bool,
    /// The source / freshness posture of the pane's content.
    pub source_freshness: M5PanelSourceFreshness,
    /// True when the freshness cue is shown directly at the pane boundary.
    pub freshness_cue_shown: bool,
    /// True when a qualified pane is presented as current / ready.
    pub overstates_readiness: bool,
    /// True when the header points back to the canonical count / selection model.
    pub references_canonical_model: bool,
    /// True when the header re-encodes the canonical count / selection model in surface-local copy.
    pub re_encodes_canonical_counts_locally: bool,
    /// True when a command-backed refresh affordance is reachable.
    pub refresh_command_available: bool,
    /// True when a command-backed entrypoint to reveal / trace the pane detail is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe panel-header projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPanelHeader {
    /// Stable identity of the header instance.
    pub header_id: String,
    /// The title-slot label named by the header.
    pub header_label: String,
    /// Whether the title slot stays stable.
    pub title_slot_stable: bool,
    /// The active-context token named by the header.
    pub active_context: String,
    /// Guardrail (MUST be `false` on a clean header): a background context reads as active.
    pub background_context_shown_as_active: bool,
    /// The source / freshness token named by the header.
    pub source_freshness: String,
    /// Whether the pane carries a non-current qualification that must be labelled at the boundary.
    pub source_is_qualified: bool,
    /// Whether the freshness cue is shown directly at the pane boundary.
    pub freshness_cue_shown: bool,
    /// Guardrail (MUST be `false` on a clean header): a qualified pane hides its freshness cue.
    pub freshness_cue_missing: bool,
    /// Guardrail (MUST be `false` on a clean header): a qualified pane is presented as current / ready.
    pub readiness_overstated: bool,
    /// Whether the header points back to the canonical count / selection model.
    pub references_canonical_model: bool,
    /// Guardrail (MUST be `false` on a clean header): the header re-encodes counts locally.
    pub re_encodes_canonical_counts_locally: bool,
    /// Whether a command-backed refresh affordance is reachable.
    pub refresh_command_available: bool,
    /// Whether a command-backed entrypoint to reveal / trace the pane detail is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the header could not read as a clean, low-noise state.
    pub degrade_reason: Option<M5PanelHeaderDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PanelNextAction,
    /// Whether the header is legible at a glance (clean header naming every fact).
    pub header_legible_at_a_glance: bool,
}

impl M5ResolvedPanelHeader {
    /// Whether this header reads as a clean, low-noise state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_local_action_cluster`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LocalActionClusterResolutionInput {
    /// Stable identity of the cluster instance.
    pub cluster_id: String,
    /// The current cluster / current-object label shown; empty means unstated.
    pub cluster_label: String,
    /// The local-action budget for the cluster.
    pub local_action_budget: M5LocalActionBudget,
    /// True when the local actions can only be discovered by pointer hover.
    pub local_actions_hover_only: bool,
    /// True when every action (including the overflow menu) is keyboard-reachable.
    pub keyboard_reachable: bool,
    /// True when advanced actions are kept as persistent clutter instead of a structured / overflow
    /// menu.
    pub advanced_actions_persistent_clutter: bool,
    /// The action placement of the cluster.
    pub action_placement: M5PanelActionPlacement,
    /// True when an overflowed local action was silently dropped rather than routed to overflow.
    pub overflowed_action_dropped: bool,
    /// The compaction / responsive-collapse mode.
    pub compaction_mode: M5PanelCompactionMode,
    /// True when compaction re-instantiated a different surface.
    pub reinstantiates_different_surface: bool,
    /// True when compaction preserves the panel identity.
    pub compaction_preserves_identity: bool,
    /// True when compaction preserves the action semantics.
    pub compaction_preserves_action_semantics: bool,
    /// True when a command-backed entrypoint to reveal / trace the pane detail is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe local-action-cluster projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLocalActionCluster {
    /// Stable identity of the cluster instance.
    pub cluster_id: String,
    /// The current cluster / current-object label named by the cluster.
    pub cluster_label: String,
    /// The local-action-budget token named by the cluster.
    pub local_action_budget: String,
    /// Guardrail (MUST be `false` on a clean cluster): the local actions are hover-only.
    pub local_actions_hover_only: bool,
    /// Whether every action (including the overflow menu) is keyboard-reachable.
    pub keyboard_reachable: bool,
    /// Guardrail (MUST be `false` on a clean cluster): advanced actions are persistent clutter.
    pub advanced_actions_persistent_clutter: bool,
    /// The action-placement token named by the cluster.
    pub action_placement: String,
    /// Guardrail (MUST be `false` on a clean cluster): an overflowed action was dropped.
    pub overflowed_action_dropped: bool,
    /// The compaction-mode token named by the cluster.
    pub compaction_mode: String,
    /// Whether the compaction mode is an actively compacted / collapsed representation.
    pub compaction_is_compacted: bool,
    /// Guardrail (MUST be `false` on a clean cluster): compaction re-instantiates a different surface.
    pub reinstantiates_different_surface: bool,
    /// Guardrail (MUST be `false` on a clean cluster): compaction loses the panel identity.
    pub compaction_loses_identity: bool,
    /// Guardrail (MUST be `false` on a clean cluster): compaction loses the action semantics.
    pub compaction_loses_action_semantics: bool,
    /// Whether a command-backed entrypoint to reveal / trace the pane detail is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the cluster could not read as a clean, discoverable state.
    pub degrade_reason: Option<M5LocalActionClusterDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PanelNextAction,
    /// Whether the cluster is legible at a glance (clean cluster naming every fact).
    pub cluster_legible_at_a_glance: bool,
}

impl M5ResolvedLocalActionCluster {
    /// Whether this cluster reads as a clean, discoverable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5PanelResolutionError {
    /// The header id was empty.
    EmptyHeaderId,
    /// The cluster id was empty.
    EmptyClusterId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5PanelResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyHeaderId => "empty_header_id",
            Self::EmptyClusterId => "empty_cluster_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5PanelResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 panel-header / local-action-cluster resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PanelResolutionError {}

/// Resolves a panel header so it is a stable, low-noise context marker: it names its stable title and
/// active context (never presenting a background context as active), labels a cached / partial /
/// stale / remote / provider-owned pane directly at the boundary (never overstating readiness), points
/// back to one canonical count / selection model instead of re-encoding counts in surface-local copy,
/// and offers command-backed refresh and reveal / detail affordances.
pub fn resolve_panel_header(
    input: M5PanelHeaderResolutionInput,
) -> Result<M5ResolvedPanelHeader, M5PanelResolutionError> {
    if input.header_id.trim().is_empty() {
        return Err(M5PanelResolutionError::EmptyHeaderId);
    }
    if string_is_forbidden(&input.header_id) || string_is_forbidden(&input.header_label) {
        return Err(M5PanelResolutionError::ForbiddenMaterial);
    }

    let source_is_qualified = input.source_freshness.is_qualified();
    let freshness_cue_missing = source_is_qualified && !input.freshness_cue_shown;
    let readiness_overstated = source_is_qualified && input.overstates_readiness;

    let degrade_reason = if input.header_label.trim().is_empty() {
        Some(M5PanelHeaderDegradeReason::HeaderTitleUnstated)
    } else if !input.title_slot_stable {
        Some(M5PanelHeaderDegradeReason::TitleSlotUnstable)
    } else if matches!(
        input.active_context,
        M5ActiveContextState::ContextUnresolved
    ) {
        Some(M5PanelHeaderDegradeReason::ActiveContextUnresolved)
    } else if input.background_context_shown_as_active {
        Some(M5PanelHeaderDegradeReason::BackgroundContextShownAsActive)
    } else if !input.source_freshness.is_resolved() {
        Some(M5PanelHeaderDegradeReason::SourceFreshnessUnresolved)
    } else if freshness_cue_missing {
        Some(M5PanelHeaderDegradeReason::FreshnessCueMissing)
    } else if readiness_overstated {
        Some(M5PanelHeaderDegradeReason::ReadinessOverstated)
    } else if input.re_encodes_canonical_counts_locally || !input.references_canonical_model {
        Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    } else if !input.refresh_command_available {
        Some(M5PanelHeaderDegradeReason::RefreshCommandMissing)
    } else if !input.detail_command_available {
        Some(M5PanelHeaderDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5PanelHeaderDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PanelNextAction::OpenPanelDetail,
    };

    Ok(M5ResolvedPanelHeader {
        header_id: input.header_id,
        header_label: input.header_label,
        title_slot_stable: input.title_slot_stable,
        active_context: input.active_context.as_str().to_owned(),
        background_context_shown_as_active: input.background_context_shown_as_active,
        source_freshness: input.source_freshness.as_str().to_owned(),
        source_is_qualified,
        freshness_cue_shown: input.freshness_cue_shown,
        freshness_cue_missing,
        readiness_overstated,
        references_canonical_model: input.references_canonical_model,
        re_encodes_canonical_counts_locally: input.re_encodes_canonical_counts_locally,
        refresh_command_available: input.refresh_command_available,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        header_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a local-action cluster so its actions stay discoverable and keyboard-reachable: advanced
/// actions live in a structured / overflow menu rather than persistent clutter, an overflowed action
/// is never silently dropped, and compaction / responsive collapse preserves the panel identity and
/// action semantics instead of re-instantiating a different surface.
pub fn resolve_local_action_cluster(
    input: M5LocalActionClusterResolutionInput,
) -> Result<M5ResolvedLocalActionCluster, M5PanelResolutionError> {
    if input.cluster_id.trim().is_empty() {
        return Err(M5PanelResolutionError::EmptyClusterId);
    }
    if string_is_forbidden(&input.cluster_id) || string_is_forbidden(&input.cluster_label) {
        return Err(M5PanelResolutionError::ForbiddenMaterial);
    }

    let compacted = input.compaction_mode.is_compacted();
    let reinstantiates_different_surface = compacted && input.reinstantiates_different_surface;
    let compaction_loses_identity = compacted && !input.compaction_preserves_identity;
    let compaction_loses_action_semantics =
        compacted && !input.compaction_preserves_action_semantics;

    let degrade_reason = if input.cluster_label.trim().is_empty() {
        Some(M5LocalActionClusterDegradeReason::ClusterIdentityUnstated)
    } else if matches!(
        input.local_action_budget,
        M5LocalActionBudget::BudgetUnknown
    ) {
        Some(M5LocalActionClusterDegradeReason::ActionBudgetUnresolved)
    } else if input.local_actions_hover_only {
        Some(M5LocalActionClusterDegradeReason::LocalActionsHoverOnly)
    } else if !input.keyboard_reachable {
        Some(M5LocalActionClusterDegradeReason::KeyboardAccessMissing)
    } else if input.advanced_actions_persistent_clutter {
        Some(M5LocalActionClusterDegradeReason::AdvancedActionsPersistentClutter)
    } else if !input.action_placement.is_resolved() {
        Some(M5LocalActionClusterDegradeReason::ActionPlacementUnresolved)
    } else if input.overflowed_action_dropped {
        Some(M5LocalActionClusterDegradeReason::OverflowedActionDropped)
    } else if !input.compaction_mode.is_resolved() {
        Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
    } else if reinstantiates_different_surface {
        Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
    } else if compaction_loses_identity {
        Some(M5LocalActionClusterDegradeReason::CompactionLosesPanelIdentity)
    } else if compaction_loses_action_semantics {
        Some(M5LocalActionClusterDegradeReason::CompactionLosesActionSemantics)
    } else if !input.detail_command_available {
        Some(M5LocalActionClusterDegradeReason::ContextTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5LocalActionClusterDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PanelNextAction::OpenPanelDetail,
    };

    Ok(M5ResolvedLocalActionCluster {
        cluster_id: input.cluster_id,
        cluster_label: input.cluster_label,
        local_action_budget: input.local_action_budget.as_str().to_owned(),
        local_actions_hover_only: input.local_actions_hover_only,
        keyboard_reachable: input.keyboard_reachable,
        advanced_actions_persistent_clutter: input.advanced_actions_persistent_clutter,
        action_placement: input.action_placement.as_str().to_owned(),
        overflowed_action_dropped: input.overflowed_action_dropped,
        compaction_mode: input.compaction_mode.as_str().to_owned(),
        compaction_is_compacted: compacted,
        reinstantiates_different_surface,
        compaction_loses_identity,
        compaction_loses_action_semantics,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        cluster_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved header and cluster examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5PanelConsumerSurface,
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
    pub anatomy_parts: Vec<M5PanelAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5PanelExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    /// Resolved panel-header examples.
    pub panel_header_examples: Vec<M5ResolvedPanelHeader>,
    /// Resolved local-action-cluster examples.
    pub local_action_cluster_examples: Vec<M5ResolvedLocalActionCluster>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the panel-header schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: local actions are never hover-only and keyboard access is never lost.
    pub hides_actions_behind_hover_only_or_loses_keyboard_access: bool,
    /// Hard invariant: readiness is never overstated and the source / freshness cue is never hidden.
    pub overstates_readiness_or_hides_source_freshness_cue: bool,
    /// Hard invariant: the header never overloads and advanced actions are never persistent clutter.
    pub overloads_header_or_keeps_advanced_actions_as_persistent_clutter: bool,
    /// Hard invariant: compaction never re-instantiates a surface or loses the panel identity.
    pub compaction_reinstantiates_surface_or_loses_panel_identity: bool,
}

impl M5PanelControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PanelAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5PanelAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PanelExportField> = self.export_fields.iter().copied().collect();
        M5PanelExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_actions_behind_hover_only_or_loses_keyboard_access
            && !self.overstates_readiness_or_hides_source_freshness_cue
            && !self.overloads_header_or_keeps_advanced_actions_as_persistent_clutter
            && !self.compaction_reinstantiates_surface_or_loses_panel_identity
    }

    /// True when every resolved example on this row is honest: no clean header hides its freshness
    /// cue, overstates readiness, re-encodes counts, or lacks a title / command; and no clean cluster
    /// hides actions behind hover, loses keyboard access, keeps persistent clutter, drops an overflow
    /// action, or re-instantiates / loses identity under compaction.
    fn examples_are_honest(&self) -> bool {
        self.panel_header_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (!ex.title_slot_stable
                    || ex.background_context_shown_as_active
                    || ex.freshness_cue_missing
                    || ex.readiness_overstated
                    || ex.re_encodes_canonical_counts_locally
                    || !ex.references_canonical_model
                    || !ex.refresh_command_available
                    || !ex.detail_command_available))
        }) && self.local_action_cluster_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.local_actions_hover_only
                    || !ex.keyboard_reachable
                    || ex.advanced_actions_persistent_clutter
                    || ex.overflowed_action_dropped
                    || ex.reinstantiates_different_surface
                    || ex.compaction_loses_identity
                    || ex.compaction_loses_action_semantics
                    || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelVocabularySet {
    /// Active-context-state tokens (bound from the frozen matrix).
    pub active_context_states: Vec<String>,
    /// Local-action-budget tokens (bound from the frozen matrix).
    pub local_action_budgets: Vec<String>,
    /// Source / freshness tokens (minted by this lane).
    pub source_freshness_kinds: Vec<String>,
    /// Action-placement tokens (minted by this lane).
    pub action_placements: Vec<String>,
    /// Compaction-mode tokens (minted by this lane).
    pub compaction_modes: Vec<String>,
    /// Panel-header degrade-reason tokens.
    pub panel_header_degrade_reasons: Vec<String>,
    /// Local-action-cluster degrade-reason tokens.
    pub local_action_cluster_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5PanelVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            active_context_states: tokens(&M5ActiveContextState::ALL, |v| v.as_str()),
            local_action_budgets: tokens(&M5LocalActionBudget::ALL, |v| v.as_str()),
            source_freshness_kinds: tokens(&M5PanelSourceFreshness::ALL, |v| v.as_str()),
            action_placements: tokens(&M5PanelActionPlacement::ALL, |v| v.as_str()),
            compaction_modes: tokens(&M5PanelCompactionMode::ALL, |v| v.as_str()),
            panel_header_degrade_reasons: tokens(&M5PanelHeaderDegradeReason::ALL, |v| v.as_str()),
            local_action_cluster_degrade_reasons: tokens(
                &M5LocalActionClusterDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5PanelAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PanelNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PanelExportField::ALL, |v| v.as_str()),
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
pub struct M5PanelGovernanceReview {
    /// The header names a stable title slot and active context.
    pub header_names_stable_title_and_active_context: bool,
    /// A cached / partial / stale / remote / provider-owned pane labels its freshness at the boundary.
    pub header_labels_source_freshness_at_boundary: bool,
    /// The header never overstates readiness for a qualified pane.
    pub header_never_overstates_readiness: bool,
    /// The header points back to one canonical count / selection model.
    pub header_references_canonical_count_and_selection_model: bool,
    /// The header offers command-backed refresh and reveal / detail affordances.
    pub header_offers_command_backed_refresh_and_detail: bool,
    /// Advanced actions live in structured menus / overflow rather than persistent clutter.
    pub advanced_actions_in_structured_menu_or_overflow: bool,
    /// Local actions stay keyboard-reachable and never hover-only.
    pub local_actions_keyboard_reachable_never_hover_only: bool,
    /// An overflowed action is never silently dropped.
    pub overflowed_action_never_dropped: bool,
    /// Compaction and responsive collapse preserve the panel identity and action semantics.
    pub compaction_preserves_panel_identity_and_action_semantics: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelConsumerProjection {
    /// Shell and explorer surfaces consume the shared header title / context / freshness grammar.
    pub shell_and_explorer_consume_shared_header_grammar: bool,
    /// Search and review surfaces consume the shared local-action / overflow grammar.
    pub search_and_review_consume_shared_action_grammar: bool,
    /// Request/data and help surfaces consume the same shared panel-header semantics.
    pub data_and_help_consume_shared_panel_header_semantics: bool,
    /// Header facts trace back to one canonical count / selection model.
    pub header_facts_trace_to_single_canonical_model: bool,
    /// Support / export reads a single canonical panel-header source.
    pub support_export_reads_single_panel_header_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PanelControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PanelControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PanelControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PanelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PanelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PanelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PanelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PanelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 panel-header and local-action-cluster controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PanelControlsPacket {
    /// Record kind; must equal [`M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PanelControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PanelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PanelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PanelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PanelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PanelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PanelControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5PanelControlsPacketInput) -> Self {
        Self {
            record_kind: M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5PanelControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_RECORD_KIND {
            violations.push(M5PanelControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_VERSION {
            violations.push(M5PanelControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PanelControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5PanelControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 panel-header / local-action-cluster controls packet serializes"),
        ) {
            violations.push(M5PanelControlsViolation::RawMaterialInExport);
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
            .expect("m5 panel-header / local-action-cluster controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,header_examples,cluster_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .panel_header_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.local_action_cluster_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.panel_header_examples.len(),
                row.local_action_cluster_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Panel-Header and Local-Action-Cluster Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Source / freshness kinds: {}\n",
            self.vocabulary_set.source_freshness_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Action placements: {}\n",
            self.vocabulary_set.action_placements.join(", ")
        ));
        out.push_str(&format!(
            "- Compaction modes: {}\n",
            self.vocabulary_set.compaction_modes.join(", ")
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
                "  - Panel-header examples: {} / local-action-cluster examples: {}\n",
                row.panel_header_examples.len(),
                row.local_action_cluster_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5PanelControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PanelControlsViolation>),
}

impl fmt::Display for M5PanelControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 panel-header / local-action-cluster controls export parse failed: {error}"
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
                    "m5 panel-header / local-action-cluster controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PanelControlsArtifactError {}

/// Validation failures emitted by [`M5PanelControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PanelControlsViolation {
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
    /// A controls row does not point at the panel-header schema.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (hover-only, overstated, dropped, or
    /// surface-swapping).
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
    /// One shared header grammar is not proven: clean header examples do not reuse the same title /
    /// action / overflow / freshness grammar across surfaces, or no freshness-cue-missing or
    /// readiness-overstated example degrades.
    OneHeaderGrammarNotProven,
    /// Compaction identity and action semantics are not proven: no compacted cluster preserves
    /// identity and semantics, or no reinstantiate-surface / loses-identity example degrades.
    CompactionIdentityAndActionSemanticsNotProven,
    /// A low-noise but sufficient header is not proven: no clean header names ownership and freshness
    /// while pointing at the canonical model, or no re-encode / persistent-clutter example degrades.
    LowNoiseSufficientHeaderNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PanelControlsViolation {
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
            Self::OneHeaderGrammarNotProven => "one_header_grammar_not_proven",
            Self::CompactionIdentityAndActionSemanticsNotProven => {
                "compaction_identity_and_action_semantics_not_proven"
            }
            Self::LowNoiseSufficientHeaderNotProven => "low_noise_sufficient_header_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_panel_header_local_action_cluster_controls_export(
) -> Result<M5PanelControlsPacket, M5PanelControlsArtifactError> {
    let packet: M5PanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-panel-header-local-action-cluster-controls-proof/support_export.json"
    )))
    .map_err(M5PanelControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PanelControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_REF,
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PanelControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5PanelControlsViolation::NoControlsRows);
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
            violations.push(M5PanelControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5PanelControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PanelControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_PANEL_HEADER_SCHEMA_REF) {
            violations.push(M5PanelControlsViolation::ComponentSchemaRefMissing);
        }
        if row.panel_header_examples.is_empty() || row.local_action_cluster_examples.is_empty() {
            violations.push(M5PanelControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5PanelControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5PanelControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.header_names_stable_title_and_active_context,
        review.header_labels_source_freshness_at_boundary,
        review.header_never_overstates_readiness,
        review.header_references_canonical_count_and_selection_model,
        review.header_offers_command_backed_refresh_and_detail,
        review.advanced_actions_in_structured_menu_or_overflow,
        review.local_actions_keyboard_reachable_never_hover_only,
        review.overflowed_action_never_dropped,
        review.compaction_preserves_panel_identity_and_action_semantics,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5PanelControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_explorer_consume_shared_header_grammar,
        projection.search_and_review_consume_shared_action_grammar,
        projection.data_and_help_consume_shared_panel_header_semantics,
        projection.header_facts_trace_to_single_canonical_model,
        projection.support_export_reads_single_panel_header_source,
    ] {
        if !ok {
            violations.push(M5PanelControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PanelControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PanelControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5PanelControlsPacket,
    violations: &mut Vec<M5PanelControlsViolation>,
) {
    let headers = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.panel_header_examples.iter())
    };
    let clusters = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.local_action_cluster_examples.iter())
    };

    // AC1: claimed M5 panes show one header grammar for title, local actions, overflow, and
    // freshness / source cues. Clean header examples cover at least two distinct source-freshness
    // postures, a freshness-cue-missing and a readiness-overstated example both degrade, and no clean
    // header hides its freshness cue or overstates readiness.
    let clean_freshness_kinds: BTreeSet<String> = headers()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.source_freshness.clone())
        .collect();
    let cue_missing_degrades = headers()
        .any(|ex| ex.degrade_reason == Some(M5PanelHeaderDegradeReason::FreshnessCueMissing));
    let readiness_overstated_degrades = headers()
        .any(|ex| ex.degrade_reason == Some(M5PanelHeaderDegradeReason::ReadinessOverstated));
    let no_clean_cue_missing_or_overstated = headers()
        .all(|ex| !(ex.is_clean() && (ex.freshness_cue_missing || ex.readiness_overstated)));
    if !(clean_freshness_kinds.len() >= 2
        && cue_missing_degrades
        && readiness_overstated_degrades
        && no_clean_cue_missing_or_overstated)
    {
        violations.push(M5PanelControlsViolation::OneHeaderGrammarNotProven);
    }

    // AC2: compaction and responsive collapse preserve the same panel identity and action semantics
    // instead of re-instantiating a different surface. At least one clean cluster is actively
    // compacted while preserving identity and semantics, a reinstantiate-surface and a loses-identity
    // example both degrade, and no clean cluster re-instantiates a surface or loses identity.
    let clean_compacted_preserving = clusters().any(|ex| {
        ex.is_clean()
            && ex.compaction_is_compacted
            && !ex.reinstantiates_different_surface
            && !ex.compaction_loses_identity
            && !ex.compaction_loses_action_semantics
    });
    let reinstantiate_degrades = clusters().any(|ex| {
        ex.degrade_reason
            == Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
    });
    let loses_identity_degrades = clusters().any(|ex| {
        ex.degrade_reason == Some(M5LocalActionClusterDegradeReason::CompactionLosesPanelIdentity)
    });
    let no_clean_reinstantiate_or_lost = clusters().all(|ex| {
        !(ex.is_clean() && (ex.reinstantiates_different_surface || ex.compaction_loses_identity))
    });
    if !(clean_compacted_preserving
        && reinstantiate_degrades
        && loses_identity_degrades
        && no_clean_reinstantiate_or_lost)
    {
        violations.push(M5PanelControlsViolation::CompactionIdentityAndActionSemanticsNotProven);
    }

    // AC3: headers remain low-noise but sufficient — a clean header explains what content the pane
    // owns (references the canonical model) and whether it is current enough to trust (names its
    // source / freshness), while a re-encode header and a persistent-clutter cluster both degrade,
    // and no clean header re-encodes counts and no clean cluster keeps persistent clutter.
    let clean_low_noise_sufficient = headers()
        .any(|ex| ex.is_clean() && ex.references_canonical_model && ex.refresh_command_available);
    let re_encode_degrades = headers().any(|ex| {
        ex.degrade_reason == Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    });
    let persistent_clutter_degrades = clusters().any(|ex| {
        ex.degrade_reason
            == Some(M5LocalActionClusterDegradeReason::AdvancedActionsPersistentClutter)
    });
    let no_clean_re_encode = headers().all(|ex| {
        !(ex.is_clean()
            && (ex.re_encodes_canonical_counts_locally || !ex.references_canonical_model))
    });
    let no_clean_persistent_clutter =
        clusters().all(|ex| !(ex.is_clean() && ex.advanced_actions_persistent_clutter));
    if !(clean_low_noise_sufficient
        && re_encode_degrades
        && persistent_clutter_degrades
        && no_clean_re_encode
        && no_clean_persistent_clutter)
    {
        violations.push(M5PanelControlsViolation::LowNoiseSufficientHeaderNotProven);
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

/// The component family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5NavigationContentComponentFamily; 1] =
    [M5NavigationContentComponentFamily::PanelHeader];
