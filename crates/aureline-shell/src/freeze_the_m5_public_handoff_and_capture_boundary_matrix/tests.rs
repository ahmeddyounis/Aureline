use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_public_handoff_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object() {
    let packet = seeded_m5_public_handoff_matrix();
    let present: std::collections::BTreeSet<_> =
        packet.object_rows.iter().map(|r| r.object_kind).collect();
    for kind in M5HandoffObjectKind::ALL {
        assert!(present.contains(&kind), "missing object {}", kind.as_str());
    }
    assert_eq!(packet.object_rows.len(), M5HandoffObjectKind::ALL.len());
}

#[test]
fn every_vocabulary_is_carried_by_some_row() {
    let packet = seeded_m5_public_handoff_matrix();
    for vocab in M5HandoffStateVocabulary::ALL {
        assert!(
            packet
                .object_rows
                .iter()
                .any(|row| row.state_vocabularies.contains(&vocab)),
            "no row carries vocabulary {}",
            vocab.as_str()
        );
    }
}

#[test]
fn missing_object_fails_validation() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet
        .object_rows
        .retain(|row| row.object_kind != M5HandoffObjectKind::ReproductionPacket);
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.vocabulary_set.provenance_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::VocabularySetDrift));
}

#[test]
fn required_vocabulary_missing_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    // The community-handoff route must declare RouteTrustClass; drop it and its
    // tokens.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5HandoffObjectKind::CommunityHandoffRoute)
        .expect("community-handoff route row present");
    row.state_vocabularies
        .retain(|v| *v != M5HandoffStateVocabulary::RouteTrustClass);
    row.route_trust_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::RequiredVocabularyMissing));
}

#[test]
fn declared_vocabulary_without_tokens_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    // Keep ProvenanceClass declared on the post-install notice but strip tokens.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5HandoffObjectKind::PostInstallNotice)
        .expect("post-install notice row present");
    row.provenance_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::DeclaredVocabularyHasNoTokens));
}

#[test]
fn undeclared_vocabulary_with_tokens_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    // The post-install notice does not declare CapturePermissionState; add one.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5HandoffObjectKind::PostInstallNotice)
        .expect("post-install notice row present");
    row.capture_permission_states
        .push(HandoffCapturePermissionState::Granted);
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::UndeclaredVocabularyHasTokens));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5HandoffObjectKind::PostInstallNotice)
        .expect("post-install notice row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::StableObjectMissingProof));
}

#[test]
fn missing_owner_role_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.object_rows[0].owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::ObjectRowIncomplete));
}

#[test]
fn every_row_names_an_owner() {
    let packet = seeded_m5_public_handoff_matrix();
    for row in &packet.object_rows {
        assert!(
            !row.owner_role.trim().is_empty(),
            "object {} has no owner",
            row.object_kind.as_str()
        );
    }
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.object_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.object_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet
        .trust_review
        .device_mic_auth_webview_never_impersonates_native_chrome = false;
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet
        .consumer_projection
        .repro_packets_show_redaction_preview = false;
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_public_handoff_matrix();
    packet.release_posture.mirror_offline_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PublicHandoffMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_object() {
    let summary = seeded_m5_public_handoff_matrix().render_markdown_summary();
    for object in M5HandoffObjectKind::ALL {
        assert!(
            summary.contains(object.as_str()),
            "summary missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_object() {
    let csv = seeded_m5_public_handoff_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    // header + one row per object
    assert_eq!(lines.len(), 1 + M5HandoffObjectKind::ALL.len());
    assert!(lines[0].starts_with("object,qualification,owner,"));
    for object in M5HandoffObjectKind::ALL {
        assert!(
            csv.contains(object.as_str()),
            "csv missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_public_handoff_matrix_export()
        .expect("checked M5 public-handoff matrix export validates");
    assert_eq!(packet.packet_id, M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_public_handoff_matrix_export()
        .expect("checked M5 public-handoff matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_public_handoff_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_objects_visible() {
    for packet in [
        seeded_m5_public_handoff_matrix_repro_redaction_held(),
        seeded_m5_public_handoff_matrix_provenance_unverified_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the object.
        assert_eq!(packet.object_rows.len(), M5HandoffObjectKind::ALL.len());
    }

    let held = seeded_m5_public_handoff_matrix_repro_redaction_held();
    let row = held
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5HandoffObjectKind::ReproductionPacket)
        .expect("reproduction-packet row present");
    assert_eq!(row.qualification, M5HandoffQualificationClass::Held);

    let narrowed = seeded_m5_public_handoff_matrix_provenance_unverified_narrowed();
    let row = narrowed
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5HandoffObjectKind::ProvenanceDisclosure)
        .expect("provenance-disclosure row present");
    assert_eq!(row.qualification, M5HandoffQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/help/m5-public-handoff/repro_redaction_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/help/m5-public-handoff/provenance_unverified_narrowed.json"
        )),
    ] {
        let packet: M5PublicHandoffMatrixPacket =
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
    let held: M5PublicHandoffMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/m5-public-handoff/repro_redaction_held.json"
    )))
    .expect("held fixture parses");
    assert_eq!(held, seeded_m5_public_handoff_matrix_repro_redaction_held());

    let narrowed: M5PublicHandoffMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/m5-public-handoff/provenance_unverified_narrowed.json"
    )))
    .expect("narrowed fixture parses");
    assert_eq!(
        narrowed,
        seeded_m5_public_handoff_matrix_provenance_unverified_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_public_handoff_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
