//! Fixture-driven coverage for monorepo partial-index search drills.
//!
//! The cases under `fixtures/search/monorepo_partial_index/` exercise partial
//! indexing, stale shards, and hidden scope. The large-workspace cases consume
//! those fixtures and prove the benchmark packet is validation-ready without a
//! human demo or checked-in generated workspace bytes.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_search::{IndexedLaneState, IndexedLaneStateInput, IndexedStateSupportArtifact};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MonorepoDrillFixture {
    record_kind: String,
    schema_version: u32,
    drill_id: String,
    scenario: String,
    acceptance_states: Vec<String>,
    reference_binding: ReferenceBinding,
    protected_lane: ProtectedLane,
    workspace_shape: WorkspaceShape,
    indexed_state_input: IndexedLaneStateInput,
    shard_states: Vec<ShardState>,
    hidden_scope: HiddenScope,
    search_projection: SearchProjection,
    unsafe_actions: UnsafeActions,
    support_export: SupportExport,
    expect: MonorepoExpect,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ReferenceBinding {
    fixture_register_row_id: String,
    reference_workspace_id: String,
    corpus_refs: Vec<String>,
    privacy_decision: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProtectedLane {
    lane_id: String,
    surface: String,
    query_family: String,
    scope_class: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WorkspaceShape {
    workspace_id: String,
    root_count: u64,
    declared_file_count: u64,
    materialized_file_count: u64,
    synthesis_mode: String,
    workset_id: String,
    workset_label: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ShardState {
    shard_id: String,
    scope_ref: String,
    state: String,
    indexed_file_count: u64,
    stale_since: Option<String>,
    omitted: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HiddenScope {
    hidden_result_count: u64,
    hidden_result_count_known: bool,
    hidden_reason_tokens: Vec<String>,
    omitted_scope_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SearchProjection {
    query: String,
    ranking_mode: String,
    readiness_state: String,
    scope_label: String,
    first_useful_result_ms: u64,
    full_index_required_for_first_result: bool,
    result_rows: Vec<ResultRow>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ResultRow {
    stable_result_id: String,
    title: String,
    path: String,
    source_layers: Vec<String>,
    freshness_label: String,
    ranking_reasons: Vec<String>,
    actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UnsafeActions {
    blocked_actions: Vec<String>,
    narrowed_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SupportExport {
    artifact_id: String,
    redaction_state: String,
    includes_raw_private_material: bool,
}

#[derive(Debug, Deserialize)]
struct MonorepoExpect {
    state_token: String,
    lane_token: String,
    result_rows_require_caveat: bool,
    current_claim_narrowed: bool,
    requires_hidden_scope_disclosure: bool,
    requires_stale_shard_disclosure: bool,
    first_useful_result_before_full_index: bool,
    blocked_actions_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LargeWorkspaceDrill {
    record_kind: String,
    schema_version: u32,
    drill_id: String,
    scenario: String,
    source_fixture_refs: Vec<String>,
    reference_binding: ReferenceBinding,
    workspace_model: WorkspaceModel,
    automation: Automation,
    drill_steps: Vec<DrillStep>,
    result_packet: ResultPacket,
    expect: LargeWorkspaceExpect,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WorkspaceModel {
    synthesis_mode: String,
    total_files: u64,
    root_count: u64,
    generated_file_count: u64,
    ignored_file_count: u64,
    materialized_fixture_files: u64,
    materialized_bytes_required: bool,
    privacy_class: String,
    source_class: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Automation {
    command: String,
    human_demo_required: bool,
    deterministic_inputs: Vec<String>,
    validation_assertions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DrillStep {
    step_id: String,
    expected_state: String,
    required_visible_truth: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ResultPacket {
    result_id: String,
    benchmark_packet_ref: String,
    captured_at: String,
    run_context: String,
    result: String,
    exercised_states: Vec<String>,
    first_useful_result_ms: u64,
    full_index_required_for_pass: bool,
    raw_private_material_excluded: bool,
}

#[derive(Debug, Deserialize)]
struct LargeWorkspaceExpect {
    min_total_files: u64,
    requires_human_demo: bool,
    benchmark_packet_contains_result: bool,
    acceptance_states: Vec<String>,
}

fn fixture_paths(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let payload = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_reference_binding(binding: &ReferenceBinding, path: &Path) {
    assert!(
        binding
            .fixture_register_row_id
            .starts_with("fixture_register:external_alpha."),
        "fixture {path:?} must cite the alpha fixture register"
    );
    assert!(
        !binding.corpus_refs.is_empty(),
        "fixture {path:?} must cite corpus refs from the benchmark register"
    );
    assert_eq!(
        binding.privacy_decision, "admit_public",
        "fixture {path:?} must remain public synthetic/admitted evidence"
    );
}

#[test]
fn monorepo_partial_index_cases_cover_required_search_states() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/search/monorepo_partial_index");
    let mut covered_states = BTreeSet::new();

    for path in fixture_paths(&fixtures_dir) {
        let fixture: MonorepoDrillFixture = read_json(&path);
        assert_eq!(
            fixture.record_kind, "monorepo_partial_index_drill_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );
        assert_reference_binding(&fixture.reference_binding, &path);

        let lane_state = IndexedLaneState::materialize(fixture.indexed_state_input);
        assert_eq!(
            lane_state.state_token, fixture.expect.state_token,
            "state token mismatch in {path:?}"
        );
        assert_eq!(
            lane_state.lane_token, fixture.expect.lane_token,
            "lane token mismatch in {path:?}"
        );
        assert_eq!(
            lane_state.result_rows_require_caveat, fixture.expect.result_rows_require_caveat,
            "row caveat mismatch in {path:?}"
        );
        assert_eq!(
            lane_state.current_claim_narrowed, fixture.expect.current_claim_narrowed,
            "claim narrowing mismatch in {path:?}"
        );

        for action in &fixture.expect.blocked_actions_include {
            assert!(
                lane_state.blocked_actions.contains(action)
                    || fixture.unsafe_actions.blocked_actions.contains(action),
                "missing blocked action {action} in {path:?}"
            );
        }

        let support_artifact = IndexedStateSupportArtifact::from_lane_states(
            fixture.support_export.artifact_id.clone(),
            lane_state.observed_at.clone(),
            std::slice::from_ref(&lane_state),
        );
        assert!(support_artifact.raw_private_material_excluded);
        assert!(
            !fixture.support_export.includes_raw_private_material,
            "fixture {path:?} must not require raw private material"
        );
        assert_eq!(
            support_artifact.unsafe_current_claim_lanes(),
            Vec::<&str>::new(),
            "support artifact overclaims current truth in {path:?}"
        );

        if fixture.expect.requires_hidden_scope_disclosure {
            assert!(
                fixture.hidden_scope.hidden_result_count > 0,
                "hidden-scope fixture {path:?} must name hidden rows"
            );
            assert!(
                fixture.hidden_scope.hidden_result_count_known,
                "hidden-scope fixture {path:?} must expose count-known truth"
            );
            assert!(
                !fixture.hidden_scope.hidden_reason_tokens.is_empty(),
                "hidden-scope fixture {path:?} must name hidden reasons"
            );
        }

        if fixture.expect.requires_stale_shard_disclosure {
            assert!(
                fixture
                    .shard_states
                    .iter()
                    .any(|shard| shard.state == "stale"),
                "stale-shard fixture {path:?} must include a stale shard"
            );
        }

        assert_eq!(
            !fixture
                .search_projection
                .full_index_required_for_first_result,
            fixture.expect.first_useful_result_before_full_index,
            "first useful result posture mismatch in {path:?}"
        );
        assert!(
            !fixture.search_projection.result_rows.is_empty(),
            "fixture {path:?} must keep at least one result row inspectable"
        );

        covered_states.extend(fixture.acceptance_states);
    }

    for required in ["partial_index", "stale_shard", "hidden_scope"] {
        assert!(
            covered_states.contains(required),
            "missing protected drill state {required}"
        );
    }
}

#[test]
fn large_workspace_drills_are_automation_validated_and_benchmark_consumed() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/search/large_workspace_alpha");
    let benchmark_packet_path = repo_root.join("artifacts/benchmarks/m2_partial_index_drill.md");
    let benchmark_packet = std::fs::read_to_string(&benchmark_packet_path).unwrap_or_else(|err| {
        panic!("benchmark packet must read at {benchmark_packet_path:?}: {err}")
    });
    let mut packet_result_count = 0_u32;

    for path in fixture_paths(&fixtures_dir) {
        let fixture: LargeWorkspaceDrill = read_json(&path);
        assert_eq!(
            fixture.record_kind, "large_workspace_alpha_drill",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );
        assert_reference_binding(&fixture.reference_binding, &path);

        assert!(
            fixture.workspace_model.total_files >= fixture.expect.min_total_files,
            "fixture {path:?} does not meet large-workspace size floor"
        );
        assert_eq!(
            fixture.workspace_model.synthesis_mode, "described_counts",
            "fixture {path:?} must avoid checked-in generated workspace bytes"
        );
        assert!(
            !fixture.workspace_model.materialized_bytes_required,
            "fixture {path:?} must not require materialized large-workspace bytes"
        );
        assert_eq!(
            fixture.automation.human_demo_required, fixture.expect.requires_human_demo,
            "human-demo posture mismatch in {path:?}"
        );
        assert!(
            !fixture.automation.human_demo_required,
            "fixture {path:?} must be machine-validatable"
        );
        assert_eq!(
            fixture.automation.command,
            "cargo test -p aureline-search --test partial_index_drill_cases",
            "fixture {path:?} must point at the protected validator"
        );

        for source_ref in &fixture.source_fixture_refs {
            assert!(
                repo_root.join(source_ref).exists(),
                "source fixture ref does not resolve in {path:?}: {source_ref}"
            );
            assert!(
                benchmark_packet.contains(source_ref)
                    || benchmark_packet.contains(&fixture.drill_id),
                "benchmark packet must cite source fixture or drill id for {path:?}"
            );
        }
        for input_ref in &fixture.automation.deterministic_inputs {
            assert!(
                repo_root.join(input_ref).exists(),
                "deterministic input does not resolve in {path:?}: {input_ref}"
            );
        }
        for expected_state in &fixture.expect.acceptance_states {
            assert!(
                fixture
                    .result_packet
                    .exercised_states
                    .contains(expected_state),
                "result packet in {path:?} is missing expected state {expected_state}"
            );
        }

        assert_eq!(
            fixture.result_packet.result, "pass",
            "drill result must pass in {path:?}"
        );
        assert!(
            !fixture.result_packet.full_index_required_for_pass,
            "large-workspace drill {path:?} must not require full indexing"
        );
        assert!(
            fixture.result_packet.raw_private_material_excluded,
            "large-workspace drill {path:?} must exclude raw private material"
        );
        assert!(
            benchmark_packet.contains(&fixture.result_packet.result_id),
            "benchmark packet must include result id from {path:?}"
        );
        assert_eq!(
            benchmark_packet.contains(&fixture.drill_id),
            fixture.expect.benchmark_packet_contains_result,
            "benchmark packet drill-id citation mismatch in {path:?}"
        );
        packet_result_count += 1;
    }

    assert!(
        packet_result_count > 0,
        "benchmark packet must consume at least one large-workspace drill result"
    );
}
