//! Keyboard, screen-reader, CLI, and export parity plus automatic claim narrowing
//! for the eight shared M5 protected-path governance components.
//!
//! This module is the accessibility / headless / export capstone over the governance
//! components frozen in
//! [`crate::freeze_the_m5_protected_path_governance_component_matrix`], implemented by the
//! protected-path / ownership, approver-matrix / review-pack, public-surface-diff /
//! merge-control, and DRI-registry / merge-readiness lanes, and adopted by the shared
//! consumers in
//! [`crate::add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned`].
//! Where the consumer lane proves owner / approver / public-surface parity across desktop
//! surfaces, this lane proves the harder claim: that protection reason, owner source,
//! advisory-versus-authoritative enforcement, approver state, review-pack freshness, and
//! public-surface change class are exposed just as honestly in assistive, headless, and
//! exported forms as they are on the desktop — and that a claim-bearing component
//! automatically narrows the moment its provider enforcement, owner coverage, approver
//! state, review-pack freshness, or public-surface diff truth stops being trustworthy.
//!
//! The honesty axes are two. First, parity across forms: every claimed component must
//! expose a keyboard label, a screen-reader label, a CLI enum token, an export enum
//! token, and a human-readable explanation field, and must render on the desktop, the
//! headless CLI, and the support export alike. No component may be pointer-only,
//! export-opaque, semantically stronger on the desktop than it is in CLI or export, or
//! collapsed to a vague `governed` label that drops the owner / approver / public-surface
//! semantics the GUI shows.
//!
//! Second, automatic narrowing: each component carries a claim about how much governed
//! authority it asserts, drawn from [`GovernanceComponentClaimTier`]. When provider
//! enforcement is advisory, stale, or a local estimate; when owner backup coverage is
//! missing; when approver state is waived or expired; when the review pack is stale
//! relative to the change; or when the public-surface diff or migration evidence is
//! partial, the claim must narrow to the ceiling permitted by that condition
//! ([`GovernanceComponentClaimCondition::permitted_ceiling`]), disclose the narrowing
//! through an explicit trigger and next action, keep the explicit owner / approver /
//! public-surface semantics, never promote an advisory owner hint to provider-authoritative
//! enforcement, never let a guarded merge hide missing backup coverage or expired approver
//! state, and never let a public-surface change read clean without its machine-generated
//! diff and migration evidence. A component may never keep asserting full governed
//! authority while one of those conditions holds.
//!
//! The packet references upstream component and consumer contracts by id rather than
//! embedding their content. Raw provider responses, credentials, and CODEOWNERS payloads
//! stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-protected-path-governance-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-protected-path-governance-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_owner_coverage_approver_state_review_pack_freshness_provider_enforcement_or_public_surface_diff_truth_is_stale_or_partial_across_claimed_m5_governance_components.md`](../../../../docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_owner_coverage_approver_state_review_pack_freshness_provider_enforcement_or_public_surface_diff_truth_is_stale_or_partial_across_claimed_m5_governance_components.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-protected-path-governance-component-accessibility-parity/`](../../../../fixtures/ui/m5-protected-path-governance-component-accessibility-parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_protected_path_governance_component_matrix::M5GovernanceComponent;

/// Stable record-kind tag carried by [`GovernanceComponentAccessibilityPacket`].
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_RECORD_KIND: &str =
    "protected_path_governance_component_accessibility_parity_truth";

/// Schema version for protected-path governance accessibility parity records.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_DOC_REF: &str =
    "docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_owner_coverage_approver_state_review_pack_freshness_provider_enforcement_or_public_surface_diff_truth_is_stale_or_partial_across_claimed_m5_governance_components.md";

/// Repo-relative path of the frozen governance component matrix these claims exercise.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this capstone extends.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-consumer.schema.json";

/// Repo-relative path of the protected-path / ownership controls contract.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-ownership-controls.schema.json";

/// Repo-relative path of the approver-matrix / review-pack controls contract.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-approver-review-pack-controls.schema.json";

/// Repo-relative path of the public-surface-diff / merge-control controls contract.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF:
    &str = "schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json";

/// Repo-relative path of the DRI-registry / merge-readiness controls contract.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF:
    &str = "schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-protected-path-governance-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-protected-path-governance-accessibility-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GOVERNANCE_COMPONENT_ACCESSIBILITY_SUMMARY_REF: &str =
    "artifacts/release/m5-protected-path-governance-accessibility-proof/summary.md";

