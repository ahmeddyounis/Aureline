//! Frozen M5 constrained-file-state, canonical-source-relation, and write-target-review matrix.
//!
//! This module locks Aureline's constrained-current-object model — the read-only, generated, policy-locked,
//! managed, projection, and captured-snapshot objects that a write-capable consumer must never treat as an
//! ordinary directly-writable file — into one export-safe packet. Every covered object class is named once
//! here and constrained by the same shared constrained-file-state role taxonomy (state_badge_classification,
//! blocked_write_reason, canonical_source_relation, exact_write_target, allowed_blocked_action_set,
//! safe_next_step_guidance, export_retain_disclosure), the same required visible state (state badge, reason,
//! canonical source or live target, exact write target, allowed actions, blocked actions, and export / retain
//! notes), the same no-one-constrained-state-class-hides-another rule, the same
//! no-generated-managed-projection-or-archived-object-silently-falls-back-to-a-lossy-direct-write rule, the
//! same no-AI-automation-import-or-repair-flow-gets-a-hidden-bypass-around-constrained-state-rules rule, the
//! same canonical-source-exact-write-target-preserved-versus-lost-sync-and-recovery-path-stay-explicit rule,
//! and the same no-constrained-object-presented-as-directly-writable-or-recovery-path-hidden rule regardless
//! of the surface that renders it.
//!
//! The matrix makes a write-constrained object mechanically distinct from an ordinary directly-writable object
//! (see [`M5ConstrainedFileStateWriteDisposition`]) so tabs, breadcrumbs, the status bar, the command palette,
//! editor banners, diff / review headers, write-review sheets, AI / automation mutation paths, and support /
//! export packets can key off the constrained state rather than guessing from a stale badge. It does not
//! redesign every artifact producer — it reuses the already-landed generated-artifact, historical-snapshot,
//! restore, and trust descriptors — it is the shared reusable constrained-object contract those consumers
//! read, and it binds back to the already-landed stable-proof-index and migration-task-row packets so
//! constrained-file-state truth is not split across scattered internal notes. The controlled vocabularies are
//! frozen in one self-describing [`M5ConstrainedFileStateVocabularySet`] rather than minted per surface. Raw
//! secret values and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_constrained_file_state_matrix,
    seeded_m5_constrained_file_state_matrix_managed_beta_narrowed,
    seeded_m5_constrained_file_state_matrix_projection_preview_narrowed,
    M5_CONSTRAINED_FILE_STATE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ConstrainedFileStateMatrixPacket`].
pub const M5_CONSTRAINED_FILE_STATE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_constrained_file_state_canonical_source_relation_and_write_target_review_matrix";

/// Schema version for M5 constrained-file-state matrix records.
pub const M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined constrained-file-state matrix schema.
pub const M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-constrained-file-state-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF: &str =
    "docs/support/m5-constrained-object-state-ops.md";

/// Repo-relative path of the canonical constrained-file-state domain schema (read-only and policy-locked
/// objects: the state badge, blocked-write reason, allowed / blocked action set, and mutation-blocked posture
/// of a current object that cannot be written in place).
pub const M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-constrained-file-state.schema.json";

/// Repo-relative path of the canonical canonical-source-relation domain schema (generated and projection
/// objects: the canonical source or backing object, the diverged-from-source state, and the regenerate /
/// detach relation that keeps derived truth joined to its source).
pub const M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-canonical-source-relation.schema.json";

/// Repo-relative path of the canonical write-target-review domain schema (managed and captured-snapshot
/// objects: the exact write target, the request-approval / restore review before any mutation, and the
/// preserved-versus-lost sync note so a constrained write is never a silent lossy fallback).
pub const M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-write-target-review.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONSTRAINED_FILE_STATE_FIXTURE_DIR: &str =
    "fixtures/editor/m5-constrained-object-states";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONSTRAINED_FILE_STATE_ARTIFACT_REF: &str =
    "artifacts/support/m5-constrained-object-state/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CONSTRAINED_FILE_STATE_CSV_REF: &str =
    "artifacts/support/m5-constrained-object-state/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CONSTRAINED_FILE_STATE_REPORT_REF: &str =
    "artifacts/program/m5-constrained-file-state-matrix.md";

/// Repo-relative path of the checked constrained-object-health dashboard.
pub const M5_CONSTRAINED_FILE_STATE_DASHBOARD_REF: &str =
    "dashboards/m5-constrained-object-health.json";

/// One of the six governed constrained-current-object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateObject {
    /// A read-only current object that cannot be written in place through the ordinary editor path.
    ReadOnly,
    /// A generated / derived artifact object whose canonical truth lives in its generator, not the artifact itself.
    Generated,
    /// A policy-locked object whose writes are gated behind an explicit approval / policy owner.
    PolicyLocked,
    /// A managed, externally-owned object whose canonical source is the managing owner, not the local workspace.
    Managed,
    /// A projection / virtual view object whose exact write target resolves back to a backing source object.
    Projection,
    /// A captured-snapshot object that preserves a past state and is not the current live object.
    CapturedSnapshot,
}

impl M5ConstrainedFileStateObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnly,
        Self::Generated,
        Self::PolicyLocked,
        Self::Managed,
        Self::Projection,
        Self::CapturedSnapshot,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Generated => "generated",
            Self::PolicyLocked => "policy_locked",
            Self::Managed => "managed",
            Self::Projection => "projection",
            Self::CapturedSnapshot => "captured_snapshot",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's constrained-file-state, canonical-source-relation, or write-target-review meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ReadOnly | Self::PolicyLocked => M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF,
            Self::Generated | Self::Projection => M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
            Self::Managed | Self::CapturedSnapshot => M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled read only role.
    pub const fn declares_read_only_roles(self) -> bool {
        matches!(self, Self::ReadOnly)
    }

    /// `true` when this class must name a controlled generated role.
    pub const fn declares_generated_roles(self) -> bool {
        matches!(self, Self::Generated)
    }

    /// `true` when this class must name a controlled policy locked role.
    pub const fn declares_policy_locked_roles(self) -> bool {
        matches!(self, Self::PolicyLocked)
    }

    /// `true` when this class must name a controlled managed role.
    pub const fn declares_managed_roles(self) -> bool {
        matches!(self, Self::Managed)
    }

    /// `true` when this class must name a controlled projection role.
    pub const fn declares_projection_roles(self) -> bool {
        matches!(self, Self::Projection)
    }

    /// `true` when this class must name a controlled captured snapshot role.
    pub const fn declares_captured_snapshot_roles(self) -> bool {
        matches!(self, Self::CapturedSnapshot)
    }
}

/// The single controlled constrained-file-state role vocabulary every shell, editor, review, AI / automation, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateRole {
    /// The visible state badge that classifies the object as read-only, generated, policy-locked, managed, projection, or captured snapshot.
    StateBadgeClassification,
    /// The reason strip that explains why a direct write is blocked or constrained.
    BlockedWriteReason,
    /// The canonical source or live target the constrained object relates back to.
    CanonicalSourceRelation,
    /// The exact write target a write-capable action would actually touch.
    ExactWriteTarget,
    /// The explicit allowed-versus-blocked action set for the constrained object.
    AllowedBlockedActionSet,
    /// The nearest safe next step (duplicate, detach, overlay, regenerate, or request approval) instead of a silent best-effort fallback.
    SafeNextStepGuidance,
    /// The export / retain note describing what is preserved versus lost on export.
    ExportRetainDisclosure,
}

impl M5ConstrainedFileStateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::StateBadgeClassification,
        Self::BlockedWriteReason,
        Self::CanonicalSourceRelation,
        Self::ExactWriteTarget,
        Self::AllowedBlockedActionSet,
        Self::SafeNextStepGuidance,
        Self::ExportRetainDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateBadgeClassification => "state_badge_classification",
            Self::BlockedWriteReason => "blocked_write_reason",
            Self::CanonicalSourceRelation => "canonical_source_relation",
            Self::ExactWriteTarget => "exact_write_target",
            Self::AllowedBlockedActionSet => "allowed_blocked_action_set",
            Self::SafeNextStepGuidance => "safe_next_step_guidance",
            Self::ExportRetainDisclosure => "export_retain_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a constrained object (`state_badge_classification`, `blocked_write_reason`,
    /// `canonical_source_relation`, `exact_write_target`). The contextual roles (`allowed_blocked_action_set`,
    /// `safe_next_step_guidance`, `export_retain_disclosure`) apply where the object class calls for them.
    pub const fn must_be_present_before_surfacing_as_constrained_object(self) -> bool {
        matches!(
            self,
            Self::StateBadgeClassification
                | Self::BlockedWriteReason
                | Self::CanonicalSourceRelation
                | Self::ExactWriteTarget
        )
    }
}

/// Write disposition that makes a write-constrained object (read-only, generated, policy-locked, managed, projection, or captured snapshot) mechanically distinct from an ordinary directly-writable object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateWriteDisposition {
    /// An ordinary directly-writable object, not write-constrained.
    DirectlyWritable,
    /// Write-constrained: read-only, in-place write blocked; duplicate to an editable copy.
    ReadOnlyBlocked,
    /// Write-constrained: generated / derived; edits flow through regenerating from the canonical source.
    RegenerateOnly,
    /// Write-constrained: policy-locked or managed; writes are gated behind an approval or managing owner.
    ApprovalGated,
    /// Write-constrained: a projection / virtual view; writing requires detaching or overlaying onto the backing source.
    DetachRequired,
    /// Write-constrained: a captured snapshot; it is restored or handed off to the live object, never mutated in place.
    RestoreOnly,
}

impl M5ConstrainedFileStateWriteDisposition {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DirectlyWritable,
        Self::ReadOnlyBlocked,
        Self::RegenerateOnly,
        Self::ApprovalGated,
        Self::DetachRequired,
        Self::RestoreOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectlyWritable => "directly_writable",
            Self::ReadOnlyBlocked => "read_only_blocked",
            Self::RegenerateOnly => "regenerate_only",
            Self::ApprovalGated => "approval_gated",
            Self::DetachRequired => "detach_required",
            Self::RestoreOnly => "restore_only",
        }
    }
    /// `true` for every write-constrained disposition, so downstream tabs, banners, palettes, review
    /// headers, and AI / automation paths can key off the constrained state rather than confusing it with
    /// an ordinary directly-writable object.
    pub const fn is_write_constrained(self) -> bool {
        !matches!(self, Self::DirectlyWritable)
    }
}

/// Controlled blocked-write role for a read-only current object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateReadOnlyRole {
    /// Read-only state badge shown so the object never looks directly writable.
    ReadOnlyBadgeShown,
    /// Blocked-write reason strip shown for the read-only object.
    BlockedWriteReasonShown,
    /// Canonical source / owning object named for the read-only object.
    CanonicalSourceNamed,
    /// Duplicate-to-editable-copy safe next step offered instead of a silent write.
    DuplicateToEditableOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// In-place writing of a read-only object, which is disallowed.
    InPlaceWriteDisallowed,
}

impl M5ConstrainedFileStateReadOnlyRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnlyBadgeShown,
        Self::BlockedWriteReasonShown,
        Self::CanonicalSourceNamed,
        Self::DuplicateToEditableOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::InPlaceWriteDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyBadgeShown => "read_only_badge_shown",
            Self::BlockedWriteReasonShown => "blocked_write_reason_shown",
            Self::CanonicalSourceNamed => "canonical_source_named",
            Self::DuplicateToEditableOffered => "duplicate_to_editable_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::InPlaceWriteDisallowed => "in_place_write_disallowed",
        }
    }
}

/// Controlled blocked-write role for a generated / derived artifact object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateGeneratedRole {
    /// Generated / derived state badge shown so the artifact never looks like a hand-authored source.
    GeneratedBadgeShown,
    /// Generator / canonical source named for the artifact.
    GeneratorSourceNamed,
    /// Any diverged-from-generator state flagged rather than silently overwritten.
    DivergedFromGeneratorFlagged,
    /// Regenerate-from-source safe next step offered instead of a lossy direct edit.
    RegenerateFromSourceOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// Direct editing of a generated artifact without regenerating, which is disallowed.
    DirectEditWithoutRegenerateDisallowed,
}

impl M5ConstrainedFileStateGeneratedRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GeneratedBadgeShown,
        Self::GeneratorSourceNamed,
        Self::DivergedFromGeneratorFlagged,
        Self::RegenerateFromSourceOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::DirectEditWithoutRegenerateDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedBadgeShown => "generated_badge_shown",
            Self::GeneratorSourceNamed => "generator_source_named",
            Self::DivergedFromGeneratorFlagged => "diverged_from_generator_flagged",
            Self::RegenerateFromSourceOffered => "regenerate_from_source_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::DirectEditWithoutRegenerateDisallowed => {
                "direct_edit_without_regenerate_disallowed"
            }
        }
    }
}

/// Controlled blocked-write role for a policy-locked object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStatePolicyLockedRole {
    /// Policy-lock state badge shown so the object never looks freely writable.
    PolicyLockBadgeShown,
    /// Lock reason and governing policy named for the object.
    LockReasonAndPolicyNamed,
    /// Canonical policy owner named as the authority for the lock.
    CanonicalPolicyOwnerNamed,
    /// Request-approval safe next step offered instead of a silent override.
    RequestApprovalOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// Silently overriding a policy lock, which is disallowed.
    SilentPolicyOverrideDisallowed,
}

impl M5ConstrainedFileStatePolicyLockedRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyLockBadgeShown,
        Self::LockReasonAndPolicyNamed,
        Self::CanonicalPolicyOwnerNamed,
        Self::RequestApprovalOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::SilentPolicyOverrideDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyLockBadgeShown => "policy_lock_badge_shown",
            Self::LockReasonAndPolicyNamed => "lock_reason_and_policy_named",
            Self::CanonicalPolicyOwnerNamed => "canonical_policy_owner_named",
            Self::RequestApprovalOffered => "request_approval_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::SilentPolicyOverrideDisallowed => "silent_policy_override_disallowed",
        }
    }
}

/// Controlled blocked-write role for a managed, externally-owned object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateManagedRole {
    /// Managed state badge shown so the object never looks locally owned.
    ManagedBadgeShown,
    /// Managing owner named as the canonical source for the object.
    ManagingOwnerNamed,
    /// Exact write target named for any managed-change request.
    ExactWriteTargetNamed,
    /// Request-managed-change safe next step offered instead of a divergent local write.
    RequestManagedChangeOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// A local divergent write to a managed object, which is disallowed.
    LocalDivergentWriteDisallowed,
}

impl M5ConstrainedFileStateManagedRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ManagedBadgeShown,
        Self::ManagingOwnerNamed,
        Self::ExactWriteTargetNamed,
        Self::RequestManagedChangeOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::LocalDivergentWriteDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedBadgeShown => "managed_badge_shown",
            Self::ManagingOwnerNamed => "managing_owner_named",
            Self::ExactWriteTargetNamed => "exact_write_target_named",
            Self::RequestManagedChangeOffered => "request_managed_change_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::LocalDivergentWriteDisallowed => "local_divergent_write_disallowed",
        }
    }
}

/// Controlled blocked-write role for a projection / virtual view object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateProjectionRole {
    /// Projection / virtual-view state badge shown so the view never looks like a concrete file.
    ProjectionBadgeShown,
    /// Backing source object named as the canonical source for the projection.
    BackingSourceNamed,
    /// Exact write target shown resolving back to the backing source object.
    WriteTargetResolvesToSource,
    /// Detach-or-overlay safe next step offered instead of writing into the virtual view.
    DetachOrOverlayOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// Writing directly into a virtual projection view, which is disallowed.
    WriteToVirtualViewDisallowed,
}

impl M5ConstrainedFileStateProjectionRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectionBadgeShown,
        Self::BackingSourceNamed,
        Self::WriteTargetResolvesToSource,
        Self::DetachOrOverlayOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::WriteToVirtualViewDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectionBadgeShown => "projection_badge_shown",
            Self::BackingSourceNamed => "backing_source_named",
            Self::WriteTargetResolvesToSource => "write_target_resolves_to_source",
            Self::DetachOrOverlayOffered => "detach_or_overlay_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::WriteToVirtualViewDisallowed => "write_to_virtual_view_disallowed",
        }
    }
}

/// Controlled blocked-write role for a captured-snapshot object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateCapturedSnapshotRole {
    /// Captured-snapshot state badge shown so the snapshot never looks like the current live object.
    SnapshotBadgeShown,
    /// Capture time and source object named for the snapshot.
    CaptureTimeAndSourceNamed,
    /// Live target or metadata-only exit named for the snapshot.
    LiveTargetOrExitNamed,
    /// Restore-or-open-live safe next step offered instead of an in-place mutation.
    RestoreOrOpenLiveOffered,
    /// A role bound to the single constrained-file-state registry.
    BoundToConstrainedFileStateRegistry,
    /// Mutating a captured snapshot in place as if it were the live object, which is disallowed.
    MutateSnapshotInPlaceDisallowed,
}

impl M5ConstrainedFileStateCapturedSnapshotRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SnapshotBadgeShown,
        Self::CaptureTimeAndSourceNamed,
        Self::LiveTargetOrExitNamed,
        Self::RestoreOrOpenLiveOffered,
        Self::BoundToConstrainedFileStateRegistry,
        Self::MutateSnapshotInPlaceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotBadgeShown => "snapshot_badge_shown",
            Self::CaptureTimeAndSourceNamed => "capture_time_and_source_named",
            Self::LiveTargetOrExitNamed => "live_target_or_exit_named",
            Self::RestoreOrOpenLiveOffered => "restore_or_open_live_offered",
            Self::BoundToConstrainedFileStateRegistry => "bound_to_constrained_file_state_registry",
            Self::MutateSnapshotInPlaceDisallowed => "mutate_snapshot_in_place_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a constrained-current-object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateSurfaceFamily {
    /// The shell surface (tab chrome, breadcrumbs, status bar).
    Shell,
    /// The editor surface (banners and reason strips).
    Editor,
    /// The review surface (diff / review headers and write-review sheets).
    Review,
    /// The AI / automation mutation surface.
    AiAutomation,
    /// The help / docs surface.
    HelpDocs,
    /// The support / export surface.
    SupportExport,
}

impl M5ConstrainedFileStateSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::AiAutomation,
        Self::HelpDocs,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Editor => "editor",
            Self::Review => "review",
            Self::AiAutomation => "ai_automation",
            Self::HelpDocs => "help_docs",
            Self::SupportExport => "support_export",
        }
    }
}

/// Classification stage a class passes through from constraint detection to a state-classified, canonical-source-resolved, write-target-resolved, and safe-action-offered constrained object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateClassificationStage {
    /// The constraint-detected stage: the object is recognised as write-constrained.
    ConstraintDetected,
    /// The state-classified stage: the constrained state badge is assigned.
    StateClassified,
    /// The canonical-source-resolved stage: the canonical source or live target is resolved.
    CanonicalSourceResolved,
    /// The write-target-resolved stage: the exact write target is resolved.
    WriteTargetResolved,
    /// The safe-action-offered stage: the nearest safe next step is offered.
    SafeActionOffered,
}

impl M5ConstrainedFileStateClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConstraintDetected,
        Self::StateClassified,
        Self::CanonicalSourceResolved,
        Self::WriteTargetResolved,
        Self::SafeActionOffered,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConstraintDetected => "constraint_detected",
            Self::StateClassified => "state_classified",
            Self::CanonicalSourceResolved => "canonical_source_resolved",
            Self::WriteTargetResolved => "write_target_resolved",
            Self::SafeActionOffered => "safe_action_offered",
        }
    }
}

/// Shared consumer surface that must agree on a class's constrained-file-state truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateConsumerSurface {
    /// The tab chrome.
    TabChrome,
    /// The breadcrumb trail.
    BreadcrumbTrail,
    /// The status bar.
    StatusBar,
    /// The command palette availability.
    CommandPalette,
    /// The editor banner / reason strip.
    EditorBanner,
    /// The diff / review header.
    DiffReviewHeader,
    /// The write-review sheet.
    WriteReviewSheet,
    /// The AI / automation mutation path.
    AiAutomationPath,
    /// The support / export packet.
    SupportExportPacket,
}

impl M5ConstrainedFileStateConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TabChrome,
        Self::BreadcrumbTrail,
        Self::StatusBar,
        Self::CommandPalette,
        Self::EditorBanner,
        Self::DiffReviewHeader,
        Self::WriteReviewSheet,
        Self::AiAutomationPath,
        Self::SupportExportPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabChrome => "tab_chrome",
            Self::BreadcrumbTrail => "breadcrumb_trail",
            Self::StatusBar => "status_bar",
            Self::CommandPalette => "command_palette",
            Self::EditorBanner => "editor_banner",
            Self::DiffReviewHeader => "diff_review_header",
            Self::WriteReviewSheet => "write_review_sheet",
            Self::AiAutomationPath => "ai_automation_path",
            Self::SupportExportPacket => "support_export_packet",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no constrained-file-state meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateAccessibilityRoute {
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

impl M5ConstrainedFileStateAccessibilityRoute {
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

/// Reason a class has degraded below its qualified constrained-state-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateDegradedReason {
    /// The constrained-state badge has gone stale.
    StateBadgeStale,
    /// The canonical source or live target is unresolved.
    CanonicalSourceUnresolved,
    /// The exact write target is unresolved.
    WriteTargetUnresolved,
    /// The allowed / blocked action set is unknown.
    AllowedActionsUnknown,
    /// The export / retain (preserved-versus-lost) state is unknown.
    ExportRetainStateUnknown,
    /// The constraint owner is unknown.
    ConstraintOwnerUnknown,
}

impl M5ConstrainedFileStateDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StateBadgeStale,
        Self::CanonicalSourceUnresolved,
        Self::WriteTargetUnresolved,
        Self::AllowedActionsUnknown,
        Self::ExportRetainStateUnknown,
        Self::ConstraintOwnerUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateBadgeStale => "state_badge_stale",
            Self::CanonicalSourceUnresolved => "canonical_source_unresolved",
            Self::WriteTargetUnresolved => "write_target_unresolved",
            Self::AllowedActionsUnknown => "allowed_actions_unknown",
            Self::ExportRetainStateUnknown => "export_retain_state_unknown",
            Self::ConstraintOwnerUnknown => "constraint_owner_unknown",
        }
    }
}

/// Mandatory label a claimed constrained-object class must be able to show. The first three are hard requirements; the remaining three make the state badge, the exact write target, and the nearest safe action mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's constrained-state role.
    ConstraintRole,
    /// The canonical source-relation descriptor the class points at.
    CanonicalReference,
    /// The constrained-state badge the class must show.
    StateBadge,
    /// The exact write target the class must state.
    ExactWriteTarget,
    /// The nearest safe action the class must offer.
    NearestSafeAction,
}

impl M5ConstrainedFileStateRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::ConstraintRole,
        Self::CanonicalReference,
        Self::StateBadge,
        Self::ExactWriteTarget,
        Self::NearestSafeAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ConstraintRole => "constraint_role",
            Self::CanonicalReference => "canonical_reference",
            Self::StateBadge => "state_badge",
            Self::ExactWriteTarget => "exact_write_target",
            Self::NearestSafeAction => "nearest_safe_action",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::ConstraintRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 constrained-file-state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateQualificationClass {
    /// Class constrained-state handling qualifies for the Stable claim.
    Stable,
    /// Class constrained-state handling is narrowed to Beta.
    Beta,
    /// Class constrained-state handling is narrowed to Preview.
    Preview,
    /// Class constrained-state handling is experimental and not claimed.
    Experimental,
    /// Class constrained-state handling is unavailable on this build.
    Unavailable,
    /// Class constrained-state handling is held pending review.
    Held,
}

impl M5ConstrainedFileStateQualificationClass {
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
    /// Whether the class may carry a public Stable constrained-state-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a constrained-object class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedFileStateDowngradeTrigger {
    /// A constrained object was shown as directly writable.
    ConstrainedObjectShownAsWritable,
    /// One constrained-state class hid another when both materially affect behavior.
    OneStateClassHidesAnother,
    /// A generated / managed / projection / archived object fell back to a silent lossy direct write.
    SilentLossyDirectWriteFallback,
    /// An AI / automation / import / repair flow bypassed the constrained-state rules.
    AiAutomationBypassedConstraint,
    /// A class left its constrained-state badge missing.
    StateBadgeMissing,
    /// A class left its blocked-write reason missing.
    BlockedWriteReasonMissing,
    /// A class left its canonical source or live target unstated.
    CanonicalSourceUnstated,
    /// A class left its exact write target unstated.
    ExactWriteTargetUnstated,
    /// A class left its nearest safe action missing.
    NearestSafeActionMissing,
    /// A class left its preserved-versus-lost sync note unstated.
    PreservedVersusLostSyncUnstated,
    /// A class left its recovery / regenerate path missing.
    RecoveryOrRegeneratePathMissing,
    /// The constrained-file-state descriptor packet has gone stale.
    ConstrainedFileStateDescriptorStale,
}

impl M5ConstrainedFileStateDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConstrainedObjectShownAsWritable,
        Self::OneStateClassHidesAnother,
        Self::SilentLossyDirectWriteFallback,
        Self::AiAutomationBypassedConstraint,
        Self::StateBadgeMissing,
        Self::BlockedWriteReasonMissing,
        Self::CanonicalSourceUnstated,
        Self::ExactWriteTargetUnstated,
        Self::NearestSafeActionMissing,
        Self::PreservedVersusLostSyncUnstated,
        Self::RecoveryOrRegeneratePathMissing,
        Self::ConstrainedFileStateDescriptorStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConstrainedObjectShownAsWritable => "constrained_object_shown_as_writable",
            Self::OneStateClassHidesAnother => "one_state_class_hides_another",
            Self::SilentLossyDirectWriteFallback => "silent_lossy_direct_write_fallback",
            Self::AiAutomationBypassedConstraint => "ai_automation_bypassed_constraint",
            Self::StateBadgeMissing => "state_badge_missing",
            Self::BlockedWriteReasonMissing => "blocked_write_reason_missing",
            Self::CanonicalSourceUnstated => "canonical_source_unstated",
            Self::ExactWriteTargetUnstated => "exact_write_target_unstated",
            Self::NearestSafeActionMissing => "nearest_safe_action_missing",
            Self::PreservedVersusLostSyncUnstated => "preserved_versus_lost_sync_unstated",
            Self::RecoveryOrRegeneratePathMissing => "recovery_or_regenerate_path_missing",
            Self::ConstrainedFileStateDescriptorStale => "constrained_file_state_descriptor_stale",
        }
    }
}

/// Required visible state a class must carry so a constrained object never reads as directly writable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateVisibleState {
    /// Constrained-state badge shown on the surface (read-only, generated, policy-locked, managed, projection, captured snapshot).
    pub state_badge: String,
    /// Reason strip explaining why a direct write is blocked or constrained.
    pub reason: String,
    /// Canonical source or live target the constrained object relates back to.
    pub canonical_source_or_live_target: String,
    /// Exact write target a write-capable action would actually touch.
    pub exact_write_target: String,
    /// Allowed actions the constrained object offers.
    pub allowed_actions: String,
    /// Blocked actions the constrained object refuses.
    pub blocked_actions: String,
    /// Export / retain note describing what is preserved versus lost on export.
    pub export_retain_notes: String,
}

impl M5ConstrainedFileStateVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.state_badge.trim().is_empty()
            && !self.reason.trim().is_empty()
            && !self.canonical_source_or_live_target.trim().is_empty()
            && !self.exact_write_target.trim().is_empty()
            && !self.allowed_actions.trim().is_empty()
            && !self.blocked_actions.trim().is_empty()
            && !self.export_retain_notes.trim().is_empty()
    }
}

/// One row in the matrix: one governed constrained-file-state object class bound to the surface-specific
/// constrained-object truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateRow {
    /// Governed constrained-file-state object class.
    pub object_class: M5ConstrainedFileStateObject,
    /// Qualification class earned by this class's constrained-state handling.
    pub qualification: M5ConstrainedFileStateQualificationClass,
    /// Evidence state this row governs (distinguishes archived / imported evidence from live, cached, and restore-capable state).
    pub write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// Owner role accountable for keeping this class's constrained-object state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's object visibly constrained and attributable.
    pub required_visible_state: M5ConstrainedFileStateVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5ConstrainedFileStateSurfaceFamily>,
    /// Classification stages this class passes through from constraint detection to a safe next step.
    pub classification_stages: Vec<M5ConstrainedFileStateClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5ConstrainedFileStateRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ConstrainedFileStateRequiredLabel>,
    /// Constrained-file-state roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5ConstrainedFileStateRole>,
    /// ReadOnly constrained-object roles this class names (ReadOnly only).
    pub read_only_roles: Vec<M5ConstrainedFileStateReadOnlyRole>,
    /// Generated constrained-object roles this class names (Generated only).
    pub generated_roles: Vec<M5ConstrainedFileStateGeneratedRole>,
    /// PolicyLocked constrained-object roles this class names (PolicyLocked only).
    pub policy_locked_roles: Vec<M5ConstrainedFileStatePolicyLockedRole>,
    /// Managed constrained-object roles this class names (Managed only).
    pub managed_roles: Vec<M5ConstrainedFileStateManagedRole>,
    /// Projection constrained-object roles this class names (Projection only).
    pub projection_roles: Vec<M5ConstrainedFileStateProjectionRole>,
    /// CapturedSnapshot constrained-object roles this class names (CapturedSnapshot only).
    pub captured_snapshot_roles: Vec<M5ConstrainedFileStateCapturedSnapshotRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5ConstrainedFileStateDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// First consumer surfaces that consume this class's constrained-file-state projection.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5ConstrainedFileStateDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's constrained-object state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets one constrained-state class hide another when both materially affect behavior. MUST be `false`.
    pub lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior: bool,
    /// Hard invariant: this class never lets generated, managed, projection, or archived objects silently fall back to a lossy direct write. MUST be `false`.
    pub lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write:
        bool,
    /// Hard invariant: this class never gives AI, automation, import, or repair flows a hidden bypass around the constrained-state rules. MUST be `false`.
    pub gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules:
        bool,
    /// Hard invariant: this class never leaves the canonical source, exact write target, preserved-versus-lost sync, or recovery / regenerate path unstated. MUST be `false`.
    pub leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated:
        bool,
    /// Hard invariant: this class never presents a constrained object as directly writable or hides the recovery / regenerate path. MUST be `false`.
    pub presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path:
        bool,
}

impl M5ConstrainedFileStateRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ConstrainedFileStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ConstrainedFileStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior
            && !self.lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write
            && !self.gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules
            && !self.leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated
            && !self.presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Write dispositions tokens.
    pub write_dispositions: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Read only roles tokens.
    pub read_only_roles: Vec<String>,
    /// Generated roles tokens.
    pub generated_roles: Vec<String>,
    /// Policy locked roles tokens.
    pub policy_locked_roles: Vec<String>,
    /// Managed roles tokens.
    pub managed_roles: Vec<String>,
    /// Projection roles tokens.
    pub projection_roles: Vec<String>,
    /// Captured snapshot roles tokens.
    pub captured_snapshot_roles: Vec<String>,
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

impl M5ConstrainedFileStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5ConstrainedFileStateObject::ALL, |v| v.as_str()),
            write_dispositions: tokens(&M5ConstrainedFileStateWriteDisposition::ALL, |v| {
                v.as_str()
            }),
            semantic_roles: tokens(&M5ConstrainedFileStateRole::ALL, |v| v.as_str()),
            read_only_roles: tokens(&M5ConstrainedFileStateReadOnlyRole::ALL, |v| v.as_str()),
            generated_roles: tokens(&M5ConstrainedFileStateGeneratedRole::ALL, |v| v.as_str()),
            policy_locked_roles: tokens(&M5ConstrainedFileStatePolicyLockedRole::ALL, |v| {
                v.as_str()
            }),
            managed_roles: tokens(&M5ConstrainedFileStateManagedRole::ALL, |v| v.as_str()),
            projection_roles: tokens(&M5ConstrainedFileStateProjectionRole::ALL, |v| v.as_str()),
            captured_snapshot_roles: tokens(
                &M5ConstrainedFileStateCapturedSnapshotRole::ALL,
                |v| v.as_str(),
            ),
            surface_families: tokens(&M5ConstrainedFileStateSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5ConstrainedFileStateClassificationStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5ConstrainedFileStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ConstrainedFileStateAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5ConstrainedFileStateDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ConstrainedFileStateRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5ConstrainedFileStateDowngradeTrigger::ALL, |v| {
                v.as_str()
            }),
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
pub struct M5ConstrainedFileStateGovernanceReview {
    /// No constrained object looks directly writable by omission.
    pub no_constrained_object_looks_directly_writable_by_omission: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Write constrained state is mechanically distinct from directly writable state.
    pub write_constrained_state_is_mechanically_distinct_from_directly_writable_state: bool,
    /// Every constrained object names its state badge and blocked write reason.
    pub every_constrained_object_names_its_state_badge_and_blocked_write_reason: bool,
    /// Every constrained object names its canonical source or live target.
    pub every_constrained_object_names_its_canonical_source_or_live_target: bool,
    /// Every constrained object names its exact write target.
    pub every_constrained_object_names_its_exact_write_target: bool,
    /// Nearest safe action is named for every constrained object.
    pub nearest_safe_action_is_named_for_every_constrained_object: bool,
    /// No generated managed projection or archived object falls back to lossy direct write.
    pub no_generated_managed_projection_or_archived_object_falls_back_to_lossy_direct_write: bool,
    /// No ai automation import or repair flow bypasses constrained state rules.
    pub no_ai_automation_import_or_repair_flow_bypasses_constrained_state_rules: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single constrained file state source.
    pub support_export_reads_single_constrained_file_state_source: bool,
    /// Shell editor review ai help and support bind to single source.
    pub shell_editor_review_ai_help_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel constrained file state vocabulary.
    pub later_rows_cannot_invent_parallel_constrained_file_state_vocabulary: bool,
    /// Constrained file state truth survives zoom and high contrast.
    pub constrained_file_state_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateConsumerProjection {
    /// Shell and editor consume shared constrained file state truth.
    pub shell_and_editor_consume_shared_constrained_file_state_truth: bool,
    /// Review and ai consume shared write target and canonical source truth.
    pub review_and_ai_consume_shared_write_target_and_canonical_source_truth: bool,
    /// Help and support export consume shared blocked write truth.
    pub help_and_support_export_consume_shared_blocked_write_truth: bool,
    /// Docs help and screenshots read single constrained file state source.
    pub docs_help_and_screenshots_read_single_constrained_file_state_source: bool,
    /// Constrained objects bind to shared canonical source relation.
    pub constrained_objects_bind_to_shared_canonical_source_relation: bool,
    /// Support export reads single constrained file state source.
    pub support_export_reads_single_constrained_file_state_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the constrained-file-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting constrained-file-state audit for the lane.
    pub constrained_file_state_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ConstrainedFileStateMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ConstrainedFileStateMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Retired-state rows.
    pub constrained_file_state_rows: Vec<M5ConstrainedFileStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ConstrainedFileStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ConstrainedFileStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ConstrainedFileStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ConstrainedFileStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ConstrainedFileStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 constrained-file-state matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedFileStateMatrixPacket {
    /// Record kind; must equal [`M5_CONSTRAINED_FILE_STATE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Retired-state rows.
    pub constrained_file_state_rows: Vec<M5ConstrainedFileStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ConstrainedFileStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ConstrainedFileStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ConstrainedFileStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ConstrainedFileStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ConstrainedFileStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ConstrainedFileStateMatrixPacket {
    /// Builds an M5 constrained-file-state matrix packet from input.
    pub fn new(input: M5ConstrainedFileStateMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CONSTRAINED_FILE_STATE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            constrained_file_state_rows: input.constrained_file_state_rows,
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

    /// Validates the M5 constrained-file-state matrix invariants.
    pub fn validate(&self) -> Vec<M5ConstrainedFileStateMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_CONSTRAINED_FILE_STATE_MATRIX_RECORD_KIND {
            violations.push(M5ConstrainedFileStateMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_VERSION {
            violations.push(M5ConstrainedFileStateMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ConstrainedFileStateMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_constrained_file_state_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 constrained-file-state matrix serializes"),
        ) {
            violations.push(M5ConstrainedFileStateMatrixViolation::RawMaterialInExport);
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
            .expect("m5 constrained-file-state matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed constrained-file-state class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,write_disposition,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.constrained_file_state_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.write_disposition.as_str(),
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

    /// Deterministic constrained-object-health dashboard JSON that shell and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .constrained_file_state_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "write_disposition": row.write_disposition.as_str(),
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
            "record_kind": "m5_constrained_object_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_CONSTRAINED_FILE_STATE_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 constrained-object-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .constrained_file_state_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Constrained-File-State, Canonical-Source-Relation, and Write-Target-Review Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.constrained_file_state_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Constrained-file-state roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Capture-lifecycle stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.constrained_file_state_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (write_disposition: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.write_disposition.as_str()
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
                "  - Exact write target: {}\n",
                row.required_visible_state.exact_write_target
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

/// Errors emitted when reading the checked-in M5 constrained-file-state matrix export.
#[derive(Debug)]
pub enum M5ConstrainedFileStateMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ConstrainedFileStateMatrixViolation>),
}

impl fmt::Display for M5ConstrainedFileStateMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 constrained-file-state matrix export parse failed: {error}"
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
                    "m5 constrained-file-state matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ConstrainedFileStateMatrixArtifactError {}

/// Validation failures emitted by [`M5ConstrainedFileStateMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ConstrainedFileStateMatrixViolation {
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
    /// A constrained-file-state row is incomplete.
    ConstrainedFileStateRowIncomplete,
    /// A constrained-file-state row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A constrained-file-state row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no constrained-file-state roles.
    SemanticRoleMissing,
    /// The ReadOnly class declares no ReadOnly constrained-object roles.
    ReadOnlyRoleMissing,
    /// The Generated class declares no Generated constrained-object roles.
    GeneratedRoleMissing,
    /// The PolicyLocked class declares no PolicyLocked constrained-object roles.
    PolicyLockedRoleMissing,
    /// The Managed class declares no Managed constrained-object roles.
    ManagedRoleMissing,
    /// The Projection class declares no Projection constrained-object roles.
    ProjectionRoleMissing,
    /// The CapturedSnapshot class declares no CapturedSnapshot constrained-object roles.
    CapturedSnapshotRoleMissing,
    /// A class omits required transition metadata.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no removal-horizon stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard constrained-file-state invariant.
    ConstrainedFileStateInvariantViolated,
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

impl M5ConstrainedFileStateMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ConstrainedFileStateRowIncomplete => "constrained_file_state_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ReadOnlyRoleMissing => "read_only_role_missing",
            Self::GeneratedRoleMissing => "generated_role_missing",
            Self::PolicyLockedRoleMissing => "policy_locked_role_missing",
            Self::ManagedRoleMissing => "managed_role_missing",
            Self::ProjectionRoleMissing => "projection_role_missing",
            Self::CapturedSnapshotRoleMissing => "captured_snapshot_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::ConstrainedFileStateInvariantViolated => {
                "constrained_file_state_invariant_violated"
            }
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 constrained-file-state matrix export.
pub fn current_stable_m5_constrained_file_state_matrix_export(
) -> Result<M5ConstrainedFileStateMatrixPacket, M5ConstrainedFileStateMatrixArtifactError> {
    let packet: M5ConstrainedFileStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-constrained-object-state/support_export.json"
    )))
    .map_err(M5ConstrainedFileStateMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ConstrainedFileStateMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF,
        M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
        M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ConstrainedFileStateMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ConstrainedFileStateMatrixViolation::VocabularySetDrift);
    }
}

