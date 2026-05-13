//! Workspace-scoped language-host supervision.
//!
//! This module owns the runtime supervision seed for language-server hosts.
//! Hosts are keyed by workspace, root, and language id; lifecycle changes
//! project to visible states before the language router consumes them.

mod records;
mod supervisor;

pub use records::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostIdentity,
    LanguageHostRuntimeStateClass, LanguageHostSnapshot, LanguageHostSupervisorEvent,
    LanguageHostSupportPacket, LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
};
pub use supervisor::{
    LanguageHostLaunchSpec, LanguageHostScopeKey, LanguageHostSupervisor,
    LanguageHostSupervisorConfig, LanguageHostSupervisorError,
};
