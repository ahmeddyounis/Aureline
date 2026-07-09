use super::*;

const PACKET_ID: &str = NOTIFICATION_ROW_MOBILE_REVIEW_CARD_PACKET_ID;

fn packet() -> NotificationRowMobileReviewCardControlsPacket {
    seeded_notification_row_mobile_review_card_controls()
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
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_VERSION
    );
}

#[test]
fn delivery_state_is_derived_not_asserted() {
    use M5CompanionFreshness as Fresh;
    use NotificationDeliveryClass as Delivery;

    // Live → live.
    let d = resolve_notification_delivery(Fresh::Live);
    assert_eq!(d.delivery_class, Delivery::Live);
    assert!(d.is_live);

    // Cached → cached, never live.
    let d = resolve_notification_delivery(Fresh::Cached);
    assert_eq!(d.delivery_class, Delivery::Cached);
    assert!(!d.is_live);
    assert!(d.needs_cached_note);

    // Stale / offline-held / expired-snapshot → stale, never live.
    for fresh in [Fresh::Stale, Fresh::OfflineHeld, Fresh::ExpiredSnapshot] {
        let d = resolve_notification_delivery(fresh);
        assert_eq!(d.delivery_class, Delivery::Stale);
        assert!(!d.is_live);
        assert!(d.needs_stale_note);
    }

    // Unknown freshness → unknown, never live.
    let d = resolve_notification_delivery(Fresh::UnknownFreshness);
    assert_eq!(d.delivery_class, Delivery::Unknown);
    assert!(!d.is_live);
    assert!(d.needs_unknown_note);
}

#[test]
fn capability_state_is_derived_not_asserted() {
    use M5CompanionComponentDisposition as Disp;
    use ReviewCapabilityClass as Capability;

    // Comment-capable → companion sufficient.
    let d = resolve_review_capability(Disp::CommentCapable);
    assert_eq!(d.capability_class, Capability::CommentCapable);
    assert!(d.companion_execution_sufficient);

    // Review-only / cached / stale floor to review-only, still companion sufficient.
    for disp in [Disp::ReviewOnly, Disp::Cached, Disp::Stale] {
        let d = resolve_review_capability(disp);
        assert_eq!(d.capability_class, Capability::ReviewOnly);
        assert!(d.companion_execution_sufficient);
    }

    // Desktop-required / handoff-ready → desktop-required, never companion sufficient.
    for disp in [Disp::DesktopRequired, Disp::HandoffReady] {
        let d = resolve_review_capability(disp);
        assert_eq!(d.capability_class, Capability::DesktopRequired);
        assert!(!d.companion_execution_sufficient);
        assert!(d.needs_desktop_required_note);
    }

    // Policy-blocked → policy-blocked, never companion sufficient.
    let d = resolve_review_capability(Disp::PolicyBlocked);
    assert_eq!(d.capability_class, Capability::PolicyBlocked);
    assert!(!d.companion_execution_sufficient);
    assert!(d.needs_policy_blocked_note);
}

#[test]
fn delivery_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .notification_rows
        .iter()
        .map(|row| row.delivery_disclosure().delivery_class)
        .collect();
    for class in NotificationDeliveryClass::ALL {
        assert!(covered.contains(&class), "missing delivery class {class:?}");
    }
}

#[test]
fn severity_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .notification_rows
        .iter()
        .map(|row| row.severity)
        .collect();
    for severity in M5CompanionSeverity::ALL {
        assert!(covered.contains(&severity), "missing severity {severity:?}");
    }
}

#[test]
fn capability_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .review_cards
        .iter()
        .map(|card| card.capability_disclosure().capability_class)
        .collect();
    for class in ReviewCapabilityClass::ALL {
        assert!(covered.contains(&class), "missing capability {class:?}");
    }
}

#[test]
fn review_kind_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .review_cards
        .iter()
        .map(|card| card.review_kind)
        .collect();
    for kind in M5CompanionReviewKind::ALL {
        assert!(covered.contains(&kind), "missing review kind {kind:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::MissingSourceContracts));
}

#[test]
fn empty_notification_rows_fails() {
    let mut packet = packet();
    packet.notification_rows.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::NotificationRowsMissing));
}

#[test]
fn empty_review_cards_fails() {
    let mut packet = packet();
    packet.review_cards.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ReviewCardsMissing));
}

#[test]
fn row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.notification_rows[0].component = M5CompanionComponentFamily::MobileReviewCard;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::NotificationRowWrongComponentClass));
}

#[test]
fn card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.review_cards[0].component = M5CompanionComponentFamily::NotificationRow;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ReviewCardWrongComponentClass));
}

#[test]
fn stale_row_claiming_live_fails() {
    let mut packet = packet();
    let row = packet
        .notification_rows
        .iter_mut()
        .find(|row| row.delivery_class == NotificationDeliveryClass::Stale)
        .expect("stale row present");
    row.claims_live = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::DeliveryStateMisrepresented));
}

#[test]
fn misdeclared_delivery_class_fails() {
    let mut packet = packet();
    packet.notification_rows[0].delivery_class = NotificationDeliveryClass::Stale;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::DeliveryStateMisrepresented));
}

#[test]
fn desktop_required_card_claiming_sufficient_fails() {
    let mut packet = packet();
    let card = packet
        .review_cards
        .iter_mut()
        .find(|card| card.capability_class == ReviewCapabilityClass::DesktopRequired)
        .expect("desktop-required card present");
    card.claims_companion_sufficient = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::CapabilityMisrepresented));
}

