use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_lifecycle_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LIFECYCLE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_family() {
    let packet = seeded_m5_lifecycle_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .object_state_rows
        .iter()
        .map(|r| r.object_family)
        .collect();
    for kind in M5LifecycleObjectFamily::ALL {
        assert!(present.contains(&kind), "missing family {}", kind.as_str());
    }
    assert_eq!(
        packet.object_state_rows.len(),
        M5LifecycleObjectFamily::ALL.len()
    );
}

#[test]
fn seeded_matrix_names_every_journey() {
    let packet = seeded_m5_lifecycle_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .journey_checkpoint_rows
        .iter()
        .map(|r| r.journey)
        .collect();
    for journey in M5CriticalJourney::ALL {
        assert!(
            present.contains(&journey),
            "missing journey {}",
            journey.as_str()
        );
    }
    assert_eq!(
        packet.journey_checkpoint_rows.len(),
        M5CriticalJourney::ALL.len()
    );
}

#[test]
fn every_object_declares_ready_and_a_status_binding() {
    let packet = seeded_m5_lifecycle_matrix();
    for row in &packet.object_state_rows {
        assert!(
            row.admitted_states.contains(&M5LifecycleState::Ready),
            "object {} missing Ready",
            row.object_family.as_str()
        );
        assert!(!row.status_code_export_field.trim().is_empty());
        assert!(!row.last_failure_reason_field.trim().is_empty());
        assert!(!row.last_failure_reason_classes.is_empty());
    }
}

#[test]
fn every_state_is_admitted_by_some_object() {
    let packet = seeded_m5_lifecycle_matrix();
    for state in M5LifecycleState::ALL {
        assert!(
            packet
                .object_state_rows
                .iter()
                .any(|row| row.admitted_states.contains(&state)),
            "no object admits state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_journey_shows_named_checkpoints_ending_in_a_terminal() {
    let packet = seeded_m5_lifecycle_matrix();
    for row in &packet.journey_checkpoint_rows {
        assert!(row.shows_named_checkpoints);
        assert!(row.checkpoints.len() >= 2);
        assert!(
            row.checkpoints.last().unwrap().is_terminal(),
            "journey {} does not end in a terminal checkpoint",
            row.journey.as_str()
        );
    }
}

#[test]
fn missing_object_family_fails_validation() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet
        .object_state_rows
        .retain(|row| row.object_family != M5LifecycleObjectFamily::AiAction);
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::RequiredObjectMissing));
}

#[test]
fn missing_journey_fails_validation() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet
        .journey_checkpoint_rows
        .retain(|row| row.journey != M5CriticalJourney::RemoteReconnect);
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::RequiredJourneyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.vocabulary_set.lifecycle_states.pop();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::VocabularySetDrift));
}

#[test]
fn object_missing_ready_state_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.object_state_rows[0]
        .admitted_states
        .retain(|s| *s != M5LifecycleState::Ready);
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::ObjectMissingReadyState));
}

#[test]
fn object_missing_status_code_field_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.object_state_rows[0].status_code_export_field.clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::StatusCodeFieldMissing));
}

#[test]
fn object_missing_last_failure_reason_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.object_state_rows[0]
        .last_failure_reason_classes
        .clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::LastFailureReasonMissing));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    let row = packet
        .object_state_rows
        .iter_mut()
        .find(|row| row.object_family == M5LifecycleObjectFamily::Workspace)
        .expect("workspace row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::StableObjectMissingProof));
}

#[test]
fn anonymous_checkpoint_journey_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.journey_checkpoint_rows[0].shows_named_checkpoints = false;
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::AnonymousOrMalformedCheckpoints));
}

#[test]
fn non_terminal_checkpoint_sequence_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    // Drop the terminal Ready checkpoint so the sequence no longer ends in a
    // terminal milestone.
    packet.journey_checkpoint_rows[0]
        .checkpoints
        .retain(|c| !c.is_terminal());
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::AnonymousOrMalformedCheckpoints));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.object_state_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.object_state_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::MissingSourceContracts));
}

#[test]
fn state_binding_review_incomplete_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet
        .state_binding_review
        .protected_journeys_show_named_checkpoints = false;
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::StateBindingReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet
        .consumer_projection
        .diagnostics_show_last_failure_reason = false;
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_lifecycle_matrix();
    packet.release_posture.telemetry_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LifecycleMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_object_and_journey() {
    let summary = seeded_m5_lifecycle_matrix().render_markdown_summary();
    for object in M5LifecycleObjectFamily::ALL {
        assert!(
            summary.contains(object.as_str()),
            "summary missing object {}",
            object.as_str()
        );
    }
    for journey in M5CriticalJourney::ALL {
        assert!(
            summary.contains(journey.as_str()),
            "summary missing journey {}",
            journey.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_object() {
    let csv = seeded_m5_lifecycle_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5LifecycleObjectFamily::ALL.len());
    assert!(lines[0].starts_with("object_family,qualification,owner,"));
    for object in M5LifecycleObjectFamily::ALL {
        assert!(
            csv.contains(object.as_str()),
            "csv missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_lifecycle_matrix_export()
        .expect("checked M5 lifecycle matrix export validates");
    assert_eq!(packet.packet_id, M5_LIFECYCLE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_lifecycle_matrix_export()
        .expect("checked M5 lifecycle matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_lifecycle_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_objects_visible() {
    for packet in [
        seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed(),
        seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.object_state_rows.len(),
            M5LifecycleObjectFamily::ALL.len()
        );
    }

    let remote = seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed();
    let row = remote
        .object_state_rows
        .iter()
        .find(|r| r.object_family == M5LifecycleObjectFamily::RemoteSession)
        .expect("remote-session row present");
    assert_eq!(row.qualification, M5LifecycleQualificationClass::Beta);

    let notebook = seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed();
    let row = notebook
        .object_state_rows
        .iter()
        .find(|r| r.object_family == M5LifecycleObjectFamily::NotebookRuntime)
        .expect("notebook-runtime row present");
    assert_eq!(row.qualification, M5LifecycleQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/m5-lifecycle-scenarios/remote_session_degraded_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/m5-lifecycle-scenarios/notebook_runtime_retest_narrowed.json"
        )),
    ] {
        let packet: M5LifecycleMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_match_seed_builders() {
    let remote: M5LifecycleMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/state/m5-lifecycle-scenarios/remote_session_degraded_narrowed.json"
    )))
    .expect("remote fixture parses");
    assert_eq!(
        remote,
        seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed()
    );

    let notebook: M5LifecycleMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/state/m5-lifecycle-scenarios/notebook_runtime_retest_narrowed.json"
    )))
    .expect("notebook fixture parses");
    assert_eq!(
        notebook,
        seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_lifecycle_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
