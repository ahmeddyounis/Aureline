use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use aureline_shell::platform_integration::{
    seeded_native_desktop_contract_packet_with_time, DesktopEntryEvent,
    NativeDesktopContractPacket, DESKTOP_ENTRY_EVENT_RECORD_KIND,
    NATIVE_DESKTOP_CONTRACT_PACKET_RECORD_KIND,
};

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("repo root")
}

#[test]
fn packet_covers_all_required_native_entry_surfaces() {
    let packet = seeded_native_desktop_contract_packet_with_time(
        "build:aureline:beta:desktop-contract",
        "2026-05-18T00:00:00Z",
    );
    packet
        .validate()
        .expect("native desktop contract validates");

    let surfaces = packet
        .desktop_entry_events
        .iter()
        .map(|event| event.source_surface_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "system_open",
        "file_association",
        "default_browser_callback",
        "protocol_handler",
        "dock_taskbar_recent",
        "dock_taskbar_jump_action",
        "os_notification_click",
        "os_badge_activation",
        "reveal_in_system_shell",
    ] {
        assert!(surfaces.contains(required), "missing {required}");
    }

    for event in &packet.desktop_entry_events {
        assert_eq!(event.record_kind, DESKTOP_ENTRY_EVENT_RECORD_KIND);
        assert!(!event.literal_target_label.is_empty());
        assert!(!event.canonical_target_ref.is_empty());
        assert!(!event.owning_channel_ref.is_empty());
        assert!(!event.owner_build_ref.is_empty());
        assert!(!event.requested_action_class_token.is_empty());
        assert!(event.exact_target_or_truthful_placeholder());
        assert!(event.no_silent_mutating_replay);
        assert!(event.raw_private_material_excluded);
    }
}

#[test]
fn notification_and_badge_reopen_are_privacy_safe_and_exact() {
    let packet = seeded_native_desktop_contract_packet_with_time(
        "build:aureline:beta:desktop-contract",
        "2026-05-18T00:00:00Z",
    );

    for surface in ["os_notification_click", "os_badge_activation"] {
        let event = packet
            .desktop_entry_events
            .iter()
            .find(|event| event.source_surface_token == surface)
            .unwrap_or_else(|| panic!("missing {surface} event"));
        assert_eq!(event.route_class_token, "notification_reopen");
        assert_eq!(event.availability_class_token, "exact_available");
        assert!(event.direct_os_execution_forbidden);
        assert!(event.lock_screen_payload_redacted);
        assert!(event.notification_summary_bounded);
        assert!(event.badge_or_progress_count_traceable);
        assert!(event.summary_surface_safe());
        assert_ne!(event.canonical_target_ref, "generic_home");
    }
}

#[test]
fn protocol_auth_and_recent_paths_do_not_bypass_review_or_recovery() {
    let packet = seeded_native_desktop_contract_packet_with_time(
        "build:aureline:beta:desktop-contract",
        "2026-05-18T00:00:00Z",
    );

    let protocol = event(&packet, "protocol_handler");
    assert_eq!(
        protocol.requested_action_class_token,
        "privileged_authority_widening"
    );
    assert!(protocol.authority_widening_review_required);
    assert!(protocol.direct_os_execution_forbidden);
    assert_eq!(
        protocol.recovery_surface_token,
        "product_owned_native_review"
    );

    let auth = event(&packet, "default_browser_callback");
    assert_eq!(auth.availability_class_token, "expired");
    assert!(auth.placeholder_recovery_required);
    assert!(auth
        .recovery_action_tokens
        .iter()
        .any(|token| token == "restart_browser_handoff"));

    let recent = event(&packet, "dock_taskbar_recent");
    assert_eq!(recent.availability_class_token, "missing_or_unmounted");
    assert!(recent.placeholder_recovery_required);
    for expected in ["locate", "open_cached_context", "close_placeholder"] {
        assert!(
            recent
                .recovery_action_tokens
                .iter()
                .any(|token| token == expected),
            "missing recent recovery action {expected}"
        );
    }
}

