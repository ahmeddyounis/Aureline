use super::*;

const PACKET_ID: &str = "m5-docs-certification:stable:0001";

fn packet() -> CertificationPacket {
    CertificationPacket::new(seeded_stable_certification_input())
}

#[test]
fn seeded_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_certifies_every_surface() {
    let packet = packet();
    let present: BTreeSet<CertifiedSurfaceLane> =
        packet.surface_rows.iter().map(|row| row.lane).collect();
    for lane in CertifiedSurfaceLane::ALL {
        assert!(present.contains(&lane), "missing surface {}", lane.as_str());
    }
}

#[test]
fn seeded_packet_has_no_narrowed_or_blocked_surfaces() {
    let packet = packet();
    assert!(packet.narrowed_surfaces().is_empty());
    assert!(packet.promotion_blockers().is_empty());
}

#[test]
fn every_row_references_canonical_schema_and_artifact() {
    for row in packet().surface_rows {
        assert_eq!(row.schema_ref, row.lane.schema_ref());
        assert_eq!(row.artifact_ref, row.lane.artifact_ref());
    }
}

#[test]
fn missing_surface_fails() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.lane != CertifiedSurfaceLane::RetrievalDebug);
    assert!(packet
        .validate()
        .contains(&CertificationViolation::RequiredSurfaceMissing));
}

#[test]
fn surface_ref_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].schema_ref = "schemas/docs/wrong.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&CertificationViolation::SurfaceRefMismatch));
}

#[test]
fn certified_promoted_surface_missing_evidence_fails() {
    let mut packet = packet();
    packet.surface_rows[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&CertificationViolation::CertifiedSurfaceMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.surface_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&CertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn surface_greener_than_matrix_fails() {
    let mut packet = packet();
    packet.surface_rows[2].not_greener_than_matrix = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::SurfaceGreenerThanMatrix));
}

#[test]
fn verdict_qualification_mismatch_fails() {
    let mut packet = packet();
    // Held qualification with a Certified (promotion-permitting) verdict.
    packet.surface_rows[0].qualification = CertificationQualificationClass::Held;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::VerdictQualificationMismatch));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CertificationViolation::MissingSourceContracts));
}

#[test]
fn compatibility_report_incomplete_fails() {
    let mut packet = packet();
    packet.compatibility_report.no_surface_greener_than_matrix = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::CompatibilityReportIncomplete));
}

#[test]
fn downgrade_rules_incomplete_fails() {
    let mut packet = packet();
    packet.downgrade_rules[0].auto_enforced = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::DowngradeRulesIncomplete));
}

#[test]
fn empty_downgrade_rules_fails() {
    let mut packet = packet();
    packet.downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&CertificationViolation::DowngradeRulesIncomplete));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_surface_greener_than_packet = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for lane in CertifiedSurfaceLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing surface {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_stable_certification_export().expect("checked certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.surface_rows.len(), CertifiedSurfaceLane::ALL.len());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    let narrowed: CertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/recall_freshness_expired_narrows.json"
    )))
    .expect("recall-narrowed fixture parses");
    assert!(narrowed.validate().is_empty(), "{:?}", narrowed.validate());
    // The recall lanes are narrowed but still promotion-permitting (Beta).
    assert!(narrowed
        .narrowed_surfaces()
        .contains(&CertifiedSurfaceLane::SemanticRecall));
    assert!(narrowed.promotion_blockers().is_empty());

    let blocked: CertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/browser_scope_expansion_blocked.json"
    )))
    .expect("browser-blocked fixture parses");
    assert!(blocked.validate().is_empty(), "{:?}", blocked.validate());
    // The browser surface is blocked from promotion but still present, not hidden.
    assert!(blocked
        .promotion_blockers()
        .contains(&CertifiedSurfaceLane::ScopedBrowserSurface));
}
