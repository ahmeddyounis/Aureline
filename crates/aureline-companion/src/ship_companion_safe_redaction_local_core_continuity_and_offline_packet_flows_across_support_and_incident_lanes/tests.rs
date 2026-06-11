use super::*;

const PACKET_ID: &str = "redaction-continuity-offline-packet-surface:stable:0001";
const PACKET_LABEL: &str =
    "Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> RedactionProofFreshness {
    RedactionProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> RedactionContinuitySurfacePacket {
    canonical_redaction_continuity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

fn healthy_observation() -> RedactionDegradationObservation {
    RedactionDegradationObservation {
        redaction_proof_available: true,
        packet_assembler_available: true,
        proof_fresh: true,
        completeness_verified: true,
        incident_attribution_available: true,
        managed_service_available: true,
        host_session_active: true,
        upstream_matrix_narrowed: false,
    }
}

#[test]
fn canonical_surface_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_surface_covers_every_section_with_bound_lanes() {
    let packet = packet();
    assert_eq!(
        packet.section_qualifications.len(),
        RedactionContinuitySection::ALL.len()
    );
    for section in RedactionContinuitySection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, section.bounded_scope());
    }
    // The three lanes this surface binds together.
    let lane = |section: RedactionContinuitySection| section.matrix_lane();
    assert_eq!(
        lane(RedactionContinuitySection::RedactionPolicy),
        M5CompanionMatrixLane::CompanionNotification
    );
    assert_eq!(
        lane(RedactionContinuitySection::OfflineIncidentPacket),
        M5CompanionMatrixLane::IncidentWorkspace
    );
    assert_eq!(
        lane(RedactionContinuitySection::LocalCoreContinuity),
        M5CompanionMatrixLane::OffboardingContinuity
    );
    assert_eq!(
        lane(RedactionContinuitySection::OfflineSupportPacket),
        M5CompanionMatrixLane::OffboardingContinuity
    );
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.redaction_policy.is_empty());
    assert!(!packet.continuity_rows.is_empty());
    assert!(!packet.incident_packets.is_empty());
    assert!(!packet.support_packets.is_empty());
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .redaction_policy
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .continuity_rows
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .incident_packets
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .support_packets
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn canonical_surface_honors_redaction_and_continuity_invariants() {
    let packet = packet();
    assert!(packet.redaction_provable_or_labeled());
    assert!(packet.no_payload_body_crosses_boundary());
    assert!(packet.incident_packet_local_path_available());
    assert!(packet.support_packet_local_path_available());
    assert!(packet.packet_completeness_honestly_qualified());
    assert!(packet.incident_packets_attributable());
    assert!(packet.local_core_continuity_preserved());
    assert!(packet.stale_state_honestly_labeled());
}

#[test]
fn unverified_redaction_without_label_fails() {
    let mut packet = packet();
    packet.redaction_policy[0].redaction_verified = false;
    packet.redaction_policy[0].redaction_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::RedactionClaimNotLabeled));
}

#[test]
fn payload_body_present_fails() {
    let mut packet = packet();
    packet.redaction_policy[0].no_payload_body = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::PayloadBodyPresent));
}

#[test]
fn missing_local_incident_path_fails() {
    let mut packet = packet();
    for item in &mut packet.incident_packets {
        item.availability = OfflinePacketAvailability::RequiresProviderAssembly;
    }
    assert!(packet
        .validate()
        .contains(&RedactionViolation::LocalIncidentPacketPathMissing));
}

#[test]
fn missing_local_support_path_fails() {
    let mut packet = packet();
    for item in &mut packet.support_packets {
        item.availability = OfflinePacketAvailability::Unavailable;
    }
    assert!(packet
        .validate()
        .contains(&RedactionViolation::LocalSupportPacketPathMissing));
}

#[test]
fn completeness_claimed_but_unverified_fails() {
    let mut packet = packet();
    packet.incident_packets[0].completeness = PacketCompleteness::CompleteVerified;
    packet.incident_packets[0].claim_verified = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::CompletenessClaimedButUnverified));
}

#[test]
fn unverified_completeness_without_label_fails() {
    let mut packet = packet();
    packet.support_packets[0].completeness = PacketCompleteness::CompleteUnverified;
    packet.support_packets[0].claim_verified = false;
    packet.support_packets[0].proof_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::CompletenessClaimNotLabeled));
}

#[test]
fn unlabeled_missing_incident_attribution_fails() {
    let mut packet = packet();
    packet.incident_packets[0].attribution_present = false;
    packet.incident_packets[0].attribution_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::IncidentAttributionNotLabeled));
}

#[test]
fn continuity_offline_mismatch_fails() {
    let mut packet = packet();
    // A local-core-authoritative capability marked unavailable offline is inconsistent.
    packet.continuity_rows[0].continuity_posture = ContinuityPosture::LocalCoreAuthoritative;
    packet.continuity_rows[0].available_offline = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ContinuityOfflineMismatch));
}

#[test]
fn continuity_flag_mismatch_fails() {
    let mut packet = packet();
    packet.continuity_rows[0].continuity_posture = ContinuityPosture::LocalCoreAuthoritative;
    packet.continuity_rows[0].requires_provider_continuity = true;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ContinuityFlagMismatch));
}

