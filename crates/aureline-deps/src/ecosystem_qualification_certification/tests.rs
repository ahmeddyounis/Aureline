use super::*;

fn packet() -> EcosystemQualificationCertification {
    current_ecosystem_qualification_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        ECOSYSTEM_QUALIFICATION_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        ECOSYSTEM_QUALIFICATION_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn matrix_is_complete_and_unique() {
    let packet = packet();
    assert_eq!(
        packet.rows.len(),
        packet.claimed_ecosystems.len() * packet.lanes.len()
    );
    for &ecosystem in &packet.claimed_ecosystems {
        for &lane in &packet.lanes {
            assert!(
                packet.row(ecosystem, lane).is_some(),
                "missing cell {}/{}",
                ecosystem.as_str(),
                lane.as_str()
            );
        }
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_maturity,
            row.effective_maturity(),
            "row {} publishes beyond the gate",
            row.row_id
        );
        assert_eq!(
            row.narrowing_action,
            row.required_narrowing(),
            "row {} narrowing diverges from the gate",
            row.row_id
        );
    }
}

#[test]
fn every_row_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            !row.qualification_packet_ref.trim().is_empty(),
            "row {} has no qualification packet",
            row.row_id
        );
        assert!(
            !row.corpus_ref.trim().is_empty(),
            "row {} has no proof corpus",
            row.row_id
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_rows_gate_consistent,
        packet.all_rows_gate_consistent()
    );
    assert_eq!(
        projection.promotable_count,
        packet.promotable_rows().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
}

#[test]
fn published_maturities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<MaturityClass> =
        packet.rows.iter().map(|r| r.published_maturity).collect();
    for maturity in MaturityClass::ALL {
        assert!(
            present.contains(&maturity),
            "no row publishes maturity {}",
            maturity.as_str()
        );
    }
}

#[test]
fn narrowing_actions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingAction> =
        packet.rows.iter().map(|r| r.narrowing_action).collect();
    for action in NarrowingAction::ALL {
        assert!(
            present.contains(&action),
            "no row exercises narrowing {}",
            action.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationFreshness> = packet
        .rows
        .iter()
        .map(|r| r.certification_freshness)
        .collect();
    for freshness in CertificationFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no row exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn blocking_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<BlockingReason> = packet
        .rows
        .iter()
        .flat_map(|r| r.blocking_reasons.iter().copied())
        .collect();
    for reason in BlockingReason::ALL {
        assert!(
            present.contains(&reason),
            "no row exercises blocking reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn promotable_rows_are_clean() {
    let packet = packet();
    assert!(
        packet.promotable_rows().count() > 0,
        "fixture needs a certified row"
    );
    for row in packet.promotable_rows() {
        assert!(row.certification_freshness.is_current());
        assert!(row.blocking_reasons.is_empty());
        assert_eq!(row.published_maturity, MaturityClass::Certified);
        assert_eq!(row.narrowing_action, NarrowingAction::None);
    }
}

#[test]
fn gate_narrows_stale_rows() {
    // A current certified row that goes stale must narrow to provisional.
    assert_eq!(
        CertificationFreshness::Stale.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        CertificationFreshness::Expired.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        CertificationFreshness::Current.maturity_ceiling(),
        MaturityClass::Certified
    );
}

#[test]
fn blocking_reason_ceilings_hold() {
    assert_eq!(
        BlockingReason::ScannerUnderqualified.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        BlockingReason::MissingCorpus.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        BlockingReason::MissingPackageLockfileEvidence.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        BlockingReason::MirrorBlocked.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        BlockingReason::Stale.maturity_ceiling(),
        MaturityClass::Provisional
    );
}

#[test]
fn missing_evidence_blocks_promotion() {
    assert!(BlockingReason::MissingCorpus.is_missing_evidence());
    assert!(BlockingReason::MissingPackageLockfileEvidence.is_missing_evidence());
    assert!(!BlockingReason::Stale.is_missing_evidence());
}

#[test]
fn validate_flags_overstated_published_maturity() {
    let mut packet = packet();
    // Force a stale row to claim it still publishes certified.
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.certification_freshness != CertificationFreshness::Current)
    {
        row.published_maturity = MaturityClass::Certified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            EcosystemQualificationCertificationViolation::OverstatedPublishedMaturity { .. }
        )));
    }
}

#[test]
fn validate_flags_narrowing_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.narrowing_action != NarrowingAction::WithholdFromPublication)
    {
        row.narrowing_action = NarrowingAction::WithholdFromPublication;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            EcosystemQualificationCertificationViolation::NarrowingActionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_promoted_row_not_clean() {
    let mut packet = packet();
    // Add a blocking reason to a promotable row without changing its published
    // maturity: it is now internally inconsistent and not clean.
    if let Some(row) = packet.rows.iter_mut().find(|r| r.is_promotable()) {
        row.blocking_reasons.push(BlockingReason::Stale);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            EcosystemQualificationCertificationViolation::OverstatedPublishedMaturity { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_matrix_cell() {
    let mut packet = packet();
    let removed = packet.rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EcosystemQualificationCertificationViolation::MissingMatrixCell { .. }
    )));
}

#[test]
fn validate_flags_unclaimed_ecosystem_row() {
    let mut packet = packet();
    // Drop an ecosystem from the claimed set while leaving its rows in place.
    packet
        .claimed_ecosystems
        .retain(|e| *e != ClaimedEcosystem::PythonPip);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EcosystemQualificationCertificationViolation::UnclaimedEcosystemRow { .. }
    )));
    // And the dropped ecosystem's closed-vocabulary mismatch is also flagged.
    assert!(violations.iter().any(|v| matches!(
        v,
        EcosystemQualificationCertificationViolation::ClosedVocabularyMismatch {
            field: "claimed_ecosystems"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&EcosystemQualificationCertificationViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ClaimedEcosystem::Cargo.as_str(), "cargo");
    assert_eq!(ClaimedEcosystem::NodePnpm.as_str(), "node_pnpm");
    assert_eq!(ClaimedEcosystem::PythonPip.as_str(), "python_pip");
    assert_eq!(
        QualificationLane::DependencyIntelligence.as_str(),
        "dependency_intelligence"
    );
    assert_eq!(QualificationLane::PackageReview.as_str(), "package_review");
    assert_eq!(QualificationLane::CodeQuality.as_str(), "code_quality");
    assert_eq!(QualificationLane::ScannerImport.as_str(), "scanner_import");
    assert_eq!(MaturityClass::Certified.as_str(), "certified");
    assert_eq!(
        NarrowingAction::WithholdFromPublication.as_str(),
        "withhold_from_publication"
    );
}

#[test]
fn maturity_rank_orders_low_to_high() {
    assert!(MaturityClass::Unsupported.rank() < MaturityClass::Underqualified.rank());
    assert!(MaturityClass::Underqualified.rank() < MaturityClass::Provisional.rank());
    assert!(MaturityClass::Provisional.rank() < MaturityClass::Certified.rank());
    assert_eq!(
        MaturityClass::Certified.min(MaturityClass::Provisional),
        MaturityClass::Provisional
    );
    assert_eq!(
        MaturityClass::Underqualified.min(MaturityClass::Certified),
        MaturityClass::Underqualified
    );
}
