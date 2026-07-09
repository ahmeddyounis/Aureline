//! Frozen M5 learning-mode-toggle, tip-card, guided-exercise-step,
//! glossary-chip-or-card, safe-explanation-banner, and progress-marker component
//! matrix.
//!
//! This module locks Aureline's reusable learning-mode and educational-surface
//! components into one export-safe packet. Every learnability-facing subcomponent M5
//! claims that still drifts too easily by first-run, guided-tour, learning-mode-panel,
//! glossary, inline-help, or CLI surface — the learning-mode toggle, the tip card, the
//! guided exercise step, the glossary chip or card, the safe explanation banner, and the
//! progress marker — is named once here and constrained by the same opt-in learning
//! state, cited source truth, explain-versus-do boundary, sandbox / no-hidden-apply
//! guarantee, and user-owned, privacy-bounded progress regardless of the surface family
//! that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families; the one controlled disposition vocabulary every consumer binds
//! (`learning_on`, `paused`, `replayable`, `sandboxed`, `cached`, `local_only`,
//! `not_installed`, `no_hidden_apply`); the learning-mode states and scopes the toggle
//! binds; the tip trigger classes and dismissal states the tip card binds; the exercise
//! step states and validation modes the guided exercise step binds; the glossary source
//! classes and citation states the glossary chip or card binds; the explanation boundary
//! classes and apply states the safe explanation banner binds; the progress ownership
//! classes and states the progress marker binds; the deployment lines every component must
//! survive; the non-visual accessibility routes; and the mandatory labels every component
//! must be able to show. It does not re-architect the learning-mode profile, tour /
//! glossary manifest, exercise rail, feature-availability, or progress-snapshot contracts
//! that already own those records — it is the shared learnability-component contract
//! layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 onboarding, tour,
//! learning-mode, glossary, or help surface may publish a learning-mode toggle, a teaching
//! tip, a guided exercise step, a glossary term, a safe explanation, or a progress marker.
//! Onboarding, guided-learning, glossary, explanation, and progress consumers all read
//! this packet so one toggle names its learning state and scope, one tip card names its
//! command binding and dismissal state, one guided exercise step names its state and that
//! it never hides an apply, one glossary chip or card names its cited source and citation
//! state, one safe explanation banner names its explain-versus-do boundary and that it
//! applies nothing without the same preview / approval model as ordinary work, and one
//! progress marker names that its progress is user-owned and default-local. No M5 lane
//! invents a second learning grammar or an alternate label for a paused, sandboxed,
//! cached, local-only, not-installed, or no-hidden-apply state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5LearningComponentVocabularySet`] rather than minted per surface. Raw docs bodies,
//! pasted paths, credentials, and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_learning_component_matrix,
    seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed,
    seeded_m5_learning_component_matrix_progress_marker_preview_narrowed,
    M5_LEARNING_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LearningComponentMatrixPacket`].
pub const M5_LEARNING_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix";

/// Schema version for M5 learning component-matrix records.
pub const M5_LEARNING_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined learning-component boundary schema.
pub const M5_LEARNING_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LEARNING_COMPONENT_DOC_REF: &str = "docs/help/m5_learning_component_matrix.md";

/// Repo-relative path of the per-component learning-mode-toggle schema.
pub const M5_LEARNING_MODE_TOGGLE_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-mode-toggle.schema.json";

/// Repo-relative path of the per-component tip-card schema.
pub const M5_TIP_CARD_SCHEMA_REF: &str = "schemas/ui/m5-tip-card.schema.json";

/// Repo-relative path of the per-component guided-exercise-step schema.
pub const M5_GUIDED_EXERCISE_STEP_SCHEMA_REF: &str =
    "schemas/ui/m5-guided-exercise-step.schema.json";

/// Repo-relative path of the per-component glossary-chip-or-card schema.
pub const M5_GLOSSARY_CHIP_CARD_SCHEMA_REF: &str = "schemas/ui/m5-glossary-chip-card.schema.json";

/// Repo-relative path of the per-component safe-explanation-banner schema.
pub const M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-safe-explanation-banner.schema.json";

/// Repo-relative path of the per-component progress-marker schema.
pub const M5_PROGRESS_MARKER_SCHEMA_REF: &str = "schemas/ui/m5-progress-marker.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_LEARNING_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-learning-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LEARNING_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LEARNING_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-learning-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_LEARNING_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-learning-component-matrix.md";

/// One of the six governed learning-component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentFamily {
    /// A learning-mode toggle carrying its learning state and scope.
    LearningModeToggle,
    /// A tip card carrying its trigger, command binding, and dismissal state.
    TipCard,
    /// A guided exercise step carrying its step state and validation mode.
    GuidedExerciseStep,
    /// A glossary chip or card carrying its cited source and citation state.
    GlossaryChipOrCard,
    /// A safe explanation banner carrying its explain-versus-do boundary and apply state.
    SafeExplanationBanner,
    /// A progress marker carrying its ownership class and progress state.
    ProgressMarker,
}

impl M5LearningComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LearningModeToggle,
        Self::TipCard,
        Self::GuidedExerciseStep,
        Self::GlossaryChipOrCard,
        Self::SafeExplanationBanner,
        Self::ProgressMarker,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LearningModeToggle => "learning_mode_toggle",
            Self::TipCard => "tip_card",
            Self::GuidedExerciseStep => "guided_exercise_step",
            Self::GlossaryChipOrCard => "glossary_chip_or_card",
            Self::SafeExplanationBanner => "safe_explanation_banner",
            Self::ProgressMarker => "progress_marker",
        }
    }

    /// `true` when this family is a learning-mode toggle and must therefore declare its
    /// learning states and scopes.
    pub const fn is_learning_mode_toggle(self) -> bool {
        matches!(self, Self::LearningModeToggle)
    }

    /// `true` when this family is a tip card and must therefore declare its tip trigger
    /// classes and dismissal states.
    pub const fn is_tip_card(self) -> bool {
        matches!(self, Self::TipCard)
    }

    /// `true` when this family is a guided exercise step and must therefore declare its
    /// exercise step states and validation modes.
    pub const fn is_guided_exercise_step(self) -> bool {
        matches!(self, Self::GuidedExerciseStep)
    }

    /// `true` when this family is a glossary chip or card and must therefore declare its
    /// glossary source classes and citation states.
    pub const fn is_glossary_chip_or_card(self) -> bool {
        matches!(self, Self::GlossaryChipOrCard)
    }

    /// `true` when this family is a safe explanation banner and must therefore declare its
    /// explanation boundary classes and apply states.
    pub const fn is_safe_explanation_banner(self) -> bool {
        matches!(self, Self::SafeExplanationBanner)
    }

    /// `true` when this family is a progress marker and must therefore declare its progress
    /// ownership classes and progress states.
    pub const fn is_progress_marker(self) -> bool {
        matches!(self, Self::ProgressMarker)
    }
}

/// The one controlled disposition vocabulary every learning-component consumer binds. These
/// are the exact acceptance-criteria labels so no surface invents a parallel word for a
/// learning-on, paused, replayable, sandboxed, cached, local-only, not-installed, or
/// no-hidden-apply state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningDisposition {
    /// Learning mode is on.
    LearningOn,
    /// Learning is paused.
    Paused,
    /// The step or tour is replayable.
    Replayable,
    /// Practice happens in a sandbox.
    Sandboxed,
    /// Content is cached.
    Cached,
    /// Progress or state is local-only.
    LocalOnly,
    /// The feature or pack is not installed.
    NotInstalled,
    /// Nothing is applied without the ordinary preview / approval model.
    NoHiddenApply,
}

impl M5LearningDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LearningOn,
        Self::Paused,
        Self::Replayable,
        Self::Sandboxed,
        Self::Cached,
        Self::LocalOnly,
        Self::NotInstalled,
        Self::NoHiddenApply,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LearningOn => "learning_on",
            Self::Paused => "paused",
            Self::Replayable => "replayable",
            Self::Sandboxed => "sandboxed",
            Self::Cached => "cached",
            Self::LocalOnly => "local_only",
            Self::NotInstalled => "not_installed",
            Self::NoHiddenApply => "no_hidden_apply",
        }
    }
}

/// Controlled learning-mode state — whether learning mode is engaged, so a learning-mode
/// toggle never leaves its enablement state implicit and learning stays opt-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningModeState {
    /// Learning mode is off (the default).
    Off,
    /// Learning mode is on.
    On,
    /// Learning mode is paused.
    Paused,
    /// Learning mode is enabled per feature family.
    PerFeatureFamily,
    /// Only sandboxed practice is enabled.
    SandboxedOnly,
    /// A learning session has ended.
    Ended,
}

impl M5LearningModeState {
    /// Every learning-mode state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Off,
        Self::On,
        Self::Paused,
        Self::PerFeatureFamily,
        Self::SandboxedOnly,
        Self::Ended,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::Paused => "paused",
            Self::PerFeatureFamily => "per_feature_family",
            Self::SandboxedOnly => "sandboxed_only",
            Self::Ended => "ended",
        }
    }
}

/// Controlled learning-mode scope — how widely a learning-mode toggle applies, so a toggle
/// never hides whether it changes one surface or the whole workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningModeScope {
    /// Applies globally.
    Global,
    /// Applies to the current workspace.
    Workspace,
    /// Applies to one feature family.
    FeatureFamily,
    /// Applies to the current session.
    Session,
    /// Applies to one surface.
    Surface,
    /// Unavailable on this build.
    Unavailable,
}

impl M5LearningModeScope {
    /// Every learning-mode scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Global,
        Self::Workspace,
        Self::FeatureFamily,
        Self::Session,
        Self::Surface,
        Self::Unavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Workspace => "workspace",
            Self::FeatureFamily => "feature_family",
            Self::Session => "session",
            Self::Surface => "surface",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Controlled tip trigger class — why a tip card appears, so a tip never leaves its trigger
/// implicit or invents a parallel trigger taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TipTriggerClass {
    /// First encounter with a surface.
    FirstEncounter,
    /// Discovery of a related feature.
    FeatureDiscovery,
    /// Recovery from an error.
    ErrorRecovery,
    /// A mode or profile change.
    ModeChange,
    /// An idle-time hint.
    IdleHint,
    /// A contextual follow-up to a prior action.
    ContextualFollowup,
}

impl M5TipTriggerClass {
    /// Every tip trigger class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstEncounter,
        Self::FeatureDiscovery,
        Self::ErrorRecovery,
        Self::ModeChange,
        Self::IdleHint,
        Self::ContextualFollowup,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstEncounter => "first_encounter",
            Self::FeatureDiscovery => "feature_discovery",
            Self::ErrorRecovery => "error_recovery",
            Self::ModeChange => "mode_change",
            Self::IdleHint => "idle_hint",
            Self::ContextualFollowup => "contextual_followup",
        }
    }
}

/// Controlled tip dismissal state — how a tip card can be dismissed, so teaching stays
/// dismissible and never blocks the user or hides its dismissal affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TipDismissalState {
    /// Dismissible right now.
    Dismissible,
    /// Already dismissed.
    Dismissed,
    /// Snoozed for later.
    Snoozed,
    /// Persistent until the user acts.
    PersistentUntilActed,
    /// Auto-expired after its window.
    AutoExpired,
    /// Suppressed by a user preference.
    SuppressedByPreference,
}

impl M5TipDismissalState {
    /// Every tip dismissal state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Dismissible,
        Self::Dismissed,
        Self::Snoozed,
        Self::PersistentUntilActed,
        Self::AutoExpired,
        Self::SuppressedByPreference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dismissible => "dismissible",
            Self::Dismissed => "dismissed",
            Self::Snoozed => "snoozed",
            Self::PersistentUntilActed => "persistent_until_acted",
            Self::AutoExpired => "auto_expired",
            Self::SuppressedByPreference => "suppressed_by_preference",
        }
    }
}

/// Controlled exercise step state — the state of a guided exercise step, so a step never
/// leaves its progress implicit and a replayable or sandboxed step is never mislabeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExerciseStepState {
    /// Not started.
    NotStarted,
    /// Active now.
    Active,
    /// Passed.
    Passed,
    /// Failed but retryable.
    FailedRetryable,
    /// Replayable.
    Replayable,
    /// Sandboxed practice.
    Sandboxed,
}

impl M5ExerciseStepState {
    /// Every exercise step state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotStarted,
        Self::Active,
        Self::Passed,
        Self::FailedRetryable,
        Self::Replayable,
        Self::Sandboxed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Active => "active",
            Self::Passed => "passed",
            Self::FailedRetryable => "failed_retryable",
            Self::Replayable => "replayable",
            Self::Sandboxed => "sandboxed",
        }
    }
}

/// Controlled exercise validation mode — how a guided exercise step checks the learner's
/// work, so an exercise never mutates live state without the ordinary preview / approval
/// model and `no_hidden_apply` stays explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExerciseValidationMode {
    /// Backed by a stable command.
    CommandBacked,
    /// Runs in a sandbox.
    SandboxedPractice,
    /// A read-only walkthrough.
    ReadOnlyWalkthrough,
    /// Gated at a checkpoint.
    CheckpointGated,
    /// Self-paced.
    SelfPaced,
    /// Never applies anything hidden.
    NoHiddenApply,
}

impl M5ExerciseValidationMode {
    /// Every exercise validation mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandBacked,
        Self::SandboxedPractice,
        Self::ReadOnlyWalkthrough,
        Self::CheckpointGated,
        Self::SelfPaced,
        Self::NoHiddenApply,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandBacked => "command_backed",
            Self::SandboxedPractice => "sandboxed_practice",
            Self::ReadOnlyWalkthrough => "read_only_walkthrough",
            Self::CheckpointGated => "checkpoint_gated",
            Self::SelfPaced => "self_paced",
            Self::NoHiddenApply => "no_hidden_apply",
        }
    }
}

/// Controlled glossary source class — where a glossary chip or card's definition comes
/// from, so glossary prose never drifts away from cited source truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GlossarySourceClass {
    /// Cited from product docs.
    CitedDocs,
    /// Cited from a spec.
    CitedSpec,
    /// Cited from a help pack.
    CitedHelpPack,
    /// A community note.
    CommunityNote,
    /// An uncited draft.
    UncitedDraft,
    /// An unknown source.
    UnknownSource,
}

impl M5GlossarySourceClass {
    /// Every glossary source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CitedDocs,
        Self::CitedSpec,
        Self::CitedHelpPack,
        Self::CommunityNote,
        Self::UncitedDraft,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CitedDocs => "cited_docs",
            Self::CitedSpec => "cited_spec",
            Self::CitedHelpPack => "cited_help_pack",
            Self::CommunityNote => "community_note",
            Self::UncitedDraft => "uncited_draft",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled glossary citation state — how a glossary chip or card preserves its canonical
/// citation, so a definition never severs or hides how current its citation is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GlossaryCitationState {
    /// Citation is current.
    CitationCurrent,
    /// Citation is version-matched.
    CitationVersioned,
    /// Citation is stale.
    CitationStale,
    /// Citation is cached.
    CitationCached,
    /// Citation is unavailable while offline.
    CitationOfflineUnavailable,
    /// Citation is missing.
    CitationMissing,
}

impl M5GlossaryCitationState {
    /// Every glossary citation state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CitationCurrent,
        Self::CitationVersioned,
        Self::CitationStale,
        Self::CitationCached,
        Self::CitationOfflineUnavailable,
        Self::CitationMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CitationCurrent => "citation_current",
            Self::CitationVersioned => "citation_versioned",
            Self::CitationStale => "citation_stale",
            Self::CitationCached => "citation_cached",
            Self::CitationOfflineUnavailable => "citation_offline_unavailable",
            Self::CitationMissing => "citation_missing",
        }
    }
}

/// Controlled explanation boundary class — how a safe explanation banner separates explain
/// from do, so an explanation never blurs into an unannounced mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationBoundaryClass {
    /// Explains only.
    ExplainOnly,
    /// Explains, then offers to do.
    ExplainThenOfferDo,
    /// A preview is required before doing.
    PreviewRequired,
    /// Approval is required before doing.
    ApprovalRequired,
    /// Only sandboxed action is offered.
    SandboxedOnly,
    /// Nothing is applied without the ordinary preview / approval model.
    NoHiddenApply,
}

impl M5ExplanationBoundaryClass {
    /// Every explanation boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExplainOnly,
        Self::ExplainThenOfferDo,
        Self::PreviewRequired,
        Self::ApprovalRequired,
        Self::SandboxedOnly,
        Self::NoHiddenApply,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::ExplainThenOfferDo => "explain_then_offer_do",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::SandboxedOnly => "sandboxed_only",
            Self::NoHiddenApply => "no_hidden_apply",
        }
    }
}

/// Controlled explanation apply state — what a safe explanation banner will actually do,
/// so an educational surface never widens mutating authority beyond ordinary work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationApplyState {
    /// Applies nothing.
    NoApply,
    /// A preview is available.
    PreviewAvailable,
    /// Approval is pending.
    ApprovalPending,
    /// Applied with undo.
    AppliedWithUndo,
    /// Apply is blocked.
    BlockedApply,
    /// A mutation was declined.
    MutationDeclined,
}

impl M5ExplanationApplyState {
    /// Every explanation apply state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoApply,
        Self::PreviewAvailable,
        Self::ApprovalPending,
        Self::AppliedWithUndo,
        Self::BlockedApply,
        Self::MutationDeclined,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApply => "no_apply",
            Self::PreviewAvailable => "preview_available",
            Self::ApprovalPending => "approval_pending",
            Self::AppliedWithUndo => "applied_with_undo",
            Self::BlockedApply => "blocked_apply",
            Self::MutationDeclined => "mutation_declined",
        }
    }
}

/// Controlled progress ownership class — who owns a progress marker's data, so progress
/// stays user-owned and default-local unless a supported sync / export path is chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProgressOwnershipClass {
    /// Local-only (the default).
    LocalOnly,
    /// User-owned and synced by choice.
    UserOwnedSynced,
    /// Exported by explicit choice.
    ExportedByChoice,
    /// Shared with a workspace by choice.
    WorkspaceShared,
    /// A cached snapshot.
    CachedSnapshot,
    /// Progress tracking is not installed.
    NotInstalled,
}

impl M5ProgressOwnershipClass {
    /// Every progress ownership class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnly,
        Self::UserOwnedSynced,
        Self::ExportedByChoice,
        Self::WorkspaceShared,
        Self::CachedSnapshot,
        Self::NotInstalled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::UserOwnedSynced => "user_owned_synced",
            Self::ExportedByChoice => "exported_by_choice",
            Self::WorkspaceShared => "workspace_shared",
            Self::CachedSnapshot => "cached_snapshot",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Controlled progress state — where a progress marker stands, so progress is never
/// overstated or an offline / local state left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProgressState {
    /// Not started.
    NotStarted,
    /// In progress.
    InProgress,
    /// Completed.
    Completed,
    /// Paused.
    Paused,
    /// Reset.
    Reset,
    /// Offline / local.
    OfflineLocal,
}

impl M5ProgressState {
    /// Every progress state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotStarted,
        Self::InProgress,
        Self::Completed,
        Self::Paused,
        Self::Reset,
        Self::OfflineLocal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Paused => "paused",
            Self::Reset => "reset",
            Self::OfflineLocal => "offline_local",
        }
    }
}

/// Claimed M5 onboarding / learnability surface family that renders / consumes a learning
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningSurfaceFamily {
    /// The first-run onboarding surface.
    FirstRunOnboarding,
    /// The guided-tour surface.
    GuidedTour,
    /// The learning-mode panel surface.
    LearningModePanel,
    /// The glossary surface.
    GlossarySurface,
    /// The inline-help surface.
    InlineHelp,
    /// The CLI help surface.
    CliHelp,
}

impl M5LearningSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstRunOnboarding,
        Self::GuidedTour,
        Self::LearningModePanel,
        Self::GlossarySurface,
        Self::InlineHelp,
        Self::CliHelp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRunOnboarding => "first_run_onboarding",
            Self::GuidedTour => "guided_tour",
            Self::LearningModePanel => "learning_mode_panel",
            Self::GlossarySurface => "glossary_surface",
            Self::InlineHelp => "inline_help",
            Self::CliHelp => "cli_help",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's learning,
/// citation, explanation, or progress truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5LearningDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerSurface {
    /// The onboarding UI.
    OnboardingUi,
    /// The tour-overlay UI.
    TourOverlayUi,
    /// The learning-panel UI.
    LearningPanelUi,
    /// The glossary UI.
    GlossaryUi,
    /// The exercise UI.
    ExerciseUi,
    /// The help-panel UI.
    HelpPanelUi,
    /// The CLI help surface.
    CliHelp,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5LearningConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::OnboardingUi,
        Self::TourOverlayUi,
        Self::LearningPanelUi,
        Self::GlossaryUi,
        Self::ExerciseUi,
        Self::HelpPanelUi,
        Self::CliHelp,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingUi => "onboarding_ui",
            Self::TourOverlayUi => "tour_overlay_ui",
            Self::LearningPanelUi => "learning_panel_ui",
            Self::GlossaryUi => "glossary_ui",
            Self::ExerciseUi => "exercise_ui",
            Self::HelpPanelUi => "help_panel_ui",
            Self::CliHelp => "cli_help",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no learning truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5LearningAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed learning component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about cited source truth, the explain-versus-do boundary, and progress
/// ownership / privacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The cited source truth behind the component.
    CitationSource,
    /// The explain-versus-do boundary the component keeps.
    ExplainVersusDoBoundary,
    /// The ownership and privacy posture of the component's progress.
    ProgressOwnershipAndPrivacy,
}

impl M5LearningRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CitationSource,
        Self::ExplainVersusDoBoundary,
        Self::ProgressOwnershipAndPrivacy,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CitationSource => "citation_source",
            Self::ExplainVersusDoBoundary => "explain_versus_do_boundary",
            Self::ProgressOwnershipAndPrivacy => "progress_ownership_and_privacy",
        }
    }
}

/// Qualification class for an M5 learning-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5LearningQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a learning component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningDowngradeTrigger {
    /// A learning-mode toggle left its learning state unstated.
    LearningModeStateUnstated,
    /// A tip card left its command binding unstated.
    TipCommandBindingUnstated,
    /// A guided exercise step left its step state unstated.
    ExerciseStepStateUnstated,
    /// A glossary chip or card severed its canonical citation.
    GlossaryCitationSevered,
    /// A safe explanation banner left its apply boundary unstated.
    ExplanationApplyBoundaryUnstated,
    /// A progress marker left its ownership posture unstated.
    ProgressOwnershipUnstated,
    /// A component hid its offline / local-only state.
    OfflineOrLocalOnlyStateHidden,
    /// A component left its sandbox boundary unstated.
    SandboxBoundaryUnstated,
    /// A component hid that its content is cached.
    CachedStateHidden,
    /// A component hid that a feature or pack is not installed.
    NotInstalledStateHidden,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5LearningDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::LearningModeStateUnstated,
        Self::TipCommandBindingUnstated,
        Self::ExerciseStepStateUnstated,
        Self::GlossaryCitationSevered,
        Self::ExplanationApplyBoundaryUnstated,
        Self::ProgressOwnershipUnstated,
        Self::OfflineOrLocalOnlyStateHidden,
        Self::SandboxBoundaryUnstated,
        Self::CachedStateHidden,
        Self::NotInstalledStateHidden,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LearningModeStateUnstated => "learning_mode_state_unstated",
            Self::TipCommandBindingUnstated => "tip_command_binding_unstated",
            Self::ExerciseStepStateUnstated => "exercise_step_state_unstated",
            Self::GlossaryCitationSevered => "glossary_citation_severed",
            Self::ExplanationApplyBoundaryUnstated => "explanation_apply_boundary_unstated",
            Self::ProgressOwnershipUnstated => "progress_ownership_unstated",
            Self::OfflineOrLocalOnlyStateHidden => "offline_or_local_only_state_hidden",
            Self::SandboxBoundaryUnstated => "sandbox_boundary_unstated",
            Self::CachedStateHidden => "cached_state_hidden",
            Self::NotInstalledStateHidden => "not_installed_state_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed learning-component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentRow {
    /// Governed component family.
    pub component_family: M5LearningComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5LearningQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 onboarding / learnability surface families that render / consume this
    /// component.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5LearningRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Controlled dispositions this component binds (must be non-empty; drawn from the one
    /// shared [`M5LearningDisposition`] vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Learning-mode states this component names (learning-mode-toggle only).
    pub learning_mode_states: Vec<M5LearningModeState>,
    /// Learning-mode scopes this component names (learning-mode-toggle only).
    pub learning_mode_scopes: Vec<M5LearningModeScope>,
    /// Tip trigger classes this component names (tip-card only).
    pub tip_trigger_classes: Vec<M5TipTriggerClass>,
    /// Tip dismissal states this component names (tip-card only).
    pub tip_dismissal_states: Vec<M5TipDismissalState>,
    /// Exercise step states this component names (guided-exercise-step only).
    pub exercise_step_states: Vec<M5ExerciseStepState>,
    /// Exercise validation modes this component names (guided-exercise-step only).
    pub exercise_validation_modes: Vec<M5ExerciseValidationMode>,
    /// Glossary source classes this component names (glossary-chip-or-card only).
    pub glossary_source_classes: Vec<M5GlossarySourceClass>,
    /// Glossary citation states this component names (glossary-chip-or-card only).
    pub glossary_citation_states: Vec<M5GlossaryCitationState>,
    /// Explanation boundary classes this component names (safe-explanation-banner only).
    pub explanation_boundary_classes: Vec<M5ExplanationBoundaryClass>,
    /// Explanation apply states this component names (safe-explanation-banner only).
    pub explanation_apply_states: Vec<M5ExplanationApplyState>,
    /// Progress ownership classes this component names (progress-marker only).
    pub progress_ownership_classes: Vec<M5ProgressOwnershipClass>,
    /// Progress states this component names (progress-marker only).
    pub progress_states: Vec<M5ProgressState>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its privacy / offline / local-only or
    /// cached state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: this component never hides its cited source. MUST be `false`.
    pub hides_citation_source: bool,
    /// Hard invariant: this component never implies a hidden apply or widened mutating
    /// authority. MUST be `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl M5LearningComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_privacy_or_offline_state
            && !self.hides_citation_source
            && !self.implies_hidden_apply_or_mutation
            && !self.invents_alternate_state_label
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Disposition tokens (the one shared consumer vocabulary).
    pub dispositions: Vec<String>,
    /// Learning-mode-state tokens.
    pub learning_mode_states: Vec<String>,
    /// Learning-mode-scope tokens.
    pub learning_mode_scopes: Vec<String>,
    /// Tip-trigger-class tokens.
    pub tip_trigger_classes: Vec<String>,
    /// Tip-dismissal-state tokens.
    pub tip_dismissal_states: Vec<String>,
    /// Exercise-step-state tokens.
    pub exercise_step_states: Vec<String>,
    /// Exercise-validation-mode tokens.
    pub exercise_validation_modes: Vec<String>,
    /// Glossary-source-class tokens.
    pub glossary_source_classes: Vec<String>,
    /// Glossary-citation-state tokens.
    pub glossary_citation_states: Vec<String>,
    /// Explanation-boundary-class tokens.
    pub explanation_boundary_classes: Vec<String>,
    /// Explanation-apply-state tokens.
    pub explanation_apply_states: Vec<String>,
    /// Progress-ownership-class tokens.
    pub progress_ownership_classes: Vec<String>,
    /// Progress-state tokens.
    pub progress_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5LearningComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5LearningComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5LearningDisposition::ALL, |v| v.as_str()),
            learning_mode_states: tokens(&M5LearningModeState::ALL, |v| v.as_str()),
            learning_mode_scopes: tokens(&M5LearningModeScope::ALL, |v| v.as_str()),
            tip_trigger_classes: tokens(&M5TipTriggerClass::ALL, |v| v.as_str()),
            tip_dismissal_states: tokens(&M5TipDismissalState::ALL, |v| v.as_str()),
            exercise_step_states: tokens(&M5ExerciseStepState::ALL, |v| v.as_str()),
            exercise_validation_modes: tokens(&M5ExerciseValidationMode::ALL, |v| v.as_str()),
            glossary_source_classes: tokens(&M5GlossarySourceClass::ALL, |v| v.as_str()),
            glossary_citation_states: tokens(&M5GlossaryCitationState::ALL, |v| v.as_str()),
            explanation_boundary_classes: tokens(&M5ExplanationBoundaryClass::ALL, |v| v.as_str()),
            explanation_apply_states: tokens(&M5ExplanationApplyState::ALL, |v| v.as_str()),
            progress_ownership_classes: tokens(&M5ProgressOwnershipClass::ALL, |v| v.as_str()),
            progress_states: tokens(&M5ProgressState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5LearningSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5LearningDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5LearningConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5LearningAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5LearningRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5LearningComponentGovernanceReview {
    /// The learning-mode toggle shows its learning state and scope.
    pub toggle_shows_learning_state_and_scope: bool,
    /// The tip card shows its command binding and dismissal state.
    pub tip_card_shows_command_binding_and_dismissal: bool,
    /// The guided exercise step shows its state and never hides an apply.
    pub exercise_step_shows_state_and_no_hidden_apply: bool,
    /// The glossary chip or card shows its cited source and citation state.
    pub glossary_shows_cited_source_and_citation_state: bool,
    /// The safe explanation banner shows its explain-versus-do boundary and never hides an
    /// apply.
    pub banner_shows_explain_versus_do_and_no_hidden_apply: bool,
    /// The progress marker shows its ownership and privacy posture.
    pub progress_marker_shows_ownership_and_privacy: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Learning stays opt-in.
    pub learning_stays_opt_in: bool,
    /// Explain and do stay separate.
    pub explain_and_do_stay_separate: bool,
    /// No component widens trust or mutating authority.
    pub no_component_widens_trust_or_mutating_authority: bool,
    /// Progress is user-owned by default.
    pub progress_user_owned_by_default: bool,
    /// Cached, offline, and local-only states stay visible.
    pub cached_offline_local_only_state_always_visible: bool,
    /// The sandboxed state is always explicit where applicable.
    pub sandboxed_state_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel learning vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerProjection {
    /// Onboarding surfaces consume the shared toggle and tip vocabulary.
    pub onboarding_surfaces_consume_toggle_and_tip_vocabulary: bool,
    /// Guided-learning surfaces consume the exercise vocabulary.
    pub guided_learning_surfaces_consume_exercise_vocabulary: bool,
    /// Glossary surfaces consume the citation vocabulary.
    pub glossary_surfaces_consume_citation_vocabulary: bool,
    /// Explanation surfaces consume the apply-boundary vocabulary.
    pub explanation_surfaces_consume_apply_boundary_vocabulary: bool,
    /// Progress surfaces consume the ownership vocabulary.
    pub progress_surfaces_consume_ownership_vocabulary: bool,
    /// Support / export reads a single canonical learning source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the learning-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting learning-component audit for the lane.
    pub learning_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LearningComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LearningComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5LearningComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LearningComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LearningComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LearningComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LearningComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LearningComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 learning-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentMatrixPacket {
    /// Record kind; must equal [`M5_LEARNING_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LEARNING_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5LearningComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LearningComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LearningComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LearningComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LearningComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LearningComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LearningComponentMatrixPacket {
    /// Builds an M5 learning-component matrix packet from stable-lane input.
    pub fn new(input: M5LearningComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_LEARNING_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_LEARNING_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 learning-component matrix invariants.
    pub fn validate(&self) -> Vec<M5LearningComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LEARNING_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5LearningComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LEARNING_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5LearningComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LearningComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 learning component matrix packet serializes"),
        ) {
            violations.push(M5LearningComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 learning component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,dispositions,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.dispositions, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Learning-Mode-Toggle, Tip-Card, Guided-Exercise-Step, Glossary-Chip-or-Card, Safe-Explanation-Banner, and Progress-Marker Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Dispositions: {}\n",
                row.dispositions
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
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

/// Errors emitted when reading the checked-in M5 learning matrix export.
#[derive(Debug)]
pub enum M5LearningComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LearningComponentMatrixViolation>),
}

impl fmt::Display for M5LearningComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 learning component matrix export parse failed: {error}"
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
                    "m5 learning component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LearningComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5LearningComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LearningComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row declares no dispositions.
    DispositionsMissing,
    /// A learning-mode-toggle component declares no learning-mode states.
    LearningModeStateMissing,
    /// A learning-mode-toggle component declares no learning-mode scopes.
    LearningModeScopeMissing,
    /// A tip-card component declares no tip trigger classes.
    TipTriggerClassMissing,
    /// A tip-card component declares no tip dismissal states.
    TipDismissalStateMissing,
    /// A guided-exercise-step component declares no exercise step states.
    ExerciseStepStateMissing,
    /// A guided-exercise-step component declares no exercise validation modes.
    ExerciseValidationModeMissing,
    /// A glossary-chip-or-card component declares no glossary source classes.
    GlossarySourceClassMissing,
    /// A glossary-chip-or-card component declares no glossary citation states.
    GlossaryCitationStateMissing,
    /// A safe-explanation-banner component declares no explanation boundary classes.
    ExplanationBoundaryClassMissing,
    /// A safe-explanation-banner component declares no explanation apply states.
    ExplanationApplyStateMissing,
    /// A progress-marker component declares no progress ownership classes.
    ProgressOwnershipClassMissing,
    /// A progress-marker component declares no progress states.
    ProgressStateMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked privacy / offline state, hidden cited
    /// source, implied hidden apply / widened mutation, or invented alternate state label).
    ComponentInvariantViolated,
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

impl M5LearningComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::LearningModeStateMissing => "learning_mode_state_missing",
            Self::LearningModeScopeMissing => "learning_mode_scope_missing",
            Self::TipTriggerClassMissing => "tip_trigger_class_missing",
            Self::TipDismissalStateMissing => "tip_dismissal_state_missing",
            Self::ExerciseStepStateMissing => "exercise_step_state_missing",
            Self::ExerciseValidationModeMissing => "exercise_validation_mode_missing",
            Self::GlossarySourceClassMissing => "glossary_source_class_missing",
            Self::GlossaryCitationStateMissing => "glossary_citation_state_missing",
            Self::ExplanationBoundaryClassMissing => "explanation_boundary_class_missing",
            Self::ExplanationApplyStateMissing => "explanation_apply_state_missing",
            Self::ProgressOwnershipClassMissing => "progress_ownership_class_missing",
            Self::ProgressStateMissing => "progress_state_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 learning matrix export.
pub fn current_stable_m5_learning_component_matrix_export(
) -> Result<M5LearningComponentMatrixPacket, M5LearningComponentMatrixArtifactError> {
    let packet: M5LearningComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-proof/support_export.json"
    )))
    .map_err(M5LearningComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LearningComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_TIP_CARD_SCHEMA_REF,
        M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
        M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LearningComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LearningComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    let present: BTreeSet<M5LearningComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5LearningComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5LearningComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5LearningComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5LearningComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::DispositionsMissing);
        }
        if family.is_learning_mode_toggle() && row.learning_mode_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::LearningModeStateMissing);
        }
        if family.is_learning_mode_toggle() && row.learning_mode_scopes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::LearningModeScopeMissing);
        }
        if family.is_tip_card() && row.tip_trigger_classes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::TipTriggerClassMissing);
        }
        if family.is_tip_card() && row.tip_dismissal_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::TipDismissalStateMissing);
        }
        if family.is_guided_exercise_step() && row.exercise_step_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ExerciseStepStateMissing);
        }
        if family.is_guided_exercise_step() && row.exercise_validation_modes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ExerciseValidationModeMissing);
        }
        if family.is_glossary_chip_or_card() && row.glossary_source_classes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::GlossarySourceClassMissing);
        }
        if family.is_glossary_chip_or_card() && row.glossary_citation_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::GlossaryCitationStateMissing);
        }
        if family.is_safe_explanation_banner() && row.explanation_boundary_classes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ExplanationBoundaryClassMissing);
        }
        if family.is_safe_explanation_banner() && row.explanation_apply_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ExplanationApplyStateMissing);
        }
        if family.is_progress_marker() && row.progress_ownership_classes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ProgressOwnershipClassMissing);
        }
        if family.is_progress_marker() && row.progress_states.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ProgressStateMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5LearningComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5LearningComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.toggle_shows_learning_state_and_scope,
        review.tip_card_shows_command_binding_and_dismissal,
        review.exercise_step_shows_state_and_no_hidden_apply,
        review.glossary_shows_cited_source_and_citation_state,
        review.banner_shows_explain_versus_do_and_no_hidden_apply,
        review.progress_marker_shows_ownership_and_privacy,
        review.no_surface_invents_alternate_state_label,
        review.learning_stays_opt_in,
        review.explain_and_do_stay_separate,
        review.no_component_widens_trust_or_mutating_authority,
        review.progress_user_owned_by_default,
        review.cached_offline_local_only_state_always_visible,
        review.sandboxed_state_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5LearningComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.onboarding_surfaces_consume_toggle_and_tip_vocabulary,
        projection.guided_learning_surfaces_consume_exercise_vocabulary,
        projection.glossary_surfaces_consume_citation_vocabulary,
        projection.explanation_surfaces_consume_apply_boundary_vocabulary,
        projection.progress_surfaces_consume_ownership_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5LearningComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LearningComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LearningComponentMatrixPacket,
    violations: &mut Vec<M5LearningComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.learning_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LearningComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
