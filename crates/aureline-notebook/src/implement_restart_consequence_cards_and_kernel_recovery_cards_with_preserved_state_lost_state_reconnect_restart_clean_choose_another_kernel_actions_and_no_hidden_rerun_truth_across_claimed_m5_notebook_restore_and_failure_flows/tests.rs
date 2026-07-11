use super::*;

const PACKET_ID: &str = RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_PACKET_ID;

fn packet() -> RestartConsequenceCardKernelRecoveryCardControlsPacket {
    seeded_restart_consequence_card_kernel_recovery_card_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        RESTART_CONSEQUENCE_CARD_KERNEL_RECOVERY_CARD_SCHEMA_VERSION
    );
}

#[test]
fn impact_and_scope_are_derived_not_asserted() {
    use M5RestartActionClass as Act;
    use M5RestartConsequenceState as Cons;
    use RestartActionScope as Scope;
    use RestartImpactClass as Impact;

    // Consequence maps 1:1 to an impact class.
    for (cons, impact, preserves) in [
        (Cons::StatePreserved, Impact::StatePreservedImpact, true),
        (Cons::StateLost, Impact::LiveStateLostImpact, false),
        (
            Cons::VariablesCleared,
            Impact::VariablesClearedImpact,
            false,
        ),
        (Cons::OutputsRetained, Impact::OutputsRetainedImpact, true),
        (Cons::OutputsCleared, Impact::OutputsClearedImpact, false),
        (Cons::NoConsequence, Impact::NoRestartImpact, true),
    ] {
        let d = resolve_restart_consequence_card(Act::RestartKernel, cons);
        assert_eq!(d.impact_class, impact);
        assert_eq!(d.preserves_state, preserves);
        assert_eq!(d.may_claim_state_preserved, preserves);
    }

    // Action maps 1:1 to a scope; ending the session affects the debugger.
    for (action, scope, ends) in [
        (Act::RestartKernel, Scope::EndsSession, true),
        (Act::RestartAndRunAll, Scope::EndsSession, true),
        (Act::ShutdownKernel, Scope::EndsSession, true),
        (Act::InterruptKernel, Scope::KeepsSession, false),
        (Act::ReconnectKernel, Scope::KeepsSession, false),
        (Act::ClearOutputs, Scope::OutputsOnly, false),
    ] {
        let d = resolve_restart_consequence_card(action, Cons::StatePreserved);
        assert_eq!(d.action_scope, scope);
        assert_eq!(d.affects_debugger_session, ends);
    }

    // Lost state and cleared outputs require a rerun and carry the matching notes.
    let d = resolve_restart_consequence_card(Act::RestartKernel, Cons::StateLost);
    assert!(d.loses_live_state);
    assert!(d.requires_rerun);
    assert!(d.needs_lost_state_note);
    assert!(d.needs_rerun_note);

    let d = resolve_restart_consequence_card(Act::RestartAndRunAll, Cons::VariablesCleared);
    assert!(d.requires_rerun);
    assert!(d.needs_variables_cleared_note);

    let d = resolve_restart_consequence_card(Act::ReconnectKernel, Cons::OutputsCleared);
    assert!(d.requires_rerun);
    assert!(d.needs_outputs_cleared_note);
    assert!(!d.affects_debugger_session);

    // A no-consequence clear-outputs preserves state and needs no rerun.
    let d = resolve_restart_consequence_card(Act::ClearOutputs, Cons::NoConsequence);
    assert!(d.preserves_state);
    assert!(!d.requires_rerun);
}

#[test]
fn posture_and_continuity_are_derived_not_asserted() {
    use KernelRecoveryPosture as Posture;
    use M5KernelRecoveryActionClass as Act;
    use M5KernelRecoveryState as State;
    use RecoveryContinuityClass as Continuity;

    // State maps 1:1 to a posture; only `recovered` may claim recovered.
    for (state, posture, recovered) in [
        (State::Recoverable, Posture::RecoverableNow, false),
        (State::ReconnectAvailable, Posture::ReconnectOffered, false),
        (State::RestartRequired, Posture::RestartNeeded, false),
        (State::NoKernelAvailable, Posture::NoKernelAvailable, false),
        (State::RecoveryBlocked, Posture::RecoveryBlocked, false),
        (State::Recovered, Posture::RecoveredClean, true),
    ] {
        let d = resolve_kernel_recovery_card(Act::Reconnect, state);
        assert_eq!(d.posture_class, posture);
        assert_eq!(d.is_recovered, recovered);
        assert_eq!(d.may_claim_recovered, recovered);
    }

    // Action maps 1:1 to continuity; only a clean session requires a rerun afterward.
    for (action, continuity, clean) in [
        (Act::Reconnect, Continuity::ContinuesSession, false),
        (Act::ReattachSession, Continuity::ContinuesSession, false),
        (Act::RestartClean, Continuity::CleanSession, true),
        (Act::StartLocalFallback, Continuity::CleanSession, true),
        (Act::ChooseAnotherKernel, Continuity::CleanSession, true),
        (Act::WaitForManaged, Continuity::AwaitsManaged, false),
    ] {
        let d = resolve_kernel_recovery_card(action, State::Recoverable);
        assert_eq!(d.continuity_class, continuity);
        assert_eq!(d.requires_rerun_after_recovery, clean);
        assert_eq!(d.needs_clean_session_note, clean);
        assert_eq!(d.preserves_continuity, continuity.preserves_continuity());
    }

    // No-kernel / blocked / restart-required states carry their notes.
    let d = resolve_kernel_recovery_card(Act::ChooseAnotherKernel, State::NoKernelAvailable);
    assert!(d.needs_no_kernel_note);
    let d = resolve_kernel_recovery_card(Act::StartLocalFallback, State::RecoveryBlocked);
    assert!(d.needs_blocked_note);
    let d = resolve_kernel_recovery_card(Act::RestartClean, State::RestartRequired);
    assert!(d.needs_restart_note);
    let d = resolve_kernel_recovery_card(Act::WaitForManaged, State::Recoverable);
    assert!(d.needs_await_note);
}

