//! Unit coverage for source-intelligence and relationship matrix packets.

use super::*;

fn repo_context() -> TargetContextRequirement {
    TargetContextRequirement {
        requirement_class: TargetContextRequirementClass::RepoScoped,
        required_fields: vec![
            TargetContextField::Provider,
            TargetContextField::WorkspaceRevision,
            TargetContextField::ExecutionOrigin,
        ],
        stable_across_surfaces: true,
        ambient_context_forbidden: true,
    }
}

fn render_context() -> TargetContextRequirement {
    TargetContextRequirement {
        requirement_class: TargetContextRequirementClass::RenderScoped,
        required_fields: vec![
            TargetContextField::Provider,
            TargetContextField::WorkspaceRevision,
            TargetContextField::ExecutionOrigin,
            TargetContextField::ToolIdentity,
        ],
        stable_across_surfaces: true,
        ambient_context_forbidden: true,
    }
}

fn planned_context() -> TargetContextRequirement {
    TargetContextRequirement {
        requirement_class: TargetContextRequirementClass::PlannedTargetResolved,
        required_fields: vec![
            TargetContextField::Provider,
            TargetContextField::WorkspaceRevision,
            TargetContextField::ExecutionOrigin,
            TargetContextField::ExecutionContextProfile,
            TargetContextField::ToolIdentity,
            TargetContextField::EnvironmentSelector,
        ],
        stable_across_surfaces: true,
        ambient_context_forbidden: true,
    }
}

fn live_context() -> TargetContextRequirement {
    TargetContextRequirement {
        requirement_class: TargetContextRequirementClass::LiveTargetResolved,
        required_fields: vec![
            TargetContextField::Provider,
            TargetContextField::AccountSubscriptionProject,
            TargetContextField::EnvironmentSelector,
            TargetContextField::NamespaceOrScope,
            TargetContextField::RegionZone,
            TargetContextField::WorkspaceRevision,
            TargetContextField::ExecutionOrigin,
            TargetContextField::ExecutionContextProfile,
            TargetContextField::ToolIdentity,
            TargetContextField::CredentialClass,
            TargetContextField::FreshnessTimestamp,
        ],
        stable_across_surfaces: true,
        ambient_context_forbidden: true,
    }
}

fn overlay_context() -> TargetContextRequirement {
    TargetContextRequirement {
        requirement_class: TargetContextRequirementClass::OverlayTargetResolved,
        required_fields: vec![
            TargetContextField::Provider,
            TargetContextField::AccountSubscriptionProject,
            TargetContextField::EnvironmentSelector,
            TargetContextField::Tenant,
            TargetContextField::WorkspaceRevision,
            TargetContextField::ExecutionOrigin,
            TargetContextField::ExecutionContextProfile,
            TargetContextField::ToolIdentity,
            TargetContextField::FreshnessTimestamp,
        ],
        stable_across_surfaces: true,
        ambient_context_forbidden: true,
    }
}

fn binding(layer: TruthLayer) -> TruthLayerBinding {
    let (target_context_requirement, console_handoff_posture, export_fidelity, file_posture) =
        match layer {
            TruthLayer::AuthoredDesired => (
                repo_context(),
                ConsoleHandoffPosture::NoConsoleHandoff,
                ExportFidelity::SourceOnlyMetadata,
                QualificationPosture::FileOnly,
            ),
            TruthLayer::RenderedExpanded => (
                render_context(),
                ConsoleHandoffPosture::NoConsoleHandoff,
                ExportFidelity::DerivedRelationshipMetadata,
                QualificationPosture::FileOnly,
            ),
            TruthLayer::PlannedValidated => (
                planned_context(),
                ConsoleHandoffPosture::ExplicitOptionalBoundary,
                ExportFidelity::DerivedRelationshipMetadata,
                QualificationPosture::InspectOnly,
            ),
            TruthLayer::ObservedLive => (
                live_context(),
                ConsoleHandoffPosture::ExplicitMutationBoundary,
                ExportFidelity::TargetScopedSnapshot,
                QualificationPosture::InspectOnly,
            ),
            TruthLayer::ProviderOverlay => (
                overlay_context(),
                ConsoleHandoffPosture::OverlayOnlyBoundary,
                ExportFidelity::HandoffReferenceOnly,
                QualificationPosture::HandoffOnly,
            ),
        };

    TruthLayerBinding {
        truth_layer: layer,
        target_context_requirement,
        live_access_prerequisite: layer.expected_prerequisite(),
        console_handoff_posture,
        export_fidelity,
        file_intelligence_posture: file_posture,
        summary: format!("{} row.", format!("{layer:?}").to_lowercase()),
    }
}

