//! Synthetic rollback-drill tests for install topology roots.

use std::path::{Path, PathBuf};

use aureline_install::{
    InstallTopologyAlphaPacket, RollbackDrillDriver, RollbackDrillError, RollbackDrillPlan,
    RollbackDrillRootRole,
};
use aureline_recovery::session_restore::records::{
    ExcludedLiveAuthorityClass, ProducerBuildStamp, SurfaceClass, SurfaceRole, TrustedRootRecord,
    WindowRole,
};
use aureline_recovery::session_restore::{
    SessionRestoreCaptureInput, SessionRestoreStore, TabGroupCaptureInput, TabItemCaptureInput,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/install/topology_alpha/install_topology_alpha_packet.json")
}

fn load_packet() -> InstallTopologyAlphaPacket {
    let bytes = std::fs::read(fixture_path()).expect("read install topology alpha fixture");
    serde_json::from_slice(&bytes).expect("parse install topology alpha fixture")
}

fn drill_plan(packet: &InstallTopologyAlphaPacket) -> RollbackDrillPlan {
    RollbackDrillPlan::portable_side_by_side(
        packet,
        "install.topology.windows.preview.side_by_side",
        "install.topology.windows.portable.stable",
    )
    .expect("portable side-by-side rollback plan")
}

fn root_ref(plan: &RollbackDrillPlan, role: RollbackDrillRootRole, needle: &str) -> String {
    plan.roots
        .iter()
        .find(|root| root.role == role && root.root_ref.contains(needle))
        .map(|root| root.root_ref.clone())
        .unwrap_or_else(|| panic!("missing {role:?} root containing {needle}"))
}

fn seed_session_restore(driver: &RollbackDrillDriver, root_ref: &str) {
    let root = driver
        .state_root_path(root_ref)
        .expect("state-root path should be safe");
    let mut store = SessionRestoreStore::new(&root);
    store
        .capture(SessionRestoreCaptureInput {
            workspace_ref: "workspace:synthetic-preview".to_string(),
            producer_build: ProducerBuildStamp {
                producer_name: "aureline".to_string(),
                producer_version: "0.0.0".to_string(),
                producer_channel: Some("preview".to_string()),
                producer_platform_class: Some("windows".to_string()),
                producer_instance_handle: None,
            },
            source_schema_version: "session-restore.v1".to_string(),
            trusted_root_refs: vec![TrustedRootRecord {
                root_id: "trusted-root:synthetic-preview".to_string(),
                trust_state: "trusted".to_string(),
                scope_ref: "scope:local".to_string(),
                policy_epoch_ref: None,
                note: None,
            }],
            active_workset_ids: vec!["workset:default".to_string()],
            dirty_buffer_journal_identities: Vec::new(),
            recovery_journal_refs: vec!["recovery:synthetic-preview".to_string()],
            local_history_snapshot_refs: Vec::new(),
            evidence_bundle_refs: vec!["evidence:synthetic-preview".to_string()],
            excluded_live_authority_classes: vec![ExcludedLiveAuthorityClass::RawSecretMaterial],
            downgrade_triggers: Vec::new(),
            window_id: "window:preview-main".to_string(),
            window_role: WindowRole::Primary,
            topology_family_ref: None,
            sibling_window_refs: Vec::new(),
            tab_groups: vec![TabGroupCaptureInput {
                group_id: "group:main".to_string(),
                ordered_tabs: vec![TabItemCaptureInput {
                    tab_id: "tab:editor".to_string(),
                    tab_label: Some("main.rs".to_string()),
                    pinned: false,
                    dirty_badge_visible: false,
                    surface_role: SurfaceRole::Editor,
                    surface_class: SurfaceClass::TextEditor,
                    restore_metadata: None,
                }],
                active_tab_id: Some("tab:editor".to_string()),
            }],
            emitted_at: "2026-05-15T00:00:00Z".to_string(),
            notes: Some("synthetic rollback drill restore seed".to_string()),
        })
        .expect("capture session restore seed");
}

#[test]
fn rollback_drill_restores_preview_and_preserves_peer_and_portable_roots() {
    let packet = load_packet();
    let plan = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed synthetic state tree");

    let target_recovery_root = root_ref(
        &plan,
        RollbackDrillRootRole::TargetRollback,
        "per_user_recovery_root.preview",
    );
    let peer_settings_root = root_ref(
        &plan,
        RollbackDrillRootRole::SideBySidePeer,
        "per_user_configuration_root.stable",
    );
    let portable_root = root_ref(
        &plan,
        RollbackDrillRootRole::PortableStateRoot,
        "portable_colocated_root.portable_stable",
    );
    seed_session_restore(&driver, &target_recovery_root);

    let peer_before = std::fs::read(
        driver
            .state_root_path(&peer_settings_root)
            .expect("peer path")
            .join("state-root.json"),
    )
    .expect("read peer state");
    let portable_before = std::fs::read(
        driver
            .state_root_path(&portable_root)
            .expect("portable path")
            .join("state-root.json"),
    )
    .expect("read portable state");

    let report = driver.run(&plan).expect("rollback drill passes");
    assert!(report.pre_state_captured);
    assert!(report.target_rolled_back);
    assert_eq!(report.expected_delta_count, 1);
    assert!(report.diffs.is_empty());

    let target_recovery_path = driver
        .state_root_path(&target_recovery_root)
        .expect("target path");
    let restore_summary = SessionRestoreStore::new(&target_recovery_path)
        .latest_summary()
        .expect("session restore summary")
        .expect("session restore should survive rollback");
    assert_eq!(restore_summary.tab_count, 1);
    assert!(!target_recovery_path
        .join("update-staging")
        .join("candidate-marker.json")
        .exists());
    assert!(target_recovery_path
        .join("rollback-evidence")
        .join("post-rollback.json")
        .exists());

    let peer_after = std::fs::read(
        driver
            .state_root_path(&peer_settings_root)
            .expect("peer path")
            .join("state-root.json"),
    )
    .expect("read peer state after");
    let portable_after = std::fs::read(
        driver
            .state_root_path(&portable_root)
            .expect("portable path")
            .join("state-root.json"),
    )
    .expect("read portable state after");
    assert_eq!(peer_after, peer_before);
    assert_eq!(portable_after, portable_before);
}

#[test]
fn corrupted_pre_state_snapshot_fails_with_typed_error() {
    let packet = load_packet();
    let plan = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed synthetic state tree");
    let mut snapshot = driver
        .capture_pre_state(&plan)
        .expect("capture pre-state snapshot");
    snapshot
        .entries
        .iter_mut()
        .find(|entry| !entry.contents.is_empty())
        .expect("at least one file entry")
        .contents
        .push(b'!');
    std::fs::write(
        driver.pre_state_snapshot_path(&plan.drill_id),
        serde_json::to_vec_pretty(&snapshot).expect("serialize corrupted snapshot"),
    )
    .expect("write corrupted snapshot");

    let err = driver
        .run_from_captured_pre_state(&plan)
        .expect_err("corrupted snapshot should fail");
    assert!(matches!(
        err,
        RollbackDrillError::CorruptedPreStateSnapshot { .. }
    ));
}
