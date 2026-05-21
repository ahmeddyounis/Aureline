//! Beta rollback-drill tests.
//!
//! The beta drill matures the alpha synthetic state-root rollback into a
//! beta-level rehearsal that ties the synthetic restore to the governed beta
//! rollback plan and the release-center rollback/revocation record model. It
//! exercises a planned rollback to a prior known-good build, post-rollback
//! durable state-root integrity, exact-build install diagnostics after
//! rollback, and an honest failure when the prior build is unavailable or
//! unverifiable.

use std::path::{Path, PathBuf};

use aureline_install::{
    ExactBuildInstallIdentity, ExactBuildManifestState, InstallTopologyAlphaPacket,
    RetainedArtifactState, RetainedArtifactVerificationState, RollbackDrillDriver,
    RollbackDrillError, RollbackDrillPlan, RollbackDrillRootRole, UpdateRollbackPlan,
};
use aureline_recovery::session_restore::records::{
    ExcludedLiveAuthorityClass, ProducerBuildStamp, SurfaceClass, SurfaceRole, TrustedRootRecord,
    WindowRole,
};
use aureline_recovery::session_restore::{
    SessionRestoreCaptureInput, SessionRestoreStore, TabGroupCaptureInput, TabItemCaptureInput,
};
use aureline_release::{
    ArtifactGraphConsistency, RollbackOrRevocationKind, RollbackOrRevocationRecord,
};
use serde::Deserialize;

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn topology_fixture_path() -> PathBuf {
    manifest_dir().join("../../fixtures/install/topology_alpha/install_topology_alpha_packet.json")
}

fn drill_fixtures_dir() -> PathBuf {
    manifest_dir().join("../../fixtures/install/rollback_drill_beta")
}

fn plan_path() -> PathBuf {
    manifest_dir().join("../../artifacts/release/m3/update_rollback/rollback_plan.json")
}

fn load_topology_packet() -> InstallTopologyAlphaPacket {
    let bytes =
        std::fs::read(topology_fixture_path()).expect("read install topology alpha fixture");
    serde_json::from_slice(&bytes).expect("parse install topology alpha fixture")
}

fn load_plan() -> UpdateRollbackPlan {
    let bytes = std::fs::read(plan_path()).expect("read beta update rollback plan");
    serde_json::from_slice(&bytes).expect("parse beta update rollback plan")
}

fn load_rollback_record(name: &str) -> RollbackOrRevocationRecord {
    let bytes =
        std::fs::read(drill_fixtures_dir().join(name)).expect("read rollback record fixture");
    serde_json::from_slice(&bytes).expect("parse rollback record fixture")
}

fn load_post_rollback_diagnostics() -> PostRollbackDiagnostics {
    let bytes =
        std::fs::read(drill_fixtures_dir().join("post_rollback_exact_build_diagnostics.json"))
            .expect("read post-rollback diagnostics fixture");
    serde_json::from_slice(&bytes).expect("parse post-rollback diagnostics fixture")
}

/// Frozen exact-build install diagnostics observed after the rollback completes.
///
/// The envelope reuses the install crate's [`ExactBuildInstallIdentity`] type so
/// the drill asserts the same exact-build truth the diagnostics surfaces render.
#[derive(Debug, Deserialize)]
struct PostRollbackDiagnostics {
    source_install_diagnostics_ref: String,
    rollback_target_exact_build_identity_ref: String,
    superseded_exact_build_identity_ref: String,
    rows: Vec<PostRollbackDiagnosticsRow>,
}

#[derive(Debug, Deserialize)]
struct PostRollbackDiagnosticsRow {
    topology_row_id: String,
    exact_build: ExactBuildInstallIdentity,
}

/// Whether the prior known-good build can honestly back a rollback.
#[derive(Debug, PartialEq, Eq)]
enum PriorBuildAvailability {
    /// The plan, release-center record, and diagnostics agree the prior build is
    /// retained, verified, and resolvable as the rollback target.
    Available,
    /// The prior build is missing, unverifiable, revoked, or inconsistent.
    Unavailable(Vec<String>),
}

impl PriorBuildAvailability {
    fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    fn reasons(&self) -> &[String] {
        match self {
            Self::Available => &[],
            Self::Unavailable(reasons) => reasons,
        }
    }
}

