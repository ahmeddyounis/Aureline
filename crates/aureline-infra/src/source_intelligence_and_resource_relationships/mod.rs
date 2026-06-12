//! Canonical M5 infrastructure source-intelligence and relationship matrix.
//!
//! This packet freezes the family, truth-layer, relation-edge, target-context,
//! live-access, console-handoff, export-fidelity, and downgrade vocabulary
//! used by infrastructure-aware surfaces. It keeps Terraform/HCL,
//! Kubernetes/Helm, devcontainer, CI/environment, and policy-manifest lanes on
//! one shared matrix so later M5 surfaces do not invent their own implicit
//! infra semantics.
//!
//! The packet extends the existing target-context and control-plane-boundary
//! model by reusing [`QualificationPosture`] for file-only, inspect-only, and
//! handoff-only downgrade profiles.

pub mod flows;
pub mod object_packet;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_context_and_control_plane_boundary::{
    InfraBoundaryFinding, InfraBoundaryFindingSeverity, QualificationPosture,
};

pub use flows::{
    InfrastructureEnvironmentSliceExplanation, InfrastructureJourneyKind,
    InfrastructureJourneyStatus, InfrastructureJourneySurface, InfrastructureRelationJourney,
    InfrastructureSurfaceView,
};
pub use object_packet::{
    seeded_source_intelligence_object_packet, validate_object_packet,
    InfrastructureConsumerProjection, InfrastructureConsumerSurface, InfrastructureObjectIdentity,
    InfrastructureObjectLineage, InfrastructureObjectRecord, InfrastructureObjectRelationRecord,
    SourceIntelligenceObjectPacket, SourceIntelligenceObjectPacketValidationReport,
    SOURCE_INTELLIGENCE_OBJECT_FIXTURE_DIR, SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    SOURCE_INTELLIGENCE_OBJECT_SCHEMA_REF, SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION,
};

/// Schema version for source-intelligence matrix packets.
pub const SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`SourceIntelligenceRelationshipMatrixPacket`].
pub const SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND: &str =
    "infra_source_intelligence_and_resource_relationship_matrix_packet";

/// JSON Schema reference for packet interchange.
pub const SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF: &str =
    "schemas/infra/source-intelligence-and-resource-relationships.schema.json";

/// Reviewer-facing documentation reference.
pub const SOURCE_INTELLIGENCE_RELATIONSHIP_DOC_REF: &str =
    "docs/infra/source-intelligence-and-resource-relationships.md";

/// Fixture corpus directory for matrix qualification and downgrade drills.
pub const SOURCE_INTELLIGENCE_RELATIONSHIP_FIXTURE_DIR: &str =
    "fixtures/infra/source-intelligence-and-resource-relationships";

/// Claimed infrastructure family covered by the M5 matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureFamily {
    /// Terraform modules, HCL files, plan outputs, and provider bindings.
    TerraformHcl,
    /// Kubernetes manifests, Helm values, rendered objects, and live workloads.
    KubernetesHelm,
    /// Devcontainer definitions and the container descriptors they resolve through.
    Devcontainer,
    /// CI workflow, environment, and rollout descriptors.
    CiEnvironment,
    /// Policy, admission, RBAC, and other enforcement-oriented manifests.
    PolicyManifest,
}

/// Distinct truth layer that must remain visible across infrastructure surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthLayer {
    /// Repo-authored desired state.
    AuthoredDesired,
    /// Rendered, expanded, or compiled output derived from source.
    RenderedExpanded,
    /// Planned, validated, or dry-run output.
    PlannedValidated,
    /// Observed or live runtime state.
    ObservedLive,
    /// Provider-owned metadata or console-only context.
    ProviderOverlay,
}

impl TruthLayer {
    const fn expected_prerequisite(self) -> LiveAccessPrerequisite {
        match self {
            Self::AuthoredDesired => LiveAccessPrerequisite::NoLiveAccessRequired,
            Self::RenderedExpanded => LiveAccessPrerequisite::RenderToolResolved,
            Self::PlannedValidated => LiveAccessPrerequisite::PlanOrValidatorResolved,
            Self::ObservedLive => LiveAccessPrerequisite::ScopedReadOnlyLiveAccess,
            Self::ProviderOverlay => LiveAccessPrerequisite::ProviderOverlaySession,
        }
    }
}

