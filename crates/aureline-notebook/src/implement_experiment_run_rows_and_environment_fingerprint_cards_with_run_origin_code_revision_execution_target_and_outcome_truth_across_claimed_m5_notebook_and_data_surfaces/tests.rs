use super::*;

const PACKET_ID: &str = EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_PACKET_ID;

fn packet() -> ExperimentRunRowEnvironmentFingerprintControlsPacket {
    seeded_experiment_run_row_environment_fingerprint_controls()
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
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_VERSION
    );
}

#[test]
fn origin_is_derived_not_asserted() {
    use M5RunOriginKind as Origin;
    use RunOriginClass as Class;

    // Notebook cell / script task → local run, first-party.
    for origin in [Origin::NotebookCell, Origin::ScriptTask] {
        let d = resolve_run_origin(origin);
        assert_eq!(d.origin_class, Class::LocalRun);
        assert!(d.is_first_party_origin);
    }

    // Scheduled task → managed run, first-party.
    let d = resolve_run_origin(Origin::ScheduledTask);
    assert_eq!(d.origin_class, Class::ManagedRun);
    assert!(d.is_first_party_origin);

    // Imported run → imported, not first-party, needs imported note.
    let d = resolve_run_origin(Origin::ImportedRun);
    assert_eq!(d.origin_class, Class::ImportedRun);
    assert!(!d.is_first_party_origin);
    assert!(d.needs_imported_note);

    // Manual attach → manually attached, not first-party, needs manual note.
    let d = resolve_run_origin(Origin::ManualAttach);
    assert_eq!(d.origin_class, Class::ManuallyAttached);
    assert!(!d.is_first_party_origin);
    assert!(d.needs_manual_attach_note);

    // Unknown origin → origin unknown, not first-party, needs unknown note.
    let d = resolve_run_origin(Origin::UnknownOrigin);
    assert_eq!(d.origin_class, Class::OriginUnknown);
    assert!(!d.is_first_party_origin);
    assert!(d.needs_unknown_origin_note);
}

#[test]
fn capture_is_derived_not_asserted() {
    use FingerprintCaptureClass as Class;
    use M5FingerprintState as State;

    // Captured complete → captured, reliably captured.
    let d = resolve_fingerprint_capture(State::CapturedComplete);
    assert_eq!(d.capture_class, Class::Captured);
    assert!(d.is_reliably_captured);

    // Captured partial → partially captured, not reliably, needs partial note.
    let d = resolve_fingerprint_capture(State::CapturedPartial);
    assert_eq!(d.capture_class, Class::PartiallyCaptured);
    assert!(!d.is_reliably_captured);
    assert!(d.needs_partial_note);

    // Pinned → pinned, reliably captured.
    let d = resolve_fingerprint_capture(State::Pinned);
    assert_eq!(d.capture_class, Class::Pinned);
    assert!(d.is_reliably_captured);

    // Missing / drifted / unavailable → uncaptured, not reliably, needs uncaptured note.
    for state in [State::CapturedMissing, State::Drifted, State::Unavailable] {
        let d = resolve_fingerprint_capture(state);
        assert_eq!(d.capture_class, Class::Uncaptured);
        assert!(!d.is_reliably_captured);
        assert!(d.needs_uncaptured_note);
    }
}

#[test]
fn origin_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .run_rows
        .iter()
        .map(|row| row.origin_disclosure().origin_class)
        .collect();
    for class in RunOriginClass::ALL {
        assert!(covered.contains(&class), "missing origin class {class:?}");
    }
}

#[test]
fn run_origin_and_status_coverage_is_complete() {
    let packet = packet();
    let origins: std::collections::BTreeSet<_> =
        packet.run_rows.iter().map(|r| r.origin_kind).collect();
    for origin in M5RunOriginKind::ALL {
        assert!(origins.contains(&origin), "missing origin {origin:?}");
    }
    let statuses: std::collections::BTreeSet<_> =
        packet.run_rows.iter().map(|r| r.status_state).collect();
    for status in M5RunStatusState::ALL {
        assert!(statuses.contains(&status), "missing status {status:?}");
    }
}

#[test]
fn capture_class_scope_and_state_coverage_is_complete() {
    let packet = packet();
    let capture: std::collections::BTreeSet<_> = packet
        .fingerprint_cards
        .iter()
        .map(|card| card.capture_disclosure().capture_class)
        .collect();
    for class in FingerprintCaptureClass::ALL {
        assert!(capture.contains(&class), "missing capture class {class:?}");
    }
    let scopes: std::collections::BTreeSet<_> = packet
        .fingerprint_cards
        .iter()
        .map(|c| c.scope_class)
        .collect();
    for scope in M5FingerprintScopeClass::ALL {
        assert!(scopes.contains(&scope), "missing scope {scope:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .fingerprint_cards
        .iter()
        .map(|c| c.fingerprint_state)
        .collect();
    for state in M5FingerprintState::ALL {
        assert!(states.contains(&state), "missing state {state:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::MissingSourceContracts));
}

#[test]
fn empty_run_rows_fails() {
    let mut packet = packet();
    packet.run_rows.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RunRowsMissing));
}

#[test]
fn empty_fingerprint_cards_fails() {
    let mut packet = packet();
    packet.fingerprint_cards.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardsMissing));
}

