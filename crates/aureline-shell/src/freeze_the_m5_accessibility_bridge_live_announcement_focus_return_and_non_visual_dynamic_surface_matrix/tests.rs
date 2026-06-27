use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_dynamic_surface_a11y_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DYNAMIC_A11Y_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object() {
    let packet = seeded_m5_dynamic_surface_a11y_matrix();
    let present: std::collections::BTreeSet<_> =
        packet.object_rows.iter().map(|r| r.object_kind).collect();
    for kind in M5DynamicSurfaceA11yObjectKind::ALL {
        assert!(present.contains(&kind), "missing object {}", kind.as_str());
    }
    assert_eq!(
        packet.object_rows.len(),
        M5DynamicSurfaceA11yObjectKind::ALL.len()
    );
}

#[test]
fn missing_object_fails_validation() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet
        .object_rows
        .retain(|row| row.object_kind != M5DynamicSurfaceA11yObjectKind::BridgeDiagnosticsPacket);
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.vocabulary_set.bridge_states.pop();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::VocabularySetDrift));
}

#[test]
fn required_vocabulary_missing_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    // The live-announcement class must declare AnnouncementPoliteness; drop it.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5DynamicSurfaceA11yObjectKind::LiveAnnouncementClass)
        .expect("live-announcement class row present");
    row.state_vocabularies
        .retain(|v| *v != M5DynamicSurfaceA11yStateVocabulary::AnnouncementPoliteness);
    row.announcement_politeness.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::RequiredVocabularyMissing));
}

#[test]
fn declared_vocabulary_without_tokens_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    // Keep BridgeState declared on the surface descriptor but strip its tokens.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| {
            row.object_kind == M5DynamicSurfaceA11yObjectKind::AccessibilitySurfaceDescriptor
        })
        .expect("surface descriptor row present");
    row.bridge_states.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::DeclaredVocabularyHasNoTokens));
}

#[test]
fn undeclared_vocabulary_with_tokens_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    // The focus-return contract does not declare AnnouncementPoliteness; add a token.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5DynamicSurfaceA11yObjectKind::FocusReturnContract)
        .expect("focus-return contract row present");
    row.announcement_politeness
        .push(A11yAnnouncementPoliteness::Assertive);
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::UndeclaredVocabularyHasTokens));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5DynamicSurfaceA11yObjectKind::FocusReturnContract)
        .expect("focus-return contract row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::StableObjectMissingProof));
}

#[test]
fn missing_owner_role_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.object_rows[0].owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ObjectRowIncomplete));
}

#[test]
fn every_row_names_an_owner() {
    let packet = seeded_m5_dynamic_surface_a11y_matrix();
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
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.object_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.object_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet
        .conformance_review
        .focus_never_teleports_or_vanishes_on_async_update = false;
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet
        .consumer_projection
        .unqualified_surfaces_labeled_when_uncovered = false;
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_surface_a11y_matrix();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5DynamicSurfaceA11yMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_object() {
    let summary = seeded_m5_dynamic_surface_a11y_matrix().render_markdown_summary();
    for object in M5DynamicSurfaceA11yObjectKind::ALL {
        assert!(
            summary.contains(object.as_str()),
            "summary missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_dynamic_surface_a11y_matrix_export()
        .expect("checked M5 dynamic-surface a11y matrix export validates");
    assert_eq!(packet.packet_id, M5_DYNAMIC_A11Y_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_dynamic_surface_a11y_matrix_export()
        .expect("checked M5 dynamic-surface a11y matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_dynamic_surface_a11y_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_objects_visible() {
    for packet in [
        seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable(),
        seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the object.
        assert_eq!(
            packet.object_rows.len(),
            M5DynamicSurfaceA11yObjectKind::ALL.len()
        );
    }

    let held = seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable();
    let row = held
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5DynamicSurfaceA11yObjectKind::BridgeDiagnosticsPacket)
        .expect("bridge-diagnostics row present");
    assert_eq!(
        row.qualification,
        M5DynamicSurfaceA11yQualificationClass::Held
    );

    let narrowed = seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed();
    let row = narrowed
        .object_rows
        .iter()
        .find(|r| r.object_kind == M5DynamicSurfaceA11yObjectKind::DenseSurfaceNonVisualSummary)
        .expect("dense-summary row present");
    assert_eq!(
        row.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-dynamic-surfaces/bridge_unavailable.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-dynamic-surfaces/dense_summary_narrowed.json"
        )),
    ] {
        let packet: M5DynamicSurfaceA11yMatrixPacket =
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
    let json = seeded_m5_dynamic_surface_a11y_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
