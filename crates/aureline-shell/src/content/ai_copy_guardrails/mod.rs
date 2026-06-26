//! AI copy guardrails, uncertainty/confidence vocabulary, and prohibited
//! high-trust phrasing checks.
//!
//! This module materializes the canonical, export-safe catalog of the controlled
//! AI wording the shell renders on trust-sensitive assistant and review surfaces,
//! plus the rejection list that blocks high-trust overclaiming. Where the
//! error/recovery copy catalog locks the *shape* of a failure explanation, this
//! catalog locks the *trust posture* of AI wording: a proposal is always
//! [`AiTaxonomyConcept::Suggested`], [`AiTaxonomyConcept::Proposed`], or
//! [`AiTaxonomyConcept::Draft`] until it is accepted; a confidence claim names its
//! evidence and never overstates certainty; a validation claim is tied to a named
//! validation state; review is named when review is required; and reversibility is
//! disclosed so an AI surface can never imply review-free completion.
//!
//! Two registers compose here. [`AiCopyTerm`] objects are the *approved* taxonomy —
//! one reserved meaning, one locale-neutral machine token, the protected
//! [`AiCopySurface`]s they govern, and the [`AiCopyConsumer`]s that must be able to
//! reconstruct them. [`ForbiddenPhrase`] objects are the *rejection* register — each
//! a lowercase pattern in a [`ForbiddenPhraseClass`] (perfection/guarantee,
//! review-free mutation, false autonomy, false validation, confidence
//! overstatement, false exhaustiveness, false freshness) with the approved terms
//! that replace it. [`AiCopyGuardrailCatalog::lint`] scans candidate copy for a
//! surface and rejects any forbidden phrase, so `Guaranteed`, `Perfect`,
//! `Done for you`, and `No review needed` can never reach a protected AI surface.
//!
//! The catalog eats its own dog food: every approved term's canonical label and
//! reserved meaning must itself pass the lint on every surface it governs, so the
//! approved vocabulary can never quietly smuggle in an overclaim. Machine-facing
//! identity stays locale-neutral — term ids, machine tokens, and phrase ids are
//! lowercase ascii (`[a-z0-9_.]`) — while human prose localizes safely around it, so
//! a localized overlay can never fork the meaning of a proposal, a confidence
//! label, or a validation state, and a support export, review handoff, narrated
//! announcement, or release/demo caption can reconstruct the exact wording the user
//! saw in-product.
//!
//! AI wording stays distinct from deterministic language-service or
//! formatter/refactor wording where that difference matters for trust: every AI
//! term is [provisional](AiCopyTerm::ai_provisional), and low-confidence and
//! review-required terms suppress direct mutation controls.
//!
//! The boundary schema is
//! [`schemas/content/m5-ai-copy-guardrails.schema.json`](../../../../../schemas/content/m5-ai-copy-guardrails.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_ai_copy_guardrails.md`](../../../../../docs/content/m5/m5_ai_copy_guardrails.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-ai-copy-guardrails/`](../../../../../fixtures/content/m5-ai-copy-guardrails/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_ai_copy_guardrail_catalog, seeded_ai_copy_guardrail_catalog_localized,
    seeded_ai_copy_guardrail_catalog_offline_mirror, AI_COPY_GUARDRAIL_CATALOG_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiCopyGuardrailCatalog`].
pub const AI_COPY_GUARDRAIL_CATALOG_RECORD_KIND: &str = "m5_ai_copy_guardrail_catalog";

/// Schema version for AI copy guardrail catalog records.
pub const AI_COPY_GUARDRAIL_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct reuse consumers a shared term must span.
pub const SHARED_TERM_MIN_REUSE_CONSUMERS: usize = 3;

/// Repo-relative path of the boundary schema.
pub const AI_COPY_GUARDRAIL_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-ai-copy-guardrails.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const AI_COPY_GUARDRAIL_CATALOG_DOC_REF: &str = "docs/content/m5/m5_ai_copy_guardrails.md";

/// Repo-relative path of the product-wide AI copy guardrails contract this catalog
/// materializes.
pub const AI_COPY_GUARDRAILS_CONTRACT_REF: &str = "docs/ai/ai_copy_guardrails_contract.md";

/// Repo-relative path of the approved AI term register the catalog projects.
pub const AI_COPY_APPROVED_TERMS_REF: &str = "artifacts/ai/approved_ai_terms.yaml";

/// Repo-relative path of the forbidden AI term register the lint register projects.
pub const AI_COPY_FORBIDDEN_TERMS_REF: &str = "artifacts/ai/forbidden_ai_terms.yaml";

/// Repo-relative path of the controlled glossary; machine tokens must align with the
/// controlled state and label vocabulary owned there.
pub const AI_COPY_CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the frozen UI copy contract (action labels, error copy).
pub const AI_COPY_UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const AI_COPY_GUARDRAIL_CATALOG_FIXTURE_DIR: &str = "fixtures/content/m5-ai-copy-guardrails";

/// Repo-relative path of the checked support-export artifact.
pub const AI_COPY_GUARDRAIL_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-ai-copy-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AI_COPY_GUARDRAIL_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-ai-copy-proof/m5_ai_copy_guardrails.md";

/// The high-trust phrases the lane is explicitly required to reject. Every one of
/// these MUST appear as a forbidden-phrase pattern.
pub const REQUIRED_FORBIDDEN_PATTERNS: [&str; 4] =
    ["guaranteed", "perfect", "done for you", "no review needed"];

/// The trust domain a controlled AI term governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiCopyDomain {
    /// Proposal state of a model output (Suggested, Proposed, Draft).
    ProposalState,
    /// Qualitative confidence of an AI claim.
    Confidence,
    /// Validation state tied to a named validation record.
    Validation,
    /// Context-coverage disclosure for an AI answer.
    ContextDisclosure,
    /// Review posture before a mutation or publication.
    ReviewPosture,
    /// Reversibility of an applied or proposed change.
    Reversibility,
}

impl AiCopyDomain {
    /// Every domain, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProposalState,
        Self::Confidence,
        Self::Validation,
        Self::ContextDisclosure,
        Self::ReviewPosture,
        Self::Reversibility,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProposalState => "proposal_state",
            Self::Confidence => "confidence",
            Self::Validation => "validation",
            Self::ContextDisclosure => "context_disclosure",
            Self::ReviewPosture => "review_posture",
            Self::Reversibility => "reversibility",
        }
    }
}

/// A required taxonomy concept the catalog must materialize at least once.
///
/// These are exactly the controlled wordings the lane is required to implement:
/// `Suggested`, `Proposed`, `Draft`, `Context used`, `Validation`, `Low confidence`,
/// `Review required`, and `Revert/Undo availability`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTaxonomyConcept {
    /// Model-proposed guidance that has not been accepted, applied, or validated.
    Suggested,
    /// A concrete change put forward for review but not yet accepted.
    Proposed,
    /// Generated content or a patch held outside canonical source truth.
    Draft,
    /// Disclosure of the context segments an answer actually used.
    ContextUsed,
    /// A validation state tied to a named validation record.
    Validation,
    /// Confidence below the surface floor because evidence is limited.
    LowConfidence,
    /// Human, policy, ownership, or write-scope review remains required.
    ReviewRequired,
    /// A prior, known-good state can be restored (revert / undo).
    RevertUndoAvailable,
}

impl AiTaxonomyConcept {
    /// Every required concept, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Suggested,
        Self::Proposed,
        Self::Draft,
        Self::ContextUsed,
        Self::Validation,
        Self::LowConfidence,
        Self::ReviewRequired,
        Self::RevertUndoAvailable,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suggested => "suggested",
            Self::Proposed => "proposed",
            Self::Draft => "draft",
            Self::ContextUsed => "context_used",
            Self::Validation => "validation",
            Self::LowConfidence => "low_confidence",
            Self::ReviewRequired => "review_required",
            Self::RevertUndoAvailable => "revert_undo_available",
        }
    }

    /// The trust domain this concept belongs to.
    pub const fn domain(self) -> AiCopyDomain {
        match self {
            Self::Suggested | Self::Proposed | Self::Draft => AiCopyDomain::ProposalState,
            Self::ContextUsed => AiCopyDomain::ContextDisclosure,
            Self::Validation => AiCopyDomain::Validation,
            Self::LowConfidence => AiCopyDomain::Confidence,
            Self::ReviewRequired => AiCopyDomain::ReviewPosture,
            Self::RevertUndoAvailable => AiCopyDomain::Reversibility,
        }
    }

    /// True when a term for this concept must suppress direct mutation controls:
    /// a low-confidence or review-required proposal can never offer direct apply.
    pub const fn suppresses_direct_mutation(self) -> bool {
        matches!(self, Self::LowConfidence | Self::ReviewRequired)
    }
}

/// A protected AI / review surface a guardrail governs.
///
/// These are the surfaces the goal names: prompt composer, patch review, notebook
/// help, docs/help, and provider/account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiCopySurface {
    /// The AI prompt composer.
    PromptComposer,
    /// The AI patch / change review surface.
    PatchReview,
    /// Notebook AI help and inline assist.
    NotebookHelp,
    /// AI explanations inside docs / help.
    DocsHelp,
    /// Provider / account / route disclosure surfaces.
    ProviderAccount,
}

impl AiCopySurface {
    /// Every protected surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PromptComposer,
        Self::PatchReview,
        Self::NotebookHelp,
        Self::DocsHelp,
        Self::ProviderAccount,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptComposer => "prompt_composer",
            Self::PatchReview => "patch_review",
            Self::NotebookHelp => "notebook_help",
            Self::DocsHelp => "docs_help",
            Self::ProviderAccount => "provider_account",
        }
    }
}

/// A consumer that must be able to reconstruct the same AI wording.
///
/// These are the reuse surfaces the goal names: UI, docs/help, support exports,
/// narrated announcements, and release/demo artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiCopyConsumer {
    /// The in-product UI.
    ProductUi,
    /// Documentation and help content.
    DocsHelp,
    /// A support / export packet.
    SupportExport,
    /// A screen-reader / narrated announcement.
    NarratedAnnouncement,
    /// A release-note or demo / screenshot artifact.
    ReleaseDemo,
}

impl AiCopyConsumer {
    /// Every reuse consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProductUi,
        Self::DocsHelp,
        Self::SupportExport,
        Self::NarratedAnnouncement,
        Self::ReleaseDemo,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::NarratedAnnouncement => "narrated_announcement",
            Self::ReleaseDemo => "release_demo",
        }
    }
}

/// The risk class a forbidden high-trust phrase belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForbiddenPhraseClass {
    /// Implies perfection, guaranteed success, or absence of risk.
    PerfectionGuarantee,
    /// Implies a mutation applies without review or approval.
    ReviewFreeMutation,
    /// Implies the assistant acted autonomously and finished the work.
    FalseAutonomy,
    /// Implies validation passed without a named validation record.
    FalseValidation,
    /// Overstates certainty about AI-inferred output.
    ConfidenceOverstatement,
    /// Implies exhaustive context or scope the scope record does not prove.
    FalseExhaustiveness,
    /// Implies the result is current when freshness is stale or unknown.
    FalseFreshness,
}

impl ForbiddenPhraseClass {
    /// Every forbidden class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PerfectionGuarantee,
        Self::ReviewFreeMutation,
        Self::FalseAutonomy,
        Self::FalseValidation,
        Self::ConfidenceOverstatement,
        Self::FalseExhaustiveness,
        Self::FalseFreshness,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerfectionGuarantee => "perfection_guarantee",
            Self::ReviewFreeMutation => "review_free_mutation",
            Self::FalseAutonomy => "false_autonomy",
            Self::FalseValidation => "false_validation",
            Self::ConfidenceOverstatement => "confidence_overstatement",
            Self::FalseExhaustiveness => "false_exhaustiveness",
            Self::FalseFreshness => "false_freshness",
        }
    }
}

/// One controlled AI wording object: one reserved meaning, one machine token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyTerm {
    /// Stable, locale-neutral term id (e.g. `term.proposal.suggested`).
    pub term_id: String,
    /// The required taxonomy concept this term materializes.
    pub concept: AiTaxonomyConcept,
    /// The trust domain (mirrors `concept.domain()`).
    pub domain: AiCopyDomain,
    /// Canonical (default-locale) display label (e.g. `Suggested`).
    pub canonical_label: String,
    /// Locale-neutral machine token (flag/code/JSON key).
    pub machine_token: String,
    /// The single reserved meaning this term holds everywhere.
    pub reserved_meaning: String,
    /// Context the surface MUST also state when it uses this term.
    pub required_context: Vec<String>,
    /// Uses that are explicitly disallowed for this term.
    pub forbidden_uses: Vec<String>,
    /// Protected surfaces this term governs.
    pub surfaces: Vec<AiCopySurface>,
    /// Reuse consumers that must reconstruct this term.
    pub consumers: Vec<AiCopyConsumer>,
    /// True when this is provisional AI wording, never a deterministic guarantee.
    /// Must be `true`; this is what keeps AI copy distinct from deterministic
    /// language-service or formatter/refactor wording.
    pub ai_provisional: bool,
    /// True when the term must suppress direct mutation controls (low confidence,
    /// review required). Mirrors `concept.suppresses_direct_mutation()`.
    pub suppresses_direct_mutation: bool,
    /// True when the term requires a named evidence, validation, or context basis.
    pub requires_evidence_basis: bool,
}

/// One forbidden high-trust phrase and the approved wording that replaces it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForbiddenPhrase {
    /// Stable, locale-neutral phrase id (e.g. `forbidden.perfection.guaranteed`).
    pub phrase_id: String,
    /// The risk class this phrase belongs to.
    pub class: ForbiddenPhraseClass,
    /// The lowercase phrase pattern the lint matches against candidate copy.
    pub pattern: String,
    /// Why the phrase is rejected on a protected surface.
    pub rejection_reason: String,
    /// Approved term ids that should replace the phrase; each must resolve.
    pub approved_replacement_term_ids: Vec<String>,
    /// Protected surfaces this phrase is forbidden on.
    pub forbidden_on: Vec<AiCopySurface>,
}

/// One lint finding: a forbidden phrase matched candidate copy on a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyLintFinding {
    /// The forbidden-phrase id that matched.
    pub phrase_id: String,
    /// The risk class of the matched phrase.
    pub class: ForbiddenPhraseClass,
    /// The lowercase pattern that matched.
    pub matched_pattern: String,
    /// The surface the candidate copy was linted for.
    pub surface: AiCopySurface,
    /// Approved replacement term ids the surface should use instead.
    pub approved_replacement_term_ids: Vec<String>,
}

/// Catalog-level trust and AI-wording-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyTrustReview {
    /// AI wording never overstates confidence about inferred output.
    pub ai_wording_never_overstates_confidence: bool,
    /// AI wording never implies review-free or autonomous completion.
    pub ai_wording_never_implies_review_free_completion: bool,
    /// AI wording never claims validation without a named validation state.
    pub ai_wording_never_claims_unproven_validation: bool,
    /// Proposal state stays explicit (Suggested/Proposed/Draft) until accepted.
    pub proposal_state_explicit_until_accepted: bool,
    /// Low-confidence and review-required terms suppress direct mutation controls.
    pub low_confidence_suppresses_direct_mutation: bool,
    /// Review-required wording names the review surface or owner.
    pub review_required_names_review_surface: bool,
    /// Reversibility (revert / undo availability) is disclosed for changes.
    pub reversibility_state_disclosed: bool,
    /// Forbidden high-trust phrases are rejected on every protected surface.
    pub forbidden_high_trust_phrases_rejected_on_protected_surfaces: bool,
    /// AI wording stays distinct from deterministic language-service wording.
    pub ai_wording_distinct_from_deterministic_service_wording: bool,
    /// Support export reconstructs the exact in-product proposal/validation copy.
    pub support_export_reconstructs_in_product_wording: bool,
    /// One catalog is the source of truth, not parallel AI-copy islands.
    pub one_catalog_not_parallel_copy_islands: bool,
    /// Machine tokens, ids, and patterns stay locale-neutral.
    pub machine_tokens_and_ids_stay_locale_neutral: bool,
    /// Human prose localizes around the locale-neutral tokens.
    pub human_prose_localizes_around_tokens: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyConsumerProjection {
    /// Product UI resolves AI wording through the catalog.
    pub product_ui_resolves_through_catalog: bool,
    /// Docs / help reuse the term identities.
    pub docs_help_reuses_term_identities: bool,
    /// Support export uses the catalog terms.
    pub support_export_uses_catalog_terms: bool,
    /// Narrated announcements reuse the term copy.
    pub narrated_announcements_reuse_term_copy: bool,
    /// Release-note / demo artifacts reuse the term copy.
    pub release_demo_artifacts_reuse_term_copy: bool,
    /// Prompt composer honors the copy guardrails.
    pub prompt_composer_honors_guardrails: bool,
    /// Patch review honors the copy guardrails.
    pub patch_review_honors_guardrails: bool,
    /// Notebook help honors the copy guardrails.
    pub notebook_help_honors_guardrails: bool,
    /// Provider / account surfaces honor the copy guardrails.
    pub provider_account_honors_guardrails: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every term.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every term.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`AiCopyGuardrailCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiCopyGuardrailCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy (e.g. `en`).
    pub reference_locale: String,
    /// Controlled AI terms.
    pub terms: Vec<AiCopyTerm>,
    /// Forbidden high-trust phrases.
    pub forbidden_phrases: Vec<ForbiddenPhrase>,
    /// Shared reuse term ids that must span multiple consumers.
    pub shared_reuse_term_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: AiCopyTrustReview,
    /// Consumer projection block.
    pub consumer_projection: AiCopyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: AiCopyProofFreshness,
    /// Release posture.
    pub release_posture: AiCopyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI copy guardrail catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiCopyGuardrailCatalog {
    /// Record kind; must equal [`AI_COPY_GUARDRAIL_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AI_COPY_GUARDRAIL_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy.
    pub reference_locale: String,
    /// Closed domain inventory (locale-neutral tokens).
    pub domain_inventory: Vec<String>,
    /// Closed taxonomy-concept inventory (locale-neutral tokens).
    pub concept_inventory: Vec<String>,
    /// Closed protected-surface inventory (locale-neutral tokens).
    pub surface_inventory: Vec<String>,
    /// Closed reuse-consumer inventory (locale-neutral tokens).
    pub consumer_inventory: Vec<String>,
    /// Closed forbidden-class inventory (locale-neutral tokens).
    pub forbidden_class_inventory: Vec<String>,
    /// Controlled AI terms.
    pub terms: Vec<AiCopyTerm>,
    /// Forbidden high-trust phrases.
    pub forbidden_phrases: Vec<ForbiddenPhrase>,
    /// Shared reuse term ids that must span multiple consumers.
    pub shared_reuse_term_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: AiCopyTrustReview,
    /// Consumer projection block.
    pub consumer_projection: AiCopyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: AiCopyProofFreshness,
    /// Release posture.
    pub release_posture: AiCopyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiCopyGuardrailCatalog {
    /// Builds a catalog packet from lane input, filling the closed inventories from
    /// the canonical enum token lists.
    pub fn new(input: AiCopyGuardrailCatalogInput) -> Self {
        Self {
            record_kind: AI_COPY_GUARDRAIL_CATALOG_RECORD_KIND.to_owned(),
            schema_version: AI_COPY_GUARDRAIL_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            domain_inventory: token_list(&AiCopyDomain::ALL, AiCopyDomain::as_str),
            concept_inventory: token_list(&AiTaxonomyConcept::ALL, AiTaxonomyConcept::as_str),
            surface_inventory: token_list(&AiCopySurface::ALL, AiCopySurface::as_str),
            consumer_inventory: token_list(&AiCopyConsumer::ALL, AiCopyConsumer::as_str),
            forbidden_class_inventory: token_list(
                &ForbiddenPhraseClass::ALL,
                ForbiddenPhraseClass::as_str,
            ),
            terms: input.terms,
            forbidden_phrases: input.forbidden_phrases,
            shared_reuse_term_ids: input.shared_reuse_term_ids,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves a term by id.
    pub fn term(&self, term_id: &str) -> Option<&AiCopyTerm> {
        self.terms.iter().find(|t| t.term_id == term_id)
    }

    /// Resolves a forbidden phrase by id.
    pub fn forbidden_phrase(&self, phrase_id: &str) -> Option<&ForbiddenPhrase> {
        self.forbidden_phrases
            .iter()
            .find(|p| p.phrase_id == phrase_id)
    }

    /// All terms materializing a taxonomy concept, in catalog order.
    pub fn terms_for_concept(&self, concept: AiTaxonomyConcept) -> Vec<&AiCopyTerm> {
        self.terms.iter().filter(|t| t.concept == concept).collect()
    }

    /// Lints candidate copy for a protected surface and returns every forbidden
    /// phrase that matches.
    ///
    /// This is the lane's lint entry point: a protected AI surface routes its copy
    /// through this check so `Guaranteed`, `Perfect`, `Done for you`, and
    /// `No review needed` can never reach the user. Matching is case-insensitive and
    /// substring-based on the lowercase pattern; the catalog keeps patterns
    /// lowercase so the result is deterministic.
    pub fn lint(&self, candidate: &str, surface: AiCopySurface) -> Vec<AiCopyLintFinding> {
        let lower = candidate.to_lowercase();
        let mut findings = Vec::new();
        for phrase in &self.forbidden_phrases {
            if phrase.forbidden_on.contains(&surface) && lower.contains(&phrase.pattern) {
                findings.push(AiCopyLintFinding {
                    phrase_id: phrase.phrase_id.clone(),
                    class: phrase.class,
                    matched_pattern: phrase.pattern.clone(),
                    surface,
                    approved_replacement_term_ids: phrase.approved_replacement_term_ids.clone(),
                });
            }
        }
        findings
    }

    /// True when candidate copy carries no forbidden phrase on the surface.
    pub fn is_clean(&self, candidate: &str, surface: AiCopySurface) -> bool {
        self.lint(candidate, surface).is_empty()
    }

    /// Renders the deterministic reference wording for a term so review, help, and
    /// support surfaces can reconstruct the exact in-product proposal, confidence,
    /// or validation language. Returns `None` if the term id is unknown.
    pub fn render_term_reference(&self, term_id: &str) -> Option<String> {
        let term = self.term(term_id)?;
        let mut out = String::new();
        out.push_str(&format!(
            "{} [{} / {}]: {}",
            term.canonical_label,
            term.concept.as_str(),
            term.domain.as_str(),
            term.reserved_meaning
        ));
        if !term.required_context.is_empty() {
            out.push_str(" Requires: ");
            out.push_str(&term.required_context.join("; "));
            out.push('.');
        }
        if term.suppresses_direct_mutation {
            out.push_str(" Direct mutation controls are suppressed.");
        }
        Some(out)
    }

    /// Maps each term id to the distinct reuse consumers that reconstruct it.
    ///
    /// This is the reuse proof: a shared term reconstructed by the UI, a support
    /// export, and a narrated announcement shows the catalog is the one source the
    /// consumers share.
    pub fn cross_consumer_reuse(
        &self,
    ) -> std::collections::BTreeMap<String, BTreeSet<&'static str>> {
        let mut reuse: std::collections::BTreeMap<String, BTreeSet<&'static str>> =
            std::collections::BTreeMap::new();
        for term in &self.terms {
            let entry = reuse.entry(term.term_id.clone()).or_default();
            for consumer in &term.consumers {
                entry.insert(consumer.as_str());
            }
        }
        reuse
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<AiCopyGuardrailViolation> {
        let mut violations = Vec::new();

        if self.record_kind != AI_COPY_GUARDRAIL_CATALOG_RECORD_KIND {
            violations.push(AiCopyGuardrailViolation::WrongRecordKind);
        }
        if self.schema_version != AI_COPY_GUARDRAIL_CATALOG_SCHEMA_VERSION {
            violations.push(AiCopyGuardrailViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiCopyGuardrailViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_terms(self, &mut violations);
        validate_forbidden_phrases(self, &mut violations);
        validate_self_lint(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_reuse(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai copy guardrail catalog serializes"),
        ) {
            violations.push(AiCopyGuardrailViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai copy guardrail catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# AI Copy Guardrails, Confidence Vocabulary, and Forbidden High-Trust Phrasing\n\n",
        );
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!(
            "- Controlled terms: {} | Forbidden phrases: {}\n",
            self.terms.len(),
            self.forbidden_phrases.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Controlled AI terms\n\n");
        for term in &self.terms {
            if let Some(rendered) = self.render_term_reference(&term.term_id) {
                out.push_str(&format!("- `{}` — {}\n", term.term_id, rendered));
            }
            out.push_str(&format!(
                "  - Surfaces: {} | Consumers: {}\n",
                term.surfaces
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                term.consumers
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        out.push_str("\n## Forbidden high-trust phrases\n\n");
        for phrase in &self.forbidden_phrases {
            out.push_str(&format!(
                "- `{}` ({}): \"{}\" — {} → {}\n",
                phrase.phrase_id,
                phrase.class.as_str(),
                phrase.pattern,
                phrase.rejection_reason,
                phrase.approved_replacement_term_ids.join(", ")
            ));
        }

        out.push_str("\n## Cross-consumer term reuse\n\n");
        for (term_id, consumers) in self.cross_consumer_reuse() {
            out.push_str(&format!(
                "- `{}`: {}\n",
                term_id,
                consumers.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum AiCopyGuardrailArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiCopyGuardrailViolation>),
}

impl fmt::Display for AiCopyGuardrailArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "ai copy guardrail catalog export parse failed: {error}"
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
                    "ai copy guardrail catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiCopyGuardrailArtifactError {}

/// Validation failures emitted by [`AiCopyGuardrailCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiCopyGuardrailViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A closed inventory drifted from the canonical token list.
    InventoryDrift,
    /// A controlled term is incomplete.
    TermIncomplete,
    /// A term id or machine token is not locale-neutral.
    TermTokenNotLocaleNeutral,
    /// A term id, label, or machine token is duplicated.
    DuplicateTerm,
    /// A term's declared domain does not match its concept's domain.
    TermDomainMismatch,
    /// An AI term is not flagged provisional.
    TermNotProvisional,
    /// A low-confidence or review-required term allows direct mutation.
    MutationSuppressionMissing,
    /// A required taxonomy concept has no term.
    TaxonomyConceptNotCovered,
    /// A term's own copy overclaims (fails the catalog's own lint).
    TermCopyOverclaims,
    /// A forbidden phrase is incomplete.
    ForbiddenPhraseIncomplete,
    /// A forbidden phrase id is not locale-neutral.
    ForbiddenPhraseIdNotLocaleNeutral,
    /// A forbidden phrase pattern is not lowercase (lint would be unstable).
    ForbiddenPatternNotLowercase,
    /// A forbidden phrase id or pattern is duplicated.
    DuplicateForbiddenPhrase,
    /// A forbidden phrase names a replacement term that does not resolve.
    ForbiddenReplacementUnresolved,
    /// A required high-trust phrase is not in the forbidden register.
    RequiredHighTrustPhraseMissing,
    /// A domain, concept, surface, consumer, or forbidden class is never used.
    CoverageGap,
    /// A shared reuse term does not span enough consumers.
    SharedTermReuseInsufficient,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiCopyGuardrailViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::TermIncomplete => "term_incomplete",
            Self::TermTokenNotLocaleNeutral => "term_token_not_locale_neutral",
            Self::DuplicateTerm => "duplicate_term",
            Self::TermDomainMismatch => "term_domain_mismatch",
            Self::TermNotProvisional => "term_not_provisional",
            Self::MutationSuppressionMissing => "mutation_suppression_missing",
            Self::TaxonomyConceptNotCovered => "taxonomy_concept_not_covered",
            Self::TermCopyOverclaims => "term_copy_overclaims",
            Self::ForbiddenPhraseIncomplete => "forbidden_phrase_incomplete",
            Self::ForbiddenPhraseIdNotLocaleNeutral => "forbidden_phrase_id_not_locale_neutral",
            Self::ForbiddenPatternNotLowercase => "forbidden_pattern_not_lowercase",
            Self::DuplicateForbiddenPhrase => "duplicate_forbidden_phrase",
            Self::ForbiddenReplacementUnresolved => "forbidden_replacement_unresolved",
            Self::RequiredHighTrustPhraseMissing => "required_high_trust_phrase_missing",
            Self::CoverageGap => "coverage_gap",
            Self::SharedTermReuseInsufficient => "shared_term_reuse_insufficient",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in catalog export.
pub fn current_ai_copy_guardrail_catalog_export(
) -> Result<AiCopyGuardrailCatalog, AiCopyGuardrailArtifactError> {
    let packet: AiCopyGuardrailCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-ai-copy-proof/support_export.json"
    )))
    .map_err(AiCopyGuardrailArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiCopyGuardrailArtifactError::Validation(violations))
    }
}

/// True when `token` is a locale-neutral machine identifier: non-empty and only
/// lowercase ascii letters, digits, `_`, and `.`.
fn is_locale_neutral(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
}

/// True when a forbidden pattern is a stable lowercase phrase: non-empty, already
/// lowercase, and only ascii letters, digits, spaces, and hyphens.
fn is_lower_phrase(pattern: &str) -> bool {
    !pattern.is_empty()
        && pattern == pattern.to_lowercase()
        && pattern
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == ' ' || c == '-')
}

fn token_list<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| as_str(*t).to_owned()).collect()
}

fn validate_source_contracts(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        AI_COPY_GUARDRAIL_CATALOG_SCHEMA_REF,
        AI_COPY_GUARDRAIL_CATALOG_DOC_REF,
        AI_COPY_GUARDRAILS_CONTRACT_REF,
        AI_COPY_APPROVED_TERMS_REF,
        AI_COPY_FORBIDDEN_TERMS_REF,
        AI_COPY_CONTROLLED_GLOSSARY_REF,
        AI_COPY_UI_COPY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AiCopyGuardrailViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    if packet.domain_inventory != token_list(&AiCopyDomain::ALL, AiCopyDomain::as_str)
        || packet.concept_inventory
            != token_list(&AiTaxonomyConcept::ALL, AiTaxonomyConcept::as_str)
        || packet.surface_inventory != token_list(&AiCopySurface::ALL, AiCopySurface::as_str)
        || packet.consumer_inventory != token_list(&AiCopyConsumer::ALL, AiCopyConsumer::as_str)
        || packet.forbidden_class_inventory
            != token_list(&ForbiddenPhraseClass::ALL, ForbiddenPhraseClass::as_str)
    {
        violations.push(AiCopyGuardrailViolation::InventoryDrift);
    }
}

fn validate_terms(packet: &AiCopyGuardrailCatalog, violations: &mut Vec<AiCopyGuardrailViolation>) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_tokens: BTreeSet<&str> = BTreeSet::new();
    let mut seen_labels: BTreeSet<&str> = BTreeSet::new();

    for term in &packet.terms {
        if term.canonical_label.trim().is_empty()
            || term.reserved_meaning.trim().is_empty()
            || term.required_context.is_empty()
            || term.surfaces.is_empty()
            || term.consumers.is_empty()
        {
            violations.push(AiCopyGuardrailViolation::TermIncomplete);
        }
        if !is_locale_neutral(&term.term_id) || !is_locale_neutral(&term.machine_token) {
            violations.push(AiCopyGuardrailViolation::TermTokenNotLocaleNeutral);
        }
        if !seen_ids.insert(term.term_id.as_str())
            || !seen_tokens.insert(term.machine_token.as_str())
            || !seen_labels.insert(term.canonical_label.as_str())
        {
            violations.push(AiCopyGuardrailViolation::DuplicateTerm);
        }
        if term.domain != term.concept.domain() {
            violations.push(AiCopyGuardrailViolation::TermDomainMismatch);
        }
        // AI wording is provisional, never a deterministic guarantee. This keeps AI
        // copy distinct from deterministic language-service / formatter wording.
        if !term.ai_provisional {
            violations.push(AiCopyGuardrailViolation::TermNotProvisional);
        }
        // Low-confidence and review-required proposals can never offer direct apply.
        let must_suppress = term.concept.suppresses_direct_mutation();
        if term.suppresses_direct_mutation != must_suppress {
            violations.push(AiCopyGuardrailViolation::MutationSuppressionMissing);
        }
    }

    // Every required taxonomy concept must be materialized at least once.
    let covered: BTreeSet<AiTaxonomyConcept> = packet.terms.iter().map(|t| t.concept).collect();
    if !AiTaxonomyConcept::ALL.iter().all(|c| covered.contains(c)) {
        violations.push(AiCopyGuardrailViolation::TaxonomyConceptNotCovered);
    }
}

fn validate_forbidden_phrases(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let term_ids: BTreeSet<&str> = packet.terms.iter().map(|t| t.term_id.as_str()).collect();
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_patterns: BTreeSet<&str> = BTreeSet::new();

    for phrase in &packet.forbidden_phrases {
        if phrase.rejection_reason.trim().is_empty()
            || phrase.approved_replacement_term_ids.is_empty()
            || phrase.forbidden_on.is_empty()
        {
            violations.push(AiCopyGuardrailViolation::ForbiddenPhraseIncomplete);
        }
        if !is_locale_neutral(&phrase.phrase_id) {
            violations.push(AiCopyGuardrailViolation::ForbiddenPhraseIdNotLocaleNeutral);
        }
        if !is_lower_phrase(&phrase.pattern) {
            violations.push(AiCopyGuardrailViolation::ForbiddenPatternNotLowercase);
        }
        if !seen_ids.insert(phrase.phrase_id.as_str())
            || !seen_patterns.insert(phrase.pattern.as_str())
        {
            violations.push(AiCopyGuardrailViolation::DuplicateForbiddenPhrase);
        }
        for term_id in &phrase.approved_replacement_term_ids {
            if !term_ids.contains(term_id.as_str()) {
                violations.push(AiCopyGuardrailViolation::ForbiddenReplacementUnresolved);
            }
        }
    }

    // Every explicitly required high-trust phrase must be rejected.
    let patterns: BTreeSet<&str> = packet
        .forbidden_phrases
        .iter()
        .map(|p| p.pattern.as_str())
        .collect();
    for required in REQUIRED_FORBIDDEN_PATTERNS {
        if !patterns.contains(required) {
            violations.push(AiCopyGuardrailViolation::RequiredHighTrustPhraseMissing);
        }
    }
}

/// The catalog eats its own dog food: every approved term's canonical label and
/// reserved meaning must pass the lint on every surface the term governs. An
/// approved term can never quietly carry an overclaim. The misuse-only
/// `forbidden_uses` field is intentionally excluded — it names what *not* to say.
fn validate_self_lint(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    for term in &packet.terms {
        let mut copy = String::new();
        copy.push_str(&term.canonical_label);
        copy.push(' ');
        copy.push_str(&term.reserved_meaning);
        copy.push(' ');
        copy.push_str(&term.required_context.join(" "));
        for surface in &term.surfaces {
            if !packet.lint(&copy, *surface).is_empty() {
                violations.push(AiCopyGuardrailViolation::TermCopyOverclaims);
                break;
            }
        }
    }
}

fn validate_coverage(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let domains: BTreeSet<AiCopyDomain> = packet.terms.iter().map(|t| t.domain).collect();
    let surfaces: BTreeSet<AiCopySurface> = packet
        .terms
        .iter()
        .flat_map(|t| t.surfaces.iter().copied())
        .collect();
    let consumers: BTreeSet<AiCopyConsumer> = packet
        .terms
        .iter()
        .flat_map(|t| t.consumers.iter().copied())
        .collect();
    let classes: BTreeSet<ForbiddenPhraseClass> =
        packet.forbidden_phrases.iter().map(|p| p.class).collect();

    let domains_covered = AiCopyDomain::ALL.iter().all(|d| domains.contains(d));
    let surfaces_covered = AiCopySurface::ALL.iter().all(|s| surfaces.contains(s));
    let consumers_covered = AiCopyConsumer::ALL.iter().all(|c| consumers.contains(c));
    let classes_covered = ForbiddenPhraseClass::ALL
        .iter()
        .all(|c| classes.contains(c));

    if !domains_covered || !surfaces_covered || !consumers_covered || !classes_covered {
        violations.push(AiCopyGuardrailViolation::CoverageGap);
    }
}

fn validate_shared_reuse(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    if packet.shared_reuse_term_ids.is_empty() {
        violations.push(AiCopyGuardrailViolation::SharedTermReuseInsufficient);
        return;
    }
    let reuse = packet.cross_consumer_reuse();
    for term_id in &packet.shared_reuse_term_ids {
        let spans = reuse.get(term_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_TERM_MIN_REUSE_CONSUMERS {
            violations.push(AiCopyGuardrailViolation::SharedTermReuseInsufficient);
        }
    }
}

fn validate_trust_review(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.ai_wording_never_overstates_confidence,
        review.ai_wording_never_implies_review_free_completion,
        review.ai_wording_never_claims_unproven_validation,
        review.proposal_state_explicit_until_accepted,
        review.low_confidence_suppresses_direct_mutation,
        review.review_required_names_review_surface,
        review.reversibility_state_disclosed,
        review.forbidden_high_trust_phrases_rejected_on_protected_surfaces,
        review.ai_wording_distinct_from_deterministic_service_wording,
        review.support_export_reconstructs_in_product_wording,
        review.one_catalog_not_parallel_copy_islands,
        review.machine_tokens_and_ids_stay_locale_neutral,
        review.human_prose_localizes_around_tokens,
    ] {
        if !ok {
            violations.push(AiCopyGuardrailViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.product_ui_resolves_through_catalog,
        projection.docs_help_reuses_term_identities,
        projection.support_export_uses_catalog_terms,
        projection.narrated_announcements_reuse_term_copy,
        projection.release_demo_artifacts_reuse_term_copy,
        projection.prompt_composer_honors_guardrails,
        projection.patch_review_honors_guardrails,
        projection.notebook_help_honors_guardrails,
        projection.provider_account_honors_guardrails,
    ] {
        if !ok {
            violations.push(AiCopyGuardrailViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AiCopyGuardrailViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &AiCopyGuardrailCatalog,
    violations: &mut Vec<AiCopyGuardrailViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(AiCopyGuardrailViolation::ReleasePostureIncomplete);
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

/// Rewrites a human prose run into a pseudo-localized form by wrapping it in locale
/// markers. Machine-facing identity (ids, tokens, patterns) never passes through
/// this function, so a localized overlay can never fork the meaning of a term.
pub fn pseudo_localize_prose(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.is_empty() {
        return prose.to_owned();
    }
    let leading = &prose[..prose.len() - prose.trim_start().len()];
    let trailing = &prose[prose.trim_end().len()..];
    format!("{leading}\u{27e6}{trimmed}\u{27e7}{trailing}")
}
