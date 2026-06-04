//! Qualification packet for preview, designer, and publish-capable surfaces.
//!
//! This module owns the release artifact that keeps preview runtimes, visual
//! designer canvases, share/export sheets, and publish/deploy helpers
//! source-first. A row can render at Stable only when it has current proof,
//! canonical source mappings, visible generated/source lineage, safe-preview
//! defaults, preview/apply/revert or dry-run lineage for side effects, and an
//! explicit boundary that does not imply browser-runtime inspection depth.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{OwnerSignoff, StableClaimLevel};

/// Supported schema version for the checked-in qualification packet.
pub const PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "preview_designer_publish_surface_qualification";

/// Repo-relative path to the checked-in packet.
pub const PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_PATH: &str =
    "artifacts/release/m4/preview-designer-publish-surface-qualification.json";

/// Embedded checked-in packet JSON.
pub const PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/preview-designer-publish-surface-qualification.json"
));

/// Promoted surface family covered by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewDesignerPublishSurfaceKind {
    /// Live or safe preview runtime surface.
    PreviewRuntime,
    /// Device or viewport preview row.
    DeviceViewportPreview,
    /// Visual designer canvas or property inspector.
    DesignerCanvas,
    /// Share or export sheet attached to a preview or designer result.
    ShareExportSheet,
    /// Publish or deploy review surface.
    PublishDeployPreview,
}

impl PreviewDesignerPublishSurfaceKind {
    /// True when the surface can create external side effects.
    pub const fn can_have_external_effects(self) -> bool {
        matches!(self, Self::PublishDeployPreview | Self::ShareExportSheet)
    }
}

/// Mapping fidelity between a surface projection and authored source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMappingQuality {
    /// Every editable element maps to canonical source.
    CanonicalSourceMapping,
    /// Mapping is partial and must be visibly narrowed.
    ApproximateMapping,
    /// The construct is unsupported for round trip.
    UnsupportedConstruct,
    /// The surface is generated only and cannot write source directly.
    GeneratedOnly,
    /// The surface is a captured snapshot with no editable source mapping.
    SnapshotOnly,
}

impl SourceMappingQuality {
    /// True when this mapping can back a Stable writable surface.
    pub const fn is_canonical(self) -> bool {
        matches!(self, Self::CanonicalSourceMapping)
    }
}

/// Source synchronization state shown on the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceSyncState {
    /// Source and projection are in sync.
    InSync,
    /// Source is newer than the displayed projection.
    SourceNewer,
    /// Projection is newer than the source and requires review.
    ProjectionNewerReviewRequired,
    /// Drift is detected and the row must narrow.
    DriftDetected,
    /// Source cannot be found.
    SourceMissing,
    /// Sync is blocked by policy or missing tooling.
    SyncBlocked,
}

impl SourceSyncState {
    /// True when the surface is synchronized enough to render Stable.
    pub const fn is_stable_ready(self) -> bool {
        matches!(self, Self::InSync)
    }
}

/// Truth class for what a user is viewing or exporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedSourceTruth {
    /// Canonical authored source is being shown.
    CanonicalSource,
    /// Live runtime output is being shown.
    LiveRuntimeOutput,
    /// Mocked or fixture-backed output is being shown.
    MockOutput,
    /// Imported output or route data is being shown.
    ImportedOutput,
    /// Cached output is being shown.
    CachedOutput,
    /// Generated projection derived from source is being shown.
    GeneratedProjection,
    /// Captured snapshot evidence is being shown.
    PreviewSnapshot,
}

impl GeneratedSourceTruth {
    /// True when the truth class requires visible non-source labeling.
    pub const fn requires_visible_lineage(self) -> bool {
        !matches!(self, Self::CanonicalSource)
    }
}

/// Safe-preview posture available before trust or side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewPosture {
    /// Safe preview is the default posture.
    SafePreviewDefault,
    /// Preview is available only after review.
    ReviewRequiredPreview,
    /// Dry run is available for side effects.
    DryRunAvailable,
    /// Closest safe preview is available when full dry run is impossible.
    ClosestSafePreview,
    /// Inspect-only mode, no apply.
    InspectOnly,
    /// No safe preview is available.
    NoSafePreview,
}

