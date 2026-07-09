//! Keyboard / screen-reader / localization / export parity and honest automatic narrowing for
//! the M5 learning-mode-toggle / tip-card / guided-exercise-step / glossary-chip-or-card /
//! safe-explanation-banner / progress-marker learning components.
//!
//! This module is the M05-1010 accessibility-and-auto-narrowing capstone over the frozen M5
//! learning-component matrix
//! ([`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`]).
//! Where the freeze matrix defines the reusable learning-mode toggle, tip card, guided exercise
//! step, glossary chip / card, safe explanation banner, and progress marker primitives, and the
//! 1005-1009 implementation / consumer lanes resolve their per-surface truth, this lane
//! certifies — per component family — that learning claims stay **keyboard-complete,
//! assistive-tech-reachable, localization/export-safe, and self-narrowing** rather than
//! presenting a paused mode, a snoozed tip, a stale exercise pack, an uncited glossary entry, an
//! unprovable explain-versus-do boundary, or blocked progress portability as still fully exact,
//! live, cited, portable learning:
//!
//! - **Keyboard / screen-reader / localization reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and localized (source-language-fallback-preserving) path into the
//!   same command binding, learning-mode state, exercise step / success-criteria, glossary
//!   citation, explain-versus-do boundary, and progress ownership the rich component shows —
//!   never a hover-only chip that strands assistive-tech or non-primary-locale users.
//!   Hierarchy-heavy families (the guided-exercise step's nested lesson / step / sub-step / hint /
//!   success-criteria lineage) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same stable
//!   command IDs, learning-mode states, exercise success-criteria, cited glossary sources,
//!   explain-versus-do boundaries, progress ownership, and narrowing reasons shown in-product so
//!   support, docs, and release proof can reconstruct exactly what the user was actually taught
//!   without screenshots or tribal knowledge.
//! - **Honest auto-narrowing.** When a learning mode is paused, a tip is snoozed, an exercise
//!   pack's freshness drifted, a glossary citation is stale, an explain-versus-do boundary cannot
//!   be proven, or progress portability is blocked, the component's learning claim auto-narrows
//!   from `ExactLearning` / `ReviewableGuidance` to a paused-mode / snoozed-tip / stale-pack /
//!   uncited-glossary / unprovable-boundary / blocked-progress projection, discloses the narrowing
//!   with a precise trigger and binding dimension, and preserves the canonical command-binding /
//!   citation / progress-ownership lineage — the underlying learning lineage is never dropped
//!   opaquely. A component with every dimension intact must NOT carry a spurious narrowing, and a
//!   stale-citation / drifted-pack / unprovable-boundary / blocked-progress state can never keep
//!   an exact learning claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the onboarding UI, tour
//!   overlay, learning panel, glossary UI, exercise UI, help panel, CLI help, product UI, and
//!   support / release exports so product, docs, and release publication stay aligned on learning
//!   downgrade behavior rather than drifting in copy — an exact-looking surface can never outrun
//!   the citation / freshness / portability proof it is being viewed away from.
//!
//! Each [`LearningComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::M5LearningComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5LearningRequiredLabel`] and
//! [`M5LearningDowngradeTrigger`] and the shared [`M5LearningConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, request bodies, and endpoint secrets
//! never cross this boundary; the packet carries only typed class tokens, opaque learning refs,
//! booleans, and controlled labels so support, release, and diagnostics exports can reconstruct
//! exactly what an accessible fallback would have shown without leaking sensitive material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5LearningComponentFamily, M5LearningConsumerSurface, M5LearningDowngradeTrigger,
    M5LearningRequiredLabel, M5_LEARNING_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1010 learning-component accessibility fallback packet.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`LearningComponentAccessibilityPacket`].
pub const LEARNING_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_learning_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`LearningComponentAccessibilityRow`].
pub const LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_learning_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/help/m5_learning_component_accessibility_fallback.md";

/// Repo-relative path of the frozen learning-component matrix this lane certifies.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_LEARNING_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-learning-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const LEARNING_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-learning-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const LEARNING_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-learning-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the guided-exercise step's
/// nested lesson / step / sub-step / hint / success-criteria lineage) and therefore MUST bind
/// their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5LearningComponentFamily) -> bool {
    matches!(family, M5LearningComponentFamily::GuidedExerciseStep)
}

