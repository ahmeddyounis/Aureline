//! Effective-settings, policy explainability, and admin-distribution audit
//! parity for M5-added settings families.
//!
//! The module mints one governed record that brings new M5 settings families
//! — notebooks, data/API, profiler, bundle, sync, and companion — into parity
//! with the stable effective-settings resolver. Each row names its stable
//! setting id, the winning value and the scope that won, the shadow chain that
//! lost, the restart posture, the validation state, the active
//! source/effective/live projection, the locked/constrained write explanation,
//! and any lifecycle-sensitive dependency marker. High-impact rows carry
//! scope-explicit, checkpointed write previews; constrained rows explain the
//! governing bundle or scope and what still works locally; and the record
//! includes an admin-distribution audit slice that shows where the active bundle
//! came from and when it last applied. The settings UI, CLI inspect, Help/About,
//! policy explainers, admin audit, and support bundles consume the same record
//! instead of cloning shadow status text.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{m5_effective_settings_corpus, M5EffectiveSettingsScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, AdminDistributionAuditRow, AuditFreshnessState, BuildError,
    EffectiveSettingsClaim, EffectiveSettingsPillars, EffectiveSettingsQualification,
    EffectiveValueReviewSheet, HighImpactClass, LifecycleDependencyKind, LifecycleDependencyMarker,
    M5EffectiveSettingRow, M5EffectiveSettingsCertification, M5EffectiveSettingsInput,
    M5SettingFamily, NarrowingReason, PolicyConstraintState, PolicyDistributionSource,
    PolicyLockState, ProjectionMode, RestartPosture, ReviewAction, ReviewExportPosture, RowTrust,
    ScopeExplicitWritePreview, SettingScope, ShadowReason, ShadowedCandidate, SurfaceClass,
    SurfaceTruthRow, ValidationState, WinningValue, WriteConstraintExplanation, WriteEffect,
    M5_EFFECTIVE_SETTINGS_RECORD_KIND, M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION,
    M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF,
};