impl SafePreviewPosture {
    /// True when a safe preview or dry-run posture exists.
    pub const fn is_safe(self) -> bool {
        !matches!(self, Self::NoSafePreview)
    }
}

/// Preview/apply/revert posture proven for a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSafetyLineage {
    /// Action is inspect-only.
    InspectOnly,
    /// Diff preview is available before apply.
    PreviewDiff,
    /// Dry run is available before mutation or external effects.
    DryRun,
    /// Apply writes only after preview or review.
    ApplyAfterReview,
    /// Revert or rollback lineage is exported.
    RevertRollbackExportable,
    /// No side-effectful action is present.
    NoSideEffect,
}

/// Browser-runtime inspection boundary recorded for the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserInspectionBoundary {
    /// Preview only; browser-runtime inspection is not claimed.
    PreviewOnlyNoInspectionClaim,
    /// Shared runtime identity is cited but DOM/CSS/console/network/storage are out of scope.
    RuntimeIdentitySharedInspectionGovernedElsewhere,
    /// External browser capture only.
    ExternalBrowserCaptureOnly,
    /// Browser inspection is explicitly blocked or not part of this row.
    InspectionBlockedOrSeparate,
}

/// Exported artifact truth disclosed by share/export/publish rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportedArtifactTruth {
    /// No export artifact is produced.
    NotExporting,
    /// Export is canonical source.
    CanonicalSourceExport,
    /// Export is a preview snapshot.
    PreviewSnapshotExport,
    /// Export is a generated projection.
    GeneratedProjectionExport,
    /// Export is an external browser capture.
    ExternalBrowserCaptureExport,
    /// Export is a publish or deploy dry-run packet.
    PublishDryRunPacket,
}

/// Publication and support destinations that must ingest the row label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationProjection {
    /// Docs and Help ingest the qualification state.
    pub docs_help: bool,
    /// About and release packets ingest the qualification state.
    pub about_release_packets: bool,
    /// Support export carries qualification, lineage, and fallback fields.
    pub support_export: bool,
    /// Share/export/publish sheets disclose generated/source truth.
    pub share_export_publish_sheets: bool,
    /// Product surface labels render the displayed lifecycle label.
    pub product_surface_labels: bool,
}

impl QualificationProjection {
    fn complete(&self) -> bool {
        self.docs_help
            && self.about_release_packets
            && self.support_export
            && self.share_export_publish_sheets
            && self.product_surface_labels
    }
}

/// Fallback paths available when a surface cannot round-trip safely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FallbackPaths {
    /// Opens canonical source.
    pub open_source: bool,
    /// Opens source diff or review sheet.
    pub open_diff: bool,
    /// Shows raw or source-only fallback.
    pub raw_source_fallback: bool,
    /// Exports rollback or recovery lineage.
    pub rollback_lineage_export: bool,
}

impl FallbackPaths {
    fn complete_for_stable(&self) -> bool {
        self.open_source && self.open_diff && self.rollback_lineage_export
    }
}

/// One preview/designer/publish qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewDesignerPublishSurfaceRow {
    /// Stable row id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Surface family.
    pub surface_kind: PreviewDesignerPublishSurfaceKind,
    /// Whether the promoted build exposes this surface.
    pub promoted_build_surface: bool,
    /// Claimed lifecycle label before family qualification.
    pub claim_label: StableClaimLevel,
    /// Label rendered after qualification or narrowing.
    pub displayed_label: StableClaimLevel,
    /// Stable proof packet, absent for narrowed rows.
    #[serde(default)]
    pub qualification_packet: Option<ProofPacket>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Source mapping quality.
    pub source_mapping_quality: SourceMappingQuality,
    /// Source-sync state.
    pub source_sync_state: SourceSyncState,
    /// Generated-versus-source truth class.
    pub generated_source_truth: GeneratedSourceTruth,
    /// Safe-preview posture.
    pub safe_preview_posture: SafePreviewPosture,
    /// Action safety lineage available on the row.
    #[serde(default)]
    pub action_safety_lineage: Vec<ActionSafetyLineage>,
    /// Browser-runtime inspection boundary.
    pub browser_inspection_boundary: BrowserInspectionBoundary,
    /// Export artifact truth.
    pub exported_artifact_truth: ExportedArtifactTruth,
    /// Visible lineage labels or chips.
    #[serde(default)]
    pub visible_lineage_labels: Vec<String>,
    /// Unsupported construct cards shown on the row.
    #[serde(default)]
    pub unsupported_construct_cards: Vec<String>,
    /// Fallback paths.
    pub fallback_paths: FallbackPaths,
    /// Publication/support projections.
    pub projection: QualificationProjection,
    /// Evidence refs for mapping, safe preview, accessibility, or drills.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewable reason this row carries its posture.
    pub rationale: String,
}

