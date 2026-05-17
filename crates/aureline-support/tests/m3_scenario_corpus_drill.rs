//! Protected drill-harness baseline for the M3 support-scenario corpus.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::m3_scenario_corpus::{
    current_m3_scenario_corpus, load_m3_support_scenario, required_beta_lane_classes,
    M3BetaLaneClass, M3ClaimDowngradeTriggerClass, M3DrillStepClass,
    M3_DRILL_HARNESS_LANE_ROW_RECORD_KIND, M3_DRILL_HARNESS_REPORT_RECORD_KIND,
    M3_DRILL_HARNESS_REPORT_REF, M3_SCENARIO_CORPUS_DIR, M3_SCENARIO_CORPUS_DOC_REF,
    M3_SCENARIO_CORPUS_MANIFEST_REF, M3_SUPPORT_SCENARIO_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn corpus_validates_against_the_beta_lane_coverage_contract() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
    assert_eq!(corpus.entries.len(), required_beta_lane_classes().len());

    let lane_set = corpus
        .scenarios()
        .map(|scenario| scenario.beta_lane_class)
        .collect::<BTreeSet<_>>();
    let expected = required_beta_lane_classes()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(lane_set, expected);
}

#[test]
fn every_scenario_pins_repo_relative_doc_schema_crate_and_test_refs_that_exist() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let scenario = &entry.scenario;
        assert_eq!(scenario.record_kind, M3_SUPPORT_SCENARIO_RECORD_KIND);
        let refs = &scenario.beta_lane_refs;
        assert!(
            root.join(&refs.doc_ref).is_file(),
            "{}: doc_ref {} missing",
            scenario.scenario_id,
            refs.doc_ref
        );
        assert!(
            root.join(&refs.schema_ref).is_file(),
            "{}: schema_ref {} missing",
            scenario.scenario_id,
            refs.schema_ref
        );
        assert!(
            root.join(&refs.crate_consumer).is_file(),
            "{}: crate_consumer {} missing",
            scenario.scenario_id,
            refs.crate_consumer
        );
        assert!(
            root.join(&refs.integration_test).is_file(),
            "{}: integration_test {} missing",
            scenario.scenario_id,
            refs.integration_test
        );
        for fixture in &scenario.primary_fixture_refs {
            assert!(
                root.join(fixture).is_file(),
                "{}: primary_fixture_ref {} missing",
                scenario.scenario_id,
                fixture
            );
        }
        assert!(
            root.join(&entry.fixture_ref).is_file(),
            "{}: scenario fixture {} missing",
            scenario.scenario_id,
            entry.fixture_ref
        );
    }
}

