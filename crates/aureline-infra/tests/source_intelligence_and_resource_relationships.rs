//! Integration coverage for source-intelligence and relationship matrix packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    seeded_source_intelligence_object_packet, InfrastructureConsumerSurface, InfrastructureFamily,
    InfrastructureJourneyStatus, InfrastructureJourneySurface, QualificationPosture,
    RelationEdgeClass, SourceIntelligenceObjectPacket, SourceIntelligenceRelationshipMatrixPacket,
    TruthLayer, SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION, SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND,
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

#[test]
fn qualified_object_fixture_validates() {
    let packet = load_object_fixture("qualified_object_packet.json");
    let report = packet.validate();
    assert!(
        report.passed,
        "qualified object fixture must pass: {:#?}",
        report.findings
    );
    assert_eq!(
        packet.record_kind,
        SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION
    );
}

#[test]
fn qualified_object_fixture_covers_consumers() {
    let packet = load_object_fixture("qualified_object_packet.json");
    let report = packet.validate();
    for required in [
        InfrastructureConsumerSurface::Graph,
        InfrastructureConsumerSurface::Review,
        InfrastructureConsumerSurface::Docs,
        InfrastructureConsumerSurface::Incident,
    ] {
        assert!(report.consumer_surfaces.contains(&required));
        assert!(packet.consumer_projection(required).is_some());
    }
}

#[test]
fn qualified_object_fixture_matches_seeded_packet() {
    let fixture = load_object_fixture("qualified_object_packet.json");
    let seeded = seeded_source_intelligence_object_packet();
    assert_eq!(fixture, seeded);
}

#[test]
fn qualified_object_fixture_supports_shared_relation_journeys() {
    let packet = load_object_fixture("qualified_object_packet.json");

    let review_journey = packet
        .surface_view(InfrastructureJourneySurface::ReviewWorkspace)
        .expect("review surface")
        .show_live_counterpart("obj:tf:planned");
    assert_eq!(review_journey.status, InfrastructureJourneyStatus::Resolved);
    assert_eq!(
        review_journey.relation_refs,
        vec!["rel:tf:live_counterpart"]
    );

    let docs_journey = packet
        .surface_view(InfrastructureJourneySurface::DocsCards)
        .expect("docs surface")
        .show_applied_by("obj:ci:observed");
    assert_eq!(docs_journey.status, InfrastructureJourneyStatus::Resolved);
    assert_eq!(docs_journey.relation_refs, vec!["rel:ci:applied_by"]);

    let incident_slice = packet
        .surface_view(InfrastructureJourneySurface::IncidentTimeline)
        .expect("incident surface")
        .explain_environment_slice("ctx:policy");
    assert_eq!(incident_slice.status, InfrastructureJourneyStatus::Resolved);
    assert!(incident_slice
        .impact_relation_refs
        .contains(&"rel:policy:impacts".to_string()));
}

#[test]
fn missing_rendered_lineage_object_fixture_fails() {
    let report = load_object_fixture("missing_rendered_lineage_object_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "derived_lineage"));
}

#[test]
fn dropping_docs_flow_edge_fails_validation() {
    let mut packet = seeded_source_intelligence_object_packet();
    let docs_projection = packet
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.surface == InfrastructureConsumerSurface::Docs)
        .expect("docs projection");
    docs_projection
        .relation_refs
        .retain(|relation_ref| relation_ref != "rel:ci:applied_by");

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "projection_flow_coverage"));
}

fn load_fixture(name: &str) -> SourceIntelligenceRelationshipMatrixPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/source-intelligence-and-resource-relationships")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}

fn load_object_fixture(name: &str) -> SourceIntelligenceObjectPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/source-intelligence-and-resource-relationships")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
