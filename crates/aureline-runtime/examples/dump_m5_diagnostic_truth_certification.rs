//! Conformance dump for the M5 diagnostic-truth certification packet.
//!
//! Prints the canonical support export (default), the Markdown summary (`summary`
//! argument), or the waiver-and-downgrade log (`waiver` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate builder.

use aureline_runtime::certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth::*;
use aureline_runtime::diagnostics::{DiagnosticOriginClass, DiagnosticSourceKind};

const PACKET_ID: &str = "m5-diagnostic-truth-certification:stable:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn cert(
    dimension: DiagnosticEvidenceDimension,
    proof_currency: DiagnosticProofCurrency,
) -> DiagnosticDimensionCertification {
    let (proof_ref, proof_fingerprint_token) = if proof_currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", dimension.as_str())),
            Some(format!("fp:{}", dimension.as_str())),
        )
    };
    DiagnosticDimensionCertification {
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

fn core(proof_currency: DiagnosticProofCurrency) -> Vec<DiagnosticDimensionCertification> {
    DiagnosticEvidenceDimension::REQUIRED_CORE
        .iter()
        .map(|dimension| cert(*dimension, proof_currency))
        .collect()
}

fn subject(
    subject_id: &str,
    source_kind: DiagnosticSourceKind,
    origin_class: DiagnosticOriginClass,
) -> CertifiedDiagnosticSubject {
    CertifiedDiagnosticSubject {
        subject_id: subject_id.to_owned(),
        source_kind,
        origin_class,
        subject_fingerprint_token: format!("fp:{subject_id}"),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    row_kind: CertifiedDiagnosticRowKind,
    label: &str,
    source_kind: DiagnosticSourceKind,
    origin_class: DiagnosticOriginClass,
    imported_row: bool,
    certifications: Vec<DiagnosticDimensionCertification>,
    claimed: DiagnosticCertificationGrade,
) -> CertifiedDiagnosticRow {
    CertifiedDiagnosticRow {
        row_id: row_id.to_owned(),
        row_kind,
        subject: subject(&format!("subject:{row_id}"), source_kind, origin_class),
        label_summary: label.to_owned(),
        imported_row,
        certifications,
        source_kind_preserved: true,
        imported_live_class_preserved: true,
        collection_completeness_visible: true,
        remap_history_append_only: true,
        mutating_routes_use_quality_session: true,
        claimed_grade: claimed,
        effective_grade: claimed,
        narrow_trigger: None,
        narrowed_label: None,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[DIAGNOSTIC_TRUTH_CERT_DOC_REF]),
    }
}

fn narrowed_collection_row() -> CertifiedDiagnosticRow {
    let mut certifications = core(DiagnosticProofCurrency::VerifiedCurrent);
    for certification in &mut certifications {
        if certification.dimension == DiagnosticEvidenceDimension::CollectionSnapshot {
            certification.proof_currency = DiagnosticProofCurrency::StaleExpired;
        }
    }
    let mut narrowed = row(
        "diag-cert:framework:stale-collection:0001",
        CertifiedDiagnosticRowKind::FrameworkRow,
        "Framework row whose collection snapshot aged outside its freshness window",
        DiagnosticSourceKind::LanguageService,
        DiagnosticOriginClass::LiveLocalSession,
        false,
        certifications,
        DiagnosticCertificationGrade::Certified,
    );
    narrowed.effective_grade = DiagnosticCertificationGrade::Uncertified;
    narrowed.narrow_trigger = Some(DiagnosticCertificationNarrowTrigger::StaleDimensionProof);
    narrowed.narrowed_label = Some(
        "Collection snapshot aged outside its freshness window; held uncertified until a fresh enumeration re-backs the claim"
            .to_owned(),
    );
    narrowed
}

fn rows() -> Vec<CertifiedDiagnosticRow> {
    vec![
        {
            let mut certifications = core(DiagnosticProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                DiagnosticEvidenceDimension::QualitySession,
                DiagnosticProofCurrency::VerifiedCurrent,
            ));
            row(
                "diag-cert:notebook:0001",
                CertifiedDiagnosticRowKind::NotebookRow,
                "Notebook row with current record, source, collection, remap, and quality-session proof",
                DiagnosticSourceKind::RuntimeOrTest,
                DiagnosticOriginClass::LiveLocalSession,
                false,
                certifications,
                DiagnosticCertificationGrade::Certified,
            )
        },
        {
            let mut certifications = core(DiagnosticProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                DiagnosticEvidenceDimension::QualitySession,
                DiagnosticProofCurrency::VerifiedCurrent,
            ));
            row(
                "diag-cert:framework:0001",
                CertifiedDiagnosticRowKind::FrameworkRow,
                "Framework row with current core proof and a current lint-autofix quality session",
                DiagnosticSourceKind::LanguageService,
                DiagnosticOriginClass::LiveLocalSession,
                false,
                certifications,
                DiagnosticCertificationGrade::Certified,
            )
        },
        row(
            "diag-cert:request-data:0001",
            CertifiedDiagnosticRowKind::RequestDataRow,
            "Request/data-tooling row with current record, source, collection, and remap proof",
            DiagnosticSourceKind::BuildOrTask,
            DiagnosticOriginClass::LiveLocalSession,
            false,
            core(DiagnosticProofCurrency::VerifiedCurrent),
            DiagnosticCertificationGrade::Certified,
        ),
        row(
            "diag-cert:preview-runtime:0001",
            CertifiedDiagnosticRowKind::PreviewRuntimeRow,
            "Preview/runtime row with current record, source, collection, and remap proof",
            DiagnosticSourceKind::RuntimeOrTest,
            DiagnosticOriginClass::LiveLocalSession,
            false,
            core(DiagnosticProofCurrency::VerifiedCurrent),
            DiagnosticCertificationGrade::Certified,
        ),
        {
            let mut certifications = core(DiagnosticProofCurrency::VerifiedCurrent);
            certifications.push(cert(
                DiagnosticEvidenceDimension::QualitySession,
                DiagnosticProofCurrency::VerifiedCurrent,
            ));
            row(
                "diag-cert:package:0001",
                CertifiedDiagnosticRowKind::PackageRow,
                "Package row with current core proof and a current lockfile-mutation quality session",
                DiagnosticSourceKind::Policy,
                DiagnosticOriginClass::LiveLocalSession,
                false,
                certifications,
                DiagnosticCertificationGrade::ReleaseCertified,
            )
        },
        {
            let mut certifications = core(DiagnosticProofCurrency::ImportedCurrent);
            certifications.push(cert(
                DiagnosticEvidenceDimension::QualitySession,
                DiagnosticProofCurrency::ImportedCurrent,
            ));
            row(
                "diag-cert:imported-scanner:0001",
                CertifiedDiagnosticRowKind::ImportedScannerRow,
                "Imported-scanner row held read-only with current imported proof that never reads as a live local rerun",
                DiagnosticSourceKind::ScannerImport,
                DiagnosticOriginClass::ImportedSnapshot,
                true,
                certifications,
                DiagnosticCertificationGrade::ProvisionallyCertified,
            )
        },
        row(
            "diag-cert:review-support-cli:0001",
            CertifiedDiagnosticRowKind::ReviewSupportCliRow,
            "Review/support/CLI row that reopens the same record, source, collection, and remap evidence",
            DiagnosticSourceKind::EditorStructural,
            DiagnosticOriginClass::LiveLocalSession,
            false,
            core(DiagnosticProofCurrency::VerifiedCurrent),
            DiagnosticCertificationGrade::ReleaseCertified,
        ),
        narrowed_collection_row(),
    ]
}

