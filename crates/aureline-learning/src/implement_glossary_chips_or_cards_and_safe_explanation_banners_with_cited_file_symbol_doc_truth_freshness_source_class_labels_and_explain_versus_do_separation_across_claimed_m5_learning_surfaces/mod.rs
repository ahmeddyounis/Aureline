//! Two reusable M5 learnability controls — the glossary chip or card and the safe explanation
//! banner — so a user can learn what a term means or why a result is suggested without the
//! surface ever letting educational prose drift away from cited source truth or quietly blur
//! into an apply-capable action. From the control alone a learner can tell exactly what a term
//! means, where its definition is cited from (a stable file, symbol, or docs source), how
//! current that citation is, whether an explanation only explains or also offers a governed
//! do, and — when a do is offered — that it uses the same preview / approval / undo model as
//! ordinary work. Explain and do stay visibly separate; nothing is applied by a hidden
//! authority.
//!
//! Aureline's frozen learning-component matrix
//! ([`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`])
//! names the glossary chip or card and the safe explanation banner as two governed component
//! families and freezes their controlled vocabulary — the glossary source classes
//! (`cited_docs`, `cited_spec`, `cited_help_pack`, `community_note`, `uncited_draft`,
//! `unknown_source`) and citation states (`citation_current`, `citation_versioned`,
//! `citation_stale`, `citation_cached`, `citation_offline_unavailable`, `citation_missing`) a
//! glossary control binds; the explanation boundary classes (`explain_only`,
//! `explain_then_offer_do`, `preview_required`, `approval_required`, `sandboxed_only`,
//! `no_hidden_apply`) and apply states (`no_apply`, `preview_available`, `approval_pending`,
//! `applied_with_undo`, `blocked_apply`, `mutation_declined`) a banner binds; the one
//! controlled disposition vocabulary; the surface families; the deployment lines; the consumer
//! surfaces; the accessibility routes; the required labels; and the downgrade triggers. This
//! module *implements* that contract as two co-equal control vectors so a claimed M5
//! onboarding, guided-tour, glossary, learning-mode, or inline-help surface can project a
//! glossary chip / card and a safe explanation banner that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_glossary_citation`] — takes a glossary control's frozen citation state and
//!    derives its citation class (cited-current, cited-stale, cited-cached, offline-unavailable,
//!    or uncited), whether the citation is currently trustworthy, and which note the control
//!    must carry — so a stale or missing citation can never read as current and a learner can
//!    always tell how grounded a definition is.
//! 2. [`resolve_explanation_apply`] — takes a banner's frozen apply state and derives its apply
//!    disposition (explain-only, preview-offered, approval-pending, applied-reversible, or
//!    apply-withheld), whether the banner only explains, and which note the banner must carry —
//!    so an explanation can never quietly become an unannounced mutation and any real apply
//!    stays reversible and governed.
//!
//! A single controls packet — [`GlossaryChipCardSafeExplanationBannerControlsPacket`] — binds
//! one vector of glossary chips / cards and one vector of safe explanation banners to the same
//! term-meaning / cited-source, freshness / source-class, explain-versus-do, apply-governance,
//! and non-visual accessibility vocabulary, so learnability stays grounded, explain and do stay
//! separate, and citation truth stays visible across desktop, headless / export, and support
//! consumers.
//!
//! The glossary source class ([`M5GlossarySourceClass`]), glossary citation state
//! ([`M5GlossaryCitationState`]), explanation boundary class ([`M5ExplanationBoundaryClass`]),
//! explanation apply state ([`M5ExplanationApplyState`]), disposition
//! ([`M5LearningDisposition`]), surface family ([`M5LearningSurfaceFamily`]), deployment line
//! ([`M5LearningDeploymentLine`]), consumer surface ([`M5LearningConsumerSurface`]),
//! accessibility route ([`M5LearningAccessibilityRoute`]), required label
//! ([`M5LearningRequiredLabel`]), and downgrade trigger ([`M5LearningDowngradeTrigger`]) are
//! reused verbatim from the frozen matrix. This module mints new vocabulary only for what that
//! matrix left implicit about the two controls themselves: the derived citation and apply
//! classes, the bounded glossary and banner actions, the cited file / symbol / docs deep-link
//! kinds, and the const boundary / source rules that keep explain and do separate. No M5
//! learnability surface invents a second glossary or explanation grammar.
//!
//! Raw docs bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every term meaning, citation reference, and control identity is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_glossary_chip_card_safe_explanation_banner_controls,
    seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited,
    seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only,
    GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_PACKET_ID,
};

// The glossary source classes and citation states, the explanation boundary classes and apply
// states, the disposition vocabulary, and the surface / deployment / consumer / accessibility /
// label / downgrade vocabularies are frozen once, in the learning-component matrix. This lane
// reuses them verbatim so it never invents a parallel glossary or explanation vocabulary.
pub use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5ExplanationApplyState, M5ExplanationBoundaryClass, M5GlossaryCitationState,
    M5GlossarySourceClass, M5LearningAccessibilityRoute, M5LearningComponentFamily,
    M5LearningConsumerSurface, M5LearningDeploymentLine, M5LearningDisposition,
    M5LearningDowngradeTrigger, M5LearningRequiredLabel, M5LearningSurfaceFamily,
    M5_GLOSSARY_CHIP_CARD_SCHEMA_REF, M5_LEARNING_COMPONENT_DOC_REF,
    M5_LEARNING_COMPONENT_SCHEMA_REF, M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`GlossaryChipCardSafeExplanationBannerControlsPacket`].
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_RECORD_KIND: &str =
    "implement_m5_glossary_chips_or_cards_and_safe_explanation_banners_with_cited_file_symbol_doc_truth_freshness_source_class_labels_and_explain_versus_do_separation_across_claimed_m5_learning_surfaces";

/// Schema version for M5 glossary-chip-card / safe-explanation-banner control records.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-glossary-chip-card-safe-explanation-banner-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_DOC_REF: &str =
    "docs/help/m5_glossary_chip_card_safe_explanation_banner_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-glossary-chip-card-safe-explanation-banner-controls";

