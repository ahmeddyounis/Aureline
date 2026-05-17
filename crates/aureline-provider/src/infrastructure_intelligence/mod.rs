//! Read-only infrastructure source-intelligence packets for claimed IaC and
//! operational connectors.
//!
//! This module owns the alpha contract that lets provider connectors surface
//! topology and relationship intelligence without gaining hidden mutation
//! authority. The packet keeps authored, rendered, planned, observed, cached,
//! and provider-overlay truth distinct; every resource and edge carries source
//! and freshness labels; and search, review, AI-context, and support
//! projections are derived from the same [`InfrastructureIntelligenceAlphaPage`]
//! instead of separate per-surface truth stores.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::registry::{FreshnessLabel, FreshnessTruth, RedactionClass};

/// Alpha schema version exported with infrastructure intelligence records.
pub const INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every infrastructure intelligence record.
pub const INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF: &str =
    "providers:infrastructure_source_intelligence_alpha:v1";

/// Stable record kind for [`InfrastructureIntelligenceAlphaPage`] payloads.
pub const INFRASTRUCTURE_INTELLIGENCE_ALPHA_PAGE_RECORD_KIND: &str =
    "infrastructure_intelligence_alpha_page_record";

/// Stable record kind for [`InfrastructureConnectorRecord`] payloads.
pub const INFRASTRUCTURE_CONNECTOR_RECORD_KIND: &str =
    "infrastructure_intelligence_connector_record";

/// Stable record kind for [`InfrastructureResourceRecord`] payloads.
pub const INFRASTRUCTURE_RESOURCE_RECORD_KIND: &str = "infrastructure_intelligence_resource_record";

/// Stable record kind for [`InfrastructureRelationshipRecord`] payloads.
pub const INFRASTRUCTURE_RELATIONSHIP_RECORD_KIND: &str =
    "infrastructure_intelligence_relationship_record";

/// Stable record kind for [`InfrastructureConsumerProjection`] payloads.
pub const INFRASTRUCTURE_CONSUMER_PROJECTION_RECORD_KIND: &str =
    "infrastructure_intelligence_consumer_projection_record";

/// Stable record kind for [`InfrastructureIntelligenceValidationReport`].
pub const INFRASTRUCTURE_INTELLIGENCE_VALIDATION_REPORT_RECORD_KIND: &str =
    "infrastructure_intelligence_validation_report";

/// Stable record kind for [`InfrastructureSearchProjection`] payloads.
pub const INFRASTRUCTURE_SEARCH_PROJECTION_RECORD_KIND: &str =
    "infrastructure_intelligence_search_projection_record";

/// Stable record kind for [`InfrastructureReviewProjection`] payloads.
pub const INFRASTRUCTURE_REVIEW_PROJECTION_RECORD_KIND: &str =
    "infrastructure_intelligence_review_projection_record";

/// Stable record kind for [`InfrastructureSupportExport`] payloads.
pub const INFRASTRUCTURE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "infrastructure_intelligence_support_export_record";

/// Reviewer-facing documentation for the alpha lane.
pub const INFRASTRUCTURE_INTELLIGENCE_DOC_REF: &str =
    "docs/runtime/m3/infrastructure_source_intelligence_alpha.md";

/// Fixture directory for the alpha lane.
pub const INFRASTRUCTURE_INTELLIGENCE_FIXTURE_DIR: &str =
    "fixtures/providers/m3/infrastructure_connectors";

/// Boundary schema for the alpha lane.
pub const INFRASTRUCTURE_INTELLIGENCE_SCHEMA_REF: &str =
    "schemas/providers/infrastructure_intelligence.schema.json";

/// Connector family surfaced by the read-only infrastructure lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureConnectorKind {
    /// Terraform or HCL workspace connector.
    TerraformWorkspace,
    /// Kubernetes manifest or cluster connector.
    KubernetesCluster,
    /// Container, compose, Docker, or devcontainer connector.
    ContainerRuntime,
    /// CI, deployment, or environment provider connector.
    CiProvider,
    /// Policy, access, admission, or enforcement connector.
    PolicyEngine,
}

impl InfrastructureConnectorKind {
    /// Stable token recorded on alpha records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerraformWorkspace => "terraform_workspace",
            Self::KubernetesCluster => "kubernetes_cluster",
            Self::ContainerRuntime => "container_runtime",
            Self::CiProvider => "ci_provider",
            Self::PolicyEngine => "policy_engine",
        }
    }
}

/// Source class for an infrastructure resource or relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSourceClass {
    /// Terraform or HCL source, state, or plan material.
    TerraformHcl,
    /// Kubernetes YAML, Kustomize, Helm, or rendered manifest material.
    KubernetesManifest,
    /// Container, compose, devcontainer, or OCI descriptor material.
    ContainerDescriptor,
    /// CI workflow, deployment descriptor, environment, or artifact material.
    CiEnvironmentDescriptor,
    /// Policy, access, RBAC, network-policy, or admission material.
    PolicyAccessConfig,
}

impl InfrastructureSourceClass {
    /// Stable token recorded on alpha records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerraformHcl => "terraform_hcl",
            Self::KubernetesManifest => "kubernetes_manifest",
            Self::ContainerDescriptor => "container_descriptor",
            Self::CiEnvironmentDescriptor => "ci_environment_descriptor",
            Self::PolicyAccessConfig => "policy_access_config",
        }
    }
}

/// Truth layer represented by a resource or edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureTruthLayer {
    /// Repo-owned desired source.
    Authored,
    /// Generated or templated render derived from source.
    Rendered,
    /// Planned or simulated state before mutation.
    Planned,
    /// Live or imported observation from a connector.
    Observed,
    /// Vendor-owned provider context that enriches navigation.
    ProviderOverlay,
    /// Cached or mirrored last-known value.
    Cached,
}

impl InfrastructureTruthLayer {
    /// Stable token recorded on alpha records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Rendered => "rendered",
            Self::Planned => "planned",
            Self::Observed => "observed",
            Self::ProviderOverlay => "provider_overlay",
            Self::Cached => "cached",
        }
    }
}

/// Read mode claimed by an infrastructure connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureReadMode {
    /// Connector only derives source relationships from local or imported data.
    ReadOnlySourceIntelligence,
    /// Connector observes live/cached provider state without mutation.
    ReadOnlyProviderOverlay,
    /// Connector only exposes an imported snapshot.
    ImportedSnapshotOnly,
}

impl InfrastructureReadMode {
    /// Stable token recorded on alpha records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlySourceIntelligence => "read_only_source_intelligence",
            Self::ReadOnlyProviderOverlay => "read_only_provider_overlay",
            Self::ImportedSnapshotOnly => "imported_snapshot_only",
        }
    }
}

/// Control-plane boundary disclosed by the connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureControlPlaneBoundary {
    /// No mutation authority is available in-product.
    ReadOnlyNoMutationAuthority,
    /// Compare, plan, or validate only; mutation remains closed.
    CompareOnlyPreview,
    /// Mutating detail requires a first-class external handoff.
    ExternalHandoffRequired,
    /// In-product mutation was claimed; invalid for this alpha lane.
    InProductMutationClaimed,
}

impl InfrastructureControlPlaneBoundary {
    /// True when the boundary stays inside the alpha read-only contract.
    pub const fn is_read_only_alpha(self) -> bool {
        !matches!(self, Self::InProductMutationClaimed)
    }
}

/// Relationship kind extracted from infrastructure source intelligence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureRelationshipKind {
    /// Terraform module-to-module dependency.
    ModuleToModule,
    /// Terraform variable/output edge.
    VariableOutput,
    /// Resource-to-provider binding.
    ResourceToProvider,
    /// Plan-to-resource edge.
    PlanToResource,
    /// Authored source to rendered object edge.
    SourceToRenderedObject,
    /// Rendered or planned object to live resource edge.
    ObjectToLiveResource,
    /// Runtime object to log or event stream edge.
    ObjectToLogOrEventStream,
    /// Service-to-port or endpoint edge.
    ServiceToPort,
    /// Pipeline or run to artifact edge.
    RunToArtifact,
    /// Policy to target resource edge.
    PolicyToTargetResource,
    /// Policy to observed enforcement result edge.
    PolicyToEnforcementResult,
    /// Resource to runbook or operational note edge.
    ResourceToRunbook,
}

/// Confidence class for one resource relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureConfidenceClass {
    /// Exact verified relationship.
    Verified,
    /// Direct relationship from a parser or connector.
    Direct,
    /// Transitive relationship derived from known edges.
    Transitive,
    /// Inferred relationship that must remain visually weaker.
    Inferred,
    /// Unknown confidence.
    Unknown,
}

/// Partiality class for indexed or retrieved infrastructure state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructurePartialityClass {
    /// Complete enough for the claimed row.
    Complete,
    /// Partial because the index is warming or narrowed.
    PartialIndex,
    /// Partial because retrieval omitted known scope.
    PartialRetrieval,
    /// Partial because permissions narrowed the visible scope.
    PermissionLimited,
    /// Unavailable for this connector or row.
    Unavailable,
}

impl InfrastructurePartialityClass {
    /// True when the class needs an explicit user-visible reason.
    pub const fn requires_reason(self) -> bool {
        !matches!(self, Self::Complete)
    }
}