/// Stable relationship edge class reused by graph, review, docs, and incident surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationEdgeClass {
    /// Authored source produces a rendered or expanded object.
    SourceOfRender,
    /// A plan, diff, or validation result applies to a target object.
    PlanFor,
    /// A source, rendered object, or plan points at one live counterpart.
    LiveCounterpartOf,
    /// A concrete run or controller applied or reconciled a resource.
    AppliedBy,
    /// A resource owns or controls another resource or surface.
    OwnedBy,
    /// A change or object impacts another runtime slice.
    Impacts,
    /// A resource or policy points at a runbook or operational guide.
    RunbookReference,
    /// A source or live object anchors review, change, or approval surfaces.
    ReviewAnchor,
    /// A provider overlay row enriches one repo-owned or live object.
    ProviderOverlayOf,
}

/// Target-context field that a truth layer must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetContextField {
    /// Provider family.
    Provider,
    /// Account, subscription, or project identity.
    AccountSubscriptionProject,
    /// Environment, workspace, cluster, or rollout selector.
    EnvironmentSelector,
    /// Namespace, service, container, or policy scope.
    NamespaceOrScope,
    /// Region or zone identity.
    RegionZone,
    /// Tenant or organization identity.
    Tenant,
    /// Workspace root plus branch, worktree, or commit scope.
    WorkspaceRevision,
    /// Execution origin such as local CLI, remote agent, or managed worker.
    ExecutionOrigin,
    /// Resolved execution-context profile.
    ExecutionContextProfile,
    /// Tool or renderer identity and version.
    ToolIdentity,
    /// Credential-handle or delegated-authority class.
    CredentialClass,
    /// Observation or refresh timestamp.
    FreshnessTimestamp,
}

/// How much target context a truth layer must resolve before it can be trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetContextRequirementClass {
    /// Repo-scoped source context only.
    RepoScoped,
    /// Repo plus renderer/tool identity.
    RenderScoped,
    /// Exact target preview and tool identity resolved.
    PlannedTargetResolved,
    /// Live target identity, freshness, and authority resolved.
    LiveTargetResolved,
    /// Provider-overlay destination and freshness resolved.
    OverlayTargetResolved,
}

/// Target-context contract attached to one truth layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetContextRequirement {
    /// Resolution class required for the truth layer.
    pub requirement_class: TargetContextRequirementClass,
    /// Required context fields that must stay stable across surfaces.
    pub required_fields: Vec<TargetContextField>,
    /// True when search, review, AI, docs, and exports share the same context shape.
    pub stable_across_surfaces: bool,
    /// True when ambient shell or provider inheritance is forbidden.
    pub ambient_context_forbidden: bool,
}

/// Minimum prerequisite that admits one truth layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveAccessPrerequisite {
    /// No live connector is required.
    NoLiveAccessRequired,
    /// Render or expansion tooling must be resolved.
    RenderToolResolved,
    /// A plan, diff, validator, or simulator must be resolved.
    PlanOrValidatorResolved,
    /// A scoped live connector or agent must be available.
    ScopedReadOnlyLiveAccess,
    /// A provider overlay session or browser/API enrichment must be available.
    ProviderOverlaySession,
}

/// Console-handoff posture attached to one truth layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleHandoffPosture {
    /// No console handoff is part of this truth layer.
    NoConsoleHandoff,
    /// Handoff is explicit but optional.
    ExplicitOptionalBoundary,
    /// Handoff is the explicit mutation boundary out of Aureline.
    ExplicitMutationBoundary,
    /// This layer exists only as an explicit overlay or handoff destination.
    OverlayOnlyBoundary,
}

/// Export fidelity admitted for one truth layer or downgrade profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFidelity {
    /// Export includes source-safe metadata only.
    SourceOnlyMetadata,
    /// Export includes relationship edges and derived metadata, but no live payloads.
    DerivedRelationshipMetadata,
    /// Export includes target-scoped snapshots with freshness and authority labels.
    TargetScopedSnapshot,
    /// Export includes handoff refs and overlay lineage only.
    HandoffReferenceOnly,
}