/// Repo-relative path of the checked support-export artifact.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_ARTIFACT_REF: &str =
    "artifacts/release/m5-glossary-chip-card-safe-explanation-banner-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_CSV_REF: &str =
    "artifacts/release/m5-glossary-chip-card-safe-explanation-banner-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_REPORT_REF: &str =
    "artifacts/design/m5-glossary-chip-card-safe-explanation-banner.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a learning control cites against, so a glossary definition and
/// an explanation are always backed by a stable command, file, symbol, or docs source the user
/// can reopen — never free-floating prose or a hidden route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable command reference in the command system.
    CommandReference,
    /// A stable file location.
    FileLocation,
    /// A stable symbol location (module path / definition site).
    SymbolLocation,
    /// A stable docs anchor.
    DocsAnchor,
    /// No deep link is bound (the control names that it cites nothing resolvable).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommandReference,
        Self::FileLocation,
        Self::SymbolLocation,
        Self::DocsAnchor,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandReference => "command_reference",
            Self::FileLocation => "file_location",
            Self::SymbolLocation => "symbol_location",
            Self::DocsAnchor => "docs_anchor",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable cited-source target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- glossary-chip-or-card vocabulary -----------------------------------

/// Derived citation class a glossary chip or card may present.
///
/// This is the glossary honesty axis: the class is derived from the frozen citation state,
/// never asserted, so a stale, offline, or missing citation can never present as current and a
/// learner can always tell how grounded a definition is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlossaryCitationClass {
    /// Cited and current (current or version-matched).
    CitedCurrent,
    /// Cited but stale.
    CitedStale,
    /// Cited from a cached copy.
    CitedCached,
    /// Citation unavailable while offline.
    OfflineUnavailable,
    /// No citation resolves (uncited).
    Uncited,
}

impl GlossaryCitationClass {
    /// Every citation class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CitedCurrent,
        Self::CitedStale,
        Self::CitedCached,
        Self::OfflineUnavailable,
        Self::Uncited,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CitedCurrent => "cited_current",
            Self::CitedStale => "cited_stale",
            Self::CitedCached => "cited_cached",
            Self::OfflineUnavailable => "offline_unavailable",
            Self::Uncited => "uncited",
        }
    }

    /// True only when the citation is current and trustworthy.
    pub const fn is_cited_current(self) -> bool {
        matches!(self, Self::CitedCurrent)
    }
}

/// True when a glossary source class names cited source truth (product docs, a spec, or a help
/// pack), so glossary prose that claims to be source-backed can never rest on a community note,
/// an uncited draft, or an unknown source.
pub const fn source_is_cited(source: M5GlossarySourceClass) -> bool {
    matches!(
        source,
        M5GlossarySourceClass::CitedDocs
            | M5GlossarySourceClass::CitedSpec
            | M5GlossarySourceClass::CitedHelpPack
    )
}

/// One keyboard-complete default action a glossary chip or card offers. Every glossary control
/// offers `OpenRelatedConcept` so a learner can always follow the concept graph; none of these
/// actions apply, mutate, or widen authority — a glossary control only ever explains or
/// navigates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlossaryEntryAction {
    /// Show the term's definition / meaning.
    ShowDefinition,
    /// Open the cited source behind the definition.
    OpenCitation,
    /// Open a related concept (always available).
    OpenRelatedConcept,
    /// Copy the term to the clipboard.
    CopyTerm,
    /// Open the full glossary surface.
    OpenGlossarySurface,
    /// Dismiss the chip.
    DismissChip,
}

impl GlossaryEntryAction {
    /// Every glossary action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShowDefinition,
        Self::OpenCitation,
        Self::OpenRelatedConcept,
        Self::CopyTerm,
        Self::OpenGlossarySurface,
        Self::DismissChip,
    ];

    /// The default actions every keyboard-complete glossary control must offer.
    pub const MANDATORY: [Self; 1] = [Self::OpenRelatedConcept];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowDefinition => "show_definition",
            Self::OpenCitation => "open_citation",
            Self::OpenRelatedConcept => "open_related_concept",
            Self::CopyTerm => "copy_term",
            Self::OpenGlossarySurface => "open_glossary_surface",
            Self::DismissChip => "dismiss_chip",
        }
    }
}

/// Disclosures a glossary chip or card must carry, derived from the citation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlossaryEntryDisclosure {
    /// The derived citation class this control may present.
    pub citation_class: GlossaryCitationClass,
    /// Whether the citation is current and trustworthy.
    pub is_cited_current: bool,
    /// Whether the control must carry an explicit stale-citation note.
    pub needs_stale_note: bool,
    /// Whether the control must carry an explicit offline-citation note.
    pub needs_offline_note: bool,
    /// Whether the control must carry an explicit uncited / missing-citation note.
    pub needs_uncited_note: bool,
}

/// Resolves the citation truth a glossary chip or card may present.
///
/// A `citation_current` or `citation_versioned` citation is cited-current. A `citation_stale`
/// citation is cited-stale. A `citation_cached` citation is cited-cached. A
/// `citation_offline_unavailable` citation is offline-unavailable. A `citation_missing`
/// citation is uncited, so a definition that is not actually backed by a current citation can
/// never read as current.
pub fn resolve_glossary_citation(state: M5GlossaryCitationState) -> GlossaryEntryDisclosure {
    use GlossaryCitationClass as Class;
    use M5GlossaryCitationState as State;

    let citation_class = match state {
        State::CitationCurrent | State::CitationVersioned => Class::CitedCurrent,
        State::CitationStale => Class::CitedStale,
        State::CitationCached => Class::CitedCached,
        State::CitationOfflineUnavailable => Class::OfflineUnavailable,
        State::CitationMissing => Class::Uncited,
    };

    GlossaryEntryDisclosure {
        citation_class,
        is_cited_current: citation_class.is_cited_current(),
        needs_stale_note: matches!(citation_class, Class::CitedStale),
        needs_offline_note: matches!(citation_class, Class::OfflineUnavailable),
        needs_uncited_note: matches!(citation_class, Class::Uncited),
    }
}