/// Canonical component contract that a row must point at for a given component.
///
/// Each of the eight shared components resolves to the checked-in schema of the implement
/// lane that produced it: the protected-path / ownership controls, the approver-matrix /
/// review-pack controls, the public-surface-diff / merge-control controls, and the
/// DRI-registry / merge-readiness controls.
pub const fn component_canonical_schema_ref(component: M5GovernanceComponent) -> &'static str {
    match component {
        M5GovernanceComponent::ProtectedPathRow | M5GovernanceComponent::OwnershipCard => {
            GOVERNANCE_COMPONENT_ACCESSIBILITY_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::ApproverMatrix | M5GovernanceComponent::ReviewPackSummary => {
            GOVERNANCE_COMPONENT_ACCESSIBILITY_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::PublicSurfaceDiffCard
        | M5GovernanceComponent::MergeControlBanner => {
            GOVERNANCE_COMPONENT_ACCESSIBILITY_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF
        }
        M5GovernanceComponent::DriRegistryRow | M5GovernanceComponent::MergeReadinessStrip => {
            GOVERNANCE_COMPONENT_ACCESSIBILITY_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF
        }
    }
}

/// The condition governing how much governed authority a component may claim.
///
/// [`GovernanceTruthTrusted`](Self::GovernanceTruthTrusted) is the baseline where the full
/// governed-authority claim is permitted. The other five are the weakening conditions
/// named by the spec: stale or advisory provider enforcement, partial owner coverage,
/// stale or waived approver state, a stale review pack, and a partial public-surface diff.
/// Each weakening condition pins the claim to a ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentClaimCondition {
    /// Provider enforcement, owner coverage, approver state, review pack, and
    /// public-surface diff truth are all trusted.
    GovernanceTruthTrusted,
    /// Provider enforcement is advisory, stale, or a local estimate.
    ProviderEnforcementStaleOrPartial,
    /// Owner backup coverage is missing or unresolved for the guarded path.
    OwnerCoveragePartial,
    /// A required approval is waived or has expired.
    ApproverStateStaleOrPartial,
    /// The review pack is stale relative to the change it gates.
    ReviewPackFreshnessStale,
    /// The public-surface diff or migration evidence is partial or ungenerated.
    PublicSurfaceDiffTruthPartial,
}

impl GovernanceComponentClaimCondition {
    /// Every condition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GovernanceTruthTrusted,
        Self::ProviderEnforcementStaleOrPartial,
        Self::OwnerCoveragePartial,
        Self::ApproverStateStaleOrPartial,
        Self::ReviewPackFreshnessStale,
        Self::PublicSurfaceDiffTruthPartial,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GovernanceTruthTrusted => "governance_truth_trusted",
            Self::ProviderEnforcementStaleOrPartial => "provider_enforcement_stale_or_partial",
            Self::OwnerCoveragePartial => "owner_coverage_partial",
            Self::ApproverStateStaleOrPartial => "approver_state_stale_or_partial",
            Self::ReviewPackFreshnessStale => "review_pack_freshness_stale",
            Self::PublicSurfaceDiffTruthPartial => "public_surface_diff_truth_partial",
        }
    }

    /// Whether this condition weakens the governed-authority claim (everything but trusted).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::GovernanceTruthTrusted)
    }

    /// The strongest claim tier this condition still permits.
    pub const fn permitted_ceiling(self) -> GovernanceComponentClaimTier {
        match self {
            Self::GovernanceTruthTrusted => GovernanceComponentClaimTier::FullGovernedAuthority,
            Self::ProviderEnforcementStaleOrPartial => {
                GovernanceComponentClaimTier::AdvisoryEnforcementOnly
            }
            Self::OwnerCoveragePartial => GovernanceComponentClaimTier::OwnerBackupCoverageMissing,
            Self::ApproverStateStaleOrPartial => {
                GovernanceComponentClaimTier::ApproverStateNarrowed
            }
            Self::ReviewPackFreshnessStale => {
                GovernanceComponentClaimTier::ReviewPackStaleDisclosed
            }
            Self::PublicSurfaceDiffTruthPartial => {
                GovernanceComponentClaimTier::PublicSurfaceEvidenceWithheld
            }
        }
    }

    /// The downgrade trigger a weakening condition must disclose, if any.
    pub const fn default_trigger(self) -> Option<GovernanceComponentAccessibilityDowngradeTrigger> {
        match self {
            Self::GovernanceTruthTrusted => None,
            Self::ProviderEnforcementStaleOrPartial => Some(
                GovernanceComponentAccessibilityDowngradeTrigger::ProviderEnforcementStaleOrPartial,
            ),
            Self::OwnerCoveragePartial => {
                Some(GovernanceComponentAccessibilityDowngradeTrigger::OwnerCoveragePartial)
            }
            Self::ApproverStateStaleOrPartial => {
                Some(GovernanceComponentAccessibilityDowngradeTrigger::ApproverStateStaleOrPartial)
            }
            Self::ReviewPackFreshnessStale => {
                Some(GovernanceComponentAccessibilityDowngradeTrigger::ReviewPackFreshnessStale)
            }
            Self::PublicSurfaceDiffTruthPartial => Some(
                GovernanceComponentAccessibilityDowngradeTrigger::PublicSurfaceDiffTruthPartial,
            ),
        }
    }

    /// The next action a weakening condition's narrow disclosure must offer.
    pub const fn next_action(self) -> GovernanceComponentClaimNextAction {
        match self {
            Self::GovernanceTruthTrusted => {
                GovernanceComponentClaimNextAction::ContinueGovernedReview
            }
            Self::ProviderEnforcementStaleOrPartial => {
                GovernanceComponentClaimNextAction::SeekProviderEnforcementClearance
            }
            Self::OwnerCoveragePartial => {
                GovernanceComponentClaimNextAction::ResolveOwnerBackupCoverage
            }
            Self::ApproverStateStaleOrPartial => {
                GovernanceComponentClaimNextAction::RefreshApproverState
            }
            Self::ReviewPackFreshnessStale => GovernanceComponentClaimNextAction::RefreshReviewPack,
            Self::PublicSurfaceDiffTruthPartial => {
                GovernanceComponentClaimNextAction::GeneratePublicSurfaceDiff
            }
        }
    }
}