/// Consumer surface derived from the provider-owned relationship packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureConsumerSurface {
    /// Search result, search panel, quick-open, or docs-search lane.
    Search,
    /// Review workspace, review packet, or diff evidence lane.
    Review,
    /// Support export, Project Doctor, or support-center lane.
    Support,
    /// AI context assembly or evidence lane.
    AiContext,
}

/// Promotion gate state for operator-truth alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructurePromotionState {
    /// Docs, support, and UI truth agree and validation passed.
    PromotionReady,
    /// Docs, support, or UI truth drift blocks promotion.
    BlockedTruthDrift,
    /// Validation defects block promotion.
    BlockedValidationDefects,
}

/// Opaque target context for connector observations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureTargetContext {
    /// Provider or platform reference.
    pub provider_ref: String,
    /// Account, project, org, or tenant reference.
    pub account_or_project_ref: String,
    /// Environment reference such as local, staging, or production.
    pub environment_ref: String,
    /// Cluster reference when the connector observes cluster state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_ref: Option<String>,
    /// Namespace or scope reference when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_ref: Option<String>,
    /// Region reference when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_ref: Option<String>,
    /// Redaction-safe connector class label.
    pub connector_class_label: String,
}

/// Resource identity safe for search, review, and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureResourceIdentity {
    /// Identity class, such as `terraform_resource` or `kubernetes_object`.
    pub identity_class: String,
    /// Resource kind label.
    pub resource_kind: String,
    /// Opaque resource name reference.
    pub resource_name_ref: String,
    /// Namespace, project, or account scope reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_or_project_ref: Option<String>,
    /// Address, selector, or endpoint reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_or_selector_ref: Option<String>,
}

/// References to upstream contracts consumed by the alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceContractRefs {
    /// Connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
    /// Provider-object schema reference.
    pub provider_object_schema_ref: String,
    /// Target-context schema reference.
    pub target_context_schema_ref: String,
    /// Search result identity schema reference.
    pub search_result_identity_schema_ref: String,
    /// Review workspace schema reference.
    pub review_workspace_schema_ref: String,
    /// Support bundle schema reference.
    pub support_bundle_schema_ref: String,
    /// Content-integrity schema reference.
    pub content_integrity_schema_ref: String,
    /// AI context evidence schema reference.
    pub ai_context_schema_ref: String,
}

impl InfrastructureIntelligenceContractRefs {
    fn all_refs(&self) -> [&str; 8] {
        [
            &self.connected_provider_registry_schema_ref,
            &self.provider_object_schema_ref,
            &self.target_context_schema_ref,
            &self.search_result_identity_schema_ref,
            &self.review_workspace_schema_ref,
            &self.support_bundle_schema_ref,
            &self.content_integrity_schema_ref,
            &self.ai_context_schema_ref,
        ]
    }
}

/// Fixture metadata used by protected infrastructure connector cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
    /// Closed axes the fixture intends to exercise.
    #[serde(default)]
    pub exercised_axes: BTreeMap<String, Vec<String>>,
}

/// Read-only connector record for one claimed infrastructure source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureConnectorRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable connector id.
    pub connector_id: String,
    /// Connector family.
    pub connector_kind: InfrastructureConnectorKind,
    /// Redaction-safe connector label.
    pub connector_label: String,
    /// Provider descriptor this connector derives from.
    pub provider_descriptor_ref: String,
    /// Target context disclosed by the connector.
    pub target_context: InfrastructureTargetContext,
    /// Primary source class read by the connector.
    pub source_class: InfrastructureSourceClass,
    /// Freshness truth for the connector observation.
    pub freshness: FreshnessTruth,
    /// Read mode claimed by the connector.
    pub read_mode: InfrastructureReadMode,
    /// Control-plane boundary disclosed before any handoff.
    pub control_plane_boundary: InfrastructureControlPlaneBoundary,
    /// Guardrail: the connector only claims source intelligence.
    pub read_only_source_intelligence: bool,
    /// Guardrail: mutation authority is excluded from this alpha packet.
    pub mutation_authority_excluded: bool,
    /// Guardrail: AI may not write through this connector.
    pub hidden_ai_writes_allowed: bool,
    /// Guardrail: provider writes may not happen behind the packet.
    pub hidden_provider_writes_allowed: bool,
    /// Export-safe connector summary.
    pub support_summary: String,
}

/// Resource node extracted from infrastructure source intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureResourceRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable resource id.
    pub resource_id: String,
    /// Connector this resource was derived from.
    pub connector_ref: String,
    /// Redaction-safe display label.
    pub display_label: String,
    /// Source artifact reference.
    pub source_artifact_ref: String,
    /// Resource identity safe for export.
    pub resource_identity: InfrastructureResourceIdentity,
    /// Source class for the row.
    pub source_class: InfrastructureSourceClass,
    /// Truth layer for the row.
    pub truth_layer: InfrastructureTruthLayer,
    /// Freshness truth for the row.
    pub freshness: FreshnessTruth,
    /// Partiality class for indexed or retrieved data.
    pub partiality: InfrastructurePartialityClass,
    /// Required reason when partiality is not complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partiality_reason: Option<String>,
    /// Guardrail: provider overlays did not replace repo truth.
    pub provider_overlay_replaces_repo_truth: bool,
    /// Guardrail: this row carries no active mutation authority.
    pub active_control_plane_mutation_authority: bool,
    /// Provenance refs used to derive the row.
    pub provenance_refs: Vec<String>,
    /// Export-safe row summary.
    pub support_summary: String,
}

/// Relationship edge extracted from infrastructure source intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureRelationshipRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable relationship id.
    pub relationship_id: String,
    /// Source resource id.
    pub from_resource_ref: String,
    /// Target resource id.
    pub to_resource_ref: String,
    /// Relationship kind.
    pub relationship_kind: InfrastructureRelationshipKind,
    /// Truth layer for this edge.
    pub truth_layer: InfrastructureTruthLayer,
    /// Source class for this edge.
    pub relationship_source: InfrastructureSourceClass,
    /// Freshness truth for the edge.
    pub freshness: FreshnessTruth,
    /// Confidence class for this edge.
    pub confidence_class: InfrastructureConfidenceClass,
    /// Partiality class for indexed or retrieved relationship data.
    pub partiality: InfrastructurePartialityClass,
    /// Required reason when partiality is not complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partiality_reason: Option<String>,
    /// Provenance refs used to derive the edge.
    pub provenance_refs: Vec<String>,
    /// Export-safe relationship summary.
    pub support_summary: String,
}

/// Consumer projection proving a surface uses the shared packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureConsumerProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable projection id.
    pub projection_id: String,
    /// Consumer surface.
    pub surface: InfrastructureConsumerSurface,
    /// Source page id being projected.
    pub source_packet_ref: String,
    /// Relationship ids consumed by the surface.
    pub relationship_refs: Vec<String>,
    /// Resource ids consumed by the surface.
    pub resource_refs: Vec<String>,
    /// Freshness truth for this projection.
    pub freshness: FreshnessTruth,
    /// Guardrail: projection consumes the same relationship packet.
    pub uses_same_relationship_packet: bool,
    /// Guardrail: projection did not mint a parallel truth store.
    pub parallel_truth_store_created: bool,
    /// Partiality class for the projection.
    pub partiality: InfrastructurePartialityClass,
    /// Required reason when partiality is not complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partiality_reason: Option<String>,
    /// Export-safe projection summary.
    pub support_summary: String,
}

/// Gate tying operator-facing truth across docs, support, and UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructurePromotionGate {
    /// Docs truth reference used by reviewers.
    pub docs_truth_ref: String,
    /// Support-export truth reference.
    pub support_export_ref: String,
    /// UI or shell truth reference.
    pub ui_truth_ref: String,
    /// True when docs, support, and UI agree about operator-facing truth.
    pub operator_truth_alignment: bool,
    /// Promotion state for the beta row.
    pub promotion_state: InfrastructurePromotionState,
    /// Export-safe gate summary.
    pub gate_summary: String,
}

/// One alpha page: connectors, resources, relationships, and projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceAlphaPage {
    /// Optional fixture metadata for validation lanes.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<InfrastructureIntelligenceFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Capture timestamp for the packet.
    pub captured_at: String,
    /// Upstream contracts consumed by reference.
    pub contract_refs: InfrastructureIntelligenceContractRefs,
    /// Connector rows claimed by the packet.
    pub connectors: Vec<InfrastructureConnectorRecord>,
    /// Resource nodes derived from connector intelligence.
    pub resources: Vec<InfrastructureResourceRecord>,
    /// Relationship edges derived from connector intelligence.
    pub relationships: Vec<InfrastructureRelationshipRecord>,
    /// Consumer projections derived from the same packet.
    pub consumer_projections: Vec<InfrastructureConsumerProjection>,
    /// Promotion gate for operator-facing truth.
    pub promotion_gate: InfrastructurePromotionGate,
    /// Export-safe page summary.
    pub support_summary: String,
}

