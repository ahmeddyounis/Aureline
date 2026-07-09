//! Canonical seed builders for the learning-mode-toggle / tip-card controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical learning-mode-toggle / tip-card packet.
pub const LEARNING_MODE_TOGGLE_TIP_CARD_PACKET_ID: &str =
    "m5-learning-mode-toggle-tip-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn toggle_source_refs() -> Vec<String> {
    strings(&[
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
    ])
}

fn tip_source_refs() -> Vec<String> {
    strings(&[M5_TIP_CARD_SCHEMA_REF, M5_LEARNING_COMPONENT_SCHEMA_REF])
}

fn toggle_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::LearningModeStateUnstated,
        M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn tip_downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::TipCommandBindingUnstated,
        M5LearningDowngradeTrigger::CachedStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

/// Builds a learning-mode toggle, deriving the activation class, the active claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn toggle(
    toggle_id: &str,
    toggle_label: &str,
    learning_state: M5LearningModeState,
    scope: M5LearningModeScope,
    scope_label: &str,
    why_now_context: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    toggle_actions: Vec<LearningToggleAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> LearningModeToggle {
    let disclosure = resolve_learning_activation(learning_state);
    LearningModeToggle {
        component: M5LearningComponentFamily::LearningModeToggle,
        toggle_id: toggle_id.to_owned(),
        toggle_label: toggle_label.to_owned(),
        learning_state,
        scope,
        scope_label: scope_label.to_owned(),
        activation_class: disclosure.activation_class,
        claims_active: disclosure.is_active_learning,
        paused_note: if disclosure.needs_paused_note {
            "Learning is paused; resume or reset it at any time".to_owned()
        } else {
            String::new()
        },
        sandboxed_note: if disclosure.needs_sandboxed_note {
            "Only sandboxed practice is active; nothing touches live state".to_owned()
        } else {
            String::new()
        },
        inactive_note: if disclosure.needs_inactive_note {
            format!(
                "Learning is {} (inactive); enable it to turn learnability on",
                learning_state.as_str()
            )
        } else {
            String::new()
        },
        scope_and_activation_note: format!(
            "Scoped to {}; activation {}",
            scope.as_str(),
            disclosure.activation_class.as_str()
        ),
        why_now_context: why_now_context.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        toggle_actions,
        dispositions,
        downgrade_triggers: toggle_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "toggle_label",
            "learning_state",
            "scope",
            "activation_class",
            "why_now_context",
            "deep_link_kind",
        ]),
        source_contract_refs: toggle_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_activation_or_scope: false,
        implies_hidden_apply_or_mutation: false,
        invents_alternate_state_label: false,
        depends_on_ephemeral_coachmark_or_hidden_routing: false,
    }
}

/// Builds a tip card, deriving the delivery class, the delivered claim, and the required
/// notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn tip(
    tip_id: &str,
    tip_label: &str,
    trigger_class: M5TipTriggerClass,
    dismissal_state: M5TipDismissalState,
    why_now_context: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    tip_actions: Vec<TipCardAction>,
    dispositions: Vec<M5LearningDisposition>,
) -> TipCard {
    let disclosure = resolve_tip_delivery(dismissal_state);
    TipCard {
        component: M5LearningComponentFamily::TipCard,
        tip_id: tip_id.to_owned(),
        tip_label: tip_label.to_owned(),
        trigger_class,
        dismissal_state,
        delivery_class: disclosure.delivery_class,
        claims_delivered: disclosure.is_delivered,
        snoozed_note: if disclosure.needs_snooze_note {
            "Snoozed for later; reopen it from Help or the command system".to_owned()
        } else {
            String::new()
        },
        withheld_note: if disclosure.needs_withheld_note {
            format!(
                "Tip is {} and off screen; reopen it from Help or the command system",
                dismissal_state.as_str()
            )
        } else {
            String::new()
        },
        why_now_context: why_now_context.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        tip_actions,
        dispositions,
        downgrade_triggers: tip_downgrade_triggers(),
        required_labels: M5LearningRequiredLabel::ALL.to_vec(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "tip_label",
            "trigger_class",
            "dismissal_state",
            "delivery_class",
            "why_now_context",
            "deep_link_kind",
        ]),
        source_contract_refs: tip_source_refs(),
        masks_privacy_or_offline_state: false,
        hides_activation_or_scope: false,
        implies_hidden_apply_or_mutation: false,
        invents_alternate_state_label: false,
        depends_on_ephemeral_coachmark_or_hidden_routing: false,
    }
}

