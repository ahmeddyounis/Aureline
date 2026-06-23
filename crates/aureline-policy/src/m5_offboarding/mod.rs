//! M5 admin-plane *offboarding wizards*: the concrete, typed instances of the
//! exit flow Aureline shows on its claimed managed-cloud, self-hosted,
//! sovereign/air-gapped, and mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — including the
//! [`OffboardingWizard`](crate::m5_admin_plane::AdminSurfaceClass::OffboardingWizard)
//! surface family, the states it admits, the controlled vocabularies it binds,
//! and the proof packet that keeps it current — this lane *renders that surface*.
//! It turns offboarding into a first-class local product flow: a user or admin
//! can, on the machine in front of them, walk the ordered export, transfer,
//! confirm, delete, and local-continuation checkpoints; see for each step what is
//! exported, who it transfers to, when a delete completes, what managed copies
//! remain, and who controls the next step; and complete the whole flow without a
//! still-active paid seat or a separate vendor console.
//!
//! Each wizard binds back to the frozen [admin-plane matrix](crate::m5_admin_plane).
//! Every machine-readable state a checkpoint or the coverage posture shows must be
//! one the matrix declares applicable for the offboarding surface
//! ([`OffboardingInvariant`] `offboarding.surface_states_within_matrix`), and every
//! owner and data-residency token it uses is a term the matrix's shared vocabulary
//! defines. So the render layer cannot drift from the frozen contract: an edit
//! that shows a state the matrix does not admit flips an invariant and fails the
//! freeze gate.
//!
//! The bundle holds one [`OffboardingPacket`] per claimed managed-bearing profile
//! and computes each invariant's `holds` flag from the rendered data, so the
//! checked-in fixture freezes the rendered wizards byte-for-byte. The spec's
//! honesty rules are enforced, not just described:
//!
//! - The flow is an *ordered* set of [`OffboardingCheckpoint`]s — review, export,
//!   transfer, confirm, delete, and local continuation — and every
//!   [`CheckpointKindClass`] appears on every profile
//!   (`offboarding.checkpoints_ordered_and_complete`).
//! - No checkpoint, trigger, or coverage view requires a still-active paid seat to
//!   recover user-owned data (`offboarding.no_paid_seat_required`): the export,
//!   delete, and local-continuation steps stay reachable through downgrade, seat
//!   loss, cancellation, and plan change.
//! - Every [`OffboardingTrigger`] — seat loss, cancellation, deprovision, org
//!   switch, or plan downgrade — explains the impacted managed features, export
//!   rights, local-safe continuation, and managed copies remaining in plain
//!   language (`offboarding.triggers_explain_impact`).
//! - Each step states its [`ManagedCopiesRemaining`] truth; when a managed copy
//!   remains it names what remains, where, when it clears, and who controls it,
//!   rather than implying everything is gone (`offboarding.managed_copies_honest`).
//! - A blocked or failed checkpoint carries a typed [`CheckpointRecovery`] — a
//!   restore checkpoint, a typed [`OffboardingDiagnosticClass`], and next-step
//!   guidance — so a failed export, transfer, or delete is repaired from a saved
//!   checkpoint rather than collapsing into a generic sign-in or billing error and
//!   restarting from zero (`offboarding.failed_flows_recoverable`).
//! - Distinct personal, workspace, team, and org [`OffboardingScopeClass`]es are
//!   kept distinct where ownership matters (`offboarding.scopes_distinguished`),
//!   and every profile guarantees the local-only continuation rights
//!   (`offboarding.local_continuation_guaranteed`).
//! - A checkpoint whose backing evidence is stale is never shown as a confirmed
//!   active/export-available/receipted step (`offboarding.no_silent_green`), and
//!   every profile stays locally inspectable without a vendor console
//!   (`offboarding.locally_inspectable_offline`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, raw record bodies, or absolute paths — only opaque object refs,
//! stable tokens, rendered metadata-safe summaries, and short reviewable sentences
//! — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{DataResidencyClass, EvidenceAgeClass, OwnerEscalationRoleClass};
// Reuse the generic completeness and export-form vocabularies the sibling
// decision-history render layer freezes, and the unified delete-outcome
// vocabulary the retention/deletion lane freezes, so offboarding labels coverage,
// export forms, and deletion schedules with the same tokens every admin surface
// uses.
pub use crate::m5_decision_history::{CompletenessClass, ExportForm, ExportFormatClass};
pub use crate::m5_retention_deletion::DeleteOutcomeClass;

#[cfg(test)]
mod tests;

/// Schema version for the offboarding bundle.
pub const M5_OFFBOARDING_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the offboarding bundle.
pub const M5_OFFBOARDING_SCHEMA_REF: &str = "schemas/admin/m5-offboarding.schema.json";

/// Stable record-kind tag for the offboarding bundle.
pub const M5_OFFBOARDING_RECORD_KIND: &str = "m5_offboarding_bundle";

/// Stable id for the canonical offboarding bundle.
pub const M5_OFFBOARDING_BUNDLE_ID: &str = "m5-offboarding:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_OFFBOARDING_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this render layer binds back to.
pub const M5_OFFBOARDING_MATRIX_REF: &str = "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the offboarding bundle current.
pub const M5_OFFBOARDING_FREEZE_GATE_REF: &str = "crates/aureline-policy/tests/m5_offboarding.rs";

// ---------------------------------------------------------------------------
// Offboarding token enums.
// ---------------------------------------------------------------------------

/// The event that opens the offboarding flow — the spec's requirement to keep
/// export, delete, and support actions reachable before downgrade, seat loss,
/// cancellation, or plan change lockout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingTriggerClass {
    /// A managed seat was lost or revoked.
    SeatLoss,
    /// A subscription was cancelled.
    SubscriptionCancellation,
    /// An admin deprovisioned the account.
    Deprovision,
    /// The user switched to a different organization.
    OrgSwitch,
    /// A plan change downgraded entitlements.
    PlanDowngrade,
}

impl OffboardingTriggerClass {
    /// All trigger classes, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::SeatLoss,
        Self::SubscriptionCancellation,
        Self::Deprovision,
        Self::OrgSwitch,
        Self::PlanDowngrade,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeatLoss => "seat_loss",
            Self::SubscriptionCancellation => "subscription_cancellation",
            Self::Deprovision => "deprovision",
            Self::OrgSwitch => "org_switch",
            Self::PlanDowngrade => "plan_downgrade",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SeatLoss => "Seat loss",
            Self::SubscriptionCancellation => "Subscription cancellation",
            Self::Deprovision => "Deprovision",
            Self::OrgSwitch => "Org switch",
            Self::PlanDowngrade => "Plan downgrade",
        }
    }
}

/// The ownership scope a checkpoint or transfer touches — the spec's requirement
/// to distinguish personal, workspace, team, and org scopes where usage or
/// ownership matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingScopeClass {
    /// Personal, user-owned artifacts on this machine.
    Personal,
    /// Workspace-owned artifacts.
    Workspace,
    /// Team-owned artifacts.
    Team,
    /// Organization-owned artifacts governed by managed policy.
    Org,
}

impl OffboardingScopeClass {
    /// All scopes, in vocabulary order.
    pub const ALL: [Self; 4] = [Self::Personal, Self::Workspace, Self::Team, Self::Org];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Workspace => "workspace",
            Self::Team => "team",
            Self::Org => "org",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Personal => "Personal",
            Self::Workspace => "Workspace",
            Self::Team => "Team",
            Self::Org => "Org",
        }
    }
}

/// The ordered checkpoint families an offboarding flow walks — review, export,
/// transfer, confirm, delete, and local continuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKindClass {
    /// Review the selected artifacts and what each later step will do to them.
    ReviewArtifacts,
    /// Export the selected artifacts in the offered formats.
    Export,
    /// Transfer ownership of shared artifacts to a named owner.
    Transfer,
    /// Confirm the plan; the explicit checkpoint that gates irreversible deletes.
    Confirm,
    /// Delete the selected artifacts on the stated deletion schedule.
    Delete,
    /// Continue locally: the local-only continuation rights that survive the exit.
    LocalContinuation,
}

impl CheckpointKindClass {
    /// All checkpoint kinds, in the order the wizard walks them.
    pub const ALL: [Self; 6] = [
        Self::ReviewArtifacts,
        Self::Export,
        Self::Transfer,
        Self::Confirm,
        Self::Delete,
        Self::LocalContinuation,
    ];

    /// The 1-based position of this kind in the ordered flow.
    pub const fn order(self) -> u32 {
        match self {
            Self::ReviewArtifacts => 1,
            Self::Export => 2,
            Self::Transfer => 3,
            Self::Confirm => 4,
            Self::Delete => 5,
            Self::LocalContinuation => 6,
        }
    }

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewArtifacts => "review_artifacts",
            Self::Export => "export",
            Self::Transfer => "transfer",
            Self::Confirm => "confirm",
            Self::Delete => "delete",
            Self::LocalContinuation => "local_continuation",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReviewArtifacts => "Review artifacts",
            Self::Export => "Export",
            Self::Transfer => "Transfer",
            Self::Confirm => "Confirm",
            Self::Delete => "Delete",
            Self::LocalContinuation => "Local continuation",
        }
    }
}

/// The current outcome of a checkpoint — done now, reachable now, queued, blocked,
/// or failed-but-recoverable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointOutcomeClass {
    /// Completed locally now, with nothing left to do.
    Completed,
    /// Reachable and runnable now (not yet run).
    AvailableNow,
    /// Accepted but completes later; queued to publish or finish on reconnect.
    Deferred,
    /// Cannot proceed — a hold or boundary blocks it — and says why.
    Blocked,
    /// Attempted and failed, but recoverable from a saved checkpoint.
    FailedRecoverable,
}

impl CheckpointOutcomeClass {
    /// All outcomes, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::Completed,
        Self::AvailableNow,
        Self::Deferred,
        Self::Blocked,
        Self::FailedRecoverable,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::AvailableNow => "available_now",
            Self::Deferred => "deferred",
            Self::Blocked => "blocked",
            Self::FailedRecoverable => "failed_recoverable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Completed => "Completed",
            Self::AvailableNow => "Available now",
            Self::Deferred => "Deferred",
            Self::Blocked => "Blocked",
            Self::FailedRecoverable => "Failed — recoverable",
        }
    }

    /// Whether this outcome must carry a typed recovery (a blocked or failed step
    /// has to offer a restore checkpoint, diagnostics, and next-step guidance).
    pub const fn requires_recovery(self) -> bool {
        matches!(self, Self::Blocked | Self::FailedRecoverable)
    }
}

/// The affordances a recoverable checkpoint offers so a failed flow is repaired
/// rather than restarted from zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAffordanceClass {
    /// A saved restore checkpoint to roll back to.
    RestoreCheckpoint,
    /// Typed diagnostics retained for the failure.
    RetainedDiagnostics,
    /// Plain-language next-step guidance.
    NextStepGuidance,
    /// Resume the flow from the saved checkpoint without starting over.
    ResumeFromCheckpoint,
}

