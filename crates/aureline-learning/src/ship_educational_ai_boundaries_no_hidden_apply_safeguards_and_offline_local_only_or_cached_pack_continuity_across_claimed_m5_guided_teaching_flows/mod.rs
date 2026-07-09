//! Educational-AI boundary states, no-hidden-apply safeguards, and
//! offline / local-only / cached-pack continuity across claimed M5 guided-teaching flows.
//!
//! This module governs the *degraded* and *disconnected* states of every reusable learning
//! component frozen in
//! [`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`].
//! The prior implement lanes narrowed the six components (the learning-mode toggle, the tip card,
//! the guided exercise step, the glossary chip/card, the safe explanation banner, and the progress
//! marker) for the live path. This lane closes the acceptance-criteria gap that remains once a
//! remote enrichment is missing, a docs pack is stale, a citation is unavailable, the network is
//! gone, or a pack was never installed: a learner must be able to tell, *before* acting, whether
//! they are looking at live, cached, local-only, offline, stale, uncited, or not-installed
//! learning content, and educational AI must never mutate live state without crossing the ordinary
//! preview / approval path.
//!
//! Every [`LearningDegradedComponentRow`] binds one governed component family, the subject it
//! teaches, a preserved subject *summary*, a stable component identity, its learning scope, and
//! one controlled continuity state. Its data-trust class is *derived* from that continuity state
//! rather than asserted, so a cached, local-only, offline, or stale surface can never read as
//! live, and its next-safe-action is *derived* too, so the copy that tells the learner what to do
//! next is never invented per component. A component whose enrichment is degraded but whose cited
//! source is still reachable (cached, local-only, offline, stale-pack) always names a source
//! fallback and offers a resolvable cited source, so learning stays useful offline. A component
//! whose citation is unavailable or whose pack was never installed degrades to an *explicit*
//! uncited / not-installed state and stops routing to a cited source it does not have.
//!
//! The educational-AI apply posture is an independent axis: a component either only explains, only
//! mutates a sandbox, or offers a "do" action that is gated behind the ordinary preview / approval
//! path. The apply disposition and the live-mutation flag are *derived* from that posture, so no
//! component can imply a hidden apply, and none can mutate live state without the preview /
//! approval crossing.
//!
//! The component families ([`M5LearningComponentFamily`]), the one controlled disposition
//! vocabulary ([`M5LearningDisposition`]), learning-mode scopes ([`M5LearningModeScope`]), required
//! labels ([`M5LearningRequiredLabel`]), surface families ([`M5LearningSurfaceFamily`]), deployment
//! lines ([`M5LearningDeploymentLine`]), consumer surfaces ([`M5LearningConsumerSurface`]),
//! accessibility routes ([`M5LearningAccessibilityRoute`]), and downgrade triggers
//! ([`M5LearningDowngradeTrigger`]) are reused directly from the frozen matrix, so this lane never
//! invents a parallel learning vocabulary. It mints new vocabulary only for what the matrix left
//! implicit about degraded and disconnected states: the controlled continuity state, the derived
//! data-trust class, the derived next-safe-action, the educational-AI apply posture and its derived
//! apply disposition, the subject kind, the reachable source kind, and the keyboard-complete safe
//! verbs that survive a degraded state.
//!
//! Raw private payloads, progress bodies, secret values, and private endpoints stay outside the
//! export boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-learning-educational-ai-continuity-controls.schema.json`](../../../../schemas/ui/m5-learning-educational-ai-continuity-controls.schema.json).
//! The contract doc is
//! [`docs/help/ship_educational_ai_boundaries_no_hidden_apply_safeguards_and_offline_local_only_or_cached_pack_continuity.md`](../../../../docs/help/ship_educational_ai_boundaries_no_hidden_apply_safeguards_and_offline_local_only_or_cached_pack_continuity.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_learning_educational_ai_continuity_controls,
    seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary,
    seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker,
    LEARNING_EDUCATIONAL_AI_CONTINUITY_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The component families, controlled disposition vocabulary, learning scopes, required labels,
// surface families, deployment lines, consumer surfaces, accessibility routes, and downgrade
// triggers are frozen once, in the learning component matrix. This lane reuses them verbatim so it
// never invents a parallel learning vocabulary.
use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5LearningAccessibilityRoute, M5LearningComponentFamily, M5LearningConsumerSurface,
    M5LearningDeploymentLine, M5LearningDisposition, M5LearningDowngradeTrigger,
    M5LearningModeScope, M5LearningRequiredLabel, M5LearningSurfaceFamily,
    M5_GLOSSARY_CHIP_CARD_SCHEMA_REF, M5_GUIDED_EXERCISE_STEP_SCHEMA_REF,
    M5_LEARNING_COMPONENT_DOC_REF, M5_LEARNING_COMPONENT_SCHEMA_REF,
    M5_LEARNING_MODE_TOGGLE_SCHEMA_REF, M5_PROGRESS_MARKER_SCHEMA_REF,
    M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF, M5_TIP_CARD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`LearningEducationalAiContinuityPacket`].
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_RECORD_KIND: &str =
    "learning_educational_ai_continuity_controls";

/// Schema version for learning educational-AI continuity control records.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-educational-ai-continuity-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_DOC_REF: &str =
    "docs/help/ship_educational_ai_boundaries_no_hidden_apply_safeguards_and_offline_local_only_or_cached_pack_continuity.md";

