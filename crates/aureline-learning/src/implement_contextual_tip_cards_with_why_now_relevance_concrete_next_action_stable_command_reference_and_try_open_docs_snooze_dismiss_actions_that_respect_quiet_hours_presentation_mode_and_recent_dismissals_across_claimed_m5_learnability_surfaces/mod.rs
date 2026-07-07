//! One reusable M5 learnability primitive — the contextual tip card — so a user can learn a
//! nearby action in place, from the card alone, without ever leaving the task or reopening a
//! detached tutorial.
//!
//! Aureline's frozen contextual-teaching / migration-bridge component matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`])
//! names the contextual tip card as one governed component family and freezes its controlled
//! vocabulary — the tip trigger classes (`first_encounter`, `feature_discovery`,
//! `error_recovery`, `mode_change`, `idle_hint`, `contextual_followup`), the dismissal states
//! (`dismissible`, `dismissed`, `snoozed`, `persistent_until_acted`, `auto_expired`,
//! `suppressed_by_preference`), and the command-backing states a tip binds — plus the
//! surface families, the deployment lines, the consumer surfaces, the accessibility routes,
//! the qualification classes, and the downgrade triggers. This module *implements* that
//! contract as one reusable resolver so a user can tell — from the tip card alone — why the
//! tip is relevant *now*, exactly what concrete next action it teaches, which stable command
//! backs that action, and whether the tip is being delivered, snoozed, or withheld, without
//! ever hijacking the workflow, spamming a recently dismissed tip, ignoring quiet hours or
//! presentation mode, or bypassing the trust / approval limits of the underlying action.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_contextual_tip_card`] — takes one tip's trigger class, command-backing state,
//!    dismissal state, the live delivery context (quiet hours, presentation mode, whether a
//!    like tip was recently dismissed, whether the underlying action requires approval), its
//!    opaque why-now relevance, its opaque stable next-action command reference, and its
//!    opaque stable tip identity, and produces one [`M5ResolvedContextualTipCard`] carrying
//!    the derived delivery posture (delivered-actionable, delivered-informational,
//!    snoozed-for-later, or withheld for quiet hours / presentation mode / a recent
//!    dismissal), the bounded try / request-approval / open-docs / snooze / dismiss actions,
//!    and whether the tip is command-backed, delivered, and reversible. It never masks the
//!    stable command reference, never turns a tip into a blocking tour, never ignores quiet
//!    hours, presentation mode, or a recent dismissal, and never lets the "try it" action
//!    bypass the same trust / approval limits that gate the underlying action.
//!
//! A single parity matrix — [`M5ContextualTipCardPacket`] — binds one row per claimed M5
//! learnability consumer (the first-run onboarding panel, the guided-tour overlay, the
//! command-palette hint, the inline editor tip, and the support tip export) to the shared
//! tip-card anatomy, the same trigger classes, command-backing states, dismissal states,
//! delivery postures, bounded actions, export fields, and non-visual accessibility routes, so
//! the why-now / next-action / command-reference / delivery vocabulary stays identical across
//! desktop, headless/export, and support consumers.
//!
//! The tip trigger class ([`M5TipTriggerClass`]), tip dismissal state
//! ([`M5TipDismissalState`]), command-backing state ([`M5CommandBackingState`]), teaching
//! surface family ([`M5TeachingSurfaceFamily`]), deployment line
//! ([`M5TeachingDeploymentLine`]), teaching consumer surface ([`M5TeachingConsumerSurface`]),
//! accessibility route ([`M5TeachingAccessibilityRoute`]), qualification class
//! ([`M5TeachingQualificationClass`]), and downgrade trigger ([`M5TeachingDowngradeTrigger`])
//! are reused verbatim from the frozen matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the tip card itself: its learnability consumers, its
//! anatomy parts, its derived delivery posture, its bounded actions, and its export fields.
//! No M5 learnability surface invents a second tip-card grammar.
//!
//! Raw docs bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every relevance line, command reference, and tip identity is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed,
    seeded_m5_contextual_tip_card_packet,
    seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed,
    M5_CONTEXTUAL_TIP_CARD_PACKET_ID,
};

