//! Accessibility-surface descriptors and OS accessibility-bridge mappings for the
//! claimed M5 custom-rendered dynamic surfaces.
//!
//! Where the frozen dynamic-surface matrix
//! ([`crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`])
//! governs *which* accessibility objects a custom surface may publish and *which*
//! controlled vocabularies they carry, this module materializes the concrete
//! per-surface truth: one [`M5AccessibilitySurfaceDescriptor`] per claimed custom
//! surface that binds a stable surface id to its semantic roles/regions, its
//! screen-reader label model, its focus-order metadata, its reduced-motion and
//! high-zoom postures, and its current OS accessibility-bridge mapping and health.
//!
//! The descriptors are the single machine-readable description shell, editor,
//! terminal, notebook, data, and review surfaces map into the OS accessibility
//! bridge — so the bridge no longer depends on per-surface hand wiring that drifts
//! from docs, diagnostics, or assistive-tech proof artifacts. The same descriptors
//! are reused in diagnostics, support exports, docs/help, and assistive-tech
//! conformance packets. When a surface's bridge or proof goes stale the claimed
//! descriptor auto-narrows rather than implying silent screen-reader completeness,
//! and high-zoom / reduced-motion declarations stay explicit on any surface whose
//! behavior changes under those modes.
//!
//! The controlled state vocabularies — semantic role class, non-visual fidelity,
//! bridge state, focus-return disposition, announcement politeness, coalescing
//! strategy, and fallback durability — are reused verbatim from the frozen matrix
//! rather than minting parallel tokens. Only the descriptor-shaped vocabularies a
//! concrete surface adds (surface family, OS bridge kind, label name source, state
//! label class, reduced-motion posture, high-zoom posture, and bridge degradation
//! reason) are minted here, and they are frozen in a self-describing
//! [`M5SurfaceDescriptorVocabularySet`]. Raw provider payloads, credentials, secret
//! material, screenshots, and untranslated free-text prose stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-surface-descriptors.schema.json`](../../../../schemas/a11y/m5-surface-descriptors.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-surface-descriptors.md`](../../../../docs/a11y/m5-surface-descriptors.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-surface-descriptors/`](../../../../fixtures/a11y/m5-surface-descriptors/).

pub mod certification;
pub mod diagnostics;
pub mod events;
pub mod summaries;

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_surface_descriptor_catalog, seeded_m5_surface_descriptor_catalog_bridge_degraded,
    seeded_m5_surface_descriptor_catalog_proof_stale_narrowed,
    M5_SURFACE_DESCRIPTOR_CATALOG_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The frozen matrix owns the canonical controlled vocabularies; reuse its tokens
