//! Fixture-driven coverage for the stable execution-evidence bundle linking
//! task events, problem records, output channels, output chunks, imported
//! provider evidence, freshness, confidence, and reopen lineage.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_execution_evidence_bundle_input, ExecutionEvidenceBundle,
    ExecutionEvidenceBundleInput, ExecutionEvidenceFindingKind,
    EXECUTION_EVIDENCE_BUNDLE_ARTIFACT_DOC_REF, EXECUTION_EVIDENCE_BUNDLE_DOC_REF,
    EXECUTION_EVIDENCE_BUNDLE_FIXTURE_DIR, EXECUTION_EVIDENCE_BUNDLE_PACKET_ARTIFACT_REF,
    EXECUTION_EVIDENCE_BUNDLE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EvidenceBundleFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: FixtureMutation,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FixtureMutation {
    None,
    DropHeuristicRawOutputBacklink,
    FlattenImportedOutputToLocalTruth,
    DropProviderProblemBacklink,
    DropReopenLineage,
    DropReleaseProjection,
    DanglingOutputChunkRef,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    source_kind_tokens: Vec<String>,
    output_channel_name_tokens: Vec<String>,
    consumer_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> EvidenceBundleFixture {
    let path = repo_root()
        .join(EXECUTION_EVIDENCE_BUNDLE_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn mutated_input(mutation: &FixtureMutation) -> ExecutionEvidenceBundleInput {
    let mut input = current_stable_execution_evidence_bundle_input();
    match mutation {
        FixtureMutation::None => {}
        FixtureMutation::DropHeuristicRawOutputBacklink => {
            input.problems[0].raw_output_backlink = None;
        }
        FixtureMutation::FlattenImportedOutputToLocalTruth => {
            input.output_channels[1].trust_state =
                aureline_runtime::ExecutionEvidenceOutputTrustState::LocalRuntimeTruth;
        }
        FixtureMutation::DropProviderProblemBacklink => {
            input.problems[1].provider_evidence_backlink = None;
        }
        FixtureMutation::DropReopenLineage => {
            input.evidence_objects[0].reopen_refs.clear();
        }
        FixtureMutation::DropReleaseProjection => {
            input.consumer_projections.retain(|projection| {
                projection.consumer_surface
                    != aureline_runtime::ExecutionEvidenceConsumerSurface::ReleasePacket
            });
        }
        FixtureMutation::DanglingOutputChunkRef => {
            input.evidence_objects[0].output_chunk_refs = vec!["output-chunk:missing".to_owned()];
        }
    }
    input
}

fn assert_token_set_matches(observed: Vec<&str>, expected: &[String], label: &str) {
    let mut observed = observed;
    observed.sort_unstable();
    let mut expected: Vec<&str> = expected.iter().map(String::as_str).collect();
    expected.sort_unstable();
    assert_eq!(observed, expected, "{label} token set drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "execution_evidence_bundle_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let bundle = ExecutionEvidenceBundle::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        bundle.promotion_state.as_str(),
        fixture.expect.promotion_state
    );
    assert_eq!(
        bundle.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        bundle
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    for expected_kind in &fixture.expect.expected_finding_kinds {
        assert!(
            bundle
                .validation_findings
                .iter()
                .any(|finding| finding.finding_kind.as_str() == expected_kind),
            "fixture {} expected finding kind {}",
            fixture.case_name,
            expected_kind
        );
    }
    assert_token_set_matches(
        bundle.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_set_matches(
        bundle.output_channel_name_tokens(),
        &fixture.expect.output_channel_name_tokens,
        "output channel name",
    );
    assert_token_set_matches(
        bundle.consumer_surface_tokens(),
        &fixture.expect.consumer_surface_tokens,
        "consumer surface",
    );

    let export = bundle.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-06T12:02:00Z",
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
    assert_exists(EXECUTION_EVIDENCE_BUNDLE_SCHEMA_REF);
    assert_exists(EXECUTION_EVIDENCE_BUNDLE_DOC_REF);
    assert_exists(EXECUTION_EVIDENCE_BUNDLE_ARTIFACT_DOC_REF);
    assert_exists(EXECUTION_EVIDENCE_BUNDLE_FIXTURE_DIR);
    assert_exists(EXECUTION_EVIDENCE_BUNDLE_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_packet_validates_stable() {
    let path = repo_root().join(EXECUTION_EVIDENCE_BUNDLE_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let bundle: ExecutionEvidenceBundle = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        bundle.validate().is_empty(),
        "checked-in bundle must validate without findings"
    );
    assert_eq!(bundle.promotion_state.as_str(), "stable");
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        ExecutionEvidenceFindingKind::HeuristicMissingRawOutputBacklink.as_str(),
        "heuristic_missing_raw_output_backlink"
    );
    assert_eq!(
        ExecutionEvidenceFindingKind::ImportedEvidenceFlattenedIntoLocalTruth.as_str(),
        "imported_evidence_flattened_into_local_truth"
    );
    assert_eq!(
        ExecutionEvidenceFindingKind::EvidenceMissingReopenLineage.as_str(),
        "evidence_missing_reopen_lineage"
    );
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn heuristic_problem_without_raw_output_backlink_blocks_stable() {
    assert_fixture_matches("heuristic_problem_without_raw_output_backlink_blocks_stable.json");
}

#[test]
fn imported_output_flattened_to_local_truth_blocks_stable() {
    assert_fixture_matches("imported_output_flattened_to_local_truth_blocks_stable.json");
}

#[test]
fn imported_problem_without_provider_backlink_blocks_stable() {
    assert_fixture_matches("imported_problem_without_provider_backlink_blocks_stable.json");
}

#[test]
fn evidence_without_reopen_lineage_blocks_stable() {
    assert_fixture_matches("evidence_without_reopen_lineage_blocks_stable.json");
}

#[test]
fn missing_release_projection_blocks_stable() {
    assert_fixture_matches("missing_release_projection_blocks_stable.json");
}

#[test]
fn dangling_output_chunk_ref_blocks_stable() {
    assert_fixture_matches("dangling_output_chunk_ref_blocks_stable.json");
}
