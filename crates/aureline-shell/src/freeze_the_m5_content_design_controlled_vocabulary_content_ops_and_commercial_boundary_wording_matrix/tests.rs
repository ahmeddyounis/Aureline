use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_content_wording_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CONTENT_WORDING_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object() {
    let packet = seeded_m5_content_wording_matrix();
    let present: std::collections::BTreeSet<_> =
        packet.object_rows.iter().map(|r| r.object_kind).collect();
    for kind in M5ContentObjectKind::ALL {
        assert!(present.contains(&kind), "missing object {}", kind.as_str());
    }
    assert_eq!(packet.object_rows.len(), M5ContentObjectKind::ALL.len());
}

#[test]
fn missing_object_fails_validation() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet
        .object_rows
        .retain(|row| row.object_kind != M5ContentObjectKind::ContentOpsArtifact);
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.vocabulary_set.hosting_boundaries.pop();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::VocabularySetDrift));
}

#[test]
fn required_vocabulary_missing_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    // The safety-critical string must declare TrustClass; drop it and its tokens.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5ContentObjectKind::SafetyCriticalUiString)
        .expect("safety-critical string row present");
    row.state_vocabularies
        .retain(|v| *v != M5ContentStateVocabulary::TrustClass);
    row.trust_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::RequiredVocabularyMissing));
}

#[test]
fn declared_vocabulary_without_tokens_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    // Keep LifecycleState declared on the glossary term but strip its tokens.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5ContentObjectKind::GlossaryTerm)
        .expect("glossary term row present");
    row.lifecycle_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::DeclaredVocabularyHasNoTokens));
}

#[test]
fn undeclared_vocabulary_with_tokens_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    // The action-label pattern does not declare HostingBoundary; add a token.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5ContentObjectKind::ActionLabelPattern)
        .expect("action-label pattern row present");
    row.hosting_boundaries
        .push(ContentHostingBoundary::ManagedCloud);
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::UndeclaredVocabularyHasTokens));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5ContentObjectKind::SafetyCriticalUiString)
        .expect("safety-critical string row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::StableObjectMissingProof));
}

#[test]
fn missing_owner_role_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.object_rows[0].owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::ObjectRowIncomplete));
}

#[test]
fn every_row_names_an_owner() {
    let packet = seeded_m5_content_wording_matrix();
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
    let mut packet = seeded_m5_content_wording_matrix();
    packet.object_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.object_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet
        .trust_review
        .ai_wording_never_overstates_confidence_or_autonomy = false;
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_objects = false;
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_content_wording_matrix();
    packet.release_posture.mirror_offline_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ContentWordingMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_object() {
    let summary = seeded_m5_content_wording_matrix().render_markdown_summary();
    for object in M5ContentObjectKind::ALL {
        assert!(
            summary.contains(object.as_str()),
            "summary missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_content_wording_matrix_export()
        .expect("checked M5 content-wording matrix export validates");
    assert_eq!(packet.packet_id, M5_CONTENT_WORDING_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_content_wording_matrix_export()
        .expect("checked M5 content-wording matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_content_wording_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_objects_visible() {
    for packet in [
        seeded_m5_content_wording_matrix_commercial_boundary_held(),
        seeded_m5_content_wording_matrix_ai_guardrail_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the object.
        assert_eq!(packet.object_rows.len(), M5ContentObjectKind::ALL.len());
    }

    let held = seeded_m5_content_wording_matrix_commercial_boundary_held();
    let row = held
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5ContentObjectKind::CommercialBoundaryWording)
        .expect("commercial-boundary row present");
    assert_eq!(row.qualification, M5ContentQualificationClass::Held);

    let narrowed = seeded_m5_content_wording_matrix_ai_guardrail_narrowed();
    let row = narrowed
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5ContentObjectKind::AiCopyGuardrail)
        .expect("ai guardrail row present");
    assert_eq!(row.qualification, M5ContentQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/commercial_boundary_wording_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/ai_copy_guardrail_narrowed.json"
        )),
    ] {
        let packet: M5ContentWordingMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_content_wording_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
