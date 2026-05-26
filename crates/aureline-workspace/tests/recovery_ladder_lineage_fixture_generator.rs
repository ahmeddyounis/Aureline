//! Fixture generator helper for the recovery-ladder lineage replay
//! gate.
//!
//! Only runs when `RECOVERY_LADDER_LINEAGE_GEN_FIXTURES=1` is set in
//! the environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/recovery_ladder_lineage/` so the replay
//! gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_recovery_ladder_inspection_hooks, project_recovery_ladder_lineage_with_hooks,
    NoRerunPosture, RecoveryLadderInputs, RecoveryLadderInspectionHook,
    RecoveryLadderInspectionHookClass, RecoveryLadderLineageRecord, RecoveryRungKind,
    RecoveryRungObservation, RecoverySupportExportInputs, RecoverySupportExportPosture,
    ReversibilityClass, RungTriggerClass, UserStatePreservationPosture,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/recovery_ladder_lineage")
}

#[allow(clippy::too_many_arguments)]
fn make_rung(
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
    captured_at: &str,
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
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_rungs(captured_at: &str) -> Vec<RecoveryRungObservation> {
    vec![
        make_rung(
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
            captured_at,
        ),
        make_rung(
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
            captured_at,
        ),
        make_rung(
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
            captured_at,
        ),
        make_rung(
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
            captured_at,
        ),
        make_rung(
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
            captured_at,
        ),
    ]
}

fn extended_rungs(captured_at: &str) -> Vec<RecoveryRungObservation> {
    let mut rungs = baseline_rungs(captured_at);
    rungs.push(make_rung(
        "ladder.typed_repair_flow",
        "Typed repair flow",
        RecoveryRungKind::TypedRepairFlow,
        RungTriggerClass::TypedRepairRequested,
        "disclosure.typed_repair_flow.trigger",
        NoRerunPosture::ExplicitUserActionRequired,
        "action.typed_repair_flow.commit",
        "disclosure.typed_repair_flow.commit",
        true,
        UserStatePreservationPosture::DroppedWithDisclosure,
        "disclosure.typed_repair_flow.export_before_repair",
        ReversibilityClass::IrreversibleWithDisclosure,
        "",
        "disclosure.typed_repair_flow.irreversibility",
        captured_at,
    ));
    rungs.push(make_rung(
        "ladder.support_export_handoff",
        "Support export handoff",
        RecoveryRungKind::SupportExportHandoff,
        RungTriggerClass::SupportEscalationInitiated,
        "disclosure.support_export_handoff.trigger",
        NoRerunPosture::TerminalNoFurtherRun,
        "",
        "",
        false,
        UserStatePreservationPosture::Preserved,
        "",
        ReversibilityClass::Reversible,
        "",
        "",
        captured_at,
    ));
    rungs
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    rungs: Vec<RecoveryRungObservation>,
) -> RecoveryLadderInputs {
    RecoveryLadderInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        rungs,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a RecoveryLadderInputs,
    inspection_hooks: &'a Vec<RecoveryLadderInspectionHook>,
    expected: &'a RecoveryLadderLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: RecoveryLadderInputs,
    inspection_hooks: Vec<RecoveryLadderInspectionHook>,
) {
    let record = project_recovery_ladder_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("RECOVERY_LADDER_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline: every required rung in canonical order, every
    // privileged rung gated behind explicit-user-action, every lossy
    // rung references export-before-repair, every checkpointed rung
    // references a rollback checkpoint.
    write_fixture(
        "baseline_recovery_ladder_stable",
        "posture:baseline_recovery_ladder",
        base_inputs(
            "workspace-rust-service-0001",
            "recovery-ladder-corpus-baseline-0001",
            "mono:1700000500",
            baseline_rungs("mono:1700000500"),
        ),
        default_recovery_ladder_inspection_hooks(),
    );

    // Extended: adds the optional typed-repair-flow and
    // support-export-handoff rungs. Still Stable.
    write_fixture(
        "extended_with_typed_repair_and_support_handoff_stable",
        "posture:extended_with_typed_repair_and_support_handoff",
        base_inputs(
            "workspace-rust-service-0001",
            "recovery-ladder-corpus-extended-0001",
            "mono:1700000510",
            extended_rungs("mono:1700000510"),
        ),
        default_recovery_ladder_inspection_hooks(),
    );

    // Narrowed: a privileged rung (cache_index_repair) downgraded to
    // auto_continue_after_checkpoint. The contract must narrow with
    // `no_rerun_posture_unsafe`.
    let mut narrowed_rungs = baseline_rungs("mono:1700000520");
    let cache_repair = narrowed_rungs
        .iter_mut()
        .find(|r| r.rung_kind == RecoveryRungKind::CacheIndexRepair)
        .expect("cache index repair seeded");
    cache_repair.no_rerun_posture = NoRerunPosture::AutoContinueAfterCheckpoint;
    write_fixture(
        "cache_repair_auto_continue_narrowed",
        "posture:cache_repair_auto_continue",
        base_inputs(
            "workspace-rust-service-0001",
            "recovery-ladder-corpus-narrowed-rerun-0001",
            "mono:1700000520",
            narrowed_rungs,
        ),
        default_recovery_ladder_inspection_hooks(),
    );

    // Narrowed: required `export_before_repair` hook is unavailable on
    // this posture (e.g. degraded headless runner).
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "recovery-ladder-corpus-narrowed-hook-0001",
        "mono:1700000530",
        baseline_rungs("mono:1700000530"),
    );
    let mut narrowed_hooks = default_recovery_ladder_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == RecoveryLadderInspectionHookClass::ExportBeforeRepair {
            hook.available = false;
            hook.disclosure = "Export-before-repair unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_export_before_repair_hook_narrowed",
        "posture:missing_export_before_repair_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
