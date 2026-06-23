//! M5 attention-routing matrix: the frozen, typed contract for Aureline's
//! notification envelopes, durable activity objects, badge aggregates, fanout
//! receipts, routing context, privacy classes, and action/retention semantics.
//!
//! Aureline routes attention. A notification is typed, privacy-aware, and
//! reopen-safe: no long-running or reviewable work lives only in a toast, badges
//! derive from deduped durable items, OS and companion fanout cannot bypass the
//! in-product preview/approval flow, suppression and quiet-hours state stays
//! separate from audit history, and every attention surface can reopen the
//! authoritative object instead of reissuing a blind side effect. Each of those
//! attention objects already has a boundary schema under `schemas/ux/` (plus the
//! sibling `schemas/events/` activity row) and at least one producing crate in
//! the shell. What was missing was a single place that names the attention object
//! *families*, freezes their stable identifiers and required fields, pins one
//! controlled vocabulary across them, maps each one to the proof packet that
//! keeps it current, and states the invariants every attention surface must hold.
//! This lane is that place.
//!
//! The matrix does four things:
//!
//! 1. **Names the attention object families** ([`AttentionObjectClass`]) and, for
//!    each, cites the canonical boundary schema(s) it binds, the crate(s) that
//!    already produce that truth, the required fields it must carry, and the
//!    [`proof packet`](AttentionObjectEntry::proof_packet_ref) that keeps it
//!    current — so docs, help, support, activity, and companion surfaces point at
//!    the same object model rather than re-expressing notification truth ad hoc.
//! 2. **Freezes one state vocabulary** ([`AttentionStateClass`]) spanning the
//!    notification lifecycle, durable-job progress, cross-client delivery, and
//!    suppression / quiet-hours states.
//! 3. **Defines the controlled vocabulary** ([`ControlledVocabulary`]) the spec
//!    requires: severity, scope, privacy class, dedupe rule, suppression,
//!    quiet-hours, stale/undelivered fanout, and reopen / authoritative-object
//!    routing. Each object declares which of those vocabularies it binds.
//! 4. **Covers every fanout channel** ([`FanoutChannelClass`]): the in-product
//!    activity center, OS native notifications, the dock/taskbar badge, the
//!    browser companion, the mobile companion, and the operator dashboard, with
//!    the delivery posture, privacy ceiling, and preview/approval rule each
//!    carries.
//!
//! [`attention_routing_matrix`] is the canonical binding: it builds the matrix
//! deterministically and computes each [`AttentionMatrixInvariant`]'s `holds`
//! flag from the built data, so the checked-in fixture and the freeze gate freeze
//! the contract byte-for-byte and an inconsistent edit flips an invariant and
//! fails CI. In particular [`AttentionMatrixInvariant`]
//! `attention.proof_packet_mapped` flips false the moment a claimed attention
//! object lacks a mapped proof packet, so stable promotion cannot harden an
//! attention claim without current proof. The record carries no message bodies,
//! credentials, raw provider payloads, hostnames, or absolute paths — only opaque
//! object refs, stable tokens, and short reviewable sentences — so it is safe for
//! support export.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the attention-routing matrix.
pub const M5_ATTENTION_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the attention-routing matrix.
pub const M5_ATTENTION_ROUTING_SCHEMA_REF: &str =
    "schemas/activity/m5-attention-routing.schema.json";

/// Stable record-kind tag for the attention-routing matrix.
pub const M5_ATTENTION_ROUTING_RECORD_KIND: &str = "m5_attention_routing_matrix";

/// Stable id for the canonical attention-routing matrix.
pub const M5_ATTENTION_ROUTING_MATRIX_ID: &str = "m5-attention-routing:matrix:0001";

/// Evaluation stamp for the canonical matrix. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ATTENTION_ROUTING_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the matrix binding current. Stable promotion runs
/// this gate; it fails when the in-code matrix drifts from the checked-in fixture
/// or any invariant flips.
pub const M5_ATTENTION_ROUTING_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_attention_routing.rs";

// ---------------------------------------------------------------------------
// Attention object families.
// ---------------------------------------------------------------------------

/// The closed set of governed attention object families this matrix freezes.
///
/// Each family is one governed attention object. Adding a family is a breaking
/// change to the matrix; renaming one breaks every consumer that resolves an
/// object by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionObjectClass {
    /// The notification envelope: the typed, privacy-classed, deduped unit of
    /// attention with a stable action target and a route back to its source.
    NotificationEnvelope,
    /// The durable activity object: a long-running or reviewable job with phase,
    /// progress, cancel/retry affordances, and a reopen anchor — never toast-only.
    ActivityObject,
    /// The badge aggregate: a coarse, deduped pending-attention count per scope
    /// derived from durable items, with its muted and suppressed reasons.
    BadgeAggregate,
    /// The fanout receipt: per-destination cross-client delivery truth, including
    /// stale and undelivered outcomes labeled rather than silently dropped.
    FanoutReceipt,
    /// The routing context: how an event is severity/scope/privacy-classed and
    /// which surfaces it is routed to versus deferred by quiet-hours.
    RoutingContext,
    /// The privacy class: the per-class rule governing lock-screen payload,
    /// companion summary, support export, and quiet-hours behavior.
    PrivacyClass,
    /// The action/retention semantics: the distinct dismiss, snooze, acknowledge,
    /// resolve, and mute actions and the retention each implies.
    ActionRetentionSemantics,
}

impl AttentionObjectClass {
    /// All object families, in matrix order.
    pub const ALL: [Self; 7] = [
        Self::NotificationEnvelope,
        Self::ActivityObject,
        Self::BadgeAggregate,
        Self::FanoutReceipt,
        Self::RoutingContext,
        Self::PrivacyClass,
        Self::ActionRetentionSemantics,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationEnvelope => "notification_envelope",
            Self::ActivityObject => "activity_object",
            Self::BadgeAggregate => "badge_aggregate",
            Self::FanoutReceipt => "fanout_receipt",
            Self::RoutingContext => "routing_context",
            Self::PrivacyClass => "privacy_class",
            Self::ActionRetentionSemantics => "action_retention_semantics",
        }
    }

    /// Stable object id, namespaced so it is unique across the product.
    pub fn object_id(self) -> String {
        format!("attention_object.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotificationEnvelope => "Notification envelope",
            Self::ActivityObject => "Durable activity object",
            Self::BadgeAggregate => "Badge aggregate",
            Self::FanoutReceipt => "Fanout receipt",
            Self::RoutingContext => "Routing context",
            Self::PrivacyClass => "Privacy class",
            Self::ActionRetentionSemantics => "Action / retention semantics",
        }
    }
}

// ---------------------------------------------------------------------------
// Unified state vocabulary.
// ---------------------------------------------------------------------------

/// One shared state vocabulary spanning every attention object.
///
/// The tokens span the notification lifecycle, durable-job progress, cross-client
/// delivery, and suppression / quiet-hours states already frozen under
/// `schemas/ux/` and `schemas/events/`; each [`AttentionStateTerm`] in the matrix
/// cites the upstream enum tokens it derives from, so this vocabulary never
/// silently diverges from the objects it summarizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionStateClass {
    /// The envelope is created but not yet routed to any surface.
    Pending,
    /// The envelope has been routed to its recommended surfaces.
    Routed,
    /// The attention has been surfaced to the user.
    Shown,
    /// The user marked it read; the badge clears but the durable record remains.
    Acknowledged,
    /// Deferred to a resume time or condition; returns automatically when it
    /// expires.
    Snoozed,
    /// Deferred by quiet-hours policy rather than user choice; tracked separately
    /// from audit history.
    QuietHoursDeferred,
    /// Muted by a source mute or suppression rule; tracked separately from audit
    /// history, and never implies the underlying event disappeared.
    Suppressed,
    /// A durable job is in progress.
    Running,
    /// A durable job is queued or waiting on a dependency.
    QueuedWaiting,
    /// A durable job partially completed; the remainder is named.
    PartiallyCompleted,
    /// A durable job failed and remains reviewable and retryable.
    Failed,
    /// A durable job completed.
    Completed,
    /// Closed because the underlying object changed or the user marked it done.
    Resolved,
    /// The badge was cleared by the user; the underlying durable record remains.
    Dismissed,
    /// Moved into durable history, reopenable but no longer active.
    Archived,
    /// A cross-client fanout copy is stale relative to the authoritative object.
    FanoutStale,
    /// A cross-client fanout copy failed or was not delivered, labeled as such.
    FanoutUndelivered,
    /// State could not be determined and requires user review.
    UnknownRequiresReview,
}