/// The learning dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5LearningComponentFamily,
) -> M5LearningComponentClaimDimension {
    match family {
        M5LearningComponentFamily::LearningModeToggle => {
            M5LearningComponentClaimDimension::LearningModeDelivery
        }
        M5LearningComponentFamily::TipCard => M5LearningComponentClaimDimension::TipDelivery,
        M5LearningComponentFamily::GuidedExerciseStep => {
            M5LearningComponentClaimDimension::ExercisePackFreshness
        }
        M5LearningComponentFamily::GlossaryChipOrCard => {
            M5LearningComponentClaimDimension::CitationFreshness
        }
        M5LearningComponentFamily::SafeExplanationBanner => {
            M5LearningComponentClaimDimension::ExplainDoBoundary
        }
        M5LearningComponentFamily::ProgressMarker => {
            M5LearningComponentClaimDimension::ProgressPortability
        }
    }
}

/// A rendered fallback modality for a learning component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentFallbackModality {
    /// A rich, structured (nested lesson / step / sub-step tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A localized / source-language-fallback text projection.
    Localized,
}

impl M5LearningComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / localized path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Localized)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Localized => "localized",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentRenderingSurface {
    /// The full-capability desktop learning surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5LearningComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / localization reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / non-primary-locale
    /// users (red).
    ViewOnlyTrap,
}

impl LearningComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / localized users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl LearningComponentExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl LearningComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The learning claim ceiling a component asserts: how strong a learning posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a learning dimension weakens so a
/// paused mode, a snoozed tip, a stale exercise pack, an uncited glossary entry, an unprovable
/// explain-versus-do boundary, or blocked progress portability can never keep an old
/// `ExactLearning` or `ReviewableGuidance` label — a stale / uncited / unprovable / blocked state
/// never masquerades as exact learning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentClaim {
    /// Exact learning: a live, command-backed, cited-current, portable-progress learning
    /// component — the strongest claim, a surface Aureline can teach against as exactly true right
    /// now.
    ExactLearning,
    /// Reviewable guidance: a self-sufficient, reviewable read-only explanation (guidance a user
    /// can read) that is not itself a certified exact-learning path.
    ReviewableGuidance,
    /// Paused-mode projection: the learning mode is paused / suppressed and only its stable
    /// command binding and scope remain available; the surface must not present it as an active
    /// live learning mode.
    PausedModeProjection,
    /// Snoozed-tip projection: the tip card is snoozed / suppressed and only its stable command
    /// binding remains available; the surface must not present it as an active live tip.
    SnoozedTipProjection,
    /// Stale-pack projection: the guided-exercise pack's freshness has drifted; the surface stays
    /// a stale-pack explanation, never an exact-current exercise.
    StalePackProjection,
    /// Uncited-glossary projection: the glossary entry's citation is stale / severed; the surface
    /// stays an uncited-glossary explanation with its last-known citation preserved, never a
    /// cited-current definition.
    UncitedGlossaryProjection,
    /// Unprovable-boundary projection: the safe-explanation banner's explain-versus-do boundary
    /// cannot be proven; the surface stays an explain-only projection, never an apply-capable
    /// action.
    UnprovableBoundaryProjection,
    /// Blocked-progress projection: progress portability (resume / export) is blocked; the surface
    /// stays a local-only progress explanation, never a portable-progress claim.
    BlockedProgressProjection,
}

impl M5LearningComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::ExactLearning,
        Self::ReviewableGuidance,
        Self::PausedModeProjection,
        Self::SnoozedTipProjection,
        Self::StalePackProjection,
        Self::UncitedGlossaryProjection,
        Self::UnprovableBoundaryProjection,
        Self::BlockedProgressProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger learning posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExactLearning => 7,
            Self::ReviewableGuidance => 6,
            Self::PausedModeProjection => 5,
            Self::SnoozedTipProjection => 4,
            Self::StalePackProjection => 3,
            Self::UncitedGlossaryProjection => 2,
            Self::UnprovableBoundaryProjection => 1,
            Self::BlockedProgressProjection => 0,
        }
    }

    /// Returns true when this claim asserts fully exact, live learning parity.
    pub const fn asserts_exact_learning(self) -> bool {
        matches!(self, Self::ExactLearning)
    }

    /// Returns true when this claim asserts a fully self-sufficient (exact or reviewable) learning
    /// projection.
    pub const fn asserts_trustworthy_learning(self) -> bool {
        matches!(self, Self::ExactLearning | Self::ReviewableGuidance)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLearning => "exact_learning",
            Self::ReviewableGuidance => "reviewable_guidance",
            Self::PausedModeProjection => "paused_mode_projection",
            Self::SnoozedTipProjection => "snoozed_tip_projection",
            Self::StalePackProjection => "stale_pack_projection",
            Self::UncitedGlossaryProjection => "uncited_glossary_projection",
            Self::UnprovableBoundaryProjection => "unprovable_boundary_projection",
            Self::BlockedProgressProjection => "blocked_progress_projection",
        }
    }
}

