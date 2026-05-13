//! Fixture-driven coverage for deep-link remap packets and history continuity.
//!
//! The fixtures under `fixtures/search/remap_packets_alpha/` exercise moved
//! files, renamed symbols, missing targets, and cross-root recovery outside an
//! active workset. The test validates both the search-owned remap packet and
//! the workspace-history continuity consumer.

use std::path::Path;

use serde::Deserialize;

use aureline_search::DeepLinkRemapPacket;
use aureline_workspace::{NavigationContinuityRecord, NavigationSurfaceClass};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    remap_packet: DeepLinkRemapPacket,
    continuity: NavigationContinuityRecord,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    outcome_class: String,
    continuity_state: String,
    deep_link_drift_state: String,
    old_target_ref: String,
    new_target_ref: Option<String>,
    requires_destination_visibility: bool,
    recovery_actions: Vec<String>,
}

#[test]
fn remap_packets_alpha_cases_validate_packet_and_continuity() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/remap_packets_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        fixtures.len() >= 4,
        "remap packet fixtures must cover remap, placeholder, and failure states"
    );

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
            .remap_packet
            .validate()
            .unwrap_or_else(|err| panic!("remap packet invalid in {path:?}: {err}"));
        fixture
            .continuity
            .validate()
            .unwrap_or_else(|err| panic!("continuity invalid in {path:?}: {err}"));

        assert_eq!(
            fixture.continuity.remap_packet_id_ref.as_deref(),
            Some(fixture.remap_packet.remap_packet_id.as_str()),
            "continuity must consume the remap packet in {path:?}"
        );
        assert_eq!(
            fixture.remap_packet.outcome_class.as_str(),
            fixture.expect.outcome_class,
            "outcome mismatch in {path:?}"
        );
        assert_eq!(
            fixture.continuity.continuity_state.as_str(),
            fixture.expect.continuity_state,
            "continuity state mismatch in {path:?}"
        );
        assert_eq!(
            fixture.remap_packet.deep_link_drift_state.as_str(),
            fixture.expect.deep_link_drift_state,
            "drift state mismatch in {path:?}"
        );
        assert_eq!(
            fixture.remap_packet.old_target.target_ref, fixture.expect.old_target_ref,
            "old target mismatch in {path:?}"
        );
        assert_eq!(
            fixture
                .remap_packet
                .new_target
                .as_ref()
                .map(|target| target.target_ref.as_str()),
            fixture.expect.new_target_ref.as_deref(),
            "new target mismatch in {path:?}"
        );

        let actual_action_tokens = fixture.remap_packet.recovery_action_tokens();
        assert_eq!(
            actual_action_tokens,
            fixture
                .expect
                .recovery_actions
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            "packet recovery actions mismatch in {path:?}"
        );
        let continuity_action_tokens = fixture
            .continuity
            .recovery_actions
            .iter()
            .map(|action| action.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            continuity_action_tokens, actual_action_tokens,
            "continuity actions must mirror packet actions in {path:?}"
        );

        assert_eq!(
            fixture.remap_packet.scope.crosses_visible_boundary(),
            fixture.expect.requires_destination_visibility,
            "destination visibility expectation mismatch in {path:?}"
        );
        if fixture.expect.requires_destination_visibility {
            assert_required_destination_surfaces(&fixture.remap_packet.destination_visibility);
            assert_required_destination_surfaces(&fixture.continuity.destination_visibility);
            assert_eq!(
                fixture.remap_packet.scope.workset_id_ref.as_deref(),
                fixture.continuity.scope_identity.workset_id_ref.as_deref(),
                "workset identity must survive cross-root recovery in {path:?}"
            );
            assert_eq!(
                fixture.remap_packet.scope.stable_scope_id_ref,
                fixture.continuity.scope_identity.stable_scope_id_ref,
                "stable scope identity must survive cross-root recovery in {path:?}"
            );
        }
    }
}

fn assert_required_destination_surfaces(
    visibility: &[aureline_workspace::NavigationDestinationVisibility],
) {
    const REQUIRED: [NavigationSurfaceClass; 5] = [
        NavigationSurfaceClass::Peek,
        NavigationSurfaceClass::Preview,
        NavigationSurfaceClass::Split,
        NavigationSurfaceClass::OpenInNewPane,
        NavigationSurfaceClass::BackNavigation,
    ];
    for surface in REQUIRED {
        let row = visibility
            .iter()
            .find(|row| row.surface_class == surface)
            .unwrap_or_else(|| panic!("missing destination visibility for {}", surface.as_str()));
        assert!(
            row.destination_repo_visible,
            "destination must be visible on {}",
            surface.as_str()
        );
        assert!(
            !row.target_root_id_ref.is_empty() && !row.target_root_label.is_empty(),
            "destination surface {} must carry root identity",
            surface.as_str()
        );
    }
}