impl AttentionStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 18] = [
        Self::Pending,
        Self::Routed,
        Self::Shown,
        Self::Acknowledged,
        Self::Snoozed,
        Self::QuietHoursDeferred,
        Self::Suppressed,
        Self::Running,
        Self::QueuedWaiting,
        Self::PartiallyCompleted,
        Self::Failed,
        Self::Completed,
        Self::Resolved,
        Self::Dismissed,
        Self::Archived,
        Self::FanoutStale,
        Self::FanoutUndelivered,
        Self::UnknownRequiresReview,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Routed => "routed",
            Self::Shown => "shown",
            Self::Acknowledged => "acknowledged",
            Self::Snoozed => "snoozed",
            Self::QuietHoursDeferred => "quiet_hours_deferred",
            Self::Suppressed => "suppressed",
            Self::Running => "running",
            Self::QueuedWaiting => "queued_waiting",
            Self::PartiallyCompleted => "partially_completed",
            Self::Failed => "failed",
            Self::Completed => "completed",
            Self::Resolved => "resolved",
            Self::Dismissed => "dismissed",
            Self::Archived => "archived",
            Self::FanoutStale => "fanout_stale",
            Self::FanoutUndelivered => "fanout_undelivered",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Routed => "Routed",
            Self::Shown => "Shown",
            Self::Acknowledged => "Acknowledged",
            Self::Snoozed => "Snoozed",
            Self::QuietHoursDeferred => "Quiet-hours deferred",
            Self::Suppressed => "Suppressed",
            Self::Running => "Running",
            Self::QueuedWaiting => "Queued / waiting",
            Self::PartiallyCompleted => "Partially completed",
            Self::Failed => "Failed",
            Self::Completed => "Completed",
            Self::Resolved => "Resolved",
            Self::Dismissed => "Dismissed",
            Self::Archived => "Archived",
            Self::FanoutStale => "Fanout stale",
            Self::FanoutUndelivered => "Fanout undelivered",
            Self::UnknownRequiresReview => "Unknown — requires review",
        }
    }

    /// Whether this state represents long-running or reviewable work that must be
    /// backed by a durable authoritative object rather than living only in an
    /// ephemeral toast.
    pub const fn requires_durable_object(self) -> bool {
        matches!(
            self,
            Self::Running
                | Self::QueuedWaiting
                | Self::PartiallyCompleted
                | Self::Failed
                | Self::Completed
                | Self::Resolved
                | Self::Archived
                | Self::Snoozed
        )
    }

    /// Whether this state is a cross-client delivery gap (the no-silent-success
    /// fanout class): the copy is stale or was not delivered.
    pub const fn is_delivery_gap(self) -> bool {
        matches!(self, Self::FanoutStale | Self::FanoutUndelivered)
    }

    /// Whether this state is a suppression / quiet-hours state, which must be
    /// stored separately from audit history.
    pub const fn is_suppression(self) -> bool {
        matches!(self, Self::Suppressed | Self::QuietHoursDeferred)
    }

    /// The upstream schema enum tokens this state derives from, for provenance.
    fn derived_from_refs(self) -> Vec<String> {
        let refs: &[&str] = match self {
            Self::Pending => &["schemas/ux/notification_route_outcome.schema.json#route_state.pending"],
            Self::Routed => &["schemas/ux/notification_route_outcome.schema.json#route_state.routed"],
            Self::Shown => &["schemas/ux/notification_route_outcome.schema.json#route_state.shown"],
            Self::Acknowledged => {
                &["schemas/ux/attention_inbox_item.schema.json#item_state.acknowledged"]
            }
            Self::Snoozed => &["schemas/ux/attention_inbox_item.schema.json#item_state.snoozed"],
            Self::QuietHoursDeferred => &[
                "schemas/ux/notification_suppression_record.schema.json#quiet_hours_mode",
                "schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json#active_quiet_hours_modes",
            ],
            Self::Suppressed => &[
                "schemas/ux/notification_suppression_record.schema.json#suppression_reason",
                "schemas/ux/notification_suppression_ledger.schema.json#suppression_lineage",
            ],
            Self::Running => &["schemas/events/activity_row.schema.json#progress_state.running"],
            Self::QueuedWaiting => {
                &["schemas/events/activity_row.schema.json#progress_state.queued_waiting"]
            }
            Self::PartiallyCompleted => {
                &["schemas/events/activity_row.schema.json#progress_state.partially_completed"]
            }
            Self::Failed => &["schemas/events/activity_row.schema.json#progress_state.failed"],
            Self::Completed => &["schemas/events/activity_row.schema.json#progress_state.completed"],
            Self::Resolved => &["schemas/ux/attention_inbox_item.schema.json#item_state.resolved"],
            Self::Dismissed => &["schemas/ux/attention_inbox_item.schema.json#item_state.dismissed"],
            Self::Archived => &["schemas/ux/m5-activity-object.schema.json#lifecycle_label.archived"],
            Self::FanoutStale => {
                &["schemas/ux/fanout_receipt.schema.json#receipt_state.stale"]
            }
            Self::FanoutUndelivered => {
                &["schemas/ux/fanout_receipt.schema.json#receipt_state.undelivered"]
            }
            Self::UnknownRequiresReview => {
                &["schemas/ux/notification_route_outcome.schema.json#route_state.unknown"]
            }
        };
        refs.iter().map(|r| (*r).to_owned()).collect()
    }
}

// ---------------------------------------------------------------------------
// Fanout channels.
// ---------------------------------------------------------------------------

/// The cross-client channels an attention object must stay truthful across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutChannelClass {
    /// The in-product activity center: the durable authoritative surface.
    InAppActivityCenter,
    /// Native OS notifications: out-of-window reminders mirroring product truth.
    OsNativeNotification,
    /// The dock / taskbar badge: a coarse, deduped pending-attention count.
    DockTaskbarBadge,
    /// The browser companion surface: a scoped mirror of the durable object.
    BrowserCompanion,
    /// The mobile companion app: review / CI / incident awareness.
    MobileCompanion,
    /// The operator / admin dashboard: managed alert and suppression visibility.
    OperatorDashboard,
}

impl FanoutChannelClass {
    /// All channels, in matrix order.
    pub const ALL: [Self; 6] = [
        Self::InAppActivityCenter,
        Self::OsNativeNotification,
        Self::DockTaskbarBadge,
        Self::BrowserCompanion,
        Self::MobileCompanion,
        Self::OperatorDashboard,
    ];

    /// Stable snake_case token for this channel.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InAppActivityCenter => "in_app_activity_center",
            Self::OsNativeNotification => "os_native_notification",
            Self::DockTaskbarBadge => "dock_taskbar_badge",
            Self::BrowserCompanion => "browser_companion",
            Self::MobileCompanion => "mobile_companion",
            Self::OperatorDashboard => "operator_dashboard",
        }
    }

    /// Stable channel id, namespaced for uniqueness.
    pub fn channel_id(self) -> String {
        format!("attention_channel.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InAppActivityCenter => "In-app activity center",
            Self::OsNativeNotification => "OS native notification",
            Self::DockTaskbarBadge => "Dock / taskbar badge",
            Self::BrowserCompanion => "Browser companion",
            Self::MobileCompanion => "Mobile companion",
            Self::OperatorDashboard => "Operator dashboard",
        }
    }
}

// ---------------------------------------------------------------------------
// Controlled vocabulary axes.
// ---------------------------------------------------------------------------

/// The named controlled-vocabulary axes this matrix defines and each object
/// declares it binds.
///
/// These are exactly the vocabularies the contract requires: severity, scope,
/// privacy class, dedupe rule, suppression, quiet-hours, stale/undelivered
/// fanout, and reopen / authoritative-object routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledVocabulary {
    /// How urgent and what class of event the attention represents.
    Severity,
    /// The namespace the attention applies to (window, workspace, session, org).
    Scope,
    /// The privacy class governing what may be shown, mirrored, or exported.
    PrivacyClass,
    /// How repeated events coalesce into one canonical attention.
    DedupeRule,
    /// Why an attention is suppressed and that it stays separate from history.
    Suppression,
    /// How quiet-hours defers an attention per its privacy class.
    QuietHours,
    /// Whether a cross-client fanout copy is delivered, stale, or undelivered.
    FanoutDelivery,
    /// Which authoritative object an attention surface reopens.
    ReopenRouting,
}

impl ControlledVocabulary {
    /// All controlled-vocabulary axes, in order.
    pub const ALL: [Self; 8] = [
        Self::Severity,
        Self::Scope,
        Self::PrivacyClass,
        Self::DedupeRule,
        Self::Suppression,
        Self::QuietHours,
        Self::FanoutDelivery,
        Self::ReopenRouting,
    ];

    /// Stable snake_case token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Severity => "severity",
            Self::Scope => "scope",
            Self::PrivacyClass => "privacy_class",
            Self::DedupeRule => "dedupe_rule",
            Self::Suppression => "suppression",
            Self::QuietHours => "quiet_hours",
            Self::FanoutDelivery => "fanout_delivery",
            Self::ReopenRouting => "reopen_routing",
        }
    }
}

// ---------------------------------------------------------------------------
// Shared, reused token vocabularies.
// ---------------------------------------------------------------------------

