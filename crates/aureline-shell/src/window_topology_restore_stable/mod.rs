//! Stable lock for **multi-window, pane-detach, split-layout, mixed-DPI, and
//! multi-monitor restore** behavior.
//!
//! This module makes desktop window restore replacement-grade on the claimed-
//! stable matrix. It mints one governed [`WindowTopologyRestoreRecord`] per
//! window reopen that binds, for a single window identity:
//!
//! - **Authority / topology separation** — workspace authority (dirty buffers,
//!   recovery journals, trust/policy, VFS identity, attached execution contexts)
//!   stays centralized while pane-tree layout, focus history, zoom/follow state,
//!   and visible surfaces stay window-local.
//! - **A versioned pane-tree with stable pane IDs** — split, move, float, pin,
//!   and close-pane mutate one versioned tree keyed by stable pane IDs.
//! - **Skeleton-first / hydrate-second restore** — the structure is recreated
//!   first; session-scoped panes (terminal, debug, notebook, preview, remote)
//!   hydrate into truthful placeholder or reconnect states and never silently
//!   reacquire live authority.
//! - **Restore-no-rerun honesty** — every session-scoped pane that did not
//!   survive keeps its slot with an in-place placeholder card and forbids
//!   command rerun and authority reacquire until an explicit user action.
//! - **Display-topology and downgrade provenance** — monitor changes, missing-
//!   extension substitutions, expired managed/remote sessions, and any downgrade
//!   from Exact to Compatible, Layout-only, or placeholder-backed are recorded
//!   so the layout change is explainable in diagnostics and support export.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//!
//! The desktop restore review, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning
//! status text. The pane-tree vocabulary, the topology-change classes, and the
//! restore adjustments are **not** reinvented here: the record projects the live
//! [`crate::windows`] workspace-management page, the [`crate::restore`] provenance
//! contract, and the [`crate::layout`] split tree.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `harden-multi-window-pane-detach-split-layout-mixed`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/harden-multi-window-pane-detach-split-layout-mixed.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live windows page, and pinned on disk under
//!   `fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/`.
//!
//! The contract narrative is
//! `docs/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md`.

pub mod corpus;
pub mod model;

pub use corpus::{window_topology_restore_corpus, WindowTopologyRestoreScenario, CORPUS_AS_OF};
pub use model::{
    required_recovery_routes, AuthoritySeparation, BuildError, DisplayTopologyProvenance,
    PaneHydrationClass, PanePlaceholderState, PaneRecoveryAction, PaneSlot, PaneSubstitutionReason,
    PaneSurfaceClass, PaneTree, PaneTreeNode, RecoveryChromeAssurance, RestoreDowngrade,
    RestoreDowngradeReason, RestoreFidelityClass, RestoreProvenance, RestoreSurfaceProjection,
    RestoreSurfaceProjectionInput, RestoreTruthSurface, WindowLocalTopologyClass,
    WindowRestoreClaimCeiling, WindowRestoreNarrowingReason, WindowRestorePillars,
    WindowRestoreQualification, WindowRestoreRecoveryAction, WindowRestoreUpstream,
    WindowTopologyRestoreInput, WindowTopologyRestoreRecord, WorkspaceAuthorityClass,
    PANE_TREE_SCHEMA_VERSION, WINDOW_TOPOLOGY_RESTORE_NOTICE, WINDOW_TOPOLOGY_RESTORE_RECORD_KIND,
    WINDOW_TOPOLOGY_RESTORE_SCHEMA_VERSION, WINDOW_TOPOLOGY_RESTORE_SHARED_CONTRACT_REF,
};
