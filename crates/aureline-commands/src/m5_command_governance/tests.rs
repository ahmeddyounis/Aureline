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
    assert!(browser.route_provenance.handoff_packet_ref.is_some());
    assert_eq!(
        browser.route_provenance.handoff_reason_class.as_deref(),
        Some("desktop_required")
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
    assert!(
        preview
            .copy_safe_introspection
            .inspect_why_not_automatable
            .available
    );
    assert!(preview
        .copy_safe_introspection
        .inspect_why_not_automatable
        .detail_ref
        .as_deref()
        .is_some_and(|value| value.contains("approval_required")));
}

#[test]
fn seeded_packet_exposes_origin_lifecycle_and_alias_disclosure() {
    let packet = seeded_m5_command_governance_packet();
    let docs_browser = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:docs_browser.open_external")
        .expect("docs browser row must exist");
    assert_eq!(
        docs_browser.origin_disclosure.origin_class,
        "built_in_extension"
    );
    assert_eq!(
        docs_browser.origin_disclosure.source_display_label,
        "Extension"
    );
    assert_eq!(docs_browser.lifecycle_disclosure.stability_label, "Beta");
    assert_eq!(docs_browser.alias_records.len(), 1);
    assert_eq!(docs_browser.alias_records[0].alias_state, "active");
    assert!(docs_browser.surface_rows.iter().all(|surface| surface
        .preview_parity
        .copy_safe_introspection
        .inspect_origin
        .available));
}

#[test]
fn seeded_packet_exposes_copy_safe_cli_and_recipe_templates_when_supported() {
    let packet = seeded_m5_command_governance_packet();
    let scaffold = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:template_scaffold.scaffold_project")
        .expect("scaffold row must exist");
    let actions = &scaffold.surface_rows[0]
        .preview_parity
        .copy_safe_introspection;
    assert!(actions.copy_command_id.available);
    assert_eq!(
        actions.copy_command_id.value.as_deref(),
        Some("cmd:template_scaffold.scaffold_project")
    );
    assert!(actions.copy_cli_form.available);
    assert!(actions
        .copy_cli_form
        .value
        .as_deref()
        .is_some_and(|value| value.starts_with("aureline template_scaffold scaffold_project")));
    assert!(actions.copy_recipe_step.available);
    assert!(actions.copy_recipe_step.value.as_deref().is_some_and(
        |value| value.contains("\"command_id\":\"cmd:template_scaffold.scaffold_project\"")
    ));
}

#[test]
fn seeded_packet_covers_every_canonical_result_outcome() {
    let packet = seeded_m5_command_governance_packet();
    for row in &packet.rows {
        for required in M5ResultOutcomeClass::required_coverage() {
            assert!(
                row.result_packet_governance
                    .outcome_rows
                    .iter()
                    .any(|outcome| outcome.outcome_class == required),
                "{} missing {}",
                row.command_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn durable_or_mutating_rows_preserve_result_export_truth() {
    let packet = seeded_m5_command_governance_packet();
    for row in &packet.rows {
        let result = &row.result_packet_governance;
        if result.durable_truth_required {
            assert!(result.preserves_copy_safe_summary, "{}", row.command_id);
            assert!(result.preserves_raw_packet_export, "{}", row.command_id);
            assert!(result.joins_support_export, "{}", row.command_id);
            assert!(result.joins_release_evidence, "{}", row.command_id);
        }
    }
}

#[test]
fn activity_joined_rows_reuse_the_m5_activity_contract() {
    let packet = seeded_m5_command_governance_packet();
    for row in packet
        .rows
        .iter()
        .filter(|row| row.result_packet_governance.joins_activity_center)
    {
        assert_eq!(
            row.result_packet_governance
                .activity_shared_contract_ref
                .as_deref(),
            Some("shell:m5_activity_objects:v1"),
            "{}",
            row.command_id
        );
        assert!(row
            .result_packet_governance
            .artifacts
            .activity_join_ref
            .as_deref()
            .is_some_and(|value| value.starts_with("activity:")));
        assert!(row
            .result_packet_governance
            .artifacts
            .exact_target_reopen_ref
            .as_deref()
            .is_some_and(|value| value.starts_with("activity:reopen:")));
    }
}

#[test]
fn rollback_and_checkpoint_requirements_follow_preview_gate_posture() {
    let packet = seeded_m5_command_governance_packet();

    let offboarding = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:offboarding.export_and_wipe")
        .expect("offboarding row must exist");
    assert!(
        offboarding
            .result_packet_governance
            .artifacts
            .rollback_handle_required
    );
    assert!(
        offboarding
            .result_packet_governance
            .artifacts
            .checkpoint_ref_required
    );

    let sync = packet
        .rows
        .iter()
        .find(|row| row.command_id == "cmd:sync.push_workspace_state")
        .expect("sync row must exist");
    assert!(
        !sync
            .result_packet_governance
            .artifacts
            .rollback_handle_required
    );
    assert!(
        sync.result_packet_governance
            .artifacts
            .checkpoint_ref_required
    );
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
        assert!(export.case_ids.contains(&row.capability_class_ref));
        assert!(export
            .case_ids
            .contains(&row.lifecycle_disclosure.lifecycle_ref));
        assert!(export
            .case_ids
            .contains(&row.origin_disclosure.runtime_origin_ref));
        for outcome in &row.result_packet_governance.outcome_rows {
            assert!(export.case_ids.contains(&outcome.export_safe_summary_ref));
            assert!(export.case_ids.contains(&outcome.raw_packet_export_ref));
            assert!(export.case_ids.contains(&outcome.support_export_case_ref));
            assert!(export.case_ids.contains(&outcome.release_evidence_ref));
        }
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
