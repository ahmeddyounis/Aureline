//! Integration coverage for source-intelligence and relationship matrix packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    InfrastructureFamily, QualificationPosture, RelationEdgeClass,
    SourceIntelligenceRelationshipMatrixPacket, TruthLayer,
    SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND,
    SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION,
};

#[test]
fn qualified_fixture_validates() {
    let packet = load_fixture("qualified_matrix_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(
        packet.record_kind,
        SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION
    );
}

#[test]
fn qualified_fixture_covers_families_truth_and_edges() {
    let report = load_fixture("qualified_matrix_packet.json").validate();
    for required in [
        InfrastructureFamily::TerraformHcl,
        InfrastructureFamily::KubernetesHelm,
        InfrastructureFamily::Devcontainer,
        InfrastructureFamily::CiEnvironment,
        InfrastructureFamily::PolicyManifest,
    ] {
        assert!(report.families.contains(&required));
    }
    for required in [
        TruthLayer::AuthoredDesired,
        TruthLayer::RenderedExpanded,
        TruthLayer::PlannedValidated,
        TruthLayer::ObservedLive,
        TruthLayer::ProviderOverlay,
    ] {
        assert!(report.truth_layers.contains(&required));
    }
    for required in [
        RelationEdgeClass::SourceOfRender,
        RelationEdgeClass::PlanFor,
        RelationEdgeClass::LiveCounterpartOf,
        RelationEdgeClass::AppliedBy,
        RelationEdgeClass::OwnedBy,
        RelationEdgeClass::Impacts,
        RelationEdgeClass::RunbookReference,
        RelationEdgeClass::ReviewAnchor,
        RelationEdgeClass::ProviderOverlayOf,
    ] {
        assert!(report.relation_edges.contains(&required));
    }
}

#[test]
fn file_only_fixture_passes_with_explicit_degrade_profiles() {
    let packet = load_fixture("file_only_downgraded_matrix_packet.json");
    let report = packet.validate();
    assert!(
        report.passed,
        "file-only downgrade fixture must pass: {:#?}",
        report.findings
    );
    for row in &packet.matrix_rows {
        assert!(row
            .downgrade_profiles
            .iter()
            .any(|profile| matches!(profile.posture, QualificationPosture::FileOnly)));
    }
}

#[test]
fn missing_truth_layer_and_profile_fixture_fails() {
    let report = load_fixture("missing_truth_layer_and_profile_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "truth_layer_coverage"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "downgrade_profile_coverage"));
}

fn load_fixture(name: &str) -> SourceIntelligenceRelationshipMatrixPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/source-intelligence-and-resource-relationships")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
