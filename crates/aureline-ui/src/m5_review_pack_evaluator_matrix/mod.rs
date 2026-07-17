//! Frozen M5 review-pack, ownership-signal, required-evidence-check, local-CI-parity-strip, AI-policy-hook, and review-template-packet matrix.
//!
//! This module locks Aureline's review-pack evaluator model — the declarative review-pack record, the
//! ownership signal, the required-evidence / required-check row, the local-CI parity strip, the AI review
//! policy hook, and the review-template packet that a review-capable consumer must treat as first-class,
//! durable, provider-aware review objects rather than ad hoc review chrome or provider-specific behavior — into
//! one export-safe packet. Every covered object class is named once here and constrained by the same shared
//! review-pack role taxonomy (pack_version_and_digest_disclosure, owner_provenance_disclosure,
//! evaluator_result_class_disclosure, local_versus_provider_parity_disclosure,
//! required_evidence_and_check_disclosure, template_attribution_disclosure,
//! pack_freshness_and_invalidation_disclosure), the same required visible state (pack label, pack version and
//! digest, owner provenance, evaluator result class, local-versus-provider parity, pack freshness state, and
//! template attribution), the same no-local-parity-estimate-masquerading-as-provider-authoritative rule, the
//! same no-hiding-ci-only-not-evaluated-here-or-provider-unavailable-behind-a-green-summary rule, the same
//! no-flattening-advisory-owner-and-enforced-owner-into-one-owner-pill rule, the same
//! no-AI-review-under-a-different-pack-version-without-disclosure rule, and the same
//! no-review-pack-version-digest-or-template-attribution-lost-on-export-publish-or-reopen rule regardless of the
//! surface that renders it.
//!
//! The matrix makes a provider-authoritative result mechanically distinct from a local parity estimate (see
//! [`M5ReviewPackParityState`]) so review detail, merge-readiness, the AI review panel, provider handoff, the
//! ownership overlay, the local-CI parity strip, and support / export packets can key off the parity state and
//! pack freshness rather than guessing from a generic green summary. It does not widen M5 into arbitrary code
//! execution inside review-pack evaluators, provider-side merge automation, or a separate server policy engine —
//! it reuses the already-landed review-workspace anchors, checks-summary / merge-readiness components,
//! provider-linked draft / publish-now / open-in-provider semantics, and AI review finding packets — it is the
//! shared reusable review-pack contract those consumers read, and it binds back to the already-landed
//! stable-proof-index and migration-task-row packets so review-pack and local-parity truth is not split across
//! scattered internal notes. The controlled vocabularies are frozen in one self-describing
//! [`M5ReviewPackVocabularySet`] rather than minted per surface. Raw paths, raw glob bodies, raw command lines,
//! raw check outputs, secret values, and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_review_pack_matrix, seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed,
    seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed, M5_REVIEW_PACK_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReviewPackMatrixPacket`].
pub const M5_REVIEW_PACK_MATRIX_RECORD_KIND: &str =
    "freeze_m5_review_pack_ownership_local_ci_parity_and_review_template_matrix";

/// Schema version for M5 review-pack matrix records.
pub const M5_REVIEW_PACK_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined review-pack evaluator matrix schema.
pub const M5_REVIEW_PACK_MATRIX_SCHEMA_REF: &str =
    "schemas/review/m5-review-pack-evaluator-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REVIEW_PACK_MATRIX_DOC_REF: &str = "docs/review/m5-review-pack-evaluator-ops.md";

/// Repo-relative path of the canonical review-pack domain schema (the declarative review pack with its version /
/// digest, scope selectors, and evaluator identity).
pub const M5_REVIEW_PACK_DOMAIN_SCHEMA_REF: &str = "schemas/review/m5-review-pack.schema.json";

/// Repo-relative path of the canonical ownership-signal domain schema (the advisory-versus-enforced owner
/// provenance for a scope slice).
pub const M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ownership-signal.schema.json";

/// Repo-relative path of the canonical review-pack-result domain schema (one required evidence / check row and
/// its evaluator result class).
pub const M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-review-pack-result.schema.json";

/// Repo-relative path of the canonical local-CI-parity domain schema (the local-parity-estimate-versus-
/// provider-authoritative state per check, including ci-only, not-evaluated-here, and provider-unavailable).
pub const M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-local-ci-parity.schema.json";

/// Repo-relative path of the canonical AI-policy-hook domain schema (the AI review run bound to a disclosed
/// pack version / digest and pack-driven policy).
pub const M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ai-policy-hook.schema.json";

/// Repo-relative path of the canonical review-template-packet domain schema (the comment / summary template and
/// its pack-bound attribution).
pub const M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-review-template-packet.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_PACK_FIXTURE_DIR: &str = "fixtures/review/m5-review-pack-parity";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_PACK_ARTIFACT_REF: &str =
    "artifacts/review/m5-review-pack-results/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_REVIEW_PACK_CSV_REF: &str = "artifacts/review/m5-review-pack-results/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REVIEW_PACK_REPORT_REF: &str = "artifacts/review/m5-review-pack-evaluator-matrix.md";

/// Repo-relative path of the checked review-pack-health dashboard.
pub const M5_REVIEW_PACK_DASHBOARD_REF: &str = "dashboards/m5-review-pack-health.json";

/// One of the six governed review-pack object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackObject {
    /// A declarative review-pack record: the repo-defined review pack with its version / digest, scope selectors, and evaluator identity that local, hosted, CI, and AI review lanes all bind to.
    ReviewPackRecord,
    /// An ownership signal: the advisory-owner-versus-enforced-owner provenance for a scope slice of a review pack.
    OwnershipSignal,
    /// A required-evidence / required-check row: one required evidence or check the pack demands, plus its evaluator result class.
    RequiredEvidenceCheckRow,
    /// A local-CI parity strip: the local-parity-estimate-versus-provider-authoritative state per check, including ci-only, not-evaluated-here, and provider-unavailable.
    LocalCiParityStrip,
    /// An AI review policy hook: the binding that runs an AI review under a disclosed review-pack version / digest and pack-driven policy.
    AiPolicyHook,
    /// A review-template packet: the comment / summary template and the attribution that stays bound to the pack it came from.
    ReviewTemplatePacket,
}

impl M5ReviewPackObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewPackRecord,
        Self::OwnershipSignal,
        Self::RequiredEvidenceCheckRow,
        Self::LocalCiParityStrip,
        Self::AiPolicyHook,
        Self::ReviewTemplatePacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPackRecord => "review_pack_record",
            Self::OwnershipSignal => "ownership_signal",
            Self::RequiredEvidenceCheckRow => "required_evidence_check_row",
            Self::LocalCiParityStrip => "local_ci_parity_strip",
            Self::AiPolicyHook => "ai_policy_hook",
            Self::ReviewTemplatePacket => "review_template_packet",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's review-pack, ownership, required-check, parity, AI-policy-hook, or template meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ReviewPackRecord => M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
            Self::OwnershipSignal => M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
            Self::RequiredEvidenceCheckRow => M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
            Self::LocalCiParityStrip => M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
            Self::AiPolicyHook => M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
            Self::ReviewTemplatePacket => M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled review pack record role.
    pub const fn declares_review_pack_record_roles(self) -> bool {
        matches!(self, Self::ReviewPackRecord)
    }

    /// `true` when this class must name a controlled ownership signal role.
    pub const fn declares_ownership_signal_roles(self) -> bool {
        matches!(self, Self::OwnershipSignal)
    }

    /// `true` when this class must name a controlled required evidence role.
    pub const fn declares_required_evidence_roles(self) -> bool {
        matches!(self, Self::RequiredEvidenceCheckRow)
    }

    /// `true` when this class must name a controlled local ci parity role.
    pub const fn declares_local_ci_parity_roles(self) -> bool {
        matches!(self, Self::LocalCiParityStrip)
    }

    /// `true` when this class must name a controlled ai policy hook role.
    pub const fn declares_ai_policy_hook_roles(self) -> bool {
        matches!(self, Self::AiPolicyHook)
    }

    /// `true` when this class must name a controlled template packet role.
    pub const fn declares_template_packet_roles(self) -> bool {
        matches!(self, Self::ReviewTemplatePacket)
    }
}

/// The single controlled review-pack role vocabulary every review, merge-readiness, AI review, provider handoff, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackRole {
    /// The review-pack version and content digest disclosed on every claimed review surface.
    PackVersionAndDigestDisclosure,
    /// The advisory-owner-versus-enforced-owner provenance disclosed for a scope slice.
    OwnerProvenanceDisclosure,
    /// The evaluator result class (provider-authoritative, local-parity-estimate, not-evaluated-here, ci-only) disclosed for a check.
    EvaluatorResultClassDisclosure,
    /// The local-parity-estimate-versus-provider-authoritative state disclosed so a local estimate never reads as authoritative mergeability.
    LocalVersusProviderParityDisclosure,
    /// The required evidence and required checks the pack demands, disclosed as an explicit set.
    RequiredEvidenceAndCheckDisclosure,
    /// The comment / summary template attribution disclosed and kept bound to the pack.
    TemplateAttributionDisclosure,
    /// The pack freshness / invalidation state (stale-pack, partial-scope, slice-omitted) disclosed for a result.
    PackFreshnessAndInvalidationDisclosure,
}

impl M5ReviewPackRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PackVersionAndDigestDisclosure,
        Self::OwnerProvenanceDisclosure,
        Self::EvaluatorResultClassDisclosure,
        Self::LocalVersusProviderParityDisclosure,
        Self::RequiredEvidenceAndCheckDisclosure,
        Self::TemplateAttributionDisclosure,
        Self::PackFreshnessAndInvalidationDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackVersionAndDigestDisclosure => "pack_version_and_digest_disclosure",
            Self::OwnerProvenanceDisclosure => "owner_provenance_disclosure",
            Self::EvaluatorResultClassDisclosure => "evaluator_result_class_disclosure",
            Self::LocalVersusProviderParityDisclosure => "local_versus_provider_parity_disclosure",
            Self::RequiredEvidenceAndCheckDisclosure => "required_evidence_and_check_disclosure",
            Self::TemplateAttributionDisclosure => "template_attribution_disclosure",
            Self::PackFreshnessAndInvalidationDisclosure => {
                "pack_freshness_and_invalidation_disclosure"
            }
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a review-pack result (`pack_version_and_digest_disclosure`,
    /// `owner_provenance_disclosure`, `evaluator_result_class_disclosure`,
    /// `local_versus_provider_parity_disclosure`). The contextual roles
    /// (`required_evidence_and_check_disclosure`, `template_attribution_disclosure`,
    /// `pack_freshness_and_invalidation_disclosure`) apply where the object class calls for them.
    pub const fn must_be_present_before_surfacing_as_a_review_pack_result(self) -> bool {
        matches!(
            self,
            Self::PackVersionAndDigestDisclosure
                | Self::OwnerProvenanceDisclosure
                | Self::EvaluatorResultClassDisclosure
                | Self::LocalVersusProviderParityDisclosure
        )
    }
}

/// Parity state that makes a provider-authoritative result (authoritative mergeability / approval truth) mechanically distinct from a local parity estimate, a ci-only or not-evaluated-here lane, a provider-unavailable lane, a stale-relative-to-base/head result, or a draft-only review state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackParityState {
    /// Provider-authoritative: the result was confirmed by the provider and carries authoritative mergeability / approval truth.
    ProviderAuthoritative,
    /// A local parity estimate computed on this machine; an estimate only, never provider-authoritative mergeability.
    LocalParityEstimate,
    /// The result is stale relative to the current base / head and must be re-evaluated before it is trusted.
    StaleRelativeToBaseHead,
    /// This check is not evaluated in this lane (for example a provider-only check) and is never shown as passing here.
    NotEvaluatedHere,
    /// This check runs only in CI, not locally, and is never folded into a local green summary.
    CiOnly,
    /// The provider is unavailable so authoritative mergeability / approval truth cannot be fetched right now.
    ProviderUnavailable,
    /// A draft-only review state: nothing has been published to any provider and no authoritative result exists yet.
    DraftOnlyReviewState,
}

impl M5ReviewPackParityState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProviderAuthoritative,
        Self::LocalParityEstimate,
        Self::StaleRelativeToBaseHead,
        Self::NotEvaluatedHere,
        Self::CiOnly,
        Self::ProviderUnavailable,
        Self::DraftOnlyReviewState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocalParityEstimate => "local_parity_estimate",
            Self::StaleRelativeToBaseHead => "stale_relative_to_base_head",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::CiOnly => "ci_only",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::DraftOnlyReviewState => "draft_only_review_state",
        }
    }
    /// `true` only for the provider-authoritative state, so downstream review detail, merge-readiness,
    /// the AI panel, provider handoff, and support / export packets can key off an authoritative
    /// provider result rather than confusing it with a local parity estimate, a ci-only lane, or a
    /// not-evaluated-here check.
    pub const fn is_provider_authoritative(self) -> bool {
        matches!(self, Self::ProviderAuthoritative)
    }
}

/// Named owner authority (advisory owner, enforced owner, no owner declared, ownership unavailable) so an advisory owner is never flattened into an enforced-owner merge gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackOwnerAuthority {
    /// An advisory owner: a suggested reviewer whose approval is informative, not a merge gate.
    AdvisoryOwner,
    /// An enforced owner: an owner whose approval is required before the scope slice can merge.
    EnforcedOwner,
    /// No owner is declared for this scope slice by the pack.
    NoOwnerDeclared,
    /// The ownership signal is unavailable because the pack or provider could not be reached.
    OwnershipUnavailable,
}

impl M5ReviewPackOwnerAuthority {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdvisoryOwner,
        Self::EnforcedOwner,
        Self::NoOwnerDeclared,
        Self::OwnershipUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryOwner => "advisory_owner",
            Self::EnforcedOwner => "enforced_owner",
            Self::NoOwnerDeclared => "no_owner_declared",
            Self::OwnershipUnavailable => "ownership_unavailable",
        }
    }
    /// `true` only for an enforced owner whose approval gates the merge, so a consumer can
    /// mechanically refuse to flatten an advisory owner into an enforced-owner pill.
    pub const fn is_enforced(self) -> bool {
        matches!(self, Self::EnforcedOwner)
    }
}

/// Named pack freshness / invalidation state (pack fresh, stale pack, partial scope, slice omitted, pack invalid) so no claimed surface lacks a named state for pack staleness or invalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackFreshness {
    /// The pack is fresh: fully evaluated against the current base / head at the current pack version and digest.
    PackFresh,
    /// A stale pack: the evaluated review-pack version / digest is behind the repo's current pack.
    StalePack,
    /// A partial scope: only part of the pack's declared scope was evaluated.
    PartialScope,
    /// A slice omitted: at least one declared scope slice was skipped and is not represented in the result.
    SliceOmitted,
    /// An invalid pack: the review pack failed to parse or validate and no result can be trusted.
    PackInvalid,
}

impl M5ReviewPackFreshness {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PackFresh,
        Self::StalePack,
        Self::PartialScope,
        Self::SliceOmitted,
        Self::PackInvalid,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackFresh => "pack_fresh",
            Self::StalePack => "stale_pack",
            Self::PartialScope => "partial_scope",
            Self::SliceOmitted => "slice_omitted",
            Self::PackInvalid => "pack_invalid",
        }
    }
    /// `true` for the stale / partial / invalid freshness states (`stale_pack`, `partial_scope`,
    /// `slice_omitted`, `pack_invalid`) so a consumer can mechanically refuse to show a stale or
    /// partially evaluated pack result as a fresh, fully evaluated one.
    pub const fn is_stale_or_partial(self) -> bool {
        matches!(
            self,
            Self::StalePack | Self::PartialScope | Self::SliceOmitted | Self::PackInvalid
        )
    }
}

/// Controlled review-pack-record role for one repo-defined review pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackRecordRole {
    /// Pack version and content digest shown so a pack is never an anonymous rule set.
    PackVersionAndDigestShown,
    /// Scope selectors named so the pack states exactly which paths / slices it governs.
    ScopeSelectorsNamed,
    /// Evaluator identity named so the same declarative evaluator is bound across local, CI, and AI lanes.
    EvaluatorIdentityNamed,
    /// Pack freshness and invalidation (stale-pack, partial-scope, slice-omitted) shown so a stale pack never looks current.
    PackFreshnessAndInvalidationShown,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Silently swapping the pack version or digest without disclosure, which is disallowed.
    SilentPackVersionSwapDisallowed,
}

impl M5ReviewPackRecordRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackVersionAndDigestShown,
        Self::ScopeSelectorsNamed,
        Self::EvaluatorIdentityNamed,
        Self::PackFreshnessAndInvalidationShown,
        Self::BoundToReviewPackRegistry,
        Self::SilentPackVersionSwapDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackVersionAndDigestShown => "pack_version_and_digest_shown",
            Self::ScopeSelectorsNamed => "scope_selectors_named",
            Self::EvaluatorIdentityNamed => "evaluator_identity_named",
            Self::PackFreshnessAndInvalidationShown => "pack_freshness_and_invalidation_shown",
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::SilentPackVersionSwapDisallowed => "silent_pack_version_swap_disallowed",
        }
    }
}

/// Controlled ownership-signal role for the advisory-versus-enforced owner provenance of a scope slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackOwnershipSignalRole {
    /// Advisory-versus-enforced owner state shown so an advisory owner is never presented as a merge gate.
    AdvisoryVersusEnforcedOwnerShown,
    /// Owned scope slice named so ownership is attributable to an exact part of the diff.
    OwnedScopeSliceNamed,
    /// Owner approval state shown so a missing enforced approval never reads as satisfied.
    OwnerApprovalStateShown,
    /// Partial-scope or slice-omitted flagged so ownership coverage is never overstated.
    PartialScopeOrSliceOmittedFlagged,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Flattening advisory-owner and enforced-owner into one owner pill, which is disallowed.
    AdvisoryAndEnforcedOwnerFlattenedDisallowed,
}

impl M5ReviewPackOwnershipSignalRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdvisoryVersusEnforcedOwnerShown,
        Self::OwnedScopeSliceNamed,
        Self::OwnerApprovalStateShown,
        Self::PartialScopeOrSliceOmittedFlagged,
        Self::BoundToReviewPackRegistry,
        Self::AdvisoryAndEnforcedOwnerFlattenedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryVersusEnforcedOwnerShown => "advisory_versus_enforced_owner_shown",
            Self::OwnedScopeSliceNamed => "owned_scope_slice_named",
            Self::OwnerApprovalStateShown => "owner_approval_state_shown",
            Self::PartialScopeOrSliceOmittedFlagged => "partial_scope_or_slice_omitted_flagged",
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::AdvisoryAndEnforcedOwnerFlattenedDisallowed => {
                "advisory_and_enforced_owner_flattened_disallowed"
            }
        }
    }
}

/// Controlled required-evidence / required-check role for one demanded evidence or check row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackRequiredEvidenceRole {
    /// Required evidence or check named so a demanded item is never an unlabelled row.
    RequiredEvidenceOrCheckNamed,
    /// Evaluator result class shown so a check states whether it is provider-authoritative or a local estimate.
    EvaluatorResultClassShown,
    /// Not-evaluated-here or ci-only state shown so an unevaluated check is never folded into a green summary.
    NotEvaluatedHereOrCiOnlyShown,
    /// Evidence anchor named so a required check joins back to the evidence that satisfies it.
    EvidenceAnchorNamed,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Hiding a ci-only, not-evaluated-here, or provider-unavailable check behind a green summary, which is disallowed.
    GreenSummaryHidingUnevaluatedCheckDisallowed,
}

impl M5ReviewPackRequiredEvidenceRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RequiredEvidenceOrCheckNamed,
        Self::EvaluatorResultClassShown,
        Self::NotEvaluatedHereOrCiOnlyShown,
        Self::EvidenceAnchorNamed,
        Self::BoundToReviewPackRegistry,
        Self::GreenSummaryHidingUnevaluatedCheckDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredEvidenceOrCheckNamed => "required_evidence_or_check_named",
            Self::EvaluatorResultClassShown => "evaluator_result_class_shown",
            Self::NotEvaluatedHereOrCiOnlyShown => "not_evaluated_here_or_ci_only_shown",
            Self::EvidenceAnchorNamed => "evidence_anchor_named",
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::GreenSummaryHidingUnevaluatedCheckDisallowed => {
                "green_summary_hiding_unevaluated_check_disallowed"
            }
        }
    }
}

/// Controlled local-CI-parity-strip role for the local-versus-provider parity of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackLocalCiParityRole {
    /// Local parity estimate labelled as an estimate so it never reads as authoritative mergeability.
    LocalParityEstimateLabelledAsEstimate,
    /// Provider-authoritative state named so authoritative truth is distinct from a local estimate.
    ProviderAuthoritativeStateNamed,
    /// Ci-only or provider-unavailable state shown so a missing lane is never silently treated as green.
    CiOnlyOrProviderUnavailableShown,
    /// Stale-relative-to-base/head flagged so a parity strip never outlives the diff it was computed on.
    StaleRelativeToBaseHeadFlagged,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Presenting a local parity estimate as provider-authoritative mergeability, which is disallowed.
    LocalEstimatePresentedAsProviderAuthoritativeDisallowed,
}

impl M5ReviewPackLocalCiParityRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalParityEstimateLabelledAsEstimate,
        Self::ProviderAuthoritativeStateNamed,
        Self::CiOnlyOrProviderUnavailableShown,
        Self::StaleRelativeToBaseHeadFlagged,
        Self::BoundToReviewPackRegistry,
        Self::LocalEstimatePresentedAsProviderAuthoritativeDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalParityEstimateLabelledAsEstimate => {
                "local_parity_estimate_labelled_as_estimate"
            }
            Self::ProviderAuthoritativeStateNamed => "provider_authoritative_state_named",
            Self::CiOnlyOrProviderUnavailableShown => "ci_only_or_provider_unavailable_shown",
            Self::StaleRelativeToBaseHeadFlagged => "stale_relative_to_base_head_flagged",
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::LocalEstimatePresentedAsProviderAuthoritativeDisallowed => {
                "local_estimate_presented_as_provider_authoritative_disallowed"
            }
        }
    }
}

/// Controlled AI-policy-hook role for an AI review bound to a review-pack version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackAiPolicyHookRole {
    /// AI run pack version and digest shown so an AI review names the pack it ran under.
    AiRunPackVersionAndDigestShown,
    /// Pack-driven policy named so an AI review hook is bound to a declarative pack, not ad hoc behavior.
    PackDrivenPolicyNamed,
    /// A divergent pack version disclosed so an AI review under a different pack is never silent.
    DivergentPackVersionDisclosed,
    /// AI result bound to the pack's evaluator result class so it inherits the same authoritative-versus-estimate truth.
    AiResultBoundToPackResultClass,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Running AI review under a different pack version without disclosure, which is disallowed.
    AiReviewUnderUndisclosedPackVersionDisallowed,
}

impl M5ReviewPackAiPolicyHookRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AiRunPackVersionAndDigestShown,
        Self::PackDrivenPolicyNamed,
        Self::DivergentPackVersionDisclosed,
        Self::AiResultBoundToPackResultClass,
        Self::BoundToReviewPackRegistry,
        Self::AiReviewUnderUndisclosedPackVersionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiRunPackVersionAndDigestShown => "ai_run_pack_version_and_digest_shown",
            Self::PackDrivenPolicyNamed => "pack_driven_policy_named",
            Self::DivergentPackVersionDisclosed => "divergent_pack_version_disclosed",
            Self::AiResultBoundToPackResultClass => "ai_result_bound_to_pack_result_class",
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::AiReviewUnderUndisclosedPackVersionDisallowed => {
                "ai_review_under_undisclosed_pack_version_disallowed"
            }
        }
    }
}

/// Controlled review-template-packet role for a comment / summary template and its attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackTemplatePacketRole {
    /// Template attribution shown so a rendered comment or summary names the template it came from.
    TemplateAttributionShown,
    /// Comment or summary template named so output is never anonymous chrome.
    CommentOrSummaryTemplateNamed,
    /// Template version and pack binding shown so the template stays joined to its review pack.
    TemplateVersionAndPackBindingShown,
    /// Template attribution preserved on export, publish, and reopen, never dropped.
    TemplateAttributionPreservedOnExport,
    /// A role bound to the single review-pack registry.
    BoundToReviewPackRegistry,
    /// Dropping template attribution when exporting, publishing, or reopening review evidence, which is disallowed.
    TemplateAttributionDroppedOnExportDisallowed,
}

impl M5ReviewPackTemplatePacketRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TemplateAttributionShown,
        Self::CommentOrSummaryTemplateNamed,
        Self::TemplateVersionAndPackBindingShown,
        Self::TemplateAttributionPreservedOnExport,
        Self::BoundToReviewPackRegistry,
        Self::TemplateAttributionDroppedOnExportDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateAttributionShown => "template_attribution_shown",
            Self::CommentOrSummaryTemplateNamed => "comment_or_summary_template_named",
            Self::TemplateVersionAndPackBindingShown => "template_version_and_pack_binding_shown",
            Self::TemplateAttributionPreservedOnExport => {
                "template_attribution_preserved_on_export"
            }
            Self::BoundToReviewPackRegistry => "bound_to_review_pack_registry",
            Self::TemplateAttributionDroppedOnExportDisallowed => {
                "template_attribution_dropped_on_export_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a review-pack object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackSurfaceFamily {
    /// The review surface (review detail, diff review headers, review-pack summaries).
    Review,
    /// The merge-readiness surface (checks-summary and mergeability).
    MergeReadiness,
    /// The AI review surface (AI review panel and policy hooks).
    AiReview,
    /// The provider handoff / open-in-provider surface.
    ProviderHandoff,
    /// The support / export surface.
    SupportExport,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ReviewPackSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Review,
        Self::MergeReadiness,
        Self::AiReview,
        Self::ProviderHandoff,
        Self::SupportExport,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
            Self::MergeReadiness => "merge_readiness",
            Self::AiReview => "ai_review",
            Self::ProviderHandoff => "provider_handoff",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Classification stage a class passes through from pack load to a scope-resolved, evidence-and-checks-evaluated, local-provider-parity-resolved, and review-result-recorded review-pack object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackClassificationStage {
    /// The pack-loaded stage: the declarative review pack and its version / digest are loaded.
    PackLoaded,
    /// The scope-resolved stage: the pack's scope selectors and owned slices are resolved.
    ScopeResolved,
    /// The evidence-and-checks-evaluated stage: required evidence and checks are evaluated to a result class.
    EvidenceAndChecksEvaluated,
    /// The local-provider-parity-resolved stage: the local-versus-provider parity strip is resolved.
    LocalProviderParityResolved,
    /// The review-result-recorded stage: the evaluator result and template attribution are recorded.
    ReviewResultRecorded,
}

impl M5ReviewPackClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PackLoaded,
        Self::ScopeResolved,
        Self::EvidenceAndChecksEvaluated,
        Self::LocalProviderParityResolved,
        Self::ReviewResultRecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackLoaded => "pack_loaded",
            Self::ScopeResolved => "scope_resolved",
            Self::EvidenceAndChecksEvaluated => "evidence_and_checks_evaluated",
            Self::LocalProviderParityResolved => "local_provider_parity_resolved",
            Self::ReviewResultRecorded => "review_result_recorded",
        }
    }
}

/// Shared consumer surface that must agree on a class's review-pack truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackConsumerSurface {
    /// The review detail surface.
    ReviewDetail,
    /// The merge-readiness component.
    MergeReadiness,
    /// The AI review panel.
    AiReviewPanel,
    /// The provider handoff surface.
    ProviderHandoff,
    /// The review-pack summary surface.
    ReviewPackSummary,
    /// The ownership overlay.
    OwnershipOverlay,
    /// The local-CI parity strip.
    LocalCiParityStrip,
    /// The support / export packet.
    SupportExportPacket,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ReviewPackConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewDetail,
        Self::MergeReadiness,
        Self::AiReviewPanel,
        Self::ProviderHandoff,
        Self::ReviewPackSummary,
        Self::OwnershipOverlay,
        Self::LocalCiParityStrip,
        Self::SupportExportPacket,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewDetail => "review_detail",
            Self::MergeReadiness => "merge_readiness",
            Self::AiReviewPanel => "ai_review_panel",
            Self::ProviderHandoff => "provider_handoff",
            Self::ReviewPackSummary => "review_pack_summary",
            Self::OwnershipOverlay => "ownership_overlay",
            Self::LocalCiParityStrip => "local_ci_parity_strip",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no review-pack meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5ReviewPackAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified review-pack-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackDegradedReason {
    /// The review-pack version / digest badge has gone stale.
    PackVersionDigestStale,
    /// The advisory-versus-enforced owner provenance is unresolved.
    OwnerProvenanceUnresolved,
    /// The evaluator result class is unresolved.
    EvaluatorResultClassUnresolved,
    /// The local-versus-provider parity state is unknown.
    LocalVersusProviderParityUnknown,
    /// The pack freshness / invalidation state is unknown.
    PackFreshnessUnknown,
    /// The comment / summary template attribution is unresolved.
    TemplateAttributionUnresolved,
}

impl M5ReviewPackDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackVersionDigestStale,
        Self::OwnerProvenanceUnresolved,
        Self::EvaluatorResultClassUnresolved,
        Self::LocalVersusProviderParityUnknown,
        Self::PackFreshnessUnknown,
        Self::TemplateAttributionUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackVersionDigestStale => "pack_version_digest_stale",
            Self::OwnerProvenanceUnresolved => "owner_provenance_unresolved",
            Self::EvaluatorResultClassUnresolved => "evaluator_result_class_unresolved",
            Self::LocalVersusProviderParityUnknown => "local_versus_provider_parity_unknown",
            Self::PackFreshnessUnknown => "pack_freshness_unknown",
            Self::TemplateAttributionUnresolved => "template_attribution_unresolved",
        }
    }
}

/// Mandatory label a claimed review-pack class must be able to show. The first three are hard requirements; the remaining three make the review-pack version / digest, the evaluator result class, and the template attribution mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's review-pack role.
    PackRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The review-pack version / digest the class must show.
    PackVersionDigest,
    /// The evaluator result class the class must state.
    EvaluatorResultClass,
    /// The comment / summary template attribution the class must state.
    TemplateAttribution,
}

impl M5ReviewPackRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::PackRole,
        Self::CanonicalReference,
        Self::PackVersionDigest,
        Self::EvaluatorResultClass,
        Self::TemplateAttribution,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::PackRole => "pack_role",
            Self::CanonicalReference => "canonical_reference",
            Self::PackVersionDigest => "pack_version_digest",
            Self::EvaluatorResultClass => "evaluator_result_class",
            Self::TemplateAttribution => "template_attribution",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::PackRole, Self::CanonicalReference];
}

/// Qualification class for an M5 review-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackQualificationClass {
    /// Class review-pack handling qualifies for the Stable claim.
    Stable,
    /// Class review-pack handling is narrowed to Beta.
    Beta,
    /// Class review-pack handling is narrowed to Preview.
    Preview,
    /// Class review-pack handling is experimental and not claimed.
    Experimental,
    /// Class review-pack handling is unavailable on this build.
    Unavailable,
    /// Class review-pack handling is held pending review.
    Held,
}

impl M5ReviewPackQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

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
    /// Whether the class may carry a public Stable review-pack-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a review-pack class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackDowngradeTrigger {
    /// A local parity estimate was shown as provider-authoritative mergeability.
    LocalEstimateShownAsProviderAuthoritative,
    /// A ci-only, not-evaluated-here, or provider-unavailable check was hidden behind a green summary.
    UnevaluatedCheckHiddenBehindGreenSummary,
    /// Advisory-owner and enforced-owner were flattened into one owner pill.
    AdvisoryAndEnforcedOwnerFlattened,
    /// An AI review ran under a different pack version without disclosure.
    AiReviewRanUnderUndisclosedPackVersion,
    /// A review-pack version or digest was dropped on export, publish, or reopen.
    PackVersionOrDigestDropped,
    /// A comment / summary template attribution was dropped on export, publish, or reopen.
    TemplateAttributionDropped,
    /// A class left its review-pack version / digest unstated.
    PackVersionDigestUnstated,
    /// A class left its advisory-versus-enforced owner provenance unstated.
    OwnerProvenanceUnstated,
    /// A class left its evaluator result class unstated.
    EvaluatorResultClassUnstated,
    /// A class left its local-versus-provider parity state unstated.
    ParityStateUnstated,
    /// A class left its pack freshness / invalidation state unstated.
    PackFreshnessUnstated,
    /// The review-pack matrix packet has gone stale.
    ReviewPackMatrixStale,
}

impl M5ReviewPackDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::LocalEstimateShownAsProviderAuthoritative,
        Self::UnevaluatedCheckHiddenBehindGreenSummary,
        Self::AdvisoryAndEnforcedOwnerFlattened,
        Self::AiReviewRanUnderUndisclosedPackVersion,
        Self::PackVersionOrDigestDropped,
        Self::TemplateAttributionDropped,
        Self::PackVersionDigestUnstated,
        Self::OwnerProvenanceUnstated,
        Self::EvaluatorResultClassUnstated,
        Self::ParityStateUnstated,
        Self::PackFreshnessUnstated,
        Self::ReviewPackMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEstimateShownAsProviderAuthoritative => {
                "local_estimate_shown_as_provider_authoritative"
            }
            Self::UnevaluatedCheckHiddenBehindGreenSummary => {
                "unevaluated_check_hidden_behind_green_summary"
            }
            Self::AdvisoryAndEnforcedOwnerFlattened => "advisory_and_enforced_owner_flattened",
            Self::AiReviewRanUnderUndisclosedPackVersion => {
                "ai_review_ran_under_undisclosed_pack_version"
            }
            Self::PackVersionOrDigestDropped => "pack_version_or_digest_dropped",
            Self::TemplateAttributionDropped => "template_attribution_dropped",
            Self::PackVersionDigestUnstated => "pack_version_digest_unstated",
            Self::OwnerProvenanceUnstated => "owner_provenance_unstated",
            Self::EvaluatorResultClassUnstated => "evaluator_result_class_unstated",
            Self::ParityStateUnstated => "parity_state_unstated",
            Self::PackFreshnessUnstated => "pack_freshness_unstated",
            Self::ReviewPackMatrixStale => "review_pack_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so a review-pack result never reads without its version / digest,
/// owner provenance, evaluator result class, or parity state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackVisibleState {
    /// Pack / object label shown on the surface (review-pack summary, ownership overlay, parity strip, template preview).
    pub pack_label: String,
    /// Review-pack version and content digest.
    pub pack_version_and_digest: String,
    /// Advisory-owner-versus-enforced-owner provenance for the covered scope slice.
    pub owner_provenance: String,
    /// Evaluator result class (provider-authoritative, local-parity-estimate, not-evaluated-here, ci-only).
    pub evaluator_result_class: String,
    /// Local-parity-estimate-versus-provider-authoritative state disclosed before any mergeability claim.
    pub local_versus_provider_parity: String,
    /// Pack freshness / invalidation state (pack fresh, stale pack, partial scope, slice omitted, pack invalid).
    pub pack_freshness_state: String,
    /// Comment / summary template attribution kept bound to the pack.
    pub template_attribution: String,
}

impl M5ReviewPackVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.pack_label.trim().is_empty()
            && !self.pack_version_and_digest.trim().is_empty()
            && !self.owner_provenance.trim().is_empty()
            && !self.evaluator_result_class.trim().is_empty()
            && !self.local_versus_provider_parity.trim().is_empty()
            && !self.pack_freshness_state.trim().is_empty()
            && !self.template_attribution.trim().is_empty()
    }
}

/// One row in the matrix: one governed review-pack object class bound to the surface-specific
/// review-pack truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackRow {
    /// Governed review-pack object class.
    pub object_class: M5ReviewPackObject,
    /// Qualification class earned by this class's review-pack handling.
    pub qualification: M5ReviewPackQualificationClass,
    /// Parity state this row governs (distinguishes a provider-authoritative result from a local parity estimate or an unevaluated lane).
    pub parity_state: M5ReviewPackParityState,
    /// Owner role accountable for keeping this class's review-pack state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's review-pack result visibly versioned, owner-attributed, and parity-honest.
    pub required_visible_state: M5ReviewPackVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5ReviewPackSurfaceFamily>,
    /// Classification stages this class passes through from pack load to a recorded review result.
    pub classification_stages: Vec<M5ReviewPackClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5ReviewPackRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ReviewPackRequiredLabel>,
    /// Review-pack roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5ReviewPackRole>,
    /// ReviewPackRecord roles this class names (ReviewPackRecord only).
    pub review_pack_record_roles: Vec<M5ReviewPackRecordRole>,
    /// OwnershipSignal roles this class names (OwnershipSignal only).
    pub ownership_signal_roles: Vec<M5ReviewPackOwnershipSignalRole>,
    /// RequiredEvidenceCheckRow roles this class names (RequiredEvidenceCheckRow only).
    pub required_evidence_roles: Vec<M5ReviewPackRequiredEvidenceRole>,
    /// LocalCiParityStrip roles this class names (LocalCiParityStrip only).
    pub local_ci_parity_roles: Vec<M5ReviewPackLocalCiParityRole>,
    /// AiPolicyHook roles this class names (AiPolicyHook only).
    pub ai_policy_hook_roles: Vec<M5ReviewPackAiPolicyHookRole>,
    /// ReviewTemplatePacket roles this class names (ReviewTemplatePacket only).
    pub template_packet_roles: Vec<M5ReviewPackTemplatePacketRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5ReviewPackDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5ReviewPackAccessibilityRoute>,
    /// First consumer surfaces that consume this class's review-pack projection.
    pub consumer_surfaces: Vec<M5ReviewPackConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's review-pack state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets a local parity estimate masquerade as provider-authoritative mergeability or approval truth. MUST be `false`.
    pub lets_a_local_parity_estimate_masquerade_as_provider_authoritative: bool,
    /// Hard invariant: this class never hides a ci-only, not-evaluated-here, or provider-unavailable state behind a green summary state. MUST be `false`.
    pub hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: bool,
    /// Hard invariant: this class never flattens advisory-owner and enforced-owner into one owner pill. MUST be `false`.
    pub flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: bool,
    /// Hard invariant: this class never lets AI review run under a different pack version without disclosure. MUST be `false`.
    pub lets_ai_review_run_under_a_different_pack_version_without_disclosure: bool,
    /// Hard invariant: this class never loses the review-pack version / digest or template attribution when exporting, publishing, or reopening review evidence. MUST be `false`.
    pub loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening:
        bool,
}

impl M5ReviewPackRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ReviewPackRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ReviewPackRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_a_local_parity_estimate_masquerade_as_provider_authoritative
            && !self.hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary
            && !self.flattens_advisory_owner_and_enforced_owner_into_one_owner_pill
            && !self.lets_ai_review_run_under_a_different_pack_version_without_disclosure
            && !self.loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Parity states tokens.
    pub parity_states: Vec<String>,
    /// Owner authorities tokens.
    pub owner_authorities: Vec<String>,
    /// Pack freshnesses tokens.
    pub pack_freshnesses: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Review pack record roles tokens.
    pub review_pack_record_roles: Vec<String>,
    /// Ownership signal roles tokens.
    pub ownership_signal_roles: Vec<String>,
    /// Required evidence roles tokens.
    pub required_evidence_roles: Vec<String>,
    /// Local ci parity roles tokens.
    pub local_ci_parity_roles: Vec<String>,
    /// Ai policy hook roles tokens.
    pub ai_policy_hook_roles: Vec<String>,
    /// Template packet roles tokens.
    pub template_packet_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5ReviewPackVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5ReviewPackObject::ALL, |v| v.as_str()),
            parity_states: tokens(&M5ReviewPackParityState::ALL, |v| v.as_str()),
            owner_authorities: tokens(&M5ReviewPackOwnerAuthority::ALL, |v| v.as_str()),
            pack_freshnesses: tokens(&M5ReviewPackFreshness::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5ReviewPackRole::ALL, |v| v.as_str()),
            review_pack_record_roles: tokens(&M5ReviewPackRecordRole::ALL, |v| v.as_str()),
            ownership_signal_roles: tokens(&M5ReviewPackOwnershipSignalRole::ALL, |v| v.as_str()),
            required_evidence_roles: tokens(&M5ReviewPackRequiredEvidenceRole::ALL, |v| v.as_str()),
            local_ci_parity_roles: tokens(&M5ReviewPackLocalCiParityRole::ALL, |v| v.as_str()),
            ai_policy_hook_roles: tokens(&M5ReviewPackAiPolicyHookRole::ALL, |v| v.as_str()),
            template_packet_roles: tokens(&M5ReviewPackTemplatePacketRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ReviewPackSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5ReviewPackClassificationStage::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ReviewPackConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ReviewPackAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5ReviewPackDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ReviewPackRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5ReviewPackDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackGovernanceReview {
    /// No local parity estimate is shown as provider authoritative.
    pub no_local_parity_estimate_is_shown_as_provider_authoritative: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Provider authoritative state is mechanically distinct from local parity estimate.
    pub provider_authoritative_state_is_mechanically_distinct_from_local_parity_estimate: bool,
    /// Every review pack names its version and digest.
    pub every_review_pack_names_its_version_and_digest: bool,
    /// Every ownership signal names advisory versus enforced owner.
    pub every_ownership_signal_names_advisory_versus_enforced_owner: bool,
    /// Every required check names its evaluator result class.
    pub every_required_check_names_its_evaluator_result_class: bool,
    /// No ci only or not evaluated here check is hidden behind a green summary.
    pub no_ci_only_or_not_evaluated_here_check_is_hidden_behind_a_green_summary: bool,
    /// Every ai review run discloses its pack version and digest.
    pub every_ai_review_run_discloses_its_pack_version_and_digest: bool,
    /// No review pack version digest or template attribution is lost on export publish or reopen.
    pub no_review_pack_version_digest_or_template_attribution_is_lost_on_export_publish_or_reopen:
        bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single review pack source.
    pub support_export_reads_single_review_pack_source: bool,
    /// Review merge ai provider and support bind to single source.
    pub review_merge_ai_provider_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel review pack vocabulary.
    pub later_rows_cannot_invent_parallel_review_pack_vocabulary: bool,
    /// Review pack truth survives zoom and high contrast.
    pub review_pack_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackConsumerProjection {
    /// Review detail and merge readiness consume shared review pack result truth.
    pub review_detail_and_merge_readiness_consume_shared_review_pack_result_truth: bool,
    /// Ai review and provider handoff consume shared pack version truth.
    pub ai_review_and_provider_handoff_consume_shared_pack_version_truth: bool,
    /// Help and support export consume shared ownership and parity truth.
    pub help_and_support_export_consume_shared_ownership_and_parity_truth: bool,
    /// Docs help and screenshots read single review pack source.
    pub docs_help_and_screenshots_read_single_review_pack_source: bool,
    /// Review packs bind to shared local ci parity relation.
    pub review_packs_bind_to_shared_local_ci_parity_relation: bool,
    /// Support export reads single review pack source.
    pub support_export_reads_single_review_pack_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the review-pack lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting review-pack audit for the lane.
    pub review_pack_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ReviewPackMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewPackMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Review-pack rows.
    pub review_pack_rows: Vec<M5ReviewPackRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReviewPackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReviewPackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewPackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReviewPackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 review-pack matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackMatrixPacket {
    /// Record kind; must equal [`M5_REVIEW_PACK_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_PACK_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Review-pack rows.
    pub review_pack_rows: Vec<M5ReviewPackRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReviewPackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReviewPackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewPackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReviewPackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReviewPackMatrixPacket {
    /// Builds an M5 review-pack matrix packet from input.
    pub fn new(input: M5ReviewPackMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_PACK_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_PACK_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            review_pack_rows: input.review_pack_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 review-pack matrix invariants.
    pub fn validate(&self) -> Vec<M5ReviewPackMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_REVIEW_PACK_MATRIX_RECORD_KIND {
            violations.push(M5ReviewPackMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_PACK_MATRIX_SCHEMA_VERSION {
            violations.push(M5ReviewPackMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReviewPackMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_review_pack_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 review-pack matrix serializes"),
        ) {
            violations.push(M5ReviewPackMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 review-pack matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed review-pack class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,parity_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.review_pack_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.parity_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic review-pack-health dashboard JSON that review and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .review_pack_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "parity_state": row.parity_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_review_pack_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_REVIEW_PACK_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 review-pack-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .review_pack_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Review Pack: Review-Pack, Ownership, Required Evidence, Local-CI Parity, AI Policy Hook, and Review Template Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.review_pack_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Review-pack roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.review_pack_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (parity_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.parity_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Evaluator result class: {}\n",
                row.required_visible_state.evaluator_result_class
            ));
            out.push_str(&format!(
                "  - Pack freshness state: {}\n",
                row.required_visible_state.pack_freshness_state
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 review-pack matrix export.
#[derive(Debug)]
pub enum M5ReviewPackMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReviewPackMatrixViolation>),
}

impl fmt::Display for M5ReviewPackMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 review-pack matrix export parse failed: {error}"
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
                    "m5 review-pack matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReviewPackMatrixArtifactError {}

/// Validation failures emitted by [`M5ReviewPackMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReviewPackMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A review-pack row is incomplete.
    ReviewPackRowIncomplete,
    /// A review-pack row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A review-pack row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no review-pack roles.
    SemanticRoleMissing,
    /// The ReviewPackRecord class declares no ReviewPackRecord roles.
    ReviewPackRecordRoleMissing,
    /// The OwnershipSignal class declares no OwnershipSignal roles.
    OwnershipSignalRoleMissing,
    /// The RequiredEvidenceCheckRow class declares no RequiredEvidenceCheckRow roles.
    RequiredEvidenceRoleMissing,
    /// The LocalCiParityStrip class declares no LocalCiParityStrip roles.
    LocalCiParityRoleMissing,
    /// The AiPolicyHook class declares no AiPolicyHook roles.
    AiPolicyHookRoleMissing,
    /// The ReviewTemplatePacket class declares no ReviewTemplatePacket roles.
    TemplatePacketRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard review-pack invariant.
    ReviewPackInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ReviewPackMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ReviewPackRowIncomplete => "review_pack_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ReviewPackRecordRoleMissing => "review_pack_record_role_missing",
            Self::OwnershipSignalRoleMissing => "ownership_signal_role_missing",
            Self::RequiredEvidenceRoleMissing => "required_evidence_role_missing",
            Self::LocalCiParityRoleMissing => "local_ci_parity_role_missing",
            Self::AiPolicyHookRoleMissing => "ai_policy_hook_role_missing",
            Self::TemplatePacketRoleMissing => "template_packet_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::ReviewPackInvariantViolated => "review_pack_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 review-pack matrix export.
pub fn current_stable_m5_review_pack_matrix_export(
) -> Result<M5ReviewPackMatrixPacket, M5ReviewPackMatrixArtifactError> {
    let packet: M5ReviewPackMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-results/support_export.json"
    )))
    .map_err(M5ReviewPackMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReviewPackMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
        M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
        M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
        M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReviewPackMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ReviewPackMatrixViolation::VocabularySetDrift);
    }
}

fn validate_review_pack_rows(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    let present: BTreeSet<M5ReviewPackObject> = packet
        .review_pack_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5ReviewPackObject::ALL {
        if !present.contains(&required) {
            violations.push(M5ReviewPackMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.review_pack_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ReviewPackMatrixViolation::ReviewPackRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ReviewPackMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5ReviewPackMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_review_pack_record_roles() && row.review_pack_record_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::ReviewPackRecordRoleMissing);
        }
        if class.declares_ownership_signal_roles() && row.ownership_signal_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::OwnershipSignalRoleMissing);
        }
        if class.declares_required_evidence_roles() && row.required_evidence_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::RequiredEvidenceRoleMissing);
        }
        if class.declares_local_ci_parity_roles() && row.local_ci_parity_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::LocalCiParityRoleMissing);
        }
        if class.declares_ai_policy_hook_roles() && row.ai_policy_hook_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::AiPolicyHookRoleMissing);
        }
        if class.declares_template_packet_roles() && row.template_packet_roles.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::TemplatePacketRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5ReviewPackMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations.push(M5ReviewPackMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations.push(M5ReviewPackMatrixViolation::ReviewPackInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_local_parity_estimate_is_shown_as_provider_authoritative,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.provider_authoritative_state_is_mechanically_distinct_from_local_parity_estimate,
        review.every_review_pack_names_its_version_and_digest,
        review.every_ownership_signal_names_advisory_versus_enforced_owner,
        review.every_required_check_names_its_evaluator_result_class,
        review.no_ci_only_or_not_evaluated_here_check_is_hidden_behind_a_green_summary,
        review.every_ai_review_run_discloses_its_pack_version_and_digest,
        review.no_review_pack_version_digest_or_template_attribution_is_lost_on_export_publish_or_reopen,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_review_pack_source,
        review.review_merge_ai_provider_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_review_pack_vocabulary,
        review.review_pack_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5ReviewPackMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_detail_and_merge_readiness_consume_shared_review_pack_result_truth,
        projection.ai_review_and_provider_handoff_consume_shared_pack_version_truth,
        projection.help_and_support_export_consume_shared_ownership_and_parity_truth,
        projection.docs_help_and_screenshots_read_single_review_pack_source,
        projection.review_packs_bind_to_shared_local_ci_parity_relation,
        projection.support_export_reads_single_review_pack_source,
    ] {
        if !ok {
            violations.push(M5ReviewPackMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReviewPackMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ReviewPackMatrixPacket,
    violations: &mut Vec<M5ReviewPackMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.review_pack_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ReviewPackMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses review / pack / owner / check / parity words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