// The tip trigger class, tip dismissal state, command-backing state, surface family,
// deployment line, consumer surface, accessibility route, qualification class, and downgrade
// triggers are frozen once, in the contextual-teaching / migration-bridge component matrix.
// This primitive reuses them verbatim so it never invents a parallel tip vocabulary.
pub use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5CommandBackingState, M5TeachingAccessibilityRoute, M5TeachingConsumerSurface,
    M5TeachingDeploymentLine, M5TeachingDowngradeTrigger, M5TeachingQualificationClass,
    M5TeachingSurfaceFamily, M5TipDismissalState, M5TipTriggerClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ContextualTipCardPacket`].
pub const M5_CONTEXTUAL_TIP_CARD_RECORD_KIND: &str =
    "implement_m5_contextual_tip_cards_with_why_now_relevance_dismiss_snooze_stable_command_references_and_quiet_hours_safe_delivery_across_claimed_m5_learnability_surfaces";

/// Schema version for M5 contextual-tip-card records.
pub const M5_CONTEXTUAL_TIP_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the contextual-tip-card boundary schema.
pub const M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF: &str = "schemas/ui/m5-contextual-tip-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CONTEXTUAL_TIP_CARD_DOC_REF: &str = "docs/help/m5_contextual_tip_card_primitive.md";

/// Repo-relative path of the frozen contextual-teaching / migration-bridge component matrix
/// this primitive narrows from.
pub const M5_CONTEXTUAL_TIP_CARD_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json";

/// Repo-relative path of the command-descriptor contract the tip's next action binds its
/// stable command reference against.
pub const M5_CONTEXTUAL_TIP_CARD_COMMAND_DESCRIPTOR_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Repo-relative path of the presentation-mode-state contract the tip's delivery respects.
pub const M5_CONTEXTUAL_TIP_CARD_PRESENTATION_MODE_REF: &str =
    "schemas/ux/presentation_mode_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONTEXTUAL_TIP_CARD_FIXTURE_DIR: &str = "fixtures/ui/m5-contextual-tip-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONTEXTUAL_TIP_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-contextual-tip-card-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CONTEXTUAL_TIP_CARD_CSV_REF: &str =
    "artifacts/release/m5-contextual-tip-card-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CONTEXTUAL_TIP_CARD_REPORT_REF: &str =
    "artifacts/design/m5-contextual-tip-card-primitive.md";

/// One claimed M5 learnability consumer that renders the shared contextual tip card. These
/// are the consumers the acceptance criteria name — the first-run onboarding panel, the
/// guided-tour overlay, the command-palette hint, the inline editor tip, and the support tip
/// export — so the same tip-card grammar works across every claimed learnability surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTipConsumerSurface {
    /// The first-run onboarding panel surface.
    FirstRunOnboardingPanel,
    /// The guided-tour overlay surface.
    GuidedTourOverlay,
    /// The command-palette hint surface.
    CommandPaletteHint,
    /// The inline editor tip surface.
    InlineEditorTip,
    /// The support tip-export surface.
    SupportTipExport,
}

impl M5ContextualTipConsumerSurface {
    /// Every claimed learnability consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstRunOnboardingPanel,
        Self::GuidedTourOverlay,
        Self::CommandPaletteHint,
        Self::InlineEditorTip,
        Self::SupportTipExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRunOnboardingPanel => "first_run_onboarding_panel",
            Self::GuidedTourOverlay => "guided_tour_overlay",
            Self::CommandPaletteHint => "command_palette_hint",
            Self::InlineEditorTip => "inline_editor_tip",
            Self::SupportTipExport => "support_tip_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstRunOnboardingPanel => "First-Run Onboarding Panel",
            Self::GuidedTourOverlay => "Guided-Tour Overlay",
            Self::CommandPaletteHint => "Command-Palette Hint",
            Self::InlineEditorTip => "Inline Editor Tip",
            Self::SupportTipExport => "Support Tip Export",
        }
    }
}

/// The derived delivery posture of a contextual tip card — the resolver's verdict about
/// whether the tip is shown right now, and if so how. Derived from the dismissal state and
/// the live delivery context, so a tip never spams a recently dismissed hint, never fires
/// through quiet hours or presentation mode, and never presents an informational hint as a
/// command-backed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTipDeliveryPosture {
    /// Delivered in place with a concrete, command-backed next action.
    DeliveredActionable,
    /// Delivered in place as an informational hint with no command backing.
    DeliveredInformational,
    /// Snoozed for later at the user's request.
    SnoozedForLater,
    /// Withheld because quiet hours are active.
    WithheldForQuietHours,
    /// Withheld because presentation mode is active.
    WithheldForPresentationMode,
    /// Withheld because this tip was already resolved or recently dismissed (non-spammy).
    WithheldAlreadyResolved,
}

impl M5ContextualTipDeliveryPosture {
    /// Every delivery posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DeliveredActionable,
        Self::DeliveredInformational,
        Self::SnoozedForLater,
        Self::WithheldForQuietHours,
        Self::WithheldForPresentationMode,
        Self::WithheldAlreadyResolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeliveredActionable => "delivered_actionable",
            Self::DeliveredInformational => "delivered_informational",
            Self::SnoozedForLater => "snoozed_for_later",
            Self::WithheldForQuietHours => "withheld_for_quiet_hours",
            Self::WithheldForPresentationMode => "withheld_for_presentation_mode",
            Self::WithheldAlreadyResolved => "withheld_already_resolved",
        }
    }

    /// True when the tip is delivered on screen (actionable or informational).
    pub const fn is_delivered(self) -> bool {
        matches!(
            self,
            Self::DeliveredActionable | Self::DeliveredInformational
        )
    }

    /// True when the tip is delivered with a concrete, command-backed next action.
    pub const fn is_actionable(self) -> bool {
        matches!(self, Self::DeliveredActionable)
    }

    /// True when the tip is withheld from the screen for a delivery-context reason.
    pub const fn is_withheld(self) -> bool {
        matches!(
            self,
            Self::WithheldForQuietHours
                | Self::WithheldForPresentationMode
                | Self::WithheldAlreadyResolved
        )
    }
}

/// One bounded action a contextual tip card offers, so a delivered tip never hides its
/// try / open-docs / snooze / dismiss affordances, and a user can act on, defer, or reverse a
/// tip in place without leaving the task. `RequestApproval` stands in for `TryNextAction`
/// whenever the underlying action requires approval, so the tip never bypasses the trust
/// boundary of the action it teaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTipAction {
    /// Try the concrete next action now (command-backed, no approval required).
    TryNextAction,
    /// Request approval for the underlying action instead of running it directly.
    RequestApproval,
    /// Open the docs for the taught action.
    OpenDocs,
    /// Snooze the tip for later.
    SnoozeTip,
    /// Dismiss the tip.
    DismissTip,
}

impl M5ContextualTipAction {
    /// Every tip action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TryNextAction,
        Self::RequestApproval,
        Self::OpenDocs,
        Self::SnoozeTip,
        Self::DismissTip,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TryNextAction => "try_next_action",
            Self::RequestApproval => "request_approval",
            Self::OpenDocs => "open_docs",
            Self::SnoozeTip => "snooze_tip",
            Self::DismissTip => "dismiss_tip",
        }
    }
}

/// Controlled contextual-tip-card anatomy part the shared card surfaces. The parts in
/// [`M5ContextualTipAnatomyPart::MANDATORY`] are required on every card so the why-now
/// relevance, the concrete next action, the stable command reference, the dismissal / snooze
/// control, and the delivery posture are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTipAnatomyPart {
    /// The why-now relevance cue.
    WhyNowRelevanceCue,
    /// The concrete next-action cue.
    ConcreteNextActionCue,
    /// The stable command-reference cue.
    StableCommandReferenceCue,
    /// The trigger-class cue.
    TriggerClassCue,
    /// The dismissal / snooze control cue.
    DismissalControlCue,
    /// The delivery-posture cue.
    DeliveryPostureCue,
    /// The trust / approval boundary cue.
    TrustBoundaryCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5ContextualTipAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::WhyNowRelevanceCue,
        Self::ConcreteNextActionCue,
        Self::StableCommandReferenceCue,
        Self::TriggerClassCue,
        Self::DismissalControlCue,
        Self::DeliveryPostureCue,
        Self::TrustBoundaryCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::WhyNowRelevanceCue,
        Self::ConcreteNextActionCue,
        Self::StableCommandReferenceCue,
        Self::DismissalControlCue,
        Self::DeliveryPostureCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WhyNowRelevanceCue => "why_now_relevance_cue",
            Self::ConcreteNextActionCue => "concrete_next_action_cue",
            Self::StableCommandReferenceCue => "stable_command_reference_cue",
            Self::TriggerClassCue => "trigger_class_cue",
            Self::DismissalControlCue => "dismissal_control_cue",
            Self::DeliveryPostureCue => "delivery_posture_cue",
            Self::TrustBoundaryCue => "trust_boundary_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the card export carries so contextual-tip-card truth is reconstructable. The
/// fields in [`M5ContextualTipExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTipExportField {
    /// The tip trigger class.
    TriggerClass,
    /// The command-backing state.
    CommandBackingState,
    /// The dismissal state.
    DismissalState,
    /// The derived delivery posture.
    DeliveryPosture,
    /// The why-now relevance line.
    WhyNowRelevance,
    /// The stable next-action command reference.
    NextActionCommandRef,
    /// The bounded available actions.
    AvailableActions,
    /// The trust / approval boundary of the underlying action.
    TrustBoundary,
}

