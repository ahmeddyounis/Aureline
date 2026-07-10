//! Two reusable M5 protected-path governance components — the approver matrix and the review-pack
//! summary — so a user can tell *which* approvers are required, *where* that requirement came from,
//! whether each approval is satisfied, pending, waived, or expired, whether a review pack was
//! evaluated locally, is provider-authoritative, is CI-only, was not evaluated here, or is stale
//! relative to base/head, and which checks or waivers are suppressed, before they trust, merge, or
//! release a governed change.
//!
//! Aureline's frozen protected-path governance component matrix
//! ([`crate::freeze_the_m5_protected_path_governance_component_matrix`]) names the approver matrix
//! and the review-pack summary as two governed component families and freezes their controlled
//! vocabulary — the one governance-state lexicon ([`M5GovernanceStateVocab`]): `advisory`,
//! `authoritative`, `covered`, `backup_missing`, `waived`, `expired`, `stale`,
//! `provider_authoritative`, and `local_estimate`. This module *implements* that contract as two
//! co-equal component vectors — a full approver matrix and a review-pack summary — that reuse the one
//! frozen lexicon and share one evaluation-locus resolver so a CI-only or local-only evaluation can
//! never masquerade as the provider's authoritative gate, and a waived or expired approval can never
//! collapse into generic `approved` language.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_evaluation_locus`] — takes an evaluation-locus source and derives the exact locus
//!   posture (provider-authoritative, local-only, CI-only, not-evaluated-here, or stale relative to
//!   base/head), whether the posture is the provider's authoritative gate, whether it was evaluated
//!   here at all, whether it is stale, and which frozen governance-state token it maps to — so a
//!   local-only or CI-only evaluation can never read as the provider's final gate, and a
//!   not-evaluated-here pack can never read as evaluated. Both the approver matrix and the review-pack
//!   summary use it, so their local-versus-provider parity stays one truth.
//! * [`resolve_approver_state`] — takes an approver-state source and derives the exact approver state
//!   (satisfied, pending, waived, or expired), whether the state is clean-satisfied, whether it needs
//!   an expiry, and which frozen governance-state token it maps to — so a waived or expired approval
//!   degrades explicitly under its own `waived` / `expired` token instead of collapsing into generic
//!   `approved` language.
//!
//! A single controls packet — [`ApproverReviewPackControlsPacket`] — binds one vector of approver
//! matrix rows and one vector of review-pack summaries to the same evaluation-locus, approver-state,
//! and suppression vocabulary, so requirement source, approver state, local-versus-provider parity,
//! suppressed-check visibility, and freshness stay explicit across the review-workspace,
//! release-candidate, governance, shiproom, CLI, and support-export consumers.
//!
//! The governance component ([`M5GovernanceComponent`]), governance-state vocabulary
//! ([`M5GovernanceStateVocab`]), downgrade trigger
//! ([`M5GovernanceComponentDowngradeTrigger`]), rollback posture
//! ([`M5GovernanceComponentRollbackPosture`]), and consumer surface
//! ([`M5GovernanceComponentConsumerSurface`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the evaluation-locus source, the derived locus posture, the approver-state source, the
//! derived approver state, the requirement-source class, the evidence-link kind, the pack capability
//! set, the pack suppression class, and the bounded row and summary actions. No M5 governed surface
//! invents a second approver or review-pack grammar.
//!
//! Raw approval logs, raw provider payloads, raw review-pack bodies, person-specific private contact
//! detail, credentials, and secrets stay outside the export boundary; every approver is carried only
//! as an export-safe role alias, and every evidence and pack reference is carried only as an opaque,
//! export-safe reference.

#[cfg(test)]
mod tests;

// The governance component family, the frozen governance-state lexicon, and the downgrade /
// rollback / consumer vocabularies are frozen once, in the protected-path governance component
// matrix. This lane reuses them verbatim so it never invents a parallel approver or review-pack
// vocabulary.
pub use crate::freeze_the_m5_protected_path_governance_component_matrix::{
    M5GovernanceComponent, M5GovernanceComponentConsumerSurface,
    M5GovernanceComponentDowngradeTrigger, M5GovernanceComponentRollbackPosture,
    M5GovernanceStateVocab, M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ApproverReviewPackControlsPacket`].
pub const APPROVER_REVIEW_PACK_CONTROLS_RECORD_KIND: &str =
    "implement_approver_matrices_and_review_pack_summaries_with_requirement_source_approval_state_local_versus_provider_evaluation_parity_suppressed_check_visibility_and_freshness_truth";

/// Schema version for M5 approver-matrix / review-pack-summary control records.
pub const APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-approver-review-pack-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const APPROVER_REVIEW_PACK_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_approver_matrices_and_review_pack_summaries.md";

/// Repo-relative path of the protected fixture directory.
pub const APPROVER_REVIEW_PACK_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-approver-review-pack-controls";

/// Repo-relative path of the checked support-export artifact.
pub const APPROVER_REVIEW_PACK_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-approver-review-pack-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const APPROVER_REVIEW_PACK_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-approver-review-pack-controls-proof/summary.md";

// ---- shared evaluation-locus vocabulary ----------------------------------

/// The source an evaluation-locus signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_evaluation_locus`]: it names whether a governed evaluation
/// (a required approval, or a whole review pack) was performed by the provider, reported by the
/// provider, evaluated only locally, reported only by CI, not evaluated here at all, or is stale
/// against the current base/head — so a local-only or CI-only evaluation can never be asserted as the
/// provider's authoritative gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationLocusSource {
    /// The provider enforces the gate authoritatively.
    ProviderEnforcedGate,
    /// The provider reports the status authoritatively.
    ProviderReportedStatus,
    /// Aureline evaluated the signal only locally.
    LocalEvaluationOnly,
    /// The signal is reported only by CI, not the provider's final gate.
    CiReportedOnly,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The evaluation is stale relative to the current base/head.
    StaleAgainstBaseHead,
}

impl EvaluationLocusSource {
    /// Every evaluation-locus source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderEnforcedGate,
        Self::ProviderReportedStatus,
        Self::LocalEvaluationOnly,
        Self::CiReportedOnly,
        Self::NotEvaluatedHere,
        Self::StaleAgainstBaseHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderEnforcedGate => "provider_enforced_gate",
            Self::ProviderReportedStatus => "provider_reported_status",
            Self::LocalEvaluationOnly => "local_evaluation_only",
            Self::CiReportedOnly => "ci_reported_only",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleAgainstBaseHead => "stale_against_base_head",
        }
    }
}

/// Derived evaluation-locus posture an approver matrix row or review-pack summary may present.
///
/// This is the AC-pinned local-versus-provider parity axis: the posture is derived from the frozen
/// locus source, never asserted, so a user can tell whether a signal is local-only, provider
/// authoritative, CI-only, not evaluated here, or stale relative to base/head — without opening raw
/// logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationLocusPosture {
    /// The provider authoritatively gates the signal.
    ProviderAuthoritative,
    /// The signal was evaluated only locally.
    LocalOnly,
    /// The signal is reported only by CI.
    CiOnly,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The signal is stale relative to the current base/head.
    StaleRelativeToHead,
}

