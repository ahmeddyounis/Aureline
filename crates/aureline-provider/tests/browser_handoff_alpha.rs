//! Fixture-driven coverage for the provider browser-handoff alpha page.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    ContinuityObservationClass, DegradedActionClass, HandoffDestinationClass,
    HandoffFollowUpActionClass, HandoffOriginClass, HandoffPacketStateClass,
    ImportSessionStateClass, ProviderBrowserHandoffAlphaPage, ReconnectOutcomeClass,
    RetainedCapabilityClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m3/browser_handoff/page.json")
}

fn load_page() -> ProviderBrowserHandoffAlphaPage {
    let text = fs::read_to_string(fixture_path()).expect("read browser-handoff alpha fixture");
    serde_json::from_str(&text).expect("parse browser-handoff alpha fixture")
}

#[test]
fn alpha_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "browser-handoff alpha fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn fixture_covers_required_origin_and_destination_axes() {
    let page = load_page();
    let report = page.validate();
    for origin in [
        HandoffOriginClass::WorkspaceReviewLane,
        HandoffOriginClass::WorkspaceRuntimeLane,
        HandoffOriginClass::WorkspaceProviderLane,
    ] {
        assert!(
            report.coverage.origin_classes.contains(&origin),
            "missing origin coverage: {origin:?}"
        );
    }
    for destination in [
        HandoffDestinationClass::CodeHostWeb,
        HandoffDestinationClass::IssueTrackerWeb,
        HandoffDestinationClass::CiProviderWeb,
    ] {
        assert!(
            report.coverage.destination_classes.contains(&destination),
            "missing destination coverage: {destination:?}"
        );
    }
}

#[test]
fn fixture_covers_required_follow_up_actions() {
    let page = load_page();
    let report = page.validate();
    for follow_up in [
        HandoffFollowUpActionClass::ReturnToLocalDraftAuthoring,
        HandoffFollowUpActionClass::ReturnToPublishLaterQueueItem,
        HandoffFollowUpActionClass::ReturnToTruthfulPlaceholder,
    ] {
        assert!(
            report
                .coverage
                .follow_up_action_classes
                .contains(&follow_up),
            "missing follow-up coverage: {follow_up:?}"
        );
    }
}

#[test]
fn fixture_covers_reconnect_outcomes_and_import_session_states() {
    let page = load_page();
    let report = page.validate();
    for outcome in [
        ReconnectOutcomeClass::RestoredAuthoritativeLocalObject,
        ReconnectOutcomeClass::RestoredTruthfulPlaceholder,
    ] {
        assert!(
            report.coverage.reconnect_outcomes.contains(&outcome),
            "missing reconnect outcome coverage: {outcome:?}"
        );
    }
    for state in [
        ImportSessionStateClass::ObservedFresh,
        ImportSessionStateClass::StaleWithinWindow,
        ImportSessionStateClass::RevokedOrDisconnected,
    ] {
        assert!(
            report.coverage.import_session_states.contains(&state),
            "missing import-session state coverage: {state:?}"
        );
    }
}

#[test]
fn continuity_observations_reuse_provider_object_vocabulary() {
    let page = load_page();
    let report = page.validate();
    for class in [
        ContinuityObservationClass::ProviderStaleWithinWindow,
        ContinuityObservationClass::ProviderRevokedOrDisconnected,
    ] {
        assert!(
            report
                .coverage
                .continuity_observation_classes
                .contains(&class),
            "missing continuity observation class: {class:?}"
        );
    }
    for observation in &page.continuity_observations {
        assert!(!observation.silent_mutation_authority_widened);
        if observation.retained_capability_class != RetainedCapabilityClass::NoCapabilityRetained {
            assert_ne!(
                observation.degraded_action,
                DegradedActionClass::NoneRequired,
                "retained capability requires a degraded action"
            );
        }
    }
}

