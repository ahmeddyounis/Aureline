//! Two reusable M5 learnability controls — the learning-mode toggle and the tip card —
//! so a user can make learnability explicit, reversible, and command-backed from the
//! control alone: they can tell when learning is active, what scope it changes, how to
//! pause / snooze / reset it, why a tip is relevant now, and exactly which stable
//! command / file / docs deep link backs the next step — never through an ephemeral
//! coachmark or hidden routing, and never at the cost of trust or data ownership.
//!
//! Aureline's frozen learning-component matrix
//! ([`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`])
//! names the learning-mode toggle and the tip card as two governed component families and
//! freezes their controlled vocabulary — the learning-mode states (`off`, `on`, `paused`,
//! `per_feature_family`, `sandboxed_only`, `ended`) and scopes (`global`, `workspace`,
//! `feature_family`, `session`, `surface`, `unavailable`) a toggle binds; the tip trigger
//! classes (`first_encounter`, `feature_discovery`, `error_recovery`, `mode_change`,
//! `idle_hint`, `contextual_followup`) and dismissal states (`dismissible`, `dismissed`,
//! `snoozed`, `persistent_until_acted`, `auto_expired`, `suppressed_by_preference`) a tip
//! binds; the one controlled disposition vocabulary; the surface families; the deployment
//! lines; the consumer surfaces; the accessibility routes; the required labels; and the
//! downgrade triggers. This module *implements* that contract as two co-equal control
//! vectors so a claimed M5 onboarding, tour, learning-mode, glossary, or help surface can
//! project a learning-mode toggle and a tip card that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_learning_activation`] — takes a toggle's learning-mode state and derives
//!    its activation class (active, scoped-active, sandboxed-active, paused, or inactive),
//!    whether learning is active, and which activation note the toggle must carry — so a
//!    paused or inactive toggle can never read as active learning.
//! 2. [`resolve_tip_delivery`] — takes a tip's dismissal state and derives its delivery
//!    class (delivered, delivered-persistent, snoozed, or withheld), whether the tip is
//!    delivered on screen, and which delivery note the tip must carry — so a dismissed,
//!    auto-expired, or suppressed tip can never read as delivered and stays reopenable.
//!
//! A single controls packet — [`LearningModeToggleTipCardControlsPacket`] — binds one
//! vector of learning-mode toggles and one vector of tip cards to the same scope,
//! activation / delivery, why-now context, stable command / file / docs deep-link, and
//! non-visual accessibility vocabulary, so learnability stays opt-in, reversible, and
//! command-backed across desktop, headless / export, and support consumers.
//!
//! The learning-mode state ([`M5LearningModeState`]), learning-mode scope
//! ([`M5LearningModeScope`]), tip trigger class ([`M5TipTriggerClass`]), tip dismissal
//! state ([`M5TipDismissalState`]), disposition ([`M5LearningDisposition`]), surface family
//! ([`M5LearningSurfaceFamily`]), deployment line ([`M5LearningDeploymentLine`]), consumer
//! surface ([`M5LearningConsumerSurface`]), accessibility route
//! ([`M5LearningAccessibilityRoute`]), required label ([`M5LearningRequiredLabel`]),
//! qualification class ([`M5LearningQualificationClass`]), and downgrade trigger
//! ([`M5LearningDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the two controls
//! themselves: the derived activation and delivery classes, the bounded toggle and tip
//! actions, and the deep-link kinds. No M5 learnability surface invents a second toggle or
//! tip grammar.
//!
//! Raw docs bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every why-now context line, deep-link reference, and control identity is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_learning_mode_toggle_tip_card_controls,
    seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused,
    seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld,
    LEARNING_MODE_TOGGLE_TIP_CARD_PACKET_ID,
};

// The learning-mode states and scopes, the tip trigger classes and dismissal states, the
// disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the learning-component matrix. This lane reuses
// them verbatim so it never invents a parallel toggle or tip vocabulary.
pub use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5LearningAccessibilityRoute, M5LearningComponentFamily, M5LearningConsumerSurface,
    M5LearningDeploymentLine, M5LearningDisposition, M5LearningDowngradeTrigger,
    M5LearningModeScope, M5LearningModeState, M5LearningQualificationClass,
    M5LearningRequiredLabel, M5LearningSurfaceFamily, M5TipDismissalState, M5TipTriggerClass,
    M5_LEARNING_COMPONENT_DOC_REF, M5_LEARNING_COMPONENT_SCHEMA_REF,
    M5_LEARNING_MODE_TOGGLE_SCHEMA_REF, M5_TIP_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`LearningModeToggleTipCardControlsPacket`].
pub const LEARNING_MODE_TOGGLE_TIP_CARD_RECORD_KIND: &str =
    "implement_m5_learning_mode_toggles_and_tip_cards_with_scope_pause_snooze_reset_why_now_context_and_command_file_docs_deep_link_truth_across_claimed_m5_onboarding_and_help_surfaces";

/// Schema version for M5 learning-mode-toggle / tip-card control records.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-mode-toggle-tip-card-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_DOC_REF: &str =
    "docs/help/m5_learning_mode_toggle_tip_card_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-learning-mode-toggle-tip-card-controls";

