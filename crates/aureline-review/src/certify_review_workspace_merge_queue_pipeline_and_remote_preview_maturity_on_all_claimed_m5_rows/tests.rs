use super::*;

const PACKET_ID: &str = "m5-review-certification:certified:0001";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> M5ReviewCertificationProofFreshness {
    M5ReviewCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> M5ReviewCertificationPacket {
    certify_from_current_exports(
        PACKET_ID.to_owned(),
        "M5 Review, Merge-Queue, Pipeline, and Remote-Preview Certification".to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn certification_covers_every_claimed_row() {
    let packet = packet();
    let present: BTreeSet<M5ClaimedRow> = packet.certified_rows.iter().map(|row| row.row).collect();
    for row in M5ClaimedRow::ALL {
        assert!(
            present.contains(&row),
            "missing claimed row {}",
            row.as_str()
        );
    }
}

#[test]
fn first_consumer_certifies_from_current_exports() {
    // Every upstream row currently has a checked-in, valid export, so none are blocked.
    let packet = packet();
    assert_eq!(packet.compatibility_report.blocked_count, 0);
    assert_eq!(packet.compatibility_report.not_certified_count, 0);
    assert!(packet.compatibility_report.all_rows_publishable);
    for row in &packet.certified_rows {
        assert!(
            row.verdict.is_publishable(),
            "row {} not publishable: {:?}",
            row.row.as_str(),
            row.verdict
        );
    }
}

#[test]
fn remote_preview_is_narrowed_to_beta() {
    let packet = packet();
    let remote = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5ClaimedRow::RemotePreviewRoute)
        .expect("remote preview row present");
    assert_eq!(
        remote.claimed_qualification,
        M5ReviewCertificationQualificationClass::Beta
    );
    assert_eq!(
        remote.verdict,
        M5ReviewCertificationVerdict::NarrowedCertified
    );
}

#[test]
fn missing_row_fails_validation() {
    let mut packet = packet();
    packet
        .certified_rows
        .retain(|row| row.row != M5ClaimedRow::PipelineViewer);
    packet.compatibility_report =
        M5ReviewCertificationCompatibilityReport::from_rows(&packet.certified_rows);
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::RequiredRowMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.certified_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.certified_rows[0].evidence_artifact_ref.clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5ReviewCertificationViolation::RowIncomplete));
}

#[test]
fn compatibility_report_mismatch_fails() {
    let mut packet = packet();
    packet.compatibility_report.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::CompatibilityReportMismatch));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remote_preview_shows_certification = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReviewCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn downgrade_automation_blocks_on_invalid_evidence() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5ReviewCertificationRowObservation {
        row: M5ClaimedRow::MergeQueueReadiness,
        evidence_valid: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let merge = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5ClaimedRow::MergeQueueReadiness)
        .expect("merge queue row present");
    assert_eq!(merge.verdict, M5ReviewCertificationVerdict::Blocked);
    assert_eq!(packet.compatibility_report.blocked_count, 1);
    assert!(!packet.compatibility_report.all_rows_publishable);
    // Still serializes and validates structurally after automation.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn downgrade_automation_narrows_on_stale_proof() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5ReviewCertificationRowObservation {
        row: M5ClaimedRow::DurableReviewHeader,
        evidence_valid: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let header = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5ClaimedRow::DurableReviewHeader)
        .expect("durable review header row present");
    assert_eq!(
        header.verdict,
        M5ReviewCertificationVerdict::NarrowedCertified
    );
    assert!(!header.proof_freshness.proof_fresh);
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = packet().render_markdown_summary();
    for row in M5ClaimedRow::ALL {
        assert!(
            summary.contains(row.as_str()),
            "summary missing row {}",
            row.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_m5_review_certification_export()
        .expect("checked M5 review certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_first_consumer_certification() {
    let checked = current_m5_review_certification_export()
        .expect("checked M5 review certification export validates");
    let regenerated = packet();
    assert_eq!(
        checked, regenerated,
        "checked export drifted from the first-consumer certification"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/merge_queue_evidence_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/durable_header_proof_stale_narrowed.json"
        )),
    ] {
        let packet: M5ReviewCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
