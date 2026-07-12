//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 tab-strip / breadcrumbs / tree-view / list-view / table-grid /
//! panel-header navigation-content components.
//!
//! This module is the M05-1114 accessibility-and-auto-narrowing capstone over the frozen M5
//! navigation-content component matrix
//! ([`crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix`]).
//! Where the freeze matrix defines the reusable tab strip, breadcrumbs, tree view, list view,
//! table / grid, and panel header primitives, and the 1109-1112 implementation lanes resolve their
//! per-surface truth, this lane certifies — per component family — that navigation and content claims
//! stay **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe, CLI/export-safe,
//! and self-narrowing** rather than presenting a stale hierarchy, a missing selection, a collapsed
//! count, a stale sort/filter provenance, or a partial source-freshness cue as still a current,
//! authoritative navigation surface:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same active context, hierarchy / path, disclosure state,
//!   selection-versus-current, item state, count scope, sort/filter provenance, and source-freshness the
//!   rich component shows — never a hover-only badge that strands assistive-tech or headless-CLI users.
//!   Hierarchy-heavy families (the tree view's nested disclosure structure) additionally bind their
//!   nested structure to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning from
//!   typed tokens and opaque refs **without a raw payload**, preserving the same active context,
//!   hierarchy, disclosure, selection, count scope, sort/filter provenance, and source-freshness shown
//!   in-product so support, help, and release proof can reconstruct exactly what the user was actually
//!   shown without leaking a raw tree body, row payload, or query internals.
//! - **Honest auto-narrowing.** When a hierarchy / path signal is stale or partial, a count scope is
//!   unresolved, a sort/filter provenance is stale, or a source-freshness cue is only partial, the
//!   component's claim auto-narrows from `current_navigation_result` / `reviewable_structure_result` to
//!   a hierarchy-unverified / count-unverified / sort-filter-unverified / source-freshness projection,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the canonical
//!   component identity / active context / last-known state. The underlying navigation / content truth is
//!   never dropped opaquely. A component with every dimension intact must NOT carry a spurious narrowing,
//!   and a stale-hierarchy / unresolved-count / stale-provenance state can never keep a current,
//!   authoritative claim — a stale hierarchy never masquerades as the live current path, and a collapsed
//!   count scope never reads as an exact total.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the explorer UI,
//!   the search UI, the review UI, the request / data UI, the help UI, the AI-context UI, the support
//!   export, and the product UI so product, help, and release publication stay aligned on downgrade
//!   behavior rather than drifting in copy — a current-looking surface can never outrun the hierarchy /
//!   count / provenance / freshness evidence it is being viewed away from.
//!
//! Each [`NavContentComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::M5NavigationContentComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5NavigationContentRequiredLabel`],
//! [`M5NavigationContentDowngradeTrigger`], and shared [`M5NavigationContentConsumerSurface`] consumer
//! surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw tree bodies, row payloads, query internals, credentials, secrets,
//! and endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque
//! component refs, booleans, and controlled labels so support, release, and diagnostics exports can
//! reconstruct exactly what an accessible fallback would have shown without leaking sensitive material or
//! a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    M5NavigationContentComponentFamily, M5NavigationContentConsumerSurface,
    M5NavigationContentDowngradeTrigger, M5NavigationContentRequiredLabel,
    M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1114 navigation-content component accessibility parity packet.
