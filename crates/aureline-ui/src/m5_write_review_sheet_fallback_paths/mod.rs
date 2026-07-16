//! Write-review sheets that turn a blocked write on a B150 constrained current object into an **explicit
//! reviewed transition** — duplicate to an editable copy, detach from a managed source, create an overlay
//! patch, request approval, or regenerate with preview — instead of a silent best-effort fallback.
//!
//! This module is the B150 write-review-sheet lane over the six constrained-current-object classes frozen in
//! [`crate::m5_constrained_file_state_matrix`]. Where the badge-group / reason-strip consumer lane
//! ([`crate::m5_file_state_badge_group_and_reason_strip_consumers`]) proves *how a constrained object is
//! labelled* and the canonical-source / write-target registries
//! ([`crate::m5_canonical_source_relation_and_write_target_review_registries`]) prove *where authoritative edits
//! land*, this lane proves *what happens when the user tries to write and the current object cannot be written
//! directly*: a reviewed sheet is shown before commit that names the exact write target, the side effects, the
//! preserved-versus-lost sync or regenerate path, any required approvals, the checkpoint / undo class, and an
//! export-safe explanation — so the transition is understood and reversible before anything changes, and no
//! constrained object is ever silently mutated through a lossy direct write.
//!
//! One sheet model is reused across every originating flow that can hit a constrained object — a direct save, a
//! code action, an AI apply, an importer, a repair, and a batch edit — so an AI apply and a direct save that
//! land on the same constrained object get the *same* reviewed transition rather than one of them slipping a
//! hidden bypass. The lane proves — by fixtures, not screenshots — that the same constrained-object profile
//! carries identical review content across the flows that reach it, that every one of the five fallback paths
//! can be reviewed before commit with explicit retained-versus-lost behaviour, and that a recovery / undo class
//! is visible before commit on every seeded path.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Five reviewed fallback paths, each retained-versus-lost explicit.** At least one duplicate, one detach,
//!    one overlay, one request-approval, and one regenerate-first path
//!    ([`WriteReviewFallbackAction`]) is reviewable before commit, each carrying an explicit
//!    [`PreservedVersusLostSync`] naming what is retained, what is lost, and the sync / regenerate path.
//! 2. **No silent lossy fallback.** Every binding is reviewed before commit and carries a write-constrained
//!    disposition; a silent lossy direct write is disabled by construction (no direct-write action can be
//!    represented, only [`WriteReviewAction::CommitReviewedTransition`] on the full sheet). No binding may let a
//!    constrained object silently fall back to a lossy direct write, give an AI / automation / import / repair
//!    flow a hidden bypass, leave the exact write target or preserved-versus-lost sync unstated, hide the
//!    recovery / undo class before commit, or let one state class hide another when both materially affect
//!    behaviour.
//! 3. **Recovery visible before commit.** Every binding names a [`CheckpointUndoClass`] — the recovery / undo
//!    class made visible before the reviewed transition commits — matched to its fallback path, and the
//!    keyboard and screen-reader routes ([`crate::m5_constrained_file_state_matrix::M5ConstrainedFileStateAccessibilityRoute`])
//!    through which the write target, reason, and recovery class can be discovered without pointer-only chrome.
//!
//! Narrowing is disclosed, never hidden: a compacted precondition notice or an exported, export-safe view
//! carries an explicit [`ReviewNarrowNote`] naming the reason, the preserved review content, and the next
//! action, so a surface may narrow *which* actions remain without ever rewording the reviewed-transition content
//! or quietly implying the object is directly writable.
//!
//! The packet references upstream constrained-file-state contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/program/m5-write-review-sheet-fallback-paths.schema.json`](../../../../schemas/program/m5-write-review-sheet-fallback-paths.schema.json).
//! The contract doc is
//! [`docs/support/m5_write_review_sheet_fallback_paths.md`](../../../../docs/support/m5_write_review_sheet_fallback_paths.md).
//! The protected fixture directory is
//! [`fixtures/editor/m5-write-review-sheet-fallback-paths/`](../../../../fixtures/editor/m5-write-review-sheet-fallback-paths/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_write_review_sheet_fallback_paths,
    seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed,
    seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed,
};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateObject,
    M5ConstrainedFileStateWriteDisposition, M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5WriteReviewSheetFallbackPathsPacket`].
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_RECORD_KIND: &str =
    "m5_write_review_sheet_fallback_path_registry";

/// Schema version for write-review-sheet fallback-path records.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PACKET_ID: &str =
    "m5-write-review-sheet-fallback-paths:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_REF: &str =
    "schemas/program/m5-write-review-sheet-fallback-paths.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_DOC_REF: &str =
    "docs/support/m5_write_review_sheet_fallback_paths.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_ARTIFACT_REF: &str =
    "artifacts/support/m5-write-review-sheet-fallback-paths/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_CSV_REF: &str =
    "artifacts/support/m5-write-review-sheet-fallback-paths/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_REPORT_REF: &str =
    "artifacts/support/m5-write-review-sheet-fallback-paths/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_FIXTURE_DIR: &str =
    "fixtures/editor/m5-write-review-sheet-fallback-paths";

/// Proof-freshness SLO in hours for this lane.
pub const M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PROOF_SLO_HOURS: u32 = 720;

/// Write-disposition sentinel words a reviewed fallback path may never fall back to; a reviewed transition must
/// always keep a real write-constrained disposition rather than implying the object is directly writable,
/// editable, or unconstrained.
const WRITE_DISPOSITION_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "directly_writable",
    "writable",
    "editable",
    "unconstrained",
];

/// One of the five reviewed fallback transitions this lane operationalizes in place of a silent best-effort
/// direct write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteReviewFallbackAction {
    /// Duplicate the constrained object into a new editable copy, leaving the original untouched.
    DuplicateToEditableCopy,
    /// Detach a local fork from a managed / externally-owned source, stopping automatic upstream sync.
    DetachFromManagedSource,
    /// Record edits as an overlay patch layered over the backing source object.
    CreateOverlayPatch,
    /// Open an approval request to the policy owner instead of silently overriding a lock.
    RequestApproval,
    /// Regenerate the artifact from its generator input with a preview before commit.
    RegenerateWithPreview,
}

impl WriteReviewFallbackAction {
    /// Every fallback action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DuplicateToEditableCopy,
        Self::DetachFromManagedSource,
        Self::CreateOverlayPatch,
        Self::RequestApproval,
        Self::RegenerateWithPreview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateToEditableCopy => "duplicate_to_editable_copy",
            Self::DetachFromManagedSource => "detach_from_managed_source",
            Self::CreateOverlayPatch => "create_overlay_patch",
            Self::RequestApproval => "request_approval",
            Self::RegenerateWithPreview => "regenerate_with_preview",
        }
    }

    /// The write-constrained disposition this fallback path is the reviewed transition for; never
    /// [`M5ConstrainedFileStateWriteDisposition::DirectlyWritable`].
    pub const fn required_write_disposition(self) -> M5ConstrainedFileStateWriteDisposition {
        match self {
            Self::DuplicateToEditableCopy => {
                M5ConstrainedFileStateWriteDisposition::ReadOnlyBlocked
            }
            Self::DetachFromManagedSource | Self::CreateOverlayPatch => {
                M5ConstrainedFileStateWriteDisposition::DetachRequired
            }
            Self::RequestApproval => M5ConstrainedFileStateWriteDisposition::ApprovalGated,
            Self::RegenerateWithPreview => M5ConstrainedFileStateWriteDisposition::RegenerateOnly,
        }
    }

    /// The recovery / undo class this fallback path always makes visible before commit.
    pub const fn required_checkpoint_undo_class(self) -> CheckpointUndoClass {
        match self {
            Self::DuplicateToEditableCopy => CheckpointUndoClass::NewCopyLeavesOriginalIntact,
            Self::DetachFromManagedSource => CheckpointUndoClass::DetachCheckpointRestorable,
            Self::CreateOverlayPatch => CheckpointUndoClass::OverlayPatchRevertible,
            Self::RequestApproval => CheckpointUndoClass::ApprovalRequestWithdrawable,
            Self::RegenerateWithPreview => CheckpointUndoClass::RegeneratePreviewDiscardable,
        }
    }
}

/// The originating flow whose blocked write is routed into the shared write-review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteReviewOriginatingFlow {
    /// A direct editor save.
    DirectSave,
    /// A code action / quick fix.
    CodeAction,
    /// An AI apply / assisted edit.
    AiApply,
    /// An importer bringing content in from outside.
    Importer,
    /// A repair / doctor flow.
    Repair,
    /// A batch edit across many files.
    BatchEdit,
}

impl WriteReviewOriginatingFlow {
    /// Every originating flow, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DirectSave,
        Self::CodeAction,
        Self::AiApply,
        Self::Importer,
        Self::Repair,
        Self::BatchEdit,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectSave => "direct_save",
            Self::CodeAction => "code_action",
            Self::AiApply => "ai_apply",
            Self::Importer => "importer",
            Self::Repair => "repair",
            Self::BatchEdit => "batch_edit",
        }
    }

    /// Whether this flow is an actor-parity mutation path (AI, automation, import, or repair) that must never
    /// get a hidden bypass around the write-review sheet.
    pub const fn is_actor_parity_mutation_flow(self) -> bool {
        matches!(self, Self::AiApply | Self::Importer | Self::Repair)
    }
}

/// The recovery / undo class a reviewed transition makes visible before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointUndoClass {
    /// A new copy is written and the original is left intact, so the transition is undone by discarding the copy.
    NewCopyLeavesOriginalIntact,
    /// Detach records a restorable checkpoint, so upstream sync can be re-linked.
    DetachCheckpointRestorable,
    /// The overlay patch is revertible, so the backing source is unchanged.
    OverlayPatchRevertible,
    /// The approval request can be withdrawn before it is granted, so nothing changes meanwhile.
    ApprovalRequestWithdrawable,
    /// The regenerate preview is discardable and keeps a restore point for the previous render.
    RegeneratePreviewDiscardable,
}

impl CheckpointUndoClass {
    /// Every checkpoint / undo class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NewCopyLeavesOriginalIntact,
        Self::DetachCheckpointRestorable,
        Self::OverlayPatchRevertible,
        Self::ApprovalRequestWithdrawable,
        Self::RegeneratePreviewDiscardable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewCopyLeavesOriginalIntact => "new_copy_leaves_original_intact",
            Self::DetachCheckpointRestorable => "detach_checkpoint_restorable",
            Self::OverlayPatchRevertible => "overlay_patch_revertible",
            Self::ApprovalRequestWithdrawable => "approval_request_withdrawable",
            Self::RegeneratePreviewDiscardable => "regenerate_preview_discardable",
        }
    }
}

/// The review posture a write-review sheet takes on one binding.
///
/// The posture governs the discoverable action set and narrowing disclosure, never the reviewed-transition
/// content: a narrowed posture still carries the same write target, side effects, preserved-versus-lost sync,
/// required approvals, checkpoint / undo class, and export-safe explanation, and discloses the narrowing through
/// an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSheetPosture {
    /// The full interactive review sheet is shown before commit; it offers the reviewed-transition commit
    /// action.
    FullReviewSheet,
    /// A compact precondition notice (a status chip, a code-action lightbulb) renders the reviewed-transition
    /// content narrowed, disclosed through a note, with no commit action.
    PreconditionNoticeCompact,
    /// An exported, export-safe-redacted rendering of the reviewed transition in a support packet.
    ExportRedacted,
}

impl ReviewSheetPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FullReviewSheet,
        Self::PreconditionNoticeCompact,
        Self::ExportRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullReviewSheet => "full_review_sheet",
            Self::PreconditionNoticeCompact => "precondition_notice_compact",
            Self::ExportRedacted => "export_redacted",
        }
    }

    /// Whether this posture narrows below the full review-sheet disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullReviewSheet)
    }
}

/// A discoverable action a write-review sheet may expose.
///
/// The set is deliberately closed and safe: there is no direct-write action variant, so a review sheet can never
/// present a control that performs a silent lossy direct write. The only write-capable action is committing the
/// reviewed transition itself, present only where the full sheet is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteReviewAction {
    /// Inspect the exact write target and side effects.
    InspectWriteTarget,
    /// Copy the preserved-versus-lost sync / regenerate summary.
    CopyPreservedVersusLost,
    /// Reveal the canonical source the constrained object relates back to.
    RevealCanonicalSource,
    /// Commit the reviewed transition (duplicate / detach / overlay / request-approval / regenerate) — only
    /// where the full review sheet is rendered.
    CommitReviewedTransition,
}

impl WriteReviewAction {
    /// The safe base action set present on every review-sheet binding.
    pub const SAFE_BASE: [Self; 3] = [
        Self::InspectWriteTarget,
        Self::CopyPreservedVersusLost,
        Self::RevealCanonicalSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectWriteTarget => "inspect_write_target",
            Self::CopyPreservedVersusLost => "copy_preserved_versus_lost",
            Self::RevealCanonicalSource => "reveal_canonical_source",
            Self::CommitReviewedTransition => "commit_reviewed_transition",
        }
    }
}

/// Why a write-review sheet narrowed its action set below a full review-sheet view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewNarrowReason {
    /// The reviewed-transition content was compacted to a precondition notice; only inspect / copy / reveal
    /// remain.
    CompactedToPreconditionNotice,
    /// An exported view redacted its surrounding detail export-safe.
    ExportRedactionNarrowed,
}

impl ReviewNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactedToPreconditionNotice => "compacted_to_precondition_notice",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewNarrowNextAction {
    /// Open the full review sheet behind the compact precondition notice.
    OpenFullReviewSheet,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ReviewNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFullReviewSheet => "open_full_review_sheet",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves the full review-sheet view or discloses a narrowed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewParityState {
    /// The reviewed-transition content and full action set are preserved and shown.
    ContentPreserved,
    /// The reviewed-transition content is preserved and a narrowed action set is explicitly disclosed.
    ContentDisclosedNarrowed,
}

impl ReviewParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentPreserved => "content_preserved",
            Self::ContentDisclosedNarrowed => "content_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this write-review-sheet lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteReviewSheetFallbackPathsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Review content drifted between the flows that render the same profile.
    ReviewContentDriftDetected,
    /// A reviewed transition dropped its write-constrained disposition and began to imply the object is directly
    /// writable.
    WriteDispositionDroppedForConstrainedObject,
    /// A constrained write path silently mutated the current object through a lossy fallback.
    SilentlyMutatesCurrentObjectThroughLossyFallback,
    /// An AI / automation / import / repair flow got a hidden bypass around the write-review sheet.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A sheet left the exact write target or preserved-versus-lost sync unstated.
    LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated,
    /// A sheet hid the recovery / undo class before commit.
    HidesRecoveryOrUndoClassBeforeCommit,
    /// A sheet let one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// An accessibility route for the write target, reason, or recovery class was dropped.
    AccessibilityRouteDropped,
    /// An export view lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream constrained-file-state contract narrowed.
    UpstreamConstrainedFileStateNarrowed,
}

impl WriteReviewSheetFallbackPathsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ReviewContentDriftDetected,
        Self::WriteDispositionDroppedForConstrainedObject,
        Self::SilentlyMutatesCurrentObjectThroughLossyFallback,
        Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
        Self::LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated,
        Self::HidesRecoveryOrUndoClassBeforeCommit,
        Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamConstrainedFileStateNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ReviewContentDriftDetected => "review_content_drift_detected",
            Self::WriteDispositionDroppedForConstrainedObject => {
                "write_disposition_dropped_for_constrained_object"
            }
            Self::SilentlyMutatesCurrentObjectThroughLossyFallback => {
                "silently_mutates_current_object_through_lossy_fallback"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated => {
                "leaves_exact_write_target_or_preserved_versus_lost_sync_unstated"
            }
            Self::HidesRecoveryOrUndoClassBeforeCommit => {
                "hides_recovery_or_undo_class_before_commit"
            }
            Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior => {
                "lets_one_state_class_hide_another_when_both_materially_affect_behavior"
            }
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamConstrainedFileStateNarrowed => {
                "upstream_constrained_file_state_narrowed"
            }
        }
    }
}

/// The explicit preserved-versus-lost sync / regenerate summary a reviewed transition carries.
///
/// Naming what is retained, what (if anything) is lost, and the sync or regenerate path is how a reviewed
/// transition stays honest instead of implying a lossless in-place write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedVersusLostSync {
    /// What the reviewed transition retains (never empty).
    pub retained: Vec<String>,
    /// What the reviewed transition loses (may be empty when nothing is lost).
    pub lost: Vec<String>,
    /// The sync or regenerate path that keeps or re-establishes truth (never empty).
    pub sync_or_regenerate_path: String,
}

impl PreservedVersusLostSync {
    /// Whether the retained set and sync / regenerate path are stated and every listed item is non-empty.
    pub fn is_explicit(&self) -> bool {
        !self.retained.is_empty()
            && self.retained.iter().all(|item| !item.trim().is_empty())
            && self.lost.iter().all(|item| !item.trim().is_empty())
            && !self.sync_or_regenerate_path.trim().is_empty()
    }
}

/// The reviewed-transition content a constrained-object profile presents.
///
/// These values must be identical across every originating flow that reviews the same profile. A flow may narrow
/// which actions remain, but it may never reword any of these values per flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteReviewSheetContent {
    /// The exact write target the reviewed transition changes.
    pub write_target_word: String,
    /// The write-constrained disposition token that keeps the object mechanically distinct from a
    /// directly-writable object.
    pub write_disposition_word: String,
    /// The side effects of committing the reviewed transition (never empty).
    pub side_effect_words: Vec<String>,
    /// The preserved-versus-lost sync / regenerate summary.
    pub preserved_versus_lost: PreservedVersusLostSync,
    /// Any approvals the reviewed transition requires (empty when none are required).
    pub required_approval_words: Vec<String>,
    /// The recovery / undo class made visible before commit.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// The canonical source the constrained object relates back to.
    pub canonical_source_word: String,
    /// The export-safe explanation of the reviewed transition.
    pub export_safe_explanation: String,
    /// The controlled labels for any co-applicable state classes (empty for a single-state object); when an
    /// object is multi-state both facts stay visible here rather than one state hiding another.
    pub co_applicable_state_labels: Vec<String>,
}

impl WriteReviewSheetContent {
    /// Whether every scalar content field and the preserved-versus-lost summary are present.
    pub fn all_present(&self) -> bool {
        !self.write_target_word.trim().is_empty()
            && !self.write_disposition_word.trim().is_empty()
            && !self.side_effect_words.is_empty()
            && self
                .side_effect_words
                .iter()
                .all(|item| !item.trim().is_empty())
            && self.preserved_versus_lost.is_explicit()
            && self
                .required_approval_words
                .iter()
                .all(|item| !item.trim().is_empty())
            && !self.canonical_source_word.trim().is_empty()
            && !self.export_safe_explanation.trim().is_empty()
    }

    /// Whether the write-disposition word is a real write-constrained disposition and never collapses to a
    /// directly-writable / writable / editable / unconstrained sentinel.
    pub fn write_disposition_satisfied(&self) -> bool {
        let disposition = self.write_disposition_word.trim().to_lowercase();
        !disposition.is_empty()
            && !WRITE_DISPOSITION_ABSENT_SENTINELS.contains(&disposition.as_str())
    }

    /// Whether every co-applicable state label is non-empty.
    pub fn co_applicable_labels_present(&self) -> bool {
        self.co_applicable_state_labels
            .iter()
            .all(|label| !label.trim().is_empty())
    }
}

/// The explicit note a narrowed posture shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewNarrowNote {
    /// Why the posture narrowed.
    pub reason: ReviewNarrowReason,
    /// Note naming the preserved reviewed-transition content (never omitted).
    pub preserved_content_note: String,
    /// The next action offered.
    pub next_action: ReviewNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a binding must carry, derived from its posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewRenderDisclosure {
    /// The parity state the posture requires.
    pub parity_state: ReviewParityState,
    /// The narrow reason the posture requires, if any.
    pub narrow_reason: Option<ReviewNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ReviewNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
    /// Whether the binding offers the reviewed-transition commit action.
    pub offers_reviewed_commit: bool,
}

/// Resolves the render disclosures a binding must carry from its posture.
///
/// The full review-sheet posture renders the full safe action set plus the reviewed-transition commit action. A
/// compact precondition notice and an exported view each narrow the action set and disclose the narrowing
/// through an explicit note — but both keep every reviewed-transition content value.
pub const fn resolve_review_render_disclosure(
    posture: ReviewSheetPosture,
) -> ReviewRenderDisclosure {
    match posture {
        ReviewSheetPosture::FullReviewSheet => ReviewRenderDisclosure {
            parity_state: ReviewParityState::ContentPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_export_detail_note: false,
            offers_reviewed_commit: true,
        },
        ReviewSheetPosture::PreconditionNoticeCompact => ReviewRenderDisclosure {
            parity_state: ReviewParityState::ContentDisclosedNarrowed,
            narrow_reason: Some(ReviewNarrowReason::CompactedToPreconditionNotice),
            narrow_next_action: Some(ReviewNarrowNextAction::OpenFullReviewSheet),
            needs_narrow_note: true,
            needs_export_detail_note: false,
            offers_reviewed_commit: false,
        },
        ReviewSheetPosture::ExportRedacted => ReviewRenderDisclosure {
            parity_state: ReviewParityState::ContentDisclosedNarrowed,
            narrow_reason: Some(ReviewNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(ReviewNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_export_detail_note: true,
            offers_reviewed_commit: false,
        },
    }
}

/// Whether a review posture is the export / support rendering that must map a profile back to canonical
/// contracts by id.
pub const fn posture_must_reference_canonical(posture: ReviewSheetPosture) -> bool {
    matches!(posture, ReviewSheetPosture::ExportRedacted)
}

/// One review binding: a constrained-object profile reviewed through one originating flow at one posture for one
/// fallback path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteReviewSheetBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable constrained-object-profile id (shared across flows that review the same profile).
    pub object_profile_id: String,
    /// Human-readable constrained-object-profile identity.
    pub object_profile_label: String,
    /// Which constrained-object class this binding reviews as its primary state.
    pub object_class: M5ConstrainedFileStateObject,
    /// Any co-applicable state classes that also apply (empty for a single-state object); when present, both
    /// facts must stay visible.
    pub co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    /// Which reviewed fallback transition this binding reviews.
    pub fallback_action: WriteReviewFallbackAction,
    /// Which originating flow's blocked write is routed here.
    pub originating_flow: WriteReviewOriginatingFlow,
    /// Which review posture this rendering takes.
    pub posture: ReviewSheetPosture,
    /// The reviewed-transition content presented (identical across flows for one profile).
    pub review_content: WriteReviewSheetContent,
    /// Whether content is preserved in full or a narrowing is disclosed.
    pub parity_state: ReviewParityState,
    /// The discoverable action set allowed on this review view.
    pub allowed_actions: Vec<WriteReviewAction>,
    /// The accessibility routes through which the write target, reason, and recovery class can be discovered
    /// without pointer-only chrome.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// The explicit narrow note; required and complete when the posture narrows.
    pub narrow_note: Option<ReviewNarrowNote>,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Positive invariant: this fallback path is reviewed before commit. MUST be `true`.
    pub reviewed_before_commit: bool,
    /// Positive invariant: the recovery / undo class is visible before commit. MUST be `true`.
    pub recovery_visible_before_commit: bool,
    /// Guardrail: this path silently mutates the current object through a lossy fallback. MUST be `false`.
    pub silently_mutates_current_object_through_lossy_fallback: bool,
    /// Guardrail: this path gives an AI / automation / import / repair flow a hidden bypass around the
    /// write-review sheet. MUST be `false`.
    pub gives_ai_automation_import_or_repair_flows_a_hidden_bypass: bool,
    /// Guardrail: this sheet leaves the exact write target or preserved-versus-lost sync unstated. MUST be
    /// `false`.
    pub leaves_exact_write_target_or_preserved_versus_lost_sync_unstated: bool,
    /// Guardrail: this sheet hides the recovery / undo class before commit. MUST be `false`.
    pub hides_recovery_or_undo_class_before_commit: bool,
    /// Guardrail: this sheet lets one state class hide another when both materially affect behavior. MUST be
    /// `false`.
    pub lets_one_state_class_hide_another_when_both_materially_affect_behavior: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl WriteReviewSheetBinding {
    /// Disclosures this binding must carry, derived from its posture.
    pub const fn disclosure(&self) -> ReviewRenderDisclosure {
        resolve_review_render_disclosure(self.posture)
    }

    /// Whether this binding renders below the full review-sheet view.
    pub const fn is_narrowed(&self) -> bool {
        self.posture.is_narrowed()
    }

    /// Whether this binding reviews a multi-state (more than one co-applicable constraint) object.
    pub fn is_multi_state(&self) -> bool {
        !self.co_applicable_states.is_empty()
    }

    /// Whether both positive review invariants hold, as required.
    pub const fn positive_invariants_hold(&self) -> bool {
        self.reviewed_before_commit && self.recovery_visible_before_commit
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.silently_mutates_current_object_through_lossy_fallback
            && !self.gives_ai_automation_import_or_repair_flows_a_hidden_bypass
            && !self.leaves_exact_write_target_or_preserved_versus_lost_sync_unstated
            && !self.hides_recovery_or_undo_class_before_commit
            && !self.lets_one_state_class_hide_another_when_both_materially_affect_behavior
    }

    /// Whether the safe base action set is present.
    pub fn has_safe_base_actions(&self) -> bool {
        WriteReviewAction::SAFE_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether the reviewed-commit action is present exactly when the posture offers it.
    pub fn commit_action_matches_posture(&self) -> bool {
        let offered = self.disclosure().offers_reviewed_commit;
        let present = self
            .allowed_actions
            .contains(&WriteReviewAction::CommitReviewedTransition);
        offered == present
    }

    /// Whether the content's write disposition matches the fallback path's required write-constrained
    /// disposition.
    pub fn write_disposition_matches_action(&self) -> bool {
        self.review_content.write_disposition_word.trim()
            == self.fallback_action.required_write_disposition().as_str()
    }

    /// Whether the content's checkpoint / undo class matches the fallback path's required recovery class.
    pub fn checkpoint_matches_action(&self) -> bool {
        self.review_content.checkpoint_undo_class
            == self.fallback_action.required_checkpoint_undo_class()
    }

    /// Whether the multi-state facets stay consistent: the binding's co-applicable state classes, the content
    /// labels, and the requirement that every co-state is distinct from the primary object class all hold, so no
    /// co-applicable state is hidden.
    pub fn multi_state_facets_consistent(&self) -> bool {
        if self.co_applicable_states.len() != self.review_content.co_applicable_state_labels.len() {
            return false;
        }
        if !self.review_content.co_applicable_labels_present() {
            return false;
        }
        let mut seen: BTreeSet<M5ConstrainedFileStateObject> = BTreeSet::new();
        seen.insert(self.object_class);
        for state in &self.co_applicable_states {
            if !seen.insert(*state) {
                return false;
            }
        }
        true
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5ConstrainedFileStateAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5ConstrainedFileStateAccessibilityRoute::ScreenReaderAnnounced)
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object_class.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteReviewSheetTrustReview {
    /// Fallback-path reuse is proven by fixtures rather than inferred from screenshots.
    pub fallback_action_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same reviewed-transition content across flows.
    pub same_profile_same_review_content_across_flows: bool,
    /// A reviewed transition's write disposition never masquerades as a directly-writable object.
    pub write_disposition_never_masquerades_as_directly_writable: bool,
    /// No constrained write path silently falls back to a lossy direct write.
    pub no_silent_lossy_direct_write_fallback: bool,
    /// AI / automation / import / repair flows never get a hidden bypass around the write-review sheet.
    pub no_hidden_bypass_for_ai_automation_import_repair: bool,
    /// Exact write target and preserved-versus-lost sync are always stated.
    pub exact_write_target_and_preserved_versus_lost_sync_always_stated: bool,
    /// A recovery / undo class is visible before commit on every path.
    pub recovery_or_undo_class_visible_before_commit: bool,
    /// A multi-state object always keeps every co-applicable state visible.
    pub multi_state_objects_keep_every_state_visible: bool,
    /// Accessibility routes for the write target, reason, and recovery class are present.
    pub accessibility_routes_present_for_write_target_reason_and_recovery: bool,
    /// Narrowing is disclosed across full, precondition-notice, and exported postures.
    pub narrowing_disclosed_across_postures: bool,
    /// Export views point at the canonical contracts.
    pub export_views_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the fallback path.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl WriteReviewSheetTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.fallback_action_reuse_proven_by_fixtures
            && self.same_profile_same_review_content_across_flows
            && self.write_disposition_never_masquerades_as_directly_writable
            && self.no_silent_lossy_direct_write_fallback
            && self.no_hidden_bypass_for_ai_automation_import_repair
            && self.exact_write_target_and_preserved_versus_lost_sync_always_stated
            && self.recovery_or_undo_class_visible_before_commit
            && self.multi_state_objects_keep_every_state_visible
            && self.accessibility_routes_present_for_write_target_reason_and_recovery
            && self.narrowing_disclosed_across_postures
            && self.export_views_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Flow-reuse projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteReviewSheetFlowProjection {
    /// The direct-save flow reuses the shared sheet.
    pub direct_save_reuses_sheet: bool,
    /// The code-action flow reuses the shared sheet.
    pub code_action_reuses_sheet: bool,
    /// The AI-apply flow reuses the shared sheet.
    pub ai_apply_reuses_sheet: bool,
    /// The importer flow reuses the shared sheet.
    pub importer_reuses_sheet: bool,
    /// The repair flow reuses the shared sheet.
    pub repair_reuses_sheet: bool,
    /// The batch-edit flow reuses the shared sheet.
    pub batch_edit_reuses_sheet: bool,
    /// Every fallback action is reviewed through two or more distinct flows.
    pub every_fallback_action_reviewed_by_two_or_more_flows: bool,
    /// Review content is identical for the same profile.
    pub review_content_identical_for_same_profile: bool,
    /// Multi-state objects keep both facts visible.
    pub multi_state_objects_keep_both_facts_visible: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a binding back to one constrained-file-state object class.
    pub export_maps_back_to_one_constrained_file_state_object: bool,
    /// Duplicate, detach, overlay, request-approval, and regenerate paths are all reviewable.
    pub duplicate_detach_overlay_request_approval_and_regenerate_all_reviewable: bool,
    /// A recovery / undo class is visible before commit on every path.
    pub recovery_visible_before_commit_on_every_path: bool,
    /// No constrained write silently mutates the current object.
    pub no_constrained_write_silently_mutates_current_object: bool,
}

impl WriteReviewSheetFlowProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.direct_save_reuses_sheet
            && self.code_action_reuses_sheet
            && self.ai_apply_reuses_sheet
            && self.importer_reuses_sheet
            && self.repair_reuses_sheet
            && self.batch_edit_reuses_sheet
            && self.every_fallback_action_reviewed_by_two_or_more_flows
            && self.review_content_identical_for_same_profile
            && self.multi_state_objects_keep_both_facts_visible
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_constrained_file_state_object
            && self.duplicate_detach_overlay_request_approval_and_regenerate_all_reviewable
            && self.recovery_visible_before_commit_on_every_path
            && self.no_constrained_write_silently_mutates_current_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteReviewSheetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5WriteReviewSheetFallbackPathsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WriteReviewSheetFallbackPathsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review bindings.
    pub review_bindings: Vec<WriteReviewSheetBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WriteReviewSheetFallbackPathsDowngradeTrigger>,
    /// Originating flows this packet covers.
    pub originating_flows: Vec<WriteReviewOriginatingFlow>,
    /// Trust review block.
    pub trust_review: WriteReviewSheetTrustReview,
    /// Flow-reuse projection block.
    pub flow_projection: WriteReviewSheetFlowProjection,
    /// Proof freshness block.
    pub proof_freshness: WriteReviewSheetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe write-review-sheet fallback-path packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WriteReviewSheetFallbackPathsPacket {
    /// Record kind; must equal [`M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review bindings.
    pub review_bindings: Vec<WriteReviewSheetBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WriteReviewSheetFallbackPathsDowngradeTrigger>,
    /// Originating flows this packet covers.
    pub originating_flows: Vec<WriteReviewOriginatingFlow>,
    /// Trust review block.
    pub trust_review: WriteReviewSheetTrustReview,
    /// Flow-reuse projection block.
    pub flow_projection: WriteReviewSheetFlowProjection,
    /// Proof freshness block.
    pub proof_freshness: WriteReviewSheetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WriteReviewSheetFallbackPathsPacket {
    /// Builds a write-review-sheet fallback-path packet from stable-lane input.
    pub fn new(input: M5WriteReviewSheetFallbackPathsPacketInput) -> Self {
        Self {
            record_kind: M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_RECORD_KIND.to_owned(),
            schema_version: M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            review_bindings: input.review_bindings,
            downgrade_triggers: input.downgrade_triggers,
            originating_flows: input.originating_flows,
            trust_review: input.trust_review,
            flow_projection: input.flow_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the write-review-sheet fallback-path invariants.
    pub fn validate(&self) -> Vec<M5WriteReviewSheetFallbackPathsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_RECORD_KIND {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_VERSION {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::DowngradeTriggersMissing);
        }
        if self.originating_flows.is_empty() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::OriginatingFlowsMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::TrustReviewIncomplete);
        }
        if !self.flow_projection.all_hold() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::FlowProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("write-review-sheet fallback-path packet serializes"),
        ) {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::RawBoundaryMaterialInExport);
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
            .expect("write-review-sheet fallback-path packet serializes")
    }

    /// Deterministic matrix CSV, one row per review binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,co_applicable_states,fallback_action,originating_flow,posture,checkpoint_undo_class,parity_state\n",
        );
        for binding in &self.review_bindings {
            let co_states = binding
                .co_applicable_states
                .iter()
                .map(|state| state.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                co_states,
                binding.fallback_action.as_str(),
                binding.originating_flow.as_str(),
                binding.posture.as_str(),
                binding.review_content.checkpoint_undo_class.as_str(),
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .review_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();
        let multi_state = self
            .review_bindings
            .iter()
            .filter(|binding| binding.is_multi_state())
            .count();

        let mut out = String::new();
        out.push_str("# Write-Review Sheets: Reviewed Fallback Transitions Across Flows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Review bindings: {} ({} narrowed, {} multi-state)\n",
            self.review_bindings.len(),
            narrowed,
            multi_state
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Review bindings\n\n");
        for binding in &self.review_bindings {
            let co_states = if binding.co_applicable_states.is_empty() {
                String::new()
            } else {
                format!(
                    " (+ {})",
                    binding
                        .co_applicable_states
                        .iter()
                        .map(|state| state.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}`{}, fallback `{}` via `{}`, posture `{}`, recovery `{}`\n",
                binding.object_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                co_states,
                binding.fallback_action.as_str(),
                binding.originating_flow.as_str(),
                binding.posture.as_str(),
                binding.review_content.checkpoint_undo_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in write-review-sheet fallback-path export.
#[derive(Debug)]
pub enum M5WriteReviewSheetFallbackPathsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WriteReviewSheetFallbackPathsViolation>),
}

impl fmt::Display for M5WriteReviewSheetFallbackPathsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "write-review-sheet fallback-path export parse failed: {error}"
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
                    "write-review-sheet fallback-path export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WriteReviewSheetFallbackPathsArtifactError {}

/// Validation failures emitted by [`M5WriteReviewSheetFallbackPathsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WriteReviewSheetFallbackPathsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No review bindings are present.
    ReviewBindingsMissing,
    /// A review binding is incomplete.
    BindingIncomplete,
    /// A binding's review content values are incomplete.
    ContentFacetIncomplete,
    /// A binding dropped its write-constrained disposition.
    WriteDispositionMissingForConstrainedObject,
    /// A binding's write disposition does not match its fallback action.
    WriteDispositionActionMismatch,
    /// A binding's checkpoint / undo class does not match its fallback action.
    CheckpointActionMismatch,
    /// A binding's parity state does not match its posture.
    ParityStateMismatch,
    /// Two flows review the same profile with different reviewed-transition content.
    ReviewContentDriftAcrossFlows,
    /// A fallback action is reviewed through fewer than two distinct flows.
    FallbackActionReuseUnproven,
    /// An export binding does not point at the canonical contracts.
    ExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-content note.
    NarrowNotePreservedContentMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-review-sheet binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding is missing the safe base action set.
    SafeBaseActionsMissing,
    /// A binding's reviewed-commit action does not match its posture.
    CommitActionPostureMismatch,
    /// A binding is not reviewed before commit.
    NotReviewedBeforeCommit,
    /// A binding does not make the recovery / undo class visible before commit.
    RecoveryNotVisibleBeforeCommit,
    /// A multi-state binding hides a co-applicable state facet.
    MultiStateFacetHidden,
    /// A binding cannot discover its state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding silently mutates the current object through a lossy fallback.
    SilentlyMutatesCurrentObjectThroughLossyFallback,
    /// A binding gives an AI / automation / import / repair flow a hidden bypass.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A binding leaves the exact write target or preserved-versus-lost sync unstated.
    LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated,
    /// A binding hides the recovery / undo class before commit.
    HidesRecoveryOrUndoClassBeforeCommit,
    /// A binding lets one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// Not every originating flow appears among the bindings.
    OriginatingFlowCoverageMissing,
    /// Not every fallback action appears among the bindings.
    FallbackActionCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No originating flows are present.
    OriginatingFlowsMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Flow projection does not satisfy required invariants.
    FlowProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5WriteReviewSheetFallbackPathsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ReviewBindingsMissing => "review_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ContentFacetIncomplete => "content_facet_incomplete",
            Self::WriteDispositionMissingForConstrainedObject => {
                "write_disposition_missing_for_constrained_object"
            }
            Self::WriteDispositionActionMismatch => "write_disposition_action_mismatch",
            Self::CheckpointActionMismatch => "checkpoint_action_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ReviewContentDriftAcrossFlows => "review_content_drift_across_flows",
            Self::FallbackActionReuseUnproven => "fallback_action_reuse_unproven",
            Self::ExportReferenceMissing => "export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedContentMissing => "narrow_note_preserved_content_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::SafeBaseActionsMissing => "safe_base_actions_missing",
            Self::CommitActionPostureMismatch => "commit_action_posture_mismatch",
            Self::NotReviewedBeforeCommit => "not_reviewed_before_commit",
            Self::RecoveryNotVisibleBeforeCommit => "recovery_not_visible_before_commit",
            Self::MultiStateFacetHidden => "multi_state_facet_hidden",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::SilentlyMutatesCurrentObjectThroughLossyFallback => {
                "silently_mutates_current_object_through_lossy_fallback"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated => {
                "leaves_exact_write_target_or_preserved_versus_lost_sync_unstated"
            }
            Self::HidesRecoveryOrUndoClassBeforeCommit => {
                "hides_recovery_or_undo_class_before_commit"
            }
            Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior => {
                "lets_one_state_class_hide_another_when_both_materially_affect_behavior"
            }
            Self::OriginatingFlowCoverageMissing => "originating_flow_coverage_missing",
            Self::FallbackActionCoverageMissing => "fallback_action_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::OriginatingFlowsMissing => "originating_flows_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::FlowProjectionIncomplete => "flow_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable write-review-sheet fallback-path export.
pub fn current_stable_m5_write_review_sheet_fallback_paths_export(
) -> Result<M5WriteReviewSheetFallbackPathsPacket, M5WriteReviewSheetFallbackPathsArtifactError> {
    let packet: M5WriteReviewSheetFallbackPathsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-write-review-sheet-fallback-paths/support_export.json"
        )))
        .map_err(M5WriteReviewSheetFallbackPathsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WriteReviewSheetFallbackPathsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5WriteReviewSheetFallbackPathsPacket,
    violations: &mut Vec<M5WriteReviewSheetFallbackPathsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_REF,
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    ];
    // The six object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5WriteReviewSheetFallbackPathsPacket,
    violations: &mut Vec<M5WriteReviewSheetFallbackPathsViolation>,
) {
    if packet.review_bindings.is_empty() {
        violations.push(M5WriteReviewSheetFallbackPathsViolation::ReviewBindingsMissing);
        return;
    }

    // One vocabulary: the reviewed-transition content must be identical for every binding that reviews the same
    // constrained-object profile.
    let mut profile_content: BTreeMap<&str, &WriteReviewSheetContent> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each fallback action must be reviewed through at least two distinct originating flows.
    let mut action_flows: BTreeMap<
        WriteReviewFallbackAction,
        BTreeSet<WriteReviewOriginatingFlow>,
    > = BTreeMap::new();
    let mut seen_flows: BTreeSet<WriteReviewOriginatingFlow> = BTreeSet::new();
    let mut seen_actions: BTreeSet<WriteReviewFallbackAction> = BTreeSet::new();

    for binding in &packet.review_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.object_profile_id.trim().is_empty()
            || binding.object_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::BindingIncomplete);
        }
        if !binding.review_content.all_present() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::ContentFacetIncomplete);
        }
        if !binding.review_content.write_disposition_satisfied() {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::WriteDispositionMissingForConstrainedObject,
            );
        }
        if !binding.write_disposition_matches_action() {
            violations
                .push(M5WriteReviewSheetFallbackPathsViolation::WriteDispositionActionMismatch);
        }
        if !binding.checkpoint_matches_action() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::CheckpointActionMismatch);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5WriteReviewSheetFallbackPathsViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5WriteReviewSheetFallbackPathsViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5WriteReviewSheetFallbackPathsViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_content_note.trim().is_empty() {
                        violations.push(
                            M5WriteReviewSheetFallbackPathsViolation::NarrowNotePreservedContentMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5WriteReviewSheetFallbackPathsViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::ExportDetailNoteMissing);
        }

        // Action rules.
        if !binding.has_safe_base_actions() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::SafeBaseActionsMissing);
        }
        if !binding.commit_action_matches_posture() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::CommitActionPostureMismatch);
        }

        // Positive review invariants.
        if !binding.reviewed_before_commit {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::NotReviewedBeforeCommit);
        }
        if !binding.recovery_visible_before_commit {
            violations
                .push(M5WriteReviewSheetFallbackPathsViolation::RecoveryNotVisibleBeforeCommit);
        }

        // Multi-state facets.
        if !binding.multi_state_facets_consistent() {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::MultiStateFacetHidden);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations
                .push(M5WriteReviewSheetFallbackPathsViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants (each must be false).
        if binding.silently_mutates_current_object_through_lossy_fallback {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::SilentlyMutatesCurrentObjectThroughLossyFallback,
            );
        }
        if binding.gives_ai_automation_import_or_repair_flows_a_hidden_bypass {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
            );
        }
        if binding.leaves_exact_write_target_or_preserved_versus_lost_sync_unstated {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::LeavesExactWriteTargetOrPreservedVersusLostSyncUnstated,
            );
        }
        if binding.hides_recovery_or_undo_class_before_commit {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::HidesRecoveryOrUndoClassBeforeCommit,
            );
        }
        if binding.lets_one_state_class_hide_another_when_both_materially_affect_behavior {
            violations.push(
                M5WriteReviewSheetFallbackPathsViolation::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
            );
        }

        // Export views must map a profile back to canonical contracts.
        if posture_must_reference_canonical(binding.posture)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::ExportReferenceMissing);
        }

        // Content-drift accumulation.
        match profile_content.get(binding.object_profile_id.as_str()) {
            None => {
                profile_content.insert(binding.object_profile_id.as_str(), &binding.review_content);
            }
            Some(existing) => {
                if **existing != binding.review_content && !drift_reported {
                    violations.push(
                        M5WriteReviewSheetFallbackPathsViolation::ReviewContentDriftAcrossFlows,
                    );
                    drift_reported = true;
                }
            }
        }

        action_flows
            .entry(binding.fallback_action)
            .or_default()
            .insert(binding.originating_flow);
        seen_flows.insert(binding.originating_flow);
        seen_actions.insert(binding.fallback_action);
    }

    // Coverage: every originating flow and every fallback action must appear.
    for flow in WriteReviewOriginatingFlow::ALL {
        if !seen_flows.contains(&flow) {
            violations
                .push(M5WriteReviewSheetFallbackPathsViolation::OriginatingFlowCoverageMissing);
            break;
        }
    }
    for action in WriteReviewFallbackAction::ALL {
        if !seen_actions.contains(&action) {
            violations
                .push(M5WriteReviewSheetFallbackPathsViolation::FallbackActionCoverageMissing);
            break;
        }
    }

    // Reuse: every present fallback action must be reviewed through two or more distinct flows.
    for flows in action_flows.values() {
        if flows.len() < 2 {
            violations.push(M5WriteReviewSheetFallbackPathsViolation::FallbackActionReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
