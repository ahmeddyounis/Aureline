//! Fixture-driven coverage for account-free local, self-hosted, and managed
//! identity-mode baseline rows.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    CurrentDeploymentBoundaryClass, EntitlementStateClass, IdentityModeAlias,
    IdentityModeBaselinePacket, IdentityModeBaselineViolation, OfflineBehaviorClass,
    PolicyFreshnessClass, PolicySourceClass, IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND,
    IDENTITY_MODE_BASELINE_SCHEMA_VERSION,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/auth/identity_mode_alpha")
}

fn load_packet(file_name: &str) -> IdentityModeBaselinePacket {
    let path = fixture_dir().join(file_name);
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

#[test]
fn baseline_all_modes_fixture_validates_and_projects() {
    let packet = load_packet("baseline_all_modes.json");
    assert_eq!(
        packet.record_kind,
        IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND
    );
    assert_eq!(packet.schema_version, IDENTITY_MODE_BASELINE_SCHEMA_VERSION);
    assert_eq!(
        packet.validate(),
        Vec::<IdentityModeBaselineViolation>::new()
    );
    assert!(packet.has_required_identity_modes());
    assert!(packet.local_core_remains_account_free());
    assert!(packet.org_rows_disclose_policy_entitlement_and_offline_behavior());
    assert!(packet.no_row_overstates_current_boundary());

    let surface_rows = packet.surface_rows();
    assert_eq!(surface_rows.len(), 3);
    assert!(surface_rows
        .iter()
        .all(|row| row.local_core_available_without_account));

    let local = packet
        .identity_mode_rows
        .iter()
        .find(|row| row.identity_mode == IdentityModeAlias::AccountFreeLocal)
        .expect("account-free local row present");
    assert_eq!(
        local.policy_source.source_class,
        PolicySourceClass::NoPolicyRequiredLocal
    );
    assert_eq!(
        local.offline_entitlement.state_class,
        EntitlementStateClass::NotApplicableAccountFree
    );
    assert!(!local.visible_recovery_required());

    let self_hosted = packet
        .identity_mode_rows
        .iter()
        .find(|row| row.identity_mode == IdentityModeAlias::SelfHostedOrg)
        .expect("self-hosted row present");
    assert_eq!(
        self_hosted.policy_source.source_class,
        PolicySourceClass::CustomerSelfHostedOrigin
    );
    assert_eq!(
        self_hosted.boundary.current_boundary_class,
        CurrentDeploymentBoundaryClass::CustomerSelfHostedControlPlane
    );
    assert!(!self_hosted.overstates_current_boundary());

    let managed = packet
        .identity_mode_rows
        .iter()
        .find(|row| row.identity_mode == IdentityModeAlias::ManagedConvenience)
        .expect("managed row present");
    assert_eq!(
        managed.policy_source.source_class,
        PolicySourceClass::VendorManagedOrigin
    );
    assert_eq!(
        managed.policy_source.freshness_class,
        PolicyFreshnessClass::AuthoritativeLive
    );
    assert_eq!(
        managed.offline_entitlement.state_class,
        EntitlementStateClass::Active
    );
    assert_eq!(
        managed.boundary.current_boundary_class,
        CurrentDeploymentBoundaryClass::VendorManagedControlPlane
    );
}

#[test]
fn managed_grace_fixture_pauses_managed_actions_without_gating_local_core() {
    let packet = load_packet("managed_grace_pauses_new_managed.json");
    assert_eq!(
        packet.validate(),
        Vec::<IdentityModeBaselineViolation>::new()
    );

    let managed = packet
        .identity_mode_rows
        .iter()
        .find(|row| row.identity_mode == IdentityModeAlias::ManagedConvenience)
        .expect("managed row present");
    assert!(managed.account_free_local_core_available());
    assert_eq!(
        managed.policy_source.freshness_class,
        PolicyFreshnessClass::StaleWithinGrace
    );
    assert_eq!(
        managed.offline_entitlement.state_class,
        EntitlementStateClass::Grace
    );
    assert_eq!(
        managed.offline_entitlement.offline_behavior_class,
        OfflineBehaviorClass::ManagedOnlyPausedVisibleRecovery
    );
    assert!(managed.offline_entitlement.managed_actions_blocked);
    assert!(managed.offline_entitlement.usage_or_admin_export_available);
    assert!(managed.visible_recovery_required());
    assert!(!managed.overstates_current_boundary());

    let surface = packet.surface_rows();
    let managed_surface = surface
        .iter()
        .find(|row| row.identity_mode_token == "managed_convenience")
        .expect("managed surface row present");
    assert!(managed_surface.local_core_available_without_account);
    assert!(managed_surface.managed_actions_blocked);
    assert!(managed_surface.visible_recovery_required);
    assert_eq!(
        managed_surface.current_boundary_token,
        "vendor_managed_control_plane"
    );
}

#[test]
fn every_identity_mode_alpha_fixture_parses_validates_and_projects() {
    let dir = fixture_dir();
    let mut parsed = 0_usize;
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let entry = entry.expect("readable directory entry");
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("json") {
            continue;
        }
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let packet: IdentityModeBaselinePacket = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        assert_eq!(
            packet.record_kind,
            IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND
        );
        assert_eq!(packet.schema_version, IDENTITY_MODE_BASELINE_SCHEMA_VERSION);
        assert!(
            packet.validate().is_empty(),
            "fixture {} must satisfy identity-mode baseline validation",
            path.display()
        );
        assert!(packet.has_required_identity_modes());
        assert!(packet.local_core_remains_account_free());
        assert!(packet.org_rows_disclose_policy_entitlement_and_offline_behavior());
        assert!(packet.no_row_overstates_current_boundary());
        let surface_rows = packet.surface_rows();
        assert_eq!(surface_rows.len(), packet.identity_mode_rows.len());
        for row in surface_rows {
            assert!(
                row.local_core_available_without_account,
                "surface row {} must not gate local core behind sign-in",
                row.row_id
            );
            assert!(
                row.policy_detail_available,
                "surface row {} must expose policy detail",
                row.row_id
            );
            assert!(
                row.entitlement_detail_available,
                "surface row {} must expose entitlement detail",
                row.row_id
            );
            assert!(
                !row.overstates_current_boundary,
                "surface row {} must not overstate deployment boundary",
                row.row_id
            );
        }
        parsed += 1;
    }
    assert!(
        parsed >= 2,
        "expected at least two identity-mode alpha fixtures; found {parsed}"
    );
}