/// The privacy class governing what an attention may show, mirror, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPrivacyClass {
    /// A short summary is allowed on out-of-window surfaces; defer unless opted in.
    SummarySafe,
    /// Generic summary only; defer to in-product surfaces by default.
    WorkspaceSensitive,
    /// Redacted summary; may bypass quiet-hours only with explicit policy.
    SecurityCritical,
    /// No raw payload; open-app affordance only; follows admin policy.
    ManagedSensitive,
}

impl NotificationPrivacyClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummarySafe => "summary_safe",
            Self::WorkspaceSensitive => "workspace_sensitive",
            Self::SecurityCritical => "security_critical",
            Self::ManagedSensitive => "managed_sensitive",
        }
    }
}

/// The scope namespace an attention applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionScopeClass {
    /// Applies to the whole app / install.
    AppGlobal,
    /// Scoped to a single window.
    Window,
    /// Scoped to a workspace / project.
    Workspace,
    /// Scoped to a session / run.
    Session,
    /// Scoped to a collaboration / shared review.
    Collaboration,
    /// Scoped to a tenant / managed org.
    TenantOrg,
}

impl AttentionScopeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppGlobal => "app_global",
            Self::Window => "window",
            Self::Workspace => "workspace",
            Self::Session => "session",
            Self::Collaboration => "collaboration",
            Self::TenantOrg => "tenant_org",
        }
    }
}

/// Default redaction posture on export / out-of-window surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionRedactionClass {
    /// Metadata-safe default — the export default for attention surfaces.
    MetadataSafeDefault,
    /// Summary text only, never the full payload.
    SummaryOnly,
    /// Payload redacted to a stable class label.
    RedactedPayload,
    /// Counts only — no titles or summaries (the badge default).
    CountOnly,
    /// Restricted to internal support.
    InternalSupportRestricted,
}

/// The consumers that render an attention object instead of restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionConsumerClass {
    /// The shell activity center.
    ShellActivityCenter,
    /// OS native notifications.
    OsNotification,
    /// Companion / cross-client surfaces.
    CompanionCrossClient,
    /// Operator / admin dashboard.
    OperatorDashboard,
    /// Support export / bundle.
    SupportExport,
    /// Help / About truth surface.
    HelpAbout,
    /// CLI / headless inspect.
    CliHeadless,
}

/// The authoritative object an attention surface can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenTargetClass {
    /// The durable activity / job row.
    ActivityJobRow,
    /// The evidence packet behind the attention.
    EvidencePacket,
    /// The policy diff that produced a managed change.
    PolicyDiff,
    /// The review request the attention belongs to.
    ReviewRequest,
    /// The incident thread the attention belongs to.
    IncidentThread,
    /// The route object that routed the attention.
    RouteObject,
    /// The audit event the attention recorded.
    AuditEvent,
}

impl ReopenTargetClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityJobRow => "activity_job_row",
            Self::EvidencePacket => "evidence_packet",
            Self::PolicyDiff => "policy_diff",
            Self::ReviewRequest => "review_request",
            Self::IncidentThread => "incident_thread",
            Self::RouteObject => "route_object",
            Self::AuditEvent => "audit_event",
        }
    }
}

/// The delivery posture a channel admits for an attention object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutDeliveryPostureClass {
    /// The durable in-product surface: the authoritative record itself.
    DurableInProduct,
    /// An out-of-window mirror of the durable object with redacted payload.
    OutOfWindowMirror,
    /// A coarse, deduped count only.
    CoarseCountOnly,
    /// A scoped, bounded mirror of the durable object.
    ScopedMirror,
    /// A read-only operator view of managed alerts and suppression.
    ReadOnlyOperator,
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One `(token, label)` definition in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionTokenDef {
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// The reused token vocabularies and the source schemas this matrix binds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionSharedVocabulary {
    /// Severity classes (`severity` controlled vocabulary).
    pub severities: Vec<AttentionTokenDef>,
    /// Scope classes (`scope`).
    pub scopes: Vec<AttentionTokenDef>,
    /// Privacy classes (`privacy_class`).
    pub privacy_classes: Vec<AttentionTokenDef>,
    /// Dedupe rules (`dedupe_rule`).
    pub dedupe_rules: Vec<AttentionTokenDef>,
    /// Suppression reasons (`suppression`).
    pub suppression_reasons: Vec<AttentionTokenDef>,
    /// Quiet-hours behaviors (`quiet_hours`).
    pub quiet_hours_behaviors: Vec<AttentionTokenDef>,
    /// Fanout delivery states (`fanout_delivery`).
    pub fanout_delivery_states: Vec<AttentionTokenDef>,
    /// Reopen targets (`reopen_routing`).
    pub reopen_targets: Vec<AttentionTokenDef>,
    /// Action semantics (dismiss / snooze / acknowledge / resolve / mute).
    pub action_semantics: Vec<AttentionTokenDef>,
    /// Retention classes governing how long state is kept and where.
    pub retention_classes: Vec<AttentionTokenDef>,
    /// Redaction classes.
    pub redaction_classes: Vec<AttentionTokenDef>,
    /// Consumer classes.
    pub consumer_classes: Vec<AttentionTokenDef>,
    /// The boundary schemas this matrix binds as truth sources.
    pub source_schema_refs: Vec<String>,
}

/// One state in the unified vocabulary, with provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionStateTerm {
    /// The state.
    pub state: AttentionStateClass,
    /// Stable token (equals `state.as_str()`), surfaced for reuse by consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether this state requires a durable authoritative object (never
    /// toast-only).
    pub requires_durable_object: bool,
    /// Whether this state is a cross-client delivery gap (stale / undelivered).
    pub is_delivery_gap: bool,
    /// Whether this state is a suppression / quiet-hours state stored separately
    /// from audit history.
    pub is_suppression: bool,
    /// The upstream schema enum tokens this state derives from.
    pub derived_from_refs: Vec<String>,
}

/// One required field an attention object must carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionFieldDef {
    /// Stable field id.
    pub field_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the field is required on every instance of the object.
    pub required: bool,
}

/// The retention rule an attention object applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRetentionRule {
    /// The retention class token (`retention_classes` vocabulary).
    pub retention_class: String,
    /// Whether suppression / quiet-hours state for this object is stored
    /// separately from audit history.
    pub separate_from_audit_history: bool,
    /// One reviewable sentence stating the rule.
    pub rule: String,
}

/// One attention object-family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionObjectEntry {
    /// The object family.
    pub object: AttentionObjectClass,
    /// Stable, namespaced object id.
    pub object_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the object.
    pub summary: String,
    /// The canonical boundary schema(s) this object binds.
    pub canonical_schema_refs: Vec<String>,
    /// The crate module(s) that already produce this truth.
    pub produced_by_refs: Vec<String>,
    /// The proof packet (contract, fixture, or evidence) that keeps this object
    /// current. Stable promotion fails when this is empty.
    pub proof_packet_ref: String,
    /// The consumers that render this object.
    pub consumed_by: Vec<AttentionConsumerClass>,
    /// The states from the unified vocabulary this object can show.
    pub applicable_states: Vec<AttentionStateClass>,
    /// The controlled-vocabulary axes this object binds.
    pub controlled_vocabularies: Vec<ControlledVocabulary>,
    /// The required fields this object must carry.
    pub required_fields: Vec<AttentionFieldDef>,
    /// The retention rule this object applies.
    pub retention_rule: AttentionRetentionRule,
    /// The default privacy class this object renders under.
    pub default_privacy: NotificationPrivacyClass,
    /// The default redaction posture on export.
    pub default_redaction: AttentionRedactionClass,
    /// The default scope namespace this object applies to.
    pub scope_default: AttentionScopeClass,
    /// Whether the object is a durable authoritative record (never toast-only).
    pub carries_durable_record: bool,
    /// The authoritative objects this object can reopen instead of reissuing a
    /// blind side effect.
    pub reopen_targets: Vec<ReopenTargetClass>,
    /// Whether the object is locally inspectable (never console-only / portal-only).
    pub locally_inspectable: bool,
    /// One reviewable sentence stating the object's attention-routing honesty rule.
    pub boundary_note: String,
    /// Whether the object is typed (never reduced to a toast-only / prose-only view).
    pub typed_not_toast_only: bool,
}

impl AttentionObjectEntry {
    /// Whether the object binds the named controlled-vocabulary axis.
    pub fn binds(&self, vocab: ControlledVocabulary) -> bool {
        self.controlled_vocabularies.contains(&vocab)
    }

    /// Whether the object can show a given state.
    pub fn can_show(&self, state: AttentionStateClass) -> bool {
        self.applicable_states.contains(&state)
    }

    /// Whether the object can reopen a given authoritative target.
    pub fn can_reopen(&self, target: ReopenTargetClass) -> bool {
        self.reopen_targets.contains(&target)
    }
}

