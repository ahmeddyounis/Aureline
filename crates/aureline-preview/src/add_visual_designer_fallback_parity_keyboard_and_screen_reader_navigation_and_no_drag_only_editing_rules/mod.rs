//! List / tree / textual fallback parity, keyboard and screen-reader navigation,
//! and no-drag-only editing rules for the M5 visual-designer components.
//!
//! This module is the M05-808 accessibility-hardening capstone over the frozen
//! M5 visual-designer component matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`]).
//! Where the freeze matrix defines the reusable canvas / tree / inspector / chip /
//! preview-row primitives and the 805-807 implementation lanes resolve their
//! per-target truth, this lane certifies — per component family — that the visual
//! designer remains **source-first and accessible rather than gesture-dependent**:
//!
//! - **List / tree / textual fallback parity.** Every canvas-heavy family (the
//!   design canvas and the breakpoint / device-preview row) binds its visual
//!   surface to an equivalent list / tree / textual path so a user is never
//!   trapped in a canvas-only workflow.
//! - **Keyboard and screen-reader reach.** Canvas selection, structure
//!   navigation, property editing, breakpoint / device switching, and source-sync
//!   inspection are each keyboard-complete and screen-reader-reachable, never a
//!   view-only chart that strands assistive-tech users.
//! - **No drag-only editing.** Any editing pattern with a pointer-drag affordance
//!   (canvas move / resize, tree reorder, inspector sliders, viewport resize)
//!   exposes a command-backed, source-aware alternative, or is explicitly gated
//!   and disclosed — a drag gesture is never the only path to a source-backed
//!   edit.
//! - **Low-resource / assistive-tech reach the same source-backed truth.** The
//!   non-visual path reaches the same source span / selection / state the canvas
//!   path shows, and low-resource rendering modes reach it too.
//! - **Honest auto-narrowing.** When an accessibility or reduced-capability state
//!   narrows a component, it discloses the narrowing with a precise trigger and
//!   preserves the key source-backed context rather than silently dropping it.
//!
//! Each [`ComponentAccessibilityRow`] keys on one
//! [`crate::M5VisualDesignerComponentFamily`] and reuses that frozen family
//! vocabulary plus [`crate::M5VisualDesignerRequiredLabel`] and
//! [`crate::M5VisualDesignerDowngradeTrigger`] rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the
//! sibling primitive packets.
//!
//! The packet is metadata-only: raw source bodies, diff hunks, credentials, and
//! provider payloads never cross this boundary; the packet carries only typed
//! class tokens, opaque summary / evidence refs, booleans, and redacted labels so
//! support and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking source.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-component-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-visual-designer-component-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/designer/m5_visual_designer_component_accessibility_fallback_contract.md`](../../../../docs/designer/m5_visual_designer_component_accessibility_fallback_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    M5VisualDesignerComponentFamily, M5VisualDesignerDowngradeTrigger,
    M5VisualDesignerRequiredLabel,
};

