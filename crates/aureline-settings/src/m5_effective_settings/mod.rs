//! Effective-settings parity for M5-added settings families.
//!
//! The module mints one governed record that brings new M5 settings families
//! — notebooks, data/API, profiler, bundle, sync, and companion — into parity
//! with the stable effective-settings resolver. Each row names its stable
//! setting id, the winning value and the scope that won, the shadow chain that
//! lost, the restart posture, the validation state, and any lifecycle-sensitive
//! dependency marker. High-impact rows carry scope-explicit, checkpointed write
//! previews; policy-locked rows can never advertise a write that would silently
//! win. The settings UI, CLI inspect, help links, policy explainers, and support
//! bundles consume the same record instead of cloning shadow status text.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{m5_effective_settings_corpus, M5EffectiveSettingsScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, BuildError, EffectiveSettingsClaim, EffectiveSettingsPillars,
    EffectiveSettingsQualification, HighImpactClass, LifecycleDependencyKind,
    LifecycleDependencyMarker, M5EffectiveSettingRow, M5EffectiveSettingsCertification,
    M5EffectiveSettingsInput, M5SettingFamily, NarrowingReason, PolicyLockState, RestartPosture,
    RowTrust, ScopeExplicitWritePreview, SettingScope, ShadowReason, ShadowedCandidate,
    SurfaceClass, SurfaceTruthRow, ValidationState, WinningValue, WriteEffect,
    M5_EFFECTIVE_SETTINGS_RECORD_KIND, M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION,
    M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF,
};
