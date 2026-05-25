//! Stable certification for the **design-token runtime** across dark, light,
//! high-contrast, reduced-motion, and density rows.
//!
//! This lane mints one governed [`DesignTokenRuntimeCertification`] per
//! appearance posture. The record proves that the semantic-token runtime —
//! shell chrome, panels, dialogs, badges, notifications, diagnostics, trust
//! states, and execution states — stays stable across every claimed appearance
//! mode and that the captured evidence and the shipped runtime read **one**
//! appearance-session value. It is a genuine projection of the live appearance
//! runtime: it ingests the design-system appearance-session contract
//! ([`aureline_design_system::seeded_appearance_session_beta_contract`]), the
//! component-state launch-surface registry
//! ([`aureline_design_system::seeded_component_state_registry`]), the semantic
//! token registry per theme
//! ([`aureline_ui::tokens::seeded_token_registry`]), and the motion presets
//! ([`aureline_ui::motion`]), so the certification can never drift from what the
//! runtime actually resolves.
//!
//! A [`DesignTokenRuntimeCertification`] binds, for one posture:
//!
//! - **One appearance-session value** — every golden capture and accessibility
//!   review packet is attributable to the same appearance-session id and
//!   revision, so screenshots and shipped behavior use one source of truth.
//! - **Mode conformance** — a row per dark, light, high-contrast (dark/light),
//!   reduced-motion, and density mode, each proving the token registry resolves
//!   and that focus rings, state badges, severity cues, and keyboard
//!   affordances survive the mode change.
//! - **Non-color cue survival** — diagnostics, policy locks, trust warnings,
//!   execution targets, selection, and focus never rely on hue alone; text,
//!   shape, border, icon, or focus-ring cues survive contrast and motion modes.
//! - **Live-apply honesty** — every appearance axis declares whether an OS
//!   theme/contrast/accent/text-scale change applies live, applies live behind
//!   a checkpoint, requires confirmation, or requires a disclosed reload/restart
//!   — never a silent lag behind the system state.
//! - **Motion suppression in the runtime** — reduced-motion and power-saving
//!   suppression is modeled in the token/runtime motion presets, not improvised
//!   per surface.
//! - **No hard-coded stable styling** — launch-critical shell surfaces honor the
//!   semantic token runtime; a surface that cannot yet narrows the claim instead
//!   of leaking bespoke colors, density, or motion into Stable.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `certify-the-design-token-runtime-across-dark-light`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/certify-the-design-token-runtime-across-dark-light.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live appearance runtime, and pinned on disk under
//!   `fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/`.
//!
//! The contract narrative is
//! `docs/ux/m4/certify-the-design-token-runtime-across-dark-light.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/certify-the-design-token-runtime-across-dark-light.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    design_token_runtime_corpus, DesignTokenRuntimeScenario, CORPUS_AS_OF,
};
pub use model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure, AppearanceModeClass,
    AppearanceModeRow, AppearanceSessionBinding, BuildError, CertificationClaimCeiling,
    CertificationNarrowingReason, CertificationPillars, CertificationQualification,
    CertificationRecoveryAction, CertificationUpstream, DesignTokenRuntimeCertification,
    EntryRouteRecord, LaunchSurfaceRow, LayoutMode, LayoutModeDisclosure, LifecycleMarker,
    LiveApplyAxisRow, LiveApplyClass, MotionSuppressionRow, NonColorCueClass, ProtectedCueClass,
    ProtectedCueRow, RecoveryActionRole, RecoveryRouteRecord, RouteSurface, StableClaimClass,
    DESIGN_TOKEN_RUNTIME_NOTICE, DESIGN_TOKEN_RUNTIME_RECORD_KIND,
    DESIGN_TOKEN_RUNTIME_SCHEMA_VERSION, DESIGN_TOKEN_RUNTIME_SHARED_CONTRACT_REF,
};