/// A glossary chip or card naming its term meaning, cited source class / freshness, derived
/// citation class, cited file / symbol / docs source, and open-definition / open-citation /
/// open-related-concept actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryEntry {
    /// Frozen component this control implements; must be `glossary_chip_or_card`.
    pub component: M5LearningComponentFamily,
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable term label; required and non-empty.
    pub term_label: String,
    /// Glossary source class, reused from the frozen matrix.
    pub source_class: M5GlossarySourceClass,
    /// Glossary citation state, reused from the frozen matrix.
    pub citation_state: M5GlossaryCitationState,
    /// Derived citation class (must equal the resolved class).
    pub citation_class: GlossaryCitationClass,
    /// Whether the control claims its citation is current (must equal the derived truth).
    pub claims_citation_current: bool,
    /// Whether the control claims its definition rests on cited source truth (must be
    /// consistent with the source class).
    pub claims_source_backed: bool,
    /// Term meaning; always required so the control names what the term means.
    pub term_meaning: String,
    /// Stale-citation note; required when the citation is stale.
    pub stale_note: String,
    /// Offline-citation note; required when the citation is offline-unavailable.
    pub offline_note: String,
    /// Uncited / missing-citation note; required when no citation resolves.
    pub citation_missing_note: String,
    /// Source-backing note; required when the source class is not cited source truth so an
    /// uncited or community definition discloses that it is not cited.
    pub source_backing_note: String,
    /// Kind of stable cited source this definition points at.
    pub citation_kind: DeepLinkKind,
    /// Opaque stable cited-source reference; required when the kind resolves.
    pub citation_ref: String,
    /// Human-readable label of the cited source; always required.
    pub citation_label: String,
    /// Keyboard-complete default actions (must include the mandatory `OpenRelatedConcept`).
    pub entry_actions: Vec<GlossaryEntryAction>,
    /// Dispositions this control binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this control can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this control can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this control.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this control keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this control offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this control's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this control.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides the cited source or its freshness. MUST be `false`.
    pub hides_citation_source_or_freshness: bool,
    /// Hard invariant: never implies an apply-capable action or hidden authority. MUST be
    /// `false`.
    pub implies_apply_capable_action_or_hidden_authority: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never lets prose drift away from cited source truth. MUST be `false`.
    pub drifts_prose_from_cited_source_truth: bool,
}

impl GlossaryEntry {
    /// Citation disclosures this control must carry, derived from the citation state.
    pub fn citation_disclosure(&self) -> GlossaryEntryDisclosure {
        resolve_glossary_citation(self.citation_state)
    }

    /// Whether the control offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<GlossaryEntryAction> = self.entry_actions.iter().copied().collect();
        GlossaryEntryAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the control declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the control offers an open-citation action.
    fn offers_citation_action(&self) -> bool {
        self.entry_actions
            .contains(&GlossaryEntryAction::OpenCitation)
    }
}

// ---- safe-explanation-banner vocabulary ---------------------------------

/// Derived apply disposition a safe explanation banner may present.
///
/// This is the explain-versus-do honesty axis: the disposition is derived from the frozen apply
/// state, never asserted, so an explanation can never quietly become an unannounced mutation
/// and a learner can always tell whether the banner only explains or also offers a governed do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationApplyDisposition {
    /// Explains only; applies nothing.
    ExplainOnly,
    /// A preview of a change is offered.
    PreviewOffered,
    /// Approval of a change is pending.
    ApprovalPending,
    /// A change was applied and is reversible with undo.
    AppliedReversible,
    /// A change was withheld (blocked or declined) — nothing was applied.
    ApplyWithheld,
}

impl ExplanationApplyDisposition {
    /// Every apply disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExplainOnly,
        Self::PreviewOffered,
        Self::ApprovalPending,
        Self::AppliedReversible,
        Self::ApplyWithheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::PreviewOffered => "preview_offered",
            Self::ApprovalPending => "approval_pending",
            Self::AppliedReversible => "applied_reversible",
            Self::ApplyWithheld => "apply_withheld",
        }
    }

    /// True only when the banner explains without offering or performing any apply.
    pub const fn is_explain_only(self) -> bool {
        matches!(self, Self::ExplainOnly)
    }
}

/// True when an explanation boundary class permits offering a governed do at all, so an
/// `explain_only` banner can never offer or perform an apply — every other boundary carries an
/// explicit preview / approval / sandbox / no-hidden-apply model.
pub const fn boundary_permits_apply(boundary: M5ExplanationBoundaryClass) -> bool {
    !matches!(boundary, M5ExplanationBoundaryClass::ExplainOnly)
}

/// One keyboard-complete default action a safe explanation banner offers. Every banner offers
/// `ShowExplanation` so a learner can always read why. `PreviewChange` and `RequestApproval`
/// are the only do-adjacent actions, and both stay behind the ordinary preview / approval
/// model; a banner never applies by a hidden authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationBannerAction {
    /// Show the grounded explanation (always available).
    ShowExplanation,
    /// Open the cited source behind the explanation.
    OpenCitation,
    /// Open a related concept.
    OpenRelatedConcept,
    /// Preview a proposed change before doing it.
    PreviewChange,
    /// Request approval before a change is applied.
    RequestApproval,
    /// Dismiss the banner.
    DismissBanner,
}

impl ExplanationBannerAction {
    /// Every banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShowExplanation,
        Self::OpenCitation,
        Self::OpenRelatedConcept,
        Self::PreviewChange,
        Self::RequestApproval,
        Self::DismissBanner,
    ];

    /// The default actions every keyboard-complete banner must offer.
    pub const MANDATORY: [Self; 1] = [Self::ShowExplanation];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowExplanation => "show_explanation",
            Self::OpenCitation => "open_citation",
            Self::OpenRelatedConcept => "open_related_concept",
            Self::PreviewChange => "preview_change",
            Self::RequestApproval => "request_approval",
            Self::DismissBanner => "dismiss_banner",
        }
    }

    /// True when this action offers or requests an apply / do.
    pub const fn is_apply_action(self) -> bool {
        matches!(self, Self::PreviewChange | Self::RequestApproval)
    }
}

