//! Canvas/list/table/textual fallback parity, keyboard and screen-reader
//! navigation, and export-safe summaries for the M5 profiler/topology
//! components.
//!
//! This module is the M05-801 accessibility hardening capstone over the frozen
//! M5 profiler/topology component matrix. It certifies, per component family,
//! that every canvas-heavy consumer (flamegraph, icicle, heap/profile compare,
//! trace timeline, topology map) binds its visual canvas to an equivalent
//! list/table/textual path with the same filter/sort/range semantics, that
//! keyboard traversal and screen-reader labeling never strand assistive-tech
//! users in a view-only chart, that zoom and high-density rendering stay legible
//! or disclose their reduction, that an export-safe summary object preserves the
//! component meaning without relying on a screenshot, and that narrower
//! rendering surfaces (companion app, read-only browser, handoff packet, CLI,
//! support export) narrow interactivity *explicitly* while keeping the same
//! labels and summary vocabulary rather than silently dropping state or actions.
//!
//! It reuses the shared copy/export, reduced-capability, support-export, and
//! auto-narrowing disclosure structs from
//! [`crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows`]
//! so the exported labels stay byte-identical to the sibling profiler component
//! packets and there is no controlled-vocabulary drift.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows::{
    AutoNarrowingContract, CopyExportProjection, ReducedCapabilityBanner, SupportExportJoin,
};

