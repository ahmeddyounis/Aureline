//! Canonical per-projection truth for M5 execution-evidence overlays: coverage,
//! flaky-test history, perf-regression notes, notebook-output verdicts, pipeline
//! annotations, and review-side markers projected away from their original run
//! surface with run/step/provider/artifact lineage intact.
//!
//! Where [`crate::m5_execution_evidence_causality_matrix`] froze the *lane* matrix
//! — one row per Problems/output/execution-evidence **surface family** — and
//! [`crate::m5_problem_records_source_task_correlation_and_rerun_jump_parity`] froze
//! the **individual Problems row**, this module freezes the **individual projected
//! overlay**. A projection re-renders execution evidence somewhere other than the
//! run that produced it: a coverage gutter over the editor, a flaky-history badge in
//! review, a perf-regression note on a diff, a notebook-output verdict, a pipeline
//! annotation, or a review-side marker. Each [`ExecutionEvidenceProjection`] is one
//! such overlay bound to the *original* run/step/provider/artifact lineage, the
//! revision-remap quality that maps the origin anchors onto the current
//! revision/cursor, the evidence freshness/stale/superseded state, the confidence
//! tier, and the reopen-to-origin target — so old evidence shown on a fresh surface
//! can never quietly masquerade as current truth.
//!
//! The projection speaks the **same** frozen vocabulary as the causality matrix
//! ([`ClaimPosture`], [`OriginClass`], [`ConfidenceTier`], [`FreshnessState`],
//! [`ReopenTarget`], [`ProofCurrency`], [`VerificationFreshness`]) rather than
//! forking a private overlay truth model. Reuse the canonical run/step/provider
//! refs, generated-artifact ids, output channels, and evidence packets already
//! landed earlier; this module binds them to one inspectable, reopenable overlay.
//!
//! Re-derivation rules ([`ExecutionEvidenceProjection::narrow`]):
//!
//! * Every projection keeps its **origin run/step and provider/artifact identity
//!   reopenable on demand** on every surface it renders: a coverage gutter, a review
//!   marker, and a support export must all be able to answer "which run, which step,
//!   which provider, which artifact produced this" without stitching raw logs.
//! * Every projection carries an explicit **revision-remap quality** and
//!   **freshness label**: an overlay anchored exactly to the current revision reads
//!   differently from one shifted-but-tracked, approximately remapped, or
//!   stale/unmapped, and a stale or superseded projection stays visibly classified
//!   rather than rendering as fresh certainty.
//! * **Imported/remote/pipeline** origins project only as a read-only overlay; they
//!   are attributable and reopenable but never claim live local authority, and a
//!   rendering surface may never render a claim wider than the projection's effective
//!   claim.
//! * A projection that flattens origin run/step or provider/artifact lineage, hides
//!   lineage from a surface, drops a heuristic raw-output backlink, loses its
//!   reopen-to-origin path, lets a rendering surface overclaim, or lets an imported
//!   overlay claim live authority floors to
//!   [`ProjectionClaim::Unreconstructable`] and keeps a raw-output / keyboard
//!   fallback rather than rendering a clean-but-false overlay. Stale/remap/labelled
//!   gaps hold a first-party projection at [`ProjectionClaim::Narrowed`] (still
//!   reopenable). Labs/unadvertised projections make no public claim and are never
//!   widened.
//!
//! [`M5ExecutionEvidenceProjectionSetPacket::validate`] confirms the packet is
//! well-formed and honest: header/identity/redaction/freshness are present, every
//! projection kind and every rendering surface is represented, overlay projections
//! name their provider, no rendering surface overclaims its projection, a floored
//! projection keeps a raw fallback, at least one projection demonstrates the
//! auto-narrowing rule, and no raw boundary material crosses the export. Downstream
//! coverage, flaky, perf, notebook, pipeline, review, support-export, AI-evidence,
//! and docs surfaces ingest this packet rather than inventing a parallel overlay
//! model.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies,
//! absolute paths, URLs, and secrets never cross this boundary; the packet carries
//! only typed class tokens, line/column numbers, booleans, opaque ids, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-execution-evidence-projections.schema.json`](../../../../schemas/tooling/m5-execution-evidence-projections.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-execution-evidence-projections.md`](../../../../docs/tooling/m5-execution-evidence-projections.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-execution-evidence-projections/support_export.json`](../../../../artifacts/tooling/m5-execution-evidence-projections/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-execution-evidence-projections/`](../../../../fixtures/tooling/m5-execution-evidence-projections/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    json_contains_forbidden_boundary_material, label_is_generic, parse_rfc3339_to_epoch_seconds,
    ClaimPosture, ConfidenceTier, FreshnessState, OriginClass, ProofCurrency, ReopenTarget,
    VerificationFreshness,
};

/// Stable record-kind tag carried by [`M5ExecutionEvidenceProjectionSetPacket`].
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND: &str =
    "m5_execution_evidence_projection_set_packet";

/// Schema version for the projection set.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical projection-set packet.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_PACKET_ID: &str =
    "m5-execution-evidence-projections:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_REF: &str =
    "schemas/tooling/m5-execution-evidence-projections.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_DOC_REF: &str =
    "docs/tooling/m5-execution-evidence-projections.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-execution-evidence-projections/support_export.json";

/// Repo-relative path of the generated certification report.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_REPORT_REF: &str =
    "artifacts/tooling/m5-execution-evidence-projections/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_EXECUTION_EVIDENCE_PROJECTIONS_FIXTURE_DIR: &str =
    "fixtures/tooling/m5-execution-evidence-projections";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

// --------------------------------------------------------------------------- //
// Frozen projection taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// Which execution-evidence overlay a projection backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionKind {
    /// Coverage overlay (covered / uncovered / partial decorations).
    CoverageOverlay,
    /// Flaky-test history badge or strip.
    FlakyTestHistory,
    /// Perf-regression note against a baseline run.
    PerfRegressionNote,
    /// Notebook cell output verdict.
    NotebookOutputVerdict,
    /// Pipeline annotation projected into code/review.
    PipelineAnnotation,
    /// Review-side marker (diff gutter / inline review annotation).
    ReviewSideMarker,
}