fn toggles() -> Vec<LearningModeToggle> {
    use DeepLinkKind as Link;
    use LearningToggleAction as Action;
    use M5LearningDisposition as Disp;
    use M5LearningModeScope as Scope;
    use M5LearningModeState as Mode;

    vec![
        // 1. On, workspace-scoped → active.
        toggle(
            "toggle-workspace-on",
            "Learning mode (this workspace)",
            Mode::On,
            Scope::Workspace,
            "This workspace",
            "You just joined this workspace; learning mode explains nearby actions in place",
            Link::CommandReference,
            "command:learning.enable",
            vec![
                Action::PauseLearning,
                Action::SnoozeLearning,
                Action::ResetLearning,
                Action::ChangeScope,
                Action::OpenDeepLink,
            ],
            vec![Disp::LearningOn],
        ),
        // 2. Per feature family → scoped-active.
        toggle(
            "toggle-feature-family",
            "Learning mode (review feature family)",
            Mode::PerFeatureFamily,
            Scope::FeatureFamily,
            "Review feature family",
            "You are learning the review feature family; learning stays scoped to it",
            Link::CommandReference,
            "command:learning.scope.feature-family",
            vec![
                Action::PauseLearning,
                Action::ResetLearning,
                Action::ChangeScope,
                Action::OpenDeepLink,
            ],
            vec![Disp::LearningOn],
        ),
        // 3. Sandboxed only → sandboxed-active (needs sandboxed note).
        toggle(
            "toggle-sandboxed",
            "Learning mode (sandbox practice)",
            Mode::SandboxedOnly,
            Scope::Session,
            "This session",
            "Practicing in a sandbox this session; nothing touches live state",
            Link::DocsAnchor,
            "docs:learning/sandbox",
            vec![
                Action::PauseLearning,
                Action::ResetLearning,
                Action::ChangeScope,
                Action::OpenDeepLink,
            ],
            vec![Disp::Sandboxed],
        ),
        // 4. Paused, global → paused (needs paused note).
        toggle(
            "toggle-global-paused",
            "Learning mode (all surfaces)",
            Mode::Paused,
            Scope::Global,
            "All surfaces",
            "You paused learning globally; resume or reset it whenever you like",
            Link::HelpTopic,
            "help:learning/pause-resume",
            vec![
                Action::EnableLearning,
                Action::ResetLearning,
                Action::OpenDeepLink,
            ],
            vec![Disp::Paused],
        ),
        // 5. Off, one surface → inactive (needs inactive note).
        toggle(
            "toggle-surface-off",
            "Learning mode (editor surface)",
            Mode::Off,
            Scope::Surface,
            "Editor surface",
            "Learning is off on this surface; turn it on to see in-place explanations",
            Link::FileLocation,
            "file:settings/learning.toml",
            vec![
                Action::EnableLearning,
                Action::ResetLearning,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalOnly],
        ),
        // 6. Ended, unavailable → inactive (needs inactive note).
        toggle(
            "toggle-session-ended",
            "Learning mode (ended session)",
            Mode::Ended,
            Scope::Unavailable,
            "Not available on this build",
            "Your learning session ended; start a new one to keep learning",
            Link::HelpTopic,
            "help:learning/session-ended",
            vec![
                Action::EnableLearning,
                Action::ResetLearning,
                Action::ChangeScope,
                Action::OpenDeepLink,
            ],
            vec![Disp::NotInstalled],
        ),
    ]
}

