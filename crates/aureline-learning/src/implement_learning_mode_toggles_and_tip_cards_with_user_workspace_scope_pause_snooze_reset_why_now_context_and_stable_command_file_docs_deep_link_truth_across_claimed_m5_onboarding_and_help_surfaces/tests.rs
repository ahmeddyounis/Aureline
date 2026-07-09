use super::*;

const PACKET_ID: &str = LEARNING_MODE_TOGGLE_TIP_CARD_PACKET_ID;

fn packet() -> LearningModeToggleTipCardControlsPacket {
    seeded_learning_mode_toggle_tip_card_controls()
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
        LEARNING_MODE_TOGGLE_TIP_CARD_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_VERSION
    );
}

#[test]
fn activation_is_derived_not_asserted() {
    use LearningActivationClass as Activation;
    use M5LearningModeState as Mode;

    // On → active.
    let d = resolve_learning_activation(Mode::On);
    assert_eq!(d.activation_class, Activation::Active);
    assert!(d.is_active_learning);

    // Per feature family → scoped-active, still active.
    let d = resolve_learning_activation(Mode::PerFeatureFamily);
    assert_eq!(d.activation_class, Activation::ScopedActive);
    assert!(d.is_active_learning);

    // Sandboxed only → sandboxed-active, still active, needs sandboxed note.
    let d = resolve_learning_activation(Mode::SandboxedOnly);
    assert_eq!(d.activation_class, Activation::SandboxedActive);
    assert!(d.is_active_learning);
    assert!(d.needs_sandboxed_note);

    // Paused → paused, never active, needs paused note.
    let d = resolve_learning_activation(Mode::Paused);
    assert_eq!(d.activation_class, Activation::Paused);
    assert!(!d.is_active_learning);
    assert!(d.is_paused);
    assert!(d.needs_paused_note);

    // Off / ended → inactive, never active, needs inactive note.
    for mode in [Mode::Off, Mode::Ended] {
        let d = resolve_learning_activation(mode);
        assert_eq!(d.activation_class, Activation::Inactive);
        assert!(!d.is_active_learning);
        assert!(d.needs_inactive_note);
    }
}

#[test]
fn delivery_is_derived_not_asserted() {
    use M5TipDismissalState as Dismissal;
    use TipDeliveryClass as Delivery;

    // Dismissible → delivered.
    let d = resolve_tip_delivery(Dismissal::Dismissible);
    assert_eq!(d.delivery_class, Delivery::Delivered);
    assert!(d.is_delivered);

    // Persistent until acted → delivered-persistent.
    let d = resolve_tip_delivery(Dismissal::PersistentUntilActed);
    assert_eq!(d.delivery_class, Delivery::DeliveredPersistent);
    assert!(d.is_delivered);

    // Snoozed → snoozed, not delivered, needs snooze note.
    let d = resolve_tip_delivery(Dismissal::Snoozed);
    assert_eq!(d.delivery_class, Delivery::Snoozed);
    assert!(!d.is_delivered);
    assert!(d.needs_snooze_note);

    // Dismissed / auto-expired / suppressed → withheld, never delivered, needs withheld note.
    for dismissal in [
        Dismissal::Dismissed,
        Dismissal::AutoExpired,
        Dismissal::SuppressedByPreference,
    ] {
        let d = resolve_tip_delivery(dismissal);
        assert_eq!(d.delivery_class, Delivery::Withheld);
        assert!(!d.is_delivered);
        assert!(d.needs_withheld_note);
    }
}

#[test]
fn activation_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .toggles
        .iter()
        .map(|toggle| toggle.activation_disclosure().activation_class)
        .collect();
    for class in LearningActivationClass::ALL {
        assert!(
            covered.contains(&class),
            "missing activation class {class:?}"
        );
    }
}

#[test]
fn learning_mode_state_and_scope_coverage_is_complete() {
    let packet = packet();
    let states: std::collections::BTreeSet<_> =
        packet.toggles.iter().map(|t| t.learning_state).collect();
    for state in M5LearningModeState::ALL {
        assert!(states.contains(&state), "missing state {state:?}");
    }
    let scopes: std::collections::BTreeSet<_> = packet.toggles.iter().map(|t| t.scope).collect();
    for scope in M5LearningModeScope::ALL {
        assert!(scopes.contains(&scope), "missing scope {scope:?}");
    }
}

#[test]
fn delivery_class_trigger_and_dismissal_coverage_is_complete() {
    let packet = packet();
    let delivery: std::collections::BTreeSet<_> = packet
        .tip_cards
        .iter()
        .map(|tip| tip.delivery_disclosure().delivery_class)
        .collect();
    for class in TipDeliveryClass::ALL {
        assert!(
            delivery.contains(&class),
            "missing delivery class {class:?}"
        );
    }
    let triggers: std::collections::BTreeSet<_> =
        packet.tip_cards.iter().map(|t| t.trigger_class).collect();
    for trigger in M5TipTriggerClass::ALL {
        assert!(triggers.contains(&trigger), "missing trigger {trigger:?}");
    }
    let dismissals: std::collections::BTreeSet<_> =
        packet.tip_cards.iter().map(|t| t.dismissal_state).collect();
    for dismissal in M5TipDismissalState::ALL {
        assert!(
            dismissals.contains(&dismissal),
            "missing dismissal {dismissal:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::MissingSourceContracts));
}

#[test]
fn empty_toggles_fails() {
    let mut packet = packet();
    packet.toggles.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::TogglesMissing));
}