/// Schema version stamped on the M05-801 accessibility fallback packet.
pub const COMPONENT_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ComponentFallbackPacket`].
pub const COMPONENT_FALLBACK_RECORD_KIND: &str = "m5_component_accessibility_fallback_packet";

/// Stable record-kind tag for each [`ComponentFallbackRow`].
pub const COMPONENT_FALLBACK_ROW_RECORD_KIND: &str = "m5_component_accessibility_fallback_row";

/// Repo-relative path to the checked-in M05-801 packet.
pub const COMPONENT_FALLBACK_PACKET_PATH: &str =
    "artifacts/perf/m5/m5-component-accessibility-fallback-components.json";

/// Embedded checked-in M05-801 packet JSON.
pub const COMPONENT_FALLBACK_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/m5-component-accessibility-fallback-components.json"
));

/// The ten frozen M5 profiler/topology component families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentFamily {
    ProfileSessionCard,
    FlamegraphView,
    IcicleView,
    CallTreeRow,
    HeapProfileCompareCard,
    TraceTimeline,
    WorksetSwitcherRow,
    TopologyNodeCard,
    OwnershipCard,
    ExplainerSectionCard,
}

impl M5ComponentFamily {
    /// Every component family, in matrix order.
    pub const ALL: [M5ComponentFamily; 10] = [
        M5ComponentFamily::ProfileSessionCard,
        M5ComponentFamily::FlamegraphView,
        M5ComponentFamily::IcicleView,
        M5ComponentFamily::CallTreeRow,
        M5ComponentFamily::HeapProfileCompareCard,
        M5ComponentFamily::TraceTimeline,
        M5ComponentFamily::WorksetSwitcherRow,
        M5ComponentFamily::TopologyNodeCard,
        M5ComponentFamily::OwnershipCard,
        M5ComponentFamily::ExplainerSectionCard,
    ];

    /// Returns true when the family renders a visual canvas or map that must
    /// bind to an equivalent non-visual list/table/textual path.
    pub const fn is_canvas_heavy(self) -> bool {
        matches!(
            self,
            M5ComponentFamily::FlamegraphView
                | M5ComponentFamily::IcicleView
                | M5ComponentFamily::HeapProfileCompareCard
                | M5ComponentFamily::TraceTimeline
                | M5ComponentFamily::TopologyNodeCard
        )
    }
}

/// A rendered fallback modality for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackModality {
    Canvas,
    List,
    Table,
    Textual,
}

impl M5FallbackModality {
    /// Returns true when the modality is reachable without interpreting a
    /// visual canvas (i.e. a keyboard/screen-reader path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Table | Self::Textual)
    }
}

/// Semantic consumer surface, matching the matrix `consumer_surface` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityConsumerSurface {
    DesktopProfilerWorkspace,
    HotspotWorkspace,
    TraceViewer,
    HeapAnalysis,
    ProfileCompare,
    TopologyMap,
    OwnershipBrowser,
    ArchitectureExplainer,
    SearchResults,
    ReviewWorkspace,
    OnboardingTour,
    AiContextPanel,
    IncidentWorkspace,
    DocsHelp,
    CliHeadless,
    SupportExport,
    ReleaseProof,
}

/// A rendering-surface capability tier. Distinct from the semantic consumer
/// surface: the same consumer may render at desktop-full capability or narrow
/// to a companion, read-only browser, handoff packet, CLI, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RenderingSurface {
    DesktopFull,
    CompanionApp,
    BrowserReadonly,
    HandoffPacket,
    CliHeadless,
    SupportExport,
}

impl M5RenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// Keyboard/screen-reader reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only chart/map that traps keyboard or assistive-tech users (red).
    ViewOnlyTrap,
}

impl NonVisualReachState {
    /// Returns true when the state never strands keyboard/assistive-tech users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }
}

/// Zoom / high-density rendering correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomDensityState {
    /// Legible and stable under zoom and high-density/compact layouts.
    LegibleUnderZoomAndDensity,
    /// Reduced legibility, but disclosed (yellow).
    DisclosedReducedLegibility,
    /// Content truncated or lost on zoom/compaction with no disclosure (red).
    TruncatedOrLostOnZoomOrDensity,
}

impl ZoomDensityState {
    /// Returns true when zoom/density never silently truncates or loses truth.
    pub const fn never_loses_truth(self) -> bool {
        !matches!(self, Self::TruncatedOrLostOnZoomOrDensity)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedLegibility)
    }
}

/// Whether an export-safe summary preserves the component meaning.
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
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderingNarrowingDisclosure {
    pub rendering_surface: M5RenderingSurface,
    pub state: NarrowingDisclosureState,
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a component fallback row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentFallbackStatus {
    /// Full canvas/non-visual/export/narrowing parity (green).
    Parity,
    /// Reduced but fully disclosed and reachable (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, or drops state silently (red).
    Stranded,
}

/// Accessibility fallback parity row for one component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentFallbackRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub component_family: M5ComponentFamily,
    pub source_family_schema_ref: String,
    /// Rendered modalities offered; canvas-heavy families must also offer a
    /// non-visual path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5FallbackModality>,
    /// The non-visual path preserves the same filter/sort/range semantics.
    pub filter_sort_range_parity: bool,
    pub keyboard_reach: NonVisualReachState,
    pub screen_reader_reach: NonVisualReachState,
    pub zoom_density: ZoomDensityState,
    pub export_summary: ExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    #[serde(default)]
    pub rendering_surfaces: Vec<M5RenderingSurface>,
    #[serde(default)]
    pub narrowing_disclosures: Vec<RenderingNarrowingDisclosure>,
    #[serde(default)]
    pub consumer_surfaces: Vec<M5AccessibilityConsumerSurface>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl ComponentFallbackRow {
    /// Returns true when a canvas-heavy family offers at least one non-visual
    /// (list/table/textual) fallback modality.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// AC1: the component never strands keyboard or assistive-tech users in a
    /// view-only chart/map.
    pub fn keyboard_and_at_reachable(&self) -> bool {
        self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.filter_sort_range_parity
            && (!self.component_family.is_canvas_heavy() || self.has_non_visual_fallback())
    }

    /// AC2: the export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && self.zoom_density.never_loses_truth()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.screenshot_only_prohibited
    }

    /// AC3: every narrower rendering surface discloses its reduced interactivity
    /// and keeps the same labels, rather than silently dropping state.
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
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Derived qualification status.
    pub fn status(&self) -> ComponentFallbackStatus {
        if !self.keyboard_and_at_reachable()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
        {
            return ComponentFallbackStatus::Stranded;
        }
        let disclosed = self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.zoom_density.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction());
        if disclosed {
            ComponentFallbackStatus::NarrowedDisclosed
        } else {
            ComponentFallbackStatus::Parity
        }
    }
}

/// Rolled-up summary of an M05-801 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentFallbackSummary {
    pub family_count: usize,
    pub canvas_heavy_family_count: usize,
    pub all_canvas_heavy_have_non_visual_fallback: bool,
    pub all_keyboard_and_screen_reader_reachable: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Checked-in M05-801 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentFallbackPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ComponentFallbackRow>,
    pub summary: ComponentFallbackSummary,
}

impl ComponentFallbackPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComponentFallbackSummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let canvas_heavy: Vec<&ComponentFallbackRow> = self
            .rows
            .iter()
            .filter(|row| row.component_family.is_canvas_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ComponentFallbackStatus::Parity => green += 1,
                ComponentFallbackStatus::NarrowedDisclosed => yellow += 1,
                ComponentFallbackStatus::Stranded => red += 1,
            }
        }

        ComponentFallbackSummary {
            family_count: self.rows.len(),
            canvas_heavy_family_count: canvas_heavy.len(),
            all_canvas_heavy_have_non_visual_fallback: canvas_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_keyboard_and_screen_reader_reachable: self
                .rows
                .iter()
                .all(ComponentFallbackRow::keyboard_and_at_reachable),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ComponentFallbackRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ComponentFallbackRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComponentFallbackViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPONENT_FALLBACK_SCHEMA_VERSION {
            violations.push(ComponentFallbackViolation::SchemaVersion {
                expected: COMPONENT_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPONENT_FALLBACK_RECORD_KIND {
            violations.push(ComponentFallbackViolation::RecordKind {
                expected: COMPONENT_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComponentFallbackViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if row.record_kind != COMPONENT_FALLBACK_ROW_RECORD_KIND
                || row.schema_version != COMPONENT_FALLBACK_SCHEMA_VERSION
                || row.source_family_schema_ref.trim().is_empty()
                || row.fallback_modalities.is_empty()
            {
                violations.push(ComponentFallbackViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Canvas-heavy families must render a canvas *and* a non-visual path.
            if row.component_family.is_canvas_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5FallbackModality::Canvas)
            {
                violations.push(ComponentFallbackViolation::CanvasHeavyMissingCanvas {
                    id: row.row_id.clone(),
                });
            }

            // AC1: no keyboard/assistive-tech user stranded in a view-only chart.
            if !row.keyboard_and_at_reachable() {
                violations.push(ComponentFallbackViolation::CanvasStrandsAssistiveTech {
                    id: row.row_id.clone(),
                });
            }

            // AC2: exports preserve meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(ComponentFallbackViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrower rendering surfaces disclose reduced interactivity.
            if !row.narrowing_disclosed() {
                violations.push(
                    ComponentFallbackViolation::NarrowerConsumerDropsStateSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Copy/export parity: text/JSON/Markdown, screenshot prohibited.
            if !has_copy_export(&row.copy_export) {
                violations.push(ComponentFallbackViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Consumer parity: a canvas surface plus at least one secondary.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ComponentFallbackViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ComponentFallbackStatus::Stranded {
                violations.push(ComponentFallbackViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified exactly once.
        for family in M5ComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(ComponentFallbackViolation::MissingFamilyCoverage { family });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ComponentFallbackViolation::SummaryMismatch);
        }

        violations
    }
}

fn has_copy_export(copy_export: &CopyExportProjection) -> bool {
    copy_export.screenshot_only_prohibited
        && copy_export.formats.iter().any(|format| format == "text")
        && copy_export.formats.iter().any(|format| format == "json")
        && copy_export
            .formats
            .iter()
            .any(|format| format == "markdown")
        && !copy_export.export_fields.is_empty()
}

/// Loads the checked-in M05-801 packet.
pub fn current_component_fallback_packet() -> Result<ComponentFallbackPacket, serde_json::Error> {
    serde_json::from_str(COMPONENT_FALLBACK_PACKET_JSON)
}

/// Validation failure for M05-801 accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentFallbackViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { id: String },
    IncompleteRow { id: String },
    CanvasHeavyMissingCanvas { id: String },
    CanvasStrandsAssistiveTech { id: String },
    ExportRequiresScreenshot { id: String },
    NarrowerConsumerDropsStateSilently { id: String },
    MissingCopyExportParity { id: String },
    MissingConsumerParity { id: String },
    StrandedRow { id: String },
    MissingFamilyCoverage { family: M5ComponentFamily },
    SummaryMismatch,
}

impl fmt::Display for ComponentFallbackViolation {
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
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete fallback row: {id}"),
            Self::CanvasHeavyMissingCanvas { id } => {
                write!(f, "canvas-heavy row {id} does not render a canvas modality")
            }
            Self::CanvasStrandsAssistiveTech { id } => {
                write!(
                    f,
                    "row {id} strands keyboard/assistive-tech users in a view-only canvas"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::NarrowerConsumerDropsStateSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing reduced interactivity"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text/JSON/Markdown copy-export parity"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => {
                write!(f, "row {id} is stranded (red) and may not ship")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for ComponentFallbackViolation {}
