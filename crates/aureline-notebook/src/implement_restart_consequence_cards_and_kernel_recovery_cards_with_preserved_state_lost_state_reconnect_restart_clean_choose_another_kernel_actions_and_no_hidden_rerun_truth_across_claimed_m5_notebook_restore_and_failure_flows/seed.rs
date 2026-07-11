//! Canonical seed builders for the restart-consequence-card / kernel-recovery-card controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code components,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical restart-consequence-card / kernel-recovery-card packet.
pub const RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_PACKET_ID: &str =
    "m5-restart-consequence-card-kernel-recovery-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn restart_source_refs() -> Vec<String> {
    strings(&[
        M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn recovery_source_refs() -> Vec<String> {
    strings(&[
        M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn restart_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::RestartConsequenceImpliedRerun,
        M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn recovery_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::RecoveryOverclaimed,
        M5NotebookKernelOutputDowngradeTrigger::RestartConsequenceImpliedRerun,
        M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

/// Builds a restart consequence card, deriving the impact class, action scope, rerun and
/// debugger-session claims, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn restart_card(
    card_id: &str,
    card_label: &str,
    restart_action: M5RestartActionClass,
    consequence_state: M5RestartConsequenceState,
    preserved_state_label: &str,
    restart_action_label: &str,
    consequence_state_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<RestartCardAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> RestartConsequenceCard {
    let disclosure = resolve_restart_consequence_card(restart_action, consequence_state);
    RestartConsequenceCard {
        component: M5NotebookKernelOutputComponentFamily::RestartConsequenceCard,
        card_id: card_id.to_owned(),
        card_label: card_label.to_owned(),
        restart_action,
        consequence_state,
        impact_class: disclosure.impact_class,
        action_scope: disclosure.action_scope,
        claims_state_preserved: disclosure.may_claim_state_preserved,
        requires_rerun: disclosure.requires_rerun,
        affects_debugger_session: disclosure.affects_debugger_session,
        preserved_state_label: preserved_state_label.to_owned(),
        lost_state_note: if disclosure.needs_lost_state_note {
            "Live kernel state is lost: in-memory variables and imports do not survive this restart"
                .to_owned()
        } else {
            String::new()
        },
        variables_cleared_note: if disclosure.needs_variables_cleared_note {
            "All variables are cleared; nothing computed in this session is kept".to_owned()
        } else {
            String::new()
        },
        outputs_cleared_note: if disclosure.needs_outputs_cleared_note {
            "Prior outputs are cleared; nothing is rendered until cells are rerun".to_owned()
        } else {
            String::new()
        },
        rerun_requirement_note: if disclosure.needs_rerun_note {
            "A rerun is required to recompute the lost state — nothing is rerun automatically"
                .to_owned()
        } else {
            String::new()
        },
        debugger_session_note: if disclosure.needs_debugger_session_note {
            "The debugger session and its frames end with this action; breakpoints stay set"
                .to_owned()
        } else {
            String::new()
        },
        restart_action_label: restart_action_label.to_owned(),
        consequence_state_label: consequence_state_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: restart_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "preserved_state_label",
            "restart_action",
            "consequence_state",
            "impact_class",
            "action_scope",
            "rerun_requirement_note",
            "debugger_session_note",
            "deep_link_kind",
        ]),
        source_contract_refs: restart_source_refs(),
        implies_rerun_on_restore_or_recovery: false,
        presents_lost_state_as_preserved: false,
        hides_consequence_behind_hover_only: false,
        collapses_recovery_into_generic_error: false,
    }
}

/// Builds a kernel recovery card, deriving the posture, continuity, recovered and rerun claims, and
/// the required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn recovery_card(
    card_id: &str,
    card_label: &str,
    recovery_action: M5KernelRecoveryActionClass,
    recovery_state: M5KernelRecoveryState,
    no_rerun_note: &str,
    recovery_action_label: &str,
    recovery_state_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<RecoveryCardAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> KernelRecoveryCard {
    let disclosure = resolve_kernel_recovery_card(recovery_action, recovery_state);
    KernelRecoveryCard {
        component: M5NotebookKernelOutputComponentFamily::KernelRecoveryCard,
        card_id: card_id.to_owned(),
        card_label: card_label.to_owned(),
        recovery_action,
        recovery_state,
        posture_class: disclosure.posture_class,
        continuity_class: disclosure.continuity_class,
        claims_recovered: disclosure.may_claim_recovered,
        claims_continuity_preserved: disclosure.preserves_continuity,
        requires_rerun_after_recovery: disclosure.requires_rerun_after_recovery,
        no_rerun_note: no_rerun_note.to_owned(),
        clean_session_note: if disclosure.needs_clean_session_note {
            "This recovery starts a clean session; prior live state is gone and must be recomputed"
                .to_owned()
        } else {
            String::new()
        },
        no_kernel_note: if disclosure.needs_no_kernel_note {
            "No kernel is available; choose another kernel or start a local fallback to continue"
                .to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Recovery is blocked here; the failure is attributable and the notebook stays editable"
                .to_owned()
        } else {
            String::new()
        },
        restart_note: if disclosure.needs_restart_note {
            "A restart is required to recover; reconnecting will not restore the session".to_owned()
        } else {
            String::new()
        },
        await_note: if disclosure.needs_await_note {
            "Awaiting a managed kernel; recovery completes once the managed workspace responds"
                .to_owned()
        } else {
            String::new()
        },
        recovery_action_label: recovery_action_label.to_owned(),
        recovery_state_label: recovery_state_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: recovery_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "recovery_action_label",
            "recovery_action",
            "recovery_state",
            "posture_class",
            "continuity_class",
            "no_rerun_note",
            "clean_session_note",
            "deep_link_kind",
        ]),
        source_contract_refs: recovery_source_refs(),
        implies_rerun_on_restore_or_recovery: false,
        presents_lost_state_as_preserved: false,
        hides_consequence_behind_hover_only: false,
        collapses_recovery_into_generic_error: false,
    }
}

fn restart_consequence_cards() -> Vec<RestartConsequenceCard> {
    use DeepLinkKind as Link;
    use M5NotebookKernelOutputDisposition as Disp;
    use M5RestartActionClass as Act;
    use M5RestartConsequenceState as Cons;
    use RestartCardAction as Action;

    vec![
        // 1. Restart kernel + state lost → live state lost, ends session (rerun + debugger notes).
        restart_card(
            "restart-kernel-state-lost",
            "Restart kernel (live state lost)",
            Act::RestartKernel,
            Cons::StateLost,
            "Preserved: notebook source and prior saved outputs stay in the .ipynb document",
            "Action: restart kernel",
            "Consequence: live kernel state is lost",
            "Restart truth: what survives and what must be recomputed before you restart",
            Link::KernelManager,
            "kernel:manager/restart-kernel",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::InterruptInstead,
                Action::OpenDeepLink,
            ],
            vec![Disp::RestartClean, Disp::Ready],
        ),
        // 2. Restart and run all + variables cleared → variables cleared, ends session.
        restart_card(
            "restart-run-all-variables-cleared",
            "Restart and run all (variables cleared)",
            Act::RestartAndRunAll,
            Cons::VariablesCleared,
            "Preserved: notebook source and cell order stay in the .ipynb document",
            "Action: restart and run all",
            "Consequence: all variables are cleared before the run",
            "Restart truth: a clean run clears every variable before re-executing the cells",
            Link::NotebookLocation,
            "notebook:run/restart-run-all",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::RestartClean, Disp::Queued],
        ),
        // 3. Interrupt kernel + state preserved → state preserved, keeps session (may claim
        //    preserved).
        restart_card(
            "interrupt-kernel-state-preserved",
            "Interrupt kernel (state preserved)",
            Act::InterruptKernel,
            Cons::StatePreserved,
            "Preserved: live kernel state, variables, and the session all survive an interrupt",
            "Action: interrupt kernel",
            "Consequence: state is preserved (only the running cell stops)",
            "Restart truth: an interrupt stops the running cell without losing session state",
            Link::KernelManager,
            "kernel:manager/interrupt",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::OpenDeepLink,
            ],
            vec![Disp::Busy, Disp::Ready],
        ),
        // 4. Shutdown kernel + outputs retained → outputs retained, ends session (debugger note).
        restart_card(
            "shutdown-kernel-outputs-retained",
            "Shut down kernel (outputs retained)",
            Act::ShutdownKernel,
            Cons::OutputsRetained,
            "Preserved: prior outputs are retained in the notebook document after shutdown",
            "Action: shut down kernel",
            "Consequence: outputs are retained (the kernel process stops)",
            "Restart truth: shutting down keeps rendered outputs but ends the live session",
            Link::SupportBundle,
            "support:bundle/shutdown-consequence",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::ExportEvidence,
            ],
            vec![Disp::Disconnected, Disp::Ready],
        ),
        // 5. Reconnect kernel + outputs cleared → outputs cleared, keeps session (rerun note).
        restart_card(
            "reconnect-kernel-outputs-cleared",
            "Reconnect kernel (outputs cleared)",
            Act::ReconnectKernel,
            Cons::OutputsCleared,
            "Preserved: notebook source stays intact; the reconnect targets the same session",
            "Action: reconnect kernel",
            "Consequence: outputs are cleared on reconnect",
            "Restart truth: reconnecting keeps the session but clears the rendered outputs",
            Link::KernelManager,
            "kernel:manager/reconnect-clear",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::OpenDeepLink,
            ],
            vec![Disp::Reconnect, Disp::StaleOutput],
        ),
        // 6. Clear outputs + no consequence → no impact, outputs only.
        restart_card(
            "clear-outputs-no-consequence",
            "Clear outputs (no session consequence)",
            Act::ClearOutputs,
            Cons::NoConsequence,
            "Preserved: live kernel state and variables survive; only rendered outputs are removed",
            "Action: clear outputs",
            "Consequence: none for the session (only cleared display)",
            "Restart truth: clearing outputs never touches the running session or its variables",
            Link::DocsAnchor,
            "docs:notebooks/restart-consequences",
            vec![
                Action::ReviewConsequences,
                Action::ConfirmRestart,
                Action::CancelRestart,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
    ]
}

