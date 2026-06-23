//! M5 *badge aggregates*: the working engine that turns a set of durable attention
//! items into deduped, per-scope badge counts, coalesces repeated failures from one
//! root cause into a single durable object, projects one shared count across every
//! badge-bearing surface, and emits stable telemetry enums — never a raw event tally.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes the
//! contract* — including the
//! [`BadgeAggregate`](crate::m5_attention_routing::AttentionObjectClass::BadgeAggregate)
//! object family and its required `scope_key` / `count` / `count_class` / `freshness` /
//! `deduped_objects` / `muted_reasons` / `suppressed_reasons` / `active_quiet_hours_modes`
//! fields — and [`m5_fanout_receipts`](crate::m5_fanout_receipts) *records per-destination
//! delivery truth*, this lane turns a badge number into *governed, deduped truth* rather
//! than a surface-local accumulation of raw events. The honesty rules the track invariant
//! requires are enforced, not described:
//!
//! - **Counts derive from deduped durable items.** A scope's badge count equals the number
//!   of distinct durable objects pending attention in that scope, after canonical-key
//!   dedupe; repeated raw events for one object never inflate the number
//!   (`badge.counts_deduped_durable_items`).
//! - **Muted, suppressed, and deferred items are excluded but explained.** They never
//!   increment the count, yet the aggregate names their muted, suppressed, and
//!   quiet-hours reasons so the number is auditable (`badge.excluded_reasons_named`,
//!   `badge.muted_suppressed_not_counted`).
//! - **Repeated failures coalesce.** Failures sharing one root cause collapse into one
//!   user-comprehensible durable object — counted once in the badge — with the correct
//!   authoritative reopen path, instead of spamming toasts, banners, badges, and companion
//!   alerts (`badge.repeated_failures_coalesce`,
//!   `badge.coalesced_failure_reopen_authoritative`).
//! - **One count across every surface.** The shell activity center, dock/taskbar badge,
//!   browser and mobile companions, and operator dashboard project the *same* aggregate;
//!   their counts and count classes match (`badge.cross_surface_parity`), and no surface
//!   widens privacy below the aggregate floor (`badge.surface_never_widens_privacy`).
//! - **Reopen to the authoritative object.** Every badge reopens an activity row anchored
//!   on the exact authoritative object (for a single-item badge) or the scope's pending
//!   list (for a coalesced badge), never an ambiguous generic shell
//!   (`badge.route_to_authoritative`).
//! - **Security advisories are never silenced.** A security advisory in an active state is
//!   always counted, regardless of any mute, suppression, or quiet-hours signal
//!   (`badge.security_never_silenced`).
//! - **Telemetry is stable enums and counts only.** Support exports and telemetry record
//!   the notification class, route, and outcome by stable token plus a count, never a
//!   message body or payload (`badge.telemetry_stable_enums_no_text`).
//!
//! The canonical [`badge_aggregates_bundle`] freezes the governed surfaces, a representative
//! durable-item corpus, every per-scope aggregate, every per-surface badge, the coalesced
//! failures, and the telemetry packet so the freeze gate and checked-in fixture pin the
//! contract byte-for-byte. Every privacy class, scope, redaction class, reopen target,
//! severity, dedupe scheme, suppression reason, and quiet-hours mode the bundle uses is one
//! the attention-routing matrix defines, so the badge path can never drift from the frozen
//! object model (`badge.matrix_bound`).
//!
//! The record carries no message bodies, credentials, raw provider payloads, hostnames,
//! device identifiers, or absolute paths — only opaque object refs, stable tokens, short
//! reviewable sentences, and counts — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionObjectClass,
    AttentionRedactionClass, AttentionRoutingMatrix, AttentionScopeClass, AttentionStateClass,
    FanoutChannelClass, NotificationPrivacyClass, ReopenTargetClass,
    M5_ATTENTION_ROUTING_MATRIX_ID,
};
use crate::m5_envelope_routing::{
    DedupeStrategyClass, NotificationSeverityClass, SourceSubsystemClass,
};

#[cfg(test)]
mod tests;

/// Schema version for the badge-aggregates bundle.
pub const M5_BADGE_AGGREGATES_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the badge-aggregates bundle.
pub const M5_BADGE_AGGREGATES_SCHEMA_REF: &str = "schemas/activity/m5-badge-aggregates.schema.json";

/// Stable record-kind tag for the badge-aggregates bundle.
pub const M5_BADGE_AGGREGATES_RECORD_KIND: &str = "m5_badge_aggregates_bundle";

/// Stable id for the canonical badge-aggregates bundle.
pub const M5_BADGE_AGGREGATES_BUNDLE_ID: &str = "m5-badge-aggregates:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding stays
/// deterministic and the fixture freezes byte-for-byte.
pub const M5_BADGE_AGGREGATES_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_BADGE_AGGREGATES_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate; it fails
/// when the in-code bundle drifts from the checked-in fixture or any invariant flips.
pub const M5_BADGE_AGGREGATES_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_badge_aggregates.rs";

/// The badge-bearing surfaces whose counts must match the same deduped durable truth: the
/// shell activity center, the dock/taskbar badge, the browser and mobile companions, and the
/// operator dashboard. The in-app activity center is the authoritative durable record; the
/// others project the same aggregate.
pub const GOVERNED_BADGE_SURFACES: [FanoutChannelClass; 5] = [
    FanoutChannelClass::InAppActivityCenter,
    FanoutChannelClass::DockTaskbarBadge,
    FanoutChannelClass::BrowserCompanion,
    FanoutChannelClass::MobileCompanion,
    FanoutChannelClass::OperatorDashboard,
];

/// The surfaces a coalesced failure protects from repeated-alert spam: without coalescing,
/// each raw failure would push one OS notification, increment the badge, ping each companion,
/// and raise an operator banner.
pub const SPAM_PRONE_SURFACES: [FanoutChannelClass; 5] = [
    FanoutChannelClass::OsNativeNotification,
    FanoutChannelClass::DockTaskbarBadge,
    FanoutChannelClass::BrowserCompanion,
    FanoutChannelClass::MobileCompanion,
    FanoutChannelClass::OperatorDashboard,
];

/// Every scope namespace, in canonical order, so per-scope aggregates are deterministic.
pub const ALL_SCOPES: [AttentionScopeClass; 6] = [
    AttentionScopeClass::AppGlobal,
    AttentionScopeClass::Window,
    AttentionScopeClass::Workspace,
    AttentionScopeClass::Session,
    AttentionScopeClass::Collaboration,
    AttentionScopeClass::TenantOrg,
];

// ---------------------------------------------------------------------------
// Badge vocabulary.
// ---------------------------------------------------------------------------

/// The coarse count class a badge number falls into, so governance and telemetry reason
/// about magnitude without echoing an exact, spammy number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountClass {
    /// No pending attention (count is zero).
    None,
    /// Exactly one pending durable object.
    Single,
    /// A few pending objects (2–9).
    Few,
    /// Many pending objects (10–98).
    Many,
    /// A saturated count (99 or more), displayed as `99+`.
    Saturated,
}

impl CountClass {
    /// All count classes, in order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::Single,
        Self::Few,
        Self::Many,
        Self::Saturated,
    ];

    /// Classifies a count into its coarse class.
    pub const fn classify(count: usize) -> Self {
        match count {
            0 => Self::None,
            1 => Self::Single,
            2..=9 => Self::Few,
            10..=98 => Self::Many,
            _ => Self::Saturated,
        }
    }

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Few => "few",
            Self::Many => "many",
            Self::Saturated => "saturated",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Single => "Single",
            Self::Few => "Few",
            Self::Many => "Many",
            Self::Saturated => "Saturated (99+)",
        }
    }
}

/// How a count is displayed on a badge: the exact number, or `99+` once saturated.
pub fn count_display(count: usize) -> String {
    if matches!(CountClass::classify(count), CountClass::Saturated) {
        "99+".to_owned()
    } else {
        count.to_string()
    }
}

/// How fresh the freshest counted item in an aggregate is. The aggregate takes the freshest
/// (lowest-ranked) of its counted items; an empty scope is [`BadgeFreshnessClass::None`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeFreshnessClass {
    /// Just now.
    Fresh,
    /// Within the recent window.
    Recent,
    /// Aging.
    Aging,
    /// Stale.
    Stale,
    /// No counted items.
    None,
}

impl BadgeFreshnessClass {
    /// All freshness classes, freshest first.
    pub const ALL: [Self; 5] = [
        Self::Fresh,
        Self::Recent,
        Self::Aging,
        Self::Stale,
        Self::None,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::None => "none",
        }
    }
}

/// Why an item is muted out of the badge count — a user- or focus-driven silencing of the
/// source, distinct from a policy suppression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeMuteReasonClass {
    /// Not muted.
    None,
    /// The user muted the source.
    UserMutedSource,
    /// Muted by an active focus / presentation mode.
    MutedByFocusMode,
    /// Muted by a per-scope rule.
    MutedByScopeRule,
}

impl BadgeMuteReasonClass {
    /// All mute reasons, in order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::UserMutedSource,
        Self::MutedByFocusMode,
        Self::MutedByScopeRule,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::UserMutedSource => "user_muted_source",
            Self::MutedByFocusMode => "muted_by_focus_mode",
            Self::MutedByScopeRule => "muted_by_scope_rule",
        }
    }

    /// Whether this is a named mute reason (anything but [`BadgeMuteReasonClass::None`]).
    pub const fn is_named(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Why an item is suppressed out of the badge count by policy or the routing engine, kept
/// distinct from a user mute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSuppressionReasonClass {
    /// Not suppressed.
    None,
    /// Suppressed by an admin / managed policy.
    PolicySuppressed,
    /// Rate-limited by the routing engine.
    RateLimited,
    /// Superseded by a newer event for the same object.
    Superseded,
}

