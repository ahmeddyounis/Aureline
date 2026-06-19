//! Guided-exercise rails for the M5 feature families: a current step, success
//! criteria, target file/surface, hint/reveal/reset/skip controls,
//! command-backed execution, an explicit sandbox/reversible preference, and a
//! strict explain-versus-do separation.
//!
//! Where [`crate::tour_and_glossary_packages`] ships the *content* a learner
//! reads (glossary terms and tour steps that point at stable objects) and
//! [`crate::qualify_learning_mode_guided_tours_and_teaching_sessions`] attaches
//! *qualification verdicts* to opaque surface refs, this module owns the
//! **practice rails themselves**: the ordered steps a learner works through, the
//! success criteria each step is checked against, the inspectable
//! hint/reveal/reset/skip controls, the user-owned progress that survives
//! restart, and the command-backing that routes every apply step through the
//! same command id, preview sheet, approval path, and trust/policy check
//! Aureline uses outside learning mode.
//!
//! ## What a rail carries
//!
//! - **Ordered steps.** Each [`ExerciseStepRecord`] names a [`ExerciseStepKind`]
//!   (explain, prepare-practice, or apply-with-approval), points at one or more
//!   [`StableTargetRef`] target files/surfaces (never coordinates alone), and
//!   carries the [`SuccessCriterion`]s it is checked against.
//! - **Command-backed apply.** An apply step's [`CommandBacking`] proves it reuses
//!   a real command id, preview sheet, approval path, and trust/policy check — not
//!   a tutorial-only shortcut that creates hidden authority.
//! - **Explain versus do.** [`ExerciseStepKind`] and [`MutationTarget`] keep
//!   explain and do separate: an educational step may open docs, preview a diff,
//!   or prepare practice in a sandbox, but a step that is not
//!   [`ExerciseStepKind::ApplyWithApproval`] must never touch real workspace
//!   state.
//! - **Sandbox/reversible preference.** [`SandboxPreference`] records whether the
//!   rail prefers a sandbox, whether reversibility is the default, and that any
//!   real-workspace mutation requires explicit opt-in.
//! - **Inspectable controls.** Each step exposes [`ExerciseAction`]s — at least
//!   hint, reveal, reset, and skip — that are inspectable, keyboard reachable,
//!   restart-safe, and never mutate workspace state.
//! - **User-owned progress.** [`ExerciseProgress`] is local-by-default,
//!   restart-safe, resumable, and never shared with the repository.
//! - **Freshness, locale, and mirror parity.** A rail reuses the
//!   [`FreshnessState`], [`LocaleOverlay`], and [`MirrorParityPosture`] vocabulary
//!   so a cached, mirrored, or local-only rail stays visibly distinct from live
//!   help.
//!
//! ## Invariants enforced
//!
//! - **No silent escalation.** An explain or prepare-practice step that touches
//!   real workspace state narrows below Stable and fails validation.
//! - **Apply is command-backed.** An apply step missing a command id, preview
//!   sheet, approval path, or trust/policy check — or that bypasses the standard
//!   command model — narrows below Stable.
//! - **Reversibility preferred.** A step that mutates real workspace state
//!   irreversibly narrows below Stable.
//! - **Controls are reachable and safe.** A hint/reveal/reset/skip control that
//!   is not inspectable, not keyboard reachable, not restart-safe, or that mutates
//!   workspace state narrows below Stable.
//! - **Progress is user-owned.** Progress that does not survive restart, is not
//!   resumable, is not local-by-default, or is shared with the repository narrows
//!   below Stable.
//! - **Cached never masquerades as live.** A non-live [`FreshnessState`] must be
//!   disclosed and agree with the mirror-parity freshness label.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_guided_exercise_rails`] produces the canonical manifest. Help/About,
//! docs/migration, support export, and release surfaces ingest it rather than
//! rephrasing exercise state by hand.
//!
//! - Schema: [`M5_GUIDED_EXERCISE_RAILS_SCHEMA_REF`]
//! - Fixture: [`M5_GUIDED_EXERCISE_RAILS_FIXTURE_REF`]
//! - Artifact: [`M5_GUIDED_EXERCISE_RAILS_ARTIFACT_REF`]
//! - Doc: [`M5_GUIDED_EXERCISE_RAILS_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_feature_family_learning_rails::{M5LearningSurfaceFamily, MirrorParityPosture};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    AccessibilityPosture, CitationProof, ExplainApplyClass, PrivacyPosture, QualificationVerdict,
    GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};
use crate::tour_and_glossary_packages::{
    citation_refs, fold_freshness_and_parity, fold_locale_overlays, FreshnessState, LocaleOverlay,
    PackageVersion, SourceClass, StableTargetRef, TargetKind, M5_TOUR_AND_GLOSSARY_SCHEMA_REF,
};

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the guided-exercise-rail records. Bumped only on
/// breaking payload changes; additive-optional fields do not bump it.
pub const M5_GUIDED_EXERCISE_RAILS_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`GuidedExerciseRail`].
pub const GUIDED_EXERCISE_RAIL_RECORD_KIND: &str = "guided_exercise_rail_record";

/// Record kind for [`M5GuidedExerciseRailManifest`].
pub const M5_GUIDED_EXERCISE_RAIL_MANIFEST_RECORD_KIND: &str =
    "m5_guided_exercise_rail_manifest_record";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the boundary schema.
pub const M5_GUIDED_EXERCISE_RAILS_SCHEMA_REF: &str =
    "schemas/help/m5-guided-exercise-rails.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_GUIDED_EXERCISE_RAILS_FIXTURE_REF: &str =
    "fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json";

/// Repository-relative path to the proof artifact.
pub const M5_GUIDED_EXERCISE_RAILS_ARTIFACT_REF: &str =
    "artifacts/ux/m5/guided-exercise-proof/ship-guided-exercise-rails.md";

/// Repository-relative path to the public doc.
pub const M5_GUIDED_EXERCISE_RAILS_DOC_REF: &str = "docs/help/m5/guided-exercise-rails.md";

// ── Step kind ────────────────────────────────────────────────────────────────

/// What a step asks the learner to do — the explain-versus-do axis.
///
/// Explain and prepare-practice steps stay on the *explain* side of the line:
/// they may open docs, preview a diff, or set up a sandbox, but they must never
/// touch real workspace state. Only an [`ExerciseStepKind::ApplyWithApproval`]
/// step may mutate the real workspace, and only through the standard
/// command/preview/approval path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStepKind {
    /// Read-only: open docs, narrate a concept, or preview a diff. No mutation.
    Explain,
    /// Prepare practice in a sandbox/scratch space. Reversible, local-only; never
    /// touches the real workspace.
    PreparePractice,
    /// Apply a change to the real workspace through the standard
    /// command/preview/approval path.
    ApplyWithApproval,
}

impl ExerciseStepKind {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explain => "explain",
            Self::PreparePractice => "prepare_practice",
            Self::ApplyWithApproval => "apply_with_approval",
        }
    }

    /// Whether this kind is permitted to mutate the real workspace.
    pub const fn is_apply_capable(self) -> bool {
        matches!(self, Self::ApplyWithApproval)
    }
}

