use super::*;

const PACKET_ID: &str = "m5-provider-workitem-governance:stable:0001";

fn packet() -> M5ProviderWorkItemGovernancePacket {
    canonical_m5_provider_workitem_governance()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != M5ProviderWorkItemGovernanceLane::DeferredPublishContinuity);
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::RequiredLaneMissing));
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = packet();
    packet
        .object_vocabulary_rows
        .retain(|row| row.object_class != ProviderWorkItemObjectClass::ProviderEventEnvelope);
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::RequiredObjectClassMissing));
}

#[test]
fn missing_identity_class_fails_validation() {
    let mut packet = packet();
    packet
        .acting_identity_rows
        .retain(|row| row.identity_class != ActingIdentityClass::DeniedScope);
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::RequiredActingIdentityMissing));
}

#[test]
fn missing_truth_state_fails_validation() {
    let mut packet = packet();
    packet
        .truth_state_rows
        .retain(|row| row.truth_state_class != ProviderTruthStateClass::QueuedPublish);
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::RequiredTruthStateMissing));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_source_contracts_fail() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.callback_events_deduplicated = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .incident_workspace_preserves_provider_lineage = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderWorkItemGovernanceViolation::ProofFreshnessIncomplete));
}

#[test]
fn downgrade_automation_narrows_expected_lanes() {
    let packet = packet().apply_downgrade_automation(M5ProviderWorkItemGovernanceDegradation {
        provider_authority_stale: true,
        publish_later_continuity_stale: true,
        callback_reconciliation_stale: true,
        ..Default::default()
    });
    let by_lane = packet
        .lane_rows
        .iter()
        .map(|row| (row.lane, row.qualification))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(
        by_lane[&M5ProviderWorkItemGovernanceLane::WorkItemObjectVocabulary],
        M5ProviderWorkItemGovernanceQualificationClass::Beta
    );
    assert_eq!(
        by_lane[&M5ProviderWorkItemGovernanceLane::ProviderLinkedMutation],
        M5ProviderWorkItemGovernanceQualificationClass::Beta
    );
    assert_eq!(
        by_lane[&M5ProviderWorkItemGovernanceLane::ActingIdentityAndEffectiveScope],
        M5ProviderWorkItemGovernanceQualificationClass::Beta
    );
    assert_eq!(
        by_lane[&M5ProviderWorkItemGovernanceLane::DeferredPublishContinuity],
        M5ProviderWorkItemGovernanceQualificationClass::Beta
    );
    assert_eq!(
        by_lane[&M5ProviderWorkItemGovernanceLane::ProviderEventReconciliation],
        M5ProviderWorkItemGovernanceQualificationClass::Preview
    );
}

#[test]
fn markdown_summary_lists_every_lane() {
    let summary = packet().render_markdown_summary();
    for lane in M5ProviderWorkItemGovernanceLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_provider_workitem_governance_export()
        .expect("checked M5 provider-work-item governance export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/callback_reconciliation_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/publish_later_continuity_held.json"
        )),
    ] {
        let packet: M5ProviderWorkItemGovernancePacket =
            serde_json::from_str(raw).expect("fixture parses");
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
