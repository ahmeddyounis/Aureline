//! Protected tests for the recovery-ladder alpha evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::recovery_ladder::{
    load_alpha_scenario, FullerModeClass, QuarantineReleaseVisibilityClass, RecoveryActionClass,
    RecoveryLadderAlpha, RecoveryLadderScenario, RecoveryLadderStateClass, RecoveryRungClass,
    RecoveryStateClass, RecoveryTargetKind, RecoveryVisibleStateClass,
    RECOVERY_LADDER_DECISION_RECORD_KIND, RECOVERY_LADDER_RELEASE_PACKET_RECORD_KIND,
    RECOVERY_LADDER_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    case_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/recovery/recovery_ladder_alpha")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_scenarios() -> Vec<RecoveryLadderScenario> {
    load_manifest()
        .case_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_alpha_scenario(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn recovery_ladder_alpha_fixtures_enter_expected_rungs_without_user_state_deletion() {
    let alpha = RecoveryLadderAlpha::new();
    let scenarios = load_scenarios();
    assert_eq!(scenarios.len(), 5);

    let mut covered_rungs = BTreeSet::new();
    for scenario in &scenarios {
        let decision = alpha
            .evaluate(scenario)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", scenario.scenario_id));

        assert_eq!(decision.record_kind, RECOVERY_LADDER_DECISION_RECORD_KIND);
        assert_eq!(
            decision.ladder_state_class,
            scenario.expected.ladder_state_class
        );
        assert_eq!(
            decision.visible_state_class,
            scenario.expected.visible_state_class
        );
        assert!(!decision.mutation.user_owned_state_deleted);
        assert!(!decision.mutation.durable_state_deleted);
        assert!(decision
            .mutation
            .preserved_state_classes
            .contains(&RecoveryStateClass::UserAuthoredFiles));
        assert!(!decision.mutation.changes.is_empty());
        assert!(!decision.return_path.restore_conditions.is_empty());
        assert!(!decision.evidence.is_empty());
        assert!(decision.doctor_finding_ref.starts_with("doctor.finding."));
        covered_rungs.insert(decision.rung_class);
    }

    assert_eq!(
        covered_rungs,
        [
            RecoveryRungClass::SafeMode,
            RecoveryRungClass::RuntimeExtensionQuarantine,
            RecoveryRungClass::OpenWithoutRestore,
            RecoveryRungClass::CacheIndexRepair,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn repeated_hidden_restarts_project_to_explicit_degraded_or_quarantined_states() {
    let alpha = RecoveryLadderAlpha::new();
    let decisions = load_scenarios()
        .iter()
        .map(|scenario| alpha.evaluate(scenario).expect("scenario evaluates"))
        .collect::<Vec<_>>();

    let crash_loop = decisions
        .iter()
        .find(|decision| decision.rung_class == RecoveryRungClass::SafeMode)
        .expect("safe mode decision exists");
    assert!(crash_loop.hidden_restart_suppressed);
    assert_eq!(
        crash_loop.visible_state_class,
        RecoveryVisibleStateClass::Degraded
    );

    let quarantined = decisions
        .iter()
        .filter(|decision| decision.rung_class == RecoveryRungClass::RuntimeExtensionQuarantine)
        .collect::<Vec<_>>();
    assert_eq!(quarantined.len(), 2);
    for decision in quarantined {
        assert!(decision.hidden_restart_suppressed);
        assert_eq!(
            decision.ladder_state_class,
            RecoveryLadderStateClass::RuntimeOrExtensionQuarantined
        );
        assert_eq!(
            decision.visible_state_class,
            RecoveryVisibleStateClass::Quarantined
        );
        assert!(decision.quarantine.is_some());
    }
}

#[test]
fn support_and_release_packets_explain_quarantines_without_private_content() {
    let alpha = RecoveryLadderAlpha::new();
    let decisions = load_scenarios()
        .iter()
        .map(|scenario| alpha.evaluate(scenario).expect("scenario evaluates"))
        .collect::<Vec<_>>();

    let support = alpha.support_packet(
        "support:recovery-ladder-alpha",
        "2026-05-14T04:10:00Z",
        &decisions,
    );
    assert_eq!(
        support.record_kind,
        RECOVERY_LADDER_SUPPORT_PACKET_RECORD_KIND
    );
    assert!(support.is_export_safe());
    assert!(support.raw_private_material_excluded);
    assert!(support.ambient_authority_excluded);

    let release = alpha.release_packet(
        "release:evidence:recovery-ladder-alpha",
        "2026-05-14T04:10:00Z",
        &decisions,
    );
    assert_eq!(
        release.record_kind,
        RECOVERY_LADDER_RELEASE_PACKET_RECORD_KIND
    );
    assert!(release.is_release_safe());
    assert!(release.raw_private_material_excluded);
    assert!(release.ambient_authority_excluded);

    let quarantine_rows = release
        .rows
        .iter()
        .filter_map(|row| row.quarantine.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(quarantine_rows.len(), 2);
    for row in quarantine_rows {
        assert!(!row.lane_ref.is_empty());
        assert!(!row.owner_ref.is_empty());
        assert!(!row.expires_at.is_empty());
        assert_eq!(
            row.release_visibility,
            QuarantineReleaseVisibilityClass::ReleaseEvidence
        );
        assert!(!row.restore_conditions.is_empty());
        assert!(!row.clear_action_id.is_empty());
        assert!(!row.reenable_action_id.is_empty());
        assert!(!row.evidence_refs.is_empty());
    }
    assert!(release
        .rows
        .iter()
        .all(|row| row.doctor_finding_ref.starts_with("doctor.finding.")));
}

#[test]
fn open_without_restore_preserves_restore_store_and_names_fuller_return() {
    let alpha = RecoveryLadderAlpha::new();
    let scenario = load_scenarios()
        .into_iter()
        .find(|scenario| scenario.requested_rung == RecoveryRungClass::OpenWithoutRestore)
        .expect("open-without-restore scenario exists");
    let decision = alpha.evaluate(&scenario).expect("scenario evaluates");

    assert_eq!(
        decision.target.target_kind,
        RecoveryTargetKind::WorkspaceSession
    );
    assert_eq!(
        decision.ladder_state_class,
        RecoveryLadderStateClass::OpenedWithoutRestore
    );
    assert!(decision
        .mutation
        .preserved_state_classes
        .contains(&RecoveryStateClass::SessionRestoreStore));
    assert_eq!(
        decision.return_path.fuller_mode_class,
        FullerModeClass::RestoreEnabledEntry
    );
    assert_eq!(
        decision.return_path.return_action.action_class,
        RecoveryActionClass::ReopenWithRestoreEnabled
    );
}

#[test]
fn cache_index_repair_is_limited_to_disposable_index_state() {
    let alpha = RecoveryLadderAlpha::new();
    let scenario = load_scenarios()
        .into_iter()
        .find(|scenario| scenario.requested_rung == RecoveryRungClass::CacheIndexRepair)
        .expect("cache/index scenario exists");
    let decision = alpha.evaluate(&scenario).expect("scenario evaluates");

    assert_eq!(
        decision.target.target_kind,
        RecoveryTargetKind::CacheIndexLane
    );
    assert_eq!(
        decision.ladder_state_class,
        RecoveryLadderStateClass::CacheIndexRepairApplied
    );
    assert!(decision
        .mutation
        .disposable_state_classes
        .contains(&RecoveryStateClass::DisposableCacheIndex));
    assert!(!decision.mutation.user_owned_state_deleted);
    assert!(!decision.mutation.durable_state_deleted);
}

#[test]
fn expired_or_anonymous_quarantine_blocks_evaluation() {
    let alpha = RecoveryLadderAlpha::new();
    let mut scenario = load_scenarios()
        .into_iter()
        .find(|scenario| {
            scenario.requested_rung == RecoveryRungClass::RuntimeExtensionQuarantine
                && scenario.target.target_kind == RecoveryTargetKind::ExtensionLane
        })
        .expect("extension quarantine scenario exists");

    let quarantine = scenario
        .quarantine
        .as_mut()
        .expect("quarantine spec exists");
    quarantine.owner_ref.clear();
    quarantine.expires_at = scenario.captured_at.clone();
    scenario.entry.doctor_finding_ref.clear();

    let report = alpha
        .evaluate(&scenario)
        .expect_err("anonymous expired quarantine must be rejected");
    let check_ids = report
        .violations
        .iter()
        .map(|violation| violation.check_id.as_str())
        .collect::<BTreeSet<_>>();
    assert!(check_ids.contains("recovery_ladder.quarantine_owner_missing"));
    assert!(check_ids.contains("recovery_ladder.quarantine_expired"));
    assert!(check_ids.contains("recovery_ladder.doctor_finding_ref_missing"));
}