impl M5ContextualTipExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TriggerClass,
        Self::CommandBackingState,
        Self::DismissalState,
        Self::DeliveryPosture,
        Self::WhyNowRelevance,
        Self::NextActionCommandRef,
        Self::AvailableActions,
        Self::TrustBoundary,
    ];

    /// The export fields every card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::TriggerClass,
        Self::CommandBackingState,
        Self::DismissalState,
        Self::DeliveryPosture,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TriggerClass => "trigger_class",
            Self::CommandBackingState => "command_backing_state",
            Self::DismissalState => "dismissal_state",
            Self::DeliveryPosture => "delivery_posture",
            Self::WhyNowRelevance => "why_now_relevance",
            Self::NextActionCommandRef => "next_action_command_ref",
            Self::AvailableActions => "available_actions",
            Self::TrustBoundary => "trust_boundary",
        }
    }
}

// ---- contextual-tip-card resolver ---------------------------------------

/// The full input to the contextual-tip-card resolver for one tip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardResolutionInput {
    /// Why the tip is being offered.
    pub trigger_class: M5TipTriggerClass,
    /// How the tip's next action is backed by a command.
    pub command_backing: M5CommandBackingState,
    /// The tip's current dismissal state.
    pub dismissal_state: M5TipDismissalState,
    /// True when quiet hours are active, so a non-urgent tip is withheld.
    pub quiet_hours_active: bool,
    /// True when presentation mode is active, so a tip is withheld.
    pub presentation_mode_active: bool,
    /// True when a like tip was recently dismissed, so it is not shown again yet.
    pub recently_dismissed: bool,
    /// True when the underlying action requires approval, so "try it" defers to a request.
    pub underlying_action_requires_approval: bool,
    /// The opaque why-now relevance line (must be non-empty).
    pub why_now_relevance: String,
    /// The opaque stable next-action command reference (must be non-empty).
    pub next_action_command_ref: String,
    /// The opaque stable tip identity (must be non-empty).
    pub tip_identity_ref: String,
}

