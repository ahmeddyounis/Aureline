//! Frozen M5 protected-path, ownership, approver, review-pack, public-surface,
//! merge-control, DRI-registry, and merge-readiness governance component matrix.
//!
//! This module locks the canonical M5 component truth for eight reusable governed
//! review, release, and shiproom surfaces — protected-path rows, ownership cards,
//! approver matrices, review-pack summaries, public-surface diff cards,
//! merge-control banners, DRI-registry rows, and merge-readiness strips — into one
//! export-safe packet. Each [`M5GovernanceComponentMatrixRow`] binds a component to
//! its maturity class, the exact advisory-versus-authoritative and
//! provider-authoritative-versus-local-estimate enforcement distinction it must
//! preserve, the frozen governance-state vocabulary it may render, its escalation
//! boundary, its backup-coverage fallback, required evidence packet refs, downgrade
//! triggers, rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether every claimed M5 governed
//! review/release surface may consume one shared component family instead of
//! feature-local governance chrome, private row text, or provider-specific badges.
//! It references upstream protected-path, owner-coverage, approver, review-pack,
//! public-surface diff, merge-control, DRI, and merge-readiness contracts by id
//! rather than embedding their content. Raw CODEOWNERS bodies, raw diff bodies, raw
//! provider payloads, credentials, and live provider responses stay outside the
//! support boundary.
//!
//! The frozen controlled vocabulary — `advisory`, `authoritative`, `covered`,
//! `backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`, and
//! `local_estimate` — is carried by [`M5GovernanceStateVocab`] so later
//! implementation rows reuse one lexicon instead of minting drifted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-protected-path-governance-component-matrix.schema.json`](../../../../schemas/ui/m5-protected-path-governance-component-matrix.schema.json).
//! The contract doc is
//! [`docs/review/m5/freeze_the_m5_protected_path_governance_component_matrix.md`](../../../../docs/review/m5/freeze_the_m5_protected_path_governance_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-protected-path-governance/`](../../../../fixtures/ui/m5-protected-path-governance/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5GovernanceComponentMatrixPacket`].
pub const M5_GOVERNANCE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_protected_path_governance_component_matrix";

/// Schema version for M5 governance-component matrix records.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-protected-path-governance-component-matrix.schema.json";

/// Repo-relative path of the M5 governance-component matrix contract doc.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/review/m5/freeze_the_m5_protected_path_governance_component_matrix.md";

/// Repo-relative path of the per-component protected-path-row contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-protected-path-row.schema.json";

/// Repo-relative path of the per-component ownership-card contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-ownership-card.schema.json";

/// Repo-relative path of the per-component approver-matrix contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-approver-matrix.schema.json";

/// Repo-relative path of the per-component review-pack-summary contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF: &str =
    "schemas/ui/m5-review-pack-summary.schema.json";

/// Repo-relative path of the per-component public-surface-diff-card contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-public-surface-diff-card.schema.json";

/// Repo-relative path of the per-component merge-control-banner contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-control-banner.schema.json";

/// Repo-relative path of the per-component DRI-registry-row contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-dri-registry-row.schema.json";

/// Repo-relative path of the per-component merge-readiness-strip contract.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-readiness-strip.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-protected-path-governance";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-protected-path-governance-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_GOVERNANCE_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/release/m5-protected-path-governance-proof/summary.md";

/// One of the eight M5 reusable governance components governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponent {
    /// Protected-path row naming why a file or surface is guarded and how.
    ProtectedPathRow,
    /// Ownership card naming owner source and coverage.
    OwnershipCard,
    /// Approver matrix naming required approvers and their state.
    ApproverMatrix,
    /// Review-pack summary naming review-pack freshness and parity.
    ReviewPackSummary,
    /// Public-surface diff card naming change class and machine-generated diff.
    PublicSurfaceDiffCard,
    /// Merge-control banner naming blockers, never a generic warning.
    MergeControlBanner,
    /// DRI-registry row naming the directly responsible individual and coverage.
    DriRegistryRow,
    /// Merge-readiness strip summarizing blocking state and ownership.
    MergeReadinessStrip,
}