impl EvaluationLocusPosture {
    /// Every evaluation-locus posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAuthoritative,
        Self::LocalOnly,
        Self::CiOnly,
        Self::NotEvaluatedHere,
        Self::StaleRelativeToHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocalOnly => "local_only",
            Self::CiOnly => "ci_only",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleRelativeToHead => "stale_relative_to_head",
        }
    }

    /// True only when the provider authoritatively gates the signal.
    pub const fn is_provider_authoritative(self) -> bool {
        matches!(self, Self::ProviderAuthoritative)
    }

    /// True when the signal was evaluated only locally.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnly)
    }

    /// True when the signal is reported only by CI.
    pub const fn is_ci_only(self) -> bool {
        matches!(self, Self::CiOnly)
    }

    /// True when the signal was evaluated on this build at all (stale counts as evaluated-but-old).
    pub const fn is_evaluated_here(self) -> bool {
        !matches!(self, Self::NotEvaluatedHere)
    }

    /// True when the signal is stale relative to the current base/head.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleRelativeToHead)
    }

    /// The frozen governance-state token this posture must render under, if the frozen lexicon names
    /// one. `ci_only` and `not_evaluated_here` are honest states the frozen lexicon does not name, so
    /// they carry no governance token and must never borrow another state's label.
    pub const fn governance_vocab(self) -> Option<M5GovernanceStateVocab> {
        match self {
            Self::ProviderAuthoritative => Some(M5GovernanceStateVocab::ProviderAuthoritative),
            Self::LocalOnly => Some(M5GovernanceStateVocab::LocalEstimate),
            Self::StaleRelativeToHead => Some(M5GovernanceStateVocab::Stale),
            Self::CiOnly | Self::NotEvaluatedHere => None,
        }
    }
}

/// Locus disclosures a component must carry, derived from the evaluation-locus source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationLocusDisclosure {
    /// The derived locus posture this component may present.
    pub posture: EvaluationLocusPosture,
    /// Whether the provider authoritatively gates the signal.
    pub is_provider_authoritative: bool,
    /// Whether the signal was evaluated only locally.
    pub is_local_only: bool,
    /// Whether the signal is reported only by CI.
    pub is_ci_only: bool,
    /// Whether the signal was evaluated on this build at all.
    pub is_evaluated_here: bool,
    /// Whether the signal is stale relative to the current base/head.
    pub is_stale: bool,
    /// Whether the component must carry an explicit local-only note.
    pub needs_local_only_note: bool,
    /// Whether the component must carry an explicit CI-only note.
    pub needs_ci_only_note: bool,
    /// Whether the component must carry an explicit not-evaluated-here note.
    pub needs_not_evaluated_note: bool,
    /// Whether the component must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// The frozen governance-state token this posture must render under, if any.
    pub governance_vocab: Option<M5GovernanceStateVocab>,
}

/// Resolves the evaluation-locus posture an approver matrix row or review-pack summary may present.
///
/// A `provider_enforced_gate` or `provider_reported_status` source is provider-authoritative; a
/// `local_evaluation_only` source is local-only; a `ci_reported_only` source is CI-only; a
/// `not_evaluated_here` source is not-evaluated-here; and a `stale_against_base_head` source is stale
/// relative to base/head — so a local-only or CI-only evaluation can never read as the provider's
/// final gate.
pub fn resolve_evaluation_locus(source: EvaluationLocusSource) -> EvaluationLocusDisclosure {
    use EvaluationLocusPosture as Posture;
    use EvaluationLocusSource as Src;

    let posture = match source {
        Src::ProviderEnforcedGate | Src::ProviderReportedStatus => Posture::ProviderAuthoritative,
        Src::LocalEvaluationOnly => Posture::LocalOnly,
        Src::CiReportedOnly => Posture::CiOnly,
        Src::NotEvaluatedHere => Posture::NotEvaluatedHere,
        Src::StaleAgainstBaseHead => Posture::StaleRelativeToHead,
    };

    EvaluationLocusDisclosure {
        posture,
        is_provider_authoritative: posture.is_provider_authoritative(),
        is_local_only: posture.is_local_only(),
        is_ci_only: posture.is_ci_only(),
        is_evaluated_here: posture.is_evaluated_here(),
        is_stale: posture.is_stale(),
        needs_local_only_note: posture.is_local_only(),
        needs_ci_only_note: posture.is_ci_only(),
        needs_not_evaluated_note: !posture.is_evaluated_here(),
        needs_stale_note: posture.is_stale(),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- approver-state vocabulary -------------------------------------------

/// The source an approver-state signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_approver_state`]: it names whether a required approval was
/// recorded, provider-confirmed, is still awaited, has changes requested, was waived by policy, or has
/// expired — so a waived or expired approval can never collapse into generic `approved` language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApproverStateSource {
    /// A required approval was recorded.
    RequiredApprovalRecorded,
    /// The provider confirmed the approval.
    ProviderConfirmedApproval,
    /// A required approval is still awaited.
    AwaitingRequiredApproval,
    /// Changes were requested; the approval is pending.
    ChangesRequestedPending,
    /// The approval was explicitly waived by policy.
    ApprovalWaivedByPolicy,
    /// A previously granted approval has expired.
    ApprovalExpired,
}

impl ApproverStateSource {
    /// Every approver-state source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RequiredApprovalRecorded,
        Self::ProviderConfirmedApproval,
        Self::AwaitingRequiredApproval,
        Self::ChangesRequestedPending,
        Self::ApprovalWaivedByPolicy,
        Self::ApprovalExpired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredApprovalRecorded => "required_approval_recorded",
            Self::ProviderConfirmedApproval => "provider_confirmed_approval",
            Self::AwaitingRequiredApproval => "awaiting_required_approval",
            Self::ChangesRequestedPending => "changes_requested_pending",
            Self::ApprovalWaivedByPolicy => "approval_waived_by_policy",
            Self::ApprovalExpired => "approval_expired",
        }
    }
}

/// Derived approver-state posture an approver matrix row may present.
///
/// This is the AC-pinned approver-honesty axis: only [`ApproverStatePosture::Satisfied`] is a clean
/// satisfied approval — a waived or expired approval degrades explicitly under its own token and never
/// collapses into generic `approved` language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApproverStatePosture {
    /// The required approval is satisfied.
    Satisfied,
    /// The required approval is pending.
    Pending,
    /// The required approval is explicitly waived.
    Waived,
    /// The required approval has expired.
    Expired,
}

impl ApproverStatePosture {
    /// Every approver-state posture, in declaration order.
    pub const ALL: [Self; 4] = [Self::Satisfied, Self::Pending, Self::Waived, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Pending => "pending",
            Self::Waived => "waived",
            Self::Expired => "expired",
        }
    }

    /// True only when the required approval is cleanly satisfied.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// True when the required approval is pending.
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Pending)
    }

    /// True when the required approval is waived.
    pub const fn is_waived(self) -> bool {
        matches!(self, Self::Waived)
    }

    /// True when the required approval has expired.
    pub const fn is_expired(self) -> bool {
        matches!(self, Self::Expired)
    }

    /// True when the approver state carries a relevant expiry (waived or expired approvals).
    pub const fn has_expiry(self) -> bool {
        matches!(self, Self::Waived | Self::Expired)
    }

    /// The frozen governance-state token this posture must render under, if the frozen lexicon names
    /// one. `satisfied` and `pending` are honest states the frozen lexicon does not name, so they
    /// carry no governance token and must never borrow the `approved`-style label of another state.
    pub const fn governance_vocab(self) -> Option<M5GovernanceStateVocab> {
        match self {
            Self::Waived => Some(M5GovernanceStateVocab::Waived),
            Self::Expired => Some(M5GovernanceStateVocab::Expired),
            Self::Satisfied | Self::Pending => None,
        }
    }
}

