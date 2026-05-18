//! Attention / review inbox triage projection.
//!
//! The [`AttentionInboxItem`] is the structural row for things that
//! need explicit human action rather than background completion. It
//! carries why the item is shown to this user, the freshness of the
//! evidence, the authority source, the suppression posture from
//! quiet-hours / focus / policy, and a closed verb set
//! ([`AttentionTriageVerb`]) that distinguishes snooze, acknowledge,
//! clear, mute, and resolve so each remains attributable in exported
//! history.
//!
//! Inbox items live alongside the chronology rows from
//! [`super::row::ActivityEventRow`]; a single canonical object can
//! have both a chronology row and an inbox item. The chronology row
//! records what happened; the inbox item records what the user is
//! being asked to do about it.

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{QuietHoursMode, SourceSubsystem, SuppressionReason};

use super::row::{ChronologyLane, DetailLink, ImportanceClass, ScopeObjectKind};

/// Schema version mirrored from the timeline packet for inbox rows.
pub const ATTENTION_INBOX_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`AttentionInboxItem`] payloads.
pub const ATTENTION_INBOX_ITEM_RECORD_KIND: &str = "attention_inbox_item_record";

/// Stable record kind for [`AttentionInboxSnapshot`] payloads.
pub const ATTENTION_INBOX_SNAPSHOT_RECORD_KIND: &str = "attention_inbox_snapshot_record";

/// Closed why-shown vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxWhyShownReason {
    /// User is the assigned reviewer of an AI apply or approval.
    AssignedReviewer,
    /// User owns the affected workspace or workspace scope.
    WorkspaceOwner,
    /// User holds the policy authority that must rule on the row.
    PolicyAuthority,
    /// User is the actor whose action originated the row and must
    /// confirm or revalidate.
    OriginatingActor,
    /// User holds an admin role and a managed policy directed the
    /// item to admins.
    AdminAddressed,
    /// Item was directly addressed to this user by another actor.
    DirectlyAddressed,
    /// Item is a recovery proposal awaiting the user.
    RecoveryAddressed,
    /// Item is a reconnect prompt the user must rule on.
    ReconnectAddressed,
    /// Item is a provider-sync prompt the user must rule on.
    ProviderSyncAddressed,
    /// Item is an update prompt the user must rule on.
    UpdateAddressed,
}

impl InboxWhyShownReason {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssignedReviewer => "assigned_reviewer",
            Self::WorkspaceOwner => "workspace_owner",
            Self::PolicyAuthority => "policy_authority",
            Self::OriginatingActor => "originating_actor",
            Self::AdminAddressed => "admin_addressed",
            Self::DirectlyAddressed => "directly_addressed",
            Self::RecoveryAddressed => "recovery_addressed",
            Self::ReconnectAddressed => "reconnect_addressed",
            Self::ProviderSyncAddressed => "provider_sync_addressed",
            Self::UpdateAddressed => "update_addressed",
        }
    }
}

/// Closed authority-source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxAuthoritySourceClass {
    /// First-party shell / runtime decided the user should be asked.
    FirstParty,
    /// Admin policy directed the prompt.
    AdminPolicy,
    /// AI agent requested approval / review.
    AiAgent,
    /// Remote service (provider, mirror, hosted review) requested
    /// approval / review.
    RemoteService,
    /// Installed extension surfaced the request.
    Extension,
    /// User explicitly snoozed and the inbox is re-presenting.
    UserSnoozeExpiry,
    /// Recovery / restore subsystem requested confirmation.
    RecoverySubsystem,
}

impl InboxAuthoritySourceClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::AdminPolicy => "admin_policy",
            Self::AiAgent => "ai_agent",
            Self::RemoteService => "remote_service",
            Self::Extension => "extension",
            Self::UserSnoozeExpiry => "user_snooze_expiry",
            Self::RecoverySubsystem => "recovery_subsystem",
        }
    }
}

/// Closed freshness vocabulary for the underlying evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionFreshnessClass {
    /// Evidence is fresh from the current observation.
    Fresh,
    /// Evidence is recent but pre-dates the current session.
    Recent,
    /// Evidence is stale and re-validation is recommended before
    /// acting.
    StaleRevalidationRecommended,
    /// Evidence is stale and revalidation is required before acting.
    StaleRevalidationRequired,
    /// Evidence is restored from a backup, snapshot, or audit trail.
    ReconstructedFromBackup,
}