/// Schema version stamped on the M05-808 accessibility fallback packet.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ComponentAccessibilityPacket`].
pub const VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_visual_designer_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ComponentAccessibilityRow`].
pub const VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_visual_designer_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_DOC_REF: &str =
    "docs/designer/m5_visual_designer_component_accessibility_fallback_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this lane
/// certifies.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const VISUAL_DESIGNER_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_DESIGNER_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/components/m5-visual-designer-component-accessibility-fallback.md";

/// The reusable component families that render a visual canvas / device preview
/// and therefore MUST bind to an equivalent non-visual (list / tree / textual)
/// path.
const fn family_is_canvas_heavy(family: M5VisualDesignerComponentFamily) -> bool {
    matches!(
        family,
        M5VisualDesignerComponentFamily::DesignCanvas
            | M5VisualDesignerComponentFamily::BreakpointPreviewRow
    )
}

/// The reusable component families that expose a pointer-drag editing affordance
/// (canvas move / resize, tree reorder, inspector sliders, viewport resize) and
/// therefore MUST expose a command-backed, source-aware alternative.
const fn family_has_drag_affordance(family: M5VisualDesignerComponentFamily) -> bool {
    matches!(
        family,
        M5VisualDesignerComponentFamily::DesignCanvas
            | M5VisualDesignerComponentFamily::StructureTreeRow
            | M5VisualDesignerComponentFamily::PropertyInspectorRow
            | M5VisualDesignerComponentFamily::BreakpointPreviewRow
    )
}

/// A rendered fallback modality for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackModality {
    /// The visual canvas / device-preview surface.
    Canvas,
    /// A flat list projection.
    List,
    /// A tree projection.
    Tree,
    /// A textual / source-first projection.
    Textual,
}

impl M5FallbackModality {
    /// Returns true when the modality is reachable without interpreting a visual
    /// canvas (i.e. a keyboard / screen-reader path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Tree | Self::Textual)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canvas => "canvas",
            Self::List => "list",
            Self::Tree => "tree",
            Self::Textual => "textual",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer
/// surface: the same component may render at desktop-full capability or narrow to
/// a companion, read-only browser, handoff packet, CLI, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityRenderingSurface {
    /// The full-capability desktop designer.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A handoff packet.
    HandoffPacket,
    /// A headless CLI surface.
    CliHeadless,
    /// A support export.
    SupportExport,
}

impl M5AccessibilityRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The semantic consumer surface a visual-designer component is embedded in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignerConsumerSurface {
    /// The design-canvas workspace.
    DesignCanvasWorkspace,
    /// The structure / layers panel.
    StructurePanel,
    /// The property inspector panel.
    PropertyPanel,
    /// The breakpoint / device preview deck.
    PreviewDeck,
    /// The source-sync rail / chip strip.
    SourceSyncRail,
    /// Docs / help.
    DocsHelp,
    /// A support export.
    SupportExport,
    /// A release-proof surface.
    ReleaseProof,
}

impl M5VisualDesignerConsumerSurface {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignCanvasWorkspace => "design_canvas_workspace",
            Self::StructurePanel => "structure_panel",
            Self::PropertyPanel => "property_panel",
            Self::PreviewDeck => "preview_deck",
            Self::SourceSyncRail => "source_sync_rail",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ReleaseProof => "release_proof",
        }
    }
}

/// Keyboard / screen-reader / low-resource reach for a component's non-visual
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only chart / map that traps keyboard or assistive-tech users (red).
    ViewOnlyTrap,
}

impl NonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech users.
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

/// Whether a pointer-drag editing pattern is matched by a command-backed,
/// source-aware alternative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DragEditingState {
    /// No drag affordance, or every drag pattern has a command-backed,
    /// source-aware alternative at full parity (green).
    CommandBackedParity,
    /// A drag pattern is explicitly gated / disclosed and reachable through a
    /// command-backed alternative, with a disclosed reduction (yellow).
    GatedDisclosed,
    /// A drag gesture is the only path to a source-backed edit (red).
    DragOnlyTrap,
}

impl DragEditingState {
    /// Returns true when no editing path depends on a drag-only gesture.
    pub const fn never_drag_only(self) -> bool {
        !matches!(self, Self::DragOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::GatedDisclosed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandBackedParity => "command_backed_parity",
            Self::GatedDisclosed => "gated_disclosed",
            Self::DragOnlyTrap => "drag_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a
/// screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl ExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl NarrowingDisclosureState {
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

/// Copy / export parity for a component's accessible fallback: the same
/// list / tree / textual truth must be copyable as text / JSON / Markdown, and a
/// screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited
    /// as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5AccessibilityRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// An honest auto-narrowing block. When a component narrows under an accessibility
/// or reduced-capability state, it names why with a precise trigger and preserves
/// the key source-backed context rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityAutoNarrow {
    /// The frozen downgrade trigger (reused vocabulary) that caused the narrowing.
    pub trigger: M5VisualDesignerDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The key source-backed context is preserved rather than dropped; must hold.
    pub preserves_source_backed_context: bool,
}

impl AccessibilityAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves source-backed context
    /// and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_source_backed_context && !label_is_generic(&self.narrowed_label)
    }
}

/// Derived qualification status for a component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentAccessibilityStatus {
    /// Full fallback / keyboard / screen-reader / no-drag / export parity (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and command-backed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, is drag-only, needs a screenshot, or drops state
    /// silently (red).
    Stranded,
}

impl ComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility fallback parity row for one visual-designer component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilityRow {
    /// Record kind; must equal [`VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5VisualDesignerComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Rendered modalities offered; a canvas-heavy family must also offer a
    /// non-visual path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5FallbackModality>,
    /// The non-visual path reaches the same source-backed truth (span / selection /
    /// state) as the canvas path; must hold.
    pub reaches_source_backed_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: NonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: NonVisualReachState,
    /// Low-resource / reduced-capability rendering reach into the non-visual path.
    pub low_resource_reach: NonVisualReachState,
    /// Whether a drag editing pattern is matched by a command-backed alternative.
    pub drag_editing: DragEditingState,
    /// The command-backed, source-aware actions that stand in for drag gestures.
    #[serde(default)]
    pub command_backed_actions: Vec<String>,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CopyExportParity,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5AccessibilityRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RenderingNarrowingDisclosure>,
    /// The honest auto-narrow block, present only when the component is narrowed by
    /// an accessibility / reduced-capability state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrow: Option<AccessibilityAutoNarrow>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualDesignerRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in.
    #[serde(default)]
    pub consumer_surfaces: Vec<M5VisualDesignerConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ComponentAccessibilityRow {
    /// Returns true when this family renders a visual canvas / device preview and
    /// must bind to a non-visual path.
    pub const fn is_canvas_heavy(&self) -> bool {
        family_is_canvas_heavy(self.component_family)
    }

    /// Returns true when this family exposes a pointer-drag editing affordance.
    pub const fn has_drag_affordance(&self) -> bool {
        family_has_drag_affordance(self.component_family)
    }

    /// Returns true when at least one non-visual (list / tree / textual) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// AC1: no editing pattern depends on a drag-only gesture — every family with a
    /// drag affordance exposes a command-backed, source-aware alternative.
    pub fn no_drag_only_editing(&self) -> bool {
        if !self.drag_editing.never_drag_only() {
            return false;
        }
        if self.has_drag_affordance() {
            !self.command_backed_actions.is_empty()
        } else {
            // A family with no drag affordance is trivially command-backed.
            self.drag_editing == DragEditingState::CommandBackedParity
        }
    }

    /// AC2: assistive-tech and low-resource modes reach the same source-backed
    /// truth as the canvas path — nothing is a view-only trap, and a canvas-heavy
    /// family offers a non-visual fallback.
    pub fn reaches_source_backed_truth_via_at(&self) -> bool {
        self.reaches_source_backed_truth
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.low_resource_reach.never_traps()
            && (!self.is_canvas_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state.
    pub fn is_reduced(&self) -> bool {
        self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.low_resource_reach.is_disclosed_reduction()
            || self.drag_editing.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC3: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, and any reduced-capability state auto-narrows honestly
    /// (a precise trigger + preserved source-backed context) rather than silently
    /// dropping key context.
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
        // Every disclosure never silently drops and preserves labels.
        let disclosures_ok = self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        });
        if !disclosures_ok {
            return false;
        }
        // A reduced component must auto-narrow honestly; a full-parity component
        // must not carry a spurious auto-narrow block.
        match (&self.auto_narrow, self.is_reduced()) {
            (Some(narrow), true) => narrow.is_honest(),
            (Some(_), false) => false,
            (None, true) => false,
            (None, false) => true,
        }
    }

    /// Derived qualification status.
    pub fn status(&self) -> ComponentAccessibilityStatus {
        if !self.no_drag_only_editing()
            || !self.reaches_source_backed_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
        {
            return ComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
low_resource={low_resource} drag={drag} export={export} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            low_resource = self.low_resource_reach.as_str(),
            drag = self.drag_editing.as_str(),
            export = self.export_summary.as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-808 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilitySummary {
    pub family_count: usize,
    pub canvas_heavy_family_count: usize,
    pub all_canvas_heavy_have_non_visual_fallback: bool,
    pub all_reach_source_backed_truth_via_at: bool,
    pub all_no_drag_only_editing: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ComponentAccessibilityRow>,
}

/// Checked-in M05-808 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ComponentAccessibilityRow>,
    pub summary: ComponentAccessibilitySummary,
}

impl ComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: ComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ComponentAccessibilitySummary {
                family_count: 0,
                canvas_heavy_family_count: 0,
                all_canvas_heavy_have_non_visual_fallback: false,
                all_reach_source_backed_truth_via_at: false,
                all_no_drag_only_editing: false,
                all_export_summaries_preserve_meaning: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5VisualDesignerComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let canvas_heavy: Vec<&ComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_canvas_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ComponentAccessibilityStatus::Parity => green += 1,
                ComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ComponentAccessibilitySummary {
            family_count: self.rows.len(),
            canvas_heavy_family_count: canvas_heavy.len(),
            all_canvas_heavy_have_non_visual_fallback: canvas_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_source_backed_truth_via_at: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::reaches_source_backed_truth_via_at),
            all_no_drag_only_editing: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::no_drag_only_editing),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ComponentAccessibilityViolation::SchemaVersion {
                expected: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ComponentAccessibilityViolation::RecordKind {
                expected: VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(ComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // A canvas-heavy family must render a canvas *and* a non-visual path.
            if row.is_canvas_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5FallbackModality::Canvas)
            {
                violations.push(ComponentAccessibilityViolation::CanvasHeavyMissingCanvas {
                    id: row.row_id.clone(),
                });
            }

            // AC1: no drag-only editing pattern.
            if !row.no_drag_only_editing() {
                violations.push(ComponentAccessibilityViolation::DragOnlyEditing {
                    id: row.row_id.clone(),
                });
            }

            // AC2: assistive-tech / low-resource reach the same source truth.
            if !row.reaches_source_backed_truth_via_at() {
                violations.push(ComponentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(ComponentAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed and auto-narrow honest.
            if !row.narrowing_disclosed() {
                violations.push(
                    ComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ComponentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ComponentAccessibilityStatus::Stranded {
                violations.push(ComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5VisualDesignerComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(ComponentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("accessibility fallback packet serializes"),
        ) {
            violations.push(ComponentAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,low_resource_reach,drag_editing,export_summary,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{low_resource},{drag},{export},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                low_resource = row.low_resource_reach.as_str(),
                drag = row.drag_editing.as_str(),
                export = row.export_summary.as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Designer Component Accessibility Fallback\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5VisualDesignerComponentFamily::ALL.len(),
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
            if let Some(narrow) = &row.auto_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: trigger={} — {}\n",
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in accessibility fallback export.
pub fn current_m5_visual_designer_a11y_fallback_export(
) -> Result<ComponentAccessibilityPacket, ComponentAccessibilityArtifactError> {
    let packet: ComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(ComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ComponentAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in accessibility fallback export.
#[derive(Debug)]
pub enum ComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ComponentAccessibilityViolation>),
}

impl fmt::Display for ComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "accessibility fallback export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ComponentAccessibilityArtifactError {}

/// Validation failure for M05-808 accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentAccessibilityViolation {
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
    CanvasHeavyMissingCanvas {
        id: String,
    },
    DragOnlyEditing {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
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
        family: M5VisualDesignerComponentFamily,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ComponentAccessibilityViolation {
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
            Self::CanvasHeavyMissingCanvas { id } => {
                write!(f, "canvas-heavy row {id} does not render a canvas modality")
            }
            Self::DragOnlyEditing { id } => {
                write!(
                    f,
                    "row {id} depends on a drag-only editing pattern with no command-backed alternative"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands assistive-tech / low-resource users from the source-backed truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows without disclosing or auto-narrowing honestly"
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
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for ComponentAccessibilityViolation {}

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

/// Builds the canonical, checked-in accessibility fallback packet. This is the one
/// source of truth shared by the tests, the example dump, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_visual_designer_a11y_fallback_packet() -> ComponentAccessibilityPacket {
    ComponentAccessibilityPacket::new(ComponentAccessibilityPacketInput {
        packet_id: "m5-visual-designer-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-03T00:00:00Z".to_owned(),
        matrix_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-designer-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5VisualDesignerRequiredLabel> {
    M5VisualDesignerRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seeded_rows() -> Vec<ComponentAccessibilityRow> {
    vec![
        // Design canvas — canvas-heavy, drag-affordant; reachable but the complex
        // canvas discloses a screen-reader reduction and gates its drag gestures
        // behind a command-backed alternative (yellow).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:design-canvas".to_owned(),
            component_family: M5VisualDesignerComponentFamily::DesignCanvas,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::Canvas,
                M5FallbackModality::Tree,
                M5FallbackModality::List,
                M5FallbackModality::Textual,
            ],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::GatedDisclosed,
            command_backed_actions: vec![
                "select_node_by_keyboard".to_owned(),
                "move_via_arrow_keys".to_owned(),
                "resize_via_command_palette".to_owned(),
                "reorder_via_structure_tree".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:design-canvas:a11y".to_owned(),
            copy_export: copy_export(&["selection_id", "canvas_state", "source_revision_ref"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::CompanionApp,
                M5AccessibilityRenderingSurface::SupportExport,
            ],
            narrowing_disclosures: vec![
                RenderingNarrowingDisclosure {
                    rendering_surface: M5AccessibilityRenderingSurface::CompanionApp,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec![
                        "identity".to_owned(),
                        "source_ownership".to_owned(),
                        "state".to_owned(),
                    ],
                    reduced_interactions: vec!["direct_canvas_drag".to_owned()],
                },
                RenderingNarrowingDisclosure {
                    rendering_surface: M5AccessibilityRenderingSurface::SupportExport,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec!["identity".to_owned(), "state".to_owned()],
                    reduced_interactions: vec!["interactive_edit".to_owned()],
                },
            ],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
                narrowed_label:
                    "The visual canvas exposes a keyboard-reachable structure tree and textual outline; complex spatial relationships are summarized for screen readers rather than dropped".to_owned(),
                preserves_source_backed_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::DesignCanvasWorkspace,
                M5VisualDesignerConsumerSurface::StructurePanel,
                M5VisualDesignerConsumerSurface::SupportExport,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("design-canvas"),
        },
        // Structure-tree row — already a tree; keyboard-complete reorder with a
        // command-backed alternative to drag reordering (green).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:structure-tree-row".to_owned(),
            component_family: M5VisualDesignerComponentFamily::StructureTreeRow,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![M5FallbackModality::Tree, M5FallbackModality::Textual],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::CommandBackedParity,
            command_backed_actions: vec![
                "move_up_down_by_keyboard".to_owned(),
                "reparent_via_command".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:structure-tree-row:a11y".to_owned(),
            copy_export: copy_export(&["node_kind", "source_span_ref", "selection_id"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5AccessibilityRenderingSurface::CliHeadless,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "source_ownership".to_owned(),
                    "state".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::StructurePanel,
                M5VisualDesignerConsumerSurface::DocsHelp,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("structure-tree-row"),
        },
        // Property-inspector row — already a list; slider drags are matched by
        // typed-value command entry (green).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:property-inspector-row".to_owned(),
            component_family: M5VisualDesignerComponentFamily::PropertyInspectorRow,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![M5FallbackModality::List, M5FallbackModality::Textual],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::CommandBackedParity,
            command_backed_actions: vec![
                "type_value_directly".to_owned(),
                "step_value_by_arrow_keys".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:property-inspector-row:a11y".to_owned(),
            copy_export: copy_export(&["value_state", "write_scope", "preview_diff"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::BrowserReadonly,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5AccessibilityRenderingSurface::BrowserReadonly,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "state".to_owned(),
                    "source_ownership".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::PropertyPanel,
                M5VisualDesignerConsumerSurface::SupportExport,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("property-inspector-row"),
        },
        // Source-sync chip — read-only inspection; keyboard/SR reachable, no drag
        // affordance (green).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:source-sync-chip".to_owned(),
            component_family: M5VisualDesignerComponentFamily::SourceSyncChip,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![M5FallbackModality::Textual, M5FallbackModality::List],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::CommandBackedParity,
            command_backed_actions: vec![],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:source-sync-chip:a11y".to_owned(),
            copy_export: copy_export(&["sync_class", "recovery_route"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::HandoffPacket,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5AccessibilityRenderingSurface::HandoffPacket,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "sync_or_freshness".to_owned(),
                    "state".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::SourceSyncRail,
                M5VisualDesignerConsumerSurface::ReleaseProof,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("source-sync-chip"),
        },
        // Breakpoint / device-preview row — canvas-heavy; low-resource rendering
        // discloses a reduction and viewport-resize drag is gated behind a
        // command-backed preset switcher (yellow).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:breakpoint-preview-row".to_owned(),
            component_family: M5VisualDesignerComponentFamily::BreakpointPreviewRow,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::Canvas,
                M5FallbackModality::List,
                M5FallbackModality::Textual,
            ],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::DisclosedReducedButReachable,
            drag_editing: DragEditingState::GatedDisclosed,
            command_backed_actions: vec![
                "switch_device_by_command".to_owned(),
                "cycle_breakpoints_by_keyboard".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:breakpoint-preview-row:a11y".to_owned(),
            copy_export: copy_export(&["device_class", "data_posture", "mapping_quality"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::CompanionApp,
                M5AccessibilityRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![
                RenderingNarrowingDisclosure {
                    rendering_surface: M5AccessibilityRenderingSurface::CompanionApp,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec![
                        "identity".to_owned(),
                        "state".to_owned(),
                        "sync_or_freshness".to_owned(),
                    ],
                    reduced_interactions: vec!["live_runtime_preview".to_owned()],
                },
                RenderingNarrowingDisclosure {
                    rendering_surface: M5AccessibilityRenderingSurface::CliHeadless,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec!["identity".to_owned(), "state".to_owned()],
                    reduced_interactions: vec!["rendered_pixels".to_owned()],
                },
            ],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
                narrowed_label:
                    "In low-resource mode the device preview renders a textual viewport/data-posture summary instead of the live runtime pixels while keeping the runtime origin and mapping quality visible".to_owned(),
                preserves_source_backed_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::PreviewDeck,
                M5VisualDesignerConsumerSurface::DocsHelp,
                M5VisualDesignerConsumerSurface::SupportExport,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("breakpoint-preview-row"),
        },
        // Unsupported-construct card — textual by nature; keyboard/SR reachable, no
        // drag affordance (green).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:unsupported-construct-card".to_owned(),
            component_family: M5VisualDesignerComponentFamily::UnsupportedConstructCard,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![M5FallbackModality::Textual],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::CommandBackedParity,
            command_backed_actions: vec![],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:unsupported-construct-card:a11y".to_owned(),
            copy_export: copy_export(&["reason", "card_label"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::SupportExport,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5AccessibilityRenderingSurface::SupportExport,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec!["identity".to_owned(), "state".to_owned()],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::PropertyPanel,
                M5VisualDesignerConsumerSurface::DocsHelp,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("unsupported-construct-card"),
        },
        // Round-trip conflict banner — textual by nature; keyboard/SR reachable, no
        // drag affordance (green).
        ComponentAccessibilityRow {
            record_kind: VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:round-trip-conflict-banner".to_owned(),
            component_family: M5VisualDesignerComponentFamily::RoundTripConflictBanner,
            source_family_schema_ref: VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            fallback_modalities: vec![M5FallbackModality::Textual, M5FallbackModality::List],
            reaches_source_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            low_resource_reach: NonVisualReachState::ReachableAndLabeled,
            drag_editing: DragEditingState::CommandBackedParity,
            command_backed_actions: vec![],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:round-trip-conflict-banner:a11y".to_owned(),
            copy_export: copy_export(&["conflict_class", "resolution_route"]),
            rendering_surfaces: vec![
                M5AccessibilityRenderingSurface::DesktopFull,
                M5AccessibilityRenderingSurface::HandoffPacket,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5AccessibilityRenderingSurface::HandoffPacket,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "state".to_owned(),
                    "source_ownership".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5VisualDesignerConsumerSurface::DesignCanvasWorkspace,
                M5VisualDesignerConsumerSurface::ReleaseProof,
            ],
            source_refs: vec![VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("round-trip-conflict-banner"),
        },
    ]
}
