//! Effective-settings schema registry, precedence engine, and
//! locked-write flow.
//!
//! This crate is the canonical truth source for settings shape and
//! resolution. Surfaces (settings UI, CLI inspect, support export,
//! shell readiness consumers) read effective values, shadow chains,
//! and lock reasons through this crate; they do not invent private
//! "configuration" reads against the filesystem or environment.
//!
//! Two layers:
//!
//! - [`schema`] — stable [`SettingDefinition`](schema::SettingDefinition)
//!   rows: `setting_id`, value type, default, allowed scopes,
//!   aliases, migrations, restart posture, lifecycle, redaction, and
//!   capability dependencies. The
//!   [`SchemaRegistry`](schema::SchemaRegistry) is the catalog of
//!   record; [`SchemaRegistry::with_seed_catalog`](schema::SchemaRegistry::with_seed_catalog)
//!   ships the small seed used by protected dogfood walks.
//! - [`resolver`] — the precedence engine and the locked-write
//!   flow. Given the registry plus a stack of per-scope overlays,
//!   [`EffectiveSettingsResolver::resolve`](resolver::EffectiveSettingsResolver::resolve)
//!   returns the [`EffectiveValue`](resolver::EffectiveValue) with
//!   the shadow chain, the lock state, and any active policy
//!   ceiling. [`EffectiveSettingsResolver::attempt_write`](resolver::EffectiveSettingsResolver::attempt_write)
//!   returns a typed [`WriteIntent`](resolver::WriteIntent) /
//!   [`WriteDenialReason`](resolver::WriteDenialReason) outcome
//!   without ever silently dropping a denied write.
//!
//! The reviewer-facing landing page is
//! `docs/settings/effective_settings_contract.md`.

#![doc(html_root_url = "https://docs.rs/aureline-settings/0.0.0")]

pub mod inspector;
pub mod resolver;
pub mod schema;

pub use resolver::{
    EffectiveSettingsResolver, EffectiveValue, LockReason, LockState, PolicyConstraint,
    ResolveError, ScopeOverlay, ShadowChainEntry, ShadowRelation, WriteAttemptOutcome,
    WriteDenialReason, WriteIntent,
};
pub use schema::{
    AliasDirection, CapabilityDependency, CapabilityDependencyKind, LifecycleLabel, MigrationRule,
    MigrationTransformClass, PreviewClass, RedactionClass, RestartPosture, SchemaRegistry,
    SchemaRegistryError, SensitivityClass, SettingAlias, SettingDefinition, SettingScope,
    SettingValue, SettingValueType, ValueValidationError,
};
