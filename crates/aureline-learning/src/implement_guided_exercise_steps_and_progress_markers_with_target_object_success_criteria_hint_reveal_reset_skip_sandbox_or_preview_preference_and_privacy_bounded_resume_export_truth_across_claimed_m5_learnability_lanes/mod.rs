//! Two reusable M5 learnability controls — the guided exercise step and the progress
//! marker — so a user can learn a structured task without the surface ever hiding state or
//! creating an irreversible trap. From the control alone a learner can tell exactly what to
//! act on, what counts as success, how to hint / reveal / reset / skip a step, whether a
//! mutating lesson runs in a sandbox or behind a preview, how much progress is completed and
//! remaining, and how to resume / reset / export that progress — with progress staying
//! user-owned and default-local, never silently shared beyond the supported scope.
//!
//! Aureline's frozen learning-component matrix
//! ([`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`])
//! names the guided exercise step and the progress marker as two governed component families
//! and freezes their controlled vocabulary — the exercise step states (`not_started`,
//! `active`, `passed`, `failed_retryable`, `replayable`, `sandboxed`) and validation modes
//! (`command_backed`, `sandboxed_practice`, `read_only_walkthrough`, `checkpoint_gated`,
//! `self_paced`, `no_hidden_apply`) an exercise binds; the progress ownership classes
//! (`local_only`, `user_owned_synced`, `exported_by_choice`, `workspace_shared`,
//! `cached_snapshot`, `not_installed`) and progress states (`not_started`, `in_progress`,
//! `completed`, `paused`, `reset`, `offline_local`) a marker binds; the one controlled
//! disposition vocabulary; the surface families; the deployment lines; the consumer
//! surfaces; the accessibility routes; the required labels; and the downgrade triggers. This
//! module *implements* that contract as two co-equal control vectors so a claimed M5
//! onboarding, guided-exercise, learning-mode, or progress surface can project a guided
//! exercise step and a progress marker that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_exercise_progress`] — takes an exercise step's frozen step state and derives
//!    its progress class (pending, in-progress, completed, retryable, or sandbox-practice),
//!    whether the step counts as completed, and which note the step must carry — so a
//!    failed-retryable step can never read as passed and a learner always knows what counts
//!    as success.
//! 2. [`resolve_progress_standing`] — takes a marker's frozen progress state and derives its
//!    standing (unstarted, underway, complete, interrupted, or offline-cached), whether the
//!    marker counts as complete, and which note the marker must carry — so a paused or reset
//!    marker can never read as complete and progress stays resumable.
//!
//! A single controls packet — [`GuidedExerciseStepProgressMarkerControlsPacket`] — binds one
//! vector of guided exercise steps and one vector of progress markers to the same target /
//! success, sandbox-or-preview, completion, resume / reset / export, ownership / privacy, and
//! non-visual accessibility vocabulary, so learnability stays observable, reversible, and
//! privacy-bounded across desktop, headless / export, and support consumers.
//!
//! The exercise step state ([`M5ExerciseStepState`]), exercise validation mode
//! ([`M5ExerciseValidationMode`]), progress ownership class ([`M5ProgressOwnershipClass`]),
//! progress state ([`M5ProgressState`]), disposition ([`M5LearningDisposition`]), surface
//! family ([`M5LearningSurfaceFamily`]), deployment line ([`M5LearningDeploymentLine`]),
//! consumer surface ([`M5LearningConsumerSurface`]), accessibility route
//! ([`M5LearningAccessibilityRoute`]), required label ([`M5LearningRequiredLabel`]), and
//! downgrade trigger ([`M5LearningDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! two controls themselves: the derived progress and standing classes, the bounded step and
//! marker actions, the target-object / resume-export deep-link kinds, and the sandbox-or-
//! preview mutation preference. No M5 learnability surface invents a second exercise or
//! progress grammar.
//!
//! Raw docs bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every success criterion, target reference, resume / export reference, and
//! control identity is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_guided_exercise_step_progress_marker_controls,
    seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable,
    seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset,
    GUIDED_EXERCISE_STEP_PROGRESS_MARKER_PACKET_ID,
};

// The exercise step states and validation modes, the progress ownership classes and progress
// states, the disposition vocabulary, and the surface / deployment / consumer / accessibility
// / label / downgrade vocabularies are frozen once, in the learning-component matrix. This
// lane reuses them verbatim so it never invents a parallel exercise or progress vocabulary.
pub use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5ExerciseStepState, M5ExerciseValidationMode, M5LearningAccessibilityRoute,
    M5LearningComponentFamily, M5LearningConsumerSurface, M5LearningDeploymentLine,
    M5LearningDisposition, M5LearningDowngradeTrigger, M5LearningRequiredLabel,
    M5LearningSurfaceFamily, M5ProgressOwnershipClass, M5ProgressState,
    M5_GUIDED_EXERCISE_STEP_SCHEMA_REF, M5_LEARNING_COMPONENT_DOC_REF,
    M5_LEARNING_COMPONENT_SCHEMA_REF, M5_PROGRESS_MARKER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`GuidedExerciseStepProgressMarkerControlsPacket`].
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_RECORD_KIND: &str =
    "implement_m5_guided_exercise_steps_and_progress_markers_with_target_object_success_criteria_hint_reveal_reset_skip_sandbox_or_preview_preference_and_privacy_bounded_resume_reset_export_truth_across_claimed_m5_learnability_lanes";

/// Schema version for M5 guided-exercise-step / progress-marker control records.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF: &str =
    "schemas/ui/m5-guided-exercise-step-progress-marker-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_DOC_REF: &str =
    "docs/help/m5_guided_exercise_step_progress_marker_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-guided-exercise-step-progress-marker-controls";