fn edge(edge_class: RelationEdgeClass) -> RelationEdgeBinding {
    let (source_truth_layers, target_truth_layers) = match edge_class {
        RelationEdgeClass::SourceOfRender => (
            vec![TruthLayer::AuthoredDesired],
            vec![TruthLayer::RenderedExpanded],
        ),
        RelationEdgeClass::PlanFor => (
            vec![TruthLayer::AuthoredDesired, TruthLayer::RenderedExpanded],
            vec![TruthLayer::PlannedValidated],
        ),
        RelationEdgeClass::LiveCounterpartOf => (
            vec![TruthLayer::RenderedExpanded, TruthLayer::PlannedValidated],
            vec![TruthLayer::ObservedLive],
        ),
        RelationEdgeClass::AppliedBy => (
            vec![TruthLayer::ObservedLive],
            vec![TruthLayer::ProviderOverlay],
        ),
        RelationEdgeClass::OwnedBy => (
            vec![TruthLayer::ObservedLive],
            vec![TruthLayer::ObservedLive],
        ),
        RelationEdgeClass::Impacts => (
            vec![TruthLayer::PlannedValidated, TruthLayer::ObservedLive],
            vec![TruthLayer::ObservedLive, TruthLayer::ProviderOverlay],
        ),
        RelationEdgeClass::RunbookReference => (
            vec![TruthLayer::AuthoredDesired, TruthLayer::ObservedLive],
            vec![TruthLayer::ProviderOverlay],
        ),
        RelationEdgeClass::ReviewAnchor => (
            vec![TruthLayer::AuthoredDesired, TruthLayer::PlannedValidated],
            vec![TruthLayer::ProviderOverlay],
        ),
        RelationEdgeClass::ProviderOverlayOf => (
            vec![TruthLayer::ProviderOverlay],
            vec![TruthLayer::ObservedLive, TruthLayer::PlannedValidated],
        ),
    };

    RelationEdgeBinding {
        edge_class,
        source_truth_layers,
        target_truth_layers,
        summary: format!("{} edge.", format!("{edge_class:?}").to_lowercase()),
    }
}

fn downgrade_profile(posture: QualificationPosture) -> DowngradeProfile {
    let (preserved_truth_layers, preserved_relation_edges, export_fidelity) = match posture {
        QualificationPosture::FileOnly => (
            vec![TruthLayer::AuthoredDesired, TruthLayer::RenderedExpanded],
            vec![
                RelationEdgeClass::SourceOfRender,
                RelationEdgeClass::ReviewAnchor,
            ],
            ExportFidelity::SourceOnlyMetadata,
        ),
        QualificationPosture::InspectOnly => (
            vec![
                TruthLayer::AuthoredDesired,
                TruthLayer::RenderedExpanded,
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
            ],
            vec![
                RelationEdgeClass::SourceOfRender,
                RelationEdgeClass::PlanFor,
                RelationEdgeClass::LiveCounterpartOf,
                RelationEdgeClass::Impacts,
                RelationEdgeClass::ReviewAnchor,
            ],
            ExportFidelity::TargetScopedSnapshot,
        ),
        QualificationPosture::HandoffOnly => (
            vec![TruthLayer::AuthoredDesired, TruthLayer::ProviderOverlay],
            vec![
                RelationEdgeClass::RunbookReference,
                RelationEdgeClass::ReviewAnchor,
                RelationEdgeClass::ProviderOverlayOf,
            ],
            ExportFidelity::HandoffReferenceOnly,
        ),
        _ => unreachable!("tests only construct required profiles"),
    };

    DowngradeProfile {
        posture,
        preserved_truth_layers,
        preserved_relation_edges,
        export_fidelity,
        summary: format!("{} downgrade.", format!("{posture:?}").to_lowercase()),
    }
}

fn row(
    family: InfrastructureFamily,
    claimed_artifact_families: &[&str],
    relation_edges: &[RelationEdgeClass],
) -> InfrastructureFamilyMatrixRow {
    InfrastructureFamilyMatrixRow {
        family,
        claimed_artifact_families: claimed_artifact_families
            .iter()
            .map(|entry| (*entry).to_string())
            .collect(),
        truth_layer_bindings: REQUIRED_TRUTH_LAYERS.iter().copied().map(binding).collect(),
        relation_edges: relation_edges.iter().copied().map(edge).collect(),
        downgrade_profiles: REQUIRED_DOWNGRADE_PROFILES
            .iter()
            .copied()
            .map(downgrade_profile)
            .collect(),
        support_summary: format!("{family:?} row.").to_lowercase(),
    }
}

