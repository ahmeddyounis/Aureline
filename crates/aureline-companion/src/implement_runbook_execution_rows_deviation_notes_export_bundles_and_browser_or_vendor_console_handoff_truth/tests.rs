use super::*;

const PACKET_ID: &str = "runbook-execution-surface:stable:0001";
const PACKET_LABEL: &str =
    "Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> RunbookExecutionProofFreshness {
    RunbookExecutionProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> RunbookExecutionSurfacePacket {
    canonical_runbook_execution_surface(
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
        RunbookExecutionSection::ALL.len()
    );
    for section in RunbookExecutionSection::ALL {
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
    assert!(!packet.execution_rows.is_empty());
    assert!(!packet.deviation_notes.is_empty());
    assert!(!packet.export_bundles.is_empty());
    assert!(!packet.external_handoffs.is_empty());
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .execution_rows
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .deviation_notes
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .export_bundles
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .external_handoffs
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn execution_automation_requires_host_approval() {
    let packet = packet();
    for item in &packet.execution_rows {
        assert!(item.requires_host_approval);
    }
    // The canonical corpus carries an automated execution row, and it requires approval.
    let automated = packet
        .execution_rows
        .iter()
        .find(|item| item.automation_class.carries_automation())
        .expect("automated execution row present");
    assert!(automated.requires_host_approval);
}

#[test]
fn external_handoffs_keep_local_fallback_and_disclose_continuity() {
    let packet = packet();
    assert!(packet.external_handoffs_have_local_fallback());
    for item in &packet.external_handoffs {
        assert!(item.external.requires_provider_continuity);
        assert!(item.external.local_fallback_available);
        assert!(!item.handoff.deep_link_ref.trim().is_empty());
    }
}

#[test]
fn incomplete_export_bundle_is_first_class_and_labeled() {
    let packet = packet();
    assert!(packet.export_bundles_honestly_labeled());
    let partial = packet
        .export_bundles
        .iter()
        .find(|item| item.bundle_state == ExportBundleState::Partial)
        .expect("partial bundle present");
    assert!(partial.incomplete_label_shown);
}

#[test]
fn deviation_outcome_is_recorded() {
    let packet = packet();
    // The corpus carries a deviated execution row and a paired deviation note.
    assert!(packet
        .execution_rows
        .iter()
        .any(|item| item.outcome.is_deviation()));
    assert!(!packet.deviation_notes.is_empty());
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
        .contains(&RunbookExecutionViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "companion_notification".to_owned();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::SectionLaneMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.execution_rows[0].read_write_scope = CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ReadOnlyScopeViolated));
}

#[test]
fn execution_without_host_approval_fails() {
    let mut packet = packet();
    packet.execution_rows[0].requires_host_approval = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExecutionAutomationNotApproved));
}

#[test]
fn export_bundle_without_redaction_check_fails() {
    let mut packet = packet();
    packet.export_bundles[0].redaction_checked = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExportBundleNotRedactionChecked));
}

#[test]
fn unlabeled_incomplete_bundle_fails() {
    let mut packet = packet();
    packet.export_bundles[0].bundle_state = ExportBundleState::Building;
    packet.export_bundles[0].incomplete_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExportBundleIncompleteNotLabeled));
}

#[test]
fn external_handoff_without_continuity_disclosure_fails() {
    let mut packet = packet();
    packet.external_handoffs[0]
        .external
        .requires_provider_continuity = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExternalHandoffContinuityNotDisclosed));
}

#[test]
fn external_handoff_without_local_fallback_fails() {
    let mut packet = packet();
    packet.external_handoffs[0]
        .external
        .local_fallback_available = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExternalHandoffMissingLocalFallback));
}

#[test]
fn external_handoff_missing_ref_fails() {
    let mut packet = packet();
    packet.external_handoffs[0].external.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ExternalHandoffRefMissing));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.export_bundles[0].freshness = CompanionFreshnessState::Stale;
    packet.export_bundles[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.execution_rows[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.export_bundles.clear();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet.scope_contract.local_fallback_always_available = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ScopeContractIncomplete));
}

#[test]
fn attribution_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .attribution_contract
        .deviations_recorded_as_first_class = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::AttributionContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::StaleStateHonestyIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.deviations_recorded_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .browser_companion_shows_external_handoff = false;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RunbookExecutionViolation::ProofFreshnessIncomplete));
}

