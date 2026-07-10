//! Shared review-workspace / merge-queue / release-center / Help / support /
//! shiproom / CLI-export consumers that keep the eight reusable protected-path
//! governance components at owner-coverage, approver, and public-surface parity
//! across every claimed M5 profile.
//!
//! This module is the consumer-adoption lane for the protected-path governance
//! components frozen in
//! [`crate::freeze_the_m5_protected_path_governance_component_matrix`] and
//! implemented by the protected-path-row / ownership-card, approver-matrix /
//! review-pack-summary, public-surface-diff-card / merge-control-banner, and
//! DRI-registry-row / merge-readiness-strip lanes. It binds each shared component to
//! the review workspace, merge queue, release center, Help surface, support packet,
//! shiproom summary, and CLI/export payload that render it, and proves — by fixtures,
//! not screenshots — that the same governed change presents the same owner-coverage,
//! approver-state, public-surface-impact, and merge-blocker language wherever it
//! appears.
//!
//! The core honesty axes are two. First, parity: for a given governed change, every
//! consumer surface must present identical parity facet values — the same
//! owner-coverage label, the same required-approver state language, the same
//! public-surface-impact language, and the same merge-blocker language. A surface may
//! narrow how much it shows when enforcement, coverage, approver, public-surface, or
//! proof evidence degrades, but it may never reword the underlying language per
//! surface, let an advisory owner hint read as provider-authoritative enforcement,
//! hide missing backup coverage or an expired approver state behind a guarded merge,
//! or let a public-surface change land without a machine-generated diff and its
//! migration / evidence context. Second, disclosure: when a surface narrows, it must
//! do so through an explicit narrow banner that names the reason, the preserved
//! facets, and the next action — the enforcement-authority and evidence-continuity
//! notes stay explicit rather than collapsing the governed change out of view.
//!
//! Component reuse is proven rather than inferred: every one of the eight shared
//! components must be adopted by at least two distinct consumers, and Help, support,
//! and CLI/export consumers must point at the canonical component contracts by id.
//! The frozen governance-state vocabulary is reused directly from the matrix
//! ([`M5GovernanceStateVocab`]) and the component identity from
//! [`M5GovernanceComponent`], so a downgrade trigger, an owner-coverage state, and an
//! enforcement-authority distinction read the same everywhere.
//!
//! The packet references upstream component contracts by id rather than embedding
//! their content. Raw diff payloads, raw CODEOWNERS bodies, live provider responses,
//! and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-protected-path-governance-component-consumer.schema.json`](../../../../schemas/ui/m5-protected-path-governance-component-consumer.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned.md`](../../../../docs/review/m5/add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-protected-path-governance-component-consumers/`](../../../../fixtures/ui/m5-protected-path-governance-component-consumers/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_protected_path_governance_component_matrix::{
    M5GovernanceComponent, M5GovernanceComponentDowngradeTrigger, M5GovernanceStateVocab,
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`GovernanceComponentConsumerPacket`].
pub const GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "governance_component_consumer_parity_truth";

/// Schema version for governance-component consumer parity records.
pub const GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const GOVERNANCE_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/review/m5/add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned.md";

/// Repo-relative path of the frozen component matrix these consumers adopt.
pub const GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF: &str =
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected-path-row / ownership-card controls contract.
pub const GOVERNANCE_COMPONENT_CONSUMER_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-ownership-controls.schema.json";

/// Repo-relative path of the approver-matrix / review-pack-summary controls contract.
pub const GOVERNANCE_COMPONENT_CONSUMER_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-approver-review-pack-controls.schema.json";

/// Repo-relative path of the public-surface-diff-card / merge-control-banner controls contract.
pub const GOVERNANCE_COMPONENT_CONSUMER_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json";

/// Repo-relative path of the DRI-registry-row / merge-readiness-strip controls contract.
pub const GOVERNANCE_COMPONENT_CONSUMER_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GOVERNANCE_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-protected-path-governance-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const GOVERNANCE_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-protected-path-governance-consumers-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GOVERNANCE_COMPONENT_CONSUMER_SUMMARY_REF: &str =
    "artifacts/release/m5-protected-path-governance-consumers-proof/summary.md";

