//! Integration coverage for infrastructure source-intelligence alpha.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use aureline_provider::{
    seeded_infrastructure_intelligence_alpha_page, InfrastructureConnectorKind,
    InfrastructureConsumerSurface, InfrastructureControlPlaneBoundary,
    InfrastructureIntelligenceAlphaPage, InfrastructurePartialityClass,
    InfrastructurePromotionState, InfrastructureRelationshipKind, InfrastructureSourceClass,
    InfrastructureTruthLayer, INFRASTRUCTURE_INTELLIGENCE_ALPHA_PAGE_RECORD_KIND,
    INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_infrastructure_intelligence_alpha_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: InfrastructureIntelligenceAlphaPage =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(
        parsed.record_kind,
        INFRASTRUCTURE_INTELLIGENCE_ALPHA_PAGE_RECORD_KIND
    );
    assert_eq!(
        parsed.schema_version,
        INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION
    );
    assert_eq!(parsed.connectors.len(), page.connectors.len());
    assert_eq!(parsed.relationships.len(), page.relationships.len());
}

#[test]
fn checked_in_fixture_validates() {
    let page = load_fixture();
    let report = page.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(
        page.promotion_gate.promotion_state,
        InfrastructurePromotionState::PromotionReady
    );
}

#[test]
fn fixture_covers_claimed_connector_and_truth_layers() {
    let page = load_fixture();
    let report = page.validate();
    let connector_kinds = report.coverage.connector_kinds;
    for required in [
        InfrastructureConnectorKind::TerraformWorkspace,
        InfrastructureConnectorKind::KubernetesCluster,
        InfrastructureConnectorKind::ContainerRuntime,
        InfrastructureConnectorKind::CiProvider,
        InfrastructureConnectorKind::PolicyEngine,
    ] {
        assert!(connector_kinds.contains(&required));
    }

    let source_classes = report.coverage.source_classes;
    for required in [
        InfrastructureSourceClass::TerraformHcl,
        InfrastructureSourceClass::KubernetesManifest,
        InfrastructureSourceClass::ContainerDescriptor,
        InfrastructureSourceClass::CiEnvironmentDescriptor,
        InfrastructureSourceClass::PolicyAccessConfig,
    ] {
        assert!(source_classes.contains(&required));
    }

    let truth_layers = report.coverage.truth_layers;
    for required in [
        InfrastructureTruthLayer::Authored,
        InfrastructureTruthLayer::Rendered,
        InfrastructureTruthLayer::Planned,
        InfrastructureTruthLayer::Observed,
        InfrastructureTruthLayer::ProviderOverlay,
    ] {
        assert!(truth_layers.contains(&required));
    }
}

#[test]
fn search_review_and_support_consume_same_relationship_packet() {
    let page = load_fixture();
    let projections: BTreeSet<_> = page
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    assert!(projections.contains(&InfrastructureConsumerSurface::Search));
    assert!(projections.contains(&InfrastructureConsumerSurface::Review));
    assert!(projections.contains(&InfrastructureConsumerSurface::Support));

    for projection in &page.consumer_projections {
        assert_eq!(projection.source_packet_ref, page.page_id);
        assert!(projection.uses_same_relationship_packet);
        assert!(!projection.parallel_truth_store_created);
    }

    let search = page.search_projection();
    let review = page.review_projection();
    let support = page.support_export_projection();
    assert_eq!(search.source_page_id, page.page_id);
    assert_eq!(review.source_page_id, page.page_id);
    assert_eq!(support.source_page_id, page.page_id);
}

#[test]
fn fixture_preserves_read_only_control_plane_boundary() {
    let page = load_fixture();
    for connector in &page.connectors {
        assert_ne!(
            connector.control_plane_boundary,
            InfrastructureControlPlaneBoundary::InProductMutationClaimed
        );
        assert!(connector.read_only_source_intelligence);
        assert!(connector.mutation_authority_excluded);
        assert!(!connector.hidden_ai_writes_allowed);
        assert!(!connector.hidden_provider_writes_allowed);
    }
    for resource in &page.resources {
        assert!(!resource.active_control_plane_mutation_authority);
        assert!(!resource.provider_overlay_replaces_repo_truth);
    }
}

#[test]
fn partial_index_and_retrieval_states_are_labelled() {
    let page = load_fixture();
    assert!(page.resources.iter().any(|resource| {
        resource.partiality == InfrastructurePartialityClass::PartialIndex
            && resource.partiality_reason.is_some()
    }));
    assert!(page.resources.iter().any(|resource| {
        resource.partiality == InfrastructurePartialityClass::PartialRetrieval
            && resource.partiality_reason.is_some()
    }));
    assert!(page.relationships.iter().any(|relationship| {
        relationship.partiality == InfrastructurePartialityClass::PartialIndex
            && relationship.partiality_reason.is_some()
    }));
}

#[test]
fn fixture_covers_core_relationship_shapes() {
    let page = load_fixture();
    let relationship_kinds: BTreeSet<_> = page
        .relationships
        .iter()
        .map(|relationship| relationship.relationship_kind)
        .collect();
    for required in [
        InfrastructureRelationshipKind::PlanToResource,
        InfrastructureRelationshipKind::SourceToRenderedObject,
        InfrastructureRelationshipKind::ObjectToLiveResource,
        InfrastructureRelationshipKind::ObjectToLogOrEventStream,
        InfrastructureRelationshipKind::RunToArtifact,
        InfrastructureRelationshipKind::PolicyToTargetResource,
        InfrastructureRelationshipKind::PolicyToEnforcementResult,
    ] {
        assert!(relationship_kinds.contains(&required));
    }
}

fn load_fixture() -> InfrastructureIntelligenceAlphaPage {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m3/infrastructure_connectors/page.json");
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
