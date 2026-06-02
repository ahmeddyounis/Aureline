//! Unit tests for the recovery-ladder sequencing lineage projection.

use super::*;

#[allow(clippy::too_many_arguments)]
fn rung(
    rung_id: &str,
    title: &str,
    kind: RecoveryRungKind,
    trigger: RungTriggerClass,
    trigger_disclosure: &str,
    no_rerun: NoRerunPosture,
    commit_action: &str,
    commit_disclosure: &str,
    touches_privileged: bool,
    user_state: UserStatePreservationPosture,
    export_disclosure: &str,
    reversibility: ReversibilityClass,
    rollback_checkpoint: &str,
    irreversibility_disclosure: &str,
) -> RecoveryRungObservation {
    RecoveryRungObservation {
        rung_id: rung_id.to_owned(),
        title: title.to_owned(),
        rung_kind: kind,
        declared_step_ordinal: kind.canonical_step_ordinal(),
        trigger_class: trigger,
        trigger_disclosure_id: trigger_disclosure.to_owned(),
        no_rerun_posture: no_rerun,
        commit_action_id: commit_action.to_owned(),
        commit_disclosure_id: commit_disclosure.to_owned(),
        touches_privileged_surface: touches_privileged,
        user_state_preservation: user_state,
        export_before_repair_disclosure_id: export_disclosure.to_owned(),
        reversibility,
        rollback_checkpoint_id: rollback_checkpoint.to_owned(),
        irreversibility_disclosure_id: irreversibility_disclosure.to_owned(),
        support_export: RecoverySupportExportInputs::metadata_safe_baseline(
            RecoverySupportExportPosture::MetadataSafeExport,
        ),
        captured_at: "mono:1700000500".to_owned(),
    }
}

fn baseline_inputs() -> RecoveryLadderInputs {
    RecoveryLadderInputs {
        workspace_ref: "workspace-rust-service-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "recovery-ladder-corpus-0001".to_owned(),
        captured_at: "mono:1700000500".to_owned(),
        rungs: vec![
            rung(
                "ladder.crash_loop_safe_mode",
                "Crash-loop safe mode",
                RecoveryRungKind::CrashLoopSafeMode,
                RungTriggerClass::RepeatedCrashLoop,
                "disclosure.crash_loop_safe_mode.trigger",
                NoRerunPosture::TerminalNoFurtherRun,
                "",
                "",
                false,
                UserStatePreservationPosture::Preserved,
                "",
                ReversibilityClass::Reversible,
                "",
                "",
            ),
            rung(
                "ladder.safe_mode_quarantine",
                "Safe-mode quarantine",
                RecoveryRungKind::SafeModeQuarantine,
                RungTriggerClass::ExtensionRegressionSuspected,
                "disclosure.safe_mode_quarantine.trigger",
                NoRerunPosture::ExplicitUserActionRequired,
                "action.safe_mode_quarantine.commit",
                "disclosure.safe_mode_quarantine.commit",
                true,
                UserStatePreservationPosture::Preserved,
                "",
                ReversibilityClass::ReversibleWithCheckpoint,
                "checkpoint.safe_mode_quarantine",
                "",
            ),
            rung(
                "ladder.open_without_restore",
                "Open without restore",
                RecoveryRungKind::OpenWithoutRestore,
                RungTriggerClass::ResumeStateUnsafe,
                "disclosure.open_without_restore.trigger",
                NoRerunPosture::ExplicitUserActionRequired,
                "action.open_without_restore.commit",
                "disclosure.open_without_restore.commit",
                true,
                UserStatePreservationPosture::PreservedAfterExportPrompt,
                "disclosure.open_without_restore.export_before_repair",
                ReversibilityClass::ReversibleWithCheckpoint,
                "checkpoint.open_without_restore",
                "",
            ),
            rung(
                "ladder.cache_index_repair",
                "Cache and index repair",
                RecoveryRungKind::CacheIndexRepair,
                RungTriggerClass::DerivedStoreInconsistent,
                "disclosure.cache_index_repair.trigger",
                NoRerunPosture::ExplicitUserActionRequired,
                "action.cache_index_repair.commit",
                "disclosure.cache_index_repair.commit",
                true,
                UserStatePreservationPosture::PreservedAfterExportPrompt,
                "disclosure.cache_index_repair.export_before_repair",
                ReversibilityClass::ReversibleWithCheckpoint,
                "checkpoint.cache_index_repair",
                "",
            ),
            rung(
                "ladder.restricted_reopen",
                "Restricted reopen",
                RecoveryRungKind::RestrictedReopen,
                RungTriggerClass::TrustPostureUnverified,
                "disclosure.restricted_reopen.trigger",
                NoRerunPosture::ExplicitUserActionRequired,
                "action.restricted_reopen.commit",
                "disclosure.restricted_reopen.commit",
                true,
                UserStatePreservationPosture::Preserved,
                "",
                ReversibilityClass::Reversible,
                "",
                "",
            ),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_recovery_ladder_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, RECOVERY_LADDER_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, RECOVERY_LADDER_LINEAGE_SCHEMA_REF);
    assert!(record.rung_sequence_coverage.all_required_rungs_present);
    assert!(record.rung_sequence_coverage.all_required_steps_ordered);
    assert_eq!(record.rung_sequence_coverage.rung_rows.len(), 5);
    assert!(record.trigger_disclosure.all_rungs_have_trigger_disclosure);
    assert!(record.no_rerun_honesty.all_privileged_rungs_safe);
    assert!(record.no_rerun_honesty.all_explicit_rungs_have_metadata);
    assert_eq!(
        record.user_state_preservation.user_state_lossy_rung_count,
        2
    );
    assert!(
        record
            .user_state_preservation
            .all_lossy_rungs_have_export_disclosure
    );
    assert_eq!(record.reversibility_truth.checkpointed_rung_count, 3);
    assert!(
        record
            .reversibility_truth
            .all_checkpointed_rungs_have_checkpoint_id
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("rll:"));
}

#[test]
fn missing_required_rung_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .rungs
        .retain(|r| r.rung_kind != RecoveryRungKind::RestrictedReopen);

    let record = project_recovery_ladder_lineage("posture.missing_restricted", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::RequiredRungMissing));
}

