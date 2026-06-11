use super::*;

const PACKET_ID: &str = "incident-workspace-surface:stable:0001";
const PACKET_LABEL: &str =
    "Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> IncidentWorkspaceProofFreshness {
    IncidentWorkspaceProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> IncidentWorkspaceSurfacePacket {
    canonical_incident_workspace_surface(
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
        IncidentWorkspaceSection::ALL.len()
    );
    for section in IncidentWorkspaceSection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, section.bounded_scope());
        // Every section inherits the single incident_workspace matrix lane.
        assert_eq!(
            row.matrix_lane_ref,
            M5CompanionMatrixLane::IncidentWorkspace.as_str()
        );
    }
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.headers.is_empty());
    assert!(!packet.evidence_timeline.is_empty());
    assert!(!packet.resource_slices.is_empty());
    assert!(!packet.runbook_packets.is_empty());
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .headers
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .evidence_timeline
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .resource_slices
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .runbook_packets
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn runbook_automation_requires_host_approval() {
    let packet = packet();
    for item in &packet.runbook_packets {
        assert!(item.requires_host_approval);
    }
    // The canonical corpus carries an automated runbook, and it requires approval.
    let automated = packet
        .runbook_packets
        .iter()
        .find(|item| item.automation_class.carries_automation())
        .expect("automated runbook present");
    assert!(automated.requires_host_approval);
}

#[test]
fn missing_evidence_span_is_first_class_and_labeled() {
    let packet = packet();
    assert!(packet.evidence_gaps_honestly_labeled());
    let missing = packet
        .evidence_timeline
        .iter()
        .find(|item| item.span_state == EvidenceSpanState::Missing)
        .expect("missing span present");
    assert!(missing.gap_label_shown);
}

#[test]
fn stale_items_are_honestly_labeled() {
    let packet = packet();
    assert!(packet.stale_state_honestly_labeled());
}

#[test]
fn missing_section_fails_validation() {
    let mut packet = packet();
    packet.section_qualifications.pop();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "companion_notification".to_owned();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::SectionLaneMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.headers[0].read_write_scope = CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::ReadOnlyScopeViolated));
}

#[test]
fn runbook_without_host_approval_fails() {
    let mut packet = packet();
    packet.runbook_packets[0].requires_host_approval = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::RunbookAutomationNotApproved));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.resource_slices[0].freshness = CompanionFreshnessState::Stale;
    packet.resource_slices[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::StaleStateNotLabeled));
}

#[test]
fn unlabeled_evidence_gap_fails() {
    let mut packet = packet();
    packet.evidence_timeline[0].span_state = EvidenceSpanState::Partial;
    packet.evidence_timeline[0].gap_label_shown = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::EvidenceGapNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.headers[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.resource_slices.clear();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet.scope_contract.runbook_read_only_unless_host_approved = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::ScopeContractIncomplete));
}

#[test]
fn attribution_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .attribution_contract
        .missing_spans_recorded_as_first_class = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::AttributionContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::StaleStateHonestyIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.missing_evidence_recorded_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .diagnostics_shows_missing_and_stale_labels = false;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&IncidentWorkspaceViolation::ProofFreshnessIncomplete));
}

#[test]
fn degradation_on_relay_unavailable_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        evidence_complete: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::RelayUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::FreshnessDowngradedToStale));
    // Every previously live/cached item is now stale and labeled.
    assert!(packet
        .headers
        .iter()
        .all(|item| item.freshness == CompanionFreshnessState::Stale && item.stale_label_shown));
    // The stable header section narrows to beta and GA to staged rollout.
    let header = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::Header)
        .expect("header section present");
    assert_eq!(header.qualification, M5CompanionQualificationClass::Beta);
    assert_eq!(header.rollout_stage, M5CompanionRolloutStage::StagedRollout);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        incident_attribution_intact: true,
        evidence_complete: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::HandoffTargetUnresolved));
    // No handoff that requires an active host still claims exact resolution.
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    // Host-independent handoffs stay exact.
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    // The runbook section narrows because an approved action can no longer relay.
    let runbook = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::RunbookPacket)
        .expect("runbook section present");
    assert_eq!(
        runbook.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incident_attribution_loss_unattributes_and_narrows() {
    let mut packet = packet();
    packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: false,
        evidence_complete: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::IncidentAttributionLost));
    // Every header and evidence span narrows to unattributed.
    assert!(packet
        .headers
        .iter()
        .all(|item| item.attribution == IncidentAttributionState::Unattributed));
    assert!(packet
        .evidence_timeline
        .iter()
        .all(|item| item.attribution == IncidentAttributionState::Unattributed));
    // Header and evidence sections narrow; resource slices stay untouched.
    let header = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::Header)
        .expect("header section present");
    assert_eq!(header.qualification, M5CompanionQualificationClass::Beta);
    let slice = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::ResourceSlice)
        .expect("resource-slice section present");
    assert_eq!(slice.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incomplete_evidence_narrows_spans_and_labels_gaps() {
    let mut packet = packet();
    packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        evidence_complete: false,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::EvidenceIncomplete));
    // No span stays present; every gap is labeled.
    assert!(packet
        .evidence_timeline
        .iter()
        .all(|item| item.span_state != EvidenceSpanState::Present));
    assert!(packet.evidence_gaps_honestly_labeled());
    // Only the evidence-timeline section narrows.
    let evidence = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::EvidenceTimeline)
        .expect("evidence section present");
    assert_eq!(evidence.qualification, M5CompanionQualificationClass::Beta);
    let header = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::Header)
        .expect("header section present");
    assert_eq!(header.qualification, M5CompanionQualificationClass::Stable);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_trust_narrowing_only_narrows_runbook() {
    let mut packet = packet();
    packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: false,
        incident_attribution_intact: true,
        evidence_complete: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&IncidentWorkspaceDegradedReason::TrustNarrowed));
    let runbook = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::RunbookPacket)
        .expect("runbook section present");
    assert_eq!(
        runbook.qualification,
        M5CompanionQualificationClass::Experimental
    );
    // The header section stays stable: trust only narrows the runbook section.
    let header = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::Header)
        .expect("header section present");
    assert_eq!(header.qualification, M5CompanionQualificationClass::Stable);
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the preview runbook section down to withheld via repeated narrowing.
    for _ in 0..4 {
        packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
            relay_available: true,
            proof_fresh: true,
            host_session_active: true,
            trust_intact: false,
            incident_attribution_intact: true,
            evidence_complete: true,
            upstream_matrix_narrowed: false,
        });
    }
    let runbook = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == IncidentWorkspaceSection::RunbookPacket)
        .expect("runbook section present");
    assert_eq!(runbook.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&IncidentWorkspaceViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_incident_workspace_surface_export()
        .expect("checked incident workspace export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_incident_workspace_surface_export()
        .expect("checked incident workspace export validates");
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
    assert!(first.contains("## Headers"));
    assert!(first.contains("## Evidence timeline"));
    assert!(first.contains("## Resource slices"));
    assert!(first.contains("## Runbook packets"));
}