/// Repo-relative path of the protected fixture directory.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-learning-educational-ai-continuity-controls";

/// Repo-relative path of the checked support-export artifact.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-educational-ai-continuity-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/release/m5-learning-educational-ai-continuity-proof/summary.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const LEARNING_EDUCATIONAL_AI_CONTINUITY_CSV_REF: &str =
    "artifacts/release/m5-learning-educational-ai-continuity-proof/matrix.csv";

// ---- continuity-state vocabulary ----------------------------------------

/// Controlled continuity state a learning component can be in. These are the exact
/// acceptance-criteria states this lane governs: `live`, `cached`, `local-only`, `offline`,
/// `stale-pack`, `citation-unavailable`, and `not-installed`. No learning component invents a
/// parallel word for any of these states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningContinuityState {
    /// Full remote enrichment is available; live learning content.
    Live,
    /// Showing last-known cached-pack content, not a live one.
    Cached,
    /// Running against local-only content; no remote enrichment is contacted.
    LocalOnly,
    /// The relay is unreachable; content is held offline pending reconnection.
    Offline,
    /// The docs pack is stale; its content is unverified until an update.
    StalePack,
    /// A cited source is unavailable; the content degrades to an explicit uncited state.
    CitationUnavailable,
    /// The feature or pack was never installed; there is nothing to enrich.
    NotInstalled,
}

impl LearningContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Live,
        Self::Cached,
        Self::LocalOnly,
        Self::Offline,
        Self::StalePack,
        Self::CitationUnavailable,
        Self::NotInstalled,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::LocalOnly => "local_only",
            Self::Offline => "offline",
            Self::StalePack => "stale_pack",
            Self::CitationUnavailable => "citation_unavailable",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Derived data-trust class a learning component may present.
///
/// This is the degraded-state honesty axis: the class is derived from the continuity state,
/// never asserted, so a cached, local-only, offline, or stale surface can never read as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningTrustClass {
    /// Live and trustable as current, with full remote enrichment.
    LiveEnriched,
    /// A cached-pack value with reduced trust; refresh for the latest.
    CachedPack,
    /// Local-only content, bounded to what is on this device.
    LocalOnlyBounded,
    /// A last-known value held offline; stale until reconnection.
    OfflineHeld,
    /// A stale docs pack whose content is unverified until an update.
    StaleUnverified,
    /// Content shown without its citation; explicitly uncited and withheld from apply.
    UncitedWithheld,
    /// The feature or pack is not installed; unavailable until installed.
    NotInstalledUnavailable,
}

impl LearningTrustClass {
    /// Every data-trust class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveEnriched,
        Self::CachedPack,
        Self::LocalOnlyBounded,
        Self::OfflineHeld,
        Self::StaleUnverified,
        Self::UncitedWithheld,
        Self::NotInstalledUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveEnriched => "live_enriched",
            Self::CachedPack => "cached_pack",
            Self::LocalOnlyBounded => "local_only_bounded",
            Self::OfflineHeld => "offline_held",
            Self::StaleUnverified => "stale_unverified",
            Self::UncitedWithheld => "uncited_withheld",
            Self::NotInstalledUnavailable => "not_installed_unavailable",
        }
    }
}

/// Derived next-safe-action a learning component names before an action, so the copy that tells
/// the learner what to do next is never invented per component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningNextSafeAction {
    /// Proceed in learning — the content is live.
    ProceedInLearning,
    /// Refresh for the latest enrichment — the content is cached.
    RefreshEnrichment,
    /// Continue with local-only content — no remote enrichment is contacted.
    ContinueLocalOnly,
    /// Retry when back online — the content is offline.
    RetryWhenOnline,
    /// Update the docs pack — the content is stale.
    UpdateDocsPack,
    /// Shown without a citation — the citation is unavailable.
    ShowUncitedExplicitly,
    /// Install to enable — the feature or pack is not installed.
    InstallToEnable,
}

impl LearningNextSafeAction {
    /// Every next-safe-action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProceedInLearning,
        Self::RefreshEnrichment,
        Self::ContinueLocalOnly,
        Self::RetryWhenOnline,
        Self::UpdateDocsPack,
        Self::ShowUncitedExplicitly,
        Self::InstallToEnable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedInLearning => "proceed_in_learning",
            Self::RefreshEnrichment => "refresh_enrichment",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::RetryWhenOnline => "retry_when_online",
            Self::UpdateDocsPack => "update_docs_pack",
            Self::ShowUncitedExplicitly => "show_uncited_explicitly",
            Self::InstallToEnable => "install_to_enable",
        }
    }
}

/// The educational-AI apply posture — an independent axis governing whether a learning component
/// only explains, only mutates a sandbox, offers a preview / approval-gated "do" action, or has
/// its apply path blocked in the current state. This is the no-hidden-apply axis: no component may
/// mutate live state without crossing the ordinary preview / approval path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationalApplyPosture {
    /// The component only explains; it never offers to change anything.
    ExplainOnly,
    /// The component offers practice that mutates a sandbox only, never live state.
    SandboxedPractice,
    /// The component offers a "do" action routed through the ordinary preview / approval path.
    PreviewThenApprove,
    /// The component's apply path is blocked in the current continuity state.
    ApplyBlocked,
}