/// One truth-layer row for one infrastructure family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthLayerBinding {
    /// Truth layer described by this row.
    pub truth_layer: TruthLayer,
    /// Target-context contract the row requires.
    pub target_context_requirement: TargetContextRequirement,
    /// Minimum prerequisite for showing this layer honestly.
    pub live_access_prerequisite: LiveAccessPrerequisite,
    /// Console-handoff posture for this layer.
    pub console_handoff_posture: ConsoleHandoffPosture,
    /// Export fidelity admitted for this layer.
    pub export_fidelity: ExportFidelity,
    /// Downgrade posture when only file intelligence exists.
    pub file_intelligence_posture: QualificationPosture,
    /// Short explanation of the layer contract.
    pub summary: String,
}

/// One stable relation-edge binding reused across consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationEdgeBinding {
    /// Stable edge class.
    pub edge_class: RelationEdgeClass,
    /// Truth layers the edge may originate from.
    pub source_truth_layers: Vec<TruthLayer>,
    /// Truth layers the edge may point at.
    pub target_truth_layers: Vec<TruthLayer>,
    /// Short explanation of the edge.
    pub summary: String,
}

/// Behavior of one downgrade profile for one infrastructure family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeProfile {
    /// Downgrade posture reused from the infra target-context contract.
    pub posture: QualificationPosture,
    /// Truth layers still visible under this posture.
    pub preserved_truth_layers: Vec<TruthLayer>,
    /// Relation-edge classes still visible under this posture.
    pub preserved_relation_edges: Vec<RelationEdgeClass>,
    /// Export fidelity admitted under this posture.
    pub export_fidelity: ExportFidelity,
    /// Short explanation of the downgrade behavior.
    pub summary: String,
}

/// One family row in the canonical M5 infrastructure matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureFamilyMatrixRow {
    /// Family covered by the row.
    pub family: InfrastructureFamily,
    /// Authored or derived artifact families this row claims.
    pub claimed_artifact_families: Vec<String>,
    /// Truth-layer rows for the family.
    pub truth_layer_bindings: Vec<TruthLayerBinding>,
    /// Stable relation edges the family supports.
    pub relation_edges: Vec<RelationEdgeBinding>,
    /// Explicit file-only, inspect-only, and handoff-only profiles.
    pub downgrade_profiles: Vec<DowngradeProfile>,
    /// Export-safe row summary.
    pub support_summary: String,
}

/// Canonical M5 packet for infrastructure source-intelligence and relationships.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceIntelligenceRelationshipMatrixPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Claimed family rows in the matrix.
    pub matrix_rows: Vec<InfrastructureFamilyMatrixRow>,
    /// Export-safe overall support summary.
    pub support_summary: String,
}

impl SourceIntelligenceRelationshipMatrixPacket {
    /// Validates the packet against the canonical M5 matrix invariants.
    pub fn validate(&self) -> SourceIntelligenceRelationshipMatrixValidationReport {
        validate_packet(self)
    }
}

