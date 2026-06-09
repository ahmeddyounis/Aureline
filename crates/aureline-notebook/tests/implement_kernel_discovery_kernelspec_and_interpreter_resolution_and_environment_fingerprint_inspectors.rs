//! End-to-end coverage for the kernel discovery, kernelspec, interpreter
//! resolution, and environment fingerprint inspectors corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    EnvironmentFingerprint, EnvironmentFingerprintFreshnessClass, InterpreterManagerClass,
    KernelDiscoveryAvailabilityClass, KernelDiscoveryCompatibilityClass, KernelDiscoveryEntry,
    Kernelspec, KernelspecDiscoverySourceClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_kernelspec_discovery_source_classes: Vec<String>,
    expected_interpreter_manager_classes: Vec<String>,
    expected_environment_fingerprint_freshness_classes: Vec<String>,
    expected_kernel_discovery_compatibility_classes: Vec<String>,
    expected_kernel_discovery_availability_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    kernelspec: Kernelspec,
    interpreter_resolution: aureline_notebook::InterpreterResolution,
    environment_fingerprint: EnvironmentFingerprint,
    kernel_discovery_entry: KernelDiscoveryEntry,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    discovery_source_class: KernelspecDiscoverySourceClass,
    manager_class: InterpreterManagerClass,
    freshness_class: EnvironmentFingerprintFreshnessClass,
    compatibility_class: KernelDiscoveryCompatibilityClass,
    availability_class: KernelDiscoveryAvailabilityClass,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    kernelspec: Vec<String>,
    #[serde(default)]
    interpreter_resolution: Vec<String>,
    #[serde(default)]
    environment_fingerprint: Vec<String>,
    #[serde(default)]
    kernel_discovery_entry: Vec<String>,
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
    findings: &[aureline_notebook::KernelDiscoveryFinding],
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
    let mut observed_discovery_sources = BTreeMap::new();
    let mut observed_managers = BTreeMap::new();
    let mut observed_freshness = BTreeMap::new();
    let mut observed_compatibility = BTreeMap::new();
    let mut observed_availability = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let ks_findings = case.kernelspec.validate();
        assert_findings_match(&case.fixture.expected.findings.kernelspec, &ks_findings);

        let ir_findings = case.interpreter_resolution.validate();
        assert_findings_match(
            &case.fixture.expected.findings.interpreter_resolution,
            &ir_findings,
        );

        let ef_findings = case.environment_fingerprint.validate();
        assert_findings_match(
            &case.fixture.expected.findings.environment_fingerprint,
            &ef_findings,
        );

        let entry_findings = case.kernel_discovery_entry.validate();
        assert_findings_match(
            &case.fixture.expected.findings.kernel_discovery_entry,
            &entry_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.kernel_discovery_entry.discovery_source_class,
            case.fixture.expected.discovery_source_class,
            "fixture {name} discovery_source_class mismatch"
        );
        assert_eq!(
            case.interpreter_resolution.manager_class, case.fixture.expected.manager_class,
            "fixture {name} manager_class mismatch"
        );
        assert_eq!(
            case.environment_fingerprint.freshness_class, case.fixture.expected.freshness_class,
            "fixture {name} freshness_class mismatch"
        );
        assert_eq!(
            case.kernel_discovery_entry.compatibility_class,
            case.fixture.expected.compatibility_class,
            "fixture {name} compatibility_class mismatch"
        );
        assert_eq!(
            case.kernel_discovery_entry.availability_class,
            case.fixture.expected.availability_class,
            "fixture {name} availability_class mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.kernelspec.display_name_label.trim().is_empty(),
            "fixture {name}: kernelspec display_name_label must not be empty"
        );
        assert!(
            !case.kernelspec.language_label.trim().is_empty(),
            "fixture {name}: kernelspec language_label must not be empty"
        );
        assert!(
            !case
                .environment_fingerprint
                .environment_identity_label
                .trim()
                .is_empty(),
            "fixture {name}: fingerprint environment_identity_label must not be empty"
        );

        observed_discovery_sources.insert(
            case.kernel_discovery_entry.discovery_source_class.as_str(),
            (),
        );
        observed_managers.insert(case.interpreter_resolution.manager_class.as_str(), ());
        observed_freshness.insert(case.environment_fingerprint.freshness_class.as_str(), ());
        observed_compatibility.insert(case.kernel_discovery_entry.compatibility_class.as_str(), ());
        observed_availability.insert(case.kernel_discovery_entry.availability_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_kernelspec_discovery_source_classes {
        assert!(
            observed_discovery_sources.contains_key(expected.as_str()),
            "no fixture exercises discovery source class '{expected}'"
        );
    }
    for expected in &manifest.expected_interpreter_manager_classes {
        assert!(
            observed_managers.contains_key(expected.as_str()),
            "no fixture exercises manager class '{expected}'"
        );
    }
    for expected in &manifest.expected_environment_fingerprint_freshness_classes {
        assert!(
            observed_freshness.contains_key(expected.as_str()),
            "no fixture exercises freshness class '{expected}'"
        );
    }
    for expected in &manifest.expected_kernel_discovery_compatibility_classes {
        assert!(
            observed_compatibility.contains_key(expected.as_str()),
            "no fixture exercises compatibility class '{expected}'"
        );
    }
    for expected in &manifest.expected_kernel_discovery_availability_classes {
        assert!(
            observed_availability.contains_key(expected.as_str()),
            "no fixture exercises availability class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet =
        aureline_notebook::current_kernel_discovery_packet().expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