impl EducationalApplyPosture {
    /// Every apply posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExplainOnly,
        Self::SandboxedPractice,
        Self::PreviewThenApprove,
        Self::ApplyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::SandboxedPractice => "sandboxed_practice",
            Self::PreviewThenApprove => "preview_then_approve",
            Self::ApplyBlocked => "apply_blocked",
        }
    }
}

/// Derived apply disposition a learning component may present, derived from its apply posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationalApplyDisposition {
    /// Nothing is mutated; the component only explains.
    NoMutation,
    /// Only a sandbox is mutated; live state is never touched.
    SandboxMutationOnly,
    /// A live mutation is offered but only after preview and approval.
    PreviewApprovalRequired,
    /// The apply path is unavailable in the current state.
    MutationUnavailable,
}

impl EducationalApplyDisposition {
    /// Every apply disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoMutation,
        Self::SandboxMutationOnly,
        Self::PreviewApprovalRequired,
        Self::MutationUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::SandboxMutationOnly => "sandbox_mutation_only",
            Self::PreviewApprovalRequired => "preview_approval_required",
            Self::MutationUnavailable => "mutation_unavailable",
        }
    }
}

/// What a learning component teaches or references. Reused across the row so a component's subject
/// identity is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningSubjectKind {
    /// A concept or glossary term.
    Concept,
    /// A command.
    Command,
    /// A file or symbol.
    FileOrSymbol,
    /// A docs topic.
    DocsTopic,
    /// An exercise task.
    ExerciseTask,
    /// A progress record.
    ProgressRecord,
}

impl LearningSubjectKind {
    /// Every subject kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Concept,
        Self::Command,
        Self::FileOrSymbol,
        Self::DocsTopic,
        Self::ExerciseTask,
        Self::ProgressRecord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Concept => "concept",
            Self::Command => "command",
            Self::FileOrSymbol => "file_or_symbol",
            Self::DocsTopic => "docs_topic",
            Self::ExerciseTask => "exercise_task",
            Self::ProgressRecord => "progress_record",
        }
    }
}

/// The kind of cited / reachable source a learning component can open. `NoSource` marks a
/// component whose cited source is unavailable or not installed — it stops routing to a source it
/// does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningSourceKind {
    /// An exact file location.
    FileLocation,
    /// An exact symbol location.
    SymbolLocation,
    /// A docs page.
    DocsPage,
    /// A stable command reference.
    CommandReference,
    /// A sandbox practice target.
    SandboxTarget,
    /// No reachable source — the citation is unavailable or the pack is not installed.
    NoSource,
}

impl LearningSourceKind {
    /// Every source kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileLocation,
        Self::SymbolLocation,
        Self::DocsPage,
        Self::CommandReference,
        Self::SandboxTarget,
        Self::NoSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileLocation => "file_location",
            Self::SymbolLocation => "symbol_location",
            Self::DocsPage => "docs_page",
            Self::CommandReference => "command_reference",
            Self::SandboxTarget => "sandbox_target",
            Self::NoSource => "no_source",
        }
    }
}

/// One keyboard-complete safe verb a learning component preserves even in a degraded state, so a
/// degraded component never hides its action affordance and never implies a hidden apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningSafeVerb {
    /// Explain the subject — the explain-only affordance that always survives a degraded state.
    Explain,
    /// Open the cited source this component references (lands on its stable cited source ref).
    OpenSource,
    /// Practice in a sandbox — never mutates live state.
    PracticeInSandbox,
    /// Refresh the component to fetch the latest enrichment.
    Refresh,
    /// Copy the stable component / source reference.
    CopyReference,
    /// Dismiss the component.
    Dismiss,
}

impl LearningSafeVerb {
    /// Every safe verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Explain,
        Self::OpenSource,
        Self::PracticeInSandbox,
        Self::Refresh,
        Self::CopyReference,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete component must offer.
    pub const MANDATORY: [Self; 1] = [Self::Explain];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explain => "explain",
            Self::OpenSource => "open_source",
            Self::PracticeInSandbox => "practice_in_sandbox",
            Self::Refresh => "refresh",
            Self::CopyReference => "copy_reference",
            Self::Dismiss => "dismiss",
        }
    }

    /// Whether this verb opens the cited source.
    fn is_open_source_verb(self) -> bool {
        matches!(self, Self::OpenSource)
    }
}

/// Disclosures a learning component must carry, derived from its continuity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinuityDisclosure {
    /// The derived data-trust class this component may present.
    pub trust_class: LearningTrustClass,
    /// The derived next-safe-action this component names.
    pub next_safe_action: LearningNextSafeAction,
    /// Whether the component may claim live, fully-enriched content.
    pub is_live: bool,
    /// Whether the component must carry an explicit continuity-state explanation.
    pub needs_continuity_explanation: bool,
    /// Whether the enrichment is degraded but a cited / cached source is still reachable, so the
    /// component must name a source fallback and offer a resolvable source.
    pub needs_source_fallback: bool,
    /// Whether the cited source is unavailable (citation missing or pack not installed) so the
    /// component must stop routing to a source it does not have.
    pub source_unavailable: bool,
}

