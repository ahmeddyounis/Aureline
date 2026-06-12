//! Field-aware sync-and-device review for M5-added feature families.
//!
//! The module mints one governed record that brings the new M5 feature families
//! — notebooks, data/API, profiler, extension bundles, and companion — into a
//! field-aware sync and device-participation model instead of an opaque
//! last-writer-wins blob. Each family ships a schema-backed scope bundle that
//! names its payload schema version, capability dependencies, redaction mode,
//! source device/profile, and local/remote revision sets. Divergences are
//! field-aware conflicts — same-key divergent, policy-locked, missing-capability,
//! machine-only, delete-versus-modify, or stale-remote — that keep local durable
//! state authoritative and never let a remote payload, a trust widening, or a
//! managed entitlement win silently. The record also exposes the durable
//! pause/resume/revoke/forget/rotate device-action catalog and the
//! offline/stale-remote/blocked-apply/E2EE-unavailable/local-only drill set so
//! the settings UI, CLI inspect, docs/help, and support exports consume one
//! shared sync object model instead of cloning sync status text.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{m5_sync_and_device_review_corpus, M5SyncAndDeviceReviewScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, BuildError, BundleSyncTrust, ConflictClass, ConflictDisposition,
    DeviceAction, DeviceActionRecord, DeviceClass, DeviceParticipationState, DrillKind,
    FieldConflict, M5SyncAndDeviceReview, M5SyncAndDeviceReviewInput, NarrowingReason,
    RedactionMode, ScopeCapabilityDependency, ScopeRevisionSets, SurfaceClass, SurfaceTruthRow,
    SyncDrill, SyncReviewClaim, SyncReviewPillars, SyncReviewQualification, SyncScopeBundle,
    SyncScopeFamily, SyncTransportState, TrustWideningClass, M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND,
    M5_SYNC_AND_DEVICE_REVIEW_SCHEMA_VERSION, M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF,
};