impl ProjectionKind {
    /// Every projection kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CoverageOverlay,
        Self::FlakyTestHistory,
        Self::PerfRegressionNote,
        Self::NotebookOutputVerdict,
        Self::PipelineAnnotation,
        Self::ReviewSideMarker,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageOverlay => "coverage_overlay",
            Self::FlakyTestHistory => "flaky_test_history",
            Self::PerfRegressionNote => "perf_regression_note",
            Self::NotebookOutputVerdict => "notebook_output_verdict",
            Self::PipelineAnnotation => "pipeline_annotation",
            Self::ReviewSideMarker => "review_side_marker",
        }
    }

    /// Whether this kind is inherently anchored to a file span on a revision, so a
    /// revision remap must be tracked rather than reported as not-anchored.
    pub const fn is_revision_anchored(self) -> bool {
        matches!(
            self,
            Self::CoverageOverlay
                | Self::PerfRegressionNote
                | Self::PipelineAnnotation
                | Self::ReviewSideMarker
        )
    }
}

/// A surface on which a projection is rendered, away from its original run surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionSurface {
    /// Editor gutter / inline overlay.
    EditorOverlay,
    /// Diff / review overlay.
    DiffReviewOverlay,
    /// Notebook overlay.
    NotebookOverlay,
    /// Pipeline overlay.
    PipelineOverlay,
    /// Incident / support overlay.
    IncidentOverlay,
    /// Activity-center timeline / history.
    TimelineHistory,
    /// Support export bundle.
    SupportExport,
    /// AI-evidence consumer.
    AiEvidence,
}

impl ProjectionSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorOverlay,
        Self::DiffReviewOverlay,
        Self::NotebookOverlay,
        Self::PipelineOverlay,
        Self::IncidentOverlay,
        Self::TimelineHistory,
        Self::SupportExport,
        Self::AiEvidence,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorOverlay => "editor_overlay",
            Self::DiffReviewOverlay => "diff_review_overlay",
            Self::NotebookOverlay => "notebook_overlay",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::IncidentOverlay => "incident_overlay",
            Self::TimelineHistory => "timeline_history",
            Self::SupportExport => "support_export",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

/// How well the origin run's anchors remap onto the current revision/cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapQuality {
    /// Anchored exactly to the current revision; no remap needed.
    ExactCurrentRevision,
    /// Lines shifted but tracked precisely onto the current revision.
    ShiftedTracked,
    /// Best-effort fuzzy remap; position is approximate.
    ApproximateRemap,
    /// Could not be remapped onto the current revision; anchor is stale.
    StaleUnmapped,
    /// Origin is not file-anchored, so no revision remap applies.
    NotAnchored,
}

impl RemapQuality {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCurrentRevision => "exact_current_revision",
            Self::ShiftedTracked => "shifted_tracked",
            Self::ApproximateRemap => "approximate_remap",
            Self::StaleUnmapped => "stale_unmapped",
            Self::NotAnchored => "not_anchored",
        }
    }

    /// Whether this quality leaves the anchor unmapped onto the current revision, so
    /// the overlay must visibly read as not-on-current-revision rather than current.
    pub const fn is_unmapped(self) -> bool {
        matches!(self, Self::StaleUnmapped)
    }
}

/// Confidence taxonomy is shared with the causality matrix; re-exported here so the
/// projection vocabulary stays self-describing.
pub use crate::m5_execution_evidence_causality_matrix::ConfidenceTier as ProjectionConfidenceTier;

// --------------------------------------------------------------------------- //
// Derived projection-claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a projection renders. A higher rank asserts more authority,
/// so a narrowed or floored projection must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionClaim {
    /// Lineage/remap/reopen broken; the projection surfaces a raw-output backlink or
    /// keyboard fallback instead of a clean-but-false overlay.
    #[serde(rename = "projection_unreconstructable")]
    Unreconstructable,
    /// Remote/pipeline/imported evidence; attributable and reopenable but never
    /// claims live local authority.
    #[serde(rename = "projection_read_only_overlay")]
    ReadOnlyOverlay,
    /// A first-party projection held below certified by a stale/remap/labelled gap,
    /// but lineage stays reopenable.
    #[serde(rename = "projection_narrowed")]
    Narrowed,
    /// Full first-party lineage preserved, fresh, remap exact/tracked, reopenable.
    #[serde(rename = "projection_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "projection_labs_not_claimed")]
    LabsNotClaimed,
}

impl ProjectionClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreconstructable => "projection_unreconstructable",
            Self::ReadOnlyOverlay => "projection_read_only_overlay",
            Self::Narrowed => "projection_narrowed",
            Self::Certified => "projection_certified",
            Self::LabsNotClaimed => "projection_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unreconstructable => Some(0),
            Self::ReadOnlyOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective claim.
    /// A rendering surface must never render wider than the projection's effective
    /// claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: ProjectionClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a projection fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionNarrowingReason {
    /// Origin run/step identity flattened away from the overlay.
    #[serde(rename = "origin_run_step_flattened")]
    OriginRunStepFlattened,
    /// Provider/artifact identity flattened away from the overlay.
    #[serde(rename = "provider_artifact_flattened")]
    ProviderArtifactFlattened,
    /// Lineage cannot be revealed on demand on some rendering surface.
    #[serde(rename = "lineage_not_visible")]
    LineageNotVisible,
    /// Heuristic projection without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// Reopen-to-origin lost; only a keyboard fallback remains.
    #[serde(rename = "reopen_target_lost")]
    ReopenTargetLost,
    /// A rendering surface renders a claim wider than the effective claim.
    #[serde(rename = "surface_overclaims")]
    SurfaceOverclaims,
    /// Imported/remote/pipeline overlay claims live local authority.
    #[serde(rename = "imported_overlay_claims_live")]
    ImportedOverlayClaimsLive,
    /// Evidence missing.
    #[serde(rename = "evidence_missing")]
    EvidenceMissing,
    /// Revision-remap quality not surfaced.
    #[serde(rename = "remap_quality_unlabeled")]
    RemapQualityUnlabeled,
    /// Anchor stale/unmapped onto the current revision but not labelled as such.
    #[serde(rename = "stale_remap_unlabeled")]
    StaleRemapUnlabeled,
    /// Evidence freshness state not surfaced.
    #[serde(rename = "freshness_unlabeled")]
    FreshnessUnlabeled,
    /// Confidence tier not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// Superseded-by-newer-run state not marked.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// First-party evidence projection stale.
    #[serde(rename = "evidence_stale")]
    StaleEvidence,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
}

