//! Shared search / graph, review, request / data, help, support, and export
//! consumers for the frozen M5 navigation-content component families (tab strip,
//! breadcrumbs, tree view, list view, table / grid, and panel header).
//!
//! This module is the M05-1113 consumer-adoption lane over the frozen M5
//! navigation-content component matrix
//! ([`crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix`]).
//! Where the freeze matrix defines the six reusable tab-strip, breadcrumbs,
//! tree-view, list-view, table/grid, and panel-header primitives — and the four
//! B132 implement lanes wire their resolvers and controls contracts — this lane
//! proves those families are shared product *primitives* rather than per-surface
//! container chrome. It adopts them across the claimed M5 navigation / content
//! consumer classes:
//!
//! 1. a shell / explorer surface,
//! 2. a search / graph surface,
//! 3. a review surface,
//! 4. a request / data surface,
//! 5. a help center surface (AC2 docs/help reference), and
//! 6. a support / export + release-packet lane (AC2).
//!
//! Each [`NavContentConsumerRow`] points back to exactly one canonical component
//! family (its per-family matrix schema) and the one canonical controls contract
//! (schema + doc + release-proof artifact) its family group belongs to, instead
//! of cloning surface-local navigation chrome. Every consumer — even a read-only,
//! inspect-only, or export-only projection — keeps the identical active-context,
//! hierarchy/path, disclosure, selection-versus-current, pinned/preview/read-only,
//! exact/loaded/all-matching count-scope, local-action-budget, and
//! overflow/freshness labels and the identical frozen navigation-content
//! disposition vocabulary. A narrower consumer discloses the reduction with a
//! reduced-capability banner (and, when it punts to another surface, a desktop /
//! companion / support-packet note) rather than renaming or dropping governed
//! navigation truth, so explorer trees, result lists, tab sets, dense tables,
//! breadcrumbs, and headers never fork navigation-content vocabulary by surface.
//! This is what makes the same active-context / count / hierarchy state render
//! with one vocabulary and one component family across every claimed consumer
//! (AC1), and lets help / support / release packets drop bespoke per-surface
//! prose (AC2).
//!
//! The five spec guardrails are enforced per row and must all stay false: no
//! consumer lets tabs masquerade as top-level workflow navigation; no consumer
//! hides counts or blocked rows behind ambiguous ellipses; no consumer makes
//! tree / list / table actions hover-only; no consumer lets a panel header become
//! a cluttered secondary toolbar; and no consumer collapses exact, loaded, and
//! all-matching count scopes into one vague total.
//!
//! The packet is metadata-only: it carries only typed class tokens, opaque
//! navigation-state refs, booleans, and redacted labels — never raw user content,
//! credentials, or tokens.
//!
//! The boundary schema is
//! [`schemas/ui/m5-navigation-content-component-consumer.schema.json`](../../../../schemas/ui/m5-navigation-content-component-consumer.schema.json).
//! The contract doc is
//! [`docs/navigation/m5_navigation_content_component_consumer_contract.md`](../../../../docs/navigation/m5_navigation_content_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix as matrix;
use crate::implement_the_m5_panel_header_and_local_action_cluster_stable_title_overflow_rule_source_freshness_cue_and_command_backed_action_primitive as panel_header_controls;
use crate::implement_the_m5_table_grid_and_panel_header_sort_filter_provenance_selection_bar_pinned_column_identity_value_qualification_and_count_scope_primitive as table_grid_controls;
use crate::implement_the_m5_tab_strip_and_breadcrumbs_active_context_item_state_hierarchy_path_source_aware_context_and_no_top_level_navigation_drift_primitive as tab_strip_controls;
use crate::implement_the_m5_tree_view_and_list_view_virtualization_disclosure_selection_focus_inline_action_budget_and_exact_loaded_hidden_scope_primitive as tree_view_controls;

pub use matrix::{
    M5NavigationContentComponentFamily, M5NavigationContentConsumerSurface,
    M5NavigationContentDisposition,
};