impl RecoveryAffordanceClass {
    /// All affordances, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::RestoreCheckpoint,
        Self::RetainedDiagnostics,
        Self::NextStepGuidance,
        Self::ResumeFromCheckpoint,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::RetainedDiagnostics => "retained_diagnostics",
            Self::NextStepGuidance => "next_step_guidance",
            Self::ResumeFromCheckpoint => "resume_from_checkpoint",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RestoreCheckpoint => "Restore checkpoint",
            Self::RetainedDiagnostics => "Retained diagnostics",
            Self::NextStepGuidance => "Next-step guidance",
            Self::ResumeFromCheckpoint => "Resume from checkpoint",
        }
    }
}

/// A *typed* diagnostic for a failed or blocked checkpoint — the spec's
/// requirement to retain typed diagnostics instead of collapsing into a generic
/// sign-in or billing error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingDiagnosticClass {
    /// The managed export needs a reauthentication before it can run.
    ExportReauthRequired,
    /// The transfer recipient could not be reached.
    TransferRecipientUnavailable,
    /// A delete is blocked by an active legal/retention hold.
    DeleteBlockedByHold,
    /// A residency/tenant boundary changed and requires an explicit recheck.
    BoundaryRecheckRequired,
    /// The managed source is offline; the step is queued to retry on reconnect.
    MirrorOfflineRetryQueued,
    /// An export completed partially and the remainder is retryable.
    PartialExportRetryable,
}

impl OffboardingDiagnosticClass {
    /// All diagnostics, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::ExportReauthRequired,
        Self::TransferRecipientUnavailable,
        Self::DeleteBlockedByHold,
        Self::BoundaryRecheckRequired,
        Self::MirrorOfflineRetryQueued,
        Self::PartialExportRetryable,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportReauthRequired => "export_reauth_required",
            Self::TransferRecipientUnavailable => "transfer_recipient_unavailable",
            Self::DeleteBlockedByHold => "delete_blocked_by_hold",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::MirrorOfflineRetryQueued => "mirror_offline_retry_queued",
            Self::PartialExportRetryable => "partial_export_retryable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExportReauthRequired => "Export reauthentication required",
            Self::TransferRecipientUnavailable => "Transfer recipient unavailable",
            Self::DeleteBlockedByHold => "Delete blocked by hold",
            Self::BoundaryRecheckRequired => "Boundary recheck required",
            Self::MirrorOfflineRetryQueued => "Mirror offline — retry queued",
            Self::PartialExportRetryable => "Partial export — retryable",
        }
    }
}

/// The disposition of managed copies after a checkpoint — the spec's
/// managed-copies-remaining truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedCopyDispositionClass {
    /// No managed copy exists or remains; the data is local-only.
    NoneRemaining,
    /// The managed copy was destroyed and carries a destruction receipt.
    DeletedWithReceipt,
    /// A managed copy remains and is scheduled for deletion.
    PendingScheduledDelete,
    /// A managed copy is retained under a legal/regulatory hold.
    RetainedUnderHold,
    /// An upstream managed copy persists until the mirror reconnects.
    RetainedUpstreamMirror,
    /// Ownership of the managed copy transferred to a named owner; the copy
    /// remains.
    TransferredToOwner,
}

impl ManagedCopyDispositionClass {
    /// All dispositions, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::NoneRemaining,
        Self::DeletedWithReceipt,
        Self::PendingScheduledDelete,
        Self::RetainedUnderHold,
        Self::RetainedUpstreamMirror,
        Self::TransferredToOwner,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRemaining => "none_remaining",
            Self::DeletedWithReceipt => "deleted_with_receipt",
            Self::PendingScheduledDelete => "pending_scheduled_delete",
            Self::RetainedUnderHold => "retained_under_hold",
            Self::RetainedUpstreamMirror => "retained_upstream_mirror",
            Self::TransferredToOwner => "transferred_to_owner",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoneRemaining => "None remaining",
            Self::DeletedWithReceipt => "Deleted with receipt",
            Self::PendingScheduledDelete => "Pending scheduled delete",
            Self::RetainedUnderHold => "Retained under hold",
            Self::RetainedUpstreamMirror => "Retained upstream (mirror)",
            Self::TransferredToOwner => "Transferred to owner",
        }
    }

    /// Whether a managed copy still exists after this checkpoint and so must name
    /// what remains, where, and when it clears.
    pub const fn remains(self) -> bool {
        matches!(
            self,
            Self::PendingScheduledDelete
                | Self::RetainedUnderHold
                | Self::RetainedUpstreamMirror
                | Self::TransferredToOwner
        )
    }
}

/// A local-only continuation right that survives the exit — bound to the
/// offboarding surface's declared local-safe actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityRightClass {
    /// Export user-owned artifacts locally.
    ExportUserOwnedArtifacts,
    /// Continue using the local-only workspace.
    ContinueLocalOnly,
    /// Keep editing local artifacts.
    EditLocalArtifacts,
    /// Capture writes to publish later when managed access returns.
    PublishLater,
}

impl ContinuityRightClass {
    /// All continuation rights, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::ExportUserOwnedArtifacts,
        Self::ContinueLocalOnly,
        Self::EditLocalArtifacts,
        Self::PublishLater,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportUserOwnedArtifacts => "export_user_owned_artifacts",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::EditLocalArtifacts => "edit_local_artifacts",
            Self::PublishLater => "publish_later",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExportUserOwnedArtifacts => "Export user-owned artifacts",
            Self::ContinueLocalOnly => "Continue local-only",
            Self::EditLocalArtifacts => "Edit local artifacts",
            Self::PublishLater => "Publish later",
        }
    }
}

// ---------------------------------------------------------------------------
// Sub-records: trigger, managed copies, transfer, deletion schedule, recovery,
// continuity.
// ---------------------------------------------------------------------------

/// One offboarding trigger, with the plain-language impact a seat loss,
/// cancellation, deprovision, org switch, or plan downgrade has on managed
/// features, export rights, local-safe continuation, and managed copies remaining.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingTrigger {
    /// The trigger class.
    pub trigger: OffboardingTriggerClass,
    /// One reviewable label.
    pub label: String,
    /// The managed features this event impacts, in plain language.
    pub impacted_features: String,
    /// The export rights that stay available, in plain language.
    pub export_rights: String,
    /// What stays local-safe and editable after the event, in plain language.
    pub local_safe_continuation: String,
    /// The managed copies remaining after the event, in plain language.
    pub managed_copies_summary: String,
    /// Whether recovering user-owned data needs a still-active paid seat (always
    /// false: offboarding never gates user-owned recovery on a paid seat).
    pub requires_active_seat_for_recovery: bool,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

/// The managed-copies-remaining truth after a checkpoint: how many copies remain,
/// where, when they clear, and who controls them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedCopiesRemaining {
    /// The disposition of the managed copies.
    pub disposition: ManagedCopyDispositionClass,
    /// One reviewable label stating the count (e.g. "None", "1 managed copy").
    pub count_label: String,
    /// Where the remaining copies live.
    pub location: DataResidencyClass,
    /// What remains, if anything (empty when none remain).
    pub what_remains: String,
    /// When the remaining copies clear (a reviewable phrase, empty when none).
    pub cleared_when: String,
    /// Who controls the remaining copies.
    pub owner: OwnerEscalationRoleClass,
    /// One reviewable sentence describing the managed-copy disposition.
    pub note: String,
}

impl ManagedCopiesRemaining {
    /// Whether a managed copy remains after this checkpoint.
    pub fn remains(&self) -> bool {
        self.disposition.remains()
    }
}

/// A transfer plan for a transfer checkpoint: who ownership moves to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferPlan {
    /// Who receives ownership.
    pub transfer_owner: OwnerEscalationRoleClass,
    /// The opaque recipient ref ownership transfers to.
    pub recipient_ref: String,
    /// The scope being transferred.
    pub scope: OffboardingScopeClass,
    /// One reviewable sentence describing the transfer.
    pub note: String,
}

/// A deletion schedule for a delete checkpoint: when the delete completes and what
/// remains until it does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionSchedule {
    /// The delete outcome (immediate / deferred / blocked).
    pub outcome: DeleteOutcomeClass,
    /// When the delete completes (a reviewable phrase, not a raw timestamp).
    pub when: String,
    /// What remains until the delete completes (empty for an immediate delete).
    pub what_remains: String,
    /// Where the remainder lives.
    pub where_remains: DataResidencyClass,
    /// Who controls the next step.
    pub next_step_owner: OwnerEscalationRoleClass,
    /// One reviewable sentence describing the schedule.
    pub note: String,
}

/// The typed recovery a blocked or failed checkpoint retains: a restore
/// checkpoint, typed diagnostics, and next-step guidance, so a failed flow is
/// repaired rather than restarted from zero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointRecovery {
    /// The affordances this recovery offers.
    pub affordances: Vec<RecoveryAffordanceClass>,
    /// The opaque ref of the saved restore checkpoint to roll back to.
    pub restore_checkpoint_ref: String,
    /// The typed diagnostic for the failure (never a generic sign-in/billing
    /// error).
    pub diagnostic: OffboardingDiagnosticClass,
    /// One reviewable sentence of typed diagnostic detail.
    pub diagnostic_detail: String,
    /// The plain-language next step to repair the flow.
    pub next_step: String,
    /// Who owns the next step.
    pub next_step_owner: OwnerEscalationRoleClass,
    /// One reviewable sentence describing the recovery.
    pub note: String,
}

impl CheckpointRecovery {
    /// Whether this recovery offers the named affordance.
    pub fn offers(&self, affordance: RecoveryAffordanceClass) -> bool {
        self.affordances.contains(&affordance)
    }
}

/// One local-only continuation right that survives the exit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityGuarantee {
    /// The continuation right.
    pub right: ContinuityRightClass,
    /// One reviewable label.
    pub label: String,
    /// Whether the right is available fully offline.
    pub available_offline: bool,
    /// Whether the right needs a paid seat (always false).
    pub requires_paid_seat: bool,
    /// One reviewable sentence describing the right.
    pub note: String,
}

// ---------------------------------------------------------------------------
// The checkpoint, the wizard, the per-profile packet, and the bundle.
// ---------------------------------------------------------------------------

