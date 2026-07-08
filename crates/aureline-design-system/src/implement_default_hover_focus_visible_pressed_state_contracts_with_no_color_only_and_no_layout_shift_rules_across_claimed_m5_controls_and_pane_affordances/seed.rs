//! Canonical seed builders for the M5 interactive-state-contract primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical interactive-state-contract primitive packet.
pub const M5_INTERACTIVE_STATE_CONTRACT_PACKET_ID: &str =
    "m5-interactive-state-contract-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked interactive-state resolution case from a full control state.
#[allow(clippy::too_many_arguments)]
fn state_case(
    control_kind: M5InteractiveControlKind,
    interactive_state: M5SharedComponentStateClass,
    pointer_available: bool,
    keyboard_focus_origin: bool,
    reduced_motion_active: bool,
    high_contrast_active: bool,
    control_identity_ref: &str,
    state_style_ref: &str,
) -> M5InteractiveStateResolutionCase {
    M5InteractiveStateResolutionCase::resolved(M5InteractiveStateResolutionInput {
        control_kind,
        interactive_state,
        pointer_available,
        keyboard_focus_origin,
        reduced_motion_active,
        high_contrast_active,
        control_identity_ref: control_identity_ref.to_owned(),
        state_style_ref: state_style_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full interactive-state anatomy, interactive
/// state, presentation, non-color cue, interaction-route, export-field, label, and accessibility
/// parity every control carries.
fn base_row(
    control_kind: M5InteractiveControlKind,
    qualification: M5ComponentStateQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    state_examples: Vec<M5InteractiveStateResolutionCase>,
) -> M5InteractiveControlRow {
    M5InteractiveControlRow {
        control_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5InteractiveStateAnatomyPart::ALL.to_vec(),
        interactive_states: interactive_states(),
        presentations: M5InteractiveStatePresentation::ALL.to_vec(),
        non_color_cues: M5InteractiveStateCue::ALL.to_vec(),
        interaction_input_routes: M5InteractionInputRoute::ALL.to_vec(),
        export_fields: M5InteractiveStateExportField::ALL.to_vec(),
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        required_labels: M5ComponentStateRequiredLabel::ALL.to_vec(),
        consumer_surfaces: M5ComponentStateConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComponentStateDowngradeTrigger::ColorOnlyTreatment,
            M5ComponentStateDowngradeTrigger::KeyboardRouteMissing,
            M5ComponentStateDowngradeTrigger::AlternateStateLabelInvented,
            M5ComponentStateDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
            M5_INTERACTIVE_STATE_CONTRACT_FOCUS_SELECTION_REF,
            M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_CONTRACT_REF,
        ]),
        state_examples,
        signals_state_by_color_only: false,
        shifts_layout_on_state_change: false,
        changes_hit_target_on_state_change: false,
        invents_private_state_name: false,
    }
}

fn rows() -> Vec<M5InteractiveControlRow> {
    use M5ComponentStateQualificationClass as Qual;
    use M5InteractiveControlKind as Control;
    use M5SharedComponentStateClass as State;

    vec![
        // 1. Push button — the resting default treatment and the pressed/active treatment, so the
        //    two most common interactive states of a labelled button read the same across surfaces
        //    with a press inset rather than a color-only change.
        base_row(
            Control::PushButton,
            Qual::Stable,
            "Push button owner",
            "The push button renders the shared interactive-state contract so its resting default treatment and its pressed/active treatment are both driven by the shared token hooks — the press is carried by an inset and a border shift, never by hue alone, and the hit target and layout never move as the button is pressed",
            "evidence:m5-interactive-state-push-button:001",
            vec![
                state_case(
                    Control::PushButton,
                    State::Default,
                    true,
                    false,
                    false,
                    false,
                    "control:command-bar.primary-action",
                    "token:state.push_button.default",
                ),
                state_case(
                    Control::PushButton,
                    State::PressedActive,
                    true,
                    false,
                    false,
                    false,
                    "control:command-bar.primary-action",
                    "token:state.push_button.pressed",
                ),
            ],
        ),
        // 2. Icon button — the pointer-hover treatment and the keyboard focus-visible ring, so an
        //    icon-only control carries hover through elevation and a cursor affordance and carries
        //    keyboard focus through a visible ring rather than a color swap.
        base_row(
            Control::IconButton,
            Qual::Stable,
            "Icon button owner",
            "The icon button renders the shared interactive-state contract so its hover treatment carries meaning through an elevation shift and a pointer cursor and its keyboard focus arrives with a visible focus ring — the icon label stays present in every state and the focus ring is shown because focus arrived from the keyboard",
            "evidence:m5-interactive-state-icon-button:001",
            vec![
                state_case(
                    Control::IconButton,
                    State::Hover,
                    true,
                    false,
                    false,
                    false,
                    "control:toolbar.split-editor",
                    "token:state.icon_button.hover",
                ),
                state_case(
                    Control::IconButton,
                    State::FocusVisible,
                    false,
                    true,
                    false,
                    false,
                    "control:toolbar.split-editor",
                    "token:state.icon_button.focus_visible",
                ),
            ],
        ),
        // 3. Menu item — the resting default treatment and the pointer-hover treatment, proving the
        //    hover row highlight is carried by a border/outline and elevation shift under
        //    reduced-motion, never a color-only fill.
        base_row(
            Control::MenuItem,
            Qual::Stable,
            "Menu item owner",
            "The menu item renders the shared interactive-state contract so its resting default treatment and its pointer-hover highlight are driven by the shared token hooks — the hover highlight is carried by a border and elevation shift that stays legible under reduced-motion, and the highlight never moves the row or its hit target",
            "evidence:m5-interactive-state-menu-item:001",
            vec![
                state_case(
                    Control::MenuItem,
                    State::Default,
                    true,
                    false,
                    false,
                    false,
                    "control:context-menu.rename-symbol",
                    "token:state.menu_item.default",
                ),
                state_case(
                    Control::MenuItem,
                    State::Hover,
                    true,
                    false,
                    true,
                    false,
                    "control:context-menu.rename-symbol",
                    "token:state.menu_item.hover",
                ),
            ],
        ),
        // 4. Pane splitter — the keyboard focus-visible ring and the pressed/active drag treatment,
        //    so the splitter affordance is keyboard-reachable and shows a focus ring, and its
        //    active drag is carried by an inset rather than color, in high-contrast.
        base_row(
            Control::PaneSplitter,
            Qual::Stable,
            "Pane splitter owner",
            "The pane splitter renders the shared interactive-state contract so its drag handle is keyboard-reachable with a visible focus ring and its active drag treatment is carried by an inset and a border shift — legible under high-contrast — never by color alone, and the splitter hit target stays stable as it is focused and dragged",
            "evidence:m5-interactive-state-pane-splitter:001",
            vec![
                state_case(
                    Control::PaneSplitter,
                    State::FocusVisible,
                    false,
                    true,
                    false,
                    true,
                    "control:workbench.editor-group-splitter",
                    "token:state.pane_splitter.focus_visible",
                ),
                state_case(
                    Control::PaneSplitter,
                    State::PressedActive,
                    true,
                    false,
                    false,
                    true,
                    "control:workbench.editor-group-splitter",
                    "token:state.pane_splitter.pressed",
                ),
            ],
        ),
        // 5. Quick-action card — the pointer-hover treatment and the keyboard focus-visible ring,
        //    so a card affordance in a pane or start surface carries hover and focus without a
        //    color-only signal and without shifting layout.
        base_row(
            Control::QuickActionCard,
            Qual::Stable,
            "Quick-action card owner",
            "The quick-action card renders the shared interactive-state contract so its hover treatment is carried by an elevation and border shift with a pointer cursor and its keyboard focus arrives with a visible focus ring — the card title stays present in every state, the layout never reflows on hover or focus, and the whole card stays one stable hit target",
            "evidence:m5-interactive-state-quick-action-card:001",
            vec![
                state_case(
                    Control::QuickActionCard,
                    State::Hover,
                    true,
                    false,
                    false,
                    false,
                    "control:start-center.new-project-card",
                    "token:state.quick_action_card.hover",
                ),
                state_case(
                    Control::QuickActionCard,
                    State::FocusVisible,
                    false,
                    true,
                    false,
                    false,
                    "control:start-center.new-project-card",
                    "token:state.quick_action_card.focus_visible",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5InteractiveStateGovernanceReview {
    M5InteractiveStateGovernanceReview {
        controls_expose_default_hover_focus_pressed: true,
        state_meaning_never_color_only: true,
        hit_targets_stay_stable: true,
        no_interaction_breaking_layout_shift: true,
        focus_visible_under_keyboard: true,
        legible_under_high_contrast_and_zoom: true,
        legible_under_reduced_motion: true,
        states_driven_by_shared_contract_and_tokens: true,
        no_one_off_per_surface_styling: true,
        states_stable_across_deployment_lines: true,
        states_stable_across_consumer_surfaces: true,
        every_control_declares_accessibility_route: true,
        support_export_reconstructs_state_truth: true,
        later_rows_cannot_invent_parallel_state_vocabulary: true,
    }
}

fn consumer_projection() -> M5InteractiveStateConsumerProjection {
    M5InteractiveStateConsumerProjection {
        controls_consume_state_vocabulary: true,
        presentation_reads_single_source: true,
        non_color_cue_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5InteractiveStateProofFreshness {
    M5InteractiveStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InteractiveStateReleasePosture {
    M5InteractiveStateReleasePosture {
        release_packet_ref: M5_INTERACTIVE_STATE_CONTRACT_ARTIFACT_REF.to_owned(),
        interactive_state_audit_ref: M5_INTERACTIVE_STATE_CONTRACT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
        M5_INTERACTIVE_STATE_CONTRACT_DOC_REF,
        M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_INTERACTIVE_STATE_CONTRACT_FOCUS_SELECTION_REF,
        M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 interactive-state-contract packet.
pub fn seeded_m5_interactive_state_contract_packet() -> M5InteractiveStateContractPacket {
    M5InteractiveStateContractPacket::new(M5InteractiveStateContractPacketInput {
        packet_id: M5_INTERACTIVE_STATE_CONTRACT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 interactive-state contract primitive: control kind, interactive state (default/hover/focus-visible/pressed-active), derived presentation posture (resting-default/pointer-hover/keyboard-focus-visible/pressed-or-active), required non-color cues, interaction input routes, and no-color-only / stable-hit-target / no-layout-shift / focus-visible guarantees"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5InteractiveStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the pane splitter control is held at Beta because a slice of splitter
/// affordances does not yet render the keyboard focus-visible ring on every profile; every control
/// stays visible.
pub fn seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed(
) -> M5InteractiveStateContractPacket {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.packet_id = "m5-interactive-state-contract-primitive:pane-splitter-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.control_kind == M5InteractiveControlKind::PaneSplitter)
        .expect("pane-splitter row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the quick-action card control is narrowed to Preview pending
/// no-layout-shift parity proof for the hover/focus transition across every density; every control
/// stays visible.
pub fn seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed(
) -> M5InteractiveStateContractPacket {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.packet_id =
        "m5-interactive-state-contract-primitive:quick-action-card-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.control_kind == M5InteractiveControlKind::QuickActionCard)
        .expect("quick-action-card row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