/// Schema version stamped on the M05-1113 consumer packet.
pub const NAV_CONTENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NavContentConsumerPacket`].
pub const NAV_CONTENT_CONSUMER_RECORD_KIND: &str = "m5_navigation_content_component_consumer_packet";

/// Stable record-kind tag carried by each [`NavContentConsumerRow`].
pub const NAV_CONTENT_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_navigation_content_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const NAV_CONTENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-navigation-content-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const NAV_CONTENT_CONSUMER_DOC_REF: &str =
    "docs/navigation/m5_navigation_content_component_consumer_contract.md";

/// Repo-relative path of the frozen navigation-content component matrix release
/// proof these consumers adopt.
pub const NAV_CONTENT_CONSUMER_MATRIX_REF: &str =
    matrix::M5_NAVIGATION_CONTENT_COMPONENT_ARTIFACT_REF;

/// Repo-relative path of the shared frozen component-matrix schema.
pub const NAV_CONTENT_CONSUMER_SHARED_SCHEMA_REF: &str =
    matrix::M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const NAV_CONTENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-navigation-content-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NAV_CONTENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-navigation-content-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NAV_CONTENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-navigation-content-component-consumer-proof/report.md";

/// Repo-relative path of the checked consumer-fixture directory.
pub const NAV_CONTENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-navigation-content-component-consumers";

/// The controlled label families a consumer must preserve identically across
/// every navigation / content surface. These are the track-invariant truth
/// pillars: active context, hierarchy / path, disclosure state,
/// selection-versus-current, pinned / preview / read-only state, exact / loaded /
/// all-matching count scope, the local-action budget, and overflow / freshness
/// semantics. The union of every row's `preserved_label_families` must cover this
/// set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 8] = [
    "active_context",
    "hierarchy_path",
    "disclosure_state",
    "selection_versus_current",
    "pinned_preview_read_only",
    "count_scope",
    "local_action_budget",
    "overflow_freshness",
];

/// The canonical navigation-content disposition vocabulary every consumer keeps
/// visible even when narrowed or export-only — the frozen
/// `M5NavigationContentDisposition` set (preview / pinned / modified / read-only /
/// blocked / exact-count / loaded-count / all-matching-count / hidden-by-filter /
/// hidden-by-policy / overflowed-local-action / stale-or-partial-hierarchy).
/// Every consumer renders the same navigation / content state with these exact
/// tokens rather than surface-local phrasing (AC1).
pub fn canonical_navigation_disposition_vocab() -> Vec<String> {
    M5NavigationContentDisposition::ALL
        .iter()
        .map(|d| d.as_str().to_owned())
        .collect()
}

/// Whether a token is one of the frozen navigation-content disposition tokens.
pub fn is_canonical_navigation_disposition(token: &str) -> bool {
    M5NavigationContentDisposition::ALL
        .iter()
        .any(|d| d.as_str() == token)
}

/// The canonical per-family matrix schema that defines a family's contract.
pub fn canonical_family_schema_ref_for(
    family: M5NavigationContentComponentFamily,
) -> &'static str {
    family.canonical_component_schema_ref()
}

/// The single primary navigation label family a component family must always
/// preserve — the axis it exists to name. A consumer may narrow authority, but it
/// must never drop this label, so the family's core active-context, hierarchy,
/// disclosure, selection, count, or local-action truth is never silently lost.
pub const fn family_primary_label(family: M5NavigationContentComponentFamily) -> &'static str {
    use M5NavigationContentComponentFamily::*;
    match family {
        TabStrip => "active_context",
        Breadcrumbs => "hierarchy_path",
        TreeView => "disclosure_state",
        ListView => "selection_versus_current",
        TableGrid => "count_scope",
        PanelHeader => "local_action_budget",
    }
}

/// The four B132 controls contracts the six component families group into. A
/// consumer must point at the one canonical controls contract for its family's
/// lane rather than inventing a surface-local one — this is the heart of the
/// "navigation surfaces no longer fork component vocabulary" acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentControlsLane {
    /// Tab-strip + breadcrumbs controls (M05-1109 lane).
    TabStripBreadcrumbs,
    /// Tree-view + list-view controls (M05-1110 lane).
    TreeViewListView,
    /// Table/grid + panel-header controls (M05-1111 lane).
    TableGridPanelHeader,
    /// Dedicated panel-header + local-action-cluster controls (M05-1112 lane).
    PanelHeaderLocalActionCluster,
}

impl M5NavContentControlsLane {
    /// Every controls lane, in declaration order.
    pub const ALL: [M5NavContentControlsLane; 4] = [
        M5NavContentControlsLane::TabStripBreadcrumbs,
        M5NavContentControlsLane::TreeViewListView,
        M5NavContentControlsLane::TableGridPanelHeader,
        M5NavContentControlsLane::PanelHeaderLocalActionCluster,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabStripBreadcrumbs => "tab_strip_breadcrumbs",
            Self::TreeViewListView => "tree_view_list_view",
            Self::TableGridPanelHeader => "table_grid_panel_header",
            Self::PanelHeaderLocalActionCluster => "panel_header_local_action_cluster",
        }
    }

    /// The canonical controls schema every surface reuses for this lane.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::TabStripBreadcrumbs => {
                tab_strip_controls::M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_REF
            }
            Self::TreeViewListView => {
                tree_view_controls::M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_REF
            }
            Self::TableGridPanelHeader => {
                table_grid_controls::M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_REF
            }
            Self::PanelHeaderLocalActionCluster => {
                panel_header_controls::M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_REF
            }
        }
    }

    /// The canonical controls contract doc for this lane.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::TabStripBreadcrumbs => {
                tab_strip_controls::M5_TAB_STRIP_BREADCRUMBS_CONTROLS_DOC_REF
            }
            Self::TreeViewListView => tree_view_controls::M5_TREE_VIEW_LIST_VIEW_CONTROLS_DOC_REF,
            Self::TableGridPanelHeader => {
                table_grid_controls::M5_TABLE_GRID_PANEL_HEADER_CONTROLS_DOC_REF
            }
            Self::PanelHeaderLocalActionCluster => {
                panel_header_controls::M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_DOC_REF
            }
        }
    }

    /// The canonical controls release-proof artifact every consumer points back
    /// to as the first-resolved truth for this lane.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::TabStripBreadcrumbs => {
                tab_strip_controls::M5_TAB_STRIP_BREADCRUMBS_CONTROLS_ARTIFACT_REF
            }
            Self::TreeViewListView => {
                tree_view_controls::M5_TREE_VIEW_LIST_VIEW_CONTROLS_ARTIFACT_REF
            }
            Self::TableGridPanelHeader => {
                table_grid_controls::M5_TABLE_GRID_PANEL_HEADER_CONTROLS_ARTIFACT_REF
            }
            Self::PanelHeaderLocalActionCluster => {
                panel_header_controls::M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_ARTIFACT_REF
            }
        }
    }
}

/// The one controls lane a component family belongs to. The six frozen families
/// group into the four B132 controls contracts; a consumer must reuse the lane's
/// canonical contract rather than forking it per surface. Panel headers adopt the
/// deepest dedicated panel-header + local-action-cluster contract (M05-1112).
pub const fn controls_lane_for(
    family: M5NavigationContentComponentFamily,
) -> M5NavContentControlsLane {
    use M5NavigationContentComponentFamily::*;
    match family {
        TabStrip | Breadcrumbs => M5NavContentControlsLane::TabStripBreadcrumbs,
        TreeView | ListView => M5NavContentControlsLane::TreeViewListView,
        TableGrid => M5NavContentControlsLane::TableGridPanelHeader,
        PanelHeader => M5NavContentControlsLane::PanelHeaderLocalActionCluster,
    }
}

/// The six claimed M5 navigation / content consumer classes that must each adopt
/// at least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// A shell / explorer surface.
    ShellExplorer,
    /// A search / graph surface.
    SearchGraph,
    /// A review surface.
    Review,
    /// A request / data surface.
    RequestData,
    /// A help center surface (AC2 docs/help reference).
    HelpCenter,
    /// A support / export + release-packet lane (AC2).
    SupportExportHelp,
}

impl ConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [ConsumerClass; 6] = [
        ConsumerClass::ShellExplorer,
        ConsumerClass::SearchGraph,
        ConsumerClass::Review,
        ConsumerClass::RequestData,
        ConsumerClass::HelpCenter,
        ConsumerClass::SupportExportHelp,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellExplorer => "shell_explorer",
            Self::SearchGraph => "search_graph",
            Self::Review => "review",
            Self::RequestData => "request_data",
            Self::HelpCenter => "help_center",
            Self::SupportExportHelp => "support_export_help",
        }
    }
}

/// The consumer class a concrete matrix consumer surface belongs to. Reuses the
/// matrix's own [`M5NavigationContentConsumerSurface`] taxonomy rather than
/// inventing a parallel one.
pub const fn consumer_class_for(surface: M5NavigationContentConsumerSurface) -> ConsumerClass {
    use M5NavigationContentConsumerSurface::*;
    match surface {
        ShellUi | ExplorerUi | ProductUi => ConsumerClass::ShellExplorer,
        SearchUi | AiContextUi => ConsumerClass::SearchGraph,
        ReviewUi => ConsumerClass::Review,
        DataUi => ConsumerClass::RequestData,
        HelpUi => ConsumerClass::HelpCenter,
        SupportExport => ConsumerClass::SupportExportHelp,
    }
}

/// True when this surface is the help / docs reference surface (AC2).
pub const fn is_help_surface(surface: M5NavigationContentConsumerSurface) -> bool {
    matches!(surface, M5NavigationContentConsumerSurface::HelpUi)
}

/// True when this surface is the support / export + release-packet surface (AC2).
pub const fn is_support_export_surface(surface: M5NavigationContentConsumerSurface) -> bool {
    matches!(surface, M5NavigationContentConsumerSurface::SupportExport)
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, override-gated,
/// export-only, policy-blocked) but never rename or drop the governed navigation
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (act on the component directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Override-gated: the action is visible but staged behind an explicit gate
    /// before it applies.
    OverrideGated,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated by policy.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::OverrideGated,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::OverrideGated => "override_gated",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot act on the
/// component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and acts on the component in-place.
    None,
    /// Punt to the desktop shell to act on the navigation state.
    DesktopShell,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a portable support / export packet.
    SupportPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore must
    /// carry a desktop / companion / support note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DesktopShell => "desktop_shell",
            Self::CompanionApp => "companion_app",
            Self::SupportPacket => "support_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full label parity across the navigation truth pillars.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// active-context / count / hierarchy identity support and automation need to
/// reconstruct the navigation state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical navigation-content component family on one
/// M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentConsumerRow {
    /// Record kind; must equal [`NAV_CONTENT_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NAV_CONTENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: ConsumerClass,
    /// The concrete surface; must belong to `consumer_class`.
    pub consumer_surface: M5NavigationContentConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5NavigationContentComponentFamily,
    /// The controls lane the family belongs to; must equal
    /// `controls_lane_for(component_family)`.
    pub controls_lane: M5NavContentControlsLane,
    /// The canonical per-family matrix schema. Must equal
    /// `canonical_family_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical controls schema for the lane. Must equal
    /// `controls_lane.canonical_schema_ref()`.
    pub canonical_controls_schema_ref: String,
    /// The canonical controls release-proof artifact(s) this consumer points back
    /// to. Must contain `controls_lane.canonical_artifact_ref()`.
    #[serde(default)]
    pub canonical_controls_artifact_refs: Vec<String>,
    /// True when the consumer references the canonical family + controls lane
    /// rather than cloning surface-local navigation chrome.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the navigation / content state the user
    /// saw, so support and automation can reconstruct it without leaking raw user
    /// content or tokens.
    pub nav_state_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The frozen navigation-content disposition vocabulary the consumer keeps
    /// visible even when narrowed.
    #[serde(default)]
    pub navigation_disposition_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / companion / support note ref; required when `handoff_target`
    /// is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Guardrail: the consumer lets tabs masquerade as top-level workflow
    /// navigation. Must be false.
    pub tabs_masquerade_as_top_level_workflow_navigation: bool,
    /// Guardrail: the consumer hides counts or blocked rows behind ambiguous
    /// ellipses. Must be false.
    pub hides_counts_or_blocked_rows_behind_ambiguous_ellipsis: bool,
    /// Guardrail: the consumer makes tree / list / table actions hover-only. Must
    /// be false.
    pub makes_tree_list_or_table_actions_hover_only: bool,
    /// Guardrail: the consumer lets a panel header become a cluttered secondary
    /// toolbar. Must be false.
    pub panel_header_becomes_cluttered_secondary_toolbar: bool,
    /// Guardrail: the consumer collapses exact, loaded, and all-matching count
    /// scopes into one vague total. Must be false.
    pub collapses_exact_loaded_and_all_matching_scopes_into_one_total: bool,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NavContentConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        consumer_class_for(self.consumer_surface) == self.consumer_class
    }

    /// AC (no fork): the consumer reuses the canonical controls contract for its
    /// family's lane rather than a surface-local one.
    pub fn controls_lane_is_canonical(&self) -> bool {
        self.controls_lane == controls_lane_for(self.component_family)
            && self.canonical_controls_schema_ref == self.controls_lane.canonical_schema_ref()
            && self
                .canonical_controls_artifact_refs
                .iter()
                .any(|r| r == self.controls_lane.canonical_artifact_ref())
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared matrix schema matches the family, a controls release-proof
    /// artifact is referenced, and no surface-local navigation chrome is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_family_schema_ref_for(self.component_family)
            && self.controls_lane_is_canonical()
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label families
    /// and frozen navigation-content disposition vocabulary rather than renaming
    /// or omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.navigation_disposition_vocab.is_empty()
            && self
                .navigation_disposition_vocab
                .iter()
                .all(|v| is_canonical_navigation_disposition(v))
    }

    /// AC (navigation truth): every row preserves the adopted family's primary
    /// navigation label — so a tab strip never hides the active context, a
    /// breadcrumb trail never hides the hierarchy, and so on.
    pub fn preserves_primary_navigation_truth(&self) -> bool {
        let primary = family_primary_label(self.component_family);
        self.preserved_label_families.iter().any(|f| f == primary)
    }

    /// AC (count truth): a dense-collection family (tree view, list view, or
    /// table / grid) always names the count scope, so exact / loaded /
    /// all-matching totals are never collapsed into one vague number.
    pub fn preserves_count_scope_when_dense(&self) -> bool {
        if self.component_family.declares_count_scope() {
            self.preserved_label_families
                .iter()
                .any(|f| f == "count_scope")
        } else {
            true
        }
    }

    /// AC2: the row carries the opaque navigation-state ref and canonical controls
    /// contract support and automation reconstruct the seen state from.
    pub fn supports_state_reconstruction(&self) -> bool {
        !self.nav_state_ref.trim().is_empty()
            && self.controls_lane_is_canonical()
            && self.copy_export.is_complete()
    }

    /// The five spec guardrails are all clear (false).
    pub fn guardrails_clear(&self) -> bool {
        self.first_failed_guardrail().is_none()
    }

    /// The first guardrail that is (wrongly) set, if any.
    pub fn first_failed_guardrail(&self) -> Option<&'static str> {
        if self.tabs_masquerade_as_top_level_workflow_navigation {
            Some("tabs_masquerade_as_top_level_workflow_navigation")
        } else if self.hides_counts_or_blocked_rows_behind_ambiguous_ellipsis {
            Some("hides_counts_or_blocked_rows_behind_ambiguous_ellipsis")
        } else if self.makes_tree_list_or_table_actions_hover_only {
            Some("makes_tree_list_or_table_actions_hover_only")
        } else if self.panel_header_becomes_cluttered_secondary_toolbar {
            Some("panel_header_becomes_cluttered_secondary_toolbar")
        } else if self.collapses_exact_loaded_and_all_matching_scopes_into_one_total {
            Some("collapses_exact_loaded_and_all_matching_scopes_into_one_total")
        } else {
            None
        }
    }

    /// AC (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == NAV_CONTENT_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == NAV_CONTENT_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.nav_state_ref.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_controls_schema_ref.trim().is_empty()
            && !self.canonical_controls_artifact_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} class={class} family={family} lane={lane} \