/// Repo-relative path of the checked support-export artifact.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-mode-toggle-tip-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_CSV_REF: &str =
    "artifacts/release/m5-learning-mode-toggle-tip-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_REPORT_REF: &str =
    "artifacts/design/m5-learning-mode-toggle-tip-card.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a learning control binds its next step against, so a
/// toggle or tip never depends on ephemeral coachmarks or hidden routing — every next step
/// is a stable command, file, docs, or help reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable command reference in the command system.
    CommandReference,
    /// A stable file location.
    FileLocation,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable help-topic reference reachable from Help.
    HelpTopic,
    /// No deep link is bound (the control names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommandReference,
        Self::FileLocation,
        Self::DocsAnchor,
        Self::HelpTopic,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandReference => "command_reference",
            Self::FileLocation => "file_location",
            Self::DocsAnchor => "docs_anchor",
            Self::HelpTopic => "help_topic",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- learning-mode-toggle vocabulary ------------------------------------

/// Derived activation class a learning-mode toggle may present.
///
/// This is the toggle honesty axis: the class is derived from the frozen learning-mode
/// state, never asserted, so a paused, ended, or off toggle can never present as active
/// learning and a user can always tell when learnability is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningActivationClass {
    /// Learning is fully active.
    Active,
    /// Learning is active but scoped to one feature family.
    ScopedActive,
    /// Only sandboxed practice is active.
    SandboxedActive,
    /// Learning is paused and resumable.
    Paused,
    /// Learning is inactive (off or ended).
    Inactive,
}

impl LearningActivationClass {
    /// Every activation class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::ScopedActive,
        Self::SandboxedActive,
        Self::Paused,
        Self::Inactive,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ScopedActive => "scoped_active",
            Self::SandboxedActive => "sandboxed_active",
            Self::Paused => "paused",
            Self::Inactive => "inactive",
        }
    }

    /// True when learning is active in any form (fully, scoped, or sandboxed).
    pub const fn is_active_learning(self) -> bool {
        matches!(
            self,
            Self::Active | Self::ScopedActive | Self::SandboxedActive
        )
    }
}

/// One keyboard-complete default action a learning-mode toggle offers, so a toggle never
/// hides its pause / snooze / reset affordance behind a pointer-only gesture and
/// learnability stays reversible and user-owned. `ResetLearning` is always offered so a
/// user can reset learning without affecting trust or data ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningToggleAction {
    /// Enable learning (from paused or inactive).
    EnableLearning,
    /// Pause learning.
    PauseLearning,
    /// Snooze learning for later.
    SnoozeLearning,
    /// Reset learning state (always available).
    ResetLearning,
    /// Change the learning scope (user / workspace / feature family).
    ChangeScope,
    /// Open the stable command / file / docs deep link.
    OpenDeepLink,
}

impl LearningToggleAction {
    /// Every toggle action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EnableLearning,
        Self::PauseLearning,
        Self::SnoozeLearning,
        Self::ResetLearning,
        Self::ChangeScope,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete toggle must offer.
    pub const MANDATORY: [Self; 1] = [Self::ResetLearning];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnableLearning => "enable_learning",
            Self::PauseLearning => "pause_learning",
            Self::SnoozeLearning => "snooze_learning",
            Self::ResetLearning => "reset_learning",
            Self::ChangeScope => "change_scope",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a learning-mode toggle must carry, derived from the learning-mode state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LearningToggleDisclosure {
    /// The derived activation class this toggle may present.
    pub activation_class: LearningActivationClass,
    /// Whether learning is active (in any form).
    pub is_active_learning: bool,
    /// Whether the toggle is paused.
    pub is_paused: bool,
    /// Whether the toggle must carry an explicit paused note.
    pub needs_paused_note: bool,
    /// Whether the toggle must carry an explicit sandboxed note.
    pub needs_sandboxed_note: bool,
    /// Whether the toggle must carry an explicit inactive note.
    pub needs_inactive_note: bool,
}

/// Resolves the activation truth a learning-mode toggle may present.
///
/// An `on` toggle is active. A `per_feature_family` toggle is scoped-active. A
/// `sandboxed_only` toggle is sandboxed-active. A `paused` toggle is paused, never active. An
/// `off` or `ended` toggle is inactive, so a toggle that is not actually running learning can
/// never read as active learning.
pub fn resolve_learning_activation(state: M5LearningModeState) -> LearningToggleDisclosure {
    use LearningActivationClass as Activation;
    use M5LearningModeState as Mode;

    let activation_class = match state {
        Mode::On => Activation::Active,
        Mode::PerFeatureFamily => Activation::ScopedActive,
        Mode::SandboxedOnly => Activation::SandboxedActive,
        Mode::Paused => Activation::Paused,
        Mode::Off | Mode::Ended => Activation::Inactive,
    };

    LearningToggleDisclosure {
        activation_class,
        is_active_learning: activation_class.is_active_learning(),
        is_paused: matches!(activation_class, Activation::Paused),
        needs_paused_note: matches!(activation_class, Activation::Paused),
        needs_sandboxed_note: matches!(activation_class, Activation::SandboxedActive),
        needs_inactive_note: matches!(activation_class, Activation::Inactive),
    }
}

/// A learning-mode toggle naming its learning state, scope, derived activation, why-now
/// context, bounded pause / snooze / reset actions, and a stable command / file / docs deep
/// link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeToggle {
    /// Frozen component this control implements; must be `learning_mode_toggle`.
    pub component: M5LearningComponentFamily,
    /// Stable toggle id.
    pub toggle_id: String,
    /// Human-readable toggle label; required and non-empty.
    pub toggle_label: String,
    /// Learning-mode state, reused from the frozen matrix.
    pub learning_state: M5LearningModeState,
    /// Learning-mode scope, reused from the frozen matrix.
    pub scope: M5LearningModeScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Derived activation class (must equal the resolved class).
    pub activation_class: LearningActivationClass,
    /// Whether the toggle claims learning is active (must equal the derived truth).
    pub claims_active: bool,
    /// Paused note; required when learning is paused.
    pub paused_note: String,
    /// Sandboxed note; required when only sandboxed practice is active.
    pub sandboxed_note: String,
    /// Inactive note; required when learning is off or ended.
    pub inactive_note: String,
    /// Scope / activation note; always required so scope and activation stay explicit.
    pub scope_and_activation_note: String,
    /// Why-now context; always required so the toggle names why learnability applies now.
    pub why_now_context: String,
    /// Kind of stable deep link this toggle binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include the mandatory `ResetLearning`).
    pub toggle_actions: Vec<LearningToggleAction>,
    /// Dispositions this toggle binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this toggle can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this toggle can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this toggle.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this toggle keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this toggle offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this toggle's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this toggle.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides whether learning is active or what scope it changes.
    /// MUST be `false`.
    pub hides_activation_or_scope: bool,
    /// Hard invariant: never implies a hidden apply or mutation. MUST be `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never depends on an ephemeral coachmark or hidden routing. MUST be
    /// `false`.
    pub depends_on_ephemeral_coachmark_or_hidden_routing: bool,
}