fn kernel_recovery_cards() -> Vec<KernelRecoveryCard> {
    use DeepLinkKind as Link;
    use M5KernelRecoveryActionClass as Act;
    use M5KernelRecoveryState as State;
    use M5NotebookKernelOutputDisposition as Disp;
    use RecoveryCardAction as Action;

    let no_rerun =
        "Recovery never reruns code or cells on its own; nothing is executed until you run it";

    vec![
        // 1. Reconnect + recovered → recovered, continues session (may claim recovered).
        recovery_card(
            "reconnect-recovered",
            "Kernel recovered by reconnect",
            Act::Reconnect,
            State::Recovered,
            no_rerun,
            "Recovery action: reconnect",
            "Recovery state: recovered (session resumed)",
            "Recovery truth: the kernel recovered by reconnecting to the same live session",
            Link::KernelManager,
            "kernel:manager/recovered",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::OpenInspectOnly,
                Action::OpenDeepLink,
            ],
            vec![Disp::Reconnect, Disp::Ready],
        ),
        // 2. Reattach session + reconnect available → reconnect offered, continues session.
        recovery_card(
            "reattach-reconnect-available",
            "Reattach session (reconnect available)",
            Act::ReattachSession,
            State::ReconnectAvailable,
            no_rerun,
            "Recovery action: reattach session",
            "Recovery state: reconnect available",
            "Recovery truth: the prior session can be reattached without losing live state",
            Link::KernelManager,
            "kernel:manager/reattach",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::OpenInspectOnly,
                Action::OpenDeepLink,
            ],
            vec![Disp::Reconnect, Disp::Disconnected],
        ),
        // 3. Restart clean + restart required → restart needed, clean session (restart + clean +
        //    rerun notes).
        recovery_card(
            "restart-clean-restart-required",
            "Restart clean (restart required)",
            Act::RestartClean,
            State::RestartRequired,
            no_rerun,
            "Recovery action: restart clean",
            "Recovery state: restart required",
            "Recovery truth: this kernel needs a clean restart; the prior session cannot resume",
            Link::KernelManager,
            "kernel:manager/restart-clean",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::RestartClean, Disp::Disconnected],
        ),
        // 4. Choose another kernel + no kernel available → no kernel, clean session (no-kernel +
        //    clean notes).
        recovery_card(
            "choose-another-no-kernel",
            "Choose another kernel (no kernel available)",
            Act::ChooseAnotherKernel,
            State::NoKernelAvailable,
            no_rerun,
            "Recovery action: choose another kernel",
            "Recovery state: no kernel available",
            "Recovery truth: no kernel is available; pick another kernel to start a clean session",
            Link::KernelManager,
            "kernel:manager/choose-kernel",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::OpenInspectOnly,
                Action::OpenDeepLink,
            ],
            vec![Disp::ChooseAnotherKernel, Disp::NoKernel],
        ),
        // 5. Start local fallback + recovery blocked → blocked, clean session (blocked + clean
        //    notes).
        recovery_card(
            "local-fallback-recovery-blocked",
            "Start local fallback (recovery blocked)",
            Act::StartLocalFallback,
            State::RecoveryBlocked,
            no_rerun,
            "Recovery action: start local fallback",
            "Recovery state: recovery blocked (remote kernel unreachable)",
            "Recovery truth: remote recovery is blocked; a local fallback starts a clean session",
            Link::SupportBundle,
            "support:bundle/recovery-blocked",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::ExportEvidence,
                Action::OpenInspectOnly,
            ],
            vec![Disp::RestartClean, Disp::Remote],
        ),
        // 6. Wait for managed + recoverable → recoverable, awaits managed (await note).
        recovery_card(
            "wait-managed-recoverable",
            "Wait for managed kernel (recoverable)",
            Act::WaitForManaged,
            State::Recoverable,
            no_rerun,
            "Recovery action: wait for managed kernel",
            "Recovery state: recoverable (managed workspace warming up)",
            "Recovery truth: the managed kernel is recoverable once the workspace responds",
            Link::DocsAnchor,
            "docs:notebooks/kernel-recovery",
            vec![
                Action::Reconnect,
                Action::RestartClean,
                Action::ChooseAnotherKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Managed, Disp::Queued],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::RestartConsequenceImpliedRerun,
        M5NotebookKernelOutputDowngradeTrigger::RecoveryOverclaimed,
        M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
        M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn restart_recovery_review() -> RestartRecoveryReview {
    RestartRecoveryReview {
        restart_card_shows_preserved_state: true,
        restart_card_shows_lost_state: true,
        restart_card_shows_debugger_session_impact: true,
        restart_card_shows_rerun_requirement: true,
        recovery_card_offers_reconnect_restart_choose: true,
        recovery_card_shows_recovery_state: true,
        recovery_card_never_implies_rerun: true,
        recovery_never_overclaims_recovered: true,
        impact_and_posture_derived_never_asserted: true,
        lost_state_never_presented_as_preserved: true,
        consequence_never_hover_only: true,
        rerun_requirement_named_before_restart: true,
        recovery_degrades_to_attributable_state_not_generic_error: true,
        kernel_origin_never_collapsed_into_one_badge: true,
        every_next_step_names_stable_deep_link: true,
        cards_consistent_across_surfaces: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        components_stable_across_deployment_lines: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> RestartRecoveryConsumerProjection {
    RestartRecoveryConsumerProjection {
        notebook_tab_reads_single_source: true,
        debug_bridge_shows_restart_consequences: true,
        support_packet_shows_recovery_state: true,
        companion_handoff_shows_recovery_summary: true,
        cli_export_preserves_no_rerun_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> RestartRecoveryProofFreshness {
    RestartRecoveryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_REF,
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_RESTART_CONSEQUENCE_CARD_SCHEMA_REF,
        M5_KERNEL_RECOVERY_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical restart-consequence-card / kernel-recovery-card controls packet.
pub fn seeded_restart_consequence_card_kernel_recovery_card_controls(
) -> RestartConsequenceCardKernelRecoveryCardControlsPacket {
    RestartConsequenceCardKernelRecoveryCardControlsPacket::new(
        RestartConsequenceCardKernelRecoveryCardControlsPacketInput {
            packet_id: RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_PACKET_ID.to_owned(),
            surface_label:
                "M5 restart consequence cards and kernel recovery cards: preserved-versus-lost state, debugger/session impact, reconnect/restart-clean/choose-another-kernel recovery, and no-hidden-rerun truth across claimed notebook restore and failure flows"
                    .to_owned(),
            restart_consequence_cards: restart_consequence_cards(),
            kernel_recovery_cards: kernel_recovery_cards(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
            restart_recovery_review: restart_recovery_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a restart consequence card whose live state is lost, which must
/// stay visibly lost and never read as preserved. Every restart action, consequence state, impact
/// class, and scope stays covered so the fixture validates on its own.
pub fn seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state(
) -> RestartConsequenceCardKernelRecoveryCardControlsPacket {
    let mut packet = seeded_restart_consequence_card_kernel_recovery_card_controls();
    packet.packet_id =
        "m5-restart-consequence-card-kernel-recovery-card-controls:fixture:restart-consequence-card-lost-state"
            .to_owned();
    packet.surface_label =
        "M5 restart consequence cards: a restart that loses live state stays visibly lost and names its rerun requirement"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a kernel recovery card that started a clean session, which must
/// name that a rerun is required and never imply one already ran. Every recovery action, recovery
/// state, posture, and continuity class stays covered so the fixture validates on its own.
pub fn seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session(
) -> RestartConsequenceCardKernelRecoveryCardControlsPacket {
    let mut packet = seeded_restart_consequence_card_kernel_recovery_card_controls();
    packet.packet_id =
        "m5-restart-consequence-card-kernel-recovery-card-controls:fixture:kernel-recovery-card-clean-session"
            .to_owned();
    packet.surface_label =
        "M5 kernel recovery cards: a clean-session recovery names its rerun requirement and never implies a rerun"
            .to_owned();
    packet
}
