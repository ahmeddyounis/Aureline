//! Shared review-detail, AI-review-panel, finding-row, review-scope-selector, publish-to-review-sheet,
//! pending-review-tray, provider-publish-review, resolution-memory-ledger, and support / export consumers that
//! keep the B151 AI-review-assist objects — the AI review finding row, the review scope selector, the
//! publish-to-review sheet, and the resolution memory row — at **one canonical vocabulary** across every
//! claimed M5 AI-review and provider-backed review surface.
//!
//! This module is the consumer-adoption capstone for the four governed AI-review-assist objects frozen in
//! [`crate::m5_ai_review_assist_matrix`] and implemented by the finding / scope-source lane
//! ([`crate::m5_ai_review_finding_and_scope_source_registries`]), the scope-selector / rerun-state lane
//! ([`crate::m5_ai_review_scope_selector_and_rerun_state_registries`]), the publish-sheet / scope-decision lane
//! ([`crate::m5_ai_review_publish_sheet_and_scope_decision_registries`]), the resolution-memory /
//! finding-lifecycle lane
//! ([`crate::m5_ai_review_resolution_memory_and_finding_lifecycle_registries`]), and the publish-continuity /
//! compare-reconcile lane
//! ([`crate::m5_ai_review_publish_continuity_and_reconcile_registries`]).
//!
//! It binds each shared AI-review-assist object to the concrete review-list / detail, pending-review-tray,
//! AI-evidence / history, support / export, and help / docs consumers — projected here through the
//! review-detail, AI-review-panel, finding-row, review-scope-selector, publish-to-review-sheet,
//! pending-review-tray, provider-publish-review, resolution-memory-ledger, and support-export surfaces — that
//! render it, and proves — by fixtures, not screenshots — that the same seeded finding presents the same
//! AI-review-role, object, registry-reference, publish-state, surface-context, and finding-lifecycle vocabulary
//! wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the four shared AI-review-assist objects must be adopted by at least two distinct
//!    consumers, so an object is proven to be shared AI-review infrastructure rather than a one-surface,
//!    feature-local fork of the finding-row, scope-selector, publish-sheet, or resolution-memory contract.
//! 2. **One vocabulary / no drift.** For a given seeded finding every consumer surface must present identical
//!    [`AiReviewStateFacetValues`] — the same AI-review-role word, the same object word, the same
//!    registry-reference word, the same publish-state word, the same surface-context word, and the same
//!    finding-lifecycle word. The AI-review-role word must be a token from the frozen
//!    [`M5AiReviewAssistRole`] vocabulary, so no surface rewrites `finding_classification`,
//!    `analyzed_scope_disclosure`, `publish_destination_disclosure`, `local_versus_provider_state`,
//!    `lifecycle_state_tracking`, `publish_export_fallback`, or `resolution_memory_disclosure` in its own
//!    words. A surface may narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying vocabulary per surface, and a role that carries
//!    finding-classification, analyzed-scope, publish-destination, or local-versus-provider meaning may never
//!    let AI review results publish or merge implicitly, hide whether output stays local or becomes a provider
//!    comment / suggested patch / check annotation, keep a stale finding looking current after diff or
//!    instruction drift, or lose a local draft when a publish fails.
//! 3. **Map back to one object.** Support / export consumers must point at the canonical per-domain schema and
//!    the frozen matrix by id, so an exported packet — and every copy / export / open-in-provider action — can
//!    always map a review-detail / AI-history / support surface back to one shared contract object rather than
//!    diverging into a surface-local payload.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`AiReviewNarrowNote`] naming the reason, the preserved vocabulary, and the next action, and an exported
//! representation additionally names its export-safe detail boundary rather than collapsing the finding out of
//! view.
//!
//! The packet references upstream AI-review-assist contracts by id rather than embedding their content. Raw
//! secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/m5-ai-review-shared-consumers.schema.json`](../../../../schemas/review/m5-ai-review-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/review/m5_ai_review_shared_consumers_one_vocabulary.md`](../../../../docs/review/m5_ai_review_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/review/m5-ai-review-shared-consumers/`](../../../../fixtures/review/m5-ai-review-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_ai_review_shared_consumers,
    seeded_m5_ai_review_shared_consumers_compact_remote_narrowed,
    seeded_m5_ai_review_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_ai_review_assist_matrix::{
    M5AiReviewAssistConsumerSurface, M5AiReviewAssistObject, M5AiReviewAssistRole,
    M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF, M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5AiReviewSharedConsumersPacket`].
pub const M5_AI_REVIEW_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_ai_review_shared_consumer_registry_parity";

/// Schema version for AI-review shared-consumer parity records.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-ai-review-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/review/m5_ai_review_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/review/m5-ai-review-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/review/m5-ai-review-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/review/m5-ai-review-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/review/m5-ai-review-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_AI_REVIEW_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Finding-lifecycle sentinel words a finding-classification / analyzed-scope / publish-destination /
/// local-versus-provider gate role may never fall back to; a gate-carrying role that changes surface
/// presentation must always keep a real finding-current-scope-bound-and-destination-disclosed continuity,
/// never showing an outdated finding as current, a suppressed finding as active, losing a local draft when a
/// publish fails, or hiding the provider destination.
const FINDING_LIFECYCLE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "outdated_finding_shown_as_current",
    "suppressed_finding_shown_as_active",
    "local_draft_lost_when_publish_failed",
    "provider_destination_hidden",
];

/// Whether a consumer surface is an export / support path that must map an object back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5AiReviewAssistConsumerSurface) -> bool {
    matches!(
        consumer,
        M5AiReviewAssistConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5AiReviewAssistRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a seeded finding's AI-review-role word must be a controlled role token
/// rather than a per-surface synonym.
pub fn is_known_ai_review_role_token(token: &str) -> bool {
    ai_review_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5AiReviewAssistRole`], if it is one.
