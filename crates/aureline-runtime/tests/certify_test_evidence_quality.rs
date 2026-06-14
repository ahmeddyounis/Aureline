use aureline_runtime::certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality::{
    current_test_evidence_certification_export, CertificationGrade, CertificationNarrowTrigger,
    CertifiedRowKind, EvidenceDimension, ProofCurrency, TestEvidenceCertificationPacket,
    TestEvidenceCertificationViolation,
};

fn fixture(name: &str) -> TestEvidenceCertificationPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_test_evidence_certification_export()
        .expect("checked-in test evidence certification export should validate");
    assert!(packet.validate().is_empty());
    for kind in CertifiedRowKind::ALL {
        assert!(
            packet.represented_row_kinds().contains(&kind),
            "missing row kind {}",
            kind.as_str()
        );
    }
    for dimension in EvidenceDimension::ALL {
        assert!(
            packet.represented_dimensions().contains(&dimension),
            "missing dimension {}",
            dimension.as_str()
        );
    }
}

#[test]
fn narrow_drill_fixture_auto_narrows() {
    let packet = fixture("framework_row_narrows_on_stale_coverage_proof.json");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.narrowed_row_count(), 1);

    let narrowed = packet
        .rows
        .iter()
        .find(|row| row.needs_narrow())
        .expect("narrowed row");
    assert_eq!(narrowed.row_kind, CertifiedRowKind::FrameworkPackRow);
    assert_eq!(
        narrowed
            .certification(EvidenceDimension::CoverageEvidence)
            .map(|c| c.proof_currency),
        Some(ProofCurrency::StaleExpired)
    );
    assert_eq!(narrowed.effective_grade, CertificationGrade::Uncertified);
    assert!(
        narrowed.effective_grade.rank() < narrowed.claimed_grade.rank(),
        "narrowed row must rank strictly below its claim"
    );
    assert_eq!(
        narrowed.narrow_trigger,
        Some(CertificationNarrowTrigger::StaleDimensionProof)
    );
}

#[test]
fn imported_ci_row_never_reads_as_local() {
    let packet = fixture("framework_row_narrows_on_stale_coverage_proof.json");
    let imported = packet
        .rows
        .iter()
        .find(|row| row.row_kind == CertifiedRowKind::CiImportRow)
        .expect("ci import row");
    assert!(imported.imported_row);
    assert!(imported.subject.is_imported());
    assert!(imported.imported_posture_consistent());
    // Imported proof backs the imported row's claim but never a local one.
    for cert in &imported.certifications {
        assert_eq!(cert.proof_currency, ProofCurrency::ImportedCurrent);
        assert!(cert.backs_claim(true));
        assert!(!cert.backs_claim(false));
    }
}

#[test]
fn claimed_row_losing_session_proof_must_narrow() {
    let mut packet = fixture("framework_row_narrows_on_stale_coverage_proof.json");
    // A claimed framework row that loses current session proof but keeps its full
    // claim must be rejected: the row auto-narrows before promotion.
    let framework = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:0001")
        .expect("framework row");
    for cert in &mut framework.certifications {
        if cert.dimension == EvidenceDimension::SessionTruth {
            cert.proof_currency = ProofCurrency::StaleExpired;
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&TestEvidenceCertificationViolation::RowNotNarrowedOnUncurrentProof)
    );
}