impl BadgeSuppressionReasonClass {
    /// All suppression reasons, in order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::PolicySuppressed,
        Self::RateLimited,
        Self::Superseded,
    ];

    /// Stable snake_case token. Each named token is one the attention-routing matrix's
    /// `suppression` vocabulary defines.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PolicySuppressed => "policy_suppressed",
            Self::RateLimited => "rate_limited",
            Self::Superseded => "superseded",
        }
    }

    /// Whether this is a named suppression reason.
    pub const fn is_named(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The active quiet-hours behavior that deferred an item out of the badge count.
///
/// Each named token is one the attention-routing matrix's `quiet_hours` vocabulary defines,
/// so the deferral reason binds back to the frozen model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuietHoursModeClass {
    /// Not deferred by quiet hours.
    None,
    /// Defer unless the user opted in.
    DeferUnlessOptedIn,
    /// Defer to in-product surfaces only.
    DeferToInProduct,
    /// Follow the admin / managed policy.
    FollowAdminPolicy,
    /// Always defer.
    AlwaysDefer,
}

impl QuietHoursModeClass {
    /// All quiet-hours modes, in order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::DeferUnlessOptedIn,
        Self::DeferToInProduct,
        Self::FollowAdminPolicy,
        Self::AlwaysDefer,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DeferUnlessOptedIn => "defer_unless_opted_in",
            Self::DeferToInProduct => "defer_to_in_product",
            Self::FollowAdminPolicy => "follow_admin_policy",
            Self::AlwaysDefer => "always_defer",
        }
    }

    /// Whether quiet hours is actively deferring this item.
    pub const fn is_deferring(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// How a durable item contributes to (or is excluded from) the badge count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeContributionClass {
    /// Counts toward the badge: a durable item pending attention, not muted, suppressed, or
    /// deferred (or a security advisory, which is always counted).
    Counted,
    /// Excluded — muted by the user or a focus/scope rule.
    Muted,
    /// Excluded — suppressed by policy or the routing engine.
    Suppressed,
    /// Excluded — deferred by quiet hours.
    QuietHoursDeferred,
    /// Excluded — already settled (acknowledged, resolved, dismissed, archived, completed).
    Settled,
}

impl BadgeContributionClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counted => "counted",
            Self::Muted => "muted",
            Self::Suppressed => "suppressed",
            Self::QuietHoursDeferred => "quiet_hours_deferred",
            Self::Settled => "settled",
        }
    }

    /// Whether this contribution increments the badge count.
    pub const fn counts(self) -> bool {
        matches!(self, Self::Counted)
    }
}

/// The stable outcome a notification reached for telemetry, derived from its badge
/// contribution and dedupe position. Carries no message text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationOutcomeClass {
    /// Counted in the badge as a distinct durable object.
    CountedInBadge,
    /// A repeated raw event coalesced into an already-counted object.
    DedupedRepeat,
    /// Muted out of the count.
    Muted,
    /// Suppressed out of the count.
    Suppressed,
    /// Deferred out of the count by quiet hours.
    QuietHoursDeferred,
    /// Already settled, so not counted.
    Settled,
}

impl NotificationOutcomeClass {
    /// All outcomes, in order.
    pub const ALL: [Self; 6] = [
        Self::CountedInBadge,
        Self::DedupedRepeat,
        Self::Muted,
        Self::Suppressed,
        Self::QuietHoursDeferred,
        Self::Settled,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CountedInBadge => "counted_in_badge",
            Self::DedupedRepeat => "deduped_repeat",
            Self::Muted => "muted",
            Self::Suppressed => "suppressed",
            Self::QuietHoursDeferred => "quiet_hours_deferred",
            Self::Settled => "settled",
        }
    }
}

// ---------------------------------------------------------------------------
// Input: the durable attention item.
// ---------------------------------------------------------------------------

/// The badge-relevant projection of a durable activity object or notification envelope.
///
/// It carries the stable identity, the canonical dedupe key, the scope and severity, the
/// state, and the mute/suppression/quiet-hours signals — but never raw message text. The
/// badge engine derives a [`BadgeContributionClass`] from these signals; it never trusts a
/// pre-counted total.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableAttentionItem {
    /// Stable, namespaced item id.
    pub item_id: String,
    /// The canonical event id used for cross-client dedupe.
    pub canonical_event_id: String,
    /// The key repeated events for the same object coalesce on. Items sharing this key in a
    /// scope count as one durable object.
    pub dedupe_key: String,
    /// Human-readable label (a control-plane label, never a message body).
    pub label: String,
    /// The source subsystem that produced the event.
    pub source_subsystem: SourceSubsystemClass,
    /// The severity class.
    pub severity: NotificationSeverityClass,
    /// The scope namespace the attention applies to.
    pub scope: AttentionScopeClass,
    /// The privacy class governing what a surface may render for this item.
    pub privacy_class: NotificationPrivacyClass,
    /// How repeated events for this item coalesce.
    pub dedupe_key_scheme: DedupeStrategyClass,
    /// The item's lifecycle state.
    pub state: AttentionStateClass,
    /// How fresh the item is.
    pub freshness: BadgeFreshnessClass,
    /// The mute reason, if the item is muted out of the count.
    pub mute_reason: BadgeMuteReasonClass,
    /// The suppression reason, if the item is suppressed out of the count.
    pub suppression_reason: BadgeSuppressionReasonClass,
    /// The active quiet-hours mode, if quiet hours deferred the item.
    pub quiet_hours_mode: QuietHoursModeClass,
    /// The root-cause key repeated failures coalesce on. Empty unless the item is a
    /// coalescible failure.
    pub root_cause_key: String,
    /// The authoritative object this item reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (never a URL, host, or path).
    pub reopen_anchor_ref: String,
    /// Whether the item is backed by a durable authoritative record (always true — a badge
    /// is never derived from toast-only attention).
    pub carries_durable_record: bool,
}

impl DurableAttentionItem {
    /// How this item contributes to the badge count.
    pub fn contribution(&self) -> BadgeContributionClass {
        // A settled item never counts, whatever its reasons.
        if !state_is_active(self.state) {
            return BadgeContributionClass::Settled;
        }
        // A security advisory in an active state is never silenced by a mute, suppression,
        // or quiet-hours signal.
        if self.severity.is_security() {
            return BadgeContributionClass::Counted;
        }
        if self.mute_reason.is_named() {
            return BadgeContributionClass::Muted;
        }
        if self.suppression_reason.is_named() {
            return BadgeContributionClass::Suppressed;
        }
        if self.quiet_hours_mode.is_deferring() {
            return BadgeContributionClass::QuietHoursDeferred;
        }
        BadgeContributionClass::Counted
    }

    /// Whether this item is a coalescible failure (a failed state with a root-cause key).
    pub fn is_coalescible_failure(&self) -> bool {
        self.state == AttentionStateClass::Failed && !self.root_cause_key.is_empty()
    }
}

/// Whether a state is an active, attention-pending state that can contribute to a badge.
///
/// Settled, snoozed, suppressed, deferred, and unknown states do not contribute; their
/// durable records remain, but the badge counts only live, pending attention.
const fn state_is_active(state: AttentionStateClass) -> bool {
    use AttentionStateClass::*;
    matches!(
        state,
        Pending | Routed | Shown | Running | QueuedWaiting | PartiallyCompleted | Failed
    )
}

// ---------------------------------------------------------------------------
// Output objects.
// ---------------------------------------------------------------------------

/// One distinct durable object a badge counts: the representative of a dedupe group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeObjectRef {
    /// The dedupe key that identifies this object.
    pub object_key: String,
    /// The canonical event id of the representative item.
    pub canonical_event_id: String,
    /// Human-readable label.
    pub label: String,
    /// The strongest severity among the events that coalesced into this object.
    pub severity: NotificationSeverityClass,
    /// Stable severity token.
    pub severity_token: String,
    /// The freshest freshness among the events that coalesced into this object.
    pub freshness: BadgeFreshnessClass,
    /// The authoritative object this badge entry reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque, exact reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// The deduped, per-scope badge aggregate: a coarse, durable-truth count with the muted,
/// suppressed, and quiet-hours reasons that explain it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregate {
    /// Stable, namespaced aggregate id.
    pub aggregate_id: String,
    /// The scope this aggregate counts.
    pub scope: AttentionScopeClass,
    /// Stable scope key token.
    pub scope_key: String,
    /// The deduped count of distinct durable objects pending attention in this scope.
    pub count: usize,
    /// The coarse count class.
    pub count_class: CountClass,
    /// Stable count-class token.
    pub count_class_token: String,
    /// How the count is displayed (the number, or `99+`).
    pub count_display: String,
    /// The number of raw counted events before dedupe, so the dedupe ratio is visible.
    pub raw_event_count: usize,
    /// The number of distinct durable objects (equal to [`count`](Self::count)).
    pub deduped_count: usize,
    /// The freshest freshness among counted items.
    pub freshness: BadgeFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// The weakest redaction a surface may apply to this badge (the privacy floor).
    pub privacy_floor: AttentionRedactionClass,
    /// Stable privacy-floor token.
    pub privacy_floor_token: String,
    /// The distinct durable objects this badge counts, each reopenable.
    pub deduped_objects: Vec<BadgeObjectRef>,
    /// The distinct named mute reasons that kept items out of the count.
    pub muted_reasons: Vec<BadgeMuteReasonClass>,
    /// The distinct named suppression reasons that kept items out of the count.
    pub suppressed_reasons: Vec<BadgeSuppressionReasonClass>,
    /// The distinct active quiet-hours modes that deferred items out of the count.
    pub active_quiet_hours_modes: Vec<QuietHoursModeClass>,
    /// The reopen target opening this badge lands on.
    pub reopen_target: ReopenTargetClass,
    /// The exact reopen anchor ref: the single object when the count is one, else the scope's
    /// pending list.
    pub reopen_anchor_ref: String,
    /// Whether the count derives from deduped durable items rather than raw events (always
    /// true for this record).
    pub derives_from_durable_items: bool,
    /// One reviewable sentence explaining the count.
    pub note: String,
}