/// Resolves the continuity-state truth a learning component may present.
///
/// A live component is enriched and proceeds in learning. A cached component has reduced trust and
/// refreshes for the latest. A local-only component continues with what is on the device. An
/// offline component is stale and retries when online. A stale-pack component updates its docs
/// pack. A citation-unavailable component degrades to an explicit uncited state and stops routing
/// to a cited source. A not-installed component installs to enable and has nothing to open.
pub fn resolve_continuity(state: LearningContinuityState) -> ContinuityDisclosure {
    use LearningContinuityState as State;
    use LearningNextSafeAction as Next;
    use LearningTrustClass as Trust;

    let trust_class = match state {
        State::Live => Trust::LiveEnriched,
        State::Cached => Trust::CachedPack,
        State::LocalOnly => Trust::LocalOnlyBounded,
        State::Offline => Trust::OfflineHeld,
        State::StalePack => Trust::StaleUnverified,
        State::CitationUnavailable => Trust::UncitedWithheld,
        State::NotInstalled => Trust::NotInstalledUnavailable,
    };

    let next_safe_action = match state {
        State::Live => Next::ProceedInLearning,
        State::Cached => Next::RefreshEnrichment,
        State::LocalOnly => Next::ContinueLocalOnly,
        State::Offline => Next::RetryWhenOnline,
        State::StalePack => Next::UpdateDocsPack,
        State::CitationUnavailable => Next::ShowUncitedExplicitly,
        State::NotInstalled => Next::InstallToEnable,
    };

    let is_live = matches!(trust_class, Trust::LiveEnriched);
    // A degraded-but-reachable state keeps a cited / cached source the learner can still open, so
    // learning stays useful offline. A citation-unavailable or not-installed state has nothing
    // cited to open, so it stops routing instead.
    let needs_source_fallback = matches!(
        trust_class,
        Trust::CachedPack | Trust::LocalOnlyBounded | Trust::OfflineHeld | Trust::StaleUnverified
    );
    let source_unavailable = matches!(
        trust_class,
        Trust::UncitedWithheld | Trust::NotInstalledUnavailable
    );

    ContinuityDisclosure {
        trust_class,
        next_safe_action,
        is_live,
        needs_continuity_explanation: !is_live,
        needs_source_fallback,
        source_unavailable,
    }
}

/// Disclosures a learning component must carry, derived from its educational-AI apply posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyDisclosure {
    /// The derived apply disposition this component may present.
    pub apply_disposition: EducationalApplyDisposition,
    /// Whether the component offers a live-state mutation at all.
    pub offers_live_mutation: bool,
    /// Whether a live mutation must cross the ordinary preview / approval path.
    pub requires_preview_approval: bool,
    /// Whether any practice the component offers is sandboxed.
    pub practice_is_sandboxed: bool,
}

/// Resolves the apply truth a learning component may present.
///
/// An explain-only component mutates nothing. A sandboxed-practice component mutates a sandbox
/// only. A preview-then-approve component offers a live mutation but only after the ordinary
/// preview / approval path. An apply-blocked component's apply path is unavailable. The only
/// posture that offers a live mutation is `PreviewThenApprove`, and it always requires the
/// preview / approval crossing — so educational AI can never mutate live state without it.
pub fn resolve_apply(posture: EducationalApplyPosture) -> ApplyDisclosure {
    use EducationalApplyDisposition as Disp;
    use EducationalApplyPosture as Posture;

    match posture {
        Posture::ExplainOnly => ApplyDisclosure {
            apply_disposition: Disp::NoMutation,
            offers_live_mutation: false,
            requires_preview_approval: false,
            practice_is_sandboxed: false,
        },
        Posture::SandboxedPractice => ApplyDisclosure {
            apply_disposition: Disp::SandboxMutationOnly,
            offers_live_mutation: false,
            requires_preview_approval: false,
            practice_is_sandboxed: true,
        },
        Posture::PreviewThenApprove => ApplyDisclosure {
            apply_disposition: Disp::PreviewApprovalRequired,
            offers_live_mutation: true,
            requires_preview_approval: true,
            practice_is_sandboxed: false,
        },
        Posture::ApplyBlocked => ApplyDisclosure {
            apply_disposition: Disp::MutationUnavailable,
            offers_live_mutation: false,
            requires_preview_approval: false,
            practice_is_sandboxed: false,
        },
    }
}