// ── Mutation target ──────────────────────────────────────────────────────────

/// The blast radius a step actually touches, used to label sandboxed,
/// local-only, reversible, and real-workspace steps explicitly.
///
/// A learner is never silently handed a wider blast radius than the step's label
/// implies: a step that touches real workspace state says so, and a step that is
/// reversible or sandboxed says so too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationTarget {
    /// The step performs no mutation (explain / read-only / preview).
    NoMutation,
    /// The step mutates a sandbox/scratch space only — local-only and reversible.
    SandboxedLocalReversible,
    /// The step touches real workspace state, reversibly, through preview/approval.
    WorkspaceReversibleApproved,
    /// The step touches real workspace state irreversibly through preview/approval.
    /// Honest, but narrows below Stable because reversibility cannot be proved.
    WorkspaceIrreversibleApproved,
}

impl MutationTarget {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::SandboxedLocalReversible => "sandboxed_local_reversible",
            Self::WorkspaceReversibleApproved => "workspace_reversible_approved",
            Self::WorkspaceIrreversibleApproved => "workspace_irreversible_approved",
        }
    }

    /// Whether the step touches real (non-sandbox) workspace state.
    pub const fn touches_real_workspace(self) -> bool {
        matches!(
            self,
            Self::WorkspaceReversibleApproved | Self::WorkspaceIrreversibleApproved
        )
    }

    /// Whether the effect is reversible (or there is no effect at all).
    pub const fn is_reversible(self) -> bool {
        matches!(
            self,
            Self::NoMutation | Self::SandboxedLocalReversible | Self::WorkspaceReversibleApproved
        )
    }

    /// Whether the step performs any mutation at all.
    pub const fn is_mutation(self) -> bool {
        !matches!(self, Self::NoMutation)
    }

    /// Whether the target qualifies Stable. An irreversible real-workspace
    /// mutation cannot, because reversibility is preferred wherever possible.
    pub const fn qualifies_stable(self) -> bool {
        self.is_reversible()
    }
}

// ── Success criterion ────────────────────────────────────────────────────────

/// The kind of deterministic check a [`SuccessCriterion`] performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuccessCriterionKind {
    /// A target file object exists.
    FileExists,
    /// A target file's content matches an expected shape.
    FileContentMatches,
    /// A target symbol object is present.
    SymbolPresent,
    /// A target command ran (through the standard command model).
    CommandRan,
    /// A command/run produced an expected output.
    OutputMatches,
    /// A docs/help node was opened (explain-side acknowledgement).
    DocsOpened,
    /// A target surface reached an expected state.
    SurfaceStateReached,
    /// A diff was previewed (explain-side; no apply).
    DiffPreviewed,
}

impl SuccessCriterionKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileExists => "file_exists",
            Self::FileContentMatches => "file_content_matches",
            Self::SymbolPresent => "symbol_present",
            Self::CommandRan => "command_ran",
            Self::OutputMatches => "output_matches",
            Self::DocsOpened => "docs_opened",
            Self::SurfaceStateReached => "surface_state_reached",
            Self::DiffPreviewed => "diff_previewed",
        }
    }
}

/// One deterministic, inspectable check a step is measured against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessCriterion {
    /// Stable id for this criterion.
    pub criterion_id: String,
    /// What kind of check this criterion performs.
    pub criterion_kind: SuccessCriterionKind,
    /// Opaque ref to the deterministic check that evaluates this criterion.
    pub check_ref: String,
    /// The stable object this criterion checks, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<StableTargetRef>,
    /// Opaque ref to the base-locale description copy.
    pub description_ref: String,
}

// ── Command backing ──────────────────────────────────────────────────────────

/// Proof that a step routes through the same governed command model Aureline uses
/// outside learning mode.
///
/// An apply step MUST reuse a real command id, a preview sheet, an approval path,
/// and a trust/policy check — never a tutorial-only shortcut that would create
/// hidden authority or a mutating bypass. Explain and prepare-practice steps use
/// [`CommandBacking::none`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandBacking {
    /// Opaque ref to the stable command id this step runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Opaque ref to the preview sheet shown before any mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_sheet_ref: Option<String>,
    /// Opaque ref to the approval path the mutation passes through.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_path_ref: Option<String>,
    /// Opaque ref to the trust/policy check applied before the command runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_policy_check_ref: Option<String>,
    /// Whether the step reuses the standard command model (not a tutorial-only
    /// shortcut). MUST be true for an apply step.
    pub uses_standard_command_model: bool,
    /// Named reason when the backing is inadequate for an apply step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl CommandBacking {
    /// Backing for a non-applying step: no command, preview, approval, or policy
    /// check, and not routed through the apply model.
    pub fn none() -> Self {
        Self {
            command_id_ref: None,
            preview_sheet_ref: None,
            approval_path_ref: None,
            trust_policy_check_ref: None,
            uses_standard_command_model: false,
            narrowing_reason: None,
        }
    }

    /// Whether the backing is complete enough for an apply step: it reuses the
    /// standard command model and names a command id, preview sheet, approval
    /// path, and trust/policy check.
    pub fn qualifies_for_apply(&self) -> bool {
        self.uses_standard_command_model
            && self.command_id_ref.is_some()
            && self.preview_sheet_ref.is_some()
            && self.approval_path_ref.is_some()
            && self.trust_policy_check_ref.is_some()
    }
}

// ── Sandbox / reversible preference ──────────────────────────────────────────

/// A rail's sandbox and reversibility preference.
///
/// Where a sandbox is possible, a rail prefers practising in it. Where it is not,
/// the rail must still keep effects reversible by default and require explicit
/// opt-in before touching real workspace state — so a learner is never silently
/// stepped from a safe sandbox into a live mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxPreference {
    /// Whether the rail prefers to run practice in a sandbox/scratch space.
    pub prefers_sandbox: bool,
    /// Whether a sandbox/scratch space is available for this rail.
    pub sandbox_available: bool,
    /// Whether effects are reversible by default.
    pub reversible_by_default: bool,
    /// Whether any real-workspace mutation requires explicit opt-in.
    pub real_workspace_mutation_requires_explicit_optin: bool,
    /// Named reason when the preference posture is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl SandboxPreference {
    /// Whether the preference satisfies Stable requirements: reversible by default
    /// and real-workspace mutation gated behind explicit opt-in.
    pub fn qualifies_stable(&self) -> bool {
        self.reversible_by_default && self.real_workspace_mutation_requires_explicit_optin
    }
}

// ── Exercise actions (hint / reveal / reset / skip) ──────────────────────────

/// One inspectable control a learner can invoke on a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseActionKind {
    /// Reveal a progressive hint without giving away the answer.
    Hint,
    /// Reveal the full solution / answer for the step.
    Reveal,
    /// Reset the step to its starting state.
    Reset,
    /// Skip the step and move on.
    Skip,
    /// Advance to the next step.
    Next,
    /// Return to the previous step.
    Previous,
}

impl ExerciseActionKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hint => "hint",
            Self::Reveal => "reveal",
            Self::Reset => "reset",
            Self::Skip => "skip",
            Self::Next => "next",
            Self::Previous => "previous",
        }
    }
}

