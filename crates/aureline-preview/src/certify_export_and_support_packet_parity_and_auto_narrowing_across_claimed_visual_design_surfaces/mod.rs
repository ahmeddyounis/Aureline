//! Export / support-packet parity and auto-narrowing certification across the
//! claimed M5 visual-design surfaces.
//!
//! This module is the M05-810 certification capstone that CLOSES the B94
//! visual-designer component lane. Where the freeze matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! froze the reusable canvas / tree / inspector / chip / preview-row primitives,
//! the 805-807 lanes resolved their per-target truth, the 808 lane certified
//! accessibility fallback, and the 809 lane adopted them across handoff
//! consumers, this lane certifies — per **claimed visual-design surface** — that
//! a visual-design claim self-narrows the moment its underlying mapping /
//! round-trip / preview-runtime truth weakens, and that the export / support
//! packet preserves exactly the mapping / runtime truth visible in-product.
//!
//! Every [`VisualDesignSurfaceCertRow`] keys on one
//! [`M5VisualDesignClaimedSurface`] and certifies three underlying truth
//! dimensions plus an always-on export-parity dimension:
//!
//! - **Mapping quality.** How well the surface maps back to canonical source
//!   ([`crate::M5BreakpointMappingQuality`]). An `unmapped` surface can only claim
//!   a source-only view; an `approximate` mapping narrows a writable claim to
//!   inspect-only.
//! - **Round-trip support.** Whether a visual action writes back to source
//!   ([`crate::RoundTripCapabilityClass`]). A surface with no write-back can never
//!   present as fully writable.
//! - **Preview-runtime freshness.** How fresh the rendered runtime is relative to
//!   its SLO ([`crate::PreviewFreshnessClass`]). A `stale` runtime narrows an
//!   interactive claim to read-only.
//! - **Export parity (always on).** The support / release export preserves the
//!   canvas / source selection identity, mapping quality, round-trip state,
//!   runtime origin, freshness, effective claim, and narrowed-capability reason —
//!   never a screenshot alone.
//!
//! The certified claim is a [`M5VisualDesignClaimTier`]: a surface declares the
//! claim it makes when every truth dimension is healthy, and the lane derives the
//! *effective* claim by narrowing the declared claim to the weakest dimension's
//! supported ceiling. A surface that keeps a claim above what its truth supports
//! is **hiding drift** and is rejected (red); a surface that narrows and discloses
//! is certified with a disclosed narrowing (yellow); a surface whose truth still
//! supports its claim is certified green.
//!
//! The packet is metadata-only: raw source bodies, diff hunks, credentials, and
//! provider payloads never cross this boundary; the packet carries only typed
//! class tokens, opaque summary / evidence refs, booleans, and redacted labels so
//! support and release exports can reconstruct exactly what the in-product surface
//! showed — and exactly how it narrowed — without leaking source.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-surface-certification.schema.json`](../../../../schemas/ui/m5-visual-designer-surface-certification.schema.json).
//! The contract doc is
//! [`docs/designer/m5_visual_designer_surface_certification_contract.md`](../../../../docs/designer/m5_visual_designer_surface_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CopyExportParity, M5BreakpointMappingQuality, M5PreviewRuntimeOrigin,
    M5VisualDesignerConsumerSurface, M5VisualDesignerDowngradeTrigger,
    M5VisualDesignerRequiredLabel, PreviewFreshnessClass, RoundTripCapabilityClass,
    SourceSyncClass,
};

