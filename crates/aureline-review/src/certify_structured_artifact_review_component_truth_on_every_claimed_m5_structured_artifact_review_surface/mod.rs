//! Surface certification of artifact-identity-bar, diff-mode-switcher,
//! structure-row, merge-decision-row, generated-artifact-notice,
//! rendered-compare-viewer, media-metadata-rail, redaction-or-trust-badge-set, and
//! compare-summary-card truth on every claimed M5 structured-artifact review
//! surface.
//!
//! This module is the closing certification capstone over the nine shared
//! structured-artifact review components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`],
//! implemented by the identity-bar / diff-mode, structure-row / compare-summary,
//! merge-decision / generated-notice, and rendered-compare / media-rail /
//! redaction-trust lanes, adopted by the shared consumers in
//! [`crate::add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned`],
//! and proven across assistive, headless, and exported forms by
//! [`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_parser_schema_render_trust_or_write_back_safety_is_unavailable_across_claimed_m5_structured_artifact_review_components`].
//!
//! Where the implement lanes ship the components and the consumer / accessibility
//! lanes prove mode / risk / provenance parity, this lane certifies the release
//! claim: that on every claimed M5 structured-artifact review surface — diff
//! toolbar, merge sheet, review workspace, help surface, support export, exported
//! artifact packet, headless CLI, and diagnostics — the same controlled component
//! truth is presented with no hidden parser/schema, render-trust, write-back, or
//! metadata drift. Each certified surface row scores six certification axes
//! ([`StructuredArtifactCertificationAxis`]): the visual, keyboard, screen-reader,
//! and CLI/export axes that every claim must always pass, the degraded-state axis
//! that narrows a claim when parser/schema state, render trust, write-back safety,
//! or metadata availability weakens, and the structured-fidelity-provenance axis
//! that keeps the certification honest — a certified surface never implies its
//! structured fidelity is full, its render is trusted, or its write-back is safe.
//!
//! A surface earns [`StructuredArtifactSurfaceClaimStatus::CertifiedParity`] only
//! when its certified claim equals its claimed claim, no axis narrows, and component
//! truth is preserved. It narrows to
//! [`StructuredArtifactSurfaceClaimStatus::NarrowedParity`] the moment an axis
//! narrows or the certified claim drops below the claimed one, and it fails to
//! [`StructuredArtifactSurfaceClaimStatus::ParityBlocked`] whenever the artifact
//! class, canonical source, diff mode, parser/schema state, compare-only /
//! write-back safety, render trust, generated-from relation, metadata visibility, or
//! redaction posture is flattened out of the export. That last rule is the delta of
//! this capstone: certification may narrow a claim but may never drop the
//! component's meaning.
//!
//! The packet references upstream component, consumer, and accessibility contracts
//! by id rather than embedding their content. Raw artifact payloads, credentials,
//! and live parser output stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-structured-artifact-review-component-certification.schema.json`](../../../../schemas/ui/m5-structured-artifact-review-component-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_parser_schema_render_trust_or_write_back_safety_is_unavailable_across_claimed_m5_structured_artifact_review_components::ArtifactReviewClaimTier;
use crate::M5ArtifactComponent;

/// Stable record-kind tag carried by [`StructuredArtifactCertificationPacket`].
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_RECORD_KIND: &str =
    "m5_structured_artifact_component_surface_certification_truth";

/// Schema version for structured-artifact review-component surface certification records.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_DOC_REF: &str =
    "docs/review/m5/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface.md";

/// Repo-relative path of the frozen component matrix this certification builds on.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this certification builds on.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-consumer.schema.json";

/// Repo-relative path of the accessibility / headless / export parity contract this certification builds on.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json";

/// Repo-relative path of the artifact-identity / diff-mode controls contract.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_IDENTITY_DIFF_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json";

/// Repo-relative path of the structure-row / compare-summary controls contract.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-structure-compare-summary-controls.schema.json";

/// Repo-relative path of the merge-decision / generated-notice controls contract.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_MERGE_GENERATED_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-decision-generated-notice-controls.schema.json";

/// Repo-relative path of the rendered-compare / media-rail / redaction-trust controls contract.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_MEDIA_TRUST_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-rendered-compare-media-trust-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-structured-artifact-review-component-certification";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/review/m5/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/review/m5/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface.md";

/// Repo-relative path of the release-proof support export.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_RELEASE_PROOF_ARTIFACT_REF: &str =
    "artifacts/release/m5-structured-artifact-review-certification-proof/support_export.json";

