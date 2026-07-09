use super::*;

const PACKET_ID: &str = INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_PACKET_ID;

type Violation = IncidentSnapshotCardDesktopHandoffSheetViolation;

fn packet() -> IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls()
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
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_VERSION
    );
}

#[test]
fn incident_awareness_is_derived_not_asserted() {
    use IncidentAwarenessClass as Awareness;
    use IncidentStatus as Status;

    // Firing → active-unacknowledged, live, open (awareness note required).
    let d = resolve_incident_awareness(Status::Firing);
    assert_eq!(d.awareness_class, Awareness::ActiveUnacknowledged);
    assert!(d.is_live_status);
    assert!(d.is_open);
    assert!(d.needs_awareness_note);

    // Acknowledged / investigating → active-acknowledged, live, open.
    for status in [Status::Acknowledged, Status::Investigating] {
        let d = resolve_incident_awareness(status);
        assert_eq!(d.awareness_class, Awareness::ActiveAcknowledged);
        assert!(d.is_live_status);
        assert!(d.is_open);
    }

    // Mitigating → mitigating, live, open.
    let d = resolve_incident_awareness(Status::Mitigating);
    assert_eq!(d.awareness_class, Awareness::Mitigating);
    assert!(d.is_open);

    // Resolved → resolved, live, not open.
    let d = resolve_incident_awareness(Status::Resolved);
    assert_eq!(d.awareness_class, Awareness::Resolved);
    assert!(d.is_resolved);
    assert!(!d.is_open);
    assert!(!d.needs_awareness_note);

    // Stale → stale-unknown, never live, stale note required.
    let d = resolve_incident_awareness(Status::Stale);
    assert_eq!(d.awareness_class, Awareness::StaleUnknown);
    assert!(!d.is_live_status);
    assert!(d.needs_stale_note);
    assert!(!d.is_open);
}

#[test]
fn handoff_open_is_derived_not_asserted() {
    use HandoffOpenClass as Open;
    use M5CompanionHandoffTarget as Target;

    // File location → opens exact location.
    let d = resolve_handoff_open(Target::FileLocation);
    assert_eq!(d.open_class, Open::OpensExactLocation);
    assert!(d.is_openable);

    // Review panel / CI pipeline run → opens exact panel.
    for target in [Target::ReviewPanel, Target::CiPipelineRun] {
        let d = resolve_handoff_open(target);
        assert_eq!(d.open_class, Open::OpensExactPanel);
        assert!(d.is_openable);
    }

    // Incident workspace / agent session → opens exact workspace.
    for target in [Target::IncidentWorkspace, Target::AgentSession] {
        let d = resolve_handoff_open(target);
        assert_eq!(d.open_class, Open::OpensExactWorkspace);
        assert!(d.is_openable);
    }

    // No handoff → not openable, not-openable note required.
    let d = resolve_handoff_open(Target::NoHandoff);
    assert_eq!(d.open_class, Open::NotOpenable);
    assert!(!d.is_openable);
    assert!(d.needs_not_openable_note);
}

#[test]
fn awareness_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .incident_snapshot_cards
        .iter()
        .map(|card| card.awareness_disclosure().awareness_class)
        .collect();
    for class in IncidentAwarenessClass::ALL {
        assert!(
            covered.contains(&class),
            "missing awareness class {class:?}"
        );
    }
}

#[test]
fn incident_status_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .incident_snapshot_cards
        .iter()
        .map(|card| card.incident_status)
        .collect();
    for status in IncidentStatus::ALL {
        assert!(
            covered.contains(&status),
            "missing incident status {status:?}"
        );
    }
}

#[test]
fn handoff_open_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .desktop_handoff_sheets
        .iter()
        .map(|sheet| sheet.open_disclosure().open_class)
        .collect();
    for class in HandoffOpenClass::ALL {
        assert!(covered.contains(&class), "missing open class {class:?}");
    }
}

#[test]
fn handoff_target_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .desktop_handoff_sheets
        .iter()
        .map(|sheet| sheet.handoff_target)
        .collect();
    for target in M5CompanionHandoffTarget::ALL {
        assert!(
            covered.contains(&target),
            "missing handoff target {target:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet.validate().contains(&Violation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&Violation::MissingSourceContracts));
}

#[test]
fn empty_incident_cards_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards.clear();
    assert!(packet
        .validate()
        .contains(&Violation::IncidentSnapshotCardsMissing));
}