impl LearningModeToggle {
    /// Activation disclosures this toggle must carry, derived from the learning-mode state.
    pub fn activation_disclosure(&self) -> LearningToggleDisclosure {
        resolve_learning_activation(self.learning_state)
    }

    /// Whether the toggle offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<LearningToggleAction> = self.toggle_actions.iter().copied().collect();
        LearningToggleAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the toggle declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the toggle offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.toggle_actions
            .contains(&LearningToggleAction::OpenDeepLink)
    }
}

// ---- tip-card vocabulary ------------------------------------------------

/// Derived delivery class a tip card may present.
///
/// This is the tip honesty axis: the class is derived from the frozen dismissal state,
/// never asserted, so a dismissed, auto-expired, or suppressed tip can never present as
/// delivered and always stays reopenable from Help or the command system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipDeliveryClass {
    /// Delivered on screen and dismissible.
    Delivered,
    /// Delivered on screen and persistent until the user acts.
    DeliveredPersistent,
    /// Snoozed for later.
    Snoozed,
    /// Withheld (dismissed, auto-expired, or suppressed) — never on screen.
    Withheld,
}

impl TipDeliveryClass {
    /// Every delivery class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Delivered,
        Self::DeliveredPersistent,
        Self::Snoozed,
        Self::Withheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "delivered",
            Self::DeliveredPersistent => "delivered_persistent",
            Self::Snoozed => "snoozed",
            Self::Withheld => "withheld",
        }
    }

    /// True when the tip is delivered on screen.
    pub const fn is_delivered(self) -> bool {
        matches!(self, Self::Delivered | Self::DeliveredPersistent)
    }
}

/// One keyboard-complete default action a tip card offers, so a tip stays optional and
/// dismissible without losing the ability to reopen it from Help or the command system.
/// `DismissTip` is always offered so a tip is never sticky.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipCardAction {
    /// Try the concrete next action now (command-backed).
    TryNextAction,
    /// Open the stable command / file / docs deep link.
    OpenDeepLink,
    /// Snooze the tip for later.
    SnoozeTip,
    /// Dismiss the tip (always available).
    DismissTip,
    /// Reopen the tip from Help.
    ReopenFromHelp,
    /// Open the stable command reference that backs the tip.
    OpenCommandReference,
}

impl TipCardAction {
    /// Every tip action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TryNextAction,
        Self::OpenDeepLink,
        Self::SnoozeTip,
        Self::DismissTip,
        Self::ReopenFromHelp,
        Self::OpenCommandReference,
    ];

    /// The default actions every keyboard-complete tip card must offer.
    pub const MANDATORY: [Self; 1] = [Self::DismissTip];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TryNextAction => "try_next_action",
            Self::OpenDeepLink => "open_deep_link",
            Self::SnoozeTip => "snooze_tip",
            Self::DismissTip => "dismiss_tip",
            Self::ReopenFromHelp => "reopen_from_help",
            Self::OpenCommandReference => "open_command_reference",
        }
    }
}

/// Disclosures a tip card must carry, derived from the dismissal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TipCardDisclosure {
    /// The derived delivery class this tip may present.
    pub delivery_class: TipDeliveryClass,
    /// Whether the tip is delivered on screen.
    pub is_delivered: bool,
    /// Whether the tip must carry an explicit snoozed note.
    pub needs_snooze_note: bool,
    /// Whether the tip must carry an explicit withheld note.
    pub needs_withheld_note: bool,
}

