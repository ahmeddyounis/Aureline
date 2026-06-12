use std::fs;
use std::path::{Path, PathBuf};

use super::*;

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m5/m5-secret-boundary-depth")
}

fn load_packet_fixture() -> M5SecretBoundaryDepthPacket {
    let path = fixture_dir().join("canonical_packet.json");
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture canonical_packet.json must parse: {err}"))
}

fn load_support_export_fixture() -> SecretBoundarySupportExport {
    let path = fixture_dir().join("support_export.json");
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture support_export.json must parse: {err}"))
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn embedded_packet_matches_seed() {
    let packet = current_m5_secret_boundary_depth_packet().expect("embedded packet parses");
    assert_eq!(packet, seeded_m5_secret_boundary_depth_packet());
}

#[test]
fn checked_fixture_packet_matches_seed() {
    assert_eq!(
        load_packet_fixture(),
        seeded_m5_secret_boundary_depth_packet()
    );
}

#[test]
fn checked_fixture_support_export_matches_projection() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let export = SecretBoundarySupportExport::from_packet(
        "m5-secret-boundary-depth:support-export",
        "2026-06-12T00:00:00Z",
        &packet,
    );
    assert_eq!(load_support_export_fixture(), export);
}

#[test]
fn every_required_domain_is_covered() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let present: BTreeSet<_> = packet.surface_rows.iter().map(|row| row.domain).collect();
    for domain in SecretBoundarySurfaceDomain::ALL {
        assert!(
            present.contains(&domain),
            "missing domain coverage for {}",
            domain.as_str()
        );
    }
}

#[test]
fn every_required_consumer_projection_is_present() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let present: BTreeSet<_> = packet
        .consumer_projections
        .iter()
        .map(|row| row.surface)
        .collect();
    for surface in SecretBoundaryConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer projection for {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_required_artifact_export_rule_is_present() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let present: BTreeSet<_> = packet
        .artifact_export_rules
        .iter()
        .map(|row| row.artifact_family)
        .collect();
    for family in SecretBoundaryArtifactFamilyClass::ALL {
        assert!(
            present.contains(&family),
            "missing artifact export rule for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_row_covers_every_deployment_profile() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    for row in &packet.surface_rows {
        let present: BTreeSet<_> = row
            .profile_parity_rows
            .iter()
            .map(|profile| profile.deployment_profile)
            .collect();
        for profile in SecretBoundaryDeploymentProfileClass::ALL {
            assert!(
                present.contains(&profile),
                "row {} missing profile {}",
                row.matrix_row_id,
                profile.as_str()
            );
        }
    }
}

#[test]
fn summary_tracks_required_degraded_states() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let states = &packet.summary.health_state_tokens_present;
    for state in [
        SecretBoundaryHealthStateClass::Missing,
        SecretBoundaryHealthStateClass::Expired,
        SecretBoundaryHealthStateClass::Revoked,
        SecretBoundaryHealthStateClass::PolicyBlocked,
        SecretBoundaryHealthStateClass::ForwardingPaused,
        SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
    ] {
        assert!(
            states.iter().any(|token| token == state.as_str()),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn summary_tracks_required_consumer_identities_and_projection_controls() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let consumer_tokens = &packet.summary.consumer_identity_tokens_present;
    for identity in SecretBoundaryConsumerIdentityClass::ALL {
        assert!(
            consumer_tokens
                .iter()
                .any(|token| token == identity.as_str()),
            "missing consumer identity {}",
            identity.as_str()
        );
    }

    let control_tokens = &packet.summary.projection_control_tokens_present;
    for control in SecretBoundaryProjectionControlClass::ALL {
        assert!(
            control_tokens.iter().any(|token| token == control.as_str()),
            "missing projection control {}",
            control.as_str()
        );
    }
}

#[test]
fn summary_tracks_required_repairable_change_classes() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let tokens = &packet.summary.repairable_change_tokens_present;
    for change_class in SecretBoundaryRepairableChangeClass::ALL {
        assert!(
            tokens.iter().any(|token| token == change_class.as_str()),
            "missing repairable change {}",
            change_class.as_str()
        );
    }
}

#[test]
fn support_export_preserves_doctor_and_support_lineage() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    let export = SecretBoundarySupportExport::from_packet(
        "m5-secret-boundary-depth:support-export",
        "2026-06-12T00:00:00Z",
        &packet,
    );
    assert!(export.project_doctor_lineage_preserved);
    assert!(export.support_bundle_lineage_preserved);
    assert!(export.workflow_history_lineage_preserved);
    assert!(export.activity_lineage_preserved);
    assert!(!export.doctor_finding_codes.is_empty());
    assert!(!export.lineage_events.is_empty());
    assert!(!export.workflow_history_rows.is_empty());
    assert!(!export.activity_rows.is_empty());
    assert_eq!(
        export.artifact_export_rules,
        packet.artifact_export_rules,
        "support export must preserve canonical artifact export rules"
    );
    assert!(export
        .rows
        .iter()
        .all(|row| !row.repairable_states.is_empty()));
}