#[test]
fn recovery_rows_cover_interruption_and_unavailable_target_truth() {
    let packet = seeded_native_desktop_contract_packet_with_time(
        "build:aureline:beta:desktop-contract",
        "2026-05-18T00:00:00Z",
    );
    let classes = packet
        .recovery_rows
        .iter()
        .map(|row| row.interruption_class_token.as_str())
        .collect::<BTreeSet<_>>();

    for required in [
        "removable_volume_loss",
        "removable_volume_return",
        "network_share_unavailable",
        "missing_root",
        "credential_store_locked",
        "display_topology_drift",
        "wake_resume",
        "sleep_expired_callback",
        "network_transition",
    ] {
        assert!(classes.contains(required), "missing recovery {required}");
    }

    for row in &packet.recovery_rows {
        assert!(!row.recovery_action_tokens.is_empty());
        assert!(row.no_silent_replay_or_authority_reuse);
        assert!(row.privileged_or_mutating_work_paused);
        assert!(!row.source_evidence_refs.is_empty());
    }
}

#[test]
fn drills_and_support_matrix_have_current_platform_proof() {
    let packet = seeded_native_desktop_contract_packet_with_time(
        "build:aureline:beta:desktop-contract",
        "2026-05-18T00:00:00Z",
    );

    let profiles = packet
        .platform_drills
        .iter()
        .map(|row| row.platform_profile_id.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "macos_15_plus_universal",
        "windows_11_23h2_plus_x86_64",
        "linux_ubuntu_24_04_gnome_wayland_x86_64",
    ] {
        assert!(profiles.contains(required), "missing platform {required}");
    }

    let drill_classes = packet
        .platform_drills
        .iter()
        .map(|row| row.drill_class_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "channel_precedence",
        "handler_spoof_resistance",
        "recent_reopen_fidelity",
        "lock_screen_redaction",
        "wake_resume_truth",
    ] {
        assert!(drill_classes.contains(required), "missing drill {required}");
    }
    assert!(packet.platform_drills.iter().all(|row| {
        row.current_proof
            && row.no_silent_mutating_replay
            && !row.required_disclosure_tokens.is_empty()
    }));

    let surfaces = packet
        .support_matrix_rows
        .iter()
        .map(|row| row.surface_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "system_open",
        "auth_callbacks",
        "file_associations",
        "recent_items",
        "privacy_safe_native_notifications",
    ] {
        assert!(
            surfaces.contains(required),
            "missing support row {required}"
        );
    }
}

#[test]
fn schema_fixture_docs_and_report_are_present() {
    let schema_path = repo_root().join("schemas/platform/desktop_entry_event.schema.json");
    let schema: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(schema_path).expect("schema reads"))
            .expect("schema parses");
    assert_eq!(
        schema["$id"].as_str(),
        Some("https://aureline.dev/schemas/platform/desktop_entry_event.schema.json")
    );

    let fixture_path =
        repo_root().join("fixtures/platform/m3/native_desktop_contract/entry_events.json");
    let fixture_events: Vec<DesktopEntryEvent> =
        serde_json::from_str(&fs::read_to_string(fixture_path).expect("fixture reads"))
            .expect("fixture parses");
    assert!(fixture_events
        .iter()
        .any(|event| event.source_surface_token == "os_notification_click"));
    assert!(fixture_events
        .iter()
        .all(DesktopEntryEvent::exact_target_or_truthful_placeholder));

    let packet_fixture_path =
        repo_root().join("fixtures/platform/m3/native_desktop_contract/packet_summary.json");
    let packet_fixture: NativeDesktopContractPacket =
        serde_json::from_str(&fs::read_to_string(packet_fixture_path).expect("packet reads"))
            .expect("packet parses");
    assert_eq!(
        packet_fixture.record_kind,
        NATIVE_DESKTOP_CONTRACT_PACKET_RECORD_KIND
    );
    packet_fixture.validate().expect("packet fixture validates");

    for path in [
        "docs/platform/m3/native_desktop_beta_contract.md",
        "artifacts/platform/m3/native_desktop_integration_report.md",
    ] {
        let body = fs::read_to_string(repo_root().join(path)).expect("doc/report reads");
        assert!(
            body.contains("system open")
                && body.contains("auth callback")
                && body.contains("privacy-safe native notification"),
            "{path} must name required beta proof surfaces"
        );
    }
}

fn event<'a>(packet: &'a NativeDesktopContractPacket, surface: &str) -> &'a DesktopEntryEvent {
    packet
        .desktop_entry_events
        .iter()
        .find(|event| event.source_surface_token == surface)
        .unwrap_or_else(|| panic!("missing {surface} event"))
}