/// A component's claim about how much governed authority it asserts.
///
/// Ordered strongest to weakest. [`FullGovernedAuthority`](Self::FullGovernedAuthority) is
/// the only tier that asserts provider-authoritative enforcement with full owner coverage,
/// satisfied approvals, a fresh review pack, and a generated public-surface diff; the rest
/// are the honest fallbacks a weakening condition narrows to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentClaimTier {
    /// Provider-authoritative enforcement with full owner coverage, satisfied approvals,
    /// a fresh review pack, and a generated public-surface diff.
    FullGovernedAuthority,
    /// Enforcement is advisory or a local estimate, never provider-authoritative.
    AdvisoryEnforcementOnly,
    /// Owner is resolved but backup coverage is missing; the guard is not fully covered.
    OwnerBackupCoverageMissing,
    /// A required approval is waived or expired; approval state is explicitly narrowed.
    ApproverStateNarrowed,
    /// The review pack is stale relative to the change and is disclosed as such.
    ReviewPackStaleDisclosed,
    /// Public-surface diff or migration evidence is withheld pending generation.
    PublicSurfaceEvidenceWithheld,
}

impl GovernanceComponentClaimTier {
    /// Every tier, in declaration order (strongest first).
    pub const ALL: [Self; 6] = [
        Self::FullGovernedAuthority,
        Self::AdvisoryEnforcementOnly,
        Self::OwnerBackupCoverageMissing,
        Self::ApproverStateNarrowed,
        Self::ReviewPackStaleDisclosed,
        Self::PublicSurfaceEvidenceWithheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullGovernedAuthority => "full_governed_authority",
            Self::AdvisoryEnforcementOnly => "advisory_enforcement_only",
            Self::OwnerBackupCoverageMissing => "owner_backup_coverage_missing",
            Self::ApproverStateNarrowed => "approver_state_narrowed",
            Self::ReviewPackStaleDisclosed => "review_pack_stale_disclosed",
            Self::PublicSurfaceEvidenceWithheld => "public_surface_evidence_withheld",
        }
    }

    /// Strength rank, higher is stronger. Used for the ceiling comparison.
    pub const fn rank(self) -> u8 {
        match self {
            Self::FullGovernedAuthority => 6,
            Self::AdvisoryEnforcementOnly => 5,
            Self::OwnerBackupCoverageMissing => 4,
            Self::ApproverStateNarrowed => 3,
            Self::ReviewPackStaleDisclosed => 2,
            Self::PublicSurfaceEvidenceWithheld => 1,
        }
    }

    /// Whether this tier asserts full provider-authoritative governed authority.
    pub const fn asserts_full_governed_authority(self) -> bool {
        matches!(self, Self::FullGovernedAuthority)
    }
}

/// A rendering form the claim must reach with identical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentRenderingSurface {
    /// The full desktop surface.
    DesktopFull,
    /// The headless CLI.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl GovernanceComponentRenderingSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopFull, Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The next action a narrow disclosure offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentClaimNextAction {
    /// Seek provider enforcement clearance before treating the guard as authoritative.
    SeekProviderEnforcementClearance,
    /// Resolve the missing owner backup coverage.
    ResolveOwnerBackupCoverage,
    /// Refresh the waived or expired approver state.
    RefreshApproverState,
    /// Refresh the stale review pack against the current change.
    RefreshReviewPack,
    /// Generate the machine public-surface diff and migration evidence.
    GeneratePublicSurfaceDiff,
    /// Continue the governed review.
    ContinueGovernedReview,
}

impl GovernanceComponentClaimNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeekProviderEnforcementClearance => "seek_provider_enforcement_clearance",
            Self::ResolveOwnerBackupCoverage => "resolve_owner_backup_coverage",
            Self::RefreshApproverState => "refresh_approver_state",
            Self::RefreshReviewPack => "refresh_review_pack",
            Self::GeneratePublicSurfaceDiff => "generate_public_surface_diff",
            Self::ContinueGovernedReview => "continue_governed_review",
        }
    }
}