#[test]
fn restart_action_consequence_impact_and_scope_coverage_is_complete() {
    let packet = packet();
    let actions: std::collections::BTreeSet<_> = packet
        .restart_consequence_cards
        .iter()
        .map(|c| c.restart_action)
        .collect();
    for action in M5RestartActionClass::ALL {
        assert!(
            actions.contains(&action),
            "missing restart action {action:?}"
        );
    }
    let consequences: std::collections::BTreeSet<_> = packet
        .restart_consequence_cards
        .iter()
        .map(|c| c.consequence_state)
        .collect();
    for cons in M5RestartConsequenceState::ALL {
        assert!(consequences.contains(&cons), "missing consequence {cons:?}");
    }
    let impacts: std::collections::BTreeSet<_> = packet
        .restart_consequence_cards
        .iter()
        .map(|c| c.restart_disclosure().impact_class)
        .collect();
    for impact in RestartImpactClass::ALL {
        assert!(impacts.contains(&impact), "missing impact class {impact:?}");
    }
    let scopes: std::collections::BTreeSet<_> = packet
        .restart_consequence_cards
        .iter()
        .map(|c| c.restart_disclosure().action_scope)
        .collect();
    for scope in RestartActionScope::ALL {
        assert!(scopes.contains(&scope), "missing action scope {scope:?}");
    }
}

#[test]
fn recovery_action_state_posture_and_continuity_coverage_is_complete() {
    let packet = packet();
    let actions: std::collections::BTreeSet<_> = packet
        .kernel_recovery_cards
        .iter()
        .map(|c| c.recovery_action)
        .collect();
    for action in M5KernelRecoveryActionClass::ALL {
        assert!(
            actions.contains(&action),
            "missing recovery action {action:?}"
        );
    }
    let states: std::collections::BTreeSet<_> = packet
        .kernel_recovery_cards
        .iter()
        .map(|c| c.recovery_state)
        .collect();
    for state in M5KernelRecoveryState::ALL {
        assert!(states.contains(&state), "missing recovery state {state:?}");
    }
    let postures: std::collections::BTreeSet<_> = packet
        .kernel_recovery_cards
        .iter()
        .map(|c| c.recovery_disclosure().posture_class)
        .collect();
    for posture in KernelRecoveryPosture::ALL {
        assert!(postures.contains(&posture), "missing posture {posture:?}");
    }
    let continuities: std::collections::BTreeSet<_> = packet
        .kernel_recovery_cards
        .iter()
        .map(|c| c.recovery_disclosure().continuity_class)
        .collect();
    for continuity in RecoveryContinuityClass::ALL {
        assert!(
            continuities.contains(&continuity),
            "missing continuity {continuity:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::MissingSourceContracts));
}

#[test]
fn empty_restart_cards_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RestartCardsMissing));
}

#[test]
fn empty_recovery_cards_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardsMissing));
}

#[test]
fn restart_card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].component =
        M5NotebookKernelOutputComponentFamily::KernelRecoveryCard;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RestartCardWrongComponentClass
    ));
}

#[test]
fn recovery_card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].component =
        M5NotebookKernelOutputComponentFamily::RestartConsequenceCard;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardWrongComponentClass
    ));
}

#[test]
fn restart_card_overclaiming_state_preserved_fails() {
    let mut packet = packet();
    let card = packet
        .restart_consequence_cards
        .iter_mut()
        .find(|c| !c.restart_disclosure().may_claim_state_preserved)
        .expect("lost-state card present");
    card.claims_state_preserved = true;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::StatePreservationOverclaimed
    ));
}

#[test]
fn restart_card_misrepresenting_impact_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].impact_class = RestartImpactClass::NoRestartImpact;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::ImpactMisrepresented));
}