impl M5GovernanceComponent {
    /// Every component, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProtectedPathRow,
        Self::OwnershipCard,
        Self::ApproverMatrix,
        Self::ReviewPackSummary,
        Self::PublicSurfaceDiffCard,
        Self::MergeControlBanner,
        Self::DriRegistryRow,
        Self::MergeReadinessStrip,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProtectedPathRow => "protected_path_row",
            Self::OwnershipCard => "ownership_card",
            Self::ApproverMatrix => "approver_matrix",
            Self::ReviewPackSummary => "review_pack_summary",
            Self::PublicSurfaceDiffCard => "public_surface_diff_card",
            Self::MergeControlBanner => "merge_control_banner",
            Self::DriRegistryRow => "dri_registry_row",
            Self::MergeReadinessStrip => "merge_readiness_strip",
        }
    }

    /// Per-component source contract ref for this component.
    pub const fn contract_ref(self) -> &'static str {
        match self {
            Self::ProtectedPathRow => {
                M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF
            }
            Self::OwnershipCard => M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
            Self::ApproverMatrix => M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
            Self::ReviewPackSummary => {
                M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF
            }
            Self::PublicSurfaceDiffCard => {
                M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF
            }
            Self::MergeControlBanner => {
                M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF
            }
            Self::DriRegistryRow => M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
            Self::MergeReadinessStrip => {
                M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF
            }
        }
    }
}

/// Maturity class for an M5 governance component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponentMaturityClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5GovernanceComponentMaturityClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponentEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this component's current maturity.
    NotApplicable,
}

impl M5GovernanceComponentEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The one frozen governance-state vocabulary every M5 governance component reuses.
///
/// This is the controlled vocabulary the acceptance criteria pins: it names
/// enforcement authority (`advisory`, `authoritative`), provenance
/// (`provider_authoritative`, `local_estimate`), owner coverage (`covered`,
/// `backup_missing`), and approver / review-pack state (`waived`, `expired`,
/// `stale`) so no consumer mints a drifted label or lets an advisory hint read as
/// authoritative enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceStateVocab {
    /// Owner or protection hint is advisory, not enforced.
    Advisory,
    /// Owner or protection rule is authoritatively enforced.
    Authoritative,
    /// Owner coverage is present for the guarded path.
    Covered,
    /// Owner backup coverage is missing for the guarded path.
    BackupMissing,
    /// A required approval is explicitly waived.
    Waived,
    /// A required approval or review-pack window has expired.
    Expired,
    /// Provider-backed truth is stale relative to what it gates.
    Stale,
    /// Enforcement is authoritative because the provider enforces it.
    ProviderAuthoritative,
    /// Value is a local estimate, not provider-confirmed truth.
    LocalEstimate,
}

impl M5GovernanceStateVocab {
    /// Every frozen vocabulary token, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Advisory,
        Self::Authoritative,
        Self::Covered,
        Self::BackupMissing,
        Self::Waived,
        Self::Expired,
        Self::Stale,
        Self::ProviderAuthoritative,
        Self::LocalEstimate,
    ];

    /// Stable token recorded in the matrix. These strings are frozen.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Authoritative => "authoritative",
            Self::Covered => "covered",
            Self::BackupMissing => "backup_missing",
            Self::Waived => "waived",
            Self::Expired => "expired",
            Self::Stale => "stale",
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocalEstimate => "local_estimate",
        }
    }
}

/// Downgrade trigger that can narrow a component below its claimed maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponentDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Owner backup coverage is missing for a guarded path.
    OwnerCoverageBackupMissing,
    /// A required approver's state has expired.
    ApproverStateExpired,
    /// The bound review pack is stale relative to the change it gates.
    ReviewPackStale,
    /// A machine-generated public-surface diff is unavailable.
    PublicSurfaceDiffUnavailable,
    /// Migration or evidence context for a public-surface change is missing.
    MigrationEvidenceMissing,
    /// The DRI registry has a coverage gap for the guarded surface.
    DriCoverageGap,
    /// Escalation / shiproom handoff is unavailable.
    EscalationHandoffUnavailable,
    /// Component trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified governance-component boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl M5GovernanceComponentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::OwnerCoverageBackupMissing,
        Self::ApproverStateExpired,
        Self::ReviewPackStale,
        Self::PublicSurfaceDiffUnavailable,
        Self::MigrationEvidenceMissing,
        Self::DriCoverageGap,
        Self::EscalationHandoffUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::OwnerCoverageBackupMissing => "owner_coverage_backup_missing",
            Self::ApproverStateExpired => "approver_state_expired",
            Self::ReviewPackStale => "review_pack_stale",
            Self::PublicSurfaceDiffUnavailable => "public_surface_diff_unavailable",
            Self::MigrationEvidenceMissing => "migration_evidence_missing",
            Self::DriCoverageGap => "dri_coverage_gap",
            Self::EscalationHandoffUnavailable => "escalation_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponentRollbackPosture {
    /// Read-only component that never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Provider mutation stays individually attributable and reviewable.
    ProviderMutationAttributable,
    /// Local continuation is preserved when provider freshness is degraded.
    LocalContinuePreserved,
    /// Escalation or provider handoff always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the component's current maturity.
    NotApplicable,
}

