//! Cited educational-AI panels, contextual why-now cards, and practice/sandbox
//! indicators for the learnability flows that run across Aureline's M5 feature
//! families.
//!
//! Where [`crate::progress_snapshots`] owns the *memory* of a learning flow and
//! [`crate::guided_exercise_rails`] / [`crate::tour_and_glossary_packages`] own
//! its *content*, this module owns the *teaching surfaces themselves*: the
//! educational-AI panels and contextual cards a person reads while they work, and
//! the practice/sandbox indicators that tell them whether a teaching space is a
//! safe sandbox or live repository state.
//!
//! Two record types carry that truth:
//!
//! - An [`EducationalPanel`] is an educational-AI answer panel or a contextual
//!   *why-now* card. When it claims repository truth it MUST cite the files,
//!   symbols, docs, examples, or commands it draws from, and it keeps the
//!   open-source / open-docs actions one step away — so an answer never sounds
//!   omniscient or action-capable. Its [`ExplainApplyClass`] keeps explain
//!   separate from do: any prepared mutation rides the standard
//!   preview/approval path, never a hidden shortcut.
//! - A [`PracticeIndicator`] declares a practice/sandbox surface's target scope,
//!   reset/discard behavior, persistence note, and whether it is local-only,
//!   simulated, or running against live repository state — so a low-risk teaching
//!   space is always visibly distinct from the live workspace.
//!
//! Both are *educational overlays*, and both carry an [`OverlayPresentation`]
//! that keeps quiet-hours, reduced-motion, accessibility, and client-scope limits
//! intact: an overlay never spams an attention surface and never creates a
//! pointer-only path.
//!
//! ## Invariants enforced
//!
//! - **Cited, not omniscient.** A panel that claims repository truth must cite at
//!   least one file/symbol/doc/example/command and keep an open-source/open-docs
//!   action one step away. A panel that presents as omniscient, or as able to act
//!   directly without approval, narrows below Stable.
//! - **Explain stays separate from do.** A panel whose explain/apply boundary is
//!   conflated, or that prepares a "do" outside the standard preview/approval
//!   model, narrows below Stable.
//! - **Sandboxes are distinct from live state.** A practice indicator must declare
//!   its target scope, reset behavior, and persistence note, and must be visibly
//!   distinct from the live workspace. A live-repo-state practice surface is an
//!   honest, disclosed choice that narrows to Beta; one that mutates live state
//!   outside the standard preview/approval model narrows to Preview.
//! - **Overlays respect attention and accessibility.** An overlay that ignores
//!   quiet-hours or reduced-motion, is not keyboard reachable, is not
//!   screen-reader labeled, is not client-scoped, or spams an attention surface
//!   narrows below Stable.
//! - **Offline and mirror parity is honest.** An overlay that dies on offline or
//!   mirrored profiles narrows to Preview; one that surfaces with a disclosed
//!   cached/mirror-stale freshness narrows to Beta.
//! - **Experts are never trapped.** A panel that traps an expert in a tutorial
//!   narrows below Stable.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_educational_ai_and_practice`] produces the canonical manifest.
//! Help/About, settings, diagnostics, support export, and docs/migration surfaces
//! ingest it rather than rephrasing educational-AI or practice posture by hand.
//!
//! - Schema: [`M5_EDUCATIONAL_AI_SCHEMA_REF`]
//! - Fixture: [`M5_EDUCATIONAL_AI_FIXTURE_REF`]
//! - Artifact: [`M5_EDUCATIONAL_AI_ARTIFACT_REF`]
//! - Doc: [`M5_EDUCATIONAL_AI_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_feature_family_learning_rails::{
    M5LearningSurfaceFamily, M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF,
};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    ExplainApplyClass, QualificationVerdict, GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};

#[cfg(test)]
mod tests;

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the educational-AI and practice records. Bumped
/// only on breaking payload changes; additive-optional fields do not bump it.
pub const M5_EDUCATIONAL_AI_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`EducationalPanel`].
pub const EDUCATIONAL_PANEL_RECORD_KIND: &str = "educational_panel";

/// Record kind for [`PracticeIndicator`].
pub const PRACTICE_INDICATOR_RECORD_KIND: &str = "practice_indicator";

/// Record kind for [`M5EducationalAiAndPracticeManifest`].
pub const M5_EDUCATIONAL_AI_MANIFEST_RECORD_KIND: &str = "m5_educational_ai_and_practice_manifest";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the educational-AI and practice schema.
pub const M5_EDUCATIONAL_AI_SCHEMA_REF: &str =
    "schemas/help/m5-educational-ai-and-practice.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_EDUCATIONAL_AI_FIXTURE_REF: &str =
    "fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json";

/// Repository-relative path to the proof artifact.
pub const M5_EDUCATIONAL_AI_ARTIFACT_REF: &str =
    "artifacts/ux/m5/educational-ai-proof/ship-cited-educational-ai-panels-and-practice-indicators.md";

/// Repository-relative path to the public doc.
pub const M5_EDUCATIONAL_AI_DOC_REF: &str = "docs/m5/educational-ai-and-practice.md";

// ── Educational surface kind ──────────────────────────────────────────────────