/// Canonical control-schema contract a consumer must point at for a given component.
///
/// Each of the eight shared components resolves to the checked-in controls schema of
/// the implement lane that produced it: the protected-path-row / ownership-card
/// controls, the approver-matrix / review-pack-summary controls, the
/// public-surface-diff-card / merge-control-banner controls, and the
/// DRI-registry-row / merge-readiness-strip controls.
pub const fn component_canonical_control_schema_ref(
    component: M5GovernanceComponent,
) -> &'static str {
    match component {
        M5GovernanceComponent::ProtectedPathRow | M5GovernanceComponent::OwnershipCard => {
            GOVERNANCE_COMPONENT_CONSUMER_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::ApproverMatrix | M5GovernanceComponent::ReviewPackSummary => {
            GOVERNANCE_COMPONENT_CONSUMER_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::PublicSurfaceDiffCard
        | M5GovernanceComponent::MergeControlBanner => {
            GOVERNANCE_COMPONENT_CONSUMER_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::DriRegistryRow | M5GovernanceComponent::MergeReadinessStrip => {
            GOVERNANCE_COMPONENT_CONSUMER_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF
        }
    }
}

/// Consumer surface that must reuse the shared governance components at parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentConsumer {
    /// Review workspace list / detail surface.
    ReviewWorkspace,
    /// Merge queue surface.
    MergeQueue,
    /// Release-center packet surface.
    ReleaseCenter,
    /// Help / About surface.
    HelpSurface,
    /// Support packet.
    SupportPacket,
    /// Shiproom / escalation summary.
    Shiproom,
    /// CLI / headless export payload.
    CliExport,
}

impl GovernanceComponentConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewWorkspace,
        Self::MergeQueue,
        Self::ReleaseCenter,
        Self::HelpSurface,
        Self::SupportPacket,
        Self::Shiproom,
        Self::CliExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::MergeQueue => "merge_queue",
            Self::ReleaseCenter => "release_center",
            Self::HelpSurface => "help_surface",
            Self::SupportPacket => "support_packet",
            Self::Shiproom => "shiproom",
            Self::CliExport => "cli_export",
        }
    }

    /// Whether this consumer is a Help, support, or CLI/export surface that must
    /// point at the canonical component contracts by id.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(
            self,
            Self::HelpSurface | Self::SupportPacket | Self::CliExport
        )
    }
}

/// A parity facet whose value must stay identical across surfaces for one change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentParityFacet {
    /// Who owns the change (owner source and coverage).
    OwnerCoverageLabel,
    /// The required-approver state language.
    ApproverStateLabel,
    /// Whether public-surface impact exists and its change class.
    PublicSurfaceImpactLabel,
    /// What is blocked (the merge-control blocker language).
    MergeBlockerLabel,
}

impl GovernanceComponentParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OwnerCoverageLabel,
        Self::ApproverStateLabel,
        Self::PublicSurfaceImpactLabel,
        Self::MergeBlockerLabel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerCoverageLabel => "owner_coverage_label",
            Self::ApproverStateLabel => "approver_state_label",
            Self::PublicSurfaceImpactLabel => "public_surface_impact_label",
            Self::MergeBlockerLabel => "merge_blocker_label",
        }
    }
}

/// The governance evidence / enforcement state that drives a surface's narrowing.
///
/// This is the honesty axis the acceptance criteria pins: consumers must degrade the
/// same way when evidence or enforcement state is stale, and never let an advisory
/// hint read as provider-authoritative enforcement, hide missing backup coverage or
/// an expired approver state, or let a public-surface change land without a
/// machine-generated diff and migration / evidence context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentEvidenceState {
    /// Enforcement is provider-authoritative and every governance evidence is fresh.
    ProviderAuthoritativeFresh,
    /// Enforcement is advisory or a local estimate, not provider-confirmed.
    EnforcementAdvisoryOrLocalEstimate,
    /// Owner backup coverage is missing for the guarded change.
    OwnerBackupCoverageMissing,
    /// A required approver's state is expired or waived.
    ApproverStateExpiredOrWaived,
    /// A machine-generated public-surface diff or its migration / evidence context is missing.
    PublicSurfaceEvidenceMissing,
    /// Provider-backed proof is stale relative to the change it gates.
    ProofStaleRelativeToChange,
}