pub const NAV_CONTENT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NavContentComponentAccessibilityPacket`].
pub const NAV_CONTENT_A11Y_RECORD_KIND: &str =
    "m5_navigation_content_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`NavContentComponentAccessibilityRow`].
pub const NAV_CONTENT_A11Y_ROW_RECORD_KIND: &str =
    "m5_navigation_content_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const NAV_CONTENT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-navigation-content-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const NAV_CONTENT_A11Y_DOC_REF: &str =
    "docs/navigation/m5_navigation_content_component_accessibility_parity.md";

/// Repo-relative path of the frozen navigation-content component matrix this lane certifies.
pub const NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF: &str = M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const NAV_CONTENT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-navigation-content-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const NAV_CONTENT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-navigation-content-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NAV_CONTENT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-navigation-content-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NAV_CONTENT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-navigation-content-component-accessibility-parity.md";

/// The reusable component families that render a non-linear hierarchy (the tree view's nested
/// disclosure structure) and therefore MUST bind their nested structure to an equivalent flat list /
/// textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5NavigationContentComponentFamily) -> bool {
    matches!(family, M5NavigationContentComponentFamily::TreeView)
}

/// The navigation / content dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5NavigationContentComponentFamily,
) -> M5NavContentComponentClaimDimension {
    match family {
        M5NavigationContentComponentFamily::TabStrip => {
            M5NavContentComponentClaimDimension::ActiveContextClarity
        }
        M5NavigationContentComponentFamily::Breadcrumbs => {
            M5NavContentComponentClaimDimension::HierarchyPathClarity
        }
        M5NavigationContentComponentFamily::TreeView => {
            M5NavContentComponentClaimDimension::DisclosureSelectionClarity
        }
        M5NavigationContentComponentFamily::ListView => {
            M5NavContentComponentClaimDimension::CountScopeClarity
        }
        M5NavigationContentComponentFamily::TableGrid => {
            M5NavContentComponentClaimDimension::SortFilterProvenanceClarity
        }
        M5NavigationContentComponentFamily::PanelHeader => {
            M5NavContentComponentClaimDimension::SourceFreshnessClarity
        }
    }
}

/// A rendered fallback modality for a navigation / content component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentComponentFallbackModality {
    /// A rich, structured (nested hierarchy / disclosure / dense-grid) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / path-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5NavContentComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentComponentRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5NavContentComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline
    /// and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl NavContentComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl NavContentComponentExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl NavContentComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The navigation / content claim ceiling a component asserts: how strong a current / authoritative
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a navigation / content
/// dimension weakens so a stale hierarchy, an unresolved count scope, a stale sort/filter provenance, or
/// a partial source-freshness cue can never keep an old `CurrentNavigationResult` or
/// `ReviewableStructureResult` label — a stale hierarchy never masquerades as the live current path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentComponentClaim {
    /// Current navigation result: a fully current, authoritative, count-honest, provenance-clear
    /// navigation surface — the strongest claim, a surface Aureline can present as exactly current right
    /// now.
    CurrentNavigationResult,
    /// Reviewable structure result: a self-sufficient, reviewable read-only dense-structure view (a
    /// tree / grid a user can review) that is not itself an authoritative live-current navigation
    /// surface.
    ReviewableStructureResult,
    /// Hierarchy-unverified projection: the hierarchy / path signal is stale / partial; the surface
    /// stays a hierarchy-unverified projection with its last-known ancestry preserved, never a live
    /// current path.
    HierarchyUnverifiedProjection,
    /// Count-unverified projection: the exact / loaded / all-matching count scope is unresolved; the
    /// surface stays a count-unverified projection with its last-known loaded scope preserved, never a
    /// single exact total.
    CountUnverifiedProjection,
    /// Sort/filter-unverified projection: the sort / filter provenance is stale; the surface stays a
    /// sort-filter-unverified projection that names the last-known ordering, never a canonically ordered
    /// grid.
    SortFilterUnverifiedProjection,
    /// Source-freshness projection: the source-freshness cue is only partial / cached; the surface
    /// stays a source-freshness projection that discloses the cached / partial cue, never a
    /// freshly-current header.
    SourceFreshnessProjection,
}

impl M5NavContentComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::CurrentNavigationResult,
        Self::ReviewableStructureResult,
        Self::HierarchyUnverifiedProjection,
        Self::CountUnverifiedProjection,
        Self::SortFilterUnverifiedProjection,
        Self::SourceFreshnessProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CurrentNavigationResult => 5,
            Self::ReviewableStructureResult => 4,
            Self::HierarchyUnverifiedProjection => 3,
            Self::CountUnverifiedProjection => 2,
            Self::SortFilterUnverifiedProjection => 1,
            Self::SourceFreshnessProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully current, authoritative navigation surface.
    pub const fn asserts_current_navigation_result(self) -> bool {
        matches!(self, Self::CurrentNavigationResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (current or reviewable) result.
    pub const fn asserts_self_sufficient_result(self) -> bool {
        matches!(
            self,
            Self::CurrentNavigationResult | Self::ReviewableStructureResult
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentNavigationResult => "current_navigation_result",
            Self::ReviewableStructureResult => "reviewable_structure_result",
            Self::HierarchyUnverifiedProjection => "hierarchy_unverified_projection",
            Self::CountUnverifiedProjection => "count_unverified_projection",
            Self::SortFilterUnverifiedProjection => "sort_filter_unverified_projection",
            Self::SourceFreshnessProjection => "source_freshness_projection",
        }
    }
}

/// The navigation / content dimension whose state governs how far a component may claim to be a fully
/// current, authoritative navigation surface. The dimensions map 1:1 to the six frozen component
/// families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentComponentClaimDimension {
    /// Active-context clarity: is the active context / current open context fully stated (tab strip /
    /// panel header)?
    ActiveContextClarity,
    /// Hierarchy-path clarity: is the hierarchy / path to the current object fully stated and current
    /// (breadcrumbs / tree view)?
    HierarchyPathClarity,
    /// Disclosure / selection clarity: are the disclosure state and selection-versus-current fully
    /// stated (tree view)?
    DisclosureSelectionClarity,
    /// Count-scope clarity: are the exact / loaded / all-matching / hidden count scopes fully stated
    /// and distinct (list view)?
    CountScopeClarity,
    /// Sort/filter-provenance clarity: is the sort / filter provenance fully stated (table / grid)?
    SortFilterProvenanceClarity,
    /// Source-freshness clarity: is the source-freshness cue (current / cached / partial / stale) fully
    /// stated at the pane boundary (panel header)?
    SourceFreshnessClarity,
}

impl M5NavContentComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActiveContextClarity,
        Self::HierarchyPathClarity,
        Self::DisclosureSelectionClarity,
        Self::CountScopeClarity,
        Self::SortFilterProvenanceClarity,
        Self::SourceFreshnessClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveContextClarity => "active_context_clarity",
            Self::HierarchyPathClarity => "hierarchy_path_clarity",
            Self::DisclosureSelectionClarity => "disclosure_selection_clarity",
            Self::CountScopeClarity => "count_scope_clarity",
            Self::SortFilterProvenanceClarity => "sort_filter_provenance_clarity",
            Self::SourceFreshnessClarity => "source_freshness_clarity",
        }
    }
}

/// The observed condition of one navigation / content dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the component's claim. The stale / partial /
/// unresolved states the lane must auto-narrow on as *weakened evidence* — a stale hierarchy, an
/// unresolved count scope, and a stale sort/filter provenance — are the states that
/// [`Self::cannot_be_shown_current`] flags. A partial source-freshness cue is an honest
/// disclosed-absence operation (a cached / partial cue shown honestly), not a truth overstatement, so it
/// is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentComponentConditionState {
    /// Fully current, authoritative, count-honest, provenance-clear — imposes no ceiling.
    FullyQualified,
    /// The hierarchy / path signal is stale / partial — claim drops to a hierarchy-unverified
    /// projection.
    HierarchyPathStale,
    /// The exact / loaded / all-matching count scope is unresolved — claim drops to a count-unverified
    /// projection.
    CountScopeUnresolved,
    /// The sort / filter provenance is stale — claim drops to a sort-filter-unverified projection.
    SortFilterProvenanceStale,
    /// The source-freshness cue is only partial / cached — claim drops to a source-freshness
    /// projection.
    SourceFreshnessPartial,
}

impl M5NavContentComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyQualified,
        Self::HierarchyPathStale,
        Self::CountScopeUnresolved,
        Self::SortFilterProvenanceStale,
        Self::SourceFreshnessPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully
    /// current, authoritative navigation surface and must never be shown as such. A partial
    /// source-freshness cue is an honest disclosed-absence operation (a cached / partial cue shown
    /// honestly), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_current(self) -> bool {
        matches!(
            self,
            Self::HierarchyPathStale | Self::CountScopeUnresolved | Self::SortFilterProvenanceStale
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5NavContentComponentClaim {
        match self {
            Self::FullyQualified => M5NavContentComponentClaim::CurrentNavigationResult,
            Self::HierarchyPathStale => M5NavContentComponentClaim::HierarchyUnverifiedProjection,
            Self::CountScopeUnresolved => M5NavContentComponentClaim::CountUnverifiedProjection,
            Self::SortFilterProvenanceStale => {
                M5NavContentComponentClaim::SortFilterUnverifiedProjection
            }
            Self::SourceFreshnessPartial => M5NavContentComponentClaim::SourceFreshnessProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5NavigationContentDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5NavigationContentDowngradeTrigger::ProofStale,
            Self::HierarchyPathStale => M5NavigationContentDowngradeTrigger::HierarchyPathUnstated,
            Self::CountScopeUnresolved => M5NavigationContentDowngradeTrigger::CountScopeCollapsed,
            Self::SortFilterProvenanceStale => M5NavigationContentDowngradeTrigger::ProofStale,
            Self::SourceFreshnessPartial => M5NavigationContentDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::HierarchyPathStale => "hierarchy_path_stale",
            Self::CountScopeUnresolved => "count_scope_unresolved",
            Self::SortFilterProvenanceStale => "sort_filter_provenance_stale",
            Self::SourceFreshnessPartial => "source_freshness_partial",
        }
    }
}

/// One navigation / content dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5NavContentComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5NavContentComponentConditionState,
}

/// An honest claim auto-narrow block. When a navigation / content dimension weakens, the component's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves
/// the canonical component identity / active context / last-known state rather than silently dropping it
/// — the underlying navigation / content truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentClaimAutoNarrow {
    /// The claim the component is narrowed to.
    pub narrowed_to: M5NavContentComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5NavContentComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5NavigationContentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity, active context, and last-known state are preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying navigation / content truth is preserved (never dropped) across the narrowing;
    /// must hold so hierarchy-unverified, count-unverified, sort-filter-unverified, and source-freshness
    /// states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl NavContentComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and navigation /
    /// content truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl NavContentComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least
    /// one export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5NavContentComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NavContentComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a navigation / content-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims current, or drops state silently (red).
    Stranded,
}

impl NavContentComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one navigation / content-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentAccessibilityRow {
    /// Record kind; must equal [`NAV_CONTENT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NAV_CONTENT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5NavigationContentComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the navigation / content component this row represents; stays visible on every
    /// surface, so this is never empty.
    pub component_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5NavContentComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical active context, hierarchy, disclosure,
    /// selection, item state, count scope, sort/filter provenance, and source-freshness as the rich
    /// surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: NavContentComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: NavContentComponentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: NavContentComponentNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: NavContentComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: NavContentComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: NavContentComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: NavContentComponentCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5NavContentComponentClaim,
    /// The observed condition of each modeled navigation / content dimension.
    #[serde(default)]
    pub claim_conditions: Vec<NavContentComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<NavContentComponentClaimAutoNarrow>,
    /// Whether the underlying navigation / content truth is preserved on this component regardless of
    /// narrowing; must hold so hierarchy-unverified, count-unverified, sort-filter-unverified, and
    /// source-freshness states never fail opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5NavContentComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<NavContentComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5NavigationContentRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5NavigationContentConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NavContentComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat non-visual
    /// path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5NavContentComponentClaimDimension,
    ) -> M5NavContentComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5NavContentComponentConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5NavContentComponentClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_condition(&self) -> Option<&NavContentComponentClaimConditionEntry> {
        let mut binding: Option<(&NavContentComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5NavContentComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5NavContentComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale hierarchy, an unresolved count scope, a stale sort/filter
    /// provenance, or a partial source-freshness cue can no longer keep an old
    /// `CurrentNavigationResult` / `ReviewableStructureResult` label. The effective claim never exceeds
    /// the permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with
    /// its frozen trigger, and preserves canonical identity and truth. When nothing narrows, no spurious
    /// narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / current honesty: a stale-hierarchy / unresolved-count / stale-provenance state never keeps
    /// a current claim — a stale hierarchy never masquerades as the live current path. When such a
    /// state is modeled, the effective claim must not assert `CurrentNavigationResult`.
    pub fn current_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_current());
        !(has_unprovable_state && self.effective_claim().asserts_current_navigation_result())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.component_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: hierarchy-unverified, count-unverified, sort-filter-unverified, and source-freshness
    /// states preserve the underlying navigation / content truth. The row must assert `truth_preserved`,
    /// and any narrow block must preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an honest
    /// claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / help / release publication stay aligned on the
    /// same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5NavigationContentRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> NavContentComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.current_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return NavContentComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            NavContentComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            NavContentComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == NAV_CONTENT_A11Y_ROW_RECORD_KIND
            && self.schema_version == NAV_CONTENT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.component_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1114 navigation / content-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_current_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`NavContentComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavContentComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<NavContentComponentAccessibilityRow>,
}

/// Checked-in M05-1114 navigation / content-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<NavContentComponentAccessibilityRow>,
    pub summary: NavContentComponentAccessibilitySummary,
}

impl NavContentComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: NavContentComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            record_kind: NAV_CONTENT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: NavContentComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_current_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5NavigationContentComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5NavContentComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5NavContentComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5NavContentComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5NavigationContentConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> NavContentComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5NavigationContentConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&NavContentComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                NavContentComponentAccessibilityStatus::Parity => green += 1,
                NavContentComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                NavContentComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        NavContentComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::claim_is_honest),
            all_current_honesty_holds: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::current_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(NavContentComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NavContentComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NAV_CONTENT_A11Y_SCHEMA_VERSION {
            violations.push(NavContentComponentAccessibilityViolation::SchemaVersion {
                expected: NAV_CONTENT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NAV_CONTENT_A11Y_RECORD_KIND {
            violations.push(NavContentComponentAccessibilityViolation::RecordKind {
                expected: NAV_CONTENT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NavContentComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NavContentComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_current())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(NavContentComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory navigation / content label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured projection *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5NavContentComponentFallbackModality::Structured)
            {
                violations.push(
                    NavContentComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a current / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    NavContentComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / current honesty: a stale-hierarchy / unresolved-count / stale-provenance state never
            // keeps a current claim.
            if !row.current_honesty_holds() {
                violations.push(
                    NavContentComponentAccessibilityViolation::WeakStateShownAsCurrent {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    NavContentComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    NavContentComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve navigation / content truth.
            if !row.preserves_truth_continuity() {
                violations.push(NavContentComponentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    NavContentComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == NavContentComponentAccessibilityStatus::Stranded {
                violations.push(NavContentComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5NavigationContentComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5NavContentComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5NavContentComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (current → … → source-freshness) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5NavContentComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Current honesty must be proven with at least one stale-hierarchy / unresolved-count /
        // stale-provenance row in the packet, so the "cannot-prove never shown as current" guarantee is
        // exercised end-to-end.
        if !has_unprovable_row {
            violations.push(NavContentComponentAccessibilityViolation::CurrentHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, explorer, search, review, data,
        // help, AI-context, support-export, and product surfaces — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5NavigationContentConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    NavContentComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(NavContentComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("navigation / content-component accessibility parity packet serializes"),
        ) {
            violations
                .push(NavContentComponentAccessibilityViolation::RawNavContentMaterialInExport);
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
            .expect("navigation / content-component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Navigation / Content-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5NavigationContentComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in navigation / content-component accessibility parity export.
pub fn current_m5_navigation_content_component_a11y_export(
) -> Result<NavContentComponentAccessibilityPacket, NavContentComponentAccessibilityArtifactError> {
    let packet: NavContentComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-navigation-content-component-accessibility-parity/support_export.json"
    )))
    .map_err(NavContentComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NavContentComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in navigation / content-component accessibility parity
/// export.
#[derive(Debug)]
pub enum NavContentComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NavContentComponentAccessibilityViolation>),
}

impl fmt::Display for NavContentComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "navigation / content-component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "navigation / content-component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for NavContentComponentAccessibilityArtifactError {}

/// Validation failure for M05-1114 navigation / content-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavContentComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5NavContentComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsCurrent {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5NavigationContentComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5NavContentComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5NavContentComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5NavContentComponentClaim,
    },
    CurrentHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5NavigationContentConsumerSurface,
    },
    SummaryMismatch,
    RawNavContentMaterialInExport,
}

impl NavContentComponentAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsCurrent { .. } => "weak_state_shown_as_current",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::CurrentHonestyUnproven => "current_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawNavContentMaterialInExport => "raw_nav_content_material_in_export",
        }
    }
}

impl fmt::Display for NavContentComponentAccessibilityViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory navigation / content label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a current / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsCurrent { id } => {
                write!(
                    f,
                    "row {id} shows a stale-hierarchy / unresolved-count / stale-provenance state as a current navigation result"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve navigation / content truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::CurrentHonestyUnproven => {
                write!(
                    f,
                    "no stale-hierarchy / unresolved-count / stale-provenance row is present to prove the current-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawNavContentMaterialInExport => {
                write!(f, "export contains raw navigation / content material")
            }
        }
    }
}

impl Error for NavContentComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
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
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "no counts"
            | "current"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const NAV_CONTENT_A11Y_PACKET_ID: &str =
    "m5-navigation-content-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in navigation / content-component accessibility parity packet. This is
/// the one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_navigation_content_component_a11y_packet() -> NavContentComponentAccessibilityPacket
{
    NavContentComponentAccessibilityPacket::new(NavContentComponentAccessibilityPacketInput {
        packet_id: NAV_CONTENT_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:navigation-content-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5NavigationContentRequiredLabel> {
    M5NavigationContentRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> NavContentComponentCopyExportParity {
    NavContentComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5NavContentComponentClaimDimension,
    state: M5NavContentComponentConditionState,
) -> NavContentComponentClaimConditionEntry {
    NavContentComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5NavigationContentConsumerSurface],
) -> Vec<M5NavigationContentConsumerSurface> {
    let mut out = vec![
        M5NavigationContentConsumerSurface::SupportExport,
        M5NavigationContentConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full
/// label and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions
/// it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: NavContentComponentNarrowingDisclosureState,
) -> Vec<NavContentComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        NavContentComponentRenderingNarrowingDisclosure {
            rendering_surface: M5NavContentComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        NavContentComponentRenderingNarrowingDisclosure {
            rendering_surface: M5NavContentComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<NavContentComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        NavContentComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions
/// while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<NavContentComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        NavContentComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5NavContentComponentRenderingSurface> {
    vec![
        M5NavContentComponentRenderingSurface::DesktopFull,
        M5NavContentComponentRenderingSurface::CliHeadless,
        M5NavContentComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5NavContentComponentFallbackModality> {
    vec![
        M5NavContentComponentFallbackModality::List,
        M5NavContentComponentFallbackModality::Textual,
        M5NavContentComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<NavContentComponentAccessibilityRow> {
    vec![
        // Tab strip (active context fully stated) — the active context / current open context is fully
        // stated and the tab strip never masquerades as top-level workflow navigation, so it is a
        // current navigation result reachable on every surface with no narrowing (green).
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:tab-strip-active-context-stated".to_owned(),
            component_family: M5NavigationContentComponentFamily::TabStrip,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:tab-strip:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:tab-strip-active-context-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "component_identity",
                "active_context",
                "per_tab_item_state",
                "overflow_disposition",
            ]),
            full_ready_claim: M5NavContentComponentClaim::CurrentNavigationResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::ActiveContextClarity,
                M5NavContentComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "component_identity",
                "active_context",
                "per_tab_item_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::ShellUi,
                M5NavigationContentConsumerSurface::ExplorerUi,
            ]),
            source_refs: vec![
                "UX Design System core navigation — Tabs".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("tab-strip-active-context-stated"),
        },
        // Tree view (disclosure / selection fully stated) — hierarchy-heavy (nested disclosure
        // structure); the disclosure state and selection-versus-current are fully stated, so it is a
        // reviewable structure result that binds its nested tree to a flat list / textual path, but its
        // dense nesting narrows the screen-reader traversal to a disclosed linear walk (yellow).
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:tree-view-disclosure-selection-stated".to_owned(),
            component_family: M5NavigationContentComponentFamily::TreeView,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:tree-view:0002".to_owned(),
            fallback_modalities: vec![
                M5NavContentComponentFallbackModality::Structured,
                M5NavContentComponentFallbackModality::List,
                M5NavContentComponentFallbackModality::Textual,
                M5NavContentComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::DisclosedReducedButReachable,
            high_zoom_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:tree-view-disclosure-selection-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "tree_identity",
                "disclosure_state",
                "selection_versus_current",
                "count_and_scope",
            ]),
            full_ready_claim: M5NavContentComponentClaim::ReviewableStructureResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::DisclosureSelectionClarity,
                M5NavContentComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "tree_identity",
                "disclosure_state",
                "selection_versus_current",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::ExplorerUi,
                M5NavigationContentConsumerSurface::AiContextUi,
            ]),
            source_refs: vec![
                "UX Design System core content — Tree view".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("tree-view-disclosure-selection-stated"),
        },
        // Breadcrumbs (hierarchy / path stale) — the hierarchy / path signal is stale / partial, so the
        // trail auto-narrows to a hierarchy-unverified projection that keeps the last-known ancestry
        // visible, never a live current path (yellow).
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:breadcrumbs-hierarchy-stale".to_owned(),
            component_family: M5NavigationContentComponentFamily::Breadcrumbs,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:breadcrumbs:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NavContentComponentNonVisualReachState::DisclosedReducedButReachable,
            reduced_motion_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:breadcrumbs-hierarchy-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "trail_identity",
                "hierarchy_path",
                "current_object",
                "last_known_ancestry",
            ]),
            full_ready_claim: M5NavContentComponentClaim::CurrentNavigationResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::HierarchyPathClarity,
                M5NavContentComponentConditionState::HierarchyPathStale,
            )],
            claim_narrow: Some(NavContentComponentClaimAutoNarrow {
                narrowed_to: M5NavContentComponentClaim::HierarchyUnverifiedProjection,
                binding_dimension: M5NavContentComponentClaimDimension::HierarchyPathClarity,
                trigger: M5NavigationContentDowngradeTrigger::HierarchyPathUnstated,
                narrowed_label:
                    "This trail's hierarchy is stale or only partially resolved — shown as a hierarchy-unverified projection that keeps the last-known ancestry and the current object visible, never as a freshly-resolved, live current path"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "trail_identity",
                "hierarchy_path",
                "current_object",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::ExplorerUi,
                M5NavigationContentConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UI/UX Spec artifact hierarchy — Breadcrumbs".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("breadcrumbs-hierarchy-stale"),
        },
        // List view (count scope unresolved) — the exact / loaded / all-matching count scope is
        // unresolved, so the list auto-narrows to a count-unverified projection that keeps the
        // last-known loaded scope visible and never collapses the scopes into one exact total, never a
        // single authoritative total (yellow).
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:list-view-count-scope-unresolved".to_owned(),
            component_family: M5NavigationContentComponentFamily::ListView,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:list-view:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:list-view-count-scope-unresolved:a11y".to_owned(),
            copy_export: copy_export(&[
                "list_identity",
                "loaded_count",
                "all_matching_count",
                "hidden_by_filter_count",
            ]),
            full_ready_claim: M5NavContentComponentClaim::CurrentNavigationResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::CountScopeClarity,
                M5NavContentComponentConditionState::CountScopeUnresolved,
            )],
            claim_narrow: Some(NavContentComponentClaimAutoNarrow {
                narrowed_to: M5NavContentComponentClaim::CountUnverifiedProjection,
                binding_dimension: M5NavContentComponentClaimDimension::CountScopeClarity,
                trigger: M5NavigationContentDowngradeTrigger::CountScopeCollapsed,
                narrowed_label:
                    "This list's exact / loaded / all-matching count scope is unresolved — shown as a count-unverified projection that keeps the loaded scope and any hidden-by-filter count distinct and visible, never collapsing the scopes into one exact total"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "list_identity",
                "loaded_count",
                "all_matching_count",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::SearchUi,
                M5NavigationContentConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Design System core content — List view".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("list-view-count-scope-unresolved"),
        },
        // Table / grid (sort / filter provenance stale) — the sort / filter provenance is stale, so the
        // grid auto-narrows to a sort-filter-unverified projection that names the last-known ordering,
        // never a canonically ordered, current grid (yellow).
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:table-grid-sort-filter-provenance-stale".to_owned(),
            component_family: M5NavigationContentComponentFamily::TableGrid,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:table-grid:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NavContentComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:table-grid-sort-filter-provenance-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "grid_identity",
                "sort_provenance",
                "filter_provenance",
                "last_known_ordering",
            ]),
            full_ready_claim: M5NavContentComponentClaim::CurrentNavigationResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::SortFilterProvenanceClarity,
                M5NavContentComponentConditionState::SortFilterProvenanceStale,
            )],
            claim_narrow: Some(NavContentComponentClaimAutoNarrow {
                narrowed_to: M5NavContentComponentClaim::SortFilterUnverifiedProjection,
                binding_dimension: M5NavContentComponentClaimDimension::SortFilterProvenanceClarity,
                trigger: M5NavigationContentDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This grid's sort / filter provenance is stale — shown as a sort-filter-unverified projection that names the last-known ordering and filter, never presenting a stale ordering as a canonically sorted, current grid"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "grid_identity",
                "sort_provenance",
                "filter_provenance",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::DataUi,
                M5NavigationContentConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Design System core content — Table / grid".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("table-grid-sort-filter-provenance-stale"),
        },
        // Panel header (source freshness partial) — the source-freshness cue is only partial / cached,
        // so the header auto-narrows to a source-freshness projection that discloses the cached /
        // partial cue and the active context, never a freshly-current header (yellow). A partial
        // source-freshness cue is an honest disclosed-absence operation, not a current overstatement.
        NavContentComponentAccessibilityRow {
            record_kind: NAV_CONTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NAV_CONTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:panel-header-source-freshness-partial".to_owned(),
            component_family: M5NavigationContentComponentFamily::PanelHeader,
            source_family_schema_ref: NAV_CONTENT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "navigation:panel-header:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NavContentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: NavContentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:panel-header-source-freshness-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "header_identity",
                "active_context",
                "source_freshness_cue",
                "partial_capture_note",
            ]),
            full_ready_claim: M5NavContentComponentClaim::CurrentNavigationResult,
            claim_conditions: vec![condition(
                M5NavContentComponentClaimDimension::SourceFreshnessClarity,
                M5NavContentComponentConditionState::SourceFreshnessPartial,
            )],
            claim_narrow: Some(NavContentComponentClaimAutoNarrow {
                narrowed_to: M5NavContentComponentClaim::SourceFreshnessProjection,
                binding_dimension: M5NavContentComponentClaimDimension::SourceFreshnessClarity,
                trigger: M5NavigationContentDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This panel's source-freshness cue is only partial or cached — shown as a source-freshness projection that discloses the cached / partial cue alongside the active context, never as a freshly-current, fully-verified header"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "header_identity",
                "active_context",
                "source_freshness_cue",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NavigationContentConsumerSurface::ShellUi,
                M5NavigationContentConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UX Design System core navigation — Panel header".to_owned(),
                NAV_CONTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("panel-header-source-freshness-partial"),
        },
    ]
}
