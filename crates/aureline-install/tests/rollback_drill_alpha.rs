// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Synthetic rollback-drill tests for install topology roots.

use std::path::{Path, PathBuf};

#[cfg(unix)]
use aureline_install::RollbackDrillRootRole;
use aureline_install::{
    InstallTopologyAlphaPacket, RollbackDrillDriver, RollbackDrillError, RollbackDrillPlan,
    ROLLBACK_DRILL_MAX_DEPTH, ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES, ROLLBACK_DRILL_MAX_FILE_BYTES,
    ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES,
};
#[cfg(unix)]
use aureline_recovery::session_restore::records::{
    ExcludedLiveAuthorityClass, ProducerBuildStamp, SurfaceClass, SurfaceRole, TrustedRootRecord,
    WindowRole,
};
#[cfg(unix)]
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

#[cfg(unix)]
fn root_ref(plan: &RollbackDrillPlan, role: RollbackDrillRootRole, needle: &str) -> String {
    plan.roots
        .iter()
        .find(|root| root.role == role && root.root_ref.contains(needle))
        .map(|root| root.root_ref.clone())
        .unwrap_or_else(|| panic!("missing {role:?} root containing {needle}"))
}

#[cfg(unix)]
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
                producer_channel: Some("experimental".to_string()),
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
                    surface_binding_ref: None,
                    pinned: false,
                    dirty_badge_visible: false,
                    surface_role: SurfaceRole::Editor,
                    surface_class: SurfaceClass::TextEditor,
                    restore_metadata: None,
                }],
                active_tab_id: Some("tab:editor".to_string()),
            }],
            pane_tree_layout: None,
            focused_group_id: Some("group:main".to_string()),
            emitted_at: "2026-05-15T00:00:00Z".to_string(),
            notes: Some("synthetic rollback drill restore seed".to_string()),
        })
        .expect("capture session restore seed");
}

#[cfg(unix)]
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

#[cfg(not(unix))]
#[test]
fn destructive_restore_fails_closed_without_stable_file_identity() {
    let packet = load_packet();
    let plan = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed synthetic state tree");
    driver
        .capture_pre_state(&plan)
        .expect("capture pre-state snapshot");

    let err = driver
        .run_from_captured_pre_state(&plan)
        .expect_err("destructive restore must fail closed on this platform");
    assert!(matches!(err, RollbackDrillError::UnsafeStateRoot { .. }));
}

#[test]
fn non_empty_unmarked_authority_is_rejected_without_exposing_host_paths() {
    let packet = load_packet();
    let plan = drill_plan(&packet);
    let parent = tempfile::tempdir().expect("parent tempdir");
    let authority = parent.path().join("private-customer-path-token");
    std::fs::create_dir(&authority).expect("create candidate authority");
    let sentinel = authority.join("do-not-touch.txt");
    std::fs::write(&sentinel, b"preserve-me").expect("write sentinel");
    let driver = RollbackDrillDriver::new(&authority);

    let err = driver
        .seed_synthetic_state_tree(&plan)
        .expect_err("a populated unmarked directory must not become synthetic authority");
    assert!(matches!(&err, RollbackDrillError::UnsafeStateRoot { .. }));
    assert_eq!(
        std::fs::read(&sentinel).expect("read sentinel"),
        b"preserve-me"
    );
    let rendered = err.to_string();
    assert!(!rendered.contains("private-customer-path-token"));
    assert!(!rendered.contains(parent.path().to_string_lossy().as_ref()));
}

#[cfg(unix)]
#[test]
fn symlink_authority_and_planted_state_roots_cannot_escape_containment() {
    use std::os::unix::fs::symlink;

    let packet = load_packet();
    let plan = drill_plan(&packet);
    let parent = tempfile::tempdir().expect("parent tempdir");
    let outside = tempfile::tempdir().expect("outside tempdir");
    let sentinel = outside.path().join("outside-sentinel.txt");
    std::fs::write(&sentinel, b"outside-safe").expect("write outside sentinel");

    let root_alias = parent.path().join("root-alias");
    symlink(outside.path(), &root_alias).expect("create authority symlink");
    let err = RollbackDrillDriver::new(&root_alias)
        .seed_synthetic_state_tree(&plan)
        .expect_err("symlink authority must fail");
    assert!(matches!(err, RollbackDrillError::UnsafeStateRoot { .. }));

    let marked_root = parent.path().join("marked-authority");
    std::fs::create_dir(&marked_root).expect("create marked root");
    let driver = RollbackDrillDriver::new(&marked_root);
    driver
        .state_root_path("probe.root")
        .expect("initialize explicit synthetic authority");
    symlink(outside.path(), marked_root.join("state-roots")).expect("plant state-roots redirect");
    let err = driver
        .seed_synthetic_state_tree(&plan)
        .expect_err("redirected state-roots authority must fail");
    assert!(matches!(err, RollbackDrillError::UnsafeStateRoot { .. }));
    assert_eq!(
        std::fs::read(&sentinel).expect("read outside sentinel"),
        b"outside-safe"
    );
}