#[test]
fn degradation_on_relay_unavailable_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        export_complete: true,
        external_reachable: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::RelayUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::FreshnessDowngradedToStale));
    // Every previously live/cached item is now stale and labeled.
    assert!(packet
        .execution_rows
        .iter()
        .all(|item| item.freshness == CompanionFreshnessState::Stale && item.stale_label_shown));
    // The stable execution-row section narrows to beta and GA to staged rollout.
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Beta);
    assert_eq!(exec.rollout_stage, M5CompanionRolloutStage::StagedRollout);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        incident_attribution_intact: true,
        export_complete: true,
        external_reachable: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::HandoffTargetUnresolved));
    // No desktop handoff that requires an active host still claims exact resolution.
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    // Host-independent handoffs stay exact.
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    // The execution-row section narrows because an approved action can no longer relay.
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incident_attribution_loss_unattributes_and_narrows() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: false,
        export_complete: true,
        external_reachable: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::IncidentAttributionLost));
    // Every execution row and deviation note narrows to unattributed.
    assert!(packet
        .execution_rows
        .iter()
        .all(|item| item.attribution == IncidentAttributionState::Unattributed));
    assert!(packet
        .deviation_notes
        .iter()
        .all(|item| item.attribution == IncidentAttributionState::Unattributed));
    // Execution-row and deviation-note sections narrow; export bundles stay untouched.
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Beta);
    let bundle = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExportBundle)
        .expect("export-bundle section present");
    assert_eq!(bundle.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incomplete_export_narrows_bundles_and_labels() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        export_complete: false,
        external_reachable: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::ExportBundleIncomplete));
    // No bundle stays ready; every incomplete bundle is labeled.
    assert!(packet
        .export_bundles
        .iter()
        .all(|item| item.bundle_state != ExportBundleState::Ready));
    assert!(packet.export_bundles_honestly_labeled());
    // Only the export-bundle section narrows.
    let bundle = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExportBundle)
        .expect("export-bundle section present");
    assert_eq!(bundle.qualification, M5CompanionQualificationClass::Preview);
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Stable);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_unreachable_external_unresolves_and_narrows() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        export_complete: true,
        external_reachable: false,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::ExternalHandoffUnavailable));
    // Every external handoff is now unresolved, but the local desktop fallback stays exact.
    assert!(packet
        .external_handoffs
        .iter()
        .all(|item| item.external.resolution == CompanionHandoffResolution::Unresolved));
    assert!(packet
        .external_handoffs
        .iter()
        .all(|item| item.handoff.resolution == CompanionHandoffResolution::Exact));
    // Only the external-handoff section narrows.
    let external = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExternalHandoff)
        .expect("external-handoff section present");
    assert_eq!(
        external.qualification,
        M5CompanionQualificationClass::Experimental
    );
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Stable);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_trust_narrowing_narrows_execution_and_external() {
    let mut packet = packet();
    packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: false,
        incident_attribution_intact: true,
        export_complete: true,
        external_reachable: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&RunbookExecutionDegradedReason::TrustNarrowed));
    let exec = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExecutionRow)
        .expect("execution-row section present");
    assert_eq!(exec.qualification, M5CompanionQualificationClass::Beta);
    let external = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExternalHandoff)
        .expect("external-handoff section present");
    assert_eq!(
        external.qualification,
        M5CompanionQualificationClass::Experimental
    );
    // The deviation-note section stays stable: trust only narrows execution and external.
    let note = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::DeviationNote)
        .expect("deviation-note section present");
    assert_eq!(note.qualification, M5CompanionQualificationClass::Stable);
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the preview external-handoff section down to withheld via repeated narrowing.
    for _ in 0..4 {
        packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
            relay_available: true,
            proof_fresh: true,
            host_session_active: true,
            trust_intact: true,
            incident_attribution_intact: true,
            export_complete: true,
            external_reachable: false,
            upstream_matrix_narrowed: false,
        });
    }
    let external = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == RunbookExecutionSection::ExternalHandoff)
        .expect("external-handoff section present");
    assert_eq!(external.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&RunbookExecutionViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_runbook_execution_surface_export()
        .expect("checked runbook execution export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_runbook_execution_surface_export()
        .expect("checked runbook execution export validates");
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
    assert!(first.contains("## Execution rows"));
    assert!(first.contains("## Deviation notes"));
    assert!(first.contains("## Export bundles"));
    assert!(first.contains("## External handoffs"));
}