impl InfrastructureIntelligenceAlphaPage {
    /// Validate the page against alpha invariants.
    pub fn validate(&self) -> InfrastructureIntelligenceValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a search-lane projection from the provider-owned packet.
    pub fn search_projection(&self) -> InfrastructureSearchProjection {
        let result_rows = self
            .resources
            .iter()
            .map(|resource| {
                let relationship_refs = self
                    .relationships
                    .iter()
                    .filter(|relationship| {
                        relationship.from_resource_ref == resource.resource_id
                            || relationship.to_resource_ref == resource.resource_id
                    })
                    .map(|relationship| relationship.relationship_id.clone())
                    .collect();
                InfrastructureSearchResultRow {
                    result_id: format!("infra_search.result.{}", resource.resource_id),
                    resource_ref: resource.resource_id.clone(),
                    relationship_refs,
                    display_label: resource.display_label.clone(),
                    source_class: resource.source_class,
                    truth_layer: resource.truth_layer,
                    freshness_class: resource.freshness.freshness_class,
                    partiality: resource.partiality,
                    provenance_refs: resource.provenance_refs.clone(),
                }
            })
            .collect();
        InfrastructureSearchProjection {
            record_kind: INFRASTRUCTURE_SEARCH_PROJECTION_RECORD_KIND.to_string(),
            schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
            source_page_id: self.page_id.clone(),
            result_rows,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    /// Build a review-lane projection from the provider-owned packet.
    pub fn review_projection(&self) -> InfrastructureReviewProjection {
        let anchor_rows = self
            .relationships
            .iter()
            .map(|relationship| InfrastructureReviewAnchorRow {
                anchor_id: format!("infra_review.anchor.{}", relationship.relationship_id),
                relationship_ref: relationship.relationship_id.clone(),
                from_resource_ref: relationship.from_resource_ref.clone(),
                to_resource_ref: relationship.to_resource_ref.clone(),
                relationship_kind: relationship.relationship_kind,
                freshness_class: relationship.freshness.freshness_class,
                confidence_class: relationship.confidence_class,
                partiality: relationship.partiality,
                support_summary: relationship.support_summary.clone(),
            })
            .collect();
        InfrastructureReviewProjection {
            record_kind: INFRASTRUCTURE_REVIEW_PROJECTION_RECORD_KIND.to_string(),
            schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
            source_page_id: self.page_id.clone(),
            anchor_rows,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    /// Build a redaction-safe support export from the provider-owned packet.
    pub fn support_export_projection(&self) -> InfrastructureSupportExport {
        InfrastructureSupportExport {
            record_kind: INFRASTRUCTURE_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
            source_page_id: self.page_id.clone(),
            connector_summaries: self
                .connectors
                .iter()
                .map(|connector| InfrastructureConnectorSummary {
                    connector_id: connector.connector_id.clone(),
                    connector_kind: connector.connector_kind,
                    source_class: connector.source_class,
                    freshness_class: connector.freshness.freshness_class,
                    read_mode: connector.read_mode,
                    control_plane_boundary: connector.control_plane_boundary,
                    support_summary: connector.support_summary.clone(),
                })
                .collect(),
            resource_summaries: self
                .resources
                .iter()
                .map(|resource| InfrastructureResourceSummary {
                    resource_id: resource.resource_id.clone(),
                    connector_ref: resource.connector_ref.clone(),
                    display_label: resource.display_label.clone(),
                    source_class: resource.source_class,
                    truth_layer: resource.truth_layer,
                    freshness_class: resource.freshness.freshness_class,
                    partiality: resource.partiality,
                    support_summary: resource.support_summary.clone(),
                })
                .collect(),
            relationship_summaries: self
                .relationships
                .iter()
                .map(|relationship| InfrastructureRelationshipSummary {
                    relationship_id: relationship.relationship_id.clone(),
                    from_resource_ref: relationship.from_resource_ref.clone(),
                    to_resource_ref: relationship.to_resource_ref.clone(),
                    relationship_kind: relationship.relationship_kind,
                    truth_layer: relationship.truth_layer,
                    freshness_class: relationship.freshness.freshness_class,
                    confidence_class: relationship.confidence_class,
                    partiality: relationship.partiality,
                    support_summary: relationship.support_summary.clone(),
                })
                .collect(),
            consumer_surface_summaries: self
                .consumer_projections
                .iter()
                .map(|projection| InfrastructureConsumerSurfaceSummary {
                    projection_id: projection.projection_id.clone(),
                    surface: projection.surface,
                    source_packet_ref: projection.source_packet_ref.clone(),
                    relationship_count: projection.relationship_refs.len(),
                    resource_count: projection.resource_refs.len(),
                    partiality: projection.partiality,
                    support_summary: projection.support_summary.clone(),
                })
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_material_excluded: true,
            mutation_authority_excluded: true,
        }
    }
}

/// Validation report emitted by the alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed while validating the page.
    pub coverage: InfrastructureIntelligenceCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<InfrastructureIntelligenceFinding>,
}

/// Coverage observed during infrastructure intelligence validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceCoverage {
    /// Connector kinds covered by the page.
    pub connector_kinds: BTreeSet<InfrastructureConnectorKind>,
    /// Source classes covered by resources and relationships.
    pub source_classes: BTreeSet<InfrastructureSourceClass>,
    /// Truth layers covered by resources and relationships.
    pub truth_layers: BTreeSet<InfrastructureTruthLayer>,
    /// Relationship kinds covered by the page.
    pub relationship_kinds: BTreeSet<InfrastructureRelationshipKind>,
    /// Consumer surfaces covered by projections.
    pub consumer_surfaces: BTreeSet<InfrastructureConsumerSurface>,
    /// Partiality classes covered by resources, relationships, and projections.
    pub partiality_classes: BTreeSet<InfrastructurePartialityClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureIntelligenceFinding {
    /// Severity of the finding.
    pub severity: InfrastructureIntelligenceFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureIntelligenceFindingSeverity {
    /// Error that blocks the packet.
    Error,
    /// Warning that keeps the packet reviewable but visibly degraded.
    Warning,
}

/// Search-lane projection derived from the provider-owned page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureSearchProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Source page id.
    pub source_page_id: String,
    /// Search result rows.
    pub result_rows: Vec<InfrastructureSearchResultRow>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Search result row for one infrastructure resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureSearchResultRow {
    /// Stable result id.
    pub result_id: String,
    /// Resource id projected into search.
    pub resource_ref: String,
    /// Relationship ids linked to this resource.
    pub relationship_refs: Vec<String>,
    /// Redaction-safe display label.
    pub display_label: String,
    /// Source class label.
    pub source_class: InfrastructureSourceClass,
    /// Truth layer label.
    pub truth_layer: InfrastructureTruthLayer,
    /// Freshness class label.
    pub freshness_class: FreshnessLabel,
    /// Partiality label.
    pub partiality: InfrastructurePartialityClass,
    /// Provenance refs used by the result.
    pub provenance_refs: Vec<String>,
}

/// Review-lane projection derived from the provider-owned page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureReviewProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Source page id.
    pub source_page_id: String,
    /// Review anchors projected from relationships.
    pub anchor_rows: Vec<InfrastructureReviewAnchorRow>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Review anchor row for one infrastructure relationship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureReviewAnchorRow {
    /// Stable anchor id.
    pub anchor_id: String,
    /// Relationship id projected into review.
    pub relationship_ref: String,
    /// Source resource id.
    pub from_resource_ref: String,
    /// Target resource id.
    pub to_resource_ref: String,
    /// Relationship kind.
    pub relationship_kind: InfrastructureRelationshipKind,
    /// Freshness class label.
    pub freshness_class: FreshnessLabel,
    /// Confidence class label.
    pub confidence_class: InfrastructureConfidenceClass,
    /// Partiality label.
    pub partiality: InfrastructurePartialityClass,
    /// Export-safe anchor summary.
    pub support_summary: String,
}

/// Redaction-safe support export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Source page id.
    pub source_page_id: String,
    /// Connector summaries safe for support bundles.
    pub connector_summaries: Vec<InfrastructureConnectorSummary>,
    /// Resource summaries safe for support bundles.
    pub resource_summaries: Vec<InfrastructureResourceSummary>,
    /// Relationship summaries safe for support bundles.
    pub relationship_summaries: Vec<InfrastructureRelationshipSummary>,
    /// Consumer surface summaries safe for support bundles.
    pub consumer_surface_summaries: Vec<InfrastructureConsumerSurfaceSummary>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: raw manifests, logs, URLs, tokens, and payloads are excluded.
    pub raw_material_excluded: bool,
    /// Guardrail: mutation authority is excluded from the support export.
    pub mutation_authority_excluded: bool,
}

/// Export-safe connector summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureConnectorSummary {
    /// Stable connector id.
    pub connector_id: String,
    /// Connector kind.
    pub connector_kind: InfrastructureConnectorKind,
    /// Source class.
    pub source_class: InfrastructureSourceClass,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Read mode.
    pub read_mode: InfrastructureReadMode,
    /// Control-plane boundary.
    pub control_plane_boundary: InfrastructureControlPlaneBoundary,
    /// Export-safe connector summary.
    pub support_summary: String,
}

/// Export-safe resource summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureResourceSummary {
    /// Stable resource id.
    pub resource_id: String,
    /// Connector id that produced the resource.
    pub connector_ref: String,
    /// Redaction-safe display label.
    pub display_label: String,
    /// Source class.
    pub source_class: InfrastructureSourceClass,
    /// Truth layer.
    pub truth_layer: InfrastructureTruthLayer,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Partiality class.
    pub partiality: InfrastructurePartialityClass,
    /// Export-safe row summary.
    pub support_summary: String,
}

/// Export-safe relationship summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureRelationshipSummary {
    /// Stable relationship id.
    pub relationship_id: String,
    /// Source resource id.
    pub from_resource_ref: String,
    /// Target resource id.
    pub to_resource_ref: String,
    /// Relationship kind.
    pub relationship_kind: InfrastructureRelationshipKind,
    /// Truth layer.
    pub truth_layer: InfrastructureTruthLayer,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Confidence class.
    pub confidence_class: InfrastructureConfidenceClass,
    /// Partiality class.
    pub partiality: InfrastructurePartialityClass,
    /// Export-safe edge summary.
    pub support_summary: String,
}