#[cfg(unix)]
#[test]
fn redirected_target_root_is_rejected_before_update_or_restore_writes() {
    use std::os::unix::fs::symlink;

    let packet = load_packet();
    let plan = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let outside = tempfile::tempdir().expect("outside tempdir");
    let sentinel = outside.path().join("outside-sentinel.txt");
    std::fs::write(&sentinel, b"outside-safe").expect("write outside sentinel");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed synthetic tree");
    driver
        .capture_pre_state(&plan)
        .expect("capture pre-state snapshot");

    let target_ref = plan.target_root_refs()[0];
    let target = driver.state_root_path(target_ref).expect("target path");
    let displaced = tempdir.path().join("displaced-target");
    std::fs::rename(&target, &displaced).expect("displace target root");
    symlink(outside.path(), &target).expect("redirect target root");

    let err = driver
        .run_from_captured_pre_state(&plan)
        .expect_err("redirected target root must fail closed");
    assert!(matches!(err, RollbackDrillError::UnsafeStateRoot { .. }));
    assert_eq!(
        std::fs::read(&sentinel).expect("read outside sentinel"),
        b"outside-safe"
    );
}

#[test]
fn oversized_file_directory_explosion_and_excessive_depth_are_bounded() {
    let packet = load_packet();
    let plan = drill_plan(&packet);

    let oversized = tempfile::tempdir().expect("oversized tempdir");
    let oversized_driver = RollbackDrillDriver::new(oversized.path());
    oversized_driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed oversized tree");
    let oversized_root = oversized_driver
        .state_root_path(plan.target_root_refs()[0])
        .expect("oversized root");
    let oversized_file =
        std::fs::File::create(oversized_root.join("oversized.bin")).expect("create oversized file");
    oversized_file
        .set_len(ROLLBACK_DRILL_MAX_FILE_BYTES + 1)
        .expect("size oversized file");
    assert!(matches!(
        oversized_driver.capture_pre_state(&plan),
        Err(RollbackDrillError::ResourceLimitExceeded { .. })
    ));

    let explosion = tempfile::tempdir().expect("explosion tempdir");
    let explosion_driver = RollbackDrillDriver::new(explosion.path());
    explosion_driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed explosion tree");
    let explosion_root = explosion_driver
        .state_root_path(plan.target_root_refs()[0])
        .expect("explosion root")
        .join("entry-explosion");
    std::fs::create_dir(&explosion_root).expect("create explosion directory");
    for index in 0..=ROLLBACK_DRILL_MAX_DIRECTORY_ENTRIES {
        std::fs::write(explosion_root.join(format!("entry-{index:04}.txt")), b"")
            .expect("write explosion entry");
    }
    assert!(matches!(
        explosion_driver.capture_pre_state(&plan),
        Err(RollbackDrillError::ResourceLimitExceeded { .. })
    ));

    let deep = tempfile::tempdir().expect("deep tempdir");
    let deep_driver = RollbackDrillDriver::new(deep.path());
    deep_driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed deep tree");
    let mut current = deep_driver
        .state_root_path(plan.target_root_refs()[0])
        .expect("deep root")
        .join("deep");
    std::fs::create_dir(&current).expect("create first deep directory");
    for _ in 0..ROLLBACK_DRILL_MAX_DEPTH {
        current = current.join("d");
        std::fs::create_dir(&current).expect("create nested directory");
    }
    assert!(matches!(
        deep_driver.capture_pre_state(&plan),
        Err(RollbackDrillError::ResourceLimitExceeded { .. })
    ));
}

#[test]
fn oversized_snapshot_document_and_entry_payload_are_rejected_before_restore() {
    let packet = load_packet();
    let plan = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan)
        .expect("seed synthetic tree");
    let mut snapshot = driver
        .capture_pre_state(&plan)
        .expect("capture pre-state snapshot");
    let snapshot_path = driver.pre_state_snapshot_path(&plan.drill_id);

    let oversized_document = std::fs::OpenOptions::new()
        .write(true)
        .open(&snapshot_path)
        .expect("open snapshot");
    oversized_document
        .set_len(ROLLBACK_DRILL_MAX_SNAPSHOT_BYTES + 1)
        .expect("size oversized snapshot");
    assert!(matches!(
        driver.run_from_captured_pre_state(&plan),
        Err(RollbackDrillError::ResourceLimitExceeded { .. })
    ));
    drop(oversized_document);

    snapshot
        .entries
        .iter_mut()
        .find(|entry| entry.entry_kind == aureline_install::RollbackDrillEntryKind::File)
        .expect("snapshot file entry")
        .contents
        .resize(ROLLBACK_DRILL_MAX_FILE_BYTES as usize + 1, 0);
    std::fs::write(
        &snapshot_path,
        serde_json::to_vec(&snapshot).expect("serialize oversized entry payload"),
    )
    .expect("write oversized entry payload");
    assert!(matches!(
        driver.run_from_captured_pre_state(&plan),
        Err(RollbackDrillError::ResourceLimitExceeded { .. })
    ));
}