/// Downgrade trigger that can narrow this accessibility lane below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComponentAccessibilityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider enforcement is advisory, stale, or a local estimate.
    ProviderEnforcementStaleOrPartial,
    /// Owner backup coverage is missing or unresolved.
    OwnerCoveragePartial,
    /// A required approval is waived or expired.
    ApproverStateStaleOrPartial,
    /// The review pack is stale relative to the change.
    ReviewPackFreshnessStale,
    /// The public-surface diff or migration evidence is partial or ungenerated.
    PublicSurfaceDiffTruthPartial,
    /// A claim was overstated relative to its permitted ceiling.
    ClaimOverstated,
    /// Parity across desktop, CLI, or export was dropped.
    ParityDropped,
    /// Consumer trust narrowed.
    TrustNarrowing,
}

impl GovernanceComponentAccessibilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderEnforcementStaleOrPartial,
        Self::OwnerCoveragePartial,
        Self::ApproverStateStaleOrPartial,
        Self::ReviewPackFreshnessStale,
        Self::PublicSurfaceDiffTruthPartial,
        Self::ClaimOverstated,
        Self::ParityDropped,
        Self::TrustNarrowing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderEnforcementStaleOrPartial => "provider_enforcement_stale_or_partial",
            Self::OwnerCoveragePartial => "owner_coverage_partial",
            Self::ApproverStateStaleOrPartial => "approver_state_stale_or_partial",
            Self::ReviewPackFreshnessStale => "review_pack_freshness_stale",
            Self::PublicSurfaceDiffTruthPartial => "public_surface_diff_truth_partial",
            Self::ClaimOverstated => "claim_overstated",
            Self::ParityDropped => "parity_dropped",
            Self::TrustNarrowing => "trust_narrowing",
        }
    }
}

/// The disclosures an accessibility row must carry, derived from its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GovernanceComponentClaimResolution {
    /// The strongest claim tier the condition permits.
    pub permitted_ceiling: GovernanceComponentClaimTier,
    /// Whether the condition requires an explicit narrow disclosure.
    pub requires_narrowing: bool,
    /// The downgrade trigger the narrow disclosure must name, if any.
    pub expected_trigger: Option<GovernanceComponentAccessibilityDowngradeTrigger>,
    /// The next action the narrow disclosure must offer.
    pub expected_next_action: GovernanceComponentClaimNextAction,
    /// Whether the row must keep the explicit owner / approver / public-surface semantics
    /// rather than dropping to a vague `governed` label.
    pub needs_governed_semantics_note: bool,
    /// Whether the row must carry an explicit advisory-not-authoritative enforcement note.
    pub needs_enforcement_authority_note: bool,
    /// Whether the row must carry an explicit missing-backup-coverage note.
    pub needs_backup_coverage_note: bool,
    /// Whether the row must carry an explicit waived/expired approver-state note.
    pub needs_approver_state_note: bool,
    /// Whether the row must carry an explicit missing public-surface diff/migration note.
    pub needs_public_surface_evidence_note: bool,
}

/// Resolves the claim narrowing an accessibility row must carry from its condition.
///
/// Trusted governance truth keeps the full governed-authority claim. Each weakening
/// condition pins the claim to a ceiling, demands an explicit narrow disclosure naming its
/// trigger and next action, and keeps the explicit owner / approver / public-surface
/// semantics so the guard is never reduced to a vague `governed` label. Stale or advisory
/// provider enforcement additionally demands an explicit advisory-not-authoritative note
/// rather than letting an advisory hint read as provider-authoritative enforcement; partial
/// owner coverage demands an explicit missing-backup-coverage note rather than letting a
/// guarded merge hide it; a waived or expired approval demands an explicit approver-state
/// note rather than hiding it; and a partial public-surface diff demands an explicit
/// missing-diff/migration note rather than letting the change read clean.
pub const fn resolve_governance_component_claim_narrowing(
    condition: GovernanceComponentClaimCondition,
) -> GovernanceComponentClaimResolution {
    GovernanceComponentClaimResolution {
        permitted_ceiling: condition.permitted_ceiling(),
        requires_narrowing: condition.is_weakening(),
        expected_trigger: condition.default_trigger(),
        expected_next_action: condition.next_action(),
        needs_governed_semantics_note: condition.is_weakening(),
        needs_enforcement_authority_note: matches!(
            condition,
            GovernanceComponentClaimCondition::ProviderEnforcementStaleOrPartial
        ),
        needs_backup_coverage_note: matches!(
            condition,
            GovernanceComponentClaimCondition::OwnerCoveragePartial
        ),
        needs_approver_state_note: matches!(
            condition,
            GovernanceComponentClaimCondition::ApproverStateStaleOrPartial
        ),
        needs_public_surface_evidence_note: matches!(
            condition,
            GovernanceComponentClaimCondition::PublicSurfaceDiffTruthPartial
        ),
    }
}