#[test]
fn recovery_card_overclaiming_recovered_fails() {
    let mut packet = packet();
    let card = packet
        .kernel_recovery_cards
        .iter_mut()
        .find(|c| !c.recovery_disclosure().may_claim_recovered)
        .expect("non-recovered card present");
    card.claims_recovered = true;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RecoveryOverclaimed));
}

#[test]
fn recovery_card_misrepresenting_continuity_fails() {
    let mut packet = packet();
    let card = packet
        .kernel_recovery_cards
        .iter_mut()
        .find(|c| c.recovery_disclosure().preserves_continuity)
        .expect("continuity-preserving card present");
    card.claims_continuity_preserved = false;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RecoveryMisrepresented));
}

#[test]
fn missing_lost_state_note_fails() {
    let mut packet = packet();
    let card = packet
        .restart_consequence_cards
        .iter_mut()
        .find(|c| c.restart_disclosure().needs_lost_state_note)
        .expect("lost-state card present");
    card.lost_state_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::LostStateNoteMissing));
}

#[test]
fn missing_rerun_note_fails() {
    let mut packet = packet();
    let card = packet
        .restart_consequence_cards
        .iter_mut()
        .find(|c| c.restart_disclosure().needs_rerun_note)
        .expect("rerun-requiring card present");
    card.rerun_requirement_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RerunNoteMissing));
}

#[test]
fn missing_debugger_session_note_fails() {
    let mut packet = packet();
    let card = packet
        .restart_consequence_cards
        .iter_mut()
        .find(|c| c.restart_disclosure().needs_debugger_session_note)
        .expect("session-ending card present");
    card.debugger_session_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::DebuggerSessionNoteMissing));
}

#[test]
fn missing_preserved_state_label_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0]
        .preserved_state_label
        .clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::PreservedStateLabelMissing));
}

#[test]
fn missing_no_rerun_note_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].no_rerun_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::NoRerunNoteMissing));
}

#[test]
fn missing_clean_session_note_fails() {
    let mut packet = packet();
    let card = packet
        .kernel_recovery_cards
        .iter_mut()
        .find(|c| c.recovery_disclosure().needs_clean_session_note)
        .expect("clean-session card present");
    card.clean_session_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::CleanSessionNoteMissing));
}

#[test]
fn missing_no_kernel_note_fails() {
    let mut packet = packet();
    let card = packet
        .kernel_recovery_cards
        .iter_mut()
        .find(|c| c.recovery_disclosure().needs_no_kernel_note)
        .expect("no-kernel card present");
    card.no_kernel_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::NoKernelNoteMissing));
}

#[test]
fn restart_card_missing_confirm_action_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].card_actions = vec![RestartCardAction::ReviewConsequences];
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RestartCardActionsIncomplete
    ));
}

#[test]
fn recovery_card_missing_reconnect_action_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].card_actions = vec![RecoveryCardAction::RestartClean];
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCardActionsIncomplete
    ));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.restart_consequence_cards[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::DispositionsMissing));
}

#[test]
fn implying_rerun_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].implies_rerun_on_restore_or_recovery = true;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RerunImpliedOnRestoreOrRecovery
    ));
}

#[test]
fn presenting_lost_state_as_preserved_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].presents_lost_state_as_preserved = true;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::LostStateShownAsPreserved));
}

#[test]
fn hiding_consequence_behind_hover_only_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].hides_consequence_behind_hover_only = true;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::ConsequenceHoverOnly));
}

#[test]
fn collapsing_recovery_into_generic_error_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].collapses_recovery_into_generic_error = true;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RecoveryCollapsedIntoGenericError
    ));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.restart_consequence_cards[0].required_labels =
        vec![M5NotebookKernelOutputRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].accessibility_routes =
        vec![M5NotebookKernelOutputAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::AccessibilityRouteMissing));
}

#[test]
fn restart_recovery_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .restart_recovery_review
        .recovery_card_never_implies_rerun = false;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::RestartRecoveryReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .cli_export_preserves_no_rerun_truth = false;
    assert!(packet.validate().contains(
        &RestartConsequenceCardKernelRecoveryCardViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.kernel_recovery_cards[0].deep_link_ref =
        "see https://internal.example/kernel".to_owned();
    assert!(packet
        .validate()
        .contains(&RestartConsequenceCardKernelRecoveryCardViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Restart consequence cards"));
    assert!(summary.contains("## Kernel recovery cards"));
    assert!(summary.contains("live_state_lost_impact"));
    assert!(summary.contains("clean_session"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 restart cards + 6 recovery cards
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("restart_consequence_card"));
    assert!(csv.contains("kernel_recovery_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_restart_consequence_card_kernel_recovery_card_export()
        .expect("checked restart consequence card kernel recovery card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-restart-consequence-card-kernel-recovery-card-controls/restart_consequence_card_lost_state.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-restart-consequence-card-kernel-recovery-card-controls/kernel_recovery_card_clean_session.json"
        )),
    ] {
        let packet: RestartConsequenceCardKernelRecoveryCardControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as restart consequence card kernel recovery card packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state(),
        seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
