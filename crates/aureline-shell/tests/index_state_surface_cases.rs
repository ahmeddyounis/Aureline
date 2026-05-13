//! Fixture-driven coverage for indexed-state shell surfaces.
//!
//! These cases reuse the canonical search fixtures and assert that the shell
//! status item, result-pane notice, and support artifact quote the same state
//! token for warming, partial, cached, stale, and paused indexed data.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{IndexedLaneState, IndexedLaneStateInput};
use aureline_shell::status::index_state::IndexStateSurfaceBundle;

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
    status_current_value_label: String,
    result_banner_label: String,
    row_caveat_label: String,
    degraded_token: Option<String>,
    support_artifact_id: String,
}

#[test]
fn shell_surfaces_quote_canonical_indexed_state_tokens() {
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
        let bundle = IndexStateSurfaceBundle::from_lane_state(
            fixture.expect.support_artifact_id.clone(),
            state.observed_at.clone(),
            &state,
        );

        assert!(
            bundle.has_single_state_token(),
            "surface tokens drifted in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.state_token, fixture.expect.state_token,
            "bundle state token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.status.current_value_label, fixture.expect.status_current_value_label,
            "status label mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.status.degraded_token, fixture.expect.degraded_token,
            "status degraded token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.result_pane.banner_label, fixture.expect.result_banner_label,
            "result banner mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.result_pane.row_caveat_label, fixture.expect.row_caveat_label,
            "row caveat mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.support_artifact.artifact_id, fixture.expect.support_artifact_id,
            "support artifact id mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            bundle.support_artifact.unsafe_current_claim_lanes(),
            Vec::<&str>::new(),
            "support artifact overclaims current truth in {path:?} ({})",
            fixture.case_name
        );
    }
}
