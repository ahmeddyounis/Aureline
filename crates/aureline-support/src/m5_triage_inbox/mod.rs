//! M5 triage inboxes: ordered, reason-bearing queues over the same canonical
//! incident/support/admin objects the detail surfaces own.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the
//! *family* of a triage inbox — what it is, the one shared state vocabulary, and
//! the invariants every surface holds. The [operator boards](crate::m5_operator_boards)
//! render that family as tile *summaries*. This lane builds the first real triage
//! **rows**: the individual items an operator actually works, each one explicit
//! about *why* it needs attention, *whose* queue it is in, *what* priority and SLA
//! apply, *where* it came from, and whether its state is local, shared, deferred,
//! or imported.
//!
//! A triage inbox is not one more chronological feed. The hard part is keeping
//! each row legible as the queue scales:
//!
//! 1. **Reason-for-attention, never a bare unread badge.** Every [`TriageRow`]
//!    carries an [`AttentionClass`] — assigned, watched, policy-blocked, stale,
//!    waiting-on-approval, or locally-deferred — and a written
//!    [`TriageRow::reason_for_attention`]. The six classes never collapse into one
//!    count.
//! 2. **Canonical object identity, never a queue-only id.** Every row carries an
//!    `object_ref` — the same canonical `aureline://` handle the detail surfaces
//!    use — and its open-detail route, every batch-review candidate, and every
//!    handoff row preserve that exact ref.
//! 3. **Priority and SLA are first-class.** Every row carries a [`PriorityClass`]
//!    and an [`SlaState`]; an at-risk or breached SLA carries a written reason.
//! 4. **Local-versus-shared/deferred truth stays visible.** Every row carries a
//!    [`ScopeClass`] and a [`SyncStateClass`]; an imported-snapshot row has no live
//!    target and is excluded from live batch actions with a stated reason.
//! 5. **No silent green.** Each row's [`TriageRow::effective_state`] is the
//!    computed no-silent-green downgrade ([`compute_effective_state`]); a stale or
//!    waived row is never reported `clear`.
//! 6. **Grouping is part of the contract.** A [`SavedTriageView`] names both its
//!    grouping ([`GroupKeyClass`]) and its order, each with a stated reason, so the
//!    inbox never reorders by a hidden rule or flattens into a feed.
//! 7. **Batch-review and handoff preserve truth.** [`BatchReviewPreview`] states
//!    what acting on the surviving set does and which rows are excluded and why;
//!    [`TriageHandoffBundle`] freezes the default view as a `snapshot_only` export
//!    that keeps the filters, grouping, order, scope, ownership, freshness, source,
//!    provider, priority, and SLA labels instead of flattening them into a list.
//!
//! [`triage_inbox_set`] is the canonical binding: it builds the inboxes
//! deterministically and computes each [`TriageInvariant`]'s `holds` flag from the
//! built data, so the checked-in fixture and the replay gate freeze the contract
//! byte-for-byte. The record carries no endpoint URLs, hostnames, credentials,
//! raw payloads, or absolute paths — only opaque object refs, stable tokens, and
//! short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::m5_operator_boards::{
    compute_effective_state, BlockerWaiverClass, FreshnessClass, ObjectKind,
};
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorPathClass, OperatorStateClass, OperatorSurfaceClass,
    RedactionClass, ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the triage-inbox set.
pub const M5_TRIAGE_INBOX_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the triage-inbox set.
pub const M5_TRIAGE_INBOX_SCHEMA_REF: &str = "schemas/ops/m5-triage-inbox.schema.json";

/// Stable record-kind tag for the triage-inbox set.
pub const M5_TRIAGE_INBOX_RECORD_KIND: &str = "m5_triage_inbox_set";

/// Stable id for the canonical triage-inbox set.
pub const M5_TRIAGE_INBOX_SET_ID: &str = "m5-triage-inbox:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_TRIAGE_INBOX_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for object identity.
pub const M5_TRIAGE_INBOX_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_TRIAGE_INBOX_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Inbox families.
// ---------------------------------------------------------------------------

/// The first real triage inboxes this lane proves the shared contract with.
///
/// Each inbox is one operator-facing queue over many canonical objects, bound to
/// the [`OperatorSurfaceClass::TriageInbox`] family. Adding an inbox is a breaking
/// change to the set; the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxClass {
    /// Incident triage: open incidents and post-incident review items.
    IncidentTriage,
    /// Support triage: open support cases.
    SupportTriage,
    /// Admin triage: pending governance approval requests and access reviews.
    AdminTriage,
}

impl InboxClass {
    /// All inboxes, in set order.
    pub const ALL: [Self; 3] = [Self::IncidentTriage, Self::SupportTriage, Self::AdminTriage];

    /// Stable snake_case token for this inbox.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentTriage => "incident_triage",
            Self::SupportTriage => "support_triage",
            Self::AdminTriage => "admin_triage",
        }
    }

    /// Stable, namespaced inbox id.
    pub fn inbox_id(self) -> String {
        format!("triage_inbox.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IncidentTriage => "Incident triage",
            Self::SupportTriage => "Support triage",
            Self::AdminTriage => "Admin triage",
        }
    }

    /// The operator-surface matrix family every triage inbox is an instance of.
    pub const fn surface(self) -> OperatorSurfaceClass {
        OperatorSurfaceClass::TriageInbox
    }
}

// ---------------------------------------------------------------------------
// Reason-for-attention.
// ---------------------------------------------------------------------------

/// Why a row is in this operator's queue.
///
/// These six classes are kept distinct on purpose: collapsing them into a single
/// unread badge is exactly the failure mode this lane forbids. The class answers
/// "why is this in my queue", which is separate from the object's
/// [`OperatorStateClass`] (its health) and its [`FreshnessClass`] (its age).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionClass {
    /// Assigned to this operator / this queue.
    Assigned,
    /// Watched by this operator, not assigned to them.
    Watched,
    /// Blocked by a policy gate the operator must resolve or escalate.
    PolicyBlocked,
    /// Surfaced because its evidence went stale and needs reconfirmation.
    Stale,
    /// Waiting on an approval before it can proceed.
    WaitingOnApproval,
    /// Captured locally and deferred (publish-later / draft) for this operator.
    LocallyDeferred,
}

impl AttentionClass {
    /// All attention classes, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::Assigned,
        Self::Watched,
        Self::PolicyBlocked,
        Self::Stale,
        Self::WaitingOnApproval,
        Self::LocallyDeferred,
    ];

    /// Stable snake_case token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Assigned => "assigned",
            Self::Watched => "watched",
            Self::PolicyBlocked => "policy_blocked",
            Self::Stale => "stale",
            Self::WaitingOnApproval => "waiting_on_approval",
            Self::LocallyDeferred => "locally_deferred",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Assigned => "Assigned",
            Self::Watched => "Watched",
            Self::PolicyBlocked => "Policy-blocked",
            Self::Stale => "Stale — reconfirm",
            Self::WaitingOnApproval => "Waiting on approval",
            Self::LocallyDeferred => "Locally deferred",
        }
    }
}

// ---------------------------------------------------------------------------
// Priority and SLA.
// ---------------------------------------------------------------------------

/// The triage priority of a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorityClass {
    /// P0 — critical.
    P0Critical,
    /// P1 — high.
    P1High,
    /// P2 — normal.
    P2Normal,
    /// P3 — low.
    P3Low,
}

impl PriorityClass {
    /// All priorities, highest first.
    pub const ALL: [Self; 4] = [Self::P0Critical, Self::P1High, Self::P2Normal, Self::P3Low];

    /// Stable snake_case token for this priority.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::P0Critical => "p0_critical",
            Self::P1High => "p1_high",
            Self::P2Normal => "p2_normal",
            Self::P3Low => "p3_low",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::P0Critical => "P0 — critical",
            Self::P1High => "P1 — high",
            Self::P2Normal => "P2 — normal",
            Self::P3Low => "P3 — low",
        }
    }

    /// Urgency rank, higher being more urgent, for priority-ordered views.
    pub const fn urgency_rank(self) -> i64 {
        match self {
            Self::P0Critical => 3,
            Self::P1High => 2,
            Self::P2Normal => 1,
            Self::P3Low => 0,
        }
    }
}

/// The SLA state of a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlaState {
    /// Within the SLA window.
    WithinSla,
    /// Approaching the SLA deadline.
    AtRisk,
    /// Past the SLA deadline.
    Breached,
    /// SLA paused during an announced read-only / drain / maintenance window.
    PausedInWindow,
    /// No SLA applies to this row.
    NoSla,
}

impl SlaState {
    /// All SLA states, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::WithinSla,
        Self::AtRisk,
        Self::Breached,
        Self::PausedInWindow,
        Self::NoSla,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinSla => "within_sla",
            Self::AtRisk => "at_risk",
            Self::Breached => "breached",
            Self::PausedInWindow => "paused_in_window",
            Self::NoSla => "no_sla",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WithinSla => "Within SLA",
            Self::AtRisk => "At risk",
            Self::Breached => "Breached",
            Self::PausedInWindow => "Paused (window)",
            Self::NoSla => "No SLA",
        }
    }

    /// Urgency rank, higher being more urgent, for SLA-ordered views.
    pub const fn urgency_rank(self) -> i64 {
        match self {
            Self::Breached => 4,
            Self::AtRisk => 3,
            Self::WithinSla => 2,
            Self::PausedInWindow => 1,
            Self::NoSla => 0,
        }
    }

    /// Whether a written SLA reason is required for this state, so an at-risk or
    /// breached SLA never reads as a bare flag.
    pub const fn requires_reason(self) -> bool {
        matches!(self, Self::AtRisk | Self::Breached)
    }
}

// ---------------------------------------------------------------------------
// Source / provider and local-versus-shared/deferred state.
// ---------------------------------------------------------------------------

/// Where a triage row came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Raised by incident alerting / the incident workspace.
    IncidentAlert,
    /// Opened through support intake.
    SupportIntake,
    /// Raised through the admin / governance approval flow.
    AdminGovernance,
    /// Raised by a release gate.
    ReleaseGate,
    /// Pushed by an external provider (webhook / managed control-plane signal).
    ProviderWebhook,
    /// Captured through the companion / browser surface.
    CompanionCapture,
    /// Imported replay evidence with no live target.
    ImportedSnapshot,
}