/// The control kinds every step MUST expose: hint, reveal, reset, and skip.
pub const REQUIRED_ACTION_KINDS: [ExerciseActionKind; 4] = [
    ExerciseActionKind::Hint,
    ExerciseActionKind::Reveal,
    ExerciseActionKind::Reset,
    ExerciseActionKind::Skip,
];

/// One inspectable, keyboard-reachable, restart-safe control bound to a step.
///
/// Controls are learning affordances, never mutating shortcuts: a hint, reveal,
/// reset, or skip MUST NOT mutate workspace state, and MUST be inspectable (not
/// trapped inside a modal tutorial), keyboard reachable, and restart-safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseAction {
    /// Which control this is.
    pub action_kind: ExerciseActionKind,
    /// Opaque ref to the base-locale control label.
    pub label_ref: String,
    /// Opaque ref to the keyboard shortcut binding. MUST be present (keyboard
    /// reachable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut_ref: Option<String>,
    /// Opaque ref to the stable command id this control routes through, when it
    /// is command-backed (e.g. reset/skip emit a learning command).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Whether the control is inspectable in the action log (not trapped inside a
    /// modal tutorial).
    pub inspectable: bool,
    /// Whether invoking the control is restart-safe.
    pub restart_safe: bool,
    /// Whether the control mutates workspace state. MUST be false — controls are
    /// learning affordances, never mutating shortcuts.
    pub mutates_workspace: bool,
    /// Named reason when the control fails an invariant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl ExerciseAction {
    /// Whether the control satisfies Stable requirements: inspectable, keyboard
    /// reachable, restart-safe, and non-mutating.
    pub fn qualifies_stable(&self) -> bool {
        self.inspectable
            && self.keyboard_shortcut_ref.is_some()
            && self.restart_safe
            && !self.mutates_workspace
    }
}

// ── Progress ─────────────────────────────────────────────────────────────────

/// User-owned, restart-safe progress for one rail.
///
/// Progress is local-by-default and never shared with the repository or
/// collaborators implicitly. It survives restart and can be resumed mid-exercise,
/// so a learner is never trapped at the start of a tutorial after closing the
/// app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseProgress {
    /// Zero-based index of the current step.
    pub current_step_index: u32,
    /// Ids of steps the learner has completed.
    #[serde(default)]
    pub completed_step_ids: Vec<String>,
    /// Ids of steps the learner has skipped.
    #[serde(default)]
    pub skipped_step_ids: Vec<String>,
    /// Opaque ref to the locally-persisted progress record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_state_ref: Option<String>,
    /// Whether progress survives an application restart.
    pub survives_restart: bool,
    /// Whether the learner can resume mid-exercise.
    pub resumable: bool,
    /// Whether progress is stored locally and owned by the user.
    pub user_owned_local: bool,
    /// Whether progress is shared with the repository. MUST be false.
    pub shared_with_repo: bool,
    /// Named reason when the progress posture is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl ExerciseProgress {
    /// A fresh, resumable progress snapshot positioned at the first step, with the
    /// given local progress-state ref.
    pub fn fresh(progress_state_ref: impl Into<String>) -> Self {
        Self {
            current_step_index: 0,
            completed_step_ids: Vec::new(),
            skipped_step_ids: Vec::new(),
            progress_state_ref: Some(progress_state_ref.into()),
            survives_restart: true,
            resumable: true,
            user_owned_local: true,
            shared_with_repo: false,
            narrowing_reason: None,
        }
    }

    /// Whether the posture satisfies Stable requirements: restart-safe, resumable,
    /// user-owned/local, and never repo-shared.
    pub fn qualifies_stable(&self) -> bool {
        self.survives_restart && self.resumable && self.user_owned_local && !self.shared_with_repo
    }
}

// ── Exercise step ────────────────────────────────────────────────────────────

/// One step of a guided-exercise rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseStepRecord {
    /// Stable id for this step, used as the locale-overlay key.
    pub step_id: String,
    /// Zero-based position of the step within the rail.
    pub position_index: u32,
    /// Opaque ref to the base-locale step title.
    pub title_ref: String,
    /// Explain-versus-do kind of this step.
    pub step_kind: ExerciseStepKind,
    /// Stable target refs the step points at — the target file/surface (at least
    /// one; no coordinate-only steps).
    pub stable_targets: Vec<StableTargetRef>,
    /// Success criteria the step is checked against (at least one).
    pub success_criteria: Vec<SuccessCriterion>,
    /// Citation proof for the step.
    pub citation: CitationProof,
    /// Explain-vs-apply separation class for the step.
    pub explain_apply_class: ExplainApplyClass,
    /// The blast radius the step touches.
    pub mutation_target: MutationTarget,
    /// Command-backing proof (complete for apply steps;
    /// [`CommandBacking::none`] for explain/prepare steps).
    pub command_backing: CommandBacking,
    /// Opaque refs to progressive hints, in reveal order.
    #[serde(default)]
    pub hint_refs: Vec<String>,
    /// Opaque ref to the full reveal/solution, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reveal_ref: Option<String>,
    /// Inspectable controls bound to this step (hint/reveal/reset/skip, plus
    /// optional navigation).
    pub actions: Vec<ExerciseAction>,
}

impl ExerciseStepRecord {
    /// Whether the step relies on brittle coordinates alone (no stable target).
    pub fn relies_on_coordinates_only(&self) -> bool {
        self.stable_targets.is_empty()
    }

    /// The set of control kinds this step exposes.
    pub fn action_kinds(&self) -> BTreeSet<ExerciseActionKind> {
        self.actions.iter().map(|a| a.action_kind).collect()
    }

    /// Whether the step keeps explain and do separate — i.e. an explain or
    /// prepare-practice step never touches real workspace state.
    pub fn explain_do_separated(&self) -> bool {
        match self.step_kind {
            ExerciseStepKind::Explain | ExerciseStepKind::PreparePractice => {
                !self.mutation_target.touches_real_workspace()
            }
            ExerciseStepKind::ApplyWithApproval => true,
        }
    }
}

// ── Guided exercise rail ─────────────────────────────────────────────────────