/// Disclosures a safe explanation banner must carry, derived from the apply state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplanationBannerDisclosure {
    /// The derived apply disposition this banner may present.
    pub apply_disposition: ExplanationApplyDisposition,
    /// Whether the banner only explains.
    pub is_explain_only: bool,
    /// Whether the banner must carry an explicit applied-with-undo note.
    pub needs_undo_note: bool,
    /// Whether the banner must carry an explicit apply-withheld note.
    pub needs_withheld_note: bool,
}

/// Resolves the apply truth a safe explanation banner may present.
///
/// A `no_apply` banner explains only. A `preview_available` banner offers a preview. An
/// `approval_pending` banner has approval pending. An `applied_with_undo` banner applied a
/// reversible change. A `blocked_apply` or `mutation_declined` banner withheld the apply, so an
/// explanation that has not actually applied anything can never read as having done so.
pub fn resolve_explanation_apply(state: M5ExplanationApplyState) -> ExplanationBannerDisclosure {
    use ExplanationApplyDisposition as Disp;
    use M5ExplanationApplyState as State;

    let apply_disposition = match state {
        State::NoApply => Disp::ExplainOnly,
        State::PreviewAvailable => Disp::PreviewOffered,
        State::ApprovalPending => Disp::ApprovalPending,
        State::AppliedWithUndo => Disp::AppliedReversible,
        State::BlockedApply | State::MutationDeclined => Disp::ApplyWithheld,
    };

    ExplanationBannerDisclosure {
        apply_disposition,
        is_explain_only: apply_disposition.is_explain_only(),
        needs_undo_note: matches!(apply_disposition, Disp::AppliedReversible),
        needs_withheld_note: matches!(apply_disposition, Disp::ApplyWithheld),
    }
}

/// A safe explanation banner naming its grounded explanation, cited source, explain-versus-do
/// boundary, derived apply disposition, and — when a do is offered — its governed preview /
/// approval / undo model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationBanner {
    /// Frozen component this control implements; must be `safe_explanation_banner`.
    pub component: M5LearningComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// Human-readable banner label; required and non-empty.
    pub banner_label: String,
    /// Explanation boundary class, reused from the frozen matrix.
    pub boundary_class: M5ExplanationBoundaryClass,
    /// Explanation apply state, reused from the frozen matrix.
    pub apply_state: M5ExplanationApplyState,
    /// Derived apply disposition (must equal the resolved disposition).
    pub apply_disposition: ExplanationApplyDisposition,
    /// Whether the banner claims it only explains (must equal the derived truth).
    pub claims_explain_only: bool,
    /// Grounded explanation body ("what this term means" / "why this result is suggested");
    /// always required so the explanation is grounded, not omniscient.
    pub explanation_body: String,
    /// Explain-versus-do note; always required so the boundary between explaining and doing
    /// stays explicit.
    pub explain_versus_do_note: String,
    /// Applied-with-undo note; required when a reversible change was applied.
    pub undo_note: String,
    /// Apply-withheld note; required when a change was blocked or declined.
    pub withheld_note: String,
    /// Whether this banner offers a governed do (preview / approval) alongside its explanation.
    pub offers_do_action: bool,
    /// Do-disclosure note; required when the banner offers a do so the do stays announced.
    pub do_disclosure_note: String,
    /// Kind of stable cited source that grounds this explanation.
    pub citation_kind: DeepLinkKind,
    /// Opaque stable cited-source reference; required when the kind resolves.
    pub citation_ref: String,
    /// Human-readable label of the cited source; always required.
    pub citation_label: String,
    /// Keyboard-complete default actions (must include the mandatory `ShowExplanation`).
    pub banner_actions: Vec<ExplanationBannerAction>,
    /// Dispositions this control binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this control can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this control can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this control.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this control keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this control offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this control's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this control.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides the cited source or its freshness. MUST be `false`.
    pub hides_citation_source_or_freshness: bool,
    /// Hard invariant: never implies an apply-capable action or hidden authority. MUST be
    /// `false`.
    pub implies_apply_capable_action_or_hidden_authority: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never lets prose drift away from cited source truth. MUST be `false`.
    pub drifts_prose_from_cited_source_truth: bool,
}

impl ExplanationBanner {
    /// Apply disclosures this control must carry, derived from the apply state.
    pub fn apply_disclosure(&self) -> ExplanationBannerDisclosure {
        resolve_explanation_apply(self.apply_state)
    }

    /// Whether the control offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ExplanationBannerAction> =
            self.banner_actions.iter().copied().collect();
        ExplanationBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the control declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the control offers an open-citation action.
    fn offers_citation_action(&self) -> bool {
        self.banner_actions
            .contains(&ExplanationBannerAction::OpenCitation)
    }

