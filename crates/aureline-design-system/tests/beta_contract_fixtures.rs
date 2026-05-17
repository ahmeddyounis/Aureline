//! Fixture replay for the beta design-system contract records.
//!
//! The fixtures under `fixtures/ux/m3/state_semantics/` and
//! `artifacts/ux/m3/component_state_screenshot_diff/` are generated from the
//! same seeded builders this test calls. A mismatch means the checked-in release
//! evidence drifted from the Rust contract.

use aureline_design_system::{
    seeded_component_state_registry, seeded_screenshot_diff_packet,
    seeded_token_conformance_packet, validate_component_state_registry,
    validate_screenshot_diff_packet, validate_token_conformance_packet,
    ComponentStateRegistryRecord, ScreenshotDiffPacket, TokenConformancePacket,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m3/state_semantics"
);

const SCREENSHOT_ARTIFACT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ux/m3/component_state_screenshot_diff/packet.json"
);

fn load<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let body =
        std::fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn registry_fixture_matches_seeded_builder() {
    let path = format!("{FIXTURE_DIR}/component_state_registry.json");
    let fixture: ComponentStateRegistryRecord = load(&path);
    validate_component_state_registry(&fixture).expect("fixture registry validates");
    assert_eq!(fixture, seeded_component_state_registry());
}

#[test]
fn screenshot_fixtures_match_seeded_builder() {
    let fixture_path = format!("{FIXTURE_DIR}/screenshot_diff_matrix.json");
    let fixture: ScreenshotDiffPacket = load(&fixture_path);
    let artifact: ScreenshotDiffPacket = load(SCREENSHOT_ARTIFACT);
    validate_screenshot_diff_packet(&fixture).expect("fixture screenshot packet validates");
    validate_screenshot_diff_packet(&artifact).expect("artifact screenshot packet validates");
    let seeded = seeded_screenshot_diff_packet();
    assert_eq!(fixture, seeded);
    assert_eq!(artifact, seeded);
}

#[test]
fn token_conformance_fixture_matches_seeded_builder() {
    let path = format!("{FIXTURE_DIR}/token_conformance_report.json");
    let fixture: TokenConformancePacket = load(&path);
    validate_token_conformance_packet(&fixture).expect("fixture token conformance validates");
    assert_eq!(fixture, seeded_token_conformance_packet());
}