/// Approver-state disclosures an approver matrix row must carry, derived from the approver-state
/// source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApproverStateDisclosure {
    /// The derived approver-state posture this row may present.
    pub posture: ApproverStatePosture,
    /// Whether the required approval is cleanly satisfied.
    pub is_satisfied: bool,
    /// Whether the required approval is pending.
    pub is_pending: bool,
    /// Whether the required approval is waived.
    pub is_waived: bool,
    /// Whether the required approval has expired.
    pub is_expired: bool,
    /// Whether the row must carry an explicit waived note.
    pub needs_waived_note: bool,
    /// Whether the row must carry an explicit expired note.
    pub needs_expired_note: bool,
    /// Whether the row must carry an explicit pending note.
    pub needs_pending_note: bool,
    /// Whether the row must carry an explicit expiry label.
    pub needs_expiry_label: bool,
    /// The frozen governance-state token this posture must render under, if any.
    pub governance_vocab: Option<M5GovernanceStateVocab>,
}

/// Resolves the approver-state posture an approver matrix row may present.
///
/// A `required_approval_recorded` or `provider_confirmed_approval` source is satisfied; an
/// `awaiting_required_approval` or `changes_requested_pending` source is pending; an
/// `approval_waived_by_policy` source is waived; and an `approval_expired` source is expired — so a
/// waived or expired approval can never collapse into generic `approved` language.
pub fn resolve_approver_state(source: ApproverStateSource) -> ApproverStateDisclosure {
    use ApproverStatePosture as Posture;
    use ApproverStateSource as Src;

    let posture = match source {
        Src::RequiredApprovalRecorded | Src::ProviderConfirmedApproval => Posture::Satisfied,
        Src::AwaitingRequiredApproval | Src::ChangesRequestedPending => Posture::Pending,
        Src::ApprovalWaivedByPolicy => Posture::Waived,
        Src::ApprovalExpired => Posture::Expired,
    };

    ApproverStateDisclosure {
        posture,
        is_satisfied: posture.is_satisfied(),
        is_pending: posture.is_pending(),
        is_waived: posture.is_waived(),
        is_expired: posture.is_expired(),
        needs_waived_note: posture.is_waived(),
        needs_expired_note: posture.is_expired(),
        needs_pending_note: posture.is_pending(),
        needs_expiry_label: posture.has_expiry(),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- approver-matrix-specific vocabulary ---------------------------------

/// The class a required approval's requirement source belongs to, so a matrix row names *where* the
/// approval requirement came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementSourceClass {
    /// A provider branch-protection rule requires the approval.
    BranchProtectionRule,
    /// A CODEOWNERS rule requires the approval.
    CodeownersRule,
    /// A local review-policy rule requires the approval.
    ReviewPolicyRule,
    /// A manual review request created the requirement.
    ManualReviewRequest,
    /// The requirement source could not be resolved.
    Unresolved,
}

impl RequirementSourceClass {
    /// Every requirement-source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BranchProtectionRule,
        Self::CodeownersRule,
        Self::ReviewPolicyRule,
        Self::ManualReviewRequest,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BranchProtectionRule => "branch_protection_rule",
            Self::CodeownersRule => "codeowners_rule",
            Self::ReviewPolicyRule => "review_policy_rule",
            Self::ManualReviewRequest => "manual_review_request",
            Self::Unresolved => "unresolved",
        }
    }
}

/// The kind of stable evidence link an approver matrix row can open, so a required approval always
/// names the exact evidence a user can reopen — never an anonymous check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLinkKind {
    /// A provider approval record.
    ProviderApprovalRecord,
    /// A CI check run.
    CiCheckRun,
    /// A local evaluation record.
    LocalEvaluationRecord,
    /// A waiver record.
    WaiverRecord,
    /// No evidence link is bound (the row names that it routes nowhere).
    NoEvidenceLink,
}

impl EvidenceLinkKind {
    /// Every evidence-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderApprovalRecord,
        Self::CiCheckRun,
        Self::LocalEvaluationRecord,
        Self::WaiverRecord,
        Self::NoEvidenceLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderApprovalRecord => "provider_approval_record",
            Self::CiCheckRun => "ci_check_run",
            Self::LocalEvaluationRecord => "local_evaluation_record",
            Self::WaiverRecord => "waiver_record",
            Self::NoEvidenceLink => "no_evidence_link",
        }
    }

    /// True when this kind names a resolvable evidence link.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoEvidenceLink)
    }
}

/// One keyboard-complete default action an approver matrix row offers.
///
/// `OpenEvidenceLink`, `InspectRequirementSource`, and `ReviewApproverState` are always offered so the
/// exact evidence, the requirement source, and the approver state stay inspectable before a user
/// trusts the sign-off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApproverMatrixAction {
    /// Open the exact evidence link (always available).
    OpenEvidenceLink,
    /// Inspect the requirement source (always available).
    InspectRequirementSource,
    /// Review the approver state (always available).
    ReviewApproverState,
    /// Inspect the evaluation parity (local versus provider).
    InspectEvaluationParity,
    /// Review the expiry.
    ReviewExpiry,
    /// Copy the export-safe approver role aliases.
    CopyApproverRoles,
}

impl ApproverMatrixAction {
    /// Every approver-matrix action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenEvidenceLink,
        Self::InspectRequirementSource,
        Self::ReviewApproverState,
        Self::InspectEvaluationParity,
        Self::ReviewExpiry,
        Self::CopyApproverRoles,
    ];

    /// The default actions every keyboard-complete approver matrix row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenEvidenceLink,
        Self::InspectRequirementSource,
        Self::ReviewApproverState,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenEvidenceLink => "open_evidence_link",
            Self::InspectRequirementSource => "inspect_requirement_source",
            Self::ReviewApproverState => "review_approver_state",
            Self::InspectEvaluationParity => "inspect_evaluation_parity",
            Self::ReviewExpiry => "review_expiry",
            Self::CopyApproverRoles => "copy_approver_roles",
        }
    }
}

// ---- review-pack-summary-specific vocabulary -----------------------------

/// One capability a review pack evaluated, so a summary names *what* the pack covers rather than
/// implying it evaluated everything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackCapability {
    /// Ownership / approval evaluation.
    OwnershipApproval,
    /// Protected-path gate evaluation.
    ProtectedPathGate,
    /// Public-surface diff evaluation.
    PublicSurfaceDiff,
    /// CI status rollup.
    CiStatusRollup,
    /// Policy-gate evaluation.
    PolicyGate,
}

impl PackCapability {
    /// Every pack capability, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OwnershipApproval,
        Self::ProtectedPathGate,
        Self::PublicSurfaceDiff,
        Self::CiStatusRollup,
        Self::PolicyGate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnershipApproval => "ownership_approval",
            Self::ProtectedPathGate => "protected_path_gate",
            Self::PublicSurfaceDiff => "public_surface_diff",
            Self::CiStatusRollup => "ci_status_rollup",
            Self::PolicyGate => "policy_gate",
        }
    }
}

/// The class a suppressed check or waiver belongs to, so a summary names *why* a check is not blocking
/// rather than silently hiding it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSuppressionClass {
    /// A required approval was waived.
    WaivedApproval,
    /// A check was skipped.
    SkippedCheck,
    /// A check was suppressed by policy.
    PolicySuppressed,
    /// A provider check was excluded from the local evaluation.
    ProviderExcluded,
}

impl PackSuppressionClass {
    /// Every suppression class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::WaivedApproval,
        Self::SkippedCheck,
        Self::PolicySuppressed,
        Self::ProviderExcluded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaivedApproval => "waived_approval",
            Self::SkippedCheck => "skipped_check",
            Self::PolicySuppressed => "policy_suppressed",
            Self::ProviderExcluded => "provider_excluded",
        }
    }
}

