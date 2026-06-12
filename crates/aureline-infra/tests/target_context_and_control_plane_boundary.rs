//! Integration coverage for infrastructure target-context boundary packets.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    ConnectorClass, InfraBoundaryFindingSeverity, InfraBoundaryPacket, QualificationPosture,
    StateClass, SurfaceKind, TruthClass, CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND,
    CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION,
};

#[test]
fn checked_in_fixture_validates() {
    let packet = load_fixture("qualified_context_parity_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(
        packet.record_kind,
        CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND
    );
    assert_eq!(packet.schema_version, CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION);
}

#[test]
fn fixture_covers_connector_truth_state_and_surface_classes() {
    let report = load_fixture("qualified_context_parity_packet.json").validate();

    for required in [
        ConnectorClass::StaticFileOnly,
        ConnectorClass::CliMediated,
        ConnectorClass::AgentMediatedLive,
        ConnectorClass::ProviderConsoleOverlay,
    ] {
        assert!(report.connector_classes.contains(&required));
    }

    for required in [
        TruthClass::Desired,
        TruthClass::Rendered,
        TruthClass::Planned,
        TruthClass::Observed,
        TruthClass::Cached,
        TruthClass::PermissionLimited,
        TruthClass::Unavailable,
        TruthClass::ProviderOverlay,
    ] {
        assert!(report.truth_classes.contains(&required));
    }

    for required in [
        StateClass::Desired,
        StateClass::Rendered,
        StateClass::Planned,
        StateClass::Observed,
        StateClass::Cached,
        StateClass::PermissionLimited,
        StateClass::Unavailable,
    ] {
        assert!(report.state_classes.contains(&required));
    }
}

#[test]
fn every_claimed_surface_uses_same_context_chip() {
    let packet = load_fixture("qualified_context_parity_packet.json");
    let surfaces: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.surface)
        .collect();
    for required in [
        SurfaceKind::Terminal,
        SurfaceKind::Logs,
        SurfaceKind::ResourceGraph,
        SurfaceKind::IncidentWorkspace,
        SurfaceKind::AiActionSheet,
        SurfaceKind::CliJson,
        SurfaceKind::BrowserConsoleHandoff,
        SurfaceKind::SupportExport,
    ] {
        assert!(surfaces.contains(&required));
    }

    for binding in &packet.surface_bindings {
        assert!(binding.uses_shared_packet);
        assert_eq!(
            binding.target_chip.context_ref,
            packet.environment_context.context_id
        );
    }
}

#[test]
fn stale_and_wrong_target_drills_downgrade_or_block() {
    let stale = load_fixture("stale_live_overlay_downgraded_packet.json");
    let wrong = load_fixture("wrong_target_action_blocked_packet.json");

    for binding in &stale.surface_bindings {
        assert_ne!(
            binding.qualification_posture,
            QualificationPosture::StableQualified
        );
    }
    assert!(stale.validate().passed);
    assert!(!wrong.validate().passed);
    assert!(wrong.validate().findings.iter().any(|finding| {
        finding.severity == InfraBoundaryFindingSeverity::Error && finding.check_id == "target_chip"
    }));
}

#[test]
fn boundary_actions_show_duration_scope_and_revocation() {
    let packet = load_fixture("qualified_context_parity_packet.json");
    for review in &packet.action_reviews {
        if review.action_kind.raises_boundary() {
            assert!(review.duration.is_some());
            assert!(review.credential_scope.is_some());
            assert!(review.revocation_path.is_some());
        }
    }
}

#[test]
fn packet_projects_m5_secret_boundary_state() {
    let packet = load_fixture("qualified_context_parity_packet.json");
    let states = packet.secret_boundary_states();
    assert_eq!(states.len(), 1);
    assert_eq!(
        states[0].matrix_row_id,
        "m5.secret.infra_connector.target_context"
    );
    assert!(states[0].delegated_credential_row.is_some());
    assert_eq!(
        states[0]
            .consumer_identity_receipt
            .consumer_identity
            .as_str(),
        "cluster_connector"
    );
    assert!(!states[0].repairable_states.is_empty());
    assert!(!states[0]
        .projection_mode_audit
        .available_controls
        .is_empty());
    assert!(!states[0].export_safety_banner.raw_secret_values_included);
}

fn load_fixture(name: &str) -> InfraBoundaryPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/target-context-and-control-plane-boundary")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