/// The resolved contextual-tip-card truth for one tip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedContextualTipCard {
    /// The tip trigger class.
    pub trigger_class: M5TipTriggerClass,
    /// The command-backing state.
    pub command_backing: M5CommandBackingState,
    /// The dismissal state.
    pub dismissal_state: M5TipDismissalState,
    /// The opaque why-now relevance line, preserved exactly from the input.
    pub why_now_relevance: String,
    /// The opaque stable next-action command reference, preserved exactly from the input.
    pub next_action_command_ref: String,
    /// The opaque stable tip identity, preserved exactly from the input.
    pub tip_identity_ref: String,
    /// The derived delivery posture.
    pub delivery_posture: M5ContextualTipDeliveryPosture,
    /// The bounded actions this tip offers.
    pub available_actions: Vec<M5ContextualTipAction>,
    /// True when the tip's next action is backed by a command.
    pub is_command_backed: bool,
    /// True when the tip is delivered on screen.
    pub is_delivered: bool,
    /// True when the underlying action requires approval before "try it" runs, preserved from
    /// the input.
    pub requires_approval_before_try: bool,
    /// The tip teaches in place and never opens a detached tutorial. ALWAYS `true`.
    pub teaches_in_place: bool,
    /// The tip never hijacks the workflow as a blocking tour. ALWAYS `false`.
    pub hijacks_workflow: bool,
    /// The tip's delivery respects quiet hours. ALWAYS `true`.
    pub respects_quiet_hours: bool,
    /// The tip's delivery respects presentation mode. ALWAYS `true`.
    pub respects_presentation_mode: bool,
    /// The tip's delivery respects a recent dismissal (non-spammy). ALWAYS `true`.
    pub respects_recent_dismissals: bool,
    /// Snooze / dismiss are reversible and never remove the underlying action. ALWAYS `true`.
    pub is_reversible: bool,
    /// The tip honors the same trust / approval limits as the underlying action. ALWAYS
    /// `true`.
    pub honors_underlying_trust_limits: bool,
}

/// Errors returned by [`resolve_contextual_tip_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ContextualTipCardResolutionError {
    /// The why-now relevance line was empty.
    EmptyWhyNowRelevance,
    /// The next-action command reference was empty.
    EmptyNextActionCommandRef,
    /// The tip identity ref was empty.
    EmptyTipIdentity,
    /// A card descriptor carried forbidden material.
    ForbiddenTipMaterial,
}

impl M5ContextualTipCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyWhyNowRelevance => "empty_why_now_relevance",
            Self::EmptyNextActionCommandRef => "empty_next_action_command_ref",
            Self::EmptyTipIdentity => "empty_tip_identity",
            Self::ForbiddenTipMaterial => "forbidden_tip_material",
        }
    }
}

impl fmt::Display for M5ContextualTipCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "contextual tip card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ContextualTipCardResolutionError {}

/// Resolves one contextual tip card from its declared trigger, command backing, dismissal
/// state, and live delivery context.
///
/// The delivery posture is derived with a suppression-first ladder so the tip always respects
/// its delivery limits before it is ever shown: an active quiet-hours window withholds the
/// tip, then an active presentation mode withholds it, then an already-resolved or recently
/// dismissed tip stays withheld so it is never spammy, then a snoozed tip stays snoozed, and
/// only a live, dismissible tip is delivered — as a command-backed *actionable* tip when it
/// has a command binding, or as an *informational* hint when it does not. The action set
/// always offers open-docs, snooze, and dismiss on a delivered tip so it is reversible in
/// place; it offers the concrete "try it" action only when the tip is command-backed, and
/// substitutes a "request approval" action whenever the underlying action requires approval,
/// so the tip never bypasses the trust boundary of the action it teaches.
pub fn resolve_contextual_tip_card(
    input: &M5ContextualTipCardResolutionInput,
) -> Result<M5ResolvedContextualTipCard, M5ContextualTipCardResolutionError> {
    if input.why_now_relevance.trim().is_empty() {
        return Err(M5ContextualTipCardResolutionError::EmptyWhyNowRelevance);
    }
    if input.next_action_command_ref.trim().is_empty() {
        return Err(M5ContextualTipCardResolutionError::EmptyNextActionCommandRef);
    }
    if input.tip_identity_ref.trim().is_empty() {
        return Err(M5ContextualTipCardResolutionError::EmptyTipIdentity);
    }
    if value_repr_is_forbidden(&input.why_now_relevance)
        || value_repr_is_forbidden(&input.next_action_command_ref)
        || value_repr_is_forbidden(&input.tip_identity_ref)
    {
        return Err(M5ContextualTipCardResolutionError::ForbiddenTipMaterial);
    }

    let is_command_backed = !matches!(
        input.command_backing,
        M5CommandBackingState::NoCommandBacking
    );
    let delivery_posture = derive_delivery_posture(input, is_command_backed);
    let available_actions = derive_tip_actions(
        delivery_posture,
        is_command_backed,
        input.underlying_action_requires_approval,
    );

    Ok(M5ResolvedContextualTipCard {
        trigger_class: input.trigger_class,
        command_backing: input.command_backing,
        dismissal_state: input.dismissal_state,
        why_now_relevance: input.why_now_relevance.clone(),
        next_action_command_ref: input.next_action_command_ref.clone(),
        tip_identity_ref: input.tip_identity_ref.clone(),
        delivery_posture,
        available_actions,
        is_command_backed,
        is_delivered: delivery_posture.is_delivered(),
        requires_approval_before_try: input.underlying_action_requires_approval,
        // The acceptance criterion: tips teach in place, never hijack the workflow, always
        // respect quiet hours / presentation mode / recent dismissals, stay reversible, and
        // honor the same trust / approval limits as the underlying action.
        teaches_in_place: true,
        hijacks_workflow: false,
        respects_quiet_hours: true,
        respects_presentation_mode: true,
        respects_recent_dismissals: true,
        is_reversible: true,
        honors_underlying_trust_limits: true,
    })
}

