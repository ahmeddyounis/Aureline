//! Fixture-driven coverage for indexed-state honesty records.
//!
//! Each fixture under `fixtures/search/index_state_truth/` materializes one
//! canonical indexed lane state and a support artifact. The protected cases
//! cover warming, partial, cached, stale, and paused states so support exports
//! cannot drift from the status/result vocabulary.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{
    IndexedLaneState, IndexedLaneStateInput, IndexedStateClass, IndexedStateSupportArtifact,
};

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    input: IndexedLaneStateInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    state_token: String,
    state_label: String,
    lane_token: String,
    current_claim_narrowed: bool,
    result_rows_require_caveat: bool,
    blocked_actions_include: Vec<String>,
    support_artifact_id: String,
}

#[test]
fn indexed_state_truth_cases_materialize_expected_tokens() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/index_state_truth");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one index_state_truth fixture must exist"
    );

    let mut covered_states = Vec::new();
    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "indexed_state_truth_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let state = IndexedLaneState::materialize(fixture.input);
        assert_eq!(
            state.record_kind,
            IndexedLaneState::RECORD_KIND,
            "record kind mismatch in {path:?}"
        );
        assert_eq!(
            state.state_token, fixture.expect.state_token,
            "state token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            state.state_label, fixture.expect.state_label,
            "state label mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            state.lane_token, fixture.expect.lane_token,
            "lane token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            state.current_claim_narrowed, fixture.expect.current_claim_narrowed,
            "claim narrowing mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            state.result_rows_require_caveat, fixture.expect.result_rows_require_caveat,
            "row caveat mismatch in {path:?} ({})",
            fixture.case_name
        );
        for action in &fixture.expect.blocked_actions_include {
            assert!(
                state.blocked_actions.contains(action),
                "missing blocked action {action} in {path:?} ({})",
                fixture.case_name
            );
        }

        let support_artifact = IndexedStateSupportArtifact::from_lane_states(
            fixture.expect.support_artifact_id.clone(),
            state.observed_at.clone(),
            std::slice::from_ref(&state),
        );
        assert_eq!(
            support_artifact.artifact_id, fixture.expect.support_artifact_id,
            "support artifact id mismatch in {path:?}"
        );
        assert!(support_artifact.raw_private_material_excluded);
        assert_eq!(
            support_artifact.unsafe_current_claim_lanes(),
            Vec::<&str>::new(),
            "support artifact overclaims current truth in {path:?}"
        );

        covered_states.push(state.state);
    }

    for required in [
        IndexedStateClass::Warming,
        IndexedStateClass::Partial,
        IndexedStateClass::Cached,
        IndexedStateClass::Stale,
        IndexedStateClass::Paused,
    ] {
        assert!(
            covered_states.contains(&required),
            "missing protected indexed state {}",
            required.as_str()
        );
    }
}