#[test]
fn summary_tracks_artifact_families_and_omitted_material_classes() {
    let packet = seeded_m5_secret_boundary_depth_packet();
    for family in SecretBoundaryArtifactFamilyClass::ALL {
        assert!(
            packet
                .summary
                .artifact_family_tokens_present
                .iter()
                .any(|token| token == family.as_str()),
            "missing artifact family {}",
            family.as_str()
        );
    }
    for omitted in SecretBoundaryOmittedMaterialClass::ALL {
        assert!(
            packet
                .summary
                .omitted_material_tokens_present
                .iter()
                .any(|token| token == omitted.as_str()),
            "missing omitted material {}",
            omitted.as_str()
        );
    }
}

#[test]
fn active_repair_state_lookup_matches_triggering_health_state() {
    let state = seeded_secret_boundary_active_repair_state(
        "m5.secret.managed.workspace_runtime",
        SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
    )
    .expect("managed runtime must expose a remote-vault repair state");
    assert_eq!(
        state.change_class,
        SecretBoundaryRepairableChangeClass::RotationRequired
    );
    assert_eq!(
        state.doctor_finding_code,
        "doctor.finding.secret_boundary.managed.rotation_required"
    );
}

#[test]
fn duplicate_row_id_fails_validation() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet.surface_rows[1].matrix_row_id = packet.surface_rows[0].matrix_row_id.clone();
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5SecretBoundaryDepthViolation::DuplicateMatrixRowId(_))));
}

#[test]
fn missing_consumer_projection_fails_validation() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet
        .consumer_projections
        .retain(|row| row.surface != SecretBoundaryConsumerSurface::Diagnostics);
    assert!(packet.validate().contains(
        &M5SecretBoundaryDepthViolation::MissingConsumerProjection(
            SecretBoundaryConsumerSurface::Diagnostics
        )
    ));
}

#[test]
fn missing_artifact_export_rule_fails_validation() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet
        .artifact_export_rules
        .retain(|row| row.artifact_family != SecretBoundaryArtifactFamilyClass::AiEvidencePackets);
    assert!(packet.validate().contains(
        &M5SecretBoundaryDepthViolation::MissingArtifactExportRule(
            SecretBoundaryArtifactFamilyClass::AiEvidencePackets
        )
    ));
}

#[test]
fn missing_repairable_state_fails_validation() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet.surface_rows[0].repairable_states.clear();
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5SecretBoundaryDepthViolation::MissingRepairableStates(row)
            if row == "m5.secret.request_workspace.send_http"
    )));
}

#[test]
fn summary_mismatch_fails_validation() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet.summary.surface_count = 999;
    assert!(packet
        .validate()
        .contains(&M5SecretBoundaryDepthViolation::SummaryMismatch));
}

#[test]
fn forwarding_paused_requires_forwarded_local_parity() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet.surface_rows[0].profile_parity_rows[1].projection_parity =
        SecretBoundaryProjectionParityClass::LocalHandle;
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5SecretBoundaryDepthViolation::ForwardingPausedParityDrift(_, _)
    )));
}

#[test]
fn forwarded_identity_requires_pause_control() {
    let mut packet = seeded_m5_secret_boundary_depth_packet();
    packet.surface_rows[0]
        .projection_control_classes
        .retain(|control| *control != SecretBoundaryProjectionControlClass::PauseForwarding);
    packet.surface_rows[0].acting_identities =
        vec![SecretBoundaryActingIdentityClass::ForwardedLocalCredential];
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5SecretBoundaryDepthViolation::ForwardedIdentityMissingPauseControl(_)
    )));
}
