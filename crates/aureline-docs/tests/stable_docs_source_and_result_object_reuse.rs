//! Fixture-driven coverage for the stable docs-source/result object reuse packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_source_result_reuse_export, current_stable_docs_source_result_reuse_packet,
    seeded_stable_docs_source_result_reuse_input, DocsObjectConsumerSurface, DocsObjectFindingKind,
    DocsObjectPromotionState, DocsObjectReusePacket, DocsObjectReusePacketInput,
    DocsObjectReuseSupportExport, DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF,
    DOCS_SOURCE_RESULT_REUSE_DOC_REF, DOCS_SOURCE_RESULT_REUSE_FIXTURE_DIR,
    DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF, DOCS_SOURCE_RESULT_REUSE_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:stable_docs_source_and_result_object_reuse:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct ReuseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsObjectReusePacketInput,
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

fn load_fixture(file_name: &str) -> ReuseFixture {
    let path = repo_root()
        .join(DOCS_SOURCE_RESULT_REUSE_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(fixture.record_kind, "docs_source_result_reuse_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsObjectReusePacket::materialize(fixture.input);
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
    assert_exists(DOCS_SOURCE_RESULT_REUSE_DOC_REF);
    assert_exists(DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF);
    assert_exists(DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF);
    assert_exists(DOCS_SOURCE_RESULT_REUSE_SUMMARY_REF);
    assert_exists(DOCS_SOURCE_RESULT_REUSE_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn project_docs_relabeled_as_vendor_fixture_blocks_stable() {
    assert_fixture_matches("project_docs_relabeled_as_vendor_blocks_stable.json");
}

#[test]
fn derived_explanation_claims_precedence_fixture_blocks_stable() {
    assert_fixture_matches("derived_explanation_claims_precedence_blocks_stable.json");
}

#[test]
fn live_external_inlined_without_handoff_fixture_blocks_stable() {
    assert_fixture_matches("live_external_inlined_without_handoff_blocks_stable.json");
}

#[test]
fn result_freshness_drift_fixture_blocks_stable() {
    assert_fixture_matches("result_freshness_drift_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_truth_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_truth_blocks_stable.json");
}

#[test]
fn checked_in_packet_reuses_objects_across_every_surface() {
    let packet =
        current_stable_docs_source_result_reuse_packet().expect("stable reuse packet validates");
    assert_eq!(packet.promotion_state, DocsObjectPromotionState::Stable);
    assert!(packet.validate().is_empty());

    let surfaces: BTreeSet<&str> = packet
        .surface_projections
        .iter()
        .map(|projection| projection.consumer_surface.as_str())
        .collect();
    for surface in DocsObjectConsumerSurface::REQUIRED {
        assert!(
            surfaces.contains(surface.as_str()),
            "stable packet must reuse objects on the {} surface",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DocsObjectReuseSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed =
        DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input())
            .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_source_result_reuse -- support-export > {}`",
        DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export =
        current_stable_docs_source_result_reuse_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        DocsObjectFindingKind::SourceResultTruthMismatch.as_str(),
        "source_result_truth_mismatch"
    );
    assert_eq!(
        DocsObjectFindingKind::SourceTrustClassMismatch.as_str(),
        "source_trust_class_mismatch"
    );
    assert_eq!(
        DocsObjectFindingKind::DerivedExplanationMasqueradesAsPrimary.as_str(),
        "derived_explanation_masquerades_as_primary"
    );
    assert_eq!(
        DocsObjectFindingKind::ConsumerSurfaceProjectionDrift.as_str(),
        "consumer_surface_projection_drift"
    );
}