#[test]
fn empty_handoff_sheets_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets.clear();
    assert!(packet
        .validate()
        .contains(&Violation::DesktopHandoffSheetsMissing));
}

#[test]
fn card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].component = M5CompanionComponentFamily::DesktopHandoffSheet;
    assert!(packet
        .validate()
        .contains(&Violation::IncidentSnapshotCardWrongComponentClass));
}

#[test]
fn sheet_wrong_component_class_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].component = M5CompanionComponentFamily::IncidentSnapshotCard;
    assert!(packet
        .validate()
        .contains(&Violation::DesktopHandoffSheetWrongComponentClass));
}

#[test]
fn stale_card_claiming_live_status_fails() {
    let mut packet = packet();
    let card = packet
        .incident_snapshot_cards
        .iter_mut()
        .find(|card| card.awareness_class == IncidentAwarenessClass::StaleUnknown)
        .expect("stale card present");
    card.claims_live_status = true;
    assert!(packet
        .validate()
        .contains(&Violation::AwarenessStateMisrepresented));
}

#[test]
fn misdeclared_awareness_class_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].awareness_class = IncidentAwarenessClass::StaleUnknown;
    assert!(packet
        .validate()
        .contains(&Violation::AwarenessStateMisrepresented));
}

#[test]
fn missing_service_or_run_identity_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].run_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ServiceOrRunIdentityMissing));
}

#[test]
fn missing_service_label_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].service_label.clear();
    assert!(packet.validate().contains(&Violation::ServiceLabelMissing));
}

#[test]
fn missing_severity_label_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].severity_label.clear();
    assert!(packet.validate().contains(&Violation::SeverityLabelMissing));
}

#[test]
fn missing_incident_stale_note_fails() {
    let mut packet = packet();
    let card = packet
        .incident_snapshot_cards
        .iter_mut()
        .find(|card| card.awareness_class == IncidentAwarenessClass::StaleUnknown)
        .expect("stale card present");
    card.stale_note.clear();
    assert!(packet.validate().contains(&Violation::StaleNoteMissing));
}

#[test]
fn missing_awareness_note_fails() {
    let mut packet = packet();
    let card = packet
        .incident_snapshot_cards
        .iter_mut()
        .find(|card| card.awareness_disclosure().is_open)
        .expect("open card present");
    card.awareness_note.clear();
    assert!(packet.validate().contains(&Violation::AwarenessNoteMissing));
}

#[test]
fn overpromising_remediation_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].implies_companion_remediation = true;
    assert!(packet
        .validate()
        .contains(&Violation::RemediationDepthOverpromised));
}

#[test]
fn missing_incident_open_verb_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].status_verbs = vec![IncidentSnapshotCardVerb::ViewTimeline];
    assert!(packet
        .validate()
        .contains(&Violation::IncidentVerbsIncomplete));
}

#[test]
fn card_handoff_verb_without_target_fails() {
    let mut packet = packet();
    // The stale card has NoHandoff; adding the handoff verb must fail.
    let card = packet
        .incident_snapshot_cards
        .iter_mut()
        .find(|card| card.handoff_target == M5CompanionHandoffTarget::NoHandoff)
        .expect("no-handoff card present");
    card.status_verbs
        .push(IncidentSnapshotCardVerb::HandoffToDesktop);
    assert!(packet
        .validate()
        .contains(&Violation::HandoffTargetUnresolved));
}

#[test]
fn missing_target_identity_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].target_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::TargetIdentityMissing));
}

#[test]
fn missing_target_object_label_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].target_object_label.clear();
    assert!(packet
        .validate()
        .contains(&Violation::TargetObjectLabelMissing));
}

#[test]
fn misdeclared_open_class_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].open_class = HandoffOpenClass::NotOpenable;
    assert!(packet
        .validate()
        .contains(&Violation::HandoffOpenMisrepresented));
}

#[test]
fn not_openable_sheet_claiming_openable_fails() {
    let mut packet = packet();
    let sheet = packet
        .desktop_handoff_sheets
        .iter_mut()
        .find(|sheet| sheet.open_class == HandoffOpenClass::NotOpenable)
        .expect("not-openable sheet present");
    sheet.claims_openable = true;
    assert!(packet
        .validate()
        .contains(&Violation::HandoffOpenMisrepresented));
}