impl GovernanceComponentEvidenceState {
    /// Every evidence state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderAuthoritativeFresh,
        Self::EnforcementAdvisoryOrLocalEstimate,
        Self::OwnerBackupCoverageMissing,
        Self::ApproverStateExpiredOrWaived,
        Self::PublicSurfaceEvidenceMissing,
        Self::ProofStaleRelativeToChange,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritativeFresh => "provider_authoritative_fresh",
            Self::EnforcementAdvisoryOrLocalEstimate => "enforcement_advisory_or_local_estimate",
            Self::OwnerBackupCoverageMissing => "owner_backup_coverage_missing",
            Self::ApproverStateExpiredOrWaived => "approver_state_expired_or_waived",
            Self::PublicSurfaceEvidenceMissing => "public_surface_evidence_missing",
            Self::ProofStaleRelativeToChange => "proof_stale_relative_to_change",
        }
    }
}

/// How much of a shared component a consumer renders.
///
/// Narrowing changes how much is shown, never the underlying parity language: a
/// narrowed surface still carries the same owner-coverage, approver-state,
/// public-surface-impact, and merge-blocker language, and discloses the narrowing
/// through an explicit banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentProjectionMode {
    /// Full parity; every governance facet renders faithfully.
    FullParity,
    /// Enforcement authority is narrowed to advisory / local estimate.
    EnforcementNarrowed,
    /// Owner-coverage is narrowed; backup coverage is missing.
    CoverageNarrowed,
    /// Approver state is narrowed; a required approval is expired or waived.
    ApprovalNarrowed,
    /// Public-surface impact is narrowed; diff or migration / evidence is missing.
    PublicSurfaceNarrowed,
    /// Provider-backed proof is stale relative to the change.
    StaleNarrowed,
}

impl GovernanceComponentProjectionMode {
    /// Every projection mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullParity,
        Self::EnforcementNarrowed,
        Self::CoverageNarrowed,
        Self::ApprovalNarrowed,
        Self::PublicSurfaceNarrowed,
        Self::StaleNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::EnforcementNarrowed => "enforcement_narrowed",
            Self::CoverageNarrowed => "coverage_narrowed",
            Self::ApprovalNarrowed => "approval_narrowed",
            Self::PublicSurfaceNarrowed => "public_surface_narrowed",
            Self::StaleNarrowed => "stale_narrowed",
        }
    }

    /// Whether this mode narrows below full parity.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentNarrowReason {
    /// Enforcement is advisory or a local estimate, not provider-authoritative.
    EnforcementAdvisoryOrLocalEstimate,
    /// Owner backup coverage is missing for the guarded change.
    OwnerBackupCoverageMissing,
    /// A required approver's state is expired or waived.
    ApproverStateExpiredOrWaived,
    /// A machine-generated public-surface diff or migration / evidence context is missing.
    PublicSurfaceDiffOrMigrationMissing,
    /// Provider-backed proof is stale relative to the change.
    ProofStaleRelativeToChange,
}

impl GovernanceComponentNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnforcementAdvisoryOrLocalEstimate => "enforcement_advisory_or_local_estimate",
            Self::OwnerBackupCoverageMissing => "owner_backup_coverage_missing",
            Self::ApproverStateExpiredOrWaived => "approver_state_expired_or_waived",
            Self::PublicSurfaceDiffOrMigrationMissing => "public_surface_diff_or_migration_missing",
            Self::ProofStaleRelativeToChange => "proof_stale_relative_to_change",
        }
    }
}

/// The next action a narrow banner offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentNarrowNextAction {
    /// Review the enforcement authority (advisory versus provider-authoritative).
    ReviewEnforcementAuthority,
    /// Review the owner-coverage / backup state.
    ReviewOwnerCoverage,
    /// Review the required-approver state.
    ReviewApproverState,
    /// Review the public-surface diff / migration evidence.
    ReviewPublicSurfaceEvidence,
    /// Refresh the provider-backed proof.
    RefreshProof,
}

