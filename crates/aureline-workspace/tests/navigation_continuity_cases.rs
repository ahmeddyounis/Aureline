//! Fixture-driven coverage for bookmark and navigation-history continuity.
//!
//! These tests read the remap fixture family from the search lane and validate
//! the workspace-history consumer projection for each case.

use std::path::Path;

use serde::Deserialize;

use aureline_workspace::{NavigationContinuityRecord, NavigationContinuityState};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    continuity: NavigationContinuityRecord,
}

#[test]
fn navigation_continuity_cases_validate_workspace_consumer() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/remap_packets_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    let mut saw_remapped = false;
    let mut saw_placeholder = false;
    let mut saw_failed = false;

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        assert_eq!(
            fixture.record_kind, "remap_packet_alpha_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(fixture.schema_version, 1, "schema mismatch in {path:?}");
        fixture
            .continuity
            .validate()
            .unwrap_or_else(|err| panic!("continuity invalid in {path:?}: {err}"));

        match fixture.continuity.continuity_state {
            NavigationContinuityState::Remapped => saw_remapped = true,
            NavigationContinuityState::RecoverablePlaceholder => saw_placeholder = true,
            NavigationContinuityState::FailedExplicitReason => saw_failed = true,
        }
    }

    assert!(
        saw_remapped,
        "fixtures must include a remapped continuity case"
    );
    assert!(
        saw_placeholder,
        "fixtures must include a recoverable placeholder continuity case"
    );
    assert!(
        saw_failed,
        "fixtures must include an explicit failed continuity case"
    );
}