fn guardrails() -> DiagnosticTruthCertificationGuardrails {
    DiagnosticTruthCertificationGuardrails {
        display_clustering_never_erases_provenance: true,
        imported_versus_live_class_stays_explicit: true,
        freshness_and_remap_states_stay_explicit: true,
        anchor_remap_is_append_only_evidence: true,
        mutating_routes_are_typed_quality_proposals: true,
        rows_auto_narrow_without_current_proof: true,
    }
}

fn consumer_projection() -> DiagnosticTruthCertificationConsumerProjection {
    DiagnosticTruthCertificationConsumerProjection {
        editor_ingests_certification: true,
        problems_ingests_certification: true,
        review_ingests_certification: true,
        cli_headless_ingests_certification: true,
        support_export_ingests_certification: true,
        ai_evidence_ingests_certification: true,
        release_debt_ingests_certification: true,
        narrowed_rows_labeled_below_claim: true,
    }
}

fn evidence_freshness() -> DiagnosticTruthCertificationFreshness {
    DiagnosticTruthCertificationFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        DIAGNOSTIC_TRUTH_CERT_SCHEMA_REF,
        DIAGNOSTIC_TRUTH_CERT_DOC_REF,
        DIAGNOSTIC_TRUTH_CERT_ARTIFACT_REF,
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
        "schemas/quality/diagnostic-record.schema.json",
        "schemas/quality/diagnostic-source-and-collection.schema.json",
        "schemas/quality/diagnostic-cluster.schema.json",
        "schemas/quality/anchor-remap-record.schema.json",
        "schemas/quality/quality-session-ledger.schema.json",
        "schemas/quality/diagnostic-quality-parity.schema.json",
    ])
}

fn packet() -> DiagnosticTruthCertificationPacket {
    DiagnosticTruthCertificationPacket::new(DiagnosticTruthCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Diagnostic-Truth Certification".to_owned(),
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

    match which.as_str() {
        "summary" => print!("{}", packet.render_markdown_summary()),
        "waiver" => print!("{}", packet.render_waiver_and_downgrade_log()),
        _ => println!("{}", packet.export_safe_json()),
    }
}