#[test]
fn stranded_continuity_local_work_fails() {
    let mut packet = packet();
    packet.continuity_rows[0].local_work_preserved = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::LocalWorkStranded));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.redaction_policy[0].freshness = CompanionFreshnessState::Stale;
    packet.redaction_policy[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.incident_packets[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RedactionViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.continuity_rows.clear();
    assert!(packet
        .validate()
        .contains(&RedactionViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .scope_contract
        .action_applied_by_local_core_not_surface = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ScopeContractIncomplete));
}

#[test]
fn honesty_contract_incomplete_fails() {
    let mut packet = packet();
    packet.honesty_contract.companion_safe_redaction_enforced = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::HonestyContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::StaleStateHonestyIncomplete));
}

#[test]
fn continuity_contract_incomplete_fails() {
    let mut packet = packet();
    packet.continuity_contract.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ContinuityContractIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&RedactionViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RedactionViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.companion_safe_redaction_enforced = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .help_about_shows_redaction_and_continuity_honesty = false;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RedactionViolation::ProofFreshnessIncomplete));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref =
        M5CompanionMatrixLane::ManagedSync.as_str().to_owned();
    assert!(packet
        .validate()
        .contains(&RedactionViolation::SectionLaneMismatch));
}

#[test]
fn degradation_on_managed_service_degraded_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        managed_service_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::ManagedServiceDegraded));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::FreshnessDowngradedToStale));
    // Every live/cached item is now stale and labeled; the local paths still exist.
    assert!(packet.stale_state_honestly_labeled());
    assert!(packet.incident_packet_local_path_available());
    assert!(packet.support_packet_local_path_available());
    assert!(packet.local_core_continuity_preserved());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_redaction_proof_lost_narrows_and_labels_redaction() {
    let mut packet = packet();
    // redact:0001 is a verified redacted_summary that should narrow to reference_only.
    assert_eq!(
        packet.redaction_policy[0].redaction_class,
        RedactionClass::RedactedSummary
    );
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        redaction_proof_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::RedactionProofUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::RedactionNarrowedToReference));
    assert_eq!(
        packet.redaction_policy[0].redaction_class,
        RedactionClass::ReferenceOnly
    );
    assert!(!packet.redaction_policy[0].redaction_verified);
    assert!(packet.redaction_policy[0].redaction_label_shown);
    // No verified redaction survives across any section.
    assert!(packet
        .redaction_policy
        .iter()
        .all(|item| !item.redaction_verified));
    assert!(packet
        .incident_packets
        .iter()
        .all(|item| !item.redaction_verified));
    assert!(packet
        .support_packets
        .iter()
        .all(|item| !item.redaction_verified));
    assert!(packet.redaction_provable_or_labeled());
    assert!(packet.no_payload_body_crosses_boundary());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_packet_assembler_down_narrows_to_local_path() {
    let mut packet = packet();
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        packet_assembler_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::PacketAssemblerUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::PacketNarrowedToLocalPath));
    // Every provider-assembled packet narrowed; the local-first path keeps working.
    assert!(packet
        .incident_packets
        .iter()
        .all(|item| item.availability != OfflinePacketAvailability::RequiresProviderAssembly));
    assert!(packet
        .support_packets
        .iter()
        .all(|item| item.availability != OfflinePacketAvailability::RequiresProviderAssembly));
    assert!(packet.incident_packet_local_path_available());
    assert!(packet.support_packet_local_path_available());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_completeness_unverified_downgrades_claims() {
    let mut packet = packet();
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        completeness_verified: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::CompletenessUnverified));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::CompletenessClaimDowngraded));
    assert!(packet
        .incident_packets
        .iter()
        .all(|item| item.completeness != PacketCompleteness::CompleteVerified));
    assert!(packet
        .support_packets
        .iter()
        .all(|item| item.completeness != PacketCompleteness::CompleteVerified));
    assert!(packet.packet_completeness_honestly_qualified());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incident_attribution_lost_narrows_and_labels_attribution() {
    let mut packet = packet();
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        incident_attribution_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::IncidentAttributionUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::IncidentAttributionNarrowed));
    // No incident packet still claims attribution, and every one is honestly labeled.
    assert!(packet
        .incident_packets
        .iter()
        .all(|item| !item.attribution_present));
    assert!(packet.incident_packets_attributable());
    let incident = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RedactionContinuitySection::OfflineIncidentPacket)
        .expect("incident section present");
    assert_eq!(
        incident.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.redaction_policy[0].handoff.requires_active_host = true;
    packet.apply_redaction_degradation(&RedactionDegradationObservation {
        host_session_active: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&RedactionDegradedReason::HandoffTargetUnresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the preview incident-packet section down to withheld via repeated stale-proof
    // narrowing (early_access → internal_only → withheld).
    for _ in 0..2 {
        packet.apply_redaction_degradation(&RedactionDegradationObservation {
            proof_fresh: false,
            ..healthy_observation()
        });
    }
    let incident = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RedactionContinuitySection::OfflineIncidentPacket)
        .expect("incident section present");
    assert_eq!(incident.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&RedactionViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_stable_redaction_continuity_surface_export().expect("checked export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked =
        current_stable_redaction_continuity_surface_export().expect("checked export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}

#[test]
fn markdown_summary_is_deterministic() {
    let packet = packet();
    let first = packet.render_markdown_summary();
    let second = packet.render_markdown_summary();
    assert_eq!(first, second);
    assert!(first.contains("## Redaction policy"));
    assert!(first.contains("## Local-core continuity"));
    assert!(first.contains("## Offline incident packets"));
    assert!(first.contains("## Offline support packets"));
}
