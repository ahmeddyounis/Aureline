//! Stable lock for the **badge aggregate**: count-class semantics, cross-client
//! / cross-window dedupe, admin / quiet-hours suppression lineage, and the
//! persistent attention summary.
//!
//! This module sits one level above the per-class durable-attention lock
//! ([`crate::notification_attention_stable`]). Where that lane mints one record
//! per durable attention class, this lane reconciles the **whole shell's badge
//! state** from the same durable object set the activity center reads, so the
//! dock/taskbar, title-bar, in-shell, and companion badges can never drift from
//! durable truth. It mints one governed [`BadgeAggregateRecord`] per shell
//! snapshot that binds:
//!
//! - **Typed count classes** — every count keyed by an [`AggregateCountClass`]
//!   (pending review/approval, failed runs, queued publish-later work,
//!   provider-auth attention, managed advisories, muted informational backlog,
//!   …), never by an arbitrary surface.
//! - **One durable object set** — the dock/taskbar, title-bar, in-shell, and
//!   companion projections are computed from the same deduped durable objects;
//!   a surface may never inflate a class above the authoritative active count.
//! - **Cross-client / cross-window dedupe** — repeated copies of the same
//!   underlying object collapse to one durable object and count once per class.
//! - **Export-safe suppression lineage** — every admin-suppressed,
//!   quiet-hours-muted, or per-class-disabled badge difference is explainable
//!   and preserves the durable object and its reopen target.
//! - **`0` means none** — a zero active badge means no current durable objects of
//!   that class, derived from durable object state, not a hidden toast.
//! - **A persistent, inspectable attention summary**.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//!
//! The activity center, dock/taskbar badge, title bar, status bar, companion
//! fanout, diagnostics, support exports, Help/About, and docs read this record
//! verbatim instead of cloning status text. The dedupe core, the per-item badge
//! reconciliation, and the count classes are **not** reinvented here: the record
//! is a genuine projection of the live attention stack in
//! [`crate::notifications`] and [`crate::attention_router`], routed through
//! [`crate::notification_envelope_corpus`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `finalize-badge-semantics-cross-client-dedupe-admin-suppression`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live attention router, and pinned on disk under
//!   `fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md`.

pub mod corpus;
pub mod model;

pub use corpus::{badge_aggregate_corpus, BadgeAggregateScenario, CORPUS_AS_OF};
pub use model::{
    class_summary_label, required_recovery_routes, AggregateCountClass, BadgeAggregateClaimCeiling,
    BadgeAggregateInput, BadgeAggregateNarrowingReason, BadgeAggregatePillars,
    BadgeAggregateQualification, BadgeAggregateRecord, BadgeAggregateUpstream, BadgeRecoveryAction,
    BadgeSuppressionReason, BadgeSurface, BuildError, ClassAggregate, CrossClientDedupeDisclosure,
    DedupedDurableObject, DurableItemDisposition, PersistentAttentionSummary, RawObjectAppearance,
    SuppressionLineageEntry, SuppressionScope, SurfaceClassCount, SurfaceProjection,
    SurfaceProjectionInput, BADGE_AGGREGATE_NOTICE, BADGE_AGGREGATE_RECORD_KIND,
    BADGE_AGGREGATE_SCHEMA_VERSION, BADGE_AGGREGATE_SHARED_CONTRACT_REF,
};