/// One suppressed check or waiver a review-pack summary makes explicit, so a guarded merge never hides
/// a suppressed check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressedCheck {
    /// The suppressed check or waiver label; required and non-empty.
    pub check_label: String,
    /// The class this suppression belongs to.
    pub suppression_class: PackSuppressionClass,
    /// The reason the check is suppressed; required and non-empty so it never reads as a silent pass.
    pub reason_label: String,
}

/// One keyboard-complete default action a review-pack summary offers.
///
/// `InspectEvaluationParity`, `ReviewSuppressedChecks`, and `OpenPackDigest` are always offered so the
/// local-versus-provider parity, the suppressed checks, and the pack digest stay inspectable before a
/// user trusts the pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSummaryAction {
    /// Inspect the evaluation parity (always available).
    InspectEvaluationParity,
    /// Review the suppressed checks or waivers (always available).
    ReviewSuppressedChecks,
    /// Open the pack digest (always available).
    OpenPackDigest,
    /// Review the base/head identity.
    ReviewBaseHeadIdentity,
    /// Inspect the capability set.
    InspectCapabilitySet,
    /// Copy the export-safe pack digest.
    CopyPackDigest,
}

impl ReviewPackSummaryAction {
    /// Every review-pack-summary action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectEvaluationParity,
        Self::ReviewSuppressedChecks,
        Self::OpenPackDigest,
        Self::ReviewBaseHeadIdentity,
        Self::InspectCapabilitySet,
        Self::CopyPackDigest,
    ];

    /// The default actions every keyboard-complete review-pack summary must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectEvaluationParity,
        Self::ReviewSuppressedChecks,
        Self::OpenPackDigest,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectEvaluationParity => "inspect_evaluation_parity",
            Self::ReviewSuppressedChecks => "review_suppressed_checks",
            Self::OpenPackDigest => "open_pack_digest",
            Self::ReviewBaseHeadIdentity => "review_base_head_identity",
            Self::InspectCapabilitySet => "inspect_capability_set",
            Self::CopyPackDigest => "copy_pack_digest",
        }
    }
}

// ---- component structs ---------------------------------------------------

