//! Stable certification for the **component-state registry** across shell,
//! editor, panel, popover, dialog, and extension-adjacent surfaces.
//!
//! This lane hardens the beta component-state registry published by
//! [`aureline_design_system`] into one governed
//! [`ComponentStateRegistryCertification`] per registry posture. The record
//! proves that product and extension surfaces share **one** component-state
//! vocabulary — hover, focus, selected, disabled, loading, warning, error,
//! blocked, recovering, degraded, and the normalized degraded family — instead
//! of local reinvention, and that the shared vocabulary, extension inheritance,
//! shell-zoning semantics, and per-permutation state fixtures all read one
//! registry value. It is a genuine projection of the live runtime: it ingests
//! the design-system component-state registry
//! ([`aureline_design_system::seeded_component_state_registry`]), the live
//! extension appearance-conformance packet
//! ([`aureline_extensions::appearance_conformance::seeded_appearance_conformance_packet`]),
//! the design-system screenshot-diff packet
//! ([`aureline_design_system::seeded_screenshot_diff_packet`]), and the shared
//! UI taxonomy ([`aureline_ui::components::ComponentStateClass`]), so the
//! certification can never drift from what the runtime actually publishes.
//!
//! A [`ComponentStateRegistryCertification`] binds, for one posture:
//!
//! - **One registry value** — every family, normalized-state, zone, and fixture
//!   row resolves against the same shared taxonomy so no surface forks the
//!   vocabulary.
//! - **Family coverage** — a row per core control, dense row, tab, tree,
//!   palette, popover, dialog, banner, job row, and inline notice, each declaring
//!   supported states, required affordances, and an accessibility note, and
//!   proving it is token-driven with focus and screen-reader semantics preserved.
//! - **Normalized states** — a row per disabled, blocked, policy-locked,
//!   reconnecting, warming, partial, stale, and recovering state, proving the
//!   treatment stays consistent across shell, review, settings, and support with
//!   a narratable reason and an action path and never hue or animation alone.
//! - **Extension inheritance honesty** — a row per appearance axis declaring
//!   whether contributed/embedded surfaces inherit the host axis fully,
//!   partially, not at all, or undisclosed, with any gap surfacing in review,
//!   diagnostics, and support/export.
//! - **Shell-zoning semantics** — a row per declared slot, proving docked-versus
//!   sheet state, min/max chrome metrics, density semantics, reduced-motion
//!   behavior, and placeholder cards are token-driven rather than hard-coded.
//! - **State-fixture coverage** — a row per launch-critical surface/state
//!   permutation, proving every permutation has a stable capture and fixture.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, design-QA packets, and support exports
//! read this record verbatim instead of cloning status text.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `harden-component-state-registry-and-theme-token-parity`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/harden-component-state-registry-and-theme-token-parity.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live contracts, and pinned on disk under
//!   `fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/`.
//!
//! The contract narrative is
//! `docs/ux/m4/harden-component-state-registry-and-theme-token-parity.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/harden-component-state-registry-and-theme-token-parity.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    component_state_registry_corpus, ComponentStateRegistryScenario, CORPUS_AS_OF,
};
pub use model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure, BuildError,
    CanonicalStateClass, CertificationClaimCeiling, CertificationInput,
    CertificationNarrowingReason, CertificationPillars, CertificationQualification,
    CertificationRecoveryAction, CertificationUpstream, ComponentFamilyClass, ComponentFamilyRow,
    ComponentStateName, ComponentStateRegistryCertification, EntryRouteRecord,
    ExtensionInheritanceRow, LaunchSurfaceClass, LayoutMode, LayoutModeDisclosure, LifecycleMarker,
    NonColorCueClass, NormalizedStateRow, RecoveryActionRole, RecoveryRouteRecord, RegistryBinding,
    RegistrySurfaceClass, RequiredAffordanceClass, RouteSurface, ShellZoneClass, ShellZoneRow,
    StableClaimClass, StateFixtureRow, ZoneLayoutMode, COMPONENT_STATE_REGISTRY_NOTICE,
    COMPONENT_STATE_REGISTRY_RECORD_KIND, COMPONENT_STATE_REGISTRY_SCHEMA_VERSION,
    COMPONENT_STATE_REGISTRY_SHARED_CONTRACT_REF,
};