/// The learning dimension whose state governs how far a component may claim to be an exact, live
/// learning surface. The dimensions map 1:1 to the six frozen component families so every family
/// carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentClaimDimension {
    /// Learning-mode delivery: is the learning mode active and command-backed, or is it paused /
    /// suppressed?
    LearningModeDelivery,
    /// Tip delivery: is the tip card active and command-backed, or is it snoozed / suppressed?
    TipDelivery,
    /// Exercise pack freshness: is the guided-exercise pack current, or has its freshness drifted?
    ExercisePackFreshness,
    /// Citation freshness: is the glossary entry cited-current, or is its citation stale / severed?
    CitationFreshness,
    /// Explain-versus-do boundary: can the safe-explanation banner prove its explain-versus-do
    /// boundary, or is the boundary unprovable?
    ExplainDoBoundary,
    /// Progress portability: is the progress marker's resume / export portable, or is progress
    /// portability blocked (local-only)?
    ProgressPortability,
}

impl M5LearningComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LearningModeDelivery,
        Self::TipDelivery,
        Self::ExercisePackFreshness,
        Self::CitationFreshness,
        Self::ExplainDoBoundary,
        Self::ProgressPortability,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LearningModeDelivery => "learning_mode_delivery",
            Self::TipDelivery => "tip_delivery",
            Self::ExercisePackFreshness => "exercise_pack_freshness",
            Self::CitationFreshness => "citation_freshness",
            Self::ExplainDoBoundary => "explain_do_boundary",
            Self::ProgressPortability => "progress_portability",
        }
    }
}

/// The observed condition of one learning dimension. Anything weaker than
/// [`Self::LiveExactLearning`] imposes a narrowing ceiling on the component's learning claim. The
/// four spec axes the lane must auto-narrow on — a stale citation, a drifted exercise pack, an
/// unprovable explain-versus-do boundary, and blocked progress portability — are
/// [`Self::CitationStale`], [`Self::ExercisePackStale`], [`Self::ExplainDoUnprovable`], and
/// [`Self::ProgressPortabilityBlocked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentConditionState {
    /// Live, command-backed, cited-current, portable-progress — imposes no ceiling.
    LiveExactLearning,
    /// The learning mode is paused / suppressed — learning claim drops to a paused-mode
    /// projection.
    LearningModePaused,
    /// The tip card is snoozed / suppressed — learning claim drops to a snoozed-tip projection.
    TipSnoozed,
    /// The guided-exercise pack's freshness has drifted — learning claim drops to a stale-pack
    /// projection.
    ExercisePackStale,
    /// The glossary citation is stale / severed — learning claim drops to an uncited-glossary
    /// projection.
    CitationStale,
    /// The explain-versus-do boundary cannot be proven — learning claim drops to an
    /// unprovable-boundary projection.
    ExplainDoUnprovable,
    /// Progress portability (resume / export) is blocked — learning claim drops to a
    /// blocked-progress projection.
    ProgressPortabilityBlocked,
}

