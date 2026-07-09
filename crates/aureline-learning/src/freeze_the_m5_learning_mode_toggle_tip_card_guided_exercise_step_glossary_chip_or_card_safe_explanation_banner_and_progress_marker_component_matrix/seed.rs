//! Canonical seed builders for the frozen M5 learning-component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical learning-component matrix.
pub const M5_LEARNING_COMPONENT_MATRIX_PACKET_ID: &str = "m5-learning-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5LearningRequiredLabel> {
    M5LearningRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5LearningRequiredLabel]) -> Vec<M5LearningRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5LearningComponentFamily,
    qualification: M5LearningQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5LearningComponentRow {
    M5LearningComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        learning_mode_states: vec![],
        learning_mode_scopes: vec![],
        tip_trigger_classes: vec![],
        tip_dismissal_states: vec![],
        exercise_step_states: vec![],
        exercise_validation_modes: vec![],
        glossary_source_classes: vec![],
        glossary_citation_states: vec![],
        explanation_boundary_classes: vec![],
        explanation_apply_states: vec![],
        progress_ownership_classes: vec![],
        progress_states: vec![],
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5LearningConsumerSurface::OnboardingUi,
            M5LearningConsumerSurface::SupportExport,
            M5LearningConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5LearningDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_privacy_or_offline_state: false,
        hides_citation_source: false,
        implies_hidden_apply_or_mutation: false,
        invents_alternate_state_label: false,
    }
}

