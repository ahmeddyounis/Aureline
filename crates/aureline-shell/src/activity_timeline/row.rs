//! Shared event/history row primitive.
//!
//! The [`ActivityEventRow`] is the single row shape every chronology
//! lane projects into. It carries a timestamp, an actor or subsystem,
//! an action verb, a scope/object reference, an outcome, the current
//! actionability class, and a non-truncating detail link. Lanes
//! distinguish themselves via [`ChronologyLane`] but otherwise share
//! identical structure so the activity center, AI evidence lane,
//! policy-change lane, provider-sync lane, update history, reconnect
//! flows, and recovery flows agree on vocabulary.

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::SourceSubsystem;

/// Stable record kind for [`ActivityEventRow`] payloads.
pub const ACTIVITY_EVENT_ROW_RECORD_KIND: &str = "activity_event_row_record";

/// Schema version carried by row payloads. Mirrors
/// [`super::ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION`].
pub const ACTIVITY_EVENT_ROW_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`TimelineGroup`] payloads.
pub const TIMELINE_GROUP_RECORD_KIND: &str = "activity_timeline_group_record";

/// Stable record kind for [`NarrativeSummaryCard`] payloads.
pub const NARRATIVE_SUMMARY_CARD_RECORD_KIND: &str = "activity_narrative_summary_card_record";

/// Chronology lane the row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyLane {
    /// Activity center: long-running execution jobs.
    ActivityCenter,
    /// AI evidence and apply review history.
    AiEvidence,
    /// Policy / governance change history.
    PolicyChanges,
    /// Provider sync, refresh, and import history.
    ProviderSync,
    /// Install, update, and download history.
    UpdateHistory,
    /// Remote attach, mirror, and reconnect history.
    ReconnectFlow,
    /// Session restore and recovery history.
    Recovery,
    /// Approval / review request history that is not bound to a job.
    Approvals,
}

impl ChronologyLane {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::AiEvidence => "ai_evidence",
            Self::PolicyChanges => "policy_changes",
            Self::ProviderSync => "provider_sync",
            Self::UpdateHistory => "update_history",
            Self::ReconnectFlow => "reconnect_flow",
            Self::Recovery => "recovery",
            Self::Approvals => "approvals",
        }
    }
}

/// Actor that produced or owns the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    /// Human operator on the local install.
    UserActor,
    /// Background automation owned by the product.
    SystemActor,
    /// AI apply or suggestion path.
    AiAgentActor,
    /// Non-AI hosted service (sync mirror, provider, symbol service).
    RemoteServiceActor,
    /// Policy or admin authority acting on behalf of the deployment.
    AdminPolicyActor,
    /// Installed extension.
    ExtensionActor,
    /// Unknown / unresolved actor identity at boundary time.
    UnknownActor,
}

impl ActorKind {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserActor => "user_actor",
            Self::SystemActor => "system_actor",
            Self::AiAgentActor => "ai_agent_actor",
            Self::RemoteServiceActor => "remote_service_actor",
            Self::AdminPolicyActor => "admin_policy_actor",
            Self::ExtensionActor => "extension_actor",
            Self::UnknownActor => "unknown_actor",
        }
    }

    /// True when the actor kind requires a non-null
    /// `actor_identity_ref`.
    pub const fn requires_identity_ref(self) -> bool {
        matches!(
            self,
            Self::UserActor
                | Self::AiAgentActor
                | Self::RemoteServiceActor
                | Self::AdminPolicyActor
                | Self::ExtensionActor
        )
    }
}

/// Closed vocabulary of what the row records as having happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionVerb {
    Started,
    Progressed,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    Unblocked,
    Held,
    Released,
    Granted,
    Narrowed,
    Revoked,
    Superseded,
    Proposed,
    Accepted,
    Rejected,
    Restored,
    Recovered,
    Published,
    Reconnected,
    Disconnected,
    Acknowledged,
    Dismissed,
    Snoozed,
    Resolved,
    Muted,
    Cleared,
    Imported,
}

impl ActionVerb {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Progressed => "progressed",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Blocked => "blocked",
            Self::Unblocked => "unblocked",
            Self::Held => "held",
            Self::Released => "released",
            Self::Granted => "granted",
            Self::Narrowed => "narrowed",
            Self::Revoked => "revoked",
            Self::Superseded => "superseded",
            Self::Proposed => "proposed",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Restored => "restored",
            Self::Recovered => "recovered",
            Self::Published => "published",
            Self::Reconnected => "reconnected",
            Self::Disconnected => "disconnected",
            Self::Acknowledged => "acknowledged",
            Self::Dismissed => "dismissed",
            Self::Snoozed => "snoozed",
            Self::Resolved => "resolved",
            Self::Muted => "muted",
            Self::Cleared => "cleared",
            Self::Imported => "imported",
        }
    }
}

