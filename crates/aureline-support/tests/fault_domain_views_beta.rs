use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_host_topology_inspector, seeded_lane_filtered_event_viewer,
    seeded_reattach_review_sheet, HOST_TOPOLOGY_SCHEMA_VERSION,
};
use aureline_support::{
    seeded_fault_domain_view_packet, FaultDomainViewPacket, FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn fault_domain_view_packet_projects_topology_without_raw_payloads() {
    let inspector = seeded_host_topology_inspector();
    let packet = seeded_fault_domain_view_packet();

    assert_eq!(packet.record_kind, FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, HOST_TOPOLOGY_SCHEMA_VERSION);
    assert_eq!(packet.rows.len(), inspector.lanes.len());
    assert_eq!(packet.restart_cards.len(), inspector.lanes.len());
    assert!(packet.is_export_safe());
    assert_eq!(packet.validate(&inspector), Vec::new());
    assert!(packet.partial_truth_row_count >= 5);
    assert!(packet.blocked_healthy_claim_count >= 3);
}

#[test]
fn packet_preserves_banners_reattach_reviews_and_restart_markers() {
    let packet = seeded_fault_domain_view_packet();

    assert!(packet
        .crash_banners
        .iter()
        .any(
            |banner| banner.failing_host_lane_ref == "lane:notebook-kernel"
                && banner.blocks_healthy_claim
        ));
    assert!(packet
        .crash_banners
        .iter()
        .any(|banner| banner.failing_host_lane_ref == "lane:extension-sandbox"));
    assert!(packet
        .reattach_reviews
        .iter()
        .any(|review| review.decision_token == "reapproval_required"));
    assert!(packet
        .event_viewer
        .rows
        .iter()
        .any(|row| row.restart_marker_token == "restart_scheduled"));
    assert!(packet
        .event_viewer
        .rows
        .iter()
        .any(|row| row.restart_marker_token == "reattached_after_review"));

    let remote_row = packet
        .rows
        .iter()
        .find(|row| row.host_lane_ref == "lane:remote-agent")
        .expect("remote row exists");
    assert_eq!(
        remote_row.reattach_decision_token.as_deref(),
        Some("reapproval_required")
    );
    assert!(remote_row
        .partial_truth_result_refs
        .contains(&"result:preview:remote-web".to_owned()));
}

#[test]
fn support_plaintext_keeps_fault_domain_and_partial_truth_labels() {
    let packet = seeded_fault_domain_view_packet();
    let plaintext = packet.render_plaintext();

    assert!(plaintext.contains("Host lane and fault-domain packet"));
    assert!(plaintext.contains("Fault domain: fault-domain:notebook-kernel"));
    assert!(plaintext.contains("Restart budget: quarantined 2/2"));
    assert!(plaintext.contains("Partial truth: result:notebook:output"));
    assert!(plaintext.contains("Reattach: reapproval_required"));
    assert!(!plaintext.contains("/Users/"));
}

#[test]
fn checked_in_support_docs_schemas_and_artifacts_exist() {
    let root = repo_root();
    for rel in [
        "schemas/runtime/host_lane.schema.json",
        "schemas/runtime/fault_domain_state.schema.json",
        "docs/support/host_lane_and_reattach_beta.md",
        "fixtures/runtime/host_topology_and_reattach/manifest.yaml",
        "artifacts/support/fault_domain_packets/host_lane_fault_domain_packet.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn packet_builder_accepts_runtime_records_from_independent_callers() {
    let inspector = seeded_host_topology_inspector();
    let review = seeded_reattach_review_sheet();
    let event_viewer = seeded_lane_filtered_event_viewer();
    let packet = FaultDomainViewPacket::from_topology(
        "fault-domain-view:caller-owned",
        "2026-05-18T12:07:00Z",
        &inspector,
        vec![review],
        event_viewer,
    );

    assert_eq!(packet.validate(&inspector), Vec::new());
    assert!(packet
        .rows
        .iter()
        .all(|row| !row.fault_domain_id.is_empty()));
}
