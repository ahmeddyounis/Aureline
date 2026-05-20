//! Freshness, evidence, and queue-order truth model for operator dashboards
//! and queues.
//!
//! ## Why one model, not four per-surface status widgets
//!
//! The service-health dashboard, the review inbox, the incident queue, the
//! support queue, and the admin queue all answer the same operator question
//! when something looks calm: *"is this row actually current, why is it
//! sorted where it is, what is hidden from me by scope, and where is the
//! evidence behind the displayed state?"*. When each surface invents its own
//! status chip and its own sort copy, two failures follow:
//!
//! - **Stale green.** A row keeps painting a confident "all clear" headline
//!   long after the probe that justified it expired, or after the
//!   last-successful evidence aged out. The operator trusts a green that no
//!   longer has support behind it.
//! - **Unexplained order and silent narrowing.** A queue sorts rows with no
//!   stated reason and quietly drops rows that fall outside the active scope,
//!   so the operator cannot tell whether the list is empty because nothing is
//!   wrong or because everything that is wrong is hidden.
//!
//! This module mints two governed records that every dashboard and queue
//! surface reads verbatim:
//!
//! - [`DashboardFreshnessCard`] — one per row/card on any surface. It carries
//!   the declared headline state, the freshness of the data behind it, the
//!   age of the last-successful evidence, the canonical durable object the
//!   "open details / inspect evidence" affordance routes to, and a derived
//!   *effective* state that **cannot stay green** once freshness or evidence
//!   has expired.
//! - [`QueueOrderReason`] — one per queue surface. It explains why each
//!   visible row is sorted where it is and how many rows are hidden by which
//!   narrowing reason, each with a canonical "reveal" route.
//!
//! The two records compose into a [`DashboardTruthView`] — the single record
//! the shell, CLI/headless inspect, diagnostics, and support exports all read
//! so they agree on freshness, order reason, and hidden-scope state for the
//! same object.
//!
//! ## The no-silent-green invariant
//!
//! Acceptance criterion: *no claimed beta dashboard remains green when
//! freshness or evidence has expired; the row visibly downgrades and
//! preserves an inspectable path.* The model enforces this structurally: a
//! card's [`EffectiveStateClass`] is [`EffectiveStateClass::Clear`] **only**
//! when its declared state is [`DisplayedStateClass::Clear`], its
//! [`FreshnessClass`] is [`FreshnessClass::Fresh`], and its
//! [`EvidenceAgeClass`] is current ([`EvidenceAgeClass::Fresh`] or
//! [`EvidenceAgeClass::Recent`]). Any other combination on a would-be-green
//! row downgrades it to [`EffectiveStateClass::Unconfirmed`], lights the
//! honesty marker, and records the precise [`DowngradeReasonClass`] set so the
//! operator can read *why* the green claim was withdrawn — never silently.
//!
//! ## What never crosses this boundary
//!
//! Raw endpoint URLs, hostnames, credentials, raw payloads, raw stack frames,
//! raw operator identity strings, and absolute paths never appear on these
//! records. Surfaces carry opaque object refs (`aureline://<class>/<id>`),
//! stable tokens, and short reviewable sentences only.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized dashboard-truth views.
pub const DASHBOARD_TRUTH_VIEW_RECORD_KIND: &str = "dashboard_truth_view_record";

/// Schema version for the [`DashboardTruthView`] payload shape.
pub const DASHBOARD_TRUTH_VIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for an individual freshness card.
pub const DASHBOARD_FRESHNESS_CARD_RECORD_KIND: &str = "dashboard_freshness_card_record";

/// Schema version for an individual freshness card payload.
pub const DASHBOARD_FRESHNESS_CARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a queue-order-reason record.
pub const QUEUE_ORDER_REASON_RECORD_KIND: &str = "queue_order_reason_record";

/// Schema version for a queue-order-reason payload.
pub const QUEUE_ORDER_REASON_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every dashboard-truth surface.
pub const DASHBOARD_TRUTH_NOTICE: &str =
    "Dashboard & queue truth: every row carries a freshness class, the age of the last-successful \
     evidence, and the canonical object its open-details path routes to; a row may render its \
     all-clear headline only while it is fresh and evidence-current — otherwise it downgrades and \
     names why. Queue rows carry an order reason and the hidden-by-scope counts. Shell, CLI / \
     headless inspect, diagnostics, and support exports read this record verbatim — surface-local \
     status copy and silent green are not admitted.";

/// Upper bound on a reviewable explanation sentence.
const MAX_EXPLANATION_CHARS: usize = 240;
/// Upper bound on a short row title.
const MAX_TITLE_CHARS: usize = 120;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Canonical durable-object URI scheme. Every "open details", "inspect
/// evidence", "resolve follow-up", and "reveal hidden" affordance must route
/// to one of these, never to a generic landing page.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one of these is rejected so the
/// chrome cannot wire an "open details" button to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
];

// ---------------------------------------------------------------------------
// Surface vocabulary
// ---------------------------------------------------------------------------

/// The dashboard or queue surface a record belongs to. Closed set; surfaces
/// MUST NOT invent families outside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardSurfaceClass {
    /// Service-health dashboard (one card per service family).
    ServiceHealth,
    /// Review inbox (one row per change-review awaiting attention).
    ReviewInbox,
    /// Incident queue (one row per incident/outage record).
    IncidentQueue,
    /// Support queue (one row per support case / export request).
    SupportQueue,
    /// Admin queue (one row per governance / admin follow-up).
    AdminQueue,
}

impl DashboardSurfaceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceHealth => "service_health",
            Self::ReviewInbox => "review_inbox",
            Self::IncidentQueue => "incident_queue",
            Self::SupportQueue => "support_queue",
            Self::AdminQueue => "admin_queue",
        }
    }

    /// Human-readable label, quoted verbatim across surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ServiceHealth => "Service health",
            Self::ReviewInbox => "Review inbox",
            Self::IncidentQueue => "Incident queue",
            Self::SupportQueue => "Support queue",
            Self::AdminQueue => "Admin queue",
        }
    }

    /// True when this surface is a queue (carries order + narrowing truth).
    /// The service-health dashboard is the only non-queue surface.
    pub const fn is_queue(self) -> bool {
        !matches!(self, Self::ServiceHealth)
    }
}

// ---------------------------------------------------------------------------
// Displayed-state vocabulary
// ---------------------------------------------------------------------------

/// The declared headline state of a row as reported by the source. This is the
/// "green / amber / red" the surface *would* show before the freshness and
/// evidence rules are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayedStateClass {
    /// Green: nothing needs the operator's attention (ready / resolved /
    /// up-to-date / clear).
    Clear,
    /// Amber: needs attention but is not blocking (degraded / waiting / due).
    Attention,
    /// Red: blocked / failing / overdue.
    Blocked,
}

impl DisplayedStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Attention => "attention",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Attention => "Needs attention",
            Self::Blocked => "Blocked",
        }
    }

    /// Severity for sorting (higher = more severe). `clear` is 0.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Clear => 0,
            Self::Attention => 1,
            Self::Blocked => 2,
        }
    }
}

/// The honest, derived headline state after applying the freshness and
/// evidence rules. This is what the chrome MUST render — never the raw
/// [`DisplayedStateClass`] alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveStateClass {
    /// Confirmed green. Only reachable when the declared state is `clear`,
    /// freshness is `fresh`, and evidence is current.
    Clear,
    /// The declared state was `clear`, but freshness or evidence cannot
    /// support a green claim, so it is withdrawn. The card names why in its
    /// downgrade reasons.
    Unconfirmed,
    /// Declared `attention` carried through (the freshness chip is still
    /// shown alongside).
    Attention,
    /// Declared `blocked` carried through.
    Blocked,
}