/// Resolves the delivery truth a tip card may present.
///
/// A `dismissible` tip is delivered. A `persistent_until_acted` tip is delivered-persistent.
/// A `snoozed` tip is snoozed. A `dismissed`, `auto_expired`, or `suppressed_by_preference`
/// tip is withheld, so a resolved or suppressed tip can never read as delivered.
pub fn resolve_tip_delivery(dismissal: M5TipDismissalState) -> TipCardDisclosure {
    use M5TipDismissalState as Dismissal;
    use TipDeliveryClass as Delivery;

    let delivery_class = match dismissal {
        Dismissal::Dismissible => Delivery::Delivered,
        Dismissal::PersistentUntilActed => Delivery::DeliveredPersistent,
        Dismissal::Snoozed => Delivery::Snoozed,
        Dismissal::Dismissed | Dismissal::AutoExpired | Dismissal::SuppressedByPreference => {
            Delivery::Withheld
        }
    };

    TipCardDisclosure {
        delivery_class,
        is_delivered: delivery_class.is_delivered(),
        needs_snooze_note: matches!(delivery_class, Delivery::Snoozed),
        needs_withheld_note: matches!(delivery_class, Delivery::Withheld),
    }
}

/// A tip card naming its trigger class, dismissal state, derived delivery, why-now context,
/// bounded try / snooze / dismiss / reopen actions, and a stable command / file / docs deep
/// link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TipCard {
    /// Frozen component this control implements; must be `tip_card`.
    pub component: M5LearningComponentFamily,
    /// Stable tip id.
    pub tip_id: String,
    /// Human-readable tip label; required and non-empty.
    pub tip_label: String,
    /// Tip trigger class, reused from the frozen matrix.
    pub trigger_class: M5TipTriggerClass,
    /// Tip dismissal state, reused from the frozen matrix.
    pub dismissal_state: M5TipDismissalState,
    /// Derived delivery class (must equal the resolved class).
    pub delivery_class: TipDeliveryClass,
    /// Whether the tip claims it is delivered on screen (must equal the derived truth).
    pub claims_delivered: bool,
    /// Snoozed note; required when the tip is snoozed.
    pub snoozed_note: String,
    /// Withheld note; required when the tip is withheld.
    pub withheld_note: String,
    /// Why-now context; always required so the tip names why it is relevant now.
    pub why_now_context: String,
    /// Kind of stable deep link this tip binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include the mandatory `DismissTip`).
    pub tip_actions: Vec<TipCardAction>,
    /// Dispositions this tip binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5LearningDisposition>,
    /// Downgrade triggers this tip can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Mandatory labels this tip can show (must include the mandatory labels).
    pub required_labels: Vec<M5LearningRequiredLabel>,
    /// Claimed M5 surface families that render this tip.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this tip keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Non-visual accessibility routes this tip offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this tip's projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this tip.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks privacy or offline / local-only state. MUST be `false`.
    pub masks_privacy_or_offline_state: bool,
    /// Hard invariant: never hides whether learning is active or what scope it changes.
    /// MUST be `false`.
    pub hides_activation_or_scope: bool,
    /// Hard invariant: never implies a hidden apply or mutation. MUST be `false`.
    pub implies_hidden_apply_or_mutation: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never depends on an ephemeral coachmark or hidden routing. MUST be
    /// `false`.
    pub depends_on_ephemeral_coachmark_or_hidden_routing: bool,
}

impl TipCard {
    /// Delivery disclosures this tip must carry, derived from the dismissal state.
    pub fn delivery_disclosure(&self) -> TipCardDisclosure {
        resolve_tip_delivery(self.dismissal_state)
    }

