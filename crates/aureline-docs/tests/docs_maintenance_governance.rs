//! Fixture-driven coverage for docs-maintenance and stale-example governance.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_docs_maintenance_and_stale_example_governance_packet,
    seeded_docs_maintenance_and_stale_example_governance_input, DocsMaintenanceArtifactClass,
    DocsMaintenanceGovernanceFindingKind, DocsMaintenanceGovernancePacket,
    DocsMaintenanceGovernancePacketInput, DocsMaintenanceGovernancePromotionState,
    DocsMaintenanceGovernanceSurface, DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_DOC_REF,
    DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF, DOCS_MAINTENANCE_GOVERNANCE_DOC_REF,
    DOCS_MAINTENANCE_GOVERNANCE_FIXTURE_DIR, DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GovernanceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsMaintenanceGovernancePacketInput,
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

fn load_fixture(file_name: &str) -> GovernanceFixture {
    let path = repo_root()
        .join(DOCS_MAINTENANCE_GOVERNANCE_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(fixture.record_kind, "docs_maintenance_governance_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must explain what it proves",
        fixture.case_name
    );

    let packet = DocsMaintenanceGovernancePacket::materialize(fixture.input);
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

#[test]
fn doc_fixture_schema_and_artifact_exist_on_disk() {
    assert_exists(DOCS_MAINTENANCE_GOVERNANCE_DOC_REF);
    assert_exists(DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_DOC_REF);
    assert_exists(DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_REF);
    assert_exists(DOCS_MAINTENANCE_GOVERNANCE_FIXTURE_DIR);
    assert_exists(DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn rendered_preview_claims_canonical_fixture_blocks_stable() {
    assert_fixture_matches("rendered_preview_claims_canonical_blocks_stable.json");
}

#[test]
fn suppression_loses_actor_fixture_needs_review() {
    assert_fixture_matches("suppression_loses_actor_blocks_review.json");
}

#[test]
fn projection_drops_publish_boundary_fixture_blocks_stable() {
    assert_fixture_matches("projection_drops_publish_boundary_blocks_stable.json");
}

#[test]
fn checked_in_packet_covers_required_artifacts_and_surfaces() {
    let packet = current_docs_maintenance_and_stale_example_governance_packet()
        .expect("checked-in governance packet validates");
    assert_eq!(
        packet.promotion_state,
        DocsMaintenanceGovernancePromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    let artifact_classes: BTreeSet<&str> = packet
        .maintenance_packets
        .iter()
        .map(|packet| packet.artifact_class.as_str())
        .collect();
    for required in DocsMaintenanceArtifactClass::REQUIRED {
        assert!(
            artifact_classes.contains(required.as_str()),
            "stable packet must cover artifact class {}",
            required.as_str()
        );
    }

    for surface in DocsMaintenanceGovernanceSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} projection",
            surface.as_str()
        );
    }
}

#[test]
fn artifact_file_matches_seeded_packet() {
    let path = repo_root().join(DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DocsMaintenanceGovernancePacket = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed = DocsMaintenanceGovernancePacket::materialize(
        seeded_docs_maintenance_and_stale_example_governance_input(),
    );
    assert_eq!(
        from_file, from_seed,
        "checked-in docs-maintenance governance packet drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_maintenance_governance -- packet > artifacts/docs/m4/docs-maintenance-and-stale-example-governance.json`",
    );
}

#[test]
fn support_export_preserves_packet_without_raw_boundary_material() {
    let packet = current_docs_maintenance_and_stale_example_governance_packet()
        .expect("checked-in governance packet validates");
    let export = packet.support_export(
        "support-export:docs-maintenance-governance:test",
        "2026-06-04T17:15:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.export_packet_id_ref, packet.packet_id);
    assert!(!export.raw_boundary_material_exported);
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        DocsMaintenanceGovernanceFindingKind::RenderedPreviewCanonicalized.as_str(),
        "rendered_preview_canonicalized"
    );
    assert_eq!(
        DocsMaintenanceGovernanceFindingKind::ConsumerProjectionDroppedVocabulary.as_str(),
        "consumer_projection_dropped_vocabulary"
    );
}