/// One ordered checkpoint in a profile's offboarding wizard: a step with its
/// scope, outcome, machine-readable state, managed-copies truth, and — where
/// relevant — its transfer plan, deletion schedule, and typed recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingCheckpoint {
    /// Stable, opaque checkpoint id (deep-linkable, export-safe).
    pub checkpoint_id: String,
    /// The checkpoint kind.
    pub kind: CheckpointKindClass,
    /// The 1-based position in the ordered flow (equals `kind.order()`).
    pub order: u32,
    /// One reviewable label.
    pub label: String,
    /// The ownership scope this checkpoint touches.
    pub scope: OffboardingScopeClass,
    /// The current outcome.
    pub outcome: CheckpointOutcomeClass,
    /// The machine-readable state (must be one the matrix admits for this
    /// surface).
    pub machine_state: AdminStateClass,
    /// The freshness of the evidence backing the step.
    pub evidence_age: EvidenceAgeClass,
    /// Where the artifacts this step touches live.
    pub location: DataResidencyClass,
    /// Who owns this step.
    pub owner: OwnerEscalationRoleClass,
    /// Whether this step needs a still-active paid seat (always false:
    /// offboarding never gates user-owned recovery on a paid seat).
    pub requires_paid_seat: bool,
    /// Whether this step is an explicit confirmation checkpoint.
    pub confirmation_required: bool,
    /// The managed-copies-remaining truth after this step.
    pub managed_copies: ManagedCopiesRemaining,
    /// The transfer plan for a transfer checkpoint, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<TransferPlan>,
    /// The deletion schedule for a delete checkpoint, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_schedule: Option<DeletionSchedule>,
    /// The typed recovery for a blocked or failed checkpoint, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CheckpointRecovery>,
    /// The schema that governs this step.
    pub governing_schema_ref: String,
    /// One reviewable sentence noting how the step is schema-governed.
    pub schema_note: String,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

impl OffboardingCheckpoint {
    /// Whether the checkpoint carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }

    /// Whether the checkpoint carries a typed recovery.
    pub fn has_recovery(&self) -> bool {
        self.recovery.is_some()
    }
}

/// The coverage posture of a rendered wizard: how complete the flow view is,
/// whether it stays locally inspectable, and whether it can be completed without a
/// paid seat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingCoverage {
    /// The coverage state (must be one the matrix admits for this surface).
    pub coverage_state: AdminStateClass,
    /// How complete the flow view is.
    pub completeness: CompletenessClass,
    /// One reviewable label for the coverage window.
    pub window_label: String,
    /// One reviewable sentence stating the coverage rule and any labeled gap.
    pub coverage_note: String,
    /// Whether the wizard is locally inspectable on this profile.
    pub locally_inspectable: bool,
    /// Whether the wizard is available without a vendor console / control plane.
    pub vendor_console_independent: bool,
    /// Whether the flow can be completed (user-owned data recovered) without a
    /// still-active paid seat.
    pub completable_without_paid_seat: bool,
}

/// The rendered offboarding wizard for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingWizard {
    /// The surface family (always [`AdminSurfaceClass::OffboardingWizard`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The triggers this wizard handles.
    pub triggers: Vec<OffboardingTrigger>,
    /// The ordered checkpoints.
    pub checkpoints: Vec<OffboardingCheckpoint>,
    /// The export forms offered.
    pub export_forms: Vec<ExportForm>,
    /// The local-only continuation rights guaranteed after the exit.
    pub continuity: Vec<ContinuityGuarantee>,
    /// The coverage posture of the wizard.
    pub coverage: OffboardingCoverage,
}

impl OffboardingWizard {
    /// Resolves a checkpoint by id, if present.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&OffboardingCheckpoint> {
        self.checkpoints
            .iter()
            .find(|c| c.checkpoint_id == checkpoint_id)
    }

    /// The distinct scopes present in the wizard.
    pub fn scopes(&self) -> std::collections::BTreeSet<OffboardingScopeClass> {
        self.checkpoints.iter().map(|c| c.scope).collect()
    }

    /// Whether the wizard renders a checkpoint of the given kind.
    pub fn has_kind(&self, kind: CheckpointKindClass) -> bool {
        self.checkpoints.iter().any(|c| c.kind == kind)
    }

    /// Whether the wizard offers a given export format.
    pub fn offers(&self, format: ExportFormatClass) -> bool {
        self.export_forms.iter().any(|f| f.format == format)
    }

    /// Whether the wizard guarantees a given continuation right.
    pub fn guarantees(&self, right: ContinuityRightClass) -> bool {
        self.continuity.iter().any(|c| c.right == right)
    }
}

/// The rendered offboarding surface for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingPacket {
    /// The admin path / profile this packet renders.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The offboarding wizard.
    pub wizard: OffboardingWizard,
}