impl SourceClass {
    /// All sources, in vocabulary order.
    pub const ALL: [Self; 7] = [
        Self::IncidentAlert,
        Self::SupportIntake,
        Self::AdminGovernance,
        Self::ReleaseGate,
        Self::ProviderWebhook,
        Self::CompanionCapture,
        Self::ImportedSnapshot,
    ];

    /// Stable snake_case token for this source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentAlert => "incident_alert",
            Self::SupportIntake => "support_intake",
            Self::AdminGovernance => "admin_governance",
            Self::ReleaseGate => "release_gate",
            Self::ProviderWebhook => "provider_webhook",
            Self::CompanionCapture => "companion_capture",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IncidentAlert => "Incident alert",
            Self::SupportIntake => "Support intake",
            Self::AdminGovernance => "Admin governance",
            Self::ReleaseGate => "Release gate",
            Self::ProviderWebhook => "Provider webhook",
            Self::CompanionCapture => "Companion capture",
            Self::ImportedSnapshot => "Imported snapshot",
        }
    }

    /// Whether the source must name a concrete external provider identity rather
    /// than the `internal` sentinel.
    pub const fn requires_external_provider(self) -> bool {
        matches!(self, Self::ProviderWebhook | Self::CompanionCapture)
    }
}

/// The provider sentinel used for rows that were not raised by an external
/// provider.
pub const INTERNAL_PROVIDER: &str = "internal";

/// Whether a row's state is local, shared, deferred, or imported.
///
/// Distinct from [`ScopeClass`] (the governance scope of the object): this names
/// the *sync* truth so a deferred publish-later capture and an imported snapshot
/// can never read as a live shared item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStateClass {
    /// Local and private to this host; not yet shared.
    LocalOnly,
    /// Shared and live with the team / org.
    SharedLive,
    /// Captured locally and queued to publish later.
    DeferredPublishLater,
    /// Imported snapshot with no live refresh path.
    ImportedSnapshot,
}

impl SyncStateClass {
    /// All sync states, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::SharedLive,
        Self::DeferredPublishLater,
        Self::ImportedSnapshot,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SharedLive => "shared_live",
            Self::DeferredPublishLater => "deferred_publish_later",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::SharedLive => "Shared (live)",
            Self::DeferredPublishLater => "Deferred (publish later)",
            Self::ImportedSnapshot => "Imported snapshot",
        }
    }

    /// Whether a row in this sync state can join a live batch-review action. An
    /// imported snapshot has no live target, so it is read-only here.
    pub const fn batch_reviewable(self) -> bool {
        !matches!(self, Self::ImportedSnapshot)
    }
}

// ---------------------------------------------------------------------------
// Shared filter / grouping / order vocabulary.
// ---------------------------------------------------------------------------

/// The facets a saved view can filter a triage inbox on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriageFacetClass {
    /// Filter by reason-for-attention class.
    Attention,
    /// Filter by priority.
    Priority,
    /// Filter by SLA state.
    Sla,
    /// Filter by source.
    Source,
    /// Filter by local-versus-shared/deferred sync state.
    SyncState,
    /// Filter by the boundary the row belongs to.
    Boundary,
    /// Filter by governance scope.
    Scope,
    /// Filter by computed effective state.
    State,
    /// Filter by evidence freshness age.
    Freshness,
    /// Filter by owner.
    Owner,
    /// Filter by canonical object kind.
    ObjectKind,
    /// Filter by blocker / waiver state.
    BlockerWaiver,
}

impl TriageFacetClass {
    /// All facets, in vocabulary order.
    pub const ALL: [Self; 12] = [
        Self::Attention,
        Self::Priority,
        Self::Sla,
        Self::Source,
        Self::SyncState,
        Self::Boundary,
        Self::Scope,
        Self::State,
        Self::Freshness,
        Self::Owner,
        Self::ObjectKind,
        Self::BlockerWaiver,
    ];

    /// Stable snake_case token for this facet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attention => "attention",
            Self::Priority => "priority",
            Self::Sla => "sla",
            Self::Source => "source",
            Self::SyncState => "sync_state",
            Self::Boundary => "boundary",
            Self::Scope => "scope",
            Self::State => "state",
            Self::Freshness => "freshness",
            Self::Owner => "owner",
            Self::ObjectKind => "object_kind",
            Self::BlockerWaiver => "blocker_waiver",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Attention => "Reason for attention",
            Self::Priority => "Priority",
            Self::Sla => "SLA",
            Self::Source => "Source",
            Self::SyncState => "Local / shared / deferred",
            Self::Boundary => "Boundary",
            Self::Scope => "Scope",
            Self::State => "Effective state",
            Self::Freshness => "Freshness",
            Self::Owner => "Owner",
            Self::ObjectKind => "Object kind",
            Self::BlockerWaiver => "Blocker / waiver",
        }
    }

    /// Whether the facet's values come from a closed token vocabulary. Owner is
    /// open.
    pub const fn closed_vocabulary(self) -> bool {
        !matches!(self, Self::Owner)
    }

    /// The closed token vocabulary for this facet, or an empty list when open.
    fn allowed_tokens(self) -> Vec<String> {
        match self {
            Self::Attention => tokens(AttentionClass::ALL.iter().map(|a| a.as_str())),
            Self::Priority => tokens(PriorityClass::ALL.iter().map(|p| p.as_str())),
            Self::Sla => tokens(SlaState::ALL.iter().map(|s| s.as_str())),
            Self::Source => tokens(SourceClass::ALL.iter().map(|s| s.as_str())),
            Self::SyncState => tokens(SyncStateClass::ALL.iter().map(|s| s.as_str())),
            Self::Boundary => tokens(OperatorPathClass::ALL.iter().map(|p| p.as_str())),
            Self::Scope => strvec(&["local_private", "shared_team", "managed_org"]),
            Self::State => tokens(OperatorStateClass::ALL.iter().map(|s| s.as_str())),
            Self::Freshness => tokens(FreshnessClass::ALL.iter().map(|f| f.as_str())),
            Self::Owner => Vec::new(),
            Self::ObjectKind => tokens(ObjectKind::ALL.iter().map(|k| k.as_str())),
            Self::BlockerWaiver => tokens(BlockerWaiverClass::ALL.iter().map(|b| b.as_str())),
        }
    }
}

/// The grouping a saved view applies. Grouping is part of the product contract:
/// a triage inbox always groups, it never flattens into a chronological feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKeyClass {
    /// Group rows by the canonical object they reference.
    Object,
    /// Group rows by computed effective-state severity.
    Severity,
    /// Group rows by owner.
    Owner,
    /// Group rows by source.
    Source,
}

impl GroupKeyClass {
    /// All group keys, in vocabulary order.
    pub const ALL: [Self; 4] = [Self::Object, Self::Severity, Self::Owner, Self::Source];

    /// Stable snake_case token for this group key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::Severity => "severity",
            Self::Owner => "owner",
            Self::Source => "source",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Object => "Object",
            Self::Severity => "Severity",
            Self::Owner => "Owner",
            Self::Source => "Source",
        }
    }
}

/// The orders a saved view can sort within each group by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderKeyClass {
    /// Order by priority.
    Priority,
    /// Order by SLA urgency.
    SlaUrgency,
    /// Order by computed effective-state severity.
    EffectiveStateSeverity,
    /// Order by evidence freshness age.
    Freshness,
    /// Order by the row's explicit rank.
    ExplicitRank,
    /// Order by owner.
    Owner,
}

impl OrderKeyClass {
    /// All order keys, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::Priority,
        Self::SlaUrgency,
        Self::EffectiveStateSeverity,
        Self::Freshness,
        Self::ExplicitRank,
        Self::Owner,
    ];

    /// Stable snake_case token for this order key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Priority => "priority",
            Self::SlaUrgency => "sla_urgency",
            Self::EffectiveStateSeverity => "effective_state_severity",
            Self::Freshness => "freshness",
            Self::ExplicitRank => "explicit_rank",
            Self::Owner => "owner",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Priority => "Priority",
            Self::SlaUrgency => "SLA urgency",
            Self::EffectiveStateSeverity => "Effective-state severity",
            Self::Freshness => "Freshness",
            Self::ExplicitRank => "Explicit rank",
            Self::Owner => "Owner",
        }
    }
}

/// The actions a triage inbox exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxActionClass {
    /// Open the canonical detail object behind a row.
    OpenDetail,
    /// Add the surviving rows to a batch-review set, each resolving to its object.
    BatchReview,
    /// Hand off the current view as a frozen, machine-readable bundle.
    Handoff,
    /// Export the current view as a frozen, machine-readable snapshot.
    ExportView,
    /// Save the current filters, grouping, and order as a named view.
    SaveView,
    /// Apply a saved view.
    ApplyView,
    /// Adjust the live filters without saving.
    Filter,
}

impl InboxActionClass {
    /// Stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::BatchReview => "batch_review",
            Self::Handoff => "handoff",
            Self::ExportView => "export_view",
            Self::SaveView => "save_view",
            Self::ApplyView => "apply_view",
            Self::Filter => "filter",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenDetail => "Open detail",
            Self::BatchReview => "Batch review",
            Self::Handoff => "Hand off",
            Self::ExportView => "Export view",
            Self::SaveView => "Save view",
            Self::ApplyView => "Apply view",
            Self::Filter => "Filter",
        }
    }

    /// Whether the action resolves to canonical detail objects rather than only
    /// rearranging the inbox's own view state.
    pub const fn routes_to_canonical_object(self) -> bool {
        matches!(self, Self::OpenDetail | Self::BatchReview | Self::Handoff)
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One filter facet in the shared filter vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageFacet {
    /// The facet.
    pub facet: TriageFacetClass,
    /// Stable token (equals `facet.as_str()`).
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the facet's values come from a closed token vocabulary.
    pub closed_vocabulary: bool,
    /// The closed token vocabulary, or an empty list for open facets like owner.
    pub allowed_tokens: Vec<String>,
}

/// One filter clause: a facet and the values that pass it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterClause {
    /// The facet this clause filters on.
    pub facet: TriageFacetClass,
    /// The values that pass; a row passes the clause if its facet value is one of
    /// these (logical OR within a clause, AND across clauses).
    pub include_tokens: Vec<String>,
}

/// The grouping a saved view applies, with a stated reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageGrouping {
    /// The group key.
    pub key: GroupKeyClass,
    /// One reviewable sentence naming the grouping, so the inbox never groups by a
    /// hidden rule or flattens into a feed.
    pub reason: String,
}

