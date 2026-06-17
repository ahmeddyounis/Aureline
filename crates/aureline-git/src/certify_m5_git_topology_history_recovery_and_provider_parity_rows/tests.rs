use super::*;

const CANONICAL_PACKET_ID: &str = "m5-git-certification:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json"
));

const STALE_TOPOLOGY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/certification-corpus/stale_topology_retest_pending.json"
));

const FAILED_PARITY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/certification-corpus/failed_provider_parity_unsupported.json"
));

const PARTIAL_HISTORY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/certification-corpus/partial_history_recovery_limited.json"
));

fn baseline() -> M5GitCertificationPacket {
    serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes")
}

#[test]
fn checked_support_export_validates() {
    let packet = current_m5_git_certification_export()
        .expect("checked M5 git certification export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn canonical_packet_validates_clean() {
    assert!(
        baseline().validate().is_empty(),
        "{:?}",
        baseline().validate()
    );
}

#[test]
fn canonical_packet_certifies_every_claimed_row() {
    let packet = baseline();
    for required in M5GitClaimRow::ALL {
        let row = packet
            .rows
            .iter()
            .find(|row| row.claim_row == required)
            .unwrap_or_else(|| panic!("missing claim row {}", required.as_str()));
        assert_eq!(
            row.verdict,
            CertificationVerdict::Certified,
            "row {} should be certified in the canonical packet",
            required.as_str()
        );
    }
    assert_eq!(packet.certified_rows().len(), M5GitClaimRow::ALL.len());
    assert!(packet.narrowed_rows().is_empty());
}

#[test]
fn every_row_carries_all_four_dimensions() {
    for row in &baseline().rows {
        for dimension in CertificationDimension::ALL {
            assert!(
                row.dimensions
                    .iter()
                    .any(|entry| entry.dimension == dimension),
                "row {} missing dimension {}",
                row.claim_row.as_str(),
                dimension.as_str()
            );
        }
    }
}

#[test]
fn history_dimension_applicability_tracks_row_nature() {
    for row in &baseline().rows {
        let entry = row
            .dimensions
            .iter()
            .find(|entry| entry.dimension == CertificationDimension::HistorySurgeryPreviewRecovery)
            .expect("history dimension present");
        assert_eq!(
            entry.applicable,
            row.claim_row.rewrites_history(),
            "row {} history applicability mismatch",
            row.claim_row.as_str()
        );
    }
}

#[test]
fn checked_degraded_fixtures_validate() {
    for raw in [
        STALE_TOPOLOGY_FIXTURE,
        FAILED_PARITY_FIXTURE,
        PARTIAL_HISTORY_FIXTURE,
    ] {
        let packet: M5GitCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn stale_topology_fixture_narrows_to_retest_pending() {
    let packet: M5GitCertificationPacket =
        serde_json::from_str(STALE_TOPOLOGY_FIXTURE).expect("stale fixture parses");
    let row = packet
        .rows
        .iter()
        .find(|row| row.claim_row == M5GitClaimRow::RepositoryTopologyHonesty)
        .expect("topology row present");
    assert_eq!(row.verdict, CertificationVerdict::RetestPending);
    assert!(row.narrowing_reason.is_some());
}

#[test]
fn failed_parity_fixture_narrows_to_unsupported() {
    let packet: M5GitCertificationPacket =
        serde_json::from_str(FAILED_PARITY_FIXTURE).expect("failed parity fixture parses");
    let row = packet
        .rows
        .iter()
        .find(|row| row.claim_row == M5GitClaimRow::PublishAndProviderParity)
        .expect("publish row present");
    assert_eq!(row.verdict, CertificationVerdict::Unsupported);
}

#[test]
fn partial_history_fixture_narrows_to_limited() {
    let packet: M5GitCertificationPacket =
        serde_json::from_str(PARTIAL_HISTORY_FIXTURE).expect("partial history fixture parses");
    let row = packet
        .rows
        .iter()
        .find(|row| row.claim_row == M5GitClaimRow::HistorySurgeryPreviewAndRecovery)
        .expect("history row present");
    assert_eq!(row.verdict, CertificationVerdict::Limited);
}

#[test]
fn fail_closed_derivation_matches_declared_automation() {
    // Failure or missing evidence is the most severe.
    let failed = DimensionQualification {
        dimension: CertificationDimension::LocalProviderParity,
        applicable: true,
        freshness: EvidenceFreshness::Current,
        proof_state: DimensionProofState::Failed,
        evidence_refs: vec!["x".to_owned()],
        summary: "x".to_owned(),
    };
    assert_eq!(
        failed.verdict_contribution(),
        Some(CertificationVerdict::Unsupported)
    );

    let missing = DimensionQualification {
        freshness: EvidenceFreshness::Missing,
        proof_state: DimensionProofState::Proven,
        ..failed.clone()
    };
    assert_eq!(
        missing.verdict_contribution(),
        Some(CertificationVerdict::Unsupported)
    );

    let stale = DimensionQualification {
        freshness: EvidenceFreshness::Stale,
        proof_state: DimensionProofState::Proven,
        ..failed.clone()
    };
    assert_eq!(
        stale.verdict_contribution(),
        Some(CertificationVerdict::RetestPending)
    );

    let not_run = DimensionQualification {
        freshness: EvidenceFreshness::Current,
        proof_state: DimensionProofState::NotRun,
        ..failed.clone()
    };
    assert_eq!(
        not_run.verdict_contribution(),
        Some(CertificationVerdict::RetestPending)
    );

    let narrowed = DimensionQualification {
        freshness: EvidenceFreshness::Current,
        proof_state: DimensionProofState::Narrowed,
        ..failed.clone()
    };
    assert_eq!(
        narrowed.verdict_contribution(),
        Some(CertificationVerdict::Limited)
    );

    let proven = DimensionQualification {
        freshness: EvidenceFreshness::Current,
        proof_state: DimensionProofState::Proven,
        ..failed.clone()
    };
    assert_eq!(
        proven.verdict_contribution(),
        Some(CertificationVerdict::Certified)
    );

    let not_applicable = DimensionQualification {
        applicable: false,
        ..failed
    };
    assert_eq!(not_applicable.verdict_contribution(), None);
}

#[test]
fn worst_dimension_wins_the_verdict() {
    let mut packet = baseline();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.claim_row == M5GitClaimRow::PublishAndProviderParity)
        .expect("publish row present");
    // One dimension fails while others stay proven; the worst wins.
    row.dimensions
        .iter_mut()
        .find(|entry| entry.dimension == CertificationDimension::LocalProviderParity)
        .expect("parity dimension present")
        .proof_state = DimensionProofState::Failed;
    assert_eq!(row.derive_verdict(), CertificationVerdict::Unsupported);
}

#[test]
fn declared_verdict_must_match_evidence() {
    let mut packet = baseline();
    // Make a dimension stale but leave the verdict at certified: must fail.
    packet.rows[0]
        .dimensions
        .iter_mut()
        .find(|entry| entry.dimension == CertificationDimension::TopologyHonesty)
        .expect("topology dimension present")
        .freshness = EvidenceFreshness::Stale;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::VerdictDoesNotMatchEvidence));
}

