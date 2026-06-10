//! End-to-end coverage for the experiment run identities, environment
//! fingerprints, dataset cards, and artifact lineage corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    ArtifactLineage, ArtifactLineageStateClass, ArtifactSaveLocationClass, DatasetCard,
    DatasetLocationClass, DatasetSensitivityRedactionClass, DatasetSourceClass,
    ExperimentEnvironmentFingerprint, ExperimentEnvironmentFingerprintFreshnessClass,
    ExperimentRunIdentity, ExperimentRunOutcomeClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_experiment_run_outcome_classes: Vec<String>,
    expected_experiment_environment_fingerprint_freshness_classes: Vec<String>,
    expected_dataset_source_classes: Vec<String>,
    expected_dataset_sensitivity_redaction_classes: Vec<String>,
    expected_dataset_location_classes: Vec<String>,
    expected_artifact_save_location_classes: Vec<String>,
    expected_artifact_lineage_state_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    experiment_run_identity: ExperimentRunIdentity,
    experiment_environment_fingerprint: ExperimentEnvironmentFingerprint,
    dataset_card: DatasetCard,
    artifact_lineage: ArtifactLineage,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    outcome_class: ExperimentRunOutcomeClass,
    freshness_class: ExperimentEnvironmentFingerprintFreshnessClass,
    source_class: DatasetSourceClass,
    sensitivity_redaction_class: DatasetSensitivityRedactionClass,
    location_class: DatasetLocationClass,
    save_location_class: ArtifactSaveLocationClass,
    lineage_state_class: ArtifactLineageStateClass,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    experiment_run_identity: Vec<String>,
    #[serde(default)]
    experiment_environment_fingerprint: Vec<String>,
    #[serde(default)]
    dataset_card: Vec<String>,
    #[serde(default)]
    artifact_lineage: Vec<String>,
}

fn read_manifest() -> Manifest {
    let path = fixture_root().join("manifest.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read manifest {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse manifest {}: {err}", path.display()))
}

