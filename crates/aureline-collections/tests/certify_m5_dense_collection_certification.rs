use aureline_collections::{
    current_m5_collection_certification_export, CertificationProofDimension,
    CertificationRegressionClass, CertificationVerdict, CollectionMatrixQualificationClass,
    DenseCollectionSurface, M5CollectionCertificationPacket, M5CollectionCertificationViolation,
    ProofStatus,
};

fn fixture(name: &str) -> M5CollectionCertificationPacket {
    let path = format!(
        "{}/../../fixtures/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_m5_collection_certification_export()
        .expect("checked-in certification export should validate");
    assert!(packet.validate().is_empty());
    for surface in DenseCollectionSurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn release_gate_drill_fixture_blocks_and_narrows() {
    let packet = fixture("certification_blocks_regression_and_auto_narrows_missing_proof.json");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.certified_row_count(), 7);
    assert_eq!(packet.blocked_row_count(), 1);
    assert_eq!(packet.narrowed_row_count(), 1);

    let blocked = packet
        .rows
        .iter()
        .find(|row| row.surface == DenseCollectionSurface::ProviderAdminTable)
        .expect("provider-admin row");
    assert_eq!(blocked.verdict, CertificationVerdict::Blocked);
    assert_eq!(
        blocked.regression,
        Some(CertificationRegressionClass::ProviderPolicyNarrowingErased)
    );
    assert!(
        blocked.certified_qualification.rank() < blocked.claimed_qualification.rank(),
        "blocked row must rank strictly below its claim"
    );

    let narrowed = packet
        .rows
        .iter()
        .find(|row| row.surface == DenseCollectionSurface::SupportExportProjection)
        .expect("support-export row");
    assert_eq!(narrowed.verdict, CertificationVerdict::AutoNarrowed);
    assert_eq!(
        narrowed.certified_qualification,
        CollectionMatrixQualificationClass::Held
    );
    assert_eq!(
        narrowed
            .proof_for(CertificationProofDimension::BatchAction)
            .map(|proof| proof.status),
        Some(ProofStatus::Missing)
    );
    assert!(narrowed.has_uncovered_dimension());
}

#[test]
fn claimed_row_losing_proof_must_narrow() {
    let mut packet = fixture("certification_blocks_regression_and_auto_narrows_missing_proof.json");
    // A claimed pipeline row that loses its result-count proof but keeps its full
    // claim must be rejected: the surface auto-narrows before promotion.
    let pipeline_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::PipelineRunList)
        .expect("pipeline row");
    let proof = pipeline_row
        .proofs
        .iter_mut()
        .find(|proof| proof.dimension == CertificationProofDimension::ResultCount)
        .expect("result-count proof");
    proof.status = ProofStatus::Missing;
    proof.backing_record_kind = None;
    proof.proof_ref = None;
    let violations = packet.validate();
    assert!(violations.contains(&M5CollectionCertificationViolation::UncoveredClaimNotNarrowed));
}