impl PreviewDesignerPublishSurfaceRow {
    /// True when this row renders at or above the Stable cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the row carries a captured, current proof packet.
    pub fn has_green_packet(&self) -> bool {
        self.qualification_packet.as_ref().is_some_and(|packet| {
            packet.has_capture() && packet.slo_state == FreshnessSloState::Current
        })
    }

    fn has_visible_generated_source_truth(&self) -> bool {
        !self.generated_source_truth.requires_visible_lineage()
            || !self.visible_lineage_labels.is_empty()
    }

    fn has_side_effect_safety(&self) -> bool {
        if !self.surface_kind.can_have_external_effects() {
            return true;
        }
        self.action_safety_lineage
            .iter()
            .any(|lineage| matches!(lineage, ActionSafetyLineage::DryRun))
            || matches!(
                self.safe_preview_posture,
                SafePreviewPosture::DryRunAvailable | SafePreviewPosture::ClosestSafePreview
            )
    }

    fn has_preview_apply_revert_lineage(&self) -> bool {
        if !self.surface_kind.can_have_external_effects() {
            return true;
        }
        self.action_safety_lineage
            .contains(&ActionSafetyLineage::PreviewDiff)
            && self
                .action_safety_lineage
                .contains(&ActionSafetyLineage::ApplyAfterReview)
            && self
                .action_safety_lineage
                .contains(&ActionSafetyLineage::RevertRollbackExportable)
    }

    fn browser_boundary_is_honest(&self) -> bool {
        matches!(
            self.browser_inspection_boundary,
            BrowserInspectionBoundary::PreviewOnlyNoInspectionClaim
                | BrowserInspectionBoundary::RuntimeIdentitySharedInspectionGovernedElsewhere
                | BrowserInspectionBoundary::ExternalBrowserCaptureOnly
                | BrowserInspectionBoundary::InspectionBlockedOrSeparate
        )
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewDesignerPublishQualificationSummary {
    /// Total promoted-build rows.
    pub promoted_surface_count: usize,
    /// Rows rendering at Stable.
    pub stable_surface_count: usize,
    /// Rows narrowed below Stable.
    pub narrowed_surface_count: usize,
    /// Stable rows with green packets.
    pub green_packet_count: usize,
    /// Rows with non-canonical source mapping.
    pub noncanonical_mapping_count: usize,
    /// Side-effectful rows that expose dry run or closest safe preview.
    pub side_effect_rows_with_safe_preview_count: usize,
}

/// Support/export projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewDesignerPublishExportRow {
    /// Stable row id.
    pub surface_id: String,
    /// Rendered lifecycle label.
    pub displayed_label: StableClaimLevel,
    /// Source mapping quality.
    pub source_mapping_quality: SourceMappingQuality,
    /// Source-sync state.
    pub source_sync_state: SourceSyncState,
    /// Generated-versus-source truth class.
    pub generated_source_truth: GeneratedSourceTruth,
    /// Export artifact truth.
    pub exported_artifact_truth: ExportedArtifactTruth,
    /// Whether rollback lineage is exportable.
    pub rollback_lineage_exportable: bool,
}

/// Support/export projection over the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewDesignerPublishExportProjection {
    /// Projection record kind.
    pub record_kind: String,
    /// Source packet id.
    pub packet_id: String,
    /// Rows exported to Help/About and support bundles.
    pub rows: Vec<PreviewDesignerPublishExportRow>,
}

/// Canonical family qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewDesignerPublishSurfaceQualification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable release document.
    pub release_doc_ref: String,
    /// User-facing help projection.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<PreviewDesignerPublishSurfaceRow>,
    /// Summary counts.
    pub summary: PreviewDesignerPublishQualificationSummary,
}