/// Closed outcome class for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeClass {
    Pending,
    InProgress,
    Succeeded,
    Failed,
    Cancelled,
    Denied,
    Held,
    Superseded,
    Recovered,
    ObservedOnly,
    AwaitingApproval,
}

impl OutcomeClass {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Denied => "denied",
            Self::Held => "held",
            Self::Superseded => "superseded",
            Self::Recovered => "recovered",
            Self::ObservedOnly => "observed_only",
            Self::AwaitingApproval => "awaiting_approval",
        }
    }
}

/// Importance class for grouping/collapse rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportanceClass {
    Routine,
    Consequential,
    SafetyCritical,
}

impl ImportanceClass {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routine => "routine",
            Self::Consequential => "consequential",
            Self::SafetyCritical => "safety_critical",
        }
    }
}

/// Current actionability of the row from the user's perspective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionabilityClass {
    /// Row is history-only; no current action is offered to the user.
    None,
    /// Row remains observable; the user may open details.
    OpenDetailsOnly,
    /// Row offers a typed reviewable action (acknowledge, snooze) but
    /// does not block work.
    Reviewable,
    /// Row needs explicit human action before work can continue.
    RequiresUserAction,
    /// Row requires revalidation (trust/policy boundary) before any
    /// action is available.
    RequiresRevalidation,
}

impl ActionabilityClass {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OpenDetailsOnly => "open_details_only",
            Self::Reviewable => "reviewable",
            Self::RequiresUserAction => "requires_user_action",
            Self::RequiresRevalidation => "requires_revalidation",
        }
    }
}

/// Closed scope-object vocabulary describing what the row is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeObjectKind {
    AiEvidenceRow,
    ApprovalRequestRow,
    DurableJobRow,
    PolicyDecisionRow,
    ProviderGrantRow,
    ProviderSyncRow,
    PublicationRow,
    RecoverySnapshotRow,
    SupportBundleRow,
    UpdateEventRow,
    ReconnectEventRow,
    WorkspaceObjectRow,
    ExtensionLifecycleRow,
}

impl ScopeObjectKind {
    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiEvidenceRow => "ai_evidence_row",
            Self::ApprovalRequestRow => "approval_request_row",
            Self::DurableJobRow => "durable_job_row",
            Self::PolicyDecisionRow => "policy_decision_row",
            Self::ProviderGrantRow => "provider_grant_row",
            Self::ProviderSyncRow => "provider_sync_row",
            Self::PublicationRow => "publication_row",
            Self::RecoverySnapshotRow => "recovery_snapshot_row",
            Self::SupportBundleRow => "support_bundle_row",
            Self::UpdateEventRow => "update_event_row",
            Self::ReconnectEventRow => "reconnect_event_row",
            Self::WorkspaceObjectRow => "workspace_object_row",
            Self::ExtensionLifecycleRow => "extension_lifecycle_row",
        }
    }
}

/// Detail-link kind. Generic home or external-URL fallbacks are not
/// allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailLinkKind {
    /// Opens the exact canonical object.
    CanonicalObjectExact,
    /// Opens the durable activity row.
    DurableActivityRow,
    /// Opens an evidence packet row.
    EvidencePacketRow,
    /// Opens a structured review sheet.
    ReviewSheet,
    /// Opens a diff view.
    DiffView,
    /// Opens a placeholder explaining why the exact target is not
    /// available.
    PlaceholderAnnounced,
    /// Opens a denial / revalidation explanation.
    DeniedRequiresRevalidation,
    /// Opens an audit-trail-only row (target is policy-redacted but
    /// audit lineage survives).
    AuditTrailOnly,
    /// Linkback is permanently lost; the row remains visible with a
    /// truthful unavailability label.
    NotAvailableLinkbackLost,
}

impl DetailLinkKind {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObjectExact => "canonical_object_exact",
            Self::DurableActivityRow => "durable_activity_row",
            Self::EvidencePacketRow => "evidence_packet_row",
            Self::ReviewSheet => "review_sheet",
            Self::DiffView => "diff_view",
            Self::PlaceholderAnnounced => "placeholder_announced",
            Self::DeniedRequiresRevalidation => "denied_requires_revalidation",
            Self::AuditTrailOnly => "audit_trail_only",
            Self::NotAvailableLinkbackLost => "not_available_linkback_lost",
        }
    }

    /// True when the link routes to an exact durable target rather
    /// than a placeholder, denial, or generic fallback.
    pub const fn opens_exact_target(self) -> bool {
        matches!(
            self,
            Self::CanonicalObjectExact
                | Self::DurableActivityRow
                | Self::EvidencePacketRow
                | Self::ReviewSheet
                | Self::DiffView
        )
    }

    /// True when the link MUST carry an `unavailability_reason_label`.
    pub const fn requires_unavailability_label(self) -> bool {
        matches!(self, Self::AuditTrailOnly | Self::NotAvailableLinkbackLost)
    }
}

