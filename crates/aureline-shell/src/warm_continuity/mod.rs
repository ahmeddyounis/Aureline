//! Warm shell startup, warm restore, and first-useful-work routing truth.
//!
//! This module makes warm shell startup replacement-grade. It mints one governed
//! [`WarmContinuityRecord`] that binds, for a single warm cycle:
//!
//! - **Skeleton-first startup** — useful chrome, available command entry, and a
//!   stable focus target painted before deep discovery (indexing, remote
//!   reconnect, provider hydration) finishes.
//! - **Honest restore provenance** — what came back *exactly*, what came back
//!   *partially*, and what now *needs review*, without implying the live runtime
//!   resumed, and with every side-effectful surface skeletoned rather than
//!   silently rerun.
//! - **Typed first-useful-work routing** — the prior active editor, a
//!   changed-files view, the README, a review packet, or a post-entry handoff
//!   card, with the route reason recorded and remembered preferences bounded so
//!   they can never widen trust or run setup.
//! - **Zone-owned truth preservation** — breadcrumbs, trust badges, and
//!   execution-target cues stay in their owning zones during warm hydration,
//!   resize, and missing-dependency fallback.
//! - **Reachable responsive fallback** — when a side surface collapses to a
//!   sheet, overlay, or overflow under a compact / degraded layout, its reopen
//!   route and last meaningful state stay explicit and keyboard-reachable.
//!
//! The desktop shell, diagnostics, support exports, Help/About, and docs read
//! this record verbatim instead of cloning status text. The canonical artifacts
//! for this lane (suggested-output stem
//! `harden_shell_startup_warm_restore_and_first_useful`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json`.
//! - [`corpus`] — the deterministic cross-surface drill corpus pinned on disk
//!   under `fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/`.
//!
//! The contract narrative is
//! `docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`.

pub mod corpus;
pub mod model;

pub use corpus::{warm_continuity_corpus, WarmContinuityScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, BuildError, CollapseTargetClass, CollapsedSurface,
    DowngradeTriggerToken, EntryCauseClass, LandingDecisionInput, LandingRouteClass,
    LandingRouteReasonClass, NoRerunSurface, RememberedPreference, ResponsiveFallbackInput,
    RestoreClassToken, RestoreItem, RestoreProvenanceClass, RestoreProvenanceInput,
    RestoreSurfaceClass, ShellZoneToken, SideEffectfulSurfaceClass, StartupMilestoneClass,
    StartupMilestoneInput, StartupTrace, WarmContinuityDisplayCopy, WarmContinuityInput,
    WarmContinuityRecord, WarmContinuitySummaryCounts, WindowClassToken, ZoneIdentityInput,
    ZoneOwnedCue, ZoneOwnedCueClass, WARM_CONTINUITY_NOTICE, WARM_CONTINUITY_RECORD_KIND,
    WARM_CONTINUITY_SCHEMA_VERSION,
};