impl M5LearningComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveExactLearning,
        Self::LearningModePaused,
        Self::TipSnoozed,
        Self::ExercisePackStale,
        Self::CitationStale,
        Self::ExplainDoUnprovable,
        Self::ProgressPortabilityBlocked,
    ];

    /// Returns true when the dimension is weaker than exact and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveExactLearning)
    }

    /// Returns true when the condition reflects a state that cannot be proven current / cited /
    /// portable and must never be shown as exact learning. A paused mode and a snoozed tip are
    /// delivery states, not exactness overstatements, so they are deliberately excluded here.
    pub const fn cannot_be_proven_exact(self) -> bool {
        matches!(
            self,
            Self::ExercisePackStale
                | Self::CitationStale
                | Self::ExplainDoUnprovable
                | Self::ProgressPortabilityBlocked
        )
    }

    /// The strongest learning claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5LearningComponentClaim {
        match self {
            Self::LiveExactLearning => M5LearningComponentClaim::ExactLearning,
            Self::LearningModePaused => M5LearningComponentClaim::PausedModeProjection,
            Self::TipSnoozed => M5LearningComponentClaim::SnoozedTipProjection,
            Self::ExercisePackStale => M5LearningComponentClaim::StalePackProjection,
            Self::CitationStale => M5LearningComponentClaim::UncitedGlossaryProjection,
            Self::ExplainDoUnprovable => M5LearningComponentClaim::UnprovableBoundaryProjection,
            Self::ProgressPortabilityBlocked => M5LearningComponentClaim::BlockedProgressProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5LearningDowngradeTrigger {
        match self {
            // The exact baseline never narrows; kept for exhaustiveness.
            Self::LiveExactLearning => M5LearningDowngradeTrigger::ProofStale,
            Self::LearningModePaused => M5LearningDowngradeTrigger::LearningModeStateUnstated,
            Self::TipSnoozed => M5LearningDowngradeTrigger::TipCommandBindingUnstated,
            Self::ExercisePackStale => M5LearningDowngradeTrigger::ExerciseStepStateUnstated,
            Self::CitationStale => M5LearningDowngradeTrigger::GlossaryCitationSevered,
            Self::ExplainDoUnprovable => {
                M5LearningDowngradeTrigger::ExplanationApplyBoundaryUnstated
            }
            Self::ProgressPortabilityBlocked => {
                M5LearningDowngradeTrigger::ProgressOwnershipUnstated
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExactLearning => "live_exact_learning",
            Self::LearningModePaused => "learning_mode_paused",
            Self::TipSnoozed => "tip_snoozed",
            Self::ExercisePackStale => "exercise_pack_stale",
            Self::CitationStale => "citation_stale",
            Self::ExplainDoUnprovable => "explain_do_unprovable",
            Self::ProgressPortabilityBlocked => "progress_portability_blocked",
        }
    }
}

/// One learning dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5LearningComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5LearningComponentConditionState,
}

/// An honest learning-claim auto-narrow block. When a learning dimension weakens, the component's
/// learning claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical command-binding / citation / progress-ownership lineage rather than
/// silently dropping it — the underlying learning lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentClaimAutoNarrow {
    /// The learning claim the component is narrowed to.
    pub narrowed_to: M5LearningComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5LearningComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5LearningDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical command binding, glossary citation, explain-versus-do boundary, and progress
    /// ownership are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying command-binding / citation / progress-ownership lineage is preserved (never
    /// dropped) across the narrowing; must hold so paused-mode, snoozed-tip, stale-pack,
    /// uncited-glossary, unprovable-boundary, and blocked-progress states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl LearningComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and learning
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl LearningComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5LearningComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: LearningComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a learning-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / localization / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims exactness, or drops state silently
    /// (red).
    Stranded,
}