#[test]
fn missing_claim_row_fails() {
    let mut packet = baseline();
    packet
        .rows
        .retain(|row| row.claim_row != M5GitClaimRow::PublishAndProviderParity);
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::RequiredClaimRowMissing));
}

#[test]
fn duplicate_claim_row_fails() {
    let mut packet = baseline();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::DuplicateClaimRow));
}

#[test]
fn row_missing_dimension_fails() {
    let mut packet = baseline();
    packet.rows[0]
        .dimensions
        .retain(|entry| entry.dimension != CertificationDimension::LocalProviderParity);
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::RowMissingDimensions));
}

#[test]
fn proven_dimension_without_evidence_fails() {
    let mut packet = baseline();
    packet.rows[0]
        .dimensions
        .iter_mut()
        .find(|entry| entry.dimension == CertificationDimension::TopologyHonesty)
        .expect("topology dimension present")
        .evidence_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::ProvenDimensionMissingEvidence));
}

#[test]
fn narrowed_row_without_reason_fails() {
    let mut packet = baseline();
    let row = &mut packet.rows[0];
    row.dimensions
        .iter_mut()
        .find(|entry| entry.dimension == CertificationDimension::TopologyHonesty)
        .expect("topology dimension present")
        .proof_state = DimensionProofState::Narrowed;
    row.verdict = row.derive_verdict();
    row.narrowing_reason = None;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::NarrowedRowMissingReason));
}

#[test]
fn certified_row_with_reason_fails() {
    let mut packet = baseline();
    packet.rows[0].narrowing_reason = Some("spurious".to_owned());
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::CertifiedRowHasNarrowingReason));
}

#[test]
fn history_applicability_mismatch_fails() {
    let mut packet = baseline();
    // Force the history dimension applicable on a read-only row.
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.claim_row == M5GitClaimRow::RepositoryTopologyHonesty)
        .expect("topology row present");
    let entry = row
        .dimensions
        .iter_mut()
        .find(|entry| entry.dimension == CertificationDimension::HistorySurgeryPreviewRecovery)
        .expect("history dimension present");
    entry.applicable = true;
    entry.proof_state = DimensionProofState::Proven;
    entry.freshness = EvidenceFreshness::Current;
    entry.evidence_refs = vec!["x".to_owned()];
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::HistoryDimensionApplicabilityMismatch));
}

#[test]
fn missing_source_contract_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::MissingSourceContracts));
}

#[test]
fn parity_audit_incomplete_fails() {
    let mut packet = baseline();
    packet.parity_audit.no_surface_claims_wider_than_row = false;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::ParityAuditIncomplete));
}

#[test]
fn downgrade_automation_inconsistent_fails() {
    let mut packet = baseline();
    packet.downgrade_automation.stale_or_unrun_narrows_to = CertificationVerdict::Limited;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::DowngradeAutomationInconsistent));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = baseline();
    packet
        .governance_review
        .release_surfaces_stop_overclaiming_on_slip = false;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::GovernanceReviewIncomplete));
}

#[test]
fn freshness_posture_incomplete_fails() {
    let mut packet = baseline();
    packet.freshness_posture.evidence_window_open = false;
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::FreshnessPostureIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.certification_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitCertificationViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = baseline().render_markdown_summary();
    for row in M5GitClaimRow::ALL {
        assert!(
            summary.contains(row.as_str()),
            "summary missing claim row {}",
            row.as_str()
        );
    }
}
