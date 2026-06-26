//! Fixture-driven coverage for the browser/provider-console handoff packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_browser_handoff_export, current_stable_browser_handoff_packet,
    seeded_stable_browser_handoff_input, BrowserHandoffConsumerSurface, BrowserHandoffPacket,
    BrowserHandoffPacketInput, BrowserHandoffPromotionState, BrowserHandoffSupportExport,
    HandoffSourceSurface, BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF, BROWSER_HANDOFF_OBJECTS_DOC_REF,
    BROWSER_HANDOFF_OBJECTS_FIXTURE_DIR, BROWSER_HANDOFF_OBJECTS_SCHEMA_REF,
    BROWSER_HANDOFF_OBJECTS_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:browser_provider_console_handoff_objects:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct HandoffFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: BrowserHandoffPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
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

fn load_fixture(file_name: &str) -> HandoffFixture {
    let path = repo_root()
        .join(BROWSER_HANDOFF_OBJECTS_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind,
        "browser_provider_console_handoff_objects_case"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = BrowserHandoffPacket::materialize(fixture.input);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        fixture.expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );

    if !fixture.expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for expected in &fixture.expect.expected_finding_kinds {
            assert!(
                observed.contains(expected.as_str()),
                "fixture {} expected finding {expected}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn doc_schema_artifact_and_fixtures_exist_on_disk() {
    assert_exists(BROWSER_HANDOFF_OBJECTS_DOC_REF);
    assert_exists(BROWSER_HANDOFF_OBJECTS_SCHEMA_REF);
    assert_exists(BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF);
    assert_exists(BROWSER_HANDOFF_OBJECTS_SUMMARY_REF);
    assert_exists(BROWSER_HANDOFF_OBJECTS_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn hidden_context_share_fixture_blocks_stable() {
    assert_fixture_matches("hidden_context_share_blocks_stable.json");
}

#[test]
fn ordinary_navigation_shares_context_fixture_blocks_stable() {
    assert_fixture_matches("ordinary_navigation_shares_context_blocks_stable.json");
}

#[test]
fn raw_browser_open_bypass_fixture_blocks_stable() {
    assert_fixture_matches("raw_browser_open_bypass_blocks_stable.json");
}

#[test]
fn return_anchor_missing_fixture_blocks_stable() {
    assert_fixture_matches("return_anchor_missing_blocks_stable.json");
}

#[test]
fn privacy_consequence_inconsistent_fixture_blocks_stable() {
    assert_fixture_matches("privacy_consequence_inconsistent_blocks_stable.json");
}

#[test]
fn exit_coverage_missing_fixture_blocks_stable() {
    assert_fixture_matches("exit_coverage_missing_blocks_stable.json");
}

#[test]
fn history_drops_handoff_fixture_blocks_stable() {
    assert_fixture_matches("history_drops_handoff_blocks_stable.json");
}

#[test]
fn blocked_handoff_presented_available_fixture_blocks_stable() {
    assert_fixture_matches("blocked_handoff_presented_available_blocks_stable.json");
}

#[test]
fn blocked_handoff_fixture_narrows_below_stable() {
    assert_fixture_matches("blocked_handoff_narrows_below_stable.json");
}

#[test]
fn shared_context_blocked_fixture_narrows_below_stable() {
    assert_fixture_matches("shared_context_blocked_narrows_below_stable.json");
}

#[test]
fn checked_in_packet_routes_every_required_exit() {
    let packet =
        current_stable_browser_handoff_packet().expect("stable browser handoff packet validates");
    assert_eq!(packet.promotion_state, BrowserHandoffPromotionState::Stable);
    assert!(packet.validate().is_empty());

    for exit in HandoffSourceSurface::REQUIRED_EXITS {
        assert!(
            packet.covered_exits().contains(&exit),
            "stable packet must route the {} exit through a handoff",
            exit.as_str()
        );
    }

    // Help, support-export, and docs-history reconstruct every handoff.
    for surface in BrowserHandoffConsumerSurface::REQUIRED_RECONSTRUCTION {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must reconstruct the handoff on the {} surface",
            surface.as_str()
        );
    }
    for surface in BrowserHandoffConsumerSurface::FULL_COVERAGE {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.surface == surface)
            .unwrap_or_else(|| panic!("{} projection present", surface.as_str()));
        for handoff in &packet.handoffs {
            assert!(
                projection.handoff_id_refs.contains(&handoff.handoff_id),
                "{} must reconstruct handoff {}",
                surface.as_str(),
                handoff.handoff_id
            );
        }
    }
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: BrowserHandoffSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input())
        .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- support-export > {}`",
        BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export = current_stable_browser_handoff_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}