/// The kind of educational surface a panel record represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationalSurfaceKind {
    /// An educational-AI answer panel a person opens to ask about the workspace.
    EducationalAiPanel,
    /// An in-place contextual card that explains why a surface matters right now.
    WhyNowCard,
}

impl EducationalSurfaceKind {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EducationalAiPanel => "educational_ai_panel",
            Self::WhyNowCard => "why_now_card",
        }
    }
}

// ── Citation ──────────────────────────────────────────────────────────────────

/// The kind of repository truth a [`Citation`] points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationKind {
    /// A source file.
    File,
    /// A named symbol (function, type, identifier).
    Symbol,
    /// An in-product or repository doc.
    Doc,
    /// A runnable example or conformance dump.
    Example,
    /// A command id from the command graph.
    Command,
}

impl CitationKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Doc => "doc",
            Self::Example => "example",
            Self::Command => "command",
        }
    }
}

/// One citation backing an educational claim.
///
/// A citation carries an opaque, stable target ref and a short privacy-safe
/// label — never a raw URL, raw absolute path, or credential body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// What kind of repository truth this citation points at.
    pub citation_kind: CitationKind,
    /// Opaque, stable ref to the cited file/symbol/doc/example/command.
    pub target_ref: String,
    /// Short, privacy-safe label shown to the reader.
    pub label: String,
}

// ── Truth-source scope ────────────────────────────────────────────────────────

/// The scope label an educational surface attaches to the truth it claims.
///
/// This is the answer to "where does this claim come from?" — and it is what
/// keeps an educational answer from sounding omniscient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthSourceScope {
    /// The claim is grounded in the live repository state and must be cited.
    LiveRepoState,
    /// The claim illustrates a simulated example, not the live workspace.
    SimulatedExample,
    /// The claim is scoped to local-only, on-device state.
    LocalOnly,
}

impl TruthSourceScope {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveRepoState => "live_repo_state",
            Self::SimulatedExample => "simulated_example",
            Self::LocalOnly => "local_only",
        }
    }
}

// ── Open-resource action ──────────────────────────────────────────────────────

/// The kind of open-the-source action a panel keeps one step away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenResourceKind {
    /// Open the cited open-source file/symbol in the editor.
    OpenSource,
    /// Open the cited open docs entry.
    OpenDocs,
}

impl OpenResourceKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSource => "open_source",
            Self::OpenDocs => "open_docs",
        }
    }
}

/// A command-backed action that opens the cited source or docs.
///
/// The action MUST be one step away (`steps_away == 1`) and keyboard reachable,
/// so the open-source/open-docs path is never buried behind a menu maze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenResourceAction {
    /// Opaque stable id for the action.
    pub action_id: String,
    /// What the action opens.
    pub resource_kind: OpenResourceKind,
    /// Opaque ref to the command that backs this action.
    pub command_id_ref: String,
    /// Short, privacy-safe label.
    pub label: String,
    /// Number of steps to reach the action from the panel. MUST be 1.
    pub steps_away: u32,
    /// Opaque ref to the keyboard shortcut; MUST be present (keyboard reachable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut_ref: Option<String>,
}

impl OpenResourceAction {
    /// Returns true when the action satisfies the one-step, keyboard-reachable
    /// requirement.
    pub fn qualifies_stable(&self) -> bool {
        self.steps_away == 1 && self.keyboard_shortcut_ref.is_some()
    }
}

// ── Offline / mirror parity ───────────────────────────────────────────────────

/// How an educational overlay behaves on offline and mirrored profiles.
///
/// Local-present is the live-authoritative default. A disclosed cached or
/// mirror-stale state is honest and narrows to Beta; a surface that simply dies
/// on offline/mirror profiles is a dead link and narrows to Preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineParity {
    /// The overlay's content is present and live on every profile.
    LivePresent,
    /// The overlay surfaces from a disclosed local cache; may lag.
    CachedDisclosed,
    /// The overlay surfaces from a disclosed mirror; may be stale.
    MirrorStaleDisclosed,
    /// The overlay is unavailable on offline/mirror profiles (a dead link).
    MissingOnOffline,
}

impl OfflineParity {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LivePresent => "live_present",
            Self::CachedDisclosed => "cached_disclosed",
            Self::MirrorStaleDisclosed => "mirror_stale_disclosed",
            Self::MissingOnOffline => "missing_on_offline",
        }
    }

    /// Returns true when the overlay surfaces (live, cached, or mirrored) rather
    /// than dying on offline/mirror profiles.
    pub const fn surfaces_offline(self) -> bool {
        !matches!(self, Self::MissingOnOffline)
    }

    /// Returns true when the parity is a disclosed-but-degraded state that
    /// narrows to Beta.
    pub const fn is_disclosed_degraded(self) -> bool {
        matches!(self, Self::CachedDisclosed | Self::MirrorStaleDisclosed)
    }
}

// ── Overlay presentation ──────────────────────────────────────────────────────

