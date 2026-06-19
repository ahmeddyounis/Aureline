//! Learning-mode profiles: opt-in, user-owned presets that tune how much
//! guidance a person sees without ever changing authority, trust, ownership, or
//! the command graph.
//!
//! Where [`crate::guided_exercise_rails`] models hands-on practice and
//! [`crate::tour_and_glossary_packages`] models the reference packs, this module
//! owns the **dial**: a [`LearningModeProfile`] declares a person's tip
//! intensity, jargon level, educational-AI explanation posture, mutation
//! guardrail, dismissals, bookmarks, and per-user/per-workspace scope, plus the
//! controls and change history that make turning learning mode on, off, paused,
//! snoozed, reset, or narrowed an inspectable, reversible, command-backed
//! action.
//!
//! ## What a profile freezes
//!
//! - **Tunable comfort axes.** [`TipIntensity`], [`JargonLevel`],
//!   [`AiExplanationPosture`], and [`MutationGuardrail`] are the only knobs a
//!   profile turns. None of them changes who owns data, what a command can do,
//!   or which trust boundary applies.
//! - **Explicit scope.** [`ScopeBinding`] records whether a profile is
//!   per-user-local or per-workspace-opt-in. A workspace profile must be
//!   explicitly opted into, is never committed to the repo, and is never shared
//!   with collaborators — onboarding preferences cannot leak into repo state.
//! - **Inspectable lifecycle.** Every profile carries the full set of
//!   [`ProfileControl`]s (enable, disable, pause, snooze, resume, reset, narrow)
//!   and a [`ProfileChangeEvent`] history. Each control is command-backed,
//!   keyboard-reachable, reversible, and never silently writes or mutates the
//!   workspace.
//!
//! ## Invariants enforced
//!
//! - **Authority and ownership never move.** A profile that sets
//!   `authority_boundary_change_allowed`, changes the command graph, or stores
//!   state anywhere but user-owned local-first storage narrows below Stable and
//!   fails validation.
//! - **Experts are never trapped.** No profile may force blocking onboarding;
//!   every state is reversible from an inspectable control.
//! - **Educational AI keeps "do" behind the fence.** A profile whose
//!   explanation posture can prepare a "do" must route it through the same
//!   preview/approval model as ordinary work; an unfenced posture fails
//!   validation.
//! - **Progress stays private.** Dismissals, bookmarks, and change history are
//!   user-owned and local-first; repo-visible or collaborator-shared state
//!   fails validation. Optional portable-profile sync is allowed only when
//!   explicitly disclosed, and a synced-but-undisclosed profile is a masquerade
//!   that fails validation.
//! - **State is inspectable, not hidden in overlays.** Every profile exposes its
//!   state, change history, and reset path through settings, Help/About,
//!   diagnostics, and support export.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_learning_mode_profiles`] produces the canonical manifest.
//! Settings, Help/About, diagnostics, and support-export surfaces ingest it
//! rather than rephrasing learning-mode state by hand.
//!
//! - Schema: [`M5_LEARNING_MODE_PROFILES_SCHEMA_REF`]
//! - Fixture: [`M5_LEARNING_MODE_PROFILES_FIXTURE_REF`]
//! - Artifact: [`M5_LEARNING_MODE_PROFILES_ARTIFACT_REF`]
//! - Doc: [`M5_LEARNING_MODE_PROFILES_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::freeze_m5_learnability_lane::DataOwnershipClass;
use crate::m5_feature_family_learning_rails::{
    M5LearningSurfaceFamily, M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF,
};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    QualificationVerdict, GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};

