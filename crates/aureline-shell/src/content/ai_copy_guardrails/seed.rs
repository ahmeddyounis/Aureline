//! Canonical seed builders for the AI copy guardrail catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests
//! both call them so the in-code catalog, the artifact, and the fixtures never
//! drift.

use super::*;

/// Stable catalog id for the canonical AI copy guardrail catalog.
pub const AI_COPY_GUARDRAIL_CATALOG_ID: &str = "m5-ai-copy-guardrail-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

use AiCopyConsumer as Co;
use AiCopySurface as Su;
use AiTaxonomyConcept as Cn;
use ForbiddenPhraseClass as Fc;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn term(
    term_id: &str,
    concept: AiTaxonomyConcept,
    canonical_label: &str,
    machine_token: &str,
    reserved_meaning: &str,
    required_context: &[&str],
    forbidden_uses: &[&str],
    surfaces: &[AiCopySurface],
    consumers: &[AiCopyConsumer],
    requires_evidence_basis: bool,
) -> AiCopyTerm {
    AiCopyTerm {
        term_id: term_id.to_owned(),
        concept,
        domain: concept.domain(),
        canonical_label: canonical_label.to_owned(),
        machine_token: machine_token.to_owned(),
        reserved_meaning: reserved_meaning.to_owned(),
        required_context: strings(required_context),
        forbidden_uses: strings(forbidden_uses),
        surfaces: surfaces.to_vec(),
        consumers: consumers.to_vec(),
        ai_provisional: true,
        suppresses_direct_mutation: concept.suppresses_direct_mutation(),
        requires_evidence_basis,
    }
}

fn phrase(
    phrase_id: &str,
    class: ForbiddenPhraseClass,
    pattern: &str,
    rejection_reason: &str,
    approved_replacement_term_ids: &[&str],
) -> ForbiddenPhrase {
    ForbiddenPhrase {
        phrase_id: phrase_id.to_owned(),
        class,
        pattern: pattern.to_owned(),
        rejection_reason: rejection_reason.to_owned(),
        approved_replacement_term_ids: strings(approved_replacement_term_ids),
        // Every high-trust overclaim is forbidden on every protected surface.
        forbidden_on: AiCopySurface::ALL.to_vec(),
    }
}