impl AttentionFreshnessClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::StaleRevalidationRecommended => "stale_revalidation_recommended",
            Self::StaleRevalidationRequired => "stale_revalidation_required",
            Self::ReconstructedFromBackup => "reconstructed_from_backup",
        }
    }

    /// True when freshness blocks an act-on action until revalidation.
    pub const fn requires_revalidation(self) -> bool {
        matches!(self, Self::StaleRevalidationRequired)
    }
}

/// Closed verb set distinguishing snooze / acknowledge / clear / mute
/// / resolve. Each verb is attributable in exported history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionTriageVerb {
    /// Open the underlying durable object or evidence.
    Open,
    /// Snooze the item until a freshness boundary expires.
    Snooze,
    /// Acknowledge the item without resolving the underlying issue.
    Acknowledge,
    /// Clear the item from the current view without resolving (history
    /// row persists).
    Clear,
    /// Mute the item's class for the current user (durable preference).
    Mute,
    /// Resolve the underlying issue and close the inbox item.
    Resolve,
    /// Escalate to an admin / reviewer.
    Escalate,
}

impl AttentionTriageVerb {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Snooze => "snooze",
            Self::Acknowledge => "acknowledge",
            Self::Clear => "clear",
            Self::Mute => "mute",
            Self::Resolve => "resolve",
            Self::Escalate => "escalate",
        }
    }

    /// True when the verb keeps the row visible in some lane.
    pub const fn preserves_durable_history(self) -> bool {
        matches!(
            self,
            Self::Snooze
                | Self::Acknowledge
                | Self::Clear
                | Self::Mute
                | Self::Resolve
                | Self::Escalate
                | Self::Open
        )
    }
}

/// Availability posture for a triage action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxAvailabilityClass {
    Enabled,
    Disabled,
    RequiresRevalidation,
    NotApplicable,
}

impl InboxAvailabilityClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::RequiresRevalidation => "requires_revalidation",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// One triage action exposed by the item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionTriageAction {
    /// Triage verb.
    pub verb: AttentionTriageVerb,
    /// Stable command id when the action is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Availability posture.
    pub availability: InboxAvailabilityClass,
    /// Disabled reason label when availability is disabled or requires
    /// revalidation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_label: Option<String>,
    /// Target identity the action operates on.
    pub target_identity_ref: String,
}

/// Quiet-hours / focus / policy suppression note carried on the item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionInboxSuppressionNote {
    /// Quiet-hours / focus / admin modes active when the item was
    /// minted.
    pub active_modes: Vec<QuietHoursMode>,
    /// Suppression reasons that explain why a transient surface was
    /// held; the inbox item itself is never destroyed by a hold.
    pub suppression_reasons: Vec<SuppressionReason>,
    /// True when a transient surface (toast, OS notification) was held.
    pub transient_surface_held: bool,
    /// True when durable evidence is preserved despite the hold.
    pub durable_history_preserved: bool,
    /// Stable label describing when the suppression releases.
    pub release_rule_label: String,
}

impl AttentionInboxSuppressionNote {
    /// Build a never-suppressed posture.
    pub fn never_suppressed() -> Self {
        Self {
            active_modes: vec![QuietHoursMode::ModeNone],
            suppression_reasons: Vec::new(),
            transient_surface_held: false,
            durable_history_preserved: true,
            release_rule_label: "Not held; durable inbox surface always visible.".into(),
        }
    }
}

