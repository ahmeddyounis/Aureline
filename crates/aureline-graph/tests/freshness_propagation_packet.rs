//! Fixture-driven coverage for the stable graph freshness-propagation
//! packet (graph object/query handles, producer identity, schema
//! version, visibility scope, retention class, epoch labels,
//! invalidation scope, and hidden-graph dependency disclosure).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_freshness_propagation_packet, FreshnessPropagationConsumerSurface,
    FreshnessPropagationFindingKind, FreshnessPropagationPacket, FreshnessPropagationPacketInput,
    FreshnessPropagationPromotionState, GraphEpochClass, HiddenGraphDependencyState,
    InvalidationScopeClass, FRESHNESS_PROPAGATION_PACKET_ARTIFACT_DOC_REF,
    FRESHNESS_PROPAGATION_PACKET_ARTIFACT_REF, FRESHNESS_PROPAGATION_PACKET_DOC_REF,
    FRESHNESS_PROPAGATION_PACKET_FIXTURE_DIR,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PropagationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: FreshnessPropagationPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    epoch_class_tokens: Vec<String>,
    freshness_class_tokens: Vec<String>,
    confidence_class_tokens: Vec<String>,
    support_export_safe: bool,
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

fn load_fixture(file_name: &str) -> PropagationFixture {
    let path = repo_root()
        .join(FRESHNESS_PROPAGATION_PACKET_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "graph_freshness_propagation_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = FreshnessPropagationPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.epoch_class_tokens(),
        expect
            .epoch_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.freshness_class_tokens(),
        expect
            .freshness_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.confidence_class_tokens(),
        expect
            .confidence_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(FRESHNESS_PROPAGATION_PACKET_DOC_REF);
    assert_exists(FRESHNESS_PROPAGATION_PACKET_ARTIFACT_DOC_REF);
    assert_exists(FRESHNESS_PROPAGATION_PACKET_FIXTURE_DIR);
    assert_exists(FRESHNESS_PROPAGATION_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn mixed_epoch_unlabeled_fixture_blocks_stable() {
    assert_fixture_matches("mixed_epoch_unlabeled_blocks_stable.json");
}

#[test]
fn full_rebuild_not_surfaced_fixture_blocks_stable() {
    assert_fixture_matches("full_rebuild_not_surfaced_blocks_stable.json");
}

#[test]
fn hidden_dependency_undisclosed_fixture_blocks_stable() {
    assert_fixture_matches("hidden_dependency_undisclosed_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_epoch_label_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_epoch_label_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_required_surfaces() {
    let packet =
        current_stable_freshness_propagation_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        FreshnessPropagationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    for surface in FreshnessPropagationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }

    // Canonical packet must cover the four runtime epoch classes
    // (local_live, remote_synced, imported_provider, cached_snapshot).
    let epoch_tokens: BTreeSet<&str> = packet.epoch_class_tokens().into_iter().collect();
    for required in [
        GraphEpochClass::LocalLive,
        GraphEpochClass::RemoteSynced,
        GraphEpochClass::ImportedProvider,
        GraphEpochClass::CachedSnapshot,
    ] {
        assert!(
            epoch_tokens.contains(required.as_str()),
            "checked-in packet must cover epoch class {}",
            required.as_str()
        );
    }
}

#[test]
fn closed_propagation_tokens_are_pinned() {
    assert_eq!(GraphEpochClass::LocalLive.as_str(), "local_live");
    assert_eq!(
        InvalidationScopeClass::FullRebuildWorkspaceEpochBoundary.as_str(),
        "full_rebuild_workspace_epoch_boundary"
    );
    assert_eq!(
        HiddenGraphDependencyState::HiddenDependencyUndisclosed.as_str(),
        "hidden_dependency_undisclosed"
    );
    assert_eq!(
        FreshnessPropagationFindingKind::MixedEpochUnlabeled.as_str(),
        "mixed_epoch_unlabeled"
    );
    assert_eq!(
        FreshnessPropagationFindingKind::FullRebuildNotSurfaced.as_str(),
        "full_rebuild_not_surfaced"
    );
    assert_eq!(
        FreshnessPropagationFindingKind::HiddenGraphDependencyUndisclosed.as_str(),
        "hidden_graph_dependency_undisclosed"
    );
    assert_eq!(
        FreshnessPropagationConsumerSurface::TopologySurface.as_str(),
        "topology_surface"
    );
}
