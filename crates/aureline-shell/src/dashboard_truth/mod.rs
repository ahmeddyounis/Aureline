//! Dashboard and queue truth: freshness, evidence, order-reason, and
//! hidden-scope honesty for the operator-facing beta surfaces.
//!
//! This module closes the stale-green and unexplained-queue gap across the
//! service-health dashboard, the review inbox, the incident queue, the
//! support queue, and the admin queue. Every row carries a freshness class,
//! the age of its last-successful evidence, and the canonical durable object
//! its open-details affordance routes to; every queue carries an order reason
//! per row and a hidden-by-scope counter per narrowing reason. A would-be-green
//! row cannot stay green once freshness or evidence has expired — it downgrades
//! to `unconfirmed` and names why.
//!
//! - [`model`] — the governed record types ([`DashboardTruthView`],
//!   [`DashboardFreshnessCard`], [`QueueOrderReason`]), their closed
//!   vocabularies, the builder, and the no-silent-green invariant.
//! - [`corpus`] — the deterministic cross-surface drill corpus pinned on disk
//!   under `fixtures/ops/m3/dashboard_and_queue_truth/`.

pub mod corpus;
pub mod model;

pub use model::{
    is_canonical_object_ref, DashboardFreshnessCard, DashboardSurfaceClass, DashboardTruthSummary,
    DashboardTruthView, DisplayedStateClass, DowngradeReasonClass, EffectiveStateClass,
    EvidenceAgeClass, EvidenceKindClass, FreshnessCardInput, FreshnessClass, HiddenScopeCounter,
    HiddenScopeInput, NarrowingReasonClass, OrderReasonClass, QueueOrderInput, QueueOrderReason,
    QueueRowInput, QueueRowOrder, ViewBuildError, DASHBOARD_FRESHNESS_CARD_RECORD_KIND,
    DASHBOARD_FRESHNESS_CARD_SCHEMA_VERSION, DASHBOARD_TRUTH_NOTICE,
    DASHBOARD_TRUTH_VIEW_RECORD_KIND, DASHBOARD_TRUTH_VIEW_SCHEMA_VERSION,
    QUEUE_ORDER_REASON_RECORD_KIND, QUEUE_ORDER_REASON_SCHEMA_VERSION,
};

pub use corpus::{dashboard_truth_corpus, DashboardTruthScenario, CORPUS_AS_OF};