    /// Whether the tip offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<TipCardAction> = self.tip_actions.iter().copied().collect();
        TipCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the tip declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LearningRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LearningRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the tip offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.tip_actions.iter().any(|action| {
            matches!(
                action,
                TipCardAction::TryNextAction
                    | TipCardAction::OpenDeepLink
                    | TipCardAction::OpenCommandReference
            )
        })
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance learnability review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeToggleTipCardReview {
    /// The learning-mode toggle names whether learning is active.
    pub toggle_shows_activation: bool,
    /// The learning-mode toggle names its user / workspace / feature-family scope.
    pub toggle_shows_scope: bool,
    /// The learning-mode toggle offers pause, snooze, and reset.
    pub toggle_offers_pause_snooze_reset: bool,
    /// The tip card names why it is relevant now.
    pub tip_shows_why_now_context: bool,
    /// The tip card stays optional and dismissible.
    pub tip_stays_optional_and_dismissible: bool,
    /// The tip card can be reopened from Help or the command system.
    pub tip_reopenable_from_help_or_commands: bool,
    /// Activation / delivery is derived from state, never asserted.
    pub activation_and_delivery_derived_never_asserted: bool,
    /// A paused or inactive toggle is never shown as active learning.
    pub inactive_never_shown_as_active: bool,
    /// A withheld tip is never shown as delivered.
    pub withheld_never_shown_as_delivered: bool,
    /// Every next step names one stable command / file / docs deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// No control depends on an ephemeral coachmark or hidden routing.
    pub no_control_depends_on_ephemeral_coachmark_or_hidden_routing: bool,
    /// No control widens trust or mutating authority.
    pub no_control_widens_trust_or_mutating_authority: bool,
    /// Progress stays user-owned and default-local.
    pub progress_user_owned_and_default_local: bool,
    /// Cached, offline, and local-only state stays visible.
    pub cached_offline_local_only_state_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl LearningModeToggleTipCardReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.toggle_shows_activation
            && self.toggle_shows_scope
            && self.toggle_offers_pause_snooze_reset
            && self.tip_shows_why_now_context
            && self.tip_stays_optional_and_dismissible
            && self.tip_reopenable_from_help_or_commands
            && self.activation_and_delivery_derived_never_asserted
            && self.inactive_never_shown_as_active
            && self.withheld_never_shown_as_delivered
            && self.every_next_step_names_stable_deep_link
            && self.no_control_depends_on_ephemeral_coachmark_or_hidden_routing
            && self.no_control_widens_trust_or_mutating_authority
            && self.progress_user_owned_and_default_local
            && self.cached_offline_local_only_state_visible
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeToggleTipCardConsumerProjection {
    /// The learning-mode panel reads a single canonical source.
    pub learning_mode_panel_reads_single_source: bool,
    /// The onboarding / tip surface reads a single canonical source.
    pub onboarding_tip_surface_reads_single_source: bool,
    /// Activation and scope are visible before a tap.
    pub activation_and_scope_visible_before_tap: bool,
    /// The why-now context is visible before a tap.
    pub why_now_context_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl LearningModeToggleTipCardConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.learning_mode_panel_reads_single_source
            && self.onboarding_tip_surface_reads_single_source
            && self.activation_and_scope_visible_before_tap
            && self.why_now_context_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeToggleTipCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`LearningModeToggleTipCardControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningModeToggleTipCardControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Learning-mode toggles.
    pub toggles: Vec<LearningModeToggle>,
    /// Tip cards.
    pub tip_cards: Vec<TipCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: LearningModeToggleTipCardReview,
    /// Consumer projection block.
    pub consumer_projection: LearningModeToggleTipCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: LearningModeToggleTipCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe learning-mode-toggle / tip-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeToggleTipCardControlsPacket {
    /// Record kind; must equal [`LEARNING_MODE_TOGGLE_TIP_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Learning-mode toggles.
    pub toggles: Vec<LearningModeToggle>,
    /// Tip cards.
    pub tip_cards: Vec<TipCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Learnability review block.
    pub learnability_review: LearningModeToggleTipCardReview,
    /// Consumer projection block.
    pub consumer_projection: LearningModeToggleTipCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: LearningModeToggleTipCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl LearningModeToggleTipCardControlsPacket {
    /// Builds a learning-mode-toggle / tip-card controls packet from stable-lane input.
    pub fn new(input: LearningModeToggleTipCardControlsPacketInput) -> Self {
        Self {
            record_kind: LEARNING_MODE_TOGGLE_TIP_CARD_RECORD_KIND.to_owned(),
            schema_version: LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            toggles: input.toggles,
            tip_cards: input.tip_cards,
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

    /// Validates the learning-mode-toggle / tip-card control invariants.
    pub fn validate(&self) -> Vec<LearningModeToggleTipCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != LEARNING_MODE_TOGGLE_TIP_CARD_RECORD_KIND {
            violations.push(LearningModeToggleTipCardViolation::WrongRecordKind);
        }
        if self.schema_version != LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_VERSION {
            violations.push(LearningModeToggleTipCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(LearningModeToggleTipCardViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(LearningModeToggleTipCardViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(LearningModeToggleTipCardViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_toggles(self, &mut violations);
        validate_tip_cards(self, &mut violations);

        if !self.learnability_review.all_hold() {
            violations.push(LearningModeToggleTipCardViolation::LearnabilityReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(LearningModeToggleTipCardViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(LearningModeToggleTipCardViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("learning mode toggle tip card packet serializes"),
        ) {
            violations.push(LearningModeToggleTipCardViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("learning mode toggle tip card packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,state_or_trigger,scope_or_dismissal,derived,active_or_delivered,deep_link_kind\n",
        );
        for toggle in &self.toggles {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "learning_mode_toggle",
                csv_field(&toggle.toggle_id),
                toggle.learning_state.as_str(),
                toggle.scope.as_str(),
                toggle.activation_disclosure().activation_class.as_str(),
                toggle.activation_disclosure().is_active_learning,
                toggle.deep_link_kind.as_str(),
            ));
        }
        for tip in &self.tip_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "tip_card",
                csv_field(&tip.tip_id),
                tip.trigger_class.as_str(),
                tip.dismissal_state.as_str(),
                tip.delivery_disclosure().delivery_class.as_str(),
                tip.delivery_disclosure().is_delivered,
                tip.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let inactive = self
            .toggles
            .iter()
            .filter(|toggle| !toggle.activation_disclosure().is_active_learning)
            .count();
        let withheld = self
            .tip_cards
            .iter()
            .filter(|tip| !tip.delivery_disclosure().is_delivered)
            .count();

        let mut out = String::new();
        out.push_str("# Learning-mode toggles and tip cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Learning-mode toggles: {} ({} not active learning)\n",
            self.toggles.len(),
            inactive
        ));
        out.push_str(&format!(
            "- Tip cards: {} ({} not delivered)\n",
            self.tip_cards.len(),
            withheld
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Learning-mode toggles\n\n");
        for toggle in &self.toggles {
            out.push_str(&format!(
                "- **{}** — state `{}`, scope `{}` → `{}`, deep link `{}`\n",
                toggle.toggle_label,
                toggle.learning_state.as_str(),
                toggle.scope.as_str(),
                toggle.activation_disclosure().activation_class.as_str(),
                toggle.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Tip cards\n\n");
        for tip in &self.tip_cards {
            out.push_str(&format!(
                "- **{}** — trigger `{}`, dismissal `{}` → `{}`, deep link `{}`\n",
                tip.tip_label,
                tip.trigger_class.as_str(),
                tip.dismissal_state.as_str(),
                tip.delivery_disclosure().delivery_class.as_str(),
                tip.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in learning-mode-toggle / tip-card export.
#[derive(Debug)]
pub enum LearningModeToggleTipCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LearningModeToggleTipCardViolation>),
}

impl fmt::Display for LearningModeToggleTipCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "learning mode toggle tip card export parse failed: {error}"
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
                    "learning mode toggle tip card export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for LearningModeToggleTipCardArtifactError {}

/// Validation failures emitted by [`LearningModeToggleTipCardControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LearningModeToggleTipCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No learning-mode toggles are present.
    TogglesMissing,
    /// A learning-mode toggle is incomplete.
    ToggleIncomplete,
    /// A learning-mode toggle carries the wrong frozen component class.
    ToggleWrongComponentClass,
    /// A learning-mode toggle misrepresents its derived activation state.
    ActivationMisrepresented,
    /// A paused toggle does not name its paused state.
    PausedNoteMissing,
    /// A sandboxed-active toggle does not name its sandboxed state.
    SandboxedNoteMissing,
    /// An inactive toggle does not name its inactive state.
    InactiveNoteMissing,
    /// A toggle does not name its scope / activation.
    ScopeAndActivationNoteMissing,
    /// A toggle does not name its scope label.
    ScopeLabelMissing,
    /// A toggle omits the mandatory `ResetLearning` action.
    ToggleActionsIncomplete,
    /// The learning-mode toggles do not cover every derived activation class.
    ActivationClassCoverageMissing,
    /// The learning-mode toggles do not cover every learning-mode state.
    LearningModeStateCoverageMissing,
    /// The learning-mode toggles do not cover every learning-mode scope.
    LearningModeScopeCoverageMissing,
    /// No tip cards are present.
    TipCardsMissing,
    /// A tip card is incomplete.
    TipCardIncomplete,
    /// A tip card carries the wrong frozen component class.
    TipCardWrongComponentClass,
    /// A tip card misrepresents its derived delivery state.
    DeliveryMisrepresented,
    /// A snoozed tip does not name its snoozed state.
    SnoozeNoteMissing,
    /// A withheld tip does not name its withheld state.
    WithheldNoteMissing,
    /// A tip card omits the mandatory `DismissTip` action.
    TipActionsIncomplete,
    /// The tip cards do not cover every derived delivery class.
    DeliveryClassCoverageMissing,
    /// The tip cards do not cover every tip trigger class.
    TipTriggerClassCoverageMissing,
    /// The tip cards do not cover every tip dismissal state.
    TipDismissalStateCoverageMissing,
    /// A control does not name its why-now context.
    WhyNowContextMissing,
    /// A control offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A control names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
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
    /// A control hides whether learning is active or what scope it changes.
    ActivationOrScopeHidden,
    /// A control implies a hidden apply or mutation.
    HiddenApplyOrMutationImplied,
    /// A control invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A control depends on an ephemeral coachmark or hidden routing.
    EphemeralCoachmarkOrHiddenRoutingUsed,
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

impl LearningModeToggleTipCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::TogglesMissing => "toggles_missing",
            Self::ToggleIncomplete => "toggle_incomplete",
            Self::ToggleWrongComponentClass => "toggle_wrong_component_class",
            Self::ActivationMisrepresented => "activation_misrepresented",
            Self::PausedNoteMissing => "paused_note_missing",
            Self::SandboxedNoteMissing => "sandboxed_note_missing",
            Self::InactiveNoteMissing => "inactive_note_missing",
            Self::ScopeAndActivationNoteMissing => "scope_and_activation_note_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::ToggleActionsIncomplete => "toggle_actions_incomplete",
            Self::ActivationClassCoverageMissing => "activation_class_coverage_missing",
            Self::LearningModeStateCoverageMissing => "learning_mode_state_coverage_missing",
            Self::LearningModeScopeCoverageMissing => "learning_mode_scope_coverage_missing",
            Self::TipCardsMissing => "tip_cards_missing",
            Self::TipCardIncomplete => "tip_card_incomplete",
            Self::TipCardWrongComponentClass => "tip_card_wrong_component_class",
            Self::DeliveryMisrepresented => "delivery_misrepresented",
            Self::SnoozeNoteMissing => "snooze_note_missing",
            Self::WithheldNoteMissing => "withheld_note_missing",
            Self::TipActionsIncomplete => "tip_actions_incomplete",
            Self::DeliveryClassCoverageMissing => "delivery_class_coverage_missing",
            Self::TipTriggerClassCoverageMissing => "tip_trigger_class_coverage_missing",
            Self::TipDismissalStateCoverageMissing => "tip_dismissal_state_coverage_missing",
            Self::WhyNowContextMissing => "why_now_context_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::PrivacyOrOfflineStateMasked => "privacy_or_offline_state_masked",
            Self::ActivationOrScopeHidden => "activation_or_scope_hidden",
            Self::HiddenApplyOrMutationImplied => "hidden_apply_or_mutation_implied",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::EphemeralCoachmarkOrHiddenRoutingUsed => {
                "ephemeral_coachmark_or_hidden_routing_used"
            }
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LearnabilityReviewIncomplete => "learnability_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable learning-mode-toggle / tip-card export.
pub fn current_learning_mode_toggle_tip_card_export(
) -> Result<LearningModeToggleTipCardControlsPacket, LearningModeToggleTipCardArtifactError> {
    let packet: LearningModeToggleTipCardControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-learning-mode-toggle-tip-card-proof/support_export.json"
        )))
        .map_err(LearningModeToggleTipCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LearningModeToggleTipCardArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &LearningModeToggleTipCardControlsPacket,
    violations: &mut Vec<LearningModeToggleTipCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF,
        LEARNING_MODE_TOGGLE_TIP_CARD_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_TIP_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(LearningModeToggleTipCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_toggles(
    packet: &LearningModeToggleTipCardControlsPacket,
    violations: &mut Vec<LearningModeToggleTipCardViolation>,
) {
    if packet.toggles.is_empty() {
        violations.push(LearningModeToggleTipCardViolation::TogglesMissing);
        return;
    }

    let mut activation_classes: BTreeSet<LearningActivationClass> = BTreeSet::new();
    let mut states: BTreeSet<M5LearningModeState> = BTreeSet::new();
    let mut scopes: BTreeSet<M5LearningModeScope> = BTreeSet::new();

    for toggle in &packet.toggles {
        let disclosure = toggle.activation_disclosure();
        activation_classes.insert(disclosure.activation_class);
        states.insert(toggle.learning_state);
        scopes.insert(toggle.scope);

        if toggle.toggle_id.trim().is_empty()
            || toggle.toggle_label.trim().is_empty()
            || toggle.fields_shown.is_empty()
            || toggle.surface_families.is_empty()
            || toggle.deployment_lines.is_empty()
            || toggle.consumer_surfaces.is_empty()
            || toggle.source_contract_refs.is_empty()
        {
            violations.push(LearningModeToggleTipCardViolation::ToggleIncomplete);
        }
        if toggle.component != M5LearningComponentFamily::LearningModeToggle {
            violations.push(LearningModeToggleTipCardViolation::ToggleWrongComponentClass);
        }
        if toggle.activation_class != disclosure.activation_class
            || toggle.claims_active != disclosure.is_active_learning
        {
            violations.push(LearningModeToggleTipCardViolation::ActivationMisrepresented);
        }
        if disclosure.needs_paused_note && toggle.paused_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::PausedNoteMissing);
        }
        if disclosure.needs_sandboxed_note && toggle.sandboxed_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::SandboxedNoteMissing);
        }
        if disclosure.needs_inactive_note && toggle.inactive_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::InactiveNoteMissing);
        }
        if toggle.scope_and_activation_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::ScopeAndActivationNoteMissing);
        }
        if toggle.scope_label.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::ScopeLabelMissing);
        }
        if !toggle.declares_mandatory_actions() {
            violations.push(LearningModeToggleTipCardViolation::ToggleActionsIncomplete);
        }
        validate_deep_link(
            toggle.offers_deep_link_action(),
            toggle.deep_link_kind,
            &toggle.deep_link_ref,
            &toggle.why_now_context,
            violations,
        );
        validate_common_control(
            &toggle.dispositions,
            &toggle.downgrade_triggers,
            toggle.declares_mandatory_labels(),
            &toggle.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: toggle.masks_privacy_or_offline_state,
                hides_activation_or_scope: toggle.hides_activation_or_scope,
                implies_hidden_apply_or_mutation: toggle.implies_hidden_apply_or_mutation,
                invents_alternate_state_label: toggle.invents_alternate_state_label,
                depends_on_ephemeral_coachmark_or_hidden_routing: toggle
                    .depends_on_ephemeral_coachmark_or_hidden_routing,
            },
            violations,
        );
    }

    for required in LearningActivationClass::ALL {
        if !activation_classes.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::ActivationClassCoverageMissing);
            break;
        }
    }
    for required in M5LearningModeState::ALL {
        if !states.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::LearningModeStateCoverageMissing);
            break;
        }
    }
    for required in M5LearningModeScope::ALL {
        if !scopes.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::LearningModeScopeCoverageMissing);
            break;
        }
    }
}

fn validate_tip_cards(
    packet: &LearningModeToggleTipCardControlsPacket,
    violations: &mut Vec<LearningModeToggleTipCardViolation>,
) {
    if packet.tip_cards.is_empty() {
        violations.push(LearningModeToggleTipCardViolation::TipCardsMissing);
        return;
    }

    let mut delivery_classes: BTreeSet<TipDeliveryClass> = BTreeSet::new();
    let mut triggers: BTreeSet<M5TipTriggerClass> = BTreeSet::new();
    let mut dismissals: BTreeSet<M5TipDismissalState> = BTreeSet::new();

    for tip in &packet.tip_cards {
        let disclosure = tip.delivery_disclosure();
        delivery_classes.insert(disclosure.delivery_class);
        triggers.insert(tip.trigger_class);
        dismissals.insert(tip.dismissal_state);

        if tip.tip_id.trim().is_empty()
            || tip.tip_label.trim().is_empty()
            || tip.fields_shown.is_empty()
            || tip.surface_families.is_empty()
            || tip.deployment_lines.is_empty()
            || tip.consumer_surfaces.is_empty()
            || tip.source_contract_refs.is_empty()
        {
            violations.push(LearningModeToggleTipCardViolation::TipCardIncomplete);
        }
        if tip.component != M5LearningComponentFamily::TipCard {
            violations.push(LearningModeToggleTipCardViolation::TipCardWrongComponentClass);
        }
        if tip.delivery_class != disclosure.delivery_class
            || tip.claims_delivered != disclosure.is_delivered
        {
            violations.push(LearningModeToggleTipCardViolation::DeliveryMisrepresented);
        }
        if disclosure.needs_snooze_note && tip.snoozed_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::SnoozeNoteMissing);
        }
        if disclosure.needs_withheld_note && tip.withheld_note.trim().is_empty() {
            violations.push(LearningModeToggleTipCardViolation::WithheldNoteMissing);
        }
        if !tip.declares_mandatory_actions() {
            violations.push(LearningModeToggleTipCardViolation::TipActionsIncomplete);
        }
        validate_deep_link(
            tip.offers_deep_link_action(),
            tip.deep_link_kind,
            &tip.deep_link_ref,
            &tip.why_now_context,
            violations,
        );
        validate_common_control(
            &tip.dispositions,
            &tip.downgrade_triggers,
            tip.declares_mandatory_labels(),
            &tip.accessibility_routes,
            ControlInvariants {
                masks_privacy_or_offline_state: tip.masks_privacy_or_offline_state,
                hides_activation_or_scope: tip.hides_activation_or_scope,
                implies_hidden_apply_or_mutation: tip.implies_hidden_apply_or_mutation,
                invents_alternate_state_label: tip.invents_alternate_state_label,
                depends_on_ephemeral_coachmark_or_hidden_routing: tip
                    .depends_on_ephemeral_coachmark_or_hidden_routing,
            },
            violations,
        );
    }