/// One fanout-channel entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutChannelEntry {
    /// The channel.
    pub channel: FanoutChannelClass,
    /// Stable, namespaced channel id.
    pub channel_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the channel.
    pub summary: String,
    /// The delivery posture this channel admits.
    pub delivery_posture: FanoutDeliveryPostureClass,
    /// The strongest privacy class this channel may render in detail.
    pub privacy_ceiling: NotificationPrivacyClass,
    /// Whether this channel may bypass the in-product preview/approval flow.
    /// Always false: fanout cannot bypass preview/approval.
    pub can_bypass_preview_approval: bool,
    /// Whether this channel carries a reopen path back to the authoritative object.
    pub carries_durable_reopen: bool,
    /// Whether this channel mirrors the in-product authoritative object rather
    /// than being the authoritative record itself.
    pub mirrors_authoritative: bool,
    /// The default redaction posture this channel applies.
    pub default_redaction: AttentionRedactionClass,
    /// Whether this channel respects quiet-hours (always true).
    pub quiet_hours_respected: bool,
    /// One reviewable sentence of channel-specific notes.
    pub notes: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionMatrixInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built matrix satisfies the invariant.
    pub holds: bool,
}

/// The frozen attention-routing matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionRoutingMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_attention_routing_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the matrix binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the matrix.
    pub summary: String,
    /// The reused token vocabularies and bound source schemas.
    pub shared_vocabulary: AttentionSharedVocabulary,
    /// The unified state vocabulary.
    pub state_vocabulary: Vec<AttentionStateTerm>,
    /// The attention object-family entries.
    pub objects: Vec<AttentionObjectEntry>,
    /// The fanout-channel entries.
    pub channels: Vec<FanoutChannelEntry>,
    /// The computed invariants.
    pub invariants: Vec<AttentionMatrixInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the matrix fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionMatrixValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for AttentionMatrixValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attention-routing matrix invalid: {}", self.reason)
    }
}

impl std::error::Error for AttentionMatrixValidationError {}

impl AttentionRoutingMatrix {
    /// Returns the entry for an object family, if present.
    pub fn object(&self, object: AttentionObjectClass) -> Option<&AttentionObjectEntry> {
        self.objects.iter().find(|o| o.object == object)
    }

    /// Returns the entry for a fanout channel, if present.
    pub fn channel(&self, channel: FanoutChannelClass) -> Option<&FanoutChannelEntry> {
        self.channels.iter().find(|c| c.channel == channel)
    }

    /// Returns the state term for a state, if present.
    pub fn state_term(&self, state: AttentionStateClass) -> Option<&AttentionStateTerm> {
        self.state_vocabulary.iter().find(|t| t.state == state)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the matrix, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_shared = self
            .shared_vocabulary
            .source_schema_refs
            .iter()
            .map(String::as_str);
        let from_states = self
            .state_vocabulary
            .iter()
            .flat_map(|t| t.derived_from_refs.iter().map(String::as_str));
        let from_objects = self.objects.iter().flat_map(|o| {
            o.canonical_schema_refs
                .iter()
                .map(String::as_str)
                .chain(o.produced_by_refs.iter().map(String::as_str))
                .chain(std::iter::once(o.proof_packet_ref.as_str()))
        });
        let from_gate = std::iter::once(self.freeze_gate_ref.as_str());
        from_shared
            .chain(from_states)
            .chain(from_objects)
            .chain(from_gate)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`AttentionMatrixInvariant`]s with the uniqueness
    /// and completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), AttentionMatrixValidationError> {
        let fail = |reason: String| Err(AttentionMatrixValidationError { reason });

        if self.record_kind != M5_ATTENTION_ROUTING_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ATTENTION_ROUTING_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every object family, channel, and state is present exactly once.
        for object in AttentionObjectClass::ALL {
            if self.objects.iter().filter(|o| o.object == object).count() != 1 {
                return fail(format!(
                    "object {} not present exactly once",
                    object.as_str()
                ));
            }
        }
        for channel in FanoutChannelClass::ALL {
            if self
                .channels
                .iter()
                .filter(|c| c.channel == channel)
                .count()
                != 1
            {
                return fail(format!(
                    "channel {} not present exactly once",
                    channel.as_str()
                ));
            }
        }
        for state in AttentionStateClass::ALL {
            if self
                .state_vocabulary
                .iter()
                .filter(|t| t.state == state)
                .count()
                != 1
            {
                return fail(format!("state {} not present exactly once", state.as_str()));
            }
        }

        // Stable ids and tokens are unique.
        if !all_unique(self.objects.iter().map(|o| o.object_id.as_str())) {
            return fail("object ids are not unique".to_owned());
        }
        if !all_unique(self.channels.iter().map(|c| c.channel_id.as_str())) {
            return fail("channel ids are not unique".to_owned());
        }
        if !all_unique(self.state_vocabulary.iter().map(|t| t.token.as_str())) {
            return fail("state tokens are not unique".to_owned());
        }