impl LearningComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one learning-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentAccessibilityRow {
    /// Record kind; must equal [`LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5LearningComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the command / citation / exercise / progress object this component teaches;
    /// stays visible on every surface, so this is never empty.
    pub learning_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / localized) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5LearningComponentFallbackModality>,
    /// The non-visual / localized path reaches the same canonical command binding, learning-mode
    /// state, exercise success-criteria, glossary citation, explain-versus-do boundary, and
    /// progress ownership as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: LearningComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: LearningComponentNonVisualReachState,
    /// Localization / source-language-fallback reach into the non-visual path.
    pub localization_reach: LearningComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: LearningComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: LearningComponentCopyExportParity,
    /// The full learning claim this family asserts when every dimension is intact.
    pub full_learning_claim: M5LearningComponentClaim,
    /// The observed condition of each modeled learning dimension.
    #[serde(default)]
    pub claim_conditions: Vec<LearningComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<LearningComponentClaimAutoNarrow>,
    /// Whether the underlying learning lineage is preserved on this component regardless of
    /// narrowing; must hold so paused-mode, snoozed-tip, stale-pack, uncited-glossary,
    /// unprovable-boundary, and blocked-progress states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5LearningComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<LearningComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl LearningComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / localized) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `LiveExactLearning` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5LearningComponentClaimDimension,
    ) -> M5LearningComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5LearningComponentConditionState::LiveExactLearning)
    }

    /// Whether any modeled dimension is weaker than exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest learning claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5LearningComponentClaim {
        let mut permitted = self.full_learning_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&LearningComponentClaimConditionEntry> {
        let mut binding: Option<(&LearningComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_learning_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5LearningComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The learning claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5LearningComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_learning_claim,
        }
    }

    /// AC / auto-narrowing honesty: a paused mode, a snoozed tip, a stale exercise pack, an uncited
    /// glossary entry, an unprovable explain-versus-do boundary, or blocked progress portability
    /// can no longer keep an old `ExactLearning` / `ReviewableGuidance` label. The effective claim
    /// never exceeds the permitted ceiling; when a dimension narrows below the full claim, an
    /// honest narrow block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical identity and
    /// learning lineage. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / exact-learning honesty: a stale-citation / drifted-pack / unprovable-boundary /
    /// blocked-progress state never keeps an exact learning claim. When such a state is modeled,
    /// the effective claim must not assert `ExactLearning`.
    pub fn exact_learning_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_proven_exact());
        !(has_unprovable_state && self.effective_claim().asserts_exact_learning())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / localization trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.learning_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.localization_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: paused-mode, snoozed-tip, stale-pack, uncited-glossary, unprovable-boundary,
    /// and blocked-progress states preserve the underlying learning lineage. The row must assert
    /// `lineage_preserved`, and any narrow block must preserve lineage continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.localization_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> LearningComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.exact_learning_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return LearningComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            LearningComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            LearningComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.learning_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} localization={localization} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            localization = self.localization_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_learning_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1010 learning-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_exact_learning_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`LearningComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<LearningComponentAccessibilityRow>,
}

/// Checked-in M05-1010 learning-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<LearningComponentAccessibilityRow>,
    pub summary: LearningComponentAccessibilitySummary,
}