    for required in TipDeliveryClass::ALL {
        if !delivery_classes.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::DeliveryClassCoverageMissing);
            break;
        }
    }
    for required in M5TipTriggerClass::ALL {
        if !triggers.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::TipTriggerClassCoverageMissing);
            break;
        }
    }
    for required in M5TipDismissalState::ALL {
        if !dismissals.contains(&required) {
            violations.push(LearningModeToggleTipCardViolation::TipDismissalStateCoverageMissing);
            break;
        }
    }
}

/// Validates the why-now context and stable deep-link truth shared by both control vectors.
///
/// A control that offers a deep-link action must name a resolvable deep-link kind, a control
/// that names a resolvable kind must carry its stable reference, and every control must name
/// why it is relevant now — so a next step is never an ephemeral coachmark or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    why_now_context: &str,
    violations: &mut Vec<LearningModeToggleTipCardViolation>,
) {
    if why_now_context.trim().is_empty() {
        violations.push(LearningModeToggleTipCardViolation::WhyNowContextMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(LearningModeToggleTipCardViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(LearningModeToggleTipCardViolation::DeepLinkRefMissing);
    }
}

/// The five hard-invariant bools every control must keep `false`.
struct ControlInvariants {
    masks_privacy_or_offline_state: bool,
    hides_activation_or_scope: bool,
    implies_hidden_apply_or_mutation: bool,
    invents_alternate_state_label: bool,
    depends_on_ephemeral_coachmark_or_hidden_routing: bool,
}

/// Validates the axes shared by both control vectors.
fn validate_common_control(
    dispositions: &[M5LearningDisposition],
    downgrade_triggers: &[M5LearningDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5LearningAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<LearningModeToggleTipCardViolation>,
) {
    if dispositions.is_empty() {
        violations.push(LearningModeToggleTipCardViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(LearningModeToggleTipCardViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(LearningModeToggleTipCardViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5LearningAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(LearningModeToggleTipCardViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_privacy_or_offline_state {
        violations.push(LearningModeToggleTipCardViolation::PrivacyOrOfflineStateMasked);
    }
    if invariants.hides_activation_or_scope {
        violations.push(LearningModeToggleTipCardViolation::ActivationOrScopeHidden);
    }
    if invariants.implies_hidden_apply_or_mutation {
        violations.push(LearningModeToggleTipCardViolation::HiddenApplyOrMutationImplied);
    }
    if invariants.invents_alternate_state_label {
        violations.push(LearningModeToggleTipCardViolation::AlternateStateLabelInvented);
    }
    if invariants.depends_on_ephemeral_coachmark_or_hidden_routing {
        violations.push(LearningModeToggleTipCardViolation::EphemeralCoachmarkOrHiddenRoutingUsed);
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
