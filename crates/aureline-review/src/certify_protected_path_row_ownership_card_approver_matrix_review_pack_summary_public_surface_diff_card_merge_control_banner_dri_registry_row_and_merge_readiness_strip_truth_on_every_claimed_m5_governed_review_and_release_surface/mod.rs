//! Surface certification of protected-path-row, ownership-card, approver-matrix,
//! review-pack-summary, public-surface-diff-card, merge-control-banner,
//! DRI-registry-row, and merge-readiness-strip truth on every claimed M5 governed
//! review and release surface.
//!
//! This module is the closing certification capstone over the eight shared
//! protected-path governance components frozen in
//! [`crate::freeze_the_m5_protected_path_governance_component_matrix`], implemented by
//! the protected-path / ownership, approver-matrix / review-pack, public-surface-diff
//! / merge-control, and DRI-registry / merge-readiness lanes, adopted by the shared
//! consumers in
//! [`crate::add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned`],
//! and proven across assistive, headless, and exported forms by
//! [`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_owner_coverage_approver_state_review_pack_freshness_provider_enforcement_or_public_surface_diff_truth_is_stale_or_partial_across_claimed_m5_governance_components`].
//!
//! Where the implement lanes ship the components and the consumer / accessibility
//! lanes prove owner / approver / public-surface parity, this lane certifies the
//! release claim: that on every claimed M5 governed review and release surface —
//! review workspace, merge queue, release center, help surface, support export,
//! shiproom, exported governance packet, and headless CLI — the same reusable
//! governance component truth is presented with no hidden enforcement authority,
//! owner-coverage, approver-state, review-pack-freshness, or public-surface-diff
//! drift. Each certified surface row scores six certification axes
//! ([`GovernanceComponentCertificationAxis`]): the visual, keyboard, screen-reader,
//! and CLI/export axes that every claim must always pass, the degraded-state axis
//! that narrows a claim when provider enforcement, owner coverage, approver state,
//! review-pack freshness, or public-surface diff truth weakens, and the
//! enforcement-ownership-provenance axis that keeps the certification honest — a
//! certified surface never implies that an advisory owner hint is
//! provider-authoritative enforcement, that a guarded merge covers a missing backup,
//! or that a public-surface change is clean without its machine-generated diff and
//! migration evidence.
//!
//! A surface earns [`GovernanceComponentSurfaceClaimStatus::CertifiedParity`] only
//! when its certified claim equals its claimed claim, no axis narrows, and component
//! truth is preserved. It narrows to
//! [`GovernanceComponentSurfaceClaimStatus::NarrowedParity`] the moment an axis
//! narrows or the certified claim drops below the claimed one, and it fails to
//! [`GovernanceComponentSurfaceClaimStatus::ParityBlocked`] whenever the protection
//! reason, owner source, advisory-versus-authoritative enforcement, approver state,
//! review-pack freshness, public-surface change class, merge-control blockers, DRI
//! coverage, or exportable escalation continuity is flattened out of the export. That
//! last rule is the delta of this capstone: certification may narrow a claim but may
//! never drop the component's meaning.
//!
//! The packet references upstream component, consumer, and accessibility contracts by
//! id rather than embedding their content. Raw provider responses, credentials, and
//! CODEOWNERS payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-protected-path-governance-component-certification.schema.json`](../../../../schemas/ui/m5-protected-path-governance-component-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_protected_path_governance_component_matrix::M5GovernanceComponent;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_owner_coverage_approver_state_review_pack_freshness_provider_enforcement_or_public_surface_diff_truth_is_stale_or_partial_across_claimed_m5_governance_components::GovernanceComponentClaimTier;

/// Stable record-kind tag carried by [`GovernanceComponentCertificationPacket`].
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_RECORD_KIND: &str =
    "m5_protected_path_governance_component_surface_certification_truth";

/// Schema version for protected-path governance-component surface certification records.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_DOC_REF: &str =
    "docs/review/m5/certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface.md";

/// Repo-relative path of the frozen governance component matrix this certification builds on.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this certification builds on.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-consumer.schema.json";

/// Repo-relative path of the accessibility / headless / export parity contract this certification builds on.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-accessibility-parity.schema.json";

/// Repo-relative path of the protected-path / ownership controls contract.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF:
    &str = "schemas/ui/m5-protected-path-ownership-controls.schema.json";