#[test]
fn every_packet_preserves_origin_destination_object_and_followup() {
    let page = load_page();
    for packet in &page.packets {
        assert!(!packet.origin.host_identity_ref.is_empty());
        assert!(!packet.origin.workspace_id_ref.is_empty());
        assert!(!packet.origin.execution_context_id_ref.is_empty());
        assert!(!packet.destination.canonical_host_ref.is_empty());
        assert!(!packet.destination.tenant_or_org_scope_ref.is_empty());
        assert!(!packet.provider_object_row_ref.is_empty());
        assert!(!packet.provider_side_object_id.is_empty());
        assert!(!packet.follow_up_summary.is_empty());
        assert!(!packet.raw_url_present);
        assert!(!packet.raw_token_material_present);
        assert!(!packet.raw_provider_payload_present);
        assert!(!packet.silent_authority_widening_taken);

        if packet.packet_state.requires_return_summary() {
            let summary = packet.return_summary.as_deref().unwrap_or("");
            assert!(
                !summary.is_empty(),
                "returned packet {} must cite a return_summary",
                packet.packet_id
            );
        }

        if matches!(
            packet.packet_state,
            HandoffPacketStateClass::ReturnedTruthfulPlaceholder
        ) {
            assert!(
                packet.placeholder_kind.is_some(),
                "truthful-placeholder packet {} must cite a placeholder_kind",
                packet.packet_id
            );
        }
    }
}

#[test]
fn reconnect_flows_resolve_to_object_row_or_placeholder() {
    let page = load_page();
    for flow in &page.reconnect_flows {
        assert!(flow.local_editing_preserved);
        assert!(!flow.silent_authority_widening_taken);
        if flow.outcome == ReconnectOutcomeClass::RestoredAuthoritativeLocalObject {
            assert!(
                flow.restored_object_row_ref.is_some(),
                "authoritative restore must name an object row: {}",
                flow.reconnect_flow_id
            );
            assert!(flow.placeholder_kind.is_none());
        }
        if flow.outcome == ReconnectOutcomeClass::RestoredTruthfulPlaceholder {
            assert!(
                flow.placeholder_kind.is_some(),
                "truthful placeholder must name a placeholder_kind: {}",
                flow.reconnect_flow_id
            );
            assert!(flow.restored_object_row_ref.is_none());
        }
    }
}

#[test]
fn support_projection_omits_action_refs() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(
        projection.record_kind,
        "provider_browser_handoff_alpha_support_export"
    );
    assert!(!json.contains("raw_url"));
    assert!(!json.contains("raw_token"));
    assert!(!json.contains("approval_ticket_ref"));
    assert!(!json.contains("integration_packet_ref"));
    assert!(!json.contains("publish_later_queue_item_ref"));
    assert!(!json.contains("follow_up_packet_ref"));
    assert!(!json.contains("superseded_session_ref"));
    assert_eq!(projection.packet_summaries.len(), page.packets.len());
    assert_eq!(
        projection.import_session_summaries.len(),
        page.import_sessions.len()
    );
    assert_eq!(
        projection.reconnect_summaries.len(),
        page.reconnect_flows.len()
    );
    assert_eq!(
        projection.continuity_summaries.len(),
        page.continuity_observations.len()
    );
}

#[test]
fn placeholder_state_requires_placeholder_kind_after_edit() {
    let mut page = load_page();
    let packet = page
        .packets
        .iter_mut()
        .find(|packet| packet.packet_id == "browser_handoff_alpha.packet.ci.99012.annotation")
        .expect("annotation packet present");
    packet.placeholder_kind = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "browser_handoff_alpha.packet_placeholder_missing"));
}

#[test]
fn restored_authoritative_outcome_requires_restored_object_after_edit() {
    let mut page = load_page();
    let flow = page
        .reconnect_flows
        .iter_mut()
        .find(|flow| flow.reconnect_flow_id == "browser_handoff_alpha.reconnect.issue.aur.104")
        .expect("restored reconnect present");
    flow.restored_object_row_ref = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| finding.check_id
        == "browser_handoff_alpha.reconnect_flow_restored_object_missing"));
}

#[test]
fn reconnect_unknown_import_session_is_rejected_after_edit() {
    let mut page = load_page();
    let flow = page
        .reconnect_flows
        .iter_mut()
        .next()
        .expect("reconnect flow present");
    flow.import_session_ref = "browser_handoff_alpha.import_session.does_not_exist".to_string();
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id
            == "browser_handoff_alpha.reconnect_flow_import_session_unknown"));
}