/// A versioned guided-exercise rail for one feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedExerciseRail {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this rail.
    pub rail_id: String,
    /// Version and revision identity.
    pub version: PackageVersion,
    /// Feature family this rail serves.
    pub family: M5LearningSurfaceFamily,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the base-locale rail title.
    pub title_ref: String,
    /// Where the rail's content originates.
    pub source_class: SourceClass,
    /// Opaque ref to the source revision.
    pub source_ref: String,
    /// Freshness state of the rail's content.
    pub freshness_state: FreshnessState,
    /// Offline/mirror parity posture.
    pub mirror_parity: MirrorParityPosture,
    /// Base locale the steps are authored in.
    pub base_locale: String,
    /// Localization overlays for additional locales.
    #[serde(default)]
    pub locale_overlays: Vec<LocaleOverlay>,
    /// Rail-level explain-vs-apply posture.
    pub explain_apply_class: ExplainApplyClass,
    /// Sandbox/reversible preference for the rail.
    pub sandbox_preference: SandboxPreference,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// User-owned progress state.
    pub progress: ExerciseProgress,
    /// Ids of rails that should be completed first (in this manifest's namespace).
    #[serde(default)]
    pub prerequisite_rail_refs: Vec<String>,
    /// Ordered steps.
    pub steps: Vec<ExerciseStepRecord>,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl GuidedExerciseRail {
    /// Recomputes [`verdict`](Self::verdict) and
    /// [`narrowing_reasons`](Self::narrowing_reasons) from current evidence.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_exercise_rail_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }

    /// The step the learner's progress currently points at, if it resolves.
    pub fn current_step(&self) -> Option<&ExerciseStepRecord> {
        self.steps.get(self.progress.current_step_index as usize)
    }

    /// The set of every stable target id this rail references.
    ///
    /// This fingerprint is invariant under localization, export, and reopen — it
    /// is how target identity is proved preserved across those operations.
    pub fn target_ref_fingerprint(&self) -> BTreeSet<String> {
        self.steps
            .iter()
            .flat_map(|s| s.stable_targets.iter().map(|t| t.target_id.clone()))
            .collect()
    }

    /// The set of every citation anchor this rail references.
    pub fn citation_ref_fingerprint(&self) -> BTreeSet<String> {
        self.steps
            .iter()
            .flat_map(|s| citation_refs(&s.citation))
            .collect()
    }

    /// The localized display-label map for `locale_tag`, if an overlay exists.
    pub fn localized_labels(&self, locale_tag: &str) -> Option<&BTreeMap<String, String>> {
        self.locale_overlays
            .iter()
            .find(|o| o.locale_tag == locale_tag)
            .map(|o| &o.localized_label_refs)
    }
}

// ── Verdict derivation ───────────────────────────────────────────────────────

/// Derives a rail's verdict and narrowing reasons from its evidence.
///
/// A rail qualifies Stable only when its freshness is live (or disclosed
/// mirror-synced), its mirror parity holds, every locale overlay preserves
/// identity and citations, its sandbox/reversible preference and privacy and
/// progress postures hold, its accessibility keeps reset/skip keyboard reachable,
/// and every step points at a stable object, carries a success criterion and a
/// live citation, keeps explain and do separate, routes any apply through the
/// standard command model, stays reversible, and exposes hint/reveal/reset/skip
/// controls that are inspectable, keyboard reachable, restart-safe, and
/// non-mutating.
pub fn derive_exercise_rail_verdict(
    rail: &GuidedExerciseRail,
) -> (QualificationVerdict, Vec<String>) {
    let label = &rail.rail_id;
    let mut verdict = QualificationVerdict::QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    fold_freshness_and_parity(
        label,
        rail.freshness_state,
        &rail.mirror_parity,
        &mut verdict,
        &mut reasons,
    );
    fold_locale_overlays(label, &rail.locale_overlays, &mut verdict, &mut reasons);

    if !rail.sandbox_preference.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &rail.sandbox_preference.narrowing_reason {
            reasons.push(format!("{label}: sandbox_preference: {r}"));
        } else {
            reasons.push(format!("{label}: sandbox_preference_inadequate"));
        }
    }

    if !rail.privacy.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &rail.privacy.narrowing_reason {
            reasons.push(format!("{label}: privacy: {r}"));
        } else {
            reasons.push(format!("{label}: privacy_posture_incomplete"));
        }
    }

    if !rail.progress.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &rail.progress.narrowing_reason {
            reasons.push(format!("{label}: progress: {r}"));
        } else {
            reasons.push(format!("{label}: progress_not_user_owned_restart_safe"));
        }
    }

    if !rail.explain_apply_class.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: explain_apply_conflated: {}",
            rail.explain_apply_class.as_str()
        ));
    }

    if !rail.accessibility.keyboard_reachable {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: not_keyboard_reachable"));
    }
    if !rail.accessibility.reset_skip_keyboard_accessible {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: reset_skip_not_keyboard_accessible"));
    }

    for step in &rail.steps {
        fold_step(label, step, &mut verdict, &mut reasons);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

/// Folds one step's evidence into the running rail verdict.
fn fold_step(
    label: &str,
    step: &ExerciseStepRecord,
    verdict: &mut QualificationVerdict,
    reasons: &mut Vec<String>,
) {
    let sid = &step.step_id;

    if step.relies_on_coordinates_only() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: step[{sid}]_coordinate_only_no_stable_target"
        ));
    }

    if step.success_criteria.is_empty() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: step[{sid}]_no_success_criterion"));
    }

    if !step.citation.has_citation {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: step[{sid}]_no_citation"));
    } else if !step.citation.all_anchors_live_authoritative {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: step[{sid}]_citation_not_live"));
    }

    if !step.explain_apply_class.qualifies_stable() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: step[{sid}]_explain_apply_conflated"));
    }

    if !step.explain_do_separated() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: step[{sid}]_educational_step_escalates_to_real_workspace"
        ));
    }

    if !step.mutation_target.qualifies_stable() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: step[{sid}]_mutation_irreversible: {}",
            step.mutation_target.as_str()
        ));
    }

    if step.step_kind.is_apply_capable() && !step.command_backing.qualifies_for_apply() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &step.command_backing.narrowing_reason {
            reasons.push(format!(
                "{label}: step[{sid}]_apply_not_command_backed: {r}"
            ));
        } else {
            reasons.push(format!("{label}: step[{sid}]_apply_not_command_backed"));
        }
    }

    let kinds = step.action_kinds();
    for required in REQUIRED_ACTION_KINDS {
        if !kinds.contains(&required) {
            *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: step[{sid}]_missing_action_{}",
                required.as_str()
            ));
        }
    }
    for action in &step.actions {
        if !action.qualifies_stable() {
            *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            if let Some(r) = &action.narrowing_reason {
                reasons.push(format!(
                    "{label}: step[{sid}]_action[{}]: {r}",
                    action.action_kind.as_str()
                ));
            } else {
                reasons.push(format!(
                    "{label}: step[{sid}]_action[{}]_not_inspectable_keyboard_or_restart_safe",
                    action.action_kind.as_str()
                ));
            }
        }
    }
}

// ── Manifest ─────────────────────────────────────────────────────────────────

/// The canonical manifest binding every guided-exercise rail across the M5
/// feature families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GuidedExerciseRailManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this manifest.
    pub manifest_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Schema, docs, and contract refs this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Guided-exercise rails.
    pub rails: Vec<GuidedExerciseRail>,
    /// Overall derived verdict — the strictest verdict across all rails.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across rails (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5GuidedExerciseRailManifest {
    /// Recomputes every rail verdict and the overall verdict from current
    /// evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();
        for rail in &mut self.rails {
            rail.sync_verdict();
            overall = overall.meet(rail.verdict);
            reasons.extend(rail.narrowing_reasons.iter().cloned());
        }
        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the rail with `rail_id`, if present.
    pub fn rail(&self, rail_id: &str) -> Option<&GuidedExerciseRail> {
        self.rails.iter().find(|r| r.rail_id == rail_id)
    }

    /// The set of every rail id the manifest defines.
    pub fn known_rail_ids(&self) -> BTreeSet<String> {
        self.rails.iter().map(|r| r.rail_id.clone()).collect()
    }
}