/// One attention / review inbox item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionInboxItem {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version mirrored from the packet.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable inbox-item id.
    pub inbox_item_id: String,
    /// Canonical event id that minted this item.
    pub canonical_event_id: String,
    /// Canonical object the item is about.
    pub canonical_object_target_ref: String,
    /// Chronology lane this item references.
    pub chronology_lane: ChronologyLane,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Scope/object kind.
    pub scope_object_kind: ScopeObjectKind,
    /// Importance class.
    pub importance_class: ImportanceClass,
    /// Why this item is shown to this user.
    pub why_shown_reason: InboxWhyShownReason,
    /// Authority that requested the user's attention.
    pub authority_source_class: InboxAuthoritySourceClass,
    /// Freshness of the underlying evidence.
    pub freshness_class: AttentionFreshnessClass,
    /// Title rendered by chrome.
    pub title_label: String,
    /// Body label rendered by chrome.
    pub body_label: String,
    /// Short scope label (workspace, provider, repository).
    pub scope_label: String,
    /// Detail link to the underlying durable object.
    pub detail_link: DetailLink,
    /// Cross-reference to a chronology row id, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_row_id_ref: Option<String>,
    /// First time this item was minted.
    pub minted_at: String,
    /// Last time the item was observed.
    pub last_observed_at: String,
    /// Snooze expiry timestamp when the user has snoozed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snoozed_until: Option<String>,
    /// True when the user has acknowledged the item without resolving
    /// the underlying issue.
    pub acknowledged: bool,
    /// True when the underlying issue is resolved and the item is in
    /// the history lane.
    pub resolved: bool,
    /// True when the item's class has been muted by the user.
    pub muted: bool,
    /// Triage actions exposed by the item.
    pub actions: Vec<AttentionTriageAction>,
    /// Quiet-hours / focus / policy suppression posture.
    pub suppression_note: AttentionInboxSuppressionNote,
}

impl AttentionInboxItem {
    /// True when the item is in the actionable inbox (not snoozed,
    /// not resolved, not muted, freshness does not block actions).
    pub fn is_actionable_now(&self) -> bool {
        !self.resolved
            && !self.muted
            && self.snoozed_until.is_none()
            && !self.freshness_class.requires_revalidation()
    }

    /// True when the item exposes the open / snooze / acknowledge /
    /// resolve verb set required for triage parity.
    pub fn exposes_triage_verb_set(&self) -> bool {
        let mut verbs: std::collections::BTreeSet<AttentionTriageVerb> =
            std::collections::BTreeSet::new();
        for action in &self.actions {
            verbs.insert(action.verb);
        }
        verbs.contains(&AttentionTriageVerb::Open)
            && verbs.contains(&AttentionTriageVerb::Snooze)
            && verbs.contains(&AttentionTriageVerb::Acknowledge)
            && verbs.contains(&AttentionTriageVerb::Resolve)
    }

    /// True when quiet-hours suppression preserves durable history.
    pub const fn quiet_hours_preserves_history(&self) -> bool {
        self.suppression_note.durable_history_preserved
    }
}

/// Deterministic inbox projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionInboxSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Items in the inbox, ordered deterministically.
    pub items: Vec<AttentionInboxItem>,
    /// Count of items currently actionable.
    pub actionable_count: usize,
    /// Count of items currently snoozed.
    pub snoozed_count: usize,
    /// Count of items currently acknowledged but unresolved.
    pub acknowledged_unresolved_count: usize,
    /// Count of resolved items in the history lane.
    pub resolved_count: usize,
    /// Count of items whose class is muted.
    pub muted_count: usize,
    /// True when at least one item declares it is honestly
    /// failed/denied/blocked.
    pub honesty_marker_present: bool,
}

impl AttentionInboxSnapshot {
    /// Build a snapshot from a deterministic, pre-ordered item list.
    pub fn from_items(items: Vec<AttentionInboxItem>) -> Self {
        let actionable_count = items.iter().filter(|item| item.is_actionable_now()).count();
        let snoozed_count = items
            .iter()
            .filter(|item| item.snoozed_until.is_some())
            .count();
        let acknowledged_unresolved_count = items
            .iter()
            .filter(|item| item.acknowledged && !item.resolved)
            .count();
        let resolved_count = items.iter().filter(|item| item.resolved).count();
        let muted_count = items.iter().filter(|item| item.muted).count();
        let honesty_marker_present = items.iter().any(|item| {
            matches!(
                item.freshness_class,
                AttentionFreshnessClass::StaleRevalidationRequired
                    | AttentionFreshnessClass::StaleRevalidationRecommended
                    | AttentionFreshnessClass::ReconstructedFromBackup
            ) || item.suppression_note.transient_surface_held
        });
        Self {
            record_kind: ATTENTION_INBOX_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
            items,
            actionable_count,
            snoozed_count,
            acknowledged_unresolved_count,
            resolved_count,
            muted_count,
            honesty_marker_present,
        }
    }

    /// Number of items in the snapshot.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// True when the snapshot has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
