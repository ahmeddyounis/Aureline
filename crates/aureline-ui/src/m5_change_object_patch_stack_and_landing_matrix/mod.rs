//! Frozen M5 change-object, patch-stack/queue, stack-edit-review, landing-candidate, portable-shelf, and worktree-cleanup-preview matrix.
//!
//! This module locks Aureline's explicit change-orchestration model — the change object, the patch stack /
//! queue, the stack-edit / review sheet, the landing-candidate sheet, the portable shelf / bundle, and the
//! worktree cleanup preview that a Git / review / AI / provider-backed consumer must treat as first-class,
//! worktree-bound, landing-aware engineering objects rather than an emergent combination of ambient branch
//! state, scattered stashes, and provider merge queues — into one export-safe packet. Every covered object
//! class is named once here and constrained by the same shared change-orchestration role taxonomy
//! (selected_change_object_disclosure, worktree_binding_disclosure, stack_membership_disclosure,
//! landing_state_disclosure, validation_freshness_disclosure, rollback_export_fallback_disclosure,
//! cleanup_safety_disclosure), the same required visible state (surface label, selected change object,
//! worktree / base identity, stack membership and order, landing-state summary, cleanup safety, and validation
//! freshness), the same no-inferring-stack-membership-from-branch-names-alone rule, the same
//! no-mutating-another-worktree-without-a-selected-change-object-and-binding rule, the same
//! no-silently-reordering-collapsing-or-retargeting-stack-members rule, the same
//! no-landing-from-ambient-branch-state rule, and the same
//! no-deleting-orphaned-worktrees-or-stale-stack-members-without-previewing-running-work-and-export-safe-evidence
//! rule regardless of the surface that renders it.
//!
//! The matrix makes a queue-eligible landing candidate mechanically distinct from a selected change, a
//! stale-validation view, a restack-required stack, a queue-blocked or protected-branch-blocked candidate, and
//! an orphaned / abandoned / exported / imported-reopened object (see [`M5ChangeOrchestrationState`]) so the
//! change-object detail, the patch-stack queue, the stack-edit / review sheet, the landing-candidate sheet, the
//! worktree-manager row, the provider merge queue, and support / export packets can key off the landing state,
//! stack membership source, and cleanup safety rather than guessing from a generic status pill. It does not
//! widen M5 into a full Git engine, a code-host backend, or a provider merge queue — it reuses the already-landed
//! AI branch / worktree agent lifecycle, merge-readiness and stack-dependency chips, Git worktree / history /
//! rebase mutation review, work-item change-orchestration / start-work / handoff flows, review bundles, and provider
//! mutation boundaries — it is the shared reusable change-orchestration contract those consumers read, and it
//! binds back to the already-landed stable-proof-index, migration-task-row, and portable-bundle packets so
//! change-orchestration truth is not split across surfaces. The controlled vocabularies are frozen in one
//! self-describing [`M5ChangeOrchestrationVocabularySet`] rather than minted per surface. Raw paths, raw glob
//! bodies, raw command lines, raw provider payloads, secret values, and private endpoints stay outside the
//! export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_change_orchestration_matrix,
    seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed,
    seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed,
    M5_CHANGE_ORCHESTRATION_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ChangeOrchestrationMatrixPacket`].
pub const M5_CHANGE_ORCHESTRATION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_change_object_patch_stack_landing_candidate_shelf_and_worktree_cleanup_matrix";

/// Schema version for M5 change-orchestration matrix records.
pub const M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined change-orchestration lifecycle matrix schema.
pub const M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF: &str =
    "schemas/change/m5-change-orchestration-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF: &str = "docs/git/m5-change-orchestration-ops.md";

/// Repo-relative path of the canonical change-orchestration domain schema (the durable change-orchestration record with its
/// provider ownership, local-versus-provider state, and linked engineering identity).
pub const M5_CHANGE_OBJECT_DOMAIN_SCHEMA_REF: &str = "schemas/change/m5-change-object.schema.json";

/// Repo-relative path of the canonical start-work-sheet domain schema (the sheet that discloses each start-work
/// side effect separately).
pub const M5_PATCH_STACK_QUEUE_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-patch-stack-queue.schema.json";

/// Repo-relative path of the canonical linked-change-panel domain schema (the relation strip keeping the four
/// relation sources distinct).
pub const M5_STACK_EDIT_REVIEW_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-stack-edit-review-sheet.schema.json";

/// Repo-relative path of the canonical ready-for-review-handoff-sheet domain schema (validation evidence plus a
/// publish-later fallback).
pub const M5_LANDING_CANDIDATE_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-landing-candidate-sheet.schema.json";

/// Repo-relative path of the canonical resolve-close-sheet domain schema (final-resolution authority plus
/// unresolved-blocker state).
pub const M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF: &str =
    "schemas/change/m5-portable-shelf.schema.json";

/// Repo-relative path of the canonical blocked-escalate-card domain schema (the engineering blocker and its
/// escalation path).
pub const M5_WORKTREE_MANAGER_ROW_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-worktree-manager-row.schema.json";

/// Repo-relative path of the work-item handoff-packet schema the matrix references for offline handoff continuity.
pub const M5_PORTABLE_BUNDLE_LANDED_SCHEMA_REF: &str = "schemas/change/portable_bundle.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHANGE_ORCHESTRATION_FIXTURE_DIR: &str = "fixtures/git/m5-change-orchestration";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHANGE_ORCHESTRATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-orchestration-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CHANGE_ORCHESTRATION_CSV_REF: &str =
    "artifacts/release/m5-change-orchestration-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CHANGE_ORCHESTRATION_REPORT_REF: &str =
    "artifacts/design/m5-change-orchestration-component-matrix.md";

/// Repo-relative path of the checked change-orchestration-health dashboard.
pub const M5_CHANGE_ORCHESTRATION_DASHBOARD_REF: &str =
    "dashboards/m5-change-orchestration-health.json";

/// One of the six governed change-orchestration object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationObject {
    /// A change object: the explicit object a non-trivial multi-file change binds to, carrying its selected worktree / base identity, working-set-patch-versus-side-branch kind, stack membership, landing state, and validation freshness.
    ChangeObject,
    /// A patch stack / queue: the ordered stack or merge / landing queue that shows member order, queue eligibility, any queue-blocked reason, and stack dependency edges instead of inferring membership from branch names.
    PatchStackQueue,
    /// A stack-edit / review sheet: the sheet that edits and reviews a stack, keeping declared-in-change-object, declared-locally, inferred-from-branch-name, and stale-or-broken membership distinct instead of one generic badge.
    StackEditReviewSheet,
    /// A landing-candidate sheet: the reviewed candidate that packages validation freshness, the protected-branch gate, and a rollback / export fallback, never letting ambient branch state read as a reviewed landing candidate.
    LandingCandidateSheet,
    /// A portable shelf / bundle: the object that exports, imports, and reopens a change object with its bundle contents, lineage, and recovery checkpoint, never dropping shelf contents on an export failure.
    PortableShelf,
    /// A worktree cleanup preview: the worktree-manager row / orphan-cleanup preview that names the cleanup target and previews running tasks, open editors, uncommitted changes, and recovery checkpoints before any deletion.
    WorktreeCleanupPreview,
}

impl M5ChangeOrchestrationObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChangeObject,
        Self::PatchStackQueue,
        Self::StackEditReviewSheet,
        Self::LandingCandidateSheet,
        Self::PortableShelf,
        Self::WorktreeCleanupPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeObject => "change_object",
            Self::PatchStackQueue => "patch_stack_queue",
            Self::StackEditReviewSheet => "stack_edit_review_sheet",
            Self::LandingCandidateSheet => "landing_candidate_sheet",
            Self::PortableShelf => "portable_shelf",
            Self::WorktreeCleanupPreview => "worktree_cleanup_preview",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's change-orchestration, start-work, linked-change, handoff, resolve, or blocker meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ChangeObject => M5_CHANGE_OBJECT_DOMAIN_SCHEMA_REF,
            Self::PatchStackQueue => M5_PATCH_STACK_QUEUE_DOMAIN_SCHEMA_REF,
            Self::StackEditReviewSheet => M5_STACK_EDIT_REVIEW_SHEET_DOMAIN_SCHEMA_REF,
            Self::LandingCandidateSheet => M5_LANDING_CANDIDATE_SHEET_DOMAIN_SCHEMA_REF,
            Self::PortableShelf => M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
            Self::WorktreeCleanupPreview => M5_WORKTREE_MANAGER_ROW_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled change orchestration record role.
    pub const fn declares_change_object_roles(self) -> bool {
        matches!(self, Self::ChangeObject)
    }

    /// `true` when this class must name a controlled start work role.
    pub const fn declares_patch_stack_queue_roles(self) -> bool {
        matches!(self, Self::PatchStackQueue)
    }

    /// `true` when this class must name a controlled linked change role.
    pub const fn declares_stack_edit_review_roles(self) -> bool {
        matches!(self, Self::StackEditReviewSheet)
    }

    /// `true` when this class must name a controlled handoff role.
    pub const fn declares_landing_candidate_roles(self) -> bool {
        matches!(self, Self::LandingCandidateSheet)
    }

    /// `true` when this class must name a controlled resolve role.
    pub const fn declares_portable_shelf_roles(self) -> bool {
        matches!(self, Self::PortableShelf)
    }

    /// `true` when this class must name a controlled blocked escalate role.
    pub const fn declares_worktree_cleanup_roles(self) -> bool {
        matches!(self, Self::WorktreeCleanupPreview)
    }
}

/// The single controlled change-orchestration role vocabulary every work-item, start-work, review, provider handoff, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationRole {
    /// The provider ownership of the tracked work item disclosed on every claimed surface.
    SelectedChangeObjectDisclosure,
    /// The local-versus-provider commit state disclosed so a local draft or queued publish never reads as a provider-committed update.
    WorktreeBindingDisclosure,
    /// The linked branch / worktree / review identity disclosed so intent joins back to concrete engineering artifacts.
    StackMembershipDisclosure,
    /// Each start-work side effect (branch, worktree, review draft, provider link) disclosed separately, never silently created.
    LandingStateDisclosure,
    /// The validation evidence packaged with a handoff disclosed as an explicit set.
    ValidationFreshnessDisclosure,
    /// The publish-later / queued-publish fallback disclosed so a deferred update is never mistaken for a committed one.
    RollbackExportFallbackDisclosure,
    /// The final-resolution authority and any unresolved blocker disclosed before a tracked item is resolved or closed.
    CleanupSafetyDisclosure,
}

impl M5ChangeOrchestrationRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SelectedChangeObjectDisclosure,
        Self::WorktreeBindingDisclosure,
        Self::StackMembershipDisclosure,
        Self::LandingStateDisclosure,
        Self::ValidationFreshnessDisclosure,
        Self::RollbackExportFallbackDisclosure,
        Self::CleanupSafetyDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedChangeObjectDisclosure => "selected_change_object_disclosure",
            Self::WorktreeBindingDisclosure => "worktree_binding_disclosure",
            Self::StackMembershipDisclosure => "stack_membership_disclosure",
            Self::LandingStateDisclosure => "landing_state_disclosure",
            Self::ValidationFreshnessDisclosure => "validation_freshness_disclosure",
            Self::RollbackExportFallbackDisclosure => "rollback_export_fallback_disclosure",
            Self::CleanupSafetyDisclosure => "cleanup_safety_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a change-orchestration result (`selected_change_object_disclosure`,
    /// `worktree_binding_disclosure`, `stack_membership_disclosure`,
    /// `landing_state_disclosure`). The contextual roles (`validation_freshness_disclosure`,
    /// `rollback_export_fallback_disclosure`, `cleanup_safety_disclosure`) apply where the
    /// object class calls for them.
    pub const fn must_be_present_before_surfacing_as_a_change_orchestration_result(self) -> bool {
        matches!(
            self,
            Self::SelectedChangeObjectDisclosure
                | Self::WorktreeBindingDisclosure
                | Self::StackMembershipDisclosure
                | Self::LandingStateDisclosure
        )
    }
}

/// Change-orchestration state that makes a queue-eligible landing candidate mechanically distinct from a selected working change, a stale-validation view, a restack-required stack, a queue-blocked or protected-branch-blocked candidate, an orphaned or abandoned worktree, and an exported or imported/reopened shelf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationState {
    /// Selected change: an explicit change object is selected with its worktree / base identity bound.
    SelectedChange,
    /// Stale validation: the change object's validation is stale relative to its current contents and must be re-run.
    StaleValidation,
    /// Restack required: the patch stack drifted and its members must be restacked before review or landing.
    RestackRequired,
    /// Queue eligible: the landing candidate passed its gates and is eligible for the merge / landing queue.
    QueueEligible,
    /// Queue blocked: the landing candidate is held out of the queue by an unmet dependency or a failing gate.
    QueueBlocked,
    /// Protected-branch blocked: the landing target is a protected branch that blocks a direct land.
    ProtectedBranchBlocked,
    /// Orphaned: the worktree or stack member no longer maps to a live change object and is a cleanup candidate.
    Orphaned,
    /// Abandoned: the change object was explicitly abandoned and retained only for recovery / export.
    Abandoned,
    /// Exported: the change object was exported to a portable shelf / bundle for handoff.
    Exported,
    /// Imported or reopened: a portable shelf / bundle was imported or a change object was reopened from one.
    ImportedReopened,
}

impl M5ChangeOrchestrationState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::SelectedChange,
        Self::StaleValidation,
        Self::RestackRequired,
        Self::QueueEligible,
        Self::QueueBlocked,
        Self::ProtectedBranchBlocked,
        Self::Orphaned,
        Self::Abandoned,
        Self::Exported,
        Self::ImportedReopened,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedChange => "selected_change",
            Self::StaleValidation => "stale_validation",
            Self::RestackRequired => "restack_required",
            Self::QueueEligible => "queue_eligible",
            Self::QueueBlocked => "queue_blocked",
            Self::ProtectedBranchBlocked => "protected_branch_blocked",
            Self::Orphaned => "orphaned",
            Self::Abandoned => "abandoned",
            Self::Exported => "exported",
            Self::ImportedReopened => "imported_reopened",
        }
    }
    /// `true` only for the queue-eligible state, so downstream change-object detail, the patch-stack queue,
    /// the landing-candidate sheet, the provider merge queue, and support / export packets can key off a
    /// queue-eligible landing candidate rather than confusing it with a selected change, a restack-required
    /// stack, a queue-blocked or protected-branch-blocked candidate, or an orphaned worktree.
    pub const fn is_queue_eligible(self) -> bool {
        matches!(self, Self::QueueEligible)
    }
}

/// Named relation source (linked by provider, linked locally, suggested by Aureline, stale or broken relation) so the four relation kinds are never flattened into one generic relation badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationStackMembershipSource {
    /// Linked by the provider: an authoritative provider-side link between the tracked item and the change.
    DeclaredInChangeObject,
    /// Linked locally: a link recorded on this machine that has not been confirmed by the provider.
    DeclaredLocally,
    /// Suggested by Aureline: an inferred relation offered as a suggestion, not an established link.
    InferredFromBranchName,
    /// A stale or broken relation: a previously linked change whose target moved or no longer resolves.
    StaleOrBrokenMembership,
}

impl M5ChangeOrchestrationStackMembershipSource {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DeclaredInChangeObject,
        Self::DeclaredLocally,
        Self::InferredFromBranchName,
        Self::StaleOrBrokenMembership,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredInChangeObject => "declared_in_change_object",
            Self::DeclaredLocally => "declared_locally",
            Self::InferredFromBranchName => "inferred_from_branch_name",
            Self::StaleOrBrokenMembership => "stale_or_broken_membership",
        }
    }
    /// `true` only for a provider-side link, so a consumer can mechanically refuse to flatten a
    /// locally linked, suggested, or stale relation into a provider-authoritative link badge.
    pub const fn is_explicitly_declared_membership(self) -> bool {
        matches!(self, Self::DeclaredInChangeObject)
    }
}

/// Named blocker / resolution state (ready to resolve, blocked by engineering, escalation open, awaiting provider write, resolution authority missing) so no claimed surface lacks a named state for an unresolved engineering blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationCleanupSafety {
    /// Ready to resolve: no engineering blocker remains and the tracked item may be resolved or closed.
    ClearToLand,
    /// Blocked by engineering: an unresolved engineering blocker prevents resolution.
    BlockedByStaleValidation,
    /// An open escalation: the blocker has been escalated and is awaiting a decision.
    BlockedByRestackRequired,
    /// Awaiting provider write: the resolution is captured locally but a provider write is still pending.
    BlockedByQueueDependency,
    /// Resolution authority missing: no actor with final-resolution authority has confirmed the close.
    BlockedByProtectedBranch,
}

impl M5ChangeOrchestrationCleanupSafety {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ClearToLand,
        Self::BlockedByStaleValidation,
        Self::BlockedByRestackRequired,
        Self::BlockedByQueueDependency,
        Self::BlockedByProtectedBranch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearToLand => "clear_to_land",
            Self::BlockedByStaleValidation => "blocked_by_stale_validation",
            Self::BlockedByRestackRequired => "blocked_by_restack_required",
            Self::BlockedByQueueDependency => "blocked_by_queue_dependency",
            Self::BlockedByProtectedBranch => "blocked_by_protected_branch",
        }
    }
    /// `true` for the blocked / escalated / pending / authority-missing states (`blocked_by_stale_validation`,
    /// `blocked_by_restack_required`, `blocked_by_queue_dependency`, `blocked_by_protected_branch`) so a consumer can
    /// mechanically refuse to auto-resolve tracked work while an engineering blocker remains.
    pub const fn is_blocked_from_landing(self) -> bool {
        matches!(
            self,
            Self::BlockedByStaleValidation
                | Self::BlockedByRestackRequired
                | Self::BlockedByQueueDependency
                | Self::BlockedByProtectedBranch
        )
    }
}

/// Controlled change-orchestration-record role for one tracked work item's change orchestration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeObjectRole {
    /// Provider ownership shown so a tracked item names who owns it upstream.
    SelectedWorktreeBaseIdentityShown,
    /// Linked branch / worktree / review identity named so intent binds to concrete artifacts.
    WorkingSetScopeNamed,
    /// Local-versus-provider commit state shown so a local draft never reads as committed.
    ChangeObjectKindShown,
    /// Intent lifecycle stage shown so a record states where it sits from captured to resolved.
    ChangeObjectValidationFreshnessShown,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Silently swapping the provider link or ownership without disclosure, which is disallowed.
    CrossWorktreeWriteWithoutBindingDisallowed,
}

impl M5ChangeObjectRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectedWorktreeBaseIdentityShown,
        Self::WorkingSetScopeNamed,
        Self::ChangeObjectKindShown,
        Self::ChangeObjectValidationFreshnessShown,
        Self::BoundToChangeOrchestrationRegistry,
        Self::CrossWorktreeWriteWithoutBindingDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedWorktreeBaseIdentityShown => "selected_worktree_base_identity_shown",
            Self::WorkingSetScopeNamed => "working_set_scope_named",
            Self::ChangeObjectKindShown => "change_object_kind_shown",
            Self::ChangeObjectValidationFreshnessShown => {
                "change_object_validation_freshness_shown"
            }
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::CrossWorktreeWriteWithoutBindingDisallowed => {
                "cross_worktree_write_without_binding_disallowed"
            }
        }
    }
}

/// Controlled start-work-sheet role for launching work with disclosed side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationPatchStackQueueRole {
    /// The branch this start-work sheet would create disclosed separately.
    StackMemberOrderShown,
    /// The worktree this start-work sheet would create disclosed separately.
    QueueEligibilityShown,
    /// The review draft this start-work sheet would create disclosed separately.
    QueueBlockedReasonShown,
    /// The provider link this start-work sheet would create disclosed separately.
    StackDependencyEdgesShown,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Silently creating a branch, worktree, review draft, or provider link without disclosing each side effect, which is disallowed.
    SilentStackReorderDisallowed,
}

impl M5ChangeOrchestrationPatchStackQueueRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StackMemberOrderShown,
        Self::QueueEligibilityShown,
        Self::QueueBlockedReasonShown,
        Self::StackDependencyEdgesShown,
        Self::BoundToChangeOrchestrationRegistry,
        Self::SilentStackReorderDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StackMemberOrderShown => "stack_member_order_shown",
            Self::QueueEligibilityShown => "queue_eligibility_shown",
            Self::QueueBlockedReasonShown => "queue_blocked_reason_shown",
            Self::StackDependencyEdgesShown => "stack_dependency_edges_shown",
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::SilentStackReorderDisallowed => "silent_stack_reorder_disallowed",
        }
    }
}

/// Controlled linked-change-panel role for the relation source of a linked change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationStackEditReviewRole {
    /// Relation source shown so linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken stay distinct.
    StackMembershipSourceShown,
    /// Linked target identity named so a relation points at an exact change.
    RestackRequiredFlagged,
    /// Stale-or-broken relation flagged so a dead link never reads as live.
    StaleOrBrokenMembershipFlagged,
    /// A suggested relation labelled as a suggestion so it never reads as an established link.
    InferredMembershipLabelledAsInferred,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Flattening linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one generic relation badge, which is disallowed.
    StackMembershipSourcesFlattenedDisallowed,
}

impl M5ChangeOrchestrationStackEditReviewRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StackMembershipSourceShown,
        Self::RestackRequiredFlagged,
        Self::StaleOrBrokenMembershipFlagged,
        Self::InferredMembershipLabelledAsInferred,
        Self::BoundToChangeOrchestrationRegistry,
        Self::StackMembershipSourcesFlattenedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StackMembershipSourceShown => "stack_membership_source_shown",
            Self::RestackRequiredFlagged => "restack_required_flagged",
            Self::StaleOrBrokenMembershipFlagged => "stale_or_broken_membership_flagged",
            Self::InferredMembershipLabelledAsInferred => {
                "inferred_membership_labelled_as_inferred"
            }
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::StackMembershipSourcesFlattenedDisallowed => {
                "stack_membership_sources_flattened_disallowed"
            }
        }
    }
}

/// Controlled ready-for-review-handoff role for packaging a review handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationLandingCandidateRole {
    /// Validation evidence shown so a handoff names the checks that back it.
    LandingValidationFreshnessShown,
    /// Publish-later fallback shown so a deferred handoff states it is not yet committed.
    ProtectedBranchGateShown,
    /// A local handoff packet labelled as local so it never reads as a provider-committed update.
    AmbientBranchStateLabelledAsNotReviewed,
    /// Handoff destination named so a review handoff states where it is going.
    LandingTargetNamed,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Letting a local handoff packet or queued publish masquerade as a provider-committed update, which is disallowed.
    LandingFromAmbientBranchStateDisallowed,
}

impl M5ChangeOrchestrationLandingCandidateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LandingValidationFreshnessShown,
        Self::ProtectedBranchGateShown,
        Self::AmbientBranchStateLabelledAsNotReviewed,
        Self::LandingTargetNamed,
        Self::BoundToChangeOrchestrationRegistry,
        Self::LandingFromAmbientBranchStateDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LandingValidationFreshnessShown => "landing_validation_freshness_shown",
            Self::ProtectedBranchGateShown => "protected_branch_gate_shown",
            Self::AmbientBranchStateLabelledAsNotReviewed => {
                "ambient_branch_state_labelled_as_not_reviewed"
            }
            Self::LandingTargetNamed => "landing_target_named",
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::LandingFromAmbientBranchStateDisallowed => {
                "landing_from_ambient_branch_state_disallowed"
            }
        }
    }
}

/// Controlled resolve-or-close-sheet role for recording final resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationPortableShelfRole {
    /// Final-resolution authority shown so a close names who confirmed it.
    ExportBundleContentsShown,
    /// Unresolved engineering blocker shown so a resolution never hides an open blocker.
    ImportReopenLineageShown,
    /// Resolution outcome named so a close states resolved-versus-closed intent.
    ShelfStateNamed,
    /// Provider-write-pending state shown so a locally captured resolution never reads as committed.
    RecoveryCheckpointShown,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Auto-resolving tracked work while engineering blockers remain unresolved, which is disallowed.
    ShelfContentsDroppedOnExportFailureDisallowed,
}

impl M5ChangeOrchestrationPortableShelfRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExportBundleContentsShown,
        Self::ImportReopenLineageShown,
        Self::ShelfStateNamed,
        Self::RecoveryCheckpointShown,
        Self::BoundToChangeOrchestrationRegistry,
        Self::ShelfContentsDroppedOnExportFailureDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportBundleContentsShown => "export_bundle_contents_shown",
            Self::ImportReopenLineageShown => "import_reopen_lineage_shown",
            Self::ShelfStateNamed => "shelf_state_named",
            Self::RecoveryCheckpointShown => "recovery_checkpoint_shown",
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::ShelfContentsDroppedOnExportFailureDisallowed => {
                "shelf_contents_dropped_on_export_failure_disallowed"
            }
        }
    }
}

/// Controlled blocked-or-escalate-card role for surfacing an engineering blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationWorktreeCleanupRole {
    /// Blocker cause named so an escalation states what is blocked.
    CleanupTargetNamed,
    /// Escalation path shown so a blocked card names where the escalation goes.
    RunningTasksAndOpenEditorsPreviewed,
    /// Local notes and linked evidence retained so nothing is dropped when a blocker is raised.
    UncommittedChangesAndCheckpointsPreviewed,
    /// Blocker state shown so a card states blocked-versus-escalated-versus-ready.
    CleanupStateShown,
    /// A role bound to the single change-orchestration registry.
    BoundToChangeOrchestrationRegistry,
    /// Dropping local notes, handoff packets, or linked evidence when provider write fails, which is disallowed.
    DeletingWithoutPreviewingRunningWorkDisallowed,
}

impl M5ChangeOrchestrationWorktreeCleanupRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CleanupTargetNamed,
        Self::RunningTasksAndOpenEditorsPreviewed,
        Self::UncommittedChangesAndCheckpointsPreviewed,
        Self::CleanupStateShown,
        Self::BoundToChangeOrchestrationRegistry,
        Self::DeletingWithoutPreviewingRunningWorkDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanupTargetNamed => "cleanup_target_named",
            Self::RunningTasksAndOpenEditorsPreviewed => "running_tasks_and_open_editors_previewed",
            Self::UncommittedChangesAndCheckpointsPreviewed => {
                "uncommitted_changes_and_checkpoints_previewed"
            }
            Self::CleanupStateShown => "cleanup_state_shown",
            Self::BoundToChangeOrchestrationRegistry => "bound_to_change_orchestration_registry",
            Self::DeletingWithoutPreviewingRunningWorkDisallowed => {
                "deleting_without_previewing_running_work_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a change-orchestration object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationSurfaceFamily {
    /// The work-item surface (work-item rows and detail headers).
    GitSurface,
    /// The start-work surface (branch / worktree creation and side-effect disclosure).
    StackQueueSurface,
    /// The review surface (ready-for-review handoff and review detail).
    Review,
    /// The provider handoff / open-in-provider / publish-later surface.
    ProviderLanding,
    /// The support / export surface.
    SupportExport,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ChangeOrchestrationSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GitSurface,
        Self::StackQueueSurface,
        Self::Review,
        Self::ProviderLanding,
        Self::SupportExport,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitSurface => "git_surface",
            Self::StackQueueSurface => "stack_queue_surface",
            Self::Review => "review",
            Self::ProviderLanding => "provider_landing",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Classification stage a class passes through from intent capture to a work-started, change-linked, handoff-packaged, and resolution-recorded change-orchestration object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationClassificationStage {
    /// The intent-captured stage: the tracked work item's change orchestration and provider ownership are captured.
    ChangeObjectSelected,
    /// The work-started stage: the branch / worktree / review draft / provider link side effects are disclosed and created.
    StackAssembled,
    /// The change-linked stage: the branch / review relation is linked with its relation source.
    StackReviewed,
    /// The handoff-packaged stage: validation evidence and the publish-later fallback are packaged for review.
    LandingEvaluated,
    /// The resolution-recorded stage: the final-resolution authority and any unresolved blocker are recorded.
    ShelvedOrCleaned,
}

impl M5ChangeOrchestrationClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ChangeObjectSelected,
        Self::StackAssembled,
        Self::StackReviewed,
        Self::LandingEvaluated,
        Self::ShelvedOrCleaned,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeObjectSelected => "change_object_selected",
            Self::StackAssembled => "stack_assembled",
            Self::StackReviewed => "stack_reviewed",
            Self::LandingEvaluated => "landing_evaluated",
            Self::ShelvedOrCleaned => "shelved_or_cleaned",
        }
    }
}

/// Shared consumer surface that must agree on a class's change-orchestration truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationConsumerSurface {
    /// The work-item detail surface.
    ChangeObjectDetail,
    /// The start-work sheet.
    PatchStackQueue,
    /// The linked-change panel.
    StackEditReviewSheet,
    /// The review detail surface.
    ReviewDetail,
    /// The ready-for-review handoff surface.
    ProviderMergeQueue,
    /// The resolve-or-close sheet.
    PortableShelf,
    /// The blocked-or-escalate card.
    WorktreeCleanupPreview,
    /// The support / export packet.
    SupportExportPacket,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ChangeOrchestrationConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ChangeObjectDetail,
        Self::PatchStackQueue,
        Self::StackEditReviewSheet,
        Self::ReviewDetail,
        Self::ProviderMergeQueue,
        Self::PortableShelf,
        Self::WorktreeCleanupPreview,
        Self::SupportExportPacket,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeObjectDetail => "change_object_detail",
            Self::PatchStackQueue => "patch_stack_queue",
            Self::StackEditReviewSheet => "stack_edit_review_sheet",
            Self::ReviewDetail => "review_detail",
            Self::ProviderMergeQueue => "provider_merge_queue",
            Self::PortableShelf => "portable_shelf",
            Self::WorktreeCleanupPreview => "worktree_cleanup_preview",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no change-orchestration meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5ChangeOrchestrationAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified change-orchestration-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationDegradedReason {
    /// The provider ownership of the tracked item is unresolved.
    SelectedChangeObjectUnresolved,
    /// The local-versus-provider commit state is unknown.
    WorktreeBindingUnknown,
    /// The linked branch / worktree / review identity is unresolved.
    StackMembershipUnresolved,
    /// One or more start-work side effects are undisclosed.
    LandingStateDisclosureIncomplete,
    /// The relation source of a linked change is unknown.
    LandingStateUnknown,
    /// The blocker / resolution-authority state is unknown.
    CleanupSafetyUnknown,
}

impl M5ChangeOrchestrationDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectedChangeObjectUnresolved,
        Self::WorktreeBindingUnknown,
        Self::StackMembershipUnresolved,
        Self::LandingStateDisclosureIncomplete,
        Self::LandingStateUnknown,
        Self::CleanupSafetyUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedChangeObjectUnresolved => "selected_change_object_unresolved",
            Self::WorktreeBindingUnknown => "worktree_binding_unknown",
            Self::StackMembershipUnresolved => "stack_membership_unresolved",
            Self::LandingStateDisclosureIncomplete => "landing_state_disclosure_incomplete",
            Self::LandingStateUnknown => "landing_state_unknown",
            Self::CleanupSafetyUnknown => "cleanup_safety_unknown",
        }
    }
}

/// Mandatory label a claimed change-orchestration class must be able to show. The first three are hard requirements; the remaining three make the local-versus-provider commit state, the relation source, and the blocker state mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's change-orchestration lifecycle role.
    LifecycleRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The local-versus-provider commit state the class must show.
    LandingState,
    /// The relation source the class must state.
    StackMembershipSource,
    /// The blocker / resolution state the class must state.
    CleanupSafety,
}

impl M5ChangeOrchestrationRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
        Self::LandingState,
        Self::StackMembershipSource,
        Self::CleanupSafety,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::LifecycleRole => "lifecycle_role",
            Self::CanonicalReference => "canonical_reference",
            Self::LandingState => "landing_state",
            Self::StackMembershipSource => "stack_membership_source",
            Self::CleanupSafety => "cleanup_safety",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 change-orchestration row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationQualificationClass {
    /// Class change-orchestration handling qualifies for the Stable claim.
    Stable,
    /// Class change-orchestration handling is narrowed to Beta.
    Beta,
    /// Class change-orchestration handling is narrowed to Preview.
    Preview,
    /// Class change-orchestration handling is experimental and not claimed.
    Experimental,
    /// Class change-orchestration handling is unavailable on this build.
    Unavailable,
    /// Class change-orchestration handling is held pending review.
    Held,
}

impl M5ChangeOrchestrationQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
    /// Whether the class may carry a public Stable change-orchestration-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a change-orchestration class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationDowngradeTrigger {
    /// A start-work side effect (branch, worktree, review draft, or provider link) was created without disclosure.
    StackMembershipInferredFromBranchNameAlone,
    /// A local handoff packet or queued publish was shown as a provider-committed update.
    CrossWorktreeWriteWithoutSelectedChangeObject,
    /// Linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken were flattened into one relation badge.
    StackMembersSilentlyReordered,
    /// Tracked work was auto-resolved while an engineering blocker remained unresolved.
    LandedFromAmbientBranchState,
    /// Local notes, a handoff packet, or linked evidence were dropped when a provider write failed.
    OrphanDeletedWithoutSafetyPreview,
    /// A class left its provider ownership unstated.
    SelectedChangeObjectUnstated,
    /// A class left its local-versus-provider commit state unstated.
    WorktreeBindingUnstated,
    /// A class left its linked branch / worktree / review identity unstated.
    StackMembershipSourceUnstated,
    /// A class left its relation source unstated.
    StackOrderUnstated,
    /// A class left its blocker / resolution state unstated.
    LandingStateUnstated,
    /// A class left its validation evidence unstated.
    ValidationFreshnessUnstated,
    /// The change-orchestration matrix packet has gone stale.
    ChangeOrchestrationMatrixStale,
}

impl M5ChangeOrchestrationDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::StackMembershipInferredFromBranchNameAlone,
        Self::CrossWorktreeWriteWithoutSelectedChangeObject,
        Self::StackMembersSilentlyReordered,
        Self::LandedFromAmbientBranchState,
        Self::OrphanDeletedWithoutSafetyPreview,
        Self::SelectedChangeObjectUnstated,
        Self::WorktreeBindingUnstated,
        Self::StackMembershipSourceUnstated,
        Self::StackOrderUnstated,
        Self::LandingStateUnstated,
        Self::ValidationFreshnessUnstated,
        Self::ChangeOrchestrationMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StackMembershipInferredFromBranchNameAlone => {
                "stack_membership_inferred_from_branch_name_alone"
            }
            Self::CrossWorktreeWriteWithoutSelectedChangeObject => {
                "cross_worktree_write_without_selected_change_object"
            }
            Self::StackMembersSilentlyReordered => "stack_members_silently_reordered",
            Self::LandedFromAmbientBranchState => "landed_from_ambient_branch_state",
            Self::OrphanDeletedWithoutSafetyPreview => "orphan_deleted_without_safety_preview",
            Self::SelectedChangeObjectUnstated => "selected_change_object_unstated",
            Self::WorktreeBindingUnstated => "worktree_binding_unstated",
            Self::StackMembershipSourceUnstated => "stack_membership_source_unstated",
            Self::StackOrderUnstated => "stack_order_unstated",
            Self::LandingStateUnstated => "landing_state_unstated",
            Self::ValidationFreshnessUnstated => "validation_freshness_unstated",
            Self::ChangeOrchestrationMatrixStale => "change_orchestration_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so a change-orchestration result never reads without its provider
/// ownership, local-versus-provider state, linked engineering identity, or relation source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationVisibleState {
    /// Class / surface label shown on the surface (work-item detail, start-work sheet, linked-change panel, card).
    pub surface_label: String,
    /// Provider ownership of the tracked work item.
    pub selected_change_object: String,
    /// Local-versus-provider commit state disclosed before any committed-update claim.
    pub worktree_base_identity: String,
    /// Linked branch / worktree / review identity bound to the change orchestration.
    pub stack_membership_and_order: String,
    /// Relation source (linked by provider, linked locally, suggested by Aureline, stale or broken).
    pub landing_state_summary: String,
    /// Blocker / resolution state (ready to resolve, blocked by engineering, escalation open, awaiting provider write, resolution authority missing).
    pub cleanup_safety: String,
    /// Validation evidence and publish-later fallback packaged with a handoff.
    pub validation_evidence: String,
}

impl M5ChangeOrchestrationVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.surface_label.trim().is_empty()
            && !self.selected_change_object.trim().is_empty()
            && !self.worktree_base_identity.trim().is_empty()
            && !self.stack_membership_and_order.trim().is_empty()
            && !self.landing_state_summary.trim().is_empty()
            && !self.cleanup_safety.trim().is_empty()
            && !self.validation_evidence.trim().is_empty()
    }
}

/// One row in the matrix: one governed change-orchestration object class bound to the surface-specific
/// change-orchestration truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationRow {
    /// Governed change-orchestration object class.
    pub object_class: M5ChangeOrchestrationObject,
    /// Qualification class earned by this class's change-orchestration handling.
    pub qualification: M5ChangeOrchestrationQualificationClass,
    /// Commit state this row governs (distinguishes a provider-committed update from a local-only draft or a queued publish).
    pub landing_state: M5ChangeOrchestrationState,
    /// Owner role accountable for keeping this class's change-orchestration state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's change-orchestration result visibly owned, provider-attributed, and commit-honest.
    pub required_visible_state: M5ChangeOrchestrationVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5ChangeOrchestrationSurfaceFamily>,
    /// Classification stages this class passes through from intent capture to a recorded resolution.
    pub classification_stages: Vec<M5ChangeOrchestrationClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5ChangeOrchestrationRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ChangeOrchestrationRequiredLabel>,
    /// Change-orchestration roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5ChangeOrchestrationRole>,
    /// ChangeObject roles this class names (ChangeObject only).
    pub change_object_roles: Vec<M5ChangeObjectRole>,
    /// PatchStackQueue roles this class names (PatchStackQueue only).
    pub patch_stack_queue_roles: Vec<M5ChangeOrchestrationPatchStackQueueRole>,
    /// StackEditReviewSheet roles this class names (StackEditReviewSheet only).
    pub stack_edit_review_roles: Vec<M5ChangeOrchestrationStackEditReviewRole>,
    /// LandingCandidateSheet roles this class names (LandingCandidateSheet only).
    pub landing_candidate_roles: Vec<M5ChangeOrchestrationLandingCandidateRole>,
    /// PortableShelf roles this class names (PortableShelf only).
    pub portable_shelf_roles: Vec<M5ChangeOrchestrationPortableShelfRole>,
    /// WorktreeCleanupPreview roles this class names (WorktreeCleanupPreview only).
    pub worktree_cleanup_roles: Vec<M5ChangeOrchestrationWorktreeCleanupRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5ChangeOrchestrationDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5ChangeOrchestrationAccessibilityRoute>,
    /// First consumer surfaces that consume this class's change-orchestration projection.
    pub consumer_surfaces: Vec<M5ChangeOrchestrationConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5ChangeOrchestrationDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's change-orchestration state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets start work silently create a branch, worktree, review draft, or provider link without separately disclosing each side effect. MUST be `false`.
    pub infers_stack_membership_from_branch_names_alone: bool,
    /// Hard invariant: this class never lets a local handoff packet or queued publish masquerade as a provider-committed update. MUST be `false`.
    pub mutates_files_in_another_worktree_without_an_explicit_selected_change_object_and_worktree_binding:
        bool,
    /// Hard invariant: this class never flattens linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken relation into one generic relation badge. MUST be `false`.
    pub silently_reorders_collapses_or_retargets_stack_members: bool,
    /// Hard invariant: this class never auto-resolves tracked work while engineering blockers remain unresolved. MUST be `false`.
    pub lands_from_ambient_branch_state_without_a_reviewed_landing_candidate: bool,
    /// Hard invariant: this class never drops local notes, handoff packets, or linked evidence when provider write fails. MUST be `false`.
    pub deletes_orphaned_worktrees_or_stale_stack_members_without_previewing_running_work_and_export_safe_evidence:
        bool,
}

impl M5ChangeOrchestrationRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ChangeOrchestrationRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ChangeOrchestrationRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.infers_stack_membership_from_branch_names_alone
            && !self.mutates_files_in_another_worktree_without_an_explicit_selected_change_object_and_worktree_binding
            && !self.silently_reorders_collapses_or_retargets_stack_members
            && !self.lands_from_ambient_branch_state_without_a_reviewed_landing_candidate
            && !self.deletes_orphaned_worktrees_or_stale_stack_members_without_previewing_running_work_and_export_safe_evidence
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Commit states tokens.
    pub landing_states: Vec<String>,
    /// Relation sources tokens.
    pub stack_membership_sources: Vec<String>,
    /// Blocker states tokens.
    pub cleanup_safetys: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Change intent record roles tokens.
    pub change_object_roles: Vec<String>,
    /// Start work roles tokens.
    pub patch_stack_queue_roles: Vec<String>,
    /// Linked change roles tokens.
    pub stack_edit_review_roles: Vec<String>,
    /// Handoff roles tokens.
    pub landing_candidate_roles: Vec<String>,
    /// Resolve roles tokens.
    pub portable_shelf_roles: Vec<String>,
    /// Blocked escalate roles tokens.
    pub worktree_cleanup_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5ChangeOrchestrationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5ChangeOrchestrationObject::ALL, |v| v.as_str()),
            landing_states: tokens(&M5ChangeOrchestrationState::ALL, |v| v.as_str()),
            stack_membership_sources: tokens(
                &M5ChangeOrchestrationStackMembershipSource::ALL,
                |v| v.as_str(),
            ),
            cleanup_safetys: tokens(&M5ChangeOrchestrationCleanupSafety::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5ChangeOrchestrationRole::ALL, |v| v.as_str()),
            change_object_roles: tokens(&M5ChangeObjectRole::ALL, |v| v.as_str()),
            patch_stack_queue_roles: tokens(&M5ChangeOrchestrationPatchStackQueueRole::ALL, |v| {
                v.as_str()
            }),
            stack_edit_review_roles: tokens(&M5ChangeOrchestrationStackEditReviewRole::ALL, |v| {
                v.as_str()
            }),
            landing_candidate_roles: tokens(&M5ChangeOrchestrationLandingCandidateRole::ALL, |v| {
                v.as_str()
            }),
            portable_shelf_roles: tokens(&M5ChangeOrchestrationPortableShelfRole::ALL, |v| {
                v.as_str()
            }),
            worktree_cleanup_roles: tokens(&M5ChangeOrchestrationWorktreeCleanupRole::ALL, |v| {
                v.as_str()
            }),
            surface_families: tokens(&M5ChangeOrchestrationSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5ChangeOrchestrationClassificationStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5ChangeOrchestrationConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ChangeOrchestrationAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5ChangeOrchestrationDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ChangeOrchestrationRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5ChangeOrchestrationDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationGovernanceReview {
    /// No local handoff packet is shown as a provider committed update.
    pub no_local_shelf_or_ambient_branch_reads_as_a_reviewed_landing_candidate: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Provider committed state is mechanically distinct from local only draft.
    pub queue_eligible_state_is_mechanically_distinct_from_selected_change: bool,
    /// Every change orchestration names its provider ownership.
    pub every_change_orchestration_names_its_selected_change_object: bool,
    /// Every start work sheet discloses each side effect separately.
    pub every_patch_stack_queue_discloses_each_side_effect_separately: bool,
    /// Every linked change names its relation source.
    pub every_linked_change_names_its_stack_membership_source: bool,
    /// No start work side effect is created without disclosure.
    pub no_cross_worktree_write_without_a_selected_change_object_and_binding: bool,
    /// Every handoff discloses its publish later fallback.
    pub every_landing_candidate_discloses_its_validation_freshness_and_protected_branch_gate: bool,
    /// No tracked work is auto resolved while engineering blockers remain.
    pub no_landing_from_ambient_branch_state: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single change orchestration source.
    pub support_export_reads_single_change_orchestration_source: bool,
    /// Work item start work review provider and support bind to single source.
    pub git_surface_start_work_review_provider_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel change orchestration vocabulary.
    pub later_rows_cannot_invent_parallel_change_orchestration_vocabulary: bool,
    /// Change intent truth survives zoom and high contrast.
    pub change_orchestration_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationConsumerProjection {
    /// Work item detail and start work consume shared change orchestration truth.
    pub change_object_detail_and_start_work_consume_shared_change_orchestration_truth: bool,
    /// Ready for review handoff and provider handoff consume shared commit state truth.
    pub provider_merge_queue_and_provider_handoff_consume_shared_landing_state_truth: bool,
    /// Help and support export consume shared relation and blocker truth.
    pub help_and_support_export_consume_shared_membership_and_cleanup_truth: bool,
    /// Docs help and screenshots read single change orchestration source.
    pub docs_help_and_screenshots_read_single_change_orchestration_source: bool,
    /// Change intents bind to shared linked change relation.
    pub change_objects_bind_to_shared_stack_membership_source: bool,
    /// Support export reads single change orchestration source.
    pub support_export_reads_single_change_orchestration_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the change-orchestration lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting change-orchestration audit for the lane.
    pub change_orchestration_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ChangeOrchestrationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChangeOrchestrationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Change-orchestration rows.
    pub change_orchestration_rows: Vec<M5ChangeOrchestrationRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChangeOrchestrationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChangeOrchestrationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeOrchestrationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeOrchestrationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChangeOrchestrationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 change-orchestration matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationMatrixPacket {
    /// Record kind; must equal [`M5_CHANGE_ORCHESTRATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Change-orchestration rows.
    pub change_orchestration_rows: Vec<M5ChangeOrchestrationRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChangeOrchestrationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChangeOrchestrationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeOrchestrationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeOrchestrationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChangeOrchestrationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChangeOrchestrationMatrixPacket {
    /// Builds an M5 change-orchestration matrix packet from input.
    pub fn new(input: M5ChangeOrchestrationMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CHANGE_ORCHESTRATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            change_orchestration_rows: input.change_orchestration_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 change-orchestration matrix invariants.
    pub fn validate(&self) -> Vec<M5ChangeOrchestrationMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_CHANGE_ORCHESTRATION_MATRIX_RECORD_KIND {
            violations.push(M5ChangeOrchestrationMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_VERSION {
            violations.push(M5ChangeOrchestrationMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChangeOrchestrationMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_change_orchestration_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 change-orchestration matrix serializes"),
        ) {
            violations.push(M5ChangeOrchestrationMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 change-orchestration matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed change-orchestration class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,landing_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.change_orchestration_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.landing_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic change-orchestration-health dashboard JSON that work-item and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .change_orchestration_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "landing_state": row.landing_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_change_orchestration_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_CHANGE_ORCHESTRATION_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 change-orchestration-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or work-item handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .change_orchestration_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Change-Object, Patch-Stack/Queue, Stack-Edit-Review, Landing-Candidate, Portable-Shelf, and Worktree-Cleanup-Preview Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.change_orchestration_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Change-orchestration roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.change_orchestration_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (landing_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.landing_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worktree / base identity: {}\n",
                row.required_visible_state.worktree_base_identity
            ));
            out.push_str(&format!(
                "  - Cleanup safety: {}\n",
                row.required_visible_state.cleanup_safety
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 change-orchestration matrix export.
#[derive(Debug)]
pub enum M5ChangeOrchestrationMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChangeOrchestrationMatrixViolation>),
}

impl fmt::Display for M5ChangeOrchestrationMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 change-orchestration matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 change-orchestration matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChangeOrchestrationMatrixArtifactError {}

/// Validation failures emitted by [`M5ChangeOrchestrationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChangeOrchestrationMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A change-orchestration row is incomplete.
    ChangeOrchestrationRowIncomplete,
    /// A change-orchestration row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A change-orchestration row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no change-orchestration roles.
    SemanticRoleMissing,
    /// The ChangeObject class declares no ChangeObject roles.
    ChangeObjectRoleMissing,
    /// The PatchStackQueue class declares no PatchStackQueue roles.
    PatchStackQueueRoleMissing,
    /// The StackEditReviewSheet class declares no StackEditReviewSheet roles.
    StackEditReviewRoleMissing,
    /// The LandingCandidateSheet class declares no LandingCandidateSheet roles.
    LandingCandidateRoleMissing,
    /// The PortableShelf class declares no PortableShelf roles.
    PortableShelfRoleMissing,
    /// The WorktreeCleanupPreview class declares no WorktreeCleanupPreview roles.
    WorktreeCleanupRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard change-orchestration invariant.
    ChangeOrchestrationInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ChangeOrchestrationMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ChangeOrchestrationRowIncomplete => "change_orchestration_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ChangeObjectRoleMissing => "change_object_role_missing",
            Self::PatchStackQueueRoleMissing => "patch_stack_queue_role_missing",
            Self::StackEditReviewRoleMissing => "stack_edit_review_role_missing",
            Self::LandingCandidateRoleMissing => "landing_candidate_role_missing",
            Self::PortableShelfRoleMissing => "portable_shelf_role_missing",
            Self::WorktreeCleanupRoleMissing => "worktree_cleanup_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::ChangeOrchestrationInvariantViolated => "change_orchestration_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 change-orchestration matrix export.
pub fn current_stable_m5_change_orchestration_matrix_export(
) -> Result<M5ChangeOrchestrationMatrixPacket, M5ChangeOrchestrationMatrixArtifactError> {
    let packet: M5ChangeOrchestrationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-orchestration-proof/support_export.json"
    )))
    .map_err(M5ChangeOrchestrationMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChangeOrchestrationMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF,
        M5_CHANGE_OBJECT_DOMAIN_SCHEMA_REF,
        M5_PATCH_STACK_QUEUE_DOMAIN_SCHEMA_REF,
        M5_STACK_EDIT_REVIEW_SHEET_DOMAIN_SCHEMA_REF,
        M5_LANDING_CANDIDATE_SHEET_DOMAIN_SCHEMA_REF,
        M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
        M5_WORKTREE_MANAGER_ROW_DOMAIN_SCHEMA_REF,
        M5_PORTABLE_BUNDLE_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ChangeOrchestrationMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ChangeOrchestrationMatrixViolation::VocabularySetDrift);
    }
}

fn validate_change_orchestration_rows(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    let present: BTreeSet<M5ChangeOrchestrationObject> = packet
        .change_orchestration_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5ChangeOrchestrationObject::ALL {
        if !present.contains(&required) {
            violations.push(M5ChangeOrchestrationMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.change_orchestration_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ChangeOrchestrationMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5ChangeOrchestrationMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_change_object_roles() && row.change_object_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::ChangeObjectRoleMissing);
        }
        if class.declares_patch_stack_queue_roles() && row.patch_stack_queue_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::PatchStackQueueRoleMissing);
        }
        if class.declares_stack_edit_review_roles() && row.stack_edit_review_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::StackEditReviewRoleMissing);
        }
        if class.declares_landing_candidate_roles() && row.landing_candidate_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::LandingCandidateRoleMissing);
        }
        if class.declares_portable_shelf_roles() && row.portable_shelf_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::PortableShelfRoleMissing);
        }
        if class.declares_worktree_cleanup_roles() && row.worktree_cleanup_roles.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::WorktreeCleanupRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5ChangeOrchestrationMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ChangeOrchestrationMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations
                .push(M5ChangeOrchestrationMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations
                .push(M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_local_shelf_or_ambient_branch_reads_as_a_reviewed_landing_candidate,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.queue_eligible_state_is_mechanically_distinct_from_selected_change,
        review.every_change_orchestration_names_its_selected_change_object,
        review.every_patch_stack_queue_discloses_each_side_effect_separately,
        review.every_linked_change_names_its_stack_membership_source,
        review.no_cross_worktree_write_without_a_selected_change_object_and_binding,
        review.every_landing_candidate_discloses_its_validation_freshness_and_protected_branch_gate,
        review.no_landing_from_ambient_branch_state,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_change_orchestration_source,
        review.git_surface_start_work_review_provider_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_change_orchestration_vocabulary,
        review.change_orchestration_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5ChangeOrchestrationMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.change_object_detail_and_start_work_consume_shared_change_orchestration_truth,
        projection.provider_merge_queue_and_provider_handoff_consume_shared_landing_state_truth,
        projection.help_and_support_export_consume_shared_membership_and_cleanup_truth,
        projection.docs_help_and_screenshots_read_single_change_orchestration_source,
        projection.change_objects_bind_to_shared_stack_membership_source,
        projection.support_export_reads_single_change_orchestration_source,
    ] {
        if !ok {
            violations.push(M5ChangeOrchestrationMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ChangeOrchestrationMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ChangeOrchestrationMatrixPacket,
    violations: &mut Vec<M5ChangeOrchestrationMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.change_orchestration_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ChangeOrchestrationMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses change / intent / provider / handoff / blocker words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
