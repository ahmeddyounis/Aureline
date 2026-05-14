use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use aureline_auth::{ContinuityStateClass, SecretBrokerAlphaPacket};
use aureline_shell::desktop_continuity_alpha::{
    seeded_desktop_continuity_alpha_packet, DesktopContinuitySupportExport,
};

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("repo root")
}

#[test]
fn packet_covers_required_os_entry_surfaces_with_literal_target_and_owner_truth() {
    let packet = seeded_desktop_continuity_alpha_packet("build:test:desktop-continuity");
    packet.validate().expect("packet validates");

    let sources = packet
        .entry_rows
        .iter()
        .map(|row| row.source_surface_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "system_open",
        "file_association",
        "default_browser_callback",
        "dock_taskbar_recent",
        "dock_taskbar_jump_action",
    ] {
        assert!(sources.contains(required), "missing {required}");
    }

    for row in &packet.entry_rows {
        assert!(!row.literal_target_label.is_empty());
        assert!(!row.resulting_mode_token.is_empty());
        assert!(!row.command_id_ref.is_empty());
        assert!(!row.object_identity_ref.is_empty());
        assert!(!row.trust_profile_boundary_token.is_empty());
        assert!(
            row.owning_channel_ref.is_some()
                || row.source_surface_token == "default_browser_callback"
        );
        assert!(
            row.owner_build_ref.is_some() || row.source_surface_token == "default_browser_callback"
        );
    }

    let file_association = packet
        .entry_rows
        .iter()
        .find(|row| row.source_surface_token == "file_association")
        .expect("file association row");
    assert_eq!(file_association.resulting_mode_token, "local_file_open");
    assert!(file_association.direct_os_execution_forbidden);
    assert_eq!(
        file_association.review_surface_token,
        "product_owned_native_review"
    );

    let auth_callback = packet
        .entry_rows
        .iter()
        .find(|row| row.source_surface_token == "default_browser_callback")
        .expect("auth callback row");
    assert_eq!(auth_callback.resulting_mode_token, "auth_callback");
    assert!(!auth_callback.execution_allowed);
    assert!(auth_callback.direct_os_execution_forbidden);
}

#[test]
fn missing_targets_and_removable_paths_degrade_to_recovery_cards() {
    let packet = seeded_desktop_continuity_alpha_packet("build:test:desktop-continuity");

    let recent = packet
        .entry_rows
        .iter()
        .find(|row| row.source_surface_token == "dock_taskbar_recent")
        .expect("dock recent row");
    assert_eq!(recent.availability_class_token, "missing_or_unmounted");
    assert_eq!(recent.review_surface_token, "placeholder_recovery_card");
    assert!(recent.placeholder_recovery_required);
    for expected in ["locate", "open_cached_context", "close_placeholder"] {
        assert!(
            recent
                .recovery_action_tokens
                .iter()
                .any(|token| token == expected),
            "missing recovery action {expected}"
        );
    }

    let removable = packet
        .target_recovery_rows
        .iter()
        .find(|row| row.target_kind_token == "removable_volume")
        .expect("removable target recovery row");
    assert_eq!(removable.availability_class_token, "missing_or_unmounted");
    assert!(removable.preserves_user_intent);
    assert!(removable
        .recovery_action_tokens
        .iter()
        .any(|token| token == "open_cached_context"));
}

#[test]
fn topology_sleep_and_network_rows_keep_visible_focus_and_block_hidden_rerun() {
    let packet = seeded_desktop_continuity_alpha_packet("build:test:desktop-continuity");

    assert!(packet.topology_rows.iter().any(|row| {
        row.event_class_token == "mixed_dpi_cross_monitor_reflow"
            && row
                .topology_change_tokens
                .contains(&"scale_changed".to_owned())
            && row.resulting_fidelity_token == "compatible_restore"
            && row.visible_bounds_preserved
            && row.focus_intent_preserved
            && row.topology_adjustment_downgraded_fidelity
    }));
    assert!(packet.topology_rows.iter().any(|row| {
        row.event_class_token == "fullscreen_or_snapped_restore"
            && row
                .adjustment_tokens
                .contains(&"fullscreen_cleared".to_owned())
            && row.resulting_fidelity_token == "compatible_restore"
    }));

    let wake = packet
        .lifecycle_rows
        .iter()
        .find(|row| row.lifecycle_event_token == "wake_from_sleep")
        .expect("wake row");
    assert!(wake
        .continuity_state_tokens
        .contains(&"reconnecting".to_owned()));
    assert!(wake
        .continuity_state_tokens
        .contains(&"resume_review_needed".to_owned()));
    assert!(wake.no_silent_rerun_or_authority_reuse);
    assert!(wake.privileged_or_mutating_work_paused);

    let network = packet
        .lifecycle_rows
        .iter()
        .find(|row| row.lifecycle_event_token == "network_transition")
        .expect("network row");
    assert!(network
        .continuity_state_tokens
        .contains(&"local_fallback".to_owned()));
    assert!(network.local_work_continues);
}