/// A learning component in one governed continuity state, preserving its subject summary, stable
/// identity, learning scope, next-safe-action, apply posture, safe verbs, and — where its
/// enrichment is degraded but reachable — a cited source fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningDegradedComponentRow {
    /// Governed component family this component belongs to, reused from the frozen matrix.
    pub component_family: M5LearningComponentFamily,
    /// Stable component id.
    pub component_id: String,
    /// Human-readable component title; required and non-empty.
    pub component_title: String,
    /// Subject kind this component teaches / references.
    pub subject_kind: LearningSubjectKind,
    /// Human-readable subject label; required and non-empty.
    pub subject_label: String,
    /// Preserved subject summary — the last-known summary shown even when full enrichment cannot
    /// be fetched, so learning continuity survives a degraded state. Required and non-empty.
    pub subject_summary_note: String,
    /// Exact cited source reference — the one stable cited source `OpenSource` lands on. Required
    /// unless the continuity state is citation-unavailable or not-installed.
    pub cited_source_ref: String,
    /// Stable component identity, preserved across degraded states; required and non-empty.
    pub stable_component_ref: String,
    /// Learning scope this component is scoped to, reused from the frozen matrix.
    pub learning_scope: M5LearningModeScope,
    /// Human-readable learning-scope label; required and non-empty.
    pub scope_label: String,
    /// Governed continuity state.
    pub continuity_state: LearningContinuityState,
    /// Derived data-trust class (must equal the resolved class).
    pub trust_class: LearningTrustClass,
    /// Whether the component claims live, fully-enriched content (must equal the derived truth).
    pub claims_live_enrichment: bool,
    /// Scope / continuity note; always required so scope and continuity stay explicit.
    pub continuity_note: String,
    /// Continuity-state explanation; required whenever the component is not live, so a degraded
    /// state is always explicit before an action.
    pub state_explanation_note: String,
    /// Derived next-safe-action (must equal the resolved action).
    pub next_safe_action: LearningNextSafeAction,
    /// Next-safe-action copy; always required so the learner always knows what to do next.
    pub next_safe_action_note: String,
    /// Source-fallback note; required whenever the enrichment is degraded but a cited source is
    /// still reachable.
    pub source_fallback_note: String,
    /// Kind of cited / reachable source, or `NoSource` when unavailable.
    pub source_kind: LearningSourceKind,
    /// Human-readable source label; always required so the source is explicit.
    pub source_label: String,
    /// Educational-AI apply posture (the no-hidden-apply axis).
    pub apply_posture: EducationalApplyPosture,
    /// Derived apply disposition (must equal the resolved disposition).
    pub apply_disposition: EducationalApplyDisposition,
    /// Whether the component offers a live-state mutation (must equal the derived truth).
    pub offers_live_mutation: bool,
    /// Explain-versus-do boundary note; always required so the apply boundary stays explicit.
    pub apply_boundary_note: String,
    /// Keyboard-complete safe verbs (must include the mandatory `Explain`).
    pub safe_verbs: Vec<LearningSafeVerb>,
    /// Controlled dispositions this component binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Mandatory labels this component can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this component.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the component projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks its privacy / offline / local-only or cached state. MUST be
    /// `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides its cited source. MUST be `false`.
    pub hides_citation_source: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a hidden apply or widened mutating authority. MUST be
    /// `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: educational AI never mutates live state without crossing the ordinary
    /// preview / approval path. MUST be `false`.
    pub mutates_live_without_preview_approval: bool,
}

impl LearningDegradedComponentRow {
    /// Continuity-state disclosures this component must carry, derived from its continuity state.
    pub fn continuity_disclosure(&self) -> ContinuityDisclosure {
        resolve_continuity(self.continuity_state)
    }

    /// Apply disclosures this component must carry, derived from its apply posture.
    pub fn apply_disclosure(&self) -> ApplyDisclosure {
        resolve_apply(self.apply_posture)
    }

    /// Whether the component offers every mandatory keyboard-complete verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<LearningSafeVerb> = self.safe_verbs.iter().copied().collect();
        LearningSafeVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the component declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the component offers an open-cited-source verb.
    fn offers_open_source(&self) -> bool {
        self.safe_verbs
            .iter()
            .any(|verb| verb.is_open_source_verb())
    }