/// The attention and accessibility posture shared by every educational overlay.
///
/// An educational overlay must respect quiet-hours and reduced-motion, stay
/// keyboard reachable and screen-reader labeled, stay scoped to the local client
/// rather than broadcast globally, and never spam an attention surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayPresentation {
    /// Whether the overlay respects quiet-hours. MUST be true.
    pub respects_quiet_hours: bool,
    /// Whether the overlay respects reduced-motion. MUST be true.
    pub respects_reduced_motion: bool,
    /// Whether the overlay is keyboard reachable. MUST be true — no pointer-only
    /// paths.
    pub keyboard_reachable: bool,
    /// Whether the overlay is screen-reader labeled. MUST be true.
    pub screen_reader_labeled: bool,
    /// Whether the overlay is scoped to the local client rather than pushed
    /// globally. MUST be true.
    pub client_scoped_not_global: bool,
    /// Whether the overlay may spam an attention surface (toasts, badges). MUST
    /// be false.
    pub spams_attention_surface: bool,
}

impl OverlayPresentation {
    /// Returns true when the presentation satisfies every Stable requirement.
    pub fn qualifies_stable(&self) -> bool {
        self.respects_quiet_hours
            && self.respects_reduced_motion
            && self.keyboard_reachable
            && self.screen_reader_labeled
            && self.client_scoped_not_global
            && !self.spams_attention_surface
    }

    /// The named narrowing reasons for each violated overlay invariant.
    pub fn narrowing_reasons(&self) -> Vec<String> {
        let mut reasons = Vec::new();
        if !self.respects_quiet_hours {
            reasons.push("overlay_ignores_quiet_hours".to_string());
        }
        if !self.respects_reduced_motion {
            reasons.push("overlay_ignores_reduced_motion".to_string());
        }
        if !self.keyboard_reachable {
            reasons.push("overlay_not_keyboard_reachable_pointer_only".to_string());
        }
        if !self.screen_reader_labeled {
            reasons.push("overlay_not_screen_reader_labeled".to_string());
        }
        if !self.client_scoped_not_global {
            reasons.push("overlay_not_client_scoped".to_string());
        }
        if self.spams_attention_surface {
            reasons.push("overlay_spams_attention_surface".to_string());
        }
        reasons
    }
}

// ── Educational panel ─────────────────────────────────────────────────────────

/// One cited educational-AI panel or contextual why-now card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EducationalPanel {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this panel.
    pub panel_id: String,
    /// Human-readable label shown in Help/About and support export.
    pub display_label: String,
    /// The M5 surface family this panel belongs to.
    pub family: M5LearningSurfaceFamily,
    /// Whether this is an educational-AI panel or a why-now card.
    pub surface_kind: EducationalSurfaceKind,
    /// Whether the panel claims repository truth (and therefore must cite it).
    pub claims_repository_truth: bool,
    /// The scope label the panel attaches to its claims.
    pub truth_source_scope: TruthSourceScope,
    /// Citations backing the panel's claims. MUST be non-empty when the panel
    /// claims live repository truth.
    #[serde(default)]
    pub citations: Vec<Citation>,
    /// Open-source / open-docs actions kept one step away. MUST be non-empty when
    /// the panel claims repository truth.
    #[serde(default)]
    pub open_resource_actions: Vec<OpenResourceAction>,
    /// The explain-versus-do boundary for this panel.
    pub explain_apply_class: ExplainApplyClass,
    /// Whether the panel presents as omniscient (claims complete knowledge). MUST
    /// be false.
    pub presents_as_omniscient: bool,
    /// Whether the panel claims it can act directly without going through the
    /// standard approval model. MUST be false.
    pub claims_direct_action_without_approval: bool,
    /// Whether any prepared mutation routes through the standard preview/approval
    /// model. MUST be true when the explain/apply class admits an Apply verb.
    pub mutation_routes_through_standard_preview_approval: bool,
    /// Whether the panel traps an expert in a tutorial they cannot dismiss. MUST
    /// be false.
    pub traps_expert_in_tutorial: bool,
    /// Attention and accessibility posture.
    pub overlay: OverlayPresentation,
    /// Offline / mirror parity posture.
    pub offline_parity: OfflineParity,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl EducationalPanel {
    /// The set of citation kinds this panel carries.
    pub fn citation_kinds(&self) -> BTreeSet<CitationKind> {
        self.citations.iter().map(|c| c.citation_kind).collect()
    }

    /// Returns true when the panel keeps at least one open-resource action of the
    /// given kind.
    pub fn has_open_resource(&self, kind: OpenResourceKind) -> bool {
        self.open_resource_actions
            .iter()
            .any(|a| a.resource_kind == kind)
    }

    /// Recomputes this panel's verdict and narrowing reasons, writing them back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_panel_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Practice / sandbox surface state ──────────────────────────────────────────

/// Whether a practice surface is local-only, simulated, or live repository state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeSurfaceState {
    /// A local-only practice surface; nothing touches the workspace or the repo.
    LocalOnly,
    /// A simulated surface; outputs are synthetic, not the live workspace.
    Simulated,
    /// A surface that runs against live repository state (higher risk; disclosed).
    LiveRepoState,
}

impl PracticeSurfaceState {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Simulated => "simulated",
            Self::LiveRepoState => "live_repo_state",
        }
    }

    /// Returns true when the surface runs against live repository state.
    pub const fn is_live_repo_state(self) -> bool {
        matches!(self, Self::LiveRepoState)
    }
}

