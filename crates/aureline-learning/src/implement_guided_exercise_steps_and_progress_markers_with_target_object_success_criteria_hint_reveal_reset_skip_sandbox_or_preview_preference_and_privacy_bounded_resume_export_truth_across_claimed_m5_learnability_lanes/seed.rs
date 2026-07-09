//! Canonical seed builders for the guided-exercise-step / progress-marker controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical guided-exercise-step / progress-marker packet.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_PACKET_ID: &str =
    "m5-guided-exercise-step-progress-marker-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn step_source_refs() -> Vec<String> {
    strings(&[
        M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

fn marker_source_refs() -> Vec<String> {
    strings(&[
        M5_PROGRESS_MARKER_SCHEMA_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

fn step_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::ExerciseStepStateUnstated,
        M5LearningDowngradeTrigger::SandboxBoundaryUnstated,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn marker_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::ProgressOwnershipUnstated,
        M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

/// Builds a guided exercise step, deriving the progress class, the completed claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn step(
    step_id: &str,
    step_label: &str,
    step_state: M5ExerciseStepState,
    validation_mode: M5ExerciseValidationMode,
    target_kind: DeepLinkKind,
    target_ref: &str,
    target_object_label: &str,
    success_criteria: &str,
    mutates_state: bool,
    mutation_preference: LessonMutationPreference,
    step_actions: Vec<ExerciseStepAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> GuidedExerciseStep {
    let disclosure = resolve_exercise_progress(step_state);
    GuidedExerciseStep {
        component: M5LearningComponentFamily::GuidedExerciseStep,
        step_id: step_id.to_owned(),
        step_label: step_label.to_owned(),
        step_state,
        validation_mode,
        progress_class: disclosure.progress_class,
        claims_completed: disclosure.is_completed,
        retry_note: if disclosure.needs_retry_note {
            "This step did not pass yet; retry it or reset — it never counts as done".to_owned()
        } else {
            String::new()
        },
        sandbox_note: if disclosure.needs_sandbox_note {
            "Practicing in a sandbox; nothing here touches live state".to_owned()
        } else {
            String::new()
        },
        target_kind,
        target_ref: target_ref.to_owned(),
        target_object_label: target_object_label.to_owned(),
        success_criteria: success_criteria.to_owned(),
        mutates_state,
        mutation_preference,
        step_actions,
        dispositions,
        downgrade_triggers: step_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "step_label",
            "step_state",
            "validation_mode",
            "progress_class",
            "target_object_label",
            "success_criteria",
        ]),
        source_contract_refs: step_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_success_criteria_or_target_identity: false,
        implies_hidden_apply_or_mutation: false,
        invents_alternate_state_label: false,
        traps_progress_without_resume_reset_export: false,
    }
}

/// Builds a progress marker, deriving the standing, the complete claim, and the required
/// notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn marker(
    marker_id: &str,
    marker_label: &str,
    progress_state: M5ProgressState,
    ownership: M5ProgressOwnershipClass,
    completed_units: u32,
    total_units: u32,
    shares_beyond_local_scope: bool,
    resume_export_kind: DeepLinkKind,
    resume_export_ref: &str,
    marker_actions: Vec<ProgressMarkerAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> ProgressMarker {
    let disclosure = resolve_progress_standing(progress_state);
    ProgressMarker {
        component: M5LearningComponentFamily::ProgressMarker,
        marker_id: marker_id.to_owned(),
        marker_label: marker_label.to_owned(),
        progress_state,
        ownership,
        standing_class: disclosure.standing_class,
        claims_complete: disclosure.is_complete,
        completed_units,
        total_units,
        interrupted_note: if disclosure.needs_interrupted_note {
            "Progress is interrupted; resume, reset, or export it — it is never lost".to_owned()
        } else {
            String::new()
        },
        offline_note: if disclosure.needs_offline_note {
            "Offline / local view; this is a cached snapshot of your own progress".to_owned()
        } else {
            String::new()
        },
        ownership_and_privacy_note: format!(
            "Progress is {}; user-owned and default-local unless you choose to sync or export",
            ownership.as_str()
        ),
        shares_beyond_local_scope,
        sharing_disclosure_note: if shares_beyond_local_scope {
            format!(
                "You chose to share progress ({}); this leaves the default-local scope",
                ownership.as_str()
            )
        } else {
            String::new()
        },
        resume_export_kind,
        resume_export_ref: resume_export_ref.to_owned(),
        marker_actions,
        dispositions,
        downgrade_triggers: marker_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "marker_label",
            "progress_state",
            "ownership",
            "standing_class",
            "completed_units",
            "total_units",
        ]),
        source_contract_refs: marker_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_success_criteria_or_target_identity: false,
        implies_hidden_apply_or_mutation: false,
        invents_alternate_state_label: false,
        traps_progress_without_resume_reset_export: false,
    }
}

fn exercise_steps() -> Vec<GuidedExerciseStep> {
    use DeepLinkKind as Link;
    use ExerciseStepAction as Action;
    use LessonMutationPreference as Mutation;
    use M5ExerciseStepState as State;
    use M5ExerciseValidationMode as Mode;
    use M5LearningDisposition as Disp;

    vec![
        // 1. Not started, command-backed → pending.
        step(
            "step-open-review",
            "Open the review this lesson works on",
            State::NotStarted,
            Mode::CommandBacked,
            Link::FileLocation,
            "file:src/review/open.rs",
            "The review file src/review/open.rs",
            "Success: the review file is open on screen and focused",
            false,
            Mutation::NoMutationNeeded,
            vec![
                Action::ShowHint,
                Action::ResetStep,
                Action::SkipStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::LocalOnly],
        ),
        // 2. Active, sandboxed practice → in progress (mutates, sandboxed).
        step(
            "step-edit-in-sandbox",
            "Make the requested change in the sandbox",
            State::Active,
            Mode::SandboxedPractice,
            Link::SurfaceLocation,
            "surface:editor.sandbox-panel",
            "The sandbox editor panel",
            "Success: the sandbox diff shows the requested edit applied",
            true,
            Mutation::SandboxPractice,
            vec![
                Action::ShowHint,
                Action::RevealSolution,
                Action::ResetStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::Sandboxed],
        ),
        // 3. Passed, read-only walkthrough → completed.
        step(
            "step-approve-review",
            "Approve the review to finish the walkthrough",
            State::Passed,
            Mode::ReadOnlyWalkthrough,
            Link::CommandReference,
            "command:review.approve",
            "The review.approve command",
            "Success: the review shows an approved status",
            false,
            Mutation::ReadOnlyWalkthrough,
            vec![
                Action::ResetStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 4. Failed but retryable, checkpoint-gated → retryable (mutates, previewed).
        step(
            "step-fix-setting",
            "Fix the setting the checkpoint expects",
            State::FailedRetryable,
            Mode::CheckpointGated,
            Link::FileLocation,
            "file:config/settings.toml",
            "The settings file config/settings.toml",
            "Success: the checkpoint reports the setting matches the expected value",
            true,
            Mutation::PreviewThenApply,
            vec![
                Action::ShowHint,
                Action::RevealSolution,
                Action::ResetStep,
                Action::SkipStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::Replayable],
        ),
        // 5. Replayable, self-paced → completed.
        step(
            "step-replay-summary",
            "Replay the summary at your own pace",
            State::Replayable,
            Mode::SelfPaced,
            Link::DocsAnchor,
            "docs:exercises/replay-summary",
            "The replay-summary docs section",
            "Success: you have replayed the summary and can restate the key idea",
            false,
            Mutation::NoMutationNeeded,
            vec![
                Action::ResetStep,
                Action::SkipStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::Replayable],
        ),
        // 6. Sandboxed, no hidden apply → sandbox practice (mutates in sandbox).
        step(
            "step-practice-playground",
            "Practice freely in the playground",
            State::Sandboxed,
            Mode::NoHiddenApply,
            Link::SurfaceLocation,
            "surface:sandbox.playground",
            "The sandbox playground surface",
            "Success: you tried the action in the playground; nothing left the sandbox",
            true,
            Mutation::SandboxPractice,
            vec![
                Action::ShowHint,
                Action::ResetStep,
                Action::CheckSuccess,
                Action::OpenTargetObject,
            ],
            vec![Disp::Sandboxed],
        ),
    ]
}

fn progress_markers() -> Vec<ProgressMarker> {
    use DeepLinkKind as Link;
    use M5LearningDisposition as Disp;
    use M5ProgressOwnershipClass as Owner;
    use M5ProgressState as State;
    use ProgressMarkerAction as Action;

    vec![
        // 1. Not started, local-only → unstarted, 0/5.
        marker(
            "marker-onboarding",
            "Onboarding lesson progress",
            State::NotStarted,
            Owner::LocalOnly,
            0,
            5,
            false,
            Link::CommandReference,
            "command:progress.resume",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::OpenResumePoint,
            ],
            vec![Disp::LocalOnly],
        ),
        // 2. In progress, user-owned synced → underway, 2/5, shared by choice.
        marker(
            "marker-review-track",
            "Review track progress",
            State::InProgress,
            Owner::UserOwnedSynced,
            2,
            5,
            true,
            Link::CommandReference,
            "command:progress.sync",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::ShareToWorkspace,
                Action::OpenResumePoint,
            ],
            vec![Disp::LearningOn],
        ),
        // 3. Completed, exported by choice → complete, 5/5, shared by choice.
        marker(
            "marker-exported",
            "Exported lesson record",
            State::Completed,
            Owner::ExportedByChoice,
            5,
            5,
            true,
            Link::FileLocation,
            "file:exports/progress.json",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::ShareToWorkspace,
                Action::OpenResumePoint,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 4. Paused, workspace-shared → interrupted, 3/6, shared by choice.
        marker(
            "marker-workspace",
            "Workspace-shared exercise progress",
            State::Paused,
            Owner::WorkspaceShared,
            3,
            6,
            true,
            Link::SurfaceLocation,
            "surface:workspace.progress-panel",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::ShareToWorkspace,
                Action::OpenResumePoint,
            ],
            vec![Disp::Paused],
        ),
        // 5. Reset, cached snapshot → interrupted, 0/4, local.
        marker(
            "marker-reset",
            "Reset practice progress",
            State::Reset,
            Owner::CachedSnapshot,
            0,
            4,
            false,
            Link::CommandReference,
            "command:progress.resume",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::OpenResumePoint,
            ],
            vec![Disp::Cached],
        ),
        // 6. Offline local, not installed → offline-cached, 1/3, local.
        marker(
            "marker-offline",
            "Offline lesson progress",
            State::OfflineLocal,
            Owner::NotInstalled,
            1,
            3,
            false,
            Link::DocsAnchor,
            "docs:progress/offline-local",
            vec![
                Action::ResumeProgress,
                Action::ResetProgress,
                Action::ExportProgress,
                Action::ViewRemaining,
                Action::OpenResumePoint,
            ],
            vec![Disp::NotInstalled],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::ExerciseStepStateUnstated,
        M5LearningDowngradeTrigger::ProgressOwnershipUnstated,
        M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
        M5LearningDowngradeTrigger::SandboxBoundaryUnstated,
        M5LearningDowngradeTrigger::CachedStateHidden,
        M5LearningDowngradeTrigger::NotInstalledStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn learnability_review() -> GuidedExerciseProgressReview {
    GuidedExerciseProgressReview {
        exercise_identifies_target_object: true,
        exercise_names_observable_success_criteria: true,
        exercise_offers_hint_reveal_reset_skip: true,
        mutating_lesson_uses_sandbox_or_preview: true,
        step_progress_derived_never_asserted: true,
        retryable_never_shown_as_passed: true,
        progress_shows_completed_and_remaining: true,
        progress_offers_resume_reset_export: true,
        completion_derived_never_asserted: true,
        incomplete_never_shown_as_complete: true,
        progress_user_owned_and_default_local: true,
        progress_never_shared_beyond_supported_scope: true,
        no_control_widens_trust_or_mutating_authority: true,
        no_irreversible_trap_without_reset_or_resume: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
    }
}

fn consumer_projection() -> GuidedExerciseProgressConsumerProjection {
    GuidedExerciseProgressConsumerProjection {
        exercise_ui_reads_single_source: true,
        progress_ui_reads_single_source: true,
        target_and_success_visible_before_start: true,
        completed_and_remaining_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> GuidedExerciseProgressProofFreshness {
    GuidedExerciseProgressProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF,
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ])
}

/// Builds the canonical guided-exercise-step / progress-marker controls packet.
pub fn seeded_guided_exercise_step_progress_marker_controls(
) -> GuidedExerciseStepProgressMarkerControlsPacket {
    GuidedExerciseStepProgressMarkerControlsPacket::new(
        GuidedExerciseStepProgressMarkerControlsPacketInput {
            packet_id: GUIDED_EXERCISE_STEP_PROGRESS_MARKER_PACKET_ID.to_owned(),
            surface_label:
                "M5 guided exercise steps and progress markers: target-object identity, observable success criteria, hint/reveal/reset/skip actions, sandbox-or-preview preference for mutating lessons, and privacy-bounded completed/remaining progress with resume/reset/export across claimed learnability lanes"
                    .to_owned(),
            exercise_steps: exercise_steps(),
            progress_markers: progress_markers(),
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

/// Scenario fixture: spotlights a failed-retryable guided exercise step that must never read
/// as passed. Every progress class, exercise step state, and validation mode stays covered so
/// the fixture validates on its own.
pub fn seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable(
) -> GuidedExerciseStepProgressMarkerControlsPacket {
    let mut packet = seeded_guided_exercise_step_progress_marker_controls();
    packet.packet_id =
        "m5-guided-exercise-step-progress-marker-controls:fixture:guided-exercise-step-retryable"
            .to_owned();
    packet.surface_label =
        "M5 guided exercise steps: a failed-retryable step never reads as passed".to_owned();
    packet
}

/// Scenario fixture: spotlights a reset progress marker that must never read as complete and
/// stays resumable / exportable without silently sharing progress. Every standing, progress
/// state, and ownership class stays covered so the fixture validates on its own.
pub fn seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset(
) -> GuidedExerciseStepProgressMarkerControlsPacket {
    let mut packet = seeded_guided_exercise_step_progress_marker_controls();
    packet.packet_id =
        "m5-guided-exercise-step-progress-marker-controls:fixture:progress-marker-reset".to_owned();
    packet.surface_label =
        "M5 progress markers: a reset marker never reads as complete yet stays resumable"
            .to_owned();
    packet
}