#[test]
fn empty_tip_cards_fails() {
    let mut packet = packet();
    packet.tip_cards.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::TipCardsMissing));
}

#[test]
fn toggle_wrong_component_class_fails() {
    let mut packet = packet();
    packet.toggles[0].component = M5LearningComponentFamily::TipCard;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ToggleWrongComponentClass));
}

#[test]
fn tip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.tip_cards[0].component = M5LearningComponentFamily::LearningModeToggle;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::TipCardWrongComponentClass));
}

#[test]
fn paused_toggle_claiming_active_fails() {
    let mut packet = packet();
    let toggle = packet
        .toggles
        .iter_mut()
        .find(|t| t.activation_class == LearningActivationClass::Paused)
        .expect("paused toggle present");
    toggle.claims_active = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ActivationMisrepresented));
}

#[test]
fn withheld_tip_claiming_delivered_fails() {
    let mut packet = packet();
    let tip = packet
        .tip_cards
        .iter_mut()
        .find(|t| t.delivery_class == TipDeliveryClass::Withheld)
        .expect("withheld tip present");
    tip.claims_delivered = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::DeliveryMisrepresented));
}

#[test]
fn missing_paused_note_fails() {
    let mut packet = packet();
    let toggle = packet
        .toggles
        .iter_mut()
        .find(|t| t.activation_class == LearningActivationClass::Paused)
        .expect("paused toggle present");
    toggle.paused_note.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::PausedNoteMissing));
}

#[test]
fn missing_inactive_note_fails() {
    let mut packet = packet();
    let toggle = packet
        .toggles
        .iter_mut()
        .find(|t| t.activation_class == LearningActivationClass::Inactive)
        .expect("inactive toggle present");
    toggle.inactive_note.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::InactiveNoteMissing));
}

#[test]
fn missing_withheld_note_fails() {
    let mut packet = packet();
    let tip = packet
        .tip_cards
        .iter_mut()
        .find(|t| t.delivery_class == TipDeliveryClass::Withheld)
        .expect("withheld tip present");
    tip.withheld_note.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::WithheldNoteMissing));
}

#[test]
fn toggle_missing_reset_action_fails() {
    let mut packet = packet();
    packet.toggles[0].toggle_actions = vec![LearningToggleAction::PauseLearning];
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ToggleActionsIncomplete));
}

#[test]
fn tip_missing_dismiss_action_fails() {
    let mut packet = packet();
    packet.tip_cards[0].tip_actions = vec![TipCardAction::TryNextAction];
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::TipActionsIncomplete));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    // The first toggle offers OpenDeepLink; blank its kind to NoDeepLink.
    packet.toggles[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.toggles[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.toggles[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::DeepLinkRefMissing));
}

#[test]
fn missing_why_now_context_fails() {
    let mut packet = packet();
    packet.tip_cards[0].why_now_context.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::WhyNowContextMissing));
}

#[test]
fn missing_scope_and_activation_note_fails() {
    let mut packet = packet();
    packet.toggles[0].scope_and_activation_note.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ScopeAndActivationNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.toggles[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::DispositionsMissing));
}

#[test]
fn toggle_masking_privacy_fails() {
    let mut packet = packet();
    packet.toggles[0].masks_privacy_or_offline_state = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::PrivacyOrOfflineStateMasked));
}

#[test]
fn toggle_hiding_activation_or_scope_fails() {
    let mut packet = packet();
    packet.toggles[0].hides_activation_or_scope = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ActivationOrScopeHidden));
}

#[test]
fn tip_implying_hidden_apply_fails() {
    let mut packet = packet();
    packet.tip_cards[0].implies_hidden_apply_or_mutation = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::HiddenApplyOrMutationImplied));
}

#[test]
fn tip_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.tip_cards[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::AlternateStateLabelInvented));
}

#[test]
fn control_depending_on_ephemeral_coachmark_fails() {
    let mut packet = packet();
    packet.toggles[0].depends_on_ephemeral_coachmark_or_hidden_routing = true;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::EphemeralCoachmarkOrHiddenRoutingUsed));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.toggles[0].required_labels = vec![M5LearningRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.tip_cards[0].accessibility_routes =
        vec![M5LearningAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::AccessibilityRouteMissing));
}

#[test]
fn learnability_review_incomplete_fails() {
    let mut packet = packet();
    packet.learnability_review.inactive_never_shown_as_active = false;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::LearnabilityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .why_now_context_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.tip_cards[0].deep_link_ref = "see https://internal.example/tip".to_owned();
    assert!(packet
        .validate()
        .contains(&LearningModeToggleTipCardViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Learning-mode toggles"));
    assert!(summary.contains("## Tip cards"));
    assert!(summary.contains("paused"));
    assert!(summary.contains("withheld"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 toggles + 6 tip cards
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("learning_mode_toggle"));
    assert!(csv.contains("tip_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_learning_mode_toggle_tip_card_export()
        .expect("checked learning mode toggle tip card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-learning-mode-toggle-tip-card-controls/learning_mode_toggle_paused.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-learning-mode-toggle-tip-card-controls/tip_card_withheld.json"
        )),
    ] {
        let packet: LearningModeToggleTipCardControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as learning mode toggle tip card packet");
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
        seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused(),
        seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
