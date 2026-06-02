//! Protected tests for the hardened recovery-ladder flow evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::{
    load_hardened_recovery_flow, HardenedRecoveryBlastRadiusClass, HardenedRecoveryCheckpointClass,
    HardenedRecoveryFlowEvaluator, HardenedRecoveryFlowRecord,
    HardenedRecoveryPreservedStateClass, HardenedRecoveryRedactionClass, HardenedRecoveryReversalClass,
    HARDENED_RECOVERY_FLOW_ARTIFACT_REF, HARDENED_RECOVERY_FLOW_DOC_REF,
    HARDENED_RECOVERY_FLOW_FIXTURE_DIR, HARDENED_RECOVERY_FLOW_RECORD_KIND,
    HARDENED_RECOVERY_FLOW_SCHEMA_REF, HARDENED_RECOVERY_FLOW_SCHEMA_VERSION,
    HARDENED_RECOVERY_FLOW_SUPPORT_PACKET_RECORD_KIND,
};
use aureline_support::recovery_ladder::RecoveryRungClass;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    flow_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(
        "fixtures/support/m4/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets",
    )
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_flows() -> Vec<HardenedRecoveryFlowRecord> {
    load_manifest()
        .flow_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_hardened_recovery_flow(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn hardened_recovery_flows_match_schema_version_and_record_kind() {
    let flows = load_flows();
    assert_eq!(flows.len(), 4);

    for flow in &flows {
        assert_eq!(flow.schema_version, HARDENED_RECOVERY_FLOW_SCHEMA_VERSION);
        assert_eq!(flow.record_kind, HARDENED_RECOVERY_FLOW_RECORD_KIND);
    }
}

#[test]
fn hardened_recovery_flows_preserve_user_authored_files() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    for flow in &load_flows() {
        evaluator
            .validate_flow(flow)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", flow.flow_id));

        assert!(
            flow.preserved_state_classes
                .contains(&HardenedRecoveryPreservedStateClass::UserAuthoredFiles),
            "flow {} must preserve user-authored files",
            flow.flow_id
        );
    }
}

#[test]
fn hardened_recovery_flows_cite_doctor_finding_ref() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    for flow in &load_flows() {
        evaluator
            .validate_flow(flow)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", flow.flow_id));

        assert!(
            flow.doctor_finding_ref.starts_with("doctor.finding."),
            "flow {} must cite a doctor.finding ref",
            flow.flow_id
        );
    }
}

#[test]
fn hardened_recovery_flows_have_non_empty_support_guidance_and_summary() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    for flow in &load_flows() {
        evaluator
            .validate_flow(flow)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", flow.flow_id));

        assert!(
            !flow.support_guidance.trim().is_empty(),
            "flow {} support_guidance must be non-empty",
            flow.flow_id
        );
        assert!(
            !flow.flow_summary.trim().is_empty(),
            "flow {} flow_summary must be non-empty",
            flow.flow_id
        );
    }
}

#[test]
fn hardened_recovery_flows_have_at_least_one_evidence_ref() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    for flow in &load_flows() {
        evaluator
            .validate_flow(flow)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", flow.flow_id));

        assert!(
            !flow.evidence_refs.is_empty(),
            "flow {} must cite at least one evidence ref",
            flow.flow_id
        );
    }
}

#[test]
fn cache_rebuild_flow_binds_to_cache_index_repair_with_regenerate_or_exact() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let cache_rebuild = flows
        .iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:cache_rebuild.alpha_v1")
        .expect("cache_rebuild flow present");

    evaluator
        .validate_flow(cache_rebuild)
        .unwrap_or_else(|err| panic!("{} failed: {err:?}", cache_rebuild.flow_id));

    assert_eq!(cache_rebuild.rung_class, RecoveryRungClass::CacheIndexRepair);
    assert_eq!(
        cache_rebuild.blast_radius_class,
        HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass
    );
    assert!(
        matches!(
            cache_rebuild.reversal_class,
            HardenedRecoveryReversalClass::Regenerate | HardenedRecoveryReversalClass::Exact
        ),
        "cache_rebuild reversal must be regenerate or exact"
    );
}

#[test]
fn settings_repair_flow_binds_to_settings_repair_with_durable_checkpoint() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let settings_repair = flows
        .iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:settings_repair.alpha_v1")
        .expect("settings_repair flow present");

    evaluator
        .validate_flow(settings_repair)
        .unwrap_or_else(|err| panic!("{} failed: {err:?}", settings_repair.flow_id));

    assert_eq!(settings_repair.rung_class, RecoveryRungClass::SettingsRepair);
    assert_eq!(
        settings_repair.checkpoint_class,
        HardenedRecoveryCheckpointClass::DurablePreApply
    );
    assert!(
        settings_repair
            .preserved_state_classes
            .contains(&HardenedRecoveryPreservedStateClass::SettingsProfileState),
        "settings_repair must preserve settings_profile_state"
    );
}

