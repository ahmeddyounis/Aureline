//! Stable certification for **sync / device-registry truth, conflict review,
//! device participation state, profile portability, and support-export parity**.
//!
//! This lane mints one governed [`SyncDeviceRegistryCertification`] per sync
//! posture. The record proves that, for one posture, device participation,
//! field-aware conflict review, snapshot-class provenance, local-authoritative
//! fallback, the secret boundary, REL-SYNC-009 merge precedence, and
//! profile-roaming / offboarding truth all resolve through **one** record — so
//! the desktop device-and-sync surface, the CLI / headless inspect, Help/About,
//! the support export, and the admin device-registry view all explain the same
//! sync truth instead of cloning prose. It is a genuine projection of the live
//! settings runtime: it ingests the seeded [`crate::schema::SchemaRegistry`],
//! resolves conflicts through the beta [`crate::sync`] path (itself a projection
//! of [`crate::inspector::conflict`] and the
//! [`crate::resolver::EffectiveSettingsResolver`]), and layers the field-aware
//! outcome class, merge class, snapshot provenance, and secret boundary on top,
//! so the certification can never drift from what the resolver actually
//! resolves.
//!
//! A [`SyncDeviceRegistryCertification`] binds, for one posture:
//!
//! - **Device participation truth** — each device exposes a stable identity,
//!   participation state, profile durability, last successful sync, selected
//!   scope set, conflict class, retained rollback checkpoint, and
//!   local-authoritative fallback posture, inspectable without a mutating action.
//! - **Field-aware conflict review** — each conflict classifies exact-match,
//!   translated, partial, stale-remote, policy-locked, or local-authoritative,
//!   names the REL-SYNC-009 merge class, and protects any overwrite with a change
//!   preview and a rollback checkpoint before apply.
//! - **Snapshot-class provenance** — local rollback checkpoint, portable profile
//!   export, managed sync snapshot, and support recovery manifest, each with
//!   included / excluded state classes, producer version, integrity hash, source
//!   provenance, and local-authoritative fallback.
//! - **The secret boundary** — dirty-buffer journals and secret material never
//!   cross the sync or export lane; only reference-only metadata is allowed.
//! - **Profile-roaming / offboarding truth** — latest successful sync manifest,
//!   extension inventory pointer, remaining-retention timeline, and local
//!   authority retained even when managed sync is unavailable.
//! - **Cross-surface parity** across the desktop UI, CLI inspect, Help/About,
//!   support export, and admin device-registry view.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `ship-sync-device-registry-conflict-review-and-support`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/ship-sync-device-registry-conflict-review-and-support.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live settings runtime, and pinned on disk under
//!   `fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/`.
//!
//! The contract narrative is
//! `docs/ux/m4/ship-sync-device-registry-conflict-review-and-support.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/ship-sync-device-registry-conflict-review-and-support.md`.

pub mod corpus;
pub mod model;

pub use corpus::{sync_device_registry_corpus, SyncDeviceRegistryScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure, BuildError,
    CertificationClaimCeiling, CertificationInput, CertificationNarrowingReason,
    CertificationPillars, CertificationQualification, CertificationRecoveryAction,
    CertificationUpstream, ConflictOutcomeClass, ConflictReviewRow, DeviceParticipationRow,
    DeviceParticipationState, EntryRouteRecord, IdentityModeClass, LayoutMode, LayoutModeDisclosure,
    LifecycleMarker, MergeClass, ProfileDurabilityClass, ProfileRoamingSummary, RecoveryActionRole,
    RecoveryRouteRecord, RouteSurface, SecretBoundaryRow, SettingCategory, SnapshotClass,
    SnapshotRow, StableClaimClass, StateClass, SurfaceClass, SurfaceParityRow,
    SyncDeviceRegistryCertification, SYNC_DEVICE_REGISTRY_NOTICE, SYNC_DEVICE_REGISTRY_RECORD_KIND,
    SYNC_DEVICE_REGISTRY_SCHEMA_VERSION, SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF,
};