authority={authority} label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            lane = self.controls_lane.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1113 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub controls_lane_count: usize,
    pub navigation_disposition_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_controls_lane: bool,
    pub all_rows_preserve_primary_navigation_truth: bool,
    pub all_dense_rows_preserve_count_scope: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_rows_guardrails_clear: bool,
    pub controls_lanes_stable_across_surfaces: bool,
    pub shell_explorer_consumer_present: bool,
    pub search_graph_consumer_present: bool,
    pub review_consumer_present: bool,
    pub request_data_consumer_present: bool,
    pub help_center_consumer_present: bool,
    pub support_export_help_consumer_present: bool,
    pub help_reference_present: bool,
    pub support_export_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub navigation_disposition_coverage_complete: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`NavContentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavContentConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<NavContentConsumerRow>,
}

/// Checked-in M05-1113 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<NavContentConsumerRow>,
    pub summary: NavContentConsumerSummary,
}

impl NavContentConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: NavContentConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NAV_CONTENT_CONSUMER_SCHEMA_VERSION,
            record_kind: NAV_CONTENT_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: NavContentConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                controls_lane_count: 0,
                navigation_disposition_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_controls_lane: false,
                all_rows_preserve_primary_navigation_truth: false,
                all_dense_rows_preserve_count_scope: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_rows_guardrails_clear: false,
                controls_lanes_stable_across_surfaces: false,
                shell_explorer_consumer_present: false,
                search_graph_consumer_present: false,
                review_consumer_present: false,
                request_data_consumer_present: false,
                help_center_consumer_present: false,
                support_export_help_consumer_present: false,
                help_reference_present: false,
                support_export_reference_present: false,
                label_family_coverage_complete: false,
                navigation_disposition_coverage_complete: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5NavigationContentComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The union of every row's navigation-content disposition vocabulary.
    pub fn covered_navigation_dispositions(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.navigation_disposition_vocab.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5NavigationContentComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<ConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether every family maps to exactly one controls lane across every surface
    /// — no surface forks the lane by consumer.
    pub fn controls_lanes_stable_across_surfaces(&self) -> bool {
        let mut per_family: BTreeMap<
            M5NavigationContentComponentFamily,
            BTreeSet<M5NavContentControlsLane>,
        > = BTreeMap::new();
        for row in &self.rows {
            per_family
                .entry(row.component_family)
                .or_default()
                .insert(row.controls_lane);
        }
        per_family.values().all(|lanes| lanes.len() <= 1)
    }

    /// Whether some help / docs surface references the canonical families (AC2).
    pub fn has_help_reference(&self) -> bool {
        self.rows
            .iter()
            .any(|r| is_help_surface(r.consumer_surface) && r.references_canonical_not_local_prose)
    }

    /// Whether some support / export surface references the canonical families —
    /// the release-packet half of AC2.
    pub fn has_support_export_reference(&self) -> bool {
        self.rows.iter().any(|r| {
            is_support_export_surface(r.consumer_surface) && r.references_canonical_not_local_prose
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> NavContentConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        let mut lanes = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
            lanes.insert(row.controls_lane);
        }

        let has_class = |c: ConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let covered_dispositions = self.covered_navigation_dispositions();

        NavContentConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            controls_lane_count: lanes.len(),
            navigation_disposition_count: covered_dispositions.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(NavContentConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(NavContentConsumerRow::preserves_labels),
            all_rows_use_canonical_controls_lane: self
                .rows
                .iter()
                .all(NavContentConsumerRow::controls_lane_is_canonical),
            all_rows_preserve_primary_navigation_truth: self
                .rows
                .iter()
                .all(NavContentConsumerRow::preserves_primary_navigation_truth),
            all_dense_rows_preserve_count_scope: self
                .rows
                .iter()
                .all(NavContentConsumerRow::preserves_count_scope_when_dense),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(NavContentConsumerRow::supports_state_reconstruction),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(NavContentConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_rows_guardrails_clear: self
                .rows
                .iter()
                .all(NavContentConsumerRow::guardrails_clear),
            controls_lanes_stable_across_surfaces: self.controls_lanes_stable_across_surfaces(),
            shell_explorer_consumer_present: has_class(ConsumerClass::ShellExplorer),
            search_graph_consumer_present: has_class(ConsumerClass::SearchGraph),
            review_consumer_present: has_class(ConsumerClass::Review),
            request_data_consumer_present: has_class(ConsumerClass::RequestData),
            help_center_consumer_present: has_class(ConsumerClass::HelpCenter),
            support_export_help_consumer_present: has_class(ConsumerClass::SupportExportHelp),
            help_reference_present: self.has_help_reference(),
            support_export_reference_present: self.has_support_export_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            navigation_disposition_coverage_complete: M5NavigationContentDisposition::ALL
                .iter()
                .all(|d| covered_dispositions.contains(d.as_str())),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NavContentConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NAV_CONTENT_CONSUMER_SCHEMA_VERSION {
            violations.push(NavContentConsumerViolation::SchemaVersion {
                expected: NAV_CONTENT_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NAV_CONTENT_CONSUMER_RECORD_KIND {
            violations.push(NavContentConsumerViolation::RecordKind {
                expected: NAV_CONTENT_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NavContentConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NavContentConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(NavContentConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer class.
            if !row.surface_class_consistent() {
                violations.push(NavContentConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local chrome.
            if !row.points_to_canonical_family() {
                violations.push(NavContentConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical controls lane per family.
            if !row.controls_lane_is_canonical() {
                violations.push(NavContentConsumerViolation::NonCanonicalControlsLane {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled label families / disposition vocab preserved.
            if !row.preserves_labels() {
                violations.push(NavContentConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (navigation truth): the family's primary label is kept.
            if !row.preserves_primary_navigation_truth() {
                violations.push(NavContentConsumerViolation::PrimaryNavigationTruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC (count truth): a dense family always names the count scope.
            if !row.preserves_count_scope_when_dense() {
                violations.push(NavContentConsumerViolation::CountScopeDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC2: navigation state is reconstructable from the opaque ref +
            // canonical controls contract.
            if !row.supports_state_reconstruction() {
                violations.push(NavContentConsumerViolation::StateNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // Disclosure: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(NavContentConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(NavContentConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Spec guardrails must all stay false.
            if let Some(guardrail) = row.first_failed_guardrail() {
                violations.push(NavContentConsumerViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                    guardrail,
                });
            }
        }

        // Cross-surface reuse spans all six claimed consumer classes.
        for class in ConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(NavContentConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5NavigationContentComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(NavContentConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_classes() == 0 {
            violations.push(NavContentConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC (no fork): families resolve to one stable controls lane per family.
        if !self.controls_lanes_stable_across_surfaces() {
            violations.push(NavContentConsumerViolation::ControlsLaneForkedAcrossSurfaces);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(NavContentConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC1: the frozen disposition vocabulary is collectively preserved.
        let covered_dispositions = self.covered_navigation_dispositions();
        for disposition in M5NavigationContentDisposition::ALL {
            if !covered_dispositions.contains(disposition.as_str()) {
                violations.push(NavContentConsumerViolation::MissingNavigationDisposition {
                    disposition: disposition.as_str().to_owned(),
                });
            }
        }

        // AC2: a help / docs consumer references the canonical components rather
        // than cloning local navigation chrome.
        if !self.has_help_reference() {
            violations.push(NavContentConsumerViolation::MissingHelpReference);
        }

        // AC2: a support / export + release-packet consumer references the
        // canonical components so release packets drop bespoke per-surface prose.
        if !self.has_support_export_reference() {
            violations.push(NavContentConsumerViolation::MissingSupportExportReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(NavContentConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(NavContentConsumerViolation::RawNavigationMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,controls_lane,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{class},{surface},{family},{lane},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                lane = row.controls_lane.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Navigation-Content Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5NavigationContentComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Controls lanes adopted: {} / {}\n",
            self.summary.controls_lane_count,
            M5NavContentControlsLane::ALL.len(),
        ));
        out.push_str(&format!(
            "- Navigation dispositions preserved: {} / {}\n",
            self.summary.navigation_disposition_count,
            M5NavigationContentDisposition::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_navigation_content_component_consumers_export(
) -> Result<NavContentConsumerPacket, NavContentConsumerArtifactError> {
    let packet: NavContentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-navigation-content-component-consumer-proof/support_export.json"
    )))
    .map_err(NavContentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NavContentConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum NavContentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NavContentConsumerViolation>),
}

impl fmt::Display for NavContentConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for NavContentConsumerArtifactError {}

/// Validation failure for M05-1113 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavContentConsumerViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    SurfaceClassMismatch { id: String },
    NotCanonicalFamily { id: String },
    NonCanonicalControlsLane { id: String },
    LabelParityBroken { id: String },
    PrimaryNavigationTruthDropped { id: String },
    CountScopeDropped { id: String },
    StateNotReconstructable { id: String },
    NarrowedWithoutDisclosure { id: String },
    MissingCopyExportParity { id: String },
    GuardrailViolated { id: String, guardrail: &'static str },
    MissingConsumerClass { class: ConsumerClass },
    MissingFamilyCoverage { family: M5NavigationContentComponentFamily },
    NoFamilyReusedAcrossClasses,
    ControlsLaneForkedAcrossSurfaces,
    MissingLabelFamily { family: String },
    MissingNavigationDisposition { disposition: String },
    MissingHelpReference,
    MissingSupportExportReference,
    SummaryMismatch,
    RawNavigationMaterialInExport,
}

impl fmt::Display for NavContentConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer class"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::NonCanonicalControlsLane { id } => {
                write!(
                    f,
                    "row {id} forks the controls lane instead of reusing the canonical contract"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical active-context, hierarchy, disclosure, \
selection, pinned/preview/read-only, count-scope, local-action-budget, or overflow/freshness label"
                )
            }
            Self::PrimaryNavigationTruthDropped { id } => {
                write!(
                    f,
                    "row {id} drops the adopted family's primary navigation label (active context, \
hierarchy, disclosure, selection, count scope, or local-action budget)"
                )
            }
            Self::CountScopeDropped { id } => {
                write!(
                    f,
                    "row {id} is a dense tree/list/table family but does not name the count scope"
                )
            }
            Self::StateNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its navigation-state ref and controls contract"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::GuardrailViolated { id, guardrail } => {
                write!(f, "row {id} violates guardrail {guardrail}")
            }
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossClasses => write!(
                f,
                "no component family is adopted across two or more consumer classes"
            ),
            Self::ControlsLaneForkedAcrossSurfaces => write!(
                f,
                "a component family resolves to more than one controls lane across surfaces"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingNavigationDisposition { disposition } => {
                write!(
                    f,
                    "navigation-content disposition token {disposition} is not preserved anywhere"
                )
            }
            Self::MissingHelpReference => write!(
                f,
                "no help / docs consumer references the canonical component families"
            ),
            Self::MissingSupportExportReference => write!(
                f,
                "no support / export consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawNavigationMaterialInExport => {
                write!(f, "export contains raw navigation material")
            }
        }
    }
}

impl Error for NavContentConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
/// Adds the navigation / content generic phrasings the spec forbids collapsing
/// into (more, items, results, hidden, overflow, ellipsis) to the shared
/// generic-label blocklist. These are matched as *whole* labels rather than
/// substrings so a descriptive banner may still name "12 more matches hidden by
/// filter" as a state without being flagged; only a banner whose entire label
/// collapses to the generic phrase is rejected.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("get started") {
        return true;
    }
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
            | "more"
            | "…"
            | "..."
            | "items"
            | "results"
            | "hidden"
            | "overflow"
            | "ellipsis"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_navigation_content_component_consumers_packet() -> NavContentConsumerPacket {
    NavContentConsumerPacket::new(NavContentConsumerPacketInput {
        packet_id: "m5-navigation-content-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: NAV_CONTENT_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:navigation-content-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5NavigationContentConsumerSurface,
    component_family: M5NavigationContentComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> NavContentConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    let controls_lane = controls_lane_for(component_family);
    NavContentConsumerRow {
        record_kind: NAV_CONTENT_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: NAV_CONTENT_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_class_for(consumer_surface),
        consumer_surface,
        component_family,
        controls_lane,
        canonical_family_schema_ref: canonical_family_schema_ref_for(component_family).to_owned(),
        canonical_controls_schema_ref: controls_lane.canonical_schema_ref().to_owned(),
        canonical_controls_artifact_refs: vec![controls_lane.canonical_artifact_ref().to_owned()],
        references_canonical_not_local_prose: true,
        nav_state_ref: format!("nav-state:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        navigation_disposition_vocab: canonical_navigation_disposition_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        tabs_masquerade_as_top_level_workflow_navigation: false,
        hides_counts_or_blocked_rows_behind_ambiguous_ellipsis: false,
        makes_tree_list_or_table_actions_hover_only: false,
        panel_header_becomes_cluttered_secondary_toolbar: false,
        collapses_exact_loaded_and_all_matching_scopes_into_one_total: false,
        source_refs: vec![
            NAV_CONTENT_CONSUMER_MATRIX_REF.to_owned(),
            NAV_CONTENT_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
            controls_lane.canonical_doc_ref().to_owned(),
        ],
        observed_at: "2026-07-12T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<NavContentConsumerRow> {
    use AuthorityMode::*;
    use HandoffTarget as H;
    use M5NavigationContentComponentFamily::*;
    use M5NavigationContentConsumerSurface::*;

    vec![
        // --- Shell / explorer ----------------------------------------------
        row(
            "consumer:shell-explorer:tab-strip",
            ShellUi,
            TabStrip,
            FullInteractive,
            &["active_context", "pinned_preview_read_only"],
            &["active_context", "pinned_preview_read_only", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:shell-explorer:tree-view",
            ExplorerUi,
            TreeView,
            FullInteractive,
            &["disclosure_state", "selection_versus_current", "count_scope"],
            &[
                "disclosure_state",
                "selection_versus_current",
                "count_scope",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:shell-explorer:breadcrumbs",
            ExplorerUi,
            Breadcrumbs,
            ReadOnly,
            &["hierarchy_path", "active_context"],
            &["hierarchy_path", "active_context", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:shell-explorer:breadcrumbs",
                "Read-only explorer breadcrumb trail: names the full hierarchy / path to the current object, including a stale-or-partial hierarchy; reordering or acting on ancestors stays in the desktop shell",
                ReadOnly,
                &["reveal_in_explorer", "act_on_ancestor"],
            )),
        ),
        row(
            "consumer:shell-explorer:panel-header",
            ShellUi,
            PanelHeader,
            ReadOnly,
            &["local_action_budget", "overflow_freshness", "active_context"],
            &[
                "local_action_budget",
                "overflow_freshness",
                "active_context",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:shell-explorer:panel-header",
                "Read-only shell panel header: names the active context, the bounded local-action budget, and the source freshness cue without becoming a secondary toolbar; refresh and reveal stay command-backed in the desktop shell",
                ReadOnly,
                &["refresh_source", "reveal_detail"],
            )),
        ),
        // --- Search / graph ------------------------------------------------
        row(
            "consumer:search-graph:list-view",
            SearchUi,
            ListView,
            ReadOnly,
            &["selection_versus_current", "count_scope"],
            &[
                "selection_versus_current",
                "count_scope",
                "controls_lane",
            ],
            H::None,
            "",
            Some(banner(
                "banner:search-graph:list-view",
                "Read-only search result list: names the exact, loaded, and all-matching counts distinctly and the selection-versus-current row; opening a result stays in the desktop shell",
                ReadOnly,
                &["open_result", "act_on_result"],
            )),
        ),
        row(
            "consumer:search-graph:table-grid",
            SearchUi,
            TableGrid,
            ReadOnly,
            &["count_scope", "selection_versus_current"],
            &["count_scope", "selection_versus_current", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:search-graph:table-grid",
                "Read-only search results grid: names the pinned-column identity, sort/filter provenance, and exact-versus-loaded-versus-all-matching count scope; acting on a match stays in the desktop shell",
                ReadOnly,
                &["sort_column", "act_on_match"],
            )),
        ),
        row(
            "consumer:search-graph:breadcrumbs",
            AiContextUi,
            Breadcrumbs,
            InspectOnly,
            &["hierarchy_path"],
            &["hierarchy_path", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:search-graph:breadcrumbs",
                "Inspect-only graph breadcrumb trail: names the source-aware hierarchy / path to the current node, including a truncated or partial ancestry; it never masquerades as top-level navigation",
                InspectOnly,
                &["navigate_ancestor", "pin_node"],
            )),
        ),
        row(
            "consumer:search-graph:tab-strip",
            SearchUi,
            TabStrip,
            ReadOnly,
            &["active_context", "pinned_preview_read_only"],
            &["active_context", "pinned_preview_read_only", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:search-graph:tab-strip",
                "Read-only search tab strip: names the active-versus-open context and per-tab preview/pinned/modified state; these tabs never masquerade as top-level workflow navigation",
                ReadOnly,
                &["reorder_tabs", "close_tab"],
            )),
        ),
        // --- Review --------------------------------------------------------
        row(
            "consumer:review:list-view",
            ReviewUi,
            ListView,
            FullInteractive,
            &["selection_versus_current", "count_scope"],
            &["selection_versus_current", "count_scope", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:review:panel-header",
            ReviewUi,
            PanelHeader,
            ReadOnly,
            &["local_action_budget", "overflow_freshness"],
            &["local_action_budget", "overflow_freshness", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:review:panel-header",
                "Read-only review panel header: names the active review context, its bounded local-action budget, and a cached/stale source-freshness cue without overloading into a toolbar",
                ReadOnly,
                &["refresh_review", "reveal_review_detail"],
            )),
        ),
        // --- Request / data ------------------------------------------------
        row(
            "consumer:request-data:table-grid",
            DataUi,
            TableGrid,
            FullInteractive,
            &["count_scope", "selection_versus_current"],
            &["count_scope", "selection_versus_current", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:request-data:tree-view",
            DataUi,
            TreeView,
            ReadOnly,
            &["disclosure_state", "count_scope"],
            &["disclosure_state", "count_scope", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:request-data:tree-view",
                "Read-only request/data tree: names disclosure state, virtualization-honest exact-versus-loaded-versus-hidden counts, and keyboard-discoverable inline actions; editing stays in the desktop shell",
                ReadOnly,
                &["edit_node", "act_on_node"],
            )),
        ),
        // --- Help center (AC2 docs/help reference) -------------------------
        row(
            "consumer:help-center:breadcrumbs",
            HelpUi,
            Breadcrumbs,
            ReadOnly,
            &["hierarchy_path"],
            &["hierarchy_path", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:help-center:breadcrumbs",
                "Read-only help center breadcrumb trail: names the docs hierarchy / path to the current article, including a partial hierarchy, from the one shared breadcrumbs contract rather than bespoke help prose",
                ReadOnly,
                &["open_article", "navigate_section"],
            )),
        ),
        row(
            "consumer:help-center:panel-header",
            HelpUi,
            PanelHeader,
            ReadOnly,
            &["local_action_budget", "overflow_freshness"],
            &["local_action_budget", "overflow_freshness", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:help-center:panel-header",
                "Read-only help center panel header: names the active help context, its bounded local-action budget, and a provider-owned source-freshness cue from the shared panel-header contract",
                ReadOnly,
                &["refresh_help_source", "reveal_help_detail"],
            )),
        ),
        // --- Support / export + release packet (AC2) -----------------------
        row(
            "consumer:support-export:tab-strip",
            SupportExport,
            TabStrip,
            ExportOnly,
            &["active_context", "pinned_preview_read_only"],
            &[
                "active_context",
                "pinned_preview_read_only",
                "nav_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:tab-strip-support-packet",
            Some(banner(
                "banner:support-export:tab-strip",
                "Export-only support replay: reconstruct the active-versus-open context and per-tab preview/pinned/modified/read-only/blocked state the user saw from the support packet",
                ExportOnly,
                &["reorder_tabs", "close_tab"],
            )),
        ),
        row(
            "consumer:support-export:table-grid",
            SupportExport,
            TableGrid,
            ExportOnly,
            &["count_scope", "selection_versus_current"],
            &[
                "count_scope",
                "selection_versus_current",
                "nav_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:table-grid-support-packet",
            Some(banner(
                "banner:support-export:table-grid",
                "Export-only support replay: reconstruct the pinned-column identity, sort/filter provenance, and exact-versus-loaded-versus-all-matching count scope the grid showed from the support packet",
                ExportOnly,
                &["sort_column", "act_on_match"],
            )),
        ),
        row(
            "consumer:support-export:list-view",
            SupportExport,
            ListView,
            ExportOnly,
            &["selection_versus_current", "count_scope"],
            &[
                "selection_versus_current",
                "count_scope",
                "nav_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:list-view-support-packet",
            Some(banner(
                "banner:support-export:list-view",
                "Export-only support replay: reconstruct the selection-versus-current row and the exact/loaded/all-matching/hidden count scope the list showed from the support packet",
                ExportOnly,
                &["open_result", "act_on_result"],
            )),
        ),
    ]
}
