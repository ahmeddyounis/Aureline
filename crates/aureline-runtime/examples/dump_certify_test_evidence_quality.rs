//! Conformance dump for the M5 test-evidence certification packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality::*;
use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::testing_identity::TestItemIdentityClass;

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

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
