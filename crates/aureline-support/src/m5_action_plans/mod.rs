//! M5 action-plan / checklist workspaces: ordered, attributable next-step plans
//! over the same canonical incident/support/admin objects the detail surfaces own.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the *family*
//! of an action plan — what it is, the one shared state vocabulary, and the
//! ownership/boundary fields every surface holds. The [triage inboxes](crate::m5_triage_inbox)
//! turn many canonical objects into reason-bearing rows. This lane builds the first
//! real action-plan **workspaces**: the ordered, ownership-bearing checklist an
//! operator works to turn an investigation into next steps, without ever implying
//! that checking a local item resolved an external object or executed a provider
//! mutation.
//!
//! An action plan is not a generic to-do list. The hard part is keeping local
//! progress honest about what did and did not change outside Aureline:
//!
//! 1. **Local checklist state and external mutation state are distinct.** Every
//!    [`PlanItem`] carries a [`ItemLocalState`] — the operator's own check-off —
//!    *and*, when it touches a provider-owned object, a separate
//!    [`ExternalMutationState`]. A row can be [`ItemLocalState::DoneLocal`] while
//!    its external object is still only previewed or approved. [`compute_resolves_external`]
//!    makes the rule executable: an item resolves its external object **only** when
//!    Aureline separately executed and confirmed the mutation, never because a box
//!    was checked.
//! 2. **Controlled item terms, shared with incident/runbook surfaces.** Each item
//!    is one of six [`PlanItemClass`] terms — observe, verify, mitigate, rollback,
//!    communicate, or custom — the same vocabulary the incident workspace's
//!    runbook steps use.
//! 3. **Approval/policy state is preserved.** Every item carries an
//!    [`ItemApprovalState`]; an executed-and-confirmed mutation must have held
//!    authority, and a non-authorized active state carries a written reason.
//! 4. **Ordered items with linked evidence and due/expiry.** Items are a contiguous
//!    1..n order; each carries its linked canonical evidence, a due/expiry pair, and
//!    a [`ItemTimeState`] so deadlines stay visible and exportable.
//! 5. **Explicit scope and boundary truth before share/export.** Every plan names a
//!    [`ScopeClass`] and a [`SharePosture`] (private, workspace-shared, or
//!    org-shared) and a [`PlanExportGate`] stating exactly what crosses the boundary
//!    on share/export at that scope.
//! 6. **Snapshot handoff and honest progress.** [`PlanHandoffBundle`] freezes the
//!    ordered items as a `snapshot_only` export preserving every truth field;
//!    [`PlanProgress`] reports local check-offs and confirmed external resolutions
//!    as separate counts so a checklist never reads as remote resolution.
//!
//! [`action_plan_set`] is the canonical binding: it builds the plans
//! deterministically and computes each [`PlanInvariant`]'s `holds` flag from the
//! built data, so the checked-in fixture and the replay gate freeze the contract
//! byte-for-byte. The record carries no endpoint URLs, hostnames, credentials, raw
//! provider payloads, or absolute paths — only opaque object refs, stable tokens,
//! and short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::m5_operator_boards::ObjectKind;
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorPathClass, OperatorSurfaceClass, RedactionClass,
    ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the action-plan set.
pub const M5_ACTION_PLANS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the action-plan set.
pub const M5_ACTION_PLANS_SCHEMA_REF: &str = "schemas/ops/m5-action-plans.schema.json";

/// Stable record-kind tag for the action-plan set.
pub const M5_ACTION_PLANS_RECORD_KIND: &str = "m5_action_plan_set";

/// Stable id for the canonical action-plan set.
pub const M5_ACTION_PLANS_SET_ID: &str = "m5-action-plans:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ACTION_PLANS_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for object identity.
pub const M5_ACTION_PLANS_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_ACTION_PLANS_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Plan families.
// ---------------------------------------------------------------------------

/// The first real action plans this lane proves the shared contract with.
///
/// Each plan is one ordered checklist over the next steps for a canonical
/// incident/support/admin object, bound to the [`OperatorSurfaceClass::ActionPlan`]
/// family. Adding a plan is a breaking change to the set; the tokens are frozen
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanClass {
    /// Incident response: the mitigation/inspection plan for an incident.
    IncidentResponse,
    /// Support remediation: the next-step plan for a support case.
    SupportRemediation,
    /// Admin access review: the review/attestation plan for an approval request.
    AdminAccessReview,
}

impl PlanClass {
    /// All plans, in set order.
    pub const ALL: [Self; 3] = [
        Self::IncidentResponse,
        Self::SupportRemediation,
        Self::AdminAccessReview,
    ];

    /// Stable snake_case token for this plan.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentResponse => "incident_response",
            Self::SupportRemediation => "support_remediation",
            Self::AdminAccessReview => "admin_access_review",
        }
    }

    /// Stable, namespaced plan id.
    pub fn plan_id(self) -> String {
        format!("action_plan.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IncidentResponse => "Incident response plan",
            Self::SupportRemediation => "Support remediation plan",
            Self::AdminAccessReview => "Admin access review plan",
        }
    }

    /// The operator-surface matrix family every action plan is an instance of.
    pub const fn surface(self) -> OperatorSurfaceClass {
        OperatorSurfaceClass::ActionPlan
    }
}

// ---------------------------------------------------------------------------
// Item class.
// ---------------------------------------------------------------------------

/// The controlled class of a plan item.
///
/// The first five tokens mirror the incident workspace's runbook step classes
/// (`observe`, `verify`, `mitigate`, `rollback`, `communicate`) verbatim, so an
/// action plan speaks the same vocabulary as the incident/runbook surfaces it
/// turns into next steps; [`PlanItemClass::Custom`] is the explicit escape hatch
/// for a step that does not fit the closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanItemClass {
    /// Read-only evidence gathering.
    Observe,
    /// Read-only validation of scope, health, or expected behavior.
    Verify,
    /// A protected-target mutation or mitigation.
    Mitigate,
    /// A rollback, restore, or compensating action.
    Rollback,
    /// An external or internal communication step.
    Communicate,
    /// A custom step that does not fit the closed vocabulary.
    Custom,
}

impl PlanItemClass {
    /// All item classes, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::Observe,
        Self::Verify,
        Self::Mitigate,
        Self::Rollback,
        Self::Communicate,
        Self::Custom,
    ];

    /// Stable snake_case token for this item class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Verify => "verify",
            Self::Mitigate => "mitigate",
            Self::Rollback => "rollback",
            Self::Communicate => "communicate",
            Self::Custom => "custom",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Observe => "Observe",
            Self::Verify => "Verify",
            Self::Mitigate => "Mitigate",
            Self::Rollback => "Rollback",
            Self::Communicate => "Communicate",
            Self::Custom => "Custom",
        }
    }

    /// Whether the class can change protected state, mirroring the incident
    /// workspace's mutating-step rule.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }

    /// Whether a verification step, which must link at least one evidence ref.
    pub const fn requires_evidence(self) -> bool {
        matches!(self, Self::Verify)
    }
}

// ---------------------------------------------------------------------------
// Local checklist state.
// ---------------------------------------------------------------------------

/// The operator's local check-off state for a plan item.
///
/// This is the *local* truth — what the operator did on their own checklist — and
/// is deliberately separate from the item's [`ExternalMutationState`]. A
/// [`ItemLocalState::DoneLocal`] item is checked off locally; it says nothing about
/// whether a provider-owned object changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemLocalState {
    /// Not started.
    NotStarted,
    /// In progress.
    InProgress,
    /// Checked off locally by the operator.
    DoneLocal,
    /// Skipped, with a stated reason.
    Skipped,
    /// Blocked locally, with a stated reason.
    BlockedLocal,
}

impl ItemLocalState {
    /// All local states, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::NotStarted,
        Self::InProgress,
        Self::DoneLocal,
        Self::Skipped,
        Self::BlockedLocal,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::DoneLocal => "done_local",
            Self::Skipped => "skipped",
            Self::BlockedLocal => "blocked_local",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotStarted => "Not started",
            Self::InProgress => "In progress",
            Self::DoneLocal => "Done (local)",
            Self::Skipped => "Skipped",
            Self::BlockedLocal => "Blocked (local)",
        }
    }

    /// Whether the operator has checked the item off locally.
    pub const fn checked_off_local(self) -> bool {
        matches!(self, Self::DoneLocal)
    }

    /// Whether a written local note is required to explain this state.
    pub const fn requires_note(self) -> bool {
        matches!(self, Self::Skipped | Self::BlockedLocal)
    }
}

// ---------------------------------------------------------------------------
// External linkage and mutation state.
// ---------------------------------------------------------------------------

/// The kind of external, provider-owned object a plan item touches.
///
/// [`ExternalLinkClass::None`] is the common case for observational, verification,
/// and communication steps that change nothing outside Aureline; every other
/// variant names a concrete external object that a real mutation would have to be
/// previewed, approved, and confirmed against — separately from the local
/// check-off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalLinkClass {
    /// No external object; the step is purely local.
    None,
    /// A provider-owned ticket / case.
    ProviderTicket,
    /// A deployment / release object.
    Deployment,
    /// A managed configuration object.
    ManagedConfig,
    /// An access grant / entitlement.
    AccessGrant,
    /// Some other external record.
    ExternalRecord,
}