fn terms() -> Vec<AiCopyTerm> {
    vec![
        term(
            "term.proposal.suggested",
            Cn::Suggested,
            "Suggested",
            "suggested",
            "Model-proposed guidance or a next step that has not been accepted, applied, run through validation, or made authoritative.",
            &[
                "Evidence basis or cited refs",
                "Confidence label and reason",
                "Next safe action such as Open source or Open diff",
            ],
            &[
                "Do not imply approval or policy admission",
                "Do not present as already applied",
            ],
            &[Su::PromptComposer, Su::PatchReview, Su::NotebookHelp],
            &[Co::ProductUi, Co::SupportExport, Co::NarratedAnnouncement],
            true,
        ),
        term(
            "term.proposal.proposed",
            Cn::Proposed,
            "Proposed",
            "proposed",
            "A concrete change put forward for review; it carries a target and scope but is not yet accepted or applied.",
            &[
                "Target identity and scope",
                "Confidence label",
                "Review or open-diff route",
            ],
            &[
                "Do not present as accepted",
                "Do not imply review is complete",
            ],
            &[Su::PatchReview, Su::PromptComposer],
            &[Co::ProductUi, Co::SupportExport, Co::ReleaseDemo],
            true,
        ),
        term(
            "term.proposal.draft",
            Cn::Draft,
            "Draft",
            "draft",
            "Generated content or a generated patch that exists outside canonical source truth and still needs review before apply or publication.",
            &[
                "Draft or request-workspace ref",
                "Source or diff route",
                "Review or discard path",
            ],
            &["Do not use for already-applied source"],
            &[Su::PromptComposer, Su::PatchReview, Su::DocsHelp],
            &[Co::ProductUi, Co::DocsHelp, Co::SupportExport],
            false,
        ),
        term(
            "term.context.context_used",
            Cn::ContextUsed,
            "Context used",
            "context_used",
            "Names the context segments an answer actually used and the scope it left out, so the surface never implies it read more than it did.",
            &[
                "Covered scope",
                "Excluded or omitted scope and reason",
                "Expand or open-detail route when available",
            ],
            &[
                "Do not pair with exhaustive phrases",
                "Do not imply the whole project was read",
            ],
            &[Su::PromptComposer, Su::NotebookHelp, Su::ProviderAccount],
            &[Co::ProductUi, Co::SupportExport, Co::NarratedAnnouncement],
            true,
        ),
        term(
            "term.validation.not_run",
            Cn::Validation,
            "Validation not run",
            "validation_not_run",
            "No validation plan has produced an outcome for this AI result; the surface must not imply checks, lint, build, or policy review passed.",
            &[
                "Reason validation is missing, skipped, or not applicable",
                "Safe validation route when available",
            ],
            &["Do not combine with passed, verified, or ready wording"],
            &[Su::PatchReview, Su::PromptComposer],
            &[Co::ProductUi, Co::SupportExport, Co::ReleaseDemo],
            false,
        ),
        term(
            "term.validation.passed",
            Cn::Validation,
            "Validation passed",
            "validation_passed",
            "A named validation plan produced a passed outcome for the declared scope; the surface names the validation plan and the exact scope it covered.",
            &[
                "Validation plan ref",
                "Declared validation scope",
                "Any excluded checks or stale evidence",
            ],
            &[
                "Do not use when the validation plan ref is absent",
                "Do not imply broader scope than the plan covered",
            ],
            &[Su::PatchReview],
            &[Co::ProductUi, Co::SupportExport, Co::DocsHelp],
            true,
        ),
        term(
            "term.confidence.low_confidence",
            Cn::LowConfidence,
            "Low confidence",
            "low_confidence",
            "The AI result is below the surface confidence floor because evidence is missing, conflicting, stale, partial, or omitted by policy.",
            &[
                "The specific limiting reason",
                "A safe next action such as Open source or Prepare preview",
                "Direct mutation controls removed",
            ],
            &[
                "Do not pair with direct apply as the primary action",
                "Do not omit the limiting reason",
            ],
            &[
                Su::PromptComposer,
                Su::PatchReview,
                Su::NotebookHelp,
                Su::DocsHelp,
            ],
            &[Co::ProductUi, Co::SupportExport, Co::NarratedAnnouncement],
            true,
        ),
        term(
            "term.review.review_required",
            Cn::ReviewRequired,
            "Review required",
            "review_required",
            "Human, policy, ownership, or write-scope review remains required before the change is applied, published, or sent to a provider.",
            &[
                "Review owner or review surface",
                "The blocked action or scope",
            ],
            &["Do not hide the review owner behind generic disabled styling"],
            &[Su::PatchReview, Su::ProviderAccount],
            &[Co::ProductUi, Co::SupportExport, Co::ReleaseDemo],
            false,
        ),
        term(
            "term.reversibility.revert_undo_available",
            Cn::RevertUndoAvailable,
            "Revert available",
            "revert_undo_available",
            "A prior, known-good state is retained, so an applied or proposed change can be reverted or undone through a named checkpoint.",
            &[
                "Checkpoint or revert-class ref",
                "The revert or undo route",
            ],
            &["Do not imply reversibility when no checkpoint is retained"],
            &[Su::PatchReview, Su::ProviderAccount],
            &[Co::ProductUi, Co::SupportExport, Co::NarratedAnnouncement],
            false,
        ),
    ]
}

