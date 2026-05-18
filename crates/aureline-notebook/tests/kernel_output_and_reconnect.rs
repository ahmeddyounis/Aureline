//! End-to-end coverage for the retained notebook preview runtime-truth corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    CellExecutionDetailRow, CellExecutionOutcomeClass, DebuggerBridgeState,
    DebuggerBridgeSupportClass, KernelOriginClass, KernelSessionSummary, OutputTrustClass,
    OutputTrustRecord, ReconnectReviewConsequenceClass, ReconnectReviewSheet,
    VariableExplorerEntry, VariableExplorerFreshnessClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m3/kernel_output_and_reconnect")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_kernel_origin_classes: Vec<String>,
    expected_output_trust_classes: Vec<String>,
    expected_variable_explorer_freshness_classes: Vec<String>,
    expected_debugger_support_classes: Vec<String>,
    expected_reconnect_consequence_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    __fixture__: FixtureMeta,
    kernel_session_summary: KernelSessionSummary,
    cell_execution_detail_row: CellExecutionDetailRow,
    variable_explorer_entry: VariableExplorerEntry,
    output_trust_record: OutputTrustRecord,
    debugger_bridge_state: DebuggerBridgeState,
    #[serde(default)]
    reconnect_review_sheet: Option<ReconnectReviewSheet>,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    kernel_origin_class: KernelOriginClass,
    output_trust_class: OutputTrustClass,
    variable_explorer_freshness_class: VariableExplorerFreshnessClass,
    debugger_support_class: DebuggerBridgeSupportClass,
    #[serde(default)]
    reconnect_consequence_class: Option<ReconnectReviewConsequenceClass>,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    kernel_session_summary: Vec<String>,
    #[serde(default)]
    cell_execution_detail_row: Vec<String>,
    #[serde(default)]
    variable_explorer_entry: Vec<String>,
    #[serde(default)]
    output_trust_record: Vec<String>,
    #[serde(default)]
    debugger_bridge_state: Vec<String>,
    #[serde(default)]
    reconnect_review_sheet: Vec<String>,
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
    findings: &[aureline_notebook::RuntimeTruthFinding],
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
    let mut observed_kernel_origin = BTreeMap::new();
    let mut observed_output_trust = BTreeMap::new();
    let mut observed_var_freshness = BTreeMap::new();
    let mut observed_debugger_support = BTreeMap::new();
    let mut observed_reconnect_consequence = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.__fixture__.name.clone();

        // Validators agree with the expected findings list.
        let summary_findings = case.kernel_session_summary.validate();
        assert_findings_match(
            &case.__fixture__.expected.findings.kernel_session_summary,
            &summary_findings,
        );

        let cell_findings = case.cell_execution_detail_row.validate();
        assert_findings_match(
            &case.__fixture__.expected.findings.cell_execution_detail_row,
            &cell_findings,
        );

        let var_findings = case.variable_explorer_entry.validate();
        assert_findings_match(
            &case.__fixture__.expected.findings.variable_explorer_entry,
            &var_findings,
        );

        let output_findings = case.output_trust_record.validate();
        assert_findings_match(
            &case.__fixture__.expected.findings.output_trust_record,
            &output_findings,
        );

        let debugger_findings = case.debugger_bridge_state.validate();
        assert_findings_match(
            &case.__fixture__.expected.findings.debugger_bridge_state,
            &debugger_findings,
        );

        if let Some(sheet) = case.reconnect_review_sheet.as_ref() {
            let sheet_findings = sheet.validate();
            assert_findings_match(
                &case.__fixture__.expected.findings.reconnect_review_sheet,
                &sheet_findings,
            );
        } else {
            assert!(
                case.__fixture__
                    .expected
                    .findings
                    .reconnect_review_sheet
                    .is_empty(),
                "fixture {name} expects reconnect-sheet findings without a sheet"
            );
        }

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.kernel_session_summary.kernel_origin_class,
            case.__fixture__.expected.kernel_origin_class,
            "fixture {name} kernel_origin_class mismatch"
        );
        assert_eq!(
            case.output_trust_record.trust_class, case.__fixture__.expected.output_trust_class,
            "fixture {name} output_trust_class mismatch"
        );
        assert_eq!(
            case.variable_explorer_entry.freshness_class,
            case.__fixture__.expected.variable_explorer_freshness_class,
            "fixture {name} variable_explorer_freshness_class mismatch"
        );
        assert_eq!(
            case.debugger_bridge_state.support_class,
            case.__fixture__.expected.debugger_support_class,
            "fixture {name} debugger_support_class mismatch"
        );
        if let Some(expected_consequence) = case.__fixture__.expected.reconnect_consequence_class {
            let sheet = case
                .reconnect_review_sheet
                .as_ref()
                .unwrap_or_else(|| panic!("fixture {name} expects a reconnect sheet"));
            assert_eq!(
                sheet.consequence_class, expected_consequence,
                "fixture {name} reconnect consequence_class mismatch"
            );
        }

        observed_kernel_origin.insert(case.kernel_session_summary.kernel_origin_class.as_str(), ());
        observed_output_trust.insert(case.output_trust_record.trust_class.as_str(), ());
        observed_var_freshness.insert(case.variable_explorer_entry.freshness_class.as_str(), ());
        observed_debugger_support.insert(case.debugger_bridge_state.support_class.as_str(), ());
        if let Some(sheet) = case.reconnect_review_sheet.as_ref() {
            observed_reconnect_consequence.insert(sheet.consequence_class.as_str(), ());
        }

        // Surface invariants the spec calls out.
        if case
            .kernel_session_summary
            .kernel_origin_class
            .is_no_kernel()
        {
            assert_eq!(
                case.cell_execution_detail_row.outcome_class,
                CellExecutionOutcomeClass::SkippedNoKernel,
                "fixture {name}: no-kernel origin must not claim a kernel-bound outcome"
            );
        }
        if case.output_trust_record.trust_class == OutputTrustClass::Stale {
            assert!(
                case.output_trust_record.stale_reason_class.is_some(),
                "fixture {name}: stale outputs must cite a stale_reason_class"
            );
        }
        if case.debugger_bridge_state.support_class == DebuggerBridgeSupportClass::Supported {
            assert!(
                case.debugger_bridge_state.kernel_session_id_ref.is_some(),
                "fixture {name}: supported debugger must carry a kernel_session_id_ref"
            );
        }
        if let Some(sheet) = case.reconnect_review_sheet.as_ref() {
            assert!(
                sheet.auto_rerun_forbidden,
                "fixture {name}: reconnect sheet must declare auto_rerun_forbidden"
            );
            assert!(
                sheet.in_flight_executions_cancelled
                    || matches!(
                        sheet.consequence_class,
                        ReconnectReviewConsequenceClass::ReopeningLiveKernelSameIdentity
                    ),
                "fixture {name}: only same-identity reopen may keep in-flight executions"
            );
        }
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected_kernel in &manifest.expected_kernel_origin_classes {
        assert!(
            observed_kernel_origin.contains_key(expected_kernel.as_str()),
            "no fixture exercises kernel origin '{expected_kernel}'"
        );
    }
    for expected_output_trust in &manifest.expected_output_trust_classes {
        assert!(
            observed_output_trust.contains_key(expected_output_trust.as_str()),
            "no fixture exercises output trust '{expected_output_trust}'"
        );
    }
    for expected_var in &manifest.expected_variable_explorer_freshness_classes {
        assert!(
            observed_var_freshness.contains_key(expected_var.as_str()),
            "no fixture exercises variable freshness '{expected_var}'"
        );
    }
    for expected_debugger in &manifest.expected_debugger_support_classes {
        assert!(
            observed_debugger_support.contains_key(expected_debugger.as_str()),
            "no fixture exercises debugger support '{expected_debugger}'"
        );
    }
    for expected_reconnect in &manifest.expected_reconnect_consequence_classes {
        assert!(
            observed_reconnect_consequence.contains_key(expected_reconnect.as_str()),
            "no fixture exercises reconnect consequence '{expected_reconnect}'"
        );
    }
}