pub fn ai_review_role_from_token(token: &str) -> Option<M5AiReviewAssistRole> {
    M5AiReviewAssistRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared AI-review-assist object a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed representation still
/// carries the same AI-review-role, object, registry-reference, publish-state, surface-context, and
/// finding-lifecycle words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl AiReviewRepresentation {
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

/// A vocabulary axis whose word must stay identical across surfaces for one finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewParityFacet {
    /// The frozen AI-review-role word.
    AiReviewRoleWord,
    /// The AI-review-object word.
    ObjectWord,
    /// The canonical registry-reference word the object points at.
    RegistryReferenceWord,
    /// The publish-state word (local draft / publish now provider comment / suggested patch / check
    /// annotation / open in provider / export fallback offline) the finding ships.
    PublishStateWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The finding-lifecycle word paired with a finding-classification / analyzed-scope / publish-destination /
    /// local-versus-provider gate role.
    FindingLifecycleWord,
}

impl AiReviewParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AiReviewRoleWord,
        Self::ObjectWord,
        Self::RegistryReferenceWord,
        Self::PublishStateWord,
        Self::SurfaceContextWord,
        Self::FindingLifecycleWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiReviewRoleWord => "ai_review_role_word",
            Self::ObjectWord => "object_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::PublishStateWord => "publish_state_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::FindingLifecycleWord => "finding_lifecycle_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared AI-review-assist object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl AiReviewNarrowReason {
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
pub enum AiReviewNarrowNextAction {
    /// Expand the object in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl AiReviewNarrowNextAction {
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
pub enum AiReviewParityState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl AiReviewParityState {
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
pub enum AiReviewSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// AI-review vocabulary drifted between surfaces for the same finding.
    AiReviewVocabularyDriftDetected,
    /// A gate-carrying role dropped its finding-lifecycle or provider-destination disclosure.
    FindingLifecycleOrDestinationDisclosureDropped,
    /// A surface let AI review results publish or merge implicitly.
    LetsAiReviewResultsPublishOrMergeImplicitly,
    /// A surface hid whether output stays local or becomes a provider comment / suggested patch / check
    /// annotation.
    HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation,
    /// A surface kept a stale finding looking current after diff or instruction drift.
    KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift,
    /// A surface lost local drafts or evidence when provider write scope was missing or a publish failed.
    LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails,
    /// A surface presented an AI review finding without its analyzed scope, publish destination, or lifecycle
    /// state.
    PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared AI-review-assist object narrowed.
    UpstreamAiReviewNarrowed,
}

impl AiReviewSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::AiReviewVocabularyDriftDetected,
        Self::FindingLifecycleOrDestinationDisclosureDropped,
        Self::LetsAiReviewResultsPublishOrMergeImplicitly,
        Self::HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation,
        Self::KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift,
        Self::LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails,
        Self::PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamAiReviewNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::AiReviewVocabularyDriftDetected => "ai_review_vocabulary_drift_detected",
            Self::FindingLifecycleOrDestinationDisclosureDropped => {
                "finding_lifecycle_or_destination_disclosure_dropped"
            }
            Self::LetsAiReviewResultsPublishOrMergeImplicitly => {
                "lets_ai_review_results_publish_or_merge_implicitly"
            }
            Self::HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation => {
                "hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation"
            }
            Self::KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift => {
                "keeps_stale_findings_looking_current_after_diff_or_instruction_drift"
            }
            Self::LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails => {
                "loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails"
            }
            Self::PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState => {
                "presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamAiReviewNarrowed => "upstream_ai_review_narrowed",
        }
    }
}