/// Reopens a manifest from its exported JSON form.
///
/// This is the round-trip used to prove a rail survives export and reopen without
/// losing citations or target identity: the reopened manifest is structurally
/// equal to the original, and so are its target/citation fingerprints.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_manifest_from_json(
    json: &str,
) -> Result<M5GuidedExerciseRailManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Validation ───────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_guided_exercise_rails`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuidedExerciseRailValidationError {
    /// Opaque id of the rail or step that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for GuidedExerciseRailValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// In-manifest prerequisite id prefix; refs with this prefix MUST resolve to a
/// present rail.
const RAIL_PREREQ_PREFIX: &str = "learning:m5:exercise_rail:";

/// Validates a manifest against the guided-exercise-rail invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any rail's stored verdict diverges from the
/// verdict derived from its evidence; when a non-live freshness state is not
/// disclosed or disagrees with the mirror-parity label; when a locale overlay
/// drops target identity or citations; when a rail has no steps or its progress
/// points past its last step; when a step relies on coordinates alone, has no
/// success criterion, is not citation-backed, conflates explain/apply, escalates
/// an educational step into a real-workspace mutation, mutates the real workspace
/// irreversibly, runs an apply that is not routed through the standard command
/// model, is missing a hint/reveal/reset/skip control, or exposes a control that
/// is not inspectable/keyboard-reachable/restart-safe or that mutates workspace
/// state; or when a prerequisite ref into the manifest's own namespace fails to
/// resolve or forms a cycle.
pub fn validate_m5_guided_exercise_rails(
    manifest: &M5GuidedExerciseRailManifest,
) -> Result<(), Vec<GuidedExerciseRailValidationError>> {
    let mut errors: Vec<GuidedExerciseRailValidationError> = Vec::new();
    let known_ids = manifest.known_rail_ids();

    for rail in &manifest.rails {
        let subject = rail.rail_id.clone();

        let (derived, _) = derive_exercise_rail_verdict(rail);
        if derived != rail.verdict {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    rail.verdict, derived
                ),
            });
        }

        check_freshness_parity(
            &subject,
            rail.freshness_state,
            &rail.mirror_parity,
            &mut errors,
        );
        check_locale_overlays(&subject, &rail.locale_overlays, &mut errors);
        check_prerequisites(
            &subject,
            &rail.prerequisite_rail_refs,
            &known_ids,
            &mut errors,
        );

        if rail.explain_apply_class == ExplainApplyClass::Conflated {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.clone(),
                message: "rail conflates explain/apply".to_string(),
            });
        }

        if rail.steps.is_empty() {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.clone(),
                message: "rail has no steps".to_string(),
            });
        } else if rail.progress.current_step_index as usize >= rail.steps.len() {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "progress current_step_index {} is past the last step",
                    rail.progress.current_step_index
                ),
            });
        }

        for step in &rail.steps {
            check_step(step, &mut errors);
        }
    }

    if let Some(cycle) = detect_prerequisite_cycle(manifest) {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: cycle,
            message: "prerequisite cycle detected".to_string(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_step(step: &ExerciseStepRecord, errors: &mut Vec<GuidedExerciseRailValidationError>) {
    let sid = step.step_id.clone();

    if step.relies_on_coordinates_only() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "step relies on coordinates alone (no stable target)".to_string(),
        });
    }
    if step.success_criteria.is_empty() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "step has no success criterion".to_string(),
        });
    }
    if !step.citation.has_citation {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "step is not citation-backed".to_string(),
        });
    }
    if step.explain_apply_class == ExplainApplyClass::Conflated {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "step conflates explain/apply".to_string(),
        });
    }
    if !step.explain_do_separated() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "educational step escalates into a real-workspace mutation".to_string(),
        });
    }
    if !step.mutation_target.qualifies_stable() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "step mutates real workspace state irreversibly".to_string(),
        });
    }
    if step.step_kind.is_apply_capable() && !step.command_backing.qualifies_for_apply() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: sid.clone(),
            message: "apply step is not routed through the standard command/preview/approval model"
                .to_string(),
        });
    }

    let kinds = step.action_kinds();
    for required in REQUIRED_ACTION_KINDS {
        if !kinds.contains(&required) {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: sid.clone(),
                message: format!(
                    "step is missing a keyboard-reachable {} control",
                    required.as_str()
                ),
            });
        }
    }
    for action in &step.actions {
        if action.mutates_workspace {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: sid.clone(),
                message: format!(
                    "{} control mutates workspace state (must be a learning-only affordance)",
                    action.action_kind.as_str()
                ),
            });
        }
        if action.keyboard_shortcut_ref.is_none() {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: sid.clone(),
                message: format!(
                    "{} control is not keyboard reachable",
                    action.action_kind.as_str()
                ),
            });
        }
        if !action.inspectable {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: sid.clone(),
                message: format!(
                    "{} control is not inspectable (trapped in a modal tutorial)",
                    action.action_kind.as_str()
                ),
            });
        }
        if !action.restart_safe {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: sid.clone(),
                message: format!(
                    "{} control is not restart-safe",
                    action.action_kind.as_str()
                ),
            });
        }
    }
}

fn check_freshness_parity(
    subject: &str,
    freshness: FreshnessState,
    parity: &MirrorParityPosture,
    errors: &mut Vec<GuidedExerciseRailValidationError>,
) {
    if parity.freshness_label != freshness.as_str() {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: subject.to_string(),
            message: format!(
                "freshness state {} disagrees with mirror-parity label {}",
                freshness.as_str(),
                parity.freshness_label
            ),
        });
    }
    if freshness.requires_disclosure() && !parity.explicit_freshness_disclosed {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: subject.to_string(),
            message: format!(
                "non-live freshness {} is not explicitly disclosed (would masquerade as live)",
                freshness.as_str()
            ),
        });
    }
    if parity.silent_dead_link_on_stale {
        errors.push(GuidedExerciseRailValidationError {
            subject_id: subject.to_string(),
            message: "rail shows a silent dead link when stale/offline".to_string(),
        });
    }
}

fn check_locale_overlays(
    subject: &str,
    overlays: &[LocaleOverlay],
    errors: &mut Vec<GuidedExerciseRailValidationError>,
) {
    let mut seen = BTreeSet::new();
    for overlay in overlays {
        if !seen.insert(overlay.locale_tag.clone()) {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.to_string(),
                message: format!("duplicate locale overlay {}", overlay.locale_tag),
            });
        }
        if !overlay.preserves_target_identity {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.to_string(),
                message: format!(
                    "locale overlay {} drops target identity",
                    overlay.locale_tag
                ),
            });
        }
        if !overlay.preserves_citations {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.to_string(),
                message: format!("locale overlay {} drops citations", overlay.locale_tag),
            });
        }
    }
}

fn check_prerequisites(
    subject: &str,
    prerequisite_refs: &[String],
    known_ids: &BTreeSet<String>,
    errors: &mut Vec<GuidedExerciseRailValidationError>,
) {
    for prereq in prerequisite_refs {
        if prereq.starts_with(RAIL_PREREQ_PREFIX) && !known_ids.contains(prereq) {
            errors.push(GuidedExerciseRailValidationError {
                subject_id: subject.to_string(),
                message: format!("unresolved prerequisite {prereq}"),
            });
        }
    }
}

/// Detects a cycle in the in-manifest prerequisite graph, returning the id of a
/// node on the cycle when one exists.
fn detect_prerequisite_cycle(manifest: &M5GuidedExerciseRailManifest) -> Option<String> {
    let known = manifest.known_rail_ids();
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for rail in &manifest.rails {
        edges.insert(
            rail.rail_id.clone(),
            rail.prerequisite_rail_refs
                .iter()
                .filter(|r| known.contains(*r))
                .cloned()
                .collect(),
        );
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Grey,
        Black,
    }
    let mut color: BTreeMap<String, Color> =
        edges.keys().map(|k| (k.clone(), Color::White)).collect();

    for start in edges.keys() {
        if color[start] != Color::White {
            continue;
        }
        let mut stack: Vec<(String, usize)> = vec![(start.clone(), 0)];
        color.insert(start.clone(), Color::Grey);
        while let Some((node, idx)) = stack.last().cloned() {
            let neighbors = edges.get(&node).cloned().unwrap_or_default();
            if idx < neighbors.len() {
                stack.last_mut().unwrap().1 += 1;
                let next = neighbors[idx].clone();
                match color.get(&next).copied().unwrap_or(Color::Black) {
                    Color::Grey => return Some(next),
                    Color::White => {
                        color.insert(next.clone(), Color::Grey);
                        stack.push((next, 0));
                    }
                    Color::Black => {}
                }
            } else {
                color.insert(node.clone(), Color::Black);
                stack.pop();
            }
        }
    }
    None
}

// ── Seeded corpus ────────────────────────────────────────────────────────────

const GENERATED_AT: &str = "2026-06-19T14:00:00Z";

fn rail_id(family: M5LearningSurfaceFamily) -> String {
    format!("{RAIL_PREREQ_PREFIX}{}:v1", family.as_str())
}

fn version(family: M5LearningSurfaceFamily) -> PackageVersion {
    PackageVersion {
        version_ref: format!("ver:m5:exercise_rail:{}:1.0.0", family.as_str()),
        revision_ref: format!("rev:m5:exercise_rail:{}:2026.06.19", family.as_str()),
    }
}

fn stable_parity(freshness: FreshnessState) -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: true,
        freshness_label: freshness.as_str().to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: None,
    }
}

fn local_only_parity() -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: false,
        freshness_label: FreshnessState::LocalOnlyDisclosed.as_str().to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: Some("exercise_rail_not_yet_mirror_synced".to_string()),
    }
}

fn local_privacy() -> PrivacyPosture {
    PrivacyPosture {
        progress_local_by_default: true,
        explicit_promotion_required_for_sharing: true,
        repo_visible: false,
        telemetry_grade_read_access: false,
        narrowing_reason: None,
    }
}

fn full_accessibility() -> AccessibilityPosture {
    AccessibilityPosture {
        keyboard_reachable: true,
        screen_reader_narration: true,
        reset_skip_keyboard_accessible: true,
        offline_degradation_accessible: true,
        reduced_motion_honored: true,
    }
}

/// A sandbox-preferring posture: a sandbox is available, effects are reversible by
/// default, and real-workspace mutation requires explicit opt-in.
fn sandbox_preferred() -> SandboxPreference {
    SandboxPreference {
        prefers_sandbox: true,
        sandbox_available: true,
        reversible_by_default: true,
        real_workspace_mutation_requires_explicit_optin: true,
        narrowing_reason: None,
    }
}

/// A posture for a rail where no sandbox is available but effects stay reversible
/// by default and real-workspace mutation requires explicit opt-in.
fn no_sandbox_but_reversible() -> SandboxPreference {
    SandboxPreference {
        prefers_sandbox: true,
        sandbox_available: false,
        reversible_by_default: true,
        real_workspace_mutation_requires_explicit_optin: true,
        narrowing_reason: None,
    }
}

fn live_citation(commands: &[&str], anchors: &[&str]) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: commands.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: true,
        narrowing_reason: None,
    }
}

fn cached_citation(commands: &[&str], anchors: &[&str]) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: commands.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: false,
        narrowing_reason: Some("anchors_cached_not_live_authoritative".to_string()),
    }
}

/// Builds the four required learning controls (hint/reveal/reset/skip) plus a next
/// control, all inspectable, keyboard reachable, restart-safe, and non-mutating.
fn standard_actions(step_token: &str) -> Vec<ExerciseAction> {
    [
        (ExerciseActionKind::Hint, "cmd:learning.exercise.hint"),
        (ExerciseActionKind::Reveal, "cmd:learning.exercise.reveal"),
        (ExerciseActionKind::Reset, "cmd:learning.exercise.reset"),
        (ExerciseActionKind::Skip, "cmd:learning.exercise.skip"),
        (ExerciseActionKind::Next, "cmd:learning.exercise.next"),
    ]
    .into_iter()
    .map(|(kind, command)| ExerciseAction {
        action_kind: kind,
        label_ref: format!("copy:base:action:{}:{step_token}", kind.as_str()),
        keyboard_shortcut_ref: Some(format!("kbd:learning.exercise.{}", kind.as_str())),
        command_id_ref: Some(command.to_string()),
        inspectable: true,
        restart_safe: true,
        mutates_workspace: false,
        narrowing_reason: None,
    })
    .collect()
}

/// Compact description of one seeded step.
struct StepSpec<'a> {
    token: &'a str,
    kind: ExerciseStepKind,
    targets: &'a [(TargetKind, &'a str)],
    commands: &'a [&'a str],
    anchors: &'a [&'a str],
    explain_apply: ExplainApplyClass,
    mutation: MutationTarget,
    command_backing: CommandBacking,
    criteria: &'a [(SuccessCriterionKind, &'a str)],
}

/// Apply-step command backing that reuses the standard command/preview/approval
/// model with a trust/policy check.
fn apply_backing(family: &str, command: &str) -> CommandBacking {
    CommandBacking {
        command_id_ref: Some(command.to_string()),
        preview_sheet_ref: Some(format!("preview_sheet:{family}")),
        approval_path_ref: Some(format!("approval_path:{family}")),
        trust_policy_check_ref: Some(format!("trust_policy_check:{family}")),
        uses_standard_command_model: true,
        narrowing_reason: None,
    }
}

fn build_rail(
    family: M5LearningSurfaceFamily,
    source_class: SourceClass,
    freshness: FreshnessState,
    parity: MirrorParityPosture,
    sandbox: SandboxPreference,
    explain_apply: ExplainApplyClass,
    steps: &[StepSpec<'_>],
    live: bool,
) -> GuidedExerciseRail {
    let rail_id = rail_id(family);
    let fam = family.as_str();

    let step_records: Vec<ExerciseStepRecord> = steps
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let step_id = format!("{rail_id}:step:{}", s.token);
            let citation = if live {
                live_citation(s.commands, s.anchors)
            } else {
                cached_citation(s.commands, s.anchors)
            };
            let success_criteria = s
                .criteria
                .iter()
                .enumerate()
                .map(|(cidx, (kind, check))| SuccessCriterion {
                    criterion_id: format!("{step_id}:criterion:{cidx}"),
                    criterion_kind: *kind,
                    check_ref: check.to_string(),
                    target: s.targets.first().map(|(k, id)| StableTargetRef {
                        target_kind: *k,
                        target_id: id.to_string(),
                    }),
                    description_ref: format!("copy:base:criterion:{}:{}", s.token, cidx),
                })
                .collect();
            let reveal_ref = if matches!(s.kind, ExerciseStepKind::Explain) {
                None
            } else {
                Some(format!("copy:base:reveal:{}", s.token))
            };
            ExerciseStepRecord {
                step_id: step_id.clone(),
                position_index: idx as u32,
                title_ref: format!("copy:base:{}:title", s.token),
                step_kind: s.kind,
                stable_targets: s
                    .targets
                    .iter()
                    .map(|(kind, id)| StableTargetRef {
                        target_kind: *kind,
                        target_id: id.to_string(),
                    })
                    .collect(),
                success_criteria,
                citation,
                explain_apply_class: s.explain_apply,
                mutation_target: s.mutation,
                command_backing: s.command_backing.clone(),
                hint_refs: vec![
                    format!("copy:base:hint:{}:1", s.token),
                    format!("copy:base:hint:{}:2", s.token),
                ],
                reveal_ref,
                actions: standard_actions(s.token),
            }
        })
        .collect();

    let step_ids: Vec<&str> = step_records.iter().map(|s| s.step_id.as_str()).collect();

    let mut rail = GuidedExerciseRail {
        record_kind: GUIDED_EXERCISE_RAIL_RECORD_KIND.to_string(),
        schema_version: M5_GUIDED_EXERCISE_RAILS_SCHEMA_VERSION,
        rail_id: rail_id.clone(),
        version: version(family),
        family,
        generated_at: GENERATED_AT.to_string(),
        title_ref: format!("copy:base:rail:{fam}:title"),
        source_class,
        source_ref: format!("source:m5:exercise_rail:{fam}:v1"),
        freshness_state: freshness,
        mirror_parity: parity,
        base_locale: "en-US".to_string(),
        locale_overlays: locale_overlays(&step_ids),
        explain_apply_class: explain_apply,
        sandbox_preference: sandbox,
        privacy: local_privacy(),
        accessibility: full_accessibility(),
        progress: ExerciseProgress::fresh(format!("learning:progress:exercise_rail:{fam}:v1")),
        prerequisite_rail_refs: vec![],
        steps: step_records,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: vec![],
    };
    rail.sync_verdict();
    rail
}

/// Builds a two-locale overlay set for the given step ids.
fn locale_overlays(ids: &[&str]) -> Vec<LocaleOverlay> {
    ["fr-FR", "ja-JP"]
        .into_iter()
        .map(|tag| {
            let localized_label_refs = ids
                .iter()
                .map(|id| (id.to_string(), format!("copy:{tag}:{id}")))
                .collect();
            LocaleOverlay {
                locale_tag: tag.to_string(),
                localized_label_refs,
                preserves_target_identity: true,
                preserves_citations: true,
                narrowing_reason: None,
            }
        })
        .collect()
}

/// Returns the seeded guided-exercise-rail manifest covering every M5 feature
/// family.
///
/// Most families ship Stable, live-authoritative rails. Each Stable rail keeps an
/// explain step (read-only), an optional prepare-practice step (sandboxed,
/// reversible), and a command-backed apply step (real workspace, reversible,
/// through preview/approval). Two families demonstrate the narrowing invariant
/// honestly:
///
/// - `preview` is not yet mirror-synced (`local_only_disclosed`), so its rail
///   narrows to Beta.
/// - `companion` ships from a cached revision (`cached_disclosed`), so its rail
///   narrows to Beta rather than masquerading as live.
///
/// The `docs_browser` rail is read-only end to end (explain steps only), proving
/// a rail need not teach an apply flow to qualify.
pub fn seeded_m5_guided_exercise_rails() -> M5GuidedExerciseRailManifest {
    use ExerciseStepKind::*;
    use ExplainApplyClass::*;
    use M5LearningSurfaceFamily::*;
    use MutationTarget::*;
    use SuccessCriterionKind::*;
    use TargetKind::*;

    let mut rails = Vec::new();

    // ── notebook: explain → prepare(sandbox) → apply ──
    rails.push(build_rail(
        Notebook,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        sandbox_preferred(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_kernel_trust",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:notebook:kernel_trust")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:notebook:kernel_trust"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:notebook:kernel_trust_opened")],
            },
            StepSpec {
                token: "scratch_cell",
                kind: PreparePractice,
                targets: &[(SurfaceObjectId, "surface:notebook:scratch_cell")],
                commands: &["cmd:notebook.run_cell"],
                anchors: &["docs:anchor:notebook:execution_model"],
                explain_apply: ApplyRequiresApproval,
                mutation: SandboxedLocalReversible,
                command_backing: apply_backing("notebook", "cmd:notebook.run_cell"),
                criteria: &[(OutputMatches, "check:notebook:scratch_output")],
            },
            StepSpec {
                token: "run_real_cell",
                kind: ApplyWithApproval,
                targets: &[
                    (CommandId, "cmd:notebook.run_cell"),
                    (SurfaceObjectId, "surface:notebook:output_region"),
                ],
                commands: &["cmd:notebook.run_cell"],
                anchors: &["docs:anchor:notebook:kernel_trust"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("notebook", "cmd:notebook.run_cell"),
                criteria: &[(CommandRan, "check:notebook:cell_ran")],
            },
        ],
        true,
    ));

    // ── request_workspace: explain → apply ──
    rails.push(build_rail(
        RequestWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        no_sandbox_but_reversible(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_auth_profiles",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:request:auth_profiles")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:request:auth_profiles"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:request:auth_profiles_read")],
            },
            StepSpec {
                token: "compose_and_send",
                kind: ApplyWithApproval,
                targets: &[(CommandId, "cmd:request.send")],
                commands: &["cmd:request.send"],
                anchors: &["docs:anchor:request:auth_profiles"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("request_workspace", "cmd:request.send"),
                criteria: &[(CommandRan, "check:request:sent")],
            },
        ],
        true,
    ));

    // ── database_workspace: explain → prepare(sandbox txn) → apply ──
    rails.push(build_rail(
        DatabaseWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        sandbox_preferred(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_statement_safety",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:database:statement_safety")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:database:statement_safety"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:database:safety_read")],
            },
            StepSpec {
                token: "sandbox_transaction",
                kind: PreparePractice,
                targets: &[(SurfaceObjectId, "surface:database:sandbox_transaction")],
                commands: &["cmd:database.run_statement"],
                anchors: &["docs:anchor:database:statement_safety"],
                explain_apply: ApplyRequiresApproval,
                mutation: SandboxedLocalReversible,
                command_backing: apply_backing("database_workspace", "cmd:database.run_statement"),
                criteria: &[(OutputMatches, "check:database:sandbox_result")],
            },
            StepSpec {
                token: "run_statement",
                kind: ApplyWithApproval,
                targets: &[(CommandId, "cmd:database.run_statement")],
                commands: &["cmd:database.run_statement"],
                anchors: &["docs:anchor:database:statement_safety"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("database_workspace", "cmd:database.run_statement"),
                criteria: &[(CommandRan, "check:database:statement_ran")],
            },
        ],
        true,
    ));

    // ── profiler_trace: explain → apply(start capture) ──
    rails.push(build_rail(
        ProfilerTrace,
        SourceClass::SemanticGraph,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        sandbox_preferred(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_capture_model",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:profiler:capture_model")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:profiler:capture_model"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:profiler:capture_model_read")],
            },
            StepSpec {
                token: "capture_and_interpret",
                kind: ApplyWithApproval,
                targets: &[
                    (CommandId, "cmd:profiler.start_capture"),
                    (GraphNodeId, "graph:node:trace:flame_graph"),
                ],
                commands: &["cmd:profiler.start_capture"],
                anchors: &["docs:anchor:trace:flame_graph"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("profiler_trace", "cmd:profiler.start_capture"),
                criteria: &[(CommandRan, "check:profiler:capture_started")],
            },
        ],
        true,
    ));

    // ── docs_browser: read-only end to end (mirror-synced, still Stable) ──
    rails.push(build_rail(
        DocsBrowser,
        SourceClass::MirroredOfficialDocs,
        FreshnessState::MirrorSyncedDisclosed,
        stable_parity(FreshnessState::MirrorSyncedDisclosed),
        SandboxPreference {
            prefers_sandbox: false,
            sandbox_available: false,
            reversible_by_default: true,
            real_workspace_mutation_requires_explicit_optin: true,
            narrowing_reason: None,
        },
        ReadOnly,
        &[
            StepSpec {
                token: "open_offline_pack",
                kind: Explain,
                targets: &[(CommandId, "cmd:docs.open_in_browser")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:docs_browser:offline_packs"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:docs_browser:offline_pack_opened")],
            },
            StepSpec {
                token: "cite_a_passage",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:docs_browser:contract")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:docs_browser:contract"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(SurfaceStateReached, "check:docs_browser:passage_cited")],
            },
        ],
        true,
    ));

    // ── preview: not yet mirror-synced → Beta ──
    rails.push(build_rail(
        Preview,
        SourceClass::ProjectDocs,
        FreshnessState::LocalOnlyDisclosed,
        local_only_parity(),
        sandbox_preferred(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_lineage_model",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:preview:lineage")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:preview:lineage"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:preview:lineage_read")],
            },
            StepSpec {
                token: "open_and_trace",
                kind: ApplyWithApproval,
                targets: &[(CommandId, "cmd:preview.open")],
                commands: &["cmd:preview.open"],
                anchors: &["docs:anchor:preview:origin_model"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("preview", "cmd:preview.open"),
                criteria: &[(SurfaceStateReached, "check:preview:traced")],
            },
        ],
        true,
    ));

    // ── template_scaffold: explain → prepare(sandbox) → apply(reversible) ──
    rails.push(build_rail(
        TemplateScaffold,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        sandbox_preferred(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_planner_model",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:scaffold:planner_model")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:scaffold:planner_model"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:scaffold:planner_model_read")],
            },
            StepSpec {
                token: "preview_plan",
                kind: PreparePractice,
                targets: &[(CommandId, "cmd:scaffold.plan")],
                commands: &["cmd:scaffold.plan"],
                anchors: &["docs:anchor:scaffold:planner_model"],
                explain_apply: ApplyRequiresApproval,
                mutation: SandboxedLocalReversible,
                command_backing: apply_backing("template_scaffold", "cmd:scaffold.plan"),
                criteria: &[(DiffPreviewed, "check:scaffold:plan_previewed")],
            },
            StepSpec {
                token: "apply_scaffold",
                kind: ApplyWithApproval,
                targets: &[
                    (CommandId, "cmd:scaffold.apply"),
                    (FileObjectId, "file:scaffold:target_folder"),
                ],
                commands: &["cmd:scaffold.apply"],
                anchors: &["docs:anchor:scaffold:lineage"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("template_scaffold", "cmd:scaffold.apply"),
                criteria: &[(FileExists, "check:scaffold:files_written")],
            },
        ],
        true,
    ));

    // ── companion: cached revision → Beta ──
    rails.push(build_rail(
        Companion,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::CachedDisclosed,
        stable_parity(FreshnessState::CachedDisclosed),
        no_sandbox_but_reversible(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_response_model",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:incident:response_model")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:incident:response_model"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:companion:response_model_read")],
            },
            StepSpec {
                token: "acknowledge_incident",
                kind: ApplyWithApproval,
                targets: &[(CommandId, "cmd:incident.acknowledge")],
                commands: &["cmd:incident.acknowledge"],
                anchors: &["docs:anchor:companion:surface_contract"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("companion", "cmd:incident.acknowledge"),
                criteria: &[(CommandRan, "check:companion:acknowledged")],
            },
        ],
        false,
    ));

    // ── sync_offboarding: explain → apply(export bundle) ──
    rails.push(build_rail(
        SyncOffboarding,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        no_sandbox_but_reversible(),
        ApplyRequiresApproval,
        &[
            StepSpec {
                token: "read_retention_model",
                kind: Explain,
                targets: &[(DocsNodeId, "docs:node:sync:retention_model")],
                commands: &["cmd:docs.open_in_browser"],
                anchors: &["docs:anchor:sync:retention_model"],
                explain_apply: ReadOnly,
                mutation: NoMutation,
                command_backing: CommandBacking::none(),
                criteria: &[(DocsOpened, "check:sync:retention_read")],
            },
            StepSpec {
                token: "export_bundle",
                kind: ApplyWithApproval,
                targets: &[(CommandId, "cmd:offboarding.export_bundle")],
                commands: &["cmd:offboarding.export_bundle"],
                anchors: &["docs:anchor:offboarding:export_and_destroy"],
                explain_apply: ApplyRequiresApproval,
                mutation: WorkspaceReversibleApproved,
                command_backing: apply_backing("sync_offboarding", "cmd:offboarding.export_bundle"),
                criteria: &[(FileExists, "check:sync:bundle_exported")],
            },
        ],
        true,
    ));

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "guided_exercise_rails_schema".to_string(),
        M5_GUIDED_EXERCISE_RAILS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "tour_and_glossary_schema".to_string(),
        M5_TOUR_AND_GLOSSARY_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "artifact_doc".to_string(),
        M5_GUIDED_EXERCISE_RAILS_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "public_doc".to_string(),
        M5_GUIDED_EXERCISE_RAILS_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "canonical_fixture".to_string(),
        M5_GUIDED_EXERCISE_RAILS_FIXTURE_REF.to_string(),
    );

    let mut manifest = M5GuidedExerciseRailManifest {
        record_kind: M5_GUIDED_EXERCISE_RAIL_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_GUIDED_EXERCISE_RAILS_SCHEMA_VERSION,
        manifest_id: "m5-guided-exercise-rails:manifest:2026.06.19-01".to_string(),
        generated_at: GENERATED_AT.to_string(),
        contract_refs,
        rails,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: vec![],
    };
    manifest.sync_verdicts();
    manifest
}

#[cfg(test)]
mod tests;
