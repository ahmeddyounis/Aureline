//! Fixture-driven coverage for connected-provider registry alpha records.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    ConnectedProviderAlphaPacket, FindingSeverity, KeyMode, MutationSurfaceState,
    PipelineOverlayKind, ProviderFamily, RegionMode, ResidencyMode, RunControlClass,
    StaleTargetRiskClass,
};
use aureline_support::capabilities::LifecycleState;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/connected_provider_alpha/registry_packet.json")
}

fn load_packet() -> ConnectedProviderAlphaPacket {
    let text = fs::read_to_string(fixture_path()).expect("read provider alpha fixture");
    serde_json::from_str(&text).expect("parse provider alpha fixture")
}

#[test]
fn registry_packet_covers_required_provider_states() {
    let packet = load_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "provider alpha fixture failed validation: {:#?}",
        report.findings
    );
    assert!(report
        .coverage
        .provider_families
        .contains(&ProviderFamily::CodeHost));
    assert!(report
        .coverage
        .provider_families
        .contains(&ProviderFamily::IssueTracker));
    assert!(report
        .coverage
        .provider_families
        .contains(&ProviderFamily::CiChecks));
    assert!(report
        .coverage
        .region_modes
        .contains(&RegionMode::ProviderDefaultDisclosed));
    assert!(report
        .coverage
        .residency_modes
        .contains(&ResidencyMode::ProviderDefault));
    assert!(report
        .coverage
        .key_modes
        .contains(&KeyMode::ProviderManaged));
    assert!(report
        .coverage
        .mutation_surface_states
        .contains(&MutationSurfaceState::LocalDraft));
    assert!(report
        .coverage
        .mutation_surface_states
        .contains(&MutationSurfaceState::PublishNow));
    assert!(report
        .coverage
        .mutation_surface_states
        .contains(&MutationSurfaceState::OpenInProvider));
    assert!(report
        .coverage
        .mutation_surface_states
        .contains(&MutationSurfaceState::PublishLaterQueue));
}

#[test]
fn unknown_region_residency_and_key_modes_are_warned() {
    let mut packet = load_packet();
    let descriptor = &mut packet.registry.descriptors[0];
    descriptor.region_mode = RegionMode::Unknown;
    descriptor.residency_mode = ResidencyMode::Unknown;
    descriptor.key_mode = KeyMode::Unknown;

    let report = packet.validate();
    assert!(
        report.passed,
        "unknown mode warnings should not hide errors"
    );
    assert_warning(&report, "provider_alpha.region_mode_unknown");
    assert_warning(&report, "provider_alpha.residency_mode_unknown");
    assert_warning(&report, "provider_alpha.key_mode_unknown");
}

#[test]
fn residency_mismatch_trips_warning_without_hiding_region_pin() {
    let mut packet = load_packet();
    let descriptor = &mut packet.registry.descriptors[2];
    descriptor.region_mode = RegionMode::CustomerRegionPinned;
    descriptor.residency_mode = ResidencyMode::ProviderDefault;

    let report = packet.validate();
    assert!(report.passed);
    assert!(report
        .coverage
        .region_modes
        .contains(&RegionMode::CustomerRegionPinned));
    assert_warning(&report, "provider_alpha.residency_mode_mismatch");
}

#[test]
fn pipeline_overlays_and_run_controls_are_auditable() {
    let packet = load_packet();
    let report = packet.validate();

    assert!(report
        .coverage
        .pipeline_overlay_kinds
        .contains(&PipelineOverlayKind::Run));
    assert!(report
        .coverage
        .pipeline_overlay_kinds
        .contains(&PipelineOverlayKind::Log));
    assert!(report
        .coverage
        .pipeline_overlay_kinds
        .contains(&PipelineOverlayKind::Artifact));
    assert!(report
        .coverage
        .pipeline_overlay_kinds
        .contains(&PipelineOverlayKind::Annotation));
    assert!(report
        .coverage
        .run_control_classes
        .contains(&RunControlClass::Rerun));
    assert!(report
        .coverage
        .run_control_classes
        .contains(&RunControlClass::Cancel));
    assert!(report
        .coverage
        .run_control_classes
        .contains(&RunControlClass::Retry));
}

#[test]
fn queue_items_remain_export_safe_and_ordered() {
    let packet = load_packet();
    for item in &packet.publish_later_queue {
        assert!(item.dependency_order_is_strict());
        assert!(item.is_export_safe());
        if item.stale_target_risk_class != StaleTargetRiskClass::TargetUnchanged {
            assert_ne!(
                item.next_safe_action,
                aureline_provider::QueueNextSafeActionClass::DrainNow
            );
        }
    }
}

#[test]
fn support_projection_does_not_inline_provider_payloads() {
    let packet = load_packet();
    let projection = packet.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");

    assert_eq!(projection.record_kind, "provider_alpha_support_export");
    assert!(!json.contains("raw_url"));
    assert!(!json.contains("token"));
    assert_eq!(
        projection.queue_summaries.len(),
        packet.publish_later_queue.len()
    );
    assert!(projection
        .descriptor_summaries
        .iter()
        .all(|summary| summary.region_mode == RegionMode::ProviderDefaultDisclosed));
    assert!(projection
        .descriptor_summaries
        .iter()
        .all(|summary| summary.residency_mode == ResidencyMode::ProviderDefault));
}

#[test]
fn tunnel_route_origin_labels_survive_provider_support_projection() {
    let packet = load_packet();
    let projection = packet.support_export_projection();
    let tunneled = projection
        .queue_summaries
        .iter()
        .find(|summary| {
            summary
                .target_ref
                .route_origin
                .as_ref()
                .is_some_and(|route| route.route_choice == "tunnel_exposed_route")
        })
        .expect("fixture carries a tunneled provider queue target");
    let route = tunneled
        .target_ref
        .route_origin
        .as_ref()
        .expect("route origin present");

    assert_eq!(route.target_class, "tunnel_exposed_target");
    assert_eq!(route.transport_label, "SSH tunnel");
    assert_eq!(
        route.tunnel_session_ref.as_deref(),
        Some("tunnel.session.ci.run.9912")
    );
}

#[test]
fn missing_browser_handoff_ref_is_rejected() {
    let mut packet = load_packet();
    let action = packet.registry.surface_claims[0]
        .actions
        .iter_mut()
        .find(|action| action.mutation_state == MutationSurfaceState::OpenInProvider)
        .expect("fixture has open-in-provider action");
    action.browser_handoff_packet_ref = None;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "provider_alpha.open_in_provider_packet_missing" }));
}

#[test]
fn stable_provider_claim_on_preview_lifecycle_is_rejected() {
    let mut packet = load_packet();
    let claim = &mut packet.registry.surface_claims[0];
    claim.capability_lifecycle_row_refs =
        vec!["capability_lifecycle:alpha.ai.routing_cost".to_string()];
    claim.claimed_lifecycle_state = Some(LifecycleState::Stable);

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "provider_alpha.surface_capability_claim_below_declared"
    }));
}

#[test]
fn non_contiguous_dependency_order_is_rejected() {
    let mut packet = load_packet();
    packet.publish_later_queue[0].dependency_chain[1].dependency_order_index = 9;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "provider_alpha.queue_dependency_order_invalid" }));
}

fn assert_warning(report: &aureline_provider::ProviderAlphaValidationReport, check_id: &str) {
    assert!(
        report.findings.iter().any(|finding| {
            finding.severity == FindingSeverity::Warning && finding.check_id == check_id
        }),
        "missing warning {check_id}: {:#?}",
        report.findings
    );
}