// rather than minting parallel synonyms.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use matrix::{
    A11yAnnouncementPoliteness, A11yBridgeState, A11yCoalescingStrategy, A11yFallbackDurability,
    A11yFocusReturnDisposition, A11yNonVisualFidelity, A11ySemanticRoleClass,
    M5DynamicSurfaceA11yConsumerSurface, M5DynamicSurfaceA11yDowngradeTrigger,
    M5DynamicSurfaceA11yProofFreshness, M5DynamicSurfaceA11yQualificationClass,
    M5DynamicSurfaceA11yReleasePosture, M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5SurfaceDescriptorCatalogPacket`].
pub const M5_SURFACE_DESCRIPTOR_RECORD_KIND: &str = "m5_accessibility_surface_descriptor_catalog";

/// Schema version for M5 accessibility-surface descriptor catalogs.
pub const M5_SURFACE_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_SURFACE_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/a11y/m5-surface-descriptors.schema.json";

/// Repo-relative path of the M5 surface-descriptor contract doc.
pub const M5_SURFACE_DESCRIPTOR_DOC_REF: &str = "docs/a11y/m5-surface-descriptors.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that
/// governs this lane's controlled vocabularies and qualification classes.
pub const M5_SURFACE_DESCRIPTOR_MATRIX_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the frozen accessibility-tree node taxonomy contract.
pub const M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF: &str =
    "docs/accessibility/accessibility_tree_contract.md";

/// Repo-relative path of the frozen focus / zoom / pointer-independence contract.
pub const M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF: &str =
    "docs/accessibility/focus_zoom_and_pointer_independence_contract.md";

/// Repo-relative path of the frozen visual-adaptation (zoom / motion) contract.
pub const M5_SURFACE_DESCRIPTOR_VISUAL_ADAPTATION_CONTRACT_REF: &str =
    "docs/accessibility/visual_adaptation_contract.md";

/// Repo-relative path of the frozen shell accessibility-bridge groundwork.
pub const M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF: &str =
    "docs/accessibility/m1_shell_bridge.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SURFACE_DESCRIPTOR_FIXTURE_DIR: &str = "fixtures/a11y/m5-surface-descriptors";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SURFACE_DESCRIPTOR_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-bridge-descriptor-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_SURFACE_DESCRIPTOR_SUMMARY_REF: &str =
    "artifacts/a11y/m5-bridge-descriptor-proof/bridge-descriptor-proof.md";

/// One claimed M5 custom-rendered surface family that must publish a descriptor.
///
/// These are exactly the dynamic surfaces the bridge contract must map: shell
/// regions, the editor and terminal canvases, dense lists/tables, notebook and
/// data cells, the review/diff surface, and durable overlay/sheet surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceFamily {
    /// Custom-rendered shell zone / landmark region.
    ShellRegion,
    /// Custom-rendered editor content canvas.
    EditorCanvas,
    /// Custom-rendered terminal / log canvas.
    TerminalCanvas,
    /// Dense list / table / data-grid collection.
    DenseCollection,
    /// Notebook cell (input + output).
    NotebookCell,
    /// Data-surface cell.
    DataCell,
    /// Review / diff hunk surface.
    ReviewDiff,
    /// Durable overlay / sheet / modal surface.
    OverlaySheet,
}

impl M5SurfaceFamily {
    /// Every claimed surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ShellRegion,
        Self::EditorCanvas,
        Self::TerminalCanvas,
        Self::DenseCollection,
        Self::NotebookCell,
        Self::DataCell,
        Self::ReviewDiff,
        Self::OverlaySheet,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellRegion => "shell_region",
            Self::EditorCanvas => "editor_canvas",
            Self::TerminalCanvas => "terminal_canvas",
            Self::DenseCollection => "dense_collection",
            Self::NotebookCell => "notebook_cell",
            Self::DataCell => "data_cell",
            Self::ReviewDiff => "review_diff",
            Self::OverlaySheet => "overlay_sheet",
        }
    }
}

/// OS accessibility bridge a descriptor maps into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceBridgeKind {
    /// Windows UI Automation.
    UiAutomation,
    /// macOS NSAccessibility.
    NsAccessibility,
    /// Linux AT-SPI.
    AtSpi,
    /// Headless inspector bridge (no platform accessibility API present).
    HeadlessInspector,
}

impl M5SurfaceBridgeKind {
    /// Every bridge kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UiAutomation,
        Self::NsAccessibility,
        Self::AtSpi,
        Self::HeadlessInspector,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiAutomation => "ui_automation",
            Self::NsAccessibility => "ns_accessibility",
            Self::AtSpi => "at_spi",
            Self::HeadlessInspector => "headless_inspector",
        }
    }
}

/// Source a screen-reader accessible name resolves from.
///
/// Aligns with the accessibility-tree contract's `name_source` taxonomy; a label
/// never originates from pixel-only rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceNameSource {
    /// A visible label rendered by the surface.
    VisibleLabel,
    /// A controlled message-id string.
    MessageIdSource,
    /// A document, file, or symbol label.
    DocumentOrSymbolLabel,
    /// A row, cell, or item identity.
    RowOrCellIdentity,
    /// A generated non-visual summary.
    GeneratedSummary,
    /// A degraded support fallback label.
    DegradedFallbackLabel,
}

impl M5SurfaceNameSource {
    /// Every name source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::VisibleLabel,
        Self::MessageIdSource,
        Self::DocumentOrSymbolLabel,
        Self::RowOrCellIdentity,
        Self::GeneratedSummary,
        Self::DegradedFallbackLabel,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleLabel => "visible_label",
            Self::MessageIdSource => "message_id_source",
            Self::DocumentOrSymbolLabel => "document_or_symbol_label",
            Self::RowOrCellIdentity => "row_or_cell_identity",
            Self::GeneratedSummary => "generated_summary",
            Self::DegradedFallbackLabel => "degraded_fallback_label",
        }
    }

    /// True when this source is a disclosed fallback rather than primary truth.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::DegradedFallbackLabel)
    }
}

/// Class of dynamic state a label model speaks alongside name/role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceStateLabelClass {
    /// Severity / diagnostic state.
    Severity,
    /// Freshness / staleness state.
    Freshness,
    /// Support / capability state.
    Support,
    /// Selection scope (including hidden-selected).
    SelectionScope,
    /// Virtualization / windowing truth.
    Virtualization,
    /// Trust / policy state.
    TrustOrPolicy,
    /// Live-region state.
    LiveRegion,
    /// No additional state class.
    NotApplicable,
}

impl M5SurfaceStateLabelClass {
    /// Every state label class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Severity,
        Self::Freshness,
        Self::Support,
        Self::SelectionScope,
        Self::Virtualization,
        Self::TrustOrPolicy,
        Self::LiveRegion,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Severity => "severity",
            Self::Freshness => "freshness",
            Self::Support => "support",
            Self::SelectionScope => "selection_scope",
            Self::Virtualization => "virtualization",
            Self::TrustOrPolicy => "trust_or_policy",
            Self::LiveRegion => "live_region",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// How a surface adapts under an OS reduced-motion request.
///
/// The first three tokens describe a surface whose behavior changes under reduced
/// motion; `motion_independent_already` and `not_applicable` describe a surface
/// that does not. The split is what the validator uses to keep the declaration
/// explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReducedMotionPosture {
    /// All animation suppressed; final state shown instantly.
    NoAnimation,
    /// Animation disabled but the animated state remains conveyed.
    AnimationDisabledStatePreserved,
    /// Motion crossfade replaced with an instant transition.
    CrossfadeReplacedWithInstant,
    /// Surface uses no motion regardless of the request.
    MotionIndependentAlready,
    /// Reduced motion does not apply to this surface.
    NotApplicable,
}

impl M5ReducedMotionPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoAnimation,
        Self::AnimationDisabledStatePreserved,
        Self::CrossfadeReplacedWithInstant,
        Self::MotionIndependentAlready,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAnimation => "no_animation",
            Self::AnimationDisabledStatePreserved => "animation_disabled_state_preserved",
            Self::CrossfadeReplacedWithInstant => "crossfade_replaced_with_instant",
            Self::MotionIndependentAlready => "motion_independent_already",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this posture describes a surface that adapts under reduced motion.
    pub const fn adapts(self) -> bool {
        matches!(
            self,
            Self::NoAnimation
                | Self::AnimationDisabledStatePreserved
                | Self::CrossfadeReplacedWithInstant
        )
    }
}

/// How a surface adapts under high OS zoom / large text.
///
/// The first three tokens describe a surface whose layout changes under high zoom;
/// `fixed_layout_no_change` and `not_applicable` describe a surface that does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HighZoomPosture {
    /// Reflows to a single column at high zoom.
    ReflowsToSingleColumn,
    /// Scrolls without clipping content at high zoom.
    ScrollsWithoutClipping,
    /// Content scales with its container at high zoom.
    ContentScalesWithContainer,
    /// Layout is fixed and does not change under zoom.
    FixedLayoutNoChange,
    /// High zoom does not apply to this surface.
    NotApplicable,
}

impl M5HighZoomPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReflowsToSingleColumn,
        Self::ScrollsWithoutClipping,
        Self::ContentScalesWithContainer,
        Self::FixedLayoutNoChange,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReflowsToSingleColumn => "reflows_to_single_column",
            Self::ScrollsWithoutClipping => "scrolls_without_clipping",
            Self::ContentScalesWithContainer => "content_scales_with_container",
            Self::FixedLayoutNoChange => "fixed_layout_no_change",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this posture describes a surface that adapts under high zoom.
    pub const fn adapts(self) -> bool {
        matches!(
            self,
            Self::ReflowsToSingleColumn
                | Self::ScrollsWithoutClipping
                | Self::ContentScalesWithContainer
        )
    }
}

/// Disclosed reason an OS accessibility bridge mapping is degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BridgeDegradationReason {
    /// No degradation; the bridge mapping is current.
    None,
    /// No platform accessibility API is available on this target.
    PlatformBridgeUnavailable,
    /// Only part of the semantic tree is mapped to the bridge.
    PartialTreeMapping,
    /// The bridge mapping is stale past its freshness floor.
    StaleMapping,
    /// Virtualization truth is not available to the bridge.
    VirtualizationTruthUnavailable,
    /// The live region is not available on the bridge.
    LiveRegionUnavailable,
    /// A policy or trust restriction blocks the mapping.
    PolicyOrTrustRestriction,
    /// Not applicable for this descriptor.
    NotApplicable,
}

impl M5BridgeDegradationReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::None,
        Self::PlatformBridgeUnavailable,
        Self::PartialTreeMapping,
        Self::StaleMapping,
        Self::VirtualizationTruthUnavailable,
        Self::LiveRegionUnavailable,
        Self::PolicyOrTrustRestriction,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PlatformBridgeUnavailable => "platform_bridge_unavailable",
            Self::PartialTreeMapping => "partial_tree_mapping",
            Self::StaleMapping => "stale_mapping",
            Self::VirtualizationTruthUnavailable => "virtualization_truth_unavailable",
            Self::LiveRegionUnavailable => "live_region_unavailable",
            Self::PolicyOrTrustRestriction => "policy_or_trust_restriction",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this reason discloses a real degradation.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::None | Self::NotApplicable)
    }
}

/// One semantic region / landmark exposed by a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceRegion {
    /// Stable region id, unique within the descriptor.
    pub region_id: String,
    /// Semantic role class for the region.
    pub role_class: A11ySemanticRoleClass,
    /// Human-readable region label.
    pub label: String,
    /// True when the region is a top-level landmark / navigation target.
    pub is_landmark: bool,
}

/// Screen-reader label model for a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceLabelModel {
    /// Stable label-model id.
    pub label_model_id: String,
    /// Source the accessible name resolves from.
    pub name_source: M5SurfaceNameSource,
    /// Dynamic state classes spoken alongside the name.
    pub state_label_classes: Vec<M5SurfaceStateLabelClass>,
    /// Durability of the name's delivery if a live region is missed.
    pub fallback_durability: A11yFallbackDurability,
    /// Non-visual fidelity the label model exposes.
    pub non_visual_fidelity: A11yNonVisualFidelity,
}

/// One ordered focus stop within a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceFocusStop {
    /// Zero-based position in the focus order.
    pub order_index: u32,
    /// Region this stop belongs to.
    pub region_id: String,
    /// Semantic role class for the stop.
    pub role_class: A11ySemanticRoleClass,
    /// True when the stop is keyboard-focusable.
    pub focusable: bool,
}

/// Focus-order metadata for a surface descriptor, including async-update return.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceFocusOrder {
    /// Stable focus-contract id.
    pub focus_contract_id: String,
    /// Ordered focus stops, by ascending `order_index` starting at zero.
    pub stops: Vec<M5SurfaceFocusStop>,
    /// Where focus returns after an asynchronous update or overlay teardown.
    pub async_return_disposition: A11yFocusReturnDisposition,
    /// Durability of the focus-return target if the prior owner is destroyed.
    pub return_fallback_durability: A11yFallbackDurability,
}

/// Reduced-motion and high-zoom posture for a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceMotionZoomPosture {
    /// Reduced-motion adaptation posture.
    pub reduced_motion: M5ReducedMotionPosture,
    /// High-zoom adaptation posture.
    pub high_zoom: M5HighZoomPosture,
    /// True when the surface's behavior changes under reduced motion.
    pub behavior_changes_under_reduced_motion: bool,
    /// True when the surface's layout changes under high zoom.
    pub behavior_changes_under_high_zoom: bool,
}

/// Native role hints for the OS accessibility bridge mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceNativeRoleHint {
    /// UI Automation control type, when mapped.
    pub ui_automation: Option<String>,
    /// NSAccessibility role, when mapped.
    pub ns_accessibility: Option<String>,
    /// AT-SPI role, when mapped.
    pub at_spi: Option<String>,
}

/// OS accessibility-bridge mapping and current health for a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceBridgeMapping {
    /// Bridge this descriptor maps into.
    pub bridge_kind: M5SurfaceBridgeKind,
    /// Current bridge connection state (health).
    pub bridge_state: A11yBridgeState,
    /// Non-visual fidelity the bridge currently delivers for this surface.
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// Native role hints used to map the primary role into the bridge.
    pub native_role_hint: M5SurfaceNativeRoleHint,
    /// Disclosed reason the mapping is degraded, if any.
    pub degradation_reason: M5BridgeDegradationReason,
}

/// Live-announcement posture for a surface descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceLiveAnnouncement {
    /// Politeness class for the surface's announcements.
    pub politeness: A11yAnnouncementPoliteness,
    /// Coalescing strategy so the live region never spams.
    pub coalescing: A11yCoalescingStrategy,
}

/// One accessibility-surface descriptor for a claimed M5 custom surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AccessibilitySurfaceDescriptor {
    /// Stable surface id, unique within the catalog.
    pub surface_id: String,
    /// Claimed custom-rendered surface family.
    pub surface_family: M5SurfaceFamily,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Owner role accountable for keeping this descriptor's truth current.
    pub owner_role: String,
    /// Qualification class earned by this surface's descriptor.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Primary semantic role class for the surface.
    pub primary_role_class: A11ySemanticRoleClass,
    /// Semantic regions / landmarks the surface exposes.
    pub regions: Vec<M5SurfaceRegion>,
    /// Screen-reader label model.
    pub label_model: M5SurfaceLabelModel,
    /// Focus-order metadata, including async return.
    pub focus_order: M5SurfaceFocusOrder,
    /// Reduced-motion and high-zoom posture.
    pub motion_zoom: M5SurfaceMotionZoomPosture,
    /// OS accessibility-bridge mapping and health.
    pub bridge_mapping: M5SurfaceBridgeMapping,
    /// Live-announcement posture.
    pub live_announcement: M5SurfaceLiveAnnouncement,
    /// Downgrade triggers that can narrow this descriptor below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this descriptor current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this descriptor.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this descriptor's qualification truth.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

impl M5AccessibilitySurfaceDescriptor {
    /// True when the descriptor's async return disposition is the non-interactive
    /// sentinel, meaning a focus stop list is not required.
    fn is_non_interactive(&self) -> bool {
        self.focus_order.async_return_disposition
            == A11yFocusReturnDisposition::FocusNotApplicableNonInteractive
    }
}

/// Self-describing controlled-vocabulary set for the descriptor-shaped tokens this
/// lane mints (the shared state vocabularies live in the frozen matrix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceDescriptorVocabularySet {
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Bridge-kind tokens.
    pub bridge_kinds: Vec<String>,
    /// Name-source tokens.
    pub name_sources: Vec<String>,
    /// State-label-class tokens.
    pub state_label_classes: Vec<String>,
    /// Reduced-motion-posture tokens.
    pub reduced_motion_postures: Vec<String>,
    /// High-zoom-posture tokens.
    pub high_zoom_postures: Vec<String>,
    /// Bridge-degradation-reason tokens.
    pub bridge_degradation_reasons: Vec<String>,
}

impl M5SurfaceDescriptorVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: M5SurfaceFamily::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            bridge_kinds: M5SurfaceBridgeKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            name_sources: M5SurfaceNameSource::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            state_label_classes: M5SurfaceStateLabelClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            reduced_motion_postures: M5ReducedMotionPosture::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            high_zoom_postures: M5HighZoomPosture::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            bridge_degradation_reasons: M5BridgeDegradationReason::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the descriptor lane.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceDescriptorConformanceReview {
    /// Every claimed custom surface family has at least one descriptor.
    pub every_claimed_surface_has_descriptor: bool,
    /// Descriptors expose semantic roles and regions, not visual-only state.
    pub descriptors_expose_roles_and_regions: bool,
    /// Descriptors carry a screen-reader label model.
    pub descriptors_carry_screen_reader_label_model: bool,
    /// Descriptors carry focus-order metadata.
    pub descriptors_carry_focus_order_metadata: bool,
    /// Focus never teleports or vanishes on an async update.
    pub focus_never_teleports_or_vanishes_on_async_update: bool,
    /// Reduced-motion / high-zoom declarations stay explicit when behavior changes.
    pub reduced_motion_and_high_zoom_declared_when_behavior_changes: bool,
    /// Bridge health is disclosed, never hidden.
    pub bridge_health_disclosed_not_hidden: bool,
    /// One descriptor contract replaces per-surface bridge hand wiring.
    pub one_descriptor_contract_not_per_surface_handwiring: bool,
    /// Descriptors are reused in diagnostics, support, docs/help, and AT proof.
    pub descriptors_reused_in_diagnostics_support_docs_and_proof: bool,
    /// Claimed descriptors auto-narrow when bridge or proof state goes stale.
    pub claimed_descriptors_auto_narrow_when_bridge_or_proof_stale: bool,
    /// No pixel-only render or pointer-only affordance is the source of truth.
    pub no_pixel_only_or_pointer_only_source_of_truth: bool,
    /// Downgrade narrows the claim rather than hiding the descriptor.
    pub downgrade_narrows_instead_of_hides: bool,
}

/// Consumer projection block: who reuses the descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceDescriptorConsumerProjection {
    /// Shell maps its regions from the descriptors.
    pub shell_consumes_descriptors: bool,
    /// Editor maps its canvas from the descriptors.
    pub editor_consumes_descriptors: bool,
    /// Terminal maps its canvas from the descriptors.
    pub terminal_consumes_descriptors: bool,
    /// Notebook maps its cells from the descriptors.
    pub notebook_consumes_descriptors: bool,
    /// Data grid maps its dense cells from the descriptors.
    pub data_grid_consumes_descriptors: bool,
    /// Review maps its diff surface from the descriptors.
    pub review_consumes_descriptors: bool,
    /// Diagnostics reuse the descriptors rather than re-deriving bridge state.
    pub diagnostics_reuse_descriptors: bool,
    /// Support export reuses the descriptors.
    pub support_export_reuses_descriptors: bool,
    /// Docs / help reuse the descriptors.
    pub docs_help_reuse_descriptors: bool,
    /// Assistive-tech conformance packets reuse the descriptors.
    pub at_conformance_packets_reuse_descriptors: bool,
}

/// Constructor input for [`M5SurfaceDescriptorCatalogPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SurfaceDescriptorCatalogPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Surface descriptors.
    pub descriptors: Vec<M5AccessibilitySurfaceDescriptor>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Descriptor-shaped controlled-vocabulary set.
    pub descriptor_vocabulary_set: M5SurfaceDescriptorVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5SurfaceDescriptorConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SurfaceDescriptorConsumerProjection,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 accessibility-surface descriptor catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceDescriptorCatalogPacket {
    /// Record kind; must equal [`M5_SURFACE_DESCRIPTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SURFACE_DESCRIPTOR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Surface descriptors.
    pub descriptors: Vec<M5AccessibilitySurfaceDescriptor>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Descriptor-shaped controlled-vocabulary set.
    pub descriptor_vocabulary_set: M5SurfaceDescriptorVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5SurfaceDescriptorConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SurfaceDescriptorConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SurfaceDescriptorCatalogPacket {
    /// Builds a descriptor catalog packet from seed input.
    pub fn new(input: M5SurfaceDescriptorCatalogPacketInput) -> Self {
        Self {
            record_kind: M5_SURFACE_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: M5_SURFACE_DESCRIPTOR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalog_label: input.catalog_label,
            descriptors: input.descriptors,
            shared_vocabulary_set: input.shared_vocabulary_set,
            descriptor_vocabulary_set: input.descriptor_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the descriptor-catalog invariants.
    pub fn validate(&self) -> Vec<M5SurfaceDescriptorViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SURFACE_DESCRIPTOR_RECORD_KIND {
            violations.push(M5SurfaceDescriptorViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SURFACE_DESCRIPTOR_SCHEMA_VERSION {
            violations.push(M5SurfaceDescriptorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SurfaceDescriptorViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_descriptors(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 surface descriptor catalog serializes"),
        ) {
            violations.push(M5SurfaceDescriptorViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 surface descriptor catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .descriptors
            .iter()
            .filter(|d| d.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Accessibility-Surface Descriptors and Bridge Mappings\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Descriptors: {} ({} stable)\n",
            self.descriptors.len(),
            stable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for descriptor in &self.descriptors {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}`\n",
                descriptor.surface_id,
                descriptor.surface_family.as_str(),
                descriptor.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", descriptor.owner_role));
            out.push_str(&format!(
                "  - Primary role: {}\n",
                descriptor.primary_role_class.as_str()
            ));
            out.push_str(&format!(
                "  - Bridge: {} / {} ({})\n",
                descriptor.bridge_mapping.bridge_kind.as_str(),
                descriptor.bridge_mapping.bridge_state.as_str(),
                descriptor.bridge_mapping.degradation_reason.as_str()
            ));
            out.push_str(&format!(
                "  - Reduced motion: {} / high zoom: {}\n",
                descriptor.motion_zoom.reduced_motion.as_str(),
                descriptor.motion_zoom.high_zoom.as_str()
            ));
            out.push_str(&format!(
                "  - Async focus return: {}\n",
                descriptor.focus_order.async_return_disposition.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in descriptor-catalog export.
#[derive(Debug)]
pub enum M5SurfaceDescriptorArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SurfaceDescriptorViolation>),
}

impl fmt::Display for M5SurfaceDescriptorArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 surface descriptor export parse failed: {error}"
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
                    "m5 surface descriptor export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SurfaceDescriptorArtifactError {}

/// Validation failures emitted by [`M5SurfaceDescriptorCatalogPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SurfaceDescriptorViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A claimed custom surface family has no descriptor.
    RequiredSurfaceFamilyMissing,
    /// A descriptor row is incomplete.
    DescriptorIncomplete,
    /// Two descriptors share a surface id.
    DuplicateSurfaceId,
    /// A descriptor's regions are empty or carry an empty id/label.
    RegionsIncomplete,
    /// A descriptor's focus order is malformed (empty when interactive, or the
    /// stop indices are not contiguous from zero).
    FocusOrderMalformed,
    /// A surface that changes under reduced motion or high zoom did not declare a
    /// concrete adaptation posture.
    MotionZoomDeclarationMissing,
    /// A degraded bridge did not disclose its degradation, narrow its claim, or
    /// carry a bridge downgrade trigger.
    BridgeDegradationNotDisclosed,
    /// A descriptor claiming Stable is missing required proof packet refs.
    StableDescriptorMissingProof,
    /// A descriptor has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A descriptor has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5SurfaceDescriptorViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceFamilyMissing => "required_surface_family_missing",
            Self::DescriptorIncomplete => "descriptor_incomplete",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::RegionsIncomplete => "regions_incomplete",
            Self::FocusOrderMalformed => "focus_order_malformed",
            Self::MotionZoomDeclarationMissing => "motion_zoom_declaration_missing",
            Self::BridgeDegradationNotDisclosed => "bridge_degradation_not_disclosed",
            Self::StableDescriptorMissingProof => "stable_descriptor_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable descriptor-catalog export.
pub fn current_stable_m5_surface_descriptor_export(
) -> Result<M5SurfaceDescriptorCatalogPacket, M5SurfaceDescriptorArtifactError> {
    let packet: M5SurfaceDescriptorCatalogPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-bridge-descriptor-proof/support_export.json"
    )))
    .map_err(M5SurfaceDescriptorArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SurfaceDescriptorArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SURFACE_DESCRIPTOR_SCHEMA_REF,
        M5_SURFACE_DESCRIPTOR_DOC_REF,
        M5_SURFACE_DESCRIPTOR_MATRIX_REF,
        M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_VISUAL_ADAPTATION_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SurfaceDescriptorViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.descriptor_vocabulary_set.matches_canonical()
    {
        violations.push(M5SurfaceDescriptorViolation::VocabularySetDrift);
    }
}

fn validate_descriptors(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let present: BTreeSet<M5SurfaceFamily> = packet
        .descriptors
        .iter()
        .map(|d| d.surface_family)
        .collect();
    for required in M5SurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5SurfaceDescriptorViolation::RequiredSurfaceFamilyMissing);
            break;
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for descriptor in &packet.descriptors {
        if !seen_ids.insert(descriptor.surface_id.as_str()) {
            violations.push(M5SurfaceDescriptorViolation::DuplicateSurfaceId);
        }

        if descriptor.surface_id.trim().is_empty()
            || descriptor.surface_label.trim().is_empty()
            || descriptor.owner_role.trim().is_empty()
            || descriptor.label_model.label_model_id.trim().is_empty()
            || descriptor.label_model.state_label_classes.is_empty()
            || descriptor.focus_order.focus_contract_id.trim().is_empty()
            || descriptor.source_contract_refs.is_empty()
        {
            violations.push(M5SurfaceDescriptorViolation::DescriptorIncomplete);
        }

        validate_descriptor_regions(descriptor, violations);
        validate_descriptor_focus_order(descriptor, violations);
        validate_descriptor_motion_zoom(descriptor, violations);
        validate_descriptor_bridge(descriptor, violations);

        if descriptor.qualification.is_stable() && descriptor.required_proof_packet_refs.is_empty()
        {
            violations.push(M5SurfaceDescriptorViolation::StableDescriptorMissingProof);
        }
        if descriptor.downgrade_triggers.is_empty() {
            violations.push(M5SurfaceDescriptorViolation::DowngradeTriggersMissing);
        }
        if descriptor.consumer_surfaces.is_empty() {
            violations.push(M5SurfaceDescriptorViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_descriptor_regions(
    descriptor: &M5AccessibilitySurfaceDescriptor,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    if descriptor.regions.is_empty() {
        violations.push(M5SurfaceDescriptorViolation::RegionsIncomplete);
        return;
    }
    let mut region_ids: BTreeSet<&str> = BTreeSet::new();
    for region in &descriptor.regions {
        if region.region_id.trim().is_empty()
            || region.label.trim().is_empty()
            || !region_ids.insert(region.region_id.as_str())
        {
            violations.push(M5SurfaceDescriptorViolation::RegionsIncomplete);
            return;
        }
    }
}

fn validate_descriptor_focus_order(
    descriptor: &M5AccessibilitySurfaceDescriptor,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let stops = &descriptor.focus_order.stops;
    if stops.is_empty() {
        // An empty focus order is only valid for an explicitly non-interactive
        // surface; every interactive surface must enumerate its focus stops.
        if !descriptor.is_non_interactive() {
            violations.push(M5SurfaceDescriptorViolation::FocusOrderMalformed);
        }
        return;
    }

    // Stop indices must be contiguous from zero and each stop must name a real
    // region, so the focus path a screen-reader user walks is exact.
    let region_ids: BTreeSet<&str> = descriptor
        .regions
        .iter()
        .map(|r| r.region_id.as_str())
        .collect();
    let mut indices: Vec<u32> = stops.iter().map(|s| s.order_index).collect();
    indices.sort_unstable();
    let contiguous = indices
        .iter()
        .enumerate()
        .all(|(expected, actual)| *actual as usize == expected);
    let regions_resolve = stops
        .iter()
        .all(|s| region_ids.contains(s.region_id.as_str()));
    if !contiguous || !regions_resolve {
        violations.push(M5SurfaceDescriptorViolation::FocusOrderMalformed);
    }
}

fn validate_descriptor_motion_zoom(
    descriptor: &M5AccessibilitySurfaceDescriptor,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let posture = &descriptor.motion_zoom;
    // A surface whose behavior changes under a mode MUST declare a concrete
    // adaptation posture; a surface that does not MUST declare a no-change posture.
    let reduced_motion_ok = if posture.behavior_changes_under_reduced_motion {
        posture.reduced_motion.adapts()
    } else {
        !posture.reduced_motion.adapts()
    };
    let high_zoom_ok = if posture.behavior_changes_under_high_zoom {
        posture.high_zoom.adapts()
    } else {
        !posture.high_zoom.adapts()
    };
    if !reduced_motion_ok || !high_zoom_ok {
        violations.push(M5SurfaceDescriptorViolation::MotionZoomDeclarationMissing);
    }
}

fn validate_descriptor_bridge(
    descriptor: &M5AccessibilitySurfaceDescriptor,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let bridge = &descriptor.bridge_mapping;
    if bridge.bridge_state == A11yBridgeState::BridgedActive {
        // A healthy bridge must not carry a degradation reason.
        if bridge.degradation_reason.is_degraded() {
            violations.push(M5SurfaceDescriptorViolation::BridgeDegradationNotDisclosed);
        }
        return;
    }

    // A degraded bridge must disclose the degradation, must not claim full
    // non-visual parity, must not stay Stable, and must carry a bridge downgrade
    // trigger so the narrowing is provable.
    let discloses_reason = bridge.degradation_reason.is_degraded();
    let fidelity_narrowed = bridge.non_visual_fidelity != A11yNonVisualFidelity::FullAccessible;
    let not_stable = !descriptor.qualification.is_stable();
    let has_bridge_trigger = descriptor.downgrade_triggers.iter().any(|t| {
        matches!(
            t,
            M5DynamicSurfaceA11yDowngradeTrigger::BridgePartialOrStale
                | M5DynamicSurfaceA11yDowngradeTrigger::BridgeUnavailable
        )
    });
    if !(discloses_reason && fidelity_narrowed && not_stable && has_bridge_trigger) {
        violations.push(M5SurfaceDescriptorViolation::BridgeDegradationNotDisclosed);
    }
}

fn validate_conformance_review(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.every_claimed_surface_has_descriptor,
        review.descriptors_expose_roles_and_regions,
        review.descriptors_carry_screen_reader_label_model,
        review.descriptors_carry_focus_order_metadata,
        review.focus_never_teleports_or_vanishes_on_async_update,
        review.reduced_motion_and_high_zoom_declared_when_behavior_changes,
        review.bridge_health_disclosed_not_hidden,
        review.one_descriptor_contract_not_per_surface_handwiring,
        review.descriptors_reused_in_diagnostics_support_docs_and_proof,
        review.claimed_descriptors_auto_narrow_when_bridge_or_proof_stale,
        review.no_pixel_only_or_pointer_only_source_of_truth,
        review.downgrade_narrows_instead_of_hides,
    ] {
        if !ok {
            violations.push(M5SurfaceDescriptorViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_descriptors,
        projection.editor_consumes_descriptors,
        projection.terminal_consumes_descriptors,
        projection.notebook_consumes_descriptors,
        projection.data_grid_consumes_descriptors,
        projection.review_consumes_descriptors,
        projection.diagnostics_reuse_descriptors,
        projection.support_export_reuses_descriptors,
        projection.docs_help_reuse_descriptors,
        projection.at_conformance_packets_reuse_descriptors,
    ] {
        if !ok {
            violations.push(M5SurfaceDescriptorViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SurfaceDescriptorViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SurfaceDescriptorCatalogPacket,
    violations: &mut Vec<M5SurfaceDescriptorViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5SurfaceDescriptorViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
