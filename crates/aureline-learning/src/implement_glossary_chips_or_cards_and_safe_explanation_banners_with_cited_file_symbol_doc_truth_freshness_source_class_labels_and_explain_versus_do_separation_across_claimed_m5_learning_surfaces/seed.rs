//! Canonical seed builders for the glossary-chip-card / safe-explanation-banner controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical glossary-chip-card / safe-explanation-banner packet.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_PACKET_ID: &str =
    "m5-glossary-chip-card-safe-explanation-banner-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn glossary_source_refs() -> Vec<String> {
    strings(&[
        M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

fn banner_source_refs() -> Vec<String> {
    strings(&[
        M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

fn glossary_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::GlossaryCitationSevered,
        M5LearningDowngradeTrigger::CachedStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn banner_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::ExplanationApplyBoundaryUnstated,
        M5LearningDowngradeTrigger::GlossaryCitationSevered,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

/// Builds a glossary chip or card, deriving the citation class, the current claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn entry(
    entry_id: &str,
    term_label: &str,
    source_class: M5GlossarySourceClass,
    citation_state: M5GlossaryCitationState,
    term_meaning: &str,
    citation_kind: DeepLinkKind,
    citation_ref: &str,
    citation_label: &str,
    entry_actions: Vec<GlossaryEntryAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> GlossaryEntry {
    let disclosure = resolve_glossary_citation(citation_state);
    let cited = source_is_cited(source_class);
    GlossaryEntry {
        component: M5LearningComponentFamily::GlossaryChipOrCard,
        entry_id: entry_id.to_owned(),
        term_label: term_label.to_owned(),
        source_class,
        citation_state,
        citation_class: disclosure.citation_class,
        claims_citation_current: disclosure.is_cited_current,
        claims_source_backed: cited,
        term_meaning: term_meaning.to_owned(),
        stale_note: if disclosure.needs_stale_note {
            "Citation is stale; this definition may lag the current source".to_owned()
        } else {
            String::new()
        },
        offline_note: if disclosure.needs_offline_note {
            "Citation is unavailable offline; showing a cached definition".to_owned()
        } else {
            String::new()
        },
        citation_missing_note: if disclosure.needs_uncited_note {
            "No citation resolves for this term; treat the definition as uncited".to_owned()
        } else {
            String::new()
        },
        source_backing_note: if cited {
            String::new()
        } else {
            format!(
                "This definition comes from a {} source, not cited source truth",
                source_class.as_str()
            )
        },
        citation_kind,
        citation_ref: citation_ref.to_owned(),
        citation_label: citation_label.to_owned(),
        entry_actions,
        dispositions,
        downgrade_triggers: glossary_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "term_label",
            "source_class",
            "citation_state",
            "citation_class",
            "term_meaning",
            "citation_label",
        ]),
        source_contract_refs: glossary_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_citation_source_or_freshness: false,
        implies_apply_capable_action_or_hidden_authority: false,
        invents_alternate_state_label: false,
        drifts_prose_from_cited_source_truth: false,
    }
}

/// Builds a safe explanation banner, deriving the apply disposition, the explain-only claim,
/// and the required notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
#[allow(clippy::too_many_arguments)]
fn banner(
    banner_id: &str,
    banner_label: &str,
    boundary_class: M5ExplanationBoundaryClass,
    apply_state: M5ExplanationApplyState,
    explanation_body: &str,
    explain_versus_do_note: &str,
    offers_do_action: bool,
    citation_kind: DeepLinkKind,
    citation_ref: &str,
    citation_label: &str,
    banner_actions: Vec<ExplanationBannerAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> ExplanationBanner {
    let disclosure = resolve_explanation_apply(apply_state);
    ExplanationBanner {
        component: M5LearningComponentFamily::SafeExplanationBanner,
        banner_id: banner_id.to_owned(),
        banner_label: banner_label.to_owned(),
        boundary_class,
        apply_state,
        apply_disposition: disclosure.apply_disposition,
        claims_explain_only: disclosure.is_explain_only,
        explanation_body: explanation_body.to_owned(),
        explain_versus_do_note: explain_versus_do_note.to_owned(),
        undo_note: if disclosure.needs_undo_note {
            "This change was applied and can be undone; nothing is irreversible".to_owned()
        } else {
            String::new()
        },
        withheld_note: if disclosure.needs_withheld_note {
            "No change was applied; the action was withheld and can be retried".to_owned()
        } else {
            String::new()
        },
        offers_do_action,
        do_disclosure_note: if offers_do_action {
            "This banner also offers to do, behind the ordinary preview / approval model".to_owned()
        } else {
            String::new()
        },
        citation_kind,
        citation_ref: citation_ref.to_owned(),
        citation_label: citation_label.to_owned(),
        banner_actions,
        dispositions,
        downgrade_triggers: banner_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "banner_label",
            "boundary_class",
            "apply_state",
            "apply_disposition",
            "explanation_body",
            "explain_versus_do_note",
        ]),
        source_contract_refs: banner_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_citation_source_or_freshness: false,
        implies_apply_capable_action_or_hidden_authority: false,
        invents_alternate_state_label: false,
        drifts_prose_from_cited_source_truth: false,
    }
}

fn glossary_entries() -> Vec<GlossaryEntry> {
    use DeepLinkKind as Link;
    use GlossaryEntryAction as Action;
    use M5GlossaryCitationState as Citation;
    use M5GlossarySourceClass as Source;
    use M5LearningDisposition as Disp;

    vec![
        // 1. Cited docs, current → cited-current (source-backed, openable citation).
        entry(
            "glossary-preview-apply",
            "Preview then apply",
            Source::CitedDocs,
            Citation::CitationCurrent,
            "A change is shown for review before it touches live state",
            Link::DocsAnchor,
            "docs:concepts/preview-then-apply",
            "Docs: Preview then apply",
            vec![
                Action::ShowDefinition,
                Action::OpenCitation,
                Action::OpenRelatedConcept,
                Action::CopyTerm,
                Action::OpenGlossarySurface,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 2. Cited spec, version-matched → cited-current (source-backed, symbol citation).
        entry(
            "glossary-sandbox",
            "Sandbox",
            Source::CitedSpec,
            Citation::CitationVersioned,
            "An isolated space where practice never touches live state",
            Link::SymbolLocation,
            "symbol:aureline_learning::sandbox::Sandbox",
            "Spec symbol: Sandbox",
            vec![
                Action::ShowDefinition,
                Action::OpenCitation,
                Action::OpenRelatedConcept,
                Action::CopyTerm,
            ],
            vec![Disp::Sandboxed],
        ),
        // 3. Cited help pack, cached → cited-cached (source-backed, file citation).
        entry(
            "glossary-review-thread",
            "Review thread",
            Source::CitedHelpPack,
            Citation::CitationCached,
            "The ordered conversation attached to a review",
            Link::FileLocation,
            "file:help/glossary/review-thread.md",
            "Help pack: Review thread",
            vec![
                Action::ShowDefinition,
                Action::OpenCitation,
                Action::OpenRelatedConcept,
                Action::OpenGlossarySurface,
            ],
            vec![Disp::Cached],
        ),
        // 4. Community note, stale → cited-stale (not source-backed, no open-citation).
        entry(
            "glossary-quiet-hours",
            "Quiet hours",
            Source::CommunityNote,
            Citation::CitationStale,
            "A user-set window when tips and prompts stay silent",
            Link::NoDeepLink,
            "",
            "Community note (uncited)",
            vec![
                Action::ShowDefinition,
                Action::OpenRelatedConcept,
                Action::CopyTerm,
                Action::DismissChip,
            ],
            vec![Disp::LocalOnly],
        ),
        // 5. Uncited draft, offline-unavailable → offline-unavailable (not source-backed).
        entry(
            "glossary-mirror-mode",
            "Mirror mode",
            Source::UncitedDraft,
            Citation::CitationOfflineUnavailable,
            "A read-only offline copy of a session or workspace",
            Link::NoDeepLink,
            "",
            "Draft definition (uncited)",
            vec![
                Action::ShowDefinition,
                Action::OpenRelatedConcept,
                Action::DismissChip,
            ],
            vec![Disp::LocalOnly],
        ),
        // 6. Unknown source, citation missing → uncited (not source-backed).
        entry(
            "glossary-handoff",
            "Handoff",
            Source::UnknownSource,
            Citation::CitationMissing,
            "Passing an in-progress task from one surface to another",
            Link::NoDeepLink,
            "",
            "Unknown source (uncited)",
            vec![
                Action::ShowDefinition,
                Action::OpenRelatedConcept,
                Action::DismissChip,
            ],
            vec![Disp::NotInstalled],
        ),
    ]
}

fn explanation_banners() -> Vec<ExplanationBanner> {
    use DeepLinkKind as Link;
    use ExplanationBannerAction as Action;
    use M5ExplanationApplyState as Apply;
    use M5ExplanationBoundaryClass as Boundary;
    use M5LearningDisposition as Disp;

    vec![
        // 1. Explain only, no apply → explain-only (no do action offered).
        banner(
            "banner-why-suggested",
            "Why this result is suggested",
            Boundary::ExplainOnly,
            Apply::NoApply,
            "This result ranks first because it matches the most recent review comment",
            "This banner explains only; it never applies a change",
            false,
            Link::DocsAnchor,
            "docs:ai/why-suggested",
            "Docs: Why a result is suggested",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::OpenRelatedConcept,
                Action::DismissBanner,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 2. Explain then offer do, preview available → preview-offered (offers do).
        banner(
            "banner-what-term-means",
            "What this term means",
            Boundary::ExplainThenOfferDo,
            Apply::PreviewAvailable,
            "This term names the sandbox; you can preview a practice change in it",
            "This banner explains, then offers a preview before any change",
            true,
            Link::SymbolLocation,
            "symbol:aureline_learning::sandbox::Sandbox",
            "Spec symbol: Sandbox",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::OpenRelatedConcept,
                Action::PreviewChange,
                Action::DismissBanner,
            ],
            vec![Disp::LearningOn],
        ),
        // 3. Preview required, approval pending → approval-pending (offers do).
        banner(
            "banner-preview-required",
            "This change needs a preview first",
            Boundary::PreviewRequired,
            Apply::ApprovalPending,
            "The suggested edit is explained here; a preview is required before it applies",
            "This banner requires a preview and approval before any change",
            true,
            Link::FileLocation,
            "file:help/ai/preview-required.md",
            "Help: Preview required",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::PreviewChange,
                Action::RequestApproval,
                Action::DismissBanner,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 4. Approval required, applied with undo → applied-reversible (offers do, undo note).
        banner(
            "banner-approval-required",
            "This change was approved and applied",
            Boundary::ApprovalRequired,
            Apply::AppliedWithUndo,
            "You approved the explained change; it applied and remains reversible with undo",
            "This banner applied only after approval and stays undoable",
            true,
            Link::CommandReference,
            "command:explanation.undo",
            "Command: undo the applied change",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::RequestApproval,
                Action::DismissBanner,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 5. Sandboxed only, blocked apply → apply-withheld (offers sandbox preview, withheld).
        banner(
            "banner-sandboxed-only",
            "This action is sandboxed",
            Boundary::SandboxedOnly,
            Apply::BlockedApply,
            "The explained action only runs in the sandbox; live apply was blocked",
            "This banner offers a sandbox preview only; live apply stays blocked",
            true,
            Link::SymbolLocation,
            "symbol:aureline_learning::sandbox::run_sandboxed",
            "Symbol: run_sandboxed",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::PreviewChange,
                Action::DismissBanner,
            ],
            vec![Disp::Sandboxed],
        ),
        // 6. No hidden apply, mutation declined → apply-withheld (offers do, withheld).
        banner(
            "banner-no-hidden-apply",
            "Nothing is applied without your approval",
            Boundary::NoHiddenApply,
            Apply::MutationDeclined,
            "You declined the explained change; nothing was applied by a hidden authority",
            "This banner never applies without the ordinary preview / approval model",
            true,
            Link::DocsAnchor,
            "docs:ai/no-hidden-apply",
            "Docs: No hidden apply",
            vec![
                Action::ShowExplanation,
                Action::OpenCitation,
                Action::RequestApproval,
                Action::DismissBanner,
            ],
            vec![Disp::NoHiddenApply],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::GlossaryCitationSevered,
        M5LearningDowngradeTrigger::ExplanationApplyBoundaryUnstated,
        M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
        M5LearningDowngradeTrigger::CachedStateHidden,
        M5LearningDowngradeTrigger::NotInstalledStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn learnability_review() -> GlossaryExplanationReview {
    GlossaryExplanationReview {
        glossary_cites_source_truth: true,
        glossary_names_term_meaning: true,
        glossary_shows_source_class_and_freshness: true,
        glossary_offers_open_related_concept: true,
        citation_freshness_derived_never_asserted: true,
        uncited_prose_never_shown_as_cited: true,
        explanation_states_explain_versus_do_boundary: true,
        explanation_cites_grounding_source: true,
        explanation_never_implies_apply_capable_action: true,
        apply_disposition_derived_never_asserted: true,
        explain_only_banner_offers_no_do_action: true,
        any_apply_uses_preview_approval_or_undo: true,
        no_control_widens_trust_or_mutating_authority: true,
        educational_surfaces_distinct_from_apply_capable_actions: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
    }
}

fn consumer_projection() -> GlossaryExplanationConsumerProjection {
    GlossaryExplanationConsumerProjection {
        glossary_ui_reads_single_source: true,
        explanation_ui_reads_single_source: true,
        citation_source_and_freshness_visible_before_trust: true,
        explain_versus_do_boundary_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> GlossaryExplanationProofFreshness {
    GlossaryExplanationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical glossary-chip-card / safe-explanation-banner controls packet.
pub fn seeded_glossary_chip_card_safe_explanation_banner_controls(
) -> GlossaryChipCardSafeExplanationBannerControlsPacket {
    GlossaryChipCardSafeExplanationBannerControlsPacket::new(
        GlossaryChipCardSafeExplanationBannerControlsPacketInput {
            packet_id: GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_PACKET_ID.to_owned(),
            surface_label:
                "M5 glossary chips/cards and safe explanation banners: term meaning with cited file/symbol/docs source truth, freshness and source-class labels, open-related-concept actions, and an explicit explain-versus-do boundary so an explanation never implies an apply-capable action or hidden authority across claimed learning surfaces"
                    .to_owned(),
            glossary_entries: glossary_entries(),
            explanation_banners: explanation_banners(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
            learnability_review: learnability_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights an uncited glossary chip that must never read as cited. Every
/// citation class, source class, and citation state stays covered so the fixture validates on
/// its own.
pub fn seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited(
) -> GlossaryChipCardSafeExplanationBannerControlsPacket {
    let mut packet = seeded_glossary_chip_card_safe_explanation_banner_controls();
    packet.packet_id =
        "m5-glossary-chip-card-safe-explanation-banner-controls:fixture:glossary-chip-card-uncited"
            .to_owned();
    packet.surface_label =
        "M5 glossary chips/cards: an uncited definition never reads as cited source truth"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights an explain-only banner that offers no do action and never
/// implies an apply-capable action. Every apply disposition, boundary class, and apply state
/// stays covered so the fixture validates on its own.
pub fn seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only(
) -> GlossaryChipCardSafeExplanationBannerControlsPacket {
    let mut packet = seeded_glossary_chip_card_safe_explanation_banner_controls();
    packet.packet_id =
        "m5-glossary-chip-card-safe-explanation-banner-controls:fixture:safe-explanation-banner-explain-only"
            .to_owned();
    packet.surface_label =
        "M5 safe explanation banners: an explain-only banner never implies an apply-capable action"
            .to_owned();
    packet
}
