//! Support/export-safe reset and portability for tour, exercise, and
//! learning-session state across Aureline's M5 feature families.
//!
//! Where [`crate::progress_snapshots`] owns the *memory* of a learnability flow
//! (how far a person got, its resume point, its disclosure state), this module
//! owns the *operations that carry that memory out or clear it*: a
//! [`LearningStateExportBundle`] is the support/export-safe packet that ports
//! tour/exercise/learning-session state to a portable profile, support bundle, or
//! local download — preserving provenance, keeping the privacy posture, redacting
//! raw payloads, disclosing cached-pack continuity, and keeping a source-language
//! escape visible when a localized or mirrored artifact is presented — and a
//! [`LearningStateResetPlan`] is the packet that clears a bounded slice of
//! learnability state with a declared target scope, an explicit protected set
//! (docs packs, bookmarks, user-authored notes are never silently deleted), and a
//! reversible restore.
//!
//! ## What an export bundle proves
//!
//! - **Provenance survives.** [`LearningStateExportBundle::source_state_refs`]
//!   names the snapshots/profiles the bundle carries, and `provenance_preserved`
//!   keeps the trail back to where the state came from.
//! - **Redaction is total.** A [`RedactionPosture`] redacts raw payloads,
//!   credential bodies, and absolute paths, and never widens data sharing — an
//!   export carries progress out without leaking workspace bodies or granting new
//!   read access.
//! - **Cached-pack continuity stays visible.** A [`CachedPackContinuity`]
//!   discloses the source class and freshness of the artifact the state was
//!   learned against; a non-live pack must be disclosed, never masqueraded as live.
//! - **The source-language escape stays one step away.** When display copy is
//!   localized, a [`SourceLanguageEscape`] keeps a command-backed escape to the
//!   source-language original, so a localized export never strands a reader in a
//!   translation.
//!
//! ## What a reset plan proves
//!
//! - **The target scope is explicit.** [`LearningStateResetPlan::target_state_kinds`]
//!   declares exactly which learnability classes are cleared.
//! - **Unrelated state is protected.** [`LearningStateResetPlan::protected_classes`]
//!   MUST list docs packs, bookmarks, and user-authored notes; a reset never
//!   silently deletes state outside the reviewed learnability scope.
//! - **Reset is reversible.** A restore is available, command-backed, and its
//!   window is disclosed — reset is never a one-way door.
//!
//! ## Invariants enforced
//!
//! - **No hidden mutating tutorial path.** A [`TutorialMutationFence`] guards every
//!   bundle and plan: no export or reset path may introduce a tutorial-only
//!   mutating shortcut, bypass the standard preview/approval model when it touches
//!   real workspace state, change an authority boundary, or drift the command
//!   graph.
//! - **State stays user-owned and local-first.** A bundle or plan whose ownership
//!   is not user-owned local-first narrows below Stable; export never widens data
//!   sharing.
//! - **Nothing happens silently.** Exports and resets are user-initiated; a silent
//!   export or reset narrows below Stable.
//! - **Continuity is honest.** A cached, local-only, or stale pack that is
//!   disclosed is an honest, user-visible deviation that narrows to Beta; an
//!   undisclosed non-live pack is a masquerade that narrows to Preview.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_learning_state_export_and_reset`] produces the canonical manifest.
//! Settings, Help/About, diagnostics, support export, and docs/migration surfaces
//! ingest it rather than rephrasing export/reset, privacy, or continuity state by
//! hand.
//!
//! - Schema: [`M5_LEARNING_STATE_PORTABILITY_SCHEMA_REF`]
//! - Fixture: [`M5_LEARNING_STATE_PORTABILITY_FIXTURE_REF`]
//! - Artifact: [`M5_LEARNING_STATE_PORTABILITY_ARTIFACT_REF`]
//! - Doc: [`M5_LEARNING_STATE_PORTABILITY_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::freeze_m5_learnability_lane::DataOwnershipClass;
use crate::m5_feature_family_learning_rails::{
    M5LearningSurfaceFamily, M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF,
};
use crate::progress_snapshots::{ExportTargetKind, M5_LEARNING_PROGRESS_SCHEMA_REF};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    QualificationVerdict, GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};
use crate::tour_and_glossary_packages::{
    FreshnessState, SourceClass, M5_TOUR_AND_GLOSSARY_SCHEMA_REF,
};

