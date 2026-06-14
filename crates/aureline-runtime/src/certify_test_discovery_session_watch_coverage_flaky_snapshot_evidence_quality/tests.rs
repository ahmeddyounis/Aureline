use super::*;

const PACKET_ID: &str = "m5-test-evidence-certification:stable:0001";
const MINTED_AT: &str = "2026-06-14T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn cert(dimension: EvidenceDimension, proof_currency: ProofCurrency) -> DimensionCertification {
    let (proof_ref, proof_fingerprint_token) = if proof_currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", dimension.as_str())),
            Some(format!("fp:{}", dimension.as_str())),
        )
    };
    DimensionCertification {
        dimension,
        proof_currency,
        proof_ref,
        proof_fingerprint_token,
        summary: format!(
            "{} certified with {} proof",
            dimension.as_str(),
            proof_currency.as_str()
        ),
    }
}

fn core(proof_currency: ProofCurrency) -> Vec<DimensionCertification> {
    EvidenceDimension::REQUIRED_CORE
        .iter()
        .map(|dimension| cert(*dimension, proof_currency))
        .collect()
}

fn subject(
    subject_id: &str,
    node_kind: DurableTestNodeKind,
    identity_class: TestItemIdentityClass,
) -> CertifiedSubject {
    CertifiedSubject {
        subject_id: subject_id.to_owned(),
        node_kind,
        subject_fingerprint_token: format!("fp:{subject_id}"),
        identity_class,
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    row_kind: CertifiedRowKind,
    label: &str,
    node_kind: DurableTestNodeKind,
    identity_class: TestItemIdentityClass,
    imported_row: bool,
    certifications: Vec<DimensionCertification>,
    claimed: CertificationGrade,
) -> CertifiedTestRow {
    CertifiedTestRow {
        row_id: row_id.to_owned(),
        row_kind,
        subject: subject(&format!("subject:{row_id}"), node_kind, identity_class),
        label_summary: label.to_owned(),
        imported_row,
        certifications,
        template_distinct_from_invocation: true,
        partial_discovery_visible: true,
        quarantine_visible_and_exportable: true,
        proposals_use_preview_apply: true,
        claimed_grade: claimed,
        effective_grade: claimed,
        narrow_trigger: None,
        narrowed_label: None,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[TEST_EVIDENCE_CERTIFICATION_DOC_REF]),
    }
}

fn narrowed_coverage_row() -> CertifiedTestRow {
    let mut certifications = core(ProofCurrency::VerifiedCurrent);
    certifications.push(cert(
        EvidenceDimension::CoverageEvidence,
        ProofCurrency::StaleExpired,
    ));
    let mut narrowed = row(
        "test-cert:framework-pack:stale-coverage:0001",
        CertifiedRowKind::FrameworkPackRow,
        "Framework-pack row whose coverage evidence aged outside its freshness window",
        DurableTestNodeKind::ParameterizedTemplate,
        TestItemIdentityClass::Stable,
        false,
        certifications,
        CertificationGrade::Certified,
    );
    narrowed.effective_grade = CertificationGrade::Uncertified;
    narrowed.narrow_trigger = Some(CertificationNarrowTrigger::StaleDimensionProof);
    narrowed.narrowed_label = Some(
        "Coverage evidence aged outside its freshness window; held uncertified until a fresh coverage run re-backs the claim"
            .to_owned(),
    );
    narrowed
}