/// Repo-relative path of the release-proof certification matrix CSV.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_RELEASE_PROOF_MATRIX_REF: &str =
    "artifacts/release/m5-structured-artifact-review-certification-proof/matrix.csv";

/// Repo-relative path of the release-proof report.
pub const M5_STRUCTURED_ARTIFACT_CERTIFICATION_RELEASE_PROOF_REPORT_REF: &str =
    "artifacts/release/m5-structured-artifact-review-certification-proof/report.md";

/// Canonical component contract that a certified surface row must cite for a
/// component it presents.
///
/// Each of the nine shared components resolves to the checked-in controls schema of
/// the lane that implemented it: the artifact-identity / diff-mode controls (which
/// govern the identity bar and diff-mode switcher), the structure / compare-summary
/// controls (structure rows and compare-summary cards), the merge-decision /
/// generated-notice controls (merge-decision rows and generated-artifact notices),
/// and the rendered-compare / media-rail / redaction-trust controls (rendered
/// compare viewers, media-metadata rails, and redaction/trust badge sets).
pub const fn certification_component_canonical_schema_ref(
    component: M5ArtifactComponent,
) -> &'static str {
    match component {
        M5ArtifactComponent::ArtifactIdentityBar | M5ArtifactComponent::DiffModeSwitcher => {
            M5_STRUCTURED_ARTIFACT_CERTIFICATION_IDENTITY_DIFF_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::StructureRow | M5ArtifactComponent::CompareSummaryCard => {
            M5_STRUCTURED_ARTIFACT_CERTIFICATION_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::MergeDecisionRow | M5ArtifactComponent::GeneratedArtifactNotice => {
            M5_STRUCTURED_ARTIFACT_CERTIFICATION_MERGE_GENERATED_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::RenderedCompareViewer
        | M5ArtifactComponent::MediaMetadataRail
        | M5ArtifactComponent::RedactionOrTrustBadgeSet => {
            M5_STRUCTURED_ARTIFACT_CERTIFICATION_MEDIA_TRUST_CONTROLS_CONTRACT_REF
        }
    }
}

/// A claimed M5 structured-artifact review surface whose component truth this packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StructuredArtifactCertifiedSurface {
    /// Desktop diff / compare toolbar surface.
    DiffToolbarSurface,
    /// Merge review sheet surface.
    MergeSheetSurface,
    /// Review workspace surface.
    ReviewWorkspaceSurface,
    /// Help / About structured-artifact review surface.
    HelpArtifactSurface,
    /// Support export bundle.
    SupportExport,
    /// Exported artifact review packet (offline / publish-later compare pack).
    ExportedArtifactPacket,
    /// Headless CLI compare / merge output.
    CliHeadless,
    /// Diagnostics review surface.
    Diagnostics,
}

impl M5StructuredArtifactCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DiffToolbarSurface,
        Self::MergeSheetSurface,
        Self::ReviewWorkspaceSurface,
        Self::HelpArtifactSurface,
        Self::SupportExport,
        Self::ExportedArtifactPacket,
        Self::CliHeadless,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffToolbarSurface => "diff_toolbar_surface",
            Self::MergeSheetSurface => "merge_sheet_surface",
            Self::ReviewWorkspaceSurface => "review_workspace_surface",
            Self::HelpArtifactSurface => "help_artifact_surface",
            Self::SupportExport => "support_export",
            Self::ExportedArtifactPacket => "exported_artifact_packet",
            Self::CliHeadless => "cli_headless",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// A certification axis scored on every certified surface row.
///
/// The first four axes are always-on: a claimed component must always pass them on
/// every surface. [`DegradedState`](Self::DegradedState) narrows a claim when
/// parser/schema state, render trust, write-back safety, or metadata availability
/// weakens. [`StructuredFidelityProvenance`](Self::StructuredFidelityProvenance) is
/// the certification-specific separation axis: it keeps the structured-vs-raw and
/// render-trust distinctions explicit so a certified surface never implies its
/// structured fidelity is full, its render is trusted, or its write-back is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredArtifactCertificationAxis {
    /// Visual rendering carries the controlled component truth.
    Visual,
    /// Keyboard reach and operation carry the controlled component truth.
    Keyboard,
    /// Screen-reader labelling carries the controlled component truth.
    ScreenReader,
    /// CLI and export forms carry the controlled component truth.
    CliExport,
    /// Degraded parser/schema, render-trust, write-back, or metadata state narrows the claim honestly.
    DegradedState,
    /// The structured-vs-raw and render-trust distinction stays explicit; certified never implies full fidelity.
    StructuredFidelityProvenance,
}

impl StructuredArtifactCertificationAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Visual,
        Self::Keyboard,
        Self::ScreenReader,
        Self::CliExport,
        Self::DegradedState,
        Self::StructuredFidelityProvenance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::StructuredFidelityProvenance => "structured_fidelity_provenance",
        }
    }

    /// Whether this axis must always be certified on every claimed surface.
    pub const fn is_always_on(self) -> bool {
        matches!(
            self,
            Self::Visual | Self::Keyboard | Self::ScreenReader | Self::CliExport
        )
    }
}

/// The certification state of a single axis on a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredArtifactAxisCertificationState {
    /// The axis is fully certified on this surface.
    Certified,
    /// The axis is certified but narrowed (an honest fallback is disclosed).
    NarrowedCertified,
    /// The axis is not certified on this surface (it is honestly out of scope here).
    NotCertifiedHere,
}

impl StructuredArtifactAxisCertificationState {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::NotCertifiedHere => "not_certified_here",
        }
    }
}

/// The certification status a surface row earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredArtifactSurfaceClaimStatus {
    /// Green: certified claim equals claimed claim, no axis narrows, truth preserved.
    CertifiedParity,
    /// Yellow: certification is narrowed but component truth is preserved.
    NarrowedParity,
    /// Red: component truth was flattened out of this surface.
    ParityBlocked,
}

impl StructuredArtifactSurfaceClaimStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedParity => "certified_parity",
            Self::NarrowedParity => "narrowed_parity",
            Self::ParityBlocked => "parity_blocked",
        }
    }

    /// Whether the surface is fully certified (green).
    pub const fn is_green(self) -> bool {
        matches!(self, Self::CertifiedParity)
    }

    /// Whether the surface is blocked (red).
    pub const fn is_red(self) -> bool {
        matches!(self, Self::ParityBlocked)
    }
}

/// Downgrade trigger that can narrow a certified surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredArtifactCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// An upstream evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Parser/schema state is uncertain and structure coverage narrowed.
    ParserSchemaUncertain,
    /// Render trust is unavailable and the rendered compare fell back to raw.
    RenderTrustUnavailable,
    /// Write-back safety is unavailable and the artifact is compare-only.
    WriteBackSafetyUnavailable,
    /// A disclosed raw / export-safe fallback is required.
    RawFallbackRequired,
    /// Metadata or content is withheld or redacted under the export/redaction posture.
    MetadataWithheldRedaction,
    /// Consumer or workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl StructuredArtifactCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ParserSchemaUncertain,
        Self::RenderTrustUnavailable,
        Self::WriteBackSafetyUnavailable,
        Self::RawFallbackRequired,
        Self::MetadataWithheldRedaction,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ParserSchemaUncertain => "parser_schema_uncertain",
            Self::RenderTrustUnavailable => "render_trust_unavailable",
            Self::WriteBackSafetyUnavailable => "write_back_safety_unavailable",
            Self::RawFallbackRequired => "raw_fallback_required",
            Self::MetadataWithheldRedaction => "metadata_withheld_redaction",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Derives the certification status of a surface from its claims and axis narrowing.
///
/// Component truth is the hard gate: if the artifact class, canonical source, diff
/// mode, parser/schema state, compare-only / write-back safety, render trust,
/// generated-from relation, metadata visibility, or redaction posture is flattened,
/// the surface is [`StructuredArtifactSurfaceClaimStatus::ParityBlocked`] regardless
/// of the claim tiers. Otherwise a certified claim below the claimed one, or any
/// narrowed axis, narrows the surface to
/// [`StructuredArtifactSurfaceClaimStatus::NarrowedParity`]; only a full, un-narrowed
/// claim earns [`StructuredArtifactSurfaceClaimStatus::CertifiedParity`].
pub const fn derive_structured_artifact_surface_claim_status(
    claimed: ArtifactReviewClaimTier,
    certified: ArtifactReviewClaimTier,
    component_truth_preserved: bool,
    has_narrowed_axes: bool,
) -> StructuredArtifactSurfaceClaimStatus {
    if !component_truth_preserved {
        StructuredArtifactSurfaceClaimStatus::ParityBlocked
    } else if certified.rank() < claimed.rank() || has_narrowed_axes {
        StructuredArtifactSurfaceClaimStatus::NarrowedParity
    } else {
        StructuredArtifactSurfaceClaimStatus::CertifiedParity
    }
}