/// Resolves whether the rollback target build is honestly available by
/// cross-checking the beta rollback plan, the release-center rollback record,
/// and the post-rollback exact-build diagnostics. A missing, unverifiable, or
/// revoked prior build yields [`PriorBuildAvailability::Unavailable`] instead of
/// silently allowing a "successful" rollback to a build that cannot be restored.
fn prior_build_availability(
    plan: &UpdateRollbackPlan,
    record: &RollbackOrRevocationRecord,
    diagnostics: &PostRollbackDiagnostics,
) -> PriorBuildAvailability {
    let mut reasons = Vec::new();
    let target = plan.rollback_target.exact_build_identity_ref.as_str();

    let report = plan.validate();
    if !report.passed {
        reasons.push(format!(
            "rollback plan failed validation ({} findings)",
            report.findings.len()
        ));
    }

    for artifact in &plan.retained_prior_artifacts {
        if !artifact.retention_state.is_exact_build_retained() {
            reasons.push(format!(
                "retained prior artifact {} is not exact-build retained",
                artifact.artifact_ref
            ));
        }
        if !artifact.verification_state.is_verified() {
            reasons.push(format!(
                "retained prior artifact {} is not verified",
                artifact.artifact_ref
            ));
        }
    }

    if record.kind != RollbackOrRevocationKind::Rollback {
        reasons.push(format!(
            "release-center record kind is {:?}, not a rollback",
            record.kind
        ));
    }
    if record.rollback_manifest_ref.is_none() {
        reasons.push("release-center rollback record has no rollback manifest".to_string());
    }
    if record.last_known_good_ref != target {
        reasons.push(
            "release-center last-known-good ref does not match the plan rollback target"
                .to_string(),
        );
    }
    if !matches!(
        record.artifact_graph_consistency,
        ArtifactGraphConsistency::ConsistentFullGraph
            | ArtifactGraphConsistency::ConsistentScopedException
    ) {
        reasons.push(format!(
            "release-center artifact graph consistency is {:?}",
            record.artifact_graph_consistency
        ));
    }
    if record
        .affected_artifact_refs
        .iter()
        .any(|affected| affected == target)
    {
        reasons.push(
            "rollback target exact-build is in the record's affected (revoked) set".to_string(),
        );
    }

    if diagnostics.rows.is_empty() {
        reasons.push("post-rollback diagnostics have no rows".to_string());
    }
    for row in &diagnostics.rows {
        if row.exact_build.exact_build_identity_ref != target {
            reasons.push(format!(
                "post-rollback diagnostics row {} does not resolve to the rollback target build",
                row.topology_row_id
            ));
        }
        if row.exact_build.manifest_state != ExactBuildManifestState::Present {
            reasons.push(format!(
                "post-rollback diagnostics row {} has no present exact-build manifest",
                row.topology_row_id
            ));
        }
    }

    if reasons.is_empty() {
        PriorBuildAvailability::Available
    } else {
        PriorBuildAvailability::Unavailable(reasons)
    }
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

fn read_state_root(driver: &RollbackDrillDriver, root_ref: &str) -> Vec<u8> {
    std::fs::read(
        driver
            .state_root_path(root_ref)
            .expect("state-root path should be safe")
            .join("state-root.json"),
    )
    .expect("read state-root.json")
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
            notes: Some("synthetic beta rollback drill restore seed".to_string()),
        })
        .expect("capture session restore seed");
}

#[test]
fn beta_rollback_drill_restores_prior_known_good_build() {
    let plan = load_plan();
    let record = load_rollback_record("release_center_rollback_record.json");
    let diagnostics = load_post_rollback_diagnostics();

    // The governed beta rollback plan validates and every cross-surface input
    // agrees the prior known-good build is available before any mutation.
    assert!(plan.validate().passed, "beta rollback plan must validate");
    let availability = prior_build_availability(&plan, &record, &diagnostics);
    assert_eq!(
        availability,
        PriorBuildAvailability::Available,
        "prior build must be available: {availability:?}"
    );

    // Drive the synthetic rollback drill against caller-provided synthetic roots.
    let packet = load_topology_packet();
    let plan_drill = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan_drill)
        .expect("seed synthetic state tree");

    let target_recovery_root = root_ref(
        &plan_drill,
        RollbackDrillRootRole::TargetRollback,
        "per_user_recovery_root.preview",
    );
    let peer_settings_root = root_ref(
        &plan_drill,
        RollbackDrillRootRole::SideBySidePeer,
        "per_user_configuration_root.stable",
    );
    let portable_root = root_ref(
        &plan_drill,
        RollbackDrillRootRole::PortableStateRoot,
        "portable_colocated_root.portable_stable",
    );
    seed_session_restore(&driver, &target_recovery_root);

    let peer_before = read_state_root(&driver, &peer_settings_root);
    let portable_before = read_state_root(&driver, &portable_root);

    let report = driver.run(&plan_drill).expect("rollback drill passes");
    assert!(report.pre_state_captured);
    assert!(report.target_rolled_back);
    assert_eq!(report.expected_delta_count, 1);
    assert!(report.diffs.is_empty());

    // Post-rollback state-root integrity: user-authored session restore survives
    // the rollback, the synthetic candidate marker is gone, and only the
    // declared post-rollback evidence delta remains.
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

    // The side-by-side peer and the portable colocated root are untouched.
    assert_eq!(read_state_root(&driver, &peer_settings_root), peer_before);
    assert_eq!(read_state_root(&driver, &portable_root), portable_before);
}