#[cfg(test)]
mod tests;

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the learning-mode-profile records. Bumped only on
/// breaking payload changes; additive-optional fields do not bump it.
pub const M5_LEARNING_MODE_PROFILES_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LearningModeProfile`].
pub const LEARNING_MODE_PROFILE_RECORD_KIND: &str = "learning_mode_profile";

/// Record kind for [`M5LearningModeProfileManifest`].
pub const M5_LEARNING_MODE_PROFILE_MANIFEST_RECORD_KIND: &str = "m5_learning_mode_profile_manifest";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the learning-mode-profile schema.
pub const M5_LEARNING_MODE_PROFILES_SCHEMA_REF: &str =
    "schemas/help/m5-learning-mode-profiles.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_LEARNING_MODE_PROFILES_FIXTURE_REF: &str =
    "fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json";

/// Repository-relative path to the proof artifact.
pub const M5_LEARNING_MODE_PROFILES_ARTIFACT_REF: &str =
    "artifacts/ux/m5/learning-mode-profile-proof/implement-learning-mode-profiles.md";

/// Repository-relative path to the public doc.
pub const M5_LEARNING_MODE_PROFILES_DOC_REF: &str = "docs/m5/learning-mode-profiles.md";

// ── Tunable comfort axes ──────────────────────────────────────────────────────

/// How insistently a profile surfaces tips and coachmarks.
///
/// Every level is dismissable and inline-friendly; no level is allowed to block
/// first useful work, so even the most insistent profile cannot trap an expert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipIntensity {
    /// Tips appear inline only and never interrupt; the expert default.
    SilentInlineOnly,
    /// Gentle, dismissable hints surface near the relevant surface.
    GentleHint,
    /// Hints prompt for an explicit acknowledgement before fading.
    PromptedAcknowledge,
}

impl TipIntensity {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentInlineOnly => "silent_inline_only",
            Self::GentleHint => "gentle_hint",
            Self::PromptedAcknowledge => "prompted_acknowledge",
        }
    }
}

/// How much specialist vocabulary a profile leaves unexplained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JargonLevel {
    /// Terms are defined inline on first use.
    Beginner,
    /// Common terms are assumed; specialist terms are defined.
    Intermediate,
    /// Most terms are assumed; only rare terms are defined.
    Advanced,
    /// No inline term definitions; the expert default.
    Expert,
}

impl JargonLevel {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beginner => "beginner",
            Self::Intermediate => "intermediate",
            Self::Advanced => "advanced",
            Self::Expert => "expert",
        }
    }
}

/// How the educational AI behaves when it could prepare an action.
///
/// Explain and do stay separate at every posture: the AI explains freely, but
/// any "do" it prepares is a preview that still rides the standard
/// preview/approval/rollback fence. No posture lets the AI mutate live state
/// directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiExplanationPosture {
    /// The AI explains only; it never prepares a mutation preview.
    ExplainOnly,
    /// The AI explains and may proactively prepare a preview that still needs
    /// approval before anything mutates.
    ExplainThenPreparePreview,
    /// The AI prepares a preview only after an explicit user "do"; the
    /// learner-safe posture.
    PreviewOnlyAfterExplicitDo,
}

impl AiExplanationPosture {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::ExplainThenPreparePreview => "explain_then_prepare_preview",
            Self::PreviewOnlyAfterExplicitDo => "preview_only_after_explicit_do",
        }
    }

    /// Returns true when this posture can prepare a "do" (a mutation preview)
    /// and therefore must be fenced by the standard preview/approval model.
    pub const fn permits_do(self) -> bool {
        matches!(
            self,
            Self::ExplainThenPreparePreview | Self::PreviewOnlyAfterExplicitDo
        )
    }
}

/// The guardrail a profile places in front of any mutation the educational AI
/// or a guided surface would prepare.
///
/// Every variant fences mutation; there is deliberately no "unfenced" value, so
/// a profile can never structurally permit a direct live-state write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationGuardrail {
    /// No mutation may be prepared at all; explanation only.
    ExplainOnlyNoMutation,
    /// Any prepared mutation must show a preview first.
    PreviewRequired,
    /// Any prepared mutation needs an explicit approval (preview + approve).
    ApprovalRequired,
    /// Mutation is blocked until the workspace trust grant is in place.
    BlockedUntilTrust,
}

impl MutationGuardrail {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnlyNoMutation => "explain_only_no_mutation",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::BlockedUntilTrust => "blocked_until_trust",
        }
    }

    /// Returns true when this guardrail still allows a preview-fenced "do".
    ///
    /// [`Self::ExplainOnlyNoMutation`] forbids any prepared mutation, so a
    /// profile pairing it with a do-capable posture is contradictory.
    pub const fn permits_fenced_do(self) -> bool {
        !matches!(self, Self::ExplainOnlyNoMutation)
    }
}

// ── Preset ────────────────────────────────────────────────────────────────────

/// The named starting point a profile is derived from.
///
/// A preset only sets defaults; every axis remains independently tunable. The
/// preset matters to validation in one way: learner-facing presets keep
/// explain-before-act on, while [`Self::ExpertMinimal`] is allowed to opt out of
/// forced pre-explanation (it never opts out of the mutation fence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningModePreset {
    /// Quietest profile: inline-only tips, expert jargon, no forced
    /// pre-explanation. Never traps the expert; still fences every mutation.
    ExpertMinimal,
    /// The balanced default: gentle hints, intermediate jargon, explain before
    /// act.
    BalancedDefault,
    /// Most guidance: prompted hints, beginner jargon, explain before act, and
    /// the AI prepares a do only after an explicit request.
    GuidedLearner,
}

impl LearningModePreset {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpertMinimal => "expert_minimal",
            Self::BalancedDefault => "balanced_default",
            Self::GuidedLearner => "guided_learner",
        }
    }

    /// Whether this preset must keep `explain_before_act_default` on.
    ///
    /// Only [`Self::ExpertMinimal`] may turn it off.
    pub const fn requires_explain_before_act(self) -> bool {
        !matches!(self, Self::ExpertMinimal)
    }
}

// ── Profile lifecycle state ─────────────────────────────────────────────────

/// The current lifecycle state of a learning-mode profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileState {
    /// Learning mode is off for this scope.
    Disabled,
    /// Learning mode is on.
    Enabled,
    /// Temporarily paused; resumable without losing progress.
    Paused,
    /// Snoozed for a bounded period; resumable without losing progress.
    Snoozed,
    /// A reset to defaults has been requested.
    ResetRequested,
}

impl ProfileState {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::Paused => "paused",
            Self::Snoozed => "snoozed",
            Self::ResetRequested => "reset_requested",
        }
    }
}

// ── Scope binding ──────────────────────────────────────────────────────────

/// Whether a profile is owned by the user or attached to a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileScope {
    /// Stored in the user's portable profile; the same across every workspace.
    UserLocal,
    /// Opt-in per workspace, layered over the user profile and stored in
    /// workspace-local (never repo-committed) settings.
    WorkspaceOptIn,
}

impl ProfileScope {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserLocal => "user_local",
            Self::WorkspaceOptIn => "workspace_opt_in",
        }
    }
}

/// How and where a profile is scoped and stored.
///
/// A workspace profile must be explicitly opted into. No profile is ever
/// committed to the repository or shared with collaborators — that would leak
/// onboarding preferences into shared state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeBinding {
    /// Named scope.
    pub scope: ProfileScope,
    /// Whether the user explicitly opted this scope in. A
    /// [`ProfileScope::WorkspaceOptIn`] profile requires this to be true.
    pub opt_in_explicit: bool,
    /// Whether the profile is committed to the repository. MUST be false.
    pub repo_committed: bool,
    /// Whether the profile is shared with collaborators. MUST be false.
    pub shared_with_collaborators: bool,
    /// Opaque ref to the storage location backing this scope.
    pub storage_location_ref: String,
}

impl ScopeBinding {
    /// Returns true when the binding satisfies Stable scope requirements.
    pub fn qualifies_stable(&self) -> bool {
        !self.repo_committed
            && !self.shared_with_collaborators
            && (self.scope == ProfileScope::UserLocal || self.opt_in_explicit)
    }
}

// ── Sync posture ───────────────────────────────────────────────────────────

/// How a profile's state moves (or does not move) across machines.
///
/// Local-only is the live-authoritative default. Portable-profile sync is an
/// allowed user choice but it must be disclosed and it narrows the profile below
/// Stable, because synced state can lag behind another machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncPosture {
    /// State never leaves the machine; live-authoritative.
    LocalOnly,
    /// State syncs through the user's portable profile; may lag across machines.
    PortableProfileSynced,
    /// Policy disables sync; the profile is pinned local-only.
    SyncBlockedByPolicy,
}

impl SyncPosture {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::PortableProfileSynced => "portable_profile_synced",
            Self::SyncBlockedByPolicy => "sync_blocked_by_policy",
        }
    }

    /// Returns true when state leaves the machine via portable-profile sync.
    pub const fn is_synced(self) -> bool {
        matches!(self, Self::PortableProfileSynced)
    }
}

// ── Dismissals and bookmarks ───────────────────────────────────────────────

/// User-owned record of which learning surfaces have been dismissed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DismissalState {
    /// Opaque refs to surfaces (tips, cards, tours) the user dismissed.
    #[serde(default)]
    pub dismissed_surface_refs: Vec<String>,
    /// Whether a dismissal can be undone. MUST be true.
    pub reversible: bool,
    /// Whether the dismissal state is user-owned and local-first. MUST be true.
    pub user_owned_local: bool,
    /// Whether dismissals follow the owning profile's scope rather than leaking
    /// to a wider scope. MUST be true.
    pub follows_profile_scope: bool,
}

impl DismissalState {
    /// Returns true when the dismissal state satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.reversible && self.user_owned_local && self.follows_profile_scope
    }
}

/// User-owned record of which learning surfaces have been bookmarked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookmarkState {
    /// Opaque refs to surfaces the user bookmarked to revisit.
    #[serde(default)]
    pub bookmarked_surface_refs: Vec<String>,
    /// Whether the bookmark state is user-owned and local-first. MUST be true.
    pub user_owned_local: bool,
    /// Whether bookmarks travel in the user's portable-profile export.
    pub exportable_in_portable_profile: bool,
}

impl BookmarkState {
    /// Returns true when the bookmark state satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.user_owned_local
    }
}

// ── Controls ───────────────────────────────────────────────────────────────

/// The kind of lifecycle transition a [`ProfileControl`] performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileControlKind {
    /// Turn learning mode on.
    Enable,
    /// Turn learning mode off.
    Disable,
    /// Pause learning mode without losing progress.
    Pause,
    /// Snooze learning mode for a bounded period.
    Snooze,
    /// Resume from paused/snoozed.
    Resume,
    /// Reset the profile to its preset defaults.
    Reset,
    /// Narrow the profile (e.g. quieter tips) without disabling it.
    Narrow,
}

impl ProfileControlKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::Pause => "pause",
            Self::Snooze => "snooze",
            Self::Resume => "resume",
            Self::Reset => "reset",
            Self::Narrow => "narrow",
        }
    }
}

/// The control kinds every profile must expose so learning mode can always be
/// turned on, off, reset, or narrowed.
pub const REQUIRED_CONTROL_KINDS: [ProfileControlKind; 4] = [
    ProfileControlKind::Enable,
    ProfileControlKind::Disable,
    ProfileControlKind::Reset,
    ProfileControlKind::Narrow,
];

/// One command-backed control that drives a profile lifecycle transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileControl {
    /// Opaque stable id for the control.
    pub control_id: String,
    /// What the control does.
    pub control_kind: ProfileControlKind,
    /// Opaque ref to the command that backs this control.
    pub command_id_ref: String,
    /// Human-readable description of the state transition.
    pub state_transition: String,
    /// Opaque ref to the keyboard shortcut; MUST be present (keyboard
    /// reachable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut_ref: Option<String>,
    /// Whether the transition is reversible. MUST be true.
    pub reversible: bool,
    /// Whether the control surfaces in the action log / inspector. MUST be true.
    pub inspectable: bool,
    /// Whether the control may write profile state silently. MUST be false.
    pub silent_write_allowed: bool,
    /// Whether the control mutates workspace state. MUST be false — controls
    /// only touch local onboarding state.
    pub mutates_workspace: bool,
}

impl ProfileControl {
    /// Returns true when the control satisfies every Stable requirement.
    pub fn qualifies_stable(&self) -> bool {
        self.keyboard_shortcut_ref.is_some()
            && self.reversible
            && self.inspectable
            && !self.silent_write_allowed
            && !self.mutates_workspace
    }
}

// ── Change history ─────────────────────────────────────────────────────────

/// The kind of change captured by a [`ProfileChangeEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileChangeKind {
    /// The profile was enabled.
    Enabled,
    /// The profile was disabled.
    Disabled,
    /// The profile was paused.
    Paused,
    /// The profile was snoozed.
    Snoozed,
    /// The profile was resumed.
    Resumed,
    /// The profile was reset to defaults.
    Reset,
    /// The profile was narrowed.
    Narrowed,
    /// The tip intensity was changed.
    TipIntensityChanged,
    /// The jargon level was changed.
    JargonLevelChanged,
    /// The scope was changed.
    ScopeChanged,
    /// The educational-AI explanation posture was changed.
    AiPostureChanged,
    /// The mutation guardrail was changed.
    MutationGuardrailChanged,
}