#[test]
fn missing_stale_note_fails() {
    let mut packet = packet();
    let row = packet
        .notification_rows
        .iter_mut()
        .find(|row| row.delivery_class == NotificationDeliveryClass::Stale)
        .expect("stale row present");
    row.stale_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::StaleNoteMissing));
}

#[test]
fn missing_cached_note_fails() {
    let mut packet = packet();
    let row = packet
        .notification_rows
        .iter_mut()
        .find(|row| row.delivery_class == NotificationDeliveryClass::Cached)
        .expect("cached row present");
    row.cached_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::CachedNoteMissing));
}

#[test]
fn missing_desktop_required_note_fails() {
    let mut packet = packet();
    let card = packet
        .review_cards
        .iter_mut()
        .find(|card| card.capability_class == ReviewCapabilityClass::DesktopRequired)
        .expect("desktop-required card present");
    card.desktop_required_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::DesktopRequiredNoteMissing));
}

#[test]
fn missing_policy_blocked_note_fails() {
    let mut packet = packet();
    let card = packet
        .review_cards
        .iter_mut()
        .find(|card| card.capability_class == ReviewCapabilityClass::PolicyBlocked)
        .expect("policy-blocked card present");
    card.policy_blocked_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::PolicyBlockedNoteMissing));
}

#[test]
fn missing_scope_and_freshness_note_fails() {
    let mut packet = packet();
    packet.notification_rows[0].scope_and_freshness_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ScopeAndFreshnessNoteMissing));
}

#[test]
fn missing_object_landing_ref_fails() {
    let mut packet = packet();
    packet.notification_rows[0].object_landing_ref.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ObjectLandingRefMissing));
}

#[test]
fn missing_open_verb_fails() {
    let mut packet = packet();
    packet.notification_rows[0].triage_verbs = vec![NotificationTriageVerb::MarkRead];
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::NotificationVerbsIncomplete));
}

#[test]
fn missing_review_open_verb_fails() {
    let mut packet = packet();
    packet.review_cards[0].review_verbs = vec![MobileReviewVerb::Comment];
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ReviewVerbsIncomplete));
}

#[test]
fn handoff_verb_without_target_fails() {
    let mut packet = packet();
    // The unknown-freshness row has NoHandoff; adding the handoff verb must fail.
    let row = packet
        .notification_rows
        .iter_mut()
        .find(|row| row.handoff_target == M5CompanionHandoffTarget::NoHandoff)
        .expect("no-handoff row present");
    row.triage_verbs
        .push(NotificationTriageVerb::HandoffToDesktop);
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::HandoffTargetUnresolved));
}

#[test]
fn missing_handoff_label_fails() {
    let mut packet = packet();
    packet.notification_rows[0].handoff_label.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::HandoffLabelMissing));
}

#[test]
fn missing_severity_label_fails() {
    let mut packet = packet();
    packet.notification_rows[0].severity_label.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::SeverityLabelMissing));
}

#[test]
fn missing_capability_note_fails() {
    let mut packet = packet();
    packet.review_cards[0].capability_note.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::CapabilityNoteMissing));
}

#[test]
fn missing_review_kind_label_fails() {
    let mut packet = packet();
    packet.review_cards[0].review_kind_label.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ReviewKindLabelMissing));
}

#[test]
fn row_masking_scope_fails() {
    let mut packet = packet();
    packet.notification_rows[0].masks_scope_or_freshness = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ScopeOrFreshnessMasked));
}

#[test]
fn card_hiding_capability_boundary_fails() {
    let mut packet = packet();
    packet.review_cards[0].hides_capability_boundary = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::CapabilityBoundaryHidden));
}

#[test]
fn card_implying_desktop_action_companion_safe_fails() {
    let mut packet = packet();
    packet.review_cards[0].implies_desktop_action_is_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::DesktopActionImpliedCompanionSafe));
}

#[test]
fn row_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.notification_rows[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::AlternateStateLabelInvented));
}

#[test]
fn routes_to_generic_activity_page_fails() {
    let mut packet = packet();
    packet.notification_rows[0].routes_to_generic_activity_page = true;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::RoutesToGenericActivityPage));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.notification_rows[0].required_labels = vec![M5CompanionRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_reasons_fails() {
    let mut packet = packet();
    packet.review_cards[0].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::DegradedReasonsMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.notification_rows[0].accessibility_routes =
        vec![M5CompanionAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::AccessibilityRouteMissing));
}

#[test]
fn glance_review_incomplete_fails() {
    let mut packet = packet();
    packet.glance_review.stale_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::GlanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .capability_boundary_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.notification_rows[0].object_landing_ref = "see https://internal.example/obj".to_owned();
    assert!(packet
        .validate()
        .contains(&NotificationRowMobileReviewCardViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Notification rows"));
    assert!(summary.contains("## Mobile review cards"));
    assert!(summary.contains("stale"));
    assert!(summary.contains("desktop_required"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 notification rows + 6 review cards
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("notification_row"));
    assert!(csv.contains("mobile_review_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_notification_row_mobile_review_card_export()
        .expect("checked notification row mobile review card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-notification-row-mobile-review-card-controls/notification_row_stale.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-notification-row-mobile-review-card-controls/mobile_review_card_desktop_required.json"
        )),
    ] {
        let packet: NotificationRowMobileReviewCardControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as notification row review card packet");
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
        seeded_notification_row_mobile_review_card_controls_notification_row_stale(),
        seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