        // Per-object structural floor: typed, evidenced, fielded, proven.
        for entry in &self.objects {
            if entry.object_id != entry.object.object_id() {
                return fail(format!("object id mismatch for {}", entry.object.as_str()));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!("object {} cites no schema", entry.object.as_str()));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!("object {} has no producer", entry.object.as_str()));
            }
            if entry.proof_packet_ref.is_empty() {
                return fail(format!(
                    "object {} has no mapped proof packet",
                    entry.object.as_str()
                ));
            }
            if entry.applicable_states.is_empty() {
                return fail(format!(
                    "object {} declares no states",
                    entry.object.as_str()
                ));
            }
            if entry.controlled_vocabularies.is_empty() {
                return fail(format!(
                    "object {} binds no controlled vocabulary",
                    entry.object.as_str()
                ));
            }
            if entry.required_fields.is_empty() {
                return fail(format!(
                    "object {} declares no required fields",
                    entry.object.as_str()
                ));
            }
            for state in &entry.applicable_states {
                if self.state_term(*state).is_none() {
                    return fail(format!(
                        "object {} references undefined state {}",
                        entry.object.as_str(),
                        state.as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("matrix is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

pub(crate) fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
pub(crate) fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical attention-routing matrix.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the built objects, channels, and states, so an inconsistent edit
/// flips an invariant rather than silently passing.
pub fn attention_routing_matrix() -> AttentionRoutingMatrix {
    let state_vocabulary = build_state_vocabulary();
    let objects = build_objects();
    let channels = build_channels();
    let shared_vocabulary = build_shared_vocabulary(&objects);
    let invariants = compute_invariants(&objects, &channels, &state_vocabulary);

    AttentionRoutingMatrix {
        record_kind: M5_ATTENTION_ROUTING_RECORD_KIND.to_owned(),
        m5_attention_routing_schema_version: M5_ATTENTION_ROUTING_SCHEMA_VERSION,
        schema_ref: M5_ATTENTION_ROUTING_SCHEMA_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        as_of: M5_ATTENTION_ROUTING_AS_OF.to_owned(),
        freeze_gate_ref: M5_ATTENTION_ROUTING_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed matrix for Aureline's attention routing — notification \
                  envelopes, durable activity objects, badge aggregates, fanout receipts, routing \
                  context, privacy classes, and action/retention semantics — across the in-app \
                  activity center, OS notifications, dock/taskbar badge, browser and mobile \
                  companions, and operator dashboard, with each object mapped to the proof packet \
                  that keeps it current. Attention is routed, typed, privacy-aware, and reopen-safe: \
                  no long-running or reviewable work lives only in a toast, badges derive from \
                  deduped durable items, fanout cannot bypass preview/approval, suppression and \
                  quiet-hours stay separate from audit history, and every surface reopens the \
                  authoritative object."
            .to_owned(),
        shared_vocabulary,
        state_vocabulary,
        objects,
        channels,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_state_vocabulary() -> Vec<AttentionStateTerm> {
    AttentionStateClass::ALL
        .iter()
        .map(|state| AttentionStateTerm {
            state: *state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            requires_durable_object: state.requires_durable_object(),
            is_delivery_gap: state.is_delivery_gap(),
            is_suppression: state.is_suppression(),
            derived_from_refs: state.derived_from_refs(),
        })
        .collect()
}

fn field(field_id: &str, label: &str, required: bool) -> AttentionFieldDef {
    AttentionFieldDef {
        field_id: field_id.to_owned(),
        label: label.to_owned(),
        required,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn retention(
    retention_class: &str,
    separate_from_audit_history: bool,
    rule: &str,
) -> AttentionRetentionRule {
    AttentionRetentionRule {
        retention_class: retention_class.to_owned(),
        separate_from_audit_history,
        rule: rule.to_owned(),
    }
}

fn build_objects() -> Vec<AttentionObjectEntry> {
    use AttentionConsumerClass::*;
    use AttentionStateClass::*;
    use ControlledVocabulary::*;
    use ReopenTargetClass::*;

    vec![
    AttentionObjectEntry {
        object: AttentionObjectClass::NotificationEnvelope,
        object_id: AttentionObjectClass::NotificationEnvelope.object_id(),
        label: AttentionObjectClass::NotificationEnvelope.label().to_owned(),
        summary: "The typed, privacy-classed, deduped unit of attention: a stable id, source \
                  subsystem, severity and scope, dedupe key, recommended surfaces, a stable action \
                  target, and a route back to the authoritative object it came from."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/notification_envelope.schema.json",
            "schemas/ux/m5-os-notification-envelope.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/notifications/envelope.rs",
            "crates/aureline-shell/src/notification_envelope_corpus/mod.rs",
        ]),
        proof_packet_ref: "docs/ux/notification_envelope_contract.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            OsNotification,
            CompanionCrossClient,
            SupportExport,
            HelpAbout,
            CliHeadless,
        ],
        applicable_states: vec![
            Pending,
            Routed,
            Shown,
            Acknowledged,
            Snoozed,
            QuietHoursDeferred,
            Suppressed,
            Resolved,
            Dismissed,
            Archived,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            Severity,
            Scope,
            PrivacyClass,
            DedupeRule,
            Suppression,
            QuietHours,
            ReopenRouting,
        ],
        required_fields: vec![
            field("notification_envelope_id", "Envelope id", true),
            field("canonical_event_id", "Canonical event id", true),
            field("source_subsystem", "Source subsystem", true),
            field("severity_class", "Severity", true),
            field("scope_ref", "Scope", true),
            field("privacy_class", "Privacy class", true),
            field("dedupe_key_ref", "Dedupe key", true),
            field("recommended_surfaces", "Recommended surfaces", true),
            field("stable_action_target", "Stable action target", true),
            field("reopen_target_ref", "Reopen target", true),
            field("created_at", "Created", true),
            field("updated_at", "Updated", true),
        ],
        retention_rule: retention(
            "durable_until_archived",
            true,
            "The envelope is durable until resolved or archived; its suppression and quiet-hours \
             markers are stored separately from audit history, never overwriting it.",
        ),
        default_privacy: NotificationPrivacyClass::SummarySafe,
        default_redaction: AttentionRedactionClass::SummaryOnly,
        scope_default: AttentionScopeClass::Workspace,
        carries_durable_record: true,
        reopen_targets: vec![
            ActivityJobRow,
            EvidencePacket,
            ReviewRequest,
            IncidentThread,
            RouteObject,
            AuditEvent,
        ],
        locally_inspectable: true,
        boundary_note: "Every envelope carries a stable action target and a reopen route; it never \
                        encodes meaningful state only in an ephemeral toast and never widens its \
                        privacy class on fanout."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::ActivityObject,
        object_id: AttentionObjectClass::ActivityObject.object_id(),
        label: AttentionObjectClass::ActivityObject.label().to_owned(),
        summary: "A long-running or reviewable durable job: its family, actor subsystem, phase and \
                  progress, cancel/retry affordances, evidence link, cost/trust/policy-impact flags, \
                  and a reopen anchor — kept reviewable after focus loss, never toast-only."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/m5-activity-object.schema.json",
            "schemas/events/activity_row.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/m5_activity_objects/mod.rs",
            "crates/aureline-shell/src/activity_center/mod.rs",
        ]),
        proof_packet_ref: "docs/ux/activity_center_alpha.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            CliHeadless,
            SupportExport,
            HelpAbout,
            OperatorDashboard,
        ],
        applicable_states: vec![
            Running,
            QueuedWaiting,
            PartiallyCompleted,
            Failed,
            Completed,
            Resolved,
            Archived,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![PrivacyClass, Suppression, ReopenRouting],
        required_fields: vec![
            field("activity_job_id", "Job id", true),
            field("job_family", "Job family", true),
            field("actor_subsystem", "Actor subsystem", true),
            field("phase", "Phase", true),
            field("progress_state", "Progress state", true),
            field("cancel_affordances", "Cancel affordances", true),
            field("retry_affordances", "Retry affordances", true),
            field("evidence_link", "Evidence link", true),
            field("cost_flag", "Cost flag", false),
            field("trust_flag", "Trust flag", false),
            field("policy_impact_flag", "Policy-impact flag", false),
            field("reopen_anchor_ref", "Reopen anchor", true),
        ],
        retention_rule: retention(
            "durable_until_archived",
            false,
            "The activity object is the durable authoritative record: it survives focus loss and \
             restart and is retained until archived or expired by policy, never reduced to a toast.",
        ),
        default_privacy: NotificationPrivacyClass::WorkspaceSensitive,
        default_redaction: AttentionRedactionClass::MetadataSafeDefault,
        scope_default: AttentionScopeClass::Workspace,
        carries_durable_record: true,
        reopen_targets: vec![ActivityJobRow, EvidencePacket, ReviewRequest, RouteObject],
        locally_inspectable: true,
        boundary_note: "Long-running and reviewable work is a durable activity object with cancel/\
                        retry affordances and a reopen anchor; a toast may announce it but is never \
                        its only record."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::BadgeAggregate,
        object_id: AttentionObjectClass::BadgeAggregate.object_id(),
        label: AttentionObjectClass::BadgeAggregate.label().to_owned(),
        summary: "A coarse, deduped pending-attention count per scope, derived from durable items \
                  rather than raw event spam, with its count class, freshness, and the muted and \
                  suppressed reasons that explain the number."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json",
        ]),
        produced_by_refs: strvec(&["crates/aureline-shell/src/badge_aggregate_stable/model.rs"]),
        proof_packet_ref: "docs/m5/notification-privacy-and-badges.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            OsNotification,
            CompanionCrossClient,
            OperatorDashboard,
            SupportExport,
        ],
        applicable_states: vec![
            Shown,
            Acknowledged,
            Dismissed,
            Suppressed,
            QuietHoursDeferred,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            Scope,
            PrivacyClass,
            DedupeRule,
            Suppression,
            QuietHours,
            ReopenRouting,
        ],
        required_fields: vec![
            field("scope_key", "Scope key", true),
            field("count", "Count", true),
            field("count_class", "Count class", true),
            field("freshness", "Freshness", true),
            field("deduped_objects", "Deduped objects", true),
            field("muted_reasons", "Muted reasons", true),
            field("suppressed_reasons", "Suppressed reasons", true),
            field("active_quiet_hours_modes", "Active quiet-hours modes", true),
        ],
        retention_rule: retention(
            "durable_until_resolved",
            true,
            "The badge count derives from the deduped set of durable items and clears as they \
             resolve; its muted and suppressed reasons are stored separately from audit history.",
        ),
        default_privacy: NotificationPrivacyClass::SummarySafe,
        default_redaction: AttentionRedactionClass::CountOnly,
        scope_default: AttentionScopeClass::AppGlobal,
        carries_durable_record: true,
        reopen_targets: vec![ActivityJobRow, RouteObject],
        locally_inspectable: true,
        boundary_note: "A badge number is always a count of deduped durable items the user can open; \
                        it is never derived from duplicate raw events or hidden provider state."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::FanoutReceipt,
        object_id: AttentionObjectClass::FanoutReceipt.object_id(),
        label: AttentionObjectClass::FanoutReceipt.label().to_owned(),
        summary: "Per-destination cross-client delivery truth: which client received which envelope, \
                  its delivery state, and an explicit stale or undelivered reason — failed fanout is \
                  visible truth, not silent best effort."
            .to_owned(),
        canonical_schema_refs: strvec(&["schemas/ux/fanout_receipt.schema.json"]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/attention_router/outcome.rs",
            "crates/aureline-shell/src/attention_router/mod.rs",
        ]),
        proof_packet_ref: "docs/ux/notification_delivery_contract.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            OsNotification,
            CompanionCrossClient,
            OperatorDashboard,
            SupportExport,
            CliHeadless,
        ],
        applicable_states: vec![
            Routed,
            Shown,
            FanoutStale,
            FanoutUndelivered,
            Suppressed,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            PrivacyClass,
            DedupeRule,
            Suppression,
            FanoutDelivery,
            ReopenRouting,
        ],
        required_fields: vec![
            field("fanout_receipt_id", "Receipt id", true),
            field("source_notification_envelope_id_ref", "Source envelope", true),
            field("canonical_event_id", "Canonical event id", true),
            field("fanout_surface_class", "Fanout surface class", true),
            field("client_scope", "Client scope", true),
            field("receipt_state", "Receipt state", true),
            field("stale_or_undelivered_reason", "Stale / undelivered reason", false),
            field("dedupe_key_scheme", "Dedupe key scheme", true),
            field("reopen_target_ref", "Reopen target", true),
            field("redaction_class", "Redaction class", true),
            field("suppression_reasons", "Suppression reasons", false),
            field("minted_at", "Minted at", true),
        ],
        retention_rule: retention(
            "durable_until_archived",
            true,
            "Each receipt is durable and names its destination and delivery state; a stale or \
             undelivered copy is labeled rather than counted as delivered, and suppression-by-policy \
             is tracked separately from audit history.",
        ),
        default_privacy: NotificationPrivacyClass::SummarySafe,
        default_redaction: AttentionRedactionClass::RedactedPayload,
        scope_default: AttentionScopeClass::Session,
        carries_durable_record: true,
        reopen_targets: vec![ActivityJobRow, RouteObject, IncidentThread],
        locally_inspectable: true,
        boundary_note: "A fanout that goes stale or fails to deliver is recorded with an explicit \
                        reason and still routes back to the authoritative object; it is never \
                        silently dropped or reported as delivered."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::RoutingContext,
        object_id: AttentionObjectClass::RoutingContext.object_id(),
        label: AttentionObjectClass::RoutingContext.label().to_owned(),
        summary: "How an event is severity/scope/privacy-classed and deduped, which surfaces it is \
                  routed to versus deferred by quiet-hours, and the reopen route it preserves — the \
                  routing decision made visible and reviewable."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/notification_route_outcome.schema.json",
            "schemas/ux/notification_event.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/attention_router/mod.rs",
            "crates/aureline-shell/src/notifications/router.rs",
            "crates/aureline-shell/src/notifications/routes.rs",
        ]),
        proof_packet_ref: "docs/ux/notification_routing_seed.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            OsNotification,
            CompanionCrossClient,
            SupportExport,
            CliHeadless,
        ],
        applicable_states: vec![
            Pending,
            Routed,
            Shown,
            QuietHoursDeferred,
            Suppressed,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![
            Severity,
            Scope,
            PrivacyClass,
            DedupeRule,
            Suppression,
            QuietHours,
            FanoutDelivery,
            ReopenRouting,
        ],
        required_fields: vec![
            field("route_context_id", "Route context id", true),
            field("canonical_event_id", "Canonical event id", true),
            field("severity_class", "Severity", true),
            field("scope_ref", "Scope", true),
            field("privacy_class", "Privacy class", true),
            field("recommended_surfaces", "Recommended surfaces", true),
            field("chosen_surfaces", "Chosen surfaces", true),
            field("dedupe_key_ref", "Dedupe key", true),
            field("quiet_hours_mode", "Quiet-hours mode", true),
            field("reopen_target_ref", "Reopen target", true),
        ],
        retention_rule: retention(
            "durable_until_resolved",
            true,
            "The routing decision is recorded with the surfaces it chose and deferred; quiet-hours \
             and suppression deferrals are stored separately from audit history.",
        ),
        default_privacy: NotificationPrivacyClass::WorkspaceSensitive,
        default_redaction: AttentionRedactionClass::SummaryOnly,
        scope_default: AttentionScopeClass::Session,
        carries_durable_record: true,
        reopen_targets: vec![
            ActivityJobRow,
            EvidencePacket,
            ReviewRequest,
            IncidentThread,
            RouteObject,
            AuditEvent,
            PolicyDiff,
        ],
        locally_inspectable: true,
        boundary_note: "The route a notification took — surfaces chosen, deferred, or suppressed — is \
                        itself reviewable and reopen-safe, so routing is never an opaque side effect."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::PrivacyClass,
        object_id: AttentionObjectClass::PrivacyClass.object_id(),
        label: AttentionObjectClass::PrivacyClass.label().to_owned(),
        summary: "The per-class rule governing what an attention may show on a lock screen, mirror \
                  to a companion summary, or place in a support export, and how quiet-hours defers \
                  it — message copy is localizable, but the privacy class is the contract."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/notification_envelope.schema.json",
            "schemas/ux/notification_suppression_record.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/notifications/quiet_hours.rs",
            "crates/aureline-shell/src/notifications/mod.rs",
        ]),
        proof_packet_ref: "docs/ux/notification_privacy_dedupe_audit.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            OsNotification,
            CompanionCrossClient,
            OperatorDashboard,
            SupportExport,
            HelpAbout,
        ],
        applicable_states: vec![Shown, QuietHoursDeferred, Suppressed, UnknownRequiresReview],
        controlled_vocabularies: vec![PrivacyClass, QuietHours, Suppression, FanoutDelivery, ReopenRouting],
        required_fields: vec![
            field("privacy_rule_id", "Privacy rule id", true),
            field("privacy_class", "Privacy class", true),
            field("lock_screen_payload_policy", "Lock-screen payload policy", true),
            field("companion_summary_policy", "Companion summary policy", true),
            field("support_export_policy", "Support export policy", true),
            field("quiet_hours_behavior", "Quiet-hours behavior", true),
        ],
        retention_rule: retention(
            "durable_until_archived",
            true,
            "The privacy-class rule is durable governance and applies on every surface; the \
             quiet-hours and suppression state it drives is kept separate from audit history.",
        ),
        default_privacy: NotificationPrivacyClass::SecurityCritical,
        default_redaction: AttentionRedactionClass::RedactedPayload,
        scope_default: AttentionScopeClass::AppGlobal,
        carries_durable_record: true,
        reopen_targets: vec![RouteObject, AuditEvent],
        locally_inspectable: true,
        boundary_note: "Privacy class governs every surface uniformly: a class can only be relaxed by \
                        explicit policy, and a stricter class is never silently widened on fanout."
            .to_owned(),
        typed_not_toast_only: true,
    },
    AttentionObjectEntry {
        object: AttentionObjectClass::ActionRetentionSemantics,
        object_id: AttentionObjectClass::ActionRetentionSemantics.object_id(),
        label: AttentionObjectClass::ActionRetentionSemantics.label().to_owned(),
        summary: "The distinct dismiss, snooze, acknowledge, resolve, and mute actions and the \
                  retention each implies: dismiss and acknowledge clear the badge but keep the \
                  record, snooze returns on a condition, resolve closes on the underlying change."
            .to_owned(),
        canonical_schema_refs: strvec(&[
            "schemas/ux/notification_suppression_record.schema.json",
            "schemas/ux/notification_suppression_ledger.schema.json",
            "schemas/ux/attention_inbox_item.schema.json",
        ]),
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/notifications/quiet_hours.rs",
            "crates/aureline-shell/src/attention_router/mod.rs",
        ]),
        proof_packet_ref: "docs/ux/notification_action_grammar.md".to_owned(),
        consumed_by: vec![
            ShellActivityCenter,
            CompanionCrossClient,
            SupportExport,
            HelpAbout,
            CliHeadless,
        ],
        applicable_states: vec![
            Acknowledged,
            Snoozed,
            Dismissed,
            Resolved,
            Suppressed,
            Archived,
            UnknownRequiresReview,
        ],
        controlled_vocabularies: vec![PrivacyClass, Suppression, QuietHours, ReopenRouting],
        required_fields: vec![
            field("action_semantic_id", "Action semantic id", true),
            field("action_kind", "Action kind", true),
            field("retention_class", "Retention class", true),
            field("resume_condition", "Resume condition", false),
            field("clears_badge", "Clears badge", true),
            field("keeps_underlying_record", "Keeps underlying record", true),
            field("reopen_target_ref", "Reopen target", true),
        ],
        retention_rule: retention(
            "suppression_state_separate",
            true,
            "Dismiss and acknowledge clear the badge without erasing the underlying state; snooze and \
             mute defer it with a resume condition; resolve closes it on the underlying change — and \
             none of these overwrite audit history.",
        ),
        default_privacy: NotificationPrivacyClass::WorkspaceSensitive,
        default_redaction: AttentionRedactionClass::MetadataSafeDefault,
        scope_default: AttentionScopeClass::Workspace,
        carries_durable_record: true,
        reopen_targets: vec![ActivityJobRow, RouteObject, AuditEvent],
        locally_inspectable: true,
        boundary_note: "Dismiss, snooze, acknowledge, resolve, and mute are distinct: clearing a \
                        badge never erases the durable record, and a snooze or mute always names its \
                        resume condition."
            .to_owned(),
        typed_not_toast_only: true,
    },
    ]
}

