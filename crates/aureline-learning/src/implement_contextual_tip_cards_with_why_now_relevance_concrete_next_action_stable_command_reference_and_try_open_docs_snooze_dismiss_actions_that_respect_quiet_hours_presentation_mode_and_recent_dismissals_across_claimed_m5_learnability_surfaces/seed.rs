//! Canonical seed builders for the M5 contextual-tip-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical contextual-tip-card primitive packet.
pub const M5_CONTEXTUAL_TIP_CARD_PACKET_ID: &str = "m5-contextual-tip-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked contextual-tip-card resolution case from a full tip state.
#[allow(clippy::too_many_arguments)]
fn tip_case(
    trigger_class: M5TipTriggerClass,
    command_backing: M5CommandBackingState,
    dismissal_state: M5TipDismissalState,
    quiet_hours_active: bool,
    presentation_mode_active: bool,
    recently_dismissed: bool,
    underlying_action_requires_approval: bool,
    why_now_relevance: &str,
    next_action_command_ref: &str,
    tip_identity_ref: &str,
) -> M5ContextualTipCardResolutionCase {
    M5ContextualTipCardResolutionCase::resolved(M5ContextualTipCardResolutionInput {
        trigger_class,
        command_backing,
        dismissal_state,
        quiet_hours_active,
        presentation_mode_active,
        recently_dismissed,
        underlying_action_requires_approval,
        why_now_relevance: why_now_relevance.to_owned(),
        next_action_command_ref: next_action_command_ref.to_owned(),
        tip_identity_ref: tip_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full tip-card anatomy, trigger class,
/// command-backing, dismissal-state, delivery-posture, action, export-field, and accessibility
/// parity every consumer carries.
fn base_row(
    consumer_surface: M5ContextualTipConsumerSurface,
    qualification: M5TeachingQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    tip_examples: Vec<M5ContextualTipCardResolutionCase>,
) -> M5ContextualTipConsumerRow {
    M5ContextualTipConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ContextualTipAnatomyPart::ALL.to_vec(),
        trigger_classes: M5TipTriggerClass::ALL.to_vec(),
        command_backing_states: M5CommandBackingState::ALL.to_vec(),
        dismissal_states: M5TipDismissalState::ALL.to_vec(),
        delivery_postures: M5ContextualTipDeliveryPosture::ALL.to_vec(),
        tip_actions: M5ContextualTipAction::ALL.to_vec(),
        export_fields: M5ContextualTipExportField::ALL.to_vec(),
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TeachingConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TeachingDowngradeTrigger::TipCommandBindingUnstated,
            M5TeachingDowngradeTrigger::CommandBackingHidden,
            M5TeachingDowngradeTrigger::AlternateStateLabelInvented,
            M5TeachingDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
            M5_CONTEXTUAL_TIP_CARD_COMMAND_DESCRIPTOR_REF,
            M5_CONTEXTUAL_TIP_CARD_PRESENTATION_MODE_REF,
        ]),
        tip_examples,
        masks_command_binding: false,
        hijacks_workflow_as_blocking_tour: false,
        ignores_quiet_hours_or_dismissals: false,
        bypasses_underlying_trust_limits: false,
    }
}