impl ProjectionNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginRunStepFlattened => "origin_run_step_flattened",
            Self::ProviderArtifactFlattened => "provider_artifact_flattened",
            Self::LineageNotVisible => "lineage_not_visible",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::ReopenTargetLost => "reopen_target_lost",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::ImportedOverlayClaimsLive => "imported_overlay_claims_live",
            Self::EvidenceMissing => "evidence_missing",
            Self::RemapQualityUnlabeled => "remap_quality_unlabeled",
            Self::StaleRemapUnlabeled => "stale_remap_unlabeled",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::StaleEvidence => "evidence_stale",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
        }
    }

    /// Whether this reason floors a projection to
    /// [`ProjectionClaim::Unreconstructable`]. Each floor reason breaks the "stay
    /// reopenable / never flatten lineage / never masquerade as live" contract
    /// outright rather than merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::OriginRunStepFlattened
                | Self::ProviderArtifactFlattened
                | Self::LineageNotVisible
                | Self::RawBacklinkMissing
                | Self::ReopenTargetLost
                | Self::SurfaceOverclaims
                | Self::ImportedOverlayClaimsLive
                | Self::EvidenceMissing
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across runs.
    /// Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::OriginRunStepFlattened => 0,
            Self::ProviderArtifactFlattened => 1,
            Self::LineageNotVisible => 2,
            Self::ReopenTargetLost => 3,
            Self::RawBacklinkMissing => 4,
            Self::SurfaceOverclaims => 5,
            Self::ImportedOverlayClaimsLive => 6,
            Self::EvidenceMissing => 7,
            Self::RemapQualityUnlabeled => 8,
            Self::StaleRemapUnlabeled => 9,
            Self::FreshnessUnlabeled => 10,
            Self::ConfidenceUnlabeled => 11,
            Self::SupersededNotMarked => 12,
            Self::StaleEvidence => 13,
            Self::StaleProof => 14,
            Self::MissingProof => 15,
        }
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<ProjectionNarrowingReason>) -> Vec<ProjectionNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Projection sub-objects.
// --------------------------------------------------------------------------- //

/// Stable identifiers binding a projection to its origin. Lineage is reconstructed
/// from these refs, never inferred from freeform display text. Absent refs serialize
/// as `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionLineage {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// Origin run ref.
    pub origin_run_ref: Option<String>,
    /// Origin step ref.
    pub origin_step_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported overlays).
    pub provider_ref: Option<String>,
    /// Generated-artifact ref (coverage report, perf trace, notebook output, …).
    pub artifact_ref: Option<String>,
    /// Output-channel ref.
    pub channel_ref: Option<String>,
    /// Evidence-bundle / packet ref.
    pub evidence_packet_ref: Option<String>,
    /// Baseline run ref used for perf/flaky comparison.
    pub baseline_ref: Option<String>,
    /// Raw-output backlink ref.
    pub raw_output_backlink_ref: Option<String>,
}

/// The remap state binding origin anchors onto the current revision/cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionRemap {
    /// Quality of the remap onto the current revision.
    pub quality: RemapQuality,
    /// Whether the projection is anchored to the current revision.
    pub anchored_to_current_revision: bool,
    /// Whether a cursor/position remap was applied to track edits since the run.
    pub cursor_remap_applied: bool,
    /// Whether the remap quality is surfaced to the user rather than hidden.
    pub remap_quality_labeled: bool,
}

/// The projection-integrity invariants every projection re-derives rather than
/// trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionIntegrity {
    /// Origin run/step identity survives into the overlay.
    pub preserves_origin_run_step: bool,
    /// Provider/artifact identity survives into the overlay.
    pub preserves_provider_artifact: bool,
    /// Origin lineage can be revealed on demand on every rendering surface.
    pub lineage_visible_on_demand: bool,
    /// The freshness state is surfaced rather than hidden.
    pub freshness_state_labeled: bool,
    /// The confidence tier is surfaced rather than hidden.
    pub confidence_label_visible: bool,
    /// Superseded state stays marked.
    pub superseded_state_marked: bool,
    /// Imported overlays stay read-only.
    pub imported_overlay_read_only: bool,
    /// A heuristic projection keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
}

/// Certification-proof currency for a projection (distinct from the evidence's own
/// freshness state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionVerification {
    /// Currency of the certification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the projection.
    pub proof_ref: Option<String>,
}

/// One surface that renders a projection, with the claim it shows and whether it can
/// reveal the origin lineage on demand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRendering {
    /// The rendering surface.
    pub surface: ProjectionSurface,
    /// The claim this surface renders.
    pub rendered_claim: ProjectionClaim,
    /// Whether the origin run/step/provider/artifact lineage is revealable here.
    pub lineage_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical projection this surface re-renders.
    pub source_projection_ref: String,
}

// --------------------------------------------------------------------------- //
// Projection + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) execution-evidence projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEvidenceProjection {
    /// Stable projection id.
    pub projection_id: String,
    /// Which overlay this projection backs.
    pub projection_kind: ProjectionKind,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Whether the projection is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable origin-lineage block.
    pub lineage: ProjectionLineage,
    /// Revision-remap block.
    pub revision_remap: RevisionRemap,
    /// Projection-integrity invariant block.
    pub integrity: ProjectionIntegrity,
    /// Certification-proof block.
    pub verification: ProjectionVerification,
    /// Surfaces that render this projection.
    pub renderings: Vec<SurfaceRendering>,
}