/// One badge-bearing surface's projection of a scope aggregate. Every surface shares the
/// same count; only the redaction posture differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceBadge {
    /// The badge-bearing surface.
    pub surface: FanoutChannelClass,
    /// Stable surface channel id.
    pub surface_id: String,
    /// The scope this badge counts.
    pub scope: AttentionScopeClass,
    /// Stable scope key token.
    pub scope_key: String,
    /// The count, identical to the aggregate's.
    pub count: usize,
    /// The count class, identical to the aggregate's.
    pub count_class: CountClass,
    /// Stable count-class token.
    pub count_class_token: String,
    /// How the count is displayed.
    pub count_display: String,
    /// The redaction this surface applies (at least as strong as the aggregate floor).
    pub applied_redaction: AttentionRedactionClass,
    /// Stable applied-redaction token.
    pub applied_redaction_token: String,
    /// The reopen target, identical to the aggregate's.
    pub reopen_target: ReopenTargetClass,
    /// The reopen anchor ref, identical to the aggregate's.
    pub reopen_anchor_ref: String,
    /// One reviewable sentence.
    pub note: String,
}

/// One governed-surface registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedSurfaceEntry {
    /// The surface.
    pub surface: FanoutChannelClass,
    /// Stable surface channel id.
    pub surface_id: String,
    /// Human-readable label.
    pub label: String,
    /// The surface's base redaction before the aggregate floor is applied.
    pub base_redaction: AttentionRedactionClass,
    /// Stable base-redaction token.
    pub base_redaction_token: String,
    /// Whether this surface holds the authoritative durable record (true only for the in-app
    /// activity center).
    pub is_durable_authoritative: bool,
    /// One reviewable sentence describing how this lane treats the surface.
    pub note: String,
}

/// One coalesced failure: repeated failures from one root cause collapsed into a single
/// durable object with the correct authoritative reopen path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoalescedFailure {
    /// Stable, namespaced coalesced-failure id.
    pub coalesced_id: String,
    /// The root cause the failures share.
    pub root_cause_key: String,
    /// Human-readable label.
    pub label: String,
    /// The scope the failures belong to.
    pub scope: AttentionScopeClass,
    /// Stable scope key token.
    pub scope_key: String,
    /// How many raw failures coalesced into this object.
    pub occurrence_count: usize,
    /// The representative item id whose authoritative object this coalesced failure reopens.
    pub representative_item_id: String,
    /// The representative item's canonical event id.
    pub canonical_event_id: String,
    /// The dedupe scheme used (root-cause collapse).
    pub dedupe_key_scheme: DedupeStrategyClass,
    /// Stable dedupe-scheme token.
    pub dedupe_key_scheme_token: String,
    /// The strongest severity among the coalesced failures.
    pub severity: NotificationSeverityClass,
    /// Stable severity token.
    pub severity_token: String,
    /// The lifecycle state of the coalesced object (failed).
    pub state: AttentionStateClass,
    /// Stable state token.
    pub state_token: String,
    /// The authoritative object this coalesced failure reopens.
    pub reopen_target: ReopenTargetClass,
    /// The exact reopen anchor ref (the representative item's authoritative anchor).
    pub reopen_anchor_ref: String,
    /// The surfaces this collapse protects from repeated-alert spam.
    pub coalesced_surfaces: Vec<FanoutChannelClass>,
    /// Whether this collapse prevented spam (more than one occurrence).
    pub spam_prevented: bool,
    /// Whether the authoritative in-product record is present (always true).
    pub durable_record_present: bool,
    /// One reviewable sentence.
    pub note: String,
}

// ---------------------------------------------------------------------------
// Telemetry.
// ---------------------------------------------------------------------------

/// One telemetry row: how many notifications of a class reached an outcome. Stable tokens
/// and a count only — no message text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryOutcomeRow {
    /// The notification class (the producing subsystem).
    pub notification_class: SourceSubsystemClass,
    /// Stable notification-class token.
    pub notification_class_token: String,
    /// The outcome.
    pub outcome: NotificationOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// How many notifications of this class reached this outcome.
    pub count: usize,
}

/// One telemetry row per route (badge-bearing surface): the total badge count it shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryRouteRow {
    /// The route (badge-bearing surface).
    pub route: FanoutChannelClass,
    /// Stable route token.
    pub route_token: String,
    /// The total badge count this route shows across every scope.
    pub badge_count_total: usize,
    /// The count class of that total.
    pub count_class: CountClass,
    /// Stable count-class token.
    pub count_class_token: String,
}

/// The badge telemetry packet: stable notification class / route / outcome enums plus counts,
/// with no message text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeTelemetry {
    /// Total durable items considered.
    pub total_items: usize,
    /// Total distinct durable objects counted across every scope.
    pub total_counted: usize,
    /// Total raw counted events before dedupe.
    pub total_raw_counted: usize,
    /// Total raw events coalesced into an already-counted object.
    pub total_deduped_repeats: usize,
    /// Total items muted out of the count.
    pub total_muted: usize,
    /// Total items suppressed out of the count.
    pub total_suppressed: usize,
    /// Total items deferred out of the count by quiet hours.
    pub total_deferred: usize,
    /// Total items already settled.
    pub total_settled: usize,
    /// Total coalesced-failure objects.
    pub total_coalesced_failures: usize,
    /// Total raw failure occurrences across all coalesced-failure objects.
    pub total_failure_occurrences: usize,
    /// Counts by notification class and outcome.
    pub outcome_rollup: Vec<TelemetryOutcomeRow>,
    /// Total badge count per route (badge-bearing surface).
    pub route_rollup: Vec<TelemetryRouteRow>,
    /// Whether any message text is captured (always false for this record).
    pub captures_message_text: bool,
    /// One reviewable sentence.
    pub note: String,
}

// ---------------------------------------------------------------------------
// The bundle.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregatesInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen badge-aggregates bundle: the governed surfaces, the durable-item corpus, every
/// per-scope aggregate, every per-surface badge, the coalesced failures, and the telemetry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregatesBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_badge_aggregates_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix this bundle binds its vocabulary back to.
    pub matrix_ref: String,
    /// The matrix id the bundle binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps the bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the bundle.
    pub summary: String,
    /// The governed badge-bearing surfaces.
    pub governed_surfaces: Vec<GovernedSurfaceEntry>,
    /// The representative durable-item corpus.
    pub items: Vec<DurableAttentionItem>,
    /// Every per-scope aggregate.
    pub aggregates: Vec<BadgeAggregate>,
    /// Every per-surface badge (one per surface per scope aggregate).
    pub surface_badges: Vec<SurfaceBadge>,
    /// The coalesced failures.
    pub coalesced_failures: Vec<CoalescedFailure>,
    /// The telemetry packet.
    pub telemetry: BadgeTelemetry,
    /// The computed invariants.
    pub invariants: Vec<BadgeAggregatesInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeAggregatesValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for BadgeAggregatesValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "badge-aggregates bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for BadgeAggregatesValidationError {}

impl BadgeAggregatesBundle {
    /// The aggregate for a scope, if present.
    pub fn aggregate(&self, scope: AttentionScopeClass) -> Option<&BadgeAggregate> {
        self.aggregates.iter().find(|a| a.scope == scope)
    }

