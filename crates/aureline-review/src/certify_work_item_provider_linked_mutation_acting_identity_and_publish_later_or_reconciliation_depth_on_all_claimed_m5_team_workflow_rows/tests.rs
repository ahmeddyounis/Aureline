use super::*;

const PACKET_ID: &str = "m5-team-workflow-certification:certified:0001";
const MINTED_AT: &str = "2026-06-12T00:00:00Z";

fn proof_freshness() -> M5TeamWorkflowCertificationProofFreshness {
    M5TeamWorkflowCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_provider_authority_stale: true,
        auto_narrow_on_publish_later_stale: true,
        auto_narrow_on_reconciliation_stale: true,
    }
}

fn packet() -> M5TeamWorkflowCertificationPacket {
    certify_from_current_exports(
        PACKET_ID.to_owned(),
        "M5 Provider-Backed Team-Workflow Certification".to_owned(),
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
    let present: BTreeSet<M5TeamWorkflowClaimedRow> =
        packet.certified_rows.iter().map(|row| row.row).collect();
    for row in M5TeamWorkflowClaimedRow::ALL {
        assert!(
            present.contains(&row),
            "missing claimed row {}",
            row.as_str()
        );
    }
}

#[test]
fn first_consumer_certifies_from_current_exports() {
    let packet = packet();
    assert_eq!(packet.compatibility_report.blocked_count, 0);
    assert_eq!(packet.compatibility_report.not_certified_count, 0);
    assert!(packet.compatibility_report.all_rows_publishable);
}

#[test]
fn reconciliation_row_is_narrowed_to_beta() {
    let packet = packet();
    let reconciliation = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5TeamWorkflowClaimedRow::ProviderEventReconciliation)
        .expect("reconciliation row present");
    assert_eq!(
        reconciliation.claimed_qualification,
        M5TeamWorkflowQualificationClass::Beta
    );
    assert_eq!(
        reconciliation.verdict,
        M5TeamWorkflowCertificationVerdict::NarrowedCertified
    );
}

#[test]
fn provider_family_badging_is_not_generic() {
    let packet = packet();
    let mutation = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5TeamWorkflowClaimedRow::ProviderLinkedMutation)
        .expect("mutation row present");
    assert!(mutation
        .provider_family_compatibility
        .iter()
        .any(|entry| { entry.posture != M5TeamWorkflowProviderFamilyPosture::Qualified }));
}

#[test]
fn missing_row_fails_validation() {
    let mut packet = packet();
    packet
        .certified_rows
        .retain(|row| row.row != M5TeamWorkflowClaimedRow::BrowserHandoffContinuity);
    packet.compatibility_report =
        M5TeamWorkflowCertificationCompatibilityReport::from_rows(&packet.certified_rows);
    assert!(packet
        .validate()
        .contains(&M5TeamWorkflowCertificationViolation::RequiredRowMissing));
}

#[test]
fn missing_provider_story_fails() {
    let mut packet = packet();
    packet.certified_rows[0]
        .provider_family_compatibility
        .clear();
    assert!(packet
        .validate()
        .contains(&M5TeamWorkflowCertificationViolation::ProviderFamilyCompatibilityMissing));
}

#[test]
fn missing_offline_story_fails() {
    let mut packet = packet();
    packet.certified_rows[0]
        .offline_publish_later_story
        .proof_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&M5TeamWorkflowCertificationViolation::OfflinePublishLaterStoryIncomplete));
}

#[test]
fn compatibility_report_mismatch_fails() {
    let mut packet = packet();
    packet.compatibility_report.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&M5TeamWorkflowCertificationViolation::CompatibilityReportMismatch));
}

#[test]
fn downgrade_automation_blocks_on_invalid_evidence() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5TeamWorkflowCertificationRowObservation {
        row: M5TeamWorkflowClaimedRow::DeferredPublishContinuity,
        evidence_valid: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let deferred = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5TeamWorkflowClaimedRow::DeferredPublishContinuity)
        .expect("deferred row present");
    assert_eq!(
        deferred.verdict,
        M5TeamWorkflowCertificationVerdict::Blocked
    );
    assert_eq!(packet.compatibility_report.blocked_count, 1);
}

#[test]
fn downgrade_automation_narrows_on_stale_proof() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5TeamWorkflowCertificationRowObservation {
        row: M5TeamWorkflowClaimedRow::ProviderLinkedMutation,
        evidence_valid: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let mutation = packet
        .certified_rows
        .iter()
        .find(|row| row.row == M5TeamWorkflowClaimedRow::ProviderLinkedMutation)
        .expect("mutation row present");
    assert_eq!(
        mutation.verdict,
        M5TeamWorkflowCertificationVerdict::NarrowedCertified
    );
    assert!(!mutation.proof_freshness.proof_fresh);
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = packet().render_markdown_summary();
    for row in M5TeamWorkflowClaimedRow::ALL {
        assert!(
            summary.contains(row.as_str()),
            "summary missing row {}",
            row.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_m5_team_workflow_certification_export()
        .expect("checked M5 team-workflow certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_first_consumer_certification() {
    let checked = current_m5_team_workflow_certification_export()
        .expect("checked M5 team-workflow certification export validates");
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
            "/../../fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/deferred_publish_evidence_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/provider_linked_mutation_proof_stale_narrowed.json"
        )),
    ] {
        let packet: M5TeamWorkflowCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
