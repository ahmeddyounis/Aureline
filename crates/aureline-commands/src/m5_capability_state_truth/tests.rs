use super::*;
use std::path::Path;

#[test]
fn seeded_packet_covers_every_required_state_definition() {
    let packet = seeded_m5_capability_state_truth_packet();
    for required in M5CapabilityStateClass::required_coverage() {
        assert!(packet
            .state_definitions
            .iter()
            .any(|definition| definition.state_class == required));
    }
}

#[test]
fn seeded_packet_covers_every_required_projection_surface() {
    let packet = seeded_m5_capability_state_truth_packet();
    for row in &packet.rows {
        for required in M5CapabilityProjectionSurfaceClass::required_coverage() {
            assert!(
                row.projection_rows
                    .iter()
                    .any(|projection| projection.surface_class == required),
                "{} missing {}",
                row.command_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn seeded_packet_passes_validation() {
    let packet = current_m5_capability_state_truth_export()
        .expect("seeded capability-state truth export must validate");
    assert_eq!(packet.packet_id, M5_CAPABILITY_STATE_TRUTH_PACKET_ID);
}

#[test]
fn claim_surfaces_preserve_dependency_markers_for_narrowed_rows() {
    let packet = seeded_m5_capability_state_truth_packet();
    for row in &packet.rows {
        if row.dependency_markers.is_empty() {
            continue;
        }
        for projection in row
            .projection_rows
            .iter()
            .filter(|projection| projection.surface_class.claim_surface())
        {
            assert!(
                projection.dependency_markers_visible,
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
            assert!(
                !projection.dependency_marker_refs.is_empty(),
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
        }
    }
}

#[test]
fn stable_facing_surfaces_do_not_overclaim_stable_wording() {
    let packet = seeded_m5_capability_state_truth_packet();
    for row in &packet.rows {
        for projection in row
            .projection_rows
            .iter()
            .filter(|projection| projection.surface_class.stable_facing())
        {
            assert!(
                !projection.stable_wording_visible,
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
        }
    }
}

#[test]
fn retest_pending_and_policy_disabled_rows_narrow_support_wording() {
    let packet = seeded_m5_capability_state_truth_packet();
    for row in packet.rows.iter().filter(|row| {
        matches!(
            row.effective_state_class,
            M5CapabilityStateClass::DisabledByPolicy | M5CapabilityStateClass::RetestPending
        )
    }) {
        for projection in row
            .projection_rows
            .iter()
            .filter(|projection| projection.surface_class.claim_surface())
        {
            assert!(
                !projection.support_wording_visible,
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
        }
    }
}

#[test]
fn desktop_cli_extension_and_browser_surfaces_keep_lifecycle_inspectable() {
    let packet = seeded_m5_capability_state_truth_packet();
    for row in &packet.rows {
        for projection in row
            .projection_rows
            .iter()
            .filter(|projection| projection.surface_class.inspection_surface())
        {
            assert!(
                !projection.inspect_detail_ref.is_empty(),
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
            assert!(
                !projection.route_or_metadata_ref.is_empty(),
                "{} {}",
                row.command_id,
                projection.surface_class.as_str()
            );
        }
    }
}

#[test]
fn support_export_quotes_state_detail_marker_and_projection_refs() {
    let packet = seeded_m5_capability_state_truth_packet();
    let export = M5CapabilityStateTruthSupportExport::from_packet(
        M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_ID.to_string(),
        packet.clone(),
    );

    assert_eq!(export.packet, packet);
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.capability_row_id));
        assert!(export.case_ids.contains(&row.lifecycle_ref));
        assert!(export.case_ids.contains(&row.rollout_state_ref));
        for marker in &row.dependency_markers {
            assert!(export.case_ids.contains(&marker.marker_ref));
            assert!(export.case_ids.contains(&marker.dependency_ref));
            assert!(export.case_ids.contains(&marker.detail_ref));
        }
        for projection in &row.projection_rows {
            assert!(export.case_ids.contains(&projection.projection_ref));
            assert!(export.case_ids.contains(&projection.inspect_detail_ref));
            assert!(export.case_ids.contains(&projection.route_or_metadata_ref));
        }
    }
}

#[test]
fn published_fixture_packet_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/m5_capability_state_truth/packet.json");
    let on_disk = std::fs::read_to_string(&path).expect("packet fixture must exist");
    let seeded = seeded_m5_capability_state_truth_packet();
    let rendered = serde_json::to_string_pretty(&seeded).expect("packet must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_support_export_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_capability_state_truth/support_export.json");
    let on_disk = std::fs::read_to_string(&path).expect("support export must exist");
    let export = M5CapabilityStateTruthSupportExport::from_packet(
        M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_ID.to_string(),
        seeded_m5_capability_state_truth_packet(),
    );
    let rendered = serde_json::to_string_pretty(&export).expect("support export must serialize");
    assert_eq!(on_disk.trim(), rendered);
}

#[test]
fn published_summary_matches_seed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/commands/m5_capability_state_truth/summary.md");
    let on_disk = std::fs::read_to_string(&path).expect("summary artifact must exist");
    let rendered = seeded_m5_capability_state_truth_packet().render_markdown();
    assert_eq!(on_disk, rendered);
}