    /// The surface badge for a `(surface, scope)` pair, if present.
    pub fn surface_badge(
        &self,
        surface: FanoutChannelClass,
        scope: AttentionScopeClass,
    ) -> Option<&SurfaceBadge> {
        self.surface_badges
            .iter()
            .find(|b| b.surface == surface && b.scope == scope)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded,
    /// no message text is captured, and every ref is a repo-relative object ref or opaque
    /// `aureline://` handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded || self.telemetry.captures_message_text {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let fixed = [
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
            self.schema_ref.as_str(),
        ]
        .into_iter();
        let from_items = self.items.iter().map(|i| i.reopen_anchor_ref.as_str());
        let from_aggregates = self.aggregates.iter().flat_map(|a| {
            std::iter::once(a.reopen_anchor_ref.as_str()).chain(
                a.deduped_objects
                    .iter()
                    .map(|o| o.reopen_anchor_ref.as_str()),
            )
        });
        let from_surface = self
            .surface_badges
            .iter()
            .map(|b| b.reopen_anchor_ref.as_str());
        let from_failures = self
            .coalesced_failures
            .iter()
            .map(|f| f.reopen_anchor_ref.as_str());
        fixed
            .chain(from_items)
            .chain(from_aggregates)
            .chain(from_surface)
            .chain(from_failures)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), BadgeAggregatesValidationError> {
        let fail = |reason: String| Err(BadgeAggregatesValidationError { reason });

        if self.record_kind != M5_BADGE_AGGREGATES_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_BADGE_AGGREGATES_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.items.is_empty() || self.aggregates.is_empty() {
            return fail("items and aggregates must be non-empty".to_owned());
        }

        // The governed surfaces are exactly the five badge-bearing surfaces.
        if self.governed_surfaces.len() != GOVERNED_BADGE_SURFACES.len()
            || !GOVERNED_BADGE_SURFACES
                .iter()
                .all(|s| self.governed_surfaces.iter().any(|e| e.surface == *s))
        {
            return fail("governed surfaces must be exactly the five badge surfaces".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.items.iter().map(|i| i.item_id.as_str())) {
            return fail("item ids are not unique".to_owned());
        }
        if !all_unique(self.aggregates.iter().map(|a| a.aggregate_id.as_str())) {
            return fail("aggregate ids are not unique".to_owned());
        }
        if !all_unique(
            self.coalesced_failures
                .iter()
                .map(|f| f.coalesced_id.as_str()),
        ) {
            return fail("coalesced-failure ids are not unique".to_owned());
        }

        // Every item carries a durable record and an exact reopen anchor.
        for item in &self.items {
            if !item.carries_durable_record {
                return fail(format!("item {} must carry a durable record", item.item_id));
            }
            if item.reopen_anchor_ref.is_empty() || item.canonical_event_id.is_empty() {
                return fail(format!(
                    "item {} is missing its anchor or canonical event id",
                    item.item_id
                ));
            }
        }

        // The aggregates, surface badges, coalesced failures, and telemetry reproduce from
        // the corpus.
        if aggregate_badges(&self.items) != self.aggregates {
            return fail("aggregates are not reproducible from the item corpus".to_owned());
        }
        if surface_badges(&self.aggregates) != self.surface_badges {
            return fail("surface badges are not reproducible from the aggregates".to_owned());
        }
        if coalesce_failures(&self.items) != self.coalesced_failures {
            return fail("coalesced failures are not reproducible from the item corpus".to_owned());
        }
        if badge_telemetry(&self.items, &self.aggregates, &self.coalesced_failures)
            != self.telemetry
        {
            return fail("telemetry is not reproducible from the corpus".to_owned());
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
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

// ---------------------------------------------------------------------------
// The aggregation engine.
// ---------------------------------------------------------------------------

/// Aggregates a durable-item corpus into one badge aggregate per scope that has items, in
/// canonical scope order.
///
/// Deterministic and pure: the same corpus yields the same aggregates every call, so a badge
/// count is reproducible in support export and CLI/headless diagnostics. The count derives
/// from deduped durable items — muted, suppressed, deferred, and settled items are excluded
/// from the number but their reasons are named.
pub fn aggregate_badges(items: &[DurableAttentionItem]) -> Vec<BadgeAggregate> {
    ALL_SCOPES
        .iter()
        .filter(|scope| items.iter().any(|i| i.scope == **scope))
        .map(|scope| aggregate_scope(*scope, items))
        .collect()
}

/// Aggregates the items of one scope into a single badge aggregate.
pub fn aggregate_scope(
    scope: AttentionScopeClass,
    items: &[DurableAttentionItem],
) -> BadgeAggregate {
    let scoped: Vec<&DurableAttentionItem> = items.iter().filter(|i| i.scope == scope).collect();

    // Counted items, deduped by key into distinct durable objects (first occurrence wins).
    let counted: Vec<&DurableAttentionItem> = scoped
        .iter()
        .copied()
        .filter(|i| i.contribution().counts())
        .collect();
    let raw_event_count = counted.len();

    let mut deduped_objects: Vec<BadgeObjectRef> = Vec::new();
    for item in &counted {
        if let Some(existing) = deduped_objects
            .iter_mut()
            .find(|o| o.object_key == item.dedupe_key)
        {
            // Fold the repeat into the existing object: keep the strongest severity and the
            // freshest freshness, but never grow the count.
            existing.severity = existing.severity.max(item.severity);
            existing.severity_token = existing.severity.as_str().to_owned();
            existing.freshness = existing.freshness.min(item.freshness);
        } else {
            deduped_objects.push(BadgeObjectRef {
                object_key: item.dedupe_key.clone(),
                canonical_event_id: item.canonical_event_id.clone(),
                label: item.label.clone(),
                severity: item.severity,
                severity_token: item.severity.as_str().to_owned(),
                freshness: item.freshness,
                reopen_target: item.reopen_target,
                reopen_anchor_ref: item.reopen_anchor_ref.clone(),
            });
        }
    }

    let count = deduped_objects.len();
    let count_class = CountClass::classify(count);

    // The freshest counted item drives the badge freshness; an empty scope is `None`.
    let freshness = deduped_objects
        .iter()
        .map(|o| o.freshness)
        .min()
        .unwrap_or(BadgeFreshnessClass::None);

    let muted_reasons = distinct_in_order(
        &BadgeMuteReasonClass::ALL,
        scoped
            .iter()
            .filter(|i| i.contribution() == BadgeContributionClass::Muted)
            .map(|i| i.mute_reason),
    );
    let suppressed_reasons = distinct_in_order(
        &BadgeSuppressionReasonClass::ALL,
        scoped
            .iter()
            .filter(|i| i.contribution() == BadgeContributionClass::Suppressed)
            .map(|i| i.suppression_reason),
    );
    let active_quiet_hours_modes = distinct_in_order(
        &QuietHoursModeClass::ALL,
        scoped
            .iter()
            .filter(|i| i.contribution() == BadgeContributionClass::QuietHoursDeferred)
            .map(|i| i.quiet_hours_mode),
    );

    // The privacy floor is the strongest redaction any counted item requires; an empty scope
    // defaults to summary-only.
    let privacy_floor = counted
        .iter()
        .map(|i| privacy_floor(i.privacy_class))
        .max()
        .unwrap_or(AttentionRedactionClass::SummaryOnly);

    // Opening the badge always lands on the activity row — anchored on the single object when
    // the count is one, else the scope's pending list — so the reopen target stays within the
    // badge object's frozen vocabulary while still routing to the exact authoritative object.
    let reopen_target = ReopenTargetClass::ActivityJobRow;
    let reopen_anchor_ref = if count == 1 {
        deduped_objects[0].reopen_anchor_ref.clone()
    } else {
        format!("aureline://activity/{}/pending", scope.as_str())
    };

    let note = aggregate_note(
        scope,
        count,
        raw_event_count,
        &muted_reasons,
        &suppressed_reasons,
        &active_quiet_hours_modes,
    );

    BadgeAggregate {
        aggregate_id: format!("m5-badge-aggregates:aggregate:{}", scope.as_str()),
        scope,
        scope_key: scope.as_str().to_owned(),
        count,
        count_class,
        count_class_token: count_class.as_str().to_owned(),
        count_display: count_display(count),
        raw_event_count,
        deduped_count: count,
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        privacy_floor,
        privacy_floor_token: redaction_token(privacy_floor).to_owned(),
        deduped_objects,
        muted_reasons,
        suppressed_reasons,
        active_quiet_hours_modes,
        reopen_target,
        reopen_anchor_ref,
        derives_from_durable_items: true,
        note,
    }
}

fn aggregate_note(
    scope: AttentionScopeClass,
    count: usize,
    raw_event_count: usize,
    muted: &[BadgeMuteReasonClass],
    suppressed: &[BadgeSuppressionReasonClass],
    deferred: &[QuietHoursModeClass],
) -> String {
    let excluded = muted.len() + suppressed.len() + deferred.len();
    if count == 0 {
        format!(
            "The {} badge shows no pending attention; {} reason class(es) explain why eligible \
             items are muted, suppressed, or deferred rather than counted.",
            scope.as_str(),
            excluded,
        )
    } else if raw_event_count > count {
        format!(
            "The {} badge counts {} distinct durable object(s) from {} raw event(s); repeats are \
             deduped, not added, and the count reopens the underlying activity row.",
            scope.as_str(),
            count,
            raw_event_count,
        )
    } else {
        format!(
            "The {} badge counts {} distinct durable object(s) pending attention; the count derives \
             from durable items and reopens the underlying activity row.",
            scope.as_str(),
            count,
        )
    }
}

/// Projects each scope aggregate onto every governed badge-bearing surface, so the shell,
/// dock/taskbar, companions, and operator dashboard show the same count.
///
/// Deterministic: the count and count class are copied from the aggregate unchanged; only the
/// redaction posture is per-surface, and it is never weaker than the aggregate floor.
pub fn surface_badges(aggregates: &[BadgeAggregate]) -> Vec<SurfaceBadge> {
    let mut out = Vec::new();
    for surface in GOVERNED_BADGE_SURFACES {
        let base = surface_base_redaction(surface);
        for aggregate in aggregates {
            let applied = base.max(aggregate.privacy_floor);
            out.push(SurfaceBadge {
                surface,
                surface_id: surface.channel_id(),
                scope: aggregate.scope,
                scope_key: aggregate.scope_key.clone(),
                count: aggregate.count,
                count_class: aggregate.count_class,
                count_class_token: aggregate.count_class_token.clone(),
                count_display: aggregate.count_display.clone(),
                applied_redaction: applied,
                applied_redaction_token: redaction_token(applied).to_owned(),
                reopen_target: aggregate.reopen_target,
                reopen_anchor_ref: aggregate.reopen_anchor_ref.clone(),
                note: format!(
                    "The {} shows the {} badge count {} (class {}); the number matches the deduped \
                     aggregate and reopens the same authoritative object.",
                    surface.label(),
                    aggregate.scope_key,
                    aggregate.count_display,
                    aggregate.count_class.as_str(),
                ),
            });
        }
    }
    out
}

/// Coalesces repeated failures sharing one root cause into one durable object per cause, in
/// canonical scope then first-seen order.
///
/// Deterministic: the same corpus yields the same coalesced failures every call. Each object
/// carries the occurrence count, the representative authoritative reopen path, and the
/// surfaces the collapse protects from repeated-alert spam.
pub fn coalesce_failures(items: &[DurableAttentionItem]) -> Vec<CoalescedFailure> {
    let mut groups: Vec<(AttentionScopeClass, String, Vec<&DurableAttentionItem>)> = Vec::new();
    for scope in ALL_SCOPES {
        for item in items
            .iter()
            .filter(|i| i.scope == scope && i.is_coalescible_failure())
        {
            if let Some((_, _, members)) = groups
                .iter_mut()
                .find(|(s, key, _)| *s == scope && *key == item.root_cause_key)
            {
                members.push(item);
            } else {
                groups.push((scope, item.root_cause_key.clone(), vec![item]));
            }
        }
    }

    groups
        .into_iter()
        .map(|(scope, root_cause_key, members)| {
            let representative = members[0];
            let occurrence_count = members.len();
            let severity = members
                .iter()
                .map(|m| m.severity)
                .max()
                .unwrap_or(representative.severity);
            CoalescedFailure {
                coalesced_id: format!(
                    "m5-badge-aggregates:coalesced:{}:{}",
                    scope.as_str(),
                    root_cause_key
                ),
                root_cause_key,
                label: representative.label.clone(),
                scope,
                scope_key: scope.as_str().to_owned(),
                occurrence_count,
                representative_item_id: representative.item_id.clone(),
                canonical_event_id: representative.canonical_event_id.clone(),
                dedupe_key_scheme: DedupeStrategyClass::RootCauseCollapse,
                dedupe_key_scheme_token: DedupeStrategyClass::RootCauseCollapse.as_str().to_owned(),
                severity,
                severity_token: severity.as_str().to_owned(),
                state: AttentionStateClass::Failed,
                state_token: AttentionStateClass::Failed.as_str().to_owned(),
                reopen_target: representative.reopen_target,
                reopen_anchor_ref: representative.reopen_anchor_ref.clone(),
                coalesced_surfaces: SPAM_PRONE_SURFACES.to_vec(),
                spam_prevented: occurrence_count > 1,
                durable_record_present: true,
                note: format!(
                    "{} failure(s) from root cause `{}` coalesce into one durable object; instead of \
                     spamming {} surface(s) with {} alerts, the count rises once and reopens the \
                     authoritative {}.",
                    occurrence_count,
                    representative.label,
                    SPAM_PRONE_SURFACES.len(),
                    occurrence_count,
                    representative.reopen_target.as_str(),
                ),
            }
        })
        .collect()
}

/// Rolls the corpus up into the badge telemetry packet: stable notification class / route /
/// outcome enums plus counts, with no message text.
pub fn badge_telemetry(
    items: &[DurableAttentionItem],
    aggregates: &[BadgeAggregate],
    coalesced_failures: &[CoalescedFailure],
) -> BadgeTelemetry {
    // Classify every item's outcome, deduping counted items by scope+key.
    let mut seen_keys: Vec<(AttentionScopeClass, String)> = Vec::new();
    let mut rows: Vec<((SourceSubsystemClass, NotificationOutcomeClass), usize)> = Vec::new();
    let mut bump = |class: SourceSubsystemClass, outcome: NotificationOutcomeClass| {
        if let Some((_, n)) = rows
            .iter_mut()
            .find(|((c, o), _)| *c == class && *o == outcome)
        {
            *n += 1;
        } else {
            rows.push(((class, outcome), 1));
        }
    };

    let mut total_counted = 0usize;
    let mut total_deduped_repeats = 0usize;
    let mut total_muted = 0usize;
    let mut total_suppressed = 0usize;
    let mut total_deferred = 0usize;
    let mut total_settled = 0usize;

    for item in items {
        let outcome = match item.contribution() {
            BadgeContributionClass::Counted => {
                let key = (item.scope, item.dedupe_key.clone());
                if seen_keys.contains(&key) {
                    total_deduped_repeats += 1;
                    NotificationOutcomeClass::DedupedRepeat
                } else {
                    seen_keys.push(key);
                    total_counted += 1;
                    NotificationOutcomeClass::CountedInBadge
                }
            }
            BadgeContributionClass::Muted => {
                total_muted += 1;
                NotificationOutcomeClass::Muted
            }
            BadgeContributionClass::Suppressed => {
                total_suppressed += 1;
                NotificationOutcomeClass::Suppressed
            }
            BadgeContributionClass::QuietHoursDeferred => {
                total_deferred += 1;
                NotificationOutcomeClass::QuietHoursDeferred
            }
            BadgeContributionClass::Settled => {
                total_settled += 1;
                NotificationOutcomeClass::Settled
            }
        };
        bump(item.source_subsystem, outcome);
    }

    // Stable row order: by subsystem token, then outcome token.
    rows.sort_by(|((c1, o1), _), ((c2, o2), _)| {
        c1.as_str()
            .cmp(c2.as_str())
            .then_with(|| o1.as_str().cmp(o2.as_str()))
    });
    let outcome_rollup = rows
        .into_iter()
        .map(|((class, outcome), count)| TelemetryOutcomeRow {
            notification_class: class,
            notification_class_token: class.as_str().to_owned(),
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            count,
        })
        .collect();

    // Every route shows the same per-scope counts, so each route total equals the deduped
    // counted total — parity at the telemetry level.
    let badge_count_total: usize = aggregates.iter().map(|a| a.count).sum();
    let route_rollup = GOVERNED_BADGE_SURFACES
        .iter()
        .map(|surface| {
            let class = CountClass::classify(badge_count_total);
            TelemetryRouteRow {
                route: *surface,
                route_token: surface.channel_id(),
                badge_count_total,
                count_class: class,
                count_class_token: class.as_str().to_owned(),
            }
        })
        .collect();

    let total_failure_occurrences = coalesced_failures.iter().map(|f| f.occurrence_count).sum();

    BadgeTelemetry {
        total_items: items.len(),
        total_counted,
        total_raw_counted: total_counted + total_deduped_repeats,
        total_deduped_repeats,
        total_muted,
        total_suppressed,
        total_deferred,
        total_settled,
        total_coalesced_failures: coalesced_failures.len(),
        total_failure_occurrences,
        outcome_rollup,
        route_rollup,
        captures_message_text: false,
        note:
            "Telemetry records the notification class, route, and outcome by stable token plus a \
               count only; no message body, payload, or secret-bearing field is captured."
                .to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Redaction / privacy helpers (the matrix enums carry no `as_str`).
// ---------------------------------------------------------------------------

/// The token for a redaction class.
fn redaction_token(r: AttentionRedactionClass) -> &'static str {
    use AttentionRedactionClass::*;
    match r {
        MetadataSafeDefault => "metadata_safe_default",
        SummaryOnly => "summary_only",
        RedactedPayload => "redacted_payload",
        CountOnly => "count_only",
        InternalSupportRestricted => "internal_support_restricted",
    }
}

/// The weakest redaction a surface may apply to a badge for this privacy class.
fn privacy_floor(p: NotificationPrivacyClass) -> AttentionRedactionClass {
    use AttentionRedactionClass::*;
    use NotificationPrivacyClass::*;
    match p {
        SummarySafe => SummaryOnly,
        WorkspaceSensitive | SecurityCritical => RedactedPayload,
        ManagedSensitive => CountOnly,
    }
}

/// The base redaction a surface applies before the aggregate floor is layered on.
fn surface_base_redaction(surface: FanoutChannelClass) -> AttentionRedactionClass {
    use AttentionRedactionClass::*;
    use FanoutChannelClass::*;
    match surface {
        // The dock / taskbar badge and OS surfaces show a bare count.
        DockTaskbarBadge | OsNativeNotification => CountOnly,
        // The activity center, companions, and operator dashboard show a count with a scope
        // label and a reopen affordance.
        InAppActivityCenter | BrowserCompanion | MobileCompanion | OperatorDashboard => SummaryOnly,
    }
}

/// Returns the values present in `values`, deduplicated and ordered by `order`. Callers pass
/// only named reasons, so the `None` variant of a vocabulary never appears.
fn distinct_in_order<T: Copy + PartialEq>(order: &[T], values: impl Iterator<Item = T>) -> Vec<T> {
    let present: Vec<T> = values.collect();
    order
        .iter()
        .copied()
        .filter(|candidate| present.iter().any(|v| v == candidate))
        .collect()
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical badge-aggregates bundle.
///
/// Deterministic: the same bytes every call. The governed surfaces and durable-item corpus
/// are fixed, every aggregate, surface badge, coalesced failure, and telemetry row is derived
/// by the engine, and each invariant's `holds` flag is computed from the built data, so an
/// inconsistent edit flips an invariant rather than silently passing.
pub fn badge_aggregates_bundle() -> BadgeAggregatesBundle {
    let governed_surfaces = build_governed_surfaces();
    let items = build_corpus();
    let aggregates = aggregate_badges(&items);
    let surface_badges = surface_badges(&aggregates);
    let coalesced_failures = coalesce_failures(&items);
    let telemetry = badge_telemetry(&items, &aggregates, &coalesced_failures);
    let invariants = compute_invariants(
        &governed_surfaces,
        &items,
        &aggregates,
        &surface_badges,
        &coalesced_failures,
        &telemetry,
    );

    BadgeAggregatesBundle {
        record_kind: M5_BADGE_AGGREGATES_RECORD_KIND.to_owned(),
        m5_badge_aggregates_schema_version: M5_BADGE_AGGREGATES_SCHEMA_VERSION,
        schema_ref: M5_BADGE_AGGREGATES_SCHEMA_REF.to_owned(),
        bundle_id: M5_BADGE_AGGREGATES_BUNDLE_ID.to_owned(),
        as_of: M5_BADGE_AGGREGATES_AS_OF.to_owned(),
        matrix_ref: M5_BADGE_AGGREGATES_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_BADGE_AGGREGATES_FREEZE_GATE_REF.to_owned(),
        summary: "Deduped, per-scope badge aggregates derived from durable items rather than raw \
                  event spam: each carries its scope key, count, count class, freshness, deduped \
                  objects, and the muted, suppressed, and quiet-hours reasons that explain the \
                  number. Repeated failures from one root cause coalesce into one durable object — \
                  counted once — with the correct authoritative reopen path instead of spamming \
                  toasts, badges, and companion alerts. The shell activity center, dock/taskbar \
                  badge, browser and mobile companions, and operator dashboard project the same \
                  count without widening privacy; a security advisory is never silenced; and \
                  telemetry records notification class, route, and outcome by stable enum and count \
                  with no message text."
            .to_owned(),
        governed_surfaces,
        items,
        aggregates,
        surface_badges,
        coalesced_failures,
        telemetry,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_governed_surfaces() -> Vec<GovernedSurfaceEntry> {
    GOVERNED_BADGE_SURFACES
        .iter()
        .map(|surface| {
            let base = surface_base_redaction(*surface);
            let is_authoritative = *surface == FanoutChannelClass::InAppActivityCenter;
            GovernedSurfaceEntry {
                surface: *surface,
                surface_id: surface.channel_id(),
                label: surface.label().to_owned(),
                base_redaction: base,
                base_redaction_token: redaction_token(base).to_owned(),
                is_durable_authoritative: is_authoritative,
                note: if is_authoritative {
                    "The authoritative durable record; its badge counts the deduped pending items \
                     directly."
                        .to_owned()
                } else {
                    format!(
                        "Projects the same deduped aggregate; the {} shows an identical count and \
                         reopens the same authoritative object.",
                        surface.label(),
                    )
                },
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn item(
    slug: &str,
    dedupe_key: &str,
    label: &str,
    source_subsystem: SourceSubsystemClass,
    severity: NotificationSeverityClass,
    scope: AttentionScopeClass,
    privacy_class: NotificationPrivacyClass,
    dedupe_key_scheme: DedupeStrategyClass,
    state: AttentionStateClass,
    freshness: BadgeFreshnessClass,
    mute_reason: BadgeMuteReasonClass,
    suppression_reason: BadgeSuppressionReasonClass,
    quiet_hours_mode: QuietHoursModeClass,
    root_cause_key: &str,
    reopen_target: ReopenTargetClass,
) -> DurableAttentionItem {
    DurableAttentionItem {
        item_id: format!("durable_item:{slug}"),
        canonical_event_id: format!("canonical_event:{slug}"),
        dedupe_key: dedupe_key.to_owned(),
        label: label.to_owned(),
        source_subsystem,
        severity,
        scope,
        privacy_class,
        dedupe_key_scheme,
        state,
        freshness,
        mute_reason,
        suppression_reason,
        quiet_hours_mode,
        root_cause_key: root_cause_key.to_owned(),
        reopen_target,
        reopen_anchor_ref: format!("aureline://object/{slug}"),
        carries_durable_record: true,
    }
}

fn build_corpus() -> Vec<DurableAttentionItem> {
    use AttentionScopeClass as Sc;
    use AttentionStateClass as St;
    use BadgeFreshnessClass as F;
    use BadgeMuteReasonClass as M;
    use BadgeSuppressionReasonClass as Sup;
    use DedupeStrategyClass as D;
    use NotificationPrivacyClass as P;
    use NotificationSeverityClass as Sev;
    use QuietHoursModeClass as Q;
    use ReopenTargetClass as R;
    use SourceSubsystemClass as S;

    // App-global scope: every eligible item is excluded, so the badge count is zero but each
    // exclusion reason is named.
    let mut items = vec![
        item(
            "app.update_muted",
            "app:update",
            "Update available",
            S::Shell,
            Sev::Informational,
            Sc::AppGlobal,
            P::SummarySafe,
            D::LatestSupersedes,
            St::Pending,
            F::Recent,
            M::MutedByFocusMode,
            Sup::None,
            Q::None,
            "",
            R::ActivityJobRow,
        ),
        item(
            "app.sync_suppressed",
            "app:sync",
            "Background sync retry",
            S::Sync,
            Sev::Informational,
            Sc::AppGlobal,
            P::SummarySafe,
            D::CountRollup,
            St::Pending,
            F::Aging,
            M::None,
            Sup::RateLimited,
            Q::None,
            "",
            R::ActivityJobRow,
        ),
        item(
            "app.tip_settled",
            "app:tip",
            "Onboarding tip",
            S::Shell,
            Sev::MinorSuccess,
            Sc::AppGlobal,
            P::SummarySafe,
            D::NoDedupe,
            St::Acknowledged,
            F::Stale,
            M::None,
            Sup::None,
            Q::None,
            "",
            R::ActivityJobRow,
        ),
    ];

    // Session scope: a progress object, a repeated failure coalesced three ways, and an AI
    // handoff — three distinct counted objects from five raw events.
    items.push(item(
        "session.build_progress",
        "session:build:42",
        "Build running",
        S::TaskRunner,
        Sev::Progress,
        Sc::Session,
        P::SummarySafe,
        D::LatestSupersedes,
        St::Running,
        F::Fresh,
        M::None,
        Sup::None,
        Q::None,
        "",
        R::ActivityJobRow,
    ));
    for n in 1..=3 {
        items.push(item(
            &format!("session.save_failed.{n}"),
            "session:save_conflict:doc7",
            "Save failed — conflict",
            S::TaskRunner,
            Sev::Degraded,
            Sc::Session,
            P::SummarySafe,
            D::RootCauseCollapse,
            St::Failed,
            F::Fresh,
            M::None,
            Sup::None,
            Q::None,
            // The root-cause key equals the dedupe key, so the coalesced failure object and the
            // badge's deduped object are the same durable object.
            "session:save_conflict:doc7",
            R::ActivityJobRow,
        ));
    }
    items.push(item(
        "session.ai_await",
        "session:ai:review:9",
        "AI change awaiting review",
        S::Ai,
        Sev::HandoffActionable,
        Sc::Session,
        P::WorkspaceSensitive,
        D::CanonicalKeyCoalesce,
        St::Pending,
        F::Fresh,
        M::None,
        Sup::None,
        Q::None,
        "",
        R::ReviewRequest,
    ));

    // Workspace scope: ten distinct counted objects (a "many" badge) plus one repeat, so the
    // count is ten from eleven raw events.
    for n in 0..10 {
        items.push(item(
            &format!("workspace.diag.{n}"),
            &format!("workspace:diag:{n}"),
            "Diagnostic pending",
            S::TaskRunner,
            Sev::Informational,
            Sc::Workspace,
            P::WorkspaceSensitive,
            D::CanonicalKeyCoalesce,
            St::Pending,
            F::Recent,
            M::None,
            Sup::None,
            Q::None,
            "",
            R::ActivityJobRow,
        ));
    }
    items.push(item(
        "workspace.diag.0.repeat",
        "workspace:diag:0",
        "Diagnostic pending",
        S::TaskRunner,
        Sev::Informational,
        Sc::Workspace,
        P::WorkspaceSensitive,
        D::CanonicalKeyCoalesce,
        St::Pending,
        F::Fresh,
        M::None,
        Sup::None,
        Q::None,
        "",
        R::ActivityJobRow,
    ));

    // Collaboration scope: a review request, a muted chatter item, and a security advisory the
    // user tried to mute but which is never silenced.
    items.push(item(
        "collab.review",
        "collab:review:thread:3",
        "Review requested",
        S::Collaboration,
        Sev::HandoffActionable,
        Sc::Collaboration,
        P::WorkspaceSensitive,
        D::LatestSupersedes,
        St::Pending,
        F::Recent,
        M::None,
        Sup::None,
        Q::None,
        "",
        R::ReviewRequest,
    ));
    items.push(item(
        "collab.chatter_muted",
        "collab:chatter:7",
        "Comment activity",
        S::Collaboration,
        Sev::Informational,
        Sc::Collaboration,
        P::WorkspaceSensitive,
        D::CountRollup,
        St::Pending,
        F::Aging,
        M::UserMutedSource,
        Sup::None,
        Q::None,
        "",
        R::ReviewRequest,
    ));
    items.push(item(
        "collab.security_advisory",
        "collab:security:token:1",
        "Credential exposure advisory",
        S::Security,
        Sev::SecurityAdvisory,
        Sc::Collaboration,
        P::SecurityCritical,
        D::CountRollup,
        St::Pending,
        F::Fresh,
        // The user muted the source and quiet hours is active, but a security advisory is
        // never silenced out of the badge.
        M::UserMutedSource,
        Sup::None,
        Q::AlwaysDefer,
        "",
        R::AuditEvent,
    ));

    // Tenant / org scope: one counted managed alert, one policy-suppressed item, and one
    // quiet-hours-deferred route warning.
    items.push(item(
        "tenant.managed_alert",
        "tenant:managed:alert:2",
        "Managed policy alert",
        S::ManagedPolicy,
        Sev::Degraded,
        Sc::TenantOrg,
        P::ManagedSensitive,
        D::LatestSupersedes,
        St::Pending,
        F::Recent,
        M::None,
        Sup::None,
        Q::None,
        "",
        R::PolicyDiff,
    ));
    items.push(item(
        "tenant.policy_suppressed",
        "tenant:policy:update:5",
        "Policy update",
        S::ManagedPolicy,
        Sev::HandoffActionable,
        Sc::TenantOrg,
        P::ManagedSensitive,
        D::LatestSupersedes,
        St::Pending,
        F::Aging,
        M::None,
        Sup::PolicySuppressed,
        Q::None,
        "",
        R::PolicyDiff,
    ));
    items.push(item(
        "tenant.route_deferred",
        "tenant:route:warn:8",
        "Managed route warning",
        S::ManagedPolicy,
        Sev::HandoffActionable,
        Sc::TenantOrg,
        P::ManagedSensitive,
        D::LatestSupersedes,
        St::Pending,
        F::Recent,
        M::None,
        Sup::None,
        Q::FollowAdminPolicy,
        "",
        R::RouteObject,
    ));

    items
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> BadgeAggregatesInvariant {
    BadgeAggregatesInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    governed_surfaces: &[GovernedSurfaceEntry],
    items: &[DurableAttentionItem],
    aggregates: &[BadgeAggregate],
    surface_badges: &[SurfaceBadge],
    coalesced_failures: &[CoalescedFailure],
    telemetry: &BadgeTelemetry,
) -> Vec<BadgeAggregatesInvariant> {
    let matrix = attention_routing_matrix();
    let mut out = Vec::new();

    // Counts derive from deduped durable items, not raw events.
    out.push(invariant(
        "badge.counts_deduped_durable_items",
        "Every aggregate's count equals the number of distinct deduped objects, equals its \
         deduped_count, is never greater than its raw event count, and derives from durable items; \
         at least one scope shows dedupe collapsing repeats (raw greater than count).",
        aggregates.iter().all(|a| {
            a.count == a.deduped_objects.len()
                && a.count == a.deduped_count
                && a.raw_event_count >= a.count
                && a.derives_from_durable_items
                && all_unique(a.deduped_objects.iter().map(|o| o.object_key.as_str()))
        }) && aggregates.iter().any(|a| a.raw_event_count > a.count),
    ));

    // Count class matches count everywhere.
    out.push(invariant(
        "badge.count_class_matches_count",
        "Every aggregate and surface badge classifies its count into the correct count class and \
         renders the matching display string, so the coarse class can never disagree with the \
         number.",
        aggregates.iter().all(|a| {
            a.count_class == CountClass::classify(a.count)
                && a.count_class_token == a.count_class.as_str()
                && a.count_display == count_display(a.count)
        }) && surface_badges.iter().all(|b| {
            b.count_class == CountClass::classify(b.count)
                && b.count_display == count_display(b.count)
        }),
    ));

    // Excluded items are named, not counted.
    out.push(invariant(
        "badge.excluded_reasons_named",
        "In every scope, each muted, suppressed, or quiet-hours-deferred item carries a named \
         reason that appears in the aggregate's muted, suppressed, or active-quiet-hours lists, and \
         a counted item carries none of those exclusion reasons.",
        aggregates.iter().all(|a| {
            let scoped: Vec<&DurableAttentionItem> =
                items.iter().filter(|i| i.scope == a.scope).collect();
            scoped.iter().all(|i| match i.contribution() {
                BadgeContributionClass::Muted => {
                    i.mute_reason.is_named() && a.muted_reasons.contains(&i.mute_reason)
                }
                BadgeContributionClass::Suppressed => {
                    i.suppression_reason.is_named()
                        && a.suppressed_reasons.contains(&i.suppression_reason)
                }
                BadgeContributionClass::QuietHoursDeferred => {
                    i.quiet_hours_mode.is_deferring()
                        && a.active_quiet_hours_modes.contains(&i.quiet_hours_mode)
                }
                BadgeContributionClass::Counted => {
                    !i.mute_reason.is_named()
                        || i.severity.is_security() // a security advisory may carry a mute signal yet still count
                }
                BadgeContributionClass::Settled => true,
            })
        }),
    ));

    // Muted / suppressed / deferred / settled items never enter the count.
    out.push(invariant(
        "badge.muted_suppressed_not_counted",
        "No muted, suppressed, deferred, or settled item appears among an aggregate's deduped \
         objects, so an excluded item never inflates the badge.",
        aggregates.iter().all(|a| {
            let counted_keys: Vec<&str> = items
                .iter()
                .filter(|i| i.scope == a.scope && i.contribution().counts())
                .map(|i| i.dedupe_key.as_str())
                .collect();
            a.deduped_objects
                .iter()
                .all(|o| counted_keys.contains(&o.object_key.as_str()))
        }),
    ));

    // Cross-surface parity: every governed surface shows the same count per scope.
    out.push(invariant(
        "badge.cross_surface_parity",
        "For every scope, every governed surface — the shell activity center, dock/taskbar badge, \
         browser and mobile companions, and operator dashboard — projects a badge whose count and \
         count class equal the deduped aggregate, so no surface drifts from durable truth.",
        GOVERNED_BADGE_SURFACES.iter().all(|surface| {
            aggregates.iter().all(|a| {
                surface_badges
                    .iter()
                    .find(|b| b.surface == *surface && b.scope == a.scope)
                    .is_some_and(|b| {
                        b.count == a.count
                            && b.count_class == a.count_class
                            && b.count_display == a.count_display
                            && b.reopen_target == a.reopen_target
                            && b.reopen_anchor_ref == a.reopen_anchor_ref
                    })
            })
        }) && surface_badges.len() == GOVERNED_BADGE_SURFACES.len() * aggregates.len(),
    ));

    // No surface widens privacy below the aggregate floor.
    out.push(invariant(
        "badge.surface_never_widens_privacy",
        "Every surface badge applies a redaction at least as strong as its aggregate's privacy \
         floor, and the dock/taskbar badge is always count-only, so a badge projection never widens \
         privacy.",
        surface_badges.iter().all(|b| {
            let floor = aggregates
                .iter()
                .find(|a| a.scope == b.scope)
                .map(|a| a.privacy_floor)
                .unwrap_or(AttentionRedactionClass::SummaryOnly);
            b.applied_redaction >= floor
                && (b.surface != FanoutChannelClass::DockTaskbarBadge
                    || b.applied_redaction == AttentionRedactionClass::CountOnly)
        }),
    ));

    // Every badge reopens an authoritative object.
    out.push(invariant(
        "badge.route_to_authoritative",
        "Every aggregate and surface badge reopens via a badge reopen target (an activity row or \
         route object) on a non-empty, export-safe anchor; a single-object badge anchors the exact \
         object, and a multi-object badge anchors the scope's pending list.",
        aggregates.iter().all(|a| {
            let target_ok = matches!(
                a.reopen_target,
                ReopenTargetClass::ActivityJobRow | ReopenTargetClass::RouteObject
            );
            let anchor_ok =
                !a.reopen_anchor_ref.is_empty() && is_export_safe_ref(&a.reopen_anchor_ref);
            let exactness_ok = if a.count == 1 {
                a.reopen_anchor_ref == a.deduped_objects[0].reopen_anchor_ref
            } else {
                a.reopen_anchor_ref == format!("aureline://activity/{}/pending", a.scope.as_str())
            };
            target_ok && anchor_ok && exactness_ok
        }) && surface_badges
            .iter()
            .all(|b| !b.reopen_anchor_ref.is_empty() && is_export_safe_ref(&b.reopen_anchor_ref)),
    ));

    // Repeated failures coalesce into one object, counted once.
    out.push(invariant(
        "badge.repeated_failures_coalesce",
        "Every coalesced failure groups all raw failures of one root cause in its scope into one \
         object whose occurrence count equals that raw count and is counted exactly once in the \
         scope badge; at least one coalesced failure collapses more than one occurrence.",
        coalesced_failures.iter().all(|f| {
            let raw = items
                .iter()
                .filter(|i| {
                    i.scope == f.scope
                        && i.is_coalescible_failure()
                        && i.root_cause_key == f.root_cause_key
                })
                .count();
            let counted_once = aggregates
                .iter()
                .find(|a| a.scope == f.scope)
                .map(|a| {
                    a.deduped_objects
                        .iter()
                        .filter(|o| o.object_key == f.root_cause_key)
                        .count()
                })
                .unwrap_or(0)
                == 1;
            f.occurrence_count == raw && counted_once && f.durable_record_present
        }) && coalesced_failures
            .iter()
            .any(|f| f.spam_prevented && f.occurrence_count > 1),
    ));

    // Coalesced failures reopen the authoritative object.
    out.push(invariant(
        "badge.coalesced_failure_reopen_authoritative",
        "Every coalesced failure reopens the representative item's exact authoritative object on an \
         export-safe anchor and names the surfaces the collapse protects from spam.",
        coalesced_failures.iter().all(|f| {
            items
                .iter()
                .find(|i| i.item_id == f.representative_item_id)
                .is_some_and(|i| {
                    f.reopen_target == i.reopen_target
                        && f.reopen_anchor_ref == i.reopen_anchor_ref
                        && is_export_safe_ref(&f.reopen_anchor_ref)
                })
                && !f.coalesced_surfaces.is_empty()
        }),
    ));

    // A security advisory is never silenced.
    out.push(invariant(
        "badge.security_never_silenced",
        "Every active-state security advisory is counted regardless of any mute, suppression, or \
         quiet-hours signal, so a security advisory is never silenced out of the badge.",
        items
            .iter()
            .filter(|i| i.severity.is_security() && state_is_active(i.state))
            .all(|i| i.contribution() == BadgeContributionClass::Counted),
    ));

    // Telemetry is stable enums and counts only, internally consistent.
    out.push(invariant(
        "badge.telemetry_stable_enums_no_text",
        "Telemetry captures no message text, its outcome rows sum to the total item count, its \
         counted/deduped/excluded totals reconcile, and every route total equals the deduped \
         counted total — so support and telemetry record class, route, and outcome by stable enum \
         and count only.",
        !telemetry.captures_message_text
            && telemetry
                .outcome_rollup
                .iter()
                .map(|r| r.count)
                .sum::<usize>()
                == telemetry.total_items
            && telemetry.total_raw_counted
                == telemetry.total_counted + telemetry.total_deduped_repeats
            && telemetry.total_items
                == telemetry.total_raw_counted
                    + telemetry.total_muted
                    + telemetry.total_suppressed
                    + telemetry.total_deferred
                    + telemetry.total_settled
            && telemetry.total_counted == aggregates.iter().map(|a| a.count).sum::<usize>()
            && telemetry
                .route_rollup
                .iter()
                .all(|r| r.badge_count_total == telemetry.total_counted)
            && telemetry.total_coalesced_failures == coalesced_failures.len(),
    ));

    // Every token binds back to the attention-routing matrix.
    out.push(invariant(
        "badge.matrix_bound",
        "Every scope, privacy class, redaction class, reopen target, severity, dedupe scheme, \
         suppression reason, and quiet-hours mode the bundle uses is one the attention-routing \
         matrix defines, and the badge-aggregate object can show the badge states, so the badge \
         path never drifts from the frozen object model.",
        matrix_bound_holds(items, aggregates, surface_badges, &matrix),
    ));

    // Everything reproduces from the corpus.
    out.push(invariant(
        "badge.deterministic_reproducible",
        "Re-running the aggregation, surface projection, coalescing, and telemetry over the corpus \
         yields identical results, so a badge count is reproducible in support export and \
         diagnostics.",
        aggregate_badges(items) == aggregates
            && self::surface_badges(aggregates) == *surface_badges
            && coalesce_failures(items) == *coalesced_failures
            && badge_telemetry(items, aggregates, coalesced_failures) == *telemetry,
    ));

    // The governed surfaces are exactly the five badge surfaces and every ref is export-safe.
    out.push(invariant(
        "badge.support_export_safe",
        "The governed surfaces are exactly the five badge-bearing surfaces, and every item, \
         aggregate, surface badge, and coalesced-failure anchor is a repo-relative object ref or \
         opaque aureline:// handle — never a URL, host, credential, message body, or absolute path.",
        governed_surfaces.len() == GOVERNED_BADGE_SURFACES.len()
            && GOVERNED_BADGE_SURFACES
                .iter()
                .all(|s| governed_surfaces.iter().any(|e| e.surface == *s))
            && items.iter().all(|i| is_export_safe_ref(&i.reopen_anchor_ref))
            && aggregates.iter().all(|a| {
                is_export_safe_ref(&a.reopen_anchor_ref)
                    && a.deduped_objects
                        .iter()
                        .all(|o| is_export_safe_ref(&o.reopen_anchor_ref))
            })
            && surface_badges
                .iter()
                .all(|b| is_export_safe_ref(&b.reopen_anchor_ref))
            && coalesced_failures
                .iter()
                .all(|f| is_export_safe_ref(&f.reopen_anchor_ref)),
    ));

    out
}

fn matrix_bound_holds(
    items: &[DurableAttentionItem],
    aggregates: &[BadgeAggregate],
    surface_badges: &[SurfaceBadge],
    matrix: &AttentionRoutingMatrix,
) -> bool {
    let tokens = |defs: &[crate::m5_attention_routing::AttentionTokenDef]| -> Vec<String> {
        defs.iter().map(|t| t.token.clone()).collect()
    };
    let scope_tokens = tokens(&matrix.shared_vocabulary.scopes);
    let privacy_tokens = tokens(&matrix.shared_vocabulary.privacy_classes);
    let redaction_tokens = tokens(&matrix.shared_vocabulary.redaction_classes);
    let reopen_tokens = tokens(&matrix.shared_vocabulary.reopen_targets);
    let severity_tokens = tokens(&matrix.shared_vocabulary.severities);
    let dedupe_tokens = tokens(&matrix.shared_vocabulary.dedupe_rules);
    let suppression_tokens = tokens(&matrix.shared_vocabulary.suppression_reasons);
    let quiet_tokens = tokens(&matrix.shared_vocabulary.quiet_hours_behaviors);
    let has = |list: &[String], token: &str| list.iter().any(|t| t == token);

    let items_bound = items.iter().all(|i| {
        has(&scope_tokens, i.scope.as_str())
            && has(&privacy_tokens, i.privacy_class.as_str())
            && has(&reopen_tokens, i.reopen_target.as_str())
            && has(&severity_tokens, i.severity.as_str())
            && has(&dedupe_tokens, i.dedupe_key_scheme.as_str())
            && (!i.suppression_reason.is_named()
                || has(&suppression_tokens, i.suppression_reason.as_str()))
            && (!i.quiet_hours_mode.is_deferring()
                || has(&quiet_tokens, i.quiet_hours_mode.as_str()))
    });

    let aggregates_bound = aggregates.iter().all(|a| {
        has(&scope_tokens, a.scope_key.as_str())
            && has(&redaction_tokens, redaction_token(a.privacy_floor))
            && has(&reopen_tokens, a.reopen_target.as_str())
            && a.suppressed_reasons
                .iter()
                .all(|r| has(&suppression_tokens, r.as_str()))
            && a.active_quiet_hours_modes
                .iter()
                .all(|q| has(&quiet_tokens, q.as_str()))
    });

    let surfaces_bound = surface_badges
        .iter()
        .all(|b| has(&redaction_tokens, redaction_token(b.applied_redaction)));

    let badge_object = matrix.object(AttentionObjectClass::BadgeAggregate);
    let states_bound = badge_object.is_some_and(|o| {
        o.can_show(AttentionStateClass::Shown)
            && o.can_show(AttentionStateClass::Acknowledged)
            && o.can_show(AttentionStateClass::Dismissed)
            && o.can_show(AttentionStateClass::Suppressed)
            && o.can_show(AttentionStateClass::QuietHoursDeferred)
            && o.can_show(AttentionStateClass::UnknownRequiresReview)
    });

    items_bound && aggregates_bound && surfaces_bound && states_bound
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn badge_aggregates_lines(bundle: &BadgeAggregatesBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Badge-aggregates bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Surfaces: {}  Items: {}  Aggregates: {}  Surface badges: {}  Coalesced failures: {}  \
         Invariants: {}",
        bundle.governed_surfaces.len(),
        bundle.items.len(),
        bundle.aggregates.len(),
        bundle.surface_badges.len(),
        bundle.coalesced_failures.len(),
        bundle.invariants.len(),
    ));

    lines.push("Aggregates:".to_owned());
    for a in &bundle.aggregates {
        lines.push(format!(
            "  - {} count={} ({}) raw={} freshness={} floor={} reopen={}",
            a.scope_key,
            a.count_display,
            a.count_class.as_str(),
            a.raw_event_count,
            a.freshness.as_str(),
            redaction_token(a.privacy_floor),
            a.reopen_anchor_ref,
        ));
        if !a.muted_reasons.is_empty()
            || !a.suppressed_reasons.is_empty()
            || !a.active_quiet_hours_modes.is_empty()
        {
            let muted: Vec<&str> = a.muted_reasons.iter().map(|r| r.as_str()).collect();
            let supp: Vec<&str> = a.suppressed_reasons.iter().map(|r| r.as_str()).collect();
            let quiet: Vec<&str> = a
                .active_quiet_hours_modes
                .iter()
                .map(|q| q.as_str())
                .collect();
            lines.push(format!(
                "      muted=[{}] suppressed=[{}] quiet_hours=[{}]",
                muted.join(", "),
                supp.join(", "),
                quiet.join(", "),
            ));
        }
    }

    lines.push("Coalesced failures:".to_owned());
    for f in &bundle.coalesced_failures {
        lines.push(format!(
            "  - {} [{}] occurrences={} reopen={} spam_prevented={}",
            f.label, f.root_cause_key, f.occurrence_count, f.reopen_anchor_ref, f.spam_prevented,
        ));
    }

    lines.push("Telemetry:".to_owned());
    lines.push(format!(
        "  items={} counted={} raw_counted={} deduped_repeats={} muted={} suppressed={} \
         deferred={} settled={} coalesced_failures={} failure_occurrences={}",
        bundle.telemetry.total_items,
        bundle.telemetry.total_counted,
        bundle.telemetry.total_raw_counted,
        bundle.telemetry.total_deduped_repeats,
        bundle.telemetry.total_muted,
        bundle.telemetry.total_suppressed,
        bundle.telemetry.total_deferred,
        bundle.telemetry.total_settled,
        bundle.telemetry.total_coalesced_failures,
        bundle.telemetry.total_failure_occurrences,
    ));
    for r in &bundle.telemetry.outcome_rollup {
        lines.push(format!(
            "    {} / {} = {}",
            r.notification_class_token, r.outcome_token, r.count
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