impl M5GovernanceComponentRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::ProviderMutationAttributable => "provider_mutation_attributable",
            Self::LocalContinuePreserved => "local_continue_preserved",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceComponentConsumerSurface {
    /// Review workspace surface.
    ReviewWorkspace,
    /// Release-candidate surface.
    ReleaseCandidate,
    /// Shiproom / escalation surface.
    Shiproom,
    /// Governance / assurance dashboard.
    GovernanceDashboard,
    /// Owner-coverage panel.
    OwnerCoveragePanel,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl M5GovernanceComponentConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::ReleaseCandidate,
        Self::Shiproom,
        Self::GovernanceDashboard,
        Self::OwnerCoveragePanel,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::ReleaseCandidate => "release_candidate",
            Self::Shiproom => "shiproom",
            Self::GovernanceDashboard => "governance_dashboard",
            Self::OwnerCoveragePanel => "owner_coverage_panel",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 governance-component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentMatrixRow {
    /// Governance component.
    pub component: M5GovernanceComponent,
    /// Maturity class earned by this component.
    pub maturity: M5GovernanceComponentMaturityClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Exact advisory-versus-authoritative and provider-authoritative-versus-local
    /// enforcement distinction this component preserves.
    pub enforcement_distinction: String,
    /// Frozen governance-state vocabulary this component may render.
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Escalation / shiproom handoff boundary this component keeps explicit.
    pub escalation_boundary: String,
    /// Backup-coverage fallback this component preserves when owner or approver state degrades.
    pub backup_coverage_fallback: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5GovernanceComponentEvidenceRequirement,
    /// Required evidence packet refs for this maturity.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this component.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentMatrixTrustReview {
    /// Advisory owner hints are never presented as authoritative enforcement.
    pub advisory_never_masquerades_as_authoritative: bool,
    /// Provider-authoritative enforcement is never flattened into a local estimate.
    pub provider_authoritative_versus_local_estimate_distinct: bool,
    /// Missing owner backup coverage is named, never hidden.
    pub owner_coverage_backup_missing_explicit: bool,
    /// Expired, waived, and stale approver state stay explicit.
    pub approver_expired_waived_stale_explicit: bool,
    /// Review-pack freshness and parity stay explicit.
    pub review_pack_freshness_and_parity_explicit: bool,
    /// Public-surface changes require a machine-generated diff.
    pub public_surface_diff_machine_generated_required: bool,
    /// Public-surface changes require migration / evidence context.
    pub migration_evidence_required_for_public_surface_change: bool,
    /// Protection reason stays explicit on every guarded path.
    pub protection_reason_always_explicit: bool,
    /// DRI coverage gaps stay explicit.
    pub dri_coverage_gap_explicit: bool,
    /// Merge-control blockers are named, never a generic warning pill.
    pub merge_control_blocker_never_generic: bool,
    /// Escalation / shiproom handoff stays explicit with a safe return path.
    pub escalation_handoff_explicit: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentMatrixConsumerProjection {
    /// Protected-path row shows protection reason and enforcement authority.
    pub protected_path_row_shows_reason_and_enforcement_authority: bool,
    /// Ownership card shows owner source and coverage.
    pub ownership_card_shows_owner_source_and_coverage: bool,
    /// Approver matrix shows required approvers and their state.
    pub approver_matrix_shows_required_and_state: bool,
    /// Review-pack summary shows freshness and parity.
    pub review_pack_summary_shows_freshness_and_parity: bool,
    /// Public-surface diff card shows change class and machine-generated diff.
    pub public_surface_diff_card_shows_change_class_and_diff: bool,
    /// Merge-control banner shows blockers, never a generic warning.
    pub merge_control_banner_shows_blockers_not_generic: bool,
    /// DRI-registry row shows the DRI and coverage.
    pub dri_registry_row_shows_dri_and_coverage: bool,
    /// Merge-readiness strip shows blocking state and ownership.
    pub merge_readiness_strip_shows_blocking_and_ownership: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_component_truth: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5GovernanceComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GovernanceComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5GovernanceComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5GovernanceComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 governance-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentMatrixPacket {
    /// Record kind; must equal [`M5_GOVERNANCE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5GovernanceComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5GovernanceComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GovernanceComponentMatrixPacket {
    /// Builds an M5 governance-component matrix packet from component input.
    pub fn new(input: M5GovernanceComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_GOVERNANCE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 governance-component matrix invariants.
    pub fn validate(&self) -> Vec<M5GovernanceComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GOVERNANCE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5GovernanceComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5GovernanceComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GovernanceComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 governance-component matrix packet serializes"),
        ) {
            violations.push(M5GovernanceComponentMatrixViolation::RawBoundaryMaterialInExport);
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
            .expect("m5 governance-component matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.maturity.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Protected-Path Governance Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Components: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component.as_str(),
                row.maturity.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Enforcement distinction: {}\n",
                row.enforcement_distinction
            ));
            out.push_str(&format!(
                "  - Escalation boundary: {}\n",
                row.escalation_boundary
            ));
            out.push_str(&format!(
                "  - Backup-coverage fallback: {}\n",
                row.backup_coverage_fallback
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 governance-component matrix export.
#[derive(Debug)]
pub enum M5GovernanceComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GovernanceComponentMatrixViolation>),
}

impl fmt::Display for M5GovernanceComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 governance-component matrix export parse failed: {error}"
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
                    "m5 governance-component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GovernanceComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5GovernanceComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GovernanceComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required component is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component claiming Stable is missing required evidence packet refs.
    StableComponentMissingEvidence,
    /// A component has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component does not name its advisory/authoritative enforcement distinction.
    EnforcementDistinctionMissing,
    /// A component does not carry a governance-state vocabulary.
    GovernanceStateVocabMissing,
    /// A component does not name its escalation boundary.
    EscalationBoundaryMissing,
    /// A component does not name its backup-coverage fallback.
    BackupCoverageFallbackMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5GovernanceComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::StableComponentMissingEvidence => "stable_component_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::EnforcementDistinctionMissing => "enforcement_distinction_missing",
            Self::GovernanceStateVocabMissing => "governance_state_vocab_missing",
            Self::EscalationBoundaryMissing => "escalation_boundary_missing",
            Self::BackupCoverageFallbackMissing => "backup_coverage_fallback_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 governance-component matrix export.
pub fn current_stable_m5_governance_component_matrix_export(
) -> Result<M5GovernanceComponentMatrixPacket, M5GovernanceComponentMatrixArtifactError> {
    let packet: M5GovernanceComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-protected-path-governance-proof/support_export.json"
    )))
    .map_err(M5GovernanceComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GovernanceComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5GovernanceComponentMatrixPacket,
    violations: &mut Vec<M5GovernanceComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GovernanceComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_component_rows(
    packet: &M5GovernanceComponentMatrixPacket,
    violations: &mut Vec<M5GovernanceComponentMatrixViolation>,
) {
    let present: BTreeSet<M5GovernanceComponent> = packet
        .component_rows
        .iter()
        .map(|row| row.component)
        .collect();
    for required in M5GovernanceComponent::ALL {
        if !present.contains(&required) {
            violations.push(M5GovernanceComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::ComponentRowIncomplete);
        }
        if row.maturity.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::StableComponentMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.enforcement_distinction.trim().is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::EnforcementDistinctionMissing);
        }
        if row.governance_state_vocab.is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::GovernanceStateVocabMissing);
        }
        if row.escalation_boundary.trim().is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::EscalationBoundaryMissing);
        }
        if row.backup_coverage_fallback.trim().is_empty() {
            violations.push(M5GovernanceComponentMatrixViolation::BackupCoverageFallbackMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5GovernanceComponentMatrixPacket,
    violations: &mut Vec<M5GovernanceComponentMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.advisory_never_masquerades_as_authoritative,
        review.provider_authoritative_versus_local_estimate_distinct,
        review.owner_coverage_backup_missing_explicit,
        review.approver_expired_waived_stale_explicit,
        review.review_pack_freshness_and_parity_explicit,
        review.public_surface_diff_machine_generated_required,
        review.migration_evidence_required_for_public_surface_change,
        review.protection_reason_always_explicit,
        review.dri_coverage_gap_explicit,
        review.merge_control_blocker_never_generic,
        review.escalation_handoff_explicit,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5GovernanceComponentMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5GovernanceComponentMatrixPacket,
    violations: &mut Vec<M5GovernanceComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.protected_path_row_shows_reason_and_enforcement_authority,
        projection.ownership_card_shows_owner_source_and_coverage,
        projection.approver_matrix_shows_required_and_state,
        projection.review_pack_summary_shows_freshness_and_parity,
        projection.public_surface_diff_card_shows_change_class_and_diff,
        projection.merge_control_banner_shows_blockers_not_generic,
        projection.dri_registry_row_shows_dri_and_coverage,
        projection.merge_readiness_strip_shows_blocking_and_ownership,
        projection.cli_headless_shows_component_truth,
        projection.support_export_shows_component_truth,
    ] {
        if !ok {
            violations.push(M5GovernanceComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5GovernanceComponentMatrixPacket,
    violations: &mut Vec<M5GovernanceComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5GovernanceComponentMatrixViolation::ProofFreshnessIncomplete);
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
