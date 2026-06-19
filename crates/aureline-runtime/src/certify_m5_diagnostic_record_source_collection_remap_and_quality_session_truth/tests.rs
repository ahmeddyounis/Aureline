use super::*;

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

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_row_kind_is_present() {
    let kinds = packet().represented_row_kinds();
    for kind in CertifiedDiagnosticRowKind::ALL {
        assert!(kinds.contains(&kind), "missing row kind: {}", kind.as_str());
    }
}

#[test]
fn every_dimension_is_certified() {
    let dimensions = packet().represented_dimensions();
    for dimension in DiagnosticEvidenceDimension::ALL {
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
        .retain(|row| row.row_kind != CertifiedDiagnosticRowKind::ImportedScannerRow);
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticTruthCertificationViolation::RequiredRowKindMissing));
    assert!(violations.contains(&DiagnosticTruthCertificationViolation::ImportedRowCaseMissing));
    assert!(violations.contains(&DiagnosticTruthCertificationViolation::SourceKindCoverageMissing));
    assert!(violations.contains(&DiagnosticTruthCertificationViolation::OriginCoverageMissing));
}

#[test]
fn missing_dimension_fails_validation() {
    let mut packet = packet();
    // Drop every quality-session certification so the dimension is unrepresented.
    for row in &mut packet.rows {
        row.certifications
            .retain(|c| c.dimension != DiagnosticEvidenceDimension::QualitySession);
    }
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::DimensionCoverageMissing));
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
        .find(|row| row.row_id == "diag-cert:framework:stale-collection:0001")
        .expect("narrowed row");
    // Re-back the stale collection proof so no row demonstrates auto-narrowing.
    for c in &mut narrowed.certifications {
        if c.dimension == DiagnosticEvidenceDimension::CollectionSnapshot {
            c.proof_currency = DiagnosticProofCurrency::VerifiedCurrent;
        }
    }
    narrowed.effective_grade = narrowed.claimed_grade;
    narrowed.narrow_trigger = None;
    narrowed.narrowed_label = None;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::NarrowedRowCaseMissing));
}

#[test]
fn claimed_row_losing_current_proof_must_narrow() {
    let mut packet = packet();
    let notebook = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:notebook:0001")
        .expect("notebook row");
    for c in &mut notebook.certifications {
        if c.dimension == DiagnosticEvidenceDimension::SourceDescriptor {
            c.proof_currency = DiagnosticProofCurrency::StaleExpired;
        }
    }
    assert!(notebook.needs_narrow());
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn missing_core_dimension_forces_narrow() {
    let mut packet = packet();
    let notebook = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:notebook:0001")
        .expect("notebook row");
    notebook
        .certifications
        .retain(|c| c.dimension != DiagnosticEvidenceDimension::AnchorRemap);
    assert!(notebook.needs_narrow());
    let violations = packet.validate();
    assert!(
        violations.contains(&DiagnosticTruthCertificationViolation::RowNotNarrowedOnUncurrentProof)
    );
}

#[test]
fn imported_proof_on_local_row_narrows() {
    let mut packet = packet();
    let framework = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:framework:0001")
        .expect("framework row");
    // Imported proof can never back a local row's claim.
    for c in &mut framework.certifications {
        if c.dimension == DiagnosticEvidenceDimension::RecordIdentity {
            c.proof_currency = DiagnosticProofCurrency::ImportedCurrent;
        }
    }
    assert!(framework.needs_narrow());
}

#[test]
fn imported_row_marker_mismatch_fails() {
    let mut packet = packet();
    let scanner = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:imported-scanner:0001")
        .expect("scanner row");
    // Drop the imported origin while keeping the imported_row flag.
    scanner.subject.origin_class = DiagnosticOriginClass::LiveLocalSession;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::ImportedReadsAsLocal));
}

#[test]
fn imported_live_class_dropped_fails() {
    let mut packet = packet();
    packet.rows[0].imported_live_class_preserved = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::ImportedReadsAsLocal));
}

#[test]
fn generic_narrowed_label_fails() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:framework:stale-collection:0001")
        .expect("narrowed row");
    narrowed.narrowed_label = Some("uncertified".to_owned());
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn fingerprint_substituting_identity_fails() {
    let mut packet = packet();
    packet.rows[0].subject.subject_fingerprint_token = packet.rows[0].subject.subject_id.clone();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn erased_source_kind_fails() {
    let mut packet = packet();
    packet.rows[0].source_kind_preserved = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::SourceKindErased));
}

#[test]
fn hidden_collection_completeness_fails() {
    let mut packet = packet();
    packet.rows[1].collection_completeness_visible = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::CollectionCompletenessHidden));
}

#[test]
fn silently_repaired_anchor_remap_fails() {
    let mut packet = packet();
    packet.rows[2].remap_history_append_only = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::AnchorRemapNotAppendOnly));
}

#[test]
fn mutating_route_bypassing_quality_session_fails() {
    let mut packet = packet();
    packet.rows[4].mutating_routes_use_quality_session = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::MutatingRouteBypassesQualitySession));
}

#[test]
fn source_kind_coverage_without_scanner_fails() {
    let mut packet = packet();
    // Re-home the imported-scanner row's source kind away from scanner import.
    let scanner = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "diag-cert:imported-scanner:0001")
        .expect("scanner row");
    scanner.subject.source_kind = DiagnosticSourceKind::LanguageService;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::SourceKindCoverageMissing));
}

#[test]
fn dimension_proof_without_fingerprint_fails() {
    let mut packet = packet();
    // A present proof ref with a fingerprint equal to the ref is not reopenable.
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_fingerprint_token = cert.proof_ref.clone();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn missing_proof_with_ref_fails() {
    let mut packet = packet();
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_currency = DiagnosticProofCurrency::MissingProof;
    // A missing proof must carry no ref; keeping one is malformed.
    assert!(!cert.is_well_formed());
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != DIAGNOSTIC_TRUTH_CERT_DOC_REF);
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.rows_auto_narrow_without_current_proof = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .release_debt_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_evidence_freshness_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthCertificationViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: DiagnosticTruthCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Diagnostic-Truth Certification"));
    assert!(summary.contains("imported_scanner_row"));
    assert!(summary.contains("Narrowed:"));
}

#[test]
fn waiver_log_names_narrowed_rows() {
    let log = packet().render_waiver_and_downgrade_log();
    assert!(log.contains("Waiver and Downgrade Log"));
    assert!(log.contains("No manual waivers"));
    assert!(log.contains("diag-cert:framework:stale-collection:0001"));
    assert!(log.contains("stale_dimension_proof"));
    assert!(log.contains("collection_snapshot"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_diagnostic_truth_certification_export()
        .expect("checked diagnostic truth certification export validates");
    assert_eq!(checked, packet());
}
