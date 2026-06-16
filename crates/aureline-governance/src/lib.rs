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

#![doc(html_root_url = "https://docs.rs/aureline-governance/0.0.0")]

pub mod interchange_matrix;
pub mod interface_freeze;
pub mod m5_boundary_and_upstream_durability;
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