impl OffboardingPacket {
    /// Resolves a checkpoint by id within this packet.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&OffboardingCheckpoint> {
        self.wizard.checkpoint(checkpoint_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the rendered bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen offboarding bundle: one packet per claimed managed-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_offboarding_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this render layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this render layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile offboarding packets.
    pub profiles: Vec<OffboardingPacket>,
    /// The computed invariants.
    pub invariants: Vec<OffboardingInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffboardingValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for OffboardingValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offboarding bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for OffboardingValidationError {}

/// The profiles the offboarding bundle covers, in bundle order.
pub const OFFBOARDING_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

impl OffboardingBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&OffboardingPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque token, never
    /// a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing. Stable token ids are audited separately by [`is_safe_token`].
    fn file_refs(&self) -> Vec<&str> {
        let mut refs = vec![
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ];
        for p in &self.profiles {
            for c in &p.wizard.checkpoints {
                refs.push(c.governing_schema_ref.as_str());
            }
        }
        refs
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            ids.push(p.wizard.surface_id.as_str());
            for c in &p.wizard.checkpoints {
                ids.push(c.checkpoint_id.as_str());
                if let Some(t) = &c.transfer {
                    ids.push(t.recipient_ref.as_str());
                }
                if let Some(r) = &c.recovery {
                    ids.push(r.restore_checkpoint_ref.as_str());
                }
            }
            for x in &p.wizard.export_forms {
                ids.push(x.artifact_ref.as_str());
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`OffboardingInvariant`]s with the
    /// coverage and resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), OffboardingValidationError> {
        let fail = |reason: String| Err(OffboardingValidationError { reason });

        if self.record_kind != M5_OFFBOARDING_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_OFFBOARDING_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in OFFBOARDING_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet).map_err(|reason| OffboardingValidationError { reason })?;
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

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Whether a state asserts a currently-confirmed positive claim, so stale
/// evidence under it would be a silent-green lie: the step is actively done now,
/// exportable now, or completed with a receipt. The other admitted states are
/// explicit non-confirmations (pending, deferred, blocked, offline, boundary
/// recheck, unknown).
fn requires_fresh_evidence(state: AdminStateClass) -> bool {
    matches!(
        state,
        AdminStateClass::ActiveEnforced
            | AdminStateClass::ExportAvailableNow
            | AdminStateClass::DeleteReceipted
    )
}

/// Per-packet structural floor checks, shared by [`OffboardingBundle::validate`].
fn validate_packet(packet: &OffboardingPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    let wizard = &packet.wizard;
    if wizard.surface != AdminSurfaceClass::OffboardingWizard {
        return Err(format!(
            "{}: wizard is not the offboarding surface",
            packet.profile.as_str()
        ));
    }
    if wizard.checkpoints.is_empty() {
        return Err(format!("{}: no checkpoints", packet.profile.as_str()));
    }
    if !all_unique(wizard.checkpoints.iter().map(|c| c.checkpoint_id.as_str())) {
        return Err(format!(
            "{}: checkpoint ids are not unique",
            packet.profile.as_str()
        ));
    }
    // Every checkpoint kind appears exactly once, in order.
    for kind in CheckpointKindClass::ALL {
        if wizard.checkpoints.iter().filter(|c| c.kind == kind).count() != 1 {
            return Err(format!(
                "{}: checkpoint kind {} not present exactly once",
                packet.profile.as_str(),
                kind.as_str()
            ));
        }
    }
    let mut prev_order = 0u32;
    for checkpoint in &wizard.checkpoints {
        if checkpoint.order != checkpoint.kind.order() {
            return Err(format!(
                "{}: checkpoint {} order {} does not match its kind",
                packet.profile.as_str(),
                checkpoint.checkpoint_id,
                checkpoint.order
            ));
        }
        if checkpoint.order <= prev_order {
            return Err(format!(
                "{}: checkpoints are not in ascending order at {}",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
        prev_order = checkpoint.order;

        if checkpoint.requires_paid_seat {
            return Err(format!(
                "{}: checkpoint {} requires a paid seat",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
        // A blocked/failed step retains a typed recovery.
        if checkpoint.outcome.requires_recovery() {
            let Some(recovery) = &checkpoint.recovery else {
                return Err(format!(
                    "{}: checkpoint {} is {} but retains no recovery",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id,
                    checkpoint.outcome.as_str()
                ));
            };
            if recovery.restore_checkpoint_ref.is_empty() || recovery.next_step.is_empty() {
                return Err(format!(
                    "{}: checkpoint {} recovery is incomplete",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                ));
            }
            for needed in [
                RecoveryAffordanceClass::RestoreCheckpoint,
                RecoveryAffordanceClass::RetainedDiagnostics,
                RecoveryAffordanceClass::NextStepGuidance,
            ] {
                if !recovery.offers(needed) {
                    return Err(format!(
                        "{}: checkpoint {} recovery lacks {}",
                        packet.profile.as_str(),
                        checkpoint.checkpoint_id,
                        needed.as_str()
                    ));
                }
            }
        }
        // A confirm checkpoint requires explicit confirmation.
        if checkpoint.kind == CheckpointKindClass::Confirm && !checkpoint.confirmation_required {
            return Err(format!(
                "{}: confirm checkpoint {} does not require confirmation",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
        // A delete checkpoint carries a deletion schedule and is confirmation-gated.
        if checkpoint.kind == CheckpointKindClass::Delete {
            if checkpoint.deletion_schedule.is_none() {
                return Err(format!(
                    "{}: delete checkpoint {} has no deletion schedule",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                ));
            }
            if !checkpoint.confirmation_required {
                return Err(format!(
                    "{}: delete checkpoint {} is not confirmation-gated",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                ));
            }
        }
        // A transfer checkpoint names a transfer owner.
        if checkpoint.kind == CheckpointKindClass::Transfer && checkpoint.transfer.is_none() {
            return Err(format!(
                "{}: transfer checkpoint {} names no transfer plan",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
        // A remaining managed copy names what/where/when.
        if checkpoint.managed_copies.remains()
            && (checkpoint.managed_copies.what_remains.is_empty()
                || checkpoint.managed_copies.cleared_when.is_empty())
        {
            return Err(format!(
                "{}: checkpoint {} leaves a managed copy without naming it",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
        if !checkpoint.has_export_parity() {
            return Err(format!(
                "{}: checkpoint {} lacks both export representations",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            ));
        }
    }
    // Both export forms are offered.
    if !wizard.offers(ExportFormatClass::MachineReadableJson)
        || !wizard.offers(ExportFormatClass::PlainLanguageHandoff)
    {
        return Err(format!(
            "{}: wizard does not offer both export forms",
            packet.profile.as_str()
        ));
    }
    // The continuation rights are all guaranteed, offline, and seat-free.
    for right in ContinuityRightClass::ALL {
        let Some(guarantee) = wizard.continuity.iter().find(|c| c.right == right) else {
            return Err(format!(
                "{}: wizard does not guarantee {}",
                packet.profile.as_str(),
                right.as_str()
            ));
        };
        if !guarantee.available_offline || guarantee.requires_paid_seat {
            return Err(format!(
                "{}: continuation right {} is not offline-safe and seat-free",
                packet.profile.as_str(),
                right.as_str()
            ));
        }
    }
    // Every trigger explains its impact and never gates recovery on a paid seat.
    if wizard.triggers.is_empty() {
        return Err(format!(
            "{}: wizard names no triggers",
            packet.profile.as_str()
        ));
    }
    for trigger in &wizard.triggers {
        if trigger.impacted_features.is_empty()
            || trigger.export_rights.is_empty()
            || trigger.local_safe_continuation.is_empty()
            || trigger.managed_copies_summary.is_empty()
        {
            return Err(format!(
                "{}: trigger {} does not fully explain its impact",
                packet.profile.as_str(),
                trigger.trigger.as_str()
            ));
        }
        if trigger.requires_active_seat_for_recovery {
            return Err(format!(
                "{}: trigger {} requires an active seat for recovery",
                packet.profile.as_str(),
                trigger.trigger.as_str()
            ));
        }
    }
    // The wizard is locally inspectable, console-independent, and seat-free.
    let coverage = &wizard.coverage;
    if !coverage.locally_inspectable
        || !coverage.vendor_console_independent
        || !coverage.completable_without_paid_seat
    {
        return Err(format!(
            "{}: wizard is not locally completable without a vendor console or paid seat",
            packet.profile.as_str()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical offboarding bundle.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the rendered packets, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn offboarding_bundle() -> OffboardingBundle {
    let profiles: Vec<OffboardingPacket> = OFFBOARDING_PROFILES
        .iter()
        .map(|p| offboarding_packet(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    OffboardingBundle {
        record_kind: M5_OFFBOARDING_RECORD_KIND.to_owned(),
        m5_offboarding_schema_version: M5_OFFBOARDING_SCHEMA_VERSION,
        schema_ref: M5_OFFBOARDING_SCHEMA_REF.to_owned(),
        bundle_id: M5_OFFBOARDING_BUNDLE_ID.to_owned(),
        as_of: M5_OFFBOARDING_AS_OF.to_owned(),
        matrix_ref: M5_OFFBOARDING_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_OFFBOARDING_FREEZE_GATE_REF.to_owned(),
        summary:
            "Rendered offboarding wizards — the ordered review, export, transfer, confirm, delete, \
             and local-continuation checkpoints for seat loss, cancellation, deprovision, org \
             switch, and plan downgrade — bound back to the frozen admin-plane matrix and rendered \
             identically for shell, CLI/headless, Help/About, support export, and procurement \
             consumers across the managed-cloud, self-hosted, sovereign/air-gapped, and \
             mirrored/offline profiles. Each step names its scope, managed copies remaining, \
             transfer owner, deletion schedule, and confirmation gate; blocked and failed steps \
             retain a restore checkpoint, typed diagnostics, and next-step guidance; and the whole \
             flow stays locally completable without a vendor console or a still-active paid seat."
                .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

/// The consumers every packet must serve identically; mirrors the matrix's
/// declared consumers for the offboarding surface.
fn parity_consumers() -> Vec<AdminConsumerClass> {
    admin_plane_matrix()
        .surface(AdminSurfaceClass::OffboardingWizard)
        .map(|entry| entry.consumed_by.clone())
        .unwrap_or_default()
}

fn offboarding_packet(profile: AdminPathClass) -> OffboardingPacket {
    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: the full offboarding flow runs live against the managed control \
             plane; the personal export and the local-continuation steps stay reachable without a \
             paid seat and the managed copy of deleted data carries a destruction receipt.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer's own control plane governs the flow; a \
             regulatory-hold-blocked delete retains a restore checkpoint, typed diagnostics, and \
             next-step guidance rather than collapsing into a generic error.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: export and local continuation complete fully offline; \
             a transfer whose tenant boundary changed offline fails into a recoverable checkpoint, \
             and a sealed-hold delete is blocked and explains who controls release.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the local export and continuation steps work with the \
             mirror offline; a transfer to an unreachable recipient and a deferred upstream delete \
             are queued to resume on reconnect with their checkpoints saved.",
        ),
        _ => (AdminDeploymentProfileClass::IndividualLocal, "Local profile."),
    };

    OffboardingPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers: parity_consumers(),
        wizard: render_wizard(profile),
    }
}

fn render_wizard(profile: AdminPathClass) -> OffboardingWizard {
    let surface = AdminSurfaceClass::OffboardingWizard;
    let summary = match profile {
        AdminPathClass::ManagedCloud => {
            "The ordered exit flow: review the selected artifacts, export user-owned data now, \
             transfer the workspace copy, confirm, delete with a destruction receipt, and continue \
             locally — completable without a paid seat."
        }
        AdminPathClass::SelfHosted => {
            "The exit flow on the customer's own control plane; the regulatory-hold-blocked delete \
             stays recoverable from a saved checkpoint with typed diagnostics instead of a generic \
             error."
        }
        AdminPathClass::SovereignAirGapped => {
            "The exit flow on an air-gapped install; export and continuation finish offline while a \
             boundary-changed transfer and a sealed-hold delete are surfaced as recoverable and \
             blocked rather than silently failing."
        }
        AdminPathClass::MirroredOffline => {
            "The exit flow with the mirror offline; local export and continuation work now, and the \
             transfer and upstream delete are queued to resume on reconnect with their checkpoints \
             saved."
        }
        _ => "Offboarding wizard.",
    };

    OffboardingWizard {
        surface,
        surface_id: surface.surface_id(),
        summary: summary.to_owned(),
        triggers: build_triggers(profile),
        checkpoints: build_checkpoints(profile),
        export_forms: build_export_forms(profile),
        continuity: build_continuity(),
        coverage: build_coverage(profile),
    }
}

// ---------------------------------------------------------------------------
// Builders.
// ---------------------------------------------------------------------------

const DEPROVISION_SCHEMA: &str = "schemas/admin/deprovision_handoff.schema.json";
const CONTINUITY_SCHEMA: &str = "schemas/storage/m5_offboarding_continuity.schema.json";
const LIFECYCLE_SCHEMA: &str = "schemas/governance/records_export_delete_lifecycle.schema.json";

fn trigger(
    trigger: OffboardingTriggerClass,
    impacted_features: &str,
    export_rights: &str,
    local_safe_continuation: &str,
    managed_copies_summary: &str,
    plain_language: &str,
) -> OffboardingTrigger {
    OffboardingTrigger {
        trigger,
        label: trigger.label().to_owned(),
        impacted_features: impacted_features.to_owned(),
        export_rights: export_rights.to_owned(),
        local_safe_continuation: local_safe_continuation.to_owned(),
        managed_copies_summary: managed_copies_summary.to_owned(),
        requires_active_seat_for_recovery: false,
        plain_language: plain_language.to_owned(),
    }
}

fn build_triggers(profile: AdminPathClass) -> Vec<OffboardingTrigger> {
    use OffboardingTriggerClass::*;
    match profile {
        AdminPathClass::ManagedCloud => vec![
            trigger(
                SeatLoss,
                "Managed AI, shared collaboration, and managed sync pause when the seat is lost.",
                "You can export all user-owned artifacts now, before the seat lapses.",
                "Your local workspace stays fully editable offline with no paid seat.",
                "No managed copy of user-owned data remains after the local export and delete.",
                "Losing your seat pauses managed features but never your local work; export now and \
                 keep editing locally.",
            ),
            trigger(
                SubscriptionCancellation,
                "Billing-bound managed features stop at the end of the period.",
                "Export stays available through the end of the period and via the local export now.",
                "Local-only continuation is unaffected by cancellation.",
                "Managed copies are deleted on the stated schedule with a destruction receipt.",
                "Cancelling stops managed features at period end; your user-owned data is exportable \
                 now and deletes with a receipt.",
            ),
            trigger(
                PlanDowngrade,
                "Entitlements above the new plan are withdrawn at the change.",
                "Export of anything the downgrade withdraws stays available before the change.",
                "Everything local-safe on the lower plan keeps working offline.",
                "Managed copies tied to withdrawn entitlements are scheduled for deletion.",
                "Downgrading withdraws higher-tier entitlements but leaves your local-safe work and \
                 export rights intact.",
            ),
        ],
        AdminPathClass::SelfHosted => vec![
            trigger(
                Deprovision,
                "The admin deprovisioned the account; managed audit and policy actions stop.",
                "Export of team-owned artifacts runs against the self-hosted plane now.",
                "Local artifacts stay editable on this machine after deprovision.",
                "A regulatory-held managed copy is retained and names who controls release.",
                "Deprovision stops managed actions but preserves local-owned work; held data is \
                 retained and explained, never silently stripped.",
            ),
            trigger(
                OrgSwitch,
                "Switching orgs reassigns managed scope to the new organization.",
                "Export of artifacts leaving with you stays available before the switch.",
                "Local continuation is unaffected by the org switch.",
                "Org-owned managed copies transfer to the org admin rather than being deleted.",
                "Switching orgs transfers org-owned copies to the new owner and leaves your local \
                 work and export rights intact.",
            ),
        ],
        AdminPathClass::SovereignAirGapped => vec![
            trigger(
                Deprovision,
                "Managed actions stop on the air-gapped install at deprovision.",
                "Export of personal artifacts completes fully offline now.",
                "Local continuation is fully available offline and seat-free.",
                "A sealed-hold managed copy is retained under the hold and names the compliance owner.",
                "Deprovision on an air-gapped install never blocks local export or continuation; \
                 held data is retained and explained.",
            ),
            trigger(
                SeatLoss,
                "Managed entitlements lapse when the offline seat expires.",
                "Personal export remains available offline regardless of the seat.",
                "Local-only continuation needs no paid seat and works offline.",
                "No managed copy of user-owned data remains after the offline export and delete.",
                "An offline seat lapse never gates your local export or continuation; both work with \
                 no paid seat.",
            ),
        ],
        AdminPathClass::MirroredOffline => vec![
            trigger(
                SubscriptionCancellation,
                "Managed features pause once the mirror confirms cancellation upstream.",
                "Local export of user-owned data is available now with the mirror offline.",
                "Local continuation works offline with no paid seat.",
                "Upstream managed copies are queued for deletion to complete on reconnect.",
                "Cancelling with the mirror offline still lets you export and continue locally; the \
                 upstream delete completes on reconnect.",
            ),
            trigger(
                OrgSwitch,
                "Org scope reassigns upstream once the mirror reconnects.",
                "Export of artifacts leaving with you is available locally now.",
                "Local continuation is unaffected by the org switch.",
                "The upstream transfer is queued and its checkpoint saved until reconnect.",
                "Switching orgs while offline saves the transfer checkpoint and resumes it on \
                 reconnect; your local export and continuation are unaffected.",
            ),
        ],
        _ => Vec::new(),
    }
}

fn mcopy(
    disposition: ManagedCopyDispositionClass,
    count_label: &str,
    location: DataResidencyClass,
    what_remains: &str,
    cleared_when: &str,
    owner: OwnerEscalationRoleClass,
    note: &str,
) -> ManagedCopiesRemaining {
    ManagedCopiesRemaining {
        disposition,
        count_label: count_label.to_owned(),
        location,
        what_remains: what_remains.to_owned(),
        cleared_when: cleared_when.to_owned(),
        owner,
        note: note.to_owned(),
    }
}

/// A managed-copies disposition for a purely local step: nothing managed remains.
fn no_managed_copy() -> ManagedCopiesRemaining {
    mcopy(
        ManagedCopyDispositionClass::NoneRemaining,
        "None",
        DataResidencyClass::LocalOnly,
        "",
        "",
        OwnerEscalationRoleClass::LocalUser,
        "No managed copy is involved; this step touches local-only data.",
    )
}

fn transfer_plan(
    transfer_owner: OwnerEscalationRoleClass,
    recipient_ref: &str,
    scope: OffboardingScopeClass,
    note: &str,
) -> TransferPlan {
    TransferPlan {
        transfer_owner,
        recipient_ref: recipient_ref.to_owned(),
        scope,
        note: note.to_owned(),
    }
}

fn schedule(
    outcome: DeleteOutcomeClass,
    when: &str,
    what_remains: &str,
    where_remains: DataResidencyClass,
    next_step_owner: OwnerEscalationRoleClass,
    note: &str,
) -> DeletionSchedule {
    DeletionSchedule {
        outcome,
        when: when.to_owned(),
        what_remains: what_remains.to_owned(),
        where_remains,
        next_step_owner,
        note: note.to_owned(),
    }
}

fn recovery(
    diagnostic: OffboardingDiagnosticClass,
    restore_checkpoint_ref: &str,
    diagnostic_detail: &str,
    next_step: &str,
    next_step_owner: OwnerEscalationRoleClass,
    note: &str,
) -> CheckpointRecovery {
    CheckpointRecovery {
        affordances: RecoveryAffordanceClass::ALL.to_vec(),
        restore_checkpoint_ref: restore_checkpoint_ref.to_owned(),
        diagnostic,
        diagnostic_detail: diagnostic_detail.to_owned(),
        next_step: next_step.to_owned(),
        next_step_owner,
        note: note.to_owned(),
    }
}

/// One concise builder for an offboarding checkpoint. `requires_paid_seat` is
/// always false — offboarding never gates user-owned recovery on a paid seat.
#[allow(clippy::too_many_arguments)]
fn checkpoint(
    checkpoint_id: &str,
    kind: CheckpointKindClass,
    label: &str,
    scope: OffboardingScopeClass,
    outcome: CheckpointOutcomeClass,
    machine_state: AdminStateClass,
    evidence_age: EvidenceAgeClass,
    location: DataResidencyClass,
    owner: OwnerEscalationRoleClass,
    confirmation_required: bool,
    managed_copies: ManagedCopiesRemaining,
    transfer: Option<TransferPlan>,
    deletion_schedule: Option<DeletionSchedule>,
    recovery: Option<CheckpointRecovery>,
    governing_schema_ref: &str,
    schema_note: &str,
    machine_summary: &str,
    plain_language: &str,
) -> OffboardingCheckpoint {
    OffboardingCheckpoint {
        checkpoint_id: checkpoint_id.to_owned(),
        kind,
        order: kind.order(),
        label: label.to_owned(),
        scope,
        outcome,
        machine_state,
        evidence_age,
        location,
        owner,
        requires_paid_seat: false,
        confirmation_required,
        managed_copies,
        transfer,
        deletion_schedule,
        recovery,
        governing_schema_ref: governing_schema_ref.to_owned(),
        schema_note: schema_note.to_owned(),
        machine_summary: machine_summary.to_owned(),
        plain_language: plain_language.to_owned(),
    }
}

fn build_checkpoints(profile: AdminPathClass) -> Vec<OffboardingCheckpoint> {
    use AdminStateClass::*;
    use CheckpointKindClass::*;
    use CheckpointOutcomeClass::*;
    use DataResidencyClass::*;
    use EvidenceAgeClass::*;
    use ManagedCopyDispositionClass::*;
    use OffboardingDiagnosticClass as Diag;
    use OffboardingScopeClass::*;
    use OwnerEscalationRoleClass::*;

    match profile {
        AdminPathClass::ManagedCloud => vec![
            checkpoint(
                "offboarding.checkpoint.managed_cloud.review",
                ReviewArtifacts,
                "Review selected artifacts",
                Personal,
                Completed,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the review runs locally.",
                "kind=review_artifacts scope=personal outcome=completed state=active_enforced \
                 managed_copies=none_remaining",
                "Review the personal artifacts selected for export, transfer, and deletion; the \
                 review runs on this machine and lists any managed copies before anything changes.",
            ),
            checkpoint(
                "offboarding.checkpoint.managed_cloud.export",
                Export,
                "Export user-owned data",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle; the local export needs no paid seat.",
                "kind=export scope=personal outcome=available_now state=export_available_now \
                 managed_copies=none_remaining",
                "Export all user-owned artifacts now — no still-active seat is required to recover \
                 your own data.",
            ),
            checkpoint(
                "offboarding.checkpoint.managed_cloud.transfer",
                Transfer,
                "Transfer the workspace copy",
                Workspace,
                Completed,
                ActiveEnforced,
                Fresh,
                SharedWorkspaceCopy,
                WorkspaceOwner,
                false,
                mcopy(
                    TransferredToOwner,
                    "1 workspace copy",
                    SharedWorkspaceCopy,
                    "the shared workspace copy",
                    "remains under the new owner after the transfer completes",
                    WorkspaceOwner,
                    "Ownership of the workspace copy transfers to the named workspace owner; the \
                     copy is not deleted.",
                ),
                Some(transfer_plan(
                    WorkspaceOwner,
                    "transfer.recipient.managed.ws_owner_07",
                    Workspace,
                    "The shared workspace copy transfers to the named workspace owner.",
                )),
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; transfer reassigns ownership.",
                "kind=transfer scope=workspace outcome=completed state=active_enforced \
                 managed_copies=transferred_to_owner",
                "Transfer the shared workspace copy to the named workspace owner so the team keeps \
                 access after you leave.",
            ),
            checkpoint(
                "offboarding.checkpoint.managed_cloud.confirm",
                Confirm,
                "Confirm the offboarding plan",
                Org,
                AvailableNow,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                true,
                mcopy(
                    PendingScheduledDelete,
                    "1 managed copy",
                    ManagedCopy,
                    "the managed copy queued for deletion",
                    "on the deletion schedule confirmed here",
                    OrgAdmin,
                    "Confirmation records who approved the plan and schedules the managed delete.",
                ),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the confirmation gate is explicit.",
                "kind=confirm scope=org outcome=available_now state=active_enforced \
                 confirmation_required=true managed_copies=pending_scheduled_delete",
                "Confirm the offboarding plan; this explicit checkpoint records who approved it and \
                 gates the irreversible delete that follows.",
            ),
            checkpoint(
                "offboarding.checkpoint.managed_cloud.delete",
                Delete,
                "Delete with a destruction receipt",
                Personal,
                Completed,
                DeleteReceipted,
                Fresh,
                ExportedSnapshot,
                LocalUser,
                true,
                mcopy(
                    DeletedWithReceipt,
                    "None",
                    ExportedSnapshot,
                    "",
                    "",
                    LocalUser,
                    "The managed copy is destroyed and carries a destruction receipt.",
                ),
                None,
                Some(schedule(
                    DeleteOutcomeClass::Immediate,
                    "now",
                    "",
                    LocalOnly,
                    LocalUser,
                    "The delete completes immediately with a destruction receipt and nothing left \
                     behind.",
                )),
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle with a durable destruction receipt.",
                "kind=delete scope=personal outcome=completed state=delete_receipted \
                 confirmation_required=true managed_copies=deleted_with_receipt",
                "After you confirm, the selected data is deleted immediately and carries a \
                 destruction receipt proving it is gone.",
            ),
            checkpoint(
                "offboarding.checkpoint.managed_cloud.local_continuation",
                LocalContinuation,
                "Continue locally",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                CONTINUITY_SCHEMA,
                "Governed by the offboarding-continuity contract; local use survives the exit.",
                "kind=local_continuation scope=personal outcome=available_now \
                 state=export_available_now managed_copies=none_remaining",
                "After offboarding your local workspace stays fully editable offline with no paid \
                 seat, and export remains available.",
            ),
        ],
        AdminPathClass::SelfHosted => vec![
            checkpoint(
                "offboarding.checkpoint.self_hosted.review",
                ReviewArtifacts,
                "Review selected artifacts",
                Team,
                Completed,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract on the self-hosted plane.",
                "kind=review_artifacts scope=team outcome=completed state=active_enforced \
                 managed_copies=none_remaining",
                "Review the team artifacts selected for export, transfer, and deletion on the \
                 self-hosted control plane.",
            ),
            checkpoint(
                "offboarding.checkpoint.self_hosted.export",
                Export,
                "Export team-owned data",
                Team,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                false,
                mcopy(
                    PendingScheduledDelete,
                    "1 self-hosted copy",
                    ManagedCopy,
                    "the self-hosted copy of the exported data",
                    "on the deletion schedule, after the confirm step",
                    OrgAdmin,
                    "Export copies the data; the self-hosted copy persists until the scheduled \
                     delete.",
                ),
                None,
                None,
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle; export runs against the self-hosted plane.",
                "kind=export scope=team outcome=available_now state=export_available_now \
                 managed_copies=pending_scheduled_delete",
                "Export the team-owned data from the self-hosted control plane now; the self-hosted \
                 copy remains until the scheduled delete.",
            ),
            checkpoint(
                "offboarding.checkpoint.self_hosted.transfer",
                Transfer,
                "Transfer org-owned artifacts",
                Org,
                Completed,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                false,
                mcopy(
                    TransferredToOwner,
                    "1 org copy",
                    ManagedCopy,
                    "the org-owned copy",
                    "remains under the org admin after the transfer",
                    OrgAdmin,
                    "Ownership of the org copy transfers to the org admin; the copy is retained.",
                ),
                Some(transfer_plan(
                    OrgAdmin,
                    "transfer.recipient.self_hosted.org_admin_02",
                    Org,
                    "The org-owned artifacts transfer to the org admin.",
                )),
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; transfer reassigns org ownership.",
                "kind=transfer scope=org outcome=completed state=active_enforced \
                 managed_copies=transferred_to_owner",
                "Transfer the org-owned artifacts to the org admin so the organization keeps \
                 access.",
            ),
            checkpoint(
                "offboarding.checkpoint.self_hosted.confirm",
                Confirm,
                "Confirm the offboarding plan",
                Org,
                AvailableNow,
                ActiveEnforced,
                Fresh,
                ManagedCopy,
                OrgAdmin,
                true,
                mcopy(
                    PendingScheduledDelete,
                    "1 self-hosted copy",
                    ManagedCopy,
                    "the self-hosted copy queued for deletion",
                    "on the deletion schedule confirmed here",
                    OrgAdmin,
                    "Confirmation records the approver and schedules the self-hosted delete.",
                ),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the confirmation gate is explicit.",
                "kind=confirm scope=org outcome=available_now state=active_enforced \
                 confirmation_required=true managed_copies=pending_scheduled_delete",
                "Confirm the offboarding plan on the self-hosted plane; the explicit checkpoint \
                 gates the delete that follows.",
            ),
            checkpoint(
                "offboarding.checkpoint.self_hosted.delete",
                Delete,
                "Delete (blocked by regulatory hold)",
                Org,
                Blocked,
                DeleteBlockedByHold,
                Fresh,
                ManagedCopy,
                SecurityOwner,
                true,
                mcopy(
                    RetainedUnderHold,
                    "1 held copy",
                    ManagedCopy,
                    "the operational audit history",
                    "when the security owner releases the regulatory hold",
                    SecurityOwner,
                    "The audit history is retained under a regulatory hold; the security owner \
                     controls release.",
                ),
                None,
                Some(schedule(
                    DeleteOutcomeClass::Blocked,
                    "when the security owner releases the regulatory hold",
                    "the operational audit history",
                    ManagedCopy,
                    SecurityOwner,
                    "Nothing is deleted while the hold is active; the hold and its owner are named.",
                )),
                Some(recovery(
                    Diag::DeleteBlockedByHold,
                    "restore.self_hosted.delete_predelete_01",
                    "An active regulatory hold blocks the delete; the diagnostic names the hold, \
                     not a generic billing or sign-in error.",
                    "Resume the delete from the saved checkpoint once the security owner releases \
                     the hold; nothing else in the flow restarts.",
                    SecurityOwner,
                    "The blocked delete keeps its checkpoint, typed diagnostics, and next step so \
                     it is resumed rather than restarted.",
                )),
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle under legal-hold honesty rules.",
                "kind=delete scope=org outcome=blocked state=delete_blocked_by_hold \
                 confirmation_required=true managed_copies=retained_under_hold recovery=present",
                "The delete is blocked by a regulatory hold; it keeps a restore checkpoint, typed \
                 diagnostics, and a named next step so it resumes from here when the hold lifts.",
            ),
            checkpoint(
                "offboarding.checkpoint.self_hosted.local_continuation",
                LocalContinuation,
                "Continue locally",
                Team,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                CONTINUITY_SCHEMA,
                "Governed by the offboarding-continuity contract; local use survives deprovision.",
                "kind=local_continuation scope=team outcome=available_now \
                 state=export_available_now managed_copies=none_remaining",
                "Your local artifacts stay editable on this machine after deprovision, with no paid \
                 seat required.",
            ),
        ],
        AdminPathClass::SovereignAirGapped => vec![
            checkpoint(
                "offboarding.checkpoint.sovereign.review",
                ReviewArtifacts,
                "Review selected artifacts",
                Personal,
                Completed,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the review runs offline.",
                "kind=review_artifacts scope=personal outcome=completed state=active_enforced \
                 managed_copies=none_remaining",
                "Review the selected artifacts on the air-gapped install; the review runs fully \
                 offline.",
            ),
            checkpoint(
                "offboarding.checkpoint.sovereign.export",
                Export,
                "Export personal data offline",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle; export completes fully offline.",
                "kind=export scope=personal outcome=available_now state=export_available_now \
                 managed_copies=none_remaining",
                "Export your personal data now; on an air-gapped install the export completes fully \
                 offline with no paid seat.",
            ),
            checkpoint(
                "offboarding.checkpoint.sovereign.transfer",
                Transfer,
                "Transfer (boundary recheck required)",
                Workspace,
                FailedRecoverable,
                BoundaryChangedRecheckRequired,
                Stale,
                LocalOnly,
                WorkspaceOwner,
                false,
                mcopy(
                    TransferredToOwner,
                    "1 workspace copy",
                    LocalOnly,
                    "the workspace copy pending the boundary recheck",
                    "when the changed tenant boundary is re-verified",
                    OrgAdmin,
                    "The transfer pauses because the tenant boundary changed offline and must be \
                     re-verified.",
                ),
                Some(transfer_plan(
                    WorkspaceOwner,
                    "transfer.recipient.sovereign.ws_owner_03",
                    Workspace,
                    "The workspace copy is destined for the named workspace owner once the boundary \
                     is re-verified.",
                )),
                None,
                Some(recovery(
                    Diag::BoundaryRecheckRequired,
                    "restore.sovereign.transfer_pretransfer_01",
                    "The tenant boundary changed offline; the typed diagnostic names the boundary \
                     recheck, not a generic sign-in error.",
                    "Re-verify the recipient boundary, then resume the transfer from the saved \
                     checkpoint without restarting the flow.",
                    OrgAdmin,
                    "The failed transfer keeps its checkpoint, typed diagnostics, and next step so \
                     it is repaired rather than restarted from zero.",
                )),
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; a boundary change pauses the \
                 transfer.",
                "kind=transfer scope=workspace outcome=failed_recoverable \
                 state=boundary_changed_recheck_required managed_copies=transferred_to_owner \
                 recovery=present",
                "The transfer failed because the tenant boundary changed offline; it keeps a \
                 restore checkpoint, typed diagnostics, and a named next step so it resumes after \
                 the recheck.",
            ),
            checkpoint(
                "offboarding.checkpoint.sovereign.confirm",
                Confirm,
                "Confirm the offboarding plan",
                Org,
                AvailableNow,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                ComplianceOwner,
                true,
                mcopy(
                    RetainedUnderHold,
                    "1 sealed copy",
                    LocalOnly,
                    "the sealed evidence packet",
                    "when the offline hold seal is lifted by the compliance owner",
                    ComplianceOwner,
                    "Confirmation records the approver; the sealed copy stays under the offline \
                     hold.",
                ),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the confirmation gate is explicit.",
                "kind=confirm scope=org outcome=available_now state=active_enforced \
                 confirmation_required=true managed_copies=retained_under_hold",
                "Confirm the offboarding plan offline; the explicit checkpoint gates the delete and \
                 names the sealed hold that remains.",
            ),
            checkpoint(
                "offboarding.checkpoint.sovereign.delete",
                Delete,
                "Delete (blocked by sealed hold)",
                Org,
                Blocked,
                DeleteBlockedByHold,
                Recent,
                LocalOnly,
                ComplianceOwner,
                true,
                mcopy(
                    RetainedUnderHold,
                    "1 sealed copy",
                    LocalOnly,
                    "the sealed evidence packet",
                    "when the offline hold seal is lifted by the compliance owner",
                    ComplianceOwner,
                    "The sealed packet stays under the hold; the compliance owner controls the \
                     seal.",
                ),
                None,
                Some(schedule(
                    DeleteOutcomeClass::Blocked,
                    "when the offline hold seal is lifted by the compliance owner",
                    "the sealed evidence packet",
                    LocalOnly,
                    ComplianceOwner,
                    "Nothing is deleted while the seal is active; the seal and its owner are named.",
                )),
                Some(recovery(
                    Diag::DeleteBlockedByHold,
                    "restore.sovereign.delete_predelete_01",
                    "A sealed offline hold blocks the delete; the typed diagnostic names the hold \
                     seal rather than a generic error.",
                    "Resume the delete from the saved checkpoint once the compliance owner lifts \
                     the seal.",
                    ComplianceOwner,
                    "The blocked delete keeps its checkpoint, typed diagnostics, and next step for a \
                     clean resume.",
                )),
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle under sealed-hold honesty rules.",
                "kind=delete scope=org outcome=blocked state=delete_blocked_by_hold \
                 confirmation_required=true managed_copies=retained_under_hold recovery=present",
                "The delete is blocked by a sealed offline hold; it keeps a restore checkpoint, \
                 typed diagnostics, and a named next step so it resumes when the seal lifts.",
            ),
            checkpoint(
                "offboarding.checkpoint.sovereign.local_continuation",
                LocalContinuation,
                "Continue locally",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                CONTINUITY_SCHEMA,
                "Governed by the offboarding-continuity contract; local use survives offline.",
                "kind=local_continuation scope=personal outcome=available_now \
                 state=export_available_now managed_copies=none_remaining",
                "Local continuation is fully available offline and needs no paid seat on the \
                 air-gapped install.",
            ),
        ],
        AdminPathClass::MirroredOffline => vec![
            checkpoint(
                "offboarding.checkpoint.mirrored.review",
                ReviewArtifacts,
                "Review selected artifacts",
                Personal,
                Completed,
                ActiveEnforced,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; the review runs locally.",
                "kind=review_artifacts scope=personal outcome=completed state=active_enforced \
                 managed_copies=none_remaining",
                "Review the selected artifacts locally even with the mirror offline.",
            ),
            checkpoint(
                "offboarding.checkpoint.mirrored.export",
                Export,
                "Export user-owned data",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle; the local export works with the mirror \
                 offline.",
                "kind=export scope=personal outcome=available_now state=export_available_now \
                 managed_copies=none_remaining",
                "Export your user-owned data now; the local export works even with the mirror \
                 offline and needs no paid seat.",
            ),
            checkpoint(
                "offboarding.checkpoint.mirrored.transfer",
                Transfer,
                "Transfer (recipient unreachable offline)",
                Team,
                FailedRecoverable,
                MirrorOfflineLastKnown,
                Stale,
                MirroredCopy,
                OrgAdmin,
                false,
                mcopy(
                    RetainedUpstreamMirror,
                    "1 upstream copy",
                    ManagedCopy,
                    "the upstream team copy",
                    "when the mirror reconnects and the recipient is reachable",
                    OrgAdmin,
                    "The transfer cannot reach the recipient while the mirror is offline; the \
                     upstream copy persists.",
                ),
                Some(transfer_plan(
                    OrgAdmin,
                    "transfer.recipient.mirrored.org_admin_05",
                    Team,
                    "The team copy is destined for the org admin once the mirror reconnects.",
                )),
                None,
                Some(recovery(
                    Diag::TransferRecipientUnavailable,
                    "restore.mirrored.transfer_pretransfer_01",
                    "The transfer recipient is unreachable while the mirror is offline; the typed \
                     diagnostic names the recipient, not a generic sign-in error.",
                    "The transfer plan is saved; resume it from the checkpoint when the mirror \
                     reconnects.",
                    OrgAdmin,
                    "The failed transfer keeps its checkpoint, typed diagnostics, and next step so \
                     it resumes on reconnect rather than restarting.",
                )),
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; an offline mirror pauses the \
                 transfer.",
                "kind=transfer scope=team outcome=failed_recoverable state=mirror_offline_last_known \
                 managed_copies=retained_upstream_mirror recovery=present",
                "The transfer can't reach its recipient while the mirror is offline; it keeps a \
                 restore checkpoint, typed diagnostics, and a named next step and resumes on \
                 reconnect.",
            ),
            checkpoint(
                "offboarding.checkpoint.mirrored.confirm",
                Confirm,
                "Confirm the offboarding plan",
                Org,
                AvailableNow,
                ActiveEnforced,
                Fresh,
                MirroredCopy,
                OrgAdmin,
                true,
                mcopy(
                    RetainedUpstreamMirror,
                    "1 upstream copy",
                    ManagedCopy,
                    "the upstream managed copy queued for deletion",
                    "when the mirror reconnects to the control plane",
                    OrgAdmin,
                    "Confirmation is recorded locally; the upstream delete is queued for reconnect.",
                ),
                None,
                None,
                None,
                DEPROVISION_SCHEMA,
                "Governed by the deprovision-handoff contract; confirmation is recorded locally.",
                "kind=confirm scope=org outcome=available_now state=active_enforced \
                 confirmation_required=true managed_copies=retained_upstream_mirror",
                "Confirm the offboarding plan locally; the upstream delete is queued to complete \
                 when the mirror reconnects.",
            ),
            checkpoint(
                "offboarding.checkpoint.mirrored.delete",
                Delete,
                "Delete (deferred to reconnect)",
                Workspace,
                Deferred,
                DeletePending,
                Stale,
                MirroredCopy,
                OrgAdmin,
                true,
                mcopy(
                    RetainedUpstreamMirror,
                    "1 upstream copy",
                    ManagedCopy,
                    "the upstream managed copy",
                    "when the mirror reconnects to the control plane",
                    OrgAdmin,
                    "The local mirror entry is queued for deletion; the authoritative upstream copy \
                     is removed on reconnect.",
                ),
                None,
                Some(schedule(
                    DeleteOutcomeClass::Deferred,
                    "when the mirror reconnects to the control plane",
                    "the upstream managed copy",
                    ManagedCopy,
                    OrgAdmin,
                    "The delete is queued offline and completes upstream when the mirror reconnects.",
                )),
                None,
                LIFECYCLE_SCHEMA,
                "Governed by the export/delete lifecycle; a queued delete completes on reconnect.",
                "kind=delete scope=workspace outcome=deferred state=delete_pending \
                 confirmation_required=true managed_copies=retained_upstream_mirror",
                "After you confirm, the delete is queued offline and completes upstream when the \
                 mirror reconnects; nothing is shown as deleted before it is.",
            ),
            checkpoint(
                "offboarding.checkpoint.mirrored.local_continuation",
                LocalContinuation,
                "Continue locally",
                Personal,
                AvailableNow,
                ExportAvailableNow,
                Fresh,
                LocalOnly,
                LocalUser,
                false,
                no_managed_copy(),
                None,
                None,
                None,
                CONTINUITY_SCHEMA,
                "Governed by the offboarding-continuity contract; local use survives the offline \
                 mirror.",
                "kind=local_continuation scope=personal outcome=available_now \
                 state=export_available_now managed_copies=none_remaining",
                "Your local workspace stays editable offline with no paid seat while the mirror is \
                 offline.",
            ),
        ],
        _ => Vec::new(),
    }
}

fn build_export_forms(profile: AdminPathClass) -> Vec<ExportForm> {
    let profile_token = profile.as_str();
    vec![
        ExportForm {
            format: ExportFormatClass::MachineReadableJson,
            label: "Machine-readable summary".to_owned(),
            artifact_ref: format!("offboarding.export.{profile_token}.machine"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "Each checkpoint's kind, scope, outcome, state, managed-copies \
                          disposition, transfer owner, deletion schedule, and recovery as JSON \
                          summary objects, copyable or exportable for tooling."
                .to_owned(),
        },
        ExportForm {
            format: ExportFormatClass::PlainLanguageHandoff,
            label: "Plain-language handoff packet".to_owned(),
            artifact_ref: format!("offboarding.export.{profile_token}.handoff"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "The same checkpoints as reviewable plain-language sentences for a \
                          support, compliance, or procurement handoff, with no raw payloads."
                .to_owned(),
        },
    ]
}

fn build_continuity() -> Vec<ContinuityGuarantee> {
    let guarantee = |right: ContinuityRightClass, note: &str| ContinuityGuarantee {
        right,
        label: right.label().to_owned(),
        available_offline: true,
        requires_paid_seat: false,
        note: note.to_owned(),
    };
    vec![
        guarantee(
            ContinuityRightClass::ExportUserOwnedArtifacts,
            "User-owned artifacts stay exportable locally with no paid seat.",
        ),
        guarantee(
            ContinuityRightClass::ContinueLocalOnly,
            "The local-only workspace keeps working after the exit, offline.",
        ),
        guarantee(
            ContinuityRightClass::EditLocalArtifacts,
            "Local artifacts stay fully editable after offboarding.",
        ),
        guarantee(
            ContinuityRightClass::PublishLater,
            "Writes made offline are captured to publish later when managed access returns.",
        ),
    ]
}

fn build_coverage(profile: AdminPathClass) -> OffboardingCoverage {
    use AdminStateClass::*;
    use CompletenessClass::*;

    match profile {
        AdminPathClass::ManagedCloud => OffboardingCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Full offboarding flow — live".to_owned(),
            coverage_note: "The managed control plane is live; every checkpoint resolves and the \
                            whole flow is completable from inside the product."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
            completable_without_paid_seat: true,
        },
        AdminPathClass::SelfHosted => OffboardingCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Full offboarding flow — self-hosted".to_owned(),
            coverage_note: "The customer's own control plane governs the flow; the held delete is \
                            recoverable and the rest of the flow completes locally."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
            completable_without_paid_seat: true,
        },
        AdminPathClass::SovereignAirGapped => OffboardingCoverage {
            coverage_state: BoundaryChangedRecheckRequired,
            completeness: PartialImported,
            window_label: "Offline flow — boundary recheck pending".to_owned(),
            coverage_note:
                "Export and continuation complete offline; the boundary-changed transfer \
                            and the sealed-hold delete are labeled recoverable and blocked rather \
                            than implied complete."
                    .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
            completable_without_paid_seat: true,
        },
        AdminPathClass::MirroredOffline => OffboardingCoverage {
            coverage_state: MirrorOfflineLastKnown,
            completeness: PartialOffline,
            window_label: "Offline flow — mirror offline".to_owned(),
            coverage_note: "Local export and continuation complete now; the offline transfer and \
                            the deferred upstream delete are queued to resume on reconnect and are \
                            labeled, never shown done."
                .to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
            completable_without_paid_seat: true,
        },
        _ => OffboardingCoverage {
            coverage_state: ActiveEnforced,
            completeness: Complete,
            window_label: "Local".to_owned(),
            coverage_note: "Local offboarding flow.".to_owned(),
            locally_inspectable: true,
            vendor_console_independent: true,
            completable_without_paid_seat: true,
        },
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> OffboardingInvariant {
    OffboardingInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(profiles: &[OffboardingPacket]) -> Vec<OffboardingInvariant> {
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| -> bool {
        matrix
            .surface(AdminSurfaceClass::OffboardingWizard)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };
    let declared_consumers = parity_consumers();
    let all_checkpoints = || profiles.iter().flat_map(|p| p.wizard.checkpoints.iter());
    let all_triggers = || profiles.iter().flat_map(|p| p.wizard.triggers.iter());

    let mut out = Vec::new();

    // Every rendered state is one the matrix admits for this surface.
    out.push(invariant(
        "offboarding.surface_states_within_matrix",
        "Every state a checkpoint or the coverage posture shows is one the frozen admin-plane \
         matrix declares applicable for the offboarding surface, so the render layer cannot drift \
         from the contract.",
        profiles.iter().all(|p| {
            p.wizard
                .checkpoints
                .iter()
                .all(|c| admitted(c.machine_state))
                && admitted(p.wizard.coverage.coverage_state)
        }),
    ));

    // The flow is ordered and every checkpoint kind appears once per profile.
    out.push(invariant(
        "offboarding.checkpoints_ordered_and_complete",
        "Every profile renders exactly one checkpoint for each kind — review, export, transfer, \
         confirm, delete, and local continuation — in ascending order, so the exit is a complete \
         ordered flow rather than a loose set of links; checkpoint ids are unique.",
        all_unique(all_checkpoints().map(|c| c.checkpoint_id.as_str()))
            && profiles.iter().all(|p| {
                CheckpointKindClass::ALL.iter().all(|kind| {
                    p.wizard
                        .checkpoints
                        .iter()
                        .filter(|c| c.kind == *kind)
                        .count()
                        == 1
                }) && p
                    .wizard
                    .checkpoints
                    .windows(2)
                    .all(|w| w[0].order < w[1].order)
                    && p.wizard
                        .checkpoints
                        .iter()
                        .all(|c| c.order == c.kind.order())
            }),
    ));

    // No paid seat required to recover user-owned data.
    out.push(invariant(
        "offboarding.no_paid_seat_required",
        "No checkpoint, trigger, or coverage view requires a still-active paid seat to recover \
         user-owned data, so export, delete, and local-continuation stay reachable through \
         downgrade, seat loss, cancellation, and plan change.",
        all_checkpoints().all(|c| !c.requires_paid_seat)
            && all_triggers().all(|t| !t.requires_active_seat_for_recovery)
            && profiles
                .iter()
                .all(|p| p.wizard.coverage.completable_without_paid_seat),
    ));

    // Every trigger explains impacted features, export rights, local continuation,
    // and managed copies remaining; every trigger class appears across the bundle.
    out.push(invariant(
        "offboarding.triggers_explain_impact",
        "Every trigger — seat loss, cancellation, deprovision, org switch, or plan downgrade — \
         explains the impacted managed features, export rights, local-safe continuation, and \
         managed copies remaining in plain language, and every trigger class appears at least once \
         across the bundle.",
        all_triggers().all(|t| {
            !t.impacted_features.is_empty()
                && !t.export_rights.is_empty()
                && !t.local_safe_continuation.is_empty()
                && !t.managed_copies_summary.is_empty()
                && !t.plain_language.is_empty()
        }) && OffboardingTriggerClass::ALL.iter().all(|class| {
            profiles
                .iter()
                .any(|p| p.wizard.triggers.iter().any(|t| t.trigger == *class))
        }),
    ));

    // Scopes are distinguished and every scope appears across the bundle.
    out.push(invariant(
        "offboarding.scopes_distinguished",
        "Checkpoints name a specific personal, workspace, team, or org scope, and across the \
         bundle every scope appears at least once, so usage and ownership are not flattened into \
         one scope.",
        OffboardingScopeClass::ALL.iter().all(|scope| {
            profiles
                .iter()
                .any(|p| p.wizard.checkpoints.iter().any(|c| c.scope == *scope))
        }),
    ));

    // Confirmation checkpoints gate irreversible deletes.
    out.push(invariant(
        "offboarding.confirmation_gates_deletes",
        "Every profile has a confirm checkpoint that requires explicit confirmation, and every \
         delete checkpoint is confirmation-gated, so an irreversible delete is never run without \
         an explicit checkpoint.",
        profiles.iter().all(|p| {
            p.wizard
                .checkpoints
                .iter()
                .any(|c| c.kind == CheckpointKindClass::Confirm && c.confirmation_required)
                && p.wizard
                    .checkpoints
                    .iter()
                    .all(|c| c.kind != CheckpointKindClass::Delete || c.confirmation_required)
        }),
    ));

    // Managed-copies-remaining truth: a remaining copy names what/where/when/who.
    out.push(invariant(
        "offboarding.managed_copies_honest",
        "Every checkpoint states its managed-copies disposition; a checkpoint that leaves a managed \
         copy names what remains, where it remains, when it clears, and who controls it, rather \
         than implying everything is gone.",
        all_checkpoints().all(|c| {
            if c.managed_copies.remains() {
                !c.managed_copies.what_remains.is_empty()
                    && !c.managed_copies.cleared_when.is_empty()
            } else {
                true
            }
        }),
    ));

    // Failed/blocked flows retain a typed recovery; at least one failure appears.
    out.push(invariant(
        "offboarding.failed_flows_recoverable",
        "Every blocked or failed checkpoint retains a typed recovery — a restore checkpoint, a \
         typed diagnostic, and next-step guidance with the restore/diagnostics/next-step \
         affordances — so a failed export, transfer, or delete is repaired from a saved checkpoint \
         rather than collapsing into a generic error; at least one failed-recoverable checkpoint \
         appears across the bundle.",
        all_checkpoints().all(|c| {
            if c.outcome.requires_recovery() {
                c.recovery.as_ref().is_some_and(|r| {
                    !r.restore_checkpoint_ref.is_empty()
                        && !r.diagnostic_detail.is_empty()
                        && !r.next_step.is_empty()
                        && r.offers(RecoveryAffordanceClass::RestoreCheckpoint)
                        && r.offers(RecoveryAffordanceClass::RetainedDiagnostics)
                        && r.offers(RecoveryAffordanceClass::NextStepGuidance)
                })
            } else {
                true
            }
        }) && profiles.iter().any(|p| {
            p.wizard
                .checkpoints
                .iter()
                .any(|c| c.outcome == CheckpointOutcomeClass::FailedRecoverable)
        }),
    ));

    // Delete checkpoints carry a deletion schedule; non-immediate names a remainder.
    out.push(invariant(
        "offboarding.deletion_schedule_present",
        "Every delete checkpoint carries a deletion schedule with an immediate/deferred/blocked \
         outcome; a deferred or blocked schedule names what remains and when it completes, and \
         across the bundle all three delete outcomes appear.",
        profiles.iter().all(|p| {
            p.wizard.checkpoints.iter().all(|c| {
                c.kind != CheckpointKindClass::Delete
                    || c.deletion_schedule.as_ref().is_some_and(|s| {
                        if s.outcome.requires_remainder() {
                            !s.what_remains.is_empty() && !s.when.is_empty()
                        } else {
                            true
                        }
                    })
            })
        }) && DeleteOutcomeClass::ALL.iter().all(|outcome| {
            all_checkpoints().any(|c| {
                c.deletion_schedule
                    .as_ref()
                    .is_some_and(|s| s.outcome == *outcome)
            })
        }),
    ));

    // Transfer checkpoints name a transfer owner.
    out.push(invariant(
        "offboarding.transfer_named",
        "Every transfer checkpoint names a transfer plan with the owner ownership moves to, so a \
         transfer always says who receives the artifacts.",
        profiles.iter().all(|p| {
            p.wizard
                .checkpoints
                .iter()
                .all(|c| c.kind != CheckpointKindClass::Transfer || c.transfer.is_some())
        }),
    ));

    // Local-only continuation rights guaranteed on every profile.
    out.push(invariant(
        "offboarding.local_continuation_guaranteed",
        "Every profile guarantees all four local-only continuation rights — export user-owned \
         artifacts, continue local-only, edit local artifacts, and publish later — each available \
         offline and free of a paid seat, and renders a local-continuation checkpoint.",
        profiles.iter().all(|p| {
            ContinuityRightClass::ALL.iter().all(|right| {
                p.wizard
                    .continuity
                    .iter()
                    .any(|g| g.right == *right && g.available_offline && !g.requires_paid_seat)
            }) && p.wizard.has_kind(CheckpointKindClass::LocalContinuation)
        }),
    ));

    // Export parity: machine summary and plain-language on every checkpoint.
    out.push(invariant(
        "offboarding.export_parity",
        "Every checkpoint carries both an export-safe machine-readable summary and a plain-language \
         handoff sentence, and every wizard offers both export forms.",
        profiles.iter().all(|p| {
            p.wizard
                .checkpoints
                .iter()
                .all(OffboardingCheckpoint::has_export_parity)
                && p.wizard.offers(ExportFormatClass::MachineReadableJson)
                && p.wizard.offers(ExportFormatClass::PlainLanguageHandoff)
        }),
    ));

    // No-silent-green: stale evidence never sits under a confirmed positive state.
    out.push(invariant(
        "offboarding.no_silent_green",
        "A checkpoint whose backing evidence is stale is never shown under a confirmed \
         active/enforced, export-available-now, or receipted state; stale steps use an explicit \
         non-confirmed state instead.",
        all_checkpoints()
            .all(|c| !(c.evidence_age.is_stale() && requires_fresh_evidence(c.machine_state))),
    ));

    // Locally inspectable without a vendor console on every profile.
    out.push(invariant(
        "offboarding.locally_inspectable_offline",
        "Every profile — including self-hosted, sovereign/air-gapped, and mirrored/offline — keeps \
         a locally inspectable offboarding wizard that does not require a vendor console or control \
         plane and is completable without a paid seat.",
        profiles.iter().all(|p| {
            let coverage = &p.wizard.coverage;
            coverage.locally_inspectable
                && coverage.vendor_console_independent
                && coverage.completable_without_paid_seat
        }),
    ));

    // Partial flow views are labeled, never implied complete.
    out.push(invariant(
        "offboarding.coverage_labeled",
        "A flow view that is offline or boundary-pending is labeled with a non-complete \
         completeness class and a coverage note, so a partial flow is never presented as complete.",
        profiles.iter().all(|p| {
            let coverage = &p.wizard.coverage;
            !coverage.coverage_note.is_empty()
                && (!coverage.completeness.is_partial()
                    || coverage.coverage_state != AdminStateClass::ActiveEnforced)
        }),
    ));

    // Cross-surface parity: one typed packet serves every declared consumer.
    out.push(invariant(
        "offboarding.consumer_parity",
        "Each profile is one typed packet consumed identically by every consumer the matrix \
         declares for the offboarding surface, so the wizard is identical across UI, CLI, \
         Help/About, support export, and procurement surfaces by construction.",
        !declared_consumers.is_empty()
            && profiles
                .iter()
                .all(|p| declared_consumers.iter().all(|c| p.consumers.contains(c))),
    ));

    // Every claimed managed-bearing profile is rendered.
    out.push(invariant(
        "offboarding.profiles_covered",
        "The bundle renders the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        OFFBOARDING_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Checkpoint outcomes are all exercised — proof the distinction is real.
    out.push(invariant(
        "offboarding.outcomes_all_present",
        "Across the bundle every checkpoint outcome — completed, available-now, deferred, blocked, \
         and failed-recoverable — appears at least once, and every managed-copies disposition \
         appears, so the distinctions are real rather than collapsed.",
        CheckpointOutcomeClass::ALL.iter().all(|outcome| {
            all_checkpoints().any(|c| c.outcome == *outcome)
        }) && ManagedCopyDispositionClass::ALL.iter().all(|disposition| {
            all_checkpoints().any(|c| c.managed_copies.disposition == *disposition)
        }),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "offboarding.export_safe",
        "Every stable surface, profile, checkpoint, recipient, restore-checkpoint, and export id is \
         an opaque token with no URL scheme or absolute path, and every governing schema is a \
         repo-relative ref, so the bundle is safe to embed in a support export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && is_safe_token(p.wizard.surface_id.as_str())
                && p.wizard.checkpoints.iter().all(|c| {
                    let transfer_ok = match &c.transfer {
                        Some(t) => is_safe_token(t.recipient_ref.as_str()),
                        None => true,
                    };
                    let recovery_ok = match &c.recovery {
                        Some(r) => is_safe_token(r.restore_checkpoint_ref.as_str()),
                        None => true,
                    };
                    is_safe_token(c.checkpoint_id.as_str())
                        && is_export_safe_ref(c.governing_schema_ref.as_str())
                        && transfer_ok
                        && recovery_ok
                })
                && p.wizard
                    .export_forms
                    .iter()
                    .all(|x| is_safe_token(x.artifact_ref.as_str()))
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn offboarding_lines(bundle: &OffboardingBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Offboarding bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        let coverage = &p.wizard.coverage;
        lines.push(format!(
            "  Coverage: state={} completeness={} window={} local={} console_independent={} \
             seat_free={}",
            coverage.coverage_state.as_str(),
            coverage.completeness.as_str(),
            coverage.window_label,
            coverage.locally_inspectable,
            coverage.vendor_console_independent,
            coverage.completable_without_paid_seat,
        ));
        lines.push("  Triggers:".to_owned());
        for t in &p.wizard.triggers {
            lines.push(format!("    - {} [{}]", t.label, t.trigger.as_str()));
            lines.push(format!("        {}", t.plain_language));
        }
        lines.push("  Checkpoints:".to_owned());
        for c in &p.wizard.checkpoints {
            lines.push(format!(
                "    {}. {} [{}] scope={} outcome={} state={} age={} owner={} confirm={} \
                 managed_copies={}",
                c.order,
                c.label,
                c.kind.as_str(),
                c.scope.as_str(),
                c.outcome.as_str(),
                c.machine_state.as_str(),
                c.evidence_age.as_str(),
                c.owner.as_str(),
                c.confirmation_required,
                c.managed_copies.disposition.as_str(),
            ));
            lines.push(format!("        {}", c.plain_language));
            if let Some(t) = &c.transfer {
                lines.push(format!(
                    "        transfer → {} ({})",
                    t.transfer_owner.as_str(),
                    t.recipient_ref,
                ));
            }
            if let Some(s) = &c.deletion_schedule {
                lines.push(format!("        delete[{}] {}", s.outcome.as_str(), s.when,));
            }
            if let Some(r) = &c.recovery {
                lines.push(format!(
                    "        recovery[{}] restore={} → {}",
                    r.diagnostic.as_str(),
                    r.restore_checkpoint_ref,
                    r.next_step,
                ));
            }
        }
        lines.push("  Continuity rights:".to_owned());
        for g in &p.wizard.continuity {
            lines.push(format!(
                "    - {} (offline={} seat_free={})",
                g.label, g.available_offline, !g.requires_paid_seat,
            ));
        }
        lines.push("  Export forms:".to_owned());
        for x in &p.wizard.export_forms {
            lines.push(format!("    - {} [{}]", x.label, x.format.as_str()));
        }
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
