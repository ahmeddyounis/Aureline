//! Lifecycle action grammar for routed attention.
//!
//! The notification action verbs — dismiss, snooze, acknowledge, mute, clear,
//! and resolve — all preserve the same canonical event identity but differ in
//! their badge, retention, and export effects. This module describes those
//! effects as typed, joinable descriptors so every surface (toast, banner,
//! status overflow, activity center, OS notification, companion push) offers
//! the same governed verbs with the same downstream meaning, and so support
//! exports can reconstruct what an action *would* do without raw copy.
//!
//! The behavioral state transitions themselves live on
//! [`crate::notifications::actions::NotificationAttentionState`]; this module
//! is the contract layer the router publishes alongside each outcome.

use serde::{Deserialize, Serialize};

use crate::notifications::actions::NotificationLifecycleActionKind;

/// Effect a lifecycle action has on the active badge count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleBadgeEffect {
    /// The active badge count is unchanged (transient chrome only).
    LeavesActiveBadge,
    /// The item stops contributing to the active badge count.
    ClearsActiveBadge,
    /// The item moves from the active count into the held/suppressed count.
    MovesToHeldCount,
}

impl LifecycleBadgeEffect {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeavesActiveBadge => "leaves_active_badge",
            Self::ClearsActiveBadge => "clears_active_badge",
            Self::MovesToHeldCount => "moves_to_held_count",
        }
    }
}

/// Effect a lifecycle action has on durable retention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleRetentionEffect {
    /// The durable activity row stays in the active inbox and in history.
    RetainsActiveRow,
    /// The item leaves the active inbox but remains reachable in history.
    RetainsHistoryOnly,
    /// The item is recorded as resolved through its owning model.
    RetainsResolvedRecord,
}

impl LifecycleRetentionEffect {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetainsActiveRow => "retains_active_row",
            Self::RetainsHistoryOnly => "retains_history_only",
            Self::RetainsResolvedRecord => "retains_resolved_record",
        }
    }
}

/// How a lifecycle action is reflected in support / retention exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleExportEffect {
    ExportedAsActive,
    ExportedAsDismissed,
    ExportedAsAcknowledged,
    ExportedAsSnoozed,
    ExportedAsMuted,
    ExportedAsCleared,
    ExportedAsResolved,
}

impl LifecycleExportEffect {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportedAsActive => "exported_as_active",
            Self::ExportedAsDismissed => "exported_as_dismissed",
            Self::ExportedAsAcknowledged => "exported_as_acknowledged",
            Self::ExportedAsSnoozed => "exported_as_snoozed",
            Self::ExportedAsMuted => "exported_as_muted",
            Self::ExportedAsCleared => "exported_as_cleared",
            Self::ExportedAsResolved => "exported_as_resolved",
        }
    }
}

/// One available lifecycle action published on a route outcome, with its
/// badge, retention, and export semantics. Surfaces bind behavior to
/// `action_kind`; the descriptor explains the downstream effect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableLifecycleAction {
    pub action_kind: NotificationLifecycleActionKind,
    pub badge_effect: LifecycleBadgeEffect,
    pub retention_effect: LifecycleRetentionEffect,
    pub export_effect: LifecycleExportEffect,
    /// True only when the action mutates the source object through its owner.
    pub mutates_source_object: bool,
    /// True when the action requires a named resume condition (snooze).
    pub requires_resume_condition: bool,
    /// True when the action requires a muted class/source ref (mute).
    pub requires_muted_class: bool,
    /// True when durable history survives the action (always true for the
    /// governed verbs — none of them erase durable truth).
    pub durable_history_preserved: bool,
}

impl AvailableLifecycleAction {
    /// The governed descriptor for one lifecycle action kind.
    pub fn for_kind(action_kind: NotificationLifecycleActionKind) -> Self {
        use NotificationLifecycleActionKind as Kind;
        match action_kind {
            Kind::Dismiss => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::LeavesActiveBadge,
                retention_effect: LifecycleRetentionEffect::RetainsActiveRow,
                export_effect: LifecycleExportEffect::ExportedAsDismissed,
                mutates_source_object: false,
                requires_resume_condition: false,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
            Kind::Acknowledge => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::ClearsActiveBadge,
                retention_effect: LifecycleRetentionEffect::RetainsActiveRow,
                export_effect: LifecycleExportEffect::ExportedAsAcknowledged,
                mutates_source_object: false,
                requires_resume_condition: false,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
            Kind::Snooze => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::MovesToHeldCount,
                retention_effect: LifecycleRetentionEffect::RetainsActiveRow,
                export_effect: LifecycleExportEffect::ExportedAsSnoozed,
                mutates_source_object: false,
                requires_resume_condition: true,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
            Kind::Mute => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::ClearsActiveBadge,
                retention_effect: LifecycleRetentionEffect::RetainsActiveRow,
                export_effect: LifecycleExportEffect::ExportedAsMuted,
                mutates_source_object: false,
                requires_resume_condition: false,
                requires_muted_class: true,
                durable_history_preserved: true,
            },
            Kind::Clear => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::ClearsActiveBadge,
                retention_effect: LifecycleRetentionEffect::RetainsHistoryOnly,
                export_effect: LifecycleExportEffect::ExportedAsCleared,
                mutates_source_object: false,
                requires_resume_condition: false,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
            Kind::Resolve => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::ClearsActiveBadge,
                retention_effect: LifecycleRetentionEffect::RetainsResolvedRecord,
                export_effect: LifecycleExportEffect::ExportedAsResolved,
                mutates_source_object: true,
                requires_resume_condition: false,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
            // Suppress is a system-side hold, not a user verb; the router
            // publishes it as a held export effect with no badge promotion.
            Kind::Suppress => Self {
                action_kind,
                badge_effect: LifecycleBadgeEffect::MovesToHeldCount,
                retention_effect: LifecycleRetentionEffect::RetainsActiveRow,
                export_effect: LifecycleExportEffect::ExportedAsActive,
                mutates_source_object: false,
                requires_resume_condition: false,
                requires_muted_class: false,
                durable_history_preserved: true,
            },
        }
    }
}

/// The six governed user verbs, in stable order, that every routed attention
/// item exposes. `Suppress` is intentionally excluded — it is a system-side
/// hold, not a user verb.
pub const GOVERNED_USER_ACTIONS: [NotificationLifecycleActionKind; 6] = [
    NotificationLifecycleActionKind::Dismiss,
    NotificationLifecycleActionKind::Snooze,
    NotificationLifecycleActionKind::Acknowledge,
    NotificationLifecycleActionKind::Mute,
    NotificationLifecycleActionKind::Clear,
    NotificationLifecycleActionKind::Resolve,
];

/// The governed user-verb descriptors in stable order.
pub fn governed_user_actions() -> Vec<AvailableLifecycleAction> {
    GOVERNED_USER_ACTIONS
        .iter()
        .copied()
        .map(AvailableLifecycleAction::for_kind)
        .collect()
}
