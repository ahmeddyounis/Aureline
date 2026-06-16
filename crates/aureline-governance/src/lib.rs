//! Governed schema and record-class registry access.
//!
//! This crate embeds the checked-in schema-family and record-class registries
//! used by trust, support, release, admin, and automation surfaces. It validates
//! that governed payload families name owners, schema versions, consent classes,
//! endpoint classes, record-class bindings, retention posture, lifecycle state,
//! and downgrade rules before downstream code renders or emits those contracts.
//!
//! It also embeds the typed standards/interchange matrix
//! ([`interchange_matrix`]): a machine-consumable projection of the canonical
//! standards register that downstream surfaces consult before claiming
//! open-format or standard compatibility.
//!
//! It embeds the typed interface-freeze register
//! ([`interface_freeze`]): the explicit, gated record of which governed
//! interface surfaces are open, soft-frozen, or hard-frozen for Beta, and what
//! exception classes may move a frozen surface.
//!
//! It embeds the stable telemetry, support-export, and usage-export
//! registry ([`telemetry_support_usage_registry`]): the per-family endpoint-policy
//! truth, consent posture, retention notes, redaction profile references, and M4
//! governance dimensions that make emitted payloads first-class product contracts.
//!
//! Finally it embeds the open/local-boundary and upstream-durability matrix
//! ([`m5_boundary_and_upstream_durability`]): the canonical, inspectable record
//! that freezes, per asset lane, the open-versus-paid boundary posture, the
//! repository-compliance/third-party-import controls, the emergency
//! signing/registry/security authority, and the continuity rules — and narrows a
//! lane the moment any of that thins out.
//!
//! On top of that matrix it embeds the versioned, per-family boundary-manifest
//! register ([`m5_versioned_boundary_manifests`]): the canonical, inspectable record
//! that publishes — per claimed M5 family and per manifest version — which
//! capabilities stay open/local, which may be productized, what guardrails preserve
//! the claim, which residual proprietary/hosted dependencies remain, and the release
//! train each manifest is linked to, narrowing the moment release-link parity, a
//! disclosure, or a guardrail thins out.
//!
//! It embeds the repository-compliance and notice-binding register
//! ([`m5_compliance_and_notice_binding`]): the canonical, inspectable record that
//! publishes — per claimed M5 artifact family, docs pack, and mirrored output — the
//! DCO/CLA contribution-provenance lane truth, the REUSE/SPDX file-level licensing
//! coverage, the third-party notice-inventory state, and the SBOM/notice binding, holding
//! the repository-compliance scan in parity with the user/admin notice/SBOM surface and
//! narrowing the moment provenance, licensing, notices, the SBOM, or the mirror thins out.
//!
//! It embeds the import-provenance and fork-review register
//! ([`m5_import_provenance_and_fork_review`]): the canonical, inspectable record that
//! publishes — per protected-path import used by an M5 family — the import provenance
//! (origin attribution, SPDX license, upstream pin), the update ownership, the divergence
//! profile, the sponsor/fork/replace decision for long-lived forks and single-source
//! imports, and the generated-code generator identity and regeneration path, holding the
//! dependency-health/import scan in parity with the user/admin import surface and narrowing
//! the moment provenance, ownership, a divergence review, a decision, or generated-code
//! provenance thins out.
//!
//! Finally it embeds the critical-upstream health register
//! ([`m5_critical_upstream_health`]): the canonical, inspectable record that publishes — per
//! critical upstream a protected M5 family depends on — the maintainer-health rating, the
//! security posture, the update and review cadence, the license clarity, the replacement
//! feasibility, the ownership, and the sponsor/fork/replace contingency and shiproom escalation
//! required for red-risk or unowned upstreams, holding the upstream-health scan in parity with
//! the governance-dashboard/promotion-packet surface and narrowing the moment maintainer health,
//! security, cadence, license, ownership, or proof thins out — so a red-risk or unowned
//! protected-path dependency cannot widen a stable claim without an approved plan.

#![doc(html_root_url = "https://docs.rs/aureline-governance/0.0.0")]

