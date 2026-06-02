//! Stable lock for notification-envelope routing, durable activity-center /
//! job-row truth, quiet-hours policy, privacy-safe OS alerts, interruptibility,
//! and exact-target reopen.
//!
//! This module makes durable attention replacement-grade on the claimed-stable
//! matrix. It mints one governed [`AttentionLockRecord`] per durable attention
//! class that binds, for a single canonical attention identity:
//!
//! - **One-envelope routing** — the alert flows through the one governed router
//!   into a single route outcome; every resolved surface keeps the same reopen
//!   target.
//! - **A durable job row** — job id, actor/subsystem, current phase, label, and
//!   cancel/retry/open-details affordances that survive look-away, sleep/resume,
//!   and restart/restore where continuity is claimed.
//! - **Coherent quiet-hours / admin suppression** — suppression may change
//!   fanout but never erases the durable object, the reopen target, or the audit
//!   trail.
//! - **Privacy-safe OS alerts** — lock-screen / notification-center copy is
//!   summary-first and never exposes secrets, raw code, AI prompt content, or
//!   high-risk action detail by default.
//! - **Interruptibility** — durable jobs and repeated failures never degrade to
//!   toast-only truth; repeats coalesce by root cause.
//! - **Exact-target reopen** — notifications, badges, and job rows reopen the
//!   authoritative object or a truthful placeholder and never re-issue a side
//!   effect from the notification surface.
//! - **Distinct lifecycle verbs** — acknowledge, resolve, dismiss, snooze, and
//!   mute are distinct transitions on one durable object.
//! - **Truthful badge counts** — derived from durable item state, not raw event
//!   fanout.
//! - **A public claim ceiling** — no row over-claims any pillar.
//! - **Automatic narrowing** — a row missing a pillar, or on a surface whose own
//!   marker is below Stable, is narrowed below Stable with a named reason.
//! - **Recovery, route, and accessibility parity** — the same item reachable
//!   from the activity center, command palette, status bar, and a menu command,
//!   keyboard-first, in normal / high-contrast / zoomed layouts.
//! - **No-account / no-managed-services availability**.
//!
//! The activity center, status bar, command palette, dock/taskbar badge, native
//! OS notifications, companion fanout, diagnostics, support exports, Help/About,
//! and docs read this record verbatim instead of cloning status text. The
//! envelope, router, lifecycle grammar, quiet-hours posture, and badge
//! reconciliation are **not** reinvented here: the record is a genuine
//! projection of the live attention stack in [`crate::notifications`] and
//! [`crate::attention_router`], routed through the corpus in
//! [`crate::notification_envelope_corpus`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `lock-notification-routing-durable-activity-center-truth-quiet`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/lock-notification-routing-durable-activity-center-truth-quiet.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live attention router, and pinned on disk under
//!   `fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/`.
//!
//! The contract narrative is
//! `docs/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md`.

pub mod corpus;
pub mod model;

pub use corpus::{attention_lock_corpus, AttentionLockScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, required_recovery_actions, snake_token, AccessibilityDisclosure,
    AttentionClaimCeiling, AttentionLockInput, AttentionLockRecord, AttentionRecoveryAction,
    AttentionRouteSurface, BadgeDisclosure, BuildError, DurableJobRow, EntryRouteRecord,
    ExactTargetReopen, Interruptibility, LayoutMode, LayoutModeDisclosure, LifecycleMarker,
    LifecycleSemantics, PrivacySafeAlert, QuietHoursPolicy, RecoveryActionRole,
    RecoveryRouteRecord, RoutingDisclosure, StableClaimClass, StableNarrowingReason,
    StableQualification, SurfaceParity, UpstreamRefs, ATTENTION_LOCK_NOTICE,
    ATTENTION_LOCK_RECORD_KIND, ATTENTION_LOCK_SCHEMA_VERSION, ATTENTION_LOCK_SHARED_CONTRACT_REF,
    REQUIRED_LIFECYCLE_VERBS,
};
