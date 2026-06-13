use super::*;

fn packet() -> M5ContentIntegrityCertificationPacket {
    frozen_m5_content_integrity_certification_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_certifies_every_family() {
    let present: BTreeSet<_> = packet().family_rows.iter().map(|row| row.family).collect();
    for family in M5ContentIntegrityArtifactFamily::ALL {
        assert!(
            present.contains(&family),
            "certification missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn frozen_packet_has_no_narrowed_rows() {
    let packet = packet();
    assert_eq!(packet.summary.narrowed_families, 0);
    for row in &packet.family_rows {
        assert!(
            !row.was_narrowed(),
            "{} narrowed unexpectedly",
            row.family.as_str()
        );
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.certified_qualification, row.claimed_qualification);
    }
}

#[test]
fn missing_proof_narrows_to_experimental() {
    let row = project_m5_content_integrity_certification(M5CertificationFamilySeed {
        family: M5ContentIntegrityArtifactFamily::AiEvidenceViewer,
        claimed_qualification: M5ContentIntegrityQualificationClass::Stable,
        dimensions: vec![
            current_dim(M5CertificationProofDimension::SuspiciousContentCues, "ok"),
            M5CertificationDimensionInput {
                dimension: M5CertificationProofDimension::SafePreviewTrustClass,
                state: M5CertificationProofState::Missing,
                note: "no safe-preview proof".to_owned(),
            },
            current_dim(M5CertificationProofDimension::RawRenderedCopyExport, "ok"),
            current_dim(
                M5CertificationProofDimension::ActiveContentContainment,
                "ok",
            ),
            current_dim(M5CertificationProofDimension::SilentRewriteGuard, "ok"),
        ],
        consumer_surfaces: vec![M5ContentIntegrityConsumerSurface::AiEvidenceViewer],
    });
    assert_eq!(
        row.certified_qualification,
        M5ContentIntegrityQualificationClass::Experimental
    );
    assert!(row.was_narrowed());
    assert_eq!(row.narrowing_reasons.len(), 1);
    assert_eq!(
        row.narrowing_reasons[0].cause,
        M5CertificationNarrowingCause::ProofMissing
    );
}

#[test]
fn stale_proof_narrows_one_step() {
    let row = project_m5_content_integrity_certification(M5CertificationFamilySeed {
        family: M5ContentIntegrityArtifactFamily::DocsBrowserPanel,
        claimed_qualification: M5ContentIntegrityQualificationClass::Stable,
        dimensions: vec![
            current_dim(M5CertificationProofDimension::SuspiciousContentCues, "ok"),
            current_dim(M5CertificationProofDimension::SafePreviewTrustClass, "ok"),
            M5CertificationDimensionInput {
                dimension: M5CertificationProofDimension::RawRenderedCopyExport,
                state: M5CertificationProofState::StalePass,
                note: "raw/rendered proof stale".to_owned(),
            },
            current_dim(
                M5CertificationProofDimension::ActiveContentContainment,
                "ok",
            ),
            current_dim(M5CertificationProofDimension::SilentRewriteGuard, "ok"),
        ],
        consumer_surfaces: vec![M5ContentIntegrityConsumerSurface::DocsBrowserPanel],
    });
    assert_eq!(
        row.certified_qualification,
        M5ContentIntegrityQualificationClass::Beta
    );
}

#[test]
fn failing_proof_holds_the_family() {
    let row = project_m5_content_integrity_certification(M5CertificationFamilySeed {
        family: M5ContentIntegrityArtifactFamily::MarketplaceInstallUpdate,
        claimed_qualification: M5ContentIntegrityQualificationClass::Stable,
        dimensions: vec![
            current_dim(M5CertificationProofDimension::SuspiciousContentCues, "ok"),
            current_dim(M5CertificationProofDimension::SafePreviewTrustClass, "ok"),
            current_dim(M5CertificationProofDimension::RawRenderedCopyExport, "ok"),
            current_dim(
                M5CertificationProofDimension::ActiveContentContainment,
                "ok",
            ),
            current_dim(M5CertificationProofDimension::SilentRewriteGuard, "ok"),
            M5CertificationDimensionInput {
                dimension: M5CertificationProofDimension::StrongDecisionDisplay,
                state: M5CertificationProofState::Failing,
                note: "strict-identity rendering regressed".to_owned(),
            },
        ],
        consumer_surfaces: vec![M5ContentIntegrityConsumerSurface::MarketplaceSurface],
    });
    assert_eq!(
        row.certified_qualification,
        M5ContentIntegrityQualificationClass::Held
    );
    assert!(row.was_narrowed());
}

#[test]
fn worst_dimension_wins_over_multiple_narrowings() {
    let row = project_m5_content_integrity_certification(M5CertificationFamilySeed {
        family: M5ContentIntegrityArtifactFamily::GeneratedArtifact,
        claimed_qualification: M5ContentIntegrityQualificationClass::Beta,
        dimensions: vec![
            M5CertificationDimensionInput {
                dimension: M5CertificationProofDimension::SuspiciousContentCues,
                state: M5CertificationProofState::StalePass,
                note: "stale".to_owned(),
            },
            M5CertificationDimensionInput {
                dimension: M5CertificationProofDimension::SafePreviewTrustClass,
                state: M5CertificationProofState::Failing,
                note: "failing".to_owned(),
            },
            current_dim(M5CertificationProofDimension::RawRenderedCopyExport, "ok"),
            current_dim(
                M5CertificationProofDimension::ActiveContentContainment,
                "ok",
            ),
            current_dim(M5CertificationProofDimension::SilentRewriteGuard, "ok"),
        ],
        consumer_surfaces: vec![M5ContentIntegrityConsumerSurface::StructuredCompareView],
    });
    // Failing (held) is weaker than stale (one step), so held wins.
    assert_eq!(
        row.certified_qualification,
        M5ContentIntegrityQualificationClass::Held
    );
    assert_eq!(row.narrowing_reasons.len(), 2);
}

#[test]
fn certified_qualification_drift_fails_validation() {
    let mut packet = packet();
    packet.family_rows[1].certified_qualification = M5ContentIntegrityQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::CertifiedQualificationDrift));
}