impl EffectiveStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Unconfirmed => "unconfirmed",
            Self::Attention => "attention",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Unconfirmed => "Unconfirmed — evidence not current",
            Self::Attention => "Needs attention",
            Self::Blocked => "Blocked",
        }
    }

    /// Severity for sorting (higher = more severe). `clear` is 0;
    /// `unconfirmed` sorts above `attention` because an unverifiable green is
    /// a stronger "look at me" signal than a known amber.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Clear => 0,
            Self::Attention => 1,
            Self::Unconfirmed => 2,
            Self::Blocked => 3,
        }
    }

    /// True when the chrome MUST light a yellow honesty chip on this row.
    pub const fn is_honest_warning(self) -> bool {
        !matches!(self, Self::Clear)
    }
}

// ---------------------------------------------------------------------------
// Freshness vocabulary
// ---------------------------------------------------------------------------

/// How current the data behind a row is, as declared by its source. Ordered
/// by severity so the worst freshness across a view rolls up deterministically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Probed within the review window; current.
    Fresh,
    /// Serving cached data because no fresh probe is in hand; the data is
    /// usable but not freshly confirmed.
    Cached,
    /// The data is past its review window; it has aged out.
    Stale,
    /// Only part of the underlying collection could be read; the row reflects
    /// an incomplete picture.
    Partial,
    /// Policy / governance scope prevents reading the current state.
    PolicyBlocked,
    /// The source is unreachable; no current state is available at all.
    Unavailable,
}

impl FreshnessClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
            Self::Partial => "Partial",
            Self::PolicyBlocked => "Policy blocked",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Severity for the freshness rollup (higher = worse).
    pub const fn severity(self) -> u8 {
        match self {
            Self::Fresh => 0,
            Self::Cached => 1,
            Self::Stale => 2,
            Self::Partial => 3,
            Self::PolicyBlocked => 4,
            Self::Unavailable => 5,
        }
    }

    /// True when the freshness is anything other than `fresh` — i.e. it
    /// represents a downgrade the chrome must surface.
    pub const fn is_downgrade(self) -> bool {
        !matches!(self, Self::Fresh)
    }
}

/// Age bucket for the last-successful evidence behind a row. Derived from the
/// evidence timestamp relative to a caller-supplied `as_of`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAgeClass {
    /// Evidence captured within the last 5 minutes.
    Fresh,
    /// Within the last hour.
    Recent,
    /// Within the last 24 hours.
    Stale,
    /// More than 24 hours ago.
    VeryStale,
    /// No successful evidence has ever been captured for this row (or the
    /// timestamp was unparseable). A row in this state can never be `clear`.
    Never,
}

impl EvidenceAgeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::VeryStale => "very_stale",
            Self::Never => "never",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Just now",
            Self::Recent => "Within the hour",
            Self::Stale => "Hours ago",
            Self::VeryStale => "More than a day ago",
            Self::Never => "No evidence yet",
        }
    }

    /// True when the evidence is current enough to back a green claim.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Fresh | Self::Recent)
    }

    /// True when the chrome should light a freshness warning even if the
    /// declared freshness is `fresh`.
    pub const fn is_warning(self) -> bool {
        matches!(self, Self::Stale | Self::VeryStale | Self::Never)
    }
}

/// The canonical durable-object class an evidence ref routes to. Closed set so
/// the chrome can label the "open details" affordance without guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKindClass {
    /// A service-health card / contract-state record.
    ServiceHealthCard,
    /// A change-review object.
    ChangeReview,
    /// An incident / outage record.
    IncidentRecord,
    /// A support case / export request.
    SupportCase,
    /// A governance audit entry.
    AuditEntry,
    /// A runbook / response packet.
    RunbookPacket,
    /// A policy decision record.
    PolicyDecision,
}

impl EvidenceKindClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceHealthCard => "service_health_card",
            Self::ChangeReview => "change_review",
            Self::IncidentRecord => "incident_record",
            Self::SupportCase => "support_case",
            Self::AuditEntry => "audit_entry",
            Self::RunbookPacket => "runbook_packet",
            Self::PolicyDecision => "policy_decision",
        }
    }

    /// Human-readable label for the open-details affordance.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ServiceHealthCard => "Service-health card",
            Self::ChangeReview => "Change review",
            Self::IncidentRecord => "Incident record",
            Self::SupportCase => "Support case",
            Self::AuditEntry => "Audit entry",
            Self::RunbookPacket => "Runbook packet",
            Self::PolicyDecision => "Policy decision",
        }
    }
}

/// Why a would-be-green row was downgraded. Multiple reasons can apply at once
/// (e.g. cached fallback whose evidence also aged out).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReasonClass {
    /// The data is being served from cache without a fresh probe.
    CachedFallback,
    /// Freshness expired — the data is past its review window.
    FreshnessExpired,
    /// The last-successful evidence aged out relative to `as_of`.
    EvidenceAgedOut,
    /// Only part of the underlying collection could be read.
    SourcePartial,
    /// Policy / governance scope blocked the read.
    PolicyBlocked,
    /// The source is offline / unreachable.
    SourceOffline,
}

impl DowngradeReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CachedFallback => "cached_fallback",
            Self::FreshnessExpired => "freshness_expired",
            Self::EvidenceAgedOut => "evidence_aged_out",
            Self::SourcePartial => "source_partial",
            Self::PolicyBlocked => "policy_blocked",
            Self::SourceOffline => "source_offline",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CachedFallback => "Serving cached data",
            Self::FreshnessExpired => "Freshness expired",
            Self::EvidenceAgedOut => "Evidence aged out",
            Self::SourcePartial => "Partial data",
            Self::PolicyBlocked => "Policy blocked",
            Self::SourceOffline => "Source offline",
        }
    }
}

// ---------------------------------------------------------------------------
// Queue order + narrowing vocabulary
// ---------------------------------------------------------------------------

/// Why a queue row is sorted where it is. Closed set so the "why here?"
/// affordance reads a stable token instead of inventing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderReasonClass {
    /// Sorted by severity, most severe first.
    SeverityDescending,
    /// Sorted by SLA / response deadline, soonest first.
    SlaDeadline,
    /// Oldest unresolved item first.
    OldestUnresolvedFirst,
    /// Most recently updated first.
    RecentlyUpdated,
    /// Pulled to the top because it is assigned to the current operator.
    AssignedToYou,
    /// Pulled up because it blocks other work.
    BlockingDependency,
    /// Manually pinned to a fixed position.
    ManualPin,
    /// Default recency ordering (no stronger reason applied).
    DefaultRecency,
}

impl OrderReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeverityDescending => "severity_descending",
            Self::SlaDeadline => "sla_deadline",
            Self::OldestUnresolvedFirst => "oldest_unresolved_first",
            Self::RecentlyUpdated => "recently_updated",
            Self::AssignedToYou => "assigned_to_you",
            Self::BlockingDependency => "blocking_dependency",
            Self::ManualPin => "manual_pin",
            Self::DefaultRecency => "default_recency",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SeverityDescending => "Most severe first",
            Self::SlaDeadline => "Deadline soonest first",
            Self::OldestUnresolvedFirst => "Oldest unresolved first",
            Self::RecentlyUpdated => "Recently updated",
            Self::AssignedToYou => "Assigned to you",
            Self::BlockingDependency => "Blocking other work",
            Self::ManualPin => "Pinned",
            Self::DefaultRecency => "Default order",
        }
    }
}

/// Why rows are hidden from the current queue view. Closed set so the
/// hidden-scope counter reads a stable token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReasonClass {
    /// Hidden by an active scope / workspace filter.
    ScopeFilter,
    /// Hidden because policy scope prevents the operator from seeing them.
    PolicyScope,
    /// Hidden by an assignee filter (e.g. "mine only").
    AssigneeFilter,
    /// Hidden because they are resolved and the view hides resolved items.
    ResolvedHidden,
    /// Hidden because they are archived.
    ArchivedHidden,
    /// Hidden by a severity floor.
    SeverityFilter,
    /// Not shown because the list could only be partially loaded while
    /// offline; the remainder is unknown, not absent.
    OfflinePartialList,
}