/// Repo-relative path of the approver-matrix / review-pack controls contract.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-approver-review-pack-controls.schema.json";

/// Repo-relative path of the public-surface-diff / merge-control controls contract.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF:
    &str = "schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json";

/// Repo-relative path of the DRI-registry / merge-readiness controls contract.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF:
    &str = "schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-protected-path-governance-component-certification";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/review/m5/certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/review/m5/certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface.md";

/// Repo-relative path of the release-proof support export.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_RELEASE_PROOF_ARTIFACT_REF: &str =
    "artifacts/release/m5-protected-path-governance-certification-proof/support_export.json";

/// Repo-relative path of the release-proof certification matrix CSV.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_RELEASE_PROOF_MATRIX_REF: &str =
    "artifacts/release/m5-protected-path-governance-certification-proof/matrix.csv";

/// Repo-relative path of the release-proof report.
pub const M5_GOVERNANCE_COMPONENT_CERTIFICATION_RELEASE_PROOF_REPORT_REF: &str =
    "artifacts/release/m5-protected-path-governance-certification-proof/report.md";

/// Canonical component contract that a certified surface row must cite for a
/// component it presents.
///
/// Each of the eight shared components resolves to the checked-in controls schema of
/// the lane that implemented it: the protected-path / ownership controls (protected
/// path rows and ownership cards), the approver-matrix / review-pack controls
/// (approver matrices and review-pack summaries), the public-surface-diff /
/// merge-control controls (public-surface diff cards and merge-control banners), and
/// the DRI-registry / merge-readiness controls (DRI registry rows and merge-readiness
/// strips).
pub const fn certification_component_canonical_schema_ref(
    component: M5GovernanceComponent,
) -> &'static str {
    match component {
        M5GovernanceComponent::ProtectedPathRow | M5GovernanceComponent::OwnershipCard => {
            M5_GOVERNANCE_COMPONENT_CERTIFICATION_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::ApproverMatrix | M5GovernanceComponent::ReviewPackSummary => {
            M5_GOVERNANCE_COMPONENT_CERTIFICATION_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::PublicSurfaceDiffCard
        | M5GovernanceComponent::MergeControlBanner => {
            M5_GOVERNANCE_COMPONENT_CERTIFICATION_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::DriRegistryRow | M5GovernanceComponent::MergeReadinessStrip => {
            M5_GOVERNANCE_COMPONENT_CERTIFICATION_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF
        }
    }
}

/// A claimed M5 governed review / release surface whose component truth this packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceCertifiedSurface {
    /// Desktop review workspace surface.
    ReviewWorkspaceSurface,
    /// Merge queue surface.
    MergeQueueSurface,
    /// Release center surface.
    ReleaseCenterSurface,
    /// Help / About governed-review surface.
    HelpGovernanceSurface,
    /// Support export bundle.
    SupportExport,
    /// Shiproom escalation surface.
    ShiproomSurface,
    /// Exported governance packet (offline / publish-later governance pack).
    ExportedGovernancePacket,
    /// Headless CLI governed-review / release output.
    CliHeadless,
}

impl M5GovernanceCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReviewWorkspaceSurface,
        Self::MergeQueueSurface,
        Self::ReleaseCenterSurface,
        Self::HelpGovernanceSurface,
        Self::SupportExport,
        Self::ShiproomSurface,
        Self::ExportedGovernancePacket,
        Self::CliHeadless,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspaceSurface => "review_workspace_surface",
            Self::MergeQueueSurface => "merge_queue_surface",
            Self::ReleaseCenterSurface => "release_center_surface",
            Self::HelpGovernanceSurface => "help_governance_surface",
            Self::SupportExport => "support_export",
            Self::ShiproomSurface => "shiproom_surface",
            Self::ExportedGovernancePacket => "exported_governance_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// A certification axis scored on every certified surface row.
///
/// The first four axes are always-on: a claimed component must always pass them on
/// every surface. [`DegradedState`](Self::DegradedState) narrows a claim when provider
/// enforcement, owner coverage, approver state, review-pack freshness, or
/// public-surface diff truth weakens. [`EnforcementOwnershipProvenance`](Self::EnforcementOwnershipProvenance)
/// is the certification-specific separation axis: it keeps the
/// advisory-versus-authoritative enforcement, owner-source, and public-surface change
/// class distinctions explicit so a certified surface never implies its enforcement is
/// provider-authoritative, its owner coverage is complete, or its public-surface change
/// is clean without evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentCertificationAxis {
    /// Visual rendering carries the controlled component truth.
    Visual,
    /// Keyboard reach and operation carry the controlled component truth.
    Keyboard,
    /// Screen-reader labelling carries the controlled component truth.
    ScreenReader,
    /// CLI and export forms carry the controlled component truth.
    CliExport,
    /// Degraded enforcement, owner-coverage, approver, review-pack, or public-surface state narrows the claim honestly.
    DegradedState,
    /// The advisory-versus-authoritative, owner-source, and public-surface distinction stays explicit; certified never implies provider authority.
    EnforcementOwnershipProvenance,
}

impl GovernanceComponentCertificationAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Visual,
        Self::Keyboard,
        Self::ScreenReader,
        Self::CliExport,
        Self::DegradedState,
        Self::EnforcementOwnershipProvenance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::EnforcementOwnershipProvenance => "enforcement_ownership_provenance",
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
pub enum GovernanceComponentAxisCertificationState {
    /// The axis is fully certified on this surface.
    Certified,
    /// The axis is certified but narrowed (an honest fallback is disclosed).
    NarrowedCertified,
    /// The axis is not certified on this surface (it is honestly out of scope here).
    NotCertifiedHere,
}

impl GovernanceComponentAxisCertificationState {
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
pub enum GovernanceComponentSurfaceClaimStatus {
    /// Green: certified claim equals claimed claim, no axis narrows, truth preserved.
    CertifiedParity,
    /// Yellow: certification is narrowed but component truth is preserved.
    NarrowedParity,
    /// Red: component truth was flattened out of this surface.
    ParityBlocked,
}

impl GovernanceComponentSurfaceClaimStatus {
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
pub enum GovernanceComponentCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// An upstream evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider enforcement is advisory, stale, or a local estimate, not authoritative.
    ProviderEnforcementAdvisoryOrStale,
    /// Owner backup coverage is missing or unresolved for the guarded path.
    OwnerBackupCoverageMissing,
    /// A required approval is waived or has expired.
    ApproverStateWaivedOrExpired,
    /// The review pack is stale relative to the change it gates.
    ReviewPackStale,
    /// The public-surface diff or migration evidence is missing or ungenerated.
    PublicSurfaceEvidenceMissing,
    /// Consumer or workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl GovernanceComponentCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ProviderEnforcementAdvisoryOrStale,
        Self::OwnerBackupCoverageMissing,
        Self::ApproverStateWaivedOrExpired,
        Self::ReviewPackStale,
        Self::PublicSurfaceEvidenceMissing,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderEnforcementAdvisoryOrStale => "provider_enforcement_advisory_or_stale",
            Self::OwnerBackupCoverageMissing => "owner_backup_coverage_missing",
            Self::ApproverStateWaivedOrExpired => "approver_state_waived_or_expired",
            Self::ReviewPackStale => "review_pack_stale",
            Self::PublicSurfaceEvidenceMissing => "public_surface_evidence_missing",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Derives the certification status of a surface from its claims and axis narrowing.
///
/// Component truth is the hard gate: if the protection reason, owner source,
/// advisory-versus-authoritative enforcement, approver state, review-pack freshness,
/// public-surface change class, merge-control blockers, DRI coverage, or exportable
/// escalation continuity is flattened, the surface is
/// [`GovernanceComponentSurfaceClaimStatus::ParityBlocked`] regardless of the claim
/// tiers. Otherwise a certified claim below the claimed one, or any narrowed axis,
/// narrows the surface to [`GovernanceComponentSurfaceClaimStatus::NarrowedParity`];
/// only a full, un-narrowed claim earns
/// [`GovernanceComponentSurfaceClaimStatus::CertifiedParity`].
pub const fn derive_governance_component_surface_claim_status(
    claimed: GovernanceComponentClaimTier,
    certified: GovernanceComponentClaimTier,
    component_truth_preserved: bool,
    has_narrowed_axes: bool,
) -> GovernanceComponentSurfaceClaimStatus {
    if !component_truth_preserved {
        GovernanceComponentSurfaceClaimStatus::ParityBlocked
    } else if certified.rank() < claimed.rank() || has_narrowed_axes {
        GovernanceComponentSurfaceClaimStatus::NarrowedParity
    } else {
        GovernanceComponentSurfaceClaimStatus::CertifiedParity
    }
}

/// One axis outcome on a certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertAxisOutcome {
    /// The certification axis scored.
    pub axis: GovernanceComponentCertificationAxis,
    /// The state the axis earned on this surface.
    pub state: GovernanceComponentAxisCertificationState,
    /// Human-readable note explaining the outcome (never empty).
    pub note: String,
}

/// One certified surface row: a claimed M5 governed review / release surface and the
/// component truth it presents, scored across the six certification axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// The claimed M5 governed review / release surface.
    pub surface: M5GovernanceCertifiedSurface,
    /// The shared components this surface presents (non-empty).
    pub components_present: Vec<M5GovernanceComponent>,
    /// The claim tier the surface claims for its components.
    pub claimed_claim: GovernanceComponentClaimTier,
    /// The claim tier the certification actually earns.
    pub certified_claim: GovernanceComponentClaimTier,
    /// The certification status the surface earns.
    pub status: GovernanceComponentSurfaceClaimStatus,
    /// Per-axis outcomes; must cover all six axes.
    pub axis_outcomes: Vec<GovernanceComponentCertAxisOutcome>,
    /// The axes that narrowed on this surface (subset of the axis outcomes).
    pub narrowed_axes: Vec<GovernanceComponentCertificationAxis>,
    /// The downgrade trigger disclosed when the surface narrows.
    pub downgrade_trigger: Option<GovernanceComponentCertificationDowngradeTrigger>,
    /// Delta invariant: the component's protection reason, owner source, enforcement
    /// authority, approver state, review-pack freshness, public-surface change class,
    /// merge blockers, DRI coverage, and escalation continuity truth is preserved
    /// (never flattened).
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

impl GovernanceComponentCertifiedSurfaceRow {
    /// The status this row should carry, derived from its claims and narrowing.
    pub fn derived_status(&self) -> GovernanceComponentSurfaceClaimStatus {
        derive_governance_component_surface_claim_status(
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
        GovernanceComponentCertificationAxis::ALL
            .iter()
            .all(|axis| {
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
        let narrowed: BTreeSet<GovernanceComponentCertificationAxis> =
            self.narrowed_axes.iter().copied().collect();
        for outcome in &self.axis_outcomes {
            let marked_narrowed =
                outcome.state == GovernanceComponentAxisCertificationState::NarrowedCertified;
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
        if !refs.contains(M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF) {
            return false;
        }
        self.components_present.iter().all(|component| {
            refs.contains(certification_component_canonical_schema_ref(*component))
        })
    }
}

/// Aggregate certification summary across all surface rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertificationSummary {
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
    /// True when all eight shared components appear across the surfaces.
    pub all_components_covered: bool,
    /// Human-readable certification note.
    pub certification_note: String,
}

impl GovernanceComponentCertificationSummary {
    /// Recomputes the summary from a surface row set.
    pub fn from_rows(rows: &[GovernanceComponentCertifiedSurfaceRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut seen_surfaces: BTreeSet<M5GovernanceCertifiedSurface> = BTreeSet::new();
        let mut seen_components: BTreeSet<M5GovernanceComponent> = BTreeSet::new();
        for row in rows {
            match row.status {
                GovernanceComponentSurfaceClaimStatus::CertifiedParity => certified += 1,
                GovernanceComponentSurfaceClaimStatus::NarrowedParity => narrowed += 1,
                GovernanceComponentSurfaceClaimStatus::ParityBlocked => blocked += 1,
            }
            seen_surfaces.insert(row.surface);
            for component in &row.components_present {
                seen_components.insert(*component);
            }
        }
        let all_surfaces_covered = M5GovernanceCertifiedSurface::ALL
            .iter()
            .all(|surface| seen_surfaces.contains(surface));
        let all_components_covered = M5GovernanceComponent::ALL
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
pub struct GovernanceComponentCertificationTrustReview {
    /// Every claimed surface presents the same controlled component truth.
    pub same_component_truth_on_every_surface: bool,
    /// Protection reason and owner source stay explicit, never generic file chrome.
    pub protection_reason_and_owner_source_explicit: bool,
    /// Advisory enforcement stays distinct from provider-authoritative enforcement.
    pub advisory_versus_authoritative_enforcement_distinct: bool,
    /// Approver state and review-pack freshness stay explicit, never flattened.
    pub approver_state_and_review_pack_freshness_explicit: bool,
    /// Public-surface change class and migration evidence stay explicit.
    pub public_surface_change_class_and_migration_evidence_explicit: bool,
    /// Enforcement authority stays explicit; certified never implies provider authority.
    pub certified_never_implies_provider_authoritative_enforcement: bool,
    /// Backup coverage and escalation continuity are preserved, never hidden behind chrome.
    pub backup_coverage_and_escalation_continuity_preserved: bool,
    /// Certification narrows a claim rather than dropping the component's meaning.
    pub narrows_instead_of_dropping_meaning: bool,
    /// A surface that flattens component truth blocks its certification.
    pub flattened_truth_blocks_certification: bool,
    /// Exportable escalation continuity is preserved across desktop, CLI, and export.
    pub exportable_escalation_continuity_preserved: bool,
}

impl GovernanceComponentCertificationTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.same_component_truth_on_every_surface
            && self.protection_reason_and_owner_source_explicit
            && self.advisory_versus_authoritative_enforcement_distinct
            && self.approver_state_and_review_pack_freshness_explicit
            && self.public_surface_change_class_and_migration_evidence_explicit
            && self.certified_never_implies_provider_authoritative_enforcement
            && self.backup_coverage_and_escalation_continuity_preserved
            && self.narrows_instead_of_dropping_meaning
            && self.flattened_truth_blocks_certification
            && self.exportable_escalation_continuity_preserved
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertificationConsumerProjection {
    /// Review workspace surface shows the certified component truth.
    pub review_workspace_surface_shows_certification: bool,
    /// Merge queue surface shows the certified component truth.
    pub merge_queue_surface_shows_certification: bool,
    /// Release center surface shows the certified component truth.
    pub release_center_surface_shows_certification: bool,
    /// Help / About governance surface shows the certified component truth.
    pub help_governance_surface_shows_certification: bool,
    /// Support export shows the certified component truth.
    pub support_export_shows_certification: bool,
    /// Shiproom surface shows the certified component truth.
    pub shiproom_surface_shows_certification: bool,
    /// Exported governance packet shows the certified component truth.
    pub exported_governance_packet_shows_certification: bool,
    /// CLI / headless shows the certified component truth.
    pub cli_headless_shows_certification: bool,
    /// Narrowed surfaces are visibly labelled rather than silently downgraded.
    pub narrowed_surfaces_visibly_labelled: bool,
}

impl GovernanceComponentCertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_surface_shows_certification
            && self.merge_queue_surface_shows_certification
            && self.release_center_surface_shows_certification
            && self.help_governance_surface_shows_certification
            && self.support_export_shows_certification
            && self.shiproom_surface_shows_certification
            && self.exported_governance_packet_shows_certification
            && self.cli_headless_shows_certification
            && self.narrowed_surfaces_visibly_labelled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to [`GovernanceComponentCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceComponentCertObservation {
    /// Surface the observation applies to.
    pub surface: M5GovernanceCertifiedSurface,
    /// True when the surface's governance truth (provider enforcement, owner
    /// coverage, approver state, review-pack freshness, and public-surface diff) is
    /// currently fresh and trusted.
    pub governance_truth_fresh: bool,
    /// True when the surface still preserves component truth.
    pub component_truth_preserved: bool,
}

/// Constructor input for [`GovernanceComponentCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceComponentCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<GovernanceComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: GovernanceComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GovernanceComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: GovernanceComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GovernanceComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe protected-path governance-component surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentCertificationPacket {
    /// Record kind; must equal [`M5_GOVERNANCE_COMPONENT_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<GovernanceComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: GovernanceComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GovernanceComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: GovernanceComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GovernanceComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GovernanceComponentCertificationPacket {
    /// Builds a protected-path governance-component surface certification packet from stable-lane input.
    pub fn new(input: GovernanceComponentCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_GOVERNANCE_COMPONENT_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_VERSION,
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

    /// Narrows surfaces whose governance truth is no longer fresh and blocks surfaces
    /// that flatten component truth, then recomputes the summary.
    ///
    /// This is the downgrade automation: a surface reported with a flattened component
    /// truth blocks (red); a still-green surface whose governance truth (provider
    /// enforcement, owner coverage, approver state, review-pack freshness, and
    /// public-surface diff) went stale narrows its full-governed-authority claim to a
    /// disclosed advisory-enforcement-only ceiling, marks the
    /// enforcement-ownership-provenance axis narrowed, and discloses the
    /// provider-enforcement trigger. Observations for surfaces not present in the
    /// packet are ignored; surfaces without an observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[GovernanceComponentCertObservation],
    ) {
        for row in &mut self.surface_rows {
            let Some(observation) = observations.iter().find(|obs| obs.surface == row.surface)
            else {
                continue;
            };
            if !observation.component_truth_preserved {
                row.component_truth_preserved = false;
            } else if !observation.governance_truth_fresh
                && row.status == GovernanceComponentSurfaceClaimStatus::CertifiedParity
            {
                if row.certified_claim.rank()
                    > GovernanceComponentClaimTier::AdvisoryEnforcementOnly.rank()
                {
                    row.certified_claim = GovernanceComponentClaimTier::AdvisoryEnforcementOnly;
                }
                if !row
                    .narrowed_axes
                    .contains(&GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance)
                {
                    row.narrowed_axes
                        .push(GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance);
                }
                for outcome in &mut row.axis_outcomes {
                    if outcome.axis
                        == GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance
                    {
                        outcome.state =
                            GovernanceComponentAxisCertificationState::NarrowedCertified;
                        outcome.note =
                            "Provider enforcement went stale; the claim narrows to advisory enforcement only and the enforcement-ownership provenance stays explicit"
                                .to_owned();
                    }
                }
                row.downgrade_trigger = Some(
                    GovernanceComponentCertificationDowngradeTrigger::ProviderEnforcementAdvisoryOrStale,
                );
            }
            row.status = row.derived_status();
        }
        self.summary = GovernanceComponentCertificationSummary::from_rows(&self.surface_rows);
    }

    /// Validates the protected-path governance-component surface certification invariants.
    pub fn validate(&self) -> Vec<GovernanceComponentCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GOVERNANCE_COMPONENT_CERTIFICATION_RECORD_KIND {
            violations.push(GovernanceComponentCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_VERSION {
            violations.push(GovernanceComponentCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GovernanceComponentCertificationViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GovernanceComponentCertificationViolation::DowngradeTriggersMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GovernanceComponentCertificationViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(GovernanceComponentCertificationViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GovernanceComponentCertificationViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("governance certification packet serializes"),
        ) {
            violations
                .push(GovernanceComponentCertificationViolation::RawGovernanceMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("governance certification packet serializes")
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
        out.push_str("# Protected-Path Governance-Component Surface Certification\n\n");
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

/// Errors emitted when reading the checked-in governance certification export.
#[derive(Debug)]
pub enum GovernanceComponentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GovernanceComponentCertificationViolation>),
}

impl fmt::Display for GovernanceComponentCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "governance certification export parse failed: {error}"
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
                    "governance certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GovernanceComponentCertificationArtifactError {}

/// Validation failures emitted by [`GovernanceComponentCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceComponentCertificationViolation {
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
    /// A surface flattened the component's protection / owner / enforcement / approver / public-surface truth.
    GovernanceComponentTruthDropped,
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
    /// Export contains raw governance boundary material.
    RawGovernanceMaterialInExport,
}

impl GovernanceComponentCertificationViolation {
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
            Self::GovernanceComponentTruthDropped => "governance_component_truth_dropped",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::SummaryMismatch => "summary_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RawGovernanceMaterialInExport => "raw_governance_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable governance certification export.
pub fn current_governance_component_certification_export(
) -> Result<GovernanceComponentCertificationPacket, GovernanceComponentCertificationArtifactError> {
    let packet: GovernanceComponentCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface/support_export.json"
    )))
    .map_err(GovernanceComponentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GovernanceComponentCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> GovernanceComponentCertificationTrustReview {
    GovernanceComponentCertificationTrustReview {
        same_component_truth_on_every_surface: true,
        protection_reason_and_owner_source_explicit: true,
        advisory_versus_authoritative_enforcement_distinct: true,
        approver_state_and_review_pack_freshness_explicit: true,
        public_surface_change_class_and_migration_evidence_explicit: true,
        certified_never_implies_provider_authoritative_enforcement: true,
        backup_coverage_and_escalation_continuity_preserved: true,
        narrows_instead_of_dropping_meaning: true,
        flattened_truth_blocks_certification: true,
        exportable_escalation_continuity_preserved: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> GovernanceComponentCertificationConsumerProjection {
    GovernanceComponentCertificationConsumerProjection {
        review_workspace_surface_shows_certification: true,
        merge_queue_surface_shows_certification: true,
        release_center_surface_shows_certification: true,
        help_governance_surface_shows_certification: true,
        support_export_shows_certification: true,
        shiproom_surface_shows_certification: true,
        exported_governance_packet_shows_certification: true,
        cli_headless_shows_certification: true,
        narrowed_surfaces_visibly_labelled: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_DOC_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF
            .to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF
            .to_owned(),
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF
            .to_owned(),
    ]
}

fn validate_source_contracts(
    packet: &GovernanceComponentCertificationPacket,
    violations: &mut Vec<GovernanceComponentCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_DOC_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_CERTIFICATION_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GovernanceComponentCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &GovernanceComponentCertificationPacket,
    violations: &mut Vec<GovernanceComponentCertificationViolation>,
) {
    if packet.surface_rows.is_empty() {
        violations.push(GovernanceComponentCertificationViolation::SurfaceRowsMissing);
        return;
    }

    let mut seen_surfaces: BTreeSet<M5GovernanceCertifiedSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5GovernanceComponent> = BTreeSet::new();

    for row in &packet.surface_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(GovernanceComponentCertificationViolation::RowIncomplete);
        }
        if row.components_present.is_empty() {
            violations.push(GovernanceComponentCertificationViolation::ComponentsMissingOnRow);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(GovernanceComponentCertificationViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(GovernanceComponentCertificationViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(GovernanceComponentCertificationViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(GovernanceComponentCertificationViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(GovernanceComponentCertificationViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_axes() {
            violations.push(GovernanceComponentCertificationViolation::AxisCoverageMissing);
        }
        if row
            .axis_outcomes
            .iter()
            .any(|outcome| outcome.note.trim().is_empty())
        {
            violations.push(GovernanceComponentCertificationViolation::AxisNoteMissing);
        }

        // A certified claim may never exceed the claim it certifies.
        if !row.certified_claim_within_claimed() {
            violations
                .push(GovernanceComponentCertificationViolation::CertifiedClaimExceedsClaimed);
        }

        if !row.narrowed_axes_consistent() {
            violations.push(GovernanceComponentCertificationViolation::NarrowedAxesInconsistent);
        }

        // A narrowed surface must disclose its downgrade trigger.
        if !row.narrowed_axes.is_empty() && row.downgrade_trigger.is_none() {
            violations.push(GovernanceComponentCertificationViolation::NarrowingWithoutTrigger);
        }

        // Delta: certification may narrow a claim but never drop component truth.
        if !row.component_truth_preserved {
            violations
                .push(GovernanceComponentCertificationViolation::GovernanceComponentTruthDropped);
        }

        // The recorded status must agree with the derived one.
        if !row.status_is_consistent() {
            violations.push(GovernanceComponentCertificationViolation::StatusMismatch);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(GovernanceComponentCertificationViolation::CanonicalContractReferenceMissing);
        }

        seen_surfaces.insert(row.surface);
        for component in &row.components_present {
            seen_components.insert(*component);
        }
    }

    for surface in M5GovernanceCertifiedSurface::ALL {
        if !seen_surfaces.contains(&surface) {
            violations.push(GovernanceComponentCertificationViolation::SurfaceCoverageMissing);
            break;
        }
    }
    for component in M5GovernanceComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GovernanceComponentCertificationViolation::ComponentCoverageMissing);
            break;
        }
    }
}

fn validate_summary(
    packet: &GovernanceComponentCertificationPacket,
    violations: &mut Vec<GovernanceComponentCertificationViolation>,
) {
    let recomputed = GovernanceComponentCertificationSummary::from_rows(&packet.surface_rows);
    if recomputed != packet.summary {
        violations.push(GovernanceComponentCertificationViolation::SummaryMismatch);
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