/// The controlled vocabulary a seeded finding presents.
///
/// These six words must be identical across every consumer surface that shows the same seeded finding. The
/// AI-review-role word must be a frozen role token; the rest are controlled words the finding's object carries.
/// A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewStateFacetValues {
    /// AI-review-role word (must be a frozen [`M5AiReviewAssistRole`] token).
    pub ai_review_role_word: String,
    /// AI-review-object word.
    pub object_word: String,
    /// Canonical registry-reference word the object points at.
    pub registry_reference_word: String,
    /// Publish-state word (local draft / publish now provider comment / suggested patch / check annotation /
    /// open in provider / export fallback offline) the finding ships.
    pub publish_state_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Finding-lifecycle word paired with a finding-classification / analyzed-scope / publish-destination /
    /// local-versus-provider gate role.
    pub finding_lifecycle_word: String,
}

impl AiReviewStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.ai_review_role_word.trim().is_empty()
            && !self.object_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.publish_state_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.finding_lifecycle_word.trim().is_empty()
    }

    /// Whether the AI-review-role word is a member of the frozen role vocabulary.
    pub fn ai_review_role_word_in_vocabulary(&self) -> bool {
        is_known_ai_review_role_token(self.ai_review_role_word.trim())
    }

    /// Whether the finding honours the lifecycle rule: a role that carries finding-classification,
    /// analyzed-scope, publish-destination, or local-versus-provider meaning must pair its surface change with
    /// a real finding-current-scope-bound-and-destination-disclosed continuity and never collapse to an
    /// outdated-finding-shown-as-current, suppressed-finding-shown-as-active,
    /// local-draft-lost-when-publish-failed, or provider-destination-hidden sentinel.
    pub fn finding_lifecycle_satisfied(&self) -> bool {
        match ai_review_role_from_token(self.ai_review_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_ai_review_finding() => {
                let lifecycle = self.finding_lifecycle_word.trim().to_lowercase();
                !lifecycle.is_empty()
                    && !FINDING_LIFECYCLE_ABSENT_SENTINELS.contains(&lifecycle.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewNarrowNote {
    /// Why the representation narrowed.
    pub reason: AiReviewNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: AiReviewNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiReviewRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: AiReviewParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<AiReviewNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<AiReviewNarrowNextAction>,
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
pub const fn resolve_ai_review_render_disclosure(
    representation: AiReviewRepresentation,
) -> AiReviewRenderDisclosure {
    match representation {
        AiReviewRepresentation::DesktopFull => AiReviewRenderDisclosure {
            parity_state: AiReviewParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        AiReviewRepresentation::CompactNarrowed => AiReviewRenderDisclosure {
            parity_state: AiReviewParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(AiReviewNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(AiReviewNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        AiReviewRepresentation::RemoteProjected => AiReviewRenderDisclosure {
            parity_state: AiReviewParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(AiReviewNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(AiReviewNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        AiReviewRepresentation::ExportedRedacted => AiReviewRenderDisclosure {
            parity_state: AiReviewParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(AiReviewNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(AiReviewNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared AI-review-assist object rendered on one consumer surface in one
/// representation for one seeded finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable finding id (shared across surfaces that show the same finding).
    pub finding_id: String,
    /// Human-readable finding identity.
    pub finding_label: String,
    /// Which shared AI-review-assist object this binding renders.
    pub object: M5AiReviewAssistObject,
    /// Which consumer surface renders it.
    pub consumer: M5AiReviewAssistConsumerSurface,
    /// Which representation this surface renders.
    pub representation: AiReviewRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one finding).
    pub state_facets: AiReviewStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: AiReviewParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<AiReviewNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets AI review results publish or merge implicitly. MUST be `false`.
    pub lets_ai_review_results_publish_or_merge_implicitly: bool,
    /// Guardrail: this surface hides whether output stays local or becomes a provider comment / suggested
    /// patch / check annotation. MUST be `false`.
    pub hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation:
        bool,
    /// Guardrail: this surface keeps stale findings looking current after diff or instruction drift. MUST be
    /// `false`.
    pub keeps_stale_findings_looking_current_after_diff_or_instruction_drift: bool,
    /// Guardrail: this surface loses local drafts or evidence when provider write scope is missing or a
    /// publish fails. MUST be `false`.
    pub loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails: bool,
    /// Guardrail: this surface presents an AI review finding without its analyzed scope, publish destination,
    /// or lifecycle state. MUST be `false`.
    pub presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state:
        bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl AiReviewConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> AiReviewRenderDisclosure {
        resolve_ai_review_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.lets_ai_review_results_publish_or_merge_implicitly
            && !self.hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation
            && !self.keeps_stale_findings_looking_current_after_diff_or_instruction_drift
            && !self.loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails
            && !self
                .presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state
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
                .any(|reference| reference == M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewSharedConsumersTrustReview {
    /// Object reuse is proven by fixtures rather than inferred from screenshots.
    pub object_reuse_proven_by_fixtures: bool,
    /// The same seeded finding presents the same vocabulary across surfaces.
    pub same_finding_same_ai_review_vocabulary_across_surfaces: bool,
    /// Every AI-review-role word is a frozen role token.
    pub ai_review_role_words_stay_in_frozen_vocabulary: bool,
    /// Gate-carrying roles never publish or merge implicitly.
    pub gate_roles_never_publish_or_merge_implicitly: bool,
    /// A surface never hides whether output stays local or becomes a provider comment / patch / annotation.
    pub output_destination_class_never_hidden: bool,
    /// A surface never keeps a stale finding looking current after diff or instruction drift.
    pub stale_findings_never_shown_as_current: bool,
    /// A surface never loses local drafts or evidence when provider write scope is missing or a publish fails.
    pub local_drafts_and_evidence_never_lost: bool,
    /// A surface never presents a finding without its analyzed scope, publish destination, or lifecycle state.
    pub finding_never_shown_without_scope_destination_and_lifecycle: bool,
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

impl M5AiReviewSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_reuse_proven_by_fixtures
            && self.same_finding_same_ai_review_vocabulary_across_surfaces
            && self.ai_review_role_words_stay_in_frozen_vocabulary
            && self.gate_roles_never_publish_or_merge_implicitly
            && self.output_destination_class_never_hidden
            && self.stale_findings_never_shown_as_current
            && self.local_drafts_and_evidence_never_lost
            && self.finding_never_shown_without_scope_destination_and_lifecycle
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.copy_export_open_provider_preserve_one_payload
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewSharedConsumersProjection {
    /// The review-detail surface consumes the shared AI-review vocabulary.
    pub review_detail_consumes_shared_ai_review_vocabulary: bool,
    /// The AI-review panel consumes the shared AI-review vocabulary.
    pub ai_review_panel_consumes_shared_ai_review_vocabulary: bool,
    /// The finding row consumes the shared AI-review vocabulary.
    pub finding_row_consumes_shared_ai_review_vocabulary: bool,
    /// The review scope selector consumes the shared AI-review vocabulary.
    pub review_scope_selector_consumes_shared_ai_review_vocabulary: bool,
    /// The publish-to-review sheet consumes the shared AI-review vocabulary.
    pub publish_to_review_sheet_consumes_shared_ai_review_vocabulary: bool,
    /// The pending-review tray consumes the shared AI-review vocabulary.
    pub pending_review_tray_consumes_shared_ai_review_vocabulary: bool,
    /// The provider-publish-review surface consumes the shared AI-review vocabulary.
    pub provider_publish_review_consumes_shared_ai_review_vocabulary: bool,
    /// The resolution-memory ledger consumes the shared AI-review vocabulary.
    pub resolution_memory_ledger_consumes_shared_ai_review_vocabulary: bool,
    /// The support / export packet consumes the shared AI-review vocabulary.
    pub support_export_packet_consumes_shared_ai_review_vocabulary: bool,
    /// Every object is adopted by two or more consumers.
    pub every_object_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same seeded finding.
    pub ai_review_vocabulary_identical_for_same_finding: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object back to one shared contract object.
    pub export_maps_back_to_one_ai_review_object: bool,
}

impl M5AiReviewSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_detail_consumes_shared_ai_review_vocabulary
            && self.ai_review_panel_consumes_shared_ai_review_vocabulary
            && self.finding_row_consumes_shared_ai_review_vocabulary
            && self.review_scope_selector_consumes_shared_ai_review_vocabulary
            && self.publish_to_review_sheet_consumes_shared_ai_review_vocabulary
            && self.pending_review_tray_consumes_shared_ai_review_vocabulary
            && self.provider_publish_review_consumes_shared_ai_review_vocabulary
            && self.resolution_memory_ledger_consumes_shared_ai_review_vocabulary
            && self.support_export_packet_consumes_shared_ai_review_vocabulary
            && self.every_object_adopted_by_two_or_more_consumers
            && self.ai_review_vocabulary_identical_for_same_finding
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_ai_review_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5AiReviewSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiReviewSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<AiReviewConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<AiReviewSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5AiReviewAssistConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5AiReviewSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiReviewSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiReviewSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI-review shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewSharedConsumersPacket {
    /// Record kind; must equal [`M5_AI_REVIEW_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<AiReviewConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<AiReviewSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5AiReviewAssistConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5AiReviewSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiReviewSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiReviewSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiReviewSharedConsumersPacket {
    /// Builds an AI-review shared-consumer packet from stable-lane input.
    pub fn new(input: M5AiReviewSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_AI_REVIEW_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the AI-review shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5AiReviewSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_REVIEW_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5AiReviewSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5AiReviewSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiReviewSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5AiReviewSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5AiReviewSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5AiReviewSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5AiReviewSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5AiReviewSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("AI-review shared-consumer packet serializes"),
        ) {
            violations.push(M5AiReviewSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("AI-review shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("object,consumer,representation,ai_review_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.ai_review_role_word,
                binding.parity_state.as_str(),
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
        out.push_str("# Shared AI-Review Consumers: One Vocabulary Across Surfaces\n\n");
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
                binding.finding_label,
                binding.binding_id,
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.ai_review_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in AI-review shared-consumer export.
#[derive(Debug)]
pub enum M5AiReviewSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiReviewSharedConsumersViolation>),
}

impl fmt::Display for M5AiReviewSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "AI-review shared-consumer export parse failed: {error}"
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
                    "AI-review shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiReviewSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5AiReviewSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiReviewSharedConsumersViolation {
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
    /// A binding's AI-review-role word is not a frozen role token.
    AiReviewRoleWordOutsideVocabulary,
    /// A binding's gate-carrying role dropped its finding lifecycle.
    FindingLifecycleMissingForGateRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same seeded finding with different vocabulary.
    AiReviewVocabularyDriftAcrossSurfaces,
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
    /// A binding lets AI review results publish or merge implicitly.
    LetsAiReviewResultsPublishOrMergeImplicitly,
    /// A binding hides whether output stays local or becomes a provider comment / patch / annotation.
    HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation,
    /// A binding keeps stale findings looking current after diff or instruction drift.
    KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift,
    /// A binding loses local drafts or evidence when provider write scope is missing or a publish fails.
    LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails,
    /// A binding presents a finding without its analyzed scope, publish destination, or lifecycle state.
    PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState,
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

impl M5AiReviewSharedConsumersViolation {
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
            Self::AiReviewRoleWordOutsideVocabulary => "ai_review_role_word_outside_vocabulary",
            Self::FindingLifecycleMissingForGateRole => "finding_lifecycle_missing_for_gate_role",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::AiReviewVocabularyDriftAcrossSurfaces => {
                "ai_review_vocabulary_drift_across_surfaces"
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
            Self::LetsAiReviewResultsPublishOrMergeImplicitly => {
                "lets_ai_review_results_publish_or_merge_implicitly"
            }
            Self::HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation => {
                "hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation"
            }
            Self::KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift => {
                "keeps_stale_findings_looking_current_after_diff_or_instruction_drift"
            }
            Self::LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails => {
                "loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails"
            }
            Self::PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState => {
                "presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state"
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

/// Reads and validates the checked-in stable AI-review shared-consumer export.
pub fn current_stable_m5_ai_review_shared_consumers_export(
) -> Result<M5AiReviewSharedConsumersPacket, M5AiReviewSharedConsumersArtifactError> {
    let packet: M5AiReviewSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-ai-review-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5AiReviewSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiReviewSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiReviewSharedConsumersPacket,
    violations: &mut Vec<M5AiReviewSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_REF,
        M5_AI_REVIEW_SHARED_CONSUMERS_DOC_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
    ];
    // The four objects each map to their own canonical domain schema; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5AiReviewAssistObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5AiReviewSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5AiReviewSharedConsumersPacket,
    violations: &mut Vec<M5AiReviewSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5AiReviewSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same seeded
    // finding.
    let mut finding_facets: BTreeMap<&str, &AiReviewStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5AiReviewAssistObject,
        BTreeSet<M5AiReviewAssistConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5AiReviewAssistConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5AiReviewAssistObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.finding_id.trim().is_empty()
            || binding.finding_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5AiReviewSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5AiReviewSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.ai_review_role_word_in_vocabulary() {
            violations.push(M5AiReviewSharedConsumersViolation::AiReviewRoleWordOutsideVocabulary);
        }
        if !binding.state_facets.finding_lifecycle_satisfied() {
            violations.push(M5AiReviewSharedConsumersViolation::FindingLifecycleMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5AiReviewSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5AiReviewSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(M5AiReviewSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5AiReviewSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5AiReviewSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations
                            .push(M5AiReviewSharedConsumersViolation::NarrowNextActionLabelMissing);
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5AiReviewSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5AiReviewSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5AiReviewSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.lets_ai_review_results_publish_or_merge_implicitly {
            violations.push(
                M5AiReviewSharedConsumersViolation::LetsAiReviewResultsPublishOrMergeImplicitly,
            );
        }
        if binding
            .hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation
        {
            violations.push(
                M5AiReviewSharedConsumersViolation::HidesWhetherOutputStaysLocalOrBecomesAProviderCommentSuggestedPatchOrCheckAnnotation,
            );
        }
        if binding.keeps_stale_findings_looking_current_after_diff_or_instruction_drift {
            violations.push(
                M5AiReviewSharedConsumersViolation::KeepsStaleFindingsLookingCurrentAfterDiffOrInstructionDrift,
            );
        }
        if binding
            .loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails
        {
            violations.push(
                M5AiReviewSharedConsumersViolation::LosesLocalDraftsOrEvidenceWhenProviderWriteScopeIsMissingOrPublishFails,
            );
        }
        if binding
            .presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state
        {
            violations.push(
                M5AiReviewSharedConsumersViolation::PresentsAnAiReviewFindingWithoutItsAnalyzedScopePublishDestinationOrLifecycleState,
            );
        }

        // Support / export consumers must map an object back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5AiReviewSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match finding_facets.get(binding.finding_id.as_str()) {
            None => {
                finding_facets.insert(binding.finding_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5AiReviewSharedConsumersViolation::AiReviewVocabularyDriftAcrossSurfaces,
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
    for consumer in M5AiReviewAssistConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5AiReviewSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object in M5AiReviewAssistObject::ALL {
        if !seen_objects.contains(&object) {
            violations.push(M5AiReviewSharedConsumersViolation::ObjectCoverageMissing);
            break;
        }
    }

    // Reuse: every present object must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5AiReviewSharedConsumersViolation::ObjectReuseUnproven);
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