fn build_channels() -> Vec<FanoutChannelEntry> {
    vec![
        FanoutChannelEntry {
            channel: FanoutChannelClass::InAppActivityCenter,
            channel_id: FanoutChannelClass::InAppActivityCenter.channel_id(),
            label: FanoutChannelClass::InAppActivityCenter.label().to_owned(),
            summary: "The durable in-product activity center: the authoritative attention surface \
                      that holds the full envelope and activity object."
                .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::DurableInProduct,
            privacy_ceiling: NotificationPrivacyClass::ManagedSensitive,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: false,
            default_redaction: AttentionRedactionClass::MetadataSafeDefault,
            quiet_hours_respected: true,
            notes:
                "The authoritative surface every other channel mirrors; dangerous actions route \
                    through its in-product preview/approval flow."
                    .to_owned(),
        },
        FanoutChannelEntry {
            channel: FanoutChannelClass::OsNativeNotification,
            channel_id: FanoutChannelClass::OsNativeNotification.channel_id(),
            label: FanoutChannelClass::OsNativeNotification.label().to_owned(),
            summary: "Native OS notifications for out-of-window reminders and time-sensitive \
                      completions, mirroring product truth with a redacted payload."
                .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::OutOfWindowMirror,
            privacy_ceiling: NotificationPrivacyClass::SummarySafe,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: true,
            default_redaction: AttentionRedactionClass::SummaryOnly,
            quiet_hours_respected: true,
            notes: "Shows a summary only and opens the in-app authoritative object; it can never \
                    apply a dangerous action without the in-product approval flow."
                .to_owned(),
        },
        FanoutChannelEntry {
            channel: FanoutChannelClass::DockTaskbarBadge,
            channel_id: FanoutChannelClass::DockTaskbarBadge.channel_id(),
            label: FanoutChannelClass::DockTaskbarBadge.label().to_owned(),
            summary: "The dock / taskbar badge: a coarse, deduped pending-attention count for the \
                      app icon."
                .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::CoarseCountOnly,
            privacy_ceiling: NotificationPrivacyClass::SummarySafe,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: true,
            default_redaction: AttentionRedactionClass::CountOnly,
            quiet_hours_respected: true,
            notes:
                "Carries a count only, derived from the deduped badge aggregate; opening it lands \
                    on the durable activity center."
                    .to_owned(),
        },
        FanoutChannelEntry {
            channel: FanoutChannelClass::BrowserCompanion,
            channel_id: FanoutChannelClass::BrowserCompanion.channel_id(),
            label: FanoutChannelClass::BrowserCompanion.label().to_owned(),
            summary: "The browser companion: a scoped, bounded mirror of the durable attention \
                      object with failed and stale fanout labeled."
                .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::ScopedMirror,
            privacy_ceiling: NotificationPrivacyClass::WorkspaceSensitive,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: true,
            default_redaction: AttentionRedactionClass::RedactedPayload,
            quiet_hours_respected: true,
            notes:
                "A bounded fan-out from the same durable object model; dangerous actions hand off \
                    to the desktop in-product review surface."
                    .to_owned(),
        },
        FanoutChannelEntry {
            channel: FanoutChannelClass::MobileCompanion,
            channel_id: FanoutChannelClass::MobileCompanion.channel_id(),
            label: FanoutChannelClass::MobileCompanion.label().to_owned(),
            summary:
                "The mobile companion: review, CI status, and incident awareness mirrored from \
                      the durable object with privacy-class-aware payloads."
                    .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::ScopedMirror,
            privacy_ceiling: NotificationPrivacyClass::SummarySafe,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: true,
            default_redaction: AttentionRedactionClass::SummaryOnly,
            quiet_hours_respected: true,
            notes:
                "Surfaces awareness only; approving a change or running a dangerous action routes \
                    back to the in-product preview/approval flow."
                    .to_owned(),
        },
        FanoutChannelEntry {
            channel: FanoutChannelClass::OperatorDashboard,
            channel_id: FanoutChannelClass::OperatorDashboard.channel_id(),
            label: FanoutChannelClass::OperatorDashboard.label().to_owned(),
            summary:
                "The operator / admin dashboard: a read-only view of managed alerts, routing, \
                      and suppression state across the fleet."
                    .to_owned(),
            delivery_posture: FanoutDeliveryPostureClass::ReadOnlyOperator,
            privacy_ceiling: NotificationPrivacyClass::ManagedSensitive,
            can_bypass_preview_approval: false,
            carries_durable_reopen: true,
            mirrors_authoritative: true,
            default_redaction: AttentionRedactionClass::InternalSupportRestricted,
            quiet_hours_respected: true,
            notes:
                "Shows managed alert routing and suppression as read-only truth; it never issues a \
                    side effect that bypasses the in-product approval flow."
                    .to_owned(),
        },
    ]
}