fn rows() -> Vec<CertifiedTestRow> {
    vec![
        {
            let mut certifications = core(ProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                EvidenceDimension::CoverageEvidence,
                ProofCurrency::VerifiedCurrent,
            ));
            row(
                "test-cert:framework-pack:0001",
                CertifiedRowKind::FrameworkPackRow,
                "Framework-pack row with current discovery, session, watch, selector, and coverage proof",
                DurableTestNodeKind::ParameterizedTemplate,
                TestItemIdentityClass::Stable,
                false,
                certifications,
                CertificationGrade::Certified,
            )
        },
        {
            let mut certifications = core(ProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                EvidenceDimension::SnapshotEvidence,
                ProofCurrency::VerifiedCurrent,
            ));
            row(
                "test-cert:notebook:0001",
                CertifiedRowKind::NotebookRow,
                "Notebook-backed row with current core proof and current snapshot evidence",
                DurableTestNodeKind::NotebookLinkedTest,
                TestItemIdentityClass::Stable,
                false,
                certifications,
                CertificationGrade::Certified,
            )
        },
        row(
            "test-cert:ai-test-generation:0001",
            CertifiedRowKind::AiTestGenerationRow,
            "AI test-generation row whose proposals preview a diff and gate behind explicit apply",
            DurableTestNodeKind::ConcreteCase,
            TestItemIdentityClass::Stable,
            false,
            core(ProofCurrency::VerifiedCurrent),
            CertificationGrade::Certified,
        ),
        {
            let mut certifications = core(ProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                EvidenceDimension::FlakyEvidence,
                ProofCurrency::VerifiedCurrent,
            ));
            row(
                "test-cert:review-panel:0001",
                CertifiedRowKind::ReviewPanelRow,
                "Review-panel row with current core proof and current flaky/quarantine evidence",
                DurableTestNodeKind::ConcreteInvocation,
                TestItemIdentityClass::Stable,
                false,
                certifications,
                CertificationGrade::ReleaseCertified,
            )
        },
        row(
            "test-cert:ci-import:0001",
            CertifiedRowKind::CiImportRow,
            "Imported-CI row held read-only with current imported proof that never reads as a local rerun",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::ImportedReadOnly,
            true,
            core(ProofCurrency::ImportedCurrent),
            CertificationGrade::ProvisionallyCertified,
        ),
        narrowed_coverage_row(),
    ]
}

fn guardrails() -> TestEvidenceCertificationGuardrails {
    TestEvidenceCertificationGuardrails {
        parameterized_templates_distinct_from_invocations: true,
        imported_ci_never_reads_as_local_rerun: true,
        quarantines_and_stale_coverage_never_hidden: true,
        proposals_use_preview_diff_apply: true,
        rows_auto_narrow_without_current_proof: true,
    }
}

fn consumer_projection() -> TestEvidenceCertificationConsumerProjection {
    TestEvidenceCertificationConsumerProjection {
        product_ingests_certification: true,
        docs_help_ingests_certification: true,
        review_ingests_certification: true,
        support_ingests_certification: true,
        release_control_ingests_certification: true,
        narrowed_rows_labeled_below_claim: true,
    }
}

fn evidence_freshness() -> TestEvidenceCertificationFreshness {
    TestEvidenceCertificationFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        TEST_EVIDENCE_CERTIFICATION_SCHEMA_REF,
        TEST_EVIDENCE_CERTIFICATION_DOC_REF,
        TEST_EVIDENCE_CERTIFICATION_ARTIFACT_REF,
        "schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json",
        "schemas/testing/durable-test-items-and-partial-discovery.schema.json",
        "schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json",
        "schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json",
        "schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json",
        "schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json",
        "schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json",
    ])
}

fn packet() -> TestEvidenceCertificationPacket {
    TestEvidenceCertificationPacket::new(TestEvidenceCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Test-Evidence Certification".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_row_kind_is_present() {
    let kinds = packet().represented_row_kinds();
    for kind in CertifiedRowKind::ALL {
        assert!(kinds.contains(&kind), "missing row kind: {}", kind.as_str());
    }
}

#[test]
fn every_dimension_is_certified() {
    let dimensions = packet().represented_dimensions();
    for dimension in EvidenceDimension::ALL {
        assert!(
            dimensions.contains(&dimension),
            "missing dimension: {}",
            dimension.as_str()
        );
    }
}

#[test]
fn missing_row_kind_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.row_kind != CertifiedRowKind::CiImportRow);
    let violations = packet.validate();
    assert!(violations.contains(&TestEvidenceCertificationViolation::RequiredRowKindMissing));
    assert!(violations.contains(&TestEvidenceCertificationViolation::ImportedRowCaseMissing));
}

#[test]
fn missing_dimension_fails_validation() {
    let mut packet = packet();
    // Drop the only snapshot-evidence certification so the dimension is unrepresented.
    for row in &mut packet.rows {
        row.certifications
            .retain(|c| c.dimension != EvidenceDimension::SnapshotEvidence);
    }
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::DimensionCoverageMissing));
}

#[test]
fn auto_narrow_case_is_present() {
    assert_eq!(packet().narrowed_row_count(), 1);
}