impl PreviewDesignerPublishSurfaceQualification {
    /// Returns rows rendered at Stable.
    pub fn stable_surfaces(&self) -> Vec<&PreviewDesignerPublishSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns rows narrowed below Stable.
    pub fn narrowed_surfaces(&self) -> Vec<&PreviewDesignerPublishSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Recomputes summary counts from row state.
    pub fn computed_summary(&self) -> PreviewDesignerPublishQualificationSummary {
        let promoted: Vec<&PreviewDesignerPublishSurfaceRow> = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .collect();
        PreviewDesignerPublishQualificationSummary {
            promoted_surface_count: promoted.len(),
            stable_surface_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            narrowed_surface_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            green_packet_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable() && surface.has_green_packet())
                .count(),
            noncanonical_mapping_count: promoted
                .iter()
                .filter(|surface| !surface.source_mapping_quality.is_canonical())
                .count(),
            side_effect_rows_with_safe_preview_count: promoted
                .iter()
                .filter(|surface| {
                    surface.surface_kind.can_have_external_effects()
                        && surface.has_side_effect_safety()
                })
                .count(),
        }
    }

    /// Builds the support/export projection from canonical row state.
    pub fn support_export_projection(&self) -> PreviewDesignerPublishExportProjection {
        PreviewDesignerPublishExportProjection {
            record_kind: "preview_designer_publish_surface_qualification_support_export"
                .to_string(),
            packet_id: self.packet_id.clone(),
            rows: self
                .surfaces
                .iter()
                .map(|surface| PreviewDesignerPublishExportRow {
                    surface_id: surface.surface_id.clone(),
                    displayed_label: surface.displayed_label,
                    source_mapping_quality: surface.source_mapping_quality,
                    source_sync_state: surface.source_sync_state,
                    generated_source_truth: surface.generated_source_truth,
                    exported_artifact_truth: surface.exported_artifact_truth,
                    rollback_lineage_exportable: surface.fallback_paths.rollback_lineage_export,
                })
                .collect(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<PreviewDesignerPublishQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(
                PreviewDesignerPublishQualificationViolation::SchemaVersion {
                    expected: PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(PreviewDesignerPublishQualificationViolation::RecordKind {
                expected: PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ids = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        for surface in &self.surfaces {
            kinds.insert(surface.surface_kind);
            if !ids.insert(surface.surface_id.clone()) {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::DuplicateSurfaceId {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.displayed_label.rank() > surface.claim_label.rank() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::DisplayedWiderThanClaim {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.promoted_build_surface
                && surface.renders_stable()
                && !surface.has_green_packet()
            {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::StableSurfaceWithoutGreenPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.owner_signoff.signed_off {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::StableSurfaceMissingOwnerSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.source_mapping_quality.is_canonical() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::StableSurfaceWithoutCanonicalMapping {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.source_sync_state.is_stable_ready() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::StableSurfaceOutOfSync {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.safe_preview_posture.is_safe() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::StableSurfaceWithoutSafePreview {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.has_visible_generated_source_truth() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::MissingGeneratedSourceTruth {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.fallback_paths.complete_for_stable() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::IncompleteFallbackPaths {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.projection.complete() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::IncompleteProjection {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.surface_kind.can_have_external_effects()
                && surface.renders_stable()
                && !surface.has_side_effect_safety()
            {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::SideEffectWithoutDryRun {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.surface_kind.can_have_external_effects()
                && surface.renders_stable()
                && !surface.has_preview_apply_revert_lineage()
            {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::MissingPreviewApplyRevertLineage {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.browser_boundary_is_honest() {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::BrowserInspectionBoundaryOverclaim {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.source_mapping_quality.is_canonical()
                && surface.unsupported_construct_cards.is_empty()
            {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::MissingUnsupportedConstructCard {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        for kind in [
            PreviewDesignerPublishSurfaceKind::PreviewRuntime,
            PreviewDesignerPublishSurfaceKind::DeviceViewportPreview,
            PreviewDesignerPublishSurfaceKind::DesignerCanvas,
            PreviewDesignerPublishSurfaceKind::ShareExportSheet,
            PreviewDesignerPublishSurfaceKind::PublishDeployPreview,
        ] {
            if !kinds.contains(&kind) {
                violations.push(
                    PreviewDesignerPublishQualificationViolation::MissingSurfaceKind {
                        surface_kind: kind,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(PreviewDesignerPublishQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in preview/designer/publish qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_preview_designer_publish_surface_qualification(
) -> Result<PreviewDesignerPublishSurfaceQualification, serde_json::Error> {
    serde_json::from_str(PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_JSON)
}

/// Validation failure for the preview/designer/publish qualification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewDesignerPublishQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// Surface ids must be unique.
    DuplicateSurfaceId { surface_id: String },
    /// Displayed lifecycle label is wider than the row claim.
    DisplayedWiderThanClaim { surface_id: String },
    /// A Stable promoted surface lacks a current captured proof packet.
    StableSurfaceWithoutGreenPacket { surface_id: String },
    /// A Stable promoted surface lacks owner sign-off.
    StableSurfaceMissingOwnerSignoff { surface_id: String },
    /// A Stable row lacks canonical source mapping.
    StableSurfaceWithoutCanonicalMapping { surface_id: String },
    /// A Stable row is not source-synchronized.
    StableSurfaceOutOfSync { surface_id: String },
    /// A Stable row lacks safe preview.
    StableSurfaceWithoutSafePreview { surface_id: String },
    /// A row lacks visible generated-versus-source truth.
    MissingGeneratedSourceTruth { surface_id: String },
    /// A Stable row lacks open-source, diff, or rollback fallback.
    IncompleteFallbackPaths { surface_id: String },
    /// Docs, Help, release packet, product label, or support projection is incomplete.
    IncompleteProjection { surface_id: String },
    /// A side-effectful Stable row lacks dry run or closest safe preview.
    SideEffectWithoutDryRun { surface_id: String },
    /// A side-effectful Stable row lacks preview/apply/revert lineage.
    MissingPreviewApplyRevertLineage { surface_id: String },
    /// Browser inspection depth is overclaimed by this preview packet.
    BrowserInspectionBoundaryOverclaim { surface_id: String },
    /// Non-canonical mapping lacks an unsupported-construct card.
    MissingUnsupportedConstructCard { surface_id: String },
    /// A required surface kind is absent.
    MissingSurfaceKind {
        surface_kind: PreviewDesignerPublishSurfaceKind,
    },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for PreviewDesignerPublishQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateSurfaceId { surface_id } => write!(f, "{surface_id} is duplicated"),
            Self::DisplayedWiderThanClaim { surface_id } => {
                write!(f, "{surface_id} displays wider than its claim")
            }
            Self::StableSurfaceWithoutGreenPacket { surface_id } => {
                write!(f, "{surface_id} is Stable without a green packet")
            }
            Self::StableSurfaceMissingOwnerSignoff { surface_id } => {
                write!(f, "{surface_id} is Stable without owner sign-off")
            }
            Self::StableSurfaceWithoutCanonicalMapping { surface_id } => {
                write!(f, "{surface_id} is Stable without canonical source mapping")
            }
            Self::StableSurfaceOutOfSync { surface_id } => {
                write!(f, "{surface_id} is Stable while source sync is not in sync")
            }
            Self::StableSurfaceWithoutSafePreview { surface_id } => {
                write!(f, "{surface_id} is Stable without safe preview")
            }
            Self::MissingGeneratedSourceTruth { surface_id } => {
                write!(f, "{surface_id} lacks generated/source truth")
            }
            Self::IncompleteFallbackPaths { surface_id } => {
                write!(f, "{surface_id} lacks source/diff/rollback fallback")
            }
            Self::IncompleteProjection { surface_id } => {
                write!(f, "{surface_id} lacks full projection coverage")
            }
            Self::SideEffectWithoutDryRun { surface_id } => {
                write!(f, "{surface_id} lacks dry run or closest safe preview")
            }
            Self::MissingPreviewApplyRevertLineage { surface_id } => {
                write!(f, "{surface_id} lacks preview/apply/revert lineage")
            }
            Self::BrowserInspectionBoundaryOverclaim { surface_id } => {
                write!(f, "{surface_id} overclaims browser inspection depth")
            }
            Self::MissingUnsupportedConstructCard { surface_id } => {
                write!(f, "{surface_id} lacks an unsupported-construct card")
            }
            Self::MissingSurfaceKind { surface_kind } => {
                write!(f, "missing surface kind {surface_kind:?}")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for PreviewDesignerPublishQualificationViolation {}
