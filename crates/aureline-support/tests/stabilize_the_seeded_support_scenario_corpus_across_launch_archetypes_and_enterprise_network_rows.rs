//! Protected tests for the stabilized seeded support-scenario corpus across
//! launch archetypes and enterprise-network rows.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::{
    current_stabilized_scenario_corpus, load_seeded_support_scenario,
    EnterpriseNetworkRowClass, LaunchArchetypeClass, StabilizedScenarioClass,
    StabilizedScenarioEvaluator, STABILIZED_SCENARIO_ARTIFACT_REF,
    STABILIZED_SCENARIO_DOC_REF, STABILIZED_SCENARIO_FIXTURE_DIR,
    STABILIZED_SCENARIO_REPORT_RECORD_KIND, STABILIZED_SCENARIO_REPORT_ROW_RECORD_KIND,
    STABILIZED_SCENARIO_SCHEMA_REF, STABILIZED_SCENARIO_SUPPORT_PACKET_RECORD_KIND,
    STABILIZED_SUPPORT_SCENARIO_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    scenario_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(STABILIZED_SCENARIO_FIXTURE_DIR)
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_scenarios() -> Vec<aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::SeededSupportScenario>{
    load_manifest()
        .scenario_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_seeded_support_scenario(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn corpus_validates_against_launch_archetype_and_network_coverage_contract() {
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
    assert_eq!(
        corpus.entries.len(),
        LaunchArchetypeClass::REQUIRED.len(),
        "corpus should cover every required launch archetype"
    );

    let archetype_set = corpus
        .scenarios()
        .map(|scenario| scenario.launch_archetype_class)
        .collect::<BTreeSet<_>>();
    let expected = LaunchArchetypeClass::REQUIRED
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(archetype_set, expected);

    let network_set = corpus
        .scenarios()
        .map(|scenario| scenario.enterprise_network_row_class)
        .collect::<BTreeSet<_>>();
    let expected_network = EnterpriseNetworkRowClass::REQUIRED
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(network_set, expected_network);
}

#[test]
fn every_scenario_pins_repo_relative_doc_schema_crate_and_test_refs_that_exist() {
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let scenario = &entry.scenario;
        assert_eq!(
            scenario.record_kind,
            STABILIZED_SUPPORT_SCENARIO_RECORD_KIND
        );
        let refs = &scenario.consumer_refs;
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
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    for scenario in corpus.scenarios() {
        assert!(
            scenario
                .validation_steps
                .iter()
                .any(|step| step.step_class == "export_support_packet"),
            "{} must include an export_support_packet step",
            scenario.scenario_id
        );

        let triggers = scenario
            .claim_downgrade_rules
            .iter()
            .map(|rule| rule.trigger_class)
            .collect::<BTreeSet<_>>();
        for required in [
            aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::ScenarioDowngradeTriggerClass::FixtureMissing,
            aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::ScenarioDowngradeTriggerClass::DrillStepUnproven,
            aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::ScenarioDowngradeTriggerClass::DrillProvesRegression,
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
fn report_projects_one_metadata_safe_row_per_scenario() {
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    let report = corpus.report("m4.stabilized_scenario_report.v1", "2026-06-02T00:00:00Z");
    assert_eq!(report.record_kind, STABILIZED_SCENARIO_REPORT_RECORD_KIND);
    assert_eq!(report.rows.len(), LaunchArchetypeClass::REQUIRED.len());
    assert!(report.is_export_safe());
    assert_eq!(report.corpus_doc_ref, STABILIZED_SCENARIO_DOC_REF);
    assert_eq!(report.corpus_schema_ref, STABILIZED_SCENARIO_SCHEMA_REF);

    let archetype_set = report
        .rows
        .iter()
        .map(|row| row.launch_archetype_class)
        .collect::<BTreeSet<_>>();
    for required in LaunchArchetypeClass::REQUIRED {
        assert!(
            archetype_set.contains(&required),
            "missing row for archetype {}",
            required.as_str()
        );
    }

    let network_set = report
        .rows
        .iter()
        .map(|row| row.enterprise_network_row_class)
        .collect::<BTreeSet<_>>();
    for required in EnterpriseNetworkRowClass::REQUIRED {
        assert!(
            network_set.contains(&required),
            "missing row for network row {}",
            required.as_str()
        );
    }

    for row in &report.rows {
        assert_eq!(row.record_kind, STABILIZED_SCENARIO_REPORT_ROW_RECORD_KIND);
        assert!(row.metadata_safe_baseline_met);
        assert!(row.contributes_to_release_gate);
        assert!(!row.scorecard_target.is_empty());
        assert!(row
            .downgrade_trigger_classes
            .iter()
            .any(|trigger| trigger == "fixture_missing"));
    }
}

#[test]
fn yaml_round_trip_load_matches_corpus_entry() {
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let scenario = load_seeded_support_scenario(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(scenario, entry.scenario);
    }
}

#[test]
fn corpus_doc_and_artifact_exist_at_declared_paths() {
    let root = repo_root();
    assert!(
        root.join(STABILIZED_SCENARIO_FIXTURE_DIR).is_dir(),
        "fixture dir {STABILIZED_SCENARIO_FIXTURE_DIR} missing"
    );
    assert!(
        root.join(STABILIZED_SCENARIO_DOC_REF).is_file(),
        "reviewer doc {STABILIZED_SCENARIO_DOC_REF} missing"
    );
    assert!(
        root.join(STABILIZED_SCENARIO_ARTIFACT_REF).is_file(),
        "artifact {STABILIZED_SCENARIO_ARTIFACT_REF} missing"
    );
}

#[test]
fn validate_refuses_a_yaml_record_with_wrong_record_kind() {
    let yaml = r#"
schema_version: 1
record_kind: not_stabilized_support_scenario_record
scenario_id: bogus
title: bogus
launch_archetype_class: first_run
enterprise_network_row_class: standard_internet
stabilized_scenario_class: blocked_user_recovery
doctor_finding_ref: doctor.finding.bogus
consumer_refs:
  doc_ref: docs/support/m4/bogus.md
  schema_ref: schemas/support/bogus.schema.json
  crate_consumer: crates/aureline-support/src/bogus/mod.rs
  integration_test: crates/aureline-support/tests/bogus.rs
starting_condition:
  state_ref: bogus
  summary: bogus
validation_steps:
  - step_id: bogus
    step_class: export_support_packet
    summary: bogus
    expected_artifact_kind: support_scenario_support_packet_record
    expected_artifact_ref: bogus
expected_first_actionable_artifact:
  artifact_kind: safe_mode_profile_record
  artifact_ref: bogus
  recovery_action_class: enter_safe_mode
  reviewer_summary: bogus
primary_fixture_refs:
  - fixtures/support/bogus.yaml
scorecard_contribution:
  scorecard_target: bogus
  coverage_class: bogus
  expected_state: green
  contributes_to_release_gate: true
claim_downgrade_rules:
  - trigger_class: fixture_missing
    claim_state_class: stale_corpus
    reviewer_note: bogus
  - trigger_class: drill_step_unproven
    claim_state_class: yellow_aging
    reviewer_note: bogus
  - trigger_class: drill_proves_regression
    claim_state_class: red_blocked
    reviewer_note: bogus
safety:
  read_only_diagnosis: true
  raw_private_material_excluded: true
  destructive_resets_present: false
  preserves_user_authored_files: true
  forbidden_fix_classes:
    - destructive_reset_without_preview
  no_touch_boundary_set:
    - user_authored_files
references:
  lane_doc_ref: docs/support/m4/bogus.md
  recovery_ladder_ref: docs/support/recovery_ladder_alpha.md
  diagnosis_latency_scorecard_ref: artifacts/support/diagnosis_latency_scorecard_alpha.yaml
emitted_at: 2026-06-02T00:00:00Z
"#;
    let scenario = load_seeded_support_scenario(yaml).expect("parses as struct");
    let corpus = aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::StabilizedScenarioCorpus {
        entries: vec![
            aureline_support::stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows::StabilizedScenarioCorpusEntry {
                fixture_ref: "fixtures/support/m4/bogus.yaml".to_owned(),
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
fn validate_refuses_a_corpus_missing_a_required_archetype() {
    let mut corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    corpus.entries.retain(|entry| {
        entry.scenario.launch_archetype_class != LaunchArchetypeClass::CrashRecovery
    });
    let violations = corpus.validate();
    assert!(violations
        .iter()
        .any(|v| v.check_id == "corpus.required_archetype_missing"
            && v.target_ref == "crash_recovery"));
}

#[test]
fn validate_refuses_a_corpus_missing_a_required_network_row() {
    let mut corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    corpus.entries.retain(|entry| {
        entry.scenario.enterprise_network_row_class != EnterpriseNetworkRowClass::AirGapped
    });
    let violations = corpus.validate();
    assert!(violations.iter().any(
        |v| v.check_id == "corpus.required_network_row_missing" && v.target_ref == "air_gapped"
    ));
}

#[test]
fn evaluator_support_packets_are_export_safe() {
    let evaluator = StabilizedScenarioEvaluator::new();
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    for scenario in corpus.scenarios() {
        let packet = evaluator
            .support_packet(
                format!("support_packet:{}", scenario.scenario_id),
                scenario.emitted_at.clone(),
                scenario,
            )
            .expect("support packet builds");

        assert_eq!(
            packet.record_kind,
            STABILIZED_SCENARIO_SUPPORT_PACKET_RECORD_KIND
        );
        assert!(packet.is_export_safe());
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert!(!packet.destructive_resets_present);
        assert_eq!(packet.schema_ref, STABILIZED_SCENARIO_SCHEMA_REF);
        assert_eq!(packet.doc_ref, STABILIZED_SCENARIO_DOC_REF);
    }
}

#[test]
fn evaluator_report_is_export_safe() {
    let evaluator = StabilizedScenarioEvaluator::new();
    let corpus = current_stabilized_scenario_corpus().expect("corpus parses");
    let report = evaluator
        .report(
            "m4.stabilized_scenario_report.v1",
            "2026-06-02T00:00:00Z",
            &corpus,
        )
        .expect("report builds");
    assert!(report.is_export_safe());
}

#[test]
fn every_scenario_cites_a_doctor_finding_ref() {
    for scenario in &load_scenarios() {
        assert!(
            scenario.doctor_finding_ref.starts_with("doctor.finding."),
            "{} must cite a doctor.finding ref",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_scenario_declares_non_empty_title() {
    for scenario in &load_scenarios() {
        assert!(
            !scenario.title.trim().is_empty(),
            "{} must have a non-empty title",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_scenario_has_at_least_one_primary_fixture_ref() {
    for scenario in &load_scenarios() {
        assert!(
            !scenario.primary_fixture_refs.is_empty(),
            "{} must have at least one primary_fixture_ref",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_scenario_preserves_user_authored_files() {
    for scenario in &load_scenarios() {
        assert!(
            scenario.safety.preserves_user_authored_files,
            "{} must preserve user-authored files",
            scenario.scenario_id
        );
        assert!(
            scenario
                .safety
                .no_touch_boundary_set
                .iter()
                .any(|b| b == "user_authored_files"),
            "{} no_touch_boundary_set must contain user_authored_files",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_scenario_forbids_destructive_resets() {
    for scenario in &load_scenarios() {
        assert!(
            !scenario.safety.destructive_resets_present,
            "{} must not declare destructive resets",
            scenario.scenario_id
        );
    }
}

#[test]
fn scenarios_cover_all_stabilized_scenario_classes() {
    let scenarios = load_scenarios();
    let covered: BTreeSet<StabilizedScenarioClass> = scenarios
        .iter()
        .map(|s| s.stabilized_scenario_class)
        .collect();
    let expected: BTreeSet<StabilizedScenarioClass> = [
        StabilizedScenarioClass::BlockedUserRecovery,
        StabilizedScenarioClass::CrashLoopCenterEvidence,
        StabilizedScenarioClass::DiagnosisRouting,
        StabilizedScenarioClass::RepairPreviewValidation,
        StabilizedScenarioClass::SafeModeTransition,
        StabilizedScenarioClass::SupportExportVerification,
    ]
    .into_iter()
    .collect();
    assert_eq!(
        covered, expected,
        "scenarios must cover all stabilized scenario classes"
    );
}