impl GovernanceComponentNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewEnforcementAuthority => "review_enforcement_authority",
            Self::ReviewOwnerCoverage => "review_owner_coverage",
            Self::ReviewApproverState => "review_approver_state",
            Self::ReviewPublicSurfaceEvidence => "review_public_surface_evidence",
            Self::RefreshProof => "refresh_proof",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentParityState {
    /// All parity facets are preserved and shown in full.
    FacetsPreserved,
    /// All parity facets are preserved, and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl GovernanceComponentParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// The parity facet values a shared component presents for one governed change.
///
/// These four values must be identical across every consumer surface that shows the
/// same governed change. A surface may narrow how much it renders, but it may never
/// reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentParityFacetValues {
    /// Who owns the change (never reworded per surface).
    pub owner_coverage_label: String,
    /// Required-approver state language (identical across surfaces).
    pub approver_state_label: String,
    /// Public-surface-impact language (identical across surfaces).
    pub public_surface_impact_label: String,
    /// Merge-blocker language (identical across surfaces).
    pub merge_blocker_label: String,
}

impl GovernanceComponentParityFacetValues {
    /// Whether every parity facet value is present.
    pub fn all_present(&self) -> bool {
        !self.owner_coverage_label.trim().is_empty()
            && !self.approver_state_label.trim().is_empty()
            && !self.public_surface_impact_label.trim().is_empty()
            && !self.merge_blocker_label.trim().is_empty()
    }
}

/// The explicit banner a narrowed surface shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentNarrowBanner {
    /// Why the surface narrowed.
    pub reason: GovernanceComponentNarrowReason,
    /// Note naming the preserved parity facets (never omitted).
    pub preserved_facets_note: String,
    /// The next action offered.
    pub next_action: GovernanceComponentNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its evidence state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GovernanceComponentDisclosure {
    /// The projection mode the evidence state requires.
    pub expected_mode: GovernanceComponentProjectionMode,
    /// The narrow reason the projection mode requires, if any.
    pub narrow_reason: Option<GovernanceComponentNarrowReason>,
    /// Whether the binding must carry an explicit narrow banner.
    pub needs_narrow_banner: bool,
    /// Whether the binding must carry an explicit enforcement-authority note.
    pub needs_enforcement_authority_note: bool,
    /// Whether the binding must carry an explicit evidence-continuity note.
    pub needs_evidence_continuity_note: bool,
}

/// Resolves the disclosures a consumer binding must carry from its evidence state.
///
/// Provider-authoritative fresh evidence renders at full parity. Advisory or
/// local-estimate enforcement narrows through an enforcement-authority note so an
/// advisory hint can never read as provider-authoritative. Missing backup coverage,
/// an expired or waived approver, and missing public-surface diff / migration
/// evidence each narrow through an evidence-continuity note that keeps who-owns,
/// what-is-blocked, and whether-public-impact-exists explicit. Stale provider proof
/// narrows through an enforcement-authority note because the provider-backed value is
/// no longer authoritative. In every case the governed change stays visible with its
/// owner-coverage, approver, and public-surface language intact.
pub fn resolve_governance_component_disclosure(
    evidence: GovernanceComponentEvidenceState,
) -> GovernanceComponentDisclosure {
    let (expected_mode, narrow_reason) = match evidence {
        GovernanceComponentEvidenceState::ProviderAuthoritativeFresh => {
            (GovernanceComponentProjectionMode::FullParity, None)
        }
        GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate => (
            GovernanceComponentProjectionMode::EnforcementNarrowed,
            Some(GovernanceComponentNarrowReason::EnforcementAdvisoryOrLocalEstimate),
        ),
        GovernanceComponentEvidenceState::OwnerBackupCoverageMissing => (
            GovernanceComponentProjectionMode::CoverageNarrowed,
            Some(GovernanceComponentNarrowReason::OwnerBackupCoverageMissing),
        ),
        GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived => (
            GovernanceComponentProjectionMode::ApprovalNarrowed,
            Some(GovernanceComponentNarrowReason::ApproverStateExpiredOrWaived),
        ),
        GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing => (
            GovernanceComponentProjectionMode::PublicSurfaceNarrowed,
            Some(GovernanceComponentNarrowReason::PublicSurfaceDiffOrMigrationMissing),
        ),
        GovernanceComponentEvidenceState::ProofStaleRelativeToChange => (
            GovernanceComponentProjectionMode::StaleNarrowed,
            Some(GovernanceComponentNarrowReason::ProofStaleRelativeToChange),
        ),
    };

    // Enforcement authority must stay explicit whenever enforcement narrows to
    // advisory / local estimate, or provider-backed proof goes stale (spec guardrail:
    // an advisory hint never reads as provider-authoritative enforcement).
    let needs_enforcement_authority_note = matches!(
        evidence,
        GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate
            | GovernanceComponentEvidenceState::ProofStaleRelativeToChange
    );
    // The who-owns / what-is-blocked / whether-public-impact-exists evidence stays
    // explicit whenever coverage, approver, or public-surface evidence narrows.
    let needs_evidence_continuity_note = matches!(
        evidence,
        GovernanceComponentEvidenceState::OwnerBackupCoverageMissing
            | GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived
            | GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing
    );

    GovernanceComponentDisclosure {
        expected_mode,
        narrow_reason,
        needs_narrow_banner: expected_mode.is_narrowed(),
        needs_enforcement_authority_note,
        needs_evidence_continuity_note,
    }
}

