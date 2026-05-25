//! Stable Start Center, recent-work, and workspace-switcher target-kind
//! disclosure truth.
//!
//! This module makes the no-workspace entry surfaces replacement-grade on the
//! claimed stable matrix. It mints one governed [`EntryTargetDisclosureRecord`]
//! per entry target that binds, for a single canonical recent-work identity:
//!
//! - **Target-kind disclosure** — the canonical target kind, path/host/provider
//!   subtitle, last-opened time, trust posture, and restore availability shown
//!   before the user commits, reusing the [`aureline_workspace`] recent-work
//!   vocabulary rather than a parallel model.
//! - **A public claim ceiling** — no row may assert a live open, remote
//!   availability, restore fidelity, or trust the product cannot prove.
//! - **Recovery before commit** — Locate, Reconnect / Reauthorize, an
//!   open-minimal route where safe, and a metadata-only Remove, with a failed
//!   open keeping (never silently discarding) the stale entry.
//! - **One model across surfaces** — the Start Center and switcher projections
//!   share identity and recovery behavior, and the switcher keeps a cancel /
//!   reopen-previous return path.
//! - **Route parity** — the same target opens from the Start Center, switcher,
//!   command palette, and a menu command, each keyboard reachable.
//! - **Accessibility** — tab order, row narration, action labels, and recovery
//!   affordances reachable in normal, high-contrast, and zoomed layouts.
//! - **No-account / no-managed-services availability** — every row stays
//!   listed even when identity or managed services are absent.
//!
//! The desktop shell, recent-work list, workspace switcher, command palette,
//! menus, diagnostics, support exports, Help/About, and docs read this record
//! verbatim instead of cloning status text. The canonical artifacts for this
//! lane (suggested-output stem
//! `stabilize-the-start-center-recent-work-list-workspace`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live Start Center and switcher builders and pinned on disk under
//!   `fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/`.
//!
//! The contract narrative is
//! `docs/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    entry_target_disclosure_corpus, seeded_stable_recent_work_registry,
    EntryTargetDisclosureScenario, CORPUS_AS_OF,
};
pub use model::{
    is_canonical_object_ref, required_recovery_actions, AccessibilityDisclosure, BuildError,
    DisclosureFacts, EntryRouteRecord, EntryRouteSurface, EntryTargetDisclosureInput,
    EntryTargetDisclosureRecord, LayoutMode, LayoutModeDisclosure, PublicClaimCeiling,
    RecoveryActionRole, RecoveryRouteRecord, SubtitleKind, SurfaceParity, TargetClass,
    ENTRY_TARGET_DISCLOSURE_NOTICE, ENTRY_TARGET_DISCLOSURE_RECORD_KIND,
    ENTRY_TARGET_DISCLOSURE_SCHEMA_VERSION, ENTRY_TARGET_DISCLOSURE_SHARED_CONTRACT_REF,
};