impl ProfileChangeKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Paused => "paused",
            Self::Snoozed => "snoozed",
            Self::Resumed => "resumed",
            Self::Reset => "reset",
            Self::Narrowed => "narrowed",
            Self::TipIntensityChanged => "tip_intensity_changed",
            Self::JargonLevelChanged => "jargon_level_changed",
            Self::ScopeChanged => "scope_changed",
            Self::AiPostureChanged => "ai_posture_changed",
            Self::MutationGuardrailChanged => "mutation_guardrail_changed",
        }
    }
}

/// One user-owned, inspectable entry in a profile's change history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileChangeEvent {
    /// Opaque stable id for the event.
    pub event_id: String,
    /// Deterministic timestamp for the change.
    pub at: String,
    /// What changed.
    pub change_kind: ProfileChangeKind,
    /// Prior value/state as a stable token.
    pub from_state: String,
    /// New value/state as a stable token.
    pub to_state: String,
    /// Whether the change was user-initiated. MUST be true — no silent changes.
    pub user_initiated: bool,
    /// Whether the event is visible in support export. MUST be true.
    pub inspectable_in_support_export: bool,
}

impl ProfileChangeEvent {
    /// Returns true when the event satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.user_initiated && self.inspectable_in_support_export
    }
}

// ── Surface exposure ───────────────────────────────────────────────────────