/// The order a saved view applies within each group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageOrder {
    /// The order key.
    pub key: OrderKeyClass,
    /// Whether the order is descending.
    pub descending: bool,
    /// One reviewable sentence naming the order, so the inbox never sorts by a
    /// hidden rule.
    pub reason: String,
}

/// A named, shareable filter-grouping-and-order over a triage inbox.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedTriageView {
    /// Stable, namespaced view id.
    pub view_id: String,
    /// Stable token, unique within the inbox.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the view.
    pub summary: String,
    /// Whether the view is shared with the team or private to its owner.
    pub shared: bool,
    /// Governance scope of the view.
    pub scope: ScopeClass,
    /// The owner of the saved view.
    pub owner: String,
    /// The filter clauses, applied with AND across clauses.
    pub filters: Vec<FilterClause>,
    /// The grouping applied before ordering.
    pub group_by: TriageGrouping,
    /// The order applied within each group.
    pub order: TriageOrder,
}

/// One triage row: one item an operator works, summarizing one canonical object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageRow {
    /// Stable, inbox-namespaced presentation handle. Not the object's identity;
    /// [`TriageRow::object_ref`] is.
    pub row_id: String,
    /// The canonical object handle this row summarizes — the same ref the detail
    /// surfaces use. Never a queue-only id.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// Short title.
    pub title: String,
    /// The owner, shown first-class and never hover-only.
    pub owner: String,
    /// Who holds the decision right for this object.
    pub decision_right: String,
    /// Why this row is in the operator's queue.
    pub attention_class: AttentionClass,
    /// A written reason-for-attention; never collapsed into a bare unread badge.
    pub reason_for_attention: String,
    /// The triage priority.
    pub priority: PriorityClass,
    /// The SLA state.
    pub sla_state: SlaState,
    /// A written SLA reason; required whenever the SLA is at risk or breached.
    pub sla_reason: String,
    /// Where the row came from.
    pub source: SourceClass,
    /// The provider identity, or [`INTERNAL_PROVIDER`] when not provider-raised.
    pub provider: String,
    /// The boundary this row belongs to.
    pub boundary: OperatorPathClass,
    /// Governance scope of the object.
    pub scope: ScopeClass,
    /// Local-versus-shared/deferred sync truth.
    pub sync_state: SyncStateClass,
    /// The state the inbox would headline before the no-silent-green downgrade.
    pub displayed_state: OperatorStateClass,
    /// The canonical evidence object behind the displayed state.
    pub evidence_ref: String,
    /// The freshness age of that evidence.
    pub freshness: FreshnessClass,
    /// The blocker / waiver state of the object.
    pub blocker_waiver: BlockerWaiverClass,
    /// A visible blocker/waiver reason; required whenever something is blocked or
    /// waived, never hidden behind hover-only chrome.
    pub blocker_reason: String,
    /// The computed effective state ([`compute_effective_state`]); a stale or
    /// waived row can never be `clear`.
    pub effective_state: OperatorStateClass,
    /// The row's explicit rank within its inbox's default order.
    pub rank: u32,
    /// The open-detail route; equals [`TriageRow::object_ref`].
    pub open_detail_ref: String,
    /// Whether the row can join a live batch-review action.
    pub batch_reviewable: bool,
    /// Why the row is excluded from live batch actions; required whenever
    /// `batch_reviewable` is false, empty otherwise.
    pub batch_excluded_reason: String,
}

/// One action a triage inbox exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboxAction {
    /// The action.
    pub action: InboxActionClass,
    /// Human-readable label.
    pub label: String,
    /// Whether the action resolves to a canonical detail object.
    pub routes_to_canonical_object: bool,
    /// One reviewable sentence describing the action.
    pub summary: String,
}

/// One row of a frozen handoff bundle, preserving the truth fields outside the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedTriageRow {
    /// 1-based position in the exported order.
    pub rank_in_export: u32,
    /// The row presentation handle.
    pub row_id: String,
    /// The canonical object handle.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// Short title.
    pub title: String,
    /// The owner.
    pub owner: String,
    /// Who holds the decision right.
    pub decision_right: String,
    /// The reason-for-attention class.
    pub attention_class: AttentionClass,
    /// The written reason-for-attention, preserved verbatim.
    pub reason_for_attention: String,
    /// The priority.
    pub priority: PriorityClass,
    /// The SLA state.
    pub sla_state: SlaState,
    /// The written SLA reason, preserved verbatim.
    pub sla_reason: String,
    /// The source.
    pub source: SourceClass,
    /// The provider identity.
    pub provider: String,
    /// The boundary.
    pub boundary: OperatorPathClass,
    /// The governance scope.
    pub scope: ScopeClass,
    /// The local-versus-shared/deferred sync state.
    pub sync_state: SyncStateClass,
    /// The computed effective state.
    pub effective_state: OperatorStateClass,
    /// The evidence freshness age.
    pub freshness: FreshnessClass,
    /// The blocker / waiver state.
    pub blocker_waiver: BlockerWaiverClass,
    /// The visible blocker/waiver reason, preserved verbatim.
    pub blocker_reason: String,
    /// The open-detail route, preserved so the export still points at the object.
    pub open_detail_ref: String,
}

/// A triage inbox's saved view, frozen as a machine-readable handoff bundle.
///
/// The bundle preserves the exact filters, grouping, order, scope, ownership,
/// freshness, source, provider, priority, and SLA labels of the live inbox so the
/// truth survives outside the UI instead of flattening into a plain-text list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageHandoffBundle {
    /// Stable, namespaced bundle id.
    pub bundle_id: String,
    /// The inbox this bundle belongs to.
    pub inbox: InboxClass,
    /// The inbox's stable id.
    pub inbox_id: String,
    /// The applied view's id.
    pub applied_view_id: String,
    /// The applied view's token.
    pub applied_view_token: String,
    /// The applied filters, preserved verbatim.
    pub filters: Vec<FilterClause>,
    /// The applied grouping, preserved verbatim.
    pub group_by: TriageGrouping,
    /// The applied order, preserved verbatim.
    pub order: TriageOrder,
    /// The applied view's scope.
    pub scope: ScopeClass,
    /// The applied view's owner.
    pub view_owner: String,
    /// The redaction posture of the bundle.
    pub redaction_class: RedactionClass,
    /// Live-versus-snapshot posture; always snapshot for a frozen bundle.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// One reviewable sentence summarizing the bundle and what handing it off does.
    pub summary: String,
    /// The number of rows in the bundle.
    pub row_count: u32,
    /// The resolved, ordered rows.
    pub rows: Vec<ExportedTriageRow>,
}

/// One batch-review candidate, resolving to a row's exact canonical object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewCandidate {
    /// The row presentation handle.
    pub row_id: String,
    /// The canonical object handle the batch action resolves to.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// The reason-for-attention class.
    pub attention_class: AttentionClass,
    /// The computed effective state.
    pub effective_state: OperatorStateClass,
}

/// One row excluded from a batch action, with a stated reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewExclusion {
    /// The row presentation handle.
    pub row_id: String,
    /// The canonical object handle.
    pub object_ref: String,
    /// Why the row is excluded from the live batch action.
    pub reason: String,
}

/// What batch-reviewing the surviving set of a saved view does.
///
/// Built deterministically from the same `apply_view` projection the handoff
/// bundle uses: candidates are the surviving rows that admit a live batch action,
/// exclusions are the survivors that do not (with reasons), and the outcome states
/// what the action does. Every candidate and exclusion keeps the row's exact
/// `object_ref`, so a batch action never loses object identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewPreview {
    /// Stable, namespaced preview id.
    pub preview_id: String,
    /// The inbox this preview belongs to.
    pub inbox: InboxClass,
    /// The inbox's stable id.
    pub inbox_id: String,
    /// The applied view's token.
    pub applied_view_token: String,
    /// The number of batch-review candidates.
    pub candidate_count: u32,
    /// The number of surviving rows excluded from the live batch action.
    pub excluded_count: u32,
    /// One reviewable sentence stating what acting on the candidates does.
    pub outcome: String,
    /// Whether every candidate and exclusion preserves the row's object identity.
    pub preserves_object_identity: bool,
    /// The batch-review candidates, in the view's order.
    pub candidates: Vec<BatchReviewCandidate>,
    /// The surviving rows excluded from the live batch action, in the view's order.
    pub excluded: Vec<BatchReviewExclusion>,
}

/// One triage inbox.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageInbox {
    /// The inbox family.
    pub inbox: InboxClass,
    /// Stable, namespaced inbox id.
    pub inbox_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the inbox.
    pub summary: String,
    /// The operator-surface matrix family this inbox is an instance of.
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// Governance scope of the inbox's default view.
    pub scope: ScopeClass,
    /// The consumers that render this inbox.
    pub consumed_by: Vec<ConsumerClass>,
    /// The default redaction posture on export / handoff.
    pub default_redaction: RedactionClass,
    /// Live-versus-snapshot posture of the live inbox.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The token of the default saved view.
    pub default_view: String,
    /// The saved views over this inbox.
    pub saved_views: Vec<SavedTriageView>,
    /// The actions this inbox exposes.
    pub actions: Vec<InboxAction>,
    /// The triage rows.
    pub rows: Vec<TriageRow>,
    /// The batch-review preview of the default view.
    pub batch_review: BatchReviewPreview,
    /// The frozen handoff bundle of the default view, proving export parity.
    pub handoff: TriageHandoffBundle,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen triage-inbox set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageInboxSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_triage_inbox_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The operator-surface matrix fixture this set binds for object identity.
    pub matrix_ref: String,
    /// The matrix record kind this set binds.
    pub matrix_record_kind: String,
    /// The shared filter facets every inbox reuses.
    pub filter_facets: Vec<TriageFacet>,
    /// The group keys every inbox reuses.
    pub group_keys: Vec<TokenDef>,
    /// The order keys every inbox reuses.
    pub order_keys: Vec<TokenDef>,
    /// The attention classes rows can carry.
    pub attention_classes: Vec<TokenDef>,
    /// The priority classes rows can carry.
    pub priority_classes: Vec<TokenDef>,
    /// The SLA states rows can carry.
    pub sla_states: Vec<TokenDef>,
    /// The source classes rows can carry.
    pub source_classes: Vec<TokenDef>,
    /// The sync states rows can carry.
    pub sync_states: Vec<TokenDef>,
    /// The canonical object kinds rows point at.
    pub object_kinds: Vec<TokenDef>,
    /// The inboxes.
    pub inboxes: Vec<TriageInbox>,
    /// The computed invariants.
    pub invariants: Vec<TriageInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the triage-inbox set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriageValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for TriageValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "triage-inbox set invalid: {}", self.reason)
    }
}