/// One non-truncating detail link rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailLink {
    /// Detail-link kind.
    pub kind: DetailLinkKind,
    /// Exact target identity reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_identity_ref: Option<String>,
    /// True when the target survives the current session.
    pub is_durable: bool,
    /// Required (non-null) for `audit_trail_only` and
    /// `not_available_linkback_lost` kinds. Truthful, privacy-safe
    /// explanation of why the exact target is unreachable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailability_reason_label: Option<String>,
    /// Accessibility announcement label for the link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announcement_label: Option<String>,
}

/// One structured row in the shared chronology.
///
/// Every chronology lane mints rows in this shape so the activity
/// center, AI evidence lane, policy-change lane, provider sync,
/// update history, reconnect flow, and recovery flow can be inspected
/// with the same vocabulary. Lanes set [`Self::chronology_lane`] to
/// keep their projection separable while sharing structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityEventRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version mirrored from the timeline packet.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub event_row_id: String,
    /// Canonical event id that minted this row.
    pub canonical_event_id: String,
    /// Canonical object the row is about.
    pub canonical_object_target_ref: String,
    /// Lane this row belongs to.
    pub chronology_lane: ChronologyLane,
    /// Originating subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Actor that produced or owns the row.
    pub actor_kind: ActorKind,
    /// Opaque actor identity. Required when
    /// [`ActorKind::requires_identity_ref`] is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_identity_ref: Option<String>,
    /// Short label naming the actor or owning subsystem.
    pub actor_or_subsystem_label: String,
    /// Scope/object kind the row is about.
    pub scope_object_kind: ScopeObjectKind,
    /// Action verb the row records.
    pub action_verb: ActionVerb,
    /// Closed outcome class.
    pub outcome_class: OutcomeClass,
    /// Importance class for grouping rules.
    pub importance_class: ImportanceClass,
    /// Current actionability from the user's perspective.
    pub actionability_class: ActionabilityClass,
    /// Short privacy-safe summary label.
    pub summary_label: String,
    /// Short scope label (workspace, provider, repository).
    pub scope_label: String,
    /// Canonical UTC ISO 8601 timestamp for ordering.
    pub monotonic_timestamp: String,
    /// First time this row was observed.
    pub minted_at: String,
    /// Last time this row was observed.
    pub last_observed_at: String,
    /// Non-truncating detail link.
    pub detail_link: DetailLink,
    /// Optional back-reference to a notification canonical_event_id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_canonical_event_id_ref: Option<String>,
    /// Optional grouped-burst id from the event lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_burst_id_ref: Option<String>,
    /// Required (non-null) when `action_verb` is `superseded`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes_event_row_id_ref: Option<String>,
    /// True when quiet-hours / focus mode / similar held a fanout for
    /// this row. The durable row remains visible even when held.
    pub quiet_hours_held: bool,
    /// Repeated observations of the same canonical event collapse onto
    /// this row.
    pub occurrence_count: u32,
}

impl ActivityEventRow {
    /// True when the row exposes an exact reopen target rather than a
    /// placeholder or denial.
    pub fn reopens_exact_target(&self) -> bool {
        self.detail_link.kind.opens_exact_target() && self.detail_link.target_identity_ref.is_some()
    }

    /// True when the row needs explicit user action.
    pub const fn needs_user_action(&self) -> bool {
        matches!(
            self.actionability_class,
            ActionabilityClass::RequiresUserAction
        )
    }

    /// True when actor-identity rules are satisfied.
    pub fn actor_identity_rule_satisfied(&self) -> bool {
        if self.actor_kind.requires_identity_ref() {
            self.actor_identity_ref
                .as_ref()
                .is_some_and(|id| !id.is_empty())
        } else {
            true
        }
    }

    /// True when the detail-link unavailability rule is satisfied.
    pub fn detail_link_rule_satisfied(&self) -> bool {
        if self.detail_link.kind.requires_unavailability_label() {
            self.detail_link
                .unavailability_reason_label
                .as_ref()
                .is_some_and(|label| !label.is_empty())
        } else {
            true
        }
    }

