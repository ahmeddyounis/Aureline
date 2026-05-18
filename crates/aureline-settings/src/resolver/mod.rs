//! Effective-settings resolver: precedence engine, locked-write
//! flow, and shadow-chain inspection.

pub mod effective;
pub mod engine;
pub mod lock;

pub use effective::{
    EffectiveCapabilityDependency, EffectiveControlStack, EffectiveLastWritten,
    EffectiveSettingRecord, EffectiveValue, ShadowChainEntry, ShadowRelation,
};
pub use engine::{
    CapabilityState, EffectiveSettingsResolver, PolicyConstraint, ResolveError, ScopeOverlay,
    WriteAttemptOutcome,
};
pub use lock::{LockReason, LockState, WriteDenialReason, WriteIntent};