fn read_case(case_path: &str) -> FixtureCase {
    let path = repo_root().join(case_path);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn assert_findings_match(
    check_ids: &[String],
    findings: &[aureline_notebook::ExperimentLineageFinding],
) {
    let actual: Vec<String> = findings.iter().map(|f| f.check_id.clone()).collect();
    assert_eq!(
        actual, *check_ids,
        "expected findings {check_ids:?}, got {actual:?}"
    );
}

#[test]
fn manifest_lists_all_case_files() {
    let manifest = read_manifest();
    assert_eq!(manifest.schema_version, 1);

    for case in &manifest.case_refs {
        let path = repo_root().join(case);
        assert!(path.exists(), "manifest references missing file: {case}");
    }

    let dir = fixture_root();
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".yaml"))
        .filter(|name| name != "manifest.yaml")
        .collect();
    on_disk.sort();

    let mut referenced: Vec<String> = manifest
        .case_refs
        .iter()
        .map(|case| {
            Path::new(case)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    referenced.sort();

    assert_eq!(
        on_disk, referenced,
        "manifest case_refs must match yaml files on disk"
    );
}

#[test]
fn every_case_validates_and_matches_expectations() {
    let manifest = read_manifest();
    let mut observed_outcomes = BTreeMap::new();
    let mut observed_freshness = BTreeMap::new();
    let mut observed_sources = BTreeMap::new();
    let mut observed_sensitivity = BTreeMap::new();
    let mut observed_locations = BTreeMap::new();
    let mut observed_save_locations = BTreeMap::new();
    let mut observed_lineage_states = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let run_findings = case.experiment_run_identity.validate();
        assert_findings_match(
            &case.fixture.expected.findings.experiment_run_identity,
            &run_findings,
        );

        let fp_findings = case.experiment_environment_fingerprint.validate();
        assert_findings_match(
            &case.fixture.expected.findings.experiment_environment_fingerprint,
            &fp_findings,
        );

        let ds_findings = case.dataset_card.validate();
        assert_findings_match(
            &case.fixture.expected.findings.dataset_card,
            &ds_findings,
        );

        let art_findings = case.artifact_lineage.validate();
        assert_findings_match(
            &case.fixture.expected.findings.artifact_lineage,
            &art_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.experiment_run_identity.outcome_class,
            case.fixture.expected.outcome_class,
            "fixture {name} outcome_class mismatch"
        );
        assert_eq!(
            case.experiment_environment_fingerprint.freshness_class,
            case.fixture.expected.freshness_class,
            "fixture {name} freshness_class mismatch"
        );
        assert_eq!(
            case.dataset_card.source_class,
            case.fixture.expected.source_class,
            "fixture {name} source_class mismatch"
        );
        assert_eq!(
            case.dataset_card.sensitivity_redaction_class,
            case.fixture.expected.sensitivity_redaction_class,
            "fixture {name} sensitivity_redaction_class mismatch"
        );
        assert_eq!(
            case.dataset_card.location_class,
            case.fixture.expected.location_class,
            "fixture {name} location_class mismatch"
        );
        assert_eq!(
            case.artifact_lineage.save_location_class,
            case.fixture.expected.save_location_class,
            "fixture {name} save_location_class mismatch"
        );
        assert_eq!(
            case.artifact_lineage.lineage_state_class,
            case.fixture.expected.lineage_state_class,
            "fixture {name} lineage_state_class mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.experiment_run_identity.title.trim().is_empty(),
            "fixture {name}: run title must not be empty"
        );
        assert!(
            !case.experiment_run_identity.source_ref.trim().is_empty(),
            "fixture {name}: run source_ref must not be empty"
        );
        assert!(
            !case.experiment_environment_fingerprint
                .environment_identity_label
                .trim()
                .is_empty(),
            "fixture {name}: fingerprint environment_identity_label must not be empty"
        );
        assert!(
            !case.dataset_card.dataset_label.trim().is_empty(),
            "fixture {name}: dataset_label must not be empty"
        );
        assert!(
            !case.artifact_lineage.producing_run_ref.trim().is_empty(),
            "fixture {name}: producing_run_ref must not be empty"
        );

        observed_outcomes.insert(case.experiment_run_identity.outcome_class.as_str(), ());
        observed_freshness.insert(
            case.experiment_environment_fingerprint.freshness_class.as_str(),
            (),
        );
        observed_sources.insert(case.dataset_card.source_class.as_str(), ());
        observed_sensitivity.insert(case.dataset_card.sensitivity_redaction_class.as_str(), ());
        observed_locations.insert(case.dataset_card.location_class.as_str(), ());
        observed_save_locations.insert(case.artifact_lineage.save_location_class.as_str(), ());
        observed_lineage_states.insert(case.artifact_lineage.lineage_state_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_experiment_run_outcome_classes {
        assert!(
            observed_outcomes.contains_key(expected.as_str()),
            "no fixture exercises outcome class '{expected}'"
        );
    }
    for expected in &manifest.expected_experiment_environment_fingerprint_freshness_classes {
        assert!(
            observed_freshness.contains_key(expected.as_str()),
            "no fixture exercises freshness class '{expected}'"
        );
    }
    for expected in &manifest.expected_dataset_source_classes {
        assert!(
            observed_sources.contains_key(expected.as_str()),
            "no fixture exercises source class '{expected}'"
        );
    }
    for expected in &manifest.expected_dataset_sensitivity_redaction_classes {
        assert!(
            observed_sensitivity.contains_key(expected.as_str()),
            "no fixture exercises sensitivity/redaction class '{expected}'"
        );
    }
    for expected in &manifest.expected_dataset_location_classes {
        assert!(
            observed_locations.contains_key(expected.as_str()),
            "no fixture exercises location class '{expected}'"
        );
    }
    for expected in &manifest.expected_artifact_save_location_classes {
        assert!(
            observed_save_locations.contains_key(expected.as_str()),
            "no fixture exercises save location class '{expected}'"
        );
    }
    for expected in &manifest.expected_artifact_lineage_state_classes {
        assert!(
            observed_lineage_states.contains_key(expected.as_str()),
            "no fixture exercises lineage state class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet =
        aureline_notebook::current_experiment_lineage_packet().expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