#[test]
fn unordered_step_ordinal_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::OpenWithoutRestore)
        .expect("open without restore seeded");
    row.declared_step_ordinal = 1;

    let record = project_recovery_ladder_lineage("posture.unordered", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::RungSequenceUnordered));
}

#[test]
fn missing_trigger_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::SafeModeQuarantine)
        .expect("safe mode quarantine seeded");
    row.trigger_disclosure_id = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.missing_trigger", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::RungTriggerMissingDisclosure));
}

#[test]
fn privileged_rung_with_auto_continue_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::CacheIndexRepair)
        .expect("cache index repair seeded");
    row.no_rerun_posture = NoRerunPosture::AutoContinueAfterCheckpoint;

    let record = project_recovery_ladder_lineage("posture.auto_continue", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::NoRerunPostureUnsafe));
}

#[test]
fn explicit_rung_without_action_metadata_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::RestrictedReopen)
        .expect("restricted reopen seeded");
    row.commit_action_id = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.missing_action", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::ExplicitActionMetadataMissing));
}

#[test]
fn lossy_rung_without_export_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::OpenWithoutRestore)
        .expect("open without restore seeded");
    row.export_before_repair_disclosure_id = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.lossy_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::UserStateLossUndisclosed));
}

#[test]
fn checkpointed_rung_without_checkpoint_id_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::CacheIndexRepair)
        .expect("cache index repair seeded");
    row.rollback_checkpoint_id = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.no_checkpoint", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::ReversibilityCheckpointMissing));
}

#[test]
fn irreversible_rung_without_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::RestrictedReopen)
        .expect("restricted reopen seeded");
    row.reversibility = ReversibilityClass::IrreversibleWithDisclosure;

    let record = project_recovery_ladder_lineage("posture.no_irrev_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::IrreversibleRungMissingDisclosure));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = baseline_inputs();
    let mut hooks = default_recovery_ladder_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == RecoveryLadderInspectionHookClass::ExportBeforeRepair {
            hook.available = false;
        }
    }

    let record = project_recovery_ladder_lineage_with_hooks(
        "posture.no_export_before_repair",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_dropping_fields_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.rungs[0].support_export.includes_no_rerun_posture = false;

    let record = project_recovery_ladder_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn support_export_raising_raw_secrets_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.rungs[0].support_export.raw_secrets_excluded = false;

    let record = project_recovery_ladder_lineage("posture.raw_secrets", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.empty_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.rungs.clear();

    let record = project_recovery_ladder_lineage("posture.empty_corpus", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::CorpusEmpty));
}

#[test]
fn producer_attribution_incomplete_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref = "".to_owned();

    let record = project_recovery_ladder_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryLadderLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = baseline_inputs();
    let record = project_recovery_ladder_lineage("posture.lines", &inputs);
    let lines = recovery_ladder_lineage_lines(&record);

    assert!(lines
        .iter()
        .any(|line| line.contains("Recovery-ladder lineage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("rung_sequence_coverage")));
    assert!(lines.iter().any(|line| line == "Recovery rungs:"));
    assert!(lines.iter().any(|line| line.contains("Trigger disclosure")));
    assert!(lines.iter().any(|line| line.contains("No-rerun honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("User-state preservation")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Reversibility truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = baseline_inputs();
    let record = project_recovery_ladder_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: RecoveryLadderLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