    /// True when supersession bookkeeping is satisfied.
    pub fn supersedes_rule_satisfied(&self) -> bool {
        match self.action_verb {
            ActionVerb::Superseded => self
                .supersedes_event_row_id_ref
                .as_ref()
                .is_some_and(|id| !id.is_empty()),
            _ => true,
        }
    }

    /// True when a consequential or safety-critical row exposes a
    /// non-truncating durable detail link.
    pub fn importance_rule_satisfied(&self) -> bool {
        match self.importance_class {
            ImportanceClass::Consequential | ImportanceClass::SafetyCritical => {
                self.detail_link.is_durable && self.detail_link.kind.opens_exact_target()
            }
            ImportanceClass::Routine => true,
        }
    }
}

/// Closed timeline-group rule vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineGroupRule {
    SameCanonicalObject,
    SameGroupedBurstId,
    SameLinkedCanonicalEventId,
    SameLanePhase,
    SameActorWithinWindow,
    ExplicitUserPin,
}

impl TimelineGroupRule {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameCanonicalObject => "same_canonical_object",
            Self::SameGroupedBurstId => "same_grouped_burst_id",
            Self::SameLinkedCanonicalEventId => "same_linked_canonical_event_id",
            Self::SameLanePhase => "same_lane_phase",
            Self::SameActorWithinWindow => "same_actor_within_window",
            Self::ExplicitUserPin => "explicit_user_pin",
        }
    }
}

/// One grouped timeline view over a set of event rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineGroup {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable group id.
    pub timeline_group_id: String,
    /// Group rule.
    pub group_rule: TimelineGroupRule,
    /// Lane the group belongs to.
    pub chronology_lane: ChronologyLane,
    /// Optional canonical object target when the rule is
    /// `SameCanonicalObject`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_object_target_ref: Option<String>,
    /// Optional grouped-burst id when the rule is
    /// `SameGroupedBurstId`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_burst_id_ref: Option<String>,
    /// Optional linked canonical event id when the rule is
    /// `SameLinkedCanonicalEventId`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_canonical_event_id_ref: Option<String>,
    /// Phase boundaries that justify a section break in the group.
    pub phase_boundary_labels: Vec<String>,
    /// Member event-row ids. A group never erases a row; the row
    /// remains canonically reachable by id.
    pub member_event_row_ids: Vec<String>,
    /// Importance class of the group (the highest member importance).
    pub importance_class: ImportanceClass,
    /// True only when every member is routine and the rule permits
    /// truncation.
    pub allow_routine_row_truncation: bool,
    /// Stable label rendered by the chrome.
    pub summary_label: String,
    /// Earliest member timestamp.
    pub opened_at: String,
    /// Most recent member timestamp.
    pub last_updated_at: String,
    /// True when the group is collapsed by default.
    pub collapsed_by_default: bool,
}

impl TimelineGroup {
    /// True when the supplied member-row corpus includes all member
    /// row ids declared on the group.
    pub fn members_resolved_in(&self, rows: &[ActivityEventRow]) -> bool {
        self.member_event_row_ids
            .iter()
            .all(|id| rows.iter().any(|row| &row.event_row_id == id))
    }
}

/// One narrative summary card covering history-heavy work.
///
/// A summary card cites member rows by id; it never replaces them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrativeSummaryCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable card id.
    pub narrative_summary_card_id: String,
    /// Canonical subject object the card narrates.
    pub subject_canonical_object_target_ref: String,
    /// Lane the card belongs to.
    pub chronology_lane: ChronologyLane,
    /// Cited member-row ids. Required to be non-empty.
    pub cited_event_row_ids: Vec<String>,
    /// Optional cited group ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cited_timeline_group_ids: Vec<String>,
    /// Card importance.
    pub importance_class: ImportanceClass,
    /// Card title (short, privacy-safe).
    pub summary_title: String,
    /// Card body (short, privacy-safe).
    pub summary_body_label: String,
    /// Non-truncating detail link.
    pub detail_link: DetailLink,
    /// Earliest cited timestamp.
    pub opened_at: String,
    /// Last update timestamp.
    pub last_updated_at: String,
}

impl NarrativeSummaryCard {
    /// True when the card cites at least one row.
    pub fn cites_any_row(&self) -> bool {
        !self.cited_event_row_ids.is_empty()
    }

    /// True when the card's detail link is non-truncating.
    pub fn detail_link_durable(&self) -> bool {
        self.detail_link.is_durable && self.detail_link.kind.opens_exact_target()
    }
}