/// Repo-relative path of the checked support-export artifact.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_ARTIFACT_REF: &str =
    "artifacts/release/m5-guided-exercise-step-progress-marker-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_CSV_REF: &str =
    "artifacts/release/m5-guided-exercise-step-progress-marker-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const GUIDED_EXERCISE_STEP_PROGRESS_MARKER_REPORT_REF: &str =
    "artifacts/design/m5-guided-exercise-step-progress-marker.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a learning control binds against, so a step's target object
/// and a marker's resume / export target are always a stable command, file, surface, or docs
/// reference the user can reopen — never an ephemeral coachmark or hidden route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable command reference in the command system.
    CommandReference,
    /// A stable file location.
    FileLocation,
    /// A stable in-product surface location.
    SurfaceLocation,
    /// A stable docs anchor.
    DocsAnchor,
    /// No deep link is bound (the control names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommandReference,
        Self::FileLocation,
        Self::SurfaceLocation,
        Self::DocsAnchor,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandReference => "command_reference",
            Self::FileLocation => "file_location",
            Self::SurfaceLocation => "surface_location",
            Self::DocsAnchor => "docs_anchor",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- guided-exercise-step vocabulary ------------------------------------

/// Derived progress class a guided exercise step may present.
///
/// This is the exercise honesty axis: the class is derived from the frozen exercise step
/// state, never asserted, so a failed-retryable step can never present as passed and a
/// learner can always tell what counts as success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStepProgressClass {
    /// Not started yet.
    Pending,
    /// Active now.
    InProgress,
    /// Cleared (passed or replayable).
    Completed,
    /// Failed but retryable — never counts as success.
    Retryable,
    /// Sandbox practice — nothing touches live state.
    SandboxPractice,
}

impl ExerciseStepProgressClass {
    /// Every progress class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pending,
        Self::InProgress,
        Self::Completed,
        Self::Retryable,
        Self::SandboxPractice,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Retryable => "retryable",
            Self::SandboxPractice => "sandbox_practice",
        }
    }

    /// True only when the step counts as successfully completed.
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }
}

/// The sandbox-or-preview preference a mutating lesson must declare, so an exercise never
/// mutates live state without the same preview / approval model as ordinary work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LessonMutationPreference {
    /// The lesson runs entirely in a sandbox; nothing touches live state.
    SandboxPractice,
    /// The lesson previews the change and applies only after approval.
    PreviewThenApply,
    /// The lesson is a read-only walkthrough; it changes nothing.
    ReadOnlyWalkthrough,
    /// The lesson needs no mutation at all.
    NoMutationNeeded,
}

impl LessonMutationPreference {
    /// Every mutation preference, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SandboxPractice,
        Self::PreviewThenApply,
        Self::ReadOnlyWalkthrough,
        Self::NoMutationNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SandboxPractice => "sandbox_practice",
            Self::PreviewThenApply => "preview_then_apply",
            Self::ReadOnlyWalkthrough => "read_only_walkthrough",
            Self::NoMutationNeeded => "no_mutation_needed",
        }
    }

    /// True when this preference is safe for a lesson that mutates state — the change is
    /// either sandboxed or previewed-then-approved, never a bare hidden apply.
    pub const fn is_safe_for_mutation(self) -> bool {
        matches!(self, Self::SandboxPractice | Self::PreviewThenApply)
    }
}

/// One keyboard-complete default action a guided exercise step offers, so a step never traps
/// a learner or hides its reset affordance behind a pointer-only gesture. `ResetStep` is
/// always offered so a lesson is never an irreversible trap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStepAction {
    /// Show a hint toward the next action.
    ShowHint,
    /// Reveal the full solution.
    RevealSolution,
    /// Reset the step (always available).
    ResetStep,
    /// Skip the step and move on.
    SkipStep,
    /// Check the observable success criteria now.
    CheckSuccess,
    /// Open the stable target object this step acts on.
    OpenTargetObject,
}

impl ExerciseStepAction {
    /// Every step action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShowHint,
        Self::RevealSolution,
        Self::ResetStep,
        Self::SkipStep,
        Self::CheckSuccess,
        Self::OpenTargetObject,
    ];

    /// The default actions every keyboard-complete step must offer.
    pub const MANDATORY: [Self; 1] = [Self::ResetStep];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowHint => "show_hint",
            Self::RevealSolution => "reveal_solution",
            Self::ResetStep => "reset_step",
            Self::SkipStep => "skip_step",
            Self::CheckSuccess => "check_success",
            Self::OpenTargetObject => "open_target_object",
        }
    }
}

/// Disclosures a guided exercise step must carry, derived from the exercise step state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExerciseStepDisclosure {
    /// The derived progress class this step may present.
    pub progress_class: ExerciseStepProgressClass,
    /// Whether the step counts as completed.
    pub is_completed: bool,
    /// Whether the step must carry an explicit retryable note.
    pub needs_retry_note: bool,
    /// Whether the step must carry an explicit sandbox note.
    pub needs_sandbox_note: bool,
}

/// Resolves the progress truth a guided exercise step may present.
///
/// A `not_started` step is pending. An `active` step is in progress. A `passed` or
/// `replayable` step is completed. A `failed_retryable` step is retryable, never completed. A
/// `sandboxed` step is sandbox practice, so a step that has not actually been cleared can
/// never read as passed.
pub fn resolve_exercise_progress(state: M5ExerciseStepState) -> ExerciseStepDisclosure {
    use ExerciseStepProgressClass as Progress;
    use M5ExerciseStepState as Step;

    let progress_class = match state {
        Step::NotStarted => Progress::Pending,
        Step::Active => Progress::InProgress,
        Step::Passed | Step::Replayable => Progress::Completed,
        Step::FailedRetryable => Progress::Retryable,
        Step::Sandboxed => Progress::SandboxPractice,
    };

    ExerciseStepDisclosure {
        progress_class,
        is_completed: progress_class.is_completed(),
        needs_retry_note: matches!(progress_class, Progress::Retryable),
        needs_sandbox_note: matches!(progress_class, Progress::SandboxPractice),
    }
}