/// The re-derived projection decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionDecision {
    /// The headline claim the projection is eligible to make.
    pub claimed_projection_claim: ProjectionClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_projection_claim: ProjectionClaim,
    /// Ordered, de-duplicated reasons the projection fails to hold its headline.
    pub active_narrowing_reasons: Vec<ProjectionNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl ProjectionDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ProjectionNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this projection would overclaim.
    pub fn surface_overclaims(&self, rendered: ProjectionClaim) -> bool {
        self.effective_projection_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(
    claimed: ProjectionClaim,
    reasons: &[ProjectionNarrowingReason],
) -> ProjectionClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        ProjectionClaim::Unreconstructable
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, ProjectionClaim::ReadOnlyOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can
        // no longer certify even the read-only overlay, so it floors.
        ProjectionClaim::Unreconstructable
    } else {
        ProjectionClaim::Narrowed
    }
}

impl ExecutionEvidenceProjection {
    /// Whether this projection is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this projection is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// The headline projection claim this projection is eligible to make.
    pub fn claimed_claim(&self) -> ProjectionClaim {
        if self.is_labs() {
            ProjectionClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ProjectionClaim::ReadOnlyOverlay
        } else {
            ProjectionClaim::Certified
        }
    }

    /// Whether this projection's parse confidence is one of the explicit heuristic
    /// tiers, which must keep a raw-output backlink.
    fn is_heuristic(&self) -> bool {
        self.declared_confidence_tier.is_heuristic_tier()
    }

    /// Reasons that hold independently of how the rendering surfaces render — the
    /// intrinsic lineage/remap/freshness gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<ProjectionNarrowingReason> {
        use ProjectionNarrowingReason as R;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Lineage: origin run/step + provider/artifact + on-demand visibility.
        if !integ.preserves_origin_run_step {
            reasons.push(R::OriginRunStepFlattened);
        }
        if !integ.preserves_provider_artifact {
            reasons.push(R::ProviderArtifactFlattened);
        }
        if !integ.lineage_visible_on_demand || self.renderings.iter().any(|r| !r.lineage_visible) {
            reasons.push(R::LineageNotVisible);
        }

        // A heuristic projection must keep a raw-output backlink and a tier label.
        if self.is_heuristic() && !integ.raw_output_backlink_present {
            reasons.push(R::RawBacklinkMissing);
        }
        if !integ.confidence_label_visible {
            reasons.push(R::ConfidenceUnlabeled);
        }

        // Revision-remap quality must be labelled, and a stale/unmapped anchor must
        // read as not-on-current-revision rather than silently current.
        if !self.revision_remap.remap_quality_labeled {
            reasons.push(R::RemapQualityUnlabeled);
        }
        if self.revision_remap.quality.is_unmapped()
            && self.revision_remap.anchored_to_current_revision
        {
            reasons.push(R::StaleRemapUnlabeled);
        }

        // Freshness must be labelled.
        if !integ.freshness_state_labeled {
            reasons.push(R::FreshnessUnlabeled);
        }

        // Reopen-to-origin must survive.
        if matches!(
            self.declared_reopen_target,
            ReopenTarget::NoneKeyboardFallback
        ) {
            reasons.push(R::ReopenTargetLost);
        }