impl NarrowingReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeFilter => "scope_filter",
            Self::PolicyScope => "policy_scope",
            Self::AssigneeFilter => "assignee_filter",
            Self::ResolvedHidden => "resolved_hidden",
            Self::ArchivedHidden => "archived_hidden",
            Self::SeverityFilter => "severity_filter",
            Self::OfflinePartialList => "offline_partial_list",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ScopeFilter => "Hidden by scope filter",
            Self::PolicyScope => "Hidden by policy scope",
            Self::AssigneeFilter => "Hidden by assignee filter",
            Self::ResolvedHidden => "Resolved items hidden",
            Self::ArchivedHidden => "Archived items hidden",
            Self::SeverityFilter => "Below severity filter",
            Self::OfflinePartialList => "List incomplete (offline)",
        }
    }

    /// True when the hidden rows are unknown rather than deliberately
    /// filtered — the operator may be missing items they would want to see.
    pub const fn is_incomplete_knowledge(self) -> bool {
        matches!(self, Self::PolicyScope | Self::OfflinePartialList)
    }
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

/// Input describing a single dashboard/queue row before normalization. The
/// runtime sources (service-health probes, review index, incident store,
/// support store, admin store) mint these and the model derives the honest
/// card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessCardInput {
    /// Stable card id. Used for object identity across surfaces.
    pub card_id: String,
    /// Short row title (<= 120 chars). No raw paths or identity strings.
    pub title: String,
    /// Declared headline state from the source.
    pub displayed_state: DisplayedStateClass,
    /// Freshness of the data behind the row, as declared by the source.
    pub freshness: FreshnessClass,
    /// ISO-8601 UTC timestamp of the last-successful evidence, or `None` when
    /// none has been captured.
    #[serde(default)]
    pub last_successful_evidence_at: Option<String>,
    /// Kind of canonical durable object the evidence ref points at.
    pub evidence_kind: EvidenceKindClass,
    /// Canonical durable object ref (`aureline://<class>/<id>`) the
    /// open-details / inspect-evidence affordance routes to.
    pub evidence_ref: String,
    /// Short reviewable sentence (<= 240 chars) explaining the state.
    pub state_explanation: String,
}

/// Input describing one visible queue row's ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueRowInput {
    /// Row id; must match a [`FreshnessCardInput::card_id`] on the same view.
    pub row_id: String,
    /// Why the row sits where it does.
    pub order_reason: OrderReasonClass,
    /// Short reviewable sentence (<= 240 chars) for the "why here?" tooltip.
    pub order_explanation: String,
    /// Canonical durable object ref the "open details / resolve follow-up"
    /// affordance routes to.
    pub open_details_ref: String,
}

/// Input describing one hidden-scope bucket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenScopeInput {
    /// Why these rows are hidden.
    pub narrowing_reason: NarrowingReasonClass,
    /// How many rows are hidden for this reason. Must be > 0.
    pub hidden_count: u32,
    /// Short reviewable sentence (<= 240 chars) for the hidden-scope counter.
    pub narrowing_explanation: String,
    /// Canonical durable object ref the "reveal" affordance routes to (a
    /// scoped query/object, never a generic landing page).
    pub reveal_ref: String,
}

/// Input describing a queue surface's ordering and narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueOrderInput {
    /// Stable queue id.
    pub queue_id: String,
    /// Visible rows, in render order. The first entry is rank 1.
    pub rows: Vec<QueueRowInput>,
    /// Hidden-scope buckets.
    #[serde(default)]
    pub hidden_scope: Vec<HiddenScopeInput>,
}

// ---------------------------------------------------------------------------
// Output records
// ---------------------------------------------------------------------------

/// One normalized dashboard/queue row. Carries both the declared and the
/// derived honest state, plus the evidence route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardFreshnessCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub surface: DashboardSurfaceClass,
    pub surface_token: String,
    pub surface_label: String,
    pub title: String,
    pub displayed_state: DisplayedStateClass,
    pub displayed_state_token: String,
    pub displayed_state_label: String,
    pub effective_state: EffectiveStateClass,
    pub effective_state_token: String,
    pub effective_state_label: String,
    pub freshness: FreshnessClass,
    pub freshness_token: String,
    pub freshness_label: String,
    pub last_successful_evidence_at: Option<String>,
    pub evidence_age: EvidenceAgeClass,
    pub evidence_age_token: String,
    pub evidence_age_label: String,
    pub evidence_kind: EvidenceKindClass,
    pub evidence_kind_token: String,
    pub evidence_kind_label: String,
    pub evidence_ref: String,
    pub downgrade_reasons: Vec<DowngradeReasonClass>,
    pub downgrade_reason_tokens: Vec<String>,
    pub state_explanation: String,
    /// True when the declared state was `clear` but the effective state was
    /// withdrawn to `unconfirmed` — the no-silent-green downgrade.
    pub green_downgraded: bool,
    /// True when the chrome must light a yellow honesty chip on this row.
    pub honesty_marker_present: bool,
}

/// One visible queue row's normalized ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueRowOrder {
    pub row_id: String,
    pub order_rank: u32,
    pub order_reason: OrderReasonClass,
    pub order_reason_token: String,
    pub order_reason_label: String,
    pub order_explanation: String,
    pub open_details_ref: String,
}

/// One hidden-scope counter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenScopeCounter {
    pub narrowing_reason: NarrowingReasonClass,
    pub narrowing_reason_token: String,
    pub narrowing_reason_label: String,
    pub hidden_count: u32,
    pub narrowing_explanation: String,
    pub reveal_ref: String,
    /// True when the hidden rows are unknown rather than deliberately filtered.
    pub incomplete_knowledge: bool,
}

/// Queue-order-reason record: why visible rows are sorted as they are and what
/// is hidden by scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueOrderReason {
    pub record_kind: String,
    pub schema_version: u32,
    pub queue_id: String,
    pub surface: DashboardSurfaceClass,
    pub surface_token: String,
    pub surface_label: String,
    pub visible_row_count: u32,
    pub hidden_total: u32,
    pub total_in_scope_count: u32,
    pub rows: Vec<QueueRowOrder>,
    pub order_reasons_present: Vec<String>,
    pub hidden_scope: Vec<HiddenScopeCounter>,
    /// True when at least one row is hidden — the operator must know the list
    /// is narrowed.
    pub narrowing_present: bool,
    /// True when any hidden bucket represents unknown rows (policy scope or an
    /// incomplete offline list) rather than a deliberate filter.
    pub incomplete_knowledge_present: bool,
}

/// Summary counters across the cards on a view.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DashboardTruthSummary {
    pub total_card_count: u32,
    pub clear_card_count: u32,
    pub unconfirmed_card_count: u32,
    pub attention_card_count: u32,
    pub blocked_card_count: u32,
    pub green_downgrade_count: u32,
    pub stale_evidence_count: u32,
    pub freshness_downgrade_count: u32,
}

/// The single record every dashboard/queue surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardTruthView {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub view_id: String,
    pub surface: DashboardSurfaceClass,
    pub surface_token: String,
    pub surface_label: String,
    /// `as_of` instant used to compute every card's evidence age.
    pub as_of: String,
    pub cards: Vec<DashboardFreshnessCard>,
    /// Present for queue surfaces; `None` for the service-health dashboard.
    pub queue_order: Option<QueueOrderReason>,
    pub summary: DashboardTruthSummary,
    pub overall_effective_state: EffectiveStateClass,
    pub overall_effective_state_token: String,
    pub overall_effective_state_label: String,
    pub overall_freshness: FreshnessClass,
    pub overall_freshness_token: String,
    pub overall_freshness_label: String,
    pub honesty_marker_present: bool,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Error raised when a view input fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewBuildError {
    AsOfEmpty,
    EmptyViewId,
    EmptyCardId,
    DuplicateCardId(String),
    EmptyTitle(String),
    TitleTooLong(String),
    EmptyExplanation(String),
    ExplanationTooLong(String),
    EvidenceRefNotCanonical { card_id: String, reference: String },
    QueueExpected,
    QueueUnexpected,
    EmptyQueueId,
    QueueRowUnknownCard(String),
    DuplicateQueueRow(String),
    QueueRowMissing(String),
    EmptyOrderExplanation(String),
    OrderExplanationTooLong(String),
    OpenDetailsRefNotCanonical { row_id: String, reference: String },
    HiddenCountZero(String),
    EmptyNarrowingExplanation(String),
    NarrowingExplanationTooLong(String),
    RevealRefNotCanonical { reason: String, reference: String },
    DuplicateNarrowingReason(String),
}

