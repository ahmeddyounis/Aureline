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

pub mod descriptor;
pub mod invocation;
pub mod registry;

pub use descriptor::{
    CommandDescriptorRecord, CommandId, CommandRevisionRef, OpaqueId, PolicyContext, RepairHookRef,
};
pub use invocation::{
    CommandInvocationSession, CommandResultPacketRecord, InvocationSessionPacketRecord,
};
pub use registry::{
    CommandRegistry, CommandRegistryEntryRecord, CommandRegistrySeedRecord, EnablementSnapshot,
    RegistryError,
};
