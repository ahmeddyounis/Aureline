//! Stable certification for the **settings UI**: the effective-configuration
//! inspector, the setting-definition registry exposure, the shadow contributor
//! chain, scope-explicit previewable writes, the profile-switch review, and
//! cross-surface explanation parity.
//!
//! This lane mints one governed [`SettingsUiCertification`] per settings
//! posture. The record proves that every visible setting on a claimed-stable
//! settings surface resolves through **one** effective-setting record — so the
//! desktop UI, the CLI / headless inspect, Help/About, the diagnostics or
//! support export, and migration / import review all explain the same value,
//! winning scope, lock reason, and restart posture instead of cloning prose. It
//! is a genuine projection of the live settings runtime: it ingests the seeded
//! [`crate::schema::SchemaRegistry`], resolves effective values through the
//! [`crate::resolver::EffectiveSettingsResolver`], and projects per-setting
//! records and previewable writes through [`crate::inspector`], so the
//! certification can never drift from what the resolver actually resolves.
//!
//! A [`SettingsUiCertification`] binds, for one posture:
//!
//! - **One effective-setting record per visible setting** — value, winning
//!   scope, lock state / reason, restart posture, and the setting-definition
//!   registry fields (declared type, allowed scopes, default, migration aliases,
//!   sensitivity, redaction, capability / lifecycle dependencies, docs refs).
//! - **A shadow contributor chain** — each row's chain classifies the built-in
//!   default, channel default, active profile, temporary profile, machine-local,
//!   synced, workspace, folder / language overrides, and the policy-owned
//!   ceiling, so a setting never implies one flat value.
//! - **Scope-explicit previewable writes** — target scope, target artifact,
//!   blocked-write reason with a Diagnostics Center entry point when denied,
//!   restart impact, and any experiment / lifecycle dependency, before commit.
//! - **Cross-surface parity** — one row per desktop UI, CLI inspect, Help/About,
//!   diagnostics / support export, and migration / import review, each proving it
//!   consumes the shared record rather than cloning prose.
//! - **A profile-switch review** — immediate changes, restart-required deltas,
//!   excluded machine-local state, narrowing effects, and whether a rollback
//!   checkpoint is created before apply.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `finalize-the-settings-ui-with-effective-value-inspector`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finalize-the-settings-ui-with-effective-value-inspector.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live settings runtime, and pinned on disk under
//!   `fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md`.

pub mod corpus;
pub mod model;

pub use corpus::{settings_ui_corpus, SettingsUiScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure, BuildError,
    CertificationClaimCeiling, CertificationInput, CertificationNarrowingReason,
    CertificationPillars, CertificationQualification, CertificationRecoveryAction,
    CertificationUpstream, ContributorClass, EffectiveSettingRow, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, PreviewableWriteRow, ProfileSwitchChange,
    ProfileSwitchReview, RecoveryActionRole, RecoveryRouteRecord, RouteSurface,
    SettingsUiCertification, ShadowContributorRow, StableClaimClass, SurfaceClass,
    SurfaceParityRow, SETTINGS_UI_NOTICE, SETTINGS_UI_RECORD_KIND, SETTINGS_UI_SCHEMA_VERSION,
    SETTINGS_UI_SHARED_CONTRACT_REF,
};
