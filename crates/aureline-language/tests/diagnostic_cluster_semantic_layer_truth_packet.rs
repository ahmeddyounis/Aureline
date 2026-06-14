//! Fixture-driven coverage for the stable diagnostic-cluster semantic-layer
//! truth packet that binds diagnostic clustering, semantic-layer banners,
//! freshness/scope labels, and the cluster detail-sheet model across the M5
//! notebook, framework, preview, and generated-code surfaces. Each row names
//! the converged diagnostic source families, keeps per-provider detail and the
//! losing provider of a disagreement inspectable, matches the semantic-layer
//! banner to the freshness evidence, and binds a typed preview and rollback
//! checkpoint to every mutating fix.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_diagnostic_cluster_semantic_layer_truth_packet, ClusterLaneClass,
    DiagnosticClusterSemanticLayerTruthPacket, DiagnosticClusterSemanticLayerTruthPacketInput,
    DiagnosticClusterSurfaceClass, DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_ARTIFACT_DOC_REF,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_ARTIFACT_REF,
    DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ClusterFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DiagnosticClusterSemanticLayerTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    surface_tokens: Vec<String>,
    cluster_lane_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    diagnostic_source_tokens: Vec<String>,
    cluster_provenance_tokens: Vec<String>,
    source_differentiation_tokens: Vec<String>,
    detail_sheet_route_tokens: Vec<String>,
    semantic_layer_banner_tokens: Vec<String>,
    freshness_tokens: Vec<String>,
    scope_label_tokens: Vec<String>,
    provider_family_tokens: Vec<String>,
    conflict_tokens: Vec<String>,
    provider_disagreement_visibility_tokens: Vec<String>,
    fix_offer_tokens: Vec<String>,
    completeness_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ClusterFixture {
    let path = repo_root()
        .join(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "diagnostic_cluster_semantic_layer_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = DiagnosticClusterSemanticLayerTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
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

    assert_token_set_matches(&packet.surface_tokens(), &expect.surface_tokens, "surface");
    assert_token_set_matches(
        &packet.cluster_lane_tokens(),
        &expect.cluster_lane_tokens,
        "cluster_lane",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(
        &packet.diagnostic_source_tokens(),
        &expect.diagnostic_source_tokens,
        "diagnostic_source",
    );
    assert_token_set_matches(
        &packet.cluster_provenance_tokens(),
        &expect.cluster_provenance_tokens,
        "cluster_provenance",
    );
    assert_token_set_matches(
        &packet.source_differentiation_tokens(),
        &expect.source_differentiation_tokens,
        "source_differentiation",
    );
    assert_token_set_matches(
        &packet.detail_sheet_route_tokens(),
        &expect.detail_sheet_route_tokens,
        "detail_sheet_route",
    );
    assert_token_set_matches(
        &packet.semantic_layer_banner_tokens(),
        &expect.semantic_layer_banner_tokens,
        "semantic_layer_banner",
    );
    assert_token_set_matches(
        &packet.freshness_tokens(),
        &expect.freshness_tokens,
        "freshness",
    );
    assert_token_set_matches(
        &packet.scope_label_tokens(),
        &expect.scope_label_tokens,
        "scope_label",
    );
    assert_token_set_matches(
        &packet.provider_family_tokens(),
        &expect.provider_family_tokens,
        "provider_family",
    );
    assert_token_set_matches(
        &packet.conflict_tokens(),
        &expect.conflict_tokens,
        "conflict",
    );
    assert_token_set_matches(
        &packet.provider_disagreement_visibility_tokens(),
        &expect.provider_disagreement_visibility_tokens,
        "provider_disagreement_visibility",
    );
    assert_token_set_matches(
        &packet.fix_offer_tokens(),
        &expect.fix_offer_tokens,
        "fix_offer",
    );
    assert_token_set_matches(
        &packet.completeness_tokens(),
        &expect.completeness_tokens,
        "completeness",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-14T12:00:10Z",
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
                "fixture {} expected finding kind {kind}; observed {observed:?}",
                fixture.case_name,
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF);
    assert_exists(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF);
    assert_exists(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR);
    assert_exists(DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn cluster_provenance_collapsed_blocks_stable() {
    assert_fixture_matches("cluster_provenance_collapsed_blocks_stable.json");
}

#[test]
fn dropped_suppression_state_blocks_stable() {
    assert_fixture_matches("dropped_suppression_state_blocks_stable.json");
}

#[test]
fn sources_fused_undifferentiated_blocks_stable() {
    assert_fixture_matches("sources_fused_undifferentiated_blocks_stable.json");
}

#[test]
fn losing_provider_collapsed_blocks_stable() {
    assert_fixture_matches("losing_provider_collapsed_blocks_stable.json");
}

#[test]
fn opaque_detail_sheet_route_blocks_stable() {
    assert_fixture_matches("opaque_detail_sheet_route_blocks_stable.json");
}

#[test]
fn multi_source_without_detail_sheet_blocks_stable() {
    assert_fixture_matches("multi_source_without_detail_sheet_blocks_stable.json");
}

#[test]
fn semantic_banner_on_stale_evidence_blocks_stable() {
    assert_fixture_matches("semantic_banner_on_stale_evidence_blocks_stable.json");
}

#[test]
fn whole_workspace_scope_on_stale_evidence_blocks_stable() {
    assert_fixture_matches("whole_workspace_scope_on_stale_evidence_blocks_stable.json");
}

#[test]
fn fix_offered_without_freshness_blocks_stable() {
    assert_fixture_matches("fix_offered_without_freshness_blocks_stable.json");
}

#[test]
fn mutating_fix_without_rollback_blocks_stable() {
    assert_fixture_matches("mutating_fix_without_rollback_blocks_stable.json");
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("certified_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_surface_and_lane() {
    let packet = current_stable_diagnostic_cluster_semantic_layer_truth_packet()
        .expect("checked-in packet validates");
    assert!(packet.validate().is_empty());
    for surface in DiagnosticClusterSurfaceClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.surface_class == surface),
            "stable packet must cover surface {}",
            surface.as_str()
        );
    }
    for lane in ClusterLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.cluster_lane_class == lane),
            "stable packet must cover lane {}",
            lane.as_str()
        );
    }
}