#[test]
fn state_migration_repair_flow_binds_to_state_migration_repair_with_admin_consent() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let state_migration = flows
        .iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:state_migration_repair.alpha_v1")
        .expect("state_migration_repair flow present");

    evaluator
        .validate_flow(state_migration)
        .unwrap_or_else(|err| panic!("{} failed: {err:?}", state_migration.flow_id));

    assert_eq!(
        state_migration.rung_class,
        RecoveryRungClass::StateMigrationRepair
    );
    assert_eq!(
        state_migration.checkpoint_class,
        HardenedRecoveryCheckpointClass::DurablePreApply
    );
    assert!(
        state_migration.requires_admin_consent,
        "state_migration_repair requires admin consent"
    );
    assert!(
        state_migration
            .preserved_state_classes
            .contains(&HardenedRecoveryPreservedStateClass::DurableWorkspaceIndexes),
        "state_migration_repair must preserve durable_workspace_indexes"
    );
    assert!(
        state_migration
            .preserved_state_classes
            .contains(&HardenedRecoveryPreservedStateClass::StateMigrationJournal),
        "state_migration_repair must preserve state_migration_journal"
    );
}

#[test]
fn targeted_reset_flow_binds_to_targeted_reset_with_exactly_one_impacted_state() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let targeted_reset = flows
        .iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:targeted_reset.alpha_v1")
        .expect("targeted_reset flow present");

    evaluator
        .validate_flow(targeted_reset)
        .unwrap_or_else(|err| panic!("{} failed: {err:?}", targeted_reset.flow_id));

    assert_eq!(targeted_reset.rung_class, RecoveryRungClass::TargetedReset);
    assert_eq!(
        targeted_reset.blast_radius_class,
        HardenedRecoveryBlastRadiusClass::SingleDisposableStateClass
    );
    assert_eq!(
        targeted_reset.impacted_state_classes.len(),
        1,
        "targeted_reset must declare exactly one impacted state class"
    );
    assert_eq!(
        targeted_reset.checkpoint_class,
        HardenedRecoveryCheckpointClass::DurablePreApply
    );
}

#[test]
fn hardened_recovery_flows_cover_all_required_flow_classes() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let covered: BTreeSet<_> = flows.iter().map(|f| f.flow_class).collect();

    for required in [
        aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::HardenedRecoveryFlowClass::CacheRebuild,
        aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::HardenedRecoveryFlowClass::SettingsRepair,
        aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::HardenedRecoveryFlowClass::StateMigrationRepair,
        aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::HardenedRecoveryFlowClass::TargetedReset,
    ] {
        assert!(
            covered.contains(&required),
            "missing required flow class {:?}",
            required
        );
    }

    for flow in &flows {
        evaluator
            .validate_flow(flow)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", flow.flow_id));
    }
}

#[test]
fn support_packet_is_metadata_safe_and_redacted_by_default() {
    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let flows = load_flows();

    let packet = evaluator
        .support_packet_from_flows(&flows)
        .expect("support packet builds from all flows");

    assert_eq!(packet.schema_version, HARDENED_RECOVERY_FLOW_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        HARDENED_RECOVERY_FLOW_SUPPORT_PACKET_RECORD_KIND
    );
    assert!(packet.all_flows_valid);
    assert_eq!(packet.flow_refs.len(), flows.len());
    assert!(
        matches!(
            packet.redaction_class,
            HardenedRecoveryRedactionClass::MetadataSafeDefault
        ),
        "support packet must be metadata-safe by default"
    );
}

#[test]
fn evaluator_refuses_flow_without_user_authored_files() {
    let mut flow = load_flows()
        .into_iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:cache_rebuild.alpha_v1")
        .expect("cache_rebuild flow present");

    flow.preserved_state_classes
        .retain(|c| *c != HardenedRecoveryPreservedStateClass::UserAuthoredFiles);

    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let err = evaluator.validate_flow(&flow).expect_err("should fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "hardened_recovery_flow.user_authored_files_not_preserved"));
}

#[test]
fn evaluator_refuses_targeted_reset_with_two_impacted_states() {
    use aureline_support::harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets::HardenedRecoveryImpactedStateClass;

    let mut flow = load_flows()
        .into_iter()
        .find(|f| f.flow_id == "hardened_recovery_flow:targeted_reset.alpha_v1")
        .expect("targeted_reset flow present");

    flow.impacted_state_classes.push(HardenedRecoveryImpactedStateClass::WatcherBacklogState);

    let evaluator = HardenedRecoveryFlowEvaluator::new();
    let err = evaluator.validate_flow(&flow).expect_err("should fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "hardened_recovery_flow.targeted_reset_impacted_state_count"));
}

#[test]
fn schema_ref_and_doc_ref_are_repo_relative() {
    assert!(HARDENED_RECOVERY_FLOW_SCHEMA_REF.starts_with("schemas/"));
    assert!(HARDENED_RECOVERY_FLOW_DOC_REF.starts_with("docs/"));
    assert!(HARDENED_RECOVERY_FLOW_ARTIFACT_REF.starts_with("artifacts/"));
    assert!(HARDENED_RECOVERY_FLOW_FIXTURE_DIR.starts_with("fixtures/"));
}
