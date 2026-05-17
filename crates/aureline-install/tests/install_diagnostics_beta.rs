//! Fixture-driven tests for exact-build install diagnostics.

use std::path::{Path, PathBuf};

use aureline_install::{
    ChannelClass, ExactBuildManifestState, FleetRolloutEvidenceClass, InstallDiagnosticsPacket,
    InstallDiagnosticsSupportExport, InstallModeClass, StateRootIsolationClass,
    StateRootReviewClass, TopologySurfaceClass,
};

fn packet_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json")
}

fn support_export_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m3/install_diagnostics/support_export_projection.json")
}

fn load_packet() -> InstallDiagnosticsPacket {
    let bytes = std::fs::read(packet_path()).expect("read install diagnostics packet");
    serde_json::from_slice(&bytes).expect("parse install diagnostics packet")
}

#[test]
fn install_diagnostics_packet_passes_validation() {
    let packet = load_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "install diagnostics packet failed validation: {:#?}",
        report.findings
    );
    for mode in [
        InstallModeClass::Portable,
        InstallModeClass::SideBySidePreview,
        InstallModeClass::ManagedDeployed,
    ] {
        assert!(report.coverage.install_modes.contains(&mode));
    }
    assert!(report
        .coverage
        .state_root_review_classes
        .contains(&StateRootReviewClass::ExplicitImportReviewRequired));
    assert!(report
        .coverage
        .state_root_review_classes
        .contains(&StateRootReviewClass::PortableNoOsOwnership));
    assert!(report
        .coverage
        .state_root_review_classes
        .contains(&StateRootReviewClass::AdminPolicyReviewRequired));
}

#[test]
fn product_cli_and_support_exports_render_same_truth() {
    let packet = load_packet();
    let about = packet
        .surface_projection(TopologySurfaceClass::About)
        .truth_fingerprints();

    for surface in [
        TopologySurfaceClass::Diagnostics,
        TopologySurfaceClass::Cli,
        TopologySurfaceClass::SupportExport,
    ] {
        let projection = packet.surface_projection(surface);
        assert_eq!(projection.truth_fingerprints(), about, "{surface:?}");
    }

    let expected_support = packet.support_export_projection();
    let support_bytes =
        std::fs::read(support_export_path()).expect("read install diagnostics support export");
    let checked_in_support: InstallDiagnosticsSupportExport =
        serde_json::from_slice(&support_bytes).expect("parse support export");
    assert_eq!(checked_in_support, expected_support);
}

#[test]
fn side_by_side_and_portable_roots_do_not_share_mutable_state() {
    let packet = load_packet();

    assert!(packet.state_roots_disjoint_for_rows(
        "install.topology.windows.per_user.stable",
        "install.topology.windows.preview.side_by_side"
    ));
    assert!(packet.state_roots_disjoint_for_rows(
        "install.topology.windows.per_user.stable",
        "install.topology.windows.portable.stable"
    ));

    let portable = packet
        .row_by_topology_id("install.topology.windows.portable.stable")
        .expect("portable diagnostics row");
    assert_eq!(portable.install_mode_class, InstallModeClass::Portable);
    assert!(portable
        .durable_state_roots
        .iter()
        .all(|root| root.isolation_class == StateRootIsolationClass::PortableColocated));
    assert!(portable.policy_root_refs.is_empty());
}

#[test]
fn fleet_rollout_identifies_exact_build_without_host_log_scraping() {
    let packet = load_packet();
    let managed = packet
        .row_by_topology_id("install.topology.windows.managed.stable")
        .expect("managed diagnostics row");
    let fleet = managed.fleet_rollout.as_ref().expect("fleet diagnostics");

    assert_eq!(
        fleet.exact_build_identity_ref,
        managed.exact_build.exact_build_identity_ref
    );
    assert!(fleet.inventory_probe_available);
    assert!(fleet.deprovision_preserves_local_work);
    assert!(fleet
        .managed_package_report_ref
        .contains("managed_package_report"));
    for evidence in [
        FleetRolloutEvidenceClass::RingAssignment,
        FleetRolloutEvidenceClass::ExactBuildInventory,
        FleetRolloutEvidenceClass::ManagedPackageReport,
        FleetRolloutEvidenceClass::RollbackTarget,
        FleetRolloutEvidenceClass::VerificationStatus,
    ] {
        assert!(fleet.evidence.contains(&evidence));
    }
}

#[test]
fn portable_root_spill_is_rejected() {
    let mut packet = load_packet();
    let portable = packet
        .rows
        .iter_mut()
        .find(|row| row.topology_row_id == "install.topology.windows.portable.stable")
        .expect("portable diagnostics row");
    portable.durable_state_roots[0].state_root_ref =
        "state.per_user_configuration_root.stable".to_string();
    portable.durable_state_roots[0].isolation_class = StateRootIsolationClass::ChannelOwned;
    portable.durable_state_roots[0].owning_channel_class = Some(ChannelClass::Stable);

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "install_diagnostics.portable.root_not_colocated" }));
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "install_diagnostics.cross_channel.state_roots_overlap"
    }));
}

#[test]
fn unresolved_exact_build_identity_is_rejected() {
    let mut packet = load_packet();
    let managed = packet
        .rows
        .iter_mut()
        .find(|row| row.topology_row_id == "install.topology.windows.managed.stable")
        .expect("managed diagnostics row");
    managed.exact_build.manifest_state = ExactBuildManifestState::Reserved;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "install_diagnostics.exact_build.manifest_not_present"
    }));
}