impl ExternalLinkClass {
    /// All link classes, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::ProviderTicket,
        Self::Deployment,
        Self::ManagedConfig,
        Self::AccessGrant,
        Self::ExternalRecord,
    ];

    /// Stable snake_case token for this link class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ProviderTicket => "provider_ticket",
            Self::Deployment => "deployment",
            Self::ManagedConfig => "managed_config",
            Self::AccessGrant => "access_grant",
            Self::ExternalRecord => "external_record",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None (local only)",
            Self::ProviderTicket => "Provider ticket",
            Self::Deployment => "Deployment",
            Self::ManagedConfig => "Managed config",
            Self::AccessGrant => "Access grant",
            Self::ExternalRecord => "External record",
        }
    }

    /// Whether the item is linked to an external, provider-owned object.
    pub const fn is_external(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The state of the *external* mutation behind a plan item.
///
/// This is the remote truth — what Aureline actually did to the provider-owned
/// object — and is the heart of the no-implicit-external-resolution rule. A local
/// check-off can never advance this state; only a separately previewed, approved,
/// executed, and confirmed mutation reaches [`ExternalMutationState::ExecutedConfirmed`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalMutationState {
    /// No external mutation applies (the item is purely local).
    NotApplicable,
    /// An external mutation is identified but not started.
    NotStarted,
    /// The mutation has been previewed but not approved or executed.
    Previewed,
    /// The mutation is approved but not yet executed.
    Approved,
    /// Aureline executed the mutation and confirmed it against the provider.
    ExecutedConfirmed,
    /// The mutation was attempted and failed.
    Failed,
}

impl ExternalMutationState {
    /// All mutation states, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::NotApplicable,
        Self::NotStarted,
        Self::Previewed,
        Self::Approved,
        Self::ExecutedConfirmed,
        Self::Failed,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NotStarted => "not_started",
            Self::Previewed => "previewed",
            Self::Approved => "approved",
            Self::ExecutedConfirmed => "executed_confirmed",
            Self::Failed => "failed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotApplicable => "Not applicable (local only)",
            Self::NotStarted => "Not started",
            Self::Previewed => "Previewed",
            Self::Approved => "Approved",
            Self::ExecutedConfirmed => "Executed & confirmed",
            Self::Failed => "Failed",
        }
    }

    /// Whether this state actually resolves the linked external object. True only
    /// for a confirmed execution.
    pub const fn resolves_external(self) -> bool {
        matches!(self, Self::ExecutedConfirmed)
    }

    /// Whether an external mutation is identified and still in flight (not started,
    /// previewed, or approved) rather than confirmed, failed, or absent.
    pub const fn in_flight(self) -> bool {
        matches!(self, Self::NotStarted | Self::Previewed | Self::Approved)
    }
}

/// Computes whether a plan item resolves its linked external object.
///
/// This is the no-implicit-external-resolution rule made executable: an item
/// resolves its external object **only** when its [`ExternalMutationState`] is
/// [`ExecutedConfirmed`](ExternalMutationState::ExecutedConfirmed). The item's
/// local check-off is intentionally not an input — checking a box locally never
/// resolves a provider-owned object.
pub fn compute_resolves_external(mutation: ExternalMutationState) -> bool {
    mutation.resolves_external()
}

// ---------------------------------------------------------------------------
// Approval / policy state.
// ---------------------------------------------------------------------------

/// The approval/policy state of a plan item.
///
/// The tokens mirror the incident workspace's approval-state vocabulary so the
/// plan preserves the same approval/policy truth the runbook surfaces carry. An
/// item that reaches [`ExternalMutationState::ExecutedConfirmed`] must have held
/// authority ([`ItemApprovalState::is_authorized`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemApprovalState {
    /// The item never required approval.
    NotRequired,
    /// A current approval or grant is present.
    Current,
    /// Approval is pending.
    Pending,
    /// Approval is blocked.
    Blocked,
    /// Approval expired.
    Expired,
    /// Approval was revoked.
    Revoked,
    /// Approval was required but is missing.
    Missing,
    /// Policy forbids approval for this action.
    Forbidden,
}

impl ItemApprovalState {
    /// All approval states, in vocabulary order.
    pub const ALL: [Self; 8] = [
        Self::NotRequired,
        Self::Current,
        Self::Pending,
        Self::Blocked,
        Self::Expired,
        Self::Revoked,
        Self::Missing,
        Self::Forbidden,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Current => "current",
            Self::Pending => "pending",
            Self::Blocked => "blocked",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Missing => "missing",
            Self::Forbidden => "forbidden",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRequired => "Not required",
            Self::Current => "Current",
            Self::Pending => "Pending",
            Self::Blocked => "Blocked",
            Self::Expired => "Expired",
            Self::Revoked => "Revoked",
            Self::Missing => "Missing",
            Self::Forbidden => "Forbidden",
        }
    }

    /// Whether the state grants live authority for a mutation.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::NotRequired | Self::Current)
    }

    /// Whether a written approval reason is required for this state.
    pub const fn requires_reason(self) -> bool {
        !matches!(self, Self::NotRequired | Self::Current)
    }
}

// ---------------------------------------------------------------------------
// Due / expiry time state.
// ---------------------------------------------------------------------------

/// The deadline state of a plan item.
///
/// Held explicitly (rather than recomputed against a wall clock) so the set is
/// deterministic and the fixture freezes byte-for-byte; the builder sets the state
/// to agree with the item's `due`/`expiry` strings, and the validator re-checks the
/// agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemTimeState {
    /// No due date or expiry applies.
    NoDeadline,
    /// On track within the due window.
    OnTrack,
    /// Approaching the due date.
    DueSoon,
    /// Past the due date.
    Overdue,
    /// Past the expiry; the item is no longer valid as written.
    Expired,
}

impl ItemTimeState {
    /// All time states, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::NoDeadline,
        Self::OnTrack,
        Self::DueSoon,
        Self::Overdue,
        Self::Expired,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDeadline => "no_deadline",
            Self::OnTrack => "on_track",
            Self::DueSoon => "due_soon",
            Self::Overdue => "overdue",
            Self::Expired => "expired",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoDeadline => "No deadline",
            Self::OnTrack => "On track",
            Self::DueSoon => "Due soon",
            Self::Overdue => "Overdue",
            Self::Expired => "Expired",
        }
    }

    /// Whether this state requires a `due` timestamp.
    pub const fn requires_due(self) -> bool {
        matches!(self, Self::OnTrack | Self::DueSoon | Self::Overdue)
    }

    /// Whether this state requires an `expiry` timestamp.
    pub const fn requires_expiry(self) -> bool {
        matches!(self, Self::Expired)
    }

    /// Whether this state forbids any deadline (both `due` and `expiry` empty).
    pub const fn forbids_deadline(self) -> bool {
        matches!(self, Self::NoDeadline)
    }

    /// Whether a written reason is required for this state.
    pub const fn requires_reason(self) -> bool {
        matches!(self, Self::Overdue | Self::Expired)
    }
}

// ---------------------------------------------------------------------------
// Share posture / scope.
// ---------------------------------------------------------------------------

/// Whether a plan is private, workspace-shared, or org-shared.
///
/// Maps one-to-one onto the governance [`ScopeClass`] but names the operator-facing
/// share boundary explicitly, since the acceptance contract is stated in those
/// terms: a plan can be private, workspace-shared, or org-shared, with explicit
/// boundary truth before save/export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharePosture {
    /// Private to this host; nothing crosses a share boundary.
    Private,
    /// Shared with the workspace / team.
    WorkspaceShared,
    /// Shared org-wide under managed governance.
    OrgShared,
}

impl SharePosture {
    /// All share postures, in vocabulary order.
    pub const ALL: [Self; 3] = [Self::Private, Self::WorkspaceShared, Self::OrgShared];

    /// Stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::WorkspaceShared => "workspace_shared",
            Self::OrgShared => "org_shared",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Private => "Private",
            Self::WorkspaceShared => "Workspace-shared",
            Self::OrgShared => "Org-shared",
        }
    }

    /// The governance scope this posture corresponds to.
    pub const fn scope(self) -> ScopeClass {
        match self {
            Self::Private => ScopeClass::LocalPrivate,
            Self::WorkspaceShared => ScopeClass::SharedTeam,
            Self::OrgShared => ScopeClass::ManagedOrg,
        }
    }

    /// Whether sharing/exporting at this posture requires an explicit operator
    /// acknowledgement of what crosses the boundary.
    pub const fn requires_boundary_ack(self) -> bool {
        !matches!(self, Self::Private)
    }
}

// ---------------------------------------------------------------------------
// Plan actions.
// ---------------------------------------------------------------------------

/// The actions an action plan exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanActionClass {
    /// Open the canonical detail object behind an item or the plan subject.
    OpenItemDetail,
    /// Capture evidence against an item (local-safe).
    CaptureEvidence,
    /// Draft a note on an item (local-safe).
    DraftNote,
    /// Check an item off locally (local-safe; never mutates an external object).
    MarkItemDoneLocal,
    /// Preview the external mutation an item would make (no side effect).
    PreviewMutation,
    /// Request approval for an item's external mutation.
    RequestApproval,
    /// Export the plan as a frozen, machine-readable snapshot (local-safe).
    ExportPlanSnapshot,
    /// Share the plan at its scope, with explicit boundary truth.
    SharePlan,
}