fn component_rows() -> Vec<M5LearningComponentRow> {
    use M5ExerciseStepState as ES;
    use M5ExerciseValidationMode as EV;
    use M5ExplanationApplyState as EA;
    use M5ExplanationBoundaryClass as EB;
    use M5GlossaryCitationState as GC;
    use M5GlossarySourceClass as GS;
    use M5LearningComponentFamily as F;
    use M5LearningConsumerSurface as C;
    use M5LearningDisposition as DI;
    use M5LearningDowngradeTrigger as D;
    use M5LearningModeScope as MS;
    use M5LearningModeState as MST;
    use M5LearningQualificationClass as Q;
    use M5LearningRequiredLabel as L;
    use M5ProgressOwnershipClass as PO;
    use M5ProgressState as PS;
    use M5TipDismissalState as TD;
    use M5TipTriggerClass as TT;

    let mut rows = Vec::new();

    // 1. Learning-mode toggle.
    let mut row = base_row(
        F::LearningModeToggle,
        Q::Stable,
        "Learning-mode toggle owner",
        "One learning-mode-toggle model naming whether learning mode is on, off, paused, per feature family, sandboxed-only, or ended and how widely it applies (global, per workspace, per feature family, per session, per surface, or unavailable), so learning stays opt-in, never traps an expert in a tutorial, and its cached, local-only, and not-installed states stay visible",
        "evidence:m5-learning-mode-toggle-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_LEARNING_MODE_TOGGLE_SCHEMA_REF],
    );
    row.dispositions = vec![DI::LearningOn, DI::Paused, DI::LocalOnly, DI::NotInstalled];
    row.learning_mode_states = MST::ALL.to_vec();
    row.learning_mode_scopes = MS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProgressOwnershipAndPrivacy]);
    row.consumer_surfaces = vec![
        C::OnboardingUi,
        C::LearningPanelUi,
        C::TourOverlayUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LearningModeStateUnstated,
        D::CachedStateHidden,
        D::NotInstalledStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Tip card.
    let mut row = base_row(
        F::TipCard,
        Q::Stable,
        "Tip card owner",
        "One tip-card model naming why a teaching tip appears (first encounter, feature discovery, error recovery, mode change, idle hint, or contextual follow-up), the cited source behind it, and how it can be dismissed, so teaching stays contextual, dismissible, and citation-backed and never blocks the user or drifts from cited source truth",
        "evidence:m5-tip-card-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_TIP_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::LearningOn, DI::Cached];
    row.tip_trigger_classes = TT::ALL.to_vec();
    row.tip_dismissal_states = TD::ALL.to_vec();
    row.required_labels = labels_with(&[L::CitationSource]);
    row.consumer_surfaces = vec![
        C::OnboardingUi,
        C::HelpPanelUi,
        C::TourOverlayUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TipCommandBindingUnstated,
        D::CachedStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Guided exercise step.
    let mut row = base_row(
        F::GuidedExerciseStep,
        Q::Stable,
        "Guided exercise step owner",
        "One guided-exercise-step model naming the state of a practice step (not started, active, passed, failed but retryable, replayable, or sandboxed) and how it validates the learner's work (command-backed, sandboxed practice, read-only walkthrough, checkpoint-gated, self-paced, or no hidden apply), so an exercise is replayable, keeps explain and do separate, and never mutates live state without the ordinary preview and approval model",
        "evidence:m5-guided-exercise-step-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_GUIDED_EXERCISE_STEP_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Replayable, DI::Sandboxed, DI::NoHiddenApply];
    row.exercise_step_states = ES::ALL.to_vec();
    row.exercise_validation_modes = EV::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExplainVersusDoBoundary]);
    row.consumer_surfaces = vec![
        C::ExerciseUi,
        C::TourOverlayUi,
        C::LearningPanelUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExerciseStepStateUnstated,
        D::SandboxBoundaryUnstated,
        D::ExplanationApplyBoundaryUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Glossary chip or card.
    let mut row = base_row(
        F::GlossaryChipOrCard,
        Q::Stable,
        "Glossary chip or card owner",
        "One glossary-chip-or-card model naming where a definition comes from (cited docs, cited spec, cited help pack, a community note, an uncited draft, or an unknown source) and how current its citation is (current, version-matched, stale, cached, offline-unavailable, or missing), so glossary prose never drifts away from cited source truth and a definition never severs or hides its canonical citation",
        "evidence:m5-glossary-chip-card-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_GLOSSARY_CHIP_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Cached, DI::LocalOnly];
    row.glossary_source_classes = GS::ALL.to_vec();
    row.glossary_citation_states = GC::ALL.to_vec();
    row.required_labels = labels_with(&[L::CitationSource]);
    row.consumer_surfaces = vec![
        C::GlossaryUi,
        C::HelpPanelUi,
        C::CliHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GlossaryCitationSevered,
        D::CachedStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Safe explanation banner.
    let mut row = base_row(
        F::SafeExplanationBanner,
        Q::Stable,
        "Safe explanation banner owner",
        "One safe-explanation-banner model naming how an explanation separates explain from do (explain only, explain then offer to do, preview required, approval required, sandboxed only, or no hidden apply) and what it will actually do (apply nothing, preview available, approval pending, applied with undo, blocked apply, or mutation declined), so an educational explanation never widens mutating authority and applies nothing without the same preview and approval model as ordinary work",
        "evidence:m5-safe-explanation-banner-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::NoHiddenApply, DI::Sandboxed];
    row.explanation_boundary_classes = EB::ALL.to_vec();
    row.explanation_apply_states = EA::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExplainVersusDoBoundary]);
    row.consumer_surfaces = vec![
        C::HelpPanelUi,
        C::ExerciseUi,
        C::CliHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExplanationApplyBoundaryUnstated,
        D::SandboxBoundaryUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Progress marker.
    let mut row = base_row(
        F::ProgressMarker,
        Q::Stable,
        "Progress marker owner",
        "One progress-marker model naming who owns a learner's progress (local-only, user-owned and synced by choice, exported by choice, workspace-shared by choice, a cached snapshot, or not installed) and where that progress stands (not started, in progress, completed, paused, reset, or offline / local), so progress stays user-owned and default-local unless a supported sync or export path is explicitly chosen and an offline or local-only state is never left implicit",
        "evidence:m5-progress-marker-parity:001",
        &[M5_LEARNING_COMPONENT_SCHEMA_REF, M5_PROGRESS_MARKER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::LocalOnly, DI::Cached, DI::NotInstalled, DI::Paused];
    row.progress_ownership_classes = PO::ALL.to_vec();
    row.progress_states = PS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProgressOwnershipAndPrivacy]);
    row.consumer_surfaces = vec![
        C::LearningPanelUi,
        C::OnboardingUi,
        C::HelpPanelUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ProgressOwnershipUnstated,
        D::OfflineOrLocalOnlyStateHidden,
        D::CachedStateHidden,
        D::NotInstalledStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5LearningComponentGovernanceReview {
    M5LearningComponentGovernanceReview {
        toggle_shows_learning_state_and_scope: true,
        tip_card_shows_command_binding_and_dismissal: true,
        exercise_step_shows_state_and_no_hidden_apply: true,
        glossary_shows_cited_source_and_citation_state: true,
        banner_shows_explain_versus_do_and_no_hidden_apply: true,
        progress_marker_shows_ownership_and_privacy: true,
        no_surface_invents_alternate_state_label: true,
        learning_stays_opt_in: true,
        explain_and_do_stay_separate: true,
        no_component_widens_trust_or_mutating_authority: true,
        progress_user_owned_by_default: true,
        cached_offline_local_only_state_always_visible: true,
        sandboxed_state_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5LearningComponentConsumerProjection {
    M5LearningComponentConsumerProjection {
        onboarding_surfaces_consume_toggle_and_tip_vocabulary: true,
        guided_learning_surfaces_consume_exercise_vocabulary: true,
        glossary_surfaces_consume_citation_vocabulary: true,
        explanation_surfaces_consume_apply_boundary_vocabulary: true,
        progress_surfaces_consume_ownership_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5LearningComponentProofFreshness {
    M5LearningComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LearningComponentReleasePosture {
    M5LearningComponentReleasePosture {
        proof_packet_ref: M5_LEARNING_COMPONENT_ARTIFACT_REF.to_owned(),
        learning_component_audit_ref: M5_LEARNING_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_TIP_CARD_SCHEMA_REF,
        M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 learning-component matrix packet.
pub fn seeded_m5_learning_component_matrix() -> M5LearningComponentMatrixPacket {
    M5LearningComponentMatrixPacket::new(M5LearningComponentMatrixPacketInput {
        packet_id: M5_LEARNING_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 learning-mode-toggle, tip-card, guided-exercise-step, glossary-chip-or-card, safe-explanation-banner, and progress-marker component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5LearningComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the learning-mode toggle is held at Beta because a slice of the
/// per-feature-family scope does not yet round-trip across every learnability surface; every
/// component stays visible.
pub fn seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed(
) -> M5LearningComponentMatrixPacket {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.packet_id = "m5-learning-components:learning-mode-toggle-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5LearningComponentFamily::LearningModeToggle)
        .expect("learning-mode-toggle row present");
    row.qualification = M5LearningQualificationClass::Beta;
    packet
}

/// Narrowed variant: the progress marker is narrowed to Preview pending user-owned,
/// default-local export parity proof across every surface; every component stays visible.
pub fn seeded_m5_learning_component_matrix_progress_marker_preview_narrowed(
) -> M5LearningComponentMatrixPacket {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.packet_id = "m5-learning-components:progress-marker-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5LearningComponentFamily::ProgressMarker)
        .expect("progress-marker row present");
    row.qualification = M5LearningQualificationClass::Preview;
    packet
}