/// Derives the delivery posture with a suppression-first ladder so a tip never fires through
/// quiet hours or presentation mode and never spams a recently dismissed hint.
fn derive_delivery_posture(
    input: &M5ContextualTipCardResolutionInput,
    is_command_backed: bool,
) -> M5ContextualTipDeliveryPosture {
    use M5ContextualTipDeliveryPosture as Posture;
    use M5TipDismissalState as Dismissal;

    if input.quiet_hours_active {
        return Posture::WithheldForQuietHours;
    }
    if input.presentation_mode_active {
        return Posture::WithheldForPresentationMode;
    }
    if input.recently_dismissed
        || matches!(
            input.dismissal_state,
            Dismissal::Dismissed | Dismissal::AutoExpired | Dismissal::SuppressedByPreference
        )
    {
        return Posture::WithheldAlreadyResolved;
    }
    if matches!(input.dismissal_state, Dismissal::Snoozed) {
        return Posture::SnoozedForLater;
    }
    // Dismissible or persistent-until-acted and clear to show: an actionable tip when it is
    // command-backed, an informational hint otherwise.
    if is_command_backed {
        Posture::DeliveredActionable
    } else {
        Posture::DeliveredInformational
    }
}

/// Derives the bounded action set from the delivery posture, the command backing, and whether
/// the underlying action requires approval.
///
/// A delivered tip always offers open-docs, snooze, and dismiss so it stays reversible in
/// place; an actionable tip additionally offers "try it" — or "request approval" when the
/// underlying action requires approval. A snoozed tip offers only dismiss so it can still be
/// resolved permanently; a withheld tip is off screen and offers nothing.
fn derive_tip_actions(
    delivery_posture: M5ContextualTipDeliveryPosture,
    is_command_backed: bool,
    requires_approval: bool,
) -> Vec<M5ContextualTipAction> {
    use M5ContextualTipAction as Action;
    use M5ContextualTipDeliveryPosture as Posture;

    let mut actions = Vec::new();
    match delivery_posture {
        Posture::DeliveredActionable => {
            if requires_approval {
                actions.push(Action::RequestApproval);
            } else {
                actions.push(Action::TryNextAction);
            }
            actions.push(Action::OpenDocs);
            actions.push(Action::SnoozeTip);
            actions.push(Action::DismissTip);
        }
        Posture::DeliveredInformational => {
            // No command backing → no "try it"; the tip still teaches and stays reversible.
            let _ = is_command_backed;
            actions.push(Action::OpenDocs);
            actions.push(Action::SnoozeTip);
            actions.push(Action::DismissTip);
        }
        Posture::SnoozedForLater => {
            actions.push(Action::DismissTip);
        }
        Posture::WithheldForQuietHours
        | Posture::WithheldForPresentationMode
        | Posture::WithheldAlreadyResolved => {}
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked contextual-tip-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardResolutionCase {
    /// The resolver input.
    pub input: M5ContextualTipCardResolutionInput,
    /// The resolved truth. Must equal `resolve_contextual_tip_card(&input)`.
    pub resolved: M5ResolvedContextualTipCard,
}

impl M5ContextualTipCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ContextualTipCardResolutionInput) -> Self {
        let resolved =
            resolve_contextual_tip_card(&input).expect("seed contextual tip card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_contextual_tip_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input tip identity, relevance, and command
    /// reference exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.tip_identity_ref == self.input.tip_identity_ref
            && self.resolved.why_now_relevance == self.input.why_now_relevance
            && self.resolved.next_action_command_ref == self.input.next_action_command_ref
    }

    /// True when the resolved case teaches in place, never hijacks the workflow, respects
    /// every delivery limit, stays reversible, and honors the underlying trust limits.
    pub fn preserves_reversibility(&self) -> bool {
        self.resolved.teaches_in_place
            && !self.resolved.hijacks_workflow
            && self.resolved.respects_quiet_hours
            && self.resolved.respects_presentation_mode
            && self.resolved.respects_recent_dismissals
            && self.resolved.is_reversible
            && self.resolved.honors_underlying_trust_limits
    }
}

/// One row in the primitive matrix: one learnability consumer bound to the shared tip-card
/// anatomy, trigger classes, command-backing states, dismissal states, delivery postures,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipConsumerRow {
    /// Learnability consumer family.
    pub consumer_surface: M5ContextualTipConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 learnability surface families that render / consume this card.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Anatomy parts this card renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ContextualTipAnatomyPart>,
    /// Tip trigger classes this consumer distinguishes.
    pub trigger_classes: Vec<M5TipTriggerClass>,
    /// Command-backing states this consumer distinguishes.
    pub command_backing_states: Vec<M5CommandBackingState>,
    /// Dismissal states this consumer distinguishes.
    pub dismissal_states: Vec<M5TipDismissalState>,
    /// Delivery postures this consumer distinguishes.
    pub delivery_postures: Vec<M5ContextualTipDeliveryPosture>,
    /// Bounded tip actions this consumer offers.
    pub tip_actions: Vec<M5ContextualTipAction>,
    /// Export fields this card carries (must include the mandatory fields).
    pub export_fields: Vec<M5ContextualTipExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Teaching subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked tip-card resolutions proving the resolver on this consumer.
    pub tip_examples: Vec<M5ContextualTipCardResolutionCase>,
    /// Hard invariant: this consumer never masks its stable command binding. MUST be `false`.
    pub masks_command_binding: bool,
    /// Hard invariant: this consumer never hijacks the workflow as a blocking tour. MUST be
    /// `false`.
    pub hijacks_workflow_as_blocking_tour: bool,
    /// Hard invariant: this consumer never ignores quiet hours, presentation mode, or a recent
    /// dismissal. MUST be `false`.
    pub ignores_quiet_hours_or_dismissals: bool,
    /// Hard invariant: this consumer never bypasses the underlying trust / approval limits.
    /// MUST be `false`.
    pub bypasses_underlying_trust_limits: bool,
}