    /// Whether the component names a resolvable cited source (verb plus a non-`no_source` kind).
    fn offers_resolvable_source(&self) -> bool {
        self.offers_open_source() && self.source_kind != LearningSourceKind::NoSource
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance trust review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningEducationalAiContinuityGlanceReview {
    /// Every component names its subject summary and stable identity.
    pub every_component_names_subject_summary_and_identity: bool,
    /// Every component states its continuity.
    pub every_component_states_its_continuity: bool,
    /// Every component states its next-safe-action before an action.
    pub every_component_states_next_safe_action: bool,
    /// The degraded state is explicit before an action.
    pub degraded_state_is_explicit_before_action: bool,
    /// Live, cached, local-only, and not-installed content are distinguishable before an action.
    pub live_cached_local_only_not_installed_distinguishable: bool,
    /// A cached or stale component is never shown as live.
    pub cached_or_stale_never_shown_as_live: bool,
    /// The data-trust class is derived from the continuity state, never asserted.
    pub trust_class_derived_never_asserted: bool,
    /// Learning stays useful offline while preserving exact cached / local-only / not-installed
    /// truth.
    pub learning_stays_useful_offline: bool,
    /// Educational AI never mutates live state without crossing the preview / approval path.
    pub educational_ai_never_mutates_live_without_preview_approval: bool,
    /// No component implies a hidden apply.
    pub no_component_implies_hidden_apply: bool,
    /// A not-installed or uncited state stops routing to a cited source it does not have.
    pub not_installed_or_uncited_state_stops_source_routing: bool,
    /// Every degraded-but-reachable state names a cited or cached source fallback.
    pub every_reachable_state_names_a_cited_or_cached_source: bool,
    /// The subject identity is always explicit.
    pub subject_identity_always_explicit: bool,
    /// The learning scope is always explicit.
    pub learning_scope_always_explicit: bool,
    /// No component invents an alternate label for a governed state.
    pub no_component_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl LearningEducationalAiContinuityGlanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.every_component_names_subject_summary_and_identity
            && self.every_component_states_its_continuity
            && self.every_component_states_next_safe_action
            && self.degraded_state_is_explicit_before_action
            && self.live_cached_local_only_not_installed_distinguishable
            && self.cached_or_stale_never_shown_as_live
            && self.trust_class_derived_never_asserted
            && self.learning_stays_useful_offline
            && self.educational_ai_never_mutates_live_without_preview_approval
            && self.no_component_implies_hidden_apply
            && self.not_installed_or_uncited_state_stops_source_routing
            && self.every_reachable_state_names_a_cited_or_cached_source
            && self.subject_identity_always_explicit
            && self.learning_scope_always_explicit
            && self.no_component_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningEducationalAiContinuityConsumerProjection {
    /// The learning / onboarding UIs read a single canonical source.
    pub learning_surfaces_read_single_source: bool,
    /// The help / docs UIs read a single canonical source.
    pub help_surfaces_read_single_source: bool,
    /// The first glance names state, scope, and citation without drilling in.
    pub first_glance_names_state_scope_and_citation: bool,
    /// The next-safe-action is visible before an action.
    pub next_safe_action_visible_before_action: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl LearningEducationalAiContinuityConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.learning_surfaces_read_single_source
            && self.help_surfaces_read_single_source
            && self.first_glance_names_state_scope_and_citation
            && self.next_safe_action_visible_before_action
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningEducationalAiContinuityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`LearningEducationalAiContinuityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningEducationalAiContinuityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Degraded learning components.
    pub components: Vec<LearningDegradedComponentRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Glance review block.
    pub glance_review: LearningEducationalAiContinuityGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: LearningEducationalAiContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: LearningEducationalAiContinuityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe learning educational-AI continuity controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningEducationalAiContinuityPacket {
    /// Record kind; must equal [`LEARNING_EDUCATIONAL_AI_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Degraded learning components.
    pub components: Vec<LearningDegradedComponentRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Glance review block.
    pub glance_review: LearningEducationalAiContinuityGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: LearningEducationalAiContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: LearningEducationalAiContinuityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl LearningEducationalAiContinuityPacket {
    /// Builds a learning educational-AI continuity controls packet from stable-lane input.
    pub fn new(input: LearningEducationalAiContinuityPacketInput) -> Self {
        Self {
            record_kind: LEARNING_EDUCATIONAL_AI_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            components: input.components,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            glance_review: input.glance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the learning educational-AI continuity control invariants.
    pub fn validate(&self) -> Vec<LearningEducationalAiContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != LEARNING_EDUCATIONAL_AI_CONTINUITY_RECORD_KIND {
            violations.push(LearningEducationalAiContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_VERSION {
            violations.push(LearningEducationalAiContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(LearningEducationalAiContinuityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_components(self, &mut violations);

        if !self.glance_review.all_hold() {
            violations.push(LearningEducationalAiContinuityViolation::GlanceReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(LearningEducationalAiContinuityViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(LearningEducationalAiContinuityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("learning educational-AI continuity packet serializes"),
        ) {
            violations.push(LearningEducationalAiContinuityViolation::RawBoundaryMaterialInExport);
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
            .expect("learning educational-AI continuity packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,family,subject_kind,learning_scope,continuity_state,trust_class,claims_live,next_safe_action,apply_posture,apply_disposition,source_kind\n",
        );
        for component in &self.components {
            let disclosure = component.continuity_disclosure();
            let apply = component.apply_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                "learning_degraded_component",
                csv_field(&component.component_id),
                component.component_family.as_str(),
                component.subject_kind.as_str(),
                component.learning_scope.as_str(),
                component.continuity_state.as_str(),
                disclosure.trust_class.as_str(),
                disclosure.is_live,
                disclosure.next_safe_action.as_str(),
                component.apply_posture.as_str(),
                apply.apply_disposition.as_str(),
                component.source_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .components
            .iter()
            .filter(|component| !component.continuity_disclosure().is_live)
            .count();
        let offers_apply = self
            .components
            .iter()
            .filter(|component| component.apply_disclosure().offers_live_mutation)
            .count();

        let mut out = String::new();
        out.push_str("# Learning educational-AI continuity controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Degraded components: {} ({} not live, {} offering a preview/approval-gated apply)\n",
            self.components.len(),
            not_live,
            offers_apply
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Degraded components\n\n");
        for component in &self.components {
            let disclosure = component.continuity_disclosure();
            let apply = component.apply_disclosure();
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, state `{}` → trust `{}`, next `{}`, apply `{}`, source `{}`\n",
                component.component_title,
                component.component_family.as_str(),
                component.learning_scope.as_str(),
                component.continuity_state.as_str(),
                disclosure.trust_class.as_str(),
                disclosure.next_safe_action.as_str(),
                apply.apply_disposition.as_str(),
                component.source_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in learning educational-AI continuity export.
#[derive(Debug)]
pub enum LearningEducationalAiContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LearningEducationalAiContinuityViolation>),
}

impl fmt::Display for LearningEducationalAiContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "learning educational-AI continuity export parse failed: {error}"
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
                    "learning educational-AI continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for LearningEducationalAiContinuityArtifactError {}

/// Validation failures emitted by [`LearningEducationalAiContinuityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LearningEducationalAiContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No degraded components are present.
    ComponentsMissing,
    /// A degraded component is incomplete.
    ComponentIncomplete,
    /// A component does not name its exact cited source reference.
    CitedSourceRefMissing,
    /// A component names a cited source when its citation is unavailable / not installed.
    UncitedStateClaimsSource,
    /// A component does not name its stable component identity.
    StableComponentRefMissing,
    /// A component does not preserve its subject summary.
    SubjectSummaryMissing,
    /// A component misrepresents its derived continuity state.
    ContinuityStateMisrepresented,
    /// A degraded component does not name its continuity-state explanation.
    StateExplanationMissing,
    /// A component does not name its continuity note.
    ContinuityNoteMissing,
    /// A component misrepresents its derived next-safe-action.
    NextSafeActionMisrepresented,
    /// A component does not name its next-safe-action copy.
    NextSafeActionNoteMissing,
    /// A degraded-but-reachable component does not name its source fallback.
    SourceFallbackMissing,
    /// A degraded-but-reachable component does not offer a resolvable cited source.
    SourceFallbackRouteMissing,
    /// A citation-unavailable / not-installed component still routes to a cited source.
    UnavailableSourceStillOpens,
    /// A component misrepresents its derived apply disposition or live-mutation flag.
    ApplyPostureMisrepresented,
    /// A component does not name its explain-versus-do boundary.
    ApplyBoundaryNoteMissing,
    /// A component mutates live state without crossing the preview / approval path.
    LiveMutationWithoutPreviewApproval,
    /// A component omits the mandatory `Explain` verb.
    SafeVerbsIncomplete,
    /// A component declares no controlled dispositions.
    DispositionsMissing,
    /// The components do not cover every continuity state.
    ContinuityStateCoverageMissing,
    /// The components do not cover every component family.
    ComponentFamilyCoverageMissing,
    /// The components do not cover every apply posture.
    ApplyPostureCoverageMissing,
    /// A component does not name its source label.
    SourceLabelMissing,
    /// A component does not name its scope label.
    ScopeLabelMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component masks its privacy / offline / local-only state.
    PrivacyOrOfflineStateMasked,
    /// A component hides its cited source.
    CitationSourceHidden,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A component implies a hidden apply or widened mutating authority.
    HiddenApplyImplied,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Glance review does not satisfy required invariants.
    GlanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl LearningEducationalAiContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ComponentsMissing => "components_missing",
            Self::ComponentIncomplete => "component_incomplete",
            Self::CitedSourceRefMissing => "cited_source_ref_missing",
            Self::UncitedStateClaimsSource => "uncited_state_claims_source",
            Self::StableComponentRefMissing => "stable_component_ref_missing",
            Self::SubjectSummaryMissing => "subject_summary_missing",
            Self::ContinuityStateMisrepresented => "continuity_state_misrepresented",
            Self::StateExplanationMissing => "state_explanation_missing",
            Self::ContinuityNoteMissing => "continuity_note_missing",
            Self::NextSafeActionMisrepresented => "next_safe_action_misrepresented",
            Self::NextSafeActionNoteMissing => "next_safe_action_note_missing",
            Self::SourceFallbackMissing => "source_fallback_missing",
            Self::SourceFallbackRouteMissing => "source_fallback_route_missing",
            Self::UnavailableSourceStillOpens => "unavailable_source_still_opens",
            Self::ApplyPostureMisrepresented => "apply_posture_misrepresented",
            Self::ApplyBoundaryNoteMissing => "apply_boundary_note_missing",
            Self::LiveMutationWithoutPreviewApproval => "live_mutation_without_preview_approval",
            Self::SafeVerbsIncomplete => "safe_verbs_incomplete",
            Self::DispositionsMissing => "dispositions_missing",
            Self::ContinuityStateCoverageMissing => "continuity_state_coverage_missing",
            Self::ComponentFamilyCoverageMissing => "component_family_coverage_missing",
            Self::ApplyPostureCoverageMissing => "apply_posture_coverage_missing",
            Self::SourceLabelMissing => "source_label_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::PrivacyOrOfflineStateMasked => "privacy_or_offline_state_masked",
            Self::CitationSourceHidden => "citation_source_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::HiddenApplyImplied => "hidden_apply_implied",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GlanceReviewIncomplete => "glance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in learning educational-AI continuity export.
pub fn current_learning_educational_ai_continuity_export(
) -> Result<LearningEducationalAiContinuityPacket, LearningEducationalAiContinuityArtifactError> {
    let packet: LearningEducationalAiContinuityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-educational-ai-continuity-proof/support_export.json"
    )))
        .map_err(LearningEducationalAiContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LearningEducationalAiContinuityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &LearningEducationalAiContinuityPacket,
    violations: &mut Vec<LearningEducationalAiContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_REF,
        LEARNING_EDUCATIONAL_AI_CONTINUITY_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_PROGRESS_MARKER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(LearningEducationalAiContinuityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_components(
    packet: &LearningEducationalAiContinuityPacket,
    violations: &mut Vec<LearningEducationalAiContinuityViolation>,
) {
    if packet.components.is_empty() {
        violations.push(LearningEducationalAiContinuityViolation::ComponentsMissing);
        return;
    }

    let mut continuity_states: BTreeSet<LearningContinuityState> = BTreeSet::new();
    let mut component_families: BTreeSet<M5LearningComponentFamily> = BTreeSet::new();
    let mut apply_postures: BTreeSet<EducationalApplyPosture> = BTreeSet::new();

    for component in &packet.components {
        let disclosure = component.continuity_disclosure();
        let apply = component.apply_disclosure();
        continuity_states.insert(component.continuity_state);
        component_families.insert(component.component_family);
        apply_postures.insert(component.apply_posture);

        if component.component_id.trim().is_empty()
            || component.component_title.trim().is_empty()
            || component.subject_label.trim().is_empty()
            || component.fields_shown.is_empty()
            || component.surface_families.is_empty()
            || component.deployment_lines.is_empty()
            || component.consumer_surfaces.is_empty()
            || component.source_contract_refs.is_empty()
        {
            violations.push(LearningEducationalAiContinuityViolation::ComponentIncomplete);
        }
        // A reachable state must name its cited source; an unavailable state must NOT claim one.
        if disclosure.source_unavailable {
            if !component.cited_source_ref.trim().is_empty() {
                violations.push(LearningEducationalAiContinuityViolation::UncitedStateClaimsSource);
            }
        } else if component.cited_source_ref.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::CitedSourceRefMissing);
        }
        if component.stable_component_ref.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::StableComponentRefMissing);
        }
        if component.subject_summary_note.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::SubjectSummaryMissing);
        }
        if component.trust_class != disclosure.trust_class
            || component.claims_live_enrichment != disclosure.is_live
        {
            violations
                .push(LearningEducationalAiContinuityViolation::ContinuityStateMisrepresented);
        }
        if disclosure.needs_continuity_explanation
            && component.state_explanation_note.trim().is_empty()
        {
            violations.push(LearningEducationalAiContinuityViolation::StateExplanationMissing);
        }
        if component.continuity_note.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::ContinuityNoteMissing);
        }
        if component.next_safe_action != disclosure.next_safe_action {
            violations.push(LearningEducationalAiContinuityViolation::NextSafeActionMisrepresented);
        }
        if component.next_safe_action_note.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::NextSafeActionNoteMissing);
        }
        if disclosure.needs_source_fallback {
            if component.source_fallback_note.trim().is_empty() {
                violations.push(LearningEducationalAiContinuityViolation::SourceFallbackMissing);
            }
            if !component.offers_resolvable_source() {
                violations
                    .push(LearningEducationalAiContinuityViolation::SourceFallbackRouteMissing);
            }
        }
        if disclosure.source_unavailable && component.offers_resolvable_source() {
            violations.push(LearningEducationalAiContinuityViolation::UnavailableSourceStillOpens);
        }
        // The apply disposition and the live-mutation flag are derived from the posture, never
        // asserted, so no component can imply a hidden apply.
        if component.apply_disposition != apply.apply_disposition
            || component.offers_live_mutation != apply.offers_live_mutation
        {
            violations.push(LearningEducationalAiContinuityViolation::ApplyPostureMisrepresented);
        }
        if component.apply_boundary_note.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::ApplyBoundaryNoteMissing);
        }
        if component.mutates_live_without_preview_approval {
            violations
                .push(LearningEducationalAiContinuityViolation::LiveMutationWithoutPreviewApproval);
        }
        if component.source_label.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::SourceLabelMissing);
        }
        if component.scope_label.trim().is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::ScopeLabelMissing);
        }
        if !component.declares_mandatory_verbs() {
            violations.push(LearningEducationalAiContinuityViolation::SafeVerbsIncomplete);
        }
        if component.dispositions.is_empty() {
            violations.push(LearningEducationalAiContinuityViolation::DispositionsMissing);
        }
        validate_common_control(
            component.declares_mandatory_labels(),
            &component.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: component.masks_privacy_or_offline_state,
                hides_citation_source: component.hides_citation_source,
                invents_alternate_state_label: component.invents_alternate_state_label,
                implies_hidden_apply_or_mutation: component.implies_hidden_apply_or_mutation,
            },
            violations,
        );
    }