impl PlanActionClass {
    /// All actions, in vocabulary order.
    pub const ALL: [Self; 8] = [
        Self::OpenItemDetail,
        Self::CaptureEvidence,
        Self::DraftNote,
        Self::MarkItemDoneLocal,
        Self::PreviewMutation,
        Self::RequestApproval,
        Self::ExportPlanSnapshot,
        Self::SharePlan,
    ];

    /// Stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenItemDetail => "open_item_detail",
            Self::CaptureEvidence => "capture_evidence",
            Self::DraftNote => "draft_note",
            Self::MarkItemDoneLocal => "mark_item_done_local",
            Self::PreviewMutation => "preview_mutation",
            Self::RequestApproval => "request_approval",
            Self::ExportPlanSnapshot => "export_plan_snapshot",
            Self::SharePlan => "share_plan",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenItemDetail => "Open detail",
            Self::CaptureEvidence => "Capture evidence",
            Self::DraftNote => "Draft note",
            Self::MarkItemDoneLocal => "Mark done (local)",
            Self::PreviewMutation => "Preview mutation",
            Self::RequestApproval => "Request approval",
            Self::ExportPlanSnapshot => "Export snapshot",
            Self::SharePlan => "Share plan",
        }
    }

    /// Whether the action is local-safe: it never mutates or resolves an external
    /// provider-owned object.
    pub const fn local_safe(self) -> bool {
        matches!(
            self,
            Self::OpenItemDetail
                | Self::CaptureEvidence
                | Self::DraftNote
                | Self::MarkItemDoneLocal
                | Self::PreviewMutation
                | Self::ExportPlanSnapshot
        )
    }

    /// Whether the action resolves to canonical detail objects rather than only
    /// rearranging the plan's own state.
    pub const fn routes_to_canonical_object(self) -> bool {
        matches!(self, Self::OpenItemDetail)
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One ordered item in an action plan.
///
/// The item separates its *local* check-off ([`PlanItem::local_state`]) from the
/// *external* mutation behind it ([`PlanItem::external_mutation_state`]): the local
/// state is the operator's own progress, the external state is the only thing that
/// can resolve a provider-owned object, and [`PlanItem::resolves_external_object`]
/// is the computed bridge that is true only for a confirmed execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanItem {
    /// Stable, plan-namespaced item id.
    pub item_id: String,
    /// 1-based position in the plan's order.
    pub ordinal: u32,
    /// Short title.
    pub title: String,
    /// The controlled item class.
    pub item_class: PlanItemClass,
    /// A written step intent; never empty.
    pub intent: String,
    /// The owning role for this item.
    pub owner: String,
    /// Who holds the decision right for this item.
    pub decision_right: String,
    /// The operator's local check-off state.
    pub local_state: ItemLocalState,
    /// A written local note; required when the item is skipped or blocked locally.
    pub local_note: String,
    /// The kind of external object this item touches.
    pub external_link: ExternalLinkClass,
    /// The canonical handle of the external object, or empty when the item is local.
    pub external_object_ref: String,
    /// The state of the external mutation behind this item.
    pub external_mutation_state: ExternalMutationState,
    /// A written note describing the mutation; required whenever the item is linked
    /// to an external object, empty otherwise.
    pub mutation_note: String,
    /// Whether this item actually resolves its external object
    /// ([`compute_resolves_external`]); true only for a confirmed execution, never
    /// from a local check-off.
    pub resolves_external_object: bool,
    /// The approval/policy state of this item.
    pub approval_state: ItemApprovalState,
    /// A written approval reason; required when the approval state is non-authorized.
    pub approval_reason: String,
    /// The deadline state of this item.
    pub time_state: ItemTimeState,
    /// The due timestamp, or empty when no due date applies.
    pub due: String,
    /// The expiry timestamp, or empty when no expiry applies.
    pub expiry: String,
    /// A written reason for an overdue or expired item; empty otherwise.
    pub time_reason: String,
    /// The boundary this item's action sits on.
    pub boundary: OperatorPathClass,
    /// The canonical evidence handles linked to this item.
    pub linked_evidence: Vec<String>,
}

/// The explicit boundary truth a plan states before it is saved or shared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanExportGate {
    /// The governance scope of the plan.
    pub scope: ScopeClass,
    /// The operator-facing share posture.
    pub share_posture: SharePosture,
    /// Whether sharing/exporting requires an explicit operator acknowledgement.
    pub requires_boundary_ack: bool,
    /// One reviewable sentence naming exactly what crosses the boundary on
    /// share/export at this scope.
    pub crosses_on_share: String,
    /// The redaction posture on share/export.
    pub redaction_class: RedactionClass,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// One action an action plan exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanAction {
    /// The action.
    pub action: PlanActionClass,
    /// Human-readable label.
    pub label: String,
    /// Whether the action is local-safe (never mutates an external object).
    pub local_safe: bool,
    /// Whether the action resolves to a canonical detail object.
    pub routes_to_canonical_object: bool,
    /// One reviewable sentence describing the action.
    pub summary: String,
}

/// The computed progress of an action plan.
///
/// Local check-offs and confirmed external resolutions are reported as separate
/// counts on purpose: `done_local` is what the operator checked off, while
/// `externally_resolved` is what Aureline actually executed and confirmed. The two
/// are never merged, so a checklist can never read as remote resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanProgress {
    /// Total items in the plan.
    pub total_items: u32,
    /// Items not started.
    pub not_started: u32,
    /// Items in progress.
    pub in_progress: u32,
    /// Items checked off locally.
    pub done_local: u32,
    /// Items skipped.
    pub skipped: u32,
    /// Items blocked locally.
    pub blocked_local: u32,
    /// Items linked to an external object.
    pub external_linked: u32,
    /// Items whose external object was actually executed and confirmed.
    pub externally_resolved: u32,
    /// Items with an external mutation still in flight (not started, previewed, or
    /// approved).
    pub mutations_in_flight: u32,
    /// Items whose external mutation failed.
    pub mutations_failed: u32,
    /// Items that are overdue.
    pub overdue: u32,
    /// Items that are expired.
    pub expired: u32,
    /// One reviewable sentence that reports local check-offs and confirmed external
    /// resolutions separately, never conflating them.
    pub headline: String,
}

/// An action plan's ordered items, frozen as a machine-readable handoff bundle.
///
/// The bundle preserves the exact ordered items, their local and external states,
/// approvals, evidence, due/expiry, scope, ownership, and boundary truth of the
/// live plan so the truth survives outside the UI instead of flattening into a
/// plain-text list. It is always `snapshot_only`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanHandoffBundle {
    /// Stable, namespaced bundle id.
    pub bundle_id: String,
    /// The plan this bundle belongs to.
    pub plan: PlanClass,
    /// The plan's stable id.
    pub plan_id: String,
    /// The canonical object the plan addresses.
    pub subject_object_ref: String,
    /// The plan's governance scope.
    pub scope: ScopeClass,
    /// The plan's share posture.
    pub share_posture: SharePosture,
    /// The plan owner.
    pub plan_owner: String,
    /// The redaction posture of the bundle.
    pub redaction_class: RedactionClass,
    /// Live-versus-snapshot posture; always snapshot for a frozen bundle.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// What crosses the boundary on share/export, preserved verbatim.
    pub crosses_on_share: String,
    /// One reviewable sentence summarizing the bundle and what handing it off does.
    pub summary: String,
    /// The number of items in the bundle.
    pub item_count: u32,
    /// The resolved, ordered items.
    pub items: Vec<PlanItem>,
    /// The computed progress, preserved in the snapshot.
    pub progress: PlanProgress,
}

/// One action plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionPlan {
    /// The plan family.
    pub plan: PlanClass,
    /// Stable, namespaced plan id.
    pub plan_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the plan.
    pub summary: String,
    /// The operator-surface matrix family this plan is an instance of.
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// The canonical object this plan turns into next steps.
    pub subject_object_ref: String,
    /// The kind of canonical subject object.
    pub subject_object_kind: ObjectKind,
    /// The owning role for the whole plan.
    pub owning_role: String,
    /// Who holds the decision right for the plan.
    pub decision_right: String,
    /// The plan's governance scope.
    pub scope: ScopeClass,
    /// The plan's operator-facing share posture.
    pub share_posture: SharePosture,
    /// The consumers that render this plan.
    pub consumed_by: Vec<ConsumerClass>,
    /// The default redaction posture on export / handoff.
    pub default_redaction: RedactionClass,
    /// Live-versus-snapshot posture of the live plan.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// One reviewable sentence stating the plan's boundary honesty.
    pub boundary_note: String,
    /// The explicit boundary truth stated before save/share/export.
    pub export_gate: PlanExportGate,
    /// The actions this plan exposes.
    pub actions: Vec<PlanAction>,
    /// The ordered items.
    pub items: Vec<PlanItem>,
    /// The computed progress.
    pub progress: PlanProgress,
    /// The frozen handoff bundle of the plan, proving export parity.
    pub handoff: PlanHandoffBundle,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen action-plan set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionPlanSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_action_plans_schema_version: u32,
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
    /// The item classes items can carry.
    pub item_classes: Vec<TokenDef>,
    /// The local states items can carry.
    pub local_states: Vec<TokenDef>,
    /// The external link classes items can carry.
    pub external_link_classes: Vec<TokenDef>,
    /// The external mutation states items can carry.
    pub external_mutation_states: Vec<TokenDef>,
    /// The approval states items can carry.
    pub approval_states: Vec<TokenDef>,
    /// The time states items can carry.
    pub time_states: Vec<TokenDef>,
    /// The share postures plans can carry.
    pub share_postures: Vec<TokenDef>,
    /// The canonical object kinds plans address.
    pub object_kinds: Vec<TokenDef>,
    /// The plans.
    pub plans: Vec<ActionPlan>,
    /// The computed invariants.
    pub invariants: Vec<PlanInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the action-plan set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for PlanValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "action-plan set invalid: {}", self.reason)
    }
}