fn validate_constrained_file_state_rows(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    let present: BTreeSet<M5ConstrainedFileStateObject> = packet
        .constrained_file_state_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5ConstrainedFileStateObject::ALL {
        if !present.contains(&required) {
            violations.push(M5ConstrainedFileStateMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.constrained_file_state_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations
                .push(M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ConstrainedFileStateMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5ConstrainedFileStateMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_read_only_roles() && row.read_only_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::ReadOnlyRoleMissing);
        }
        if class.declares_generated_roles() && row.generated_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::GeneratedRoleMissing);
        }
        if class.declares_policy_locked_roles() && row.policy_locked_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::PolicyLockedRoleMissing);
        }
        if class.declares_managed_roles() && row.managed_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::ManagedRoleMissing);
        }
        if class.declares_projection_roles() && row.projection_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::ProjectionRoleMissing);
        }
        if class.declares_captured_snapshot_roles() && row.captured_snapshot_roles.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::CapturedSnapshotRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5ConstrainedFileStateMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ConstrainedFileStateMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations
                .push(M5ConstrainedFileStateMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations
                .push(M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_constrained_object_looks_directly_writable_by_omission,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.write_constrained_state_is_mechanically_distinct_from_directly_writable_state,
        review.every_constrained_object_names_its_state_badge_and_blocked_write_reason,
        review.every_constrained_object_names_its_canonical_source_or_live_target,
        review.every_constrained_object_names_its_exact_write_target,
        review.nearest_safe_action_is_named_for_every_constrained_object,
        review.no_generated_managed_projection_or_archived_object_falls_back_to_lossy_direct_write,
        review.no_ai_automation_import_or_repair_flow_bypasses_constrained_state_rules,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_constrained_file_state_source,
        review.shell_editor_review_ai_help_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_constrained_file_state_vocabulary,
        review.constrained_file_state_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5ConstrainedFileStateMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_editor_consume_shared_constrained_file_state_truth,
        projection.review_and_ai_consume_shared_write_target_and_canonical_source_truth,
        projection.help_and_support_export_consume_shared_blocked_write_truth,
        projection.docs_help_and_screenshots_read_single_constrained_file_state_source,
        projection.constrained_objects_bind_to_shared_canonical_source_relation,
        projection.support_export_reads_single_constrained_file_state_source,
    ] {
        if !ok {
            violations.push(M5ConstrainedFileStateMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ConstrainedFileStateMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ConstrainedFileStateMatrixPacket,
    violations: &mut Vec<M5ConstrainedFileStateMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.constrained_file_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ConstrainedFileStateMatrixViolation::ReleasePostureIncomplete);
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
/// deliberately uses read-only / generated / policy-locked / managed / projection words; what is rejected is a raw secret
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