#[test]
fn run_row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.run_rows[0].component = M5ExperimentComponentFamily::EnvironmentFingerprintCard;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RunRowWrongComponentClass));
}

#[test]
fn fingerprint_card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].component = M5ExperimentComponentFamily::ExperimentRunRow;
    assert!(packet.validate().contains(
        &ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardWrongComponentClass
    ));
}

#[test]
fn imported_run_claiming_first_party_fails() {
    let mut packet = packet();
    let row = packet
        .run_rows
        .iter_mut()
        .find(|r| r.origin_class == RunOriginClass::ImportedRun)
        .expect("imported run present");
    row.claims_first_party_origin = true;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::OriginMisrepresented));
}

#[test]
fn uncaptured_card_claiming_captured_fails() {
    let mut packet = packet();
    let card = packet
        .fingerprint_cards
        .iter_mut()
        .find(|c| c.capture_class == FingerprintCaptureClass::Uncaptured)
        .expect("uncaptured card present");
    card.claims_captured = true;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::CaptureMisrepresented));
}

#[test]
fn missing_imported_note_fails() {
    let mut packet = packet();
    let row = packet
        .run_rows
        .iter_mut()
        .find(|r| r.origin_class == RunOriginClass::ImportedRun)
        .expect("imported run present");
    row.imported_note.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::ImportedNoteMissing));
}

#[test]
fn missing_unknown_origin_note_fails() {
    let mut packet = packet();
    let row = packet
        .run_rows
        .iter_mut()
        .find(|r| r.origin_class == RunOriginClass::OriginUnknown)
        .expect("unknown-origin run present");
    row.unknown_origin_note.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::UnknownOriginNoteMissing));
}

#[test]
fn missing_uncaptured_note_fails() {
    let mut packet = packet();
    let card = packet
        .fingerprint_cards
        .iter_mut()
        .find(|c| c.capture_class == FingerprintCaptureClass::Uncaptured)
        .expect("uncaptured card present");
    card.uncaptured_note.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::UncapturedNoteMissing));
}

#[test]
fn missing_code_revision_fails() {
    let mut packet = packet();
    packet.run_rows[0].code_revision.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::CodeRevisionMissing));
}

#[test]
fn missing_interpreter_or_kernel_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0]
        .interpreter_or_kernel_label
        .clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::InterpreterOrKernelMissing));
}

#[test]
fn run_row_missing_compare_action_fails() {
    let mut packet = packet();
    packet.run_rows[0].run_actions = vec![RunRowAction::OpenRun];
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RunRowActionsIncomplete));
}

#[test]
fn fingerprint_card_missing_export_action_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].card_actions = vec![FingerprintCardAction::InspectFingerprint];
    assert!(packet.validate().contains(
        &ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardActionsIncomplete
    ));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    // The first run row offers OpenDeepLink; blank its kind to NoDeepLink.
    packet.run_rows[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.run_rows[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.run_rows[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::ContextNoteMissing));
}

#[test]
fn missing_origin_and_status_note_fails() {
    let mut packet = packet();
    packet.run_rows[0].origin_and_status_note.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::OriginAndStatusNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.run_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::DispositionsMissing));
}

#[test]
fn run_row_masking_provenance_fails() {
    let mut packet = packet();
    packet.run_rows[0].masks_provenance_or_sensitivity_state = true;
    assert!(packet.validate().contains(
        &ExperimentRunRowEnvironmentFingerprintViolation::ProvenanceOrSensitivityStateMasked
    ));
}

#[test]
fn run_row_hiding_origin_or_revision_fails() {
    let mut packet = packet();
    packet.run_rows[0].hides_run_origin_or_revision = true;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RunOriginOrRevisionHidden));
}

#[test]
fn card_implying_apples_to_apples_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].implies_apples_to_apples_without_parity = true;
    assert!(packet.validate().contains(
        &ExperimentRunRowEnvironmentFingerprintViolation::ApplesToApplesImpliedWithoutParity
    ));
}

#[test]
fn card_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::AlternateStateLabelInvented));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.run_rows[0].required_labels = vec![M5ExperimentRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.fingerprint_cards[0].accessibility_routes =
        vec![M5ExperimentAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::AccessibilityRouteMissing));
}

#[test]
fn experiment_review_incomplete_fails() {
    let mut packet = packet();
    packet.experiment_review.uncaptured_never_shown_as_captured = false;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::ExperimentReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .environment_capture_visible_before_trust = false;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.run_rows[0].deep_link_ref = "see https://internal.example/run".to_owned();
    assert!(packet
        .validate()
        .contains(&ExperimentRunRowEnvironmentFingerprintViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Experiment run rows"));
    assert!(summary.contains("## Environment fingerprint cards"));
    assert!(summary.contains("imported_run"));
    assert!(summary.contains("uncaptured"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 run rows + 6 fingerprint cards
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("experiment_run_row"));
    assert!(csv.contains("environment_fingerprint_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_experiment_run_row_environment_fingerprint_export()
        .expect("checked experiment run row fingerprint export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls/run_row_imported.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls/fingerprint_card_uncaptured.json"
        )),
    ] {
        let packet: ExperimentRunRowEnvironmentFingerprintControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as experiment run row fingerprint packet");
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
        seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported(),
        seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