#[test]
fn exact_build_install_diagnostics_resolve_to_prior_build_after_rollback() {
    let plan = load_plan();
    let diagnostics = load_post_rollback_diagnostics();
    let target = plan.rollback_target.exact_build_identity_ref.as_str();

    assert!(
        !diagnostics.rows.is_empty(),
        "post-rollback diagnostics must carry at least one row"
    );
    for row in &diagnostics.rows {
        assert_eq!(
            row.exact_build.exact_build_identity_ref, target,
            "diagnostics row {} must resolve to the rollback target build",
            row.topology_row_id
        );
        assert_eq!(
            row.exact_build.manifest_state,
            ExactBuildManifestState::Present
        );
        assert_eq!(row.exact_build.release_channel_class, "stable");
        assert_eq!(
            row.exact_build.product_version,
            plan.rollback_target.version
        );
        // Diagnostics flipped away from the superseded candidate build.
        assert_ne!(
            row.exact_build.exact_build_identity_ref,
            plan.current_build.exact_build_identity_ref
        );
    }

    // The diagnostics envelope quotes the same install-diagnostics packet the
    // plan references, and names the build it superseded by rolling back.
    assert_eq!(
        diagnostics.source_install_diagnostics_ref,
        plan.source_refs.install_diagnostics_ref
    );
    assert_eq!(diagnostics.rollback_target_exact_build_identity_ref, target);
    assert_eq!(
        diagnostics.superseded_exact_build_identity_ref,
        plan.current_build.exact_build_identity_ref
    );
}

#[test]
fn rollback_to_revoked_prior_build_fails_honestly() {
    let plan = load_plan();
    let record = load_rollback_record("release_center_rollback_record_missing_prior_build.json");
    let diagnostics = load_post_rollback_diagnostics();

    let availability = prior_build_availability(&plan, &record, &diagnostics);
    assert!(
        !availability.is_available(),
        "a revoked prior build must not be reported as an available rollback target"
    );
    let reasons = availability.reasons().join(" | ");
    assert!(
        reasons.contains("not a rollback")
            || reasons.contains("affected (revoked)")
            || reasons.contains("last-known-good"),
        "honest failure must explain the prior build is unavailable: {reasons}"
    );
}

#[test]
fn rollback_to_unverifiable_prior_artifact_fails_validation() {
    let mut plan = load_plan();
    plan.retained_prior_artifacts[0].retention_state = RetainedArtifactState::MissingBlocked;
    plan.retained_prior_artifacts[0].verification_state =
        RetainedArtifactVerificationState::MissingBlocked;

    let report = plan.validate();
    assert!(
        !report.passed,
        "a missing/unverifiable retained prior artifact must fail validation"
    );
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "retained_artifacts.not_exact_build_retained"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "retained_artifacts.not_verified"));

    // The honest failure also propagates through the cross-surface readiness.
    let record = load_rollback_record("release_center_rollback_record.json");
    let diagnostics = load_post_rollback_diagnostics();
    assert!(!prior_build_availability(&plan, &record, &diagnostics).is_available());
}

#[test]
fn rollback_drill_without_captured_prior_state_fails_honestly() {
    let packet = load_topology_packet();
    let plan_drill = drill_plan(&packet);
    let tempdir = tempfile::tempdir().expect("tempdir");
    let driver = RollbackDrillDriver::new(tempdir.path());
    driver
        .seed_synthetic_state_tree(&plan_drill)
        .expect("seed synthetic state tree");

    // The prior known-good state was never captured, so restoring from it is
    // impossible: the drill must refuse rather than report a success.
    let err = driver
        .run_from_captured_pre_state(&plan_drill)
        .expect_err("rollback without a captured prior-build snapshot must fail");
    assert!(matches!(
        err,
        RollbackDrillError::Io { .. } | RollbackDrillError::CorruptedPreStateSnapshot { .. }
    ));
}

#[test]
fn post_rollback_diagnostics_without_present_manifest_fail_honestly() {
    let plan = load_plan();
    let record = load_rollback_record("release_center_rollback_record.json");
    let mut diagnostics = load_post_rollback_diagnostics();
    diagnostics.rows[0].exact_build.manifest_state = ExactBuildManifestState::Reserved;

    let availability = prior_build_availability(&plan, &record, &diagnostics);
    assert!(
        !availability.is_available(),
        "diagnostics that cannot resolve the prior build must not report availability"
    );
}