pub mod interchange_matrix;
pub mod interface_freeze;
pub mod m5_boundary_and_upstream_durability;
pub mod m5_compliance_and_notice_binding;
pub mod m5_critical_upstream_health;
pub mod m5_import_provenance_and_fork_review;
pub mod m5_versioned_boundary_manifests;
pub mod schema_registry;
pub mod telemetry_support_usage_registry;

pub use interchange_matrix::{
    current_standards_interchange_matrix, ExportExpectation, ImportExpectation,
    InterchangeMatrixRow, InterchangeMatrixSummary, InterchangeMatrixViolation,
    StandardsInterchangeMatrix, SupportPosture, STANDARDS_INTERCHANGE_MATRIX_JSON,
    STANDARDS_INTERCHANGE_MATRIX_PATH, STANDARDS_INTERCHANGE_MATRIX_RECORD_KIND,
    STANDARDS_INTERCHANGE_MATRIX_SCHEMA_VERSION,
};
pub use interface_freeze::{
    current_interface_freeze_register, FreezeExceptionClass, FreezeState, InterfaceFreezeRegister,
    InterfaceFreezeRow, InterfaceFreezeSummary, InterfaceFreezeViolation, RecordedFreezeException,
    SurfaceClass, VersionSource, INTERFACE_FREEZE_REGISTER_JSON, INTERFACE_FREEZE_REGISTER_PATH,
    INTERFACE_FREEZE_REGISTER_RECORD_KIND, INTERFACE_FREEZE_REGISTER_SCHEMA_VERSION,
};
pub use m5_boundary_and_upstream_durability::{
    current_m5_boundary_and_upstream_durability, AssetLane, BackupCoverage, BoundaryCutline,
    BoundaryDurabilityMatrix, BoundaryDurabilityRow, BoundaryPosture, BoundaryReuseRow,
    ContinuityCoverage, ControlBinding, ControlDimension, ControlState, CriticalUpstream,
    DurabilityReason, DurabilityState, EmergencyAuthority, FreshnessSlo, FreshnessSloState,
    GovernanceRule, LifecycleLabel, MatrixAction, MatrixSummary, MatrixViolation, OwnerSignoff,
    ProofPacket, Publication, PublicationDecision, RiskClass, SignerQuorum, SourceContractRefs,
    SupportClass, Waiver, M5_BOUNDARY_AND_UPSTREAM_DURABILITY_JSON,
    M5_BOUNDARY_AND_UPSTREAM_DURABILITY_PATH, M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND,
    M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION,
};
pub use m5_compliance_and_notice_binding::{
    current_m5_compliance_and_notice_binding, ClaState, ComplianceAction, ComplianceControl,
    ComplianceCutline, CompliancePosture, ComplianceReason, ComplianceRecord, ComplianceRegister,
    ComplianceReuseRow, ComplianceRule, ComplianceState, ComplianceSummary, ContributionProvenance,
    ControlDimension as ComplianceControlDimension, ControlState as ComplianceControlState,
    DcoState, LicensingCoverage, MirrorBinding, NoticeInventory, NoticeState,
    Publication as ComplianceNoticePublication,
    PublicationDecision as ComplianceNoticePublicationDecision,
    RegisterViolation as ComplianceRegisterViolation, SbomBindingState, SbomFormat,
    SbomNoticeBinding, ScanSurfaceParity, ScopeKind,
    SourceContractRefs as ComplianceNoticeSourceContractRefs,
    M5_COMPLIANCE_AND_NOTICE_BINDING_JSON, M5_COMPLIANCE_AND_NOTICE_BINDING_PATH,
    M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND, M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION,
};
pub use m5_critical_upstream_health::{
    current_m5_critical_upstream_health, ContingencyDisposition, ContingencyPlan, ContingencyState,
    ControlDimension as UpstreamHealthControlDimension, ControlState as UpstreamHealthControlState,
    CriticalUpstreamHealthRegister, EscalationState, HealthAction, HealthControl, HealthCutline,
    HealthGrade, HealthReason, HealthRule, HealthState, HealthSummary, LicenseClarity,
    LicenseProfile, MaintainerHealth, MaintainerRating, OwnershipState as UpstreamOwnershipState,
    Posture as UpstreamHealthPosture, Publication as UpstreamHealthPublication,
    PublicationDecision as UpstreamHealthPublicationDecision,
    RegisterViolation as UpstreamHealthRegisterViolation, ReplacementFeasibility, ReviewCadence,
    ReviewCadenceState, ScanSurfaceParity as UpstreamHealthScanSurfaceParity, SecurityPosture,
    SecurityProfile, ShiproomEscalation, SourceContractRefs as UpstreamHealthSourceContractRefs,
    UpdateCadence, UpdateCadenceProfile, UpstreamHealthRecord, UpstreamHealthReuseRow,
    UpstreamKind, UpstreamOwnership, M5_CRITICAL_UPSTREAM_HEALTH_JSON,
    M5_CRITICAL_UPSTREAM_HEALTH_PATH, M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND,
    M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION,
};
pub use m5_import_provenance_and_fork_review::{
    current_m5_import_provenance_and_fork_review, ControlDimension as ImportControlDimension,
    ControlState as ImportControlState, DecisionDisposition, DecisionRecord, DecisionState,
    DivergenceProfile, DivergenceReviewState, DivergenceState, GeneratorProvenance, ImportAction,
    ImportControl, ImportCutline, ImportKind, ImportProvenance, ImportReason, ImportRecord,
    ImportRegister, ImportReuseRow, ImportRule, ImportState, ImportSummary, LicenseState,
    ManifestSurfaceParity, OriginState, OwnershipState, Posture as ImportPosture,
    Publication as ImportPublication, PublicationDecision as ImportPublicationDecision,
    RegisterViolation as ImportRegisterViolation, SourceContractRefs as ImportSourceContractRefs,
    UpdateOwnership, UpstreamPinState, M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_JSON,
    M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_PATH, M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND,
    M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION,
};
pub use m5_versioned_boundary_manifests::{
    current_m5_versioned_boundary_manifests, BoundaryManifest, BoundaryManifestRegister,
    DependencyClass, Guardrail, GuardrailKind, GuardrailRule, GuardrailState, LaneDisposition,
    M5Family, ManifestAction, ManifestCutline, ManifestLaneEntry, ManifestReason, ManifestReuseRow,
    ManifestState, ManifestSummary, Publication as VersionedBoundaryPublication,
    PublicationDecision as VersionedBoundaryPublicationDecision, RegisterViolation, ReleaseLink,
    ReleaseLinkParity, ReleaseLinkState, ResidualDependency,
    SourceContractRefs as VersionedBoundarySourceContractRefs,
    M5_VERSIONED_BOUNDARY_MANIFESTS_JSON, M5_VERSIONED_BOUNDARY_MANIFESTS_PATH,
    M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND, M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION,
};
pub use schema_registry::{
    load_default_record_class_registry, load_default_schema_registry, validate_default_registries,
    DowngradeRule, GovernanceSurfaceClass, GovernedRecordClassRegistry, GovernedRecordClassRow,
    GovernedSchemaRegistry, GovernedSchemaRow, PacketVersionSupport, SchemaRegistryError,
    SchemaRegistryValidationReport, SeparationRule, SurfaceProjection, SurfaceSchemaRow,
    GOVERNED_RECORD_CLASS_REGISTRY_JSON, GOVERNED_RECORD_CLASS_REGISTRY_PATH,
    GOVERNED_SCHEMA_REGISTRY_JSON, GOVERNED_SCHEMA_REGISTRY_PATH,
};
pub use telemetry_support_usage_registry::{
    current_registry as current_telemetry_support_usage_registry,
    load_registry as load_telemetry_support_usage_registry,
    validate_registry as validate_telemetry_support_usage_registry, ContextClass,
    ContextEndpointPolicyRow, DeprecatedFieldHandling, EndpointPolicyTruth, PartialOutcomeMarker,
    RegistryLoadError, RegistryViolation, TelemetrySupportUsageRegistry, TelemetrySupportUsageRow,
    TelemetrySupportUsageSummary, TELEMETRY_SUPPORT_USAGE_REGISTRY_JSON,
    TELEMETRY_SUPPORT_USAGE_REGISTRY_PATH, TELEMETRY_SUPPORT_USAGE_REGISTRY_RECORD_KIND,
    TELEMETRY_SUPPORT_USAGE_REGISTRY_SCHEMA_VERSION,
};