/// The parity state a projection mode requires.
pub const fn parity_state_for_mode(
    mode: GovernanceComponentProjectionMode,
) -> GovernanceComponentParityState {
    match mode {
        GovernanceComponentProjectionMode::FullParity => {
            GovernanceComponentParityState::FacetsPreserved
        }
        GovernanceComponentProjectionMode::EnforcementNarrowed
        | GovernanceComponentProjectionMode::CoverageNarrowed
        | GovernanceComponentProjectionMode::ApprovalNarrowed
        | GovernanceComponentProjectionMode::PublicSurfaceNarrowed
        | GovernanceComponentProjectionMode::StaleNarrowed => {
            GovernanceComponentParityState::FacetsDisclosedNarrowed
        }
    }
}

/// One consumer binding: a shared component rendered on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable governed-change id (shared across surfaces that show the same change).
    pub governed_change_id: String,
    /// Human-readable governed-change identity.
    pub governed_change_label: String,
    /// Which shared component this binding renders.
    pub component: M5GovernanceComponent,
    /// Which consumer surface renders it.
    pub consumer: GovernanceComponentConsumer,
    /// Governance evidence / enforcement state that drives narrowing.
    pub evidence_state: GovernanceComponentEvidenceState,
    /// How much of the component this surface renders.
    pub projection_mode: GovernanceComponentProjectionMode,
    /// The parity facet values presented (identical across surfaces for one change).
    pub parity_facets: GovernanceComponentParityFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: GovernanceComponentParityState,
    /// The explicit narrow banner; required and complete when the binding narrows.
    pub narrow_banner: Option<GovernanceComponentNarrowBanner>,
    /// Enforcement-authority note; required and non-empty when the disclosure demands it.
    pub enforcement_authority_note: String,
    /// Evidence-continuity note; required and non-empty when the disclosure demands it.
    pub evidence_continuity_note: String,
    /// Frozen governance-state vocabulary tokens this binding reuses (never empty).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Guardrail: an advisory owner hint reads as provider-authoritative enforcement.
    pub advisory_owner_reads_as_provider_authoritative: bool,
    /// Guardrail: a guarded merge hides missing backup coverage.
    pub guarded_merge_hides_missing_backup_coverage: bool,
    /// Guardrail: a guarded merge hides an expired approver state.
    pub guarded_merge_hides_expired_approver_state: bool,
    /// Guardrail: a public-surface change hides its diff or migration / evidence context.
    pub public_surface_change_hides_diff_or_migration_evidence: bool,
    /// Guardrail: this surface rewords the parity labels per surface.
    pub rewords_governance_labels_per_surface: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl GovernanceComponentConsumerBinding {
    /// Disclosures this binding must carry, derived from its evidence state.
    pub fn disclosure(&self) -> GovernanceComponentDisclosure {
        resolve_governance_component_disclosure(self.evidence_state)
    }

    /// Whether this binding renders below full parity.
    pub fn is_narrowed(&self) -> bool {
        self.projection_mode.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub fn guardrails_hold(&self) -> bool {
        !self.advisory_owner_reads_as_provider_authoritative
            && !self.guarded_merge_hides_missing_backup_coverage
            && !self.guarded_merge_hides_expired_approver_state
            && !self.public_surface_change_hides_diff_or_migration_evidence
            && !self.rewords_governance_labels_per_surface
    }

    /// Whether this binding points at the canonical component controls and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_control_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF
            })
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentConsumerTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same governed change presents the same language across surfaces.
    pub same_change_same_language_across_surfaces: bool,
    /// An advisory owner hint never reads as provider-authoritative enforcement.
    pub advisory_never_reads_as_provider_authoritative: bool,
    /// A guarded merge never hides missing backup coverage.
    pub guarded_merge_never_hides_missing_backup_coverage: bool,
    /// A guarded merge never hides an expired approver state.
    pub guarded_merge_never_hides_expired_approver_state: bool,
    /// A public-surface change never hides its diff or migration / evidence context.
    pub public_surface_change_never_hides_diff_or_migration_evidence: bool,
    /// Owner-coverage labels are identical across surfaces.
    pub owner_coverage_labels_identical_across_surfaces: bool,
    /// Required-approver state language is identical across surfaces.
    pub approver_state_language_identical_across_surfaces: bool,
    /// Public-surface-impact language is identical across surfaces.
    pub public_surface_impact_language_identical_across_surfaces: bool,
    /// Help, support, and CLI/export consumers point at the canonical contracts.
    pub help_support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl GovernanceComponentConsumerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_change_same_language_across_surfaces
            && self.advisory_never_reads_as_provider_authoritative
            && self.guarded_merge_never_hides_missing_backup_coverage
            && self.guarded_merge_never_hides_expired_approver_state
            && self.public_surface_change_never_hides_diff_or_migration_evidence
            && self.owner_coverage_labels_identical_across_surfaces
            && self.approver_state_language_identical_across_surfaces
            && self.public_surface_impact_language_identical_across_surfaces
            && self.help_support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentConsumerProjection {
    /// The review workspace reuses the shared components.
    pub review_workspace_reuses_shared_components: bool,
    /// The merge queue reuses the shared components.
    pub merge_queue_reuses_shared_components: bool,
    /// The release center reuses the shared components.
    pub release_center_reuses_shared_components: bool,
    /// The Help surface reuses the shared components.
    pub help_surface_reuses_shared_components: bool,
    /// The support packet reuses the shared components.
    pub support_packet_reuses_shared_components: bool,
    /// The shiproom summary reuses the shared components.
    pub shiproom_reuses_shared_components: bool,
    /// The CLI/export payload reuses the shared components.
    pub cli_export_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Parity facets are identical for the same governed change.
    pub parity_facets_identical_for_same_change: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
}