fn packet() -> SourceIntelligenceRelationshipMatrixPacket {
    SourceIntelligenceRelationshipMatrixPacket {
        record_kind: SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND.to_string(),
        schema_version: SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION,
        packet_id: "infra-source-intelligence:m5".to_string(),
        captured_at: "2026-06-12T18:40:00Z".to_string(),
        matrix_rows: vec![
            row(
                InfrastructureFamily::TerraformHcl,
                &["*.tf", "*.tfvars", "terraform plan json"],
                &TERRAFORM_REQUIRED_EDGES,
            ),
            row(
                InfrastructureFamily::KubernetesHelm,
                &[
                    "kubernetes yaml",
                    "helm values",
                    "rendered manifest snapshot",
                ],
                &KUBERNETES_REQUIRED_EDGES,
            ),
            row(
                InfrastructureFamily::Devcontainer,
                &[
                    "devcontainer.json",
                    "compose expansion",
                    "workspace image metadata",
                ],
                &DEVCONTAINER_REQUIRED_EDGES,
            ),
            row(
                InfrastructureFamily::CiEnvironment,
                &[
                    "workflow yaml",
                    "environment template",
                    "deployment descriptor",
                ],
                &CI_REQUIRED_EDGES,
            ),
            row(
                InfrastructureFamily::PolicyManifest,
                &["rego policy", "rbac yaml", "admission manifest"],
                &POLICY_REQUIRED_EDGES,
            ),
        ],
        support_summary: "Stable M5 infra source-intelligence matrix.".to_string(),
    }
}

#[test]
fn seeded_object_packet_validates() {
    let packet = seeded_source_intelligence_object_packet();
    let report = packet.validate();
    assert!(report.passed, "object packet must pass: {:#?}", report.findings);
}

#[test]
fn seeded_object_packet_covers_required_consumer_surfaces() {
    let report = seeded_source_intelligence_object_packet().validate();
    for required in [
        InfrastructureConsumerSurface::Graph,
        InfrastructureConsumerSurface::Review,
        InfrastructureConsumerSurface::Docs,
        InfrastructureConsumerSurface::Incident,
    ] {
        assert!(report.consumer_surfaces.contains(&required));
    }
}

#[test]
fn seeded_object_packet_resolves_projection_refs() {
    let packet = seeded_source_intelligence_object_packet();
    let projection = packet
        .consumer_projection(InfrastructureConsumerSurface::Incident)
        .expect("incident projection exists");
    assert!(projection
        .object_refs
        .iter()
        .all(|object_ref| packet.object(object_ref).is_some()));
    assert!(projection
        .relation_refs
        .iter()
        .all(|relation_ref| packet.relation(relation_ref).is_some()));
}

#[test]
fn valid_packet_passes() {
    let report = packet().validate();
    assert!(report.passed, "expected pass: {:#?}", report.findings);
    assert_eq!(report.families.len(), 5);
    assert_eq!(report.truth_layers.len(), 5);
    assert_eq!(report.relation_edges.len(), 9);
}

#[test]
fn missing_family_is_rejected() {
    let mut pkt = packet();
    pkt.matrix_rows.pop();
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "family_coverage"));
}

#[test]
fn provider_overlay_requires_overlay_boundary() {
    let mut pkt = packet();
    let overlay_binding = pkt.matrix_rows[0]
        .truth_layer_bindings
        .iter_mut()
        .find(|binding| binding.truth_layer == TruthLayer::ProviderOverlay)
        .expect("provider-overlay row");
    overlay_binding.console_handoff_posture = ConsoleHandoffPosture::ExplicitOptionalBoundary;

    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "provider_overlay_handoff"));
}

#[test]
fn file_only_profile_cannot_preserve_live_truth() {
    let mut pkt = packet();
    let file_only = pkt.matrix_rows[1]
        .downgrade_profiles
        .iter_mut()
        .find(|profile| matches!(profile.posture, QualificationPosture::FileOnly))
        .expect("file-only profile");
    file_only
        .preserved_truth_layers
        .push(TruthLayer::ObservedLive);

    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "file_only_live_truth"));
}
