//! Notification routing for toast, banner, and status surfaces.
//!
//! This module is the protected-row consumer for the typed notification
//! envelope frozen at
//! [`/schemas/ux/notification_envelope.schema.json`] and its reviewer-facing
//! seed at [`/docs/ux/notification_routing_seed.md`]. The shell does NOT
//! mint a parallel notification vocabulary: it deserializes envelopes,
//! routes them onto toast / banner / status / durable-activity surfaces,
//! collapses repeats by `dedupe_key_scheme` + `dedupe_key_ref`, and keeps
//! every routed surface pointed at the same `reopen_target` so a toast and
//! a status row both reopen the same canonical object.
//!
//! ## Pieces
//!
//! - [`envelope`] — typed Rust mirror of the boundary schema. Adapters and
//!   tests deserialize fixtures into [`envelope::NotificationEnvelope`].
//! - [`router`] — the dedupe-aware [`router::NotificationRouter`] that takes
//!   an envelope and emits a [`router::RoutedNotification`] (one
//!   [`router::SurfaceRoute`] per recommended surface).
//! - [`routes`] — per-surface row projections
//!   ([`routes::NotificationSurfaceRow`],
//!   [`routes::NotificationSurfaceSnapshot`]) the chrome reads when it
//!   draws toasts, banners, status items, and durable activity rows.
//! - [`external`] — privacy-safe OS, lock-screen, and companion payload
//!   projection with one safe primary action plus exact reopen.
//! - [`actions`] and [`audit`] — post-route action semantics, badge
//!   reconciliation, and suppression/dedupe audit records.
//!
//! ## Failure-drill posture
//!
//! When the same canonical event arrives multiple times, the router emits
//! `delivered` receipts on the first emission and `deduped_canonical_event`
//! (or `deduped_grouped_burst`) receipts on subsequent emissions for
//! surfaces that have already lit up. The reopen target ref is preserved on
//! every receipt so the deduped row in the activity center still leads to
//! the same canonical object — preventing split-brain state across surfaces.

pub mod actions;
pub mod audit;
pub mod envelope;
pub mod external;
pub mod quiet_hours;
pub mod router;
pub mod routes;

pub use actions::{
    BadgeClass, NotificationActionRequest, NotificationAttentionState,
    NotificationBadgeReconciliation, NotificationLifecycleActionKind,
    NOTIFICATION_ACTION_STATE_SCHEMA_VERSION, NOTIFICATION_ATTENTION_STATE_RECORD_KIND,
    NOTIFICATION_BADGE_RECONCILIATION_RECORD_KIND,
};
pub use audit::{
    NotificationSuppressionAuditEntry, NotificationSuppressionAuditReport,
    NotificationSuppressionExplanationClass, NOTIFICATION_SUPPRESSION_AUDIT_ENTRY_RECORD_KIND,
    NOTIFICATION_SUPPRESSION_AUDIT_REPORT_RECORD_KIND,
    NOTIFICATION_SUPPRESSION_AUDIT_SCHEMA_VERSION,
};
pub use envelope::{
    ClientScope, DedupeKeyScheme, FanoutReceipt, FanoutReceiptState, FanoutSurfaceClass,
    NotificationEnvelope, PrivacyClass, PrivacyPayloadClass, QuietHoursMode, RedactionClass,
    ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem, StableAction,
    StaleOrUndeliveredReason, StaleOrUndeliveredReasonClass, SuppressionReason, SuppressionState,
    FANOUT_RECEIPT_SCHEMA_VERSION, NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
};
pub use external::{
    ExternalNotificationPayload, ForbiddenShortcutActionClass,
    EXTERNAL_NOTIFICATION_PAYLOAD_RECORD_KIND, EXTERNAL_NOTIFICATION_PAYLOAD_SCHEMA_VERSION,
};
pub use quiet_hours::{
    BadgeSeverityCounts, DurableBadgeProjection, QuietHoursPosture,
    DURABLE_BADGE_PROJECTION_RECORD_KIND, DURABLE_BADGE_PROJECTION_SCHEMA_VERSION,
};
pub use router::{
    NotificationRouter, NotificationRoutingError, RoutedNotification, SurfaceRoute,
    ROUTED_NOTIFICATION_RECORD_KIND, ROUTED_NOTIFICATION_SCHEMA_VERSION,
};
pub use routes::{
    NotificationSurfaceRow, NotificationSurfaceSnapshot, NOTIFICATION_SURFACE_ROW_RECORD_KIND,
    NOTIFICATION_SURFACE_ROW_SCHEMA_VERSION,
};
