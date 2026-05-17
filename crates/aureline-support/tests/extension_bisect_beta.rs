//! Protected tests for the extension-bisect orchestration beta evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::extension_bisect::{
    load_extension_bisect_finding, load_extension_bisect_restore, load_extension_bisect_session,
    load_extension_bisect_step, BisectEntryReasonClass, BisectSessionClass, BlastRadiusClass,
    CohortActivationResultClass, DisabledCapabilityClass, EscalationActionClass,
    ExtensionBisectEvaluator, ExtensionBisectFinding, ExtensionBisectRestore,
    ExtensionBisectSession, ExtensionBisectStep, FindingClass, PreservedCapabilityClass,
    PreservedStateClass, RestoreDispositionClass, ReviewGateClass, StepClass, StepVerdictClass,
    EXTENSION_BISECT_DOC_REF, EXTENSION_BISECT_FINDING_RECORD_KIND,
    EXTENSION_BISECT_RESTORE_RECORD_KIND, EXTENSION_BISECT_SCHEMA_REF,
    EXTENSION_BISECT_SESSION_RECORD_KIND, EXTENSION_BISECT_STEP_RECORD_KIND,
    EXTENSION_BISECT_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    scenarios: Vec<ScenarioEntry>,
}

#[derive(Debug, Deserialize)]
struct ScenarioEntry {
    scenario_id: String,
    session_file: String,
    step_files: Vec<String>,
    finding_file: String,
    restore_file: String,
}

struct LoadedScenario {
    scenario_id: String,
    session: ExtensionBisectSession,
    steps: Vec<ExtensionBisectStep>,
    finding: ExtensionBisectFinding,
    restore: ExtensionBisectRestore,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/recovery/m3/extension_bisect")
}

fn read_yaml(file: &str) -> String {
    let path = fixture_dir().join(file);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"))
}

fn load_manifest() -> Manifest {
    let yaml = read_yaml("manifest.yaml");
    serde_yaml::from_str(&yaml).expect("parse manifest")
}

fn load_scenarios() -> Vec<LoadedScenario> {
    load_manifest()
        .scenarios
        .into_iter()
        .map(|entry| LoadedScenario {
            scenario_id: entry.scenario_id.clone(),
            session: load_extension_bisect_session(&read_yaml(&entry.session_file))
                .unwrap_or_else(|err| panic!("parse {}: {err}", entry.session_file)),
            steps: entry
                .step_files
                .iter()
                .map(|file| {
                    load_extension_bisect_step(&read_yaml(file))
                        .unwrap_or_else(|err| panic!("parse {file}: {err}"))
                })
                .collect(),
            finding: load_extension_bisect_finding(&read_yaml(&entry.finding_file))
                .unwrap_or_else(|err| panic!("parse {}: {err}", entry.finding_file)),
            restore: load_extension_bisect_restore(&read_yaml(&entry.restore_file))
                .unwrap_or_else(|err| panic!("parse {}: {err}", entry.restore_file)),
        })
        .collect()
}

