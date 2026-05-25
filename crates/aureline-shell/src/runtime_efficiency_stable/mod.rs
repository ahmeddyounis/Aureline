//! Stable lock for **battery, thermal, suspend-resume, and user-visible
//! runtime-efficiency adaptation** on a claimed-stable desktop row.
//!
//! This module makes runtime-efficiency adaptation replacement-grade on the
//! claimed-stable matrix. It mints one governed [`RuntimeEfficiencyRecord`] per
//! efficiency posture that binds, for a single efficiency-state identity:
//!
//! - **A materialized runtime-efficiency state** — `Nominal`, `EfficiencyAware`,
//!   `ThermalConstrained`, `ProtectCore`, or `Recovery`, each bound to named
//!   shed-work classes, protected foreground paths, resume conditions, and
//!   export-safe diagnostics.
//! - **Background shed before foreground** — speculative indexing, extension
//!   warmup, AI background jobs, uploads, and provider-overlay refresh pause or
//!   throttle before typing, save, navigation, or quick-open ever regress.
//! - **Protected foreground latency bands** — editing, save, direct navigation,
//!   quick-open, and the command palette stay within published bands at every
//!   posture.
//! - **Hidden-pane quiescence** — a hidden, occluded, or off-screen pane commits
//!   no paint and runs no decorative animation or speculative poll.
//! - **A surfaced queue-governor reason** — battery saver, thermal clamp,
//!   low-disk, suspend, and resume transitions name the governor reason, the
//!   paused lanes, and the resume owner across shell, status, and diagnostics so
//!   they never masquerade as generic slowness or stale data.
//! - **Preserved durable state** — adaptation never skips save durability, loses
//!   a dirty buffer, or hides a user-owned artifact.
//! - **Per-OS conformance** — macOS, Windows, and Linux each carry current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//! - **Recovery, route, and accessibility parity** and **no-account /
//!   no-managed-services availability**.
//!
//! The shell status strip, the in-product diagnostics review, the CLI inspector,
//! Help/About, and the diagnostics support export read this record verbatim
//! instead of cloning status text. The power/thermal policy, the
//! workload-budget decisions, the render-visibility audit, and the
//! suspend-resume continuity vocabulary are **not** reinvented here: the record
//! is a genuine projection of the live efficiency runtime in
//! [`crate::efficiency`] and the suspend-resume / power-posture page in
//! [`crate::runtime_adaptation`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `stabilize-battery-thermal-suspend-resume-and-user-visible`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live efficiency runtime, and pinned on disk under
//!   `fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/`.
//!
//! The contract narrative is
//! `docs/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md`.

pub mod corpus;
pub mod model;

pub use corpus::{runtime_efficiency_corpus, RuntimeEfficiencyScenario, CORPUS_AS_OF};
pub use model::{
    required_recovery_routes, BuildError, EfficiencyClaimCeiling, EfficiencyNarrowingReason,
    EfficiencyPillars, EfficiencyQualification, EfficiencyRecoveryAction,
    EfficiencySurfaceProjection, EfficiencySurfaceProjectionInput, EfficiencyTruthSurface,
    EfficiencyUpstream, GovernorReasonClass, PlatformConformanceRow, PlatformProfileClass,
    ProtectedForegroundPath, ProtectedPathRow, QueueGovernorDisclosure, ResumeOwner,
    RuntimeEfficiencyInput, RuntimeEfficiencyRecord, ShedWorkRow, SuspendResumeContinuity,
    ALL_EFFICIENCY_STATES, RUNTIME_EFFICIENCY_NOTICE, RUNTIME_EFFICIENCY_RECORD_KIND,
    RUNTIME_EFFICIENCY_SCHEMA_VERSION, RUNTIME_EFFICIENCY_SHARED_CONTRACT_REF,
};