/// Export-safe consumer surface summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureConsumerSurfaceSummary {
    /// Stable projection id.
    pub projection_id: String,
    /// Consumer surface.
    pub surface: InfrastructureConsumerSurface,
    /// Source page id.
    pub source_packet_ref: String,
    /// Count of consumed relationships.
    pub relationship_count: usize,
    /// Count of consumed resources.
    pub resource_count: usize,
    /// Partiality class for the projection.
    pub partiality: InfrastructurePartialityClass,
    /// Export-safe projection summary.
    pub support_summary: String,
}

struct Validator<'a> {
    page: &'a InfrastructureIntelligenceAlphaPage,
    connector_ids: BTreeSet<&'a str>,
    resource_ids: BTreeSet<&'a str>,
    relationship_ids: BTreeSet<&'a str>,
    projection_ids: BTreeSet<&'a str>,
    coverage: InfrastructureIntelligenceCoverage,
    findings: Vec<InfrastructureIntelligenceFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a InfrastructureIntelligenceAlphaPage) -> Self {
        Self {
            page,
            connector_ids: BTreeSet::new(),
            resource_ids: BTreeSet::new(),
            relationship_ids: BTreeSet::new(),
            projection_ids: BTreeSet::new(),
            coverage: InfrastructureIntelligenceCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_connectors();
        self.validate_resources();
        self.validate_relationships();
        self.validate_consumer_projections();
        self.validate_promotion_gate();
        self.validate_required_coverage();
    }

    fn finish(self) -> InfrastructureIntelligenceValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != InfrastructureIntelligenceFindingSeverity::Error);
        InfrastructureIntelligenceValidationReport {
            record_kind: INFRASTRUCTURE_INTELLIGENCE_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == INFRASTRUCTURE_INTELLIGENCE_ALPHA_PAGE_RECORD_KIND,
            "infrastructure_intelligence.page_record_kind",
            "page.record_kind must match the infrastructure intelligence page record kind",
        );
        self.expect(
            page.schema_version == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
            "infrastructure_intelligence.page_schema_version",
            "page.schema_version must match the alpha schema version",
        );
        self.expect(
            page.shared_contract_ref == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF,
            "infrastructure_intelligence.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "infrastructure_intelligence.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.captured_at.trim().is_empty(),
            "infrastructure_intelligence.captured_at_missing",
            "page.captured_at must be non-empty",
        );
        self.expect(
            !page.support_summary.trim().is_empty(),
            "infrastructure_intelligence.support_summary_missing",
            "page.support_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "infrastructure_intelligence.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.connectors.is_empty(),
            "infrastructure_intelligence.connectors_missing",
            "page must contain at least one infrastructure connector",
        );
        self.expect(
            !page.resources.is_empty(),
            "infrastructure_intelligence.resources_missing",
            "page must contain at least one resource row",
        );
        self.expect(
            !page.relationships.is_empty(),
            "infrastructure_intelligence.relationships_missing",
            "page must contain at least one relationship row",
        );
    }

    fn validate_connectors(&mut self) {
        for connector in &self.page.connectors {
            self.expect(
                connector.record_kind == INFRASTRUCTURE_CONNECTOR_RECORD_KIND,
                "infrastructure_intelligence.connector_record_kind",
                "connector.record_kind is wrong",
            );
            self.expect(
                connector.schema_version == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
                "infrastructure_intelligence.connector_schema_version",
                "connector.schema_version is wrong",
            );
            self.expect(
                connector.shared_contract_ref
                    == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF,
                "infrastructure_intelligence.connector_shared_contract_ref",
                "connector.shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.connector_ids.insert(&connector.connector_id);
            self.expect(
                id_is_unique,
                "infrastructure_intelligence.connector_duplicate",
                "connector_id values must be unique within a page",
            );
            self.expect(
                !connector.connector_label.trim().is_empty()
                    && !connector.provider_descriptor_ref.trim().is_empty()
                    && !connector.support_summary.trim().is_empty(),
                "infrastructure_intelligence.connector_required_text_missing",
                "connector label, descriptor ref, and support summary must be non-empty",
            );
            self.validate_target_context(&connector.target_context);
            self.validate_freshness(&connector.freshness, "connector");
            self.expect(
                connector.control_plane_boundary.is_read_only_alpha(),
                "infrastructure_intelligence.connector_mutation_boundary_claimed",
                "infrastructure intelligence alpha cannot claim in-product mutation authority",
            );
            self.expect(
                connector.read_only_source_intelligence,
                "infrastructure_intelligence.connector_not_read_only",
                "connector must declare read_only_source_intelligence=true",
            );
            self.expect(
                connector.mutation_authority_excluded,
                "infrastructure_intelligence.connector_mutation_authority_not_excluded",
                "connector must exclude mutation authority from this packet",
            );
            self.expect(
                !connector.hidden_ai_writes_allowed,
                "infrastructure_intelligence.connector_hidden_ai_writes",
                "hidden AI writes are forbidden for infrastructure source intelligence",
            );
            self.expect(
                !connector.hidden_provider_writes_allowed,
                "infrastructure_intelligence.connector_hidden_provider_writes",
                "hidden provider writes are forbidden for infrastructure source intelligence",
            );
            self.coverage
                .connector_kinds
                .insert(connector.connector_kind);
            self.coverage.source_classes.insert(connector.source_class);
            self.coverage
                .partiality_classes
                .insert(InfrastructurePartialityClass::Complete);
        }
    }

    fn validate_resources(&mut self) {
        for resource in &self.page.resources {
            self.expect(
                resource.record_kind == INFRASTRUCTURE_RESOURCE_RECORD_KIND,
                "infrastructure_intelligence.resource_record_kind",
                "resource.record_kind is wrong",
            );
            self.expect(
                resource.schema_version == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
                "infrastructure_intelligence.resource_schema_version",
                "resource.schema_version is wrong",
            );
            self.expect(
                resource.shared_contract_ref
                    == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF,
                "infrastructure_intelligence.resource_shared_contract_ref",
                "resource.shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.resource_ids.insert(&resource.resource_id);
            self.expect(
                id_is_unique,
                "infrastructure_intelligence.resource_duplicate",
                "resource_id values must be unique within a page",
            );
            self.expect(
                self.connector_ids.contains(resource.connector_ref.as_str()),
                "infrastructure_intelligence.resource_connector_unknown",
                "resource.connector_ref must reference a connector in the page",
            );
            self.expect(
                !resource.display_label.trim().is_empty()
                    && !resource.source_artifact_ref.trim().is_empty()
                    && !resource.support_summary.trim().is_empty(),
                "infrastructure_intelligence.resource_required_text_missing",
                "resource display label, source artifact ref, and support summary must be non-empty",
            );
            self.validate_resource_identity(&resource.resource_identity);
            self.validate_freshness(&resource.freshness, "resource");
            self.validate_partiality(
                resource.partiality,
                resource.partiality_reason.as_deref(),
                "infrastructure_intelligence.resource_partiality_reason_missing",
                "resource partiality must be labeled with a reason",
            );
            self.expect(
                !resource.provider_overlay_replaces_repo_truth,
                "infrastructure_intelligence.resource_overlay_replaces_repo_truth",
                "provider overlays may enrich navigation but may not replace repo truth",
            );
            self.expect(
                !resource.active_control_plane_mutation_authority,
                "infrastructure_intelligence.resource_active_mutation_authority",
                "resource rows may not carry active control-plane mutation authority",
            );
            self.expect(
                !resource.provenance_refs.is_empty()
                    && resource
                        .provenance_refs
                        .iter()
                        .all(|reference| !reference.trim().is_empty()),
                "infrastructure_intelligence.resource_provenance_missing",
                "resource rows must carry at least one provenance ref",
            );
            self.coverage.source_classes.insert(resource.source_class);
            self.coverage.truth_layers.insert(resource.truth_layer);
            self.coverage.partiality_classes.insert(resource.partiality);
        }
    }

    fn validate_relationships(&mut self) {
        for relationship in &self.page.relationships {
            self.expect(
                relationship.record_kind == INFRASTRUCTURE_RELATIONSHIP_RECORD_KIND,
                "infrastructure_intelligence.relationship_record_kind",
                "relationship.record_kind is wrong",
            );
            self.expect(
                relationship.schema_version == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
                "infrastructure_intelligence.relationship_schema_version",
                "relationship.schema_version is wrong",
            );
            self.expect(
                relationship.shared_contract_ref
                    == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF,
                "infrastructure_intelligence.relationship_shared_contract_ref",
                "relationship.shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.relationship_ids.insert(&relationship.relationship_id);
            self.expect(
                id_is_unique,
                "infrastructure_intelligence.relationship_duplicate",
                "relationship_id values must be unique within a page",
            );
            self.expect(
                self.resource_ids
                    .contains(relationship.from_resource_ref.as_str()),
                "infrastructure_intelligence.relationship_from_unknown",
                "relationship.from_resource_ref must reference a resource in the page",
            );
            self.expect(
                self.resource_ids
                    .contains(relationship.to_resource_ref.as_str()),
                "infrastructure_intelligence.relationship_to_unknown",
                "relationship.to_resource_ref must reference a resource in the page",
            );
            self.validate_freshness(&relationship.freshness, "relationship");
            self.validate_partiality(
                relationship.partiality,
                relationship.partiality_reason.as_deref(),
                "infrastructure_intelligence.relationship_partiality_reason_missing",
                "relationship partiality must be labeled with a reason",
            );
            self.expect(
                !relationship.provenance_refs.is_empty()
                    && relationship
                        .provenance_refs
                        .iter()
                        .all(|reference| !reference.trim().is_empty()),
                "infrastructure_intelligence.relationship_provenance_missing",
                "relationship rows must carry at least one provenance ref",
            );
            self.expect(
                !relationship.support_summary.trim().is_empty(),
                "infrastructure_intelligence.relationship_support_summary_missing",
                "relationship.support_summary must be non-empty",
            );
            self.coverage
                .relationship_kinds
                .insert(relationship.relationship_kind);
            self.coverage
                .source_classes
                .insert(relationship.relationship_source);
            self.coverage.truth_layers.insert(relationship.truth_layer);
            self.coverage
                .partiality_classes
                .insert(relationship.partiality);
        }
    }

    fn validate_consumer_projections(&mut self) {
        for projection in &self.page.consumer_projections {
            self.expect(
                projection.record_kind == INFRASTRUCTURE_CONSUMER_PROJECTION_RECORD_KIND,
                "infrastructure_intelligence.projection_record_kind",
                "consumer projection record_kind is wrong",
            );
            self.expect(
                projection.schema_version == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
                "infrastructure_intelligence.projection_schema_version",
                "consumer projection schema_version is wrong",
            );
            self.expect(
                projection.shared_contract_ref
                    == INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF,
                "infrastructure_intelligence.projection_shared_contract_ref",
                "consumer projection shared_contract_ref must match the shared contract id",
            );
            let id_is_unique = self.projection_ids.insert(&projection.projection_id);
            self.expect(
                id_is_unique,
                "infrastructure_intelligence.projection_duplicate",
                "projection_id values must be unique within a page",
            );
            self.expect(
                projection.source_packet_ref == self.page.page_id,
                "infrastructure_intelligence.projection_source_packet_mismatch",
                "consumer projections must cite the same page id they consume",
            );
            self.expect(
                projection.uses_same_relationship_packet,
                "infrastructure_intelligence.projection_not_shared_packet",
                "search, review, support, and AI projections must consume the same relationship packet",
            );
            self.expect(
                !projection.parallel_truth_store_created,
                "infrastructure_intelligence.projection_parallel_truth_store",
                "consumer projections must not create parallel truth stores",
            );
            self.expect(
                projection
                    .relationship_refs
                    .iter()
                    .all(|reference| self.relationship_ids.contains(reference.as_str())),
                "infrastructure_intelligence.projection_relationship_unknown",
                "consumer projection relationship refs must exist in the page",
            );
            self.expect(
                projection
                    .resource_refs
                    .iter()
                    .all(|reference| self.resource_ids.contains(reference.as_str())),
                "infrastructure_intelligence.projection_resource_unknown",
                "consumer projection resource refs must exist in the page",
            );
            self.validate_freshness(&projection.freshness, "consumer projection");
            self.validate_partiality(
                projection.partiality,
                projection.partiality_reason.as_deref(),
                "infrastructure_intelligence.projection_partiality_reason_missing",
                "consumer projection partiality must be labeled with a reason",
            );
            self.expect(
                !projection.support_summary.trim().is_empty(),
                "infrastructure_intelligence.projection_support_summary_missing",
                "consumer projection support_summary must be non-empty",
            );
            self.coverage.consumer_surfaces.insert(projection.surface);
            self.coverage
                .partiality_classes
                .insert(projection.partiality);
        }
    }

    fn validate_promotion_gate(&mut self) {
        let gate = &self.page.promotion_gate;
        self.expect(
            !gate.docs_truth_ref.trim().is_empty()
                && !gate.support_export_ref.trim().is_empty()
                && !gate.ui_truth_ref.trim().is_empty()
                && !gate.gate_summary.trim().is_empty(),
            "infrastructure_intelligence.promotion_gate_refs_missing",
            "promotion gate must cite docs, support, UI truth refs, and a summary",
        );
        if gate.operator_truth_alignment {
            self.expect(
                gate.promotion_state == InfrastructurePromotionState::PromotionReady,
                "infrastructure_intelligence.promotion_gate_alignment_state_mismatch",
                "aligned operator truth must use promotion_ready",
            );
        } else {
            self.expect(
                gate.promotion_state != InfrastructurePromotionState::PromotionReady,
                "infrastructure_intelligence.promotion_gate_truth_drift_not_blocked",
                "operator-truth drift must block promotion",
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        for connector_kind in [
            InfrastructureConnectorKind::TerraformWorkspace,
            InfrastructureConnectorKind::KubernetesCluster,
            InfrastructureConnectorKind::ContainerRuntime,
            InfrastructureConnectorKind::CiProvider,
            InfrastructureConnectorKind::PolicyEngine,
        ] {
            self.expect(
                self.coverage.connector_kinds.contains(&connector_kind),
                "infrastructure_intelligence.coverage_connector_kind_missing",
                "page must cover Terraform, Kubernetes, container, CI, and policy connectors",
            );
        }
        for source_class in [
            InfrastructureSourceClass::TerraformHcl,
            InfrastructureSourceClass::KubernetesManifest,
            InfrastructureSourceClass::ContainerDescriptor,
            InfrastructureSourceClass::CiEnvironmentDescriptor,
            InfrastructureSourceClass::PolicyAccessConfig,
        ] {
            self.expect(
                self.coverage.source_classes.contains(&source_class),
                "infrastructure_intelligence.coverage_source_class_missing",
                "page must cover every claimed infrastructure source class",
            );
        }
        for truth_layer in [
            InfrastructureTruthLayer::Authored,
            InfrastructureTruthLayer::Rendered,
            InfrastructureTruthLayer::Planned,
            InfrastructureTruthLayer::Observed,
            InfrastructureTruthLayer::ProviderOverlay,
        ] {
            self.expect(
                self.coverage.truth_layers.contains(&truth_layer),
                "infrastructure_intelligence.coverage_truth_layer_missing",
                "page must cover authored, rendered, planned, observed, and provider-overlay truth",
            );
        }
        for surface in [
            InfrastructureConsumerSurface::Search,
            InfrastructureConsumerSurface::Review,
            InfrastructureConsumerSurface::Support,
        ] {
            self.expect(
                self.coverage.consumer_surfaces.contains(&surface),
                "infrastructure_intelligence.coverage_consumer_surface_missing",
                "page must cover search, review, and support consumer projections",
            );
        }
    }

    fn validate_target_context(&mut self, target_context: &InfrastructureTargetContext) {
        self.expect(
            !target_context.provider_ref.trim().is_empty()
                && !target_context.account_or_project_ref.trim().is_empty()
                && !target_context.environment_ref.trim().is_empty()
                && !target_context.connector_class_label.trim().is_empty(),
            "infrastructure_intelligence.target_context_required_fields_missing",
            "target context must carry provider, account/project, environment, and connector class",
        );
    }

    fn validate_resource_identity(&mut self, identity: &InfrastructureResourceIdentity) {
        self.expect(
            !identity.identity_class.trim().is_empty()
                && !identity.resource_kind.trim().is_empty()
                && !identity.resource_name_ref.trim().is_empty(),
            "infrastructure_intelligence.resource_identity_required_fields_missing",
            "resource identity must carry identity class, kind, and name ref",
        );
    }

    fn validate_freshness(&mut self, freshness: &FreshnessTruth, owner: &str) {
        self.expect(
            !freshness.freshness_floor_ref.trim().is_empty(),
            "infrastructure_intelligence.freshness_floor_missing",
            &format!("{owner} freshness must cite a freshness_floor_ref"),
        );
        if !matches!(freshness.freshness_class, FreshnessLabel::Fresh) {
            self.expect(
                freshness
                    .degraded_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "infrastructure_intelligence.freshness_degraded_reason_missing",
                &format!("{owner} degraded freshness must carry a reason"),
            );
        }
    }

    fn validate_partiality(
        &mut self,
        partiality: InfrastructurePartialityClass,
        partiality_reason: Option<&str>,
        check_id: &str,
        message: &str,
    ) {
        if partiality.requires_reason() {
            self.expect(
                partiality_reason.is_some_and(|reason| !reason.trim().is_empty()),
                check_id,
                message,
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(InfrastructureIntelligenceFinding {
                severity: InfrastructureIntelligenceFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

fn fresh(observed_at: &str, floor_ref: &str) -> FreshnessTruth {
    FreshnessTruth {
        freshness_class: FreshnessLabel::Fresh,
        observed_at: Some(observed_at.to_string()),
        freshness_floor_ref: floor_ref.to_string(),
        stale_after: Some("PT15M".to_string()),
        degraded_reason: None,
        import_session_ref: None,
    }
}

fn stale(observed_at: &str, floor_ref: &str, reason: &str) -> FreshnessTruth {
    FreshnessTruth {
        freshness_class: FreshnessLabel::StaleWithinWindow,
        observed_at: Some(observed_at.to_string()),
        freshness_floor_ref: floor_ref.to_string(),
        stale_after: Some("PT15M".to_string()),
        degraded_reason: Some(reason.to_string()),
        import_session_ref: Some(format!("import.session.{floor_ref}")),
    }
}

fn connector(
    id: &str,
    connector_kind: InfrastructureConnectorKind,
    label: &str,
    provider_descriptor_ref: &str,
    source_class: InfrastructureSourceClass,
    read_mode: InfrastructureReadMode,
    boundary: InfrastructureControlPlaneBoundary,
    freshness: FreshnessTruth,
    support_summary: &str,
) -> InfrastructureConnectorRecord {
    InfrastructureConnectorRecord {
        record_kind: INFRASTRUCTURE_CONNECTOR_RECORD_KIND.to_string(),
        schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
        shared_contract_ref: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF.to_string(),
        connector_id: id.to_string(),
        connector_kind,
        connector_label: label.to_string(),
        provider_descriptor_ref: provider_descriptor_ref.to_string(),
        target_context: InfrastructureTargetContext {
            provider_ref: format!("provider.ref.{id}"),
            account_or_project_ref: format!("account.project.{id}"),
            environment_ref: "environment.staging".to_string(),
            cluster_ref: (connector_kind == InfrastructureConnectorKind::KubernetesCluster)
                .then(|| "cluster.checkout.staging".to_string()),
            namespace_ref: (connector_kind == InfrastructureConnectorKind::KubernetesCluster)
                .then(|| "namespace.checkout".to_string()),
            region_ref: Some("region.us-west".to_string()),
            connector_class_label: connector_kind.as_str().to_string(),
        },
        source_class,
        freshness,
        read_mode,
        control_plane_boundary: boundary,
        read_only_source_intelligence: true,
        mutation_authority_excluded: true,
        hidden_ai_writes_allowed: false,
        hidden_provider_writes_allowed: false,
        support_summary: support_summary.to_string(),
    }
}

fn resource(
    id: &str,
    connector_ref: &str,
    label: &str,
    source_artifact_ref: &str,
    identity_class: &str,
    resource_kind: &str,
    source_class: InfrastructureSourceClass,
    truth_layer: InfrastructureTruthLayer,
    freshness: FreshnessTruth,
    partiality: InfrastructurePartialityClass,
    partiality_reason: Option<&str>,
    support_summary: &str,
) -> InfrastructureResourceRecord {
    InfrastructureResourceRecord {
        record_kind: INFRASTRUCTURE_RESOURCE_RECORD_KIND.to_string(),
        schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
        shared_contract_ref: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF.to_string(),
        resource_id: id.to_string(),
        connector_ref: connector_ref.to_string(),
        display_label: label.to_string(),
        source_artifact_ref: source_artifact_ref.to_string(),
        resource_identity: InfrastructureResourceIdentity {
            identity_class: identity_class.to_string(),
            resource_kind: resource_kind.to_string(),
            resource_name_ref: format!("resource.name.{id}"),
            namespace_or_project_ref: Some("scope.checkout".to_string()),
            address_or_selector_ref: Some(format!("selector.{id}")),
        },
        source_class,
        truth_layer,
        freshness,
        partiality,
        partiality_reason: partiality_reason.map(str::to_string),
        provider_overlay_replaces_repo_truth: false,
        active_control_plane_mutation_authority: false,
        provenance_refs: vec![format!("provenance.{id}")],
        support_summary: support_summary.to_string(),
    }
}

fn relationship(
    id: &str,
    from: &str,
    to: &str,
    relationship_kind: InfrastructureRelationshipKind,
    truth_layer: InfrastructureTruthLayer,
    source_class: InfrastructureSourceClass,
    freshness: FreshnessTruth,
    confidence_class: InfrastructureConfidenceClass,
    partiality: InfrastructurePartialityClass,
    partiality_reason: Option<&str>,
    support_summary: &str,
) -> InfrastructureRelationshipRecord {
    InfrastructureRelationshipRecord {
        record_kind: INFRASTRUCTURE_RELATIONSHIP_RECORD_KIND.to_string(),
        schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
        shared_contract_ref: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF.to_string(),
        relationship_id: id.to_string(),
        from_resource_ref: from.to_string(),
        to_resource_ref: to.to_string(),
        relationship_kind,
        truth_layer,
        relationship_source: source_class,
        freshness,
        confidence_class,
        partiality,
        partiality_reason: partiality_reason.map(str::to_string),
        provenance_refs: vec![format!("provenance.{id}")],
        support_summary: support_summary.to_string(),
    }
}

fn projection(
    id: &str,
    surface: InfrastructureConsumerSurface,
    page_id: &str,
    relationship_refs: Vec<String>,
    resource_refs: Vec<String>,
    partiality: InfrastructurePartialityClass,
    partiality_reason: Option<&str>,
    support_summary: &str,
) -> InfrastructureConsumerProjection {
    InfrastructureConsumerProjection {
        record_kind: INFRASTRUCTURE_CONSUMER_PROJECTION_RECORD_KIND.to_string(),
        schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
        shared_contract_ref: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF.to_string(),
        projection_id: id.to_string(),
        surface,
        source_packet_ref: page_id.to_string(),
        relationship_refs,
        resource_refs,
        freshness: fresh("2026-05-17T16:00:00Z", &format!("freshness.{id}")),
        uses_same_relationship_packet: true,
        parallel_truth_store_created: false,
        partiality,
        partiality_reason: partiality_reason.map(str::to_string),
        support_summary: support_summary.to_string(),
    }
}

/// Builds the protected alpha fixture packet.
pub fn seeded_infrastructure_intelligence_alpha_page() -> InfrastructureIntelligenceAlphaPage {
    let page_id = "infra_intelligence_alpha.page.claimed_connectors".to_string();
    let connectors = vec![
        connector(
            "terraform.checkout",
            InfrastructureConnectorKind::TerraformWorkspace,
            "Terraform checkout workspace",
            "provider_descriptor.infrastructure.terraform",
            InfrastructureSourceClass::TerraformHcl,
            InfrastructureReadMode::ReadOnlySourceIntelligence,
            InfrastructureControlPlaneBoundary::CompareOnlyPreview,
            fresh(
                "2026-05-17T16:00:00Z",
                "freshness.infrastructure.terraform.checkout",
            ),
            "Terraform connector extracts module, provider, and plan relationships without apply authority.",
        ),
        connector(
            "kubernetes.checkout",
            InfrastructureConnectorKind::KubernetesCluster,
            "Kubernetes checkout cluster",
            "provider_descriptor.infrastructure.kubernetes",
            InfrastructureSourceClass::KubernetesManifest,
            InfrastructureReadMode::ReadOnlyProviderOverlay,
            InfrastructureControlPlaneBoundary::ExternalHandoffRequired,
            fresh(
                "2026-05-17T16:00:00Z",
                "freshness.infrastructure.kubernetes.checkout",
            ),
            "Kubernetes connector links authored and rendered manifests to observed resources as read-only topology.",
        ),
        connector(
            "container.checkout",
            InfrastructureConnectorKind::ContainerRuntime,
            "Container checkout runtime",
            "provider_descriptor.infrastructure.container",
            InfrastructureSourceClass::ContainerDescriptor,
            InfrastructureReadMode::ReadOnlyProviderOverlay,
            InfrastructureControlPlaneBoundary::ReadOnlyNoMutationAuthority,
            stale(
                "2026-05-17T15:48:00Z",
                "freshness.infrastructure.container.checkout",
                "Container log snapshot is twelve minutes older than the connector freshness floor.",
            ),
            "Container connector surfaces service, port, and log-stream relationships with cached freshness labels.",
        ),
        connector(
            "ci.checkout",
            InfrastructureConnectorKind::CiProvider,
            "CI checkout deployment pipeline",
            "provider_descriptor.infrastructure.ci",
            InfrastructureSourceClass::CiEnvironmentDescriptor,
            InfrastructureReadMode::ReadOnlyProviderOverlay,
            InfrastructureControlPlaneBoundary::ExternalHandoffRequired,
            fresh("2026-05-17T16:00:00Z", "freshness.infrastructure.ci.checkout"),
            "CI connector links runs and artifacts as read-only operational evidence.",
        ),
        connector(
            "policy.checkout",
            InfrastructureConnectorKind::PolicyEngine,
            "Policy checkout admission lane",
            "provider_descriptor.infrastructure.policy",
            InfrastructureSourceClass::PolicyAccessConfig,
            InfrastructureReadMode::ReadOnlyProviderOverlay,
            InfrastructureControlPlaneBoundary::ReadOnlyNoMutationAuthority,
            fresh(
                "2026-05-17T16:00:00Z",
                "freshness.infrastructure.policy.checkout",
            ),
            "Policy connector links policy source to target resources and enforcement observations without mutation.",
        ),
    ];

    let resources = vec![
        resource(
            "resource.terraform.module.checkout",
            "terraform.checkout",
            "Terraform module checkout",
            "source.infra.checkout.main_tf",
            "terraform_module",
            "module",
            InfrastructureSourceClass::TerraformHcl,
            InfrastructureTruthLayer::Authored,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.tf.module"),
            InfrastructurePartialityClass::Complete,
            None,
            "Authored Terraform module remains the source-owned desired state.",
        ),
        resource(
            "resource.terraform.provider.aws",
            "terraform.checkout",
            "Terraform AWS provider",
            "source.infra.checkout.providers_tf",
            "terraform_provider",
            "provider",
            InfrastructureSourceClass::TerraformHcl,
            InfrastructureTruthLayer::Authored,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.tf.provider"),
            InfrastructurePartialityClass::Complete,
            None,
            "Authored provider binding is extracted from Terraform source.",
        ),
        resource(
            "resource.terraform.plan.checkout_api",
            "terraform.checkout",
            "Terraform plan checkout-api service",
            "plan.infra.checkout_api.20260517",
            "terraform_planned_resource",
            "aws_service",
            InfrastructureSourceClass::TerraformHcl,
            InfrastructureTruthLayer::Planned,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.tf.plan"),
            InfrastructurePartialityClass::Complete,
            None,
            "Planned resource stays compare-only and does not imply apply.",
        ),
        resource(
            "resource.k8s.manifest.checkout_api",
            "kubernetes.checkout",
            "Kubernetes authored checkout-api manifest",
            "source.k8s.checkout.deployment_yaml",
            "kubernetes_manifest",
            "Deployment",
            InfrastructureSourceClass::KubernetesManifest,
            InfrastructureTruthLayer::Authored,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.k8s.manifest"),
            InfrastructurePartialityClass::Complete,
            None,
            "Authored manifest is distinct from rendered and live cluster state.",
        ),
        resource(
            "resource.k8s.rendered.checkout_api",
            "kubernetes.checkout",
            "Kubernetes rendered checkout-api deployment",
            "render.k8s.checkout.deployment",
            "kubernetes_rendered_object",
            "Deployment",
            InfrastructureSourceClass::KubernetesManifest,
            InfrastructureTruthLayer::Rendered,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.k8s.rendered"),
            InfrastructurePartialityClass::Complete,
            None,
            "Rendered deployment carries source-set provenance and render freshness.",
        ),
        resource(
            "resource.k8s.live.checkout_api",
            "kubernetes.checkout",
            "Kubernetes live checkout-api deployment",
            "connector.snapshot.k8s.checkout.live_deployment",
            "kubernetes_live_resource",
            "Deployment",
            InfrastructureSourceClass::KubernetesManifest,
            InfrastructureTruthLayer::Observed,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.k8s.live"),
            InfrastructurePartialityClass::Complete,
            None,
            "Observed live deployment is labeled as connector truth and remains read-only.",
        ),
        resource(
            "resource.container.service.checkout_api",
            "container.checkout",
            "Container service checkout-api",
            "source.compose.checkout.compose_yaml",
            "container_service",
            "service",
            InfrastructureSourceClass::ContainerDescriptor,
            InfrastructureTruthLayer::Authored,
            stale(
                "2026-05-17T15:48:00Z",
                "freshness.resource.container.service",
                "Compose service source was read from the last cached container connector snapshot.",
            ),
            InfrastructurePartialityClass::PartialIndex,
            Some("Container connector index is warming; service-to-port edges are present but sibling services are deferred."),
            "Container service row is cached and explicitly partial while the index warms.",
        ),
        resource(
            "resource.container.logs.checkout_api",
            "container.checkout",
            "Container checkout-api log stream",
            "connector.snapshot.container.logs.checkout_api",
            "container_log_stream",
            "log_stream",
            InfrastructureSourceClass::ContainerDescriptor,
            InfrastructureTruthLayer::Cached,
            stale(
                "2026-05-17T15:48:00Z",
                "freshness.resource.container.logs",
                "Log stream is cached; live tail was not requested.",
            ),
            InfrastructurePartialityClass::PartialRetrieval,
            Some("Only the bounded cached log window was retrieved for relationship evidence."),
            "Log stream is labeled cached and partial; no live tail is implied.",
        ),
        resource(
            "resource.ci.run.checkout_deploy",
            "ci.checkout",
            "CI deploy run checkout-api",
            "provider.overlay.ci.run.checkout_deploy",
            "ci_pipeline_run",
            "pipeline_run",
            InfrastructureSourceClass::CiEnvironmentDescriptor,
            InfrastructureTruthLayer::Observed,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.ci.run"),
            InfrastructurePartialityClass::Complete,
            None,
            "Observed CI run is read-only provider evidence.",
        ),
        resource(
            "resource.ci.artifact.checkout_bundle",
            "ci.checkout",
            "CI artifact checkout deployment bundle",
            "provider.overlay.ci.artifact.checkout_bundle",
            "ci_artifact",
            "artifact",
            InfrastructureSourceClass::CiEnvironmentDescriptor,
            InfrastructureTruthLayer::ProviderOverlay,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.ci.artifact"),
            InfrastructurePartialityClass::Complete,
            None,
            "Provider artifact overlay enriches review evidence without replacing local build truth.",
        ),
        resource(
            "resource.policy.network.checkout_api",
            "policy.checkout",
            "Policy network checkout-api",
            "source.policy.checkout.network_yaml",
            "policy_rule",
            "NetworkPolicy",
            InfrastructureSourceClass::PolicyAccessConfig,
            InfrastructureTruthLayer::Authored,
            fresh("2026-05-17T16:00:00Z", "freshness.resource.policy.network"),
            InfrastructurePartialityClass::Complete,
            None,
            "Authored policy source stays separate from enforcement observations.",
        ),
        resource(
            "resource.policy.enforcement.checkout_api",
            "policy.checkout",
            "Policy enforcement checkout-api",
            "connector.snapshot.policy.enforcement.checkout_api",
            "policy_enforcement_result",
            "enforcement_result",
            InfrastructureSourceClass::PolicyAccessConfig,
            InfrastructureTruthLayer::Observed,
            fresh(
                "2026-05-17T16:00:00Z",
                "freshness.resource.policy.enforcement",
            ),
            InfrastructurePartialityClass::Complete,
            None,
            "Observed enforcement result is read-only connector truth.",
        ),
    ];

    let relationships = vec![
        relationship(
            "relationship.tf.module_to_provider",
            "resource.terraform.module.checkout",
            "resource.terraform.provider.aws",
            InfrastructureRelationshipKind::ResourceToProvider,
            InfrastructureTruthLayer::Authored,
            InfrastructureSourceClass::TerraformHcl,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.tf.provider"),
            InfrastructureConfidenceClass::Direct,
            InfrastructurePartialityClass::Complete,
            None,
            "Terraform source binds checkout module resources to the AWS provider.",
        ),
        relationship(
            "relationship.tf.plan_to_resource",
            "resource.terraform.module.checkout",
            "resource.terraform.plan.checkout_api",
            InfrastructureRelationshipKind::PlanToResource,
            InfrastructureTruthLayer::Planned,
            InfrastructureSourceClass::TerraformHcl,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.tf.plan"),
            InfrastructureConfidenceClass::Verified,
            InfrastructurePartialityClass::Complete,
            None,
            "Terraform plan links the authored module to a planned checkout-api resource.",
        ),
        relationship(
            "relationship.k8s.source_to_rendered",
            "resource.k8s.manifest.checkout_api",
            "resource.k8s.rendered.checkout_api",
            InfrastructureRelationshipKind::SourceToRenderedObject,
            InfrastructureTruthLayer::Rendered,
            InfrastructureSourceClass::KubernetesManifest,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.k8s.rendered"),
            InfrastructureConfidenceClass::Verified,
            InfrastructurePartialityClass::Complete,
            None,
            "Authored Kubernetes manifest resolves to the rendered deployment object.",
        ),
        relationship(
            "relationship.k8s.rendered_to_live",
            "resource.k8s.rendered.checkout_api",
            "resource.k8s.live.checkout_api",
            InfrastructureRelationshipKind::ObjectToLiveResource,
            InfrastructureTruthLayer::Observed,
            InfrastructureSourceClass::KubernetesManifest,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.k8s.live"),
            InfrastructureConfidenceClass::Direct,
            InfrastructurePartialityClass::Complete,
            None,
            "Rendered deployment is linked to the observed live deployment without implying mutation.",
        ),
        relationship(
            "relationship.container.service_to_logs",
            "resource.container.service.checkout_api",
            "resource.container.logs.checkout_api",
            InfrastructureRelationshipKind::ObjectToLogOrEventStream,
            InfrastructureTruthLayer::Cached,
            InfrastructureSourceClass::ContainerDescriptor,
            stale(
                "2026-05-17T15:48:00Z",
                "freshness.edge.container.logs",
                "Service-to-log edge came from a cached connector snapshot.",
            ),
            InfrastructureConfidenceClass::Direct,
            InfrastructurePartialityClass::PartialIndex,
            Some("Container connector index has not loaded sibling service log edges yet."),
            "Container service is linked to a cached log stream with explicit partial-index truth.",
        ),
        relationship(
            "relationship.ci.run_to_artifact",
            "resource.ci.run.checkout_deploy",
            "resource.ci.artifact.checkout_bundle",
            InfrastructureRelationshipKind::RunToArtifact,
            InfrastructureTruthLayer::ProviderOverlay,
            InfrastructureSourceClass::CiEnvironmentDescriptor,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.ci.artifact"),
            InfrastructureConfidenceClass::Direct,
            InfrastructurePartialityClass::Complete,
            None,
            "CI run links to provider artifact overlay as read-only evidence.",
        ),
        relationship(
            "relationship.policy.to_live_resource",
            "resource.policy.network.checkout_api",
            "resource.k8s.live.checkout_api",
            InfrastructureRelationshipKind::PolicyToTargetResource,
            InfrastructureTruthLayer::Authored,
            InfrastructureSourceClass::PolicyAccessConfig,
            fresh("2026-05-17T16:00:00Z", "freshness.edge.policy.target"),
            InfrastructureConfidenceClass::Transitive,
            InfrastructurePartialityClass::Complete,
            None,
            "Policy source targets the live checkout-api deployment by stable selector ref.",
        ),
        relationship(
            "relationship.policy.to_enforcement",
            "resource.policy.network.checkout_api",
            "resource.policy.enforcement.checkout_api",
            InfrastructureRelationshipKind::PolicyToEnforcementResult,
            InfrastructureTruthLayer::Observed,
            InfrastructureSourceClass::PolicyAccessConfig,
            fresh(
                "2026-05-17T16:00:00Z",
                "freshness.edge.policy.enforcement",
            ),
            InfrastructureConfidenceClass::Verified,
            InfrastructurePartialityClass::Complete,
            None,
            "Policy source links to observed enforcement result with read-only provenance.",
        ),
    ];

    let all_relationship_refs = relationships
        .iter()
        .map(|relationship| relationship.relationship_id.clone())
        .collect::<Vec<_>>();
    let all_resource_refs = resources
        .iter()
        .map(|resource| resource.resource_id.clone())
        .collect::<Vec<_>>();
    let consumer_projections = vec![
        projection(
            "projection.search.infrastructure",
            InfrastructureConsumerSurface::Search,
            &page_id,
            all_relationship_refs.clone(),
            all_resource_refs.clone(),
            InfrastructurePartialityClass::PartialIndex,
            Some("Search includes the container connector's labeled partial-index lane."),
            "Search consumes the provider-owned relationship packet and preserves partial-index labels.",
        ),
        projection(
            "projection.review.infrastructure",
            InfrastructureConsumerSurface::Review,
            &page_id,
            all_relationship_refs.clone(),
            all_resource_refs.clone(),
            InfrastructurePartialityClass::PartialRetrieval,
            Some("Review includes the cached log-stream relationship as a bounded retrieved slice."),
            "Review anchors cite the same relationship ids used by search and support.",
        ),
        projection(
            "projection.support.infrastructure",
            InfrastructureConsumerSurface::Support,
            &page_id,
            all_relationship_refs.clone(),
            all_resource_refs.clone(),
            InfrastructurePartialityClass::Complete,
            None,
            "Support export summarizes the same packet without raw manifests, logs, URLs, or tokens.",
        ),
        projection(
            "projection.ai_context.infrastructure",
            InfrastructureConsumerSurface::AiContext,
            &page_id,
            all_relationship_refs,
            all_resource_refs,
            InfrastructurePartialityClass::PartialRetrieval,
            Some("AI context may cite only relationship metadata and must not call mutating tools from this packet."),
            "AI context uses source-labeled relationship metadata only; hidden writes remain forbidden.",
        ),
    ];

    InfrastructureIntelligenceAlphaPage {
        fixture_metadata: Some(InfrastructureIntelligenceFixtureMetadata {
            name: "infrastructure_source_intelligence_alpha_claimed_connectors".to_string(),
            scenario: "Terraform, Kubernetes, container, CI, and policy connectors expose read-only resource relationships with source, freshness, partiality, and control-plane boundary labels.".to_string(),
            exercised_axes: BTreeMap::from([
                (
                    "connector_kind".to_string(),
                    vec![
                        "terraform_workspace".to_string(),
                        "kubernetes_cluster".to_string(),
                        "container_runtime".to_string(),
                        "ci_provider".to_string(),
                        "policy_engine".to_string(),
                    ],
                ),
                (
                    "consumer_surface".to_string(),
                    vec![
                        "search".to_string(),
                        "review".to_string(),
                        "support".to_string(),
                        "ai_context".to_string(),
                    ],
                ),
            ]),
        }),
        record_kind: INFRASTRUCTURE_INTELLIGENCE_ALPHA_PAGE_RECORD_KIND.to_string(),
        schema_version: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SCHEMA_VERSION,
        shared_contract_ref: INFRASTRUCTURE_INTELLIGENCE_ALPHA_SHARED_CONTRACT_REF.to_string(),
        page_id,
        captured_at: "2026-05-17T16:00:00Z".to_string(),
        contract_refs: InfrastructureIntelligenceContractRefs {
            connected_provider_registry_schema_ref:
                "schemas/providers/connected_provider_registry.schema.json".to_string(),
            provider_object_schema_ref: "schemas/providers/provider_object.schema.json".to_string(),
            target_context_schema_ref: "schemas/runtime/target_context.schema.json".to_string(),
            search_result_identity_schema_ref: "schemas/search/search_result_identity.schema.json"
                .to_string(),
            review_workspace_schema_ref: "schemas/review/review_workspace.schema.json"
                .to_string(),
            support_bundle_schema_ref: "schemas/support/support_bundle.schema.json".to_string(),
            content_integrity_schema_ref: "schemas/security/text_representation_policy.schema.json"
                .to_string(),
            ai_context_schema_ref: "schemas/ai/composer_context_evidence_beta.schema.json"
                .to_string(),
        },
        connectors,
        resources,
        relationships,
        consumer_projections,
        promotion_gate: InfrastructurePromotionGate {
            docs_truth_ref: INFRASTRUCTURE_INTELLIGENCE_DOC_REF.to_string(),
            support_export_ref: "projection.support.infrastructure".to_string(),
            ui_truth_ref: "ui.infrastructure.relationship_view.alpha".to_string(),
            operator_truth_alignment: true,
            promotion_state: InfrastructurePromotionState::PromotionReady,
            gate_summary:
                "Docs, support export, and UI truth all describe a read-only source-intelligence lane."
                    .to_string(),
        },
        support_summary:
            "Infrastructure source-intelligence alpha exposes read-only relationship packets for claimed IaC and operational connectors."
                .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates() {
        let page = seeded_infrastructure_intelligence_alpha_page();
        let report = page.validate();
        assert!(
            report.passed,
            "seeded page must pass: {:#?}",
            report.findings
        );
        assert!(report
            .coverage
            .consumer_surfaces
            .contains(&InfrastructureConsumerSurface::Search));
        assert!(report
            .coverage
            .consumer_surfaces
            .contains(&InfrastructureConsumerSurface::Review));
        assert!(report
            .coverage
            .consumer_surfaces
            .contains(&InfrastructureConsumerSurface::Support));
    }

    #[test]
    fn hidden_provider_writes_fail() {
        let mut page = seeded_infrastructure_intelligence_alpha_page();
        page.connectors[0].hidden_provider_writes_allowed = true;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "infrastructure_intelligence.connector_hidden_provider_writes"
        }));
    }

    #[test]
    fn active_control_plane_authority_fails() {
        let mut page = seeded_infrastructure_intelligence_alpha_page();
        page.resources[0].active_control_plane_mutation_authority = true;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "infrastructure_intelligence.resource_active_mutation_authority"
        }));
    }

    #[test]
    fn unlabelled_partiality_fails() {
        let mut page = seeded_infrastructure_intelligence_alpha_page();
        page.relationships[0].partiality = InfrastructurePartialityClass::PartialRetrieval;
        page.relationships[0].partiality_reason = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "infrastructure_intelligence.relationship_partiality_reason_missing"
        }));
    }

    #[test]
    fn projections_derive_from_same_page() {
        let page = seeded_infrastructure_intelligence_alpha_page();
        let search = page.search_projection();
        let review = page.review_projection();
        let support = page.support_export_projection();
        assert_eq!(search.source_page_id, page.page_id);
        assert_eq!(review.source_page_id, page.page_id);
        assert_eq!(support.source_page_id, page.page_id);
        assert!(support.raw_material_excluded);
        assert!(support.mutation_authority_excluded);
    }

    #[test]
    fn support_export_excludes_raw_material_markers() {
        let page = seeded_infrastructure_intelligence_alpha_page();
        let json = serde_json::to_string(&page.support_export_projection()).expect("serialize");
        assert!(!json.contains("https://"));
        assert!(!json.contains("Bearer "));
        assert!(!json.contains("raw_payload"));
        assert!(!json.contains("kubectl apply"));
    }
}
