//! Canonical command-descriptor types and the seeded command registry.
//!
//! This crate is the runtime counterpart to the frozen command contracts in:
//!
//! - `docs/commands/command_descriptor_contract.md`
//! - `schemas/commands/command_descriptor.schema.json`
//! - `schemas/commands/command_registry_entry.schema.json`
//! - `artifacts/commands/command_registry_seed.yaml`
//!
//! It intentionally models the descriptor/registry as product objects (stable
//! IDs, lifecycle, capability classes, and structured enablement reasons) so
//! every consuming surface can project from the same canonical source.

#![doc(html_root_url = "https://docs.rs/aureline-commands/0.0.0")]

pub mod alpha;
pub mod authority;
pub mod descriptor;
pub mod enablement;
pub mod finalize_command_parity;
pub mod finalize_traffic_origin_and_exposure_chips;
pub mod harden_high_risk_command;
pub mod invocation;
pub mod registry;
pub mod stabilize_client_origin_route_class;
pub mod stabilize_command_contract;

pub use alpha::{
    alpha_command_registry, AlphaCommandClaimRecord, AlphaCommandRegistryRecord,
    AlphaDiscoverabilityConsumerRef, AlphaRegistryError, AlphaSurfaceParityRecord,
};
pub use authority::{
    CommandAuthorityProjection, CommandAuthorityScenarioRecord, InvocationLineageRecord,
    SurfaceInvocationRecord,
};
pub use descriptor::{
    CommandDescriptorRecord, CommandId, CommandOriginMetadata, CommandRevisionRef, OpaqueId,
    PolicyContext, RepairHookRef,
};
pub use enablement::{
    CommandEnablementContext, DisabledReasonCode, DisabledReasonRecord, EnablementDecisionClass,
    EnablementSnapshot, PreflightDecision, PreflightDecisionClass,
};
pub use finalize_command_parity::{
    current_finalize_command_parity_export, ClearHistoryRuleClass,
    CommandParityFinalizationArtifactError, CommandParityFinalizationPacket,
    CommandParityFinalizationPacketInput, CommandParityFinalizationViolation,
    DisabledReasonChipRow, DiscoverabilityProjectionRow, DiscoverabilityRecord,
    DiscoverabilitySurfaceClass, HistoryPolicyClass, ModifierActionClass,
    ModifierActionFooterContract, QuerySessionPrivacyContract, RedactionPostureClass,
    FINALIZE_COMMAND_PARITY_ARTIFACT_REF, FINALIZE_COMMAND_PARITY_DESCRIPTOR_CONTRACT_REF,
    FINALIZE_COMMAND_PARITY_DISCOVERABILITY_CONTRACT_REF, FINALIZE_COMMAND_PARITY_DOC_REF,
    FINALIZE_COMMAND_PARITY_FIXTURE_DIR, FINALIZE_COMMAND_PARITY_PALETTE_ROW_CONTRACT_REF,
    FINALIZE_COMMAND_PARITY_QUERY_SESSION_CONTRACT_REF, FINALIZE_COMMAND_PARITY_RECORD_KIND,
    FINALIZE_COMMAND_PARITY_SCHEMA_REF, FINALIZE_COMMAND_PARITY_SCHEMA_VERSION,
    FINALIZE_COMMAND_PARITY_SUMMARY_REF,
};
pub use finalize_traffic_origin_and_exposure_chips::{
    current_traffic_origin_exposure_chips_export, ExposureChipClass,
    FinalizeTrafficOriginExposureChipsArtifactError, FinalizeTrafficOriginExposureChipsPacket,
    FinalizeTrafficOriginExposureChipsPacketInput, FinalizeTrafficOriginExposureChipsViolation,
    PortExplainabilityRecord, PortProtocolClass, PublishTargetClass,
    PublishTargetExplainabilityRecord, TrafficOriginChipRow, TrafficOriginClass,
    TunnelExplainabilityRecord, TunnelKindClass,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_ARTIFACT_REF,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DESCRIPTOR_CONTRACT_REF,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DOC_REF,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_FIXTURE_DIR,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_PARITY_CONTRACT_REF,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_RECORD_KIND,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_REF,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_VERSION,
    FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SUMMARY_REF,
};
pub use harden_high_risk_command::{
    current_high_risk_command_hardening_export, ApprovalAuthorityClass, ApprovalLineageContract,
    ApprovalLineageRecord, ApprovalStepClass, HighRiskClass, HighRiskCommandHardeningArtifactError,
    HighRiskCommandHardeningPacket, HighRiskCommandHardeningPacketInput,
    HighRiskCommandHardeningViolation, HighRiskPreviewContract, HighRiskSurfaceParityRow,
    PreviewRequirementClass, RollbackHandleContract, RollbackPostureClass,
    HARDEN_HIGH_RISK_COMMAND_ARTIFACT_REF, HARDEN_HIGH_RISK_COMMAND_DESCRIPTOR_CONTRACT_REF,
    HARDEN_HIGH_RISK_COMMAND_DOC_REF, HARDEN_HIGH_RISK_COMMAND_FIXTURE_DIR,
    HARDEN_HIGH_RISK_COMMAND_PARITY_CONTRACT_REF, HARDEN_HIGH_RISK_COMMAND_RECORD_KIND,
    HARDEN_HIGH_RISK_COMMAND_SCHEMA_REF, HARDEN_HIGH_RISK_COMMAND_SCHEMA_VERSION,
    HARDEN_HIGH_RISK_COMMAND_SUMMARY_REF,
};
pub use invocation::{
    CommandInvocationSession, CommandResultPacketRecord, InvocationSessionPacketRecord,
};
pub use registry::{
    CommandDescriptorPublicContractRecord, CommandPreviewGateMetadata, CommandRegistry,
    CommandRegistryEntryRecord, CommandRegistrySeedRecord, RegistryError,
};
pub use stabilize_client_origin_route_class::{
    current_client_origin_route_class_export, ActionRouteClass, ApprovalScopeRecord,
    CapabilityBoundaryClass, CapabilityRouteInspector, ClientOriginClass,
    ClientOriginRouteClassArtifactError, ClientOriginRouteClassPacket,
    ClientOriginRouteClassPacketInput, ClientOriginRouteClassViolation, InspectorSurfaceRow,
    RevalidationTriggerClass, TargetContextClass,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_ARTIFACT_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DESCRIPTOR_CONTRACT_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DOC_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_FIXTURE_DIR,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_PARITY_CONTRACT_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_RECORD_KIND,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_VERSION,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SUMMARY_REF,
};
pub use stabilize_command_contract::{
    current_stable_command_contract_stabilization_export, CommandContractEvidenceExport,
    CommandContractStabilizationArtifactError, CommandContractStabilizationPacket,
    CommandContractStabilizationPacketInput, CommandContractStabilizationViolation,
    CommandResultCodeClass, CommandSurfaceClass, DisabledReasonCaseClass, DisabledReasonCaseRow,
    PaletteActionClass, PaletteDiagnosticsContract, ResultContractStabilization,
    StableContractRefs, StableDescriptorFieldClass, StableDescriptorFieldRow, SurfaceParityRow,
    SurfaceQualificationClass, STABILIZE_COMMAND_CONTRACT_ARTIFACT_REF,
    STABILIZE_COMMAND_CONTRACT_DESCRIPTOR_CONTRACT_REF, STABILIZE_COMMAND_CONTRACT_DOC_REF,
    STABILIZE_COMMAND_CONTRACT_FIXTURE_DIR, STABILIZE_COMMAND_CONTRACT_PARITY_CONTRACT_REF,
    STABILIZE_COMMAND_CONTRACT_RECORD_KIND, STABILIZE_COMMAND_CONTRACT_SCHEMA_REF,
    STABILIZE_COMMAND_CONTRACT_SCHEMA_VERSION, STABILIZE_COMMAND_CONTRACT_SUMMARY_REF,
};
