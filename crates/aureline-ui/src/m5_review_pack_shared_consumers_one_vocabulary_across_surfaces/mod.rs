//! Shared review-detail, merge-readiness, AI-review-panel, provider-handoff, review-pack-summary,
//! ownership-overlay, local-CI-parity-strip, and support / export consumers that keep the B152 review-pack
//! evaluator objects — the review-pack record, the ownership signal, the required-evidence / required-check
//! row, the local-CI parity strip, the AI review policy hook, and the review-template packet — at **one
//! canonical vocabulary** across every claimed M5 review, merge-readiness, AI-review, and provider-backed
//! review surface.
//!
//! This module is the consumer-adoption capstone for the six governed review-pack object classes frozen in
//! [`crate::m5_review_pack_evaluator_matrix`] and implemented by the review-pack record / result lane
//! ([`crate::m5_review_pack_record_and_result_registries`]), the ownership-signal / owner-conflict lane, the
//! required-evidence-check / local-CI-parity lane, the AI-policy-hook / AI-policy-result lane, the
//! review-template-packet / publish-attribution lane
//! ([`crate::m5_review_template_packet_and_publish_attribution_registries`]), and the invalidation /
//! rerun-compare lane ([`crate::m5_review_pack_invalidation_and_rerun_compare_registries`]).
//!
//! It binds each shared review-pack object to the concrete review-list / detail, merge-readiness,
//! merge-queue, AI review, support / export, and browser / provider-handoff consumers — projected here through
//! the review-detail, merge-readiness, AI-review-panel, provider-handoff, review-pack-summary,
//! ownership-overlay, local-CI-parity-strip, support-export, and help / docs surfaces — that render it, and
//! proves — by fixtures, not screenshots — that the same seeded review-pack subject presents the same
//! review-pack-role, object, registry-reference, parity-state, surface-context, and pack-freshness vocabulary
//! wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the six shared review-pack objects must be adopted by at least two distinct
//!    consumers, so an object is proven to be shared review-pack infrastructure rather than a one-surface,
//!    feature-local fork of the review-pack record, ownership signal, required-check row, parity strip,
//!    AI-policy hook, or template packet contract.
//! 2. **One vocabulary / no drift.** For a given seeded review-pack subject every consumer surface must
//!    present identical [`ReviewPackSharedStateFacetValues`] — the same review-pack-role word, the same object
//!    word, the same registry-reference word, the same parity-state word, the same surface-context word, and
//!    the same pack-freshness word. The review-pack-role word must be a token from the frozen
//!    [`M5ReviewPackRole`] vocabulary, so no surface rewrites `pack_version_and_digest_disclosure`,
//!    `owner_provenance_disclosure`, `evaluator_result_class_disclosure`,
//!    `local_versus_provider_parity_disclosure`, `required_evidence_and_check_disclosure`,
//!    `template_attribution_disclosure`, or `pack_freshness_and_invalidation_disclosure` in its own words. A
//!    surface may narrow *how much* it shows across desktop, compact, remote, and exported representations, but
//!    it may never reword the underlying vocabulary per surface, and a role that carries pack-version / digest,
//!    owner-provenance, evaluator-result-class, or local-versus-provider meaning may never let a local parity
//!    estimate masquerade as provider-authoritative, hide a ci-only / not-evaluated-here / provider-unavailable
//!    state behind a green summary, flatten advisory-owner and enforced-owner into one owner pill, let AI
//!    review run under a different pack version without disclosure, or lose the review-pack version / digest or
//!    template attribution on export.
//! 3. **Map back to one object.** Support / export consumers must point at the canonical per-domain schema and
//!    the frozen matrix by id, so an exported packet — and every copy / export / open-in-provider action — can
//!    always map a review-detail / merge-readiness / support surface back to one shared contract object rather
//!    than diverging into a surface-local payload.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`ReviewPackSharedNarrowNote`] naming the reason, the preserved vocabulary, and the next action, and an
//! exported representation additionally names its export-safe detail boundary rather than collapsing the
//! subject out of view.
//!
//! The packet references upstream review-pack contracts by id rather than embedding their content. Raw secret
//! values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/m5-review-pack-shared-consumers.schema.json`](../../../../schemas/review/m5-review-pack-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/review/m5_review_pack_shared_consumers_one_vocabulary.md`](../../../../docs/review/m5_review_pack_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/review/m5-review-pack-shared-consumers/`](../../../../fixtures/review/m5-review-pack-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_review_pack_shared_consumers,
    seeded_m5_review_pack_shared_consumers_compact_remote_narrowed,
    seeded_m5_review_pack_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_review_pack_evaluator_matrix::{
    M5ReviewPackConsumerSurface, M5ReviewPackObject, M5ReviewPackRole,
    M5_REVIEW_PACK_MATRIX_DOC_REF, M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ReviewPackSharedConsumersPacket`].
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_review_pack_shared_consumer_registry_parity";

/// Schema version for review-pack shared-consumer parity records.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-review-pack-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/review/m5-review-pack-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/review/m5_review_pack_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/review/m5-review-pack-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/review/m5-review-pack-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/review/m5-review-pack-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/review/m5-review-pack-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_REVIEW_PACK_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Pack-freshness sentinel words a pack-version / owner-provenance / evaluator-result / local-versus-provider
/// gate role may never fall back to; a gate-carrying role that changes surface presentation must always keep a
/// real pack-fresh-scope-bound-and-parity-disclosed continuity, never showing a stale pack as fresh, a partial
/// scope as full, a local estimate as provider-authoritative, or an advisory owner as enforced.
const PACK_FRESHNESS_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "stale_pack_shown_as_fresh",
    "partial_scope_shown_as_full",
    "local_estimate_shown_as_provider_authoritative",
    "advisory_owner_shown_as_enforced",
];

/// Whether a consumer surface is an export / support path that must map an object back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5ReviewPackConsumerSurface) -> bool {
    matches!(consumer, M5ReviewPackConsumerSurface::SupportExportPacket)
}

/// Whether `token` is a member of the frozen [`M5ReviewPackRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a seeded subject's review-pack-role word must be a controlled role token
/// rather than a per-surface synonym.
pub fn is_known_review_pack_role_token(token: &str) -> bool {
    review_pack_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ReviewPackRole`], if it is one.
pub fn review_pack_role_from_token(token: &str) -> Option<M5ReviewPackRole> {
    M5ReviewPackRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared review-pack object a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed representation still
/// carries the same review-pack-role, object, registry-reference, parity-state, surface-context, and
/// pack-freshness words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSharedRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl ReviewPackSharedRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSharedParityFacet {
    /// The frozen review-pack-role word.
    ReviewPackRoleWord,
    /// The review-pack-object word.
    ObjectWord,
    /// The canonical registry-reference word the object points at.
    RegistryReferenceWord,
    /// The parity-state word (provider-authoritative / local-parity-estimate / ci-only / not-evaluated-here /
    /// provider-unavailable / stale-relative-to-base-head / draft-only-review-state) the subject ships.
    ParityStateWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The pack-freshness word paired with a pack-version / owner-provenance / evaluator-result /
    /// local-versus-provider gate role.
    PackFreshnessWord,
}

impl ReviewPackSharedParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewPackRoleWord,
        Self::ObjectWord,
        Self::RegistryReferenceWord,
        Self::ParityStateWord,
        Self::SurfaceContextWord,
        Self::PackFreshnessWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPackRoleWord => "review_pack_role_word",
            Self::ObjectWord => "object_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::ParityStateWord => "parity_state_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::PackFreshnessWord => "pack_freshness_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared review-pack object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSharedNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl ReviewPackSharedNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSharedNarrowNextAction {
    /// Expand the object in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ReviewPackSharedNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackSharedVocabularyState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ReviewPackSharedVocabularyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Review-pack vocabulary drifted between surfaces for the same subject.
    ReviewPackVocabularyDriftDetected,
    /// A gate-carrying role dropped its pack-freshness or parity disclosure.
    PackFreshnessOrParityDisclosureDropped,
    /// A surface let a local parity estimate masquerade as provider-authoritative.
    LetsALocalParityEstimateMasqueradeAsProviderAuthoritative,
    /// A surface hid a ci-only, not-evaluated-here, or provider-unavailable state behind a green summary.
    HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary,
    /// A surface flattened advisory-owner and enforced-owner into one owner pill.
    FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill,
    /// A surface let AI review run under a different pack version without disclosure.
    LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure,
    /// A surface lost the review-pack version / digest or template attribution on export / publish / reopen.
    LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared review-pack object narrowed.
    UpstreamReviewPackNarrowed,
}

impl M5ReviewPackSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ReviewPackVocabularyDriftDetected,
        Self::PackFreshnessOrParityDisclosureDropped,
        Self::LetsALocalParityEstimateMasqueradeAsProviderAuthoritative,
        Self::HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary,
        Self::FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill,
        Self::LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure,
        Self::LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamReviewPackNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ReviewPackVocabularyDriftDetected => "review_pack_vocabulary_drift_detected",
            Self::PackFreshnessOrParityDisclosureDropped => {
                "pack_freshness_or_parity_disclosure_dropped"
            }
            Self::LetsALocalParityEstimateMasqueradeAsProviderAuthoritative => {
                "lets_a_local_parity_estimate_masquerade_as_provider_authoritative"
            }
            Self::HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary => {
                "hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary"
            }
            Self::FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill => {
                "flattens_advisory_owner_and_enforced_owner_into_one_owner_pill"
            }
            Self::LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure => {
                "lets_ai_review_run_under_a_different_pack_version_without_disclosure"
            }
            Self::LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening => {
                "loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamReviewPackNarrowed => "upstream_review_pack_narrowed",
        }
    }
}

/// The controlled vocabulary a seeded review-pack subject presents.
///
/// These six words must be identical across every consumer surface that shows the same seeded subject. The
/// review-pack-role word must be a frozen role token; the rest are controlled words the subject's object
/// carries. A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSharedStateFacetValues {
    /// Review-pack-role word (must be a frozen [`M5ReviewPackRole`] token).
    pub review_pack_role_word: String,
    /// Review-pack-object word.
    pub object_word: String,
    /// Canonical registry-reference word the object points at.
    pub registry_reference_word: String,
    /// Parity-state word (provider-authoritative / local-parity-estimate / ci-only / not-evaluated-here /
    /// provider-unavailable / stale-relative-to-base-head / draft-only-review-state) the subject ships.
    pub parity_state_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Pack-freshness word paired with a pack-version / owner-provenance / evaluator-result /
    /// local-versus-provider gate role.
    pub pack_freshness_word: String,
}

impl ReviewPackSharedStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.review_pack_role_word.trim().is_empty()
            && !self.object_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.parity_state_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.pack_freshness_word.trim().is_empty()
    }

    /// Whether the review-pack-role word is a member of the frozen role vocabulary.
    pub fn review_pack_role_word_in_vocabulary(&self) -> bool {
        is_known_review_pack_role_token(self.review_pack_role_word.trim())
    }

    /// Whether the subject honours the pack-freshness rule: a role that carries pack-version / digest,
    /// owner-provenance, evaluator-result-class, or local-versus-provider meaning must pair its surface change
    /// with a real pack-fresh-scope-bound-and-parity-disclosed continuity and never collapse to a
    /// stale-pack-shown-as-fresh, partial-scope-shown-as-full,
    /// local-estimate-shown-as-provider-authoritative, or advisory-owner-shown-as-enforced sentinel.
    pub fn pack_freshness_satisfied(&self) -> bool {
        match review_pack_role_from_token(self.review_pack_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_a_review_pack_result() => {
                let freshness = self.pack_freshness_word.trim().to_lowercase();
                !freshness.is_empty()
                    && !PACK_FRESHNESS_ABSENT_SENTINELS.contains(&freshness.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSharedNarrowNote {
    /// Why the representation narrowed.
    pub reason: ReviewPackSharedNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: ReviewPackSharedNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewPackSharedRenderDisclosure {
    /// The vocabulary state the representation requires.
    pub vocabulary_state: ReviewPackSharedVocabularyState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<ReviewPackSharedNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ReviewPackSharedNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure depth, a
/// remote-projected representation names its remote source, and an exported representation names its
/// export-safe-detail boundary — but all three keep every vocabulary word and disclose the narrowing through an
/// explicit note.
pub const fn resolve_review_pack_shared_render_disclosure(
    representation: ReviewPackSharedRepresentation,
) -> ReviewPackSharedRenderDisclosure {
    match representation {
        ReviewPackSharedRepresentation::DesktopFull => ReviewPackSharedRenderDisclosure {
            vocabulary_state: ReviewPackSharedVocabularyState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ReviewPackSharedRepresentation::CompactNarrowed => ReviewPackSharedRenderDisclosure {
            vocabulary_state: ReviewPackSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ReviewPackSharedNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(ReviewPackSharedNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ReviewPackSharedRepresentation::RemoteProjected => ReviewPackSharedRenderDisclosure {
            vocabulary_state: ReviewPackSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ReviewPackSharedNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(ReviewPackSharedNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        ReviewPackSharedRepresentation::ExportedRedacted => ReviewPackSharedRenderDisclosure {
            vocabulary_state: ReviewPackSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ReviewPackSharedNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(ReviewPackSharedNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared review-pack object rendered on one consumer surface in one representation for
/// one seeded review-pack subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSharedConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable subject id (shared across surfaces that show the same subject).
    pub subject_id: String,
    /// Human-readable subject identity.
    pub subject_label: String,
    /// Which shared review-pack object this binding renders.
    pub object: M5ReviewPackObject,
    /// Which consumer surface renders it.
    pub consumer: M5ReviewPackConsumerSurface,
    /// Which representation this surface renders.
    pub representation: ReviewPackSharedRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one subject).
    pub state_facets: ReviewPackSharedStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub vocabulary_state: ReviewPackSharedVocabularyState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<ReviewPackSharedNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets a local parity estimate masquerade as provider-authoritative. MUST be
    /// `false`.
    pub lets_a_local_parity_estimate_masquerade_as_provider_authoritative: bool,
    /// Guardrail: this surface hides a ci-only, not-evaluated-here, or provider-unavailable state behind a
    /// green summary. MUST be `false`.
    pub hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: bool,
    /// Guardrail: this surface flattens advisory-owner and enforced-owner into one owner pill. MUST be
    /// `false`.
    pub flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: bool,
    /// Guardrail: this surface lets AI review run under a different pack version without disclosure. MUST be
    /// `false`.
    pub lets_ai_review_run_under_a_different_pack_version_without_disclosure: bool,
    /// Guardrail: this surface loses the review-pack version / digest or template attribution when exporting,
    /// publishing, or reopening. MUST be `false`.
    pub loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening:
        bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ReviewPackSharedConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> ReviewPackSharedRenderDisclosure {
        resolve_review_pack_shared_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.lets_a_local_parity_estimate_masquerade_as_provider_authoritative
            && !self.hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary
            && !self.flattens_advisory_owner_and_enforced_owner_into_one_owner_pill
            && !self.lets_ai_review_run_under_a_different_pack_version_without_disclosure
            && !self
                .loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_REVIEW_PACK_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackSharedConsumersTrustReview {
    /// Object reuse is proven by fixtures rather than inferred from screenshots.
    pub object_reuse_proven_by_fixtures: bool,
    /// The same seeded subject presents the same vocabulary across surfaces.
    pub same_subject_same_review_pack_vocabulary_across_surfaces: bool,
    /// Every review-pack-role word is a frozen role token.
    pub review_pack_role_words_stay_in_frozen_vocabulary: bool,
    /// Gate-carrying roles never let a local estimate read as provider-authoritative.
    pub gate_roles_never_let_local_estimate_read_as_provider_authoritative: bool,
    /// A surface never hides a ci-only, not-evaluated-here, or provider-unavailable state.
    pub ci_only_not_evaluated_here_and_provider_unavailable_never_hidden: bool,
    /// A surface never flattens advisory-owner and enforced-owner into one owner pill.
    pub advisory_and_enforced_owner_never_flattened: bool,
    /// AI review never runs under a different pack version without disclosure.
    pub ai_review_never_runs_under_a_different_pack_version_without_disclosure: bool,
    /// A surface never loses the review-pack version / digest or template attribution on export / publish /
    /// reopen.
    pub pack_version_digest_and_template_attribution_never_lost: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Copy / export / open-in-provider actions preserve one canonical payload rather than diverging.
    pub copy_export_open_provider_preserve_one_payload: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl M5ReviewPackSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_reuse_proven_by_fixtures
            && self.same_subject_same_review_pack_vocabulary_across_surfaces
            && self.review_pack_role_words_stay_in_frozen_vocabulary
            && self.gate_roles_never_let_local_estimate_read_as_provider_authoritative
            && self.ci_only_not_evaluated_here_and_provider_unavailable_never_hidden
            && self.advisory_and_enforced_owner_never_flattened
            && self.ai_review_never_runs_under_a_different_pack_version_without_disclosure
            && self.pack_version_digest_and_template_attribution_never_lost
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.copy_export_open_provider_preserve_one_payload
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackSharedConsumersProjection {
    /// The review-detail surface consumes the shared review-pack vocabulary.
    pub review_detail_consumes_shared_review_pack_vocabulary: bool,
    /// The merge-readiness component consumes the shared review-pack vocabulary.
    pub merge_readiness_consumes_shared_review_pack_vocabulary: bool,
    /// The AI-review panel consumes the shared review-pack vocabulary.
    pub ai_review_panel_consumes_shared_review_pack_vocabulary: bool,
    /// The provider-handoff surface consumes the shared review-pack vocabulary.
    pub provider_handoff_consumes_shared_review_pack_vocabulary: bool,
    /// The review-pack-summary surface consumes the shared review-pack vocabulary.
    pub review_pack_summary_consumes_shared_review_pack_vocabulary: bool,
    /// The ownership overlay consumes the shared review-pack vocabulary.
    pub ownership_overlay_consumes_shared_review_pack_vocabulary: bool,
    /// The local-CI-parity strip consumes the shared review-pack vocabulary.
    pub local_ci_parity_strip_consumes_shared_review_pack_vocabulary: bool,
    /// The support / export packet consumes the shared review-pack vocabulary.
    pub support_export_packet_consumes_shared_review_pack_vocabulary: bool,
    /// The help / docs surface consumes the shared review-pack vocabulary.
    pub help_docs_consumes_shared_review_pack_vocabulary: bool,
    /// Every object is adopted by two or more consumers.
    pub every_object_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same seeded subject.
    pub review_pack_vocabulary_identical_for_same_subject: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object back to one shared contract object.
    pub export_maps_back_to_one_review_pack_object: bool,
}

impl M5ReviewPackSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_detail_consumes_shared_review_pack_vocabulary
            && self.merge_readiness_consumes_shared_review_pack_vocabulary
            && self.ai_review_panel_consumes_shared_review_pack_vocabulary
            && self.provider_handoff_consumes_shared_review_pack_vocabulary
            && self.review_pack_summary_consumes_shared_review_pack_vocabulary
            && self.ownership_overlay_consumes_shared_review_pack_vocabulary
            && self.local_ci_parity_strip_consumes_shared_review_pack_vocabulary
            && self.support_export_packet_consumes_shared_review_pack_vocabulary
            && self.help_docs_consumes_shared_review_pack_vocabulary
            && self.every_object_adopted_by_two_or_more_consumers
            && self.review_pack_vocabulary_identical_for_same_subject
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_review_pack_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ReviewPackSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewPackSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ReviewPackSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ReviewPackSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ReviewPackConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ReviewPackSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewPackSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewPackSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review-pack shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewPackSharedConsumersPacket {
    /// Record kind; must equal [`M5_REVIEW_PACK_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ReviewPackSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ReviewPackSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ReviewPackConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ReviewPackSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewPackSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewPackSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReviewPackSharedConsumersPacket {
    /// Builds a review-pack shared-consumer packet from stable-lane input.
    pub fn new(input: M5ReviewPackSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_PACK_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the review-pack shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5ReviewPackSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REVIEW_PACK_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5ReviewPackSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5ReviewPackSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReviewPackSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ReviewPackSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ReviewPackSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ReviewPackSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ReviewPackSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ReviewPackSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review-pack shared-consumer packet serializes"),
        ) {
            violations.push(M5ReviewPackSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("review-pack shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("object,consumer,representation,review_pack_role_word,vocabulary_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.review_pack_role_word,
                binding.vocabulary_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Shared Review-Pack Consumers: One Vocabulary Across Surfaces\n\n");
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
                "- **{}** [`{}`]: object `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.subject_label,
                binding.binding_id,
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.review_pack_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review-pack shared-consumer export.
#[derive(Debug)]
pub enum M5ReviewPackSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReviewPackSharedConsumersViolation>),
}

impl fmt::Display for M5ReviewPackSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "review-pack shared-consumer export parse failed: {error}"
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
                    "review-pack shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReviewPackSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5ReviewPackSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReviewPackSharedConsumersViolation {
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
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's review-pack-role word is not a frozen role token.
    ReviewPackRoleWordOutsideVocabulary,
    /// A binding's gate-carrying role dropped its pack freshness.
    PackFreshnessMissingForGateRole,
    /// A binding's vocabulary state does not match its representation.
    VocabularyStateMismatch,
    /// Two surfaces show the same seeded subject with different vocabulary.
    ReviewPackVocabularyDriftAcrossSurfaces,
    /// A shared object is not adopted by at least two distinct consumers.
    ObjectReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding lets a local parity estimate masquerade as provider-authoritative.
    LetsALocalParityEstimateMasqueradeAsProviderAuthoritative,
    /// A binding hides a ci-only, not-evaluated-here, or provider-unavailable state behind a green summary.
    HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary,
    /// A binding flattens advisory-owner and enforced-owner into one owner pill.
    FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill,
    /// A binding lets AI review run under a different pack version without disclosure.
    LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure,
    /// A binding loses the review-pack version / digest or template attribution on export / publish / reopen.
    LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object appears among the bindings.
    ObjectCoverageMissing,
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

impl M5ReviewPackSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::ReviewPackRoleWordOutsideVocabulary => "review_pack_role_word_outside_vocabulary",
            Self::PackFreshnessMissingForGateRole => "pack_freshness_missing_for_gate_role",
            Self::VocabularyStateMismatch => "vocabulary_state_mismatch",
            Self::ReviewPackVocabularyDriftAcrossSurfaces => {
                "review_pack_vocabulary_drift_across_surfaces"
            }
            Self::ObjectReuseUnproven => "object_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => "narrow_note_preserved_vocabulary_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::LetsALocalParityEstimateMasqueradeAsProviderAuthoritative => {
                "lets_a_local_parity_estimate_masquerade_as_provider_authoritative"
            }
            Self::HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary => {
                "hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary"
            }
            Self::FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill => {
                "flattens_advisory_owner_and_enforced_owner_into_one_owner_pill"
            }
            Self::LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure => {
                "lets_ai_review_run_under_a_different_pack_version_without_disclosure"
            }
            Self::LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening => {
                "loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectCoverageMissing => "object_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable review-pack shared-consumer export.
pub fn current_stable_m5_review_pack_shared_consumers_export(
) -> Result<M5ReviewPackSharedConsumersPacket, M5ReviewPackSharedConsumersArtifactError> {
    let packet: M5ReviewPackSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5ReviewPackSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReviewPackSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ReviewPackSharedConsumersPacket,
    violations: &mut Vec<M5ReviewPackSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_REF,
        M5_REVIEW_PACK_SHARED_CONSUMERS_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
    ];
    // The six objects each map to their own canonical domain schema; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ReviewPackObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ReviewPackSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ReviewPackSharedConsumersPacket,
    violations: &mut Vec<M5ReviewPackSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5ReviewPackSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same seeded
    // subject.
    let mut subject_facets: BTreeMap<&str, &ReviewPackSharedStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<M5ReviewPackObject, BTreeSet<M5ReviewPackConsumerSurface>> =
        BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ReviewPackConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ReviewPackObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.subject_id.trim().is_empty()
            || binding.subject_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ReviewPackSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5ReviewPackSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.review_pack_role_word_in_vocabulary() {
            violations
                .push(M5ReviewPackSharedConsumersViolation::ReviewPackRoleWordOutsideVocabulary);
        }
        if !binding.state_facets.pack_freshness_satisfied() {
            violations.push(M5ReviewPackSharedConsumersViolation::PackFreshnessMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.vocabulary_state != disclosure.vocabulary_state {
            violations.push(M5ReviewPackSharedConsumersViolation::VocabularyStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5ReviewPackSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(M5ReviewPackSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5ReviewPackSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5ReviewPackSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5ReviewPackSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5ReviewPackSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5ReviewPackSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5ReviewPackSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.lets_a_local_parity_estimate_masquerade_as_provider_authoritative {
            violations.push(
                M5ReviewPackSharedConsumersViolation::LetsALocalParityEstimateMasqueradeAsProviderAuthoritative,
            );
        }
        if binding.hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary {
            violations.push(
                M5ReviewPackSharedConsumersViolation::HidesCiOnlyNotEvaluatedHereOrProviderUnavailableBehindAGreenSummary,
            );
        }
        if binding.flattens_advisory_owner_and_enforced_owner_into_one_owner_pill {
            violations.push(
                M5ReviewPackSharedConsumersViolation::FlattensAdvisoryOwnerAndEnforcedOwnerIntoOneOwnerPill,
            );
        }
        if binding.lets_ai_review_run_under_a_different_pack_version_without_disclosure {
            violations.push(
                M5ReviewPackSharedConsumersViolation::LetsAiReviewRunUnderADifferentPackVersionWithoutDisclosure,
            );
        }
        if binding
            .loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening
        {
            violations.push(
                M5ReviewPackSharedConsumersViolation::LosesReviewPackVersionDigestOrTemplateAttributionWhenExportingPublishingOrReopening,
            );
        }

        // Support / export consumers must map an object back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ReviewPackSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match subject_facets.get(binding.subject_id.as_str()) {
            None => {
                subject_facets.insert(binding.subject_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5ReviewPackSharedConsumersViolation::ReviewPackVocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object);
    }

    // Coverage: every consumer surface and every object must appear.
    for consumer in M5ReviewPackConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ReviewPackSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object in M5ReviewPackObject::ALL {
        if !seen_objects.contains(&object) {
            violations.push(M5ReviewPackSharedConsumersViolation::ObjectCoverageMissing);
            break;
        }
    }

    // Reuse: every present object must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ReviewPackSharedConsumersViolation::ObjectReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
