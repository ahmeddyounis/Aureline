//! Fixture-driven coverage for the M5 task-event adapter-policy baseline:
//! the native-first adapter ladder, the raw-payload-retention matrix, the closed
//! downgrade vocabulary, the six consumer bindings, and the arbitration rows that
//! keep lower-priority adapters from masquerading as native/BSP/BEP truth.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_task_event_adapter_policy_input, BuildTestInteropConfidence,
    BuildTestInteropSourceKind, DowngradeReason, RawPayloadRetentionClass,
    TaskEventAdapterPolicyBaseline, TaskEventAdapterPolicyBaselineInput, TaskEventConsumer,
    TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF,
    TASK_EVENT_ADAPTER_POLICY_CAPABILITY_SCHEMA_REF, TASK_EVENT_ADAPTER_POLICY_DOC_REF,
    TASK_EVENT_ADAPTER_POLICY_ENVELOPE_SCHEMA_REF, TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PolicyFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    source_kind_tokens: Vec<String>,
    consumer_tokens: Vec<String>,
    downgrade_reason_tokens: Vec<String>,
    support_export_safe: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> PolicyFixture {
    let path = repo_root()
        .join(TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_task_event_adapter_policy`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> TaskEventAdapterPolicyBaselineInput {
    let mut input = current_stable_task_event_adapter_policy_input();
    match mutation {
        "none" => {}
        "ladder_out_of_order" => {
            for rung in &mut input.priority_ladder {
                if rung.source_kind == BuildTestInteropSourceKind::Native {
                    rung.priority_rank = 5;
                } else if rung.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    rung.priority_rank = 1;
                }
            }
        }
        "heuristic_overclaims_ceiling" => {
            for rung in &mut input.priority_ladder {
                if rung.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    rung.confidence_ceiling = BuildTestInteropConfidence::High;
                }
            }
        }
        "retention_default_invalid" => {
            for cell in &mut input.retention_matrix {
                if cell.source_kind == BuildTestInteropSourceKind::HeuristicParser
                    && cell.retention_class == RawPayloadRetentionClass::MetadataDigestOnly
                {
                    cell.is_default = false;
                }
            }
        }
        "downgrade_vocabulary_drop" => {
            input
                .downgrade_vocabulary
                .retain(|entry| entry.reason != DowngradeReason::ReplayGap);
        }
        "consumer_binding_missing" => {
            input
                .consumer_bindings
                .retain(|binding| binding.consumer != TaskEventConsumer::NotebookRun);
        }
        "arbitration_shadow_not_downgraded" => {
            let row = input
                .arbitration_rows
                .iter_mut()
                .find(|r| r.arbitration_id == "arbitration:native-over-heuristic")
                .expect("seed has the native-over-heuristic arbitration");
            let shadow = &mut row.shadow_events[0];
            shadow.downgraded = false;
            shadow.downgrade_reason = None;
        }
        "arbitration_winner_swapped" => {
            let row = input
                .arbitration_rows
                .iter_mut()
                .find(|r| r.arbitration_id == "arbitration:bsp-over-structured")
                .expect("seed has the bsp-over-structured arbitration");
            std::mem::swap(&mut row.winning_event, &mut row.shadow_events[0]);
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_set(observed: Vec<&str>, expected: &[String], label: &str) {
    let mut observed = observed;
    observed.sort_unstable();
    let mut expected: Vec<&str> = expected.iter().map(String::as_str).collect();
    expected.sort_unstable();
    assert_eq!(observed, expected, "{label} token set drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_task_event_adapter_policy_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let baseline = TaskEventAdapterPolicyBaseline::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        baseline.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        baseline.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        baseline
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = baseline
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        observed_kinds,
        fixture
            .expect
            .expected_finding_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} finding kinds drift",
        fixture.case_name
    );
    assert_token_set(
        baseline.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_set(
        baseline.consumer_tokens(),
        &fixture.expect.consumer_tokens,
        "consumer",
    );
    assert_token_set(
        baseline.downgrade_reason_tokens(),
        &fixture.expect.downgrade_reason_tokens,
        "downgrade reason",
    );

    let export = baseline.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-17T00:01:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(TASK_EVENT_ADAPTER_POLICY_CAPABILITY_SCHEMA_REF);
    assert_exists(TASK_EVENT_ADAPTER_POLICY_ENVELOPE_SCHEMA_REF);
    assert_exists(TASK_EVENT_ADAPTER_POLICY_DOC_REF);
    assert_exists(TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR);
    assert_exists(TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF);
}

#[test]
fn checked_in_baseline_validates_clean() {
    let path = repo_root().join(TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("baseline artifact {path:?} must read: {err}"));
    let baseline: TaskEventAdapterPolicyBaseline = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("baseline artifact {path:?} must parse: {err}"));
    assert!(
        baseline.validate().is_empty(),
        "checked-in baseline must validate without findings: {:?}",
        baseline.validate()
    );
    assert_eq!(baseline.promotion_state.as_str(), "stable");
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn ladder_out_of_order_blocks_stable() {
    assert_fixture_matches("ladder_out_of_order_blocks_stable.json");
}

#[test]
fn heuristic_overclaims_ceiling_blocks_stable() {
    assert_fixture_matches("heuristic_overclaims_ceiling_blocks_stable.json");
}

#[test]
fn retention_default_invalid_blocks_stable() {
    assert_fixture_matches("retention_default_invalid_blocks_stable.json");
}

#[test]
fn downgrade_vocabulary_drift_blocks_stable() {
    assert_fixture_matches("downgrade_vocabulary_drift_blocks_stable.json");
}

#[test]
fn consumer_binding_missing_blocks_stable() {
    assert_fixture_matches("consumer_binding_missing_blocks_stable.json");
}

#[test]
fn arbitration_shadow_not_downgraded_blocks_stable() {
    assert_fixture_matches("arbitration_shadow_not_downgraded_blocks_stable.json");
}

#[test]
fn arbitration_winner_not_highest_priority_blocks_stable() {
    assert_fixture_matches("arbitration_winner_not_highest_priority_blocks_stable.json");
}
