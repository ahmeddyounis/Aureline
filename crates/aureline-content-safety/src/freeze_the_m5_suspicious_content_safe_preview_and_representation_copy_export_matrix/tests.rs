use super::*;

fn packet() -> M5ContentIntegrityMatrixPacket {
    frozen_stable_m5_content_integrity_matrix_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_names_every_family() {
    let present: std::collections::BTreeSet<_> =
        packet().family_rows.iter().map(|row| row.family).collect();
    for family in M5ContentIntegrityArtifactFamily::ALL {
        assert!(
            present.contains(&family),
            "matrix missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = packet();
    packet
        .family_rows
        .retain(|row| row.family != M5ContentIntegrityArtifactFamily::AiEvidenceViewer);
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn stable_family_missing_evidence_fails() {
    let mut packet = packet();
    let row = packet
        .family_rows
        .iter_mut()
        .find(|row| row.qualification.is_stable())
        .expect("a stable family row exists");
    row.required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::StableFamilyMissingEvidence));
}

#[test]
fn empty_trust_class_ladder_fails() {
    let mut packet = packet();
    packet.family_rows[0].trust_class_ladder.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::TrustClassLadderMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.family_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.family_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn raw_copy_unreachable_on_divergence_fails() {
    let mut packet = packet();
    let row = packet
        .family_rows
        .iter_mut()
        .find(|row| {
            row.raw_rendered_posture
                == M5ContentIntegrityRawRenderedPosture::RawAndRenderedDistinctBothReachable
        })
        .expect("a divergent family row exists");
    row.copy_export_representation =
        M5ContentIntegrityCopyExportRepresentation::MetadataOnlyNoRawBody;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::RawCopyUnreachableOnDivergence));
}

#[test]
fn strong_decision_display_mode_too_weak_fails() {
    let mut packet = packet();
    let row = packet
        .family_rows
        .iter_mut()
        .find(|row| row.family == M5ContentIntegrityArtifactFamily::MarketplaceInstallUpdate)
        .expect("marketplace family row exists");
    row.display_mode = M5ContentIntegrityDisplayMode::OrdinaryBrowsing;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::StrongDecisionDisplayModeTooWeak));
}

#[test]
fn active_content_in_review_surface_fails() {
    let mut packet = packet();
    let row = packet
        .family_rows
        .iter_mut()
        .find(|row| row.family == M5ContentIntegrityArtifactFamily::AiEvidenceViewer)
        .expect("ai evidence family row exists");
    row.active_content_policy = M5ContentIntegrityActiveContentPolicy::TrustedLocalOnly;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::ActiveContentInReviewSurface));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.rendered_copy_never_masquerades_as_raw = false;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .copy_export_labels_raw_versus_rendered = false;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_family() {
    let summary = packet().render_markdown_summary();
    for family in M5ContentIntegrityArtifactFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_content_integrity_matrix_export()
        .expect("checked M5 content-integrity matrix export validates");
    assert_eq!(checked.packet_id, M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_content_integrity_matrix_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the matrix bin"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/marketplace_install_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/notebook_active_content_blocked_narrowed.json"
        )),
    ] {
        let packet: M5ContentIntegrityMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