impl std::fmt::Display for ViewBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsOfEmpty => write!(f, "as_of must not be empty"),
            Self::EmptyViewId => write!(f, "view_id must not be empty"),
            Self::EmptyCardId => write!(f, "card_id must not be empty"),
            Self::DuplicateCardId(id) => write!(f, "duplicate card_id: {id}"),
            Self::EmptyTitle(id) => write!(f, "title must not be empty for card {id}"),
            Self::TitleTooLong(id) => {
                write!(f, "title for card {id} exceeds {MAX_TITLE_CHARS} chars")
            }
            Self::EmptyExplanation(id) => {
                write!(f, "state_explanation must not be empty for card {id}")
            }
            Self::ExplanationTooLong(id) => write!(
                f,
                "state_explanation for card {id} exceeds {MAX_EXPLANATION_CHARS} chars",
            ),
            Self::EvidenceRefNotCanonical { card_id, reference } => write!(
                f,
                "evidence_ref {reference:?} for card {card_id} is not a canonical durable object \
                 ref (expected {CANONICAL_OBJECT_SCHEME}<class>/<id>, not a generic landing page)",
            ),
            Self::QueueExpected => {
                write!(f, "queue surface requires queue_order to be present")
            }
            Self::QueueUnexpected => write!(
                f,
                "non-queue surface (service_health) must not carry queue_order",
            ),
            Self::EmptyQueueId => write!(f, "queue_id must not be empty"),
            Self::QueueRowUnknownCard(id) => {
                write!(f, "queue row {id} has no matching card on the view")
            }
            Self::DuplicateQueueRow(id) => write!(f, "duplicate queue row id: {id}"),
            Self::QueueRowMissing(id) => {
                write!(f, "card {id} has no queue-order row on a queue surface")
            }
            Self::EmptyOrderExplanation(id) => {
                write!(f, "order_explanation must not be empty for row {id}")
            }
            Self::OrderExplanationTooLong(id) => write!(
                f,
                "order_explanation for row {id} exceeds {MAX_EXPLANATION_CHARS} chars",
            ),
            Self::OpenDetailsRefNotCanonical { row_id, reference } => write!(
                f,
                "open_details_ref {reference:?} for row {row_id} is not a canonical durable object \
                 ref",
            ),
            Self::HiddenCountZero(reason) => {
                write!(f, "hidden_count for narrowing reason {reason} must be > 0")
            }
            Self::EmptyNarrowingExplanation(reason) => write!(
                f,
                "narrowing_explanation must not be empty for narrowing reason {reason}",
            ),
            Self::NarrowingExplanationTooLong(reason) => write!(
                f,
                "narrowing_explanation for {reason} exceeds {MAX_EXPLANATION_CHARS} chars",
            ),
            Self::RevealRefNotCanonical { reason, reference } => write!(
                f,
                "reveal_ref {reference:?} for narrowing reason {reason} is not a canonical durable \
                 object ref",
            ),
            Self::DuplicateNarrowingReason(reason) => {
                write!(f, "duplicate narrowing reason: {reason}")
            }
        }
    }
}

impl std::error::Error for ViewBuildError {}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

impl DashboardTruthView {
    /// Build a dashboard-truth view from row inputs and optional queue
    /// ordering.
    ///
    /// `as_of` is the chrome's "now" used to compute evidence-age buckets.
    /// Queue surfaces MUST pass a [`QueueOrderInput`] with one row per card;
    /// the service-health dashboard MUST pass `None`.
    pub fn build(
        view_id: impl Into<String>,
        surface: DashboardSurfaceClass,
        as_of: impl Into<String>,
        cards: Vec<FreshnessCardInput>,
        queue: Option<QueueOrderInput>,
    ) -> Result<Self, ViewBuildError> {
        let view_id = view_id.into();
        let as_of = as_of.into();
        if view_id.trim().is_empty() {
            return Err(ViewBuildError::EmptyViewId);
        }
        if as_of.trim().is_empty() {
            return Err(ViewBuildError::AsOfEmpty);
        }

        validate_card_inputs(&cards)?;

        // Queue presence must match the surface class.
        match (surface.is_queue(), queue.is_some()) {
            (true, false) => return Err(ViewBuildError::QueueExpected),
            (false, true) => return Err(ViewBuildError::QueueUnexpected),
            _ => {}
        }

        let mut projected: Vec<DashboardFreshnessCard> = cards
            .iter()
            .map(|input| project_card(surface, input, &as_of))
            .collect();
        projected.sort_by(card_sort_key);

        let queue_order = match queue {
            Some(input) => Some(build_queue_order(surface, &input, &projected)?),
            None => None,
        };

        let summary = compute_summary(&projected);
        let overall_effective_state = rollup_effective_state(&projected);
        let overall_freshness = rollup_freshness(&projected);
        let honesty_marker_present = projected.iter().any(|c| c.honesty_marker_present)
            || queue_order.as_ref().is_some_and(|q| q.narrowing_present);

        Ok(Self {
            record_kind: DASHBOARD_TRUTH_VIEW_RECORD_KIND.to_owned(),
            schema_version: DASHBOARD_TRUTH_VIEW_SCHEMA_VERSION,
            notice: DASHBOARD_TRUTH_NOTICE.to_owned(),
            view_id,
            surface,
            surface_token: surface.as_str().to_owned(),
            surface_label: surface.label().to_owned(),
            as_of,
            cards: projected,
            queue_order,
            summary,
            overall_effective_state,
            overall_effective_state_token: overall_effective_state.as_str().to_owned(),
            overall_effective_state_label: overall_effective_state.label().to_owned(),
            overall_freshness,
            overall_freshness_token: overall_freshness.as_str().to_owned(),
            overall_freshness_label: overall_freshness.label().to_owned(),
            honesty_marker_present,
        })
    }

    /// Cards in the deterministic render order (worst effective state first).
    pub fn cards_for_render(&self) -> &[DashboardFreshnessCard] {
        &self.cards
    }

    /// Cards whose effective state is anything other than `clear`.
    pub fn impaired_cards(&self) -> Vec<&DashboardFreshnessCard> {
        self.cards
            .iter()
            .filter(|c| c.effective_state != EffectiveStateClass::Clear)
            .collect()
    }

    /// Cards that were withdrawn from a green claim (the stale-green guard).
    pub fn green_downgraded_cards(&self) -> Vec<&DashboardFreshnessCard> {
        self.cards.iter().filter(|c| c.green_downgraded).collect()
    }