#[test]
fn missing_opens_on_desktop_note_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0]
        .opens_on_desktop_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::OpensOnDesktopNoteMissing));
}

#[test]
fn missing_not_openable_note_fails() {
    let mut packet = packet();
    let sheet = packet
        .desktop_handoff_sheets
        .iter_mut()
        .find(|sheet| sheet.open_class == HandoffOpenClass::NotOpenable)
        .expect("not-openable sheet present");
    sheet.not_openable_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::NotOpenableNoteMissing));
}

#[test]
fn missing_auth_tenant_reminder_fails() {
    let mut packet = packet();
    let sheet = packet
        .desktop_handoff_sheets
        .iter_mut()
        .find(|sheet| sheet.auth_context.needs_reminder())
        .expect("reminder-bearing sheet present");
    sheet.auth_tenant_reminder_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::AuthTenantReminderMissing));
}

#[test]
fn ambiguous_handoff_into_not_openable_fails() {
    let mut packet = packet();
    // A not-openable sheet that offers open-on-desktop must fail.
    let sheet = packet
        .desktop_handoff_sheets
        .iter_mut()
        .find(|sheet| sheet.open_class == HandoffOpenClass::NotOpenable)
        .expect("not-openable sheet present");
    sheet
        .handoff_verbs
        .push(DesktopHandoffSheetVerb::OpenOnDesktop);
    let found = packet.validate();
    assert!(found.contains(&Violation::AmbiguousHandoffOffered));
    assert!(found.contains(&Violation::HandoffTargetUnresolved));
}

#[test]
fn missing_sheet_open_verb_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].handoff_verbs = vec![DesktopHandoffSheetVerb::OpenOnDesktop];
    assert!(packet
        .validate()
        .contains(&Violation::DesktopHandoffVerbsIncomplete));
}

#[test]
fn missing_object_landing_ref_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].object_landing_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ObjectLandingRefMissing));
}

#[test]
fn missing_handoff_label_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].handoff_label.clear();
    assert!(packet.validate().contains(&Violation::HandoffLabelMissing));
}

#[test]
fn missing_scope_and_freshness_note_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0]
        .scope_and_freshness_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::ScopeAndFreshnessNoteMissing));
}

#[test]
fn card_masking_scope_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].masks_scope_or_freshness = true;
    assert!(packet
        .validate()
        .contains(&Violation::ScopeOrFreshnessMasked));
}

#[test]
fn sheet_implying_desktop_action_companion_safe_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].implies_desktop_action_is_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&Violation::DesktopActionImpliedCompanionSafe));
}

#[test]
fn card_hiding_capability_boundary_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].hides_capability_boundary = true;
    assert!(packet
        .validate()
        .contains(&Violation::CapabilityBoundaryHidden));
}

#[test]
fn sheet_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&Violation::AlternateStateLabelInvented));
}

#[test]
fn routes_to_generic_activity_page_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].routes_to_generic_activity_page = true;
    assert!(packet
        .validate()
        .contains(&Violation::RoutesToGenericActivityPage));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].required_labels = vec![M5CompanionRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&Violation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_reasons_fails() {
    let mut packet = packet();
    packet.desktop_handoff_sheets[0].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&Violation::DegradedReasonsMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].accessibility_routes =
        vec![M5CompanionAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&Violation::AccessibilityRouteMissing));
}

#[test]
fn glance_review_incomplete_fails() {
    let mut packet = packet();
    packet.glance_review.stale_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&Violation::GlanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remediation_and_open_posture_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&Violation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&Violation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.incident_snapshot_cards[0].object_landing_ref =
        "see https://internal.example/inc".to_owned();
    assert!(packet
        .validate()
        .contains(&Violation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Incident-snapshot cards"));
    assert!(summary.contains("## Desktop-handoff sheets"));
    assert!(summary.contains("stale_unknown"));
    assert!(summary.contains("not_openable"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 incident-snapshot cards + 6 desktop-handoff sheets
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("incident_snapshot_card"));
    assert!(csv.contains("desktop_handoff_sheet"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_incident_snapshot_card_desktop_handoff_sheet_export()
        .expect("checked incident snapshot card desktop handoff sheet export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls/incident_snapshot_card_stale.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls/desktop_handoff_sheet_not_openable.json"
        )),
    ] {
        let packet: IncidentSnapshotCardDesktopHandoffSheetControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as incident snapshot card desktop handoff sheet packet");
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
        seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale(),
        seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