#[test]
fn credential_store_interruptions_are_visible_and_metadata_only() {
    let packet = seeded_desktop_continuity_alpha_packet("build:test:desktop-continuity");
    let states = packet
        .credential_store_rows
        .iter()
        .map(|row| row.continuity_state_token.as_str())
        .collect::<BTreeSet<_>>();
    assert!(states.contains("paused_credential_store_locked"));
    assert!(states.contains("paused_credential_store_unavailable"));
    assert!(states.contains("paused_trust_store_changed"));

    for row in &packet.credential_store_rows {
        assert!(!row.affected_capability_tokens.is_empty());
        assert!(row.local_work_continues);
        assert!(row.credentialed_actions_paused);
        assert!(row.plaintext_downgrade_forbidden);
        assert!(row.stale_authority_reuse_forbidden);
        assert!(row.denial_reason_token.is_some());
        assert!(row
            .recovery_action_tokens
            .iter()
            .any(|token| token == "export_support_metadata"));
    }

    let fixture_path = repo_root()
        .join("fixtures/auth/secret_broker_alpha/failure_locked_unavailable_trust_changed.json");
    let fixture: SecretBrokerAlphaPacket =
        serde_json::from_str(&fs::read_to_string(fixture_path).expect("fixture reads"))
            .expect("secret broker fixture parses");
    fixture.validate().expect("secret broker fixture validates");
    assert!(fixture
        .rows
        .iter()
        .any(|row| row.continuity.continuity_state
            == ContinuityStateClass::PausedCredentialStoreLocked));
}

#[test]
fn support_export_reconstructs_interruption_without_ui_text_scrape() {
    let packet = seeded_desktop_continuity_alpha_packet("build:test:desktop-continuity");
    let export = packet.support_export(
        "support.desktop_continuity.alpha.test",
        "2026-05-14T00:20:00Z",
    );

    assert!(export.redaction_safe());
    assert!(export.reconstructs_interruption_cause);
    assert!(export.reconstructs_continuity_state);
    assert!(export.reconstructs_recovery_choice);
    assert!(export.reconstructs_resulting_fidelity);
    assert!(!export.ui_text_scrape_required);

    let families = export
        .rows
        .iter()
        .map(|row| row.row_family_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "os_entry",
        "target_recovery",
        "display_topology",
        "lifecycle_interruption",
        "credential_store_interruption",
    ] {
        assert!(families.contains(required), "missing family {required}");
    }
    assert!(export.rows.iter().all(|row| {
        !row.interruption_cause_token.is_empty()
            && !row.continuity_state_tokens.is_empty()
            && !row.recovery_choice_tokens.is_empty()
            && !row.resulting_fidelity_token.is_empty()
            && !row.support_field_refs.is_empty()
            && !row.raw_secret_values_exported
            && !row.raw_handle_ids_exported
    }));
}

#[test]
fn governed_artifact_and_fixtures_round_trip() {
    let matrix_path = repo_root().join("artifacts/ux/desktop_continuity_alpha_matrix.yaml");
    let matrix: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(matrix_path).expect("matrix reads"))
            .expect("matrix parses");
    assert_eq!(
        matrix["runtime_packet"]["record_kind"].as_str(),
        Some("desktop_continuity_alpha_packet_record")
    );
    assert_eq!(
        matrix["support_export_reconstruction"]["forbidden"][0].as_str(),
        Some("raw_secret_values_exported")
    );

    let support_fixture_path =
        repo_root().join("fixtures/ux/desktop_continuity_alpha/support_export_reconstruction.json");
    let support_fixture: DesktopContinuitySupportExport =
        serde_json::from_str(&fs::read_to_string(support_fixture_path).expect("fixture reads"))
            .expect("support export fixture parses");
    assert!(support_fixture.redaction_safe());
    assert!(support_fixture
        .rows
        .iter()
        .any(|row| row.row_family_token == "credential_store_interruption"));

    let workspace_fixture_path = repo_root()
        .join("fixtures/workspace/desktop_continuity_alpha/mixed_dpi_restore_visible.json");
    let workspace_fixture: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(workspace_fixture_path).expect("fixture reads"))
            .expect("workspace fixture parses");
    assert_eq!(
        workspace_fixture["resulting_fidelity"].as_str(),
        Some("compatible_restore")
    );
    assert_eq!(
        workspace_fixture["visible_bounds_preserved"].as_bool(),
        Some(true)
    );

    let auth_projection_path = repo_root().join(
        "fixtures/auth/credential_store_interruption_alpha/interruption_support_projection.json",
    );
    let auth_projection: DesktopContinuitySupportExport =
        serde_json::from_str(&fs::read_to_string(auth_projection_path).expect("fixture reads"))
            .expect("auth support projection fixture parses");
    assert!(auth_projection.redaction_safe());
    assert!(auth_projection
        .rows
        .iter()
        .all(|row| row.row_family_token == "credential_store_interruption"));
}