/// A guided exercise step naming its target object, observable success criteria, derived
/// progress, hint / reveal / reset / skip actions, and sandbox-or-preview preference for a
/// mutating lesson.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseStep {
    /// Frozen component this control implements; must be `guided_exercise_step`.
    pub component: M5LearningComponentFamily,
    /// Stable step id.
    pub step_id: String,
    /// Human-readable step label; required and non-empty.
    pub step_label: String,
    /// Exercise step state, reused from the frozen matrix.
    pub step_state: M5ExerciseStepState,
    /// Exercise validation mode, reused from the frozen matrix.
    pub validation_mode: M5ExerciseValidationMode,
    /// Derived progress class (must equal the resolved class).
    pub progress_class: ExerciseStepProgressClass,
    /// Whether the step claims it is completed (must equal the derived truth).
    pub claims_completed: bool,
    /// Retryable note; required when the step failed but is retryable.
    pub retry_note: String,
    /// Sandbox note; required when the step is sandbox practice.
    pub sandbox_note: String,
    /// Kind of stable target object this step acts on.
    pub target_kind: DeepLinkKind,
    /// Opaque stable target-object reference; required when the kind resolves.
    pub target_ref: String,
    /// Human-readable label of the exact object to act on; always required.
    pub target_object_label: String,
    /// Observable success criteria; always required so the step names what counts as success.
    pub success_criteria: String,
    /// Whether this lesson mutates live state.
    pub mutates_state: bool,
    /// Sandbox-or-preview preference (must be safe when the lesson mutates state).
    pub mutation_preference: LessonMutationPreference,
    /// Keyboard-complete default actions (must include the mandatory `ResetStep`).
    pub step_actions: Vec<ExerciseStepAction>,
    /// Dispositions this step binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this step can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this step can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this step.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this step keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this step offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this step's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this step.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides the success criteria or target identity. MUST be `false`.
    pub hides_success_criteria_or_target_identity: bool,
    /// Hard invariant: never implies a hidden apply or mutation. MUST be `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never traps progress without a reset / resume / export route. MUST be
    /// `false`.
    pub traps_progress_without_resume_reset_export: bool,
}

impl GuidedExerciseStep {
    /// Progress disclosures this step must carry, derived from the exercise step state.
    pub fn progress_disclosure(&self) -> ExerciseStepDisclosure {
        resolve_exercise_progress(self.step_state)
    }

    /// Whether the step offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ExerciseStepAction> = self.step_actions.iter().copied().collect();
        ExerciseStepAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the step declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the step offers a target-opening action.
    fn offers_target_action(&self) -> bool {
        self.step_actions
            .contains(&ExerciseStepAction::OpenTargetObject)
    }
}

// ---- progress-marker vocabulary -----------------------------------------

/// Derived standing a progress marker may present.
///
/// This is the progress honesty axis: the standing is derived from the frozen progress state,
/// never asserted, so a paused or reset marker can never present as complete and a learner
/// can always tell what is done and what remains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressStanding {
    /// Nothing done yet.
    Unstarted,
    /// Underway.
    Underway,
    /// Complete.
    Complete,
    /// Interrupted (paused or reset) — resumable, never complete.
    Interrupted,
    /// Offline / cached local view.
    OfflineCached,
}

impl ProgressStanding {
    /// Every standing, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Unstarted,
        Self::Underway,
        Self::Complete,
        Self::Interrupted,
        Self::OfflineCached,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unstarted => "unstarted",
            Self::Underway => "underway",
            Self::Complete => "complete",
            Self::Interrupted => "interrupted",
            Self::OfflineCached => "offline_cached",
        }
    }

    /// True only when the marker counts as complete.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// One keyboard-complete default action a progress marker offers, so progress stays
/// user-owned and recoverable rather than trapped inside a transient banner. `ResumeProgress`,
/// `ResetProgress`, and `ExportProgress` are always offered so progress is never a trap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressMarkerAction {
    /// Resume from the last resume point (always available).
    ResumeProgress,
    /// Reset progress (always available).
    ResetProgress,
    /// Export the progress record (always available).
    ExportProgress,
    /// View the remaining steps.
    ViewRemaining,
    /// Share progress to a workspace by explicit choice.
    ShareToWorkspace,
    /// Open the stable resume / export target.
    OpenResumePoint,
}

impl ProgressMarkerAction {
    /// Every marker action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResumeProgress,
        Self::ResetProgress,
        Self::ExportProgress,
        Self::ViewRemaining,
        Self::ShareToWorkspace,
        Self::OpenResumePoint,
    ];

    /// The default actions every keyboard-complete marker must offer so progress stays
    /// user-owned and recoverable.
    pub const MANDATORY: [Self; 3] = [
        Self::ResumeProgress,
        Self::ResetProgress,
        Self::ExportProgress,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeProgress => "resume_progress",
            Self::ResetProgress => "reset_progress",
            Self::ExportProgress => "export_progress",
            Self::ViewRemaining => "view_remaining",
            Self::ShareToWorkspace => "share_to_workspace",
            Self::OpenResumePoint => "open_resume_point",
        }
    }
}

/// Disclosures a progress marker must carry, derived from the progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressMarkerDisclosure {
    /// The derived standing this marker may present.
    pub standing_class: ProgressStanding,
    /// Whether the marker counts as complete.
    pub is_complete: bool,
    /// Whether the marker must carry an explicit interrupted note.
    pub needs_interrupted_note: bool,
    /// Whether the marker must carry an explicit offline note.
    pub needs_offline_note: bool,
}

