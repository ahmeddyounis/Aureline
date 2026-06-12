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
pub mod automation;
pub mod descriptor;
pub mod enablement;
pub mod finalize_command_parity;
pub mod finalize_traffic_origin_and_exposure_chips;
pub mod freeze_command_governance_contract;
pub mod harden_ai_and_command_support_export_parity_audit;
pub mod harden_high_risk_command;
pub mod invocation;
pub mod m5_command_governance;
pub mod publish_capability_route_inspector;
pub mod registry;
pub mod stabilize_client_origin_route_class;
pub mod stabilize_command_contract;
pub mod stabilize_command_discoverability_records_alias_history;

pub use alpha::{
    alpha_command_registry, AlphaCommandClaimRecord, AlphaCommandRegistryRecord,
    AlphaDiscoverabilityConsumerRef, AlphaRegistryError, AlphaSurfaceParityRecord,
};
pub use authority::{
    CommandAuthorityProjection, CommandAuthorityScenarioRecord, InvocationLineageRecord,
    SurfaceInvocationRecord,
};
pub use automation::{
    automation_display_labels, current_safe_automation_qualification_export, labels_include,
    why_not_automatable_reason, AutomationArtifactAuthorityClass, AutomationCapabilityClass,
    AutomationClassManifestRow, AutomationIdempotencyHintClass, AutomationLifecycleLabelClass,
    AutomationManifestExportImportContract, AutomationObjectClass, AutomationPreviewPolicyClass,
    AutomationProvenanceClass, AutomationStorageFormClass, AutomationSurfaceActionClass,
    AutomationTrustRequirementClass, CommandAutomationSurfaceContract, ControlledAutomationLabel,
    SafeAutomationEvidenceExport, SafeAutomationQualificationArtifactError,
    SafeAutomationQualificationPacket, SafeAutomationQualificationPacketInput,
    SafeAutomationQualificationViolation, SAFE_AUTOMATION_MANIFEST_SCHEMA_REF,
    SAFE_AUTOMATION_MATRIX_REF, SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF,
    SAFE_AUTOMATION_QUALIFICATION_RECORD_KIND, SAFE_AUTOMATION_QUALIFICATION_SCHEMA_VERSION,
    SAFE_AUTOMATION_RECIPE_MACRO_CONTRACT_REF, SAFE_AUTOMATION_SHAREABILITY_CONTRACT_REF,
    SAFE_AUTOMATION_SUPPORT_EXPORT_REF,
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
pub use freeze_command_governance_contract::{
    current_frozen_command_governance_contract_export, CommandDescriptorFieldClass,
    CommandGovernanceContractArtifactError, CommandGovernanceContractPacket,
    CommandGovernanceContractPacketInput, CommandGovernanceContractViolation,
    CommandGovernanceSurfaceClass, DescriptorFieldRow, DowngradeRuleRow, DowngradeTriggerClass,
    EvidenceFreshnessClass, FeatureFamilyClass, FeatureFamilyGovernanceRow, GovernanceClaimClass,
    GovernanceContractRefs, GovernanceEvidenceExport, InvocationSessionFieldClass,
    InvocationSessionFieldRow, LifecycleDependencyClass, LifecycleDependencyRow,
    LifecycleDependencyVocabularyRow, PublicationConsumerSurfaceClass, ResultPacketFieldClass,
    ResultPacketFieldRow, SurfaceGovernanceRow, FREEZE_COMMAND_GOVERNANCE_CAPABILITY_REGISTRY_REF,
    FREEZE_COMMAND_GOVERNANCE_CLAIM_PUBLICATION_REF,
    FREEZE_COMMAND_GOVERNANCE_CONTRACT_ARTIFACT_REF, FREEZE_COMMAND_GOVERNANCE_CONTRACT_DOC_REF,
    FREEZE_COMMAND_GOVERNANCE_CONTRACT_FIXTURE_DIR, FREEZE_COMMAND_GOVERNANCE_CONTRACT_RECORD_KIND,
    FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_REF,
    FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_VERSION,
    FREEZE_COMMAND_GOVERNANCE_CONTRACT_SUMMARY_REF,
    FREEZE_COMMAND_GOVERNANCE_DESCRIPTOR_CONTRACT_REF,
    FREEZE_COMMAND_GOVERNANCE_DISABLED_REASON_REF,
    FREEZE_COMMAND_GOVERNANCE_INVOCATION_PARITY_CONTRACT_REF,
    FREEZE_COMMAND_GOVERNANCE_LIFECYCLE_ADR_REF,
};
pub use harden_ai_and_command_support_export_parity_audit::{
    current_harden_ai_and_command_support_export_parity_audit_export,
    AiAndCommandSupportExportParityAuditArtifactError,
    AiAndCommandSupportExportParityAuditViolation, AuditLineageContract,
    AuditLineageRequirementClass, ExportParityContract, ExportParityRequirementClass,
    HardenAiAndCommandSupportExportParityAuditPacket,
    HardenAiAndCommandSupportExportParityAuditPacketInput, ShiproomInclusionClass,
    ShiproomInclusionContract, SupportExportParityRow, SupportExportSurfaceClass,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_ARTIFACT_REF,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DESCRIPTOR_CONTRACT_REF,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DOC_REF,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_FIXTURE_DIR,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_PARITY_CONTRACT_REF,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_RECORD_KIND,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_REF,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_VERSION,
    HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SUMMARY_REF,
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
pub use m5_command_governance::{
    current_m5_command_governance_export, seeded_m5_command_governance_packet,
    validate_m5_command_governance_packet, M5ApprovalModelClass, M5ApprovalParityPacketRecord,
    M5CommandOutcomeProjectionRow,
    M5CommandGovernancePacket, M5CommandGovernanceRow, M5CommandGovernanceSummary,
    M5CommandGovernanceSupportExport, M5CommandGovernanceValidationError,
    M5CopySafeIntrospectionRecord, M5DisabledReasonFamilyClass, M5DisabledReasonPacketRecord,
    M5GovernanceSurfaceClass, M5InvocationPreviewParityRecord, M5ResultArtifactProjectionRecord,
    M5ResultExecutionProfileClass, M5ResultOutcomeClass, M5ResultPacketGovernanceRecord,
    M5RoutePostureClass, M5SurfaceGovernanceRow, M5_COMMAND_GOVERNANCE_DOC_REF,
    M5_COMMAND_GOVERNANCE_FIXTURE_DIR, M5_COMMAND_GOVERNANCE_PACKET_ID,
    M5_COMMAND_GOVERNANCE_RECORD_KIND, M5_COMMAND_GOVERNANCE_SCHEMA_REF,
    M5_COMMAND_GOVERNANCE_SCHEMA_VERSION, M5_COMMAND_GOVERNANCE_SUMMARY_REF,
    M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID, M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_REF,
};
pub use publish_capability_route_inspector::{
    current_capability_route_inspector_export, CapabilityRouteInspectorArtifactError,
    CapabilityRouteInspectorPacket, CapabilityRouteInspectorPacketInput,
    CapabilityRouteInspectorViolation, DriftClass, FlowClass, FlowPublicationRecord,
    KeyboardReachabilityRecord, LineagePreservationRecord, PublicationSurfaceRow,
    ReapprovalPolicyRecord, CAPABILITY_ROUTE_INSPECTOR_ARTIFACT_REF,
    CAPABILITY_ROUTE_INSPECTOR_DESCRIPTOR_CONTRACT_REF, CAPABILITY_ROUTE_INSPECTOR_DOC_REF,
    CAPABILITY_ROUTE_INSPECTOR_FIXTURE_DIR, CAPABILITY_ROUTE_INSPECTOR_PARITY_CONTRACT_REF,
    CAPABILITY_ROUTE_INSPECTOR_RECORD_KIND, CAPABILITY_ROUTE_INSPECTOR_SCHEMA_REF,
    CAPABILITY_ROUTE_INSPECTOR_SCHEMA_VERSION, CAPABILITY_ROUTE_INSPECTOR_SUMMARY_REF,
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
    RevalidationTriggerClass, TargetContextClass, STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_ARTIFACT_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DESCRIPTOR_CONTRACT_REF,
    STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DOC_REF, STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_FIXTURE_DIR,
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
pub use stabilize_command_discoverability_records_alias_history::{
    current_command_discoverability_export, DiscoverabilityAccessibilityRecord,
    DiscoverabilityAliasRecord, DiscoverabilityAutomationSupportClass,
    DiscoverabilityCurrentKeybindingRecord, DiscoverabilityProjectionRefs,
    DiscoverabilityQuerySessionPolicyRecord, DiscoverabilitySupportArtifactError,
    DiscoverabilitySupportPacket, DiscoverabilitySupportViolation,
    DiscoverabilitySurfaceClass as CommandDiscoverabilitySurfaceClass,
    DiscoverabilitySurfaceProjectionRow, ProtectedCommandDiscoverabilityRecord,
    QuerySessionProviderClass, QuerySessionSyncPostureClass, QuerySessionTextMaterialClass,
    STABILIZE_COMMAND_DISCOVERABILITY_ARTIFACT_REF, STABILIZE_COMMAND_DISCOVERABILITY_DOC_REF,
    STABILIZE_COMMAND_DISCOVERABILITY_FIXTURE_DIR,
    STABILIZE_COMMAND_DISCOVERABILITY_QUERY_SESSION_SCHEMA_REF,
    STABILIZE_COMMAND_DISCOVERABILITY_RECORD_KIND, STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_REF,
    STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_VERSION,
    STABILIZE_COMMAND_DISCOVERABILITY_SUMMARY_REF,
};
