//! Canonical seed builders for the learning educational-AI continuity controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical learning educational-AI continuity packet.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_PACKET_ID: &str =
    "m5-learning-educational-ai-continuity-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The per-component canonical schema ref for a governed family.
fn family_component_schema_ref(family: M5LearningComponentFamily) -> &'static str {
    use M5LearningComponentFamily as Family;
    match family {
        Family::LearningModeToggle => M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        Family::TipCard => M5_TIP_CARD_SCHEMA_REF,
        Family::GuidedExerciseStep => M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        Family::GlossaryChipOrCard => M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        Family::SafeExplanationBanner => M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        Family::ProgressMarker => M5_PROGRESS_MARKER_SCHEMA_REF,
    }
}

/// The per-component source refs: the component's own canonical schema plus the matrix schema.
fn row_source_refs(family: M5LearningComponentFamily) -> Vec<String> {
    strings(&[
        family_component_schema_ref(family),
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

/// Builds a degraded learning component, deriving the trust class, the live claim, the
/// next-safe-action, the apply disposition, and the required notes from the honest inputs so the
/// seed is always self-consistent with the resolvers.
#[allow(clippy::too_many_arguments)]
fn degraded_component(
    component_family: M5LearningComponentFamily,
    component_id: &str,
    component_title: &str,
    subject_kind: LearningSubjectKind,
    subject_label: &str,
    cited_source_ref: &str,
    stable_component_ref: &str,
    learning_scope: M5LearningModeScope,
    scope_label: &str,
    continuity_state: LearningContinuityState,
    source_kind: LearningSourceKind,
    source_label: &str,
    apply_posture: EducationalApplyPosture,
    dispositions: Vec<M5LearningDisposition>,
    safe_verbs: Vec<LearningSafeVerb>,
) -> LearningDegradedComponentRow {
    let disclosure = resolve_continuity(continuity_state);
    let apply = resolve_apply(apply_posture);
    LearningDegradedComponentRow {
        component_family,
        component_id: component_id.to_owned(),
        component_title: component_title.to_owned(),
        subject_kind,
        subject_label: subject_label.to_owned(),
        subject_summary_note: format!("Last-known summary preserved for {subject_label}"),
        cited_source_ref: cited_source_ref.to_owned(),
        stable_component_ref: stable_component_ref.to_owned(),
        learning_scope,
        scope_label: scope_label.to_owned(),
        continuity_state,
        trust_class: disclosure.trust_class,
        claims_live_enrichment: disclosure.is_live,
        continuity_note: format!(
            "Scoped to {}; continuity {}",
            learning_scope.as_str(),
            continuity_state.as_str()
        ),
        state_explanation_note: if disclosure.needs_continuity_explanation {
            format!(
                "State {}: shown as {}, not live",
                continuity_state.as_str(),
                disclosure.trust_class.as_str()
            )
        } else {
            String::new()
        },
        next_safe_action: disclosure.next_safe_action,
        next_safe_action_note: format!(
            "Next safe action: {}",
            disclosure.next_safe_action.as_str().replace('_', " ")
        ),
        source_fallback_note: if disclosure.needs_source_fallback {
            format!(
                "Source fallback: open the cited {} that is still reachable",
                source_kind.as_str()
            )
        } else {
            String::new()
        },
        source_kind,
        source_label: source_label.to_owned(),
        apply_posture,
        apply_disposition: apply.apply_disposition,
        offers_live_mutation: apply.offers_live_mutation,
        apply_boundary_note: format!(
            "Explain versus do: {}",
            apply.apply_disposition.as_str().replace('_', " ")
        ),
        safe_verbs,
        dispositions,
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "component_title",
            "subject_label",
            "subject_summary_note",
            "stable_component_ref",
            "learning_scope",
            "continuity_state",
            "trust_class",
            "next_safe_action",
            "apply_disposition",
            "source_kind",
        ]),
        source_contract_refs: row_source_refs(component_family),
        masks_privacy_or_offline_state: false,
        hides_citation_source: false,
        invents_alternate_state_label: false,
        implies_hidden_apply_or_mutation: false,
        mutates_live_without_preview_approval: false,
    }
}

fn degraded_components() -> Vec<LearningDegradedComponentRow> {
    use EducationalApplyPosture as Apply;
    use LearningContinuityState as State;
    use LearningSafeVerb as Verb;
    use LearningSourceKind as Source;
    use LearningSubjectKind as Subject;
    use M5LearningComponentFamily as Family;
    use M5LearningDisposition as Disp;
    use M5LearningModeScope as Scope;

    vec![
        // 1. Learning-mode toggle, live: full enrichment, explain-only, proceed in learning.
        degraded_component(
            Family::LearningModeToggle,
            "component-live-learning-mode-toggle",
            "Learning mode (live)",
            Subject::Command,
            "learning-mode toggle",
            "command:aureline.learning.toggle",
            "learning_mode_toggle:global",
            Scope::Workspace,
            "This workspace",
            State::Live,
            Source::CommandReference,
            "Open the learning-mode command",
            Apply::ExplainOnly,
            vec![Disp::LearningOn, Disp::NoHiddenApply],
            vec![
                Verb::Explain,
                Verb::OpenSource,
                Verb::Refresh,
                Verb::Dismiss,
            ],
        ),
        // 2. Tip card, cached: reduced trust, explain-only, refresh for the latest, docs reachable.
        degraded_component(
            Family::TipCard,
            "component-cached-tip-card",
            "Tip: keyboard palette (cached)",
            Subject::DocsTopic,
            "keyboard palette tip",
            "docs:help/keyboard-palette",
            "tip_card:keyboard-palette",
            Scope::Surface,
            "This surface",
            State::Cached,
            Source::DocsPage,
            "Open the cached docs page",
            Apply::ExplainOnly,
            vec![Disp::Cached, Disp::NoHiddenApply],
            vec![
                Verb::Explain,
                Verb::OpenSource,
                Verb::Refresh,
                Verb::CopyReference,
                Verb::Dismiss,
            ],
        ),
        // 3. Guided exercise step, local-only: sandboxed practice, continue local-only.
        degraded_component(
            Family::GuidedExerciseStep,
            "component-local-only-guided-exercise-step",
            "Exercise: rename a symbol (local-only)",
            Subject::ExerciseTask,
            "rename-symbol exercise",
            "sandbox:exercise/rename-symbol",
            "guided_exercise_step:rename-symbol",
            Scope::Session,
            "This session",
            State::LocalOnly,
            Source::SandboxTarget,
            "Open the local sandbox target",
            Apply::SandboxedPractice,
            vec![Disp::LocalOnly, Disp::Sandboxed, Disp::NoHiddenApply],
            vec![
                Verb::Explain,
                Verb::OpenSource,
                Verb::PracticeInSandbox,
                Verb::CopyReference,
                Verb::Dismiss,
            ],
        ),
        // 4. Glossary chip/card, offline: held stale, explain-only, cited file still reachable.
        degraded_component(
            Family::GlossaryChipOrCard,
            "component-offline-glossary-chip",
            "Glossary: change object (offline)",
            Subject::Concept,
            "change-object concept",
            "file:crates/aureline-change-objects/src/lib.rs",
            "glossary_chip_or_card:change-object",
            Scope::FeatureFamily,
            "Change-objects feature family",
            State::Offline,
            Source::FileLocation,
            "Open the cited file location",
            Apply::ExplainOnly,
            vec![Disp::Cached, Disp::NoHiddenApply],
            vec![
                Verb::Explain,
                Verb::OpenSource,
                Verb::CopyReference,
                Verb::Dismiss,
            ],
        ),
        // 5. Safe explanation banner, stale-pack: preview-then-approve do action, update the pack.
        degraded_component(
            Family::SafeExplanationBanner,
            "component-stale-pack-safe-explanation-banner",
            "Explanation: apply the suggested fix (stale pack)",
            Subject::FileOrSymbol,
            "suggested-fix explanation",
            "symbol:aureline_review::apply_suggested_fix",
            "safe_explanation_banner:suggested-fix",
            Scope::FeatureFamily,
            "Review feature family",
            State::StalePack,
            Source::SymbolLocation,
            "Open the cited symbol location",
            Apply::PreviewThenApprove,
            vec![Disp::NoHiddenApply],
            vec![
                Verb::Explain,
                Verb::OpenSource,
                Verb::Refresh,
                Verb::CopyReference,
                Verb::Dismiss,
            ],
        ),
        // 6. Glossary chip/card, citation-unavailable: degrade to explicit uncited state, apply
        //    blocked, no cited source to open.
        degraded_component(
            Family::GlossaryChipOrCard,
            "component-citation-unavailable-glossary-card",
            "Glossary: build farm (citation unavailable)",
            Subject::Concept,
            "build-farm concept",
            "",
            "glossary_chip_or_card:build-farm",
            Scope::FeatureFamily,
            "Build-farm feature family",
            State::CitationUnavailable,
            Source::NoSource,
            "No cited source: citation is unavailable",
            Apply::ApplyBlocked,
            vec![Disp::NoHiddenApply],
            vec![Verb::Explain, Verb::CopyReference, Verb::Dismiss],
        ),
        // 7. Progress marker, not-installed: the progress pack was never installed, apply blocked,
        //    nothing to open — install to enable.
        degraded_component(
            Family::ProgressMarker,
            "component-not-installed-progress-marker",
            "Progress: guided tour (not installed)",
            Subject::ProgressRecord,
            "guided-tour progress",
            "",
            "progress_marker:guided-tour",
            Scope::Unavailable,
            "Not available on this build",
            State::NotInstalled,
            Source::NoSource,
            "No cited source: pack is not installed",
            Apply::ApplyBlocked,
            vec![Disp::NotInstalled, Disp::LocalOnly, Disp::NoHiddenApply],
            vec![Verb::Explain, Verb::CopyReference, Verb::Dismiss],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    M5LearningDowngradeTrigger::ALL.to_vec()
}

fn glance_review() -> LearningEducationalAiContinuityGlanceReview {
    LearningEducationalAiContinuityGlanceReview {
        every_component_names_subject_summary_and_identity: true,
        every_component_states_its_continuity: true,
        every_component_states_next_safe_action: true,
        degraded_state_is_explicit_before_action: true,
        live_cached_local_only_not_installed_distinguishable: true,
        cached_or_stale_never_shown_as_live: true,
        trust_class_derived_never_asserted: true,
        learning_stays_useful_offline: true,
        educational_ai_never_mutates_live_without_preview_approval: true,
        no_component_implies_hidden_apply: true,
        not_installed_or_uncited_state_stops_source_routing: true,
        every_reachable_state_names_a_cited_or_cached_source: true,
        subject_identity_always_explicit: true,
        learning_scope_always_explicit: true,
        no_component_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> LearningEducationalAiContinuityConsumerProjection {
    LearningEducationalAiContinuityConsumerProjection {
        learning_surfaces_read_single_source: true,
        help_surfaces_read_single_source: true,
        first_glance_names_state_scope_and_citation: true,
        next_safe_action_visible_before_action: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> LearningEducationalAiContinuityProofFreshness {
    LearningEducationalAiContinuityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_REF,
        LEARNING_EDUCATIONAL_AI_CONTINUITY_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ])
}

/// Builds the canonical learning educational-AI continuity controls packet.
pub fn seeded_learning_educational_ai_continuity_controls() -> LearningEducationalAiContinuityPacket
{
    LearningEducationalAiContinuityPacket::new(LearningEducationalAiContinuityPacketInput {
        packet_id: LEARNING_EDUCATIONAL_AI_CONTINUITY_PACKET_ID.to_owned(),
        surface_label:
            "M5 learning educational-AI continuity: live, cached, local-only, offline, stale-pack, citation-unavailable, and not-installed states with subject-first continuity, derived trust and next-safe-action, no-hidden-apply preview/approval boundaries, safe explain verbs, and a cited source fallback while learning stays useful offline"
                .to_owned(),
        components: degraded_components(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        glance_review: glance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a glossary card whose citation is unavailable, so it must degrade
/// to an explicit uncited state and stop routing to a cited source it does not have. Every
/// continuity state, component family, and apply posture stays covered so the fixture validates on
/// its own.
pub fn seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary(
) -> LearningEducationalAiContinuityPacket {
    let mut packet = seeded_learning_educational_ai_continuity_controls();
    packet.packet_id =
        "m5-learning-educational-ai-continuity-controls:fixture:citation-unavailable-glossary"
            .to_owned();
    packet.surface_label =
        "M5 learning components: a glossary card whose citation is unavailable degrades to an explicit uncited state and stops routing to a cited source it does not have"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a not-installed progress marker that must name its install action
/// and stop routing instead of opening a pack that is not installed, while still explaining
/// offline. Every continuity state, component family, and apply posture stays covered so the
/// fixture validates on its own.
pub fn seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker(
) -> LearningEducationalAiContinuityPacket {
    let mut packet = seeded_learning_educational_ai_continuity_controls();
    packet.packet_id =
        "m5-learning-educational-ai-continuity-controls:fixture:not-installed-progress-marker"
            .to_owned();
    packet.surface_label =
        "M5 learning components: a not-installed progress marker names its install action and stops routing instead of opening a pack that is not installed"
            .to_owned();
    packet
}