/// The explicit narrow disclosure a claim-narrowed row shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentClaimNarrowing {
    /// The downgrade trigger the narrowing discloses.
    pub trigger: GovernanceComponentAccessibilityDowngradeTrigger,
    /// The claim tier the narrowing pins the component to.
    pub narrowed_to: GovernanceComponentClaimTier,
    /// Note naming the truth preserved through the narrowing (never omitted).
    pub preserved_truth_note: String,
    /// The next action offered.
    pub next_action: GovernanceComponentClaimNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// One accessibility row: a claimed component under one condition, exposed across
/// keyboard, screen-reader, CLI, and export forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentAccessibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Which shared component this row claims.
    pub component: M5GovernanceComponent,
    /// The condition governing the claim.
    pub condition: GovernanceComponentClaimCondition,
    /// The claim tier the component effectively asserts.
    pub effective_claim: GovernanceComponentClaimTier,
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
    /// The rendering surfaces this row reaches (must cover all three).
    pub rendering_surfaces: Vec<GovernanceComponentRenderingSurface>,
    /// The explicit narrow disclosure; required and complete when the claim narrows.
    pub narrowing: Option<GovernanceComponentClaimNarrowing>,
    /// Explicit owner / approver / public-surface semantics note; required and non-empty
    /// when the claim narrows so the guard never drops to a vague `governed` label.
    pub governed_semantics_note: String,
    /// Advisory-not-authoritative enforcement note; required and non-empty when provider
    /// enforcement is advisory, stale, or a local estimate.
    pub enforcement_authority_note: String,
    /// Missing-backup-coverage note; required and non-empty when owner coverage is partial.
    pub backup_coverage_note: String,
    /// Waived/expired approver-state note; required and non-empty when approver state is stale.
    pub approver_state_note: String,
    /// Missing public-surface diff/migration note; required and non-empty when the
    /// public-surface diff truth is partial.
    pub public_surface_evidence_note: String,
    /// Guardrail: this component is reachable only by pointer.
    pub is_pointer_only: bool,
    /// Guardrail: this component omits itself from the export.
    pub is_export_opaque: bool,
    /// Guardrail: this component claims more on the desktop than in CLI or export.
    pub desktop_stronger_than_cli: bool,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl GovernanceComponentAccessibilityRow {
    /// The disclosures this row must carry, derived from its condition.
    pub const fn resolution(&self) -> GovernanceComponentClaimResolution {
        resolve_governance_component_claim_narrowing(self.condition)
    }

    /// Whether this row narrows below the full governed-authority claim.
    pub const fn is_narrowed(&self) -> bool {
        self.condition.is_weakening()
    }

    /// Whether this row reaches all three rendering surfaces.
    pub fn covers_all_rendering_surfaces(&self) -> bool {
        GovernanceComponentRenderingSurface::ALL
            .iter()
            .all(|surface| self.rendering_surfaces.contains(surface))
    }

    /// Whether every accessibility field is present.
    pub fn accessibility_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.is_pointer_only && !self.is_export_opaque && !self.desktop_stronger_than_cli
    }

    /// Whether this row points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF
            })
    }

    /// Whether the effective claim is honest under the row's condition: it never exceeds
    /// the permitted ceiling, and a weakening condition narrows the claim down to exactly
    /// that ceiling.
    pub fn claim_is_honest(&self) -> bool {
        let resolution = self.resolution();
        let ceiling = resolution.permitted_ceiling;
        if self.effective_claim.rank() > ceiling.rank() {
            return false;
        }
        if resolution.requires_narrowing {
            self.effective_claim == ceiling
                && self
                    .narrowing
                    .as_ref()
                    .is_some_and(|narrowing| narrowing.narrowed_to == ceiling)
        } else {
            self.effective_claim == ceiling && self.narrowing.is_none()
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentAccessibilityTrustReview {
    /// Every claim is keyboard-reachable.
    pub keyboard_reachable_on_every_claim: bool,
    /// Every claim carries a screen-reader label.
    pub screen_reader_labeled_on_every_claim: bool,
    /// Every claim exposes a CLI enum token.
    pub cli_enum_exposed_on_every_claim: bool,
    /// Every claim exposes an export enum token.
    pub export_enum_exposed_on_every_claim: bool,
    /// Every claim carries an explanation field.
    pub explanation_field_present_on_every_claim: bool,
    /// No component is pointer-only.
    pub no_component_pointer_only: bool,
    /// No component is export-opaque.
    pub no_component_export_opaque: bool,
    /// No component claims more on the desktop than in CLI or export.
    pub desktop_never_stronger_than_cli: bool,
    /// The claim narrows whenever governance evidence weakens.
    pub claim_narrows_when_governance_evidence_weakens: bool,
    /// Governed authority is never overstated while a weakening condition holds.
    pub governed_authority_never_overstated_under_weakening: bool,
    /// The owner / approver / public-surface semantics are kept explicit, never `governed`.
    pub owner_approver_public_surface_semantics_kept_explicit: bool,
    /// An advisory owner hint is never promoted to provider-authoritative enforcement.
    pub advisory_never_promoted_to_provider_authoritative: bool,
}

impl GovernanceComponentAccessibilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.keyboard_reachable_on_every_claim
            && self.screen_reader_labeled_on_every_claim
            && self.cli_enum_exposed_on_every_claim
            && self.export_enum_exposed_on_every_claim
            && self.explanation_field_present_on_every_claim
            && self.no_component_pointer_only
            && self.no_component_export_opaque
            && self.desktop_never_stronger_than_cli
            && self.claim_narrows_when_governance_evidence_weakens
            && self.governed_authority_never_overstated_under_weakening
            && self.owner_approver_public_surface_semantics_kept_explicit
            && self.advisory_never_promoted_to_provider_authoritative
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentAccessibilityProjection {
    /// Keyboard and screen-reader labels are exposed.
    pub exposes_keyboard_and_screen_reader_labels: bool,
    /// CLI and export enums are exposed.
    pub exposes_cli_and_export_enums: bool,
    /// Explanation fields are exposed.
    pub exposes_explanation_fields: bool,
    /// The claim auto-narrows when provider enforcement is stale or advisory.
    pub auto_narrows_on_stale_provider_enforcement: bool,
    /// The claim auto-narrows when owner coverage is partial.
    pub auto_narrows_on_partial_owner_coverage: bool,
    /// The claim auto-narrows when approver state is stale or waived.
    pub auto_narrows_on_stale_approver_state: bool,
    /// The claim auto-narrows when the review pack is stale.
    pub auto_narrows_on_stale_review_pack: bool,
    /// The claim auto-narrows when the public-surface diff is partial.
    pub auto_narrows_on_partial_public_surface_diff: bool,
    /// Desktop, CLI, and export semantics are identical.
    pub desktop_cli_export_semantics_identical: bool,
    /// Narrowing prevents overstated governed authority.
    pub narrowing_prevents_overstated_governed_authority: bool,
}

impl GovernanceComponentAccessibilityProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exposes_keyboard_and_screen_reader_labels
            && self.exposes_cli_and_export_enums
            && self.exposes_explanation_fields
            && self.auto_narrows_on_stale_provider_enforcement
            && self.auto_narrows_on_partial_owner_coverage
            && self.auto_narrows_on_stale_approver_state
            && self.auto_narrows_on_stale_review_pack
            && self.auto_narrows_on_partial_public_surface_diff
            && self.desktop_cli_export_semantics_identical
            && self.narrowing_prevents_overstated_governed_authority
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentAccessibilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GovernanceComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceComponentAccessibilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<GovernanceComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GovernanceComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<GovernanceComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: GovernanceComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: GovernanceComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe protected-path governance accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComponentAccessibilityPacket {
    /// Record kind; must equal [`GOVERNANCE_COMPONENT_ACCESSIBILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<GovernanceComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GovernanceComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<GovernanceComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: GovernanceComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: GovernanceComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: GovernanceComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GovernanceComponentAccessibilityPacket {
    /// Builds a protected-path governance accessibility packet from stable-lane input.
    pub fn new(input: GovernanceComponentAccessibilityPacketInput) -> Self {
        Self {
            record_kind: GOVERNANCE_COMPONENT_ACCESSIBILITY_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            accessibility_rows: input.accessibility_rows,
            downgrade_triggers: input.downgrade_triggers,
            rendering_surfaces: input.rendering_surfaces,
            trust_review: input.trust_review,
            projection: input.projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the protected-path governance accessibility parity invariants.
    pub fn validate(&self) -> Vec<GovernanceComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GOVERNANCE_COMPONENT_ACCESSIBILITY_RECORD_KIND {
            violations.push(GovernanceComponentAccessibilityViolation::WrongRecordKind);
        }
        if self.schema_version != GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION {
            violations.push(GovernanceComponentAccessibilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GovernanceComponentAccessibilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::DowngradeTriggersMissing);
        }
        if self.rendering_surfaces.is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::RenderingSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GovernanceComponentAccessibilityViolation::TrustReviewIncomplete);
        }
        if !self.projection.all_hold() {
            violations.push(GovernanceComponentAccessibilityViolation::ProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GovernanceComponentAccessibilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("protected-path governance accessibility packet serializes"),
        ) {
            violations.push(GovernanceComponentAccessibilityViolation::RawBoundaryMaterialInExport);
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
            .expect("protected-path governance accessibility packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .accessibility_rows
            .iter()
            .filter(|row| row.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Protected-Path Governance Accessibility, Headless, and Export Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Accessibility rows: {} ({} claim-narrowed)\n",
            self.accessibility_rows.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Accessibility rows\n\n");
        for row in &self.accessibility_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: condition `{}`, claim `{}`\n",
                row.component.as_str(),
                row.row_id,
                row.condition.as_str(),
                row.effective_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in protected-path governance accessibility export.
#[derive(Debug)]
pub enum GovernanceComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GovernanceComponentAccessibilityViolation>),
}

impl fmt::Display for GovernanceComponentAccessibilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "protected-path governance accessibility export parse failed: {error}"
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
                    "protected-path governance accessibility export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GovernanceComponentAccessibilityArtifactError {}

/// Validation failures emitted by [`GovernanceComponentAccessibilityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceComponentAccessibilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No accessibility rows are present.
    AccessibilityRowsMissing,
    /// An accessibility row is incomplete.
    RowIncomplete,
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
    /// A row does not reach all three rendering surfaces.
    RenderingSurfaceCoverageMissing,
    /// A component is reachable only by pointer.
    PointerOnlyComponent,
    /// A component omits itself from the export.
    ExportOpaqueComponent,
    /// A component claims more on the desktop than in CLI or export.
    DesktopStrongerThanCli,
    /// A row's effective claim exceeds the ceiling its condition permits.
    ClaimCeilingExceeded,
    /// A weakening condition is missing its explicit narrow disclosure.
    ClaimNarrowingMissing,
    /// A baseline condition unexpectedly carries a narrow disclosure.
    ClaimNarrowingUnexpected,
    /// A narrow disclosure pins the claim to the wrong tier.
    NarrowedToMismatch,
    /// A narrow disclosure names the wrong trigger.
    NarrowTriggerMismatch,
    /// A narrow disclosure offers the wrong next action.
    NarrowNextActionMismatch,
    /// A narrow disclosure is missing its preserved-truth note.
    NarrowPreservedTruthMissing,
    /// A narrow disclosure is missing its next-action copy.
    NarrowNextActionMissing,
    /// A row that must keep the owner/approver/public-surface semantics is missing its note.
    GovernedSemanticsNoteMissing,
    /// A row that must keep advisory enforcement explicit is missing its note.
    EnforcementAuthorityNoteMissing,
    /// A row that must keep missing backup coverage explicit is missing its note.
    BackupCoverageNoteMissing,
    /// A row that must keep waived/expired approver state explicit is missing its note.
    ApproverStateNoteMissing,
    /// A row that must keep the missing public-surface diff/migration explicit is missing its note.
    PublicSurfaceEvidenceNoteMissing,
    /// A row does not point at the canonical component and matrix contracts.
    CanonicalContractReferenceMissing,
    /// Not every shared component appears among the rows.
    ComponentCoverageMissing,
    /// Not every claim condition appears among the rows.
    ConditionCoverageMissing,
    /// Not every claim tier appears as an effective claim.
    ClaimTierCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No rendering surfaces are present.
    RenderingSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GovernanceComponentAccessibilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::AccessibilityRowsMissing => "accessibility_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::RenderingSurfaceCoverageMissing => "rendering_surface_coverage_missing",
            Self::PointerOnlyComponent => "pointer_only_component",
            Self::ExportOpaqueComponent => "export_opaque_component",
            Self::DesktopStrongerThanCli => "desktop_stronger_than_cli",
            Self::ClaimCeilingExceeded => "claim_ceiling_exceeded",
            Self::ClaimNarrowingMissing => "claim_narrowing_missing",
            Self::ClaimNarrowingUnexpected => "claim_narrowing_unexpected",
            Self::NarrowedToMismatch => "narrowed_to_mismatch",
            Self::NarrowTriggerMismatch => "narrow_trigger_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowPreservedTruthMissing => "narrow_preserved_truth_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::GovernedSemanticsNoteMissing => "governed_semantics_note_missing",
            Self::EnforcementAuthorityNoteMissing => "enforcement_authority_note_missing",
            Self::BackupCoverageNoteMissing => "backup_coverage_note_missing",
            Self::ApproverStateNoteMissing => "approver_state_note_missing",
            Self::PublicSurfaceEvidenceNoteMissing => "public_surface_evidence_note_missing",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::ConditionCoverageMissing => "condition_coverage_missing",
            Self::ClaimTierCoverageMissing => "claim_tier_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RenderingSurfacesMissing => "rendering_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ProjectionIncomplete => "projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable protected-path governance accessibility export.
pub fn current_governance_component_accessibility_export(
) -> Result<GovernanceComponentAccessibilityPacket, GovernanceComponentAccessibilityArtifactError> {
    let packet: GovernanceComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-protected-path-governance-accessibility-proof/support_export.json"
    )))
    .map_err(GovernanceComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GovernanceComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GovernanceComponentAccessibilityPacket,
    violations: &mut Vec<GovernanceComponentAccessibilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GOVERNANCE_COMPONENT_ACCESSIBILITY_SCHEMA_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_DOC_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF,
        GOVERNANCE_COMPONENT_ACCESSIBILITY_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GovernanceComponentAccessibilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &GovernanceComponentAccessibilityPacket,
    violations: &mut Vec<GovernanceComponentAccessibilityViolation>,
) {
    if packet.accessibility_rows.is_empty() {
        violations.push(GovernanceComponentAccessibilityViolation::AccessibilityRowsMissing);
        return;
    }

    let mut seen_components: BTreeSet<M5GovernanceComponent> = BTreeSet::new();
    let mut seen_conditions: BTreeSet<GovernanceComponentClaimCondition> = BTreeSet::new();
    let mut seen_tiers: BTreeSet<GovernanceComponentClaimTier> = BTreeSet::new();

    for row in &packet.accessibility_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::RowIncomplete);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_rendering_surfaces() {
            violations
                .push(GovernanceComponentAccessibilityViolation::RenderingSurfaceCoverageMissing);
        }

        // AC1 guardrails: parity across desktop, CLI, and export.
        if row.is_pointer_only {
            violations.push(GovernanceComponentAccessibilityViolation::PointerOnlyComponent);
        }
        if row.is_export_opaque {
            violations.push(GovernanceComponentAccessibilityViolation::ExportOpaqueComponent);
        }
        if row.desktop_stronger_than_cli {
            violations.push(GovernanceComponentAccessibilityViolation::DesktopStrongerThanCli);
        }

        let resolution = row.resolution();
        let ceiling = resolution.permitted_ceiling;

        // AC2 core: a claim may never exceed the ceiling its condition permits.
        if row.effective_claim.rank() > ceiling.rank() {
            violations.push(GovernanceComponentAccessibilityViolation::ClaimCeilingExceeded);
        }

        // Narrow-disclosure presence and completeness.
        if resolution.requires_narrowing {
            match &row.narrowing {
                None => {
                    violations
                        .push(GovernanceComponentAccessibilityViolation::ClaimNarrowingMissing);
                }
                Some(narrowing) => {
                    if narrowing.narrowed_to != ceiling {
                        violations
                            .push(GovernanceComponentAccessibilityViolation::NarrowedToMismatch);
                    }
                    if Some(narrowing.trigger) != resolution.expected_trigger {
                        violations
                            .push(GovernanceComponentAccessibilityViolation::NarrowTriggerMismatch);
                    }
                    if narrowing.next_action != resolution.expected_next_action {
                        violations.push(
                            GovernanceComponentAccessibilityViolation::NarrowNextActionMismatch,
                        );
                    }
                    if narrowing.preserved_truth_note.trim().is_empty() {
                        violations.push(
                            GovernanceComponentAccessibilityViolation::NarrowPreservedTruthMissing,
                        );
                    }
                    if narrowing.next_action_label.trim().is_empty() {
                        violations.push(
                            GovernanceComponentAccessibilityViolation::NarrowNextActionMissing,
                        );
                    }
                }
            }
        } else if row.narrowing.is_some() {
            violations.push(GovernanceComponentAccessibilityViolation::ClaimNarrowingUnexpected);
        }

        if resolution.needs_governed_semantics_note && row.governed_semantics_note.trim().is_empty()
        {
            violations
                .push(GovernanceComponentAccessibilityViolation::GovernedSemanticsNoteMissing);
        }
        if resolution.needs_enforcement_authority_note
            && row.enforcement_authority_note.trim().is_empty()
        {
            violations
                .push(GovernanceComponentAccessibilityViolation::EnforcementAuthorityNoteMissing);
        }
        if resolution.needs_backup_coverage_note && row.backup_coverage_note.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::BackupCoverageNoteMissing);
        }
        if resolution.needs_approver_state_note && row.approver_state_note.trim().is_empty() {
            violations.push(GovernanceComponentAccessibilityViolation::ApproverStateNoteMissing);
        }
        if resolution.needs_public_surface_evidence_note
            && row.public_surface_evidence_note.trim().is_empty()
        {
            violations
                .push(GovernanceComponentAccessibilityViolation::PublicSurfaceEvidenceNoteMissing);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(GovernanceComponentAccessibilityViolation::CanonicalContractReferenceMissing);
        }

        seen_components.insert(row.component);
        seen_conditions.insert(row.condition);
        seen_tiers.insert(row.effective_claim);
    }

    // Coverage: every component, every condition, and every claim tier must appear.
    for component in M5GovernanceComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GovernanceComponentAccessibilityViolation::ComponentCoverageMissing);
            break;
        }
    }
    for condition in GovernanceComponentClaimCondition::ALL {
        if !seen_conditions.contains(&condition) {
            violations.push(GovernanceComponentAccessibilityViolation::ConditionCoverageMissing);
            break;
        }
    }
    for tier in GovernanceComponentClaimTier::ALL {
        if !seen_tiers.contains(&tier) {
            violations.push(GovernanceComponentAccessibilityViolation::ClaimTierCoverageMissing);
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