    /// Deterministic plaintext block for support exports and reviewer previews.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Dashboard & queue truth\n");
        out.push_str(&format!("View: {}\n", self.view_id));
        out.push_str(&format!(
            "Surface: {} ({})\n",
            self.surface.label(),
            self.surface_token,
        ));
        out.push_str(&format!("As of: {}\n", self.as_of));
        out.push_str(&format!(
            "Overall: {} ({}) | Freshness: {} ({})\n",
            self.overall_effective_state.label(),
            self.overall_effective_state_token,
            self.overall_freshness.label(),
            self.overall_freshness_token,
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out.push_str(&format!(
            "Summary: total={}, clear={}, unconfirmed={}, attention={}, blocked={}, green_downgrades={}, stale_evidence={}, freshness_downgrades={}\n\n",
            self.summary.total_card_count,
            self.summary.clear_card_count,
            self.summary.unconfirmed_card_count,
            self.summary.attention_card_count,
            self.summary.blocked_card_count,
            self.summary.green_downgrade_count,
            self.summary.stale_evidence_count,
            self.summary.freshness_downgrade_count,
        ));

        for card in &self.cards {
            out.push_str(&format!(
                "- {} [{}] state={}/{} freshness={} evidence_age={}\n",
                card.card_id,
                if card.honesty_marker_present {
                    "warn"
                } else {
                    "ok"
                },
                card.displayed_state_token,
                card.effective_state_token,
                card.freshness_token,
                card.evidence_age_token,
            ));
            if !card.downgrade_reasons.is_empty() {
                out.push_str("    downgrade reasons: ");
                out.push_str(&card.downgrade_reason_tokens.join(", "));
                out.push('\n');
            }
            out.push_str(&format!("    explain: {}\n", card.state_explanation));
            out.push_str(&format!(
                "    evidence: {} -> {}\n",
                card.evidence_kind_token, card.evidence_ref,
            ));
        }

        if let Some(queue) = &self.queue_order {
            out.push('\n');
            out.push_str(&format!(
                "Queue order ({}): visible={}, hidden={}, in_scope={}\n",
                queue.queue_id,
                queue.visible_row_count,
                queue.hidden_total,
                queue.total_in_scope_count,
            ));
            for row in &queue.rows {
                out.push_str(&format!(
                    "  #{} {} order={} -> {}\n",
                    row.order_rank, row.row_id, row.order_reason_token, row.open_details_ref,
                ));
            }
            for hidden in &queue.hidden_scope {
                out.push_str(&format!(
                    "  hidden {} x{} ({}) -> {}\n",
                    hidden.narrowing_reason_token,
                    hidden.hidden_count,
                    if hidden.incomplete_knowledge {
                        "unknown"
                    } else {
                        "filtered"
                    },
                    hidden.reveal_ref,
                ));
            }
        }

        out
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn validate_card_inputs(cards: &[FreshnessCardInput]) -> Result<(), ViewBuildError> {
    let mut seen = std::collections::BTreeSet::new();
    for card in cards {
        if card.card_id.trim().is_empty() {
            return Err(ViewBuildError::EmptyCardId);
        }
        if !seen.insert(card.card_id.clone()) {
            return Err(ViewBuildError::DuplicateCardId(card.card_id.clone()));
        }

        let title = card.title.trim();
        if title.is_empty() {
            return Err(ViewBuildError::EmptyTitle(card.card_id.clone()));
        }
        if title.chars().count() > MAX_TITLE_CHARS {
            return Err(ViewBuildError::TitleTooLong(card.card_id.clone()));
        }

        let explanation = card.state_explanation.trim();
        if explanation.is_empty() {
            return Err(ViewBuildError::EmptyExplanation(card.card_id.clone()));
        }
        if explanation.chars().count() > MAX_EXPLANATION_CHARS {
            return Err(ViewBuildError::ExplanationTooLong(card.card_id.clone()));
        }

        if !is_canonical_object_ref(&card.evidence_ref) {
            return Err(ViewBuildError::EvidenceRefNotCanonical {
                card_id: card.card_id.clone(),
                reference: card.evidence_ref.clone(),
            });
        }
    }
    Ok(())
}

fn project_card(
    surface: DashboardSurfaceClass,
    input: &FreshnessCardInput,
    as_of: &str,
) -> DashboardFreshnessCard {
    let evidence_age = match &input.last_successful_evidence_at {
        Some(ts) => derive_age(ts, as_of),
        None => EvidenceAgeClass::Never,
    };

    let downgrade_reasons = downgrade_reasons(input.freshness, evidence_age);
    let effective_state = effective_state(input.displayed_state, input.freshness, evidence_age);
    let green_downgraded = input.displayed_state == DisplayedStateClass::Clear
        && effective_state == EffectiveStateClass::Unconfirmed;
    let honesty_marker_present = effective_state.is_honest_warning()
        || input.freshness.is_downgrade()
        || evidence_age.is_warning();

    let downgrade_reason_tokens = downgrade_reasons
        .iter()
        .map(|r| r.as_str().to_owned())
        .collect();

    DashboardFreshnessCard {
        record_kind: DASHBOARD_FRESHNESS_CARD_RECORD_KIND.to_owned(),
        schema_version: DASHBOARD_FRESHNESS_CARD_SCHEMA_VERSION,
        card_id: input.card_id.clone(),
        surface,
        surface_token: surface.as_str().to_owned(),
        surface_label: surface.label().to_owned(),
        title: input.title.trim().to_owned(),
        displayed_state: input.displayed_state,
        displayed_state_token: input.displayed_state.as_str().to_owned(),
        displayed_state_label: input.displayed_state.label().to_owned(),
        effective_state,
        effective_state_token: effective_state.as_str().to_owned(),
        effective_state_label: effective_state.label().to_owned(),
        freshness: input.freshness,
        freshness_token: input.freshness.as_str().to_owned(),
        freshness_label: input.freshness.label().to_owned(),
        last_successful_evidence_at: input.last_successful_evidence_at.clone(),
        evidence_age,
        evidence_age_token: evidence_age.as_str().to_owned(),
        evidence_age_label: evidence_age.label().to_owned(),
        evidence_kind: input.evidence_kind,
        evidence_kind_token: input.evidence_kind.as_str().to_owned(),
        evidence_kind_label: input.evidence_kind.label().to_owned(),
        evidence_ref: input.evidence_ref.trim().to_owned(),
        downgrade_reasons,
        downgrade_reason_tokens,
        state_explanation: input.state_explanation.trim().to_owned(),
        green_downgraded,
        honesty_marker_present,
    }
}

/// The no-silent-green rule. A row is `clear` only when it is declared clear,
/// fresh, and evidence-current. Any other combination on a clear row becomes
/// `unconfirmed`; attention/blocked carry through.
fn effective_state(
    displayed: DisplayedStateClass,
    freshness: FreshnessClass,
    evidence_age: EvidenceAgeClass,
) -> EffectiveStateClass {
    match displayed {
        DisplayedStateClass::Clear => {
            if freshness == FreshnessClass::Fresh && evidence_age.is_current() {
                EffectiveStateClass::Clear
            } else {
                EffectiveStateClass::Unconfirmed
            }
        }
        DisplayedStateClass::Attention => EffectiveStateClass::Attention,
        DisplayedStateClass::Blocked => EffectiveStateClass::Blocked,
    }
}

/// The set of reasons explaining a freshness/evidence downgrade. Stable,
/// sorted, de-duplicated.
fn downgrade_reasons(
    freshness: FreshnessClass,
    evidence_age: EvidenceAgeClass,
) -> Vec<DowngradeReasonClass> {
    let mut reasons = Vec::new();
    match freshness {
        FreshnessClass::Fresh => {}
        FreshnessClass::Cached => reasons.push(DowngradeReasonClass::CachedFallback),
        FreshnessClass::Stale => reasons.push(DowngradeReasonClass::FreshnessExpired),
        FreshnessClass::Partial => reasons.push(DowngradeReasonClass::SourcePartial),
        FreshnessClass::PolicyBlocked => reasons.push(DowngradeReasonClass::PolicyBlocked),
        FreshnessClass::Unavailable => reasons.push(DowngradeReasonClass::SourceOffline),
    }
    if evidence_age.is_warning() {
        reasons.push(DowngradeReasonClass::EvidenceAgedOut);
    }
    reasons.sort();
    reasons.dedup();
    reasons
}

fn card_sort_key(a: &DashboardFreshnessCard, b: &DashboardFreshnessCard) -> Ordering {
    b.effective_state
        .severity()
        .cmp(&a.effective_state.severity())
        .then_with(|| b.freshness.severity().cmp(&a.freshness.severity()))
        .then_with(|| a.card_id.cmp(&b.card_id))
}

fn compute_summary(cards: &[DashboardFreshnessCard]) -> DashboardTruthSummary {
    let mut summary = DashboardTruthSummary {
        total_card_count: cards.len() as u32,
        ..DashboardTruthSummary::default()
    };
    for card in cards {
        match card.effective_state {
            EffectiveStateClass::Clear => summary.clear_card_count += 1,
            EffectiveStateClass::Unconfirmed => summary.unconfirmed_card_count += 1,
            EffectiveStateClass::Attention => summary.attention_card_count += 1,
            EffectiveStateClass::Blocked => summary.blocked_card_count += 1,
        }
        if card.green_downgraded {
            summary.green_downgrade_count += 1;
        }
        if card.evidence_age.is_warning() {
            summary.stale_evidence_count += 1;
        }
        if card.freshness.is_downgrade() {
            summary.freshness_downgrade_count += 1;
        }
    }
    summary
}

fn rollup_effective_state(cards: &[DashboardFreshnessCard]) -> EffectiveStateClass {
    cards
        .iter()
        .map(|c| c.effective_state)
        .max_by_key(|s| s.severity())
        .unwrap_or(EffectiveStateClass::Clear)
}

fn rollup_freshness(cards: &[DashboardFreshnessCard]) -> FreshnessClass {
    cards
        .iter()
        .map(|c| c.freshness)
        .max_by_key(|f| f.severity())
        .unwrap_or(FreshnessClass::Fresh)
}

fn build_queue_order(
    surface: DashboardSurfaceClass,
    input: &QueueOrderInput,
    cards: &[DashboardFreshnessCard],
) -> Result<QueueOrderReason, ViewBuildError> {
    if input.queue_id.trim().is_empty() {
        return Err(ViewBuildError::EmptyQueueId);
    }

    let card_ids: std::collections::BTreeSet<&str> =
        cards.iter().map(|c| c.card_id.as_str()).collect();

    let mut seen_rows = std::collections::BTreeSet::new();
    let mut rows = Vec::with_capacity(input.rows.len());
    for (idx, row) in input.rows.iter().enumerate() {
        if !card_ids.contains(row.row_id.as_str()) {
            return Err(ViewBuildError::QueueRowUnknownCard(row.row_id.clone()));
        }
        if !seen_rows.insert(row.row_id.clone()) {
            return Err(ViewBuildError::DuplicateQueueRow(row.row_id.clone()));
        }
        let explanation = row.order_explanation.trim();
        if explanation.is_empty() {
            return Err(ViewBuildError::EmptyOrderExplanation(row.row_id.clone()));
        }
        if explanation.chars().count() > MAX_EXPLANATION_CHARS {
            return Err(ViewBuildError::OrderExplanationTooLong(row.row_id.clone()));
        }
        if !is_canonical_object_ref(&row.open_details_ref) {
            return Err(ViewBuildError::OpenDetailsRefNotCanonical {
                row_id: row.row_id.clone(),
                reference: row.open_details_ref.clone(),
            });
        }
        rows.push(QueueRowOrder {
            row_id: row.row_id.clone(),
            order_rank: (idx as u32) + 1,
            order_reason: row.order_reason,
            order_reason_token: row.order_reason.as_str().to_owned(),
            order_reason_label: row.order_reason.label().to_owned(),
            order_explanation: explanation.to_owned(),
            open_details_ref: row.open_details_ref.trim().to_owned(),
        });
    }

    // Every visible card on a queue surface must have a stated order row.
    for card in cards {
        if !seen_rows.contains(&card.card_id) {
            return Err(ViewBuildError::QueueRowMissing(card.card_id.clone()));
        }
    }

    let mut seen_reasons = std::collections::BTreeSet::new();
    let mut hidden_scope = Vec::with_capacity(input.hidden_scope.len());
    let mut hidden_total: u32 = 0;
    for hidden in &input.hidden_scope {
        let reason_token = hidden.narrowing_reason.as_str();
        if hidden.hidden_count == 0 {
            return Err(ViewBuildError::HiddenCountZero(reason_token.to_owned()));
        }
        if !seen_reasons.insert(hidden.narrowing_reason) {
            return Err(ViewBuildError::DuplicateNarrowingReason(
                reason_token.to_owned(),
            ));
        }
        let explanation = hidden.narrowing_explanation.trim();
        if explanation.is_empty() {
            return Err(ViewBuildError::EmptyNarrowingExplanation(
                reason_token.to_owned(),
            ));
        }
        if explanation.chars().count() > MAX_EXPLANATION_CHARS {
            return Err(ViewBuildError::NarrowingExplanationTooLong(
                reason_token.to_owned(),
            ));
        }
        if !is_canonical_object_ref(&hidden.reveal_ref) {
            return Err(ViewBuildError::RevealRefNotCanonical {
                reason: reason_token.to_owned(),
                reference: hidden.reveal_ref.clone(),
            });
        }
        hidden_total = hidden_total.saturating_add(hidden.hidden_count);
        hidden_scope.push(HiddenScopeCounter {
            narrowing_reason: hidden.narrowing_reason,
            narrowing_reason_token: reason_token.to_owned(),
            narrowing_reason_label: hidden.narrowing_reason.label().to_owned(),
            hidden_count: hidden.hidden_count,
            narrowing_explanation: explanation.to_owned(),
            reveal_ref: hidden.reveal_ref.trim().to_owned(),
            incomplete_knowledge: hidden.narrowing_reason.is_incomplete_knowledge(),
        });
    }
    // Sort hidden buckets by reason token for deterministic output.
    hidden_scope.sort_by(|a, b| a.narrowing_reason_token.cmp(&b.narrowing_reason_token));

    let visible_row_count = rows.len() as u32;
    let mut order_reasons_present: Vec<String> =
        rows.iter().map(|r| r.order_reason_token.clone()).collect();
    order_reasons_present.sort();
    order_reasons_present.dedup();

    let narrowing_present = hidden_total > 0;
    let incomplete_knowledge_present = hidden_scope.iter().any(|h| h.incomplete_knowledge);

    Ok(QueueOrderReason {
        record_kind: QUEUE_ORDER_REASON_RECORD_KIND.to_owned(),
        schema_version: QUEUE_ORDER_REASON_SCHEMA_VERSION,
        queue_id: input.queue_id.trim().to_owned(),
        surface,
        surface_token: surface.as_str().to_owned(),
        surface_label: surface.label().to_owned(),
        visible_row_count,
        hidden_total,
        total_in_scope_count: visible_row_count.saturating_add(hidden_total),
        rows,
        order_reasons_present,
        hidden_scope,
        narrowing_present,
        incomplete_knowledge_present,
    })
}

/// True when `reference` is a canonical durable object ref the chrome can route
/// to: `aureline://<class>/<id>` where `<class>` is a specific object class
/// (not a generic landing page) and `<id>` is non-empty.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.chars().count() > MAX_REF_CHARS {
        return false;
    }
    let rest = match reference.strip_prefix(CANONICAL_OBJECT_SCHEME) {
        Some(rest) => rest,
        None => return false,
    };
    let (class, id) = match rest.split_once('/') {
        Some(parts) => parts,
        None => return false,
    };
    if class.is_empty() || id.is_empty() {
        return false;
    }
    if GENERIC_LANDING_CLASSES.contains(&class) {
        return false;
    }
    true
}

// Parse `YYYY-MM-DDTHH:MM` into elapsed-minute terms relative to `as_of`.
// Mirrors the bucketing used by the service-health aggregator so the two
// surfaces agree on age thresholds.
fn derive_age(evidence_at: &str, as_of: &str) -> EvidenceAgeClass {
    let last = match parse_timestamp_minutes(evidence_at) {
        Some(v) => v,
        None => return EvidenceAgeClass::Never,
    };
    let now = match parse_timestamp_minutes(as_of) {
        Some(v) => v,
        None => return EvidenceAgeClass::Never,
    };
    if now < last {
        // Evidence timestamp is in the future relative to `as_of`; treat as
        // never rather than fabricate a fresh chip.
        return EvidenceAgeClass::Never;
    }
    let delta = now - last;
    if delta <= 5 {
        EvidenceAgeClass::Fresh
    } else if delta <= 60 {
        EvidenceAgeClass::Recent
    } else if delta <= 60 * 24 {
        EvidenceAgeClass::Stale
    } else {
        EvidenceAgeClass::VeryStale
    }
}

fn parse_timestamp_minutes(input: &str) -> Option<i64> {
    let bytes = input.as_bytes();
    if bytes.len() < 16 {
        return None;
    }
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    if bytes[10] != b'T' && bytes[10] != b' ' {
        return None;
    }
    if bytes[13] != b':' {
        return None;
    }
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let month: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let day: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    let hour: u32 = std::str::from_utf8(&bytes[11..13]).ok()?.parse().ok()?;
    let minute: u32 = std::str::from_utf8(&bytes[14..16]).ok()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 {
        return None;
    }
    let day_number = days_from_civil(year, month, day);
    Some(day_number * 24 * 60 + i64::from(hour) * 60 + i64::from(minute))
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let m_i = m as i64;
    let doy = (153 * (if m_i > 2 { m_i - 3 } else { m_i + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_card(id: &str, evidence_at: Option<&str>) -> FreshnessCardInput {
        FreshnessCardInput {
            card_id: id.to_owned(),
            title: format!("Row {id}"),
            displayed_state: DisplayedStateClass::Clear,
            freshness: FreshnessClass::Fresh,
            last_successful_evidence_at: evidence_at.map(str::to_owned),
            evidence_kind: EvidenceKindClass::ServiceHealthCard,
            evidence_ref: format!("aureline://service_health_card/{id}"),
            state_explanation: "All probes current.".to_owned(),
        }
    }

    #[test]
    fn fresh_clear_dashboard_stays_green_with_no_marker() {
        let cards = vec![
            clear_card("card:a", Some("2026-05-20T11:58")),
            clear_card("card:b", Some("2026-05-20T11:55")),
        ];
        let view = DashboardTruthView::build(
            "view:test:clear",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            cards,
            None,
        )
        .expect("build");
        assert_eq!(view.summary.clear_card_count, 2);
        assert_eq!(view.overall_effective_state, EffectiveStateClass::Clear);
        assert!(!view.honesty_marker_present);
        for card in &view.cards {
            assert_eq!(card.effective_state, EffectiveStateClass::Clear);
            assert!(!card.green_downgraded);
            assert!(card.downgrade_reasons.is_empty());
        }
    }

    #[test]
    fn stale_freshness_forces_green_downgrade() {
        let mut card = clear_card("card:stale", Some("2026-05-20T11:58"));
        card.freshness = FreshnessClass::Stale;
        let view = DashboardTruthView::build(
            "view:test:stale",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![card],
            None,
        )
        .unwrap();
        let card = &view.cards[0];
        assert_eq!(card.displayed_state, DisplayedStateClass::Clear);
        assert_eq!(card.effective_state, EffectiveStateClass::Unconfirmed);
        assert!(card.green_downgraded);
        assert!(card
            .downgrade_reasons
            .contains(&DowngradeReasonClass::FreshnessExpired));
        assert!(view.honesty_marker_present);
        assert_eq!(view.summary.green_downgrade_count, 1);
    }

    #[test]
    fn aged_out_evidence_forces_green_downgrade_even_when_source_fresh() {
        // Source claims fresh, but the last-successful evidence is a day old.
        let card = clear_card("card:aged", Some("2026-05-18T12:00"));
        let view = DashboardTruthView::build(
            "view:test:aged",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![card],
            None,
        )
        .unwrap();
        let card = &view.cards[0];
        assert_eq!(card.freshness, FreshnessClass::Fresh);
        assert_eq!(card.evidence_age, EvidenceAgeClass::VeryStale);
        assert_eq!(card.effective_state, EffectiveStateClass::Unconfirmed);
        assert!(card.green_downgraded);
        assert!(card
            .downgrade_reasons
            .contains(&DowngradeReasonClass::EvidenceAgedOut));
    }

    #[test]
    fn never_evidence_cannot_be_green() {
        let card = clear_card("card:never", None);
        let view = DashboardTruthView::build(
            "view:test:never",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![card],
            None,
        )
        .unwrap();
        let card = &view.cards[0];
        assert_eq!(card.evidence_age, EvidenceAgeClass::Never);
        assert_eq!(card.effective_state, EffectiveStateClass::Unconfirmed);
        assert!(card.green_downgraded);
    }

    #[test]
    fn partial_and_offline_carry_distinct_downgrade_reasons() {
        let mut partial = clear_card("card:partial", Some("2026-05-20T11:58"));
        partial.freshness = FreshnessClass::Partial;
        let mut offline = clear_card("card:offline", Some("2026-05-20T11:58"));
        offline.freshness = FreshnessClass::Unavailable;
        let view = DashboardTruthView::build(
            "view:test:po",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![partial, offline],
            None,
        )
        .unwrap();
        let partial = view
            .cards
            .iter()
            .find(|c| c.card_id == "card:partial")
            .unwrap();
        let offline = view
            .cards
            .iter()
            .find(|c| c.card_id == "card:offline")
            .unwrap();
        assert_eq!(
            partial.downgrade_reasons,
            vec![DowngradeReasonClass::SourcePartial]
        );
        assert_eq!(
            offline.downgrade_reasons,
            vec![DowngradeReasonClass::SourceOffline]
        );
    }

    #[test]
    fn attention_and_blocked_carry_through_with_freshness_chip() {
        let mut attention = clear_card("card:att", Some("2026-05-20T11:58"));
        attention.displayed_state = DisplayedStateClass::Attention;
        let mut blocked = clear_card("card:blk", Some("2026-05-20T11:58"));
        blocked.displayed_state = DisplayedStateClass::Blocked;
        let view = DashboardTruthView::build(
            "view:test:ab",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![attention, blocked],
            None,
        )
        .unwrap();
        // Sorted worst-first: blocked before attention.
        assert_eq!(view.cards[0].effective_state, EffectiveStateClass::Blocked);
        assert_eq!(
            view.cards[1].effective_state,
            EffectiveStateClass::Attention
        );
        assert!(!view.cards[0].green_downgraded);
        assert_eq!(view.overall_effective_state, EffectiveStateClass::Blocked);
    }

    #[test]
    fn queue_surface_requires_queue_order() {
        let err = DashboardTruthView::build(
            "view:test:q",
            DashboardSurfaceClass::ReviewInbox,
            "2026-05-20T12:00",
            vec![clear_card("card:a", Some("2026-05-20T11:58"))],
            None,
        )
        .unwrap_err();
        assert_eq!(err, ViewBuildError::QueueExpected);
    }

    #[test]
    fn dashboard_surface_rejects_queue_order() {
        let queue = QueueOrderInput {
            queue_id: "queue:x".to_owned(),
            rows: vec![],
            hidden_scope: vec![],
        };
        let err = DashboardTruthView::build(
            "view:test:dq",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![clear_card("card:a", Some("2026-05-20T11:58"))],
            Some(queue),
        )
        .unwrap_err();
        assert_eq!(err, ViewBuildError::QueueUnexpected);
    }

    #[test]
    fn queue_orders_rows_and_counts_hidden_scope() {
        let cards = vec![
            {
                let mut c = clear_card("card:r1", Some("2026-05-20T11:58"));
                c.displayed_state = DisplayedStateClass::Blocked;
                c.evidence_kind = EvidenceKindClass::ChangeReview;
                c.evidence_ref = "aureline://change_review/r1".to_owned();
                c
            },
            {
                let mut c = clear_card("card:r2", Some("2026-05-20T11:58"));
                c.displayed_state = DisplayedStateClass::Attention;
                c.evidence_kind = EvidenceKindClass::ChangeReview;
                c.evidence_ref = "aureline://change_review/r2".to_owned();
                c
            },
        ];
        let queue = QueueOrderInput {
            queue_id: "queue:review".to_owned(),
            rows: vec![
                QueueRowInput {
                    row_id: "card:r1".to_owned(),
                    order_reason: OrderReasonClass::SeverityDescending,
                    order_explanation: "Blocking review, sorted to top.".to_owned(),
                    open_details_ref: "aureline://change_review/r1".to_owned(),
                },
                QueueRowInput {
                    row_id: "card:r2".to_owned(),
                    order_reason: OrderReasonClass::AssignedToYou,
                    order_explanation: "Assigned to you.".to_owned(),
                    open_details_ref: "aureline://change_review/r2".to_owned(),
                },
            ],
            hidden_scope: vec![HiddenScopeInput {
                narrowing_reason: NarrowingReasonClass::ScopeFilter,
                hidden_count: 4,
                narrowing_explanation: "4 reviews outside the active workspace scope.".to_owned(),
                reveal_ref: "aureline://change_review_query/all_scopes".to_owned(),
            }],
        };
        let view = DashboardTruthView::build(
            "view:test:queue",
            DashboardSurfaceClass::ReviewInbox,
            "2026-05-20T12:00",
            cards,
            Some(queue),
        )
        .unwrap();
        let q = view.queue_order.as_ref().unwrap();
        assert_eq!(q.visible_row_count, 2);
        assert_eq!(q.hidden_total, 4);
        assert_eq!(q.total_in_scope_count, 6);
        assert!(q.narrowing_present);
        assert_eq!(q.rows[0].order_rank, 1);
        assert_eq!(q.rows[0].row_id, "card:r1");
        assert!(view.honesty_marker_present);
    }

    #[test]
    fn queue_row_for_unknown_card_is_rejected() {
        let cards = vec![{
            let mut c = clear_card("card:r1", Some("2026-05-20T11:58"));
            c.evidence_ref = "aureline://incident_record/r1".to_owned();
            c.evidence_kind = EvidenceKindClass::IncidentRecord;
            c
        }];
        let queue = QueueOrderInput {
            queue_id: "queue:inc".to_owned(),
            rows: vec![QueueRowInput {
                row_id: "card:ghost".to_owned(),
                order_reason: OrderReasonClass::DefaultRecency,
                order_explanation: "x".to_owned(),
                open_details_ref: "aureline://incident_record/ghost".to_owned(),
            }],
            hidden_scope: vec![],
        };
        let err = DashboardTruthView::build(
            "view:test:ghost",
            DashboardSurfaceClass::IncidentQueue,
            "2026-05-20T12:00",
            cards,
            Some(queue),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ViewBuildError::QueueRowUnknownCard("card:ghost".to_owned())
        );
    }

    #[test]
    fn generic_landing_evidence_ref_is_rejected() {
        let mut card = clear_card("card:a", Some("2026-05-20T11:58"));
        card.evidence_ref = "aureline://dashboard/home".to_owned();
        let err = DashboardTruthView::build(
            "view:test:landing",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![card],
            None,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ViewBuildError::EvidenceRefNotCanonical { .. }
        ));
    }

    #[test]
    fn non_canonical_evidence_ref_is_rejected() {
        let mut card = clear_card("card:a", Some("2026-05-20T11:58"));
        card.evidence_ref = "https://example.com/thing".to_owned();
        let err = DashboardTruthView::build(
            "view:test:url",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![card],
            None,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ViewBuildError::EvidenceRefNotCanonical { .. }
        ));
    }

    #[test]
    fn duplicate_card_id_is_rejected() {
        let a = clear_card("card:dup", Some("2026-05-20T11:58"));
        let b = clear_card("card:dup", Some("2026-05-20T11:58"));
        let err = DashboardTruthView::build(
            "view:test:dup",
            DashboardSurfaceClass::ServiceHealth,
            "2026-05-20T12:00",
            vec![a, b],
            None,
        )
        .unwrap_err();
        assert_eq!(err, ViewBuildError::DuplicateCardId("card:dup".to_owned()));
    }

    #[test]
    fn missing_queue_row_for_card_is_rejected() {
        let cards = vec![
            {
                let mut c = clear_card("card:r1", Some("2026-05-20T11:58"));
                c.evidence_ref = "aureline://support_case/r1".to_owned();
                c.evidence_kind = EvidenceKindClass::SupportCase;
                c
            },
            {
                let mut c = clear_card("card:r2", Some("2026-05-20T11:58"));
                c.evidence_ref = "aureline://support_case/r2".to_owned();
                c.evidence_kind = EvidenceKindClass::SupportCase;
                c
            },
        ];
        let queue = QueueOrderInput {
            queue_id: "queue:support".to_owned(),
            rows: vec![QueueRowInput {
                row_id: "card:r1".to_owned(),
                order_reason: OrderReasonClass::DefaultRecency,
                order_explanation: "Default order.".to_owned(),
                open_details_ref: "aureline://support_case/r1".to_owned(),
            }],
            hidden_scope: vec![],
        };
        let err = DashboardTruthView::build(
            "view:test:missing",
            DashboardSurfaceClass::SupportQueue,
            "2026-05-20T12:00",
            cards,
            Some(queue),
        )
        .unwrap_err();
        assert_eq!(err, ViewBuildError::QueueRowMissing("card:r2".to_owned()));
    }

    #[test]
    fn canonical_ref_helper_accepts_and_rejects() {
        assert!(is_canonical_object_ref("aureline://incident_record/inc-22"));
        assert!(is_canonical_object_ref("aureline://change_review/cr-1"));
        assert!(!is_canonical_object_ref("aureline://home/x"));
        assert!(!is_canonical_object_ref("aureline://incident_record/"));
        assert!(!is_canonical_object_ref("aureline://incident_record"));
        assert!(!is_canonical_object_ref("incident_record/inc-22"));
        assert!(!is_canonical_object_ref(""));
    }

    #[test]
    fn plaintext_includes_envelope_cards_and_queue() {
        let cards = vec![{
            let mut c = clear_card("card:r1", Some("2026-05-20T11:58"));
            c.displayed_state = DisplayedStateClass::Attention;
            c.evidence_ref = "aureline://incident_record/r1".to_owned();
            c.evidence_kind = EvidenceKindClass::IncidentRecord;
            c
        }];
        let queue = QueueOrderInput {
            queue_id: "queue:inc".to_owned(),
            rows: vec![QueueRowInput {
                row_id: "card:r1".to_owned(),
                order_reason: OrderReasonClass::SeverityDescending,
                order_explanation: "Most severe first.".to_owned(),
                open_details_ref: "aureline://incident_record/r1".to_owned(),
            }],
            hidden_scope: vec![HiddenScopeInput {
                narrowing_reason: NarrowingReasonClass::SeverityFilter,
                hidden_count: 3,
                narrowing_explanation: "3 below the severity filter.".to_owned(),
                reveal_ref: "aureline://incident_query/all_severities".to_owned(),
            }],
        };
        let view = DashboardTruthView::build(
            "view:test:plain",
            DashboardSurfaceClass::IncidentQueue,
            "2026-05-20T12:00",
            cards,
            Some(queue),
        )
        .unwrap();
        let text = view.render_plaintext();
        assert!(text.contains("Dashboard & queue truth"));
        assert!(text.contains("View: view:test:plain"));
        assert!(text.contains("card:r1"));
        assert!(text.contains("Queue order (queue:inc)"));
        assert!(text.contains("hidden severity_filter x3"));
    }
}