/// Resolves the standing truth a progress marker may present.
///
/// A `not_started` marker is unstarted. An `in_progress` marker is underway. A `completed`
/// marker is complete. A `paused` or `reset` marker is interrupted, never complete. An
/// `offline_local` marker is offline-cached, so a marker that is not actually finished can
/// never read as complete.
pub fn resolve_progress_standing(state: M5ProgressState) -> ProgressMarkerDisclosure {
    use M5ProgressState as Progress;
    use ProgressStanding as Standing;

    let standing_class = match state {
        Progress::NotStarted => Standing::Unstarted,
        Progress::InProgress => Standing::Underway,
        Progress::Completed => Standing::Complete,
        Progress::Paused | Progress::Reset => Standing::Interrupted,
        Progress::OfflineLocal => Standing::OfflineCached,
    };

    ProgressMarkerDisclosure {
        standing_class,
        is_complete: standing_class.is_complete(),
        needs_interrupted_note: matches!(standing_class, Standing::Interrupted),
        needs_offline_note: matches!(standing_class, Standing::OfflineCached),
    }
}

/// True when a progress ownership class shares progress beyond the default-local scope, so a
/// marker never silently shares progress: only a synced, exported, or workspace-shared
/// ownership may claim to share beyond local.
pub const fn ownership_shares_beyond_local(ownership: M5ProgressOwnershipClass) -> bool {
    matches!(
        ownership,
        M5ProgressOwnershipClass::UserOwnedSynced
            | M5ProgressOwnershipClass::ExportedByChoice
            | M5ProgressOwnershipClass::WorkspaceShared
    )
}

/// A progress marker naming its completed / remaining state, derived standing, ownership /
/// privacy posture, resume / reset / export actions, and stable resume / export target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressMarker {
    /// Frozen component this control implements; must be `progress_marker`.
    pub component: M5LearningComponentFamily,
    /// Stable marker id.
    pub marker_id: String,
    /// Human-readable marker label; required and non-empty.
    pub marker_label: String,
    /// Progress state, reused from the frozen matrix.
    pub progress_state: M5ProgressState,
    /// Progress ownership class, reused from the frozen matrix.
    pub ownership: M5ProgressOwnershipClass,
    /// Derived standing (must equal the resolved standing).
    pub standing_class: ProgressStanding,
    /// Whether the marker claims it is complete (must equal the derived truth).
    pub claims_complete: bool,
    /// Completed unit count.
    pub completed_units: u32,
    /// Total unit count (completed must never exceed total; complete iff completed == total).
    pub total_units: u32,
    /// Interrupted note; required when the marker is paused or reset.
    pub interrupted_note: String,
    /// Offline note; required when the marker is an offline / cached view.
    pub offline_note: String,
    /// Ownership / privacy note; always required so ownership stays explicit.
    pub ownership_and_privacy_note: String,
    /// Whether this marker shares progress beyond the default-local scope.
    pub shares_beyond_local_scope: bool,
    /// Sharing disclosure note; required when the marker shares beyond local scope.
    pub sharing_disclosure_note: String,
    /// Kind of stable resume / export target this marker binds against.
    pub resume_export_kind: DeepLinkKind,
    /// Opaque stable resume / export reference; required when the kind resolves.
    pub resume_export_ref: String,
    /// Keyboard-complete default actions (must include the mandatory resume / reset / export).
    pub marker_actions: Vec<ProgressMarkerAction>,
    /// Dispositions this marker binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this marker can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this marker can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this marker.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this marker keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this marker offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this marker's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this marker.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides the success criteria or target identity. MUST be `false`.
    pub hides_success_criteria_or_target_identity: bool,
    /// Hard invariant: never implies a hidden apply or mutation. MUST be `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never traps progress without a reset / resume / export route. MUST be
    /// `false`.
    pub traps_progress_without_resume_reset_export: bool,
}

impl ProgressMarker {
    /// Standing disclosures this marker must carry, derived from the progress state.
    pub fn standing_disclosure(&self) -> ProgressMarkerDisclosure {
        resolve_progress_standing(self.progress_state)
    }