fn forbidden_phrases() -> Vec<ForbiddenPhrase> {
    vec![
        phrase(
            "forbidden.perfection.guaranteed",
            Fc::PerfectionGuarantee,
            "guaranteed",
            "AI-inferred output is provisional and cannot promise guaranteed success or absence of risk.",
            &["term.confidence.low_confidence", "term.validation.not_run"],
        ),
        phrase(
            "forbidden.perfection.perfect",
            Fc::PerfectionGuarantee,
            "perfect",
            "AI-inferred output is provisional and cannot be described as perfect.",
            &["term.confidence.low_confidence", "term.proposal.suggested"],
        ),
        phrase(
            "forbidden.review_free.no_review_needed",
            Fc::ReviewFreeMutation,
            "no review needed",
            "AI copy cannot waive the review and approval a mutation requires.",
            &["term.review.review_required", "term.validation.not_run"],
        ),
        phrase(
            "forbidden.review_free.auto_apply",
            Fc::ReviewFreeMutation,
            "auto-apply",
            "AI copy cannot imply a change applies without review or approval.",
            &["term.review.review_required", "term.proposal.draft"],
        ),
        phrase(
            "forbidden.autonomy.done_for_you",
            Fc::FalseAutonomy,
            "done for you",
            "AI copy cannot imply the assistant autonomously finished the work.",
            &["term.proposal.suggested", "term.review.review_required"],
        ),
        phrase(
            "forbidden.autonomy.fully_autonomous",
            Fc::FalseAutonomy,
            "fully autonomous",
            "AI copy cannot claim autonomous completion on a trust-sensitive surface.",
            &["term.proposal.suggested", "term.proposal.proposed"],
        ),
        phrase(
            "forbidden.validation.validated",
            Fc::FalseValidation,
            "validated",
            "Validation language is reserved for a named validation state and outcome.",
            &["term.validation.not_run", "term.validation.passed"],
        ),
        phrase(
            "forbidden.validation.safe_to_apply",
            Fc::FalseValidation,
            "safe to apply",
            "Safety-to-apply requires a named validation outcome, not AI prose.",
            &["term.validation.not_run", "term.review.review_required"],
        ),
        phrase(
            "forbidden.confidence.definitely",
            Fc::ConfidenceOverstatement,
            "definitely",
            "AI copy must state evidence and confidence class instead of pretending certainty.",
            &["term.confidence.low_confidence", "term.proposal.suggested"],
        ),
        phrase(
            "forbidden.confidence.knows_the_codebase",
            Fc::ConfidenceOverstatement,
            "knows the codebase",
            "AI copy must not pretend inference is direct knowledge of the codebase.",
            &["term.context.context_used", "term.confidence.low_confidence"],
        ),
        phrase(
            "forbidden.exhaustiveness.all_files",
            Fc::FalseExhaustiveness,
            "all files",
            "Scope breadth must come from a scope object, not AI prose.",
            &["term.context.context_used", "term.confidence.low_confidence"],
        ),
        phrase(
            "forbidden.exhaustiveness.nothing_else_affected",
            Fc::FalseExhaustiveness,
            "nothing else affected",
            "Impact breadth must come from a scope object, not AI prose.",
            &["term.context.context_used", "term.review.review_required"],
        ),
        phrase(
            "forbidden.freshness.up_to_date",
            Fc::FalseFreshness,
            "up to date",
            "Freshness language must match the governing freshness state.",
            &["term.confidence.low_confidence", "term.context.context_used"],
        ),
        phrase(
            "forbidden.freshness.latest_docs",
            Fc::FalseFreshness,
            "latest docs",
            "Docs freshness must match the governing freshness state, not AI prose.",
            &["term.context.context_used", "term.confidence.low_confidence"],
        ),
    ]
}

fn shared_reuse_term_ids() -> Vec<String> {
    strings(&[
        "term.proposal.suggested",
        "term.confidence.low_confidence",
        "term.context.context_used",
    ])
}

fn trust_review() -> AiCopyTrustReview {
    AiCopyTrustReview {
        ai_wording_never_overstates_confidence: true,
        ai_wording_never_implies_review_free_completion: true,
        ai_wording_never_claims_unproven_validation: true,
        proposal_state_explicit_until_accepted: true,
        low_confidence_suppresses_direct_mutation: true,
        review_required_names_review_surface: true,
        reversibility_state_disclosed: true,
        forbidden_high_trust_phrases_rejected_on_protected_surfaces: true,
        ai_wording_distinct_from_deterministic_service_wording: true,
        support_export_reconstructs_in_product_wording: true,
        one_catalog_not_parallel_copy_islands: true,
        machine_tokens_and_ids_stay_locale_neutral: true,
        human_prose_localizes_around_tokens: true,
    }
}