    /// Whether the control offers any do-adjacent (preview / approval) action.
    fn offers_apply_action(&self) -> bool {
        self.banner_actions
            .iter()
            .any(|action| action.is_apply_action())
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance learnability review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryExplanationReview {
    /// The glossary control cites source truth instead of free-floating prose.
    pub glossary_cites_source_truth: bool,
    /// The glossary control names the term meaning.
    pub glossary_names_term_meaning: bool,
    /// The glossary control shows its source class and citation freshness.
    pub glossary_shows_source_class_and_freshness: bool,
    /// The glossary control offers an open-related-concept action.
    pub glossary_offers_open_related_concept: bool,
    /// Citation freshness is derived from state, never asserted.
    pub citation_freshness_derived_never_asserted: bool,
    /// Uncited prose is never shown as cited.
    pub uncited_prose_never_shown_as_cited: bool,
    /// The banner states its explain-versus-do boundary.
    pub explanation_states_explain_versus_do_boundary: bool,
    /// The banner cites the grounding source behind its explanation.
    pub explanation_cites_grounding_source: bool,
    /// The banner never implies an apply-capable action from an explanation.
    pub explanation_never_implies_apply_capable_action: bool,
    /// The banner's apply disposition is derived from state, never asserted.
    pub apply_disposition_derived_never_asserted: bool,
    /// An explain-only banner offers no do action.
    pub explain_only_banner_offers_no_do_action: bool,
    /// Any apply uses the ordinary preview / approval / undo model.
    pub any_apply_uses_preview_approval_or_undo: bool,
    /// No control widens trust or mutating authority.
    pub no_control_widens_trust_or_mutating_authority: bool,
    /// Educational surfaces stay visibly distinct from apply-capable actions.
    pub educational_surfaces_distinct_from_apply_capable_actions: bool,
    /// Cached, offline, and local-only state stays visible.
    pub cached_offline_local_only_state_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
}

impl GlossaryExplanationReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.glossary_cites_source_truth
            && self.glossary_names_term_meaning
            && self.glossary_shows_source_class_and_freshness
            && self.glossary_offers_open_related_concept
            && self.citation_freshness_derived_never_asserted
            && self.uncited_prose_never_shown_as_cited
            && self.explanation_states_explain_versus_do_boundary
            && self.explanation_cites_grounding_source
            && self.explanation_never_implies_apply_capable_action
            && self.apply_disposition_derived_never_asserted
            && self.explain_only_banner_offers_no_do_action
            && self.any_apply_uses_preview_approval_or_undo
            && self.no_control_widens_trust_or_mutating_authority
            && self.educational_surfaces_distinct_from_apply_capable_actions
            && self.cached_offline_local_only_state_visible
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryExplanationConsumerProjection {
    /// The glossary surface reads a single canonical source.
    pub glossary_ui_reads_single_source: bool,
    /// The explanation surface reads a single canonical source.
    pub explanation_ui_reads_single_source: bool,
    /// The cited source and citation freshness are visible before a learner trusts a definition.
    pub citation_source_and_freshness_visible_before_trust: bool,
    /// The explain-versus-do boundary is visible before a tap.
    pub explain_versus_do_boundary_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl GlossaryExplanationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.glossary_ui_reads_single_source
            && self.explanation_ui_reads_single_source
            && self.citation_source_and_freshness_visible_before_trust
            && self.explain_versus_do_boundary_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryExplanationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GlossaryChipCardSafeExplanationBannerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlossaryChipCardSafeExplanationBannerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Glossary chips / cards.
    pub glossary_entries: Vec<GlossaryEntry>,
    /// Safe explanation banners.
    pub explanation_banners: Vec<ExplanationBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: GlossaryExplanationReview,
    /// Consumer projection block.
    pub consumer_projection: GlossaryExplanationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GlossaryExplanationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe glossary-chip-card / safe-explanation-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryChipCardSafeExplanationBannerControlsPacket {
    /// Record kind; must equal [`GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Glossary chips / cards.
    pub glossary_entries: Vec<GlossaryEntry>,
    /// Safe explanation banners.
    pub explanation_banners: Vec<ExplanationBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: GlossaryExplanationReview,
    /// Consumer projection block.
    pub consumer_projection: GlossaryExplanationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GlossaryExplanationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GlossaryChipCardSafeExplanationBannerControlsPacket {
    /// Builds a glossary-chip-card / safe-explanation-banner controls packet from stable-lane
    /// input.
    pub fn new(input: GlossaryChipCardSafeExplanationBannerControlsPacketInput) -> Self {
        Self {
            record_kind: GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_RECORD_KIND.to_owned(),
            schema_version: GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            glossary_entries: input.glossary_entries,
            explanation_banners: input.explanation_banners,
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

    /// Validates the glossary-chip-card / safe-explanation-banner control invariants.
    pub fn validate(&self) -> Vec<GlossaryChipCardSafeExplanationBannerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_RECORD_KIND {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::WrongRecordKind);
        }
        if self.schema_version != GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_VERSION {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_glossary_entries(self, &mut violations);
        validate_explanation_banners(self, &mut violations);

        if !self.learnability_review.all_hold() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::LearnabilityReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("glossary chip card safe explanation banner packet serializes"),
        ) {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::RawMaterialInExport);
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
            .expect("glossary chip card safe explanation banner packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,source_or_boundary,state,derived,cited_or_explain_only,deep_link_kind\n",
        );
        for entry in &self.glossary_entries {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "glossary_chip_or_card",
                csv_field(&entry.entry_id),
                entry.source_class.as_str(),
                entry.citation_state.as_str(),
                entry.citation_disclosure().citation_class.as_str(),
                entry.citation_disclosure().is_cited_current,
                entry.citation_kind.as_str(),
            ));
        }
        for banner in &self.explanation_banners {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "safe_explanation_banner",
                csv_field(&banner.banner_id),
                banner.boundary_class.as_str(),
                banner.apply_state.as_str(),
                banner.apply_disclosure().apply_disposition.as_str(),
                banner.apply_disclosure().is_explain_only,
                banner.citation_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let uncited = self
            .glossary_entries
            .iter()
            .filter(|entry| !entry.citation_disclosure().is_cited_current)
            .count();
        let explain_only = self
            .explanation_banners
            .iter()
            .filter(|banner| banner.apply_disclosure().is_explain_only)
            .count();

        let mut out = String::new();
        out.push_str("# Glossary chips/cards and safe explanation banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Glossary chips/cards: {} ({} not cited-current)\n",
            self.glossary_entries.len(),
            uncited
        ));
        out.push_str(&format!(
            "- Safe explanation banners: {} ({} explain-only)\n",
            self.explanation_banners.len(),
            explain_only
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Glossary chips and cards\n\n");
        for entry in &self.glossary_entries {
            out.push_str(&format!(
                "- **{}** — source `{}`, citation `{}` → `{}`, cited via `{}`\n",
                entry.term_label,
                entry.source_class.as_str(),
                entry.citation_state.as_str(),
                entry.citation_disclosure().citation_class.as_str(),
                entry.citation_kind.as_str(),
            ));
        }

        out.push_str("\n## Safe explanation banners\n\n");
        for banner in &self.explanation_banners {
            out.push_str(&format!(
                "- **{}** — boundary `{}`, apply `{}` → `{}`, offers-do {}\n",
                banner.banner_label,
                banner.boundary_class.as_str(),
                banner.apply_state.as_str(),
                banner.apply_disclosure().apply_disposition.as_str(),
                banner.offers_do_action,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in glossary / banner export.
#[derive(Debug)]
pub enum GlossaryChipCardSafeExplanationBannerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GlossaryChipCardSafeExplanationBannerViolation>),
}

impl fmt::Display for GlossaryChipCardSafeExplanationBannerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "glossary chip card safe explanation banner export parse failed: {error}"
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
                    "glossary chip card safe explanation banner export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GlossaryChipCardSafeExplanationBannerArtifactError {}

/// Validation failures emitted by
/// [`GlossaryChipCardSafeExplanationBannerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlossaryChipCardSafeExplanationBannerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No glossary chips / cards are present.
    GlossaryEntriesMissing,
    /// A glossary chip / card is incomplete.
    GlossaryEntryIncomplete,
    /// A glossary chip / card carries the wrong frozen component class.
    GlossaryEntryWrongComponentClass,
    /// A glossary chip / card misrepresents its derived citation class.
    GlossaryCitationClassMisrepresented,
    /// A stale-citation glossary control does not name its stale state.
    GlossaryStaleNoteMissing,
    /// An offline-citation glossary control does not name its offline state.
    GlossaryOfflineNoteMissing,
    /// An uncited glossary control does not name its missing-citation state.
    GlossaryCitationMissingNoteMissing,
    /// A glossary control drawing on an uncited source does not disclose that it is not cited.
    GlossarySourceBackingNoteMissing,
    /// A glossary control claims to rest on cited source truth but its source is uncited.
    GlossarySourceBackedClaimUnsupported,
    /// A glossary control does not name the term meaning.
    GlossaryTermMeaningMissing,
    /// A glossary control does not name the label of its cited source.
    GlossaryCitationLabelMissing,
    /// A cited glossary control does not offer an open-citation action.
    GlossaryCitedWithoutOpenCitation,
    /// A glossary control offers an open-citation action but its citation does not resolve.
    GlossaryCitationUnresolved,
    /// A glossary control names a citation kind but not its stable reference.
    GlossaryCitationRefMissing,
    /// A glossary control omits the mandatory `OpenRelatedConcept` action.
    GlossaryActionsIncomplete,
    /// The glossary controls do not cover every derived citation class.
    GlossaryCitationClassCoverageMissing,
    /// The glossary controls do not cover every glossary source class.
    GlossarySourceClassCoverageMissing,
    /// The glossary controls do not cover every glossary citation state.
    GlossaryCitationStateCoverageMissing,
    /// No safe explanation banners are present.
    ExplanationBannersMissing,
    /// A safe explanation banner is incomplete.
    ExplanationBannerIncomplete,
    /// A safe explanation banner carries the wrong frozen component class.
    ExplanationBannerWrongComponentClass,
    /// A safe explanation banner misrepresents its derived apply disposition.
    ApplyDispositionMisrepresented,
    /// An applied banner does not name its undo state.
    UndoNoteMissing,
    /// A withheld banner does not name its withheld state.
    WithheldNoteMissing,
    /// A banner does not name its grounded explanation body.
    ExplanationBodyMissing,
    /// A banner does not name its explain-versus-do boundary.
    ExplainVersusDoNoteMissing,
    /// A banner does not name the label of its cited grounding source.
    BannerCitationLabelMissing,
    /// A banner offers an open-citation action but its citation does not resolve.
    BannerCitationUnresolved,
    /// A banner names a citation kind but not its stable reference.
    BannerCitationRefMissing,
    /// An explain-only banner offers a do / apply action.
    ExplainOnlyBannerOffersDoAction,
    /// A banner presents an apply state its boundary does not permit.
    ApplyStateBeyondBoundary,
    /// A banner offers a do but does not disclose it.
    DoDisclosureNoteMissing,
    /// A banner carries a do-adjacent action but does not declare that it offers a do.
    ApplyActionWithoutDoDisclosure,
    /// A banner omits the mandatory `ShowExplanation` action.
    ExplanationBannerActionsIncomplete,
    /// The banners do not cover every derived apply disposition.
    ExplanationApplyDispositionCoverageMissing,
    /// The banners do not cover every explanation boundary class.
    ExplanationBoundaryClassCoverageMissing,
    /// The banners do not cover every explanation apply state.
    ExplanationApplyStateCoverageMissing,
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
    /// A control hides its cited source or citation freshness.
    CitationSourceOrFreshnessHidden,
    /// A control implies an apply-capable action or hidden authority.
    ApplyCapableActionOrHiddenAuthorityImplied,
    /// A control invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A control lets prose drift away from cited source truth.
    ProseDriftsFromCitedSourceTruth,
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

impl GlossaryChipCardSafeExplanationBannerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::GlossaryEntriesMissing => "glossary_entries_missing",
            Self::GlossaryEntryIncomplete => "glossary_entry_incomplete",
            Self::GlossaryEntryWrongComponentClass => "glossary_entry_wrong_component_class",
            Self::GlossaryCitationClassMisrepresented => "glossary_citation_class_misrepresented",
            Self::GlossaryStaleNoteMissing => "glossary_stale_note_missing",
            Self::GlossaryOfflineNoteMissing => "glossary_offline_note_missing",
            Self::GlossaryCitationMissingNoteMissing => "glossary_citation_missing_note_missing",
            Self::GlossarySourceBackingNoteMissing => "glossary_source_backing_note_missing",
            Self::GlossarySourceBackedClaimUnsupported => {
                "glossary_source_backed_claim_unsupported"
            }
            Self::GlossaryTermMeaningMissing => "glossary_term_meaning_missing",
            Self::GlossaryCitationLabelMissing => "glossary_citation_label_missing",
            Self::GlossaryCitedWithoutOpenCitation => "glossary_cited_without_open_citation",
            Self::GlossaryCitationUnresolved => "glossary_citation_unresolved",
            Self::GlossaryCitationRefMissing => "glossary_citation_ref_missing",
            Self::GlossaryActionsIncomplete => "glossary_actions_incomplete",
            Self::GlossaryCitationClassCoverageMissing => {
                "glossary_citation_class_coverage_missing"
            }
            Self::GlossarySourceClassCoverageMissing => "glossary_source_class_coverage_missing",
            Self::GlossaryCitationStateCoverageMissing => {
                "glossary_citation_state_coverage_missing"
            }
            Self::ExplanationBannersMissing => "explanation_banners_missing",
            Self::ExplanationBannerIncomplete => "explanation_banner_incomplete",
            Self::ExplanationBannerWrongComponentClass => {
                "explanation_banner_wrong_component_class"
            }
            Self::ApplyDispositionMisrepresented => "apply_disposition_misrepresented",
            Self::UndoNoteMissing => "undo_note_missing",
            Self::WithheldNoteMissing => "withheld_note_missing",
            Self::ExplanationBodyMissing => "explanation_body_missing",
            Self::ExplainVersusDoNoteMissing => "explain_versus_do_note_missing",
            Self::BannerCitationLabelMissing => "banner_citation_label_missing",
            Self::BannerCitationUnresolved => "banner_citation_unresolved",
            Self::BannerCitationRefMissing => "banner_citation_ref_missing",
            Self::ExplainOnlyBannerOffersDoAction => "explain_only_banner_offers_do_action",
            Self::ApplyStateBeyondBoundary => "apply_state_beyond_boundary",
            Self::DoDisclosureNoteMissing => "do_disclosure_note_missing",
            Self::ApplyActionWithoutDoDisclosure => "apply_action_without_do_disclosure",
            Self::ExplanationBannerActionsIncomplete => "explanation_banner_actions_incomplete",
            Self::ExplanationApplyDispositionCoverageMissing => {
                "explanation_apply_disposition_coverage_missing"
            }
            Self::ExplanationBoundaryClassCoverageMissing => {
                "explanation_boundary_class_coverage_missing"
            }
            Self::ExplanationApplyStateCoverageMissing => {
                "explanation_apply_state_coverage_missing"
            }
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::PrivacyOrOfflineStateMasked => "privacy_or_offline_state_masked",
            Self::CitationSourceOrFreshnessHidden => "citation_source_or_freshness_hidden",
            Self::ApplyCapableActionOrHiddenAuthorityImplied => {
                "apply_capable_action_or_hidden_authority_implied"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProseDriftsFromCitedSourceTruth => "prose_drifts_from_cited_source_truth",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LearnabilityReviewIncomplete => "learnability_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable glossary / banner export.
pub fn current_glossary_chip_card_safe_explanation_banner_export() -> Result<
    GlossaryChipCardSafeExplanationBannerControlsPacket,
    GlossaryChipCardSafeExplanationBannerArtifactError,
> {
    let packet: GlossaryChipCardSafeExplanationBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-glossary-chip-card-safe-explanation-banner-proof/support_export.json"
        )))
        .map_err(GlossaryChipCardSafeExplanationBannerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GlossaryChipCardSafeExplanationBannerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &GlossaryChipCardSafeExplanationBannerControlsPacket,
    violations: &mut Vec<GlossaryChipCardSafeExplanationBannerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_GLOSSARY_CHIP_CARD_SCHEMA_REF,
        M5_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_glossary_entries(
    packet: &GlossaryChipCardSafeExplanationBannerControlsPacket,
    violations: &mut Vec<GlossaryChipCardSafeExplanationBannerViolation>,
) {
    if packet.glossary_entries.is_empty() {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryEntriesMissing);
        return;
    }

    let mut citation_classes: BTreeSet<GlossaryCitationClass> = BTreeSet::new();
    let mut sources: BTreeSet<M5GlossarySourceClass> = BTreeSet::new();
    let mut states: BTreeSet<M5GlossaryCitationState> = BTreeSet::new();

    for entry in &packet.glossary_entries {
        let disclosure = entry.citation_disclosure();
        citation_classes.insert(disclosure.citation_class);
        sources.insert(entry.source_class);
        states.insert(entry.citation_state);

        if entry.entry_id.trim().is_empty()
            || entry.term_label.trim().is_empty()
            || entry.fields_shown.is_empty()
            || entry.surface_families.is_empty()
            || entry.deployment_lines.is_empty()
            || entry.consumer_surfaces.is_empty()
            || entry.source_contract_refs.is_empty()
        {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryEntryIncomplete);
        }
        if entry.component != M5LearningComponentFamily::GlossaryChipOrCard {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryEntryWrongComponentClass,
            );
        }
        if entry.citation_class != disclosure.citation_class
            || entry.claims_citation_current != disclosure.is_cited_current
        {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationClassMisrepresented,
            );
        }
        if disclosure.needs_stale_note && entry.stale_note.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryStaleNoteMissing);
        }
        if disclosure.needs_offline_note && entry.offline_note.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryOfflineNoteMissing);
        }
        if disclosure.needs_uncited_note && entry.citation_missing_note.trim().is_empty() {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationMissingNoteMissing,
            );
        }
        if !source_is_cited(entry.source_class) && entry.source_backing_note.trim().is_empty() {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossarySourceBackingNoteMissing,
            );
        }
        if entry.claims_source_backed && !source_is_cited(entry.source_class) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossarySourceBackedClaimUnsupported,
            );
        }
        if entry.term_meaning.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryTermMeaningMissing);
        }
        if entry.citation_label.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationLabelMissing);
        }
        if source_is_cited(entry.source_class) && !entry.offers_citation_action() {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitedWithoutOpenCitation,
            );
        }
        if entry.offers_citation_action() && !entry.citation_kind.is_resolvable() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationUnresolved);
        }
        if entry.citation_kind.is_resolvable() && entry.citation_ref.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationRefMissing);
        }
        if !entry.declares_mandatory_actions() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::GlossaryActionsIncomplete);
        }
        validate_common_control(
            &entry.dispositions,
            &entry.downgrade_triggers,
            entry.declares_mandatory_labels(),
            &entry.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: entry.masks_privacy_or_offline_state,
                hides_citation_source_or_freshness: entry.hides_citation_source_or_freshness,
                implies_apply_capable_action_or_hidden_authority: entry
                    .implies_apply_capable_action_or_hidden_authority,
                invents_alternate_state_label: entry.invents_alternate_state_label,
                drifts_prose_from_cited_source_truth: entry.drifts_prose_from_cited_source_truth,
            },
            violations,
        );
    }

    for required in GlossaryCitationClass::ALL {
        if !citation_classes.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5GlossarySourceClass::ALL {
        if !sources.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossarySourceClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5GlossaryCitationState::ALL {
        if !states.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_explanation_banners(
    packet: &GlossaryChipCardSafeExplanationBannerControlsPacket,
    violations: &mut Vec<GlossaryChipCardSafeExplanationBannerViolation>,
) {
    if packet.explanation_banners.is_empty() {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannersMissing);
        return;
    }

    let mut dispositions: BTreeSet<ExplanationApplyDisposition> = BTreeSet::new();
    let mut boundaries: BTreeSet<M5ExplanationBoundaryClass> = BTreeSet::new();
    let mut states: BTreeSet<M5ExplanationApplyState> = BTreeSet::new();

    for banner in &packet.explanation_banners {
        let disclosure = banner.apply_disclosure();
        dispositions.insert(disclosure.apply_disposition);
        boundaries.insert(banner.boundary_class);
        states.insert(banner.apply_state);

        if banner.banner_id.trim().is_empty()
            || banner.banner_label.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannerIncomplete);
        }
        if banner.component != M5LearningComponentFamily::SafeExplanationBanner {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannerWrongComponentClass,
            );
        }
        if banner.apply_disposition != disclosure.apply_disposition
            || banner.claims_explain_only != disclosure.is_explain_only
        {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ApplyDispositionMisrepresented,
            );
        }
        if disclosure.needs_undo_note && banner.undo_note.trim().is_empty() {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::UndoNoteMissing);
        }
        if disclosure.needs_withheld_note && banner.withheld_note.trim().is_empty() {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::WithheldNoteMissing);
        }
        if banner.explanation_body.trim().is_empty() {
            violations.push(GlossaryChipCardSafeExplanationBannerViolation::ExplanationBodyMissing);
        }
        if banner.explain_versus_do_note.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ExplainVersusDoNoteMissing);
        }
        if banner.citation_label.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::BannerCitationLabelMissing);
        }
        if banner.offers_citation_action() && !banner.citation_kind.is_resolvable() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::BannerCitationUnresolved);
        }
        if banner.citation_kind.is_resolvable() && banner.citation_ref.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::BannerCitationRefMissing);
        }
        // Explain-versus-do separation: an explain-only boundary never offers a do action, and
        // an apply state can never run ahead of what the boundary permits.
        if !boundary_permits_apply(banner.boundary_class) && banner.offers_apply_action() {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplainOnlyBannerOffersDoAction,
            );
        }
        if !disclosure.is_explain_only && !boundary_permits_apply(banner.boundary_class) {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::ApplyStateBeyondBoundary);
        }
        if banner.offers_do_action && banner.do_disclosure_note.trim().is_empty() {
            violations
                .push(GlossaryChipCardSafeExplanationBannerViolation::DoDisclosureNoteMissing);
        }
        if banner.offers_apply_action() && !banner.offers_do_action {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ApplyActionWithoutDoDisclosure,
            );
        }
        if !banner.declares_mandatory_actions() {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannerActionsIncomplete,
            );
        }
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: banner.masks_privacy_or_offline_state,
                hides_citation_source_or_freshness: banner.hides_citation_source_or_freshness,
                implies_apply_capable_action_or_hidden_authority: banner
                    .implies_apply_capable_action_or_hidden_authority,
                invents_alternate_state_label: banner.invents_alternate_state_label,
                drifts_prose_from_cited_source_truth: banner.drifts_prose_from_cited_source_truth,
            },
            violations,
        );
    }

    for required in ExplanationApplyDisposition::ALL {
        if !dispositions.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplanationApplyDispositionCoverageMissing,
            );
            break;
        }
    }
    for required in M5ExplanationBoundaryClass::ALL {
        if !boundaries.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplanationBoundaryClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5ExplanationApplyState::ALL {
        if !states.contains(&required) {
            violations.push(
                GlossaryChipCardSafeExplanationBannerViolation::ExplanationApplyStateCoverageMissing,
            );
            break;
        }
    }
}