#[test]
fn every_scenario_declares_export_support_packet_step_and_downgrade_baseline() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    for scenario in corpus.scenarios() {
        assert!(
            scenario
                .drill_steps
                .iter()
                .any(|step| step.step_class == M3DrillStepClass::ExportSupportPacket),
            "{} must include an export_support_packet step",
            scenario.scenario_id
        );

        let triggers = scenario
            .claim_downgrade_rules
            .iter()
            .map(|rule| rule.trigger_class)
            .collect::<BTreeSet<_>>();
        for required in [
            M3ClaimDowngradeTriggerClass::FixtureMissing,
            M3ClaimDowngradeTriggerClass::DrillStepUnproven,
            M3ClaimDowngradeTriggerClass::DrillProvesRegression,
        ] {
            assert!(
                triggers.contains(&required),
                "{} missing required downgrade trigger {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn drill_harness_report_projects_one_metadata_safe_row_per_required_lane() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    let report = corpus.drill_harness_report("m3.drill_harness_report.v1", "2026-05-16T00:00:00Z");
    assert_eq!(report.record_kind, M3_DRILL_HARNESS_REPORT_RECORD_KIND);
    assert_eq!(report.lane_rows.len(), required_beta_lane_classes().len());
    assert!(report.is_export_safe());
    assert_eq!(report.corpus_manifest_ref, M3_SCENARIO_CORPUS_MANIFEST_REF);
    assert_eq!(report.corpus_doc_ref, M3_SCENARIO_CORPUS_DOC_REF);

    let lane_set = report
        .lane_rows
        .iter()
        .map(|row| row.beta_lane_class)
        .collect::<BTreeSet<_>>();
    for required in required_beta_lane_classes() {
        assert!(
            lane_set.contains(required),
            "missing lane row for {}",
            required.as_str()
        );
    }
    for row in &report.lane_rows {
        assert_eq!(row.record_kind, M3_DRILL_HARNESS_LANE_ROW_RECORD_KIND);
        assert!(row.metadata_safe_baseline_met);
        assert!(row.contributes_to_release_gate);
        assert!(!row.scorecard_target.is_empty());
        assert!(row
            .claim_downgrade_trigger_classes
            .iter()
            .any(|trigger| trigger == "fixture_missing"));
    }
}

#[test]
fn yaml_round_trip_load_matches_corpus_entry() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let scenario = load_m3_support_scenario(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(scenario, entry.scenario);
    }
}

#[test]
fn corpus_doc_and_drill_report_exist_at_declared_paths() {
    let root = repo_root();
    assert!(
        root.join(M3_SCENARIO_CORPUS_DIR).is_dir(),
        "corpus dir {M3_SCENARIO_CORPUS_DIR} missing"
    );
    assert!(
        root.join(M3_SCENARIO_CORPUS_MANIFEST_REF).is_file(),
        "manifest {M3_SCENARIO_CORPUS_MANIFEST_REF} missing"
    );
    assert!(
        root.join(M3_SCENARIO_CORPUS_DOC_REF).is_file(),
        "reviewer doc {M3_SCENARIO_CORPUS_DOC_REF} missing"
    );
    assert!(
        root.join(M3_DRILL_HARNESS_REPORT_REF).is_file(),
        "drill report {M3_DRILL_HARNESS_REPORT_REF} missing"
    );
}

#[test]
fn corpus_refuses_a_yaml_record_with_wrong_record_kind() {
    let yaml = r#"
schema_version: 1
record_kind: not_m3_support_scenario_record
scenario_id: bogus
title: bogus
beta_lane_class: safe_mode
beta_lane_refs:
  doc_ref: docs/support/m3/safe_mode_beta.md
  schema_ref: schemas/support/safe_mode_profile.schema.json
  crate_consumer: crates/aureline-support/src/safe_mode/mod.rs
  integration_test: crates/aureline-support/tests/safe_mode_beta.rs
drill_class: failure_recovery_drill
drill_owner_lane: support_export
starting_condition:
  state_ref: bogus
  summary: bogus
drill_steps:
  - step_id: bogus
    step_class: export_support_packet
    summary: bogus
    expected_artifact_kind: safe_mode_support_packet_record
    expected_artifact_ref: bogus
expected_first_actionable_artifact:
  artifact_kind: safe_mode_profile_record
  artifact_ref: bogus
  recovery_action_class: enter_safe_mode
  reviewer_summary: bogus
primary_fixture_refs:
  - fixtures/recovery/m3/safe_mode/post_crash_loop_profile.yaml
scorecard_contribution:
  scorecard_target: m3.beta_lane.safe_mode
  coverage_class: lane_lifecycle
  expected_state: green
  contributes_to_release_gate: true
claim_downgrade_rules:
  - trigger_class: fixture_missing
    downgrade_class: stale_corpus_blocks_release_candidate
    reviewer_note: bogus
  - trigger_class: drill_step_unproven
    downgrade_class: yellow_aging_drill_evidence
    reviewer_note: bogus
  - trigger_class: drill_proves_regression
    downgrade_class: red_blocks_beta_claim
    reviewer_note: bogus
safety:
  read_only_diagnosis: true
  raw_private_material_excluded: true
  destructive_resets_present: false
  preserves_user_authored_files: true
  forbidden_fix_classes:
    - destructive_reset_without_preview
    - widen_workspace_trust
    - publish_route
    - reenable_quarantined_extension_without_preview
    - run_repo_owned_hook_for_diagnosis
  no_touch_boundary_set:
    - user_authored_files
references:
  beta_lane_doc_ref: docs/support/m3/safe_mode_beta.md
  recovery_ladder_alpha_ref: docs/support/recovery_ladder_alpha.md
  diagnosis_latency_scorecard_ref: artifacts/support/diagnosis_latency_scorecard_alpha.yaml
emitted_at: 2026-05-16T00:00:00Z
"#;
    let scenario = load_m3_support_scenario(yaml).expect("parses as struct");
    let corpus = aureline_support::m3_scenario_corpus::M3ScenarioCorpus {
        entries: vec![
            aureline_support::m3_scenario_corpus::M3ScenarioCorpusEntry {
                fixture_ref: "fixtures/support/m3/scenario_corpus/bogus.yaml".to_owned(),
                scenario,
            },
        ],
    };
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|v| v.check_id == "scenario.record_kind"));
}

#[test]
fn validate_refuses_a_corpus_missing_a_required_lane() {
    let mut corpus = current_m3_scenario_corpus().expect("corpus parses");
    corpus
        .entries
        .retain(|entry| entry.scenario.beta_lane_class != M3BetaLaneClass::RuntimeReplayPackets);
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|v| v.check_id == "corpus.required_lane_missing"
            && v.target_ref == "runtime_replay_packets"));
}

#[test]
fn validate_refuses_a_scenario_whose_drill_class_does_not_match_its_lane() {
    let mut corpus = current_m3_scenario_corpus().expect("corpus parses");
    for entry in &mut corpus.entries {
        if entry.scenario.beta_lane_class == M3BetaLaneClass::SafeMode {
            entry.scenario.drill_class =
                aureline_support::m3_scenario_corpus::M3DrillClass::ReplayDecisionDrill;
        }
    }
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|v| v.check_id == "scenario.drill_class.mismatch"));
}