/// An approver matrix row naming its approver role, requirement source, satisfied/pending/waived/
/// expired state, local-versus-provider evaluation parity, evidence link, and expiry where relevant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproverMatrixRow {
    /// Frozen component this control implements; must be `approver_matrix`.
    pub component: M5GovernanceComponent,
    /// Stable row id.
    pub row_id: String,
    /// Approver role alias; required and non-empty, an export-safe role alias not a person's name.
    pub approver_role_label: String,
    /// Requirement-source class.
    pub requirement_source_class: RequirementSourceClass,
    /// Requirement-source label; required and non-empty so where the requirement came from is
    /// explicit.
    pub requirement_source_label: String,
    /// Evaluation-locus source, resolved into the locus posture.
    pub evaluation_locus_source: EvaluationLocusSource,
    /// Derived evaluation-locus posture (must equal the resolved posture).
    pub derived_evaluation_locus: EvaluationLocusPosture,
    /// Whether the row claims provider-authoritative evaluation (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Approver-state source, resolved into the approver-state posture.
    pub approver_state_source: ApproverStateSource,
    /// Derived approver-state posture (must equal the resolved posture).
    pub derived_approver_state: ApproverStatePosture,
    /// Whether the row claims a clean satisfied approval (must equal derived truth).
    pub claims_satisfied: bool,
    /// Frozen governance-state vocabulary this row renders (must include the derived tokens).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-only note; required when the evaluation is local-only.
    pub local_only_note: String,
    /// CI-only note; required when the evaluation is CI-only.
    pub ci_only_note: String,
    /// Not-evaluated-here note; required when the evaluation was not performed on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the evaluation is stale relative to base/head.
    pub stale_note: String,
    /// Waived note; required when the approval is waived.
    pub waived_note: String,
    /// Expired note; required when the approval has expired.
    pub expired_note: String,
    /// Pending note; required when the approval is pending.
    pub pending_note: String,
    /// Expiry label; required when the approver state carries a relevant expiry.
    pub expiry_label: String,
    /// Kind of stable evidence link this row can open.
    pub evidence_link_kind: EvidenceLinkKind,
    /// Opaque stable evidence-link reference; required when the kind resolves.
    pub evidence_link_ref: String,
    /// Context note; always required so the row names what to check before trusting the sign-off.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub row_actions: Vec<ApproverMatrixAction>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its requirement source or approver state. MUST be `false`.
    pub hides_requirement_source_or_state: bool,
    /// Hard invariant: never lets a waived or expired approval read as satisfied. MUST be `false`.
    pub lets_waived_or_expired_read_as_satisfied: bool,
    /// Hard invariant: never lets a CI-only or local-only evaluation read as provider-authoritative.
    /// MUST be `false`.
    pub lets_ci_or_local_read_as_provider_authoritative: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ApproverMatrixRow {
    /// Locus disclosures this row must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> EvaluationLocusDisclosure {
        resolve_evaluation_locus(self.evaluation_locus_source)
    }

    /// Approver-state disclosures this row must carry, derived from the frozen source.
    pub fn approver_disclosure(&self) -> ApproverStateDisclosure {
        resolve_approver_state(self.approver_state_source)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ApproverMatrixAction> = self.row_actions.iter().copied().collect();
        ApproverMatrixAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

/// A review-pack summary naming its pack digest, base/head identity, capability set, local-versus-
/// provider evaluation parity, evaluation freshness, and suppressed checks or waivers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSummary {
    /// Frozen component this control implements; must be `review_pack_summary`.
    pub component: M5GovernanceComponent,
    /// Stable summary id.
    pub summary_id: String,
    /// Pack digest label; required and non-empty, an opaque export-safe digest reference.
    pub pack_digest_label: String,
    /// Base identity label; required and non-empty.
    pub base_identity_label: String,
    /// Head identity label; required and non-empty.
    pub head_identity_label: String,
    /// Capability set this pack evaluated; required and non-empty.
    pub capability_set: Vec<PackCapability>,
    /// Capability-set label; required and non-empty so what the pack covers stays explicit.
    pub capability_set_label: String,
    /// Evaluation-locus source, resolved into the locus posture.
    pub evaluation_locus_source: EvaluationLocusSource,
    /// Derived evaluation-locus posture (must equal the resolved posture).
    pub derived_evaluation_locus: EvaluationLocusPosture,
    /// Whether the summary claims provider-authoritative evaluation (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Whether the summary claims it was evaluated here (must equal derived truth).
    pub claims_evaluated_here: bool,
    /// Parity label; always required so the local-versus-provider parity stays explicit.
    pub parity_label: String,
    /// Freshness label; always required so evaluation freshness relative to base/head stays explicit.
    pub freshness_label: String,
    /// Frozen governance-state vocabulary this summary renders (must include the derived token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-only note; required when the evaluation is local-only.
    pub local_only_note: String,
    /// CI-only note; required when the evaluation is CI-only.
    pub ci_only_note: String,
    /// Not-evaluated-here note; required when the evaluation was not performed on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the evaluation is stale relative to base/head.
    pub stale_note: String,
    /// Suppressed checks or waivers this summary makes explicit.
    pub suppressed_checks: Vec<SuppressedCheck>,
    /// Suppressed-checks label; always required so suppressed-check visibility stays explicit.
    pub suppressed_checks_label: String,
    /// Context note; always required so the summary names what to check before trusting the pack.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub summary_actions: Vec<ReviewPackSummaryAction>,
    /// Downgrade triggers this summary can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this summary.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this summary.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its parity or freshness. MUST be `false`.
    pub hides_parity_or_freshness: bool,
    /// Hard invariant: never lets a CI-only or local-only evaluation read as provider-authoritative.
    /// MUST be `false`.
    pub lets_ci_or_local_read_as_provider_authoritative: bool,
    /// Hard invariant: never hides its suppressed checks or waivers. MUST be `false`.
    pub hides_suppressed_checks_or_waivers: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ReviewPackSummary {
    /// Locus disclosures this summary must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> EvaluationLocusDisclosure {
        resolve_evaluation_locus(self.evaluation_locus_source)
    }

    /// Whether the summary offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ReviewPackSummaryAction> =
            self.summary_actions.iter().copied().collect();
        ReviewPackSummaryAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- review blocks -------------------------------------------------------

/// First-glance approver-matrix / review-pack review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproverReviewPackReview {
    /// The approver matrix names each requirement source and approver state.
    pub approver_matrix_shows_requirement_source_and_state: bool,
    /// The approver matrix names each approver role and its evidence link.
    pub approver_matrix_names_role_and_evidence: bool,
    /// The approver matrix offers an open-evidence-link action.
    pub approver_matrix_offers_open_evidence_link: bool,
    /// The review-pack summary names its pack digest and base/head identity.
    pub review_pack_summary_shows_digest_and_base_head: bool,
    /// The review-pack summary names its parity and freshness.
    pub review_pack_summary_shows_parity_and_freshness: bool,
    /// The review-pack summary lists its suppressed checks and waivers.
    pub review_pack_summary_lists_suppressed_checks_and_waivers: bool,
    /// Evaluation-locus parity is derived from state, never asserted.
    pub evaluation_locus_derived_never_asserted: bool,
    /// A CI-only or local-only evaluation is never shown as provider-authoritative.
    pub ci_or_local_never_shown_as_provider_authoritative: bool,
    /// A not-evaluated-here pack is never shown as evaluated.
    pub not_evaluated_here_never_shown_as_evaluated: bool,
    /// A waived or expired approval is never shown as satisfied.
    pub waived_or_expired_never_shown_as_satisfied: bool,
    /// Staleness relative to base/head stays explicit.
    pub stale_relative_to_base_head_always_explicit: bool,
    /// Approver roles use export-safe role aliases, never person-specific contact detail.
    pub approver_roles_use_export_safe_aliases: bool,
    /// The pack capability set stays explicit.
    pub capability_set_always_explicit: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ApproverReviewPackReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.approver_matrix_shows_requirement_source_and_state
            && self.approver_matrix_names_role_and_evidence
            && self.approver_matrix_offers_open_evidence_link
            && self.review_pack_summary_shows_digest_and_base_head
            && self.review_pack_summary_shows_parity_and_freshness
            && self.review_pack_summary_lists_suppressed_checks_and_waivers
            && self.evaluation_locus_derived_never_asserted
            && self.ci_or_local_never_shown_as_provider_authoritative
            && self.not_evaluated_here_never_shown_as_evaluated
            && self.waived_or_expired_never_shown_as_satisfied
            && self.stale_relative_to_base_head_always_explicit
            && self.approver_roles_use_export_safe_aliases
            && self.capability_set_always_explicit
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproverReviewPackConsumerProjection {
    /// The review-workspace surface reads a single canonical source.
    pub review_workspace_reads_single_source: bool,
    /// The release-candidate surface reads a single canonical source.
    pub release_candidate_reads_single_source: bool,
    /// The governance and shiproom surfaces read a single canonical source.
    pub governance_and_shiproom_read_single_source: bool,
    /// Requirement source and approver state are visible before a sign-off feels safe.
    pub requirement_and_state_visible_before_signoff: bool,
    /// Parity and freshness are visible before a merge or release feels safe.
    pub parity_and_freshness_visible_before_signoff: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ApproverReviewPackConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reads_single_source
            && self.release_candidate_reads_single_source
            && self.governance_and_shiproom_read_single_source
            && self.requirement_and_state_visible_before_signoff
            && self.parity_and_freshness_visible_before_signoff
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproverReviewPackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ApproverReviewPackControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproverReviewPackControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Approver matrix rows.
    pub approver_matrix_rows: Vec<ApproverMatrixRow>,
    /// Review-pack summaries.
    pub review_pack_summaries: Vec<ReviewPackSummary>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Approver-matrix / review-pack review block.
    pub review: ApproverReviewPackReview,
    /// Consumer projection block.
    pub consumer_projection: ApproverReviewPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ApproverReviewPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe approver-matrix / review-pack-summary controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApproverReviewPackControlsPacket {
    /// Record kind; must equal [`APPROVER_REVIEW_PACK_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Approver matrix rows.
    pub approver_matrix_rows: Vec<ApproverMatrixRow>,
    /// Review-pack summaries.
    pub review_pack_summaries: Vec<ReviewPackSummary>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Approver-matrix / review-pack review block.
    pub review: ApproverReviewPackReview,
    /// Consumer projection block.
    pub consumer_projection: ApproverReviewPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ApproverReviewPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ApproverReviewPackControlsPacket {
    /// Builds an approver-matrix / review-pack-summary controls packet from stable-lane input.
    pub fn new(input: ApproverReviewPackControlsPacketInput) -> Self {
        Self {
            record_kind: APPROVER_REVIEW_PACK_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            approver_matrix_rows: input.approver_matrix_rows,
            review_pack_summaries: input.review_pack_summaries,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the approver-matrix / review-pack-summary control invariants.
    pub fn validate(&self) -> Vec<ApproverReviewPackControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != APPROVER_REVIEW_PACK_CONTROLS_RECORD_KIND {
            violations.push(ApproverReviewPackControlsViolation::WrongRecordKind);
        }
        if self.schema_version != APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_VERSION {
            violations.push(ApproverReviewPackControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ApproverReviewPackControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ApproverReviewPackControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ApproverReviewPackControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_approver_matrix_rows(self, &mut violations);
        validate_review_pack_summaries(self, &mut violations);
        validate_shared_coverage(self, &mut violations);

        if !self.review.all_hold() {
            violations.push(ApproverReviewPackControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ApproverReviewPackControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ApproverReviewPackControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("approver review-pack controls packet serializes"),
        ) {
            violations.push(ApproverReviewPackControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("approver review-pack controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let waived_or_expired = self
            .approver_matrix_rows
            .iter()
            .filter(|row| {
                let disclosure = row.approver_disclosure();
                disclosure.is_waived || disclosure.is_expired
            })
            .count();
        let not_provider_authoritative = self
            .review_pack_summaries
            .iter()
            .filter(|summary| !summary.locus_disclosure().is_provider_authoritative)
            .count();

        let mut out = String::new();
        out.push_str("# Approver matrices and review-pack summaries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Approver matrix rows: {} ({} waived or expired)\n",
            self.approver_matrix_rows.len(),
            waived_or_expired
        ));
        out.push_str(&format!(
            "- Review-pack summaries: {} ({} not provider-authoritative)\n",
            self.review_pack_summaries.len(),
            not_provider_authoritative
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Approver matrix rows\n\n");
        for row in &self.approver_matrix_rows {
            let locus = row.locus_disclosure();
            let approver = row.approver_disclosure();
            out.push_str(&format!(
                "- **{}** — requirement `{}`, state `{}`, parity `{}`, evidence `{}`\n",
                row.approver_role_label,
                row.requirement_source_class.as_str(),
                approver.posture.as_str(),
                locus.posture.as_str(),
                row.evidence_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Review-pack summaries\n\n");
        for summary in &self.review_pack_summaries {
            let locus = summary.locus_disclosure();
            out.push_str(&format!(
                "- **{}** — base `{}` → head `{}`, parity `{}`, {} suppressed check(s)\n",
                summary.pack_digest_label,
                summary.base_identity_label,
                summary.head_identity_label,
                locus.posture.as_str(),
                summary.suppressed_checks.len(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in approver review-pack controls export.
#[derive(Debug)]
pub enum ApproverReviewPackControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ApproverReviewPackControlsViolation>),
}

impl fmt::Display for ApproverReviewPackControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "approver review-pack controls export parse failed: {error}"
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
                    "approver review-pack controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ApproverReviewPackControlsArtifactError {}

/// Validation failures emitted by [`ApproverReviewPackControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApproverReviewPackControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No approver matrix rows are present.
    ApproverMatrixRowsMissing,
    /// An approver matrix row is incomplete.
    ApproverMatrixRowIncomplete,
    /// An approver matrix row carries the wrong frozen component class.
    ApproverMatrixRowWrongComponentClass,
    /// No review-pack summaries are present.
    ReviewPackSummariesMissing,
    /// A review-pack summary is incomplete.
    ReviewPackSummaryIncomplete,
    /// A review-pack summary carries the wrong frozen component class.
    ReviewPackSummaryWrongComponentClass,
    /// A component misrepresents its derived evaluation-locus posture.
    EvaluationLocusMisrepresented,
    /// A CI-only or local-only component claims provider-authoritative evaluation.
    CiOrLocalClaimsProviderAuthoritative,
    /// A not-evaluated-here component claims it was evaluated.
    NotEvaluatedClaimsEvaluated,
    /// A local-only component does not name its local-only evaluation.
    LocalOnlyNoteMissing,
    /// A CI-only component does not name its CI-only evaluation.
    CiOnlyNoteMissing,
    /// A not-evaluated-here component does not name that it was not evaluated here.
    NotEvaluatedNoteMissing,
    /// A stale component does not name its staleness.
    StaleNoteMissing,
    /// A component's governance vocabulary omits its derived locus token.
    GovernanceVocabMissingLocusToken,
    /// An approver matrix row misrepresents its derived approver-state posture.
    ApproverStateMisrepresented,
    /// A waived or expired approver row claims a satisfied approval.
    WaivedOrExpiredClaimsSatisfied,
    /// A waived approver row does not name its waiver.
    WaivedNoteMissing,
    /// An expired approver row does not name its expiry.
    ExpiredNoteMissing,
    /// A pending approver row does not name its pending state.
    PendingNoteMissing,
    /// A waived or expired approver row does not name its expiry label.
    ExpiryLabelMissing,
    /// An approver row's governance vocabulary omits its derived approver-state token.
    GovernanceVocabMissingApproverToken,
    /// An approver matrix row does not offer an open-evidence-link action.
    OpenEvidenceLinkActionMissing,
    /// An approver matrix row carries person-specific contact detail instead of a role alias.
    PersonContactDetailInAlias,
    /// A review-pack summary does not name any capability.
    CapabilitySetMissing,
    /// A review-pack summary lists an incomplete suppressed check.
    SuppressedCheckIncomplete,
    /// A component names an evidence link but not its stable reference.
    EvidenceLinkRefMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component omits a mandatory action.
    ComponentActionsIncomplete,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare any consumer surface.
    ConsumerSurfacesMissing,
    /// The components do not cover every evaluation-locus source.
    LocusSourceCoverageMissing,
    /// The components do not cover every derived evaluation-locus posture.
    LocusPostureCoverageMissing,
    /// The approver rows do not cover every approver-state source.
    ApproverStateSourceCoverageMissing,
    /// The approver rows do not cover every derived approver-state posture.
    ApproverStatePostureCoverageMissing,
    /// The approver rows do not cover every evidence-link kind.
    EvidenceLinkKindCoverageMissing,
    /// The approver rows do not cover every requirement-source class.
    RequirementSourceClassCoverageMissing,
    /// The review-pack summaries alone do not cover every evaluation-locus posture.
    PackLocusCoverageMissing,
    /// The review-pack summaries do not cover every pack capability.
    CapabilityCoverageMissing,
    /// The review-pack summaries do not cover every suppression class.
    SuppressionClassCoverageMissing,
    /// A component hides its requirement source, approver state, parity, or freshness.
    RequirementStateOrParityHidden,
    /// A component lets a CI-only or local-only evaluation masquerade as provider-authoritative.
    CiOrLocalMasqueradesAsProviderAuthoritative,
    /// A component lets a waived or expired approval read as satisfied, or hides suppressed checks.
    WaivedExpiredOrSuppressedMisrepresented,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Review does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ApproverReviewPackControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ApproverMatrixRowsMissing => "approver_matrix_rows_missing",
            Self::ApproverMatrixRowIncomplete => "approver_matrix_row_incomplete",
            Self::ApproverMatrixRowWrongComponentClass => {
                "approver_matrix_row_wrong_component_class"
            }
            Self::ReviewPackSummariesMissing => "review_pack_summaries_missing",
            Self::ReviewPackSummaryIncomplete => "review_pack_summary_incomplete",
            Self::ReviewPackSummaryWrongComponentClass => {
                "review_pack_summary_wrong_component_class"
            }
            Self::EvaluationLocusMisrepresented => "evaluation_locus_misrepresented",
            Self::CiOrLocalClaimsProviderAuthoritative => {
                "ci_or_local_claims_provider_authoritative"
            }
            Self::NotEvaluatedClaimsEvaluated => "not_evaluated_claims_evaluated",
            Self::LocalOnlyNoteMissing => "local_only_note_missing",
            Self::CiOnlyNoteMissing => "ci_only_note_missing",
            Self::NotEvaluatedNoteMissing => "not_evaluated_note_missing",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::GovernanceVocabMissingLocusToken => "governance_vocab_missing_locus_token",
            Self::ApproverStateMisrepresented => "approver_state_misrepresented",
            Self::WaivedOrExpiredClaimsSatisfied => "waived_or_expired_claims_satisfied",
            Self::WaivedNoteMissing => "waived_note_missing",
            Self::ExpiredNoteMissing => "expired_note_missing",
            Self::PendingNoteMissing => "pending_note_missing",
            Self::ExpiryLabelMissing => "expiry_label_missing",
            Self::GovernanceVocabMissingApproverToken => "governance_vocab_missing_approver_token",
            Self::OpenEvidenceLinkActionMissing => "open_evidence_link_action_missing",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::CapabilitySetMissing => "capability_set_missing",
            Self::SuppressedCheckIncomplete => "suppressed_check_incomplete",
            Self::EvidenceLinkRefMissing => "evidence_link_ref_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::ComponentActionsIncomplete => "component_actions_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LocusSourceCoverageMissing => "locus_source_coverage_missing",
            Self::LocusPostureCoverageMissing => "locus_posture_coverage_missing",
            Self::ApproverStateSourceCoverageMissing => "approver_state_source_coverage_missing",
            Self::ApproverStatePostureCoverageMissing => "approver_state_posture_coverage_missing",
            Self::EvidenceLinkKindCoverageMissing => "evidence_link_kind_coverage_missing",
            Self::RequirementSourceClassCoverageMissing => {
                "requirement_source_class_coverage_missing"
            }
            Self::PackLocusCoverageMissing => "pack_locus_coverage_missing",
            Self::CapabilityCoverageMissing => "capability_coverage_missing",
            Self::SuppressionClassCoverageMissing => "suppression_class_coverage_missing",
            Self::RequirementStateOrParityHidden => "requirement_state_or_parity_hidden",
            Self::CiOrLocalMasqueradesAsProviderAuthoritative => {
                "ci_or_local_masquerades_as_provider_authoritative"
            }
            Self::WaivedExpiredOrSuppressedMisrepresented => {
                "waived_expired_or_suppressed_misrepresented"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable approver review-pack controls export.
///
/// This is the first real consumer of the approver-matrix / review-pack component lane: a review-
/// workspace, release-candidate, governance, shiproom, or support-export surface calls it to ingest
/// the canonical components rather than cloning governance text.
///
/// # Errors
///
/// Returns [`ApproverReviewPackControlsArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_approver_review_pack_controls_export(
) -> Result<ApproverReviewPackControlsPacket, ApproverReviewPackControlsArtifactError> {
    let packet: ApproverReviewPackControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-approver-review-pack-controls-proof/support_export.json"
    )))
    .map_err(ApproverReviewPackControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ApproverReviewPackControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ApproverReviewPackControlsPacket,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_REF,
        APPROVER_REVIEW_PACK_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ApproverReviewPackControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component maps into the shared check.
struct ControlInvariants {
    requirement_state_or_parity_hidden: bool,
    ci_or_local_masquerades_as_provider_authoritative: bool,
    waived_expired_or_suppressed_misrepresented: bool,
    invents_alternate_state_label: bool,
}

/// Validates the evaluation-locus posture, notes, and cross-checks shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_locus(
    disclosure: &EvaluationLocusDisclosure,
    derived_evaluation_locus: EvaluationLocusPosture,
    claims_provider_authoritative: bool,
    governance_state_vocab: &[M5GovernanceStateVocab],
    local_only_note: &str,
    ci_only_note: &str,
    not_evaluated_note: &str,
    stale_note: &str,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    if derived_evaluation_locus != disclosure.posture
        || claims_provider_authoritative != disclosure.is_provider_authoritative
    {
        violations.push(ApproverReviewPackControlsViolation::EvaluationLocusMisrepresented);
    }
    if (disclosure.is_ci_only || disclosure.is_local_only) && claims_provider_authoritative {
        violations.push(ApproverReviewPackControlsViolation::CiOrLocalClaimsProviderAuthoritative);
    }
    if disclosure.needs_local_only_note && local_only_note.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::LocalOnlyNoteMissing);
    }
    if disclosure.needs_ci_only_note && ci_only_note.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::CiOnlyNoteMissing);
    }
    if disclosure.needs_not_evaluated_note && not_evaluated_note.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::NotEvaluatedNoteMissing);
    }
    if disclosure.needs_stale_note && stale_note.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::StaleNoteMissing);
    }
    if let Some(token) = disclosure.governance_vocab {
        if !governance_state_vocab.contains(&token) {
            violations.push(ApproverReviewPackControlsViolation::GovernanceVocabMissingLocusToken);
        }
    }
}

/// Validates the axes shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_common_control(
    evidence_link_kind: EvidenceLinkKind,
    evidence_link_ref: &str,
    context_note: &str,
    declares_mandatory_actions: bool,
    downgrade_triggers: &[M5GovernanceComponentDowngradeTrigger],
    consumer_surfaces: &[M5GovernanceComponentConsumerSurface],
    invariants: ControlInvariants,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::ContextNoteMissing);
    }
    if evidence_link_kind.is_resolvable() && evidence_link_ref.trim().is_empty() {
        violations.push(ApproverReviewPackControlsViolation::EvidenceLinkRefMissing);
    }
    if !declares_mandatory_actions {
        violations.push(ApproverReviewPackControlsViolation::ComponentActionsIncomplete);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ApproverReviewPackControlsViolation::DowngradeTriggersMissing);
    }
    if consumer_surfaces.is_empty() {
        violations.push(ApproverReviewPackControlsViolation::ConsumerSurfacesMissing);
    }
    if invariants.requirement_state_or_parity_hidden {
        violations.push(ApproverReviewPackControlsViolation::RequirementStateOrParityHidden);
    }
    if invariants.ci_or_local_masquerades_as_provider_authoritative {
        violations
            .push(ApproverReviewPackControlsViolation::CiOrLocalMasqueradesAsProviderAuthoritative);
    }
    if invariants.waived_expired_or_suppressed_misrepresented {
        violations
            .push(ApproverReviewPackControlsViolation::WaivedExpiredOrSuppressedMisrepresented);
    }
    if invariants.invents_alternate_state_label {
        violations.push(ApproverReviewPackControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_approver_matrix_rows(
    packet: &ApproverReviewPackControlsPacket,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    if packet.approver_matrix_rows.is_empty() {
        violations.push(ApproverReviewPackControlsViolation::ApproverMatrixRowsMissing);
        return;
    }

    for row in &packet.approver_matrix_rows {
        let locus = row.locus_disclosure();
        let approver = row.approver_disclosure();

        if row.row_id.trim().is_empty()
            || row.approver_role_label.trim().is_empty()
            || row.requirement_source_label.trim().is_empty()
            || row.context_note.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ApproverReviewPackControlsViolation::ApproverMatrixRowIncomplete);
        }
        if row.component != M5GovernanceComponent::ApproverMatrix {
            violations
                .push(ApproverReviewPackControlsViolation::ApproverMatrixRowWrongComponentClass);
        }
        if alias_carries_contact_detail(&row.approver_role_label) {
            violations.push(ApproverReviewPackControlsViolation::PersonContactDetailInAlias);
        }
        validate_shared_locus(
            &locus,
            row.derived_evaluation_locus,
            row.claims_provider_authoritative,
            &row.governance_state_vocab,
            &row.local_only_note,
            &row.ci_only_note,
            &row.not_evaluated_note,
            &row.stale_note,
            violations,
        );
        if row.derived_approver_state != approver.posture
            || row.claims_satisfied != approver.is_satisfied
        {
            violations.push(ApproverReviewPackControlsViolation::ApproverStateMisrepresented);
        }
        if (approver.is_waived || approver.is_expired) && row.claims_satisfied {
            violations.push(ApproverReviewPackControlsViolation::WaivedOrExpiredClaimsSatisfied);
        }
        if approver.needs_waived_note && row.waived_note.trim().is_empty() {
            violations.push(ApproverReviewPackControlsViolation::WaivedNoteMissing);
        }
        if approver.needs_expired_note && row.expired_note.trim().is_empty() {
            violations.push(ApproverReviewPackControlsViolation::ExpiredNoteMissing);
        }
        if approver.needs_pending_note && row.pending_note.trim().is_empty() {
            violations.push(ApproverReviewPackControlsViolation::PendingNoteMissing);
        }
        if approver.needs_expiry_label && row.expiry_label.trim().is_empty() {
            violations.push(ApproverReviewPackControlsViolation::ExpiryLabelMissing);
        }
        if let Some(token) = approver.governance_vocab {
            if !row.governance_state_vocab.contains(&token) {
                violations
                    .push(ApproverReviewPackControlsViolation::GovernanceVocabMissingApproverToken);
            }
        }
        if !row
            .row_actions
            .contains(&ApproverMatrixAction::OpenEvidenceLink)
        {
            violations.push(ApproverReviewPackControlsViolation::OpenEvidenceLinkActionMissing);
        }
        validate_common_control(
            row.evidence_link_kind,
            &row.evidence_link_ref,
            &row.context_note,
            row.declares_mandatory_actions(),
            &row.downgrade_triggers,
            &row.consumer_surfaces,
            ControlInvariants {
                requirement_state_or_parity_hidden: row.hides_requirement_source_or_state,
                ci_or_local_masquerades_as_provider_authoritative: row
                    .lets_ci_or_local_read_as_provider_authoritative,
                waived_expired_or_suppressed_misrepresented: row
                    .lets_waived_or_expired_read_as_satisfied,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    let mut state_sources: BTreeSet<ApproverStateSource> = BTreeSet::new();
    let mut state_postures: BTreeSet<ApproverStatePosture> = BTreeSet::new();
    let mut evidence_kinds: BTreeSet<EvidenceLinkKind> = BTreeSet::new();
    let mut requirement_classes: BTreeSet<RequirementSourceClass> = BTreeSet::new();
    for row in &packet.approver_matrix_rows {
        state_sources.insert(row.approver_state_source);
        state_postures.insert(row.approver_disclosure().posture);
        evidence_kinds.insert(row.evidence_link_kind);
        requirement_classes.insert(row.requirement_source_class);
    }
    if ApproverStateSource::ALL
        .iter()
        .any(|source| !state_sources.contains(source))
    {
        violations.push(ApproverReviewPackControlsViolation::ApproverStateSourceCoverageMissing);
    }
    if ApproverStatePosture::ALL
        .iter()
        .any(|posture| !state_postures.contains(posture))
    {
        violations.push(ApproverReviewPackControlsViolation::ApproverStatePostureCoverageMissing);
    }
    if EvidenceLinkKind::ALL
        .iter()
        .any(|kind| !evidence_kinds.contains(kind))
    {
        violations.push(ApproverReviewPackControlsViolation::EvidenceLinkKindCoverageMissing);
    }
    if RequirementSourceClass::ALL
        .iter()
        .any(|class| !requirement_classes.contains(class))
    {
        violations.push(ApproverReviewPackControlsViolation::RequirementSourceClassCoverageMissing);
    }
}

fn validate_review_pack_summaries(
    packet: &ApproverReviewPackControlsPacket,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    if packet.review_pack_summaries.is_empty() {
        violations.push(ApproverReviewPackControlsViolation::ReviewPackSummariesMissing);
        return;
    }

    let mut locus_postures: BTreeSet<EvaluationLocusPosture> = BTreeSet::new();
    let mut capabilities: BTreeSet<PackCapability> = BTreeSet::new();
    let mut suppression_classes: BTreeSet<PackSuppressionClass> = BTreeSet::new();

    for summary in &packet.review_pack_summaries {
        let locus = summary.locus_disclosure();
        locus_postures.insert(locus.posture);
        for capability in &summary.capability_set {
            capabilities.insert(*capability);
        }
        for suppressed in &summary.suppressed_checks {
            suppression_classes.insert(suppressed.suppression_class);
        }

        if summary.summary_id.trim().is_empty()
            || summary.pack_digest_label.trim().is_empty()
            || summary.base_identity_label.trim().is_empty()
            || summary.head_identity_label.trim().is_empty()
            || summary.capability_set_label.trim().is_empty()
            || summary.parity_label.trim().is_empty()
            || summary.freshness_label.trim().is_empty()
            || summary.suppressed_checks_label.trim().is_empty()
            || summary.context_note.trim().is_empty()
            || summary.source_contract_refs.is_empty()
        {
            violations.push(ApproverReviewPackControlsViolation::ReviewPackSummaryIncomplete);
        }
        if summary.capability_set.is_empty() {
            violations.push(ApproverReviewPackControlsViolation::CapabilitySetMissing);
        }
        if summary.component != M5GovernanceComponent::ReviewPackSummary {
            violations
                .push(ApproverReviewPackControlsViolation::ReviewPackSummaryWrongComponentClass);
        }
        validate_shared_locus(
            &locus,
            summary.derived_evaluation_locus,
            summary.claims_provider_authoritative,
            &summary.governance_state_vocab,
            &summary.local_only_note,
            &summary.ci_only_note,
            &summary.not_evaluated_note,
            &summary.stale_note,
            violations,
        );
        if summary.claims_evaluated_here != locus.is_evaluated_here {
            violations.push(ApproverReviewPackControlsViolation::EvaluationLocusMisrepresented);
        }
        if !locus.is_evaluated_here && summary.claims_evaluated_here {
            violations.push(ApproverReviewPackControlsViolation::NotEvaluatedClaimsEvaluated);
        }
        for suppressed in &summary.suppressed_checks {
            if suppressed.check_label.trim().is_empty() || suppressed.reason_label.trim().is_empty()
            {
                violations.push(ApproverReviewPackControlsViolation::SuppressedCheckIncomplete);
            }
        }
        // A review-pack summary binds no openable evidence link of its own, but it must still name a
        // context note and declare its mandatory actions, triggers, and surfaces.
        validate_common_control(
            EvidenceLinkKind::NoEvidenceLink,
            "",
            &summary.context_note,
            summary.declares_mandatory_actions(),
            &summary.downgrade_triggers,
            &summary.consumer_surfaces,
            ControlInvariants {
                requirement_state_or_parity_hidden: summary.hides_parity_or_freshness,
                ci_or_local_masquerades_as_provider_authoritative: summary
                    .lets_ci_or_local_read_as_provider_authoritative,
                waived_expired_or_suppressed_misrepresented: summary
                    .hides_suppressed_checks_or_waivers,
                invents_alternate_state_label: summary.invents_alternate_state_label,
            },
            violations,
        );
    }

    // AC-1: the review-pack summaries alone must let a user tell whether a pack is local-only,
    // provider authoritative, CI-only, not evaluated here, or stale relative to base/head.
    if EvaluationLocusPosture::ALL
        .iter()
        .any(|posture| !locus_postures.contains(posture))
    {
        violations.push(ApproverReviewPackControlsViolation::PackLocusCoverageMissing);
    }
    if PackCapability::ALL
        .iter()
        .any(|capability| !capabilities.contains(capability))
    {
        violations.push(ApproverReviewPackControlsViolation::CapabilityCoverageMissing);
    }
    if PackSuppressionClass::ALL
        .iter()
        .any(|class| !suppression_classes.contains(class))
    {
        violations.push(ApproverReviewPackControlsViolation::SuppressionClassCoverageMissing);
    }
}

/// Validates that the union of both component vectors covers every locus source and posture.
fn validate_shared_coverage(
    packet: &ApproverReviewPackControlsPacket,
    violations: &mut Vec<ApproverReviewPackControlsViolation>,
) {
    let mut sources: BTreeSet<EvaluationLocusSource> = BTreeSet::new();
    let mut postures: BTreeSet<EvaluationLocusPosture> = BTreeSet::new();

    for row in &packet.approver_matrix_rows {
        sources.insert(row.evaluation_locus_source);
        postures.insert(row.locus_disclosure().posture);
    }
    for summary in &packet.review_pack_summaries {
        sources.insert(summary.evaluation_locus_source);
        postures.insert(summary.locus_disclosure().posture);
    }

    if EvaluationLocusSource::ALL
        .iter()
        .any(|source| !sources.contains(source))
    {
        violations.push(ApproverReviewPackControlsViolation::LocusSourceCoverageMissing);
    }
    if EvaluationLocusPosture::ALL
        .iter()
        .any(|posture| !postures.contains(posture))
    {
        violations.push(ApproverReviewPackControlsViolation::LocusPostureCoverageMissing);
    }
}

/// Whether an approver alias carries person-specific private contact detail (an email address) rather
/// than an export-safe role alias.
fn alias_carries_contact_detail(alias: &str) -> bool {
    alias.contains('@')
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