impl std::error::Error for PlanValidationError {}

impl ActionPlanSet {
    /// Returns the plan, if present.
    pub fn plan(&self, plan: PlanClass) -> Option<&ActionPlan> {
        self.plans.iter().find(|p| p.plan == plan)
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
        for plan in &self.plans {
            refs.push(plan.subject_object_ref.as_str());
            for item in &plan.items {
                if !item.external_object_ref.is_empty() {
                    refs.push(item.external_object_ref.as_str());
                }
                for ev in &item.linked_evidence {
                    refs.push(ev.as_str());
                }
            }
            refs.push(plan.handoff.subject_object_ref.as_str());
            for item in &plan.handoff.items {
                if !item.external_object_ref.is_empty() {
                    refs.push(item.external_object_ref.as_str());
                }
                for ev in &item.linked_evidence {
                    refs.push(ev.as_str());
                }
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), PlanValidationError> {
        let fail = |reason: String| Err(PlanValidationError { reason });

        if self.record_kind != M5_ACTION_PLANS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ACTION_PLANS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_ACTION_PLANS_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }

        // Every plan is present exactly once.
        for plan in PlanClass::ALL {
            if self.plans.iter().filter(|p| p.plan == plan).count() != 1 {
                return fail(format!("plan {} not present exactly once", plan.as_str()));
            }
        }

        // Ids are unique across the whole set.
        if !all_unique(self.plans.iter().map(|p| p.plan_id.as_str())) {
            return fail("plan ids are not unique".to_owned());
        }
        if !all_unique(
            self.plans
                .iter()
                .flat_map(|p| p.items.iter().map(|i| i.item_id.as_str())),
        ) {
            return fail("item ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

        for plan in &self.plans {
            if plan.plan_id != plan.plan.plan_id() {
                return fail(format!("plan id mismatch for {}", plan.plan.as_str()));
            }
            if plan.surface != plan.plan.surface()
                || plan.surface_id != plan.surface.surface_id()
                || matrix.surface(plan.surface).is_none()
            {
                return fail(format!(
                    "plan {} does not bind a canonical matrix surface",
                    plan.plan.as_str()
                ));
            }
            if !plan.subject_object_ref.starts_with("aureline://") {
                return fail(format!(
                    "plan {} subject is not a canonical handle",
                    plan.plan.as_str()
                ));
            }
            if plan.owning_role.is_empty() || plan.decision_right.is_empty() {
                return fail(format!(
                    "plan {} hides owner/decision-right",
                    plan.plan.as_str()
                ));
            }
            if plan.items.is_empty() {
                return fail(format!("plan {} has no items", plan.plan.as_str()));
            }
            // Scope / share-posture / export-gate boundary truth.
            if plan.share_posture.scope() != plan.scope {
                return fail(format!(
                    "plan {} share posture disagrees with its scope",
                    plan.plan.as_str()
                ));
            }
            validate_export_gate(plan).map_err(|reason| PlanValidationError { reason })?;
            // Required actions are offered.
            for required in [
                PlanActionClass::OpenItemDetail,
                PlanActionClass::PreviewMutation,
                PlanActionClass::RequestApproval,
                PlanActionClass::ExportPlanSnapshot,
            ] {
                if !plan.actions.iter().any(|a| a.action == required) {
                    return fail(format!(
                        "plan {} must offer the {} action",
                        plan.plan.as_str(),
                        required.as_str()
                    ));
                }
            }

            // Items are a contiguous 1..n order.
            for (idx, item) in plan.items.iter().enumerate() {
                if item.ordinal != (idx as u32) + 1 {
                    return fail(format!(
                        "plan {} item {} is out of order",
                        plan.plan.as_str(),
                        item.item_id
                    ));
                }
                validate_item(plan.plan, item).map_err(|reason| PlanValidationError { reason })?;
            }

            // Progress parity.
            let recomputed_progress = compute_progress(&plan.items);
            if plan.progress != recomputed_progress {
                return fail(format!(
                    "plan {} progress does not match its items",
                    plan.plan.as_str()
                ));
            }
            // Handoff parity.
            let recomputed_handoff = compute_handoff(plan);
            if plan.handoff != recomputed_handoff {
                return fail(format!(
                    "plan {} handoff does not match its items",
                    plan.plan.as_str()
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("action-plan set is not support-export safe".to_owned());
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

fn validate_export_gate(plan: &ActionPlan) -> Result<(), String> {
    let gate = &plan.export_gate;
    if gate.scope != plan.scope {
        return Err(format!(
            "plan {} export gate scope mismatch",
            plan.plan.as_str()
        ));
    }
    if gate.share_posture != plan.share_posture {
        return Err(format!(
            "plan {} export gate share posture mismatch",
            plan.plan.as_str()
        ));
    }
    if gate.requires_boundary_ack != plan.share_posture.requires_boundary_ack() {
        return Err(format!(
            "plan {} export gate boundary-ack flag is inconsistent with its posture",
            plan.plan.as_str()
        ));
    }
    if gate.redaction_class != plan.default_redaction {
        return Err(format!(
            "plan {} export gate redaction mismatch",
            plan.plan.as_str()
        ));
    }
    if gate.crosses_on_share.is_empty() {
        return Err(format!(
            "plan {} export gate hides what crosses the boundary",
            plan.plan.as_str()
        ));
    }
    if !gate.raw_payload_excluded {
        return Err(format!(
            "plan {} export gate must exclude raw payloads",
            plan.plan.as_str()
        ));
    }
    Ok(())
}

fn validate_item(plan: PlanClass, item: &PlanItem) -> Result<(), String> {
    let where_ = || format!("plan {} item {}", plan.as_str(), item.item_id);
    if item.title.is_empty() || item.intent.is_empty() {
        return Err(format!("{} hides its title/intent", where_()));
    }
    if item.owner.is_empty() || item.decision_right.is_empty() {
        return Err(format!("{} hides owner/decision-right", where_()));
    }
    // Local-note requirement.
    if item.local_state.requires_note() && item.local_note.is_empty() {
        return Err(format!(
            "{} is skipped/blocked without a local note",
            where_()
        ));
    }
    // External-linkage consistency.
    if item.external_link.is_external() {
        if !item.external_object_ref.starts_with("aureline://") {
            return Err(format!(
                "{} external link names no canonical object",
                where_()
            ));
        }
        if item.external_mutation_state == ExternalMutationState::NotApplicable {
            return Err(format!(
                "{} is externally linked but its mutation state is not_applicable",
                where_()
            ));
        }
        if item.mutation_note.is_empty() {
            return Err(format!("{} hides its external-mutation note", where_()));
        }
    } else {
        if !item.external_object_ref.is_empty() {
            return Err(format!(
                "{} is local-only but names an external object",
                where_()
            ));
        }
        if item.external_mutation_state != ExternalMutationState::NotApplicable {
            return Err(format!(
                "{} is local-only but carries an external mutation state",
                where_()
            ));
        }
        if !item.mutation_note.is_empty() {
            return Err(format!(
                "{} is local-only but carries a mutation note",
                where_()
            ));
        }
    }
    // The no-implicit-external-resolution rule.
    if item.resolves_external_object != compute_resolves_external(item.external_mutation_state) {
        return Err(format!(
            "{} resolves-external flag is not the computed value",
            where_()
        ));
    }
    // A confirmed external mutation must have held authority.
    if item.external_mutation_state == ExternalMutationState::ExecutedConfirmed
        && !item.approval_state.is_authorized()
    {
        return Err(format!(
            "{} reached a confirmed mutation without holding approval authority",
            where_()
        ));
    }
    // Approval reason requirement.
    if item.approval_state.requires_reason() && item.approval_reason.is_empty() {
        return Err(format!("{} hides its approval reason", where_()));
    }
    // Time-state consistency.
    if item.time_state.forbids_deadline() && (!item.due.is_empty() || !item.expiry.is_empty()) {
        return Err(format!("{} claims no deadline but carries one", where_()));
    }
    if item.time_state.requires_due() && item.due.is_empty() {
        return Err(format!(
            "{} has a due-based state but no due date",
            where_()
        ));
    }
    if item.time_state.requires_expiry() && item.expiry.is_empty() {
        return Err(format!("{} is expired but carries no expiry", where_()));
    }
    if item.time_state.requires_reason() && item.time_reason.is_empty() {
        return Err(format!("{} is overdue/expired without a reason", where_()));
    }
    // Evidence refs are canonical; verify steps must link evidence.
    for ev in &item.linked_evidence {
        if !ev.starts_with("aureline://") {
            return Err(format!("{} links a non-canonical evidence ref", where_()));
        }
    }
    if item.item_class.requires_evidence() && item.linked_evidence.is_empty() {
        return Err(format!(
            "{} is a verify step with no linked evidence",
            where_()
        ));
    }
    Ok(())
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

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}

// ---------------------------------------------------------------------------
// Progress and handoff computation.
// ---------------------------------------------------------------------------

/// Builds the computed progress of an ordered item set.
///
/// Local check-offs and confirmed external resolutions are counted separately and
/// reported in a headline that never conflates them.
pub fn compute_progress(items: &[PlanItem]) -> PlanProgress {
    let count = |pred: &dyn Fn(&PlanItem) -> bool| items.iter().filter(|i| pred(i)).count() as u32;

    let total_items = items.len() as u32;
    let not_started = count(&|i| i.local_state == ItemLocalState::NotStarted);
    let in_progress = count(&|i| i.local_state == ItemLocalState::InProgress);
    let done_local = count(&|i| i.local_state == ItemLocalState::DoneLocal);
    let skipped = count(&|i| i.local_state == ItemLocalState::Skipped);
    let blocked_local = count(&|i| i.local_state == ItemLocalState::BlockedLocal);
    let external_linked = count(&|i| i.external_link.is_external());
    let externally_resolved = count(&|i| i.resolves_external_object);
    let mutations_in_flight = count(&|i| i.external_mutation_state.in_flight());
    let mutations_failed = count(&|i| i.external_mutation_state == ExternalMutationState::Failed);
    let overdue = count(&|i| i.time_state == ItemTimeState::Overdue);
    let expired = count(&|i| i.time_state == ItemTimeState::Expired);

    let headline = format!(
        "{total_items} steps — {done_local} checked off locally, {externally_resolved} external \
         objects executed and confirmed, {mutations_in_flight} external mutations still pending \
         preview/approval/execution, {mutations_failed} failed, {overdue} overdue, {expired} \
         expired. A local check-off never resolves a provider-owned object."
    );

    PlanProgress {
        total_items,
        not_started,
        in_progress,
        done_local,
        skipped,
        blocked_local,
        external_linked,
        externally_resolved,
        mutations_in_flight,
        mutations_failed,
        overdue,
        expired,
        headline,
    }
}

/// Builds the frozen handoff bundle of an action plan.
fn compute_handoff(plan: &ActionPlan) -> PlanHandoffBundle {
    let items = plan.items.clone();
    let item_count = items.len() as u32;
    let progress = compute_progress(&items);
    PlanHandoffBundle {
        bundle_id: format!("{}.handoff", plan.plan_id),
        plan: plan.plan,
        plan_id: plan.plan_id.clone(),
        subject_object_ref: plan.subject_object_ref.clone(),
        scope: plan.scope,
        share_posture: plan.share_posture,
        plan_owner: plan.owning_role.clone(),
        redaction_class: plan.default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
        crosses_on_share: plan.export_gate.crosses_on_share.clone(),
        summary: format!(
            "Frozen handoff of action plan {} — {item_count} ordered steps; each step's local \
             check-off, external-mutation state, approval, evidence, due/expiry, scope, ownership, \
             and boundary preserved as a snapshot. A checked-off step is never reported as a \
             resolved external object.",
            plan.plan_id
        ),
        item_count,
        items,
        progress,
    }
}

/// Exports an action plan as a frozen handoff bundle.
pub fn export_plan(plan: &ActionPlan) -> PlanHandoffBundle {
    compute_handoff(plan)
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical action-plan set.
///
/// Deterministic: the same bytes every call. Each item's resolves-external flag,
/// each plan's progress and handoff bundle, and every invariant `holds` flag are
/// computed from the built data, so an inconsistent edit flips an invariant rather
/// than silently passing.
pub fn action_plan_set() -> ActionPlanSet {
    let plans = build_plans();
    let invariants = compute_invariants(&plans);

    ActionPlanSet {
        record_kind: M5_ACTION_PLANS_RECORD_KIND.to_owned(),
        m5_action_plans_schema_version: M5_ACTION_PLANS_SCHEMA_VERSION,
        schema_ref: M5_ACTION_PLANS_SCHEMA_REF.to_owned(),
        set_id: M5_ACTION_PLANS_SET_ID.to_owned(),
        as_of: M5_ACTION_PLANS_AS_OF.to_owned(),
        summary: "The first real Aureline operator action-plan / checklist workspaces — incident \
                  response, support remediation, and admin access review — as ordered, \
                  ownership-bearing next-step plans over canonical incident/support/admin objects. \
                  Each item keeps its local check-off distinct from any external mutation, links \
                  canonical evidence, carries approval/policy state and due/expiry, and never lets \
                  a local checkoff resolve a provider-owned object; plans declare explicit scope \
                  and boundary truth before share/export and freeze a snapshot handoff that \
                  preserves every truth field, all bound to the operator-surface matrix."
            .to_owned(),
        matrix_ref: M5_ACTION_PLANS_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_ACTION_PLANS_MATRIX_RECORD_KIND.to_owned(),
        item_classes: token_defs(PlanItemClass::ALL.iter().map(|c| (c.as_str(), c.label()))),
        local_states: token_defs(ItemLocalState::ALL.iter().map(|s| (s.as_str(), s.label()))),
        external_link_classes: token_defs(
            ExternalLinkClass::ALL
                .iter()
                .map(|c| (c.as_str(), c.label())),
        ),
        external_mutation_states: token_defs(
            ExternalMutationState::ALL
                .iter()
                .map(|s| (s.as_str(), s.label())),
        ),
        approval_states: token_defs(
            ItemApprovalState::ALL
                .iter()
                .map(|s| (s.as_str(), s.label())),
        ),
        time_states: token_defs(ItemTimeState::ALL.iter().map(|s| (s.as_str(), s.label()))),
        share_postures: token_defs(SharePosture::ALL.iter().map(|s| (s.as_str(), s.label()))),
        object_kinds: token_defs(ObjectKind::ALL.iter().map(|k| (k.as_str(), k.label()))),
        plans,
        invariants,
        raw_payload_excluded: true,
    }
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

/// All fields a plan item carries, before id/resolves-external computation.
struct ItemSpec<'a> {
    n: u32,
    title: &'a str,
    item_class: PlanItemClass,
    intent: &'a str,
    owner: &'a str,
    decision_right: &'a str,
    local_state: ItemLocalState,
    local_note: &'a str,
    external_link: ExternalLinkClass,
    external_object_ref: &'a str,
    external_mutation_state: ExternalMutationState,
    mutation_note: &'a str,
    approval_state: ItemApprovalState,
    approval_reason: &'a str,
    time_state: ItemTimeState,
    due: &'a str,
    expiry: &'a str,
    time_reason: &'a str,
    boundary: OperatorPathClass,
    evidence: &'a [&'a str],
}

fn item(plan: PlanClass, spec: ItemSpec<'_>) -> PlanItem {
    PlanItem {
        item_id: format!("{}.item.{:04}", plan.plan_id(), spec.n),
        ordinal: spec.n,
        title: spec.title.to_owned(),
        item_class: spec.item_class,
        intent: spec.intent.to_owned(),
        owner: spec.owner.to_owned(),
        decision_right: spec.decision_right.to_owned(),
        local_state: spec.local_state,
        local_note: spec.local_note.to_owned(),
        external_link: spec.external_link,
        external_object_ref: spec.external_object_ref.to_owned(),
        external_mutation_state: spec.external_mutation_state,
        mutation_note: spec.mutation_note.to_owned(),
        resolves_external_object: compute_resolves_external(spec.external_mutation_state),
        approval_state: spec.approval_state,
        approval_reason: spec.approval_reason.to_owned(),
        time_state: spec.time_state,
        due: spec.due.to_owned(),
        expiry: spec.expiry.to_owned(),
        time_reason: spec.time_reason.to_owned(),
        boundary: spec.boundary,
        linked_evidence: strvec(spec.evidence),
    }
}

fn export_gate(
    scope: ScopeClass,
    share_posture: SharePosture,
    redaction_class: RedactionClass,
    crosses_on_share: &str,
) -> PlanExportGate {
    PlanExportGate {
        scope,
        share_posture,
        requires_boundary_ack: share_posture.requires_boundary_ack(),
        crosses_on_share: crosses_on_share.to_owned(),
        redaction_class,
        raw_payload_excluded: true,
    }
}

fn default_actions() -> Vec<PlanAction> {
    [
        (
            PlanActionClass::OpenItemDetail,
            "Open the canonical incident/support/admin object behind an item or the plan subject.",
        ),
        (
            PlanActionClass::CaptureEvidence,
            "Capture canonical evidence against an item; local-safe and never mutates an external \
             object.",
        ),
        (
            PlanActionClass::DraftNote,
            "Draft a note on an item; local-safe.",
        ),
        (
            PlanActionClass::MarkItemDoneLocal,
            "Check an item off locally; local-safe and never resolves or mutates the linked \
             external object.",
        ),
        (
            PlanActionClass::PreviewMutation,
            "Preview the external mutation an item would make, with no side effect, on the separate \
             previewed/approved/confirmed path.",
        ),
        (
            PlanActionClass::RequestApproval,
            "Request approval for an item's external mutation before it can be executed.",
        ),
        (
            PlanActionClass::ExportPlanSnapshot,
            "Export the plan as a frozen, machine-readable snapshot that preserves every truth \
             field; local-safe.",
        ),
        (
            PlanActionClass::SharePlan,
            "Share the plan at its scope, after acknowledging exactly what crosses the boundary.",
        ),
    ]
    .into_iter()
    .map(|(action, summary)| PlanAction {
        action,
        label: action.label().to_owned(),
        local_safe: action.local_safe(),
        routes_to_canonical_object: action.routes_to_canonical_object(),
        summary: summary.to_owned(),
    })
    .collect()
}

/// Assembles a plan, computing its progress and frozen handoff.
#[allow(clippy::too_many_arguments)]
fn assemble_plan(
    plan: PlanClass,
    summary: &str,
    subject_object_ref: &str,
    subject_object_kind: ObjectKind,
    owning_role: &str,
    decision_right: &str,
    share_posture: SharePosture,
    consumed_by: Vec<ConsumerClass>,
    default_redaction: RedactionClass,
    boundary_note: &str,
    export_gate: PlanExportGate,
    items: Vec<PlanItem>,
) -> ActionPlan {
    let scope = share_posture.scope();
    let progress = compute_progress(&items);
    let mut built = ActionPlan {
        plan,
        plan_id: plan.plan_id(),
        label: plan.label().to_owned(),
        summary: summary.to_owned(),
        surface: plan.surface(),
        surface_id: plan.surface().surface_id(),
        subject_object_ref: subject_object_ref.to_owned(),
        subject_object_kind,
        owning_role: owning_role.to_owned(),
        decision_right: decision_right.to_owned(),
        scope,
        share_posture,
        consumed_by,
        default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        boundary_note: boundary_note.to_owned(),
        export_gate,
        actions: default_actions(),
        items,
        progress,
        // Placeholder; replaced below once the plan is otherwise complete so the
        // handoff sees the final scope/owner/gate.
        handoff: PlanHandoffBundle {
            bundle_id: String::new(),
            plan,
            plan_id: plan.plan_id(),
            subject_object_ref: subject_object_ref.to_owned(),
            scope,
            share_posture,
            plan_owner: owning_role.to_owned(),
            redaction_class: default_redaction,
            live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
            crosses_on_share: String::new(),
            summary: String::new(),
            item_count: 0,
            items: Vec::new(),
            progress: compute_progress(&[]),
        },
    };
    built.handoff = compute_handoff(&built);
    built
}

fn build_plans() -> Vec<ActionPlan> {
    use ConsumerClass::*;
    use ExternalLinkClass as Link;
    use ExternalMutationState as Mut;
    use ItemApprovalState as Appr;
    use ItemLocalState as Local;
    use ItemTimeState as Time;
    use OperatorPathClass as Path;
    use PlanItemClass as Class;

    let incident = {
        let p = PlanClass::IncidentResponse;
        let items = vec![
            item(
                p,
                ItemSpec {
                    n: 1,
                    title: "Gather auth-latency signal slice",
                    item_class: Class::Observe,
                    intent: "Collect the latency metric slice and recent error logs for the \
                             affected auth path.",
                    owner: "on_call_driver",
                    decision_right: "incident_commander",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Local,
                    evidence: &["aureline://evidence/inc-3001-latency-slice"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 2,
                    title: "Verify blast radius is one region",
                    item_class: Class::Verify,
                    intent: "Confirm the latency spike is confined to the primary region before \
                             any mitigation.",
                    owner: "on_call_driver",
                    decision_right: "incident_commander",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::OnTrack,
                    due: "2026-06-22T02:00:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/inc-3001-blast-radius"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 3,
                    title: "Apply managed connection-pool config",
                    item_class: Class::Mitigate,
                    intent: "Raise the auth connection-pool ceiling via the managed config to \
                             absorb the latency spike.",
                    owner: "platform_operator",
                    decision_right: "incident_commander",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::ManagedConfig,
                    external_object_ref: "aureline://managed-config/auth-pool-ceiling",
                    external_mutation_state: Mut::ExecutedConfirmed,
                    mutation_note: "Aureline previewed, approved, executed, and confirmed the \
                                    config change against the managed control plane; the confirmed \
                                    revision is recorded separately from this local check-off.",
                    approval_state: Appr::Current,
                    approval_reason: "",
                    time_state: Time::OnTrack,
                    due: "2026-06-22T02:15:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/inc-3001-config-confirm"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 4,
                    title: "Open provider escalation ticket",
                    item_class: Class::Mitigate,
                    intent: "Escalate the upstream auth-provider latency to the provider via a \
                             tracked ticket.",
                    owner: "platform_operator",
                    decision_right: "incident_commander",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::ProviderTicket,
                    external_object_ref: "aureline://provider-ticket/auth-esc-77",
                    external_mutation_state: Mut::Approved,
                    mutation_note:
                        "The escalation is approved but not yet filed with the provider; \
                                    checking this step off locally does not open the provider \
                                    ticket — the filing stays on the separate execute-and-confirm \
                                    path.",
                    approval_state: Appr::Current,
                    approval_reason: "",
                    time_state: Time::DueSoon,
                    due: "2026-06-22T03:00:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/inc-3001-escalation-draft"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 5,
                    title: "Stage rollback of last auth deploy",
                    item_class: Class::Rollback,
                    intent: "Prepare a rollback of the most recent auth deployment in case the \
                             config change does not hold.",
                    owner: "release_operator",
                    decision_right: "incident_commander",
                    local_state: Local::NotStarted,
                    local_note: "",
                    external_link: Link::Deployment,
                    external_object_ref: "aureline://deployment/auth-2026-06-21",
                    external_mutation_state: Mut::NotStarted,
                    mutation_note: "Rollback is staged but not started; no deployment has been \
                                    rolled back.",
                    approval_state: Appr::Pending,
                    approval_reason: "Awaiting incident-commander sign-off before any rollback.",
                    time_state: Time::Overdue,
                    due: "2026-06-22T02:30:00Z",
                    expiry: "",
                    time_reason:
                        "Overdue: rollback readiness was due 30m ago and is still pending \
                                  approval.",
                    boundary: Path::Managed,
                    evidence: &[],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 6,
                    title: "Post status update to incident channel",
                    item_class: Class::Communicate,
                    intent: "Share current state and next checkpoint with stakeholders.",
                    owner: "incident_scribe",
                    decision_right: "incident_commander",
                    local_state: Local::InProgress,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Local,
                    evidence: &[],
                },
            ),
        ];
        assemble_plan(
            p,
            "The ordered response plan for the auth-latency incident: observe and verify, apply \
             the confirmed managed config change, escalate to the provider and stage a rollback on \
             the separate execute-and-confirm path, and communicate. A checked-off step never \
             implies the provider ticket or deployment changed.",
            "aureline://incident/inc-3001",
            ObjectKind::IncidentRecord,
            "on_call_driver",
            "incident_commander",
            SharePosture::WorkspaceShared,
            vec![
                ShellUi,
                CliHeadless,
                IncidentWorkspace,
                SupportExport,
                ManagedService,
            ],
            RedactionClass::OperatorOnlyRestricted,
            "A mutating step discloses its approval admission and external-mutation state, and \
             never implies a managed apply happened just because the box is checked.",
            export_gate(
                ScopeClass::SharedTeam,
                SharePosture::WorkspaceShared,
                RedactionClass::OperatorOnlyRestricted,
                "Item titles, intents, ordered local check-off states, external-mutation states, \
                 approvals, evidence refs, due/expiry, and ownership become visible to the \
                 workspace; raw provider payloads, credentials, and endpoint URLs never cross.",
            ),
            items,
        )
    };

    let support = {
        let p = PlanClass::SupportRemediation;
        let items = vec![
            item(
                p,
                ItemSpec {
                    n: 1,
                    title: "Reproduce the reported failure locally",
                    item_class: Class::Verify,
                    intent: "Confirm the customer-reported failure reproduces against a local \
                             repro before proposing a remediation.",
                    owner: "support_engineer",
                    decision_right: "support_lead",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::OnTrack,
                    due: "2026-06-22T04:00:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Local,
                    evidence: &["aureline://evidence/case-8801-repro"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 2,
                    title: "Propose provider ticket update",
                    item_class: Class::Mitigate,
                    intent: "Draft the upstream provider-ticket update that would request the \
                             config fix.",
                    owner: "support_engineer",
                    decision_right: "support_lead",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::ProviderTicket,
                    external_object_ref: "aureline://provider-ticket/case-8801-fix",
                    external_mutation_state: Mut::Previewed,
                    mutation_note:
                        "The provider-ticket update is previewed only; checking this \
                                    step off locally does not post the update or change the \
                                    provider ticket. Posting stays on the approve-and-confirm path.",
                    approval_state: Appr::Pending,
                    approval_reason:
                        "Provider-ticket update awaits support-lead approval before it \
                                      can be posted.",
                    time_state: Time::DueSoon,
                    due: "2026-06-22T05:00:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/case-8801-ticket-preview"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 3,
                    title: "Send customer holding response",
                    item_class: Class::Communicate,
                    intent: "Acknowledge the case and set expectations with the customer.",
                    owner: "support_engineer",
                    decision_right: "support_lead",
                    local_state: Local::Skipped,
                    local_note: "Skipped here: the customer holding response was sent by the \
                                 support lead out of band.",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Local,
                    evidence: &[],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 4,
                    title: "Attempt hotfix deploy to canary",
                    item_class: Class::Mitigate,
                    intent: "Deploy the candidate hotfix to the canary ring to validate the fix.",
                    owner: "release_operator",
                    decision_right: "support_lead",
                    local_state: Local::BlockedLocal,
                    local_note: "Blocked locally: the canary deploy pipeline is in an announced \
                                 read-only window.",
                    external_link: Link::Deployment,
                    external_object_ref: "aureline://deployment/case-8801-hotfix",
                    external_mutation_state: Mut::Failed,
                    mutation_note: "The canary deploy was attempted and failed closed; the \
                                    deployment object is unchanged and the failure reason is \
                                    recorded separately.",
                    approval_state: Appr::Current,
                    approval_reason: "",
                    time_state: Time::Overdue,
                    due: "2026-06-22T05:30:00Z",
                    expiry: "",
                    time_reason: "Overdue: hotfix validation was due 1h ago and is blocked by the \
                                  read-only window.",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/case-8801-deploy-fail"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 5,
                    title: "Open follow-up tracking record",
                    item_class: Class::Custom,
                    intent: "Track the remaining remediation work in a follow-up record so it is \
                             not lost when the case closes.",
                    owner: "support_engineer",
                    decision_right: "support_lead",
                    local_state: Local::InProgress,
                    local_note: "",
                    external_link: Link::ExternalRecord,
                    external_object_ref: "aureline://external-record/case-8801-followup",
                    external_mutation_state: Mut::NotStarted,
                    mutation_note:
                        "The follow-up record is not yet created externally; this local \
                                    step does not create it.",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Remote,
                    evidence: &[],
                },
            ),
        ];
        assemble_plan(
            p,
            "The private remediation plan for a support case: reproduce locally, draft and preview \
             the provider-ticket update, and attempt a hotfix on the separate execute path. The \
             plan is a private draft until the operator changes its scope; a local check-off never \
             posts a provider ticket or changes a deployment.",
            "aureline://support-case/case-8801",
            ObjectKind::SupportCase,
            "support_engineer",
            "support_lead",
            SharePosture::Private,
            vec![ShellUi, CliHeadless, SupportExport],
            RedactionClass::PrivateTriageOnly,
            "A remediation step that touches a provider ticket or deployment discloses its \
             external-mutation state and never implies the external object changed from a local \
             check-off.",
            export_gate(
                ScopeClass::LocalPrivate,
                SharePosture::Private,
                RedactionClass::PrivateTriageOnly,
                "Stays on this host as a private draft; nothing crosses a share boundary until the \
                 operator changes scope. Export produces a local snapshot only.",
            ),
            items,
        )
    };

    let admin =
        {
            let p = PlanClass::AdminAccessReview;
            let items =
                vec![
            item(
                p,
                ItemSpec {
                    n: 1,
                    title: "Review the access request context",
                    item_class: Class::Observe,
                    intent: "Read the access request, requester, and the resource it targets.",
                    owner: "access_reviewer",
                    decision_right: "security_owner",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/req-501-context"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 2,
                    title: "Verify the requester's current attestation",
                    item_class: Class::Verify,
                    intent: "Confirm the requester's training/attestation is current before any \
                             grant.",
                    owner: "access_reviewer",
                    decision_right: "security_owner",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::NotRequired,
                    approval_reason: "",
                    time_state: Time::OnTrack,
                    due: "2026-06-22T06:00:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/req-501-attestation"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 3,
                    title: "Grant scoped access entitlement",
                    item_class: Class::Mitigate,
                    intent: "Grant the requested, time-bounded access entitlement once approved.",
                    owner: "access_reviewer",
                    decision_right: "security_owner",
                    local_state: Local::NotStarted,
                    local_note: "",
                    external_link: Link::AccessGrant,
                    external_object_ref: "aureline://access-grant/req-501-grant",
                    external_mutation_state: Mut::NotStarted,
                    mutation_note: "The grant has not been made; policy forbids the reviewer \
                                    self-approving, so no entitlement was issued.",
                    approval_state: Appr::Forbidden,
                    approval_reason: "Policy forbids self-approval; this grant requires a separate \
                                      security-owner approval.",
                    time_state: Time::Expired,
                    due: "",
                    expiry: "2026-06-21T00:00:00Z",
                    time_reason: "Expired: the original access window lapsed; a fresh request is \
                                  required before any grant.",
                    boundary: Path::Managed,
                    evidence: &[],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 4,
                    title: "Revoke the requester's stale prior grant",
                    item_class: Class::Rollback,
                    intent: "Revoke the requester's previously issued, now-stale entitlement.",
                    owner: "access_reviewer",
                    decision_right: "security_owner",
                    local_state: Local::DoneLocal,
                    local_note: "",
                    external_link: Link::AccessGrant,
                    external_object_ref: "aureline://access-grant/req-501-prior",
                    external_mutation_state: Mut::ExecutedConfirmed,
                    mutation_note: "Aureline executed and confirmed the revocation of the stale \
                                    prior grant against the managed directory; the confirmation is \
                                    recorded separately from this local check-off.",
                    approval_state: Appr::Current,
                    approval_reason: "",
                    time_state: Time::OnTrack,
                    due: "2026-06-22T06:30:00Z",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &["aureline://evidence/req-501-revoke-confirm"],
                },
            ),
            item(
                p,
                ItemSpec {
                    n: 5,
                    title: "Record the residency decision rationale",
                    item_class: Class::Custom,
                    intent: "Capture the data-residency rationale for the review decision.",
                    owner: "access_reviewer",
                    decision_right: "security_owner",
                    local_state: Local::BlockedLocal,
                    local_note: "Blocked locally pending data-residency confirmation from the \
                                 regional owner.",
                    external_link: Link::None,
                    external_object_ref: "",
                    external_mutation_state: Mut::NotApplicable,
                    mutation_note: "",
                    approval_state: Appr::Blocked,
                    approval_reason: "Approval blocked until the residency owner confirms the \
                                      region.",
                    time_state: Time::NoDeadline,
                    due: "",
                    expiry: "",
                    time_reason: "",
                    boundary: Path::Managed,
                    evidence: &[],
                },
            ),
        ];
            assemble_plan(
            p,
            "The org-shared access-review plan for an approval request: observe and verify, hold \
             the forbidden self-approval grant, confirm the revocation of a stale prior grant on \
             the separate execute path, and record the residency rationale. A local check-off \
             never issues or revokes an entitlement on its own.",
            "aureline://admin-approval/req-501",
            ObjectKind::AdminApprovalRequest,
            "access_reviewer",
            "security_owner",
            SharePosture::OrgShared,
            vec![ShellUi, CliHeadless, AdminQueue, SupportExport, ManagedService],
            RedactionClass::OperatorOnlyRestricted,
            "A grant or revoke step discloses its approval admission and external-mutation state; \
             a local check-off never issues or revokes an entitlement.",
            export_gate(
                ScopeClass::ManagedOrg,
                SharePosture::OrgShared,
                RedactionClass::OperatorOnlyRestricted,
                "Item titles, intents, local check-off states, external-mutation and approval \
                 states, evidence refs, due/expiry, and ownership become visible org-wide under \
                 managed governance; raw directory payloads, credentials, and endpoint URLs never \
                 cross.",
            ),
            items,
        )
        };

    vec![incident, support, admin]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn compute_invariants(plans: &[ActionPlan]) -> Vec<PlanInvariant> {
    let all_items: Vec<&PlanItem> = plans.iter().flat_map(|p| p.items.iter()).collect();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

    let surface_binding = plans.iter().all(|p| {
        p.surface == p.plan.surface()
            && p.surface_id == p.surface.surface_id()
            && matrix.surface(p.surface).is_some()
    });

    let canonical_object_linkage = plans.iter().all(|p| {
        p.subject_object_ref.starts_with("aureline://")
            && p.items.iter().all(|i| {
                (i.external_object_ref.is_empty()
                    || i.external_object_ref.starts_with("aureline://"))
                    && i.linked_evidence
                        .iter()
                        .all(|e| e.starts_with("aureline://"))
            })
    });

    let ordered_items = plans.iter().all(|p| {
        !p.items.is_empty()
            && p.items
                .iter()
                .enumerate()
                .all(|(idx, i)| i.ordinal == (idx as u32) + 1)
    });

    let item_intent_present = all_items
        .iter()
        .all(|i| !i.intent.is_empty() && !i.owner.is_empty() && !i.decision_right.is_empty());

    let item_classes_distinct = PlanItemClass::ALL
        .iter()
        .all(|class| all_items.iter().any(|i| i.item_class == *class));

    // The central guardrail: a local check-off never resolves an external object.
    let resolves_matches_mutation = all_items.iter().all(|i| {
        i.resolves_external_object == compute_resolves_external(i.external_mutation_state)
    });
    let local_done_never_resolves = all_items.iter().any(|i| {
        i.local_state == ItemLocalState::DoneLocal
            && i.external_link.is_external()
            && !i.resolves_external_object
    });
    let local_checkoff_never_resolves_external =
        resolves_matches_mutation && local_done_never_resolves;

    let external_mutation_linkage_explicit = all_items.iter().all(|i| {
        if i.external_link.is_external() {
            i.external_object_ref.starts_with("aureline://")
                && i.external_mutation_state != ExternalMutationState::NotApplicable
                && !i.mutation_note.is_empty()
        } else {
            i.external_object_ref.is_empty()
                && i.external_mutation_state == ExternalMutationState::NotApplicable
                && i.mutation_note.is_empty()
        }
    });

    let approval_state_preserved = all_items.iter().all(|i| {
        (!i.approval_state.requires_reason() || !i.approval_reason.is_empty())
            && (i.external_mutation_state != ExternalMutationState::ExecutedConfirmed
                || i.approval_state.is_authorized())
    });

    let evidence_linked = all_items.iter().all(|i| {
        i.linked_evidence
            .iter()
            .all(|e| e.starts_with("aureline://"))
            && (!i.item_class.requires_evidence() || !i.linked_evidence.is_empty())
    });

    let due_expiry_visible = all_items.iter().all(|i| {
        (!i.time_state.forbids_deadline() || (i.due.is_empty() && i.expiry.is_empty()))
            && (!i.time_state.requires_due() || !i.due.is_empty())
            && (!i.time_state.requires_expiry() || !i.expiry.is_empty())
            && (!i.time_state.requires_reason() || !i.time_reason.is_empty())
    });

    let local_note_visible = all_items
        .iter()
        .all(|i| !i.local_state.requires_note() || !i.local_note.is_empty());

    let scope_boundary_truth = plans.iter().all(|p| {
        p.share_posture.scope() == p.scope
            && p.export_gate.scope == p.scope
            && p.export_gate.share_posture == p.share_posture
            && p.export_gate.requires_boundary_ack == p.share_posture.requires_boundary_ack()
            && p.export_gate.redaction_class == p.default_redaction
            && !p.export_gate.crosses_on_share.is_empty()
            && p.export_gate.raw_payload_excluded
    });
    let share_postures_distinct = SharePosture::ALL
        .iter()
        .all(|posture| plans.iter().any(|p| p.share_posture == *posture));

    let handoff_export_parity = plans.iter().all(|p| {
        compute_handoff(p) == p.handoff
            && p.handoff.live_vs_snapshot == LiveSnapshotClass::SnapshotOnly
    });

    let handoff_preserves_truth = plans.iter().all(|p| {
        p.handoff.items == p.items
            && p.handoff.progress == compute_progress(&p.items)
            && p.handoff.crosses_on_share == p.export_gate.crosses_on_share
    });

    let progress_no_silent_resolution = plans.iter().all(|p| {
        let resolved = p
            .items
            .iter()
            .filter(|i| i.resolves_external_object)
            .count() as u32;
        let done = p
            .items
            .iter()
            .filter(|i| i.local_state == ItemLocalState::DoneLocal)
            .count() as u32;
        p.progress.externally_resolved == resolved && p.progress.done_local == done
    }) && plans
        .iter()
        .any(|p| p.progress.done_local > p.progress.externally_resolved);

    let local_safe_actions_present = plans.iter().all(|p| {
        p.actions
            .iter()
            .any(|a| a.action == PlanActionClass::MarkItemDoneLocal && a.local_safe)
            && p.actions
                .iter()
                .any(|a| a.action == PlanActionClass::PreviewMutation)
            && p.actions
                .iter()
                .all(|a| a.local_safe == a.action.local_safe())
    });

    let first_real_plans_present = PlanClass::ALL
        .iter()
        .all(|c| plans.iter().any(|p| p.plan == *c));

    let stable_ids_unique = all_unique(plans.iter().map(|p| p.plan_id.as_str()))
        && all_unique(
            plans
                .iter()
                .flat_map(|p| p.items.iter().map(|i| i.item_id.as_str())),
        );

    vec![
        invariant(
            "action_plan.surface_binding",
            "Every plan binds the action-plan matrix surface family by the matrix's own surface \
             id.",
            surface_binding,
        ),
        invariant(
            "action_plan.canonical_object_linkage",
            "Every plan addresses a canonical aureline:// subject, and every item's external object \
             and linked evidence are canonical handles.",
            canonical_object_linkage,
        ),
        invariant(
            "action_plan.ordered_items",
            "Every plan's items form a contiguous 1..n order.",
            ordered_items,
        ),
        invariant(
            "action_plan.item_intent_present",
            "Every item names a written intent, an owner, and a decision right.",
            item_intent_present,
        ),
        invariant(
            "action_plan.item_classes_distinct",
            "The set proves all six item classes — observe, verify, mitigate, rollback, \
             communicate, and custom — without collapsing them.",
            item_classes_distinct,
        ),
        invariant(
            "action_plan.local_checkoff_never_resolves_external",
            "An item resolves its external object only when its mutation is executed and confirmed, \
             never from a local check-off; at least one locally-done item leaves its external object \
             unresolved.",
            local_checkoff_never_resolves_external,
        ),
        invariant(
            "action_plan.external_mutation_linkage_explicit",
            "An externally linked item names a canonical object, a real mutation state, and a \
             mutation note; a local-only item carries none of these.",
            external_mutation_linkage_explicit,
        ),
        invariant(
            "action_plan.approval_state_preserved",
            "Every item preserves its approval state with a reason when non-authorized, and a \
             confirmed mutation held approval authority.",
            approval_state_preserved,
        ),
        invariant(
            "action_plan.evidence_linked",
            "Every linked evidence ref is a canonical handle, and every verification step links at \
             least one.",
            evidence_linked,
        ),
        invariant(
            "action_plan.due_expiry_visible",
            "Every item's deadline state agrees with its due/expiry timestamps, and overdue/expired \
             items carry a reason.",
            due_expiry_visible,
        ),
        invariant(
            "action_plan.local_note_visible",
            "Every skipped or locally-blocked item carries a written local note.",
            local_note_visible,
        ),
        invariant(
            "action_plan.scope_boundary_truth",
            "Every plan declares a scope and a matching export gate that names what crosses the \
             boundary on share/export and requires acknowledgement above private scope.",
            scope_boundary_truth,
        ),
        invariant(
            "action_plan.share_postures_distinct",
            "The set proves a private, a workspace-shared, and an org-shared plan.",
            share_postures_distinct,
        ),
        invariant(
            "action_plan.handoff_export_parity",
            "Each plan's frozen handoff equals re-exporting it and is labeled snapshot_only.",
            handoff_export_parity,
        ),
        invariant(
            "action_plan.handoff_preserves_truth",
            "Each handoff preserves the exact ordered items, the computed progress, and the \
             boundary-truth sentence.",
            handoff_preserves_truth,
        ),
        invariant(
            "action_plan.progress_no_silent_resolution",
            "Progress reports local check-offs and confirmed external resolutions as separate \
             counts, and at least one plan has more local check-offs than resolved external \
             objects.",
            progress_no_silent_resolution,
        ),
        invariant(
            "action_plan.local_safe_actions_present",
            "Every plan offers a local-safe local check-off and a separate preview-mutation action, \
             and each action's local-safe flag is computed.",
            local_safe_actions_present,
        ),
        invariant(
            "action_plan.first_real_plans_present",
            "The incident-response, support-remediation, and admin-access-review plans are all \
             present.",
            first_real_plans_present,
        ),
        invariant(
            "action_plan.stable_ids_unique",
            "Plan ids and item ids are unique.",
            stable_ids_unique,
        ),
    ]
}

fn invariant(id: &str, statement: &str, holds: bool) -> PlanInvariant {
    PlanInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the action-plan set as human-readable lines for headless / support
/// surfaces that cannot show the live UI.
pub fn action_plan_lines(set: &ActionPlanSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator action plans — {} plans, {} item classes, {} invariants (as of {})",
        set.plans.len(),
        set.item_classes.len(),
        set.invariants.len(),
        set.as_of
    ));
    lines.push(format!(
        "bound matrix: {} ({})",
        set.matrix_ref, set.matrix_record_kind
    ));
    for plan in &set.plans {
        lines.push(String::new());
        lines.push(format!(
            "[{}] {} — surface {} — subject {} — scope {} / {} — {} items",
            plan.plan.as_str(),
            plan.label,
            plan.surface_id,
            plan.subject_object_ref,
            scope_token(plan.scope),
            plan.share_posture.as_str(),
            plan.items.len()
        ));
        for it in &plan.items {
            lines.push(format!(
                "  {}. [{}] {} | local={} | ext={}/{} | resolves={} | appr={} | {} | due={}",
                it.ordinal,
                it.item_class.as_str(),
                it.title,
                it.local_state.as_str(),
                it.external_link.as_str(),
                it.external_mutation_state.as_str(),
                it.resolves_external_object,
                it.approval_state.as_str(),
                it.time_state.as_str(),
                if it.due.is_empty() { "-" } else { &it.due },
            ));
        }
        lines.push(format!("  progress: {}", plan.progress.headline));
        lines.push(format!(
            "  export gate: scope {} ({}), boundary-ack {} — {}",
            scope_token(plan.export_gate.scope),
            plan.export_gate.share_posture.as_str(),
            plan.export_gate.requires_boundary_ack,
            plan.export_gate.crosses_on_share
        ));
        lines.push(format!(
            "  handoff: {} items, {} (snapshot)",
            plan.handoff.item_count, plan.handoff.plan_id
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