    for required in LearningContinuityState::ALL {
        if !continuity_states.contains(&required) {
            violations
                .push(LearningEducationalAiContinuityViolation::ContinuityStateCoverageMissing);
            break;
        }
    }
    for required in M5LearningComponentFamily::ALL {
        if !component_families.contains(&required) {
            violations
                .push(LearningEducationalAiContinuityViolation::ComponentFamilyCoverageMissing);
            break;
        }
    }
    for required in EducationalApplyPosture::ALL {
        if !apply_postures.contains(&required) {
            violations.push(LearningEducationalAiContinuityViolation::ApplyPostureCoverageMissing);
            break;
        }
    }
}

/// The four hard-invariant bools every control shares with the frozen matrix.
struct ControlInvariants {
    masks_privacy_or_offline_state: bool,
    hides_citation_source: bool,
    invents_alternate_state_label: bool,
    implies_hidden_apply_or_mutation: bool,
}

/// Validates the axes shared by every component.
fn validate_common_control(
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5LearningAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<LearningEducationalAiContinuityViolation>,
) {
    if !declares_mandatory_labels {
        violations.push(LearningEducationalAiContinuityViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5LearningAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(LearningEducationalAiContinuityViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_privacy_or_offline_state {
        violations.push(LearningEducationalAiContinuityViolation::PrivacyOrOfflineStateMasked);
    }
    if invariants.hides_citation_source {
        violations.push(LearningEducationalAiContinuityViolation::CitationSourceHidden);
    }
    if invariants.invents_alternate_state_label {
        violations.push(LearningEducationalAiContinuityViolation::AlternateStateLabelInvented);
    }
    if invariants.implies_hidden_apply_or_mutation {
        violations.push(LearningEducationalAiContinuityViolation::HiddenApplyImplied);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
///
/// The learning vocabulary carries no secret-value words, so this check flags only raw-*value*
/// shapes that must never cross the boundary: a password / passphrase literal, a bearer literal, a
/// URL scheme, or a PEM header.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