/// Validates one source-intelligence and relationship matrix packet.
pub fn validate_packet(
    packet: &SourceIntelligenceRelationshipMatrixPacket,
) -> SourceIntelligenceRelationshipMatrixValidationReport {
    let mut findings = Vec::new();
    let mut families = BTreeSet::new();
    let mut truth_layers = BTreeSet::new();
    let mut relation_edges = BTreeSet::new();

    if packet.record_kind != SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the infra source-intelligence matrix discriminator.",
        ));
    }
    if packet.schema_version != SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }

    for row in &packet.matrix_rows {
        if !families.insert(row.family) {
            findings.push(error(
                "duplicate_family",
                "Matrix packet contains the same infrastructure family more than once.",
            ));
        }
        if row.claimed_artifact_families.is_empty()
            || row
                .claimed_artifact_families
                .iter()
                .any(|family| family.trim().is_empty())
        {
            findings.push(error(
                "claimed_artifacts",
                "Family row is missing claimed artifact families.",
            ));
        }

        let mut row_truth_layers = BTreeSet::new();
        for binding in &row.truth_layer_bindings {
            truth_layers.insert(binding.truth_layer);
            if !row_truth_layers.insert(binding.truth_layer) {
                findings.push(error(
                    "duplicate_truth_layer",
                    "Family row repeats a truth-layer binding.",
                ));
            }
            validate_truth_layer_binding(row, binding, &mut findings);
        }
        for required in REQUIRED_TRUTH_LAYERS {
            if !row_truth_layers.contains(&required) {
                findings.push(error(
                    "truth_layer_coverage",
                    "Family row is missing a required truth layer.",
                ));
            }
        }

        let mut row_edges = BTreeSet::new();
        for edge in &row.relation_edges {
            relation_edges.insert(edge.edge_class);
            if !row_edges.insert(edge.edge_class) {
                findings.push(error(
                    "duplicate_relation_edge",
                    "Family row repeats a relation-edge class.",
                ));
            }
            if edge.source_truth_layers.is_empty() || edge.target_truth_layers.is_empty() {
                findings.push(error(
                    "relation_edge_layers",
                    "Relation-edge binding is missing source or target truth layers.",
                ));
            }
            if edge.summary.trim().is_empty() {
                findings.push(error(
                    "relation_edge_summary",
                    "Relation-edge binding is missing a summary.",
                ));
            }
        }
        for required in required_edges_for(row.family) {
            if !row_edges.contains(&required) {
                findings.push(error(
                    "relation_edge_coverage",
                    "Family row is missing a required relation-edge class.",
                ));
            }
        }

        let mut row_profiles = Vec::new();
        for profile in &row.downgrade_profiles {
            if row_profiles.contains(&profile.posture) {
                findings.push(error(
                    "duplicate_downgrade_profile",
                    "Family row repeats a downgrade profile.",
                ));
            }
            row_profiles.push(profile.posture);
            if !is_required_profile(profile.posture) {
                findings.push(error(
                    "unexpected_downgrade_profile",
                    "Family row defines a profile outside file-only, inspect-only, or handoff-only.",
                ));
            }
            if profile.preserved_truth_layers.is_empty()
                || profile.preserved_relation_edges.is_empty()
                || profile.summary.trim().is_empty()
            {
                findings.push(error(
                    "downgrade_profile_shape",
                    "Downgrade profile is missing preserved truth, edges, or summary.",
                ));
            }
            if matches!(profile.posture, QualificationPosture::FileOnly)
                && profile.preserved_truth_layers.iter().any(|layer| {
                    matches!(
                        layer,
                        TruthLayer::ObservedLive | TruthLayer::ProviderOverlay
                    )
                })
            {
                findings.push(error(
                    "file_only_live_truth",
                    "File-only profile preserves live or provider-overlay truth.",
                ));
            }
            if matches!(profile.posture, QualificationPosture::HandoffOnly)
                && !profile
                    .preserved_truth_layers
                    .contains(&TruthLayer::ProviderOverlay)
            {
                findings.push(error(
                    "handoff_only_overlay",
                    "Handoff-only profile does not preserve provider-overlay truth.",
                ));
            }
        }
        for required in REQUIRED_DOWNGRADE_PROFILES {
            if !row_profiles.contains(&required) {
                findings.push(error(
                    "downgrade_profile_coverage",
                    "Family row is missing a required downgrade profile.",
                ));
            }
        }

        for binding in &row.truth_layer_bindings {
            if !row_profiles.contains(&binding.file_intelligence_posture) {
                findings.push(error(
                    "file_intelligence_profile",
                    "Truth-layer binding points at a missing downgrade profile.",
                ));
            }
        }
    }

    for required in REQUIRED_FAMILIES {
        if !families.contains(&required) {
            findings.push(error(
                "family_coverage",
                "Packet is missing a required infrastructure family.",
            ));
        }
    }
    for required in REQUIRED_TRUTH_LAYERS {
        if !truth_layers.contains(&required) {
            findings.push(error(
                "global_truth_layer_coverage",
                "Packet does not define the full shared truth-layer vocabulary.",
            ));
        }
    }
    for required in REQUIRED_RELATION_EDGES {
        if !relation_edges.contains(&required) {
            findings.push(error(
                "global_relation_edge_coverage",
                "Packet does not define the full shared relation-edge vocabulary.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);
    SourceIntelligenceRelationshipMatrixValidationReport {
        record_kind: "infra_source_intelligence_and_resource_relationship_matrix_validation_report"
            .to_string(),
        schema_version: SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        families,
        truth_layers,
        relation_edges,
        findings,
    }
}

fn validate_truth_layer_binding(
    row: &InfrastructureFamilyMatrixRow,
    binding: &TruthLayerBinding,
    findings: &mut Vec<InfraBoundaryFinding>,
) {
    let fields = &binding.target_context_requirement.required_fields;
    if fields.is_empty() {
        findings.push(error(
            "target_context_fields",
            "Truth-layer binding is missing required target-context fields.",
        ));
    }
    if !binding.target_context_requirement.stable_across_surfaces {
        findings.push(error(
            "target_context_stability",
            "Truth-layer binding does not keep target context stable across surfaces.",
        ));
    }
    if !binding.target_context_requirement.ambient_context_forbidden {
        findings.push(error(
            "ambient_context_forbidden",
            "Truth-layer binding allows ambient context inheritance.",
        ));
    }
    if !fields.contains(&TargetContextField::Provider)
        || !fields.contains(&TargetContextField::ExecutionOrigin)
        || !fields.contains(&TargetContextField::WorkspaceRevision)
    {
        findings.push(error(
            "target_context_minimums",
            "Truth-layer binding is missing provider, execution-origin, or workspace-revision context.",
        ));
    }
    if matches!(
        binding.target_context_requirement.requirement_class,
        TargetContextRequirementClass::PlannedTargetResolved
            | TargetContextRequirementClass::LiveTargetResolved
            | TargetContextRequirementClass::OverlayTargetResolved
    ) && (!fields.contains(&TargetContextField::ExecutionContextProfile)
        || !fields.contains(&TargetContextField::ToolIdentity))
    {
        findings.push(error(
            "target_context_execution",
            "Planned, live, or overlay truth is missing execution-profile or tool identity.",
        ));
    }
    if matches!(
        binding.target_context_requirement.requirement_class,
        TargetContextRequirementClass::LiveTargetResolved
            | TargetContextRequirementClass::OverlayTargetResolved
    ) && (!fields.contains(&TargetContextField::AccountSubscriptionProject)
        || !fields.contains(&TargetContextField::FreshnessTimestamp))
    {
        findings.push(error(
            "target_context_live_fields",
            "Live or overlay truth is missing account/project or freshness context.",
        ));
    }
    if binding.live_access_prerequisite != binding.truth_layer.expected_prerequisite() {
        findings.push(error(
            "truth_layer_prerequisite",
            "Truth-layer binding does not use the expected live-access prerequisite.",
        ));
    }
    if matches!(binding.truth_layer, TruthLayer::ProviderOverlay)
        && !matches!(
            binding.console_handoff_posture,
            ConsoleHandoffPosture::OverlayOnlyBoundary
        )
    {
        findings.push(error(
            "provider_overlay_handoff",
            "Provider-overlay truth must remain an explicit overlay or handoff boundary.",
        ));
    }
    if matches!(binding.truth_layer, TruthLayer::AuthoredDesired)
        && matches!(
            binding.console_handoff_posture,
            ConsoleHandoffPosture::OverlayOnlyBoundary
        )
    {
        findings.push(error(
            "authored_handoff",
            "Authored truth cannot be represented as overlay-only handoff state.",
        ));
    }
    if !is_required_profile(binding.file_intelligence_posture) {
        findings.push(error(
            "file_intelligence_posture",
            "Truth-layer binding uses an unsupported file-intelligence downgrade posture.",
        ));
    }
    if binding.summary.trim().is_empty() {
        findings.push(error(
            "truth_layer_summary",
            "Truth-layer binding is missing a summary.",
        ));
    }
    if matches!(row.family, InfrastructureFamily::Devcontainer)
        && matches!(binding.truth_layer, TruthLayer::PlannedValidated)
        && matches!(
            binding.export_fidelity,
            ExportFidelity::HandoffReferenceOnly
        )
    {
        findings.push(error(
            "devcontainer_plan_export",
            "Devcontainer plan or validation truth cannot narrow directly to handoff-only export.",
        ));
    }
}

/// Validation report emitted for a source-intelligence matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceIntelligenceRelationshipMatrixValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// True when no error-severity finding was emitted.
    pub passed: bool,
    /// Infrastructure families covered by the packet.
    pub families: BTreeSet<InfrastructureFamily>,
    /// Truth layers covered by the packet.
    pub truth_layers: BTreeSet<TruthLayer>,
    /// Relation-edge classes covered by the packet.
    pub relation_edges: BTreeSet<RelationEdgeClass>,
    /// Findings emitted during validation.
    pub findings: Vec<InfraBoundaryFinding>,
}

const REQUIRED_FAMILIES: [InfrastructureFamily; 5] = [
    InfrastructureFamily::TerraformHcl,
    InfrastructureFamily::KubernetesHelm,
    InfrastructureFamily::Devcontainer,
    InfrastructureFamily::CiEnvironment,
    InfrastructureFamily::PolicyManifest,
];

const REQUIRED_TRUTH_LAYERS: [TruthLayer; 5] = [
    TruthLayer::AuthoredDesired,
    TruthLayer::RenderedExpanded,
    TruthLayer::PlannedValidated,
    TruthLayer::ObservedLive,
    TruthLayer::ProviderOverlay,
];

const REQUIRED_RELATION_EDGES: [RelationEdgeClass; 9] = [
    RelationEdgeClass::SourceOfRender,
    RelationEdgeClass::PlanFor,
    RelationEdgeClass::LiveCounterpartOf,
    RelationEdgeClass::AppliedBy,
    RelationEdgeClass::OwnedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::RunbookReference,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

const REQUIRED_DOWNGRADE_PROFILES: [QualificationPosture; 3] = [
    QualificationPosture::FileOnly,
    QualificationPosture::InspectOnly,
    QualificationPosture::HandoffOnly,
];

const TERRAFORM_REQUIRED_EDGES: [RelationEdgeClass; 7] = [
    RelationEdgeClass::PlanFor,
    RelationEdgeClass::LiveCounterpartOf,
    RelationEdgeClass::AppliedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::RunbookReference,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

const KUBERNETES_REQUIRED_EDGES: [RelationEdgeClass; 8] = [
    RelationEdgeClass::SourceOfRender,
    RelationEdgeClass::PlanFor,
    RelationEdgeClass::LiveCounterpartOf,
    RelationEdgeClass::OwnedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::RunbookReference,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

const DEVCONTAINER_REQUIRED_EDGES: [RelationEdgeClass; 6] = [
    RelationEdgeClass::SourceOfRender,
    RelationEdgeClass::LiveCounterpartOf,
    RelationEdgeClass::OwnedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

const CI_REQUIRED_EDGES: [RelationEdgeClass; 6] = [
    RelationEdgeClass::PlanFor,
    RelationEdgeClass::AppliedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::RunbookReference,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

const POLICY_REQUIRED_EDGES: [RelationEdgeClass; 7] = [
    RelationEdgeClass::SourceOfRender,
    RelationEdgeClass::PlanFor,
    RelationEdgeClass::OwnedBy,
    RelationEdgeClass::Impacts,
    RelationEdgeClass::RunbookReference,
    RelationEdgeClass::ReviewAnchor,
    RelationEdgeClass::ProviderOverlayOf,
];

fn required_edges_for(family: InfrastructureFamily) -> &'static [RelationEdgeClass] {
    match family {
        InfrastructureFamily::TerraformHcl => &TERRAFORM_REQUIRED_EDGES,
        InfrastructureFamily::KubernetesHelm => &KUBERNETES_REQUIRED_EDGES,
        InfrastructureFamily::Devcontainer => &DEVCONTAINER_REQUIRED_EDGES,
        InfrastructureFamily::CiEnvironment => &CI_REQUIRED_EDGES,
        InfrastructureFamily::PolicyManifest => &POLICY_REQUIRED_EDGES,
    }
}

const fn is_required_profile(posture: QualificationPosture) -> bool {
    matches!(
        posture,
        QualificationPosture::FileOnly
            | QualificationPosture::InspectOnly
            | QualificationPosture::HandoffOnly
    )
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests;
