//! Notification action semantics and badge reconciliation.
//!
//! This module keeps notification-adjacent verbs distinct after routing.
//! Transient dismissal, acknowledgement, snooze, mute, resolve, and system
//! suppression all preserve the same canonical event identity, but they have
//! different badge and retention effects. Badge projections read these typed
//! states instead of deriving counts from raw delivery history.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Skip-serialize predicate for boolean fields that default to false, so
/// optional flags do not appear in fixtures unless they are actually set.
fn is_false(value: &bool) -> bool {
    !*value
}

/// Schema version for notification action-state records.
pub const NOTIFICATION_ACTION_STATE_SCHEMA_VERSION: u32 = 1;
/// Stable record kind for one notification attention state.
pub const NOTIFICATION_ATTENTION_STATE_RECORD_KIND: &str = "notification_attention_state_record";
/// Stable record kind for one badge reconciliation snapshot.
pub const NOTIFICATION_BADGE_RECONCILIATION_RECORD_KIND: &str =
    "notification_badge_reconciliation_record";

/// Badge class whose count is reconciled from durable notification state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeClass {
    /// Items that need review.
    NeedsReview,
    /// Failed runs or retries that remain inspectable.
    FailedRuns,
    /// Mentions or direct collaboration requests.
    Mentions,
    /// Security notices.
    SecurityNotices,
    /// Session join, control, or handoff requests.
    SessionRequests,
    /// Offline publish work pending drain.
    OfflinePublishPending,
    /// Durable jobs that are queued or running.
    DurableRunningCount,
    /// Items held by quiet hours, snooze, mute, or policy suppression.
    HeldOrSuppressedCount,
    /// Completed work that is unread.
    CompletionUnread,
}

impl BadgeClass {
    /// Stable token recorded in fixtures, exports, and badge projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeedsReview => "needs_review",
            Self::FailedRuns => "failed_runs",
            Self::Mentions => "mentions",
            Self::SecurityNotices => "security_notices",
            Self::SessionRequests => "session_requests",
            Self::OfflinePublishPending => "offline_publish_pending",
            Self::DurableRunningCount => "durable_running_count",
            Self::HeldOrSuppressedCount => "held_or_suppressed_count",
            Self::CompletionUnread => "completion_unread",
        }
    }
}

/// User or system action applied to a notification item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationLifecycleActionKind {
    /// Closes transient chrome only; durable state remains active.
    Dismiss,
    /// Clears attention and badge count without mutating the source object.
    Acknowledge,
    /// Defers re-interruption until a named resume condition.
    Snooze,
    /// Silences future deliveries of the named class or source.
    Mute,
    /// Removes the item from the active attention list and badge while
    /// keeping it reachable in durable history. Unlike acknowledge it leaves
    /// the active inbox view; unlike resolve it never mutates the source.
    Clear,
    /// Marks the underlying object resolved through its owning model.
    Resolve,
    /// Records a system-side hold such as quiet hours or admin policy.
    Suppress,
}

impl NotificationLifecycleActionKind {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dismiss => "dismiss",
            Self::Acknowledge => "acknowledge",
            Self::Snooze => "snooze",
            Self::Mute => "mute",
            Self::Clear => "clear",
            Self::Resolve => "resolve",
            Self::Suppress => "suppress",
        }
    }
}

/// One request to update a notification attention state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationActionRequest {
    /// Stable action id from the invoking surface.
    pub action_id: String,
    /// Action kind.
    pub action_kind: NotificationLifecycleActionKind,
    /// Canonical event being acted on.
    pub canonical_event_id: String,
    /// Badge class affected by this action.
    pub badge_class: BadgeClass,
    /// Target identity affected by the action.
    pub target_identity_ref: String,
    /// Required resume condition for snooze.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_condition_label: Option<String>,
    /// Required class ref for mute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_class_ref: Option<String>,
    /// Action time.
    pub acted_at: String,
}

impl NotificationActionRequest {
    /// Builds a request with the required stable identity fields.
    pub fn new(
        action_id: impl Into<String>,
        action_kind: NotificationLifecycleActionKind,
        canonical_event_id: impl Into<String>,
        badge_class: BadgeClass,
        target_identity_ref: impl Into<String>,
        acted_at: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            action_kind,
            canonical_event_id: canonical_event_id.into(),
            badge_class,
            target_identity_ref: target_identity_ref.into(),
            resume_condition_label: None,
            muted_class_ref: None,
            acted_at: acted_at.into(),
        }
    }

    /// Adds a snooze resume condition.
    pub fn with_resume_condition(mut self, label: impl Into<String>) -> Self {
        self.resume_condition_label = Some(label.into());
        self
    }

    /// Adds the class muted by a mute action.
    pub fn with_muted_class(mut self, muted_class_ref: impl Into<String>) -> Self {
        self.muted_class_ref = Some(muted_class_ref.into());
        self
    }
}