fn rows() -> Vec<M5ContextualTipConsumerRow> {
    use M5CommandBackingState as Backing;
    use M5ContextualTipConsumerSurface as Surface;
    use M5TeachingQualificationClass as Qual;
    use M5TipDismissalState as Dismissal;
    use M5TipTriggerClass as Trigger;

    vec![
        // 1. First-run onboarding panel — a first-encounter tip whose next action is a bound
        //    command (delivered actionable, offering try/open-docs/snooze/dismiss), and an
        //    idle-time hint with no command backing (delivered as an informational hint).
        base_row(
            Surface::FirstRunOnboardingPanel,
            Qual::Stable,
            "First-run onboarding panel owner",
            "The first-run onboarding panel renders the shared contextual tip card so a first-encounter tip whose concrete next action is a bound command is delivered in place as an actionable tip that can be tried, snoozed, or dismissed without leaving the task, and an idle-time hint with no command backing is delivered as an informational tip that still teaches and stays reversible — never a blocking tour",
            "evidence:m5-tip-card-first-run-onboarding:001",
            vec![
                tip_case(
                    Trigger::FirstEncounter,
                    Backing::BoundCommand,
                    Dismissal::Dismissible,
                    false,
                    false,
                    false,
                    false,
                    "You just opened your first project — jump straight to the command palette to find any action by name",
                    "command:command-palette.open",
                    "tip:onboarding:command-palette",
                ),
                tip_case(
                    Trigger::IdleHint,
                    Backing::NoCommandBacking,
                    Dismissal::Dismissible,
                    false,
                    false,
                    false,
                    false,
                    "While you get your bearings, remember the activity bar groups related tools together",
                    "docs:onboarding.activity-bar",
                    "tip:onboarding:activity-bar-orientation",
                ),
            ],
        ),
        // 2. Guided-tour overlay — a feature-discovery tip whose underlying action requires
        //    approval (delivered actionable but offering request-approval instead of a direct
        //    try), and a mode-change tip the user snoozed (snoozed for later, still dismissible).
        base_row(
            Surface::GuidedTourOverlay,
            Qual::Stable,
            "Guided-tour overlay owner",
            "The guided-tour overlay renders the shared contextual tip card so a feature-discovery tip whose underlying action requires approval is delivered as an actionable tip that offers request-approval rather than running the action directly — never bypassing the trust boundary — and a mode-change tip the user snoozed stays snoozed for later while remaining permanently dismissible",
            "evidence:m5-tip-card-guided-tour-overlay:001",
            vec![
                tip_case(
                    Trigger::FeatureDiscovery,
                    Backing::DeepLinkCommand,
                    Dismissal::Dismissible,
                    false,
                    false,
                    false,
                    true,
                    "You can share this workspace — sharing changes who can see your work, so it runs through an approval step",
                    "command:workspace.share",
                    "tip:tour:workspace-share",
                ),
                tip_case(
                    Trigger::ModeChange,
                    Backing::PaletteEntry,
                    Dismissal::Snoozed,
                    false,
                    false,
                    false,
                    false,
                    "You switched to learning mode — a tour of the changed layout is ready whenever you are",
                    "command:tour.learning-mode-layout",
                    "tip:tour:learning-mode-layout",
                ),
            ],
        ),
        // 3. Command-palette hint — a contextual follow-up tip withheld because quiet hours are
        //    active (nothing shown, nothing spammed), and an error-recovery tip bound to a
        //    command and persistent until acted (delivered actionable).
        base_row(
            Surface::CommandPaletteHint,
            Qual::Stable,
            "Command-palette hint owner",
            "The command-palette hint renders the shared contextual tip card so a contextual follow-up tip is withheld while quiet hours are active — respecting the do-not-disturb window rather than interrupting — and an error-recovery tip bound to a command and persistent until acted is delivered as an actionable tip that names the exact recovery command and stays reversible",
            "evidence:m5-tip-card-command-palette-hint:001",
            vec![
                tip_case(
                    Trigger::ContextualFollowup,
                    Backing::KeybindingRoute,
                    Dismissal::Dismissible,
                    true,
                    false,
                    false,
                    false,
                    "Right after that search you often refine results — a keybinding narrows them in place",
                    "command:search.refine",
                    "tip:palette:search-refine",
                ),
                tip_case(
                    Trigger::ErrorRecovery,
                    Backing::BoundCommand,
                    Dismissal::PersistentUntilActed,
                    false,
                    false,
                    false,
                    false,
                    "That command failed because the buffer was unsaved — save it, then retry from here",
                    "command:file.save-and-retry",
                    "tip:palette:save-and-retry",
                ),
            ],
        ),
        // 4. Inline editor tip — a mode-change tip withheld while presentation mode is active
        //    (never interrupts a live demo), and an idle hint that was already dismissed
        //    (withheld as already resolved, never re-shown).
        base_row(
            Surface::InlineEditorTip,
            Qual::Stable,
            "Inline editor tip owner",
            "The inline editor tip renders the shared contextual tip card so a mode-change tip is withheld while presentation mode is active — never interrupting a live demo or screen share — and an idle hint the user already dismissed is withheld as already resolved so it is never re-shown, keeping tips non-spammy",
            "evidence:m5-tip-card-inline-editor:001",
            vec![
                tip_case(
                    Trigger::ModeChange,
                    Backing::BoundCommand,
                    Dismissal::Dismissible,
                    false,
                    true,
                    false,
                    false,
                    "You entered zen mode — a command toggles the minimap back if you miss it",
                    "command:view.toggle-minimap",
                    "tip:inline:zen-minimap",
                ),
                tip_case(
                    Trigger::IdleHint,
                    Backing::BoundCommand,
                    Dismissal::Dismissed,
                    false,
                    false,
                    false,
                    false,
                    "Multi-cursor editing speeds up repetitive edits — add a cursor on the next match",
                    "command:editor.add-cursor-next-match",
                    "tip:inline:multi-cursor",
                ),
            ],
        ),
        // 5. Support tip export — a first-encounter tip withheld because a like tip was
        //    recently dismissed (non-spammy), and a contextual follow-up tip bound to a
        //    deep-link command (delivered actionable); the same tip a support agent reads.
        base_row(
            Surface::SupportTipExport,
            Qual::Stable,
            "Support tip export owner",
            "The support tip export renders the shared contextual tip card so a first-encounter tip is withheld because a like tip was recently dismissed — proving the non-spammy guard survives export — and a contextual follow-up tip bound to a deep-link command is delivered as an actionable tip whose export reconstructs its why-now relevance, command reference, and delivery posture without leaking raw docs bodies",
            "evidence:m5-tip-card-support-export:001",
            vec![
                tip_case(
                    Trigger::FirstEncounter,
                    Backing::BoundCommand,
                    Dismissal::Dismissible,
                    false,
                    false,
                    true,
                    false,
                    "Keyboard shortcuts speed up navigation — open the shortcut cheat sheet",
                    "command:help.keyboard-shortcuts",
                    "tip:support:keyboard-shortcuts",
                ),
                tip_case(
                    Trigger::ContextualFollowup,
                    Backing::DeepLinkCommand,
                    Dismissal::Dismissible,
                    false,
                    false,
                    false,
                    false,
                    "After importing settings, review what changed — open the migration report to see the diff",
                    "command:migration.open-report",
                    "tip:support:migration-report",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5ContextualTipCardGovernanceReview {
    M5ContextualTipCardGovernanceReview {
        tip_card_shows_why_now_relevance: true,
        tip_card_shows_concrete_next_action: true,
        tip_card_shows_stable_command_reference: true,
        tip_card_shows_dismissal_and_snooze_controls: true,
        tips_never_hijack_workflow_as_blocking_tour: true,
        tips_respect_quiet_hours: true,
        tips_respect_presentation_mode: true,
        tips_respect_recent_dismissals: true,
        tips_honor_underlying_trust_and_approval_limits: true,
        tips_remain_reversible_and_command_backed: true,
        users_learn_without_leaving_task: true,
        tips_stable_across_deployment_lines: true,
        tips_stable_across_consumer_surfaces: true,
        every_tip_declares_accessibility_route: true,
        support_export_reconstructs_tip_truth: true,
        later_rows_cannot_invent_parallel_tip_vocabulary: true,
    }
}

fn consumer_projection() -> M5ContextualTipCardConsumerProjection {
    M5ContextualTipCardConsumerProjection {
        learnability_surfaces_consume_tip_vocabulary: true,
        delivery_posture_reads_single_source: true,
        action_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5ContextualTipCardProofFreshness {
    M5ContextualTipCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ContextualTipCardReleasePosture {
    M5ContextualTipCardReleasePosture {
        release_packet_ref: M5_CONTEXTUAL_TIP_CARD_ARTIFACT_REF.to_owned(),
        contextual_tip_audit_ref: M5_CONTEXTUAL_TIP_CARD_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
        M5_CONTEXTUAL_TIP_CARD_DOC_REF,
        M5_CONTEXTUAL_TIP_CARD_COMPONENT_MATRIX_REF,
        M5_CONTEXTUAL_TIP_CARD_COMMAND_DESCRIPTOR_REF,
        M5_CONTEXTUAL_TIP_CARD_PRESENTATION_MODE_REF,
    ])
}

/// Builds the canonical M5 contextual-tip-card packet.
pub fn seeded_m5_contextual_tip_card_packet() -> M5ContextualTipCardPacket {
    M5ContextualTipCardPacket::new(M5ContextualTipCardPacketInput {
        packet_id: M5_CONTEXTUAL_TIP_CARD_PACKET_ID.to_owned(),
        matrix_label:
            "M5 contextual-tip-card primitive: tip trigger class, command-backing state, dismissal state, why-now relevance, concrete next action, stable command reference, derived delivery posture (delivered-actionable/delivered-informational/snoozed-for-later/withheld-for-quiet-hours/withheld-for-presentation-mode/withheld-already-resolved), and bounded try/request-approval/open-docs/snooze/dismiss actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5ContextualTipCardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the command-palette hint consumer is held at Beta because a slice of
/// palette hints does not yet render the keyboard-route cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed(
) -> M5ContextualTipCardPacket {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.packet_id = "m5-contextual-tip-card-primitive:command-palette-hint-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ContextualTipConsumerSurface::CommandPaletteHint)
        .expect("command-palette-hint row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support tip export consumer is narrowed to Preview pending
/// quiet-hours / recent-dismissal delivery-posture parity proof across every deployment; every
/// consumer stays visible.
pub fn seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed(
) -> M5ContextualTipCardPacket {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.packet_id =
        "m5-contextual-tip-card-primitive:support-tip-export-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ContextualTipConsumerSurface::SupportTipExport)
        .expect("support-tip-export row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