    /// Whether the marker offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ProgressMarkerAction> = self.marker_actions.iter().copied().collect();
        ProgressMarkerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the marker declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the marker offers a resume / export target action.
    fn offers_resume_export_action(&self) -> bool {
        self.marker_actions.iter().any(|action| {
            matches!(
                action,
                ProgressMarkerAction::OpenResumePoint | ProgressMarkerAction::ExportProgress
            )
        })
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance learnability review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseProgressReview {
    /// The guided exercise step identifies exactly what to act on.
    pub exercise_identifies_target_object: bool,
    /// The guided exercise step names its observable success criteria.
    pub exercise_names_observable_success_criteria: bool,
    /// The guided exercise step offers hint, reveal, reset, and skip.
    pub exercise_offers_hint_reveal_reset_skip: bool,
    /// A mutating lesson uses a sandbox or preview / approval model.
    pub mutating_lesson_uses_sandbox_or_preview: bool,
    /// Step progress is derived from state, never asserted.
    pub step_progress_derived_never_asserted: bool,
    /// A failed or retryable step is never shown as passed.
    pub retryable_never_shown_as_passed: bool,
    /// The progress marker shows completed and remaining state.
    pub progress_shows_completed_and_remaining: bool,
    /// The progress marker offers resume, reset, and export.
    pub progress_offers_resume_reset_export: bool,
    /// Completion is derived from state, never asserted.
    pub completion_derived_never_asserted: bool,
    /// An incomplete marker is never shown as complete.
    pub incomplete_never_shown_as_complete: bool,
    /// Progress stays user-owned and default-local.
    pub progress_user_owned_and_default_local: bool,
    /// Progress is never shared beyond the supported scope.
    pub progress_never_shared_beyond_supported_scope: bool,
    /// No control widens trust or mutating authority.
    pub no_control_widens_trust_or_mutating_authority: bool,
    /// No lesson is an irreversible trap without reset / resume / export.
    pub no_irreversible_trap_without_reset_or_resume: bool,
    /// Cached, offline, and local-only state stays visible.
    pub cached_offline_local_only_state_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
}

impl GuidedExerciseProgressReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exercise_identifies_target_object
            && self.exercise_names_observable_success_criteria
            && self.exercise_offers_hint_reveal_reset_skip
            && self.mutating_lesson_uses_sandbox_or_preview
            && self.step_progress_derived_never_asserted
            && self.retryable_never_shown_as_passed
            && self.progress_shows_completed_and_remaining
            && self.progress_offers_resume_reset_export
            && self.completion_derived_never_asserted
            && self.incomplete_never_shown_as_complete
            && self.progress_user_owned_and_default_local
            && self.progress_never_shared_beyond_supported_scope
            && self.no_control_widens_trust_or_mutating_authority
            && self.no_irreversible_trap_without_reset_or_resume
            && self.cached_offline_local_only_state_visible
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseProgressConsumerProjection {
    /// The exercise surface reads a single canonical source.
    pub exercise_ui_reads_single_source: bool,
    /// The progress surface reads a single canonical source.
    pub progress_ui_reads_single_source: bool,
    /// The target object and success criteria are visible before starting.
    pub target_and_success_visible_before_start: bool,
    /// Completed and remaining state is visible before a tap.
    pub completed_and_remaining_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl GuidedExerciseProgressConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exercise_ui_reads_single_source
            && self.progress_ui_reads_single_source
            && self.target_and_success_visible_before_start
            && self.completed_and_remaining_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseProgressProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GuidedExerciseStepProgressMarkerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuidedExerciseStepProgressMarkerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Guided exercise steps.
    pub exercise_steps: Vec<GuidedExerciseStep>,
    /// Progress markers.
    pub progress_markers: Vec<ProgressMarker>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: GuidedExerciseProgressReview,
    /// Consumer projection block.
    pub consumer_projection: GuidedExerciseProgressConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GuidedExerciseProgressProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe guided-exercise-step / progress-marker controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseStepProgressMarkerControlsPacket {
    /// Record kind; must equal [`GUIDED_EXERCISE_STEP_PROGRESS_MARKER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Guided exercise steps.
    pub exercise_steps: Vec<GuidedExerciseStep>,
    /// Progress markers.
    pub progress_markers: Vec<ProgressMarker>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: GuidedExerciseProgressReview,
    /// Consumer projection block.
    pub consumer_projection: GuidedExerciseProgressConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GuidedExerciseProgressProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GuidedExerciseStepProgressMarkerControlsPacket {
    /// Builds a guided-exercise-step / progress-marker controls packet from stable-lane input.
    pub fn new(input: GuidedExerciseStepProgressMarkerControlsPacketInput) -> Self {
        Self {
            record_kind: GUIDED_EXERCISE_STEP_PROGRESS_MARKER_RECORD_KIND.to_owned(),
            schema_version: GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            exercise_steps: input.exercise_steps,
            progress_markers: input.progress_markers,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            learnability_review: input.learnability_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the guided-exercise-step / progress-marker control invariants.
    pub fn validate(&self) -> Vec<GuidedExerciseStepProgressMarkerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GUIDED_EXERCISE_STEP_PROGRESS_MARKER_RECORD_KIND {
            violations.push(GuidedExerciseStepProgressMarkerViolation::WrongRecordKind);
        }
        if self.schema_version != GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_VERSION {
            violations.push(GuidedExerciseStepProgressMarkerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_exercise_steps(self, &mut violations);
        validate_progress_markers(self, &mut violations);

        if !self.learnability_review.all_hold() {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::LearnabilityReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("guided exercise step progress marker packet serializes"),
        ) {
            violations.push(GuidedExerciseStepProgressMarkerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("guided exercise step progress marker packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,state,mode_or_ownership,derived,completed_or_complete,deep_link_kind\n",
        );
        for step in &self.exercise_steps {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "guided_exercise_step",
                csv_field(&step.step_id),
                step.step_state.as_str(),
                step.validation_mode.as_str(),
                step.progress_disclosure().progress_class.as_str(),
                step.progress_disclosure().is_completed,
                step.target_kind.as_str(),
            ));
        }
        for marker in &self.progress_markers {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "progress_marker",
                csv_field(&marker.marker_id),
                marker.progress_state.as_str(),
                marker.ownership.as_str(),
                marker.standing_disclosure().standing_class.as_str(),
                marker.standing_disclosure().is_complete,
                marker.resume_export_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let unpassed = self
            .exercise_steps
            .iter()
            .filter(|step| !step.progress_disclosure().is_completed)
            .count();
        let incomplete = self
            .progress_markers
            .iter()
            .filter(|marker| !marker.standing_disclosure().is_complete)
            .count();

        let mut out = String::new();
        out.push_str("# Guided exercise steps and progress markers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Guided exercise steps: {} ({} not yet completed)\n",
            self.exercise_steps.len(),
            unpassed
        ));
        out.push_str(&format!(
            "- Progress markers: {} ({} not complete)\n",
            self.progress_markers.len(),
            incomplete
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Guided exercise steps\n\n");
        for step in &self.exercise_steps {
            out.push_str(&format!(
                "- **{}** — state `{}`, mode `{}` → `{}`, target `{}`\n",
                step.step_label,
                step.step_state.as_str(),
                step.validation_mode.as_str(),
                step.progress_disclosure().progress_class.as_str(),
                step.target_kind.as_str(),
            ));
        }

        out.push_str("\n## Progress markers\n\n");
        for marker in &self.progress_markers {
            out.push_str(&format!(
                "- **{}** — state `{}`, ownership `{}` → `{}`, {}/{} done\n",
                marker.marker_label,
                marker.progress_state.as_str(),
                marker.ownership.as_str(),
                marker.standing_disclosure().standing_class.as_str(),
                marker.completed_units,
                marker.total_units,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in guided-exercise-step / progress-marker export.
#[derive(Debug)]
pub enum GuidedExerciseStepProgressMarkerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GuidedExerciseStepProgressMarkerViolation>),
}

impl fmt::Display for GuidedExerciseStepProgressMarkerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "guided exercise step progress marker export parse failed: {error}"
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
                    "guided exercise step progress marker export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GuidedExerciseStepProgressMarkerArtifactError {}

/// Validation failures emitted by
/// [`GuidedExerciseStepProgressMarkerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuidedExerciseStepProgressMarkerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No guided exercise steps are present.
    ExerciseStepsMissing,
    /// A guided exercise step is incomplete.
    ExerciseStepIncomplete,
    /// A guided exercise step carries the wrong frozen component class.
    ExerciseStepWrongComponentClass,
    /// A guided exercise step misrepresents its derived progress state.
    ExerciseProgressMisrepresented,
    /// A retryable step does not name its retryable state.
    RetryNoteMissing,
    /// A sandbox-practice step does not name its sandbox state.
    ExerciseSandboxNoteMissing,
    /// A step offers a target action but its target does not resolve exactly.
    TargetObjectUnresolved,
    /// A step names a target kind but not its stable reference.
    TargetObjectRefMissing,
    /// A step does not name its target-object label.
    TargetObjectLabelMissing,
    /// A step does not name its observable success criteria.
    SuccessCriteriaMissing,
    /// A mutating lesson does not run in a sandbox or behind a preview / approval model.
    MutatingLessonWithoutSandboxOrPreview,
    /// A step omits the mandatory `ResetStep` action.
    ExerciseStepActionsIncomplete,
    /// The exercise steps do not cover every derived progress class.
    ExerciseProgressClassCoverageMissing,
    /// The exercise steps do not cover every exercise step state.
    ExerciseStepStateCoverageMissing,
    /// The exercise steps do not cover every exercise validation mode.
    ExerciseValidationModeCoverageMissing,
    /// No progress markers are present.
    ProgressMarkersMissing,
    /// A progress marker is incomplete.
    ProgressMarkerIncomplete,
    /// A progress marker carries the wrong frozen component class.
    ProgressMarkerWrongComponentClass,
    /// A progress marker misrepresents its derived completion state.
    CompletionMisrepresented,
    /// A progress marker misrepresents its completed / remaining counts.
    ProgressCountMisrepresented,
    /// An interrupted marker does not name its interrupted state.
    InterruptedNoteMissing,
    /// An offline marker does not name its offline state.
    OfflineNoteMissing,
    /// A marker does not name its ownership / privacy posture.
    OwnershipAndPrivacyNoteMissing,
    /// A marker claims to share progress beyond its supported ownership scope.
    ProgressSharedBeyondScope,
    /// A sharing marker does not disclose that it shares progress.
    SharingDisclosureMissing,
    /// A marker offers a resume / export action but its target does not resolve exactly.
    ResumeExportUnresolved,
    /// A marker names a resume / export kind but not its stable reference.
    ResumeExportRefMissing,
    /// A marker omits the mandatory resume / reset / export actions.
    ProgressMarkerActionsIncomplete,
    /// The progress markers do not cover every derived standing.
    ProgressStandingCoverageMissing,
    /// The progress markers do not cover every progress state.
    ProgressStateCoverageMissing,
    /// The progress markers do not cover every progress ownership class.
    ProgressOwnershipCoverageMissing,
    /// A control does not bind any disposition.
    DispositionsMissing,
    /// A control does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A control does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control masks its privacy or offline / local-only state.
    PrivacyOrOfflineStateMasked,
    /// A control hides its success criteria or target identity.
    SuccessCriteriaOrTargetHidden,
    /// A control implies a hidden apply or mutation.
    HiddenApplyOrMutationImplied,
    /// A control invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A control traps progress without a reset / resume / export route.
    ProgressTrappedWithoutResumeResetExport,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Learnability review does not satisfy required invariants.
    LearnabilityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl GuidedExerciseStepProgressMarkerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ExerciseStepsMissing => "exercise_steps_missing",
            Self::ExerciseStepIncomplete => "exercise_step_incomplete",
            Self::ExerciseStepWrongComponentClass => "exercise_step_wrong_component_class",
            Self::ExerciseProgressMisrepresented => "exercise_progress_misrepresented",
            Self::RetryNoteMissing => "retry_note_missing",
            Self::ExerciseSandboxNoteMissing => "exercise_sandbox_note_missing",
            Self::TargetObjectUnresolved => "target_object_unresolved",
            Self::TargetObjectRefMissing => "target_object_ref_missing",
            Self::TargetObjectLabelMissing => "target_object_label_missing",
            Self::SuccessCriteriaMissing => "success_criteria_missing",
            Self::MutatingLessonWithoutSandboxOrPreview => {
                "mutating_lesson_without_sandbox_or_preview"
            }
            Self::ExerciseStepActionsIncomplete => "exercise_step_actions_incomplete",
            Self::ExerciseProgressClassCoverageMissing => {
                "exercise_progress_class_coverage_missing"
            }
            Self::ExerciseStepStateCoverageMissing => "exercise_step_state_coverage_missing",
            Self::ExerciseValidationModeCoverageMissing => {
                "exercise_validation_mode_coverage_missing"
            }
            Self::ProgressMarkersMissing => "progress_markers_missing",
            Self::ProgressMarkerIncomplete => "progress_marker_incomplete",
            Self::ProgressMarkerWrongComponentClass => "progress_marker_wrong_component_class",
            Self::CompletionMisrepresented => "completion_misrepresented",
            Self::ProgressCountMisrepresented => "progress_count_misrepresented",
            Self::InterruptedNoteMissing => "interrupted_note_missing",
            Self::OfflineNoteMissing => "offline_note_missing",
            Self::OwnershipAndPrivacyNoteMissing => "ownership_and_privacy_note_missing",
            Self::ProgressSharedBeyondScope => "progress_shared_beyond_scope",
            Self::SharingDisclosureMissing => "sharing_disclosure_missing",
            Self::ResumeExportUnresolved => "resume_export_unresolved",
            Self::ResumeExportRefMissing => "resume_export_ref_missing",
            Self::ProgressMarkerActionsIncomplete => "progress_marker_actions_incomplete",
            Self::ProgressStandingCoverageMissing => "progress_standing_coverage_missing",
            Self::ProgressStateCoverageMissing => "progress_state_coverage_missing",
            Self::ProgressOwnershipCoverageMissing => "progress_ownership_coverage_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::PrivacyOrOfflineStateMasked => "privacy_or_offline_state_masked",
            Self::SuccessCriteriaOrTargetHidden => "success_criteria_or_target_hidden",
            Self::HiddenApplyOrMutationImplied => "hidden_apply_or_mutation_implied",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProgressTrappedWithoutResumeResetExport => {
                "progress_trapped_without_resume_reset_export"
            }
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LearnabilityReviewIncomplete => "learnability_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable guided-exercise-step / progress-marker export.
pub fn current_guided_exercise_step_progress_marker_export() -> Result<
    GuidedExerciseStepProgressMarkerControlsPacket,
    GuidedExerciseStepProgressMarkerArtifactError,
> {
    let packet: GuidedExerciseStepProgressMarkerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-guided-exercise-step-progress-marker-proof/support_export.json"
        )))
        .map_err(GuidedExerciseStepProgressMarkerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GuidedExerciseStepProgressMarkerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GuidedExerciseStepProgressMarkerControlsPacket,
    violations: &mut Vec<GuidedExerciseStepProgressMarkerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF,
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GuidedExerciseStepProgressMarkerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_exercise_steps(
    packet: &GuidedExerciseStepProgressMarkerControlsPacket,
    violations: &mut Vec<GuidedExerciseStepProgressMarkerViolation>,
) {
    if packet.exercise_steps.is_empty() {
        violations.push(GuidedExerciseStepProgressMarkerViolation::ExerciseStepsMissing);
        return;
    }

    let mut progress_classes: BTreeSet<ExerciseStepProgressClass> = BTreeSet::new();
    let mut states: BTreeSet<M5ExerciseStepState> = BTreeSet::new();
    let mut modes: BTreeSet<M5ExerciseValidationMode> = BTreeSet::new();

    for step in &packet.exercise_steps {
        let disclosure = step.progress_disclosure();
        progress_classes.insert(disclosure.progress_class);
        states.insert(step.step_state);
        modes.insert(step.validation_mode);

        if step.step_id.trim().is_empty()
            || step.step_label.trim().is_empty()
            || step.fields_shown.is_empty()
            || step.surface_families.is_empty()
            || step.deployment_lines.is_empty()
            || step.consumer_surfaces.is_empty()
            || step.source_contract_refs.is_empty()
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ExerciseStepIncomplete);
        }
        if step.component != M5LearningComponentFamily::GuidedExerciseStep {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ExerciseStepWrongComponentClass);
        }
        if step.progress_class != disclosure.progress_class
            || step.claims_completed != disclosure.is_completed
        {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ExerciseProgressMisrepresented);
        }
        if disclosure.needs_retry_note && step.retry_note.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::RetryNoteMissing);
        }
        if disclosure.needs_sandbox_note && step.sandbox_note.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ExerciseSandboxNoteMissing);
        }
        if step.target_object_label.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::TargetObjectLabelMissing);
        }
        if step.success_criteria.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::SuccessCriteriaMissing);
        }
        if step.mutates_state && !step.mutation_preference.is_safe_for_mutation() {
            violations.push(
                GuidedExerciseStepProgressMarkerViolation::MutatingLessonWithoutSandboxOrPreview,
            );
        }
        if !step.declares_mandatory_actions() {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ExerciseStepActionsIncomplete);
        }
        if step.offers_target_action() && !step.target_kind.is_resolvable() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::TargetObjectUnresolved);
        }
        if step.target_kind.is_resolvable() && step.target_ref.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::TargetObjectRefMissing);
        }
        validate_common_control(
            &step.dispositions,
            &step.downgrade_triggers,
            step.declares_mandatory_labels(),
            &step.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: step.masks_privacy_or_offline_state,
                hides_success_criteria_or_target_identity: step
                    .hides_success_criteria_or_target_identity,
                implies_hidden_apply_or_mutation: step.implies_hidden_apply_or_mutation,
                invents_alternate_state_label: step.invents_alternate_state_label,
                traps_progress_without_resume_reset_export: step
                    .traps_progress_without_resume_reset_export,
            },
            violations,
        );
    }

    for required in ExerciseStepProgressClass::ALL {
        if !progress_classes.contains(&required) {
            violations.push(
                GuidedExerciseStepProgressMarkerViolation::ExerciseProgressClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5ExerciseStepState::ALL {
        if !states.contains(&required) {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ExerciseStepStateCoverageMissing);
            break;
        }
    }
    for required in M5ExerciseValidationMode::ALL {
        if !modes.contains(&required) {
            violations.push(
                GuidedExerciseStepProgressMarkerViolation::ExerciseValidationModeCoverageMissing,
            );
            break;
        }
    }
}

fn validate_progress_markers(
    packet: &GuidedExerciseStepProgressMarkerControlsPacket,
    violations: &mut Vec<GuidedExerciseStepProgressMarkerViolation>,
) {
    if packet.progress_markers.is_empty() {
        violations.push(GuidedExerciseStepProgressMarkerViolation::ProgressMarkersMissing);
        return;
    }

    let mut standings: BTreeSet<ProgressStanding> = BTreeSet::new();
    let mut states: BTreeSet<M5ProgressState> = BTreeSet::new();
    let mut ownerships: BTreeSet<M5ProgressOwnershipClass> = BTreeSet::new();

    for marker in &packet.progress_markers {
        let disclosure = marker.standing_disclosure();
        standings.insert(disclosure.standing_class);
        states.insert(marker.progress_state);
        ownerships.insert(marker.ownership);

        if marker.marker_id.trim().is_empty()
            || marker.marker_label.trim().is_empty()
            || marker.fields_shown.is_empty()
            || marker.surface_families.is_empty()
            || marker.deployment_lines.is_empty()
            || marker.consumer_surfaces.is_empty()
            || marker.source_contract_refs.is_empty()
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ProgressMarkerIncomplete);
        }
        if marker.component != M5LearningComponentFamily::ProgressMarker {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ProgressMarkerWrongComponentClass);
        }
        if marker.standing_class != disclosure.standing_class
            || marker.claims_complete != disclosure.is_complete
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::CompletionMisrepresented);
        }
        if marker.completed_units > marker.total_units
            || (disclosure.is_complete && marker.completed_units != marker.total_units)
            || (!disclosure.is_complete
                && marker.total_units > 0
                && marker.completed_units == marker.total_units)
        {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ProgressCountMisrepresented);
        }
        if disclosure.needs_interrupted_note && marker.interrupted_note.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::InterruptedNoteMissing);
        }
        if disclosure.needs_offline_note && marker.offline_note.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::OfflineNoteMissing);
        }
        if marker.ownership_and_privacy_note.trim().is_empty() {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::OwnershipAndPrivacyNoteMissing);
        }
        if marker.shares_beyond_local_scope && !ownership_shares_beyond_local(marker.ownership) {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ProgressSharedBeyondScope);
        }
        if marker.shares_beyond_local_scope && marker.sharing_disclosure_note.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::SharingDisclosureMissing);
        }
        if !marker.declares_mandatory_actions() {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ProgressMarkerActionsIncomplete);
        }
        if marker.offers_resume_export_action() && !marker.resume_export_kind.is_resolvable() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ResumeExportUnresolved);
        }
        if marker.resume_export_kind.is_resolvable() && marker.resume_export_ref.trim().is_empty() {
            violations.push(GuidedExerciseStepProgressMarkerViolation::ResumeExportRefMissing);
        }
        validate_common_control(
            &marker.dispositions,
            &marker.downgrade_triggers,
            marker.declares_mandatory_labels(),
            &marker.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: marker.masks_privacy_or_offline_state,
                hides_success_criteria_or_target_identity: marker
                    .hides_success_criteria_or_target_identity,
                implies_hidden_apply_or_mutation: marker.implies_hidden_apply_or_mutation,
                invents_alternate_state_label: marker.invents_alternate_state_label,
                traps_progress_without_resume_reset_export: marker
                    .traps_progress_without_resume_reset_export,
            },
            violations,
        );
    }

    for required in ProgressStanding::ALL {
        if !standings.contains(&required) {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ProgressStandingCoverageMissing);
            break;
        }
    }
    for required in M5ProgressState::ALL {
        if !states.contains(&required) {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ProgressStateCoverageMissing);
            break;
        }
    }
    for required in M5ProgressOwnershipClass::ALL {
        if !ownerships.contains(&required) {
            violations
                .push(GuidedExerciseStepProgressMarkerViolation::ProgressOwnershipCoverageMissing);
            break;
        }
    }
}