fn tip_cards() -> Vec<TipCard> {
    use DeepLinkKind as Link;
    use M5LearningDisposition as Disp;
    use M5TipDismissalState as Dismissal;
    use M5TipTriggerClass as Trigger;
    use TipCardAction as Action;

    vec![
        // 1. First encounter, dismissible → delivered.
        tip(
            "tip-first-encounter",
            "Open the exact object this references",
            Trigger::FirstEncounter,
            Dismissal::Dismissible,
            "First time here: open lands on the exact object, never a generic page",
            Link::CommandReference,
            "command:review.open",
            vec![
                Action::TryNextAction,
                Action::OpenDeepLink,
                Action::SnoozeTip,
                Action::DismissTip,
            ],
            vec![Disp::Replayable],
        ),
        // 2. Feature discovery, persistent until acted → delivered-persistent.
        tip(
            "tip-feature-discovery",
            "Jump to the next diff hunk",
            Trigger::FeatureDiscovery,
            Dismissal::PersistentUntilActed,
            "You have unreviewed hunks; a command jumps to the next one",
            Link::CommandReference,
            "command:diff.next-hunk",
            vec![
                Action::TryNextAction,
                Action::OpenCommandReference,
                Action::DismissTip,
            ],
            vec![Disp::NoHiddenApply],
        ),
        // 3. Error recovery, snoozed → snoozed (needs snooze note).
        tip(
            "tip-error-recovery",
            "Recover from the last failed step",
            Trigger::ErrorRecovery,
            Dismissal::Snoozed,
            "The last step failed; the docs show the exact recovery command",
            Link::DocsAnchor,
            "docs:tips/error-recovery",
            vec![
                Action::OpenDeepLink,
                Action::DismissTip,
                Action::ReopenFromHelp,
            ],
            vec![Disp::Cached],
        ),
        // 4. Mode change, dismissed → withheld (needs withheld note).
        tip(
            "tip-mode-change",
            "What changed when you switched modes",
            Trigger::ModeChange,
            Dismissal::Dismissed,
            "You switched modes; this explains what that changed",
            Link::HelpTopic,
            "help:tips/mode-change",
            vec![Action::DismissTip, Action::ReopenFromHelp],
            vec![Disp::LocalOnly],
        ),
        // 5. Idle hint, auto-expired → withheld (needs withheld note).
        tip(
            "tip-idle-hint",
            "A quiet hint for when you are ready",
            Trigger::IdleHint,
            Dismissal::AutoExpired,
            "An idle hint that expired on its own; reopen it any time",
            Link::DocsAnchor,
            "docs:tips/idle",
            vec![Action::DismissTip, Action::ReopenFromHelp],
            vec![Disp::NotInstalled],
        ),
        // 6. Contextual follow-up, suppressed by preference → withheld (needs withheld note).
        tip(
            "tip-contextual-followup",
            "A follow-up to your last action",
            Trigger::ContextualFollowup,
            Dismissal::SuppressedByPreference,
            "You turned these follow-ups off; reopen them from Help when you want",
            Link::HelpTopic,
            "help:tips/followup",
            vec![Action::DismissTip, Action::ReopenFromHelp],
            vec![Disp::Paused],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5LearningDowngradeTrigger> {
    vec![
        M5LearningDowngradeTrigger::LearningModeStateUnstated,
        M5LearningDowngradeTrigger::TipCommandBindingUnstated,
        M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
        M5LearningDowngradeTrigger::CachedStateHidden,
        M5LearningDowngradeTrigger::NotInstalledStateHidden,
        M5LearningDowngradeTrigger::AlternateStateLabelInvented,
        M5LearningDowngradeTrigger::ProofStale,
    ]
}

fn learnability_review() -> LearningModeToggleTipCardReview {
    LearningModeToggleTipCardReview {
        toggle_shows_activation: true,
        toggle_shows_scope: true,
        toggle_offers_pause_snooze_reset: true,
        tip_shows_why_now_context: true,
        tip_stays_optional_and_dismissible: true,
        tip_reopenable_from_help_or_commands: true,
        activation_and_delivery_derived_never_asserted: true,
        inactive_never_shown_as_active: true,
        withheld_never_shown_as_delivered: true,
        every_next_step_names_stable_deep_link: true,
        no_control_depends_on_ephemeral_coachmark_or_hidden_routing: true,
        no_control_widens_trust_or_mutating_authority: true,
        progress_user_owned_and_default_local: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> LearningModeToggleTipCardConsumerProjection {
    LearningModeToggleTipCardConsumerProjection {
        learning_mode_panel_reads_single_source: true,
        onboarding_tip_surface_reads_single_source: true,
        activation_and_scope_visible_before_tap: true,
        why_now_context_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> LearningModeToggleTipCardProofFreshness {
    LearningModeToggleTipCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF,
        LEARNING_MODE_TOGGLE_TIP_CARD_DOC_REF,
        M5_LEARNING_COMPONENT_SCHEMA_REF,
        M5_LEARNING_COMPONENT_DOC_REF,
        M5_LEARNING_MODE_TOGGLE_SCHEMA_REF,
        M5_TIP_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical learning-mode-toggle / tip-card controls packet.
pub fn seeded_learning_mode_toggle_tip_card_controls() -> LearningModeToggleTipCardControlsPacket {
    LearningModeToggleTipCardControlsPacket::new(LearningModeToggleTipCardControlsPacketInput {
        packet_id: LEARNING_MODE_TOGGLE_TIP_CARD_PACKET_ID.to_owned(),
        surface_label:
            "M5 learning-mode toggles and tip cards: opt-in learning state, user/workspace/feature-family scope, pause/snooze/reset actions, why-now context, and stable command/file/docs deep links across claimed onboarding and help surfaces"
                .to_owned(),
        toggles: toggles(),
        tip_cards: tip_cards(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        learnability_review: learnability_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a paused learning-mode toggle that must never read as
/// active. Every activation class, learning-mode state, and scope stays covered so the
/// fixture validates on its own.
pub fn seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused(
) -> LearningModeToggleTipCardControlsPacket {
    let mut packet = seeded_learning_mode_toggle_tip_card_controls();
    packet.packet_id =
        "m5-learning-mode-toggle-tip-card-controls:fixture:learning-mode-toggle-paused".to_owned();
    packet.surface_label =
        "M5 learning-mode toggles: a paused toggle never reads as active learning".to_owned();
    packet
}

/// Scenario fixture: spotlights a withheld tip card that must never read as delivered yet
/// stays reopenable from Help. Every delivery class, trigger class, and dismissal state stays
/// covered so the fixture validates on its own.
pub fn seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld(
) -> LearningModeToggleTipCardControlsPacket {
    let mut packet = seeded_learning_mode_toggle_tip_card_controls();
    packet.packet_id =
        "m5-learning-mode-toggle-tip-card-controls:fixture:tip-card-withheld".to_owned();
    packet.surface_label =
        "M5 tip cards: a withheld tip never reads as delivered yet stays reopenable".to_owned();
    packet
}