/// The five hard-invariant bools every control must keep `false`.
struct ControlInvariants {
    masks_privacy_or_offline_state: bool,
    hides_citation_source_or_freshness: bool,
    implies_apply_capable_action_or_hidden_authority: bool,
    invents_alternate_state_label: bool,
    drifts_prose_from_cited_source_truth: bool,
}

/// Validates the axes shared by both control vectors.
fn validate_common_control(
    dispositions: &[M5LearningDisposition],
    downgrade_triggers: &[M5LearningDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5LearningAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<GlossaryChipCardSafeExplanationBannerViolation>,
) {
    if dispositions.is_empty() {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5LearningAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(GlossaryChipCardSafeExplanationBannerViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_privacy_or_offline_state {
        violations
            .push(GlossaryChipCardSafeExplanationBannerViolation::PrivacyOrOfflineStateMasked);
    }
    if invariants.hides_citation_source_or_freshness {
        violations
            .push(GlossaryChipCardSafeExplanationBannerViolation::CitationSourceOrFreshnessHidden);
    }
    if invariants.implies_apply_capable_action_or_hidden_authority {
        violations.push(
            GlossaryChipCardSafeExplanationBannerViolation::ApplyCapableActionOrHiddenAuthorityImplied,
        );
    }
    if invariants.invents_alternate_state_label {
        violations
            .push(GlossaryChipCardSafeExplanationBannerViolation::AlternateStateLabelInvented);
    }
    if invariants.drifts_prose_from_cited_source_truth {
        violations
            .push(GlossaryChipCardSafeExplanationBannerViolation::ProseDriftsFromCitedSourceTruth);
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
