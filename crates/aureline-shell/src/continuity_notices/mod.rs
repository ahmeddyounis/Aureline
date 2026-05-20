//! Maintenance, drain, failover, and tenant-migration continuity notices.
//!
//! This module closes the gap between service-health truth ("a service is
//! degraded") and real operational change ("a planned read-only window starts
//! at this exact time, your publish-later work is queued and survives, your
//! local edits keep working, and your tenant moved to a new region"). It mints
//! one governed [`ContinuityNoticeView`] the desktop shell, the activity center
//! / durable history, CLI / headless inspect, diagnostics, and support exports
//! all read verbatim — composing the upstream `maintenance_notice_record`,
//! `tenant_migration_event_record`, `failover_banner_record`, and
//! `local_safe_baseline_record` boundary records rather than forking a per-
//! surface banner.
//!
//! - [`model`] — the governed record types ([`ContinuityNoticeView`], its
//!   closed vocabularies, the builder, the no-silent-current freshness
//!   downgrade, and the boundary-preserved-after-recovery invariant).
//! - [`corpus`] — the deterministic cross-surface drill corpus pinned on disk
//!   under `fixtures/ops/m3/maintenance_and_failover_notices/`.
//!
//! The view's schema boundary is
//! `schemas/ops/continuity_notice_view.schema.json`; the contract narrative is
//! `docs/ops/m3/maintenance_failover_truth.md`.

pub mod corpus;
pub mod model;

pub use corpus::{continuity_notice_corpus, ContinuityNoticeScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, AffectedScope, BlockStateClass, BlockedWriteInput, BlockedWriteRow,
    BoundaryAxisClass, BoundaryAxisInput, BoundaryAxisRow, BoundaryAxisStateClass, BoundaryChange,
    BoundaryChangeInput, ContinuityNoticeInput, ContinuityNoticeSummary, ContinuityNoticeView,
    DeploymentProfileClass, DisplayCopy, DowngradeReasonClass, EffectiveFreshnessClass,
    FreshnessClass, HostedMutationInput, HostedMutationRow, LifecycleInput, LocalContinuity,
    LocalContinuityInput, LocalCoreStatusClass, ManagedActionClass, NoticeCategoryClass,
    NoticeKindClass, NoticeLifecycle, NoticeSchedule, PlanClass, RefreshAgeClass,
    ResidencyScopeClass, ResumeTriggerClass, SaferThanRetryGuidanceClass, ScheduleInput, ScopeInput,
    ServiceClass, TimeBasisClass, ViewBuildError, WriteContinuityPostureClass,
    CONTINUITY_NOTICE_NOTICE, CONTINUITY_NOTICE_VIEW_RECORD_KIND,
    CONTINUITY_NOTICE_VIEW_SCHEMA_VERSION,
};