/// Schema version stamped on the M05-810 surface-certification packet.
pub const VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualDesignSurfaceCertPacket`].
pub const VISUAL_DESIGNER_SURFACE_CERT_RECORD_KIND: &str =
    "m5_visual_designer_surface_certification_packet";

/// Stable record-kind tag carried by each [`VisualDesignSurfaceCertRow`].
pub const VISUAL_DESIGNER_SURFACE_CERT_ROW_RECORD_KIND: &str =
    "m5_visual_designer_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_DESIGNER_SURFACE_CERT_DOC_REF: &str =
    "docs/designer/m5_visual_designer_surface_certification_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this lane
/// certifies against.
pub const VISUAL_DESIGNER_SURFACE_CERT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the release bundle each row cites for its truth (AC2).
pub const VISUAL_DESIGNER_SURFACE_CERT_BUNDLE_REF: &str =
    "artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_DESIGNER_SURFACE_CERT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-surface-certification";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const VISUAL_DESIGNER_SURFACE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_DESIGNER_SURFACE_CERT_CSV_REF: &str =
    "artifacts/release/m5-visual-designer-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_DESIGNER_SURFACE_CERT_REPORT_REF: &str =
    "artifacts/components/m5-visual-designer-surface-certification.md";

/// The certified interactive-claim tier a visual-design surface makes. Higher
/// capability means a stronger public claim; the lane never lets a surface hold a
/// claim above what its underlying truth supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignClaimTier {
    /// Fully interactive and writable: visual edits round-trip to source.
    FullyInteractiveWritable,
    /// Inspect-only: mapped structure is navigable but nothing writes back.
    InspectOnly,
    /// Read-only: the rendered view is shown but not mapped for interaction.
    ReadOnly,
    /// Source-only: only the canonical source is trustworthy; the visual mapping
    /// is not claimed.
    SourceOnly,
}

impl M5VisualDesignClaimTier {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 4] = [
        Self::FullyInteractiveWritable,
        Self::InspectOnly,
        Self::ReadOnly,
        Self::SourceOnly,
    ];

    /// Capability rank; higher is a stronger claim.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullyInteractiveWritable => 3,
            Self::InspectOnly => 2,
            Self::ReadOnly => 1,
            Self::SourceOnly => 0,
        }
    }

    /// Returns the weaker (lower-rank) of two claim tiers.
    pub const fn weaker_of(self, other: Self) -> Self {
        if self.capability_rank() <= other.capability_rank() {
            self
        } else {
            other
        }
    }

    /// True when the tier claims a writable / interactive designer flow.
    pub const fn is_interactive_writable(self) -> bool {
        matches!(self, Self::FullyInteractiveWritable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyInteractiveWritable => "fully_interactive_writable",
            Self::InspectOnly => "inspect_only",
            Self::ReadOnly => "read_only",
            Self::SourceOnly => "source_only",
        }
    }

    /// A precise, non-generic label safe to render on any surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullyInteractiveWritable => "Fully interactive (writes back to source)",
            Self::InspectOnly => "Inspect-only (no write-back)",
            Self::ReadOnly => "Read-only view",
            Self::SourceOnly => "Source-only (visual mapping not claimed)",
        }
    }
}

/// The underlying truth dimension whose weakening narrowed a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignTruthDimension {
    /// How well the surface maps back to canonical source.
    MappingQuality,
    /// Whether a visual action writes back to source.
    RoundTripSupport,
    /// How fresh the rendered runtime is relative to its SLO.
    PreviewRuntimeFreshness,
    /// Whether the export preserves the same truth visible in-product.
    ExportParity,
}

impl M5VisualDesignTruthDimension {
    /// Every truth dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MappingQuality,
        Self::RoundTripSupport,
        Self::PreviewRuntimeFreshness,
        Self::ExportParity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MappingQuality => "mapping_quality",
            Self::RoundTripSupport => "round_trip_support",
            Self::PreviewRuntimeFreshness => "preview_runtime_freshness",
            Self::ExportParity => "export_parity",
        }
    }
}

/// The certification state of one truth dimension on a surface, relative to the
/// surface's declared claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisCertificationState {
    /// The observed truth supports the declared claim on this dimension (green).
    Certified,
    /// The observed truth weakened, and the claim narrowed and disclosed to stay
    /// within what the dimension supports (yellow).
    DisclosedNarrowed,
    /// The observed truth weakened but the claim stayed above what the dimension
    /// supports — drift is hidden (red).
    UndisclosedDrift,
}

impl AxisCertificationState {
    /// True when the dimension never hides drift.
    pub const fn never_hides_drift(self) -> bool {
        !matches!(self, Self::UndisclosedDrift)
    }

    /// True when the dimension carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The maximum claim tier a mapping-quality value supports.
const fn mapping_supported_ceiling(quality: M5BreakpointMappingQuality) -> M5VisualDesignClaimTier {
    match quality {
        M5BreakpointMappingQuality::Exact => M5VisualDesignClaimTier::FullyInteractiveWritable,
        M5BreakpointMappingQuality::Approximate => M5VisualDesignClaimTier::InspectOnly,
        M5BreakpointMappingQuality::Unmapped => M5VisualDesignClaimTier::SourceOnly,
    }
}

/// The maximum claim tier a round-trip capability supports.
const fn round_trip_supported_ceiling(
    round_trip: RoundTripCapabilityClass,
) -> M5VisualDesignClaimTier {
    match round_trip {
        RoundTripCapabilityClass::ExactSourceRoundTrip
        | RoundTripCapabilityClass::ApproximateSourceRoundTrip => {
            M5VisualDesignClaimTier::FullyInteractiveWritable
        }
        RoundTripCapabilityClass::InspectOnlyNoWrite | RoundTripCapabilityClass::NoRoundTrip => {
            M5VisualDesignClaimTier::InspectOnly
        }
        RoundTripCapabilityClass::SourceOnlyFallback => M5VisualDesignClaimTier::SourceOnly,
    }
}

/// The maximum claim tier a preview-runtime freshness value supports.
const fn freshness_supported_ceiling(freshness: PreviewFreshnessClass) -> M5VisualDesignClaimTier {
    match freshness {
        PreviewFreshnessClass::Fresh => M5VisualDesignClaimTier::FullyInteractiveWritable,
        PreviewFreshnessClass::Aging => M5VisualDesignClaimTier::InspectOnly,
        PreviewFreshnessClass::Stale | PreviewFreshnessClass::Unknown => {
            M5VisualDesignClaimTier::ReadOnly
        }
    }
}

/// The claimed visual-design surface a certification row keys on. Spans the
/// designer / preview / browser-runtime / framework surfaces plus the docs / help,
/// support, and release evidence surfaces that must preserve the same truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignClaimedSurface {
    /// The interactive design-canvas workspace.
    DesignCanvasWorkspace,
    /// The structure / layers tree panel.
    StructureTreePanel,
    /// The property-inspector panel.
    PropertyInspectorPanel,
    /// The source-sync / round-trip rail.
    SourceRoundTripRail,
    /// The breakpoint / device-preview deck.
    BreakpointDevicePreviewDeck,
    /// The framework-pack live preview.
    FrameworkPackPreview,
    /// The browser-runtime inspection surface.
    BrowserRuntimeInspection,
    /// Docs / help embeds.
    DocsHelpEmbeds,
    /// A support export.
    SupportExport,
    /// A release-proof surface.
    ReleaseProof,
}

impl M5VisualDesignClaimedSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::DesignCanvasWorkspace,
        Self::StructureTreePanel,
        Self::PropertyInspectorPanel,
        Self::SourceRoundTripRail,
        Self::BreakpointDevicePreviewDeck,
        Self::FrameworkPackPreview,
        Self::BrowserRuntimeInspection,
        Self::DocsHelpEmbeds,
        Self::SupportExport,
        Self::ReleaseProof,
    ];

    /// The docs / help, support, and release surfaces that must preserve the same
    /// mapping / runtime truth and narrowing that the in-product surfaces show.
    pub const EVIDENCE_SURFACES: [Self; 3] = [
        Self::DocsHelpEmbeds,
        Self::SupportExport,
        Self::ReleaseProof,
    ];

    /// True when this surface is an evidence surface (docs / help, support export,
    /// or release proof).
    pub const fn is_evidence_surface(self) -> bool {
        matches!(
            self,
            Self::DocsHelpEmbeds | Self::SupportExport | Self::ReleaseProof
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignCanvasWorkspace => "design_canvas_workspace",
            Self::StructureTreePanel => "structure_tree_panel",
            Self::PropertyInspectorPanel => "property_inspector_panel",
            Self::SourceRoundTripRail => "source_round_trip_rail",
            Self::BreakpointDevicePreviewDeck => "breakpoint_device_preview_deck",
            Self::FrameworkPackPreview => "framework_pack_preview",
            Self::BrowserRuntimeInspection => "browser_runtime_inspection",
            Self::DocsHelpEmbeds => "docs_help_embeds",
            Self::SupportExport => "support_export",
            Self::ReleaseProof => "release_proof",
        }
    }
}

/// A named field the export / support packet preserves so a stale or partial
/// visual-design lane can never present as fully writable or fully mapped in an
/// export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignCertExportField {
    /// The canvas / source selection identity.
    SelectionIdentity,
    /// The mapping-quality class.
    MappingQuality,
    /// The round-trip capability class.
    RoundTripState,
    /// The runtime origin.
    RuntimeOrigin,
    /// The preview-runtime freshness class.
    PreviewFreshness,
    /// The source-sync class.
    SourceSyncState,
    /// The derived effective claim after narrowing.
    EffectiveClaim,
    /// The narrowed-capability reason.
    NarrowedReason,
}

impl M5VisualDesignCertExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SelectionIdentity,
        Self::MappingQuality,
        Self::RoundTripState,
        Self::RuntimeOrigin,
        Self::PreviewFreshness,
        Self::SourceSyncState,
        Self::EffectiveClaim,
        Self::NarrowedReason,
    ];

    /// The export fields every certified row MUST preserve so support / release
    /// exports carry the same mapping / runtime truth visible in-product.
    pub const MANDATORY: [Self; 7] = [
        Self::SelectionIdentity,
        Self::MappingQuality,
        Self::RoundTripState,
        Self::RuntimeOrigin,
        Self::PreviewFreshness,
        Self::EffectiveClaim,
        Self::NarrowedReason,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionIdentity => "selection_identity",
            Self::MappingQuality => "mapping_quality",
            Self::RoundTripState => "round_trip_state",
            Self::RuntimeOrigin => "runtime_origin",
            Self::PreviewFreshness => "preview_freshness",
            Self::SourceSyncState => "source_sync_state",
            Self::EffectiveClaim => "effective_claim",
            Self::NarrowedReason => "narrowed_reason",
        }
    }
}

/// An honest auto-narrowing block. When a surface narrows its claim because a truth
/// dimension weakened, it names the dimension, the frozen downgrade trigger, and a
/// precise reason, and preserves the source-backed truth rather than dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimAutoNarrow {
    /// The frozen downgrade trigger (reused vocabulary) that caused the narrowing.
    pub trigger: M5VisualDesignerDowngradeTrigger,
    /// The truth dimension that weakened.
    pub weakened_dimension: M5VisualDesignTruthDimension,
    /// The claim the surface declared when its truth was healthy.
    pub narrowed_from: M5VisualDesignClaimTier,
    /// The effective claim after narrowing.
    pub narrowed_to: M5VisualDesignClaimTier,
    /// A precise, non-generic reason label safe to render.
    pub reason_label: String,
    /// The source-backed truth is preserved rather than dropped; must hold.
    pub preserves_source_truth: bool,
}

impl ClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it actually narrows, preserves
    /// source-backed truth, and carries a precise, non-generic reason.
    pub fn is_honest(&self) -> bool {
        self.narrowed_to.capability_rank() < self.narrowed_from.capability_rank()
            && self.preserves_source_truth
            && !label_is_generic(&self.reason_label)
    }
}

/// Derived certification status for a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceCertStatus {
    /// The observed truth supports the declared claim on every dimension (green).
    Certified,
    /// A truth dimension weakened and the claim narrowed and disclosed (yellow).
    NarrowedDisclosed,
    /// The claim hides drift or the export drops truth (red) — may not ship.
    Blocked,
}

impl SurfaceCertStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Blocked => "blocked",
        }
    }
}

/// Certification row for one claimed visual-design surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignSurfaceCertRow {
    /// Record kind; must equal [`VISUAL_DESIGNER_SURFACE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed surface this row certifies.
    pub claimed_surface: M5VisualDesignClaimedSurface,
    /// The claim the surface declares when its underlying truth is healthy.
    pub declared_claim: M5VisualDesignClaimTier,
    /// The effective claim the surface presents after auto-narrowing.
    pub effective_claim: M5VisualDesignClaimTier,
    /// Observed mapping quality (reused frozen vocabulary).
    pub mapping_quality: M5BreakpointMappingQuality,
    /// Observed round-trip capability (reused frozen vocabulary).
    pub round_trip_support: RoundTripCapabilityClass,
    /// Observed preview-runtime freshness (reused frozen vocabulary).
    pub preview_runtime_freshness: PreviewFreshnessClass,
    /// Observed runtime origin (reused frozen vocabulary).
    pub runtime_origin: M5PreviewRuntimeOrigin,
    /// Observed source-sync state (reused frozen vocabulary).
    pub source_sync: SourceSyncClass,
    /// The honest auto-narrow block, present only when the surface narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrow: Option<ClaimAutoNarrow>,
    /// The copy / export parity of the surface's support / release export.
    pub copy_export: CopyExportParity,
    /// The named export fields the support / release packet preserves.
    #[serde(default)]
    pub preserved_export_fields: Vec<M5VisualDesignCertExportField>,
    /// The required labels the surface preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualDesignerRequiredLabel>,
    /// Semantic consumer surfaces this claim is surfaced in.
    #[serde(default)]
    pub consumer_surfaces: Vec<M5VisualDesignerConsumerSurface>,
    /// The release bundle this row cites for its truth (AC2); must equal the
    /// packet-level bundle ref.
    pub certification_bundle_ref: String,
    /// Ref to the frozen matrix schema this row certifies against.
    pub source_family_schema_ref: String,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualDesignSurfaceCertRow {
    /// The maximum claim tier the mapping-quality dimension supports.
    pub const fn mapping_ceiling(&self) -> M5VisualDesignClaimTier {
        mapping_supported_ceiling(self.mapping_quality)
    }

    /// The maximum claim tier the round-trip dimension supports.
    pub const fn round_trip_ceiling(&self) -> M5VisualDesignClaimTier {
        round_trip_supported_ceiling(self.round_trip_support)
    }

    /// The maximum claim tier the preview-runtime freshness dimension supports.
    pub const fn freshness_ceiling(&self) -> M5VisualDesignClaimTier {
        freshness_supported_ceiling(self.preview_runtime_freshness)
    }

    /// The weakest supported ceiling across the three truth dimensions.
    pub fn overall_supported_ceiling(&self) -> M5VisualDesignClaimTier {
        self.mapping_ceiling()
            .weaker_of(self.round_trip_ceiling())
            .weaker_of(self.freshness_ceiling())
    }

    /// The effective claim the surface *should* present: the declared claim
    /// narrowed to the weakest supported ceiling.
    pub fn expected_effective_claim(&self) -> M5VisualDesignClaimTier {
        self.declared_claim
            .weaker_of(self.overall_supported_ceiling())
    }

    /// The truth dimension whose ceiling binds the effective claim (the weakest).
    /// Returns [`M5VisualDesignTruthDimension::ExportParity`] only when no truth
    /// dimension narrowed (all healthy).
    pub fn binding_dimension(&self) -> M5VisualDesignTruthDimension {
        let ceiling = self.overall_supported_ceiling();
        if self.mapping_ceiling() == ceiling
            && self.mapping_ceiling().capability_rank()
                < M5VisualDesignClaimTier::FullyInteractiveWritable.capability_rank()
        {
            M5VisualDesignTruthDimension::MappingQuality
        } else if self.freshness_ceiling() == ceiling
            && self.freshness_ceiling().capability_rank()
                < M5VisualDesignClaimTier::FullyInteractiveWritable.capability_rank()
        {
            M5VisualDesignTruthDimension::PreviewRuntimeFreshness
        } else if self.round_trip_ceiling() == ceiling
            && self.round_trip_ceiling().capability_rank()
                < M5VisualDesignClaimTier::FullyInteractiveWritable.capability_rank()
        {
            M5VisualDesignTruthDimension::RoundTripSupport
        } else {
            M5VisualDesignTruthDimension::ExportParity
        }
    }

    /// True when the surface holds a claim above what its truth supports (drift is
    /// hidden).
    pub fn hides_drift(&self) -> bool {
        self.effective_claim.capability_rank() > self.expected_effective_claim().capability_rank()
    }

    /// True when the surface narrowed further than its truth requires.
    pub fn over_narrowed(&self) -> bool {
        self.effective_claim.capability_rank() < self.expected_effective_claim().capability_rank()
    }

    /// The certification state of one truth dimension relative to the declared and
    /// effective claim.
    fn axis_state(&self, ceiling: M5VisualDesignClaimTier) -> AxisCertificationState {
        if self.declared_claim.capability_rank() <= ceiling.capability_rank() {
            AxisCertificationState::Certified
        } else if self.effective_claim.capability_rank() <= ceiling.capability_rank() {
            AxisCertificationState::DisclosedNarrowed
        } else {
            AxisCertificationState::UndisclosedDrift
        }
    }

    /// The mapping-quality axis state.
    pub fn mapping_axis(&self) -> AxisCertificationState {
        self.axis_state(self.mapping_ceiling())
    }

    /// The round-trip axis state.
    pub fn round_trip_axis(&self) -> AxisCertificationState {
        self.axis_state(self.round_trip_ceiling())
    }

    /// The preview-runtime freshness axis state.
    pub fn freshness_axis(&self) -> AxisCertificationState {
        self.axis_state(self.freshness_ceiling())
    }

    /// The always-on export-parity axis state: the export preserves the mandatory
    /// truth fields as text / JSON / Markdown, never a screenshot alone.
    pub fn export_axis(&self) -> AxisCertificationState {
        if self.export_preserves_truth() {
            AxisCertificationState::Certified
        } else {
            AxisCertificationState::UndisclosedDrift
        }
    }

    /// Whether the export preserves the mandatory mapping / runtime truth fields.
    pub fn export_preserves_truth(&self) -> bool {
        self.copy_export.is_complete()
            && M5VisualDesignCertExportField::MANDATORY
                .iter()
                .all(|field| self.preserved_export_fields.contains(field))
    }

    /// Whether any truth dimension carries a disclosed reduction (yellow).
    pub fn is_narrowed(&self) -> bool {
        self.mapping_axis().is_disclosed_reduction()
            || self.round_trip_axis().is_disclosed_reduction()
            || self.freshness_axis().is_disclosed_reduction()
    }

    /// AC1: a stale / partial / unsupported surface can no longer present as fully
    /// writable or fully mapped — the effective claim never exceeds what the truth
    /// supports.
    pub fn claim_tracks_truth(&self) -> bool {
        !self.hides_drift() && !self.over_narrowed()
    }

    /// AC3: when the surface narrowed, it discloses an honest auto-narrow whose
    /// dimension matches the binding dimension; when it did not narrow, it carries
    /// no spurious auto-narrow.
    pub fn narrowing_disclosed(&self) -> bool {
        let narrowed =
            self.effective_claim.capability_rank() < self.declared_claim.capability_rank();
        match (&self.auto_narrow, narrowed) {
            (Some(narrow), true) => {
                narrow.is_honest()
                    && narrow.narrowed_from == self.declared_claim
                    && narrow.narrowed_to == self.effective_claim
                    && narrow.weakened_dimension == self.binding_dimension()
            }
            (Some(_), false) => false,
            (None, true) => false,
            (None, false) => true,
        }
    }

    /// Derived certification status.
    pub fn status(&self) -> SurfaceCertStatus {
        if self.hides_drift()
            || !self.export_preserves_truth()
            || self.over_narrowed()
            || !self.narrowing_disclosed()
        {
            return SurfaceCertStatus::Blocked;
        }
        if self.effective_claim.capability_rank() < self.declared_claim.capability_rank() {
            SurfaceCertStatus::NarrowedDisclosed
        } else {
            SurfaceCertStatus::Certified
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_DESIGNER_SURFACE_CERT_ROW_RECORD_KIND
            && self.schema_version == VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.certification_bundle_ref.trim().is_empty()
            && !self.preserved_export_fields.is_empty()
            && !self.required_labels.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} declared={declared} effective={effective} \
mapping={mapping} round_trip={round_trip} freshness={freshness} status={status}",
            surface = self.claimed_surface.as_str(),
            declared = self.declared_claim.as_str(),
            effective = self.effective_claim.as_str(),
            mapping = self.mapping_axis().as_str(),
            round_trip = self.round_trip_axis().as_str(),
            freshness = self.freshness_axis().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-810 surface-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignSurfaceCertSummary {
    pub surface_count: usize,
    pub all_claims_track_truth: bool,
    pub all_exports_preserve_truth: bool,
    pub all_narrowing_disclosed: bool,
    pub evidence_surfaces_present: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`VisualDesignSurfaceCertPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualDesignSurfaceCertPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    pub rows: Vec<VisualDesignSurfaceCertRow>,
}

/// Checked-in M05-810 surface-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignSurfaceCertPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualDesignSurfaceCertRow>,
    pub summary: VisualDesignSurfaceCertSummary,
}

impl VisualDesignSurfaceCertPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: VisualDesignSurfaceCertPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION,
            record_kind: VISUAL_DESIGNER_SURFACE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            certification_bundle_ref: input.certification_bundle_ref,
            rows: input.rows,
            summary: VisualDesignSurfaceCertSummary {
                surface_count: 0,
                all_claims_track_truth: false,
                all_exports_preserve_truth: false,
                all_narrowing_disclosed: false,
                evidence_surfaces_present: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5VisualDesignClaimedSurface> {
        self.rows.iter().map(|r| r.claimed_surface).collect()
    }

    /// Whether the docs / help, support, and release evidence surfaces are all
    /// certified.
    pub fn evidence_surfaces_present(&self) -> bool {
        let represented = self.represented_surfaces();
        M5VisualDesignClaimedSurface::EVIDENCE_SURFACES
            .iter()
            .all(|s| represented.contains(s))
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualDesignSurfaceCertSummary {
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                SurfaceCertStatus::Certified => green += 1,
                SurfaceCertStatus::NarrowedDisclosed => yellow += 1,
                SurfaceCertStatus::Blocked => red += 1,
            }
        }

        VisualDesignSurfaceCertSummary {
            surface_count: self.rows.len(),
            all_claims_track_truth: self
                .rows
                .iter()
                .all(VisualDesignSurfaceCertRow::claim_tracks_truth),
            all_exports_preserve_truth: self
                .rows
                .iter()
                .all(VisualDesignSurfaceCertRow::export_preserves_truth),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(VisualDesignSurfaceCertRow::narrowing_disclosed),
            evidence_surfaces_present: self.evidence_surfaces_present(),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualDesignSurfaceCertViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION {
            violations.push(VisualDesignSurfaceCertViolation::SchemaVersion {
                expected: VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_DESIGNER_SURFACE_CERT_RECORD_KIND {
            violations.push(VisualDesignSurfaceCertViolation::RecordKind {
                expected: VISUAL_DESIGNER_SURFACE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
            || self.certification_bundle_ref.trim().is_empty()
        {
            violations.push(VisualDesignSurfaceCertViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        let mut label_union = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualDesignSurfaceCertViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_surfaces.insert(row.claimed_surface);
            label_union.extend(row.required_labels.iter().copied());

            if !row.is_complete() {
                violations.push(VisualDesignSurfaceCertViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the claim never exceeds what the truth supports.
            if row.hides_drift() {
                violations.push(VisualDesignSurfaceCertViolation::ClaimHidesDrift {
                    id: row.row_id.clone(),
                });
            }
            if row.over_narrowed() {
                violations.push(VisualDesignSurfaceCertViolation::OverNarrowedClaim {
                    id: row.row_id.clone(),
                });
            }

            // AC2: the export preserves the same mapping / runtime truth.
            if !row.export_preserves_truth() {
                violations.push(VisualDesignSurfaceCertViolation::ExportDropsTruth {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed and auto-narrow honest.
            if !row.narrowing_disclosed() {
                violations.push(VisualDesignSurfaceCertViolation::NarrowingUndisclosed {
                    id: row.row_id.clone(),
                });
            }

            // Each row cites the one certification bundle.
            if row.certification_bundle_ref != self.certification_bundle_ref {
                violations.push(VisualDesignSurfaceCertViolation::BundleRefMismatch {
                    id: row.row_id.clone(),
                });
            }

            // Required labels stay within the frozen vocabulary.
            if row.required_labels.is_empty() {
                violations.push(VisualDesignSurfaceCertViolation::MissingRequiredLabels {
                    id: row.row_id.clone(),
                });
            }

            // Consumer parity: at least two consumer surfaces surface the claim.
            if row.consumer_surfaces.len() < 2 {
                violations.push(VisualDesignSurfaceCertViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No blocked (red) rows may ship.
            if row.status() == SurfaceCertStatus::Blocked {
                violations.push(VisualDesignSurfaceCertViolation::BlockedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed surface is certified at least once.
        for surface in M5VisualDesignClaimedSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations
                    .push(VisualDesignSurfaceCertViolation::MissingSurfaceCoverage { surface });
            }
        }

        // AC3: the docs / help, support, and release evidence surfaces are present.
        if !self.evidence_surfaces_present() {
            violations.push(VisualDesignSurfaceCertViolation::MissingEvidenceSurface);
        }

        // The union of preserved required labels covers the frozen set.
        for label in M5VisualDesignerRequiredLabel::ALL {
            if !label_union.contains(&label) {
                violations.push(VisualDesignSurfaceCertViolation::MissingLabelCoverage { label });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualDesignSurfaceCertViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("surface certification packet serializes"),
        ) {
            violations.push(VisualDesignSurfaceCertViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("surface certification packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,claimed_surface,declared_claim,effective_claim,mapping_axis,round_trip_axis,freshness_axis,export_axis,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{surface},{declared},{effective},{mapping},{round_trip},{freshness},{export},{status}\n",
                id = row.row_id,
                surface = row.claimed_surface.as_str(),
                declared = row.declared_claim.as_str(),
                effective = row.effective_claim.as_str(),
                mapping = row.mapping_axis().as_str(),
                round_trip = row.round_trip_axis().as_str(),
                freshness = row.freshness_axis().as_str(),
                export = row.export_axis().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Designer Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Bundle: `{}`\n", self.certification_bundle_ref));
        out.push_str(&format!(
            "- Surfaces: {} certified across {} / {} claimed surfaces\n",
            self.summary.surface_count,
            self.represented_surfaces().len(),
            M5VisualDesignClaimedSurface::ALL.len(),
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
                row.claimed_surface.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.auto_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: dimension={} trigger={} — {}\n",
                    narrow.weakened_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.reason_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in surface-certification export.
pub fn current_m5_visual_designer_surface_certification_export(
) -> Result<VisualDesignSurfaceCertPacket, VisualDesignSurfaceCertArtifactError> {
    let packet: VisualDesignSurfaceCertPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json"
    )))
    .map_err(VisualDesignSurfaceCertArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualDesignSurfaceCertArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in surface-certification export.
#[derive(Debug)]
pub enum VisualDesignSurfaceCertArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualDesignSurfaceCertViolation>),
}

impl fmt::Display for VisualDesignSurfaceCertArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "surface certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "surface certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualDesignSurfaceCertArtifactError {}

/// Validation failure for M05-810 surface-certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualDesignSurfaceCertViolation {
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
    ClaimHidesDrift {
        id: String,
    },
    OverNarrowedClaim {
        id: String,
    },
    ExportDropsTruth {
        id: String,
    },
    NarrowingUndisclosed {
        id: String,
    },
    BundleRefMismatch {
        id: String,
    },
    MissingRequiredLabels {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    BlockedRow {
        id: String,
    },
    MissingSurfaceCoverage {
        surface: M5VisualDesignClaimedSurface,
    },
    MissingEvidenceSurface,
    MissingLabelCoverage {
        label: M5VisualDesignerRequiredLabel,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for VisualDesignSurfaceCertViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::ClaimHidesDrift { id } => {
                write!(
                    f,
                    "row {id} claims a capability above what its mapping / round-trip / runtime truth supports"
                )
            }
            Self::OverNarrowedClaim { id } => {
                write!(f, "row {id} narrows further than its truth requires")
            }
            Self::ExportDropsTruth { id } => {
                write!(
                    f,
                    "row {id} export does not preserve the mandatory mapping / runtime truth fields"
                )
            }
            Self::NarrowingUndisclosed { id } => {
                write!(
                    f,
                    "row {id} narrows without an honest, matching auto-narrow disclosure"
                )
            }
            Self::BundleRefMismatch { id } => {
                write!(
                    f,
                    "row {id} cites a certification bundle other than the packet bundle"
                )
            }
            Self::MissingRequiredLabels { id } => {
                write!(f, "row {id} preserves no required labels")
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::BlockedRow { id } => write!(f, "row {id} is blocked (red) and may not ship"),
            Self::MissingSurfaceCoverage { surface } => {
                write!(
                    f,
                    "claimed surface {surface:?} is not certified in the packet"
                )
            }
            Self::MissingEvidenceSurface => {
                write!(
                    f,
                    "docs/help, support, or release evidence surface is not certified"
                )
            }
            Self::MissingLabelCoverage { label } => {
                write!(f, "required label {label:?} is not preserved by any row")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for VisualDesignSurfaceCertViolation {}

/// Whether a narrowed reason label is a generic non-answer rather than a precise
/// label.
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
            | "stale"
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

/// Builds the canonical, checked-in surface-certification packet. This is the one
/// source of truth shared by the tests, the example dump, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_visual_designer_surface_certification_packet() -> VisualDesignSurfaceCertPacket {
    VisualDesignSurfaceCertPacket::new(VisualDesignSurfaceCertPacketInput {
        packet_id: "m5-visual-designer-surface-certification:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: VISUAL_DESIGNER_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: VISUAL_DESIGNER_SURFACE_CERT_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-designer-surface-cert:{id}")]
}

fn all_required_labels() -> Vec<M5VisualDesignerRequiredLabel> {
    M5VisualDesignerRequiredLabel::ALL.to_vec()
}

fn full_export_fields() -> Vec<M5VisualDesignCertExportField> {
    M5VisualDesignCertExportField::ALL.to_vec()
}

fn full_copy_export() -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: M5VisualDesignCertExportField::ALL
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        screenshot_only_prohibited: true,
    }
}

/// A green (certified) row whose observed truth still supports its declared claim.
#[allow(clippy::too_many_arguments)]
fn green_row(
    row_id: &str,
    surface: M5VisualDesignClaimedSurface,
    declared: M5VisualDesignClaimTier,
    mapping: M5BreakpointMappingQuality,
    round_trip: RoundTripCapabilityClass,
    freshness: PreviewFreshnessClass,
    origin: M5PreviewRuntimeOrigin,
    sync: SourceSyncClass,
    consumers: Vec<M5VisualDesignerConsumerSurface>,
    ev_id: &str,
) -> VisualDesignSurfaceCertRow {
    let mut row = VisualDesignSurfaceCertRow {
        record_kind: VISUAL_DESIGNER_SURFACE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        claimed_surface: surface,
        declared_claim: declared,
        effective_claim: declared,
        mapping_quality: mapping,
        round_trip_support: round_trip,
        preview_runtime_freshness: freshness,
        runtime_origin: origin,
        source_sync: sync,
        auto_narrow: None,
        copy_export: full_copy_export(),
        preserved_export_fields: full_export_fields(),
        required_labels: all_required_labels(),
        consumer_surfaces: consumers,
        certification_bundle_ref: VISUAL_DESIGNER_SURFACE_CERT_BUNDLE_REF.to_owned(),
        source_family_schema_ref: VISUAL_DESIGNER_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        source_refs: vec![VISUAL_DESIGNER_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(ev_id),
    };
    // The declared claim must already sit within the supported ceiling for a green
    // row; pin the effective claim to the expected value.
    row.effective_claim = row.expected_effective_claim();
    row
}

/// A yellow (narrowed / disclosed) row whose observed truth weakened, narrowing
/// its claim with an honest disclosure.
#[allow(clippy::too_many_arguments)]
fn yellow_row(
    row_id: &str,
    surface: M5VisualDesignClaimedSurface,
    declared: M5VisualDesignClaimTier,
    mapping: M5BreakpointMappingQuality,
    round_trip: RoundTripCapabilityClass,
    freshness: PreviewFreshnessClass,
    origin: M5PreviewRuntimeOrigin,
    sync: SourceSyncClass,
    consumers: Vec<M5VisualDesignerConsumerSurface>,
    trigger: M5VisualDesignerDowngradeTrigger,
    reason: &str,
    ev_id: &str,
) -> VisualDesignSurfaceCertRow {
    let mut row = green_row(
        row_id, surface, declared, mapping, round_trip, freshness, origin, sync, consumers, ev_id,
    );
    let effective = row.expected_effective_claim();
    row.effective_claim = effective;
    row.auto_narrow = Some(ClaimAutoNarrow {
        trigger,
        weakened_dimension: row.binding_dimension(),
        narrowed_from: declared,
        narrowed_to: effective,
        reason_label: reason.to_owned(),
        preserves_source_truth: true,
    });
    row
}

fn seeded_rows() -> Vec<VisualDesignSurfaceCertRow> {
    use M5VisualDesignClaimTier::*;
    use M5VisualDesignClaimedSurface as S;
    use M5VisualDesignerConsumerSurface as C;

    vec![
        // Design-canvas workspace — exact mapping, exact round-trip, fresh runtime;
        // the full interactive claim holds (green).
        green_row(
            "cert:design-canvas-workspace",
            S::DesignCanvasWorkspace,
            FullyInteractiveWritable,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::InSyncFromSource,
            vec![C::DesignCanvasWorkspace, C::StructurePanel],
            "design-canvas-workspace",
        ),
        // Structure-tree panel — inspect-only navigation over an exact mapping; the
        // conservative claim already matches the round-trip truth (green).
        green_row(
            "cert:structure-tree-panel",
            S::StructureTreePanel,
            InspectOnly,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::InspectOnlyNoWrite,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::InSyncFromSource,
            vec![C::StructurePanel, C::DocsHelp],
            "structure-tree-panel",
        ),
        // Property-inspector panel — exact mapping and exact round-trip keep the
        // writable claim (green).
        green_row(
            "cert:property-inspector-panel",
            S::PropertyInspectorPanel,
            FullyInteractiveWritable,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::InSyncFromSource,
            vec![C::PropertyPanel, C::SupportExport],
            "property-inspector-panel",
        ),
        // Source round-trip rail — approximate mapping narrows a writable claim to
        // inspect-only, disclosed (yellow).
        yellow_row(
            "cert:source-round-trip-rail",
            S::SourceRoundTripRail,
            FullyInteractiveWritable,
            M5BreakpointMappingQuality::Approximate,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::PendingRebuild,
            vec![C::SourceSyncRail, C::ReleaseProof],
            M5VisualDesignerDowngradeTrigger::UnmappedSource,
            "Source mapping resolved only approximately; the rail narrows to inspect-only and keeps the source-first diff before any write-back",
            "source-round-trip-rail",
        ),
        // Breakpoint / device-preview deck — a stale runtime narrows the interactive
        // claim to read-only, disclosed (yellow).
        yellow_row(
            "cert:breakpoint-device-preview-deck",
            S::BreakpointDevicePreviewDeck,
            FullyInteractiveWritable,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::InspectOnlyNoWrite,
            PreviewFreshnessClass::Stale,
            M5PreviewRuntimeOrigin::CapturedSnapshot,
            SourceSyncClass::DriftedFromSource,
            vec![C::PreviewDeck, C::SupportExport],
            M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
            "Preview runtime is past its freshness SLO; the deck narrows to a read-only captured view and keeps the runtime origin and mapping quality visible",
            "breakpoint-device-preview-deck",
        ),
        // Framework-pack preview — an aging runtime narrows the writable claim to
        // inspect-only, disclosed (yellow).
        yellow_row(
            "cert:framework-pack-preview",
            S::FrameworkPackPreview,
            FullyInteractiveWritable,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewFreshnessClass::Aging,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::PendingRebuild,
            vec![C::PreviewDeck, C::DocsHelp],
            M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
            "Framework-pack runtime is aging toward its freshness SLO; the preview narrows to inspect-only until the runtime refreshes",
            "framework-pack-preview",
        ),
        // Browser-runtime inspection — inspect-only over an approximate mapping and
        // no write-back; the conservative claim already matches the truth (green).
        green_row(
            "cert:browser-runtime-inspection",
            S::BrowserRuntimeInspection,
            InspectOnly,
            M5BreakpointMappingQuality::Approximate,
            RoundTripCapabilityClass::InspectOnlyNoWrite,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LiveDevRuntime,
            SourceSyncClass::RuntimeOnlyNoSource,
            vec![C::SourceSyncRail, C::SupportExport],
            "browser-runtime-inspection",
        ),
        // Docs / help embeds — a read-only embed over an inspect-only round-trip; the
        // read-only claim holds (green).
        green_row(
            "cert:docs-help-embeds",
            S::DocsHelpEmbeds,
            ReadOnly,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::InspectOnlyNoWrite,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LocalMockRuntime,
            SourceSyncClass::InSyncFromSource,
            vec![C::DocsHelp, C::ReleaseProof],
            "docs-help-embeds",
        ),
        // Support export — source-only fallback with a stale captured runtime; the
        // source-only claim already matches the weakest truth (green).
        green_row(
            "cert:support-export",
            S::SupportExport,
            SourceOnly,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::SourceOnlyFallback,
            PreviewFreshnessClass::Stale,
            M5PreviewRuntimeOrigin::CapturedSnapshot,
            SourceSyncClass::DriftedFromSource,
            vec![C::SupportExport, C::DocsHelp],
            "support-export",
        ),
        // Release proof — a read-only evidence surface over an inspect-only
        // round-trip; the read-only claim holds (green).
        green_row(
            "cert:release-proof",
            S::ReleaseProof,
            ReadOnly,
            M5BreakpointMappingQuality::Exact,
            RoundTripCapabilityClass::InspectOnlyNoWrite,
            PreviewFreshnessClass::Fresh,
            M5PreviewRuntimeOrigin::LocalMockRuntime,
            SourceSyncClass::InSyncFromSource,
            vec![C::ReleaseProof, C::SupportExport],
            "release-proof",
        ),
    ]
}