/// Durable attention state after one or more notification actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationAttentionState {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Canonical event id; never changes across action transitions.
    pub canonical_event_id: String,
    /// Badge class this state contributes to.
    pub badge_class: BadgeClass,
    /// True when a transient delivery is still visible.
    pub transient_delivery_visible: bool,
    /// True when the durable row/history remains reachable.
    pub durable_item_retained: bool,
    /// True when this item contributes to the active badge count.
    pub active_badge_counted: bool,
    /// True when this item contributes to held/suppressed count.
    pub held_or_suppressed_counted: bool,
    /// True after acknowledge clears active attention.
    pub acknowledged: bool,
    /// True after clear removes the item from the active attention list while
    /// keeping it in durable history. Skipped when false so existing
    /// attention-state fixtures stay byte-identical.
    #[serde(default, skip_serializing_if = "is_false")]
    pub cleared_from_active_view: bool,
    /// True while the item is snoozed.
    pub snoozed: bool,
    /// True while future deliveries for the class/source are muted.
    pub muted: bool,
    /// True after the owning source object was explicitly resolved.
    pub resolved: bool,
    /// True only when the source object changed through its owner.
    pub source_object_mutated: bool,
    /// Snooze resume condition when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_condition_label: Option<String>,
    /// Muted class when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_class_ref: Option<String>,
    /// Last action applied to this state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_action_kind: Option<NotificationLifecycleActionKind>,
    /// Time of the last action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_action_at: Option<String>,
}

impl NotificationAttentionState {
    /// Creates an active state for a newly durable notification item.
    pub fn active(canonical_event_id: impl Into<String>, badge_class: BadgeClass) -> Self {
        Self {
            record_kind: NOTIFICATION_ATTENTION_STATE_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_ACTION_STATE_SCHEMA_VERSION,
            canonical_event_id: canonical_event_id.into(),
            badge_class,
            transient_delivery_visible: true,
            durable_item_retained: true,
            active_badge_counted: true,
            held_or_suppressed_counted: false,
            acknowledged: false,
            cleared_from_active_view: false,
            snoozed: false,
            muted: false,
            resolved: false,
            source_object_mutated: false,
            resume_condition_label: None,
            muted_class_ref: None,
            last_action_kind: None,
            last_action_at: None,
        }
    }

    /// Applies an action request while preserving canonical identity.
    pub fn apply(&mut self, request: &NotificationActionRequest) {
        if self.canonical_event_id != request.canonical_event_id {
            return;
        }
        self.badge_class = request.badge_class;
        self.last_action_kind = Some(request.action_kind);
        self.last_action_at = Some(request.acted_at.clone());
        match request.action_kind {
            NotificationLifecycleActionKind::Dismiss => {
                self.transient_delivery_visible = false;
            }
            NotificationLifecycleActionKind::Acknowledge => {
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.acknowledged = true;
            }
            NotificationLifecycleActionKind::Snooze => {
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.held_or_suppressed_counted = true;
                self.snoozed = true;
                self.resume_condition_label = request.resume_condition_label.clone();
            }
            NotificationLifecycleActionKind::Mute => {
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.muted = true;
                self.muted_class_ref = request
                    .muted_class_ref
                    .clone()
                    .or_else(|| Some(request.badge_class.as_str().to_owned()));
            }
            NotificationLifecycleActionKind::Clear => {
                // Clear empties the active list view and badge but keeps the
                // item in durable history. It never moves the item into the
                // held/suppressed count and never mutates the source object.
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.held_or_suppressed_counted = false;
                self.cleared_from_active_view = true;
            }
            NotificationLifecycleActionKind::Resolve => {
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.held_or_suppressed_counted = false;
                self.resolved = true;
                self.source_object_mutated = true;
            }
            NotificationLifecycleActionKind::Suppress => {
                self.transient_delivery_visible = false;
                self.active_badge_counted = false;
                self.held_or_suppressed_counted = true;
            }
        }
    }
}