impl std::error::Error for TriageValidationError {}

impl TriageInboxSet {
    /// Returns the inbox, if present.
    pub fn inbox(&self, inbox: InboxClass) -> Option<&TriageInbox> {
        self.inboxes.iter().find(|i| i.inbox == inbox)
    }

    /// Returns the filter facet, if present.
    pub fn facet(&self, facet: TriageFacetClass) -> Option<&TriageFacet> {
        self.filter_facets.iter().find(|f| f.facet == facet)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().iter().all(|r| is_export_safe_ref(r))
    }

    /// Every ref string carried by the set, for export-safety auditing.
    fn all_refs(&self) -> Vec<&str> {
        let mut refs = vec![self.matrix_ref.as_str(), self.schema_ref.as_str()];
        for inbox in &self.inboxes {
            for row in &inbox.rows {
                refs.push(row.object_ref.as_str());
                refs.push(row.evidence_ref.as_str());
                refs.push(row.open_detail_ref.as_str());
            }
            for row in &inbox.handoff.rows {
                refs.push(row.object_ref.as_str());
                refs.push(row.open_detail_ref.as_str());
            }
            for c in &inbox.batch_review.candidates {
                refs.push(c.object_ref.as_str());
            }
            for e in &inbox.batch_review.excluded {
                refs.push(e.object_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), TriageValidationError> {
        let fail = |reason: String| Err(TriageValidationError { reason });

        if self.record_kind != M5_TRIAGE_INBOX_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_TRIAGE_INBOX_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_TRIAGE_INBOX_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }

        // Every inbox is present exactly once.
        for inbox in InboxClass::ALL {
            if self.inboxes.iter().filter(|i| i.inbox == inbox).count() != 1 {
                return fail(format!("inbox {} not present exactly once", inbox.as_str()));
            }
        }

        // Ids are unique across the whole set.
        if !all_unique(self.inboxes.iter().map(|i| i.inbox_id.as_str())) {
            return fail("inbox ids are not unique".to_owned());
        }
        if !all_unique(
            self.inboxes
                .iter()
                .flat_map(|i| i.saved_views.iter().map(|v| v.view_id.as_str())),
        ) {
            return fail("view ids are not unique".to_owned());
        }
        if !all_unique(
            self.inboxes
                .iter()
                .flat_map(|i| i.rows.iter().map(|r| r.row_id.as_str())),
        ) {
            return fail("row ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

        for inbox in &self.inboxes {
            if inbox.inbox_id != inbox.inbox.inbox_id() {
                return fail(format!("inbox id mismatch for {}", inbox.inbox.as_str()));
            }
            if inbox.surface != inbox.inbox.surface()
                || inbox.surface_id != inbox.surface.surface_id()
                || matrix.surface(inbox.surface).is_none()
            {
                return fail(format!(
                    "inbox {} does not bind a canonical matrix surface",
                    inbox.inbox.as_str()
                ));
            }
            if inbox.rows.is_empty() {
                return fail(format!("inbox {} has no rows", inbox.inbox.as_str()));
            }
            if inbox.saved_views.is_empty() {
                return fail(format!("inbox {} has no saved views", inbox.inbox.as_str()));
            }
            let Some(default) = inbox
                .saved_views
                .iter()
                .find(|v| v.token == inbox.default_view)
            else {
                return fail(format!(
                    "inbox {} default view {} is not a saved view",
                    inbox.inbox.as_str(),
                    inbox.default_view
                ));
            };
            // Open-detail, batch-review, and handoff must be offered and route to
            // canonical objects.
            for required in [
                InboxActionClass::OpenDetail,
                InboxActionClass::BatchReview,
                InboxActionClass::Handoff,
            ] {
                if !inbox
                    .actions
                    .iter()
                    .any(|a| a.action == required && a.routes_to_canonical_object)
                {
                    return fail(format!(
                        "inbox {} must offer a canonical {} action",
                        inbox.inbox.as_str(),
                        required.as_str()
                    ));
                }
            }

            for row in &inbox.rows {
                if !row.object_ref.starts_with("aureline://") {
                    return fail(format!(
                        "row {} object_ref is not a canonical handle",
                        row.row_id
                    ));
                }
                if row.open_detail_ref != row.object_ref {
                    return fail(format!(
                        "row {} open-detail does not route to its object",
                        row.row_id
                    ));
                }
                if row.owner.is_empty() || row.decision_right.is_empty() {
                    return fail(format!("row {} hides owner/decision-right", row.row_id));
                }
                if row.reason_for_attention.is_empty() {
                    return fail(format!("row {} hides its reason-for-attention", row.row_id));
                }
                if row.provider.is_empty() {
                    return fail(format!("row {} hides its provider", row.row_id));
                }
                if row.source.requires_external_provider() && row.provider == INTERNAL_PROVIDER {
                    return fail(format!(
                        "row {} is provider-raised but names no external provider",
                        row.row_id
                    ));
                }
                if row.sla_state.requires_reason() && row.sla_reason.is_empty() {
                    return fail(format!(
                        "row {} hides its at-risk/breached SLA reason",
                        row.row_id
                    ));
                }
                if row.blocker_waiver.requires_reason() && row.blocker_reason.is_empty() {
                    return fail(format!(
                        "row {} is blocked/waived without a visible reason",
                        row.row_id
                    ));
                }
                let expected_batch = row.sync_state.batch_reviewable();
                if row.batch_reviewable != expected_batch {
                    return fail(format!(
                        "row {} batch_reviewable disagrees with its sync state",
                        row.row_id
                    ));
                }
                if row.batch_reviewable != row.batch_excluded_reason.is_empty() {
                    // batch_reviewable == true  <=> batch_excluded_reason empty.
                    return fail(format!(
                        "row {} batch-exclusion reason is inconsistent with batch_reviewable",
                        row.row_id
                    ));
                }
                if row.sync_state == SyncStateClass::ImportedSnapshot
                    && row.boundary != OperatorPathClass::ImportedSnapshot
                {
                    return fail(format!(
                        "row {} imported snapshot must sit on the imported boundary",
                        row.row_id
                    ));
                }
                let expected =
                    compute_effective_state(row.displayed_state, row.freshness, row.blocker_waiver);
                if row.effective_state != expected {
                    return fail(format!(
                        "row {} effective state is not the computed no-silent-green state",
                        row.row_id
                    ));
                }
            }

            // Every saved view groups, orders, and filters on defined facets.
            for view in &inbox.saved_views {
                if view.group_by.reason.is_empty() {
                    return fail(format!("view {} groups by a hidden rule", view.view_id));
                }
                if view.order.reason.is_empty() {
                    return fail(format!("view {} orders by a hidden rule", view.view_id));
                }
                for clause in &view.filters {
                    let Some(facet) = self.facet(clause.facet) else {
                        return fail(format!(
                            "view {} filters on undefined facet {}",
                            view.view_id,
                            clause.facet.as_str()
                        ));
                    };
                    if facet.closed_vocabulary {
                        for value in &clause.include_tokens {
                            if !facet.allowed_tokens.contains(value) {
                                return fail(format!(
                                    "view {} uses value {} outside facet {}",
                                    view.view_id,
                                    value,
                                    clause.facet.as_str()
                                ));
                            }
                        }
                    }
                }
            }

            // Handoff parity: the frozen bundle equals re-applying the default view.
            let recomputed_handoff = compute_handoff(
                inbox.inbox,
                &inbox.inbox_id,
                inbox.default_redaction,
                &inbox.rows,
                default,
            );
            if inbox.handoff != recomputed_handoff {
                return fail(format!(
                    "inbox {} handoff does not match its default view",
                    inbox.inbox.as_str()
                ));
            }
            // Batch-review parity.
            let recomputed_batch =
                compute_batch_review(inbox.inbox, &inbox.inbox_id, &inbox.rows, default);
            if inbox.batch_review != recomputed_batch {
                return fail(format!(
                    "inbox {} batch-review preview does not match its default view",
                    inbox.inbox.as_str()
                ));
            }
            if !inbox.batch_review.preserves_object_identity {
                return fail(format!(
                    "inbox {} batch-review loses object identity",
                    inbox.inbox.as_str()
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("triage-inbox set is not support-export safe".to_owned());
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

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
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

/// Severity rank of an operator state, higher being more urgent. Used to order
/// inboxes by effective-state severity and to group by severity.
fn state_severity_rank(state: OperatorStateClass) -> i64 {
    use OperatorStateClass::*;
    match state {
        Blocked => 100,
        FailoverInProgress => 95,
        MigrationInProgress => 90,
        BoundaryDriftRecheckRequired => 85,
        ReadOnlyWindow => 70,
        DrainWindow => 65,
        Attention => 60,
        Unconfirmed => 50,
        Reconciling => 40,
        ScheduledWindow => 35,
        EmbeddedBoundaryHandoff => 30,
        ImportedSnapshotNoLive => 20,
        UnknownRequiresReview => 15,
        Clear => 0,
    }
}

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}

// ---------------------------------------------------------------------------
// View application, handoff, and batch review.
// ---------------------------------------------------------------------------

/// The value a row presents for a filter facet.
fn row_facet_value(row: &TriageRow, facet: TriageFacetClass) -> String {
    match facet {
        TriageFacetClass::Attention => row.attention_class.as_str().to_owned(),
        TriageFacetClass::Priority => row.priority.as_str().to_owned(),
        TriageFacetClass::Sla => row.sla_state.as_str().to_owned(),
        TriageFacetClass::Source => row.source.as_str().to_owned(),
        TriageFacetClass::SyncState => row.sync_state.as_str().to_owned(),
        TriageFacetClass::Boundary => row.boundary.as_str().to_owned(),
        TriageFacetClass::Scope => scope_token(row.scope).to_owned(),
        TriageFacetClass::State => row.effective_state.as_str().to_owned(),
        TriageFacetClass::Freshness => row.freshness.as_str().to_owned(),
        TriageFacetClass::Owner => row.owner.clone(),
        TriageFacetClass::ObjectKind => row.object_kind.as_str().to_owned(),
        TriageFacetClass::BlockerWaiver => row.blocker_waiver.as_str().to_owned(),
    }
}

/// Whether a row passes every clause of a saved view (AND across clauses).
fn row_passes(row: &TriageRow, view: &SavedTriageView) -> bool {
    view.filters.iter().all(|clause| {
        clause
            .include_tokens
            .contains(&row_facet_value(row, clause.facet))
    })
}

/// Comparison of two rows by a view's group key. Grouping direction is fixed per
/// key: severity groups most-severe-first, every other key sorts ascending.
fn group_cmp(a: &TriageRow, b: &TriageRow, key: GroupKeyClass) -> std::cmp::Ordering {
    match key {
        GroupKeyClass::Object => a.object_ref.cmp(&b.object_ref),
        GroupKeyClass::Severity => {
            state_severity_rank(b.effective_state).cmp(&state_severity_rank(a.effective_state))
        }
        GroupKeyClass::Owner => a.owner.cmp(&b.owner),
        GroupKeyClass::Source => a.source.as_str().cmp(b.source.as_str()),
    }
}

/// Comparison of two rows by a view's order key, honoring the descending flag.
fn order_cmp(a: &TriageRow, b: &TriageRow, order: &TriageOrder) -> std::cmp::Ordering {
    let primary = match order.key {
        OrderKeyClass::Priority => a.priority.urgency_rank().cmp(&b.priority.urgency_rank()),
        OrderKeyClass::SlaUrgency => a.sla_state.urgency_rank().cmp(&b.sla_state.urgency_rank()),
        OrderKeyClass::EffectiveStateSeverity => {
            state_severity_rank(a.effective_state).cmp(&state_severity_rank(b.effective_state))
        }
        OrderKeyClass::Freshness => a.freshness.age_rank().cmp(&b.freshness.age_rank()),
        OrderKeyClass::ExplicitRank => a.rank.cmp(&b.rank),
        OrderKeyClass::Owner => a.owner.cmp(&b.owner),
    };
    if order.descending {
        primary.reverse()
    } else {
        primary
    }
}

/// Applies a saved view to a row set, returning the filtered, grouped, ordered
/// rows.
///
/// Deterministic: rows are filtered by the view's clauses, grouped by the view's
/// group key, ordered within each group by the view's order key, and tie-broken by
/// `row_id` so the result never depends on input order.
pub fn apply_view<'a>(rows: &'a [TriageRow], view: &SavedTriageView) -> Vec<&'a TriageRow> {
    let mut kept: Vec<&TriageRow> = rows.iter().filter(|r| row_passes(r, view)).collect();
    kept.sort_by(|a, b| {
        group_cmp(a, b, view.group_by.key)
            .then_with(|| order_cmp(a, b, &view.order))
            .then_with(|| a.row_id.cmp(&b.row_id))
    });
    kept
}

/// Builds the frozen handoff bundle of a saved view over a row set.
fn compute_handoff(
    inbox: InboxClass,
    inbox_id: &str,
    redaction_class: RedactionClass,
    rows: &[TriageRow],
    view: &SavedTriageView,
) -> TriageHandoffBundle {
    let ordered = apply_view(rows, view);
    let exported: Vec<ExportedTriageRow> = ordered
        .iter()
        .enumerate()
        .map(|(idx, row)| ExportedTriageRow {
            rank_in_export: (idx as u32) + 1,
            row_id: row.row_id.clone(),
            object_ref: row.object_ref.clone(),
            object_kind: row.object_kind,
            title: row.title.clone(),
            owner: row.owner.clone(),
            decision_right: row.decision_right.clone(),
            attention_class: row.attention_class,
            reason_for_attention: row.reason_for_attention.clone(),
            priority: row.priority,
            sla_state: row.sla_state,
            sla_reason: row.sla_reason.clone(),
            source: row.source,
            provider: row.provider.clone(),
            boundary: row.boundary,
            scope: row.scope,
            sync_state: row.sync_state,
            effective_state: row.effective_state,
            freshness: row.freshness,
            blocker_waiver: row.blocker_waiver,
            blocker_reason: row.blocker_reason.clone(),
            open_detail_ref: row.open_detail_ref.clone(),
        })
        .collect();
    let row_count = exported.len() as u32;
    TriageHandoffBundle {
        bundle_id: format!("{inbox_id}.handoff.{}", view.token),
        inbox,
        inbox_id: inbox_id.to_owned(),
        applied_view_id: view.view_id.clone(),
        applied_view_token: view.token.clone(),
        filters: view.filters.clone(),
        group_by: view.group_by.clone(),
        order: view.order.clone(),
        scope: view.scope,
        view_owner: view.owner.clone(),
        redaction_class,
        live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
        summary: format!(
            "Frozen handoff of inbox {inbox_id} via saved view '{}' — {row_count} rows; filters, \
             grouping, order, scope, ownership, freshness, source, provider, priority, and SLA \
             labels preserved as a snapshot.",
            view.token
        ),
        row_count,
        rows: exported,
    }
}

/// Builds the batch-review preview of a saved view over a row set.
fn compute_batch_review(
    inbox: InboxClass,
    inbox_id: &str,
    rows: &[TriageRow],
    view: &SavedTriageView,
) -> BatchReviewPreview {
    let ordered = apply_view(rows, view);
    let mut candidates = Vec::new();
    let mut excluded = Vec::new();
    for row in ordered {
        if row.batch_reviewable {
            candidates.push(BatchReviewCandidate {
                row_id: row.row_id.clone(),
                object_ref: row.object_ref.clone(),
                object_kind: row.object_kind,
                attention_class: row.attention_class,
                effective_state: row.effective_state,
            });
        } else {
            excluded.push(BatchReviewExclusion {
                row_id: row.row_id.clone(),
                object_ref: row.object_ref.clone(),
                reason: row.batch_excluded_reason.clone(),
            });
        }
    }
    let candidate_count = candidates.len() as u32;
    let excluded_count = excluded.len() as u32;
    BatchReviewPreview {
        preview_id: format!("{inbox_id}.batch_review.{}", view.token),
        inbox,
        inbox_id: inbox_id.to_owned(),
        applied_view_token: view.token.clone(),
        candidate_count,
        excluded_count,
        outcome: format!(
            "Batch-reviewing inbox {inbox_id} via saved view '{}' opens {candidate_count} canonical \
             objects for review and acknowledgement; {excluded_count} surviving rows are excluded \
             from the live action with stated reasons. Each candidate resolves to its exact object \
             handle, so the action never loses object identity.",
            view.token
        ),
        preserves_object_identity: true,
        candidates,
        excluded,
    }
}

/// Exports an inbox's saved view by token, recomputing the handoff bundle.
///
/// Returns `None` if the token names no saved view on the inbox.
pub fn export_triage_view(inbox: &TriageInbox, view_token: &str) -> Option<TriageHandoffBundle> {
    let view = inbox.saved_views.iter().find(|v| v.token == view_token)?;
    Some(compute_handoff(
        inbox.inbox,
        &inbox.inbox_id,
        inbox.default_redaction,
        &inbox.rows,
        view,
    ))
}

/// Previews batch-review over an inbox's saved view by token.
///
/// Returns `None` if the token names no saved view on the inbox.
pub fn batch_review_view(inbox: &TriageInbox, view_token: &str) -> Option<BatchReviewPreview> {
    let view = inbox.saved_views.iter().find(|v| v.token == view_token)?;
    Some(compute_batch_review(
        inbox.inbox,
        &inbox.inbox_id,
        &inbox.rows,
        view,
    ))
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical triage-inbox set.
///
/// Deterministic: the same bytes every call. Row effective states, each inbox's
/// default-view handoff and batch-review preview, and every invariant `holds` flag
/// are computed from the built data, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn triage_inbox_set() -> TriageInboxSet {
    let filter_facets = build_filter_facets();
    let inboxes = build_inboxes();
    let invariants = compute_invariants(&inboxes);

    TriageInboxSet {
        record_kind: M5_TRIAGE_INBOX_RECORD_KIND.to_owned(),
        m5_triage_inbox_schema_version: M5_TRIAGE_INBOX_SCHEMA_VERSION,
        schema_ref: M5_TRIAGE_INBOX_SCHEMA_REF.to_owned(),
        set_id: M5_TRIAGE_INBOX_SET_ID.to_owned(),
        as_of: M5_TRIAGE_INBOX_AS_OF.to_owned(),
        summary: "The first real Aureline operator triage inboxes — incident, support, and admin \
                  queues — as reason-bearing rows over many canonical objects. Every row names why \
                  it needs attention, whose queue it is in, its priority and SLA, where it came \
                  from, and whether its state is local, shared, deferred, or imported, with computed \
                  no-silent-green state, named grouping and order, batch-review and handoff that \
                  preserve exact object identity and queue truth, all bound to the operator-surface \
                  matrix."
            .to_owned(),
        matrix_ref: M5_TRIAGE_INBOX_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_TRIAGE_INBOX_MATRIX_RECORD_KIND.to_owned(),
        filter_facets,
        group_keys: token_defs(GroupKeyClass::ALL.iter().map(|k| (k.as_str(), k.label()))),
        order_keys: token_defs(OrderKeyClass::ALL.iter().map(|k| (k.as_str(), k.label()))),
        attention_classes: token_defs(
            AttentionClass::ALL.iter().map(|a| (a.as_str(), a.label())),
        ),
        priority_classes: token_defs(PriorityClass::ALL.iter().map(|p| (p.as_str(), p.label()))),
        sla_states: token_defs(SlaState::ALL.iter().map(|s| (s.as_str(), s.label()))),
        source_classes: token_defs(SourceClass::ALL.iter().map(|s| (s.as_str(), s.label()))),
        sync_states: token_defs(SyncStateClass::ALL.iter().map(|s| (s.as_str(), s.label()))),
        object_kinds: token_defs(ObjectKind::ALL.iter().map(|k| (k.as_str(), k.label()))),
        inboxes,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_filter_facets() -> Vec<TriageFacet> {
    TriageFacetClass::ALL
        .iter()
        .map(|facet| TriageFacet {
            facet: *facet,
            token: facet.as_str().to_owned(),
            label: facet.label().to_owned(),
            closed_vocabulary: facet.closed_vocabulary(),
            allowed_tokens: facet.allowed_tokens(),
        })
        .collect()
}

fn tokens<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<String> {
    iter.map(|s| s.to_owned()).collect()
}

fn token_defs<'a>(iter: impl Iterator<Item = (&'a str, &'a str)>) -> Vec<TokenDef> {
    iter.map(|(token, label)| TokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    })
    .collect()
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// All fields a triage row carries, before id/effective-state/batch computation.
struct RowSpec<'a> {
    n: u32,
    object_ref: &'a str,
    kind: ObjectKind,
    title: &'a str,
    owner: &'a str,
    decision_right: &'a str,
    attention: AttentionClass,
    reason_for_attention: &'a str,
    priority: PriorityClass,
    sla_state: SlaState,
    sla_reason: &'a str,
    source: SourceClass,
    provider: &'a str,
    boundary: OperatorPathClass,
    scope: ScopeClass,
    sync_state: SyncStateClass,
    displayed: OperatorStateClass,
    evidence_ref: &'a str,
    freshness: FreshnessClass,
    blocker_waiver: BlockerWaiverClass,
    blocker_reason: &'a str,
    rank: u32,
}

fn row(inbox: InboxClass, spec: RowSpec<'_>) -> TriageRow {
    let batch_reviewable = spec.sync_state.batch_reviewable();
    let batch_excluded_reason = if batch_reviewable {
        String::new()
    } else {
        "Imported snapshot has no live target; excluded from live batch actions.".to_owned()
    };
    TriageRow {
        row_id: format!("{}.row.{:04}", inbox.inbox_id(), spec.n),
        object_ref: spec.object_ref.to_owned(),
        object_kind: spec.kind,
        title: spec.title.to_owned(),
        owner: spec.owner.to_owned(),
        decision_right: spec.decision_right.to_owned(),
        attention_class: spec.attention,
        reason_for_attention: spec.reason_for_attention.to_owned(),
        priority: spec.priority,
        sla_state: spec.sla_state,
        sla_reason: spec.sla_reason.to_owned(),
        source: spec.source,
        provider: spec.provider.to_owned(),
        boundary: spec.boundary,
        scope: spec.scope,
        sync_state: spec.sync_state,
        displayed_state: spec.displayed,
        evidence_ref: spec.evidence_ref.to_owned(),
        freshness: spec.freshness,
        blocker_waiver: spec.blocker_waiver,
        blocker_reason: spec.blocker_reason.to_owned(),
        effective_state: compute_effective_state(
            spec.displayed,
            spec.freshness,
            spec.blocker_waiver,
        ),
        rank: spec.rank,
        open_detail_ref: spec.object_ref.to_owned(),
        batch_reviewable,
        batch_excluded_reason,
    }
}

fn clause(facet: TriageFacetClass, values: &[&str]) -> FilterClause {
    FilterClause {
        facet,
        include_tokens: strvec(values),
    }
}

fn grouping(key: GroupKeyClass, reason: &str) -> TriageGrouping {
    TriageGrouping {
        key,
        reason: reason.to_owned(),
    }
}

fn order(key: OrderKeyClass, descending: bool, reason: &str) -> TriageOrder {
    TriageOrder {
        key,
        descending,
        reason: reason.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn view(
    inbox: InboxClass,
    token: &str,
    label: &str,
    summary: &str,
    shared: bool,
    scope: ScopeClass,
    owner: &str,
    filters: Vec<FilterClause>,
    group_by: TriageGrouping,
    order: TriageOrder,
) -> SavedTriageView {
    SavedTriageView {
        view_id: format!("{}.view.{token}", inbox.inbox_id()),
        token: token.to_owned(),
        label: label.to_owned(),
        summary: summary.to_owned(),
        shared,
        scope,
        owner: owner.to_owned(),
        filters,
        group_by,
        order,
    }
}

fn default_actions() -> Vec<InboxAction> {
    [
        (
            InboxActionClass::OpenDetail,
            "Open the canonical incident/support/admin object behind a row.",
        ),
        (
            InboxActionClass::BatchReview,
            "Add the surviving rows to a batch-review set; each entry resolves to its canonical \
             object and excluded rows state why.",
        ),
        (
            InboxActionClass::Handoff,
            "Hand off the current view as a frozen bundle that preserves filters, grouping, order, \
             scope, ownership, source, provider, priority, and SLA labels.",
        ),
        (
            InboxActionClass::ApplyView,
            "Apply a saved view's filters, grouping, and order.",
        ),
        (
            InboxActionClass::SaveView,
            "Save the current filters, grouping, and order as a named, shareable view.",
        ),
        (
            InboxActionClass::Filter,
            "Adjust the live filters across the shared facet vocabulary.",
        ),
        (
            InboxActionClass::ExportView,
            "Export the current view as a frozen, machine-readable snapshot.",
        ),
    ]
    .into_iter()
    .map(|(action, summary)| InboxAction {
        action,
        label: action.label().to_owned(),
        routes_to_canonical_object: action.routes_to_canonical_object(),
        summary: summary.to_owned(),
    })
    .collect()
}

/// Assembles an inbox, computing its default-view handoff and batch review.
#[allow(clippy::too_many_arguments)]
fn assemble_inbox(
    inbox: InboxClass,
    summary: &str,
    scope: ScopeClass,
    consumed_by: Vec<ConsumerClass>,
    default_redaction: RedactionClass,
    default_view: &str,
    saved_views: Vec<SavedTriageView>,
    rows: Vec<TriageRow>,
) -> TriageInbox {
    let default = saved_views
        .iter()
        .find(|v| v.token == default_view)
        .expect("default view must be one of the saved views");
    let handoff = compute_handoff(inbox, &inbox.inbox_id(), default_redaction, &rows, default);
    let batch_review = compute_batch_review(inbox, &inbox.inbox_id(), &rows, default);
    TriageInbox {
        inbox,
        inbox_id: inbox.inbox_id(),
        label: inbox.label().to_owned(),
        summary: summary.to_owned(),
        surface: inbox.surface(),
        surface_id: inbox.surface().surface_id(),
        scope,
        consumed_by,
        default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        default_view: default_view.to_owned(),
        saved_views,
        actions: default_actions(),
        rows,
        batch_review,
        handoff,
    }
}

fn build_inboxes() -> Vec<TriageInbox> {
    use AttentionClass as A;
    use BlockerWaiverClass as BW;
    use ConsumerClass::*;
    use FreshnessClass as F;
    use OperatorPathClass as P;
    use OperatorStateClass as S;
    use PriorityClass as Pri;
    use SlaState as Sla;
    use SourceClass as Src;
    use SyncStateClass as Sync;

    let incident = {
        let i = InboxClass::IncidentTriage;
        let rows = vec![
            row(
                i,
                RowSpec {
                    n: 1,
                    object_ref: "aureline://incident/inc-3001",
                    kind: ObjectKind::IncidentRecord,
                    title: "Auth provider latency spike",
                    owner: "on_call_driver",
                    decision_right: "incident_commander",
                    attention: A::Assigned,
                    reason_for_attention: "Paged to your queue: managed auth latency breached the \
                                           incident SLA.",
                    priority: Pri::P0Critical,
                    sla_state: Sla::Breached,
                    sla_reason: "Breached: 18m over the P0 managed-incident SLA.",
                    source: Src::ProviderWebhook,
                    provider: "auth_provider",
                    boundary: P::Managed,
                    scope: ScopeClass::SharedTeam,
                    sync_state: Sync::SharedLive,
                    displayed: S::Attention,
                    evidence_ref: "aureline://evidence/inc-3001-alert",
                    freshness: F::Fresh,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 1,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 2,
                    object_ref: "aureline://review-item/rev-901",
                    kind: ObjectKind::ReviewItem,
                    title: "Post-incident review draft: token refresh",
                    owner: "review_lead",
                    decision_right: "review_lead",
                    attention: A::Watched,
                    reason_for_attention: "You're watching this review draft; its evidence went \
                                           stale and needs reconfirmation before sharing.",
                    priority: Pri::P2Normal,
                    sla_state: Sla::NoSla,
                    sla_reason: "",
                    source: Src::IncidentAlert,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Local,
                    scope: ScopeClass::LocalPrivate,
                    sync_state: Sync::LocalOnly,
                    displayed: S::Clear,
                    evidence_ref: "aureline://evidence/rev-901-summary",
                    freshness: F::Stale,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 4,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 3,
                    object_ref: "aureline://incident/inc-3002",
                    kind: ObjectKind::IncidentRecord,
                    title: "Managed control-plane errors",
                    owner: "on_call_driver",
                    decision_right: "incident_commander",
                    attention: A::WaitingOnApproval,
                    reason_for_attention:
                        "Waiting on a managed approval before the mitigation can \
                                           run.",
                    priority: Pri::P1High,
                    sla_state: Sla::AtRisk,
                    sla_reason: "At risk: 30m to the P1 mitigation SLA.",
                    source: Src::IncidentAlert,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Managed,
                    scope: ScopeClass::SharedTeam,
                    sync_state: Sync::SharedLive,
                    displayed: S::Attention,
                    evidence_ref: "aureline://evidence/inc-3002-alert",
                    freshness: F::Recent,
                    blocker_waiver: BW::Blocked,
                    blocker_reason:
                        "Mitigation needs a managed approval that has not been granted.",
                    rank: 2,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 4,
                    object_ref: "aureline://incident/inc-2900",
                    kind: ObjectKind::IncidentRecord,
                    title: "Imported incident replay (last quarter)",
                    owner: "on_call_driver",
                    decision_right: "incident_commander",
                    attention: A::Watched,
                    reason_for_attention:
                        "Imported replay evidence under review; read-only with no \
                                           live target.",
                    priority: Pri::P3Low,
                    sla_state: Sla::NoSla,
                    sla_reason: "",
                    source: Src::ImportedSnapshot,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::ImportedSnapshot,
                    scope: ScopeClass::SharedTeam,
                    sync_state: Sync::ImportedSnapshot,
                    displayed: S::ImportedSnapshotNoLive,
                    evidence_ref: "aureline://evidence/inc-2900-replay",
                    freshness: F::Never,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 5,
                },
            ),
        ];
        let views = vec![
            view(
                i,
                "by_severity_then_priority",
                "By severity, then priority",
                "Every open item grouped by effective-state severity; within a group, highest \
                 priority first. Stale and waived items never sort as clear.",
                true,
                ScopeClass::SharedTeam,
                "on_call_driver",
                vec![],
                grouping(
                    GroupKeyClass::Severity,
                    "Group by computed effective-state severity so blocked work sits above clear.",
                ),
                order(OrderKeyClass::Priority, true, "Highest priority first."),
            ),
            view(
                i,
                "waiting_and_blocked",
                "Waiting and blocked",
                "Only items waiting on approval or policy-blocked, grouped by owner, ordered by SLA \
                 urgency.",
                true,
                ScopeClass::SharedTeam,
                "on_call_driver",
                vec![clause(
                    TriageFacetClass::Attention,
                    &["waiting_on_approval", "policy_blocked"],
                )],
                grouping(GroupKeyClass::Owner, "Group by owner so each driver sees their queue."),
                order(OrderKeyClass::SlaUrgency, true, "Most SLA-urgent first."),
            ),
        ];
        assemble_inbox(
            i,
            "Open incidents and post-incident review items, triaged against the canonical incident \
             objects the incident workspace owns.",
            ScopeClass::SharedTeam,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport, CompanionBrowser],
            RedactionClass::MetadataSafeDefault,
            "by_severity_then_priority",
            views,
            rows,
        )
    };

    let support = {
        let i = InboxClass::SupportTriage;
        let rows = vec![
            row(
                i,
                RowSpec {
                    n: 1,
                    object_ref: "aureline://support-case/case-8801",
                    kind: ObjectKind::SupportCase,
                    title: "Export bundle stuck preparing",
                    owner: "support_lead",
                    decision_right: "support_lead",
                    attention: A::Assigned,
                    reason_for_attention: "Assigned to you: a customer export bundle has not \
                                           progressed.",
                    priority: Pri::P1High,
                    sla_state: Sla::WithinSla,
                    sla_reason: "",
                    source: Src::SupportIntake,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Local,
                    scope: ScopeClass::SharedTeam,
                    sync_state: Sync::SharedLive,
                    displayed: S::Attention,
                    evidence_ref: "aureline://evidence/case-8801-trace",
                    freshness: F::Recent,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 2,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 2,
                    object_ref: "aureline://support-case/case-8802",
                    kind: ObjectKind::SupportCase,
                    title: "Cannot reach managed control plane",
                    owner: "support_lead",
                    decision_right: "support_lead",
                    attention: A::PolicyBlocked,
                    reason_for_attention:
                        "Policy-blocked: a boundary recheck is required before a \
                                           managed reply.",
                    priority: Pri::P1High,
                    sla_state: Sla::Breached,
                    sla_reason: "Breached: 1h over the managed-support SLA.",
                    source: Src::ProviderWebhook,
                    provider: "managed_control_plane",
                    boundary: P::Managed,
                    scope: ScopeClass::SharedTeam,
                    sync_state: Sync::SharedLive,
                    displayed: S::Attention,
                    evidence_ref: "aureline://evidence/case-8802-route",
                    freshness: F::Fresh,
                    blocker_waiver: BW::Blocked,
                    blocker_reason: "Boundary drift detected; recheck required before a managed \
                                     reply.",
                    rank: 1,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 3,
                    object_ref: "aureline://support-case/case-8810",
                    kind: ObjectKind::SupportCase,
                    title: "Redaction policy question (offline draft)",
                    owner: "support_triage",
                    decision_right: "support_lead",
                    attention: A::LocallyDeferred,
                    reason_for_attention: "Captured offline in the companion; queued to publish \
                                           later when you reconnect.",
                    priority: Pri::P3Low,
                    sla_state: Sla::PausedInWindow,
                    sla_reason: "",
                    source: Src::CompanionCapture,
                    provider: "companion_browser",
                    boundary: P::BrowserWebview,
                    scope: ScopeClass::LocalPrivate,
                    sync_state: Sync::DeferredPublishLater,
                    displayed: S::Clear,
                    evidence_ref: "aureline://evidence/case-8810-note",
                    freshness: F::Fresh,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 3,
                },
            ),
        ];
        let views = vec![
            view(
                i,
                "by_source_then_sla",
                "By source, then SLA",
                "Every open case grouped by source so provider-raised cases stay distinct from \
                 intake; within a group, most SLA-urgent first.",
                true,
                ScopeClass::SharedTeam,
                "support_lead",
                vec![],
                grouping(
                    GroupKeyClass::Source,
                    "Group by source so a provider webhook never hides among intake cases.",
                ),
                order(OrderKeyClass::SlaUrgency, true, "Most SLA-urgent first."),
            ),
            view(
                i,
                "breaching_only",
                "Breaching only",
                "Cases whose SLA is breached or at risk, grouped by severity, ordered by SLA \
                 urgency.",
                true,
                ScopeClass::SharedTeam,
                "support_lead",
                vec![clause(TriageFacetClass::Sla, &["breached", "at_risk"])],
                grouping(
                    GroupKeyClass::Severity,
                    "Group by severity so blocked breaches sit on top.",
                ),
                order(OrderKeyClass::SlaUrgency, true, "Most SLA-urgent first."),
            ),
        ];
        assemble_inbox(
            i,
            "Open support cases, triaged against the canonical support-case objects the support \
             center owns.",
            ScopeClass::SharedTeam,
            vec![ShellUi, CliHeadless, SupportExport, CompanionBrowser],
            RedactionClass::InternalSupportRestricted,
            "by_source_then_sla",
            views,
            rows,
        )
    };

    let admin = {
        let i = InboxClass::AdminTriage;
        let rows = vec![
            row(
                i,
                RowSpec {
                    n: 1,
                    object_ref: "aureline://admin-approval/req-501",
                    kind: ObjectKind::AdminApprovalRequest,
                    title: "Grant managed deploy role",
                    owner: "org_admin",
                    decision_right: "org_admin",
                    attention: A::WaitingOnApproval,
                    reason_for_attention:
                        "Waiting on your approval: a managed deploy role request.",
                    priority: Pri::P1High,
                    sla_state: Sla::AtRisk,
                    sla_reason: "At risk: 2h to the access-request SLA.",
                    source: Src::AdminGovernance,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Managed,
                    scope: ScopeClass::ManagedOrg,
                    sync_state: Sync::SharedLive,
                    displayed: S::Attention,
                    evidence_ref: "aureline://evidence/req-501-request",
                    freshness: F::Fresh,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 1,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 2,
                    object_ref: "aureline://admin-approval/req-518",
                    kind: ObjectKind::AdminApprovalRequest,
                    title: "Quarterly access review",
                    owner: "security_admin",
                    decision_right: "security_admin",
                    attention: A::Stale,
                    reason_for_attention: "Its last attestation is very stale; reconfirm before \
                                           relying on it.",
                    priority: Pri::P2Normal,
                    sla_state: Sla::NoSla,
                    sla_reason: "",
                    source: Src::AdminGovernance,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Managed,
                    scope: ScopeClass::ManagedOrg,
                    sync_state: Sync::SharedLive,
                    displayed: S::Clear,
                    evidence_ref: "aureline://evidence/req-518-attestation",
                    freshness: F::VeryStale,
                    blocker_waiver: BW::None,
                    blocker_reason: "",
                    rank: 3,
                },
            ),
            row(
                i,
                RowSpec {
                    n: 3,
                    object_ref: "aureline://admin-approval/req-522",
                    kind: ObjectKind::AdminApprovalRequest,
                    title: "Emergency access waiver",
                    owner: "org_admin",
                    decision_right: "security_admin",
                    attention: A::Watched,
                    reason_for_attention: "Watching a waived access exception that expires \
                                           2026-07-15.",
                    priority: Pri::P2Normal,
                    sla_state: Sla::NoSla,
                    sla_reason: "",
                    source: Src::AdminGovernance,
                    provider: INTERNAL_PROVIDER,
                    boundary: P::Managed,
                    scope: ScopeClass::ManagedOrg,
                    sync_state: Sync::SharedLive,
                    displayed: S::Clear,
                    evidence_ref: "aureline://evidence/req-522-waiver",
                    freshness: F::Fresh,
                    blocker_waiver: BW::Waived,
                    blocker_reason:
                        "Access waived by security until 2026-07-15; acknowledged risk.",
                    rank: 2,
                },
            ),
        ];
        let views = vec![
            view(
                i,
                "by_owner_then_priority",
                "By owner, then priority",
                "Every pending request grouped by owner so each admin sees their queue; within a \
                 group, highest priority first.",
                true,
                ScopeClass::ManagedOrg,
                "org_admin",
                vec![],
                grouping(
                    GroupKeyClass::Owner,
                    "Group by owner so each admin sees their queue.",
                ),
                order(OrderKeyClass::Priority, true, "Highest priority first."),
            ),
            view(
                i,
                "stale_attestations",
                "Stale attestations",
                "Requests surfaced because their evidence went stale, grouped by severity, oldest \
                 evidence first.",
                false,
                ScopeClass::ManagedOrg,
                "security_admin",
                vec![clause(
                    TriageFacetClass::Freshness,
                    &["stale", "very_stale", "never"],
                )],
                grouping(
                    GroupKeyClass::Severity,
                    "Group by severity so downgraded items sit on top.",
                ),
                order(OrderKeyClass::Freshness, true, "Oldest evidence first."),
            ),
        ];
        assemble_inbox(
            i,
            "Pending governance approval requests and access reviews, triaged against the canonical \
             admin objects the admin console owns.",
            ScopeClass::ManagedOrg,
            vec![ShellUi, CliHeadless, AdminQueue, ManagedService, SupportExport],
            RedactionClass::OperatorOnlyRestricted,
            "by_owner_then_priority",
            views,
            rows,
        )
    };

    vec![incident, support, admin]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn compute_invariants(inboxes: &[TriageInbox]) -> Vec<TriageInvariant> {
    let all_rows: Vec<&TriageRow> = inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

    let canonical_object_identity = inboxes.iter().all(|i| {
        i.rows
            .iter()
            .all(|r| r.object_ref.starts_with("aureline://") && r.open_detail_ref == r.object_ref)
            && i.handoff.rows.iter().all(|r| {
                r.object_ref.starts_with("aureline://") && r.open_detail_ref == r.object_ref
            })
            && i.batch_review
                .candidates
                .iter()
                .all(|c| c.object_ref.starts_with("aureline://"))
    });

    let surface_binding = inboxes.iter().all(|i| {
        i.surface == i.inbox.surface()
            && i.surface_id == i.surface.surface_id()
            && matrix.surface(i.surface).is_some()
    });

    let reason_for_attention_present = all_rows.iter().all(|r| !r.reason_for_attention.is_empty());

    let attention_classes_distinct = AttentionClass::ALL
        .iter()
        .all(|class| all_rows.iter().any(|r| r.attention_class == *class));

    let priority_sla_present = all_rows
        .iter()
        .all(|r| !r.sla_state.requires_reason() || !r.sla_reason.is_empty());

    let source_provider_present = all_rows.iter().all(|r| {
        !r.provider.is_empty()
            && (!r.source.requires_external_provider() || r.provider != INTERNAL_PROVIDER)
    });

    let local_shared_deferred_truth = all_rows.iter().all(|r| {
        r.batch_reviewable == r.sync_state.batch_reviewable()
            && r.batch_reviewable == r.batch_excluded_reason.is_empty()
            && (r.sync_state != SyncStateClass::ImportedSnapshot
                || r.boundary == OperatorPathClass::ImportedSnapshot)
    });

    let no_silent_green = all_rows.iter().all(|r| {
        r.effective_state
            == compute_effective_state(r.displayed_state, r.freshness, r.blocker_waiver)
    });

    let owner_blocker_visible = all_rows.iter().all(|r| {
        !r.owner.is_empty()
            && !r.decision_right.is_empty()
            && (!r.blocker_waiver.requires_reason() || !r.blocker_reason.is_empty())
    });

    let saved_views_named = inboxes.iter().all(|i| {
        i.saved_views.iter().any(|v| v.token == i.default_view)
            && i.saved_views
                .iter()
                .all(|v| !v.group_by.reason.is_empty() && !v.order.reason.is_empty())
    });

    let shared_filter_vocabulary = inboxes.iter().all(|i| {
        i.saved_views.iter().all(|v| {
            v.filters.iter().all(|c| {
                let facet = TriageFacetClass::ALL.iter().find(|f| **f == c.facet);
                match facet {
                    None => false,
                    Some(facet) => {
                        !facet.closed_vocabulary()
                            || c.include_tokens
                                .iter()
                                .all(|val| facet.allowed_tokens().contains(val))
                    }
                }
            })
        })
    });

    let grouping_is_contract = inboxes
        .iter()
        .all(|i| i.saved_views.iter().all(|v| !v.group_by.reason.is_empty()));

    let batch_review_preserves_identity = inboxes.iter().all(|i| {
        i.batch_review.preserves_object_identity
            && i.batch_review.candidates.iter().all(|c| {
                i.rows
                    .iter()
                    .any(|r| r.row_id == c.row_id && r.object_ref == c.object_ref)
            })
            && i.batch_review.excluded.iter().all(|e| {
                i.rows.iter().any(|r| {
                    r.row_id == e.row_id && r.object_ref == e.object_ref && !e.reason.is_empty()
                })
            })
    });

    let handoff_preserves_truth = inboxes.iter().all(|i| {
        i.handoff.live_vs_snapshot == LiveSnapshotClass::SnapshotOnly
            && i.handoff.rows.iter().all(|er| {
                i.rows.iter().any(|r| {
                    r.row_id == er.row_id
                        && r.object_ref == er.object_ref
                        && r.source == er.source
                        && r.provider == er.provider
                        && r.freshness == er.freshness
                        && r.owner == er.owner
                        && r.priority == er.priority
                        && r.sla_state == er.sla_state
                        && r.scope == er.scope
                        && r.sync_state == er.sync_state
                        && r.blocker_reason == er.blocker_reason
                })
            })
    });

    let handoff_export_parity = inboxes.iter().all(|i| {
        let default = i.saved_views.iter().find(|v| v.token == i.default_view);
        match default {
            None => false,
            Some(default) => {
                compute_handoff(i.inbox, &i.inbox_id, i.default_redaction, &i.rows, default)
                    == i.handoff
            }
        }
    });

    let first_real_inboxes_present = InboxClass::ALL
        .iter()
        .all(|c| inboxes.iter().any(|i| i.inbox == *c));

    let stable_ids_unique = all_unique(inboxes.iter().map(|i| i.inbox_id.as_str()))
        && all_unique(
            inboxes
                .iter()
                .flat_map(|i| i.saved_views.iter().map(|v| v.view_id.as_str())),
        )
        && all_unique(
            inboxes
                .iter()
                .flat_map(|i| i.rows.iter().map(|r| r.row_id.as_str())),
        );

    vec![
        invariant(
            "triage.canonical_object_identity",
            "Every row carries an aureline:// object handle, routes open-detail to it, and keeps \
             that exact ref in handoff rows and batch-review candidates.",
            canonical_object_identity,
        ),
        invariant(
            "triage.surface_binding",
            "Every inbox binds the triage-inbox matrix surface family by the matrix's own surface \
             id.",
            surface_binding,
        ),
        invariant(
            "triage.reason_for_attention_present",
            "Every row names a written reason-for-attention instead of a bare unread badge.",
            reason_for_attention_present,
        ),
        invariant(
            "triage.attention_classes_distinct",
            "The inbox set proves all six attention classes — assigned, watched, policy-blocked, \
             stale, waiting-on-approval, and locally-deferred — without collapsing them.",
            attention_classes_distinct,
        ),
        invariant(
            "triage.priority_sla_present",
            "Every row carries a priority and an SLA state; an at-risk or breached SLA carries a \
             written reason.",
            priority_sla_present,
        ),
        invariant(
            "triage.source_provider_present",
            "Every row names a source and a provider; provider-raised rows name a concrete external \
             provider.",
            source_provider_present,
        ),
        invariant(
            "triage.local_shared_deferred_truth",
            "Every row's local-versus-shared/deferred sync state agrees with its batch-reviewability \
             and exclusion reason, and an imported snapshot sits on the imported boundary.",
            local_shared_deferred_truth,
        ),
        invariant(
            "triage.no_silent_green",
            "Every row's effective state equals the computed no-silent-green state.",
            no_silent_green,
        ),
        invariant(
            "triage.owner_blocker_visible",
            "Every row names an owner and decision right; blocked/waived rows carry a visible \
             reason.",
            owner_blocker_visible,
        ),
        invariant(
            "triage.saved_views_named",
            "Every inbox has a saved view, its default resolves, and every view names both its \
             grouping and its order.",
            saved_views_named,
        ),
        invariant(
            "triage.shared_filter_vocabulary",
            "Every filter clause references a defined facet and uses valid values on closed facets.",
            shared_filter_vocabulary,
        ),
        invariant(
            "triage.grouping_is_contract",
            "Every saved view declares a stated grouping, so the inbox never flattens into a \
             chronological feed.",
            grouping_is_contract,
        ),
        invariant(
            "triage.batch_review_preserves_identity",
            "Every batch-review candidate and exclusion resolves to a row's exact object handle, \
             and exclusions state a reason.",
            batch_review_preserves_identity,
        ),
        invariant(
            "triage.handoff_preserves_truth",
            "Every handoff bundle is a snapshot that preserves each row's source, provider, \
             freshness, ownership, priority, SLA, scope, sync state, and blocker reason.",
            handoff_preserves_truth,
        ),
        invariant(
            "triage.handoff_export_parity",
            "Each inbox's frozen handoff equals re-applying its default view.",
            handoff_export_parity,
        ),
        invariant(
            "triage.first_real_inboxes_present",
            "The incident, support, and admin triage inboxes are all present.",
            first_real_inboxes_present,
        ),
        invariant(
            "triage.stable_ids_unique",
            "Inbox ids, view ids, and row ids are unique.",
            stable_ids_unique,
        ),
    ]
}

fn invariant(id: &str, statement: &str, holds: bool) -> TriageInvariant {
    TriageInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the triage-inbox set as human-readable lines for headless / support
/// surfaces that cannot show the live UI.
pub fn triage_inbox_lines(set: &TriageInboxSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator triage inboxes — {} inboxes, {} filter facets, {} group keys, {} invariants \
         (as of {})",
        set.inboxes.len(),
        set.filter_facets.len(),
        set.group_keys.len(),
        set.invariants.len(),
        set.as_of
    ));
    lines.push(format!(
        "bound matrix: {} ({})",
        set.matrix_ref, set.matrix_record_kind
    ));
    for inbox in &set.inboxes {
        lines.push(String::new());
        lines.push(format!(
            "[{}] {} — surface {} — scope {} — default view {} — {} rows",
            inbox.inbox.as_str(),
            inbox.label,
            inbox.surface_id,
            scope_token(inbox.scope),
            inbox.default_view,
            inbox.rows.len()
        ));
        for row in &inbox.rows {
            lines.push(format!(
                "  - {} [{}] {} | {} | {} | {} | src={}/{} | {} | {} -> {}",
                row.object_ref,
                row.attention_class.as_str(),
                row.title,
                row.priority.as_str(),
                row.sla_state.as_str(),
                row.sync_state.as_str(),
                row.source.as_str(),
                row.provider,
                row.freshness.as_str(),
                row.displayed_state.as_str(),
                row.effective_state.as_str(),
            ));
        }
        lines.push(format!(
            "  batch-review: {} candidates, {} excluded — {}",
            inbox.batch_review.candidate_count,
            inbox.batch_review.excluded_count,
            inbox.batch_review.outcome
        ));
        lines.push(format!(
            "  handoff: {} rows, {} (group by {}, order by {})",
            inbox.handoff.row_count,
            inbox.handoff.live_vs_snapshot_token(),
            inbox.handoff.group_by.key.as_str(),
            inbox.handoff.order.key.as_str(),
        ));
    }
    lines.push(String::new());
    lines.push("invariants:".to_owned());
    for inv in &set.invariants {
        lines.push(format!(
            "  [{}] {} — {}",
            if inv.holds { "OK" } else { "FAIL" },
            inv.invariant_id,
            inv.statement
        ));
    }
    lines
}

impl TriageHandoffBundle {
    fn live_vs_snapshot_token(&self) -> &'static str {
        match self.live_vs_snapshot {
            LiveSnapshotClass::LiveOnly => "live_only",
            LiveSnapshotClass::SnapshotCapable => "snapshot_capable",
            LiveSnapshotClass::SnapshotOnly => "snapshot_only",
        }
    }
}