#[test]
fn bisect_sessions_preserve_tested_states_suspected_sets_and_user_visible_findings() {
    let evaluator = ExtensionBisectEvaluator::new();
    let scenarios = load_scenarios();
    assert_eq!(scenarios.len(), 3);

    let mut covered_session_classes = BTreeSet::new();
    let mut covered_finding_classes = BTreeSet::new();
    for scenario in &scenarios {
        evaluator
            .validate_session(&scenario.session)
            .unwrap_or_else(|err| panic!("{} session failed: {err:?}", scenario.scenario_id));
        assert_eq!(
            scenario.session.record_kind,
            EXTENSION_BISECT_SESSION_RECORD_KIND
        );
        assert!(!scenario.session.candidate_extensions.is_empty());
        assert!(!scenario.session.tested_states.is_empty());
        assert!(!scenario.session.suspected_extension_sets.is_empty());
        assert!(!scenario.session.findings.is_empty());
        assert!(!scenario.session.destructive_resets_present);
        assert!(!scenario.session.user_owned_state_deleted);
        assert!(!scenario.session.durable_state_deleted);

        for step in &scenario.steps {
            evaluator
                .validate_step(step)
                .unwrap_or_else(|err| panic!("step {} failed: {err:?}", step.step_id));
            assert_eq!(step.record_kind, EXTENSION_BISECT_STEP_RECORD_KIND);
        }

        evaluator
            .validate_finding(&scenario.finding)
            .unwrap_or_else(|err| panic!("finding failed: {err:?}"));
        assert_eq!(
            scenario.finding.record_kind,
            EXTENSION_BISECT_FINDING_RECORD_KIND
        );

        evaluator
            .validate_restore(&scenario.restore)
            .unwrap_or_else(|err| panic!("restore failed: {err:?}"));
        assert_eq!(
            scenario.restore.record_kind,
            EXTENSION_BISECT_RESTORE_RECORD_KIND
        );

        covered_session_classes.insert(scenario.session.session_class);
        covered_finding_classes.insert(scenario.finding.finding_class);
    }

    assert_eq!(
        covered_session_classes,
        [
            BisectSessionClass::PostCrashLoopSession,
            BisectSessionClass::RegressionSuspectedSession,
            BisectSessionClass::PolicyForcedSession,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );

    assert!(covered_finding_classes.contains(&FindingClass::SingleExtensionSuspected));
    assert!(covered_finding_classes.contains(&FindingClass::MultiExtensionSuspected));
    assert!(covered_finding_classes.contains(&FindingClass::BisectAbortedNoFinding));
}

#[test]
fn bisect_disables_only_minimal_scope_and_restores_prior_state_afterward() {
    let evaluator = ExtensionBisectEvaluator::new();
    let mut covered_restore_dispositions = BTreeSet::new();
    let mut covered_blast_radius = BTreeSet::new();

    for scenario in load_scenarios() {
        for required in [
            PreservedCapabilityClass::LocalEditing,
            PreservedCapabilityClass::BasicNavigation,
            PreservedCapabilityClass::LocalDiagnosticsExport,
            PreservedCapabilityClass::SupportBundlePreview,
            PreservedCapabilityClass::ProjectDoctorSurfaces,
            PreservedCapabilityClass::ExtensionBisectExitAction,
        ] {
            assert!(
                scenario.session.preserved_capability_classes.contains(&required),
                "session {} missing preserved capability {:?}",
                scenario.session.session_id,
                required
            );
        }
        for required in [
            PreservedStateClass::UserAuthoredFiles,
            PreservedStateClass::OpenBufferSelection,
            PreservedStateClass::WorkspaceTrustStore,
            PreservedStateClass::CredentialStore,
            PreservedStateClass::SessionRestoreStore,
            PreservedStateClass::SupportExportStore,
            PreservedStateClass::ExtensionPriorStateSnapshot,
        ] {
            assert!(
                scenario.session.preserved_state_classes.contains(&required),
                "session {} missing preserved state {:?}",
                scenario.session.session_id,
                required
            );
        }
        assert!(scenario
            .session
            .disabled_capability_classes
            .contains(&DisabledCapabilityClass::ExtensionAutoActivation));
        assert!(!scenario
            .session
            .disabled_capability_classes
            .contains(&DisabledCapabilityClass::RemoteHelperAttach)
            || scenario.session.blast_radius_class == BlastRadiusClass::FullExtensionHost
            || scenario.session.blast_radius_class == BlastRadiusClass::AllThirdPartyExtensions
            || scenario.session.blast_radius_class == BlastRadiusClass::CohortOnly
            || scenario.session.blast_radius_class == BlastRadiusClass::SingleExtensionLane);

        // The restore record returns the prior extension state and observes
        // user-owned and durable state preservation.
        assert!(scenario
            .restore
            .preserved_state_classes_observed
            .contains(&PreservedStateClass::UserAuthoredFiles));
        assert!(scenario
            .restore
            .preserved_state_classes_observed
            .contains(&PreservedStateClass::ExtensionPriorStateSnapshot));
        assert!(!scenario.restore.user_owned_state_deleted);
        assert!(!scenario.restore.durable_state_deleted);
        assert!(scenario.restore.restore_disposition_class.is_admissible());

        // Every restored extension row references a declared candidate.
        let declared_ids: BTreeSet<&str> = scenario
            .session
            .candidate_extensions
            .iter()
            .map(|candidate| candidate.extension_id.as_str())
            .collect();
        for row in &scenario.restore.restored_extensions {
            assert!(
                declared_ids.contains(row.extension_id.as_str()),
                "restore row {} references unknown candidate",
                row.extension_id
            );
        }

        covered_restore_dispositions.insert(scenario.restore.restore_disposition_class);
        covered_blast_radius.insert(scenario.session.blast_radius_class);

        // Bind to the evaluator for an end-to-end check.
        evaluator
            .support_packet(
                format!("support:extension-bisect-beta:{}", scenario.scenario_id),
                "2026-05-15T15:00:00Z",
                &scenario.session,
                &scenario.steps,
                &scenario.finding,
                &scenario.restore,
            )
            .unwrap_or_else(|err| panic!("packet build failed: {err:?}"));
    }

    // The corpus must exercise restore dispositions that prove the prior
    // state was restored exactly, restored with quarantine of the suspect,
    // and left unchanged when the bisect aborted.
    for required in [
        RestoreDispositionClass::PriorStateRestoredExact,
        RestoreDispositionClass::PriorStateRestoredWithQuarantine,
        RestoreDispositionClass::PriorStateUnchanged,
    ] {
        assert!(
            covered_restore_dispositions.contains(&required),
            "corpus missing restore disposition {:?}",
            required
        );
    }
    assert!(covered_blast_radius.contains(&BlastRadiusClass::CohortOnly));
    assert!(covered_blast_radius.contains(&BlastRadiusClass::AllThirdPartyExtensions));
}

#[test]
fn support_packet_excludes_raw_private_content_and_ambient_authority() {
    let evaluator = ExtensionBisectEvaluator::new();
    for scenario in load_scenarios() {
        let packet = evaluator
            .support_packet(
                format!("support:extension-bisect-beta:{}", scenario.scenario_id),
                "2026-05-15T15:00:00Z",
                &scenario.session,
                &scenario.steps,
                &scenario.finding,
                &scenario.restore,
            )
            .unwrap_or_else(|err| panic!("{} packet failed: {err:?}", scenario.scenario_id));

        assert_eq!(
            packet.record_kind,
            EXTENSION_BISECT_SUPPORT_PACKET_RECORD_KIND
        );
        assert_eq!(packet.doc_ref, EXTENSION_BISECT_DOC_REF);
        assert_eq!(packet.schema_ref, EXTENSION_BISECT_SCHEMA_REF);
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert!(!packet.destructive_resets_present);
        assert!(packet.is_export_safe());
        assert!(packet.doctor_finding_ref.starts_with("doctor.finding."));
        assert!(packet
            .preserved_capability_classes
            .contains(&PreservedCapabilityClass::LocalEditing));
        assert!(packet
            .preserved_state_classes
            .contains(&PreservedStateClass::UserAuthoredFiles));
        assert_eq!(packet.step_rows.len(), scenario.steps.len());
        assert!(!packet.candidate_rows.is_empty());
        assert!(!packet.user_finding_rows.is_empty());
    }
}

#[test]
fn post_crash_loop_scenario_attributes_a_single_extension_and_quarantines_it_at_restore() {
    let evaluator = ExtensionBisectEvaluator::new();
    let scenario = load_scenarios()
        .into_iter()
        .find(|loaded| loaded.scenario_id == "post_crash_loop_single_suspect")
        .expect("post-crash-loop scenario exists");

    assert_eq!(
        scenario.session.session_class,
        BisectSessionClass::PostCrashLoopSession
    );
    assert_eq!(
        scenario.session.entry_reason_class,
        BisectEntryReasonClass::CrashLoopDetected
    );
    assert_eq!(
        scenario.session.review_gate_class,
        ReviewGateClass::UserConfirmationRequired
    );
    assert_eq!(
        scenario.finding.finding_class,
        FindingClass::SingleExtensionSuspected
    );
    assert_eq!(scenario.finding.suspect_extension_refs.len(), 1);
    assert!(scenario
        .finding
        .escalation_actions
        .contains(&EscalationActionClass::EscalateToExtensionQuarantine));
    assert_eq!(
        scenario.restore.restore_disposition_class,
        RestoreDispositionClass::PriorStateRestoredWithQuarantine
    );

    let baseline_step_count = scenario
        .steps
        .iter()
        .filter(|step| {
            matches!(
                step.step_class,
                StepClass::InitialBaselineCheck | StepClass::ExitBaselineCheck
            )
        })
        .count();
    assert_eq!(baseline_step_count, 2);

    evaluator
        .support_packet(
            "support:extension-bisect-beta:post-crash-loop",
            "2026-05-15T15:00:00Z",
            &scenario.session,
            &scenario.steps,
            &scenario.finding,
            &scenario.restore,
        )
        .expect("packet builds");
}

#[test]
fn evaluator_refuses_destructive_or_inconsistent_records() {
    let evaluator = ExtensionBisectEvaluator::new();
    let scenario = load_scenarios()
        .into_iter()
        .find(|loaded| loaded.scenario_id == "post_crash_loop_single_suspect")
        .expect("post-crash-loop scenario exists");

    // Refuses a session that drops the user-authored-files preservation.
    let mut without_user_files = scenario.session.clone();
    without_user_files
        .preserved_state_classes
        .retain(|class| *class != PreservedStateClass::UserAuthoredFiles);
    let report = evaluator
        .validate_session(&without_user_files)
        .expect_err("must reject session without user-authored-files preservation");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.user_authored_files_must_be_preserved"));

    // Refuses a session that drops the prior-state snapshot preservation.
    let mut without_snapshot = scenario.session.clone();
    without_snapshot
        .preserved_state_classes
        .retain(|class| *class != PreservedStateClass::ExtensionPriorStateSnapshot);
    let report = evaluator
        .validate_session(&without_snapshot)
        .expect_err("must reject session without prior-state snapshot preservation");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.prior_state_snapshot_must_be_preserved"));

    // Refuses a session that declares destructive resets.
    let mut destructive_session = scenario.session.clone();
    destructive_session.destructive_resets_present = true;
    let report = evaluator
        .validate_session(&destructive_session)
        .expect_err("must reject destructive session");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.destructive_reset_declared"));

    // Refuses a session that omits the extension_auto_activation gate.
    let mut without_gate = scenario.session.clone();
    without_gate
        .disabled_capability_classes
        .retain(|class| *class != DisabledCapabilityClass::ExtensionAutoActivation);
    let report = evaluator
        .validate_session(&without_gate)
        .expect_err("must reject session without auto-activation gate");
    assert!(report.violations.iter().any(|v| v.check_id
        == "extension_bisect.extension_auto_activation_must_be_disabled"));

    // Refuses a step that deletes user-owned state.
    let mut destructive_step = scenario.steps[0].clone();
    destructive_step.user_owned_state_deleted = true;
    let report = evaluator
        .validate_step(&destructive_step)
        .expect_err("must reject destructive step");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.step_deletes_user_owned_state"));

    // Refuses a cohort_activation step with no members.
    let mut empty_cohort = scenario.steps[1].clone();
    empty_cohort.cohort_member_refs.clear();
    let report = evaluator
        .validate_step(&empty_cohort)
        .expect_err("must reject cohort step with empty members");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.cohort_step_missing_members"));

    // Refuses an aborted activation that does not record the matching verdict.
    let mut crossed_step = scenario.steps[1].clone();
    crossed_step.activation_result_class = CohortActivationResultClass::AbortedByPolicy;
    crossed_step.verdict_class = StepVerdictClass::CohortCleared;
    let report = evaluator
        .validate_step(&crossed_step)
        .expect_err("must reject aborted step with mismatched verdict");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.aborted_step_verdict_mismatch"));

    // Refuses a finding that names single-suspicion without suspect refs.
    let mut finding_missing_suspect = scenario.finding.clone();
    finding_missing_suspect.suspect_extension_refs.clear();
    let report = evaluator
        .validate_finding(&finding_missing_suspect)
        .expect_err("must reject single-extension finding without suspect");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.finding_missing_suspect_refs"));

    // Refuses a restore that deletes user-owned state.
    let mut destructive_restore = scenario.restore.clone();
    destructive_restore.user_owned_state_deleted = true;
    let report = evaluator
        .validate_restore(&destructive_restore)
        .expect_err("must reject destructive restore");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.restore_deletes_user_owned_state"));

    // Refuses a restore that drops user-authored-files preservation.
    let mut restore_without_user_files = scenario.restore.clone();
    restore_without_user_files
        .preserved_state_classes_observed
        .retain(|class| *class != PreservedStateClass::UserAuthoredFiles);
    let report = evaluator
        .validate_restore(&restore_without_user_files)
        .expect_err("must reject restore without user-authored-files");
    assert!(report.violations.iter().any(|v| v.check_id
        == "extension_bisect.restore_must_preserve_user_authored_files"));

    // Refuses a restore whose disposition is pending review.
    let mut deferred_restore = scenario.restore.clone();
    deferred_restore.restore_disposition_class =
        RestoreDispositionClass::RestoreDeferredPendingReview;
    let report = evaluator
        .validate_restore(&deferred_restore)
        .expect_err("must reject deferred restore at packet time");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.restore_disposition_pending_review"));

    // support_packet refuses mismatched session/finding/restore refs.
    let mut mismatched_finding = scenario.finding.clone();
    mismatched_finding.session_ref = "extension_bisect_session:does_not_exist".to_owned();
    let report = evaluator
        .support_packet(
            "support:extension-bisect-beta:mismatch",
            "2026-05-15T15:00:00Z",
            &scenario.session,
            &scenario.steps,
            &mismatched_finding,
            &scenario.restore,
        )
        .expect_err("must reject finding with mismatched session_ref");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "extension_bisect.finding_session_ref_mismatch"));
}
