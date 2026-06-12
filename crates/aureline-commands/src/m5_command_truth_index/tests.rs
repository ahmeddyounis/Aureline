use super::*;
use std::path::Path;

#[test]
fn seeded_packet_covers_every_required_truth_surface() {
    let packet = seeded_m5_command_truth_index_packet();
    for row in &packet.rows {
        for required in M5CommandTruthSurfaceClass::ALL {
            assert!(
                row.surface_rows
                    .iter()
                    .any(|surface| surface.surface_class == required),
                "{} missing {}",
                row.command_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn seeded_packet_passes_validation() {
    let packet = current_m5_command_truth_index_export()
        .expect("seeded M5 command truth index must validate");
    assert_eq!(packet.packet_id, M5_COMMAND_TRUTH_INDEX_PACKET_ID);
}

#[test]
fn non_stable_rows_do_not_publish_stable_wording() {
    let packet = seeded_m5_command_truth_index_packet();
    for row in packet
        .rows
        .iter()
        .filter(|row| row.effective_state_class != M5CapabilityStateClass::Stable)
    {
        assert!(!row.stable_wording_allowed, "{}", row.command_id);
        assert!(row
            .surface_rows
            .iter()
            .all(|surface| !surface.stable_wording_visible));
    }
}

#[test]
fn policy_blocked_and_retest_pending_rows_stay_explicit() {
    let packet = seeded_m5_command_truth_index_packet();

    let sync = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:sync.push_workspace_state")
        .expect("sync row must exist");
    assert_eq!(
        sync.truth_state_class,
        M5CommandTruthStateClass::PolicyBlocked
    );
    assert_eq!(
        sync.effective_state_class,
        M5CapabilityStateClass::DisabledByPolicy
    );

    let docs = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:docs_browser.open_external")
        .expect("docs browser row must exist");
    assert_eq!(
        docs.truth_state_class,
        M5CommandTruthStateClass::RetestPending
    );
    assert_eq!(
        docs.effective_state_class,
        M5CapabilityStateClass::RetestPending
    );
}

#[test]
fn support_export_quotes_packet_and_projection_refs() {
    let packet = seeded_m5_command_truth_index_packet();
    let export = M5CommandTruthIndexSupportExport::from_packet(
        M5_COMMAND_TRUTH_INDEX_SUPPORT_EXPORT_ID.to_string(),
        packet.clone(),
    );

    assert_eq!(export.packet, packet);
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export
        .case_ids
        .contains(&packet.source_command_governance_ref));
    assert!(export
        .case_ids
        .contains(&packet.source_capability_state_ref));
    assert!(export
        .case_ids
        .contains(&packet.source_rollout_inventory_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.command_id));
        assert!(export.case_ids.contains(&row.capability_id));
        assert!(export.case_ids.contains(&row.help_about_projection_ref));
        assert!(export.case_ids.contains(&row.diagnostics_projection_ref));
        assert!(export.case_ids.contains(&row.release_center_projection_ref));
        assert!(export.case_ids.contains(&row.support_export_projection_ref));
        assert!(export.case_ids.contains(&row.public_truth_projection_ref));
        for evidence_ref in &row.evidence_refs {
            assert!(export.case_ids.contains(evidence_ref));
        }
    }
}

#[test]
fn published_packet_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/m5_command_truth_index/packet.json");
    let on_disk = std::fs::read_to_string(&path).expect("packet fixture must exist");
    let seeded = seeded_m5_command_truth_index_packet();
    let rendered = serde_json::to_string_pretty(&seeded).expect("packet must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_support_export_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_command_truth_index/support_export.json");
    let on_disk = std::fs::read_to_string(&path).expect("support export must exist");
    let export = M5CommandTruthIndexSupportExport::from_packet(
        M5_COMMAND_TRUTH_INDEX_SUPPORT_EXPORT_ID.to_string(),
        seeded_m5_command_truth_index_packet(),
    );
    let rendered = serde_json::to_string_pretty(&export).expect("support export must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_summary_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_command_truth_index/summary.md");
    let on_disk = std::fs::read_to_string(&path).expect("summary artifact must exist");
    let rendered = seeded_m5_command_truth_index_packet().render_markdown();
    assert_eq!(on_disk, rendered);
}