impl M5ContextualTipConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ContextualTipAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ContextualTipAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5ContextualTipExportField> =
            self.export_fields.iter().copied().collect();
        M5ContextualTipExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_command_binding
            && !self.hijacks_workflow_as_blocking_tour
            && !self.ignores_quiet_hours_or_dismissals
            && !self.bypasses_underlying_trust_limits
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardVocabularySet {
    /// Learnability-consumer tokens.
    pub tip_consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Delivery-posture tokens.
    pub delivery_postures: Vec<String>,
    /// Tip-action tokens.
    pub tip_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Tip-trigger-class tokens (reused from the frozen matrix).
    pub trigger_classes: Vec<String>,
    /// Command-backing-state tokens (reused from the frozen matrix).
    pub command_backing_states: Vec<String>,
    /// Tip-dismissal-state tokens (reused from the frozen matrix).
    pub dismissal_states: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Teaching-consumer-surface tokens (reused from the frozen matrix).
    pub teaching_consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ContextualTipCardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            tip_consumer_surfaces: tokens(&M5ContextualTipConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ContextualTipAnatomyPart::ALL, |v| v.as_str()),
            delivery_postures: tokens(&M5ContextualTipDeliveryPosture::ALL, |v| v.as_str()),
            tip_actions: tokens(&M5ContextualTipAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ContextualTipExportField::ALL, |v| v.as_str()),
            trigger_classes: tokens(&M5TipTriggerClass::ALL, |v| v.as_str()),
            command_backing_states: tokens(&M5CommandBackingState::ALL, |v| v.as_str()),
            dismissal_states: tokens(&M5TipDismissalState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TeachingSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TeachingDeploymentLine::ALL, |v| v.as_str()),
            teaching_consumer_surfaces: tokens(&M5TeachingConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TeachingAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ContextualTipCardGovernanceReview {
    /// The tip card shows its why-now relevance.
    pub tip_card_shows_why_now_relevance: bool,
    /// The tip card shows its concrete next action.
    pub tip_card_shows_concrete_next_action: bool,
    /// The tip card shows its stable command reference.
    pub tip_card_shows_stable_command_reference: bool,
    /// The tip card shows its dismissal and snooze controls.
    pub tip_card_shows_dismissal_and_snooze_controls: bool,
    /// Tips never hijack the workflow as a blocking tour.
    pub tips_never_hijack_workflow_as_blocking_tour: bool,
    /// Tips respect quiet hours.
    pub tips_respect_quiet_hours: bool,
    /// Tips respect presentation mode.
    pub tips_respect_presentation_mode: bool,
    /// Tips respect recent dismissals (non-spammy).
    pub tips_respect_recent_dismissals: bool,
    /// Tips honor the same trust / approval limits as the underlying action.
    pub tips_honor_underlying_trust_and_approval_limits: bool,
    /// Tips remain reversible and command-backed.
    pub tips_remain_reversible_and_command_backed: bool,
    /// Users learn nearby actions without leaving the task or reopening a detached tutorial.
    pub users_learn_without_leaving_task: bool,
    /// Tip cards keep the same truth across every deployment line.
    pub tips_stable_across_deployment_lines: bool,
    /// Tip cards keep the same truth across desktop, headless/export, and support consumers.
    pub tips_stable_across_consumer_surfaces: bool,
    /// Every tip declares a non-visual accessibility route.
    pub every_tip_declares_accessibility_route: bool,
    /// The support / export packet reconstructs tip truth.
    pub support_export_reconstructs_tip_truth: bool,
    /// Later M5 rows cannot invent parallel tip-card vocabulary.
    pub later_rows_cannot_invent_parallel_tip_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardConsumerProjection {
    /// Learnability surfaces consume the shared tip-card vocabulary.
    pub learnability_surfaces_consume_tip_vocabulary: bool,
    /// The delivery-posture resolver reads a single canonical source.
    pub delivery_posture_reads_single_source: bool,
    /// The action-set derivation reads a single canonical source.
    pub action_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop tip cards read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the contextual tip card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting contextual-tip-card audit.
    pub contextual_tip_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ContextualTipCardPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContextualTipCardPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Learnability-consumer rows.
    pub rows: Vec<M5ContextualTipConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContextualTipCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ContextualTipCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContextualTipCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContextualTipCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ContextualTipCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 contextual-tip-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTipCardPacket {
    /// Record kind; must equal [`M5_CONTEXTUAL_TIP_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONTEXTUAL_TIP_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Learnability-consumer rows.
    pub rows: Vec<M5ContextualTipConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContextualTipCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ContextualTipCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContextualTipCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContextualTipCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ContextualTipCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ContextualTipCardPacket {
    /// Builds an M5 contextual-tip-card-primitive packet from stable-lane input.
    pub fn new(input: M5ContextualTipCardPacketInput) -> Self {
        Self {
            record_kind: M5_CONTEXTUAL_TIP_CARD_RECORD_KIND.to_owned(),
            schema_version: M5_CONTEXTUAL_TIP_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 contextual-tip-card-primitive invariants.
    pub fn validate(&self) -> Vec<M5ContextualTipCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONTEXTUAL_TIP_CARD_RECORD_KIND {
            violations.push(M5ContextualTipCardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONTEXTUAL_TIP_CARD_SCHEMA_VERSION {
            violations.push(M5ContextualTipCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ContextualTipCardViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_trigger_class_coverage(self, &mut violations);
        validate_delivery_posture_coverage(self, &mut violations);
        validate_action_coverage(self, &mut violations);
        validate_reversibility(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 contextual tip card primitive packet serializes"),
        ) {
            violations.push(M5ContextualTipCardViolation::RawMaterialInExport);
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
            .expect("m5 contextual tip card primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per learnability consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,trigger_classes,command_backing_states,delivery_postures,tip_actions,tip_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.trigger_classes, |v| v.as_str()),
                join_tokens(&row.command_backing_states, |v| v.as_str()),
                join_tokens(&row.delivery_postures, |v| v.as_str()),
                join_tokens(&row.tip_actions, |v| v.as_str()),
                row.tip_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Contextual-Tip-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Learnability consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Delivery postures: {}\n",
            self.vocabulary_set.delivery_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Tip actions: {}\n",
            self.vocabulary_set.tip_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Trigger classes: {}\n",
            self.vocabulary_set.trigger_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Learnability consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked tips: {}\n", row.tip_examples.len()));
            for case in &row.tip_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (command-backed `{}`, delivered `{}`, approval `{}`)\n",
                    case.resolved.tip_identity_ref,
                    case.resolved.trigger_class.as_str(),
                    case.resolved.dismissal_state.as_str(),
                    case.resolved.delivery_posture.as_str(),
                    case.resolved.is_command_backed,
                    case.resolved.is_delivered,
                    case.resolved.requires_approval_before_try,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 contextual-tip-card-primitive export.
#[derive(Debug)]
pub enum M5ContextualTipCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContextualTipCardViolation>),
}

impl fmt::Display for M5ContextualTipCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 contextual tip card primitive export parse failed: {error}"
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
                    "m5 contextual tip card primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ContextualTipCardArtifactError {}

/// Validation failures emitted by [`M5ContextualTipCardPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContextualTipCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required learnability consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A learnability-consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked tip resolutions.
    TipExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every tip trigger class.
    TriggerClassCoverageUnproven,
    /// The worked resolutions do not prove a delivered-actionable, a delivered-informational,
    /// a snoozed, and a withheld tip.
    DeliveryPostureCoverageUnproven,
    /// The worked resolutions do not prove the try, request-approval, open-docs, snooze, and
    /// dismiss actions.
    ActionCoverageUnproven,
    /// A worked resolution does not teach in place, stay reversible, and honor trust limits.
    ReversibilityUnproven,
    /// A worked resolution does not preserve its exact tip identity, relevance, and command
    /// reference.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ContextualTipCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::TipExampleMissing => "tip_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::TriggerClassCoverageUnproven => "trigger_class_coverage_unproven",
            Self::DeliveryPostureCoverageUnproven => "delivery_posture_coverage_unproven",
            Self::ActionCoverageUnproven => "action_coverage_unproven",
            Self::ReversibilityUnproven => "reversibility_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 contextual-tip-card-primitive export.
pub fn current_stable_m5_contextual_tip_card_export(
) -> Result<M5ContextualTipCardPacket, M5ContextualTipCardArtifactError> {
    let packet: M5ContextualTipCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-tip-card-primitive-proof/support_export.json"
    )))
    .map_err(M5ContextualTipCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ContextualTipCardArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
        M5_CONTEXTUAL_TIP_CARD_DOC_REF,
        M5_CONTEXTUAL_TIP_CARD_COMPONENT_MATRIX_REF,
        M5_CONTEXTUAL_TIP_CARD_COMMAND_DESCRIPTOR_REF,
        M5_CONTEXTUAL_TIP_CARD_PRESENTATION_MODE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ContextualTipCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ContextualTipCardViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let present: BTreeSet<M5ContextualTipConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5ContextualTipConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ContextualTipCardViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.trigger_classes.is_empty()
            || row.command_backing_states.is_empty()
            || row.dismissal_states.is_empty()
            || row.delivery_postures.is_empty()
            || row.tip_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5ContextualTipCardViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ContextualTipCardViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5ContextualTipCardViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ContextualTipCardViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ContextualTipCardViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ContextualTipCardViolation::DowngradeTriggersMissing);
        }
        if row.tip_examples.is_empty() {
            violations.push(M5ContextualTipCardViolation::TipExampleMissing);
        }
        if row
            .tip_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ContextualTipCardViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ContextualTipCardViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ContextualTipCardViolation::RowInvariantViolated);
        }
    }
}

/// Every tip trigger class must be exercised by some worked resolution — the implementation
/// requirement that a tip always names why it is relevant *now* across every trigger.
fn validate_trigger_class_coverage(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let exercised: BTreeSet<M5TipTriggerClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.tip_examples.iter())
        .map(|case| case.resolved.trigger_class)
        .collect();
    let covered = M5TipTriggerClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5ContextualTipCardViolation::TriggerClassCoverageUnproven);
    }
}

/// At least one worked resolution must prove a delivered-actionable tip, one a
/// delivered-informational tip, one a snoozed tip, and one a withheld tip — the acceptance
/// criterion that tips teach in place yet stay non-spammy and quiet-hours-safe.
fn validate_delivery_posture_coverage(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.tip_examples.iter());
    let has_actionable = cases().any(|case| {
        case.resolved.delivery_posture == M5ContextualTipDeliveryPosture::DeliveredActionable
    });
    let has_informational = cases().any(|case| {
        case.resolved.delivery_posture == M5ContextualTipDeliveryPosture::DeliveredInformational
    });
    let has_snoozed = cases().any(|case| {
        case.resolved.delivery_posture == M5ContextualTipDeliveryPosture::SnoozedForLater
    });
    let has_withheld = cases().any(|case| case.resolved.delivery_posture.is_withheld());
    if !(has_actionable && has_informational && has_snoozed && has_withheld) {
        violations.push(M5ContextualTipCardViolation::DeliveryPostureCoverageUnproven);
    }
}

/// At least one worked resolution must prove each of the try, request-approval, open-docs,
/// snooze, and dismiss actions — the implementation requirement that a tip offers try / open
/// docs / snooze / dismiss and never bypasses the underlying trust boundary.
fn validate_action_coverage(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.tip_examples.iter());
    let covered = M5ContextualTipAction::ALL
        .iter()
        .all(|action| cases().any(|case| case.resolved.available_actions.contains(action)));
    if !covered {
        violations.push(M5ContextualTipCardViolation::ActionCoverageUnproven);
    }
}