impl GovernanceComponentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reuses_shared_components
            && self.merge_queue_reuses_shared_components
            && self.release_center_reuses_shared_components
            && self.help_surface_reuses_shared_components
            && self.support_packet_reuses_shared_components
            && self.shiproom_reuses_shared_components
            && self.cli_export_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.parity_facets_identical_for_same_change
            && self.narrowing_disclosed_not_hidden
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GovernanceComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<GovernanceComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<GovernanceComponentConsumer>,
    /// Trust review block.
    pub trust_review: GovernanceComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GovernanceComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe governance-component consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentConsumerPacket {
    /// Record kind; must equal [`GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<GovernanceComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<GovernanceComponentConsumer>,
    /// Trust review block.
    pub trust_review: GovernanceComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: GovernanceComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GovernanceComponentConsumerPacket {
    /// Builds a governance-component consumer packet from stable-lane input.
    pub fn new(input: GovernanceComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the governance-component consumer parity invariants.
    pub fn validate(&self) -> Vec<GovernanceComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(GovernanceComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(GovernanceComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GovernanceComponentConsumerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GovernanceComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GovernanceComponentConsumerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GovernanceComponentConsumerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GovernanceComponentConsumerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GovernanceComponentConsumerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("governance-component consumer packet serializes"),
        ) {
            violations.push(GovernanceComponentConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("governance-component consumer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Protected-Path Governance Component Consumers: Owner, Approver, and Public-Surface Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, mode `{}`\n",
                binding.governed_change_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.projection_mode.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in governance consumer export.
#[derive(Debug)]
pub enum GovernanceComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GovernanceComponentConsumerViolation>),
}

impl fmt::Display for GovernanceComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "governance-component consumer export parse failed: {error}"
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
                    "governance-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GovernanceComponentConsumerArtifactError {}

/// Validation failures emitted by [`GovernanceComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's parity facet values are incomplete.
    ParityFacetIncomplete,
    /// A binding does not reuse any frozen governance-state vocabulary token.
    GovernanceStateVocabMissing,
    /// A binding's projection mode does not match its evidence state.
    ProjectionModeMismatch,
    /// A binding's parity state does not match its projection mode.
    ParityStateMismatch,
    /// Two surfaces show the same governed change with different parity language.
    ParityDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    GovernanceComponentReuseUnproven,
    /// A Help, support, or CLI/export binding does not point at the canonical contracts.
    HelpSupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow banner.
    NarrowBannerMissing,
    /// A narrow banner's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow banner is missing its preserved-facets note.
    NarrowBannerPreservedFacetsMissing,
    /// A narrow banner is missing its next-action copy.
    NarrowNextActionMissing,
    /// A binding that must keep an enforcement-authority note is missing it.
    EnforcementAuthorityNoteMissing,
    /// A binding that needs an explicit evidence-continuity note is missing it.
    EvidenceContinuityNoteMissing,
    /// A binding lets an advisory owner hint read as provider-authoritative enforcement.
    AdvisoryOwnerReadsAsProviderAuthoritative,
    /// A binding lets a guarded merge hide missing backup coverage.
    GuardedMergeHidesMissingBackupCoverage,
    /// A binding lets a guarded merge hide an expired approver state.
    GuardedMergeHidesExpiredApproverState,
    /// A binding lets a public-surface change hide its diff or migration / evidence context.
    PublicSurfaceChangeHidesDiffOrMigrationEvidence,
    /// A binding rewords the parity labels per surface.
    GovernanceLabelsRewordedPerSurface,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GovernanceComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ParityFacetIncomplete => "parity_facet_incomplete",
            Self::GovernanceStateVocabMissing => "governance_state_vocab_missing",
            Self::ProjectionModeMismatch => "projection_mode_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ParityDriftAcrossSurfaces => "parity_drift_across_surfaces",
            Self::GovernanceComponentReuseUnproven => "governance_component_reuse_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::NarrowBannerMissing => "narrow_banner_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowBannerPreservedFacetsMissing => "narrow_banner_preserved_facets_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::EnforcementAuthorityNoteMissing => "enforcement_authority_note_missing",
            Self::EvidenceContinuityNoteMissing => "evidence_continuity_note_missing",
            Self::AdvisoryOwnerReadsAsProviderAuthoritative => {
                "advisory_owner_reads_as_provider_authoritative"
            }
            Self::GuardedMergeHidesMissingBackupCoverage => {
                "guarded_merge_hides_missing_backup_coverage"
            }
            Self::GuardedMergeHidesExpiredApproverState => {
                "guarded_merge_hides_expired_approver_state"
            }
            Self::PublicSurfaceChangeHidesDiffOrMigrationEvidence => {
                "public_surface_change_hides_diff_or_migration_evidence"
            }
            Self::GovernanceLabelsRewordedPerSurface => "governance_labels_reworded_per_surface",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable governance consumer export.
pub fn current_governance_component_consumer_export(
) -> Result<GovernanceComponentConsumerPacket, GovernanceComponentConsumerArtifactError> {
    let packet: GovernanceComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-protected-path-governance-consumers-proof/support_export.json"
    )))
    .map_err(GovernanceComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GovernanceComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GovernanceComponentConsumerPacket,
    violations: &mut Vec<GovernanceComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_REF,
        GOVERNANCE_COMPONENT_CONSUMER_DOC_REF,
        GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF,
        GOVERNANCE_COMPONENT_CONSUMER_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_CONSUMER_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_CONSUMER_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_CONSUMER_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GovernanceComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &GovernanceComponentConsumerPacket,
    violations: &mut Vec<GovernanceComponentConsumerViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(GovernanceComponentConsumerViolation::ConsumerBindingsMissing);
        return;
    }

    // Parity: the parity facet values must be identical for every binding that
    // renders the same governed change.
    let mut change_facets: BTreeMap<&str, &GovernanceComponentParityFacetValues> = BTreeMap::new();
    let mut parity_drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5GovernanceComponent,
        BTreeSet<GovernanceComponentConsumer>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<GovernanceComponentConsumer> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5GovernanceComponent> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.governed_change_id.trim().is_empty()
            || binding.governed_change_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(GovernanceComponentConsumerViolation::BindingIncomplete);
        }
        if !binding.parity_facets.all_present() {
            violations.push(GovernanceComponentConsumerViolation::ParityFacetIncomplete);
        }
        if binding.governance_state_vocab.is_empty() {
            violations.push(GovernanceComponentConsumerViolation::GovernanceStateVocabMissing);
        }

        let disclosure = binding.disclosure();

        if binding.projection_mode != disclosure.expected_mode {
            violations.push(GovernanceComponentConsumerViolation::ProjectionModeMismatch);
        }
        if binding.parity_state != parity_state_for_mode(binding.projection_mode) {
            violations.push(GovernanceComponentConsumerViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_banner {
            match &binding.narrow_banner {
                None => {
                    violations.push(GovernanceComponentConsumerViolation::NarrowBannerMissing);
                }
                Some(banner) => {
                    if Some(banner.reason) != disclosure.narrow_reason {
                        violations.push(GovernanceComponentConsumerViolation::NarrowReasonMismatch);
                    }
                    if banner.preserved_facets_note.trim().is_empty() {
                        violations.push(
                            GovernanceComponentConsumerViolation::NarrowBannerPreservedFacetsMissing,
                        );
                    }
                    if banner.next_action_label.trim().is_empty() {
                        violations
                            .push(GovernanceComponentConsumerViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if binding.narrow_banner.is_some() {
            // A full-parity binding must not carry a narrow banner.
            violations.push(GovernanceComponentConsumerViolation::NarrowBannerMissing);
        }

        if disclosure.needs_enforcement_authority_note
            && binding.enforcement_authority_note.trim().is_empty()
        {
            violations.push(GovernanceComponentConsumerViolation::EnforcementAuthorityNoteMissing);
        }
        if disclosure.needs_evidence_continuity_note
            && binding.evidence_continuity_note.trim().is_empty()
        {
            violations.push(GovernanceComponentConsumerViolation::EvidenceContinuityNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.advisory_owner_reads_as_provider_authoritative {
            violations.push(
                GovernanceComponentConsumerViolation::AdvisoryOwnerReadsAsProviderAuthoritative,
            );
        }
        if binding.guarded_merge_hides_missing_backup_coverage {
            violations
                .push(GovernanceComponentConsumerViolation::GuardedMergeHidesMissingBackupCoverage);
        }
        if binding.guarded_merge_hides_expired_approver_state {
            violations
                .push(GovernanceComponentConsumerViolation::GuardedMergeHidesExpiredApproverState);
        }
        if binding.public_surface_change_hides_diff_or_migration_evidence {
            violations.push(
                GovernanceComponentConsumerViolation::PublicSurfaceChangeHidesDiffOrMigrationEvidence,
            );
        }
        if binding.rewords_governance_labels_per_surface {
            violations
                .push(GovernanceComponentConsumerViolation::GovernanceLabelsRewordedPerSurface);
        }

        // Help / support / export consumers must point at the canonical contracts.
        if binding.consumer.is_help_support_or_export() && !binding.points_at_canonical_contracts()
        {
            violations
                .push(GovernanceComponentConsumerViolation::HelpSupportExportReferenceMissing);
        }

        // Parity drift accumulation.
        match change_facets.get(binding.governed_change_id.as_str()) {
            None => {
                change_facets.insert(binding.governed_change_id.as_str(), &binding.parity_facets);
            }
            Some(existing) => {
                if **existing != binding.parity_facets && !parity_drift_reported {
                    violations
                        .push(GovernanceComponentConsumerViolation::ParityDriftAcrossSurfaces);
                    parity_drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer and every component must appear.
    for consumer in GovernanceComponentConsumer::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(GovernanceComponentConsumerViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5GovernanceComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GovernanceComponentConsumerViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(GovernanceComponentConsumerViolation::GovernanceComponentReuseUnproven);
            break;
        }
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