/// Reconciled count for one badge class after action transitions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationBadgeReconciliation {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Badge class being reconciled.
    pub badge_class: BadgeClass,
    /// Active count after deduping by canonical event id.
    pub active_count: u32,
    /// Held or suppressed count after deduping by canonical event id.
    pub held_or_suppressed_count: u32,
    /// Acknowledged item count.
    pub acknowledged_count: u32,
    /// Snoozed item count.
    pub snoozed_count: u32,
    /// Muted item count.
    pub muted_count: u32,
    /// Resolved item count.
    pub resolved_count: u32,
    /// Durable retained item count.
    pub durable_retained_count: u32,
    /// True when every counted state kept durable history.
    pub durable_history_preserved: bool,
    /// Privacy-safe label for compact badge and OS summary surfaces.
    pub privacy_safe_summary_label: String,
}

impl NotificationBadgeReconciliation {
    /// Builds a badge reconciliation from durable attention states.
    pub fn for_badge_class(states: &[NotificationAttentionState], badge_class: BadgeClass) -> Self {
        let mut by_event: BTreeMap<&str, &NotificationAttentionState> = BTreeMap::new();
        for state in states
            .iter()
            .filter(|state| state.badge_class == badge_class)
        {
            by_event.insert(state.canonical_event_id.as_str(), state);
        }

        let mut active_count = 0u32;
        let mut held_or_suppressed_count = 0u32;
        let mut acknowledged_count = 0u32;
        let mut snoozed_count = 0u32;
        let mut muted_count = 0u32;
        let mut resolved_count = 0u32;
        let mut durable_retained_count = 0u32;
        let mut durable_history_preserved = true;

        for state in by_event.values() {
            if state.active_badge_counted {
                active_count += 1;
            }
            if state.held_or_suppressed_counted {
                held_or_suppressed_count += 1;
            }
            if state.acknowledged {
                acknowledged_count += 1;
            }
            if state.snoozed {
                snoozed_count += 1;
            }
            if state.muted {
                muted_count += 1;
            }
            if state.resolved {
                resolved_count += 1;
            }
            if state.durable_item_retained {
                durable_retained_count += 1;
            } else {
                durable_history_preserved = false;
            }
        }

        Self {
            record_kind: NOTIFICATION_BADGE_RECONCILIATION_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_ACTION_STATE_SCHEMA_VERSION,
            badge_class,
            active_count,
            held_or_suppressed_count,
            acknowledged_count,
            snoozed_count,
            muted_count,
            resolved_count,
            durable_retained_count,
            durable_history_preserved,
            privacy_safe_summary_label: badge_summary_label(
                badge_class,
                active_count,
                held_or_suppressed_count,
            ),
        }
    }
}

fn badge_summary_label(
    badge_class: BadgeClass,
    active_count: u32,
    held_or_suppressed_count: u32,
) -> String {
    let singular = badge_class.singular_label();
    let plural = badge_class.plural_label();
    match (active_count, held_or_suppressed_count) {
        (0, 0) => format!("No {plural}"),
        (1, 0) => format!("1 {singular}"),
        (n, 0) => format!("{n} {plural}"),
        (0, 1) => format!("No active {plural}; 1 held"),
        (0, h) => format!("No active {plural}; {h} held"),
        (1, 1) => format!("1 {singular}; 1 held"),
        (1, h) => format!("1 {singular}; {h} held"),
        (n, 1) => format!("{n} {plural}; 1 held"),
        (n, h) => format!("{n} {plural}; {h} held"),
    }
}

impl BadgeClass {
    fn singular_label(self) -> &'static str {
        match self {
            Self::NeedsReview => "review item",
            Self::FailedRuns => "failed run",
            Self::Mentions => "mention",
            Self::SecurityNotices => "security notice",
            Self::SessionRequests => "session request",
            Self::OfflinePublishPending => "offline publish item",
            Self::DurableRunningCount => "running item",
            Self::HeldOrSuppressedCount => "held item",
            Self::CompletionUnread => "unread completion",
        }
    }

    fn plural_label(self) -> &'static str {
        match self {
            Self::NeedsReview => "review items",
            Self::FailedRuns => "failed runs",
            Self::Mentions => "mentions",
            Self::SecurityNotices => "security notices",
            Self::SessionRequests => "session requests",
            Self::OfflinePublishPending => "offline publish items",
            Self::DurableRunningCount => "running items",
            Self::HeldOrSuppressedCount => "held items",
            Self::CompletionUnread => "unread completions",
        }
    }
}