#[test]
fn missing_narrowed_case_fails_validation() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:stale-coverage:0001")
        .expect("narrowed row");
    // Re-back the stale coverage proof so no row demonstrates auto-narrowing.
    for c in &mut narrowed.certifications {
        if c.dimension == EvidenceDimension::CoverageEvidence {
            c.proof_currency = ProofCurrency::VerifiedCurrent;
        }
    }
    narrowed.effective_grade = narrowed.claimed_grade;
    narrowed.narrow_trigger = None;
    narrowed.narrowed_label = None;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::NarrowedRowCaseMissing));
}

#[test]
fn claimed_row_losing_current_proof_must_narrow() {
    let mut packet = packet();
    // A claimed framework row whose session proof goes stale but keeps its full
    // claim must be rejected: the row auto-narrows before promotion.
    let framework = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:0001")
        .expect("framework row");
    for c in &mut framework.certifications {
        if c.dimension == EvidenceDimension::SessionTruth {
            c.proof_currency = ProofCurrency::StaleExpired;
        }
    }
    assert!(framework.needs_narrow());
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn missing_core_dimension_forces_narrow() {
    let mut packet = packet();
    let framework = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:0001")
        .expect("framework row");
    framework
        .certifications
        .retain(|c| c.dimension != EvidenceDimension::WatchTruth);
    assert!(framework.needs_narrow());
    let violations = packet.validate();
    assert!(
        violations.contains(&TestEvidenceCertificationViolation::RowNotNarrowedOnUncurrentProof)
    );
}

#[test]
fn imported_proof_on_local_row_narrows() {
    let mut packet = packet();
    let framework = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:0001")
        .expect("framework row");
    // Imported proof can never back a local row's claim.
    for c in &mut framework.certifications {
        if c.dimension == EvidenceDimension::DiscoveryTruth {
            c.proof_currency = ProofCurrency::ImportedCurrent;
        }
    }
    assert!(framework.needs_narrow());
}

#[test]
fn imported_row_marker_mismatch_fails() {
    let mut packet = packet();
    let ci = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:ci-import:0001")
        .expect("ci row");
    // Drop the imported subject identity while keeping the imported_row flag.
    ci.subject.identity_class = TestItemIdentityClass::Stable;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::ImportedReadsAsLocal));
}

#[test]
fn generic_narrowed_label_fails() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "test-cert:framework-pack:stale-coverage:0001")
        .expect("narrowed row");
    narrowed.narrowed_label = Some("uncertified".to_owned());
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn fingerprint_substituting_identity_fails() {
    let mut packet = packet();
    packet.rows[0].subject.subject_fingerprint_token = packet.rows[0].subject.subject_id.clone();
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn template_collapsed_with_invocation_fails() {
    let mut packet = packet();
    packet.rows[0].template_distinct_from_invocation = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn hidden_partial_discovery_fails() {
    let mut packet = packet();
    packet.rows[1].partial_discovery_visible = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::PartialDiscoveryHidden));
}

#[test]
fn hidden_quarantine_fails() {
    let mut packet = packet();
    packet.rows[3].quarantine_visible_and_exportable = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::QuarantineHidden));
}

#[test]
fn proposal_bypassing_preview_fails() {
    let mut packet = packet();
    packet.rows[2].proposals_use_preview_apply = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::ProposalBypassesPreview));
}

#[test]
fn dimension_proof_without_fingerprint_fails() {
    let mut packet = packet();
    // A present proof ref with a fingerprint equal to the ref is not reopenable.
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_fingerprint_token = cert.proof_ref.clone();
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn missing_proof_with_ref_fails() {
    let mut packet = packet();
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_currency = ProofCurrency::MissingProof;
    // A missing proof must carry no ref; keeping one is malformed.
    assert!(!cert.is_well_formed());
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != TEST_EVIDENCE_CERTIFICATION_DOC_REF);
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.rows_auto_narrow_without_current_proof = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .release_control_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_evidence_freshness_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&TestEvidenceCertificationViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: TestEvidenceCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Test Discovery"));
    assert!(summary.contains("framework_pack_row"));
    assert!(summary.contains("Narrowed:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_test_evidence_certification_export()
        .expect("checked test evidence certification export validates");
    assert_eq!(checked, packet());
}