/// One axis outcome on a certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertAxisOutcome {
    /// The certification axis scored.
    pub axis: StructuredArtifactCertificationAxis,
    /// The state the axis earned on this surface.
    pub state: StructuredArtifactAxisCertificationState,
    /// Human-readable note explaining the outcome (never empty).
    pub note: String,
}

/// One certified surface row: a claimed M5 structured-artifact review surface and
/// the component truth it presents, scored across the six certification axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// The claimed M5 structured-artifact review surface.
    pub surface: M5StructuredArtifactCertifiedSurface,
    /// The shared components this surface presents (non-empty).
    pub components_present: Vec<M5ArtifactComponent>,
    /// The claim tier the surface claims for its components.
    pub claimed_claim: ArtifactReviewClaimTier,
    /// The claim tier the certification actually earns.
    pub certified_claim: ArtifactReviewClaimTier,
    /// The certification status the surface earns.
    pub status: StructuredArtifactSurfaceClaimStatus,
    /// Per-axis outcomes; must cover all six axes.
    pub axis_outcomes: Vec<StructuredArtifactCertAxisOutcome>,
    /// The axes that narrowed on this surface (subset of the axis outcomes).
    pub narrowed_axes: Vec<StructuredArtifactCertificationAxis>,
    /// The downgrade trigger disclosed when the surface narrows.
    pub downgrade_trigger: Option<StructuredArtifactCertificationDowngradeTrigger>,
    /// Delta invariant: the component's artifact class, canonical source, diff mode,
    /// parser/schema, compare-only / write-back safety, render trust, generated-from,
    /// metadata, and redaction truth is preserved (never flattened).
    pub component_truth_preserved: bool,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl StructuredArtifactCertifiedSurfaceRow {
    /// The status this row should carry, derived from its claims and narrowing.
    pub fn derived_status(&self) -> StructuredArtifactSurfaceClaimStatus {
        derive_structured_artifact_surface_claim_status(
            self.claimed_claim,
            self.certified_claim,
            self.component_truth_preserved,
            !self.narrowed_axes.is_empty(),
        )
    }

    /// Whether the recorded status matches the derived one.
    pub fn status_is_consistent(&self) -> bool {
        self.status == self.derived_status()
    }

    /// Whether every axis is scored on this row.
    pub fn covers_all_axes(&self) -> bool {
        StructuredArtifactCertificationAxis::ALL.iter().all(|axis| {
            self.axis_outcomes
                .iter()
                .any(|outcome| outcome.axis == *axis)
        })
    }

    /// Whether every parity / export field is present.
    pub fn parity_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether the certified claim stays at or below the claimed one.
    pub fn certified_claim_within_claimed(&self) -> bool {
        self.certified_claim.rank() <= self.claimed_claim.rank()
    }

    /// Whether the narrowed axes agree with the axis outcomes marked narrowed.
    pub fn narrowed_axes_consistent(&self) -> bool {
        let narrowed: BTreeSet<StructuredArtifactCertificationAxis> =
            self.narrowed_axes.iter().copied().collect();
        for outcome in &self.axis_outcomes {
            let marked_narrowed =
                outcome.state == StructuredArtifactAxisCertificationState::NarrowedCertified;
            if marked_narrowed != narrowed.contains(&outcome.axis) {
                return false;
            }
        }
        true
    }

    /// Whether this row cites the canonical matrix and each present component's schema.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF) {
            return false;
        }
        self.components_present.iter().all(|component| {
            refs.contains(certification_component_canonical_schema_ref(*component))
        })
    }
}

/// Aggregate certification summary across all surface rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertificationSummary {
    /// Total certified surface rows.
    pub total_rows: u32,
    /// Count of green (fully certified) surfaces.
    pub certified_count: u32,
    /// Count of yellow (narrowed) surfaces.
    pub narrowed_count: u32,
    /// Count of red (blocked) surfaces.
    pub blocked_count: u32,
    /// True when every surface preserves component truth (no red).
    pub all_rows_preserve_component_truth: bool,
    /// True when all eight claimed surfaces are covered.
    pub all_surfaces_covered: bool,
    /// True when all nine shared components appear across the surfaces.
    pub all_components_covered: bool,
    /// Human-readable certification note.
    pub certification_note: String,
}

