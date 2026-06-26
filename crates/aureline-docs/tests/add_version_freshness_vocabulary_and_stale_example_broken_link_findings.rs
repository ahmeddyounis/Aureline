//! Fixture-driven coverage for the docs version-freshness findings packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_version_freshness_export, current_stable_docs_version_freshness_packet,
    seeded_stable_docs_version_freshness_input, DocsVersionFreshnessConsumerSurface,
    DocsVersionFreshnessPacket, DocsVersionFreshnessPacketInput,
    DocsVersionFreshnessPromotionState, DocsVersionFreshnessState,
    DocsVersionFreshnessSupportExport, DOCS_VERSION_FRESHNESS_ARTIFACT_REF,
    DOCS_VERSION_FRESHNESS_DOC_REF, DOCS_VERSION_FRESHNESS_FIXTURE_DIR,
    DOCS_VERSION_FRESHNESS_SCHEMA_REF, DOCS_VERSION_FRESHNESS_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_version_freshness_findings:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct VersionFreshnessFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsVersionFreshnessPacketInput,
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

fn load_fixture(file_name: &str) -> VersionFreshnessFixture {
    let path = repo_root()
        .join(DOCS_VERSION_FRESHNESS_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(fixture.record_kind, "docs_version_freshness_findings_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsVersionFreshnessPacket::materialize(fixture.input);
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
    assert_exists(DOCS_VERSION_FRESHNESS_DOC_REF);
    assert_exists(DOCS_VERSION_FRESHNESS_SCHEMA_REF);
    assert_exists(DOCS_VERSION_FRESHNESS_ARTIFACT_REF);
    assert_exists(DOCS_VERSION_FRESHNESS_SUMMARY_REF);
    assert_exists(DOCS_VERSION_FRESHNESS_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn cached_shares_exact_confidence_fixture_blocks_stable() {
    assert_fixture_matches("cached_shares_exact_confidence_blocks_stable.json");
}

#[test]
fn version_mismatch_hidden_fixture_blocks_stable() {
    assert_fixture_matches("version_mismatch_hidden_blocks_stable.json");
}

#[test]
fn broken_link_finding_fixture_blocks_stable() {
    assert_fixture_matches("broken_link_finding_blocks_stable.json");
}

#[test]
fn finding_actions_dropped_fixture_blocks_stable() {
    assert_fixture_matches("finding_actions_dropped_blocks_stable.json");
}

#[test]
fn finding_orphan_fixture_blocks_stable() {
    assert_fixture_matches("finding_orphan_blocks_stable.json");
}

#[test]
fn state_distinction_collapsed_fixture_blocks_stable() {
    assert_fixture_matches("state_distinction_collapsed_blocks_stable.json");
}

#[test]
fn vocabulary_coverage_incomplete_fixture_blocks_stable() {
    assert_fixture_matches("vocabulary_coverage_incomplete_blocks_stable.json");
}

#[test]
fn policy_blocked_reason_missing_fixture_blocks_stable() {
    assert_fixture_matches("policy_blocked_reason_missing_blocks_stable.json");
}

#[test]
fn checked_in_packet_carries_the_full_vocabulary_and_every_surface() {
    let packet = current_stable_docs_version_freshness_packet()
        .expect("stable docs version-freshness packet validates");
    assert_eq!(
        packet.promotion_state,
        DocsVersionFreshnessPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    for state in DocsVersionFreshnessState::ALL {
        assert!(
            packet.state_tokens().contains(&state.as_str()),
            "stable packet must exercise the {} state",
            state.as_str()
        );
    }
    for surface in DocsVersionFreshnessConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must project the {} surface",
            surface.as_str()
        );
    }
    // The canonical packet carries actionable stale-example and broken-link
    // findings while staying stable.
    assert!(!packet.findings.is_empty());
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(DOCS_VERSION_FRESHNESS_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DocsVersionFreshnessSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input())
            .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- support-export > {}`",
        DOCS_VERSION_FRESHNESS_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export =
        current_stable_docs_version_freshness_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}