/// Where a profile's state, change history, and reset path are visible.
///
/// Learning-mode state must be inspectable wherever a user or a support flow
/// would look for it — never hidden inside a transient overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceExposure {
    /// Visible in settings. MUST be true.
    pub in_settings: bool,
    /// Visible in Help/About. MUST be true.
    pub in_help_about: bool,
    /// Visible in diagnostics. MUST be true.
    pub in_diagnostics: bool,
    /// Visible in support export. MUST be true.
    pub in_support_export: bool,
    /// Whether the state is reachable only through a transient overlay. MUST be
    /// false.
    pub hidden_in_transient_overlay_only: bool,
}

impl SurfaceExposure {
    /// Returns true when exposure satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.in_settings
            && self.in_help_about
            && self.in_diagnostics
            && self.in_support_export
            && !self.hidden_in_transient_overlay_only
    }
}

// ── Profile ────────────────────────────────────────────────────────────────

/// One learning-mode profile: a user-owned, opt-in dial over Aureline's
/// learnability surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProfile {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this profile.
    pub profile_id: String,
    /// Human-readable label shown in settings and support export.
    pub display_label: String,
    /// The preset this profile is derived from.
    pub preset: LearningModePreset,
    /// Current lifecycle state.
    pub profile_state: ProfileState,
    /// Scope and storage binding.
    pub scope_binding: ScopeBinding,
    /// Tip intensity.
    pub tip_intensity: TipIntensity,
    /// Jargon level.
    pub jargon_level: JargonLevel,
    /// Educational-AI explanation posture.
    pub ai_explanation_posture: AiExplanationPosture,
    /// Mutation guardrail.
    pub mutation_guardrail: MutationGuardrail,
    /// Whether the educational AI routes any prepared "do" through the standard
    /// preview/approval model. MUST be true.
    pub educational_ai_uses_standard_preview_approval: bool,
    /// Whether explain-before-act is on by default for this profile.
    pub explain_before_act_default: bool,
    /// Whether the profile is allowed to change an authority boundary. MUST be
    /// false.
    pub authority_boundary_change_allowed: bool,
    /// Whether the command graph stays unchanged under this profile. MUST be
    /// true.
    pub command_graph_unchanged: bool,
    /// Data-ownership class. MUST be user-owned local-first.
    pub data_ownership: DataOwnershipClass,
    /// Whether the profile may force blocking onboarding. MUST be false.
    pub blocking_onboarding_allowed: bool,
    /// Sync posture.
    pub sync_posture: SyncPosture,
    /// Whether portable-profile sync is disclosed to the user. A synced profile
    /// MUST set this true.
    pub sync_disclosed: bool,
    /// Dismissal state.
    pub dismissals: DismissalState,
    /// Bookmark state.
    pub bookmarks: BookmarkState,
    /// Lifecycle controls.
    pub controls: Vec<ProfileControl>,
    /// User-owned change history.
    #[serde(default)]
    pub change_history: Vec<ProfileChangeEvent>,
    /// Where the profile state is exposed.
    pub exposure: SurfaceExposure,
    /// The M5 surface families this profile applies to.
    pub applies_to_families: Vec<M5LearningSurfaceFamily>,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearningModeProfile {
    /// The set of control kinds this profile exposes.
    pub fn control_kinds(&self) -> BTreeSet<ProfileControlKind> {
        self.controls.iter().map(|c| c.control_kind).collect()
    }

    /// Recomputes this profile's verdict and narrowing reasons from its
    /// evidence, writing them back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_learning_mode_profile_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Verdict derivation ───────────────────────────────────────────────────────

/// Derives a profile's verdict and narrowing reasons from its evidence.
///
/// Hard safety violations (authority change, non-user ownership, command-graph
/// drift, blocking onboarding, an unfenced educational-AI "do", repo/collaborator
/// leakage, an undisclosed sync masquerade, a non-inspectable control, or hidden
/// state) narrow to [`QualificationVerdict::NarrowedPreview`]. A disclosed
/// portable-profile sync is an honest, user-chosen deviation and narrows to
/// [`QualificationVerdict::NarrowedBeta`]. With no findings the profile is
/// [`QualificationVerdict::QualifiedStable`].
pub fn derive_learning_mode_profile_verdict(
    profile: &LearningModeProfile,
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();
    let hard = |reasons: &mut Vec<String>, reason: &str| {
        reasons.push(reason.to_string());
    };

    // ── Hard safety violations ──
    if profile.authority_boundary_change_allowed {
        hard(&mut reasons, "authority_boundary_change_allowed");
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.command_graph_unchanged {
        hard(&mut reasons, "command_graph_changed");
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.data_ownership.qualifies_stable() {
        hard(&mut reasons, "data_ownership_not_user_owned_local_first");
        verdict = verdict.meet(NarrowedPreview);
    }
    if profile.blocking_onboarding_allowed {
        hard(&mut reasons, "blocking_onboarding_allowed_traps_experts");
        verdict = verdict.meet(NarrowedPreview);
    }
    if profile.preset.requires_explain_before_act() && !profile.explain_before_act_default {
        hard(&mut reasons, "learner_profile_dropped_explain_before_act");
        verdict = verdict.meet(NarrowedPreview);
    }
    if profile.ai_explanation_posture.permits_do()
        && !profile.educational_ai_uses_standard_preview_approval
    {
        hard(
            &mut reasons,
            "educational_ai_do_outside_standard_preview_approval",
        );
        verdict = verdict.meet(NarrowedPreview);
    }
    if profile.ai_explanation_posture.permits_do()
        && !profile.mutation_guardrail.permits_fenced_do()
    {
        hard(&mut reasons, "ai_posture_prepares_do_the_guardrail_forbids");
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.scope_binding.qualifies_stable() {
        if profile.scope_binding.repo_committed {
            hard(&mut reasons, "profile_committed_into_repo_state");
        }
        if profile.scope_binding.shared_with_collaborators {
            hard(&mut reasons, "profile_shared_with_collaborators");
        }
        if profile.scope_binding.scope == ProfileScope::WorkspaceOptIn
            && !profile.scope_binding.opt_in_explicit
        {
            hard(&mut reasons, "workspace_scope_not_explicitly_opted_in");
        }
        verdict = verdict.meet(NarrowedPreview);
    }
    if profile.sync_posture.is_synced() && !profile.sync_disclosed {
        hard(&mut reasons, "synced_profile_not_disclosed_masquerade");
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.exposure.qualifies_stable() {
        hard(
            &mut reasons,
            "profile_state_hidden_from_inspectable_surfaces",
        );
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.dismissals.qualifies_stable() {
        hard(&mut reasons, "dismissals_not_user_owned_reversible");
        verdict = verdict.meet(NarrowedPreview);
    }
    if !profile.bookmarks.qualifies_stable() {
        hard(&mut reasons, "bookmarks_not_user_owned");
        verdict = verdict.meet(NarrowedPreview);
    }
    for control in &profile.controls {
        if !control.qualifies_stable() {
            reasons.push(format!(
                "control_{}_not_inspectable_keyboard_reversible_non_mutating",
                control.control_kind.as_str()
            ));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    for required in REQUIRED_CONTROL_KINDS {
        if !profile.controls.iter().any(|c| c.control_kind == required) {
            reasons.push(format!("missing_{}_control", required.as_str()));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    for event in &profile.change_history {
        if !event.qualifies_stable() {
            reasons.push(format!(
                "change_event_{}_not_user_initiated_inspectable",
                event.event_id
            ));
            verdict = verdict.meet(NarrowedPreview);
        }
    }

    // ── Disclosed, honest narrowing ──
    if profile.sync_posture.is_synced() && profile.sync_disclosed {
        reasons.push("portable_profile_sync_enabled_state_may_lag_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ─────────────────────────────────────────────────────────────────

/// The canonical manifest binding every learning-mode profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningModeProfileManifest {
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
    /// Learning-mode profiles.
    pub profiles: Vec<LearningModeProfile>,
    /// Overall derived verdict — the strictest verdict across all profiles.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across profiles (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5LearningModeProfileManifest {
    /// Recomputes every profile verdict and the overall verdict from current
    /// evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();
        for profile in &mut self.profiles {
            profile.sync_verdict();
            overall = overall.meet(profile.verdict);
            reasons.extend(profile.narrowing_reasons.iter().cloned());
        }
        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the profile with `profile_id`, if present.
    pub fn profile(&self, profile_id: &str) -> Option<&LearningModeProfile> {
        self.profiles.iter().find(|p| p.profile_id == profile_id)
    }

    /// The set of every profile id the manifest defines.
    pub fn known_profile_ids(&self) -> BTreeSet<String> {
        self.profiles.iter().map(|p| p.profile_id.clone()).collect()
    }
}

/// Reopens a profile manifest from its exported JSON form.
///
/// This is the round-trip used to prove a profile survives export and reopen
/// without losing scope, control, or history identity: the reopened manifest is
/// structurally equal to the original.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_profile_manifest_from_json(
    json: &str,
) -> Result<M5LearningModeProfileManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Validation ───────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_learning_mode_profiles`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningModeProfileValidationError {
    /// Opaque id of the profile or manifest that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for LearningModeProfileValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a manifest against the learning-mode-profile invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any profile's stored verdict diverges from the
/// verdict derived from its evidence; when a profile would change an authority
/// boundary, drift the command graph, store state outside user-owned local-first
/// ownership, force blocking onboarding, drop explain-before-act on a learner
/// preset, prepare an educational-AI "do" outside the standard preview/approval
/// model, commit the profile to the repo, share it with collaborators, scope it
/// to a workspace without an explicit opt-in, sync without disclosure, hide its
/// state from inspectable surfaces, keep non-reversible or non-user-owned
/// dismissals/bookmarks, expose a control that is not keyboard-reachable,
/// reversible, inspectable, and non-mutating, omit a required control, or carry a
/// change event that was not user-initiated and inspectable; or when two profiles
/// share an id.
pub fn validate_m5_learning_mode_profiles(
    manifest: &M5LearningModeProfileManifest,
) -> Result<(), Vec<LearningModeProfileValidationError>> {
    let mut errors: Vec<LearningModeProfileValidationError> = Vec::new();

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for profile in &manifest.profiles {
        let subject = profile.profile_id.clone();
        let err = |message: String| LearningModeProfileValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_ids.insert(profile.profile_id.as_str()) {
            errors.push(err(format!("duplicate profile id {}", profile.profile_id)));
        }

        // Stored verdict must agree with the derived verdict.
        let (derived, derived_reasons) = derive_learning_mode_profile_verdict(profile);
        if derived != profile.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                profile.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != profile.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        // Hard structural invariants.
        if profile.authority_boundary_change_allowed {
            errors.push(err("profile changes an authority boundary".to_string()));
        }
        if !profile.command_graph_unchanged {
            errors.push(err("profile changes the command graph".to_string()));
        }
        if !profile.data_ownership.qualifies_stable() {
            errors.push(err(
                "profile state is not user-owned local-first".to_string()
            ));
        }
        if profile.blocking_onboarding_allowed {
            errors.push(err("profile forces blocking onboarding".to_string()));
        }
        if profile.preset.requires_explain_before_act() && !profile.explain_before_act_default {
            errors.push(err(
                "learner-safe profile drops explain-before-act default".to_string()
            ));
        }
        if profile.ai_explanation_posture.permits_do()
            && !profile.educational_ai_uses_standard_preview_approval
        {
            errors.push(err(
                "educational AI prepares a do outside the standard preview/approval model"
                    .to_string(),
            ));
        }
        if profile.ai_explanation_posture.permits_do()
            && !profile.mutation_guardrail.permits_fenced_do()
        {
            errors.push(err(
                "AI posture prepares a do the mutation guardrail forbids".to_string(),
            ));
        }
        if profile.scope_binding.repo_committed {
            errors.push(err("profile is committed into repo state".to_string()));
        }
        if profile.scope_binding.shared_with_collaborators {
            errors.push(err("profile is shared with collaborators".to_string()));
        }
        if profile.scope_binding.scope == ProfileScope::WorkspaceOptIn
            && !profile.scope_binding.opt_in_explicit
        {
            errors.push(err(
                "workspace-scoped profile is not explicitly opted in".to_string()
            ));
        }
        if profile.sync_posture.is_synced() && !profile.sync_disclosed {
            errors.push(err(
                "synced profile does not disclose sync (masquerade)".to_string()
            ));
        }
        if !profile.exposure.qualifies_stable() {
            errors.push(err(
                "profile state is hidden from settings/help/diagnostics/support".to_string(),
            ));
        }
        if !profile.dismissals.qualifies_stable() {
            errors.push(err(
                "dismissals are not user-owned, reversible, and scope-following".to_string(),
            ));
        }
        if !profile.bookmarks.qualifies_stable() {
            errors.push(err("bookmarks are not user-owned local-first".to_string()));
        }

        for control in &profile.controls {
            if control.mutates_workspace {
                errors.push(err(format!(
                    "control {} mutates workspace state",
                    control.control_kind.as_str()
                )));
            }
            if control.silent_write_allowed {
                errors.push(err(format!(
                    "control {} permits a silent write",
                    control.control_kind.as_str()
                )));
            }
            if control.keyboard_shortcut_ref.is_none() {
                errors.push(err(format!(
                    "control {} is not keyboard reachable",
                    control.control_kind.as_str()
                )));
            }
            if !control.reversible {
                errors.push(err(format!(
                    "control {} is not reversible",
                    control.control_kind.as_str()
                )));
            }
            if !control.inspectable {
                errors.push(err(format!(
                    "control {} is not inspectable",
                    control.control_kind.as_str()
                )));
            }
        }
        let kinds = profile.control_kinds();
        for required in REQUIRED_CONTROL_KINDS {
            if !kinds.contains(&required) {
                errors.push(err(format!(
                    "profile is missing the {} control",
                    required.as_str()
                )));
            }
        }

        for event in &profile.change_history {
            if !event.user_initiated {
                errors.push(err(format!(
                    "change event {} is not user-initiated (silent change)",
                    event.event_id
                )));
            }
            if !event.inspectable_in_support_export {
                errors.push(err(format!(
                    "change event {} is not inspectable in support export",
                    event.event_id
                )));
            }
        }

        if profile.applies_to_families.is_empty() {
            errors.push(err("profile applies to no surface family".to_string()));
        }
    }

    // Manifest-level: overall verdict must fold the members.
    let mut expected_overall = QualificationVerdict::QualifiedStable;
    for profile in &manifest.profiles {
        expected_overall = expected_overall.meet(profile.verdict);
    }
    if expected_overall != manifest.overall_verdict {
        errors.push(LearningModeProfileValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: format!(
                "manifest overall verdict {} disagrees with folded member verdict {}",
                manifest.overall_verdict.as_str(),
                expected_overall.as_str()
            ),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ── Seed builders ────────────────────────────────────────────────────────────

/// Builds the standard, fully-inspectable control set every profile exposes.
fn standard_controls(token: &str) -> Vec<ProfileControl> {
    let spec = [
        (
            ProfileControlKind::Enable,
            "cmd:learning.enable",
            "disabled → enabled",
            "kb:learning.enable",
        ),
        (
            ProfileControlKind::Disable,
            "cmd:learning.disable",
            "enabled → disabled",
            "kb:learning.disable",
        ),
        (
            ProfileControlKind::Pause,
            "cmd:learning.pause",
            "enabled → paused",
            "kb:learning.pause",
        ),
        (
            ProfileControlKind::Snooze,
            "cmd:learning.snooze",
            "enabled → snoozed",
            "kb:learning.snooze",
        ),
        (
            ProfileControlKind::Resume,
            "cmd:learning.resume",
            "paused/snoozed → enabled",
            "kb:learning.resume",
        ),
        (
            ProfileControlKind::Reset,
            "cmd:learning.reset",
            "any → preset defaults",
            "kb:learning.reset",
        ),
        (
            ProfileControlKind::Narrow,
            "cmd:learning.narrow",
            "enabled → quieter",
            "kb:learning.narrow",
        ),
    ];
    spec.iter()
        .map(|(kind, command, transition, shortcut)| ProfileControl {
            control_id: format!("learning:m5:profile:{token}:control:{}", kind.as_str()),
            control_kind: *kind,
            command_id_ref: (*command).to_string(),
            state_transition: (*transition).to_string(),
            keyboard_shortcut_ref: Some((*shortcut).to_string()),
            reversible: true,
            inspectable: true,
            silent_write_allowed: false,
            mutates_workspace: false,
        })
        .collect()
}

/// Builds a small, deterministic change history for a profile.
fn standard_change_history(token: &str, tip_change: ProfileChangeKind) -> Vec<ProfileChangeEvent> {
    vec![
        ProfileChangeEvent {
            event_id: format!("learning:m5:profile:{token}:event:enabled"),
            at: "2026-06-19T00:00:00Z".to_string(),
            change_kind: ProfileChangeKind::Enabled,
            from_state: "disabled".to_string(),
            to_state: "enabled".to_string(),
            user_initiated: true,
            inspectable_in_support_export: true,
        },
        ProfileChangeEvent {
            event_id: format!("learning:m5:profile:{token}:event:tuned"),
            at: "2026-06-19T00:05:00Z".to_string(),
            change_kind: tip_change,
            from_state: "preset_default".to_string(),
            to_state: "user_tuned".to_string(),
            user_initiated: true,
            inspectable_in_support_export: true,
        },
    ]
}

/// Specification for one seeded profile, expanded into a full record by
/// [`build_profile`].
struct ProfileSpec {
    token: &'static str,
    display_label: &'static str,
    preset: LearningModePreset,
    tip_intensity: TipIntensity,
    jargon_level: JargonLevel,
    ai_explanation_posture: AiExplanationPosture,
    mutation_guardrail: MutationGuardrail,
    explain_before_act_default: bool,
    scope: ProfileScope,
    sync_posture: SyncPosture,
}

/// Expands a [`ProfileSpec`] into a full, validated [`LearningModeProfile`].
fn build_profile(spec: ProfileSpec) -> LearningModeProfile {
    let synced = spec.sync_posture.is_synced();
    let mut profile = LearningModeProfile {
        record_kind: LEARNING_MODE_PROFILE_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_MODE_PROFILES_SCHEMA_VERSION,
        profile_id: format!("learning:m5:profile:{}", spec.token),
        display_label: spec.display_label.to_string(),
        preset: spec.preset,
        profile_state: ProfileState::Enabled,
        scope_binding: ScopeBinding {
            scope: spec.scope,
            opt_in_explicit: true,
            repo_committed: false,
            shared_with_collaborators: false,
            storage_location_ref: match spec.scope {
                ProfileScope::UserLocal => "store:user_profile:learning".to_string(),
                ProfileScope::WorkspaceOptIn => "store:workspace_local:learning".to_string(),
            },
        },
        tip_intensity: spec.tip_intensity,
        jargon_level: spec.jargon_level,
        ai_explanation_posture: spec.ai_explanation_posture,
        mutation_guardrail: spec.mutation_guardrail,
        educational_ai_uses_standard_preview_approval: true,
        explain_before_act_default: spec.explain_before_act_default,
        authority_boundary_change_allowed: false,
        command_graph_unchanged: true,
        data_ownership: DataOwnershipClass::UserOwnedLocalFirst,
        blocking_onboarding_allowed: false,
        sync_posture: spec.sync_posture,
        sync_disclosed: synced,
        dismissals: DismissalState {
            dismissed_surface_refs: vec![format!("surface:tip:{}:welcome", spec.token)],
            reversible: true,
            user_owned_local: true,
            follows_profile_scope: true,
        },
        bookmarks: BookmarkState {
            bookmarked_surface_refs: vec![format!("surface:glossary:{}:trust_model", spec.token)],
            user_owned_local: true,
            exportable_in_portable_profile: true,
        },
        controls: standard_controls(spec.token),
        change_history: standard_change_history(spec.token, ProfileChangeKind::TipIntensityChanged),
        exposure: SurfaceExposure {
            in_settings: true,
            in_help_about: true,
            in_diagnostics: true,
            in_support_export: true,
            hidden_in_transient_overlay_only: false,
        },
        applies_to_families: M5LearningSurfaceFamily::ALL.to_vec(),
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    profile.sync_verdict();
    profile
}

/// Produces the canonical seeded learning-mode-profile manifest.
///
/// Three user-local presets ship Stable — [`LearningModePreset::ExpertMinimal`],
/// [`LearningModePreset::BalancedDefault`], and
/// [`LearningModePreset::GuidedLearner`] — and one workspace-opt-in,
/// portable-profile-synced profile narrows to
/// [`QualificationVerdict::NarrowedBeta`] with its sync disclosed, so the overall
/// manifest verdict is `narrowed_beta`.
pub fn seeded_m5_learning_mode_profiles() -> M5LearningModeProfileManifest {
    let profiles = vec![
        build_profile(ProfileSpec {
            token: "expert_minimal_user_local",
            display_label: "Expert (minimal guidance)",
            preset: LearningModePreset::ExpertMinimal,
            tip_intensity: TipIntensity::SilentInlineOnly,
            jargon_level: JargonLevel::Expert,
            ai_explanation_posture: AiExplanationPosture::ExplainThenPreparePreview,
            mutation_guardrail: MutationGuardrail::ApprovalRequired,
            explain_before_act_default: false,
            scope: ProfileScope::UserLocal,
            sync_posture: SyncPosture::LocalOnly,
        }),
        build_profile(ProfileSpec {
            token: "balanced_default_user_local",
            display_label: "Balanced (default)",
            preset: LearningModePreset::BalancedDefault,
            tip_intensity: TipIntensity::GentleHint,
            jargon_level: JargonLevel::Intermediate,
            ai_explanation_posture: AiExplanationPosture::ExplainThenPreparePreview,
            mutation_guardrail: MutationGuardrail::ApprovalRequired,
            explain_before_act_default: true,
            scope: ProfileScope::UserLocal,
            sync_posture: SyncPosture::LocalOnly,
        }),
        build_profile(ProfileSpec {
            token: "guided_learner_user_local",
            display_label: "Guided learner",
            preset: LearningModePreset::GuidedLearner,
            tip_intensity: TipIntensity::PromptedAcknowledge,
            jargon_level: JargonLevel::Beginner,
            ai_explanation_posture: AiExplanationPosture::PreviewOnlyAfterExplicitDo,
            mutation_guardrail: MutationGuardrail::ApprovalRequired,
            explain_before_act_default: true,
            scope: ProfileScope::UserLocal,
            sync_posture: SyncPosture::LocalOnly,
        }),
        build_profile(ProfileSpec {
            token: "balanced_workspace_opt_in",
            display_label: "Balanced (this workspace)",
            preset: LearningModePreset::BalancedDefault,
            tip_intensity: TipIntensity::GentleHint,
            jargon_level: JargonLevel::Intermediate,
            ai_explanation_posture: AiExplanationPosture::ExplainThenPreparePreview,
            mutation_guardrail: MutationGuardrail::ApprovalRequired,
            explain_before_act_default: true,
            scope: ProfileScope::WorkspaceOptIn,
            sync_posture: SyncPosture::PortableProfileSynced,
        }),
    ];

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "schema".to_string(),
        M5_LEARNING_MODE_PROFILES_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "doc".to_string(),
        M5_LEARNING_MODE_PROFILES_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "artifact".to_string(),
        M5_LEARNING_MODE_PROFILES_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "feature_family_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );

    let mut manifest = M5LearningModeProfileManifest {
        record_kind: M5_LEARNING_MODE_PROFILE_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_MODE_PROFILES_SCHEMA_VERSION,
        manifest_id: "learning:m5:profile_manifest:v1".to_string(),
        generated_at: "2026-06-19T00:00:00Z".to_string(),
        contract_refs,
        profiles,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: Vec::new(),
    };
    manifest.sync_verdicts();
    manifest
}