impl LearningComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: LearningComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: LearningComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_exact_learning_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LearningComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5LearningComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5LearningComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Learning claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5LearningComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5LearningConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> LearningComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5LearningConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&LearningComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                LearningComponentAccessibilityStatus::Parity => green += 1,
                LearningComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                LearningComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        LearningComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::claim_is_honest),
            all_exact_learning_honesty_holds: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::exact_learning_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(LearningComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<LearningComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(LearningComponentAccessibilityViolation::SchemaVersion {
                expected: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != LEARNING_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(LearningComponentAccessibilityViolation::RecordKind {
                expected: LEARNING_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(LearningComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(LearningComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_proven_exact())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(LearningComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory learning label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5LearningComponentFallbackModality::Structured)
            {
                violations.push(
                    LearningComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts an exact / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(LearningComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a stale / uncited / unprovable / blocked state never keeps an exact learning
            // claim.
            if !row.exact_learning_honesty_holds() {
                violations.push(
                    LearningComponentAccessibilityViolation::UnprovableStateShownAsExact {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / localization reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    LearningComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    LearningComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: paused-mode, snoozed-tip, stale-pack, uncited-glossary,
            // unprovable-boundary, and blocked-progress states preserve learning lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(LearningComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    LearningComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == LearningComponentAccessibilityStatus::Stranded {
                violations.push(LearningComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5LearningComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5LearningComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the exact baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5LearningComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every learning claim tier appears as an effective claim, so the full narrowing
        // spectrum (exact-learning → … → blocked-progress) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5LearningComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Exact-learning honesty must be proven with at least one stale / uncited / unprovable /
        // blocked row in the packet, so the "cannot-prove never shown as exact" guarantee is
        // exercised end-to-end.
        if !has_unprovable_row {
            violations.push(LearningComponentAccessibilityViolation::ExactLearningHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the onboarding, tour-overlay,
        // learning-panel, glossary, exercise, help-panel, CLI help, support export, and product UI
        // — so every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5LearningConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    LearningComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(LearningComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("learning-component accessibility fallback packet serializes"),
        ) {
            violations.push(LearningComponentAccessibilityViolation::RawLearningMaterialInExport);
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
            .expect("learning-component accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,localization_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{localization},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                localization = row.localization_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_learning_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Learning-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5LearningComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_learning_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in learning-component accessibility fallback export.
pub fn current_m5_learning_component_a11y_fallback_export(
) -> Result<LearningComponentAccessibilityPacket, LearningComponentAccessibilityArtifactError> {
    let packet: LearningComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-accessibility-fallback/support_export.json"
    )))
    .map_err(LearningComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LearningComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in learning-component accessibility fallback export.
#[derive(Debug)]
pub enum LearningComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LearningComponentAccessibilityViolation>),
}

impl fmt::Display for LearningComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "learning-component accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "learning-component accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for LearningComponentAccessibilityArtifactError {}

/// Validation failure for M05-1010 learning-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearningComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5LearningComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnprovableStateShownAsExact {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    LineageDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5LearningComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5LearningComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5LearningComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5LearningComponentClaim,
    },
    ExactLearningHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5LearningConsumerSurface,
    },
    SummaryMismatch,
    RawLearningMaterialInExport,
}

impl fmt::Display for LearningComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory learning label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts an exact / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::UnprovableStateShownAsExact { id } => {
                write!(
                    f,
                    "row {id} shows a stale / uncited / unprovable / blocked state as exact learning"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / localization users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve learning lineage across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "learning claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::ExactLearningHonestyUnproven => {
                write!(
                    f,
                    "no stale / uncited / unprovable / blocked row is present to prove the exact-learning-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawLearningMaterialInExport => {
                write!(f, "export contains raw learning material")
            }
        }
    }
}

impl Error for LearningComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "snoozed"
            | "paused"
            | "partial"
            | "stale"
            | "stale pack"
            | "uncited"
            | "unprovable"
            | "not portable"
            | "local only"
            | "local-only"
            | "no binding"
            | "source language"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in learning-component accessibility fallback packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_learning_component_a11y_fallback_packet() -> LearningComponentAccessibilityPacket {
    LearningComponentAccessibilityPacket::new(LearningComponentAccessibilityPacketInput {
        packet_id: "m5-learning-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:learning-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5LearningRequiredLabel> {
    M5LearningRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> LearningComponentCopyExportParity {
    LearningComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5LearningComponentClaimDimension,
    state: M5LearningComponentConditionState,
) -> LearningComponentClaimConditionEntry {
    LearningComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// help — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5LearningConsumerSurface]) -> Vec<M5LearningConsumerSurface> {
    let mut out = vec![
        M5LearningConsumerSurface::SupportExport,
        M5LearningConsumerSurface::CliHelp,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: LearningComponentNarrowingDisclosureState,
) -> Vec<LearningComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        LearningComponentRenderingNarrowingDisclosure {
            rendering_surface: M5LearningComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        LearningComponentRenderingNarrowingDisclosure {
            rendering_surface: M5LearningComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<LearningComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        LearningComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<LearningComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        LearningComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5LearningComponentRenderingSurface> {
    vec![
        M5LearningComponentRenderingSurface::DesktopFull,
        M5LearningComponentRenderingSurface::CliHeadless,
        M5LearningComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5LearningComponentFallbackModality> {
    vec![
        M5LearningComponentFallbackModality::List,
        M5LearningComponentFallbackModality::Textual,
        M5LearningComponentFallbackModality::Localized,
    ]
}

fn seeded_rows() -> Vec<LearningComponentAccessibilityRow> {
    vec![
        // Learning-mode toggle (live / on) — the learning mode is active, command-backed, and
        // scoped, so it is fully exact learning and reachable on every surface (green).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:learning-mode-toggle-live".to_owned(),
            component_family: M5LearningComponentFamily::LearningModeToggle,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:learning-mode-toggle:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:learning-mode-toggle-live:a11y".to_owned(),
            copy_export: copy_export(&[
                "mode_identity",
                "mode_state",
                "mode_scope",
                "keyboard_route",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::LearningModeDelivery,
                M5LearningComponentConditionState::LiveExactLearning,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["mode_identity", "mode_state", "mode_scope"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::OnboardingUi,
                M5LearningConsumerSurface::LearningPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 learning-mode toggles".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("learning-mode-toggle-live"),
        },
        // Safe explanation banner (explain-only, live) — the explain-versus-do boundary is stated
        // and the banner is a self-sufficient, reviewable read-only explanation (not itself an
        // exact-learning path), reachable on every surface (green).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:safe-explanation-banner-explain-only".to_owned(),
            component_family: M5LearningComponentFamily::SafeExplanationBanner,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:safe-explanation-banner:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:safe-explanation-banner-explain-only:a11y".to_owned(),
            copy_export: copy_export(&[
                "explanation_identity",
                "explain_versus_do_boundary",
                "cited_source",
                "keyboard_route",
            ]),
            full_learning_claim: M5LearningComponentClaim::ReviewableGuidance,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::ExplainDoBoundary,
                M5LearningComponentConditionState::LiveExactLearning,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "explanation_identity",
                "explain_versus_do_boundary",
                "cited_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::ProductUi,
                M5LearningConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 safe explanation banners".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("safe-explanation-banner-explain-only"),
        },
        // Learning-mode toggle (paused) — the learning mode has been paused, so it auto-narrows to
        // a paused-mode projection rather than presenting itself as an active live mode, while
        // keeping its identity, state, and scope visible (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:learning-mode-toggle-paused".to_owned(),
            component_family: M5LearningComponentFamily::LearningModeToggle,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:learning-mode-toggle:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:learning-mode-toggle-paused:a11y".to_owned(),
            copy_export: copy_export(&[
                "mode_identity",
                "mode_state",
                "mode_scope",
                "pause_state",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::LearningModeDelivery,
                M5LearningComponentConditionState::LearningModePaused,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::PausedModeProjection,
                binding_dimension: M5LearningComponentClaimDimension::LearningModeDelivery,
                trigger: M5LearningDowngradeTrigger::LearningModeStateUnstated,
                narrowed_label:
                    "Learning mode is paused for now — shown as a paused-mode projection with its scope and stable command binding still reachable, never as an active live learning mode"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "mode_identity",
                "mode_state",
                "mode_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::LearningPanelUi,
                M5LearningConsumerSurface::OnboardingUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 learning-mode pause / resume".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("learning-mode-toggle-paused"),
        },
        // Tip card (snoozed) — the tip has been snoozed, so it auto-narrows to a snoozed-tip
        // projection rather than presenting itself as an active live tip, while keeping its
        // identity, trigger, and command binding visible (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:tip-card-snoozed".to_owned(),
            component_family: M5LearningComponentFamily::TipCard,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:tip-card:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:tip-card-snoozed:a11y".to_owned(),
            copy_export: copy_export(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
                "snooze_state",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::TipDelivery,
                M5LearningComponentConditionState::TipSnoozed,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::SnoozedTipProjection,
                binding_dimension: M5LearningComponentClaimDimension::TipDelivery,
                trigger: M5LearningDowngradeTrigger::TipCommandBindingUnstated,
                narrowed_label:
                    "This tip is snoozed for now — shown as a snoozed-tip projection with its trigger and stable command binding still reachable, never as an active live tip"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::TourOverlayUi,
                M5LearningConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 tip card snooze / dismissal".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("tip-card-snoozed"),
        },
        // Guided exercise step (pack freshness drifted) — hierarchy-heavy (nested lesson / step /
        // sub-step / hint / success-criteria lineage); the exercise pack's freshness has drifted,
        // so the step auto-narrows to a stale-pack projection and binds its nested step lineage to
        // a flat list / textual path (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:guided-exercise-step-stale-pack".to_owned(),
            component_family: M5LearningComponentFamily::GuidedExerciseStep,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:guided-exercise-step:0005".to_owned(),
            fallback_modalities: vec![
                M5LearningComponentFallbackModality::Structured,
                M5LearningComponentFallbackModality::List,
                M5LearningComponentFallbackModality::Textual,
                M5LearningComponentFallbackModality::Localized,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::DisclosedReducedButReachable,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:guided-exercise-step-stale-pack:a11y".to_owned(),
            copy_export: copy_export(&[
                "exercise_identity",
                "step_state",
                "success_criteria",
                "pack_freshness",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::ExercisePackFreshness,
                M5LearningComponentConditionState::ExercisePackStale,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::StalePackProjection,
                binding_dimension: M5LearningComponentClaimDimension::ExercisePackFreshness,
                trigger: M5LearningDowngradeTrigger::ExerciseStepStateUnstated,
                narrowed_label:
                    "This exercise pack's freshness has drifted — shown as a stale-pack projection that names the step state and success criteria from the last-known pack, never as an exact-current exercise"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "exercise_identity",
                "step_state",
                "success_criteria",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::ExerciseUi,
                M5LearningConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 guided exercise steps".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("guided-exercise-step-stale-pack"),
        },
        // Glossary chip / card (citation stale) — the glossary entry's citation is stale, so the
        // card auto-narrows to an uncited-glossary projection that keeps its last-known citation
        // preserved, never masquerading as a cited-current definition. Its localized source-
        // language fallback is disclosed-reduced but still reachable (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:glossary-chip-card-citation-stale".to_owned(),
            component_family: M5LearningComponentFamily::GlossaryChipOrCard,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:glossary-chip-card:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::DisclosedReducedButReachable,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:glossary-chip-card-citation-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "term_identity",
                "citation_source",
                "source_class",
                "freshness_state",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::CitationFreshness,
                M5LearningComponentConditionState::CitationStale,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::UncitedGlossaryProjection,
                binding_dimension: M5LearningComponentClaimDimension::CitationFreshness,
                trigger: M5LearningDowngradeTrigger::GlossaryCitationSevered,
                narrowed_label:
                    "This glossary entry's citation is out of date — shown as an uncited-glossary projection with its last-known cited source and source class preserved, never as a cited-current definition"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "term_identity",
                "citation_source",
                "source_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::GlossaryUi,
                M5LearningConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 glossary chips / cards".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("glossary-chip-card-citation-stale"),
        },
        // Safe explanation banner (explain-versus-do boundary unprovable) — the explain-versus-do
        // boundary cannot be proven, so the banner auto-narrows to an unprovable-boundary
        // projection that stays explain-only, never offering an apply-capable action (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:safe-explanation-banner-unprovable-boundary".to_owned(),
            component_family: M5LearningComponentFamily::SafeExplanationBanner,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:safe-explanation-banner:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:safe-explanation-banner-unprovable-boundary:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "explanation_identity",
                "explain_versus_do_boundary",
                "apply_boundary",
                "cited_source",
            ]),
            full_learning_claim: M5LearningComponentClaim::ReviewableGuidance,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::ExplainDoBoundary,
                M5LearningComponentConditionState::ExplainDoUnprovable,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::UnprovableBoundaryProjection,
                binding_dimension: M5LearningComponentClaimDimension::ExplainDoBoundary,
                trigger: M5LearningDowngradeTrigger::ExplanationApplyBoundaryUnstated,
                narrowed_label:
                    "The explain-versus-do boundary can't be proven here — shown as an unprovable-boundary projection that stays explain-only with its cited source preserved, never offering an apply-capable action"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "explanation_identity",
                "explain_versus_do_boundary",
                "cited_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::ProductUi,
                M5LearningConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 explain-versus-do boundaries".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("safe-explanation-banner-unprovable-boundary"),
        },
        // Progress marker (portability blocked) — resume / export portability is blocked, so the
        // marker auto-narrows to a blocked-progress projection that stays a local-only progress
        // explanation with its ownership / privacy preserved, never a portable-progress claim
        // (yellow).
        LearningComponentAccessibilityRow {
            record_kind: LEARNING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:progress-marker-portability-blocked".to_owned(),
            component_family: M5LearningComponentFamily::ProgressMarker,
            source_family_schema_ref: LEARNING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            learning_context_ref: "learning:progress-marker:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            localization_reach: LearningComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: LearningComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:progress-marker-portability-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "progress_identity",
                "progress_state",
                "ownership_and_privacy",
                "portability_state",
            ]),
            full_learning_claim: M5LearningComponentClaim::ExactLearning,
            claim_conditions: vec![condition(
                M5LearningComponentClaimDimension::ProgressPortability,
                M5LearningComponentConditionState::ProgressPortabilityBlocked,
            )],
            claim_narrow: Some(LearningComponentClaimAutoNarrow {
                narrowed_to: M5LearningComponentClaim::BlockedProgressProjection,
                binding_dimension: M5LearningComponentClaimDimension::ProgressPortability,
                trigger: M5LearningDowngradeTrigger::ProgressOwnershipUnstated,
                narrowed_label:
                    "Resume / export of this progress is blocked — shown as a blocked-progress projection that stays a local-only progress explanation with its ownership and privacy preserved, never a portable-progress claim"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "progress_identity",
                "progress_state",
                "ownership_and_privacy",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5LearningConsumerSurface::LearningPanelUi,
                M5LearningConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §16.38 progress markers".to_owned(),
                LEARNING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("progress-marker-portability-blocked"),
        },
    ]
}