#[test]
fn missing_required_dimension_fails_validation() {
    let mut packet = packet();
    packet.family_rows[0]
        .dimension_proofs
        .retain(|proof| proof.dimension != M5CertificationProofDimension::SuspiciousContentCues);
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::RequiredDimensionMissing));
}

#[test]
fn strong_decision_family_requires_strict_identity_dimension() {
    let mut packet = packet();
    let row = packet
        .family_rows
        .iter_mut()
        .find(|row| row.family == M5ContentIntegrityArtifactFamily::MarketplaceInstallUpdate)
        .expect("marketplace row exists");
    row.dimension_proofs
        .retain(|proof| proof.dimension != M5CertificationProofDimension::StrongDecisionDisplay);
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::RequiredDimensionMissing));
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = packet();
    packet
        .family_rows
        .retain(|row| row.family != M5ContentIntegrityArtifactFamily::AiEvidenceViewer);
    // Re-derive the summary so only RequiredFamilyMissing is asserted.
    packet.summary = derive_summary(&packet.family_rows);
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::RequiredFamilyMissing));
}

#[test]
fn summary_count_mismatch_fails_validation() {
    let mut packet = packet();
    packet.summary.certified_stable += 1;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::SummaryCountMismatch));
}

#[test]
fn review_incomplete_fails_validation() {
    let mut packet = packet();
    packet.review.auto_narrow_on_missing_or_stale_proof = false;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails_validation() {
    let mut packet = packet();
    packet.consumer_projection.shiproom_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn missing_source_contracts_fails_validation() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails_validation() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ContentIntegrityCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn proof_refs_cover_every_dimension_lane() {
    let packet = packet();
    let mut refs: BTreeSet<&str> = BTreeSet::new();
    for row in &packet.family_rows {
        for proof in &row.dimension_proofs {
            if proof.applicability == M5CertificationDimensionApplicability::Required {
                refs.insert(proof.backing_packet_ref.as_str());
            }
        }
    }
    // Every dimension's backing lane is cited by at least one certified row.
    for dimension in M5CertificationProofDimension::ALL {
        let lane = M5CertificationProofLane::for_dimension(dimension);
        assert!(
            refs.contains(lane.packet_ref()),
            "no row cites the lane backing {}",
            dimension.as_str()
        );
    }
    // The matrix lane is referenced as a source contract (it supplies the claims).
    let contracts: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for lane in M5CertificationProofLane::ALL {
        assert!(
            contracts.contains(lane.packet_ref()),
            "source contracts omit upstream lane {}",
            lane.as_str()
        );
    }
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
    let checked = current_m5_content_integrity_certification_export()
        .expect("checked M5 certification export validates");
    assert_eq!(
        checked.packet_id,
        M5_CONTENT_INTEGRITY_CERTIFICATION_PACKET_ID
    );
    assert_eq!(
        checked,
        frozen_m5_content_integrity_certification_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the certification bin"
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_narrow() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5/m5_content_integrity_certification/notebook_safe_preview_proof_missing.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5/m5_content_integrity_certification/marketplace_strict_identity_stale.json"
        )),
    ] {
        let packet: M5ContentIntegrityCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
        assert!(
            packet.summary.narrowed_families >= 1,
            "narrowing fixture has no narrowed family"
        );
        assert!(packet.family_rows.iter().any(|row| row.was_narrowed()));
    }
}