/// The five hard-invariant bools every control must keep `false`.
struct ControlInvariants {
    masks_privacy_or_offline_state: bool,
    hides_success_criteria_or_target_identity: bool,
    implies_hidden_apply_or_mutation: bool,
    invents_alternate_state_label: bool,
    traps_progress_without_resume_reset_export: bool,
}

/// Validates the axes shared by both control vectors.
fn validate_common_control(
    dispositions: &[M5LearningDisposition],
    downgrade_triggers: &[M5LearningDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5LearningAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<GuidedExerciseStepProgressMarkerViolation>,
) {
    if dispositions.is_empty() {
        violations.push(GuidedExerciseStepProgressMarkerViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(GuidedExerciseStepProgressMarkerViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(GuidedExerciseStepProgressMarkerViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5LearningAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(GuidedExerciseStepProgressMarkerViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_privacy_or_offline_state {
        violations.push(GuidedExerciseStepProgressMarkerViolation::PrivacyOrOfflineStateMasked);
    }
    if invariants.hides_success_criteria_or_target_identity {
        violations.push(GuidedExerciseStepProgressMarkerViolation::SuccessCriteriaOrTargetHidden);
    }
    if invariants.implies_hidden_apply_or_mutation {
        violations.push(GuidedExerciseStepProgressMarkerViolation::HiddenApplyOrMutationImplied);
    }
    if invariants.invents_alternate_state_label {
        violations.push(GuidedExerciseStepProgressMarkerViolation::AlternateStateLabelInvented);
    }
    if invariants.traps_progress_without_resume_reset_export {
        violations.push(
            GuidedExerciseStepProgressMarkerViolation::ProgressTrappedWithoutResumeResetExport,
        );
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
