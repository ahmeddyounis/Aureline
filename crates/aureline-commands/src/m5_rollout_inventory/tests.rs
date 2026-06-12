use super::*;
use std::path::Path;

#[test]
fn seeded_packet_passes_validation() {
    let packet = current_m5_rollout_inventory_export()
        .expect("seeded M5 rollout inventory export must validate");
    assert_eq!(packet.packet_id, M5_ROLLOUT_INVENTORY_PACKET_ID);
}

#[test]
fn every_row_covers_required_stable_facing_surfaces() {
    let packet = seeded_m5_rollout_inventory_packet();
    for row in &packet.rows {
        for required in M5RolloutConsumerSurfaceClass::ALL {
            assert!(
                row.surfaced_in
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
fn policy_blocked_and_retest_rows_are_seeded_explicitly() {
    let packet = seeded_m5_rollout_inventory_packet();

    let sync = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:sync.push_workspace_state")
        .expect("sync row must exist");
    assert_eq!(
        sync.effective_state_class,
        M5RolloutStateClass::DisabledByPolicy
    );
    assert!(sync.kill_switches.iter().any(|kill| kill.source_class
        == M5KillSwitchSourceClass::AdminPolicyCeiling
        && kill.active));

    let docs = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:docs_browser.open_external")
        .expect("docs row must exist");
    assert_eq!(
        docs.effective_state_class,
        M5RolloutStateClass::RetestPending
    );
    assert_eq!(docs.promotion_state, M5PromotionStateClass::RetestRequired);
}

#[test]
fn active_kill_switches_follow_source_precedence() {
    let packet = seeded_m5_rollout_inventory_packet();
    let sync = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:sync.push_workspace_state")
        .expect("sync row must exist");

    let active_sources = sync
        .active_kill_switches()
        .into_iter()
        .map(|kill| kill.source_class)
        .collect::<Vec<_>>();

    assert_eq!(
        active_sources,
        vec![
            M5KillSwitchSourceClass::AdminPolicyCeiling,
            M5KillSwitchSourceClass::UserOptInOrLocalPreviewToggle,
        ]
    );
}

#[test]
fn support_export_quotes_rollout_refs() {
    let packet = seeded_m5_rollout_inventory_packet();
    let export = M5RolloutInventorySupportExport::from_packet(
        M5_ROLLOUT_INVENTORY_SUPPORT_EXPORT_ID.to_string(),
        packet.clone(),
    );

    assert_eq!(export.packet, packet);
    for row in &export.packet.rows {
        assert!(export.case_ids.contains(&row.command_id));
        assert!(export.case_ids.contains(&row.capability_id));
        assert!(export.case_ids.contains(&row.owner_ref));
        assert!(export.case_ids.contains(&row.rollout_state_ref));
        for affected in &row.affected_capability_ids {
            assert!(export.case_ids.contains(affected));
        }
        for kill in &row.kill_switches {
            assert!(export.case_ids.contains(&kill.source_ref));
        }
    }
}

#[test]
fn published_packet_artifact_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_rollout_inventory/packet.json");
    let on_disk = std::fs::read_to_string(&path).expect("packet artifact must exist");
    let rendered =
        serde_json::to_string_pretty(&seeded_m5_rollout_inventory_packet()).expect("serializes");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_fixture_packet_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/m5_rollout_inventory/packet.json");
    let on_disk = std::fs::read_to_string(&path).expect("packet fixture must exist");
    let rendered =
        serde_json::to_string_pretty(&seeded_m5_rollout_inventory_packet()).expect("serializes");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_support_export_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_rollout_inventory/support_export.json");
    let on_disk = std::fs::read_to_string(&path).expect("support export must exist");
    let rendered = serde_json::to_string_pretty(&M5RolloutInventorySupportExport::from_packet(
        M5_ROLLOUT_INVENTORY_SUPPORT_EXPORT_ID.to_string(),
        seeded_m5_rollout_inventory_packet(),
    ))
    .expect("serializes");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_summary_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_rollout_inventory/summary.md");
    let on_disk = std::fs::read_to_string(&path).expect("summary artifact must exist");
    assert_eq!(
        on_disk,
        seeded_m5_rollout_inventory_packet().render_markdown()
    );
}