impl StructuredArtifactCertificationSummary {
    /// Recomputes the summary from a surface row set.
    pub fn from_rows(rows: &[StructuredArtifactCertifiedSurfaceRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut seen_surfaces: BTreeSet<M5StructuredArtifactCertifiedSurface> = BTreeSet::new();
        let mut seen_components: BTreeSet<M5ArtifactComponent> = BTreeSet::new();
        for row in rows {
            match row.status {
                StructuredArtifactSurfaceClaimStatus::CertifiedParity => certified += 1,
                StructuredArtifactSurfaceClaimStatus::NarrowedParity => narrowed += 1,
                StructuredArtifactSurfaceClaimStatus::ParityBlocked => blocked += 1,
            }
            seen_surfaces.insert(row.surface);
            for component in &row.components_present {
                seen_components.insert(*component);
            }
        }
        let all_surfaces_covered = M5StructuredArtifactCertifiedSurface::ALL
            .iter()
            .all(|surface| seen_surfaces.contains(surface));
        let all_components_covered = M5ArtifactComponent::ALL
            .iter()
            .all(|component| seen_components.contains(component));
        let all_preserve = blocked == 0;
        let certification_note = if all_preserve {
            format!(
                "{certified} surface(s) certified, {narrowed} narrowed; all preserve component truth"
            )
        } else {
            format!("{blocked} surface(s) blocked: component truth was flattened")
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            all_rows_preserve_component_truth: all_preserve,
            all_surfaces_covered,
            all_components_covered,
            certification_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertificationTrustReview {
    /// Every claimed surface presents the same controlled component truth.
    pub same_component_truth_on_every_surface: bool,
    /// Artifact class and canonical source stay explicit, never generic file chrome.
    pub artifact_class_and_canonical_source_explicit: bool,
    /// Diff mode and parser/schema state stay explicit, never flattened to raw silently.
    pub diff_mode_and_parser_schema_explicit: bool,
    /// Compare-only stays distinct from writable / write-back-safe state.
    pub compare_only_versus_write_back_distinct: bool,
    /// Render trust and generated-from relation stay explicit.
    pub render_trust_and_generated_from_explicit: bool,
    /// Structured fidelity stays explicit; certified never implies full fidelity.
    pub certified_never_implies_full_fidelity: bool,
    /// Raw / export-safe fallback stays explicit when render, schema, or merge fidelity narrows.
    pub raw_export_safe_fallback_explicit: bool,
    /// Metadata visibility and redaction posture are preserved, never hidden behind chrome.
    pub metadata_and_redaction_posture_preserved: bool,
    /// Certification narrows a claim rather than dropping the component's meaning.
    pub narrows_instead_of_dropping_meaning: bool,
    /// A surface that flattens component truth blocks its certification.
    pub flattened_truth_blocks_certification: bool,
}

impl StructuredArtifactCertificationTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.same_component_truth_on_every_surface
            && self.artifact_class_and_canonical_source_explicit
            && self.diff_mode_and_parser_schema_explicit
            && self.compare_only_versus_write_back_distinct
            && self.render_trust_and_generated_from_explicit
            && self.certified_never_implies_full_fidelity
            && self.raw_export_safe_fallback_explicit
            && self.metadata_and_redaction_posture_preserved
            && self.narrows_instead_of_dropping_meaning
            && self.flattened_truth_blocks_certification
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertificationConsumerProjection {
    /// Diff toolbar surface shows the certified component truth.
    pub diff_toolbar_surface_shows_certification: bool,
    /// Merge sheet surface shows the certified component truth.
    pub merge_sheet_surface_shows_certification: bool,
    /// Review workspace surface shows the certified component truth.
    pub review_workspace_surface_shows_certification: bool,
    /// Help / About artifact surface shows the certified component truth.
    pub help_artifact_surface_shows_certification: bool,
    /// Support export shows the certified component truth.
    pub support_export_shows_certification: bool,
    /// Exported artifact packet shows the certified component truth.
    pub exported_artifact_packet_shows_certification: bool,
    /// CLI / headless shows the certified component truth.
    pub cli_headless_shows_certification: bool,
    /// Diagnostics shows the certified component truth.
    pub diagnostics_shows_certification: bool,
    /// Narrowed surfaces are visibly labelled rather than silently downgraded.
    pub narrowed_surfaces_visibly_labelled: bool,
}

impl StructuredArtifactCertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diff_toolbar_surface_shows_certification
            && self.merge_sheet_surface_shows_certification
            && self.review_workspace_surface_shows_certification
            && self.help_artifact_surface_shows_certification
            && self.support_export_shows_certification
            && self.exported_artifact_packet_shows_certification
            && self.cli_headless_shows_certification
            && self.diagnostics_shows_certification
            && self.narrowed_surfaces_visibly_labelled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to [`StructuredArtifactCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredArtifactCertObservation {
    /// Surface the observation applies to.
    pub surface: M5StructuredArtifactCertifiedSurface,
    /// True when the surface's structured fidelity (parser/schema and render trust) is currently full.
    pub structured_fidelity_fresh: bool,
    /// True when the surface still preserves component truth.
    pub component_truth_preserved: bool,
}

/// Constructor input for [`StructuredArtifactCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredArtifactCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<StructuredArtifactCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: StructuredArtifactCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<StructuredArtifactCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: StructuredArtifactCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StructuredArtifactCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StructuredArtifactCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe structured-artifact review-component surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredArtifactCertificationPacket {
    /// Record kind; must equal [`M5_STRUCTURED_ARTIFACT_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<StructuredArtifactCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: StructuredArtifactCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<StructuredArtifactCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: StructuredArtifactCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StructuredArtifactCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StructuredArtifactCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl StructuredArtifactCertificationPacket {
    /// Builds a structured-artifact review-component surface certification packet from stable-lane input.
    pub fn new(input: StructuredArtifactCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_STRUCTURED_ARTIFACT_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            summary: input.summary,
            downgrade_triggers: input.downgrade_triggers,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows surfaces whose structured fidelity is no longer full and blocks
    /// surfaces that flatten component truth, then recomputes the summary.
    ///
    /// This is the downgrade automation: a surface reported with a flattened
    /// component truth blocks (red); a still-green surface whose structured fidelity
    /// (parser/schema and render trust) went stale narrows its full-fidelity claim to
    /// a disclosed raw fallback, marks the structured-fidelity-provenance axis
    /// narrowed, and discloses the render-trust trigger. Observations for surfaces
    /// not present in the packet are ignored; surfaces without an observation are
    /// left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[StructuredArtifactCertObservation],
    ) {
        for row in &mut self.surface_rows {
            let Some(observation) = observations.iter().find(|obs| obs.surface == row.surface)
            else {
                continue;
            };
            if !observation.component_truth_preserved {
                row.component_truth_preserved = false;
            } else if !observation.structured_fidelity_fresh
                && row.status == StructuredArtifactSurfaceClaimStatus::CertifiedParity
            {
                if row.certified_claim.rank() > ArtifactReviewClaimTier::RawFallbackDisclosed.rank()
                {
                    row.certified_claim = ArtifactReviewClaimTier::RawFallbackDisclosed;
                }
                if !row
                    .narrowed_axes
                    .contains(&StructuredArtifactCertificationAxis::StructuredFidelityProvenance)
                {
                    row.narrowed_axes
                        .push(StructuredArtifactCertificationAxis::StructuredFidelityProvenance);
                }
                for outcome in &mut row.axis_outcomes {
                    if outcome.axis
                        == StructuredArtifactCertificationAxis::StructuredFidelityProvenance
                    {
                        outcome.state = StructuredArtifactAxisCertificationState::NarrowedCertified;
                        outcome.note =
                            "Render and parser trust went stale; the claim narrows to a disclosed raw fallback and the structured-fidelity provenance stays explicit"
                                .to_owned();
                    }
                }
                row.downgrade_trigger =
                    Some(StructuredArtifactCertificationDowngradeTrigger::RenderTrustUnavailable);
            }
            row.status = row.derived_status();
        }
        self.summary = StructuredArtifactCertificationSummary::from_rows(&self.surface_rows);
    }

    /// Validates the structured-artifact review-component surface certification invariants.
    pub fn validate(&self) -> Vec<StructuredArtifactCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STRUCTURED_ARTIFACT_CERTIFICATION_RECORD_KIND {
            violations.push(StructuredArtifactCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_VERSION {
            violations.push(StructuredArtifactCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(StructuredArtifactCertificationViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(StructuredArtifactCertificationViolation::DowngradeTriggersMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(StructuredArtifactCertificationViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(StructuredArtifactCertificationViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(StructuredArtifactCertificationViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("structured-artifact certification packet serializes"),
        ) {
            violations.push(StructuredArtifactCertificationViolation::RawArtifactMaterialInExport);
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
            .expect("structured-artifact certification packet serializes")
    }

    /// Deterministic certification matrix CSV for release proof.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n",
        );
        for row in &self.surface_rows {
            let narrowed = row
                .narrowed_axes
                .iter()
                .map(|axis| axis.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
                row.status.as_str(),
                narrowed,
                row.component_truth_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Structured-Artifact Review-Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_rows,
            self.summary.certified_count,
            self.summary.narrowed_count,
            self.summary.blocked_count,
        ));
        out.push_str(&format!(
            "- All surfaces preserve component truth: {}\n",
            self.summary.all_rows_preserve_component_truth
        ));
        out.push_str(&format!("- Note: {}\n", self.summary.certification_note));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Certified surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: `{}` (claimed `{}`, certified `{}`)\n",
                row.surface.as_str(),
                row.row_id,
                row.status.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in structured-artifact certification export.
#[derive(Debug)]
pub enum StructuredArtifactCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StructuredArtifactCertificationViolation>),
}

impl fmt::Display for StructuredArtifactCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "structured-artifact certification export parse failed: {error}"
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
                    "structured-artifact certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for StructuredArtifactCertificationArtifactError {}

/// Validation failures emitted by [`StructuredArtifactCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuredArtifactCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No surface rows are present.
    SurfaceRowsMissing,
    /// A surface row is incomplete.
    RowIncomplete,
    /// A surface row lists no components.
    ComponentsMissingOnRow,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not score all six certification axes.
    AxisCoverageMissing,
    /// An axis outcome is missing its explanatory note.
    AxisNoteMissing,
    /// A certified claim exceeds the claimed claim it certifies.
    CertifiedClaimExceedsClaimed,
    /// The recorded status does not agree with the derived one.
    StatusMismatch,
    /// The narrowed-axis list disagrees with the axis outcomes marked narrowed.
    NarrowedAxesInconsistent,
    /// A narrowed surface is missing its disclosed downgrade trigger.
    NarrowingWithoutTrigger,
    /// A surface flattened the component's artifact class / source / mode / trust / metadata truth.
    StructuredArtifactComponentTruthDropped,
    /// A row does not cite the canonical matrix and component contracts.
    CanonicalContractReferenceMissing,
    /// Not every claimed surface appears among the rows.
    SurfaceCoverageMissing,
    /// Not every shared component appears across the surfaces.
    ComponentCoverageMissing,
    /// The summary does not agree with the surface rows.
    SummaryMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// Export contains raw artifact boundary material.
    RawArtifactMaterialInExport,
}

impl StructuredArtifactCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceRowsMissing => "surface_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::ComponentsMissingOnRow => "components_missing_on_row",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::AxisCoverageMissing => "axis_coverage_missing",
            Self::AxisNoteMissing => "axis_note_missing",
            Self::CertifiedClaimExceedsClaimed => "certified_claim_exceeds_claimed",
            Self::StatusMismatch => "status_mismatch",
            Self::NarrowedAxesInconsistent => "narrowed_axes_inconsistent",
            Self::NarrowingWithoutTrigger => "narrowing_without_trigger",
            Self::StructuredArtifactComponentTruthDropped => {
                "structured_artifact_component_truth_dropped"
            }
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::SummaryMismatch => "summary_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RawArtifactMaterialInExport => "raw_artifact_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable structured-artifact certification export.
pub fn current_structured_artifact_certification_export(
) -> Result<StructuredArtifactCertificationPacket, StructuredArtifactCertificationArtifactError> {
    let packet: StructuredArtifactCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface/support_export.json"
    )))
    .map_err(StructuredArtifactCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StructuredArtifactCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> StructuredArtifactCertificationTrustReview {
    StructuredArtifactCertificationTrustReview {
        same_component_truth_on_every_surface: true,
        artifact_class_and_canonical_source_explicit: true,
        diff_mode_and_parser_schema_explicit: true,
        compare_only_versus_write_back_distinct: true,
        render_trust_and_generated_from_explicit: true,
        certified_never_implies_full_fidelity: true,
        raw_export_safe_fallback_explicit: true,
        metadata_and_redaction_posture_preserved: true,
        narrows_instead_of_dropping_meaning: true,
        flattened_truth_blocks_certification: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> StructuredArtifactCertificationConsumerProjection {
    StructuredArtifactCertificationConsumerProjection {
        diff_toolbar_surface_shows_certification: true,
        merge_sheet_surface_shows_certification: true,
        review_workspace_surface_shows_certification: true,
        help_artifact_surface_shows_certification: true,
        support_export_shows_certification: true,
        exported_artifact_packet_shows_certification: true,
        cli_headless_shows_certification: true,
        diagnostics_shows_certification: true,
        narrowed_surfaces_visibly_labelled: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_DOC_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_CONSUMER_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_IDENTITY_DIFF_CONTROLS_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_MERGE_GENERATED_CONTROLS_CONTRACT_REF.to_owned(),
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_MEDIA_TRUST_CONTROLS_CONTRACT_REF.to_owned(),
    ]
}

fn validate_source_contracts(
    packet: &StructuredArtifactCertificationPacket,
    violations: &mut Vec<StructuredArtifactCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_SCHEMA_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_DOC_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_CONSUMER_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_IDENTITY_DIFF_CONTROLS_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_MERGE_GENERATED_CONTROLS_CONTRACT_REF,
        M5_STRUCTURED_ARTIFACT_CERTIFICATION_MEDIA_TRUST_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(StructuredArtifactCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &StructuredArtifactCertificationPacket,
    violations: &mut Vec<StructuredArtifactCertificationViolation>,
) {
    if packet.surface_rows.is_empty() {
        violations.push(StructuredArtifactCertificationViolation::SurfaceRowsMissing);
        return;
    }

    let mut seen_surfaces: BTreeSet<M5StructuredArtifactCertifiedSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5ArtifactComponent> = BTreeSet::new();

    for row in &packet.surface_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(StructuredArtifactCertificationViolation::RowIncomplete);
        }
        if row.components_present.is_empty() {
            violations.push(StructuredArtifactCertificationViolation::ComponentsMissingOnRow);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(StructuredArtifactCertificationViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(StructuredArtifactCertificationViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(StructuredArtifactCertificationViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(StructuredArtifactCertificationViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(StructuredArtifactCertificationViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_axes() {
            violations.push(StructuredArtifactCertificationViolation::AxisCoverageMissing);
        }
        if row
            .axis_outcomes
            .iter()
            .any(|outcome| outcome.note.trim().is_empty())
        {
            violations.push(StructuredArtifactCertificationViolation::AxisNoteMissing);
        }

        // AC2 core: a certified claim may never exceed the claim it certifies.
        if !row.certified_claim_within_claimed() {
            violations.push(StructuredArtifactCertificationViolation::CertifiedClaimExceedsClaimed);
        }

        if !row.narrowed_axes_consistent() {
            violations.push(StructuredArtifactCertificationViolation::NarrowedAxesInconsistent);
        }

        // A narrowed surface must disclose its downgrade trigger.
        if !row.narrowed_axes.is_empty() && row.downgrade_trigger.is_none() {
            violations.push(StructuredArtifactCertificationViolation::NarrowingWithoutTrigger);
        }

        // Delta: certification may narrow a claim but never drop component truth.
        if !row.component_truth_preserved {
            violations.push(
                StructuredArtifactCertificationViolation::StructuredArtifactComponentTruthDropped,
            );
        }

        // The recorded status must agree with the derived one.
        if !row.status_is_consistent() {
            violations.push(StructuredArtifactCertificationViolation::StatusMismatch);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(StructuredArtifactCertificationViolation::CanonicalContractReferenceMissing);
        }

        seen_surfaces.insert(row.surface);
        for component in &row.components_present {
            seen_components.insert(*component);
        }
    }

    for surface in M5StructuredArtifactCertifiedSurface::ALL {
        if !seen_surfaces.contains(&surface) {
            violations.push(StructuredArtifactCertificationViolation::SurfaceCoverageMissing);
            break;
        }
    }
    for component in M5ArtifactComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(StructuredArtifactCertificationViolation::ComponentCoverageMissing);
            break;
        }
    }
}

fn validate_summary(
    packet: &StructuredArtifactCertificationPacket,
    violations: &mut Vec<StructuredArtifactCertificationViolation>,
) {
    let recomputed = StructuredArtifactCertificationSummary::from_rows(&packet.surface_rows);
    if recomputed != packet.summary {
        violations.push(StructuredArtifactCertificationViolation::SummaryMismatch);
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
