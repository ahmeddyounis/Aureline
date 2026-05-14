//! Fixture-driven tests for the install-topology alpha contract.

use std::path::{Path, PathBuf};

use aureline_install::{
    ChannelClass, HandlerKind, HiddenGlobalStateGuarantee, InstallModeClass,
    InstallTopologyAlphaPacket, MirrorOfflineVerificationState, RepairVerifySupport,
    RollbackOwnerClass, TopologySurfaceClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/install/topology_alpha/install_topology_alpha_packet.json")
}

fn load_packet() -> InstallTopologyAlphaPacket {
    let bytes = std::fs::read(fixture_path()).expect("read install topology alpha fixture");
    serde_json::from_slice(&bytes).expect("parse install topology alpha fixture")
}

#[test]
fn topology_alpha_fixture_passes_validation() {
    let packet = load_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "install topology alpha fixture failed validation: {:#?}",
        report.findings
    );
    for mode in [
        InstallModeClass::PerUserInstalled,
        InstallModeClass::PerMachineInstalled,
        InstallModeClass::Portable,
        InstallModeClass::ManagedDeployed,
        InstallModeClass::SideBySidePreview,
        InstallModeClass::OfflineBundle,
    ] {
        assert!(report.coverage.install_modes.contains(&mode));
    }
    assert!(report
        .coverage
        .mirror_offline_states
        .contains(&MirrorOfflineVerificationState::MirrorMetadataVerified));
    assert!(report
        .coverage
        .mirror_offline_states
        .contains(&MirrorOfflineVerificationState::OfflineBundleVerified));
}

#[test]
fn product_and_support_surfaces_project_the_same_truth() {
    let packet = load_packet();
    let about = packet
        .surface_projection(TopologySurfaceClass::About)
        .truth_fingerprints();

    for surface in [
        TopologySurfaceClass::Update,
        TopologySurfaceClass::Diagnostics,
        TopologySurfaceClass::InstallReview,
        TopologySurfaceClass::Cli,
        TopologySurfaceClass::SupportExport,
    ] {
        let projection = packet.surface_projection(surface);
        assert_eq!(projection.truth_fingerprints(), about, "{surface:?}");
    }

    let support = packet.support_export_projection();
    assert_eq!(support.record_kind, "install_topology_support_export");
    assert_eq!(support.redaction_class, "metadata_only_no_paths_or_secrets");
    assert_eq!(support.projection.truth_fingerprints(), about);
}

#[test]
fn side_by_side_channels_have_distinct_roots_and_reviewed_handler_change() {
    let packet = load_packet();

    assert!(packet.state_roots_disjoint_for_channels(ChannelClass::Stable, ChannelClass::Preview));

    let stable = packet
        .row_by_id("install.topology.windows.per_user.stable")
        .expect("stable row");
    let preview = packet
        .row_by_id("install.topology.windows.preview.side_by_side")
        .expect("preview row");

    assert_eq!(stable.paired_channel_class, Some(ChannelClass::Preview));
    assert_eq!(preview.paired_channel_class, Some(ChannelClass::Stable));
    assert_ne!(
        stable.handler_ownership.selected_owner_channel,
        preview.handler_ownership.selected_owner_channel
    );

    let handler_preview = &packet.handler_ownership_change_previews[0];
    assert!(handler_preview.previewed_before_commit);
    assert!(handler_preview.commit_requires_acknowledgement);
    assert_eq!(handler_preview.before_owner_channel, ChannelClass::Stable);
    assert_eq!(handler_preview.after_owner_channel, ChannelClass::Preview);
    assert!(handler_preview
        .affected_handlers
        .contains(&HandlerKind::FileAssociation));
    assert!(handler_preview
        .affected_handlers
        .contains(&HandlerKind::ProtocolHandler));

    let diagnostic = &packet.stale_handler_owner_diagnostics[0];
    assert!(diagnostic.diagnosed_without_installer_logs);
    assert_eq!(diagnostic.expected_owner_channel, ChannelClass::Stable);
    assert_eq!(diagnostic.observed_owner_channel, ChannelClass::Preview);
}

#[test]
fn portable_and_silent_deployment_rows_disclose_limits_and_owners() {
    let packet = load_packet();
    let portable = packet
        .row_by_id("install.topology.windows.portable.stable")
        .expect("portable row");

    assert_eq!(portable.install_mode_class, InstallModeClass::Portable);
    assert!(
        !portable
            .shell_integration_limits
            .file_associations_may_register
    );
    assert!(
        !portable
            .shell_integration_limits
            .protocol_handlers_may_register
    );
    assert_eq!(
        portable
            .shell_integration_limits
            .hidden_global_state_guarantee,
        HiddenGlobalStateGuarantee::NoHiddenGlobalDurableState
    );
    assert_eq!(
        portable.rollback_posture.rollback_owner,
        RollbackOwnerClass::User
    );
    assert!(!portable
        .silent_deployment_posture
        .disclosed_limits
        .is_empty());

    let managed = packet
        .row_by_id("install.topology.windows.managed.stable")
        .expect("managed row");
    assert_eq!(
        managed.rollback_posture.rollback_owner,
        RollbackOwnerClass::ManagedFleet
    );
    assert!(managed.policy_bootstrap_injection_available);
    assert!(managed.inventory_hooks_available);
    assert!(managed
        .repair_verify_support
        .contains(&RepairVerifySupport::RollbackToPreviousBuild));
}

#[test]
fn portable_host_global_handler_claim_is_rejected() {
    let mut packet = load_packet();
    let portable = packet
        .rows
        .iter_mut()
        .find(|row| row.topology_row_id == "install.topology.windows.portable.stable")
        .expect("portable row");
    portable
        .shell_integration_limits
        .file_associations_may_register = true;
    portable
        .handler_ownership
        .file_association_registration_class = "user_or_admin_selectable_candidate_handler".into();

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "install_topology.portable.global_integration_claimed"
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "install_topology.portable.handler_ownership_not_closed"
    }));
}

#[test]
fn missing_handler_preview_is_rejected() {
    let mut packet = load_packet();
    packet.handler_ownership_change_previews.clear();

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "install_topology.handler_preview.missing"));
}
