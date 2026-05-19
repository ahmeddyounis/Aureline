//! Capability records and artifact dependency markers.
//!
//! This crate is the shared vocabulary every capability-sensitive
//! artifact persists when it is saved, exported, synced, or imported.
//! It exists so settings exports, profiles, workflow bundles,
//! portable-state packages, recipes, saved views, migration packets,
//! support exports, and sync artifacts never silently lose meaning
//! when they cross to a target that lacks the Labs, Preview,
//! Beta-only, policy-gated, or host-specific capability they depend
//! on.
//!
//! Two record kinds live here:
//!
//! - [`CapabilityRecord`] — a stable description of one capability
//!   the artifact may depend on. It carries the capability id, the
//!   lifecycle state, the support promise, the dependency class
//!   ([`DependencyClass::Labs`], [`DependencyClass::Preview`],
//!   [`DependencyClass::BetaOnly`], [`DependencyClass::PolicyGated`],
//!   [`DependencyClass::HostSpecific`]), and the typed
//!   import/downgrade behavior.
//! - [`ArtifactDependencyMarker`] — the per-artifact persisted marker
//!   that names the required capability, the resolved
//!   import-behavior class, the fallback path the user can recover
//!   through, and the degrade/import note that surfaces use to
//!   explain *what changed* on a target lacking the capability.
//!
//! The crate also exposes
//! [`project_marker_for_host_surface`](markers::project_marker_for_host_surface)
//! so settings inspectors, import-review sheets, bundle detail pages,
//! downgrade flows, headless / CLI inspect output, and docs / help
//! pages all consume the same warning vocabulary.
//!
//! The crate is data-only. It mints no kill switches, opens no RPC,
//! and does not parse YAML; it intentionally does not duplicate the
//! governance-owned lifecycle registry or the experiments inventory
//! that already live elsewhere in the workspace.

#![doc(html_root_url = "https://docs.rs/aureline-capabilities/0.0.0")]

pub mod dependency_markers;

pub use dependency_markers::{
    catalog_default_capabilities, project_marker_for_host_surface, validate_artifact_markers,
    validate_capability_record, validate_marker, ArtifactClass, ArtifactDependencyMarker,
    BehaviorOnMissing, CapabilityRecord, DependencyClass, EffectOnImport, HostSurface,
    MarkerHostProjection, MarkerValidationError, SupportPromise, ARTIFACT_DEPENDENCY_MARKER_KIND,
    ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION, ARTIFACT_DEPENDENCY_MARKER_SHARED_CONTRACT_REF,
    CAPABILITY_RECORD_KIND, CAPABILITY_RECORD_SCHEMA_VERSION,
    CAPABILITY_RECORD_SHARED_CONTRACT_REF,
};

pub use dependency_markers::lifecycle::CapabilityLifecycleState;