fn build_shared_vocabulary(objects: &[AttentionObjectEntry]) -> AttentionSharedVocabulary {
    let def = |token: &str, label: &str| AttentionTokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    };

    // The bound source schemas are exactly the union of every object's cited
    // schema, plus the durable activity row the badge and progress lean on.
    let mut source_schema_refs: Vec<String> = objects
        .iter()
        .flat_map(|o| o.canonical_schema_refs.iter().cloned())
        .chain(std::iter::once(
            "schemas/events/activity_row.schema.json".to_owned(),
        ))
        .collect();
    source_schema_refs.sort();
    source_schema_refs.dedup();

    AttentionSharedVocabulary {
        severities: vec![
            def("minor_success", "Minor success / undo"),
            def("informational", "Informational"),
            def("degraded", "Workspace degradation"),
            def("progress", "Long-running progress"),
            def(
                "handoff_actionable",
                "Review / incident / collaboration handoff",
            ),
            def("security_advisory", "Security advisory / revocation"),
        ],
        scopes: vec![
            def("app_global", "App-global"),
            def("window", "Window"),
            def("workspace", "Workspace"),
            def("session", "Session"),
            def("collaboration", "Collaboration"),
            def("tenant_org", "Tenant / org"),
        ],
        privacy_classes: vec![
            def("summary_safe", "Summary-safe"),
            def("workspace_sensitive", "Workspace-sensitive"),
            def("security_critical", "Security-critical"),
            def("managed_sensitive", "Managed-sensitive"),
        ],
        dedupe_rules: vec![
            def("canonical_key_coalesce", "Coalesce by canonical key"),
            def(
                "root_cause_collapse",
                "Collapse repeats from one root cause",
            ),
            def("latest_supersedes", "Latest supersedes prior"),
            def("count_rollup", "Roll up into a count"),
            def("no_dedupe", "No dedupe"),
        ],
        suppression_reasons: vec![
            def("user_muted_source", "User muted the source"),
            def("policy_suppressed", "Suppressed by policy"),
            def("already_acknowledged", "Already acknowledged"),
            def("superseded", "Superseded by a newer event"),
            def("rate_limited", "Rate-limited"),
        ],
        quiet_hours_behaviors: vec![
            def("defer_unless_opted_in", "Defer unless user opted in"),
            def("defer_to_in_product", "Defer to in-product surfaces"),
            def("may_bypass_with_policy", "May bypass with explicit policy"),
            def("follow_admin_policy", "Follow admin policy"),
            def("always_defer", "Always defer"),
        ],
        fanout_delivery_states: vec![
            def("delivered", "Delivered"),
            def("pending", "Pending"),
            def("stale", "Stale"),
            def("undelivered", "Undelivered"),
            def("superseded", "Superseded"),
            def("suppressed_by_policy", "Suppressed by policy"),
        ],
        reopen_targets: vec![
            def("activity_job_row", "Activity / job row"),
            def("evidence_packet", "Evidence packet"),
            def("policy_diff", "Policy diff"),
            def("review_request", "Review request"),
            def("incident_thread", "Incident thread"),
            def("route_object", "Route object"),
            def("audit_event", "Audit event"),
        ],
        action_semantics: vec![
            def("dismiss", "Dismiss — clear badge, keep record"),
            def("snooze", "Snooze — defer with resume condition"),
            def("acknowledge", "Acknowledge — mark read, keep record"),
            def("resolve", "Resolve — close on underlying change"),
            def("mute", "Mute — suppress from source until unmuted"),
        ],
        retention_classes: vec![
            def("durable_until_resolved", "Durable until resolved"),
            def("durable_until_archived", "Durable until archived"),
            def("policy_expiry", "Expired by policy"),
            def("ephemeral_toast", "Ephemeral toast (announcement only)"),
            def(
                "suppression_state_separate",
                "Suppression state stored separately",
            ),
        ],
        redaction_classes: vec![
            def("metadata_safe_default", "Metadata-safe default"),
            def("summary_only", "Summary only"),
            def("redacted_payload", "Redacted payload"),
            def("count_only", "Count only"),
            def("internal_support_restricted", "Internal-support restricted"),
        ],
        consumer_classes: vec![
            def("shell_activity_center", "Shell activity center"),
            def("os_notification", "OS notification"),
            def("companion_cross_client", "Companion / cross-client"),
            def("operator_dashboard", "Operator dashboard"),
            def("support_export", "Support export"),
            def("help_about", "Help / About"),
            def("cli_headless", "CLI / headless"),
        ],
        source_schema_refs,
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> AttentionMatrixInvariant {
    AttentionMatrixInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    objects: &[AttentionObjectEntry],
    channels: &[FanoutChannelEntry],
    states: &[AttentionStateTerm],
) -> Vec<AttentionMatrixInvariant> {
    use AttentionStateClass::*;
    use ControlledVocabulary::*;

    let mut out = Vec::new();

    // Every object points at a canonical object and a producer.
    out.push(invariant(
        "attention.canonical_object_identity",
        "Every attention object cites at least one canonical boundary schema and at least one \
         producing crate, so docs/help/support/activity/companion point at the same objects.",
        objects
            .iter()
            .all(|o| !o.canonical_schema_refs.is_empty() && !o.produced_by_refs.is_empty()),
    ));

    // Release-automation binding: every object maps to a proof packet. A claimed
    // attention object with no mapped proof row flips this false and fails
    // promotion.
    out.push(invariant(
        "attention.proof_packet_mapped",
        "Every attention object maps to a non-empty proof packet that keeps it current, so stable \
         promotion fails when a claimed attention surface lacks a mapped proof row.",
        objects.iter().all(|o| !o.proof_packet_ref.is_empty()),
    ));

    // No toast-only truth: any object that can show a durable-requiring state is a
    // durable authoritative record.
    out.push(invariant(
        "attention.no_toast_only_truth",
        "No long-running or reviewable work lives only in a toast: every object that can show a \
         running, queued, failed, completed, resolved, snoozed, or archived state carries a durable \
         authoritative record.",
        objects.iter().all(|o| {
            let needs_durable = o
                .applicable_states
                .iter()
                .any(|s| s.requires_durable_object());
            !needs_durable || o.carries_durable_record
        }),
    ));

    // Badges derive from deduped durable items.
    out.push(invariant(
        "attention.badges_from_deduped_durable",
        "The badge aggregate binds the dedupe vocabulary, is a durable record, and reopens the \
         underlying activity row, so a badge count derives from deduped durable items rather than \
         raw event spam.",
        objects
            .iter()
            .find(|o| o.object == AttentionObjectClass::BadgeAggregate)
            .is_some_and(|o| {
                o.binds(DedupeRule)
                    && o.carries_durable_record
                    && o.can_reopen(ReopenTargetClass::ActivityJobRow)
            }),
    ));

    // Fanout cannot bypass preview/approval.
    out.push(invariant(
        "attention.fanout_cannot_bypass_preview_approval",
        "No fanout channel — OS, dock, browser, mobile, or operator — may bypass the in-product \
         preview/approval flow.",
        channels.iter().all(|c| !c.can_bypass_preview_approval),
    ));

    // Fanout failure is visible truth, not silent best effort.
    out.push(invariant(
        "attention.fanout_no_silent_failure",
        "The fanout receipt can show both stale and undelivered states and binds the \
         fanout-delivery vocabulary, and any object that can show a delivery-gap state binds it, so \
         failed fanout is labeled rather than silently dropped.",
        objects
            .iter()
            .find(|o| o.object == AttentionObjectClass::FanoutReceipt)
            .is_some_and(|o| {
                o.can_show(FanoutStale) && o.can_show(FanoutUndelivered) && o.binds(FanoutDelivery)
            })
            && objects.iter().all(|o| {
                let shows_gap = o.applicable_states.iter().any(|s| s.is_delivery_gap());
                !shows_gap || o.binds(FanoutDelivery)
            }),
    ));

    // Suppression / quiet-hours stay separate from audit history.
    out.push(invariant(
        "attention.suppression_separate_from_history",
        "Every object that can show a suppressed state binds the suppression vocabulary, every \
         object that can show a quiet-hours-deferred state binds the quiet-hours vocabulary, and \
         either case keeps that state separate from audit history.",
        objects.iter().all(|o| {
            let shows_suppressed = o.can_show(Suppressed);
            let shows_quiet_hours = o.can_show(QuietHoursDeferred);
            if !shows_suppressed && !shows_quiet_hours {
                return true;
            }
            (!shows_suppressed || o.binds(Suppression))
                && (!shows_quiet_hours || o.binds(QuietHours))
                && o.retention_rule.separate_from_audit_history
        }),
    ));

    // Every surface reopens the authoritative object.
    out.push(invariant(
        "attention.reopen_authoritative",
        "Every attention object binds the reopen-routing vocabulary and names at least one \
         authoritative target it can reopen, so a surface reopens the object rather than reissuing a \
         blind side effect.",
        objects
            .iter()
            .all(|o| o.binds(ReopenRouting) && !o.reopen_targets.is_empty()),
    ));

    // Privacy-aware routing: every object binds the privacy-class vocabulary.
    out.push(invariant(
        "attention.privacy_class_governed",
        "Every attention object binds the privacy-class vocabulary, so what is shown, mirrored, or \
         exported is privacy-aware on every surface.",
        objects.iter().all(|o| o.binds(PrivacyClass)),
    ));

    // The envelope is fully typed for routing.
    out.push(invariant(
        "attention.envelope_routed_and_typed",
        "The notification envelope binds severity, scope, privacy class, and dedupe rule, so \
         attention is routed and typed rather than emitted as an ad hoc toast.",
        objects
            .iter()
            .find(|o| o.object == AttentionObjectClass::NotificationEnvelope)
            .is_some_and(|o| {
                o.binds(Severity) && o.binds(Scope) && o.binds(PrivacyClass) && o.binds(DedupeRule)
            }),
    ));

    // Every named controlled vocabulary is actually bound by some object.
    out.push(invariant(
        "attention.controlled_vocabulary_complete",
        "Each of the eight named controlled vocabularies — severity, scope, privacy class, dedupe \
         rule, suppression, quiet-hours, fanout delivery, and reopen routing — is bound by at least \
         one object.",
        ControlledVocabulary::ALL
            .iter()
            .all(|v| objects.iter().any(|o| o.binds(*v))),
    ));

    // Stable ids and tokens defined once and unique.
    out.push(invariant(
        "attention.stable_ids_unique",
        "Object ids, channel ids, and state tokens are each defined once and unique, so consumers \
         can resolve an object, channel, or state by a stable token.",
        all_unique(objects.iter().map(|o| o.object_id.as_str()))
            && all_unique(channels.iter().map(|c| c.channel_id.as_str()))
            && all_unique(states.iter().map(|t| t.token.as_str())),
    ));

    // Every fanout channel is covered.
    out.push(invariant(
        "attention.all_channels_covered",
        "The matrix covers the in-app activity center, OS notification, dock/taskbar badge, browser \
         and mobile companions, and operator dashboard fanout channels.",
        FanoutChannelClass::ALL
            .iter()
            .all(|class| channels.iter().any(|c| c.channel == *class)),
    ));

    // Every object family is present.
    out.push(invariant(
        "attention.all_objects_present",
        "Every governed attention object family in the matrix is present exactly once.",
        AttentionObjectClass::ALL
            .iter()
            .all(|class| objects.iter().filter(|o| o.object == *class).count() == 1),
    ));

    // Typed, never toast-only / prose-only.
    out.push(invariant(
        "attention.typed_not_toast_only",
        "Every object is typed and locally inspectable: it carries state terms, required fields, \
         and schema refs and is never reduced to a toast-only or prose-only view.",
        objects.iter().all(|o| {
            o.typed_not_toast_only
                && o.locally_inspectable
                && !o.applicable_states.is_empty()
                && !o.required_fields.is_empty()
                && !o.canonical_schema_refs.is_empty()
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the matrix as human-readable lines for CLI/headless and support.
pub fn attention_routing_lines(matrix: &AttentionRoutingMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Attention-routing matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(matrix.summary.clone());
    lines.push(format!(
        "Objects: {}  Channels: {}  States: {}  Invariants: {}",
        matrix.objects.len(),
        matrix.channels.len(),
        matrix.state_vocabulary.len(),
        matrix.invariants.len(),
    ));

    lines.push("Objects:".to_owned());
    for o in &matrix.objects {
        let states: Vec<&str> = o.applicable_states.iter().map(|st| st.as_str()).collect();
        let vocab: Vec<&str> = o
            .controlled_vocabularies
            .iter()
            .map(|v| v.as_str())
            .collect();
        let reopen: Vec<&str> = o.reopen_targets.iter().map(|t| t.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] privacy={} scope={} durable={}",
            o.object.as_str(),
            o.object_id,
            o.default_privacy.as_str(),
            o.scope_default.as_str(),
            o.carries_durable_record,
        ));
        lines.push(format!("      {}", o.summary));
        lines.push(format!("      states: {}", states.join(", ")));
        lines.push(format!("      vocabularies: {}", vocab.join(", ")));
        lines.push(format!(
            "      schemas: {}",
            o.canonical_schema_refs.join(", ")
        ));
        lines.push(format!("      proof: {}", o.proof_packet_ref));
        lines.push(format!("      reopen: {}", reopen.join(", ")));
    }

    lines.push("Channels:".to_owned());
    for c in &matrix.channels {
        lines.push(format!(
            "  - {} [{}] posture={:?} privacy_ceiling={} bypass_preview_approval={}",
            c.channel.as_str(),
            c.channel_id,
            c.delivery_posture,
            c.privacy_ceiling.as_str(),
            c.can_bypass_preview_approval,
        ));
        lines.push(format!("      {}", c.summary));
    }

    lines.push("Invariants:".to_owned());
    for i in &matrix.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