#[cfg(test)]
mod tests;

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the learning-state portability records. Bumped
/// only on breaking payload changes; additive-optional fields do not bump it.
pub const M5_LEARNING_STATE_PORTABILITY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LearningStateExportBundle`].
pub const LEARNING_STATE_EXPORT_BUNDLE_RECORD_KIND: &str = "learning_state_export_bundle";

/// Record kind for [`LearningStateResetPlan`].
pub const LEARNING_STATE_RESET_PLAN_RECORD_KIND: &str = "learning_state_reset_plan";

/// Record kind for [`M5LearningStatePortabilityManifest`].
pub const M5_LEARNING_STATE_PORTABILITY_MANIFEST_RECORD_KIND: &str =
    "m5_learning_state_portability_manifest";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the learning-state portability schema.
pub const M5_LEARNING_STATE_PORTABILITY_SCHEMA_REF: &str =
    "schemas/help/learning-session-export.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_LEARNING_STATE_PORTABILITY_FIXTURE_REF: &str =
    "fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json";

/// Repository-relative path to the proof artifact.
pub const M5_LEARNING_STATE_PORTABILITY_ARTIFACT_REF: &str =
    "artifacts/ux/m5/learning-state-portability-proof/add-learning-state-export-and-reset.md";

/// Repository-relative path to the public doc.
pub const M5_LEARNING_STATE_PORTABILITY_DOC_REF: &str = "docs/m5/learning-state-portability.md";

// ── Learnability state kinds ──────────────────────────────────────────────────

/// A class of learnability state an export or reset operation may touch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityStateKind {
    /// Progress through a guided tour.
    TourState,
    /// Progress through a hands-on guided exercise rail.
    ExerciseState,
    /// An in-progress learning session (the resumable session container).
    LearningSessionState,
    /// Progress through a glossary-pack walkthrough.
    GlossaryState,
    /// A user's opt-in learning-mode profile.
    LearningModeProfile,
    /// Per-surface contextual hint and coachmark dismissal state.
    ContextualHintState,
}

impl LearnabilityStateKind {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TourState => "tour_state",
            Self::ExerciseState => "exercise_state",
            Self::LearningSessionState => "learning_session_state",
            Self::GlossaryState => "glossary_state",
            Self::LearningModeProfile => "learning_mode_profile",
            Self::ContextualHintState => "contextual_hint_state",
        }
    }
}

// ── Protected state classes ───────────────────────────────────────────────────

/// A class of user-owned state that a learnability reset MUST NOT delete.
///
/// These are *outside* the reviewed learnability scope: clearing tour or exercise
/// progress never silently removes the docs packs a person installed, the
/// bookmarks they saved, or the notes they authored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedStateClass {
    /// An installed docs/knowledge pack.
    DocsPack,
    /// A user-saved bookmark.
    Bookmark,
    /// A user-authored note.
    UserAuthoredNote,
    /// An installed model pack.
    ModelPack,
    /// A saved checkpoint.
    Checkpoint,
    /// An installed template pack.
    TemplatePack,
}

impl ProtectedStateClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPack => "docs_pack",
            Self::Bookmark => "bookmark",
            Self::UserAuthoredNote => "user_authored_note",
            Self::ModelPack => "model_pack",
            Self::Checkpoint => "checkpoint",
            Self::TemplatePack => "template_pack",
        }
    }
}

/// The protected classes every reset plan MUST list as preserved, so a reset can
/// never silently erase unrelated user-owned help/docs state.
pub const REQUIRED_PROTECTED_CLASSES: [ProtectedStateClass; 3] = [
    ProtectedStateClass::DocsPack,
    ProtectedStateClass::Bookmark,
    ProtectedStateClass::UserAuthoredNote,
];

// ── Redaction posture ─────────────────────────────────────────────────────────

/// The redaction posture an export applies before learnability state leaves the
/// device.
///
/// An export carries progress out; it MUST redact raw payloads, credential
/// bodies, and absolute paths, and it MUST NOT widen data sharing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionPosture {
    /// Whether the export redacts raw provider payloads. MUST be true.
    pub redacts_raw_payloads: bool,
    /// Whether the export redacts credential bodies. MUST be true.
    pub redacts_credentials: bool,
    /// Whether the export redacts raw absolute paths. MUST be true.
    pub redacts_absolute_paths: bool,
    /// Whether the export widens who can read the state (repo, collaborators, a
    /// new service). MUST be false.
    pub widens_data_sharing: bool,
}

impl RedactionPosture {
    /// Returns true when the posture satisfies Stable redaction requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.redacts_raw_payloads
            && self.redacts_credentials
            && self.redacts_absolute_paths
            && !self.widens_data_sharing
    }
}

// ── Tutorial mutation fence ───────────────────────────────────────────────────

/// The fence that keeps an export or reset path from becoming a hidden mutating
/// tutorial shortcut.
///
/// No learnability portability or reset path may introduce a tutorial-only
/// mutating shortcut, bypass the standard preview/approval model when it touches
/// real workspace state, change an authority boundary, or drift the command
/// graph. Explain stays separate from do, and any do uses the same path as
/// ordinary work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TutorialMutationFence {
    /// Whether the path introduces a tutorial-only mutating shortcut. MUST be
    /// false.
    pub introduces_tutorial_only_mutating_shortcut: bool,
    /// Whether the path bypasses the standard preview/approval model. MUST be
    /// false.
    pub bypasses_preview_approval: bool,
    /// Whether the path touches real workspace state at all.
    pub touches_real_workspace_state: bool,
    /// Whether, when it touches real workspace state, it routes through the
    /// standard preview/approval model. MUST be true when
    /// `touches_real_workspace_state` is true.
    pub uses_standard_preview_approval_when_touching_workspace: bool,
    /// Whether the path is allowed to change an authority boundary. MUST be false.
    pub authority_boundary_change_allowed: bool,
    /// Whether the command graph stays unchanged. MUST be true.
    pub command_graph_unchanged: bool,
}

impl TutorialMutationFence {
    /// A clean fence: touches no workspace state, opens no shortcut, drifts
    /// nothing.
    pub fn clean() -> Self {
        Self {
            introduces_tutorial_only_mutating_shortcut: false,
            bypasses_preview_approval: false,
            touches_real_workspace_state: false,
            uses_standard_preview_approval_when_touching_workspace: true,
            authority_boundary_change_allowed: false,
            command_graph_unchanged: true,
        }
    }

    /// Returns true when the fence satisfies every Stable requirement.
    pub fn qualifies_stable(&self) -> bool {
        !self.introduces_tutorial_only_mutating_shortcut
            && !self.bypasses_preview_approval
            && !self.authority_boundary_change_allowed
            && self.command_graph_unchanged
            && (!self.touches_real_workspace_state
                || self.uses_standard_preview_approval_when_touching_workspace)
    }
}

/// Folds a [`TutorialMutationFence`]'s findings into a running verdict.
///
/// Every fence breach is a hard safety violation and narrows to
/// [`QualificationVerdict::NarrowedPreview`].
fn fold_mutation_fence(
    fence: &TutorialMutationFence,
    label: &str,
    verdict: &mut QualificationVerdict,
    reasons: &mut Vec<String>,
) {
    use QualificationVerdict::NarrowedPreview;
    if fence.introduces_tutorial_only_mutating_shortcut {
        reasons.push(format!(
            "{label}_introduces_tutorial_only_mutating_shortcut"
        ));
        *verdict = verdict.meet(NarrowedPreview);
    }
    if fence.bypasses_preview_approval {
        reasons.push(format!("{label}_bypasses_preview_approval"));
        *verdict = verdict.meet(NarrowedPreview);
    }
    if fence.authority_boundary_change_allowed {
        reasons.push(format!("{label}_changes_authority_boundary"));
        *verdict = verdict.meet(NarrowedPreview);
    }
    if !fence.command_graph_unchanged {
        reasons.push(format!("{label}_changes_command_graph"));
        *verdict = verdict.meet(NarrowedPreview);
    }
    if fence.touches_real_workspace_state
        && !fence.uses_standard_preview_approval_when_touching_workspace
    {
        reasons.push(format!(
            "{label}_touches_workspace_outside_standard_preview_approval"
        ));
        *verdict = verdict.meet(NarrowedPreview);
    }
}

// ── Source-language escape ────────────────────────────────────────────────────

/// The source-language escape hatch for a localized export.
///
/// When display copy is localized, a reader must always be able to step back to
/// the source-language original. The escape is command-backed and one step away,
/// and localization never disturbs provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLanguageEscape {
    /// Whether the presented copy is localized away from its source language.
    pub presented_localized: bool,
    /// BCP-47 source (authoring) locale, e.g. `en-US`.
    pub source_locale: String,
    /// BCP-47 presented locale, e.g. `fr-FR`.
    pub presented_locale: String,
    /// Whether an escape to the source-language original is available. MUST be
    /// true when `presented_localized` is true.
    pub escape_to_source_available: bool,
    /// Opaque ref to the command that backs the source-language escape. MUST be
    /// present when `presented_localized` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escape_command_ref: Option<String>,
    /// Whether localization preserves provenance (it changes display copy only,
    /// never identity or anchors). MUST be true.
    pub preserves_provenance: bool,
}

impl SourceLanguageEscape {
    /// An unlocalized escape: source and presented locale match, so no escape is
    /// required.
    pub fn unlocalized(locale: &str) -> Self {
        Self {
            presented_localized: false,
            source_locale: locale.to_string(),
            presented_locale: locale.to_string(),
            escape_to_source_available: false,
            escape_command_ref: None,
            preserves_provenance: true,
        }
    }

    /// Returns true when the escape satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.preserves_provenance
            && (!self.presented_localized
                || (self.escape_to_source_available && self.escape_command_ref.is_some()))
    }
}

// ── Cached-pack continuity ────────────────────────────────────────────────────

/// The cached/offline continuity disclosure for the pack a learnability flow was
/// learned against.
///
/// A cached, mirrored, local-only, or stale pack stays visibly distinct from live
/// authoritative knowledge: a non-live source MUST be disclosed, never
/// masqueraded as current.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedPackContinuity {
    /// Where the pack's content originates.
    pub source_class: SourceClass,
    /// How current the pack is relative to the live authoritative source.
    pub freshness: FreshnessState,
    /// Whether a non-live freshness state is disclosed to the user. MUST be true
    /// whenever `freshness` is not live.
    pub continuity_disclosed: bool,
    /// Opaque ref to the offline mirror the pack was served from, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_mirror_ref: Option<String>,
}

impl CachedPackContinuity {
    /// Returns true when a non-live freshness state is disclosed (no masquerade).
    pub fn disclosure_is_honest(&self) -> bool {
        self.freshness.is_live() || self.continuity_disclosed
    }
}

// ── Export bundle ─────────────────────────────────────────────────────────────

/// One support/export-safe bundle that ports a slice of learnability state out of
/// the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningStateExportBundle {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this bundle.
    pub bundle_id: String,
    /// Human-readable label shown in the export sheet and support export.
    pub display_label: String,
    /// The M5 surface family this state belongs to.
    pub family: M5LearningSurfaceFamily,
    /// The class of learnability state this bundle carries.
    pub state_kind: LearnabilityStateKind,
    /// Where the export is written.
    pub target_kind: ExportTargetKind,
    /// Opaque refs to the source state (snapshots/profiles) the bundle carries —
    /// the provenance trail. MUST be non-empty.
    pub source_state_refs: Vec<String>,
    /// Whether the export preserves provenance back to its source state. MUST be
    /// true.
    pub provenance_preserved: bool,
    /// Data-ownership class of the exported state. MUST be user-owned local-first.
    pub data_ownership: DataOwnershipClass,
    /// Redaction posture applied before the state leaves the device.
    pub redaction: RedactionPosture,
    /// Source-language escape posture for localized copy.
    pub source_language: SourceLanguageEscape,
    /// Cached/offline continuity disclosure for the pack the state was learned
    /// against.
    pub cached_pack: CachedPackContinuity,
    /// Tutorial-mutation fence.
    pub mutation_fence: TutorialMutationFence,
    /// Whether the bundle is safe to include in support exports.
    pub safe_for_support_export: bool,
    /// Whether the bundle includes step-level progress state.
    pub includes_progress_state: bool,
    /// Whether the export was user-initiated. MUST be true — no silent exports.
    pub user_initiated: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearningStateExportBundle {
    /// Recomputes this bundle's verdict and narrowing reasons, writing them back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_export_bundle_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Reset plan ────────────────────────────────────────────────────────────────

/// One reset plan that clears a bounded slice of learnability state while
/// protecting unrelated user-owned state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningStateResetPlan {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this plan.
    pub plan_id: String,
    /// Human-readable label shown in the reset sheet and support export.
    pub display_label: String,
    /// The learnability classes this reset clears. MUST be non-empty.
    pub target_state_kinds: Vec<LearnabilityStateKind>,
    /// The classes this reset preserves. MUST list docs packs, bookmarks, and
    /// user-authored notes.
    pub protected_classes: Vec<ProtectedStateClass>,
    /// Whether the reset may delete state outside its declared scope. MUST be
    /// false.
    pub silently_deletes_outside_scope: bool,
    /// Whether a restore is available after the reset. MUST be true — reset is
    /// reversible.
    pub restore_available: bool,
    /// Whether the restore window is disclosed. MUST be true when
    /// `restore_available` is true.
    pub restore_window_disclosed: bool,
    /// Opaque ref to the command that backs the restore. MUST be present when
    /// `restore_available` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_command_ref: Option<String>,
    /// Data-ownership class of the reset state. MUST be user-owned local-first.
    pub data_ownership: DataOwnershipClass,
    /// Tutorial-mutation fence.
    pub mutation_fence: TutorialMutationFence,
    /// Whether the plan is safe to include in support exports.
    pub safe_for_support_export: bool,
    /// Whether the reset was user-initiated. MUST be true — no silent resets.
    pub user_initiated: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearningStateResetPlan {
    /// The set of protected classes this plan declares.
    pub fn protected_set(&self) -> BTreeSet<ProtectedStateClass> {
        self.protected_classes.iter().copied().collect()
    }

    /// Returns true when the plan protects every class in
    /// [`REQUIRED_PROTECTED_CLASSES`].
    pub fn protects_required_classes(&self) -> bool {
        let set = self.protected_set();
        REQUIRED_PROTECTED_CLASSES.iter().all(|c| set.contains(c))
    }

    /// Recomputes this plan's verdict and narrowing reasons, writing them back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_reset_plan_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Verdict derivation ────────────────────────────────────────────────────────

/// Derives an export bundle's verdict and narrowing reasons from its evidence.
///
/// Hard safety violations (incomplete redaction, widened sharing, non-user
/// ownership, dropped provenance, a silent export, a missing source-language
/// escape on localized copy, an undisclosed non-live pack masquerade, or any
/// tutorial-mutation-fence breach) narrow to
/// [`QualificationVerdict::NarrowedPreview`]. A disclosed cached/local-only/stale
/// pack is an honest, user-visible deviation that narrows to
/// [`QualificationVerdict::NarrowedBeta`]. With no findings the bundle is
/// [`QualificationVerdict::QualifiedStable`].
pub fn derive_export_bundle_verdict(
    bundle: &LearningStateExportBundle,
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    // ── Redaction and sharing ──
    if !bundle.redaction.redacts_raw_payloads {
        reasons.push("export_does_not_redact_raw_payloads".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !bundle.redaction.redacts_credentials {
        reasons.push("export_does_not_redact_credentials".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !bundle.redaction.redacts_absolute_paths {
        reasons.push("export_does_not_redact_absolute_paths".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if bundle.redaction.widens_data_sharing {
        reasons.push("export_widens_data_sharing".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Ownership, provenance, intent ──
    if !bundle.data_ownership.qualifies_stable() {
        reasons.push("exported_state_not_user_owned_local_first".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !bundle.provenance_preserved {
        reasons.push("export_drops_provenance".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if bundle.source_state_refs.is_empty() {
        reasons.push("export_has_no_source_state_ref".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !bundle.user_initiated {
        reasons.push("export_not_user_initiated_silent".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── No hidden mutating tutorial path ──
    fold_mutation_fence(&bundle.mutation_fence, "export", &mut verdict, &mut reasons);

    // ── Source-language escape ──
    if !bundle.source_language.preserves_provenance {
        reasons.push("localization_drops_provenance".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if bundle.source_language.presented_localized
        && !(bundle.source_language.escape_to_source_available
            && bundle.source_language.escape_command_ref.is_some())
    {
        reasons.push("localized_export_without_source_language_escape".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Cached-pack continuity ──
    if !bundle.cached_pack.disclosure_is_honest() {
        reasons.push("cached_pack_continuity_not_disclosed_masquerade".to_string());
        verdict = verdict.meet(NarrowedPreview);
    } else if !bundle.cached_pack.freshness.qualifies_stable() {
        reasons.push("cached_pack_content_may_lag_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

/// Derives a reset plan's verdict and narrowing reasons from its evidence.
///
/// A reset narrows to [`QualificationVerdict::NarrowedPreview`] when it has no
/// target scope, may delete state outside its scope, fails to protect docs packs
/// / bookmarks / user notes, is not reversible (no command-backed restore with a
/// disclosed window), stores non-user-owned state, runs silently, or breaches the
/// tutorial-mutation fence. With no findings the plan is
/// [`QualificationVerdict::QualifiedStable`].
pub fn derive_reset_plan_verdict(
    plan: &LearningStateResetPlan,
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    if plan.target_state_kinds.is_empty() {
        reasons.push("reset_has_no_target_scope".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if plan.silently_deletes_outside_scope {
        reasons.push("reset_silently_deletes_outside_scope".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    let protected = plan.protected_set();
    for required in REQUIRED_PROTECTED_CLASSES {
        if !protected.contains(&required) {
            reasons.push(format!("reset_does_not_protect_{}", required.as_str()));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    if !plan.restore_available {
        reasons.push("reset_not_reversible_no_restore".to_string());
        verdict = verdict.meet(NarrowedPreview);
    } else {
        if !plan.restore_window_disclosed {
            reasons.push("reset_restore_window_not_disclosed".to_string());
            verdict = verdict.meet(NarrowedPreview);
        }
        if plan.restore_command_ref.is_none() {
            reasons.push("reset_restore_not_command_backed".to_string());
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    if !plan.data_ownership.qualifies_stable() {
        reasons.push("reset_state_not_user_owned_local_first".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !plan.user_initiated {
        reasons.push("reset_not_user_initiated_silent".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    fold_mutation_fence(&plan.mutation_fence, "reset", &mut verdict, &mut reasons);

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// The canonical manifest binding every export bundle and reset plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningStatePortabilityManifest {
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
    /// Export bundles.
    pub export_bundles: Vec<LearningStateExportBundle>,
    /// Reset plans.
    pub reset_plans: Vec<LearningStateResetPlan>,
    /// Overall derived verdict — the strictest verdict across bundles and plans.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across records (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5LearningStatePortabilityManifest {
    /// Recomputes every bundle and plan verdict and the overall verdict from
    /// current evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();

        for bundle in &mut self.export_bundles {
            bundle.sync_verdict();
            overall = overall.meet(bundle.verdict);
            reasons.extend(bundle.narrowing_reasons.iter().cloned());
        }
        for plan in &mut self.reset_plans {
            plan.sync_verdict();
            overall = overall.meet(plan.verdict);
            reasons.extend(plan.narrowing_reasons.iter().cloned());
        }

        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the export bundle with `bundle_id`, if present.
    pub fn export_bundle(&self, bundle_id: &str) -> Option<&LearningStateExportBundle> {
        self.export_bundles
            .iter()
            .find(|b| b.bundle_id == bundle_id)
    }

    /// Returns the reset plan with `plan_id`, if present.
    pub fn reset_plan(&self, plan_id: &str) -> Option<&LearningStateResetPlan> {
        self.reset_plans.iter().find(|p| p.plan_id == plan_id)
    }
}

/// Reopens a portability manifest from its exported JSON form.
///
/// This is the round-trip used to prove learnability state survives export and
/// reopen without losing provenance, redaction, continuity, source-language
/// escape, or reset-scope identity: the reopened manifest is structurally equal to
/// the original.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_portability_manifest_from_json(
    json: &str,
) -> Result<M5LearningStatePortabilityManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Validation ──────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_learning_state_export_and_reset`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningStatePortabilityValidationError {
    /// Opaque id of the bundle, plan, or manifest that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for LearningStatePortabilityValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a manifest against the learning-state portability invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any record's stored verdict diverges from the
/// verdict derived from its evidence; when an export fails to redact raw
/// payloads/credentials/absolute paths, widens data sharing, stores non-user-owned
/// state, drops provenance, carries no source-state ref, runs silently, localizes
/// without a source-language escape, masquerades a non-live pack as live, or
/// breaches the tutorial-mutation fence; when a reset has no target scope, may
/// delete outside its scope, fails to protect docs packs / bookmarks / user notes,
/// is irreversible, stores non-user-owned state, runs silently, or breaches the
/// fence; when two bundles or two plans share an id; when the manifest carries no
/// export bundle or no reset plan; or when the manifest's overall verdict does not
/// fold its members.
pub fn validate_m5_learning_state_export_and_reset(
    manifest: &M5LearningStatePortabilityManifest,
) -> Result<(), Vec<LearningStatePortabilityValidationError>> {
    let mut errors: Vec<LearningStatePortabilityValidationError> = Vec::new();

    // ── Export bundles ──
    let mut seen_bundle_ids: BTreeSet<&str> = BTreeSet::new();
    for bundle in &manifest.export_bundles {
        let subject = bundle.bundle_id.clone();
        let err = |message: String| LearningStatePortabilityValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_bundle_ids.insert(bundle.bundle_id.as_str()) {
            errors.push(err(format!("duplicate bundle id {}", bundle.bundle_id)));
        }

        let (derived, derived_reasons) = derive_export_bundle_verdict(bundle);
        if derived != bundle.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                bundle.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != bundle.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if !bundle.redaction.redacts_raw_payloads {
            errors.push(err("export does not redact raw payloads".to_string()));
        }
        if !bundle.redaction.redacts_credentials {
            errors.push(err("export does not redact credentials".to_string()));
        }
        if !bundle.redaction.redacts_absolute_paths {
            errors.push(err("export does not redact absolute paths".to_string()));
        }
        if bundle.redaction.widens_data_sharing {
            errors.push(err("export widens data sharing".to_string()));
        }
        if !bundle.data_ownership.qualifies_stable() {
            errors.push(err(
                "exported state is not user-owned local-first".to_string()
            ));
        }
        if !bundle.provenance_preserved {
            errors.push(err("export drops provenance".to_string()));
        }
        if bundle.source_state_refs.is_empty() {
            errors.push(err("export carries no source-state ref".to_string()));
        }
        if !bundle.user_initiated {
            errors.push(err(
                "export is not user-initiated (silent export)".to_string()
            ));
        }
        validate_mutation_fence(&bundle.mutation_fence, "export", &err, &mut errors);
        if !bundle.source_language.preserves_provenance {
            errors.push(err("localization drops provenance".to_string()));
        }
        if bundle.source_language.presented_localized
            && !(bundle.source_language.escape_to_source_available
                && bundle.source_language.escape_command_ref.is_some())
        {
            errors.push(err(
                "localized export lacks a command-backed source-language escape".to_string(),
            ));
        }
        if !bundle.cached_pack.disclosure_is_honest() {
            errors.push(err(
                "non-live cached pack is not disclosed (continuity masquerade)".to_string(),
            ));
        }
    }

    // ── Reset plans ──
    let mut seen_plan_ids: BTreeSet<&str> = BTreeSet::new();
    for plan in &manifest.reset_plans {
        let subject = plan.plan_id.clone();
        let err = |message: String| LearningStatePortabilityValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_plan_ids.insert(plan.plan_id.as_str()) {
            errors.push(err(format!("duplicate plan id {}", plan.plan_id)));
        }

        let (derived, derived_reasons) = derive_reset_plan_verdict(plan);
        if derived != plan.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                plan.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != plan.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if plan.target_state_kinds.is_empty() {
            errors.push(err("reset has no target scope".to_string()));
        }
        if plan.silently_deletes_outside_scope {
            errors.push(err("reset may delete state outside its scope".to_string()));
        }
        let protected = plan.protected_set();
        for required in REQUIRED_PROTECTED_CLASSES {
            if !protected.contains(&required) {
                errors.push(err(format!(
                    "reset does not protect {} (would erase unrelated user state)",
                    required.as_str()
                )));
            }
        }
        if !plan.restore_available {
            errors.push(err("reset is not reversible (no restore)".to_string()));
        } else {
            if !plan.restore_window_disclosed {
                errors.push(err("reset restore window is not disclosed".to_string()));
            }
            if plan.restore_command_ref.is_none() {
                errors.push(err("reset restore is not command-backed".to_string()));
            }
        }
        if !plan.data_ownership.qualifies_stable() {
            errors.push(err("reset state is not user-owned local-first".to_string()));
        }
        if !plan.user_initiated {
            errors.push(err("reset is not user-initiated (silent reset)".to_string()));
        }
        validate_mutation_fence(&plan.mutation_fence, "reset", &err, &mut errors);
    }

    // ── Manifest-level ──
    if manifest.export_bundles.is_empty() {
        errors.push(LearningStatePortabilityValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: "manifest carries no export bundle".to_string(),
        });
    }
    if manifest.reset_plans.is_empty() {
        errors.push(LearningStatePortabilityValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: "manifest carries no reset plan".to_string(),
        });
    }

    let mut expected_overall = QualificationVerdict::QualifiedStable;
    for bundle in &manifest.export_bundles {
        expected_overall = expected_overall.meet(bundle.verdict);
    }
    for plan in &manifest.reset_plans {
        expected_overall = expected_overall.meet(plan.verdict);
    }
    if expected_overall != manifest.overall_verdict {
        errors.push(LearningStatePortabilityValidationError {
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

/// Pushes a typed validation error for every breach of a [`TutorialMutationFence`].
fn validate_mutation_fence(
    fence: &TutorialMutationFence,
    label: &str,
    err: &dyn Fn(String) -> LearningStatePortabilityValidationError,
    errors: &mut Vec<LearningStatePortabilityValidationError>,
) {
    if fence.introduces_tutorial_only_mutating_shortcut {
        errors.push(err(format!(
            "{label} introduces a tutorial-only mutating shortcut"
        )));
    }
    if fence.bypasses_preview_approval {
        errors.push(err(format!(
            "{label} bypasses the standard preview/approval model"
        )));
    }
    if fence.authority_boundary_change_allowed {
        errors.push(err(format!("{label} changes an authority boundary")));
    }
    if !fence.command_graph_unchanged {
        errors.push(err(format!("{label} changes the command graph")));
    }
    if fence.touches_real_workspace_state
        && !fence.uses_standard_preview_approval_when_touching_workspace
    {
        errors.push(err(format!(
            "{label} touches workspace state outside the standard preview/approval model"
        )));
    }
}

// ── Seed builders ─────────────────────────────────────────────────────────────

/// A redaction posture that redacts everything and widens nothing — the default.
fn full_redaction() -> RedactionPosture {
    RedactionPosture {
        redacts_raw_payloads: true,
        redacts_credentials: true,
        redacts_absolute_paths: true,
        widens_data_sharing: false,
    }
}

/// Specification for one seeded export bundle, expanded by [`build_bundle`].
struct BundleSpec {
    token: &'static str,
    display_label: &'static str,
    family: M5LearningSurfaceFamily,
    state_kind: LearnabilityStateKind,
    target_kind: ExportTargetKind,
    source_state_refs: &'static [&'static str],
    source_language: SourceLanguageEscape,
    cached_pack: CachedPackContinuity,
}

/// Expands a [`BundleSpec`] into a full [`LearningStateExportBundle`].
fn build_bundle(spec: BundleSpec) -> LearningStateExportBundle {
    let mut bundle = LearningStateExportBundle {
        record_kind: LEARNING_STATE_EXPORT_BUNDLE_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_STATE_PORTABILITY_SCHEMA_VERSION,
        bundle_id: format!("learning:m5:export:{}", spec.token),
        display_label: spec.display_label.to_string(),
        family: spec.family,
        state_kind: spec.state_kind,
        target_kind: spec.target_kind,
        source_state_refs: spec
            .source_state_refs
            .iter()
            .map(|r| r.to_string())
            .collect(),
        provenance_preserved: true,
        data_ownership: DataOwnershipClass::UserOwnedLocalFirst,
        redaction: full_redaction(),
        source_language: spec.source_language,
        cached_pack: spec.cached_pack,
        mutation_fence: TutorialMutationFence::clean(),
        safe_for_support_export: true,
        includes_progress_state: true,
        user_initiated: true,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    bundle.sync_verdict();
    bundle
}

/// Builds a reset plan over the given target classes, protecting everything else.
fn build_reset_plan(
    token: &str,
    display_label: &str,
    target_state_kinds: Vec<LearnabilityStateKind>,
) -> LearningStateResetPlan {
    let mut plan = LearningStateResetPlan {
        record_kind: LEARNING_STATE_RESET_PLAN_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_STATE_PORTABILITY_SCHEMA_VERSION,
        plan_id: format!("learning:m5:reset:{token}"),
        display_label: display_label.to_string(),
        target_state_kinds,
        protected_classes: vec![
            ProtectedStateClass::DocsPack,
            ProtectedStateClass::Bookmark,
            ProtectedStateClass::UserAuthoredNote,
            ProtectedStateClass::ModelPack,
            ProtectedStateClass::Checkpoint,
            ProtectedStateClass::TemplatePack,
        ],
        silently_deletes_outside_scope: false,
        restore_available: true,
        restore_window_disclosed: true,
        restore_command_ref: Some("cmd:learning.reset.restore".to_string()),
        data_ownership: DataOwnershipClass::UserOwnedLocalFirst,
        mutation_fence: TutorialMutationFence::clean(),
        safe_for_support_export: true,
        user_initiated: true,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    plan.sync_verdict();
    plan
}

/// Produces the canonical seeded learning-state portability manifest.
///
/// Three export bundles span three feature families and three flow kinds: a
/// notebook tour exported to a portable profile from a live pack (Stable), a
/// request-workspace exercise exported to a support bundle from a localized,
/// cached, mirrored pack — demonstrating both a source-language escape and
/// cached-pack continuity ([`QualificationVerdict::NarrowedBeta`]) — and a
/// docs/browser learning session exported to a portable profile from a
/// mirror-synced pack (Stable). Two reset plans clear bounded scopes — a full
/// local-learnability reset and a learning-session-only reset — each protecting
/// docs packs, bookmarks, user notes, model packs, checkpoints, and template
/// packs with a reversible restore. The cached/localized bundle narrows the
/// overall manifest verdict to `narrowed_beta`.
pub fn seeded_m5_learning_state_export_and_reset() -> M5LearningStatePortabilityManifest {
    let export_bundles = vec![
        build_bundle(BundleSpec {
            token: "notebook_tour_portable",
            display_label: "Export notebook tour progress",
            family: M5LearningSurfaceFamily::Notebook,
            state_kind: LearnabilityStateKind::TourState,
            target_kind: ExportTargetKind::PortableProfile,
            source_state_refs: &["learning:m5:progress:notebook_intro_tour"],
            source_language: SourceLanguageEscape::unlocalized("en-US"),
            cached_pack: CachedPackContinuity {
                source_class: SourceClass::ProjectDocs,
                freshness: FreshnessState::LiveAuthoritative,
                continuity_disclosed: false,
                offline_mirror_ref: None,
            },
        }),
        build_bundle(BundleSpec {
            token: "request_exercise_support_localized_cached",
            display_label: "Export request exercise progress (support bundle)",
            family: M5LearningSurfaceFamily::RequestWorkspace,
            state_kind: LearnabilityStateKind::ExerciseState,
            target_kind: ExportTargetKind::SupportBundle,
            source_state_refs: &["learning:m5:progress:request_workspace_first_call"],
            source_language: SourceLanguageEscape {
                presented_localized: true,
                source_locale: "en-US".to_string(),
                presented_locale: "fr-FR".to_string(),
                escape_to_source_available: true,
                escape_command_ref: Some("cmd:learning.export.view_source_language".to_string()),
                preserves_provenance: true,
            },
            cached_pack: CachedPackContinuity {
                source_class: SourceClass::MirroredOfficialDocs,
                freshness: FreshnessState::CachedDisclosed,
                continuity_disclosed: true,
                offline_mirror_ref: Some("mirror:m5:docs:fr-FR".to_string()),
            },
        }),
        build_bundle(BundleSpec {
            token: "docs_session_portable_mirrored",
            display_label: "Export docs learning session",
            family: M5LearningSurfaceFamily::DocsBrowser,
            state_kind: LearnabilityStateKind::LearningSessionState,
            target_kind: ExportTargetKind::PortableProfile,
            source_state_refs: &["learning:m5:progress:docs_browser_glossary"],
            source_language: SourceLanguageEscape::unlocalized("en-US"),
            cached_pack: CachedPackContinuity {
                source_class: SourceClass::MirroredOfficialDocs,
                freshness: FreshnessState::MirrorSyncedDisclosed,
                continuity_disclosed: true,
                offline_mirror_ref: Some("mirror:m5:docs:en-US".to_string()),
            },
        }),
    ];

    let reset_plans = vec![
        build_reset_plan(
            "all_local_learnability",
            "Reset all local learning progress",
            vec![
                LearnabilityStateKind::TourState,
                LearnabilityStateKind::ExerciseState,
                LearnabilityStateKind::LearningSessionState,
                LearnabilityStateKind::GlossaryState,
                LearnabilityStateKind::LearningModeProfile,
                LearnabilityStateKind::ContextualHintState,
            ],
        ),
        build_reset_plan(
            "learning_session_only",
            "Reset this learning session",
            vec![LearnabilityStateKind::LearningSessionState],
        ),
    ];

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "schema".to_string(),
        M5_LEARNING_STATE_PORTABILITY_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "doc".to_string(),
        M5_LEARNING_STATE_PORTABILITY_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "artifact".to_string(),
        M5_LEARNING_STATE_PORTABILITY_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "progress_schema".to_string(),
        M5_LEARNING_PROGRESS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "tour_and_glossary_schema".to_string(),
        M5_TOUR_AND_GLOSSARY_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "feature_family_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );

    let mut manifest = M5LearningStatePortabilityManifest {
        record_kind: M5_LEARNING_STATE_PORTABILITY_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_STATE_PORTABILITY_SCHEMA_VERSION,
        manifest_id: "learning:m5:state_portability_manifest:v1".to_string(),
        generated_at: "2026-06-19T00:00:00Z".to_string(),
        contract_refs,
        export_bundles,
        reset_plans,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: Vec::new(),
    };
    manifest.sync_verdicts();
    manifest
}