/// Every worked resolution must teach in place, never hijack the workflow, respect every
/// delivery limit, stay reversible, and honor the underlying trust limits — the acceptance
/// criterion that tips remain reversible, command-backed, and non-spammy.
fn validate_reversibility(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.tip_examples.iter())
        .all(|case| case.preserves_reversibility());
    if !preserved {
        violations.push(M5ContextualTipCardViolation::ReversibilityUnproven);
    }
}

/// Every worked resolution must preserve its exact tip identity, relevance, and command
/// reference — the invariant that the tip card never rewrites what it teaches.
fn validate_identity_preservation(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.tip_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5ContextualTipCardViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tip_card_shows_why_now_relevance,
        review.tip_card_shows_concrete_next_action,
        review.tip_card_shows_stable_command_reference,
        review.tip_card_shows_dismissal_and_snooze_controls,
        review.tips_never_hijack_workflow_as_blocking_tour,
        review.tips_respect_quiet_hours,
        review.tips_respect_presentation_mode,
        review.tips_respect_recent_dismissals,
        review.tips_honor_underlying_trust_and_approval_limits,
        review.tips_remain_reversible_and_command_backed,
        review.users_learn_without_leaving_task,
        review.tips_stable_across_deployment_lines,
        review.tips_stable_across_consumer_surfaces,
        review.every_tip_declares_accessibility_route,
        review.support_export_reconstructs_tip_truth,
        review.later_rows_cannot_invent_parallel_tip_vocabulary,
    ] {
        if !ok {
            violations.push(M5ContextualTipCardViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.learnability_surfaces_consume_tip_vocabulary,
        projection.delivery_posture_reads_single_source,
        projection.action_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ContextualTipCardViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ContextualTipCardViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ContextualTipCardPacket,
    violations: &mut Vec<M5ContextualTipCardViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.contextual_tip_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ContextualTipCardViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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