        // Evidence freshness / superseded / missing.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::EvidenceMissing),
            FreshnessState::SupersededByNewerRun if !integ.superseded_state_marked => {
                reasons.push(R::SupersededNotMarked);
            }
            // An overlay snapshot is expected to be cached/stale; a first-party live
            // surface showing a stale projection has aged out of currency.
            FreshnessState::StaleExpired if !overlay => {
                reasons.push(R::StaleEvidence);
            }
            _ => {}
        }

        // Certification-proof currency (distinct from the evidence's own freshness).
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(R::MissingProof),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(R::StaleProof);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(R::StaleProof);
            }
            _ => {}
        }

        // Imported/remote/pipeline overlays must stay read-only.
        if overlay && !integ.imported_overlay_read_only {
            reasons.push(R::ImportedOverlayClaimsLive);
        }

        reasons
    }

    /// Every reason this projection fails to hold its headline claim, including a
    /// rendering surface that overclaims relative to the intrinsic effective claim.
    pub fn projection_reasons(&self, stale_window: bool) -> Vec<ProjectionNarrowingReason> {
        let claimed = self.claimed_claim();
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic = derive_effective(claimed, &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic.overclaims_as(r.rendered_claim))
        {
            reasons.push(ProjectionNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive the effective projection claim, reasons, and narrowed flag.
    pub fn narrow(&self, stale_window: bool) -> ProjectionDecision {
        let claimed = self.claimed_claim();

        // Labs/unadvertised projections make no public claim, so they never accrue
        // governance narrowing; they hold their non-claiming token.
        if matches!(claimed, ProjectionClaim::LabsNotClaimed) {
            return ProjectionDecision {
                claimed_projection_claim: ProjectionClaim::LabsNotClaimed,
                effective_projection_claim: ProjectionClaim::LabsNotClaimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }

        let reasons = self.projection_reasons(stale_window);
        let effective = derive_effective(claimed, &reasons);
        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        ProjectionDecision {
            claimed_projection_claim: claimed,
            effective_projection_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// The effective confidence tier: a floored projection cannot assert a tier
    /// beyond unmapped/needs-review.
    pub fn effective_confidence(&self, effective: ProjectionClaim) -> ConfidenceTier {
        if matches!(effective, ProjectionClaim::Unreconstructable) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored projection.
    pub fn narrowed_label(&self, decision: &ProjectionDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or("narrowed", ProjectionNarrowingReason::as_str)
            .replace('_', " ");
        let reopen = self.declared_reopen_target.as_str().replace('_', " ");
        let claimed = decision.claimed_projection_claim.as_str();
        let effective = decision.effective_projection_claim;
        let label = if matches!(effective, ProjectionClaim::Unreconstructable) {
            format!(
                "Floored to {} below the {claimed} claim: {trigger}; the {reopen} stays reopenable rather than rendering a clean-but-false overlay",
                effective.as_str()
            )
        } else {
            format!(
                "Held at {} below the {claimed} claim: {trigger}; lineage stays reopenable via the {reopen} until re-verified",
                effective.as_str()
            )
        };
        Some(label)
    }

    /// Whether a non-labs projection that floors keeps a reopen fallback rather than
    /// hiding lineage behind a clean-but-false claim.
    fn floored_keeps_fallback(&self, effective: ProjectionClaim) -> bool {
        if !matches!(effective, ProjectionClaim::Unreconstructable) {
            return true;
        }
        self.declared_reopen_target.is_raw_fallback()
            || self.integrity.raw_output_backlink_present
            || opt_present(&self.lineage.raw_output_backlink_ref)
    }

    /// Whether any rendering surface renders wider than the projection's effective
    /// claim.
    fn surface_overclaims(&self, effective: ProjectionClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// Structural checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5ExecutionEvidenceProjectionViolation>) {
        if self.projection_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.execution_context_ref.trim().is_empty()
        {
            out.push(M5ExecutionEvidenceProjectionViolation::ProjectionMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.lineage.provider_ref) {
            out.push(M5ExecutionEvidenceProjectionViolation::OverlayMissingProviderRef);
        }
        if self.renderings.is_empty() {
            out.push(M5ExecutionEvidenceProjectionViolation::ProjectionMissingRendering);
        }
        for rendering in &self.renderings {
            if rendering.source_projection_ref.trim().is_empty() {
                out.push(M5ExecutionEvidenceProjectionViolation::RenderingMissingSourceRef);
            }
        }
    }
}

/// Whether an optional ref is present and non-empty.
fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|inner| !inner.trim().is_empty())
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for an [`M5ExecutionEvidenceProjectionSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionEvidenceProjectionSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-projection rows.
    pub projections: Vec<ExecutionEvidenceProjection>,
}

/// Export-safe M5 execution-evidence projection-set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionEvidenceProjectionSetPacket {
    /// Record kind; must equal [`M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal
    /// [`M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-projection rows.
    pub projections: Vec<ExecutionEvidenceProjection>,
}

/// The distribution of effective projection claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionClaimDistribution {
    /// Projections effective at [`ProjectionClaim::Certified`].
    pub certified: usize,
    /// Projections effective at [`ProjectionClaim::Narrowed`].
    pub narrowed: usize,
    /// Projections effective at [`ProjectionClaim::ReadOnlyOverlay`].
    pub overlay: usize,
    /// Projections effective at [`ProjectionClaim::Unreconstructable`].
    pub unreconstructable: usize,
    /// Projections effective at [`ProjectionClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5ExecutionEvidenceProjectionSetPacket {
    /// Builds a projection-set packet, sealing the record-kind, schema, and taxonomy
    /// version constants.
    pub fn new(input: M5ExecutionEvidenceProjectionSetInput) -> Self {
        Self {
            record_kind: M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND.to_owned(),
            schema_version: M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            projections: input.projections,
        }
    }

    /// Whether the verification window has elapsed by `as_of`.
    pub fn freshness_stale_at(&self, as_of: &str) -> bool {
        if !self.verification_freshness.auto_downgrade_on_stale {
            return false;
        }
        let last =
            parse_rfc3339_to_epoch_seconds(&self.verification_freshness.last_verification_refresh);
        let now = parse_rfc3339_to_epoch_seconds(as_of);
        match (last, now) {
            (Some(last), Some(now)) => {
                now - last
                    > i64::from(self.verification_freshness.verification_freshness_slo_hours) * 3600
            }
            _ => false,
        }
    }

    /// Whether the window has elapsed by the packet's own `as_of`.
    pub fn stale_window(&self) -> bool {
        self.freshness_stale_at(&self.as_of)
    }

    /// Re-derive the decision for every projection, paired with its id.
    pub fn decisions(&self) -> Vec<(String, ProjectionDecision)> {
        let stale_window = self.stale_window();
        self.projections
            .iter()
            .map(|p| (p.projection_id.clone(), p.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective projection claims.
    pub fn claim_distribution(&self) -> ProjectionClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = ProjectionClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unreconstructable: 0,
            labs: 0,
        };
        for p in &self.projections {
            match p.narrow(stale_window).effective_projection_claim {
                ProjectionClaim::Certified => dist.certified += 1,
                ProjectionClaim::Narrowed => dist.narrowed += 1,
                ProjectionClaim::ReadOnlyOverlay => dist.overlay += 1,
                ProjectionClaim::Unreconstructable => dist.unreconstructable += 1,
                ProjectionClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of projections whose effective claim ranks below their claimed claim.
    pub fn narrowed_projection_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.projections
            .iter()
            .filter(|p| p.narrow(stale_window).narrowed)
            .count()
    }

    /// Projection kinds represented by some projection.
    pub fn represented_kinds(&self) -> BTreeSet<ProjectionKind> {
        self.projections.iter().map(|p| p.projection_kind).collect()
    }

    /// Rendering surfaces represented by some projection.
    pub fn represented_surfaces(&self) -> BTreeSet<ProjectionSurface> {
        self.projections
            .iter()
            .flat_map(|p| p.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the execution-evidence projection invariants.
    pub fn validate(&self) -> Vec<M5ExecutionEvidenceProjectionViolation> {
        use M5ExecutionEvidenceProjectionViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION {
            violations.push(V::WrongTaxonomyVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }
        if !REDACTION_CLASS_TOKENS.contains(&self.redaction_class_token.as_str()) {
            violations.push(V::InvalidRedactionClass);
        }
        if self.verification_freshness.verification_freshness_slo_hours == 0
            || self
                .verification_freshness
                .last_verification_refresh
                .trim()
                .is_empty()
        {
            violations.push(V::EvidenceFreshnessIncomplete);
        }
        if self.projections.is_empty() {
            violations.push(V::EmptyProjections);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for p in &self.projections {
            if !seen.insert(p.projection_id.as_str()) {
                violations.push(V::DuplicateProjectionId);
            }
        }

        let kinds = self.represented_kinds();
        if ProjectionKind::ALL.iter().any(|k| !kinds.contains(k)) {
            violations.push(V::ProjectionKindMissing);
        }
        let surfaces = self.represented_surfaces();
        if ProjectionSurface::ALL.iter().any(|s| !surfaces.contains(s)) {
            violations.push(V::ProjectionSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for p in &self.projections {
            p.structural_violations(&mut violations);
            let decision = p.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || p.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedProjectionMissingLabelOrTrigger);
                }
            }
            if !p.floored_keeps_fallback(decision.effective_projection_claim) {
                violations.push(V::FlooredProjectionLosesFallback);
            }
            if p.surface_overclaims(decision.effective_projection_claim) {
                violations.push(V::RenderingSurfaceOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedProjectionCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("execution-evidence projection packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("execution-evidence projection packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Execution-Evidence Projection Overlays\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Projections: {}\n", self.projections.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} read-only overlay, {} unreconstructable, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unreconstructable, dist.labs
        ));

        out.push_str("| Projection | Kind | Origin | Claimed | Effective | Remap | Confidence |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for p in &self.projections {
            let decision = p.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                p.projection_id,
                p.projection_kind.as_str(),
                p.origin_class.as_str(),
                decision.claimed_projection_claim.as_str(),
                decision.effective_projection_claim.as_str(),
                p.revision_remap.quality.as_str(),
                p.effective_confidence(decision.effective_projection_claim)
                    .as_str(),
            ));
        }

        out.push('\n');
        for p in &self.projections {
            let decision = p.narrow(stale_window);
            if let Some(label) = p.narrowed_label(&decision) {
                out.push_str(&format!("- Narrowed: `{}` — {}\n", p.projection_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5ExecutionEvidenceProjectionArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5ExecutionEvidenceProjectionViolation>),
}

impl fmt::Display for M5ExecutionEvidenceProjectionArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "execution-evidence projection support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "execution-evidence projection support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5ExecutionEvidenceProjectionArtifactError {}

/// Invariant violations reported by
/// [`M5ExecutionEvidenceProjectionSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionEvidenceProjectionViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Taxonomy version is wrong.
    WrongTaxonomyVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Redaction-class token is not one of the allowed values.
    InvalidRedactionClass,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// The packet carries no projections.
    EmptyProjections,
    /// Two projections share an id.
    DuplicateProjectionId,
    /// A required projection kind is unrepresented.
    ProjectionKindMissing,
    /// A required rendering surface is unrepresented.
    ProjectionSurfaceMissing,
    /// A projection is missing its id, label, or execution-context ref.
    ProjectionMissingIdentity,
    /// An overlay-origin projection does not name its provider.
    OverlayMissingProviderRef,
    /// A projection renders on no surface.
    ProjectionMissingRendering,
    /// A rendering is missing its source-projection backlink.
    RenderingMissingSourceRef,
    /// A floored projection lost its raw-output / keyboard reopen fallback.
    FlooredProjectionLosesFallback,
    /// A narrowed projection is missing its precise label or trigger.
    NarrowedProjectionMissingLabelOrTrigger,
    /// A rendering surface renders wider than the projection's effective claim.
    RenderingSurfaceOverclaims,
    /// No projection demonstrates the auto-narrowing rule.
    DowngradedProjectionCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ExecutionEvidenceProjectionViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyProjections => "empty_projections",
            Self::DuplicateProjectionId => "duplicate_projection_id",
            Self::ProjectionKindMissing => "projection_kind_missing",
            Self::ProjectionSurfaceMissing => "projection_surface_missing",
            Self::ProjectionMissingIdentity => "projection_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::ProjectionMissingRendering => "projection_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::FlooredProjectionLosesFallback => "floored_projection_loses_fallback",
            Self::NarrowedProjectionMissingLabelOrTrigger => {
                "narrowed_projection_missing_label_or_trigger"
            }
            Self::RenderingSurfaceOverclaims => "rendering_surface_overclaims",
            Self::DowngradedProjectionCaseMissing => "downgraded_projection_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream coverage, flaky, perf, notebook,
/// pipeline, review, support-export, AI-evidence, and docs surfaces use to ingest the
/// frozen projection set instead of cloning provider-local overlay state.
///
/// # Errors
///
/// Returns [`M5ExecutionEvidenceProjectionArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_execution_evidence_projection_set(
) -> Result<M5ExecutionEvidenceProjectionSetPacket, M5ExecutionEvidenceProjectionArtifactError> {
    let packet: M5ExecutionEvidenceProjectionSetPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/tooling/m5-execution-evidence-projections/support_export.json"
        )))
        .map_err(M5ExecutionEvidenceProjectionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExecutionEvidenceProjectionArtifactError::Validation(
            violations,
        ))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded projection set: the in-crate source of truth the checked-in
/// support export and report are regenerated from.
pub fn seeded_execution_evidence_projection_set() -> M5ExecutionEvidenceProjectionSetPacket {
    M5ExecutionEvidenceProjectionSetPacket::new(M5ExecutionEvidenceProjectionSetInput {
        packet_id: M5_EXECUTION_EVIDENCE_PROJECTIONS_PACKET_ID.to_owned(),
        label: "M5 execution-evidence projection overlays — preserved run/step/provider/artifact lineage".to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        projections: seed_projections(),
    })
}

/// Renderings that show a `claim` cleanly across the named surfaces.
fn renderings(
    source_ref: &str,
    claim: ProjectionClaim,
    surfaces: &[ProjectionSurface],
    read_only: bool,
) -> Vec<SurfaceRendering> {
    surfaces
        .iter()
        .map(|&surface| SurfaceRendering {
            surface,
            rendered_claim: claim,
            lineage_visible: true,
            read_only,
            source_projection_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> ProjectionIntegrity {
    ProjectionIntegrity {
        preserves_origin_run_step: true,
        preserves_provider_artifact: true,
        lineage_visible_on_demand: true,
        freshness_state_labeled: true,
        confidence_label_visible: true,
        superseded_state_marked: true,
        imported_overlay_read_only: true,
        raw_output_backlink_present: true,
    }
}

/// A verified-current proof block.
fn verified(proof_ref: &str) -> ProjectionVerification {
    ProjectionVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// An exact-on-current-revision remap block.
fn exact_remap() -> RevisionRemap {
    RevisionRemap {
        quality: RemapQuality::ExactCurrentRevision,
        anchored_to_current_revision: true,
        cursor_remap_applied: false,
        remap_quality_labeled: true,
    }
}

fn seed_projections() -> Vec<ExecutionEvidenceProjection> {
    vec![
        // 1. Coverage overlay over the editor from a local test run — certified.
        ExecutionEvidenceProjection {
            projection_id: "projection:coverage-local-test:0001".to_owned(),
            projection_kind: ProjectionKind::CoverageOverlay,
            label_summary:
                "Coverage gutter over the editor projected from a local test run with exact revision anchoring."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                origin_run_ref: Some("run.local.test.coverage.0001".to_owned()),
                origin_step_ref: Some("step.local.test.coverage.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.coverage.lcov.0001".to_owned()),
                channel_ref: Some("channel.local.test.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.coverage.0001".to_owned()),
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.local.test.coverage.0001".to_owned()),
            },
            revision_remap: exact_remap(),
            integrity: clean_integrity(),
            verification: verified("proof.local.coverage.0001"),
            renderings: renderings(
                "projection:coverage-local-test:0001",
                ProjectionClaim::Certified,
                &[
                    ProjectionSurface::EditorOverlay,
                    ProjectionSurface::DiffReviewOverlay,
                    ProjectionSurface::SupportExport,
                ],
                false,
            ),
        },
        // 2. Flaky-test history badge in review — certified, shifted-but-tracked.
        ExecutionEvidenceProjection {
            projection_id: "projection:flaky-history-local-test:0001".to_owned(),
            projection_kind: ProjectionKind::FlakyTestHistory,
            label_summary:
                "Flaky-test history strip projected into the timeline and editor, shifted but tracked onto the current revision."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                origin_run_ref: Some("run.local.test.flaky.0001".to_owned()),
                origin_step_ref: Some("step.local.test.flaky.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.flaky.history.0001".to_owned()),
                channel_ref: Some("channel.local.test.0002".to_owned()),
                evidence_packet_ref: Some("evidence.local.flaky.0001".to_owned()),
                baseline_ref: Some("run.local.test.flaky.baseline.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.flaky.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::ShiftedTracked,
                anchored_to_current_revision: true,
                cursor_remap_applied: true,
                remap_quality_labeled: true,
            },
            integrity: clean_integrity(),
            verification: verified("proof.local.flaky.0001"),
            renderings: renderings(
                "projection:flaky-history-local-test:0001",
                ProjectionClaim::Certified,
                &[
                    ProjectionSurface::EditorOverlay,
                    ProjectionSurface::TimelineHistory,
                    ProjectionSurface::AiEvidence,
                ],
                false,
            ),
        },
        // 3. Perf-regression note on a diff — narrowed by an approximate remap.
        ExecutionEvidenceProjection {
            projection_id: "projection:perf-regression-local-task:0001".to_owned(),
            projection_kind: ProjectionKind::PerfRegressionNote,
            label_summary:
                "Perf-regression note against a baseline run, approximately remapped onto an edited diff and held below certified."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::HeuristicHigh,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                origin_run_ref: Some("run.local.bench.perf.0001".to_owned()),
                origin_step_ref: Some("step.local.bench.perf.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.perf.trace.0001".to_owned()),
                channel_ref: Some("channel.local.bench.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.perf.0001".to_owned()),
                baseline_ref: Some("run.local.bench.perf.baseline.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.bench.perf.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::ApproximateRemap,
                anchored_to_current_revision: true,
                cursor_remap_applied: true,
                remap_quality_labeled: true,
            },
            // A first-party stale-proof gap narrows but stays reopenable.
            integrity: clean_integrity(),
            verification: ProjectionVerification {
                proof_currency: ProofCurrency::StaleExpired,
                proof_ref: Some("proof.local.perf.0001".to_owned()),
            },
            // Narrowed via stale proof; renderings must render the narrowed claim.
            renderings: renderings(
                "projection:perf-regression-local-task:0001",
                ProjectionClaim::Narrowed,
                &[
                    ProjectionSurface::DiffReviewOverlay,
                    ProjectionSurface::EditorOverlay,
                    ProjectionSurface::IncidentOverlay,
                ],
                false,
            ),
        },
        // 4. Notebook-output verdict — certified, not file-anchored.
        ExecutionEvidenceProjection {
            projection_id: "projection:notebook-verdict-cell:0001".to_owned(),
            projection_kind: ProjectionKind::NotebookOutputVerdict,
            label_summary:
                "Notebook cell output verdict projected onto the notebook surface with cell-anchored lineage."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::NotebookRun,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.notebook.primary".to_owned(),
                origin_run_ref: Some("run.local.notebook.cell.0001".to_owned()),
                origin_step_ref: Some("step.local.notebook.cell.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.notebook.output.0001".to_owned()),
                channel_ref: Some("channel.local.notebook.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.notebook.0001".to_owned()),
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.local.notebook.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::NotAnchored,
                anchored_to_current_revision: false,
                cursor_remap_applied: false,
                remap_quality_labeled: true,
            },
            integrity: clean_integrity(),
            verification: verified("proof.local.notebook.0001"),
            renderings: renderings(
                "projection:notebook-verdict-cell:0001",
                ProjectionClaim::Certified,
                &[
                    ProjectionSurface::NotebookOverlay,
                    ProjectionSurface::SupportExport,
                ],
                false,
            ),
        },
        // 5. Pipeline annotation projected into review — read-only overlay.
        ExecutionEvidenceProjection {
            projection_id: "projection:pipeline-annotation-provider:0001".to_owned(),
            projection_kind: ProjectionKind::PipelineAnnotation,
            label_summary:
                "Pipeline provider annotation projected into the diff/review and pipeline overlays as an attributable read-only overlay."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::PipelineProviderRun,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.remote.pipeline.primary".to_owned(),
                origin_run_ref: Some("run.pipeline.provider.0001".to_owned()),
                origin_step_ref: Some("step.pipeline.provider.0001".to_owned()),
                provider_ref: Some("provider.pipeline.ci.0001".to_owned()),
                artifact_ref: Some("artifact.pipeline.annotation.0001".to_owned()),
                channel_ref: Some("channel.pipeline.provider.0001".to_owned()),
                evidence_packet_ref: Some("evidence.pipeline.provider.0001".to_owned()),
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.pipeline.provider.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::ShiftedTracked,
                anchored_to_current_revision: true,
                cursor_remap_applied: true,
                remap_quality_labeled: true,
            },
            integrity: clean_integrity(),
            verification: ProjectionVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.pipeline.provider.0001".to_owned()),
            },
            renderings: renderings(
                "projection:pipeline-annotation-provider:0001",
                ProjectionClaim::ReadOnlyOverlay,
                &[
                    ProjectionSurface::DiffReviewOverlay,
                    ProjectionSurface::PipelineOverlay,
                    ProjectionSurface::IncidentOverlay,
                ],
                true,
            ),
        },
        // 6. Review-side marker from a local task — certified.
        ExecutionEvidenceProjection {
            projection_id: "projection:review-marker-local-task:0001".to_owned(),
            projection_kind: ProjectionKind::ReviewSideMarker,
            label_summary:
                "Review-side marker projected into the diff/review overlay from a local task with exact revision anchoring."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::EditorAnchor,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                origin_run_ref: Some("run.local.task.review.0001".to_owned()),
                origin_step_ref: Some("step.local.task.review.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.review.marker.0001".to_owned()),
                channel_ref: Some("channel.local.task.0003".to_owned()),
                evidence_packet_ref: Some("evidence.local.review.0001".to_owned()),
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.local.task.review.0001".to_owned()),
            },
            revision_remap: exact_remap(),
            integrity: clean_integrity(),
            verification: verified("proof.local.review.0001"),
            renderings: renderings(
                "projection:review-marker-local-task:0001",
                ProjectionClaim::Certified,
                &[
                    ProjectionSurface::DiffReviewOverlay,
                    ProjectionSurface::EditorOverlay,
                ],
                false,
            ),
        },
        // 7. Imported provider coverage overlay — read-only overlay.
        ExecutionEvidenceProjection {
            projection_id: "projection:coverage-imported-provider:0001".to_owned(),
            projection_kind: ProjectionKind::CoverageOverlay,
            label_summary:
                "Imported provider coverage overlay surfaced read-only, cached within window and never claiming live local authority."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ImportedProviderEvidence,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.remote.import.primary".to_owned(),
                origin_run_ref: Some("run.import.provider.coverage.0001".to_owned()),
                origin_step_ref: None,
                provider_ref: Some("provider.import.coverage.0001".to_owned()),
                artifact_ref: Some("artifact.import.coverage.0001".to_owned()),
                channel_ref: Some("channel.import.provider.0001".to_owned()),
                evidence_packet_ref: Some("evidence.import.coverage.0001".to_owned()),
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.import.coverage.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::ApproximateRemap,
                anchored_to_current_revision: false,
                cursor_remap_applied: false,
                remap_quality_labeled: true,
            },
            integrity: clean_integrity(),
            verification: ProjectionVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.import.coverage.0001".to_owned()),
            },
            renderings: renderings(
                "projection:coverage-imported-provider:0001",
                ProjectionClaim::ReadOnlyOverlay,
                &[
                    ProjectionSurface::EditorOverlay,
                    ProjectionSurface::SupportExport,
                    ProjectionSurface::AiEvidence,
                ],
                true,
            ),
        },
        // 8. Labs notebook-output verdict — makes no public claim.
        ExecutionEvidenceProjection {
            projection_id: "projection:notebook-verdict-labs:0001".to_owned(),
            projection_kind: ProjectionKind::NotebookOutputVerdict,
            label_summary:
                "Labs notebook-output verdict overlay; unadvertised, makes no public claim and is never widened."
                    .to_owned(),
            claim_posture: ClaimPosture::LabsUnadvertised,
            origin_class: OriginClass::NotebookRun,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            lineage: ProjectionLineage {
                execution_context_ref: "exec-context.local.notebook.labs".to_owned(),
                origin_run_ref: Some("run.local.notebook.labs.0001".to_owned()),
                origin_step_ref: None,
                provider_ref: None,
                artifact_ref: None,
                channel_ref: Some("channel.local.notebook.labs.0001".to_owned()),
                evidence_packet_ref: None,
                baseline_ref: None,
                raw_output_backlink_ref: Some("raw.local.notebook.labs.0001".to_owned()),
            },
            revision_remap: RevisionRemap {
                quality: RemapQuality::NotAnchored,
                anchored_to_current_revision: false,
                cursor_remap_applied: false,
                remap_quality_labeled: true,
            },
            integrity: clean_integrity(),
            verification: ProjectionVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
            renderings: renderings(
                "projection:notebook-verdict-labs:0001",
                ProjectionClaim::LabsNotClaimed,
                &[ProjectionSurface::NotebookOverlay],
                false,
            ),
        },
    ]
}
