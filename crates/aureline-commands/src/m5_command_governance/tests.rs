use super::*;
use std::path::Path;

#[test]
fn seeded_packet_covers_every_required_surface() {
    let packet = seeded_m5_command_governance_packet();
    for row in &packet.rows {
        for required in M5GovernanceSurfaceClass::required_coverage() {
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
    let packet = current_m5_command_governance_export()
        .expect("seeded M5 command-governance export must validate");
    assert_eq!(packet.packet_id, M5_COMMAND_GOVERNANCE_PACKET_ID);
}

#[test]
fn seeded_packet_preserves_actor_target_trust_and_rollout_lineage() {
    let packet = seeded_m5_command_governance_packet();
    for row in &packet.rows {
        for surface in &row.surface_rows {
            let approval = &surface.approval_parity_packet;
            assert!(!approval.actor_ref.is_empty(), "{}", row.command_id);
            assert!(!approval.target_ref.is_empty(), "{}", row.command_id);
            assert!(!approval.trust_epoch_ref.is_empty(), "{}", row.command_id);
            assert!(!approval.rollout_state_ref.is_empty(), "{}", row.command_id);

            for packet in &surface.disabled_reason_packets {
                assert!(!packet.actor_ref.is_empty(), "{}", row.command_id);
                assert!(!packet.target_ref.is_empty(), "{}", row.command_id);
                assert!(!packet.trust_epoch_ref.is_empty(), "{}", row.command_id);
                assert!(!packet.rollout_state_ref.is_empty(), "{}", row.command_id);
            }
        }
    }
}

#[test]
fn seeded_packet_routes_browser_companion_to_handoff_when_surface_is_not_supported() {
    let packet = seeded_m5_command_governance_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:trace_replay.replay_session")
        .expect("trace replay row must exist");
    let browser = row
        .surface_rows
        .iter()
        .find(|surface| surface.surface_class == M5GovernanceSurfaceClass::BrowserCompanion)
        .expect("browser companion row must exist");
    assert_eq!(
        browser.route_posture_class,
        M5RoutePostureClass::DesktopHandoffRequired
    );
}

#[test]
fn seeded_packet_keeps_copy_safe_why_not_automatable_reason_in_sync() {
    let packet = seeded_m5_command_governance_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:secret_broker.open_credential_rotation")
        .expect("secret broker rotation row must exist");
    let preview = &row.surface_rows[0].preview_parity;
    assert_eq!(
        preview.why_not_automatable_reason.as_deref(),
        Some("approval_required")
    );
    assert!(preview.copy_safe_introspection.inspect_why_not_automatable);
}

#[test]
fn support_export_quotes_packet_and_command_ids() {
    let packet = seeded_m5_command_governance_packet();
    let export = M5CommandGovernanceSupportExport::from_packet(
        M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID.to_string(),
        packet.clone(),
    );

    assert_eq!(export.packet, packet);
    assert!(export.case_ids.contains(&packet.packet_id));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.command_id));
        assert!(export.case_ids.contains(&row.command_revision_ref));
    }
}

#[test]
fn published_fixture_packet_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/m5_command_governance/packet.json");
    let on_disk = std::fs::read_to_string(&path).expect("packet fixture must exist");
    let seeded = seeded_m5_command_governance_packet();
    let rendered = serde_json::to_string_pretty(&seeded).expect("packet must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_support_export_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_command_governance/support_export.json");
    let on_disk = std::fs::read_to_string(&path).expect("support export must exist");
    let export = M5CommandGovernanceSupportExport::from_packet(
        M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID.to_string(),
        seeded_m5_command_governance_packet(),
    );
    let rendered = serde_json::to_string_pretty(&export).expect("support export must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_summary_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_command_governance/summary.md");
    let on_disk = std::fs::read_to_string(&path).expect("summary artifact must exist");
    let rendered = seeded_m5_command_governance_packet().render_markdown();
    assert_eq!(on_disk, rendered);
}