/// How a practice surface resets or discards its work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResetBehavior {
    /// Work is discarded automatically when the practice surface is closed.
    DiscardOnExit,
    /// Work persists until the user runs an explicit reset action.
    ExplicitResetAction,
    /// Work persists until the user clears it (kept across sessions).
    PersistsUntilCleared,
}

impl ResetBehavior {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscardOnExit => "discard_on_exit",
            Self::ExplicitResetAction => "explicit_reset_action",
            Self::PersistsUntilCleared => "persists_until_cleared",
        }
    }
}

// ── Practice indicator ────────────────────────────────────────────────────────

/// One practice/sandbox indicator declaring a teaching space's scope, reset
/// behavior, persistence, and live-versus-sandbox state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeIndicator {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this indicator.
    pub indicator_id: String,
    /// Human-readable label shown in the practice surface and support export.
    pub display_label: String,
    /// The M5 surface family this practice space belongs to.
    pub family: M5LearningSurfaceFamily,
    /// Whether the surface is local-only, simulated, or live repository state.
    pub surface_state: PracticeSurfaceState,
    /// Short, privacy-safe label describing the target scope.
    pub target_scope_label: String,
    /// Opaque refs to the targets the practice surface may touch. MUST be
    /// non-empty — the target scope is always declared.
    #[serde(default)]
    pub target_scope_refs: Vec<String>,
    /// How the surface resets or discards work.
    pub reset_behavior: ResetBehavior,
    /// Short, privacy-safe note describing what persists. MUST be non-empty.
    pub persistence_note: String,
    /// Whether the surface is visibly distinct from the live workspace. MUST be
    /// true.
    pub distinct_from_live_workspace: bool,
    /// Whether the surface mutates live repository state.
    pub mutates_live_state: bool,
    /// Whether any live mutation routes through the standard preview/approval
    /// model. MUST be true when the surface mutates live state.
    pub mutation_routes_through_standard_preview_approval: bool,
    /// Whether the surface's work is reversible or discardable. MUST be true.
    pub reversible_or_discardable: bool,
    /// Attention and accessibility posture.
    pub overlay: OverlayPresentation,
    /// Offline / mirror parity posture.
    pub offline_parity: OfflineParity,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl PracticeIndicator {
    /// Recomputes this indicator's verdict and narrowing reasons, writing them
    /// back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_practice_indicator_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Verdict derivation ────────────────────────────────────────────────────────

/// Derives an educational panel's verdict and narrowing reasons from its
/// evidence.
///
/// Hard violations (a repository-truth claim with no citation or no
/// open-resource action, a panel that presents as omniscient or as able to act
/// directly without approval, a conflated explain/apply boundary, a prepared
/// "do" outside the standard preview/approval model, an open-resource action
/// that is not one step away or not keyboard reachable, an overlay that ignores
/// quiet-hours/reduced-motion/accessibility/client-scope or spams attention, a
/// panel that traps an expert, or an offline/mirror dead link) narrow to
/// [`QualificationVerdict::NarrowedPreview`]. A disclosed cached/mirror-stale
/// offline parity narrows to [`QualificationVerdict::NarrowedBeta`]. With no
/// findings the panel is [`QualificationVerdict::QualifiedStable`].
pub fn derive_panel_verdict(panel: &EducationalPanel) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    // ── Citations and open-resource actions for repository-truth claims ──
    if panel.claims_repository_truth {
        if panel.truth_source_scope == TruthSourceScope::LiveRepoState && panel.citations.is_empty()
        {
            reasons.push("repository_truth_claim_without_citation".to_string());
            verdict = verdict.meet(NarrowedPreview);
        }
        if panel.open_resource_actions.is_empty() {
            reasons.push("repository_truth_claim_without_open_resource_action".to_string());
            verdict = verdict.meet(NarrowedPreview);
        }
    }

    // ── Omniscient / action-capable masquerade ──
    if panel.presents_as_omniscient {
        reasons.push("panel_presents_as_omniscient".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if panel.claims_direct_action_without_approval {
        reasons.push("panel_claims_direct_action_without_approval".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Explain-versus-do separation ──
    if !panel.explain_apply_class.qualifies_stable() {
        reasons.push("explain_and_do_conflated".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if panel.explain_apply_class == ExplainApplyClass::ApplyRequiresApproval
        && !panel.mutation_routes_through_standard_preview_approval
    {
        reasons.push("educational_ai_do_outside_standard_preview_approval".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Open-resource actions one step away ──
    for action in &panel.open_resource_actions {
        if !action.qualifies_stable() {
            reasons.push(format!(
                "open_resource_action_{}_not_one_step_away_or_not_keyboard_reachable",
                action.resource_kind.as_str()
            ));
            verdict = verdict.meet(NarrowedPreview);
        }
    }

    // ── Expert trap ──
    if panel.traps_expert_in_tutorial {
        reasons.push("panel_traps_expert_in_tutorial".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Overlay attention / accessibility ──
    for reason in panel.overlay.narrowing_reasons() {
        reasons.push(reason);
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Offline / mirror parity ──
    if !panel.offline_parity.surfaces_offline() {
        reasons.push("offline_mirror_dead_link".to_string());
        verdict = verdict.meet(NarrowedPreview);
    } else if panel.offline_parity.is_disclosed_degraded() {
        reasons.push("offline_mirror_freshness_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

/// Derives a practice indicator's verdict and narrowing reasons from its
/// evidence.
///
/// Hard violations (an undeclared target scope, a missing persistence note, a
/// surface that is not distinct from the live workspace, a live mutation outside
/// the standard preview/approval model, work that is neither reversible nor
/// discardable, an overlay that ignores quiet-hours/reduced-motion/
/// accessibility/client-scope or spams attention, or an offline/mirror dead
/// link) narrow to [`QualificationVerdict::NarrowedPreview`]. A live-repo-state
/// practice surface (an honest, disclosed higher-risk choice) and a disclosed
/// cached/mirror-stale offline parity narrow to
/// [`QualificationVerdict::NarrowedBeta`]. With no findings the indicator is
/// [`QualificationVerdict::QualifiedStable`].
pub fn derive_practice_indicator_verdict(
    indicator: &PracticeIndicator,
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    // ── Declared scope, reset, and persistence ──
    if indicator.target_scope_refs.is_empty() {
        reasons.push("practice_target_scope_undeclared".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if indicator.persistence_note.trim().is_empty() {
        reasons.push("practice_persistence_note_missing".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Distinct from live workspace ──
    if !indicator.distinct_from_live_workspace {
        reasons.push("practice_surface_not_distinct_from_live_workspace".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Live mutations fenced behind preview/approval ──
    if indicator.mutates_live_state && !indicator.mutation_routes_through_standard_preview_approval
    {
        reasons.push("live_practice_mutation_outside_standard_preview_approval".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Reversible or discardable ──
    if !indicator.reversible_or_discardable {
        reasons.push("practice_changes_not_reversible_or_discardable".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Overlay attention / accessibility ──
    for reason in indicator.overlay.narrowing_reasons() {
        reasons.push(reason);
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Offline / mirror dead link ──
    if !indicator.offline_parity.surfaces_offline() {
        reasons.push("offline_mirror_dead_link".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // ── Disclosed, honest narrowing ──
    if indicator.surface_state.is_live_repo_state() {
        reasons.push("live_repo_state_practice_touches_real_workspace_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }
    if indicator.offline_parity.is_disclosed_degraded() {
        reasons.push("offline_mirror_freshness_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// The canonical manifest binding every educational panel and practice
/// indicator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EducationalAiAndPracticeManifest {
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
    /// Educational-AI panels and why-now cards.
    pub panels: Vec<EducationalPanel>,
    /// Practice / sandbox indicators.
    pub practice_indicators: Vec<PracticeIndicator>,
    /// Overall derived verdict — the strictest verdict across panels and
    /// indicators.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across records (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5EducationalAiAndPracticeManifest {
    /// Recomputes every panel and indicator verdict and the overall verdict from
    /// current evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();

        for panel in &mut self.panels {
            panel.sync_verdict();
            overall = overall.meet(panel.verdict);
            reasons.extend(panel.narrowing_reasons.iter().cloned());
        }
        for indicator in &mut self.practice_indicators {
            indicator.sync_verdict();
            overall = overall.meet(indicator.verdict);
            reasons.extend(indicator.narrowing_reasons.iter().cloned());
        }

        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the panel with `panel_id`, if present.
    pub fn panel(&self, panel_id: &str) -> Option<&EducationalPanel> {
        self.panels.iter().find(|p| p.panel_id == panel_id)
    }

    /// Returns the practice indicator with `indicator_id`, if present.
    pub fn practice_indicator(&self, indicator_id: &str) -> Option<&PracticeIndicator> {
        self.practice_indicators
            .iter()
            .find(|i| i.indicator_id == indicator_id)
    }

    /// The set of every M5 family represented by a panel or practice indicator.
    pub fn families_covered(&self) -> BTreeSet<M5LearningSurfaceFamily> {
        self.panels
            .iter()
            .map(|p| p.family)
            .chain(self.practice_indicators.iter().map(|i| i.family))
            .collect()
    }
}

/// Reopens an educational-AI/practice manifest from its exported JSON form.
///
/// This is the round-trip used to prove citations, scope labels, and practice
/// indicators survive export and reopen without losing identity: the reopened
/// manifest is structurally equal to the original.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_educational_ai_manifest_from_json(
    json: &str,
) -> Result<M5EducationalAiAndPracticeManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Validation ────────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_educational_ai_and_practice`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EducationalAiValidationError {
    /// Opaque id of the panel, indicator, or manifest that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for EducationalAiValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a manifest against the educational-AI and practice invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any record's stored verdict diverges from the
/// verdict derived from its evidence; when a panel claims repository truth
/// without a citation or an open-resource action, presents as omniscient or as
/// able to act directly without approval, conflates explain and do, prepares a
/// "do" outside the standard preview/approval model, keeps an open-resource
/// action that is not one step away or not keyboard reachable, traps an expert,
/// carries an overlay that ignores quiet-hours/reduced-motion/accessibility/
/// client-scope or spams attention, or is an offline/mirror dead link; when a
/// practice indicator omits its target scope or persistence note, is not
/// distinct from the live workspace, mutates live state outside the standard
/// preview/approval model, is neither reversible nor discardable, or carries a
/// failing overlay or offline parity; when two panels or two indicators share an
/// id; when the manifest has no panel or no practice indicator; or when the
/// manifest's overall verdict does not fold its members.
pub fn validate_m5_educational_ai_and_practice(
    manifest: &M5EducationalAiAndPracticeManifest,
) -> Result<(), Vec<EducationalAiValidationError>> {
    let mut errors: Vec<EducationalAiValidationError> = Vec::new();

    if manifest.panels.is_empty() {
        errors.push(EducationalAiValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: "manifest has no educational panel".to_string(),
        });
    }
    if manifest.practice_indicators.is_empty() {
        errors.push(EducationalAiValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: "manifest has no practice indicator".to_string(),
        });
    }

    // ── Panels ──
    let mut seen_panel_ids: BTreeSet<&str> = BTreeSet::new();
    for panel in &manifest.panels {
        let subject = panel.panel_id.clone();
        let err = |message: String| EducationalAiValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_panel_ids.insert(panel.panel_id.as_str()) {
            errors.push(err(format!("duplicate panel id {}", panel.panel_id)));
        }

        let (derived, derived_reasons) = derive_panel_verdict(panel);
        if derived != panel.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                panel.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != panel.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if panel.claims_repository_truth {
            if panel.truth_source_scope == TruthSourceScope::LiveRepoState
                && panel.citations.is_empty()
            {
                errors.push(err(
                    "panel claims live repository truth without a citation".to_string()
                ));
            }
            if panel.open_resource_actions.is_empty() {
                errors.push(err(
                    "panel claims repository truth without keeping an open-source/open-docs action one step away"
                        .to_string(),
                ));
            }
        }
        if panel.presents_as_omniscient {
            errors.push(err("panel presents as omniscient".to_string()));
        }
        if panel.claims_direct_action_without_approval {
            errors.push(err(
                "panel claims it can act directly without approval".to_string()
            ));
        }
        if !panel.explain_apply_class.qualifies_stable() {
            errors.push(err("panel conflates explain and do".to_string()));
        }
        if panel.explain_apply_class == ExplainApplyClass::ApplyRequiresApproval
            && !panel.mutation_routes_through_standard_preview_approval
        {
            errors.push(err(
                "panel prepares a do outside the standard preview/approval model".to_string(),
            ));
        }
        for action in &panel.open_resource_actions {
            if action.steps_away != 1 {
                errors.push(err(format!(
                    "open-resource action {} is not one step away",
                    action.resource_kind.as_str()
                )));
            }
            if action.keyboard_shortcut_ref.is_none() {
                errors.push(err(format!(
                    "open-resource action {} is not keyboard reachable",
                    action.resource_kind.as_str()
                )));
            }
        }
        if panel.traps_expert_in_tutorial {
            errors.push(err("panel traps an expert in a tutorial".to_string()));
        }
        if !panel.overlay.qualifies_stable() {
            errors.push(err(
                "panel overlay ignores quiet-hours/reduced-motion/accessibility/client-scope or spams attention"
                    .to_string(),
            ));
        }
        if !panel.offline_parity.surfaces_offline() {
            errors.push(err("panel is an offline/mirror dead link".to_string()));
        }
    }

    // ── Practice indicators ──
    let mut seen_indicator_ids: BTreeSet<&str> = BTreeSet::new();
    for indicator in &manifest.practice_indicators {
        let subject = indicator.indicator_id.clone();
        let err = |message: String| EducationalAiValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_indicator_ids.insert(indicator.indicator_id.as_str()) {
            errors.push(err(format!(
                "duplicate indicator id {}",
                indicator.indicator_id
            )));
        }

        let (derived, derived_reasons) = derive_practice_indicator_verdict(indicator);
        if derived != indicator.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                indicator.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != indicator.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if indicator.target_scope_refs.is_empty() {
            errors.push(err(
                "practice indicator does not declare a target scope".to_string()
            ));
        }
        if indicator.persistence_note.trim().is_empty() {
            errors.push(err("practice indicator has no persistence note".to_string()));
        }
        if !indicator.distinct_from_live_workspace {
            errors.push(err(
                "practice surface is not distinct from the live workspace".to_string(),
            ));
        }
        if indicator.mutates_live_state
            && !indicator.mutation_routes_through_standard_preview_approval
        {
            errors.push(err(
                "practice surface mutates live state outside the standard preview/approval model"
                    .to_string(),
            ));
        }
        if !indicator.reversible_or_discardable {
            errors.push(err(
                "practice surface work is neither reversible nor discardable".to_string(),
            ));
        }
        if !indicator.overlay.qualifies_stable() {
            errors.push(err(
                "practice overlay ignores quiet-hours/reduced-motion/accessibility/client-scope or spams attention"
                    .to_string(),
            ));
        }
        if !indicator.offline_parity.surfaces_offline() {
            errors.push(err(
                "practice indicator is an offline/mirror dead link".to_string()
            ));
        }
    }

    // ── Manifest-level: overall verdict must fold the members ──
    let mut expected_overall = QualificationVerdict::QualifiedStable;
    for panel in &manifest.panels {
        expected_overall = expected_overall.meet(panel.verdict);
    }
    for indicator in &manifest.practice_indicators {
        expected_overall = expected_overall.meet(indicator.verdict);
    }
    if expected_overall != manifest.overall_verdict {
        errors.push(EducationalAiValidationError {
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

// ── Seed builders ─────────────────────────────────────────────────────────────

/// A fully-respectful overlay posture — the default for every seeded record.
fn respectful_overlay() -> OverlayPresentation {
    OverlayPresentation {
        respects_quiet_hours: true,
        respects_reduced_motion: true,
        keyboard_reachable: true,
        screen_reader_labeled: true,
        client_scoped_not_global: true,
        spams_attention_surface: false,
    }
}

/// Builds an open-resource action one step away and keyboard reachable.
fn open_action(token: &str, kind: OpenResourceKind, label: &str) -> OpenResourceAction {
    OpenResourceAction {
        action_id: format!("learning:m5:edu:{token}:open:{}", kind.as_str()),
        resource_kind: kind,
        command_id_ref: format!("cmd:learning.educational.{}", kind.as_str()),
        label: label.to_string(),
        steps_away: 1,
        keyboard_shortcut_ref: Some(format!("kb:learning.educational.{}", kind.as_str())),
    }
}

/// Specification for one seeded educational panel, expanded by [`build_panel`].
struct PanelSpec {
    token: &'static str,
    display_label: &'static str,
    family: M5LearningSurfaceFamily,
    surface_kind: EducationalSurfaceKind,
    claims_repository_truth: bool,
    truth_source_scope: TruthSourceScope,
    citations: Vec<(CitationKind, &'static str)>,
    explain_apply_class: ExplainApplyClass,
    offline_parity: OfflineParity,
}

/// Expands a [`PanelSpec`] into a full [`EducationalPanel`].
fn build_panel(spec: PanelSpec) -> EducationalPanel {
    let citations: Vec<Citation> = spec
        .citations
        .iter()
        .map(|(kind, label)| Citation {
            citation_kind: *kind,
            target_ref: format!("ref:{}:{}:{}", spec.token, kind.as_str(), label),
            label: (*label).to_string(),
        })
        .collect();

    // A panel that touches repository truth keeps both open-source and open-docs
    // one step away.
    let open_resource_actions = vec![
        open_action(spec.token, OpenResourceKind::OpenSource, "Open source"),
        open_action(spec.token, OpenResourceKind::OpenDocs, "Open docs"),
    ];

    let mutation_routes_through_standard_preview_approval =
        spec.explain_apply_class != ExplainApplyClass::Conflated;

    let mut panel = EducationalPanel {
        record_kind: EDUCATIONAL_PANEL_RECORD_KIND.to_string(),
        schema_version: M5_EDUCATIONAL_AI_SCHEMA_VERSION,
        panel_id: format!("learning:m5:edu:panel:{}", spec.token),
        display_label: spec.display_label.to_string(),
        family: spec.family,
        surface_kind: spec.surface_kind,
        claims_repository_truth: spec.claims_repository_truth,
        truth_source_scope: spec.truth_source_scope,
        citations,
        open_resource_actions,
        explain_apply_class: spec.explain_apply_class,
        presents_as_omniscient: false,
        claims_direct_action_without_approval: false,
        mutation_routes_through_standard_preview_approval,
        traps_expert_in_tutorial: false,
        overlay: respectful_overlay(),
        offline_parity: spec.offline_parity,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    panel.sync_verdict();
    panel
}

/// Specification for one seeded practice indicator, expanded by
/// [`build_indicator`].
struct IndicatorSpec {
    token: &'static str,
    display_label: &'static str,
    family: M5LearningSurfaceFamily,
    surface_state: PracticeSurfaceState,
    target_scope_label: &'static str,
    reset_behavior: ResetBehavior,
    persistence_note: &'static str,
}

/// Expands an [`IndicatorSpec`] into a full [`PracticeIndicator`].
fn build_indicator(spec: IndicatorSpec) -> PracticeIndicator {
    let mutates_live_state = spec.surface_state.is_live_repo_state();
    let mut indicator = PracticeIndicator {
        record_kind: PRACTICE_INDICATOR_RECORD_KIND.to_string(),
        schema_version: M5_EDUCATIONAL_AI_SCHEMA_VERSION,
        indicator_id: format!("learning:m5:practice:{}", spec.token),
        display_label: spec.display_label.to_string(),
        family: spec.family,
        surface_state: spec.surface_state,
        target_scope_label: spec.target_scope_label.to_string(),
        target_scope_refs: vec![format!("scope:{}:target", spec.token)],
        reset_behavior: spec.reset_behavior,
        persistence_note: spec.persistence_note.to_string(),
        distinct_from_live_workspace: true,
        mutates_live_state,
        // Live mutations always ride the standard preview/approval fence.
        mutation_routes_through_standard_preview_approval: true,
        reversible_or_discardable: true,
        overlay: respectful_overlay(),
        offline_parity: OfflineParity::LivePresent,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    indicator.sync_verdict();
    indicator
}

/// Produces the canonical seeded educational-AI and practice manifest.
///
/// Four panels span four feature families: a notebook educational-AI panel that
/// cites a file, symbol, and command from live repo state (Stable), a
/// request-workspace why-now card citing docs and an example (Stable), a
/// database-workspace why-now card that surfaces from a disclosed local cache
/// ([`QualificationVerdict::NarrowedBeta`]), and a docs/browser panel that
/// teaches from a simulated example with an approval-gated apply (Stable). Three
/// practice indicators cover three families: a simulated notebook sandbox
/// (Stable), a local-only request-workspace scratch space (Stable), and a
/// live-repo-state database practice surface that is honestly disclosed
/// ([`QualificationVerdict::NarrowedBeta`]). The narrowest member propagates, so
/// the overall manifest verdict is `narrowed_beta`.
pub fn seeded_m5_educational_ai_and_practice() -> M5EducationalAiAndPracticeManifest {
    let panels = vec![
        build_panel(PanelSpec {
            token: "notebook_explain",
            display_label: "Notebook: explain this cell",
            family: M5LearningSurfaceFamily::Notebook,
            surface_kind: EducationalSurfaceKind::EducationalAiPanel,
            claims_repository_truth: true,
            truth_source_scope: TruthSourceScope::LiveRepoState,
            citations: vec![
                (CitationKind::File, "notebook_cell"),
                (CitationKind::Symbol, "run_cell"),
                (CitationKind::Command, "notebook.run"),
            ],
            explain_apply_class: ExplainApplyClass::FullySeparated,
            offline_parity: OfflineParity::LivePresent,
        }),
        build_panel(PanelSpec {
            token: "request_workspace_why_now",
            display_label: "Request workspace: why this matters now",
            family: M5LearningSurfaceFamily::RequestWorkspace,
            surface_kind: EducationalSurfaceKind::WhyNowCard,
            claims_repository_truth: true,
            truth_source_scope: TruthSourceScope::LiveRepoState,
            citations: vec![
                (CitationKind::Doc, "request_workspace_overview"),
                (CitationKind::Example, "first_call_dump"),
            ],
            explain_apply_class: ExplainApplyClass::ReadOnly,
            offline_parity: OfflineParity::LivePresent,
        }),
        build_panel(PanelSpec {
            token: "database_why_now_cached",
            display_label: "Database workspace: statement-safety context",
            family: M5LearningSurfaceFamily::DatabaseWorkspace,
            surface_kind: EducationalSurfaceKind::WhyNowCard,
            claims_repository_truth: true,
            truth_source_scope: TruthSourceScope::LiveRepoState,
            citations: vec![(CitationKind::Doc, "statement_safety")],
            explain_apply_class: ExplainApplyClass::ReadOnly,
            offline_parity: OfflineParity::CachedDisclosed,
        }),
        build_panel(PanelSpec {
            token: "docs_browser_simulated",
            display_label: "Docs browser: trust-model walkthrough",
            family: M5LearningSurfaceFamily::DocsBrowser,
            surface_kind: EducationalSurfaceKind::EducationalAiPanel,
            claims_repository_truth: false,
            truth_source_scope: TruthSourceScope::SimulatedExample,
            citations: vec![(CitationKind::Example, "trust_model_demo")],
            explain_apply_class: ExplainApplyClass::ApplyRequiresApproval,
            offline_parity: OfflineParity::LivePresent,
        }),
    ];

    let practice_indicators = vec![
        build_indicator(IndicatorSpec {
            token: "notebook_sandbox",
            display_label: "Notebook practice sandbox",
            family: M5LearningSurfaceFamily::Notebook,
            surface_state: PracticeSurfaceState::Simulated,
            target_scope_label: "A throwaway practice notebook",
            reset_behavior: ResetBehavior::DiscardOnExit,
            persistence_note: "Discarded when you close the practice notebook.",
        }),
        build_indicator(IndicatorSpec {
            token: "request_workspace_scratch",
            display_label: "Request workspace scratch space",
            family: M5LearningSurfaceFamily::RequestWorkspace,
            surface_state: PracticeSurfaceState::LocalOnly,
            target_scope_label: "A local-only scratch request",
            reset_behavior: ResetBehavior::ExplicitResetAction,
            persistence_note: "Kept locally on this device until you reset it.",
        }),
        build_indicator(IndicatorSpec {
            token: "database_live_practice",
            display_label: "Database workspace live practice",
            family: M5LearningSurfaceFamily::DatabaseWorkspace,
            surface_state: PracticeSurfaceState::LiveRepoState,
            target_scope_label: "Your connected database (live)",
            reset_behavior: ResetBehavior::ExplicitResetAction,
            persistence_note:
                "Runs against your live database; any change rides the standard preview/approval path.",
        }),
    ];

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "schema".to_string(),
        M5_EDUCATIONAL_AI_SCHEMA_REF.to_string(),
    );
    contract_refs.insert("doc".to_string(), M5_EDUCATIONAL_AI_DOC_REF.to_string());
    contract_refs.insert(
        "artifact".to_string(),
        M5_EDUCATIONAL_AI_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "feature_family_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );

    let mut manifest = M5EducationalAiAndPracticeManifest {
        record_kind: M5_EDUCATIONAL_AI_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_EDUCATIONAL_AI_SCHEMA_VERSION,
        manifest_id: "learning:m5:educational_ai_and_practice_manifest:v1".to_string(),
        generated_at: "2026-06-19T00:00:00Z".to_string(),
        contract_refs,
        panels,
        practice_indicators,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: Vec::new(),
    };
    manifest.sync_verdicts();
    manifest
}
