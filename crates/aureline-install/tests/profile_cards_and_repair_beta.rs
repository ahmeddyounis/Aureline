//! Fixture-driven tests for beta install-profile cards and repair diagnostics.

use std::path::{Path, PathBuf};

use aureline_install::{
    DurableStateRootClass, InstallOperationKind, InstallProfileBetaPacket,
    InstallProfileBetaSupportExport, OperationProfileClass, RepairVerifyPacket,
    RepairVerifySupportExport, RolloutLaneClass,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/install/m3/profile_cards_and_repair")
}

fn load_profile_packet() -> InstallProfileBetaPacket {
    let bytes = std::fs::read(fixture_dir().join("profile_cards_packet.json"))
        .expect("read profile packet");
    serde_json::from_slice(&bytes).expect("parse profile packet")
}

fn load_repair_packet() -> RepairVerifyPacket {
    let bytes = std::fs::read(fixture_dir().join("repair_verify_uninstall_packet.json"))
        .expect("read repair packet");
    serde_json::from_slice(&bytes).expect("parse repair packet")
}

#[test]
fn install_profile_beta_packet_passes_validation() {
    let packet = load_profile_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "install profile packet failed validation: {:#?}",
        report.findings
    );
    for lane in [
        RolloutLaneClass::Canary,
        RolloutLaneClass::Pilot,
        RolloutLaneClass::Broad,
        RolloutLaneClass::Lts,
    ] {
        assert!(report.coverage.rollout_lanes.contains(&lane));
    }
}

#[test]
fn portable_profile_suppresses_machine_global_integrations() {
    let packet = load_profile_packet();
    let portable = packet
        .card_by_id("card.windows.x86_64.portable.portable_stable")
        .expect("portable card");

    assert!(portable.portable_mode.active);
    assert!(portable.portable_mode.durable_roots_colocated);
    assert!(portable
        .durable_state_roots
        .iter()
        .all(|root| root.durable_state_root_class == DurableStateRootClass::PortableColocatedRoot));
}

#[test]
fn side_by_side_import_requires_compare_keep_separate_and_checkpoint() {
    let packet = load_profile_packet();
    let sheet = packet
        .side_by_side_import_sheets
        .first()
        .expect("side-by-side import sheet");

    assert!(sheet.compare_semantics.can_compare_before_apply);
    assert!(sheet.skip_semantics.skip_preserves_source);
    assert!(!sheet.skip_semantics.skip_writes_target);
    assert!(sheet.checkpoint.created_before_apply);
    assert!(sheet.checkpoint.checkpoint_ref.is_some());
    assert!(sheet
        .domain_rows
        .iter()
        .any(|row| row.action == aureline_install::ImportDomainAction::KeepSeparate));
}

#[test]
fn profile_support_export_is_projected_from_packet() {
    let packet = load_profile_packet();
    let expected = packet.support_export_projection();
    let bytes = std::fs::read(fixture_dir().join("profile_cards_support_export.json"))
        .expect("read profile support export");
    let checked_in: InstallProfileBetaSupportExport =
        serde_json::from_slice(&bytes).expect("parse profile support export");

    assert_eq!(checked_in, expected);
}

#[test]
fn repair_verify_uninstall_packet_passes_validation() {
    let packet = load_repair_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "repair/verify packet failed validation: {:#?}",
        report.findings
    );
    for kind in [
        InstallOperationKind::Repair,
        InstallOperationKind::Verify,
        InstallOperationKind::Uninstall,
    ] {
        assert!(report.coverage.operation_kinds.contains(&kind));
    }
    for profile in [
        OperationProfileClass::EnterpriseManaged,
        OperationProfileClass::SilentInstall,
    ] {
        assert!(report.coverage.profile_classes.contains(&profile));
    }
}

#[test]
fn uninstall_rows_preserve_user_state_and_remove_install_state() {
    let packet = load_repair_packet();

    let uninstall_rows: Vec<_> = packet
        .operations
        .iter()
        .filter(|operation| operation.operation_kind == InstallOperationKind::Uninstall)
        .collect();
    assert!(!uninstall_rows.is_empty());
    assert!(uninstall_rows
        .iter()
        .all(|row| !row.preserved_state_root_refs.is_empty()));
    assert!(uninstall_rows
        .iter()
        .all(|row| !row.removed_install_state_refs.is_empty()));
}

#[test]
fn repair_support_export_is_projected_from_packet() {
    let packet = load_repair_packet();
    let expected = packet.support_export_projection();
    let bytes = std::fs::read(fixture_dir().join("repair_verify_uninstall_support_export.json"))
        .expect("read repair support export");
    let checked_in: RepairVerifySupportExport =
        serde_json::from_slice(&bytes).expect("parse repair support export");

    assert_eq!(checked_in, expected);
}

#[test]
fn failed_verify_without_failure_summary_is_rejected() {
    let mut packet = load_repair_packet();
    let failed = packet
        .operations
        .iter_mut()
        .find(|operation| operation.operation_id == "operation:managed.verify.signature_failed")
        .expect("failed verify operation");
    failed.failure_summary = None;

    let report = packet.validate();

    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "repair_verify.operation.failure_summary_missing" }));
}