fn consumer_projection() -> AiCopyConsumerProjection {
    AiCopyConsumerProjection {
        product_ui_resolves_through_catalog: true,
        docs_help_reuses_term_identities: true,
        support_export_uses_catalog_terms: true,
        narrated_announcements_reuse_term_copy: true,
        release_demo_artifacts_reuse_term_copy: true,
        prompt_composer_honors_guardrails: true,
        patch_review_honors_guardrails: true,
        notebook_help_honors_guardrails: true,
        provider_account_honors_guardrails: true,
    }
}

fn proof_freshness() -> AiCopyProofFreshness {
    AiCopyProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> AiCopyReleasePosture {
    AiCopyReleasePosture {
        release_packet_ref: "evidence:ai-copy-guardrail-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:ai-copy-guardrail-catalog-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        AI_COPY_GUARDRAIL_CATALOG_SCHEMA_REF,
        AI_COPY_GUARDRAIL_CATALOG_DOC_REF,
        AI_COPY_GUARDRAILS_CONTRACT_REF,
        AI_COPY_APPROVED_TERMS_REF,
        AI_COPY_FORBIDDEN_TERMS_REF,
        AI_COPY_CONTROLLED_GLOSSARY_REF,
        AI_COPY_UI_COPY_CONTRACT_REF,
    ])
}

fn base_input() -> AiCopyGuardrailCatalogInput {
    AiCopyGuardrailCatalogInput {
        catalog_id: AI_COPY_GUARDRAIL_CATALOG_ID.to_owned(),
        catalog_label:
            "AI Copy Guardrails, Confidence Vocabulary, and Forbidden High-Trust Phrasing"
                .to_owned(),
        reference_locale: "en".to_owned(),
        terms: terms(),
        forbidden_phrases: forbidden_phrases(),
        shared_reuse_term_ids: shared_reuse_term_ids(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical AI copy guardrail catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_ai_copy_guardrail_catalog() -> AiCopyGuardrailCatalog {
    AiCopyGuardrailCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose changes: term labels, reserved meanings, required-context
/// and forbidden-use lines, and forbidden-phrase rejection reasons are
/// pseudo-localized, while every term id, machine token, concept, domain, surface,
/// consumer, phrase id, class, pattern, and replacement ref stays byte-for-byte
/// identical. A localized overlay can never fork the meaning of a proposal, a
/// confidence label, a validation state, or a forbidden pattern.
pub fn seeded_ai_copy_guardrail_catalog_localized() -> AiCopyGuardrailCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-ai-copy-guardrail-catalog:localized:0001".to_owned();
    input.catalog_label = format!("{} (localized overlay)", input.catalog_label);
    input.reference_locale = "qps-ploc".to_owned();
    for term in &mut input.terms {
        term.canonical_label = pseudo_localize_prose(&term.canonical_label);
        term.reserved_meaning = pseudo_localize_prose(&term.reserved_meaning);
        term.required_context = term
            .required_context
            .iter()
            .map(|line| pseudo_localize_prose(line))
            .collect();
        term.forbidden_uses = term
            .forbidden_uses
            .iter()
            .map(|line| pseudo_localize_prose(line))
            .collect();
    }
    for phrase in &mut input.forbidden_phrases {
        // The pattern is the locale-neutral machine identity and never localizes;
        // only the human-facing rejection reason does.
        phrase.rejection_reason = pseudo_localize_prose(&phrase.rejection_reason);
    }
    AiCopyGuardrailCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity, terms, and forbidden phrases are unchanged; only the
/// catalog id and the mirror/offline release ref differ. This proves the catalog
/// survives an offline mirror without forking the meaning of any term or phrase.
pub fn seeded_ai_copy_guardrail_catalog_offline_mirror() -> AiCopyGuardrailCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-ai-copy-guardrail-catalog:offline-mirror:0001".to_owned();
    input.catalog_label = format!("{} (offline mirror)", input.catalog_label);
    input.release_posture.release_packet_ref =
        "evidence:ai-copy-guardrail-catalog-release-packet:m5:mirror".to_owned();
    AiCopyGuardrailCatalog::new(input)
}
