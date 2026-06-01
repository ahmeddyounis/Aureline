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
pub mod invocation;
pub mod registry;
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
pub use invocation::{
    CommandInvocationSession, CommandResultPacketRecord, InvocationSessionPacketRecord,
};
pub use registry::{
    CommandDescriptorPublicContractRecord, CommandPreviewGateMetadata, CommandRegistry,
    CommandRegistryEntryRecord, CommandRegistrySeedRecord, RegistryError,
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
