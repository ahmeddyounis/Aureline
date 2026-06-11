use super::*;

const PACKET_ID: &str = "companion-triage-surface:stable:0001";
const PACKET_LABEL: &str = "Companion Notification Triage, Review Queues, and CI-Status Cards";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> CompanionTriageProofFreshness {
    CompanionTriageProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> CompanionTriageSurfacePacket {
    canonical_companion_triage_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn canonical_surface_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_surface_covers_every_section() {
    let packet = packet();
    assert_eq!(
        packet.section_qualifications.len(),
        CompanionTriageSection::ALL.len()
    );
    for section in CompanionTriageSection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
    }
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    // Every section carries at least one item.
    assert!(!packet.notification_triage.is_empty());
    assert!(!packet.review_queue.is_empty());
    assert!(!packet.ci_status_cards.is_empty());
}

#[test]
fn companion_is_read_only() {
    let packet = packet();
    assert!(packet.notification_triage.iter().all(|item| item.read_only));
    // No review item can author; authority is approve/defer or escalate-only.
    for item in &packet.review_queue {
        assert!(matches!(
            item.decision_authority,
            CompanionDecisionAuthority::ApproveOrDefer | CompanionDecisionAuthority::EscalateOnly
        ));
    }
}

#[test]
fn missing_section_fails_validation() {
    let mut packet = packet();
    packet
        .section_qualifications
        .retain(|row| row.section != CompanionTriageSection::CiStatusCards);
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "managed_sync".to_owned();
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::SectionLaneMismatch));
}

#[test]
fn non_read_only_notification_fails() {
    let mut packet = packet();
    packet.notification_triage[0].read_only = false;
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::NotificationNotReadOnly));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.review_queue[0].handoff.deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.ci_status_cards.clear();
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::SectionContentMissing));
}

#[test]
fn handoff_contract_incomplete_fails() {
    let mut packet = packet();
    packet.handoff_contract.read_only_companion = false;
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::HandoffContractIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet
        .locality_disclosure
        .requires_provider_or_admin_continuity
        .clear();
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.companion_read_only = false;
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_sections = false;
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CompanionTriageViolation::ProofFreshnessIncomplete));
}

#[test]
fn degradation_on_relay_unavailable_narrows_and_stales_cards() {
    let mut packet = packet();
    packet.apply_companion_degradation(&CompanionSurfaceObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionDegradedReason::RelayUnavailable));
    // Every CI card is stale once the relay drops.
    assert!(packet
        .ci_status_cards
        .iter()
        .all(|card| card.freshness == CompanionCardFreshness::Stale));
    // The stable notification section narrows to beta and GA to staged rollout.
    let notif = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == CompanionTriageSection::NotificationTriage)
        .expect("notification section present");
    assert_eq!(notif.qualification, M5CompanionQualificationClass::Beta);
    assert_eq!(notif.rollout_stage, M5CompanionRolloutStage::StagedRollout);
    // The packet still validates: degradation narrows, it does not corrupt.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.apply_companion_degradation(&CompanionSurfaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&CompanionDegradedReason::HandoffTargetUnresolved));
    // No handoff that requires an active host still claims exact resolution.
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    // Host-independent handoffs (e.g. CI pipeline, incident workspace) stay exact.
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    assert!(!packet.all_handoffs_exact());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_trust_narrowing_only_narrows_review_queue() {
    let mut packet = packet();
    packet.apply_companion_degradation(&CompanionSurfaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: false,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionDegradedReason::TrustNarrowed));
    let review = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == CompanionTriageSection::ReviewQueue)
        .expect("review section present");
    assert_eq!(review.qualification, M5CompanionQualificationClass::Beta);
    // Notification stays stable because trust only narrows the review queue.
    let notif = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == CompanionTriageSection::NotificationTriage)
        .expect("notification section present");
    assert_eq!(notif.qualification, M5CompanionQualificationClass::Stable);
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the beta CI-status section down to held/withheld via repeated upstream narrowing.
    for _ in 0..4 {
        packet.apply_companion_degradation(&CompanionSurfaceObservation {
            relay_available: true,
            proof_fresh: true,
            host_session_active: true,
            trust_intact: true,
            upstream_matrix_narrowed: true,
        });
    }
    let ci = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == CompanionTriageSection::CiStatusCards)
        .expect("ci section present");
    assert_eq!(ci.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&CompanionTriageViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_companion_triage_surface_export()
        .expect("checked companion triage export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_companion_triage_surface_export()
        .expect("checked companion triage export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}
