//! Cross-actor constrained-write enforcement: one shared gate and safe-next-step resolver that every
//! mutation-capable actor — a direct edit / save, an AI apply, an automation recipe, an importer, a repair, and a
//! code action — is routed through, so each actor inherits the *same* state-class block and the *same*
//! safe-next-step guidance instead of an actor-specific, free-form best-effort write.
//!
//! This module is the B150 actor-parity mutation-gate lane over the six constrained-current-object classes frozen
//! in [`crate::m5_constrained_file_state_matrix`]. Where the write-review-sheet lane
//! ([`crate::m5_write_review_sheet_fallback_paths`]) proves *what a reviewed transition looks like before commit*,
//! this lane proves *that no actor can reach a constrained object except through the shared gate*: every actor
//! that could mutate a current object resolves to a [`BlockedWriteReason`] keyed to the object's state class (not
//! to the actor), the exact write target and canonical source are named, a safe next step
//! ([`WriteReviewFallbackAction`]) is offered before any write, and the gate fails closed when the actor context
//! drifts or the flow cannot explain the exact write target truthfully.
//!
//! The blocked-write reason vocabulary is a function of the constrained-object class alone, so an AI apply, a
//! repair, an importer, and a direct save that all land on the same constrained object hit the *same* structured
//! reason and the *same* safe next step. A mutation-capable actor that bypasses direct typing (AI, automation,
//! import, repair, code action) can never silently write a generated, managed, projection, or captured-snapshot
//! object just because it did not go through the editor: there is no direct-write action to represent, and the
//! only write-adjacent action opens the reviewed transition. Support / export traces preserve the actor, the
//! blocked reason, and the chosen fallback path, so an operator can see which actor was routed where and why.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **One blocked-write vocabulary across actors.** At least one constrained object is routed through the gate
//!    by an AI path, a repair path, an importer path, and a direct edit / save path, and all four resolve to the
//!    same [`BlockedWriteReason`] and the same safe next step — the reason is keyed to the state class, never to
//!    the actor.
//! 2. **No bypass write.** A mutation-capable actor cannot silently write a generated, managed, projection, or
//!    captured-snapshot object just because it bypasses direct typing; every actor is routed through the shared
//!    gate, no direct-write action can be represented, and no binding may give an AI / automation / import /
//!    repair flow a hidden bypass, use an actor-specific free-form reason instead of the state-class vocabulary,
//!    leave the exact write target or canonical source unstated, or let one state class hide another.
//! 3. **Fail closed and traceable.** The gate fails closed when the actor context drifts or a flow cannot explain
//!    the exact write target truthfully ([`FailClosedReason`]), offering no write path until the context is
//!    resolved, and every binding carries an [`ActorGateTrace`] preserving the actor, the blocked reason, and the
//!    chosen fallback path, discoverable through the keyboard and screen-reader routes
//!    ([`crate::m5_constrained_file_state_matrix::M5ConstrainedFileStateAccessibilityRoute`]).
//!
//! Narrowing is disclosed, never hidden: a fail-closed gate or an exported, export-safe view carries an explicit
//! [`GateNarrowNote`] naming the reason, the preserved gate content, and the next action, so a surface may narrow
//! *which* actions remain without ever rewording the state-class reason or quietly implying the object is
//! directly writable.
//!
//! The packet references upstream constrained-file-state contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/program/m5-cross-actor-constrained-write-enforcement.schema.json`](../../../../schemas/program/m5-cross-actor-constrained-write-enforcement.schema.json).
//! The contract doc is
//! [`docs/support/m5_cross_actor_constrained_write_enforcement.md`](../../../../docs/support/m5_cross_actor_constrained_write_enforcement.md).
//! The protected fixture directory is
//! [`fixtures/editor/m5-cross-actor-constrained-write-enforcement/`](../../../../fixtures/editor/m5-cross-actor-constrained-write-enforcement/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_cross_actor_constrained_write_enforcement,
    seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed,
    seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed,
};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateObject,
    M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF, M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};
// The safe-next-step and recovery-class vocabularies are reused from the write-review-sheet lane so the gate's
// safe next step is the exact reviewed transition an operator commits to, not a parallel restatement.
pub use crate::m5_write_review_sheet_fallback_paths::{
    CheckpointUndoClass, WriteReviewFallbackAction,
};

/// Stable record-kind tag carried by [`M5CrossActorConstrainedWriteEnforcementPacket`].
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_RECORD_KIND: &str =
    "m5_cross_actor_constrained_write_enforcement_registry";

/// Schema version for cross-actor constrained-write enforcement records.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PACKET_ID: &str =
    "m5-cross-actor-constrained-write-enforcement:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_REF: &str =
    "schemas/program/m5-cross-actor-constrained-write-enforcement.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_DOC_REF: &str =
    "docs/support/m5_cross_actor_constrained_write_enforcement.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_ARTIFACT_REF: &str =
    "artifacts/support/m5-cross-actor-constrained-write-enforcement/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_CSV_REF: &str =
    "artifacts/support/m5-cross-actor-constrained-write-enforcement/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_REPORT_REF: &str =
    "artifacts/support/m5-cross-actor-constrained-write-enforcement/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_FIXTURE_DIR: &str =
    "fixtures/editor/m5-cross-actor-constrained-write-enforcement";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PROOF_SLO_HOURS: u32 = 720;

/// Write-disposition sentinel words the gate may never resolve to; a routed actor must always keep a real
/// write-constrained disposition rather than implying the object is directly writable, editable, or unconstrained.
const WRITE_DISPOSITION_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "directly_writable",
    "writable",
    "editable",
    "unconstrained",
];

/// One of the mutation-capable actors routed through the shared constrained-write gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationActor {
    /// A direct editor edit / save — the one actor that goes through direct typing.
    DirectEditSave,
    /// An AI apply / assisted edit that writes without direct typing.
    AiApply,
    /// An automation recipe that writes without direct typing.
    AutomationRecipe,
    /// An importer bringing content in from outside without direct typing.
    Importer,
    /// A repair / doctor flow that writes without direct typing.
    Repair,
    /// A code action / quick fix that writes without direct typing.
    CodeAction,
}

impl MutationActor {
    /// Every actor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DirectEditSave,
        Self::AiApply,
        Self::AutomationRecipe,
        Self::Importer,
        Self::Repair,
        Self::CodeAction,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectEditSave => "direct_edit_save",
            Self::AiApply => "ai_apply",
            Self::AutomationRecipe => "automation_recipe",
            Self::Importer => "importer",
            Self::Repair => "repair",
            Self::CodeAction => "code_action",
        }
    }

    /// Whether this actor writes without going through direct typing. Every actor except the direct edit / save
    /// path bypasses direct typing, so it must be routed through the shared gate exactly like a direct write.
    pub const fn bypasses_direct_typing(self) -> bool {
        !matches!(self, Self::DirectEditSave)
    }

    /// Whether this actor is one of the AI / automation / import / repair flows the guardrails call out for a
    /// hidden bypass.
    pub const fn is_ai_automation_import_or_repair(self) -> bool {
        matches!(
            self,
            Self::AiApply | Self::AutomationRecipe | Self::Importer | Self::Repair
        )
    }
}

/// The structured blocked-write reason for a constrained current object, keyed to its state class and never to the
/// actor that tried to write it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedWriteReason {
    /// A read-only current object cannot be written in place; duplicate to an editable copy.
    ReadOnlyPathNotDirectlyWritable,
    /// A generated / derived artifact's truth lives in its generator; regenerate from source.
    GeneratedArtifactRegenerateOnly,
    /// A policy-locked object's write is gated behind an approval; request approval.
    PolicyLockRequiresApproval,
    /// A managed, externally-owned object is owned upstream; detach from the managed source to edit locally.
    ManagedSourceRequiresDetach,
    /// A projection / virtual view resolves back to a backing source; record edits as an overlay patch.
    ProjectionRequiresOverlayOrDetach,
    /// A captured snapshot preserves a past state and is not the live object; duplicate it into an editable copy.
    CapturedSnapshotRestoreOnly,
}

impl BlockedWriteReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnlyPathNotDirectlyWritable,
        Self::GeneratedArtifactRegenerateOnly,
        Self::PolicyLockRequiresApproval,
        Self::ManagedSourceRequiresDetach,
        Self::ProjectionRequiresOverlayOrDetach,
        Self::CapturedSnapshotRestoreOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPathNotDirectlyWritable => "read_only_path_not_directly_writable",
            Self::GeneratedArtifactRegenerateOnly => "generated_artifact_regenerate_only",
            Self::PolicyLockRequiresApproval => "policy_lock_requires_approval",
            Self::ManagedSourceRequiresDetach => "managed_source_requires_detach",
            Self::ProjectionRequiresOverlayOrDetach => "projection_requires_overlay_or_detach",
            Self::CapturedSnapshotRestoreOnly => "captured_snapshot_restore_only",
        }
    }

    /// The blocked-write reason for a constrained-object class — a pure function of the state class, proving the
    /// reason vocabulary is actor-independent.
    pub const fn for_object_class(object_class: M5ConstrainedFileStateObject) -> Self {
        match object_class {
            M5ConstrainedFileStateObject::ReadOnly => Self::ReadOnlyPathNotDirectlyWritable,
            M5ConstrainedFileStateObject::Generated => Self::GeneratedArtifactRegenerateOnly,
            M5ConstrainedFileStateObject::PolicyLocked => Self::PolicyLockRequiresApproval,
            M5ConstrainedFileStateObject::Managed => Self::ManagedSourceRequiresDetach,
            M5ConstrainedFileStateObject::Projection => Self::ProjectionRequiresOverlayOrDetach,
            M5ConstrainedFileStateObject::CapturedSnapshot => Self::CapturedSnapshotRestoreOnly,
        }
    }

    /// The constrained-object class this reason classifies.
    pub const fn object_class(self) -> M5ConstrainedFileStateObject {
        match self {
            Self::ReadOnlyPathNotDirectlyWritable => M5ConstrainedFileStateObject::ReadOnly,
            Self::GeneratedArtifactRegenerateOnly => M5ConstrainedFileStateObject::Generated,
            Self::PolicyLockRequiresApproval => M5ConstrainedFileStateObject::PolicyLocked,
            Self::ManagedSourceRequiresDetach => M5ConstrainedFileStateObject::Managed,
            Self::ProjectionRequiresOverlayOrDetach => M5ConstrainedFileStateObject::Projection,
            Self::CapturedSnapshotRestoreOnly => M5ConstrainedFileStateObject::CapturedSnapshot,
        }
    }

    /// The safe next step the gate offers for this reason — the reviewed fallback transition an operator commits
    /// to, shared with [`crate::m5_write_review_sheet_fallback_paths`].
    pub const fn safe_next_step(self) -> WriteReviewFallbackAction {
        match self {
            Self::ReadOnlyPathNotDirectlyWritable | Self::CapturedSnapshotRestoreOnly => {
                WriteReviewFallbackAction::DuplicateToEditableCopy
            }
            Self::GeneratedArtifactRegenerateOnly => {
                WriteReviewFallbackAction::RegenerateWithPreview
            }
            Self::PolicyLockRequiresApproval => WriteReviewFallbackAction::RequestApproval,
            Self::ManagedSourceRequiresDetach => WriteReviewFallbackAction::DetachFromManagedSource,
            Self::ProjectionRequiresOverlayOrDetach => {
                WriteReviewFallbackAction::CreateOverlayPatch
            }
        }
    }
}

/// The enforcement posture the shared gate takes on one actor / object binding.
///
/// The posture governs the discoverable action set and narrowing disclosure, never the state-class reason: a
/// narrowed posture still carries the same blocked reason, exact write target, canonical source, and safe next
/// step, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateEnforcementPosture {
    /// The gate is enforced and resolves the blocked reason plus the safe next step; the actor is routed to open
    /// the reviewed transition.
    EnforcedGate,
    /// The gate fails closed because the actor context drifted or the exact write target could not be explained
    /// truthfully; no write path is offered until the context is resolved.
    FailClosedOnActorDrift,
    /// An exported, export-safe-redacted rendering of the gate resolution in a support packet.
    ExportRedacted,
}

impl GateEnforcementPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::EnforcedGate,
        Self::FailClosedOnActorDrift,
        Self::ExportRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnforcedGate => "enforced_gate",
            Self::FailClosedOnActorDrift => "fail_closed_on_actor_drift",
            Self::ExportRedacted => "export_redacted",
        }
    }

    /// Whether this posture narrows below the enforced-gate disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::EnforcedGate)
    }
}

/// Why the shared gate failed closed rather than offering a write path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailClosedReason {
    /// The actor context drifted (identity, scope, or authority could not be confirmed), so the gate blocked.
    ActorContextDrifted,
    /// The flow could not explain the exact write target truthfully, so the gate blocked.
    ExactWriteTargetNotTruthfullyExplainable,
}

impl FailClosedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorContextDrifted => "actor_context_drifted",
            Self::ExactWriteTargetNotTruthfullyExplainable => {
                "exact_write_target_not_truthfully_explainable"
            }
        }
    }
}

/// A discoverable action the shared gate may expose.
///
/// The set is deliberately closed and safe: there is no direct-write action variant, so the gate can never
/// present a control that performs a silent lossy direct write for any actor. The only write-adjacent action opens
/// the reviewed transition, present only where the gate is enforced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateAction {
    /// Inspect the structured blocked-write reason keyed to the object's state class.
    InspectBlockedWriteReason,
    /// Reveal the canonical source and the exact write target.
    RevealCanonicalSourceAndWriteTarget,
    /// Copy the safe-next-step summary.
    CopySafeNextStep,
    /// Open the reviewed transition for the safe next step — only where the gate is enforced, never a direct
    /// write.
    OpenSafeNextStepReview,
}

impl GateAction {
    /// The safe base action set present on every gate binding.
    pub const SAFE_BASE: [Self; 3] = [
        Self::InspectBlockedWriteReason,
        Self::RevealCanonicalSourceAndWriteTarget,
        Self::CopySafeNextStep,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectBlockedWriteReason => "inspect_blocked_write_reason",
            Self::RevealCanonicalSourceAndWriteTarget => "reveal_canonical_source_and_write_target",
            Self::CopySafeNextStep => "copy_safe_next_step",
            Self::OpenSafeNextStepReview => "open_safe_next_step_review",
        }
    }
}

/// Why the gate narrowed its action set below an enforced-gate view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateNarrowReason {
    /// The gate failed closed on actor-context drift; only inspect / reveal / copy remain.
    FailedClosedOnActorContextDrift,
    /// An exported view redacted its surrounding detail export-safe.
    ExportRedactionNarrowed,
}

impl GateNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailedClosedOnActorContextDrift => "failed_closed_on_actor_context_drift",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateNarrowNextAction {
    /// Resolve the actor context (re-confirm identity, scope, or authority) then retry the routed write.
    ResolveActorContextThenRetry,
    /// Open the full gate detail behind the redacted export.
    OpenFullGateDetail,
}

impl GateNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveActorContextThenRetry => "resolve_actor_context_then_retry",
            Self::OpenFullGateDetail => "open_full_gate_detail",
        }
    }
}

/// Whether a binding preserves the full enforced-gate view or discloses a narrowed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateParityState {
    /// The state-class reason and full action set are preserved and shown.
    ContentPreserved,
    /// The state-class reason is preserved and a narrowed action set is explicitly disclosed.
    ContentDisclosedNarrowed,
}

impl GateParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentPreserved => "content_preserved",
            Self::ContentDisclosedNarrowed => "content_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this cross-actor enforcement lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CrossActorConstrainedWriteEnforcementDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The blocked reason drifted between the actors that reached the same object.
    BlockedReasonDriftAcrossActors,
    /// A routed actor dropped its write-constrained disposition and began to imply the object is directly
    /// writable.
    WriteDispositionDroppedForConstrainedObject,
    /// A mutation-capable actor silently wrote a constrained object by bypassing direct typing.
    ActorSilentlyWritesConstrainedObjectBypassingDirectTyping,
    /// An AI / automation / import / repair flow got a hidden bypass around the shared gate.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A binding used an actor-specific free-form reason instead of the state-class vocabulary.
    UsesActorSpecificFreeFormBlockedReason,
    /// A binding left the exact write target or canonical source unstated.
    LeavesExactWriteTargetOrCanonicalSourceUnstated,
    /// The gate stopped failing closed when the actor context drifted.
    StopsFailingClosedOnActorContextDrift,
    /// A binding let one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// An accessibility route for the reason, target, or safe next step was dropped.
    AccessibilityRouteDropped,
    /// An export view lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream constrained-file-state contract narrowed.
    UpstreamConstrainedFileStateNarrowed,
}

impl M5CrossActorConstrainedWriteEnforcementDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::BlockedReasonDriftAcrossActors,
        Self::WriteDispositionDroppedForConstrainedObject,
        Self::ActorSilentlyWritesConstrainedObjectBypassingDirectTyping,
        Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
        Self::UsesActorSpecificFreeFormBlockedReason,
        Self::LeavesExactWriteTargetOrCanonicalSourceUnstated,
        Self::StopsFailingClosedOnActorContextDrift,
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
            Self::BlockedReasonDriftAcrossActors => "blocked_reason_drift_across_actors",
            Self::WriteDispositionDroppedForConstrainedObject => {
                "write_disposition_dropped_for_constrained_object"
            }
            Self::ActorSilentlyWritesConstrainedObjectBypassingDirectTyping => {
                "actor_silently_writes_constrained_object_bypassing_direct_typing"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::UsesActorSpecificFreeFormBlockedReason => {
                "uses_actor_specific_free_form_blocked_reason"
            }
            Self::LeavesExactWriteTargetOrCanonicalSourceUnstated => {
                "leaves_exact_write_target_or_canonical_source_unstated"
            }
            Self::StopsFailingClosedOnActorContextDrift => {
                "stops_failing_closed_on_actor_context_drift"
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

/// The gate resolution presented for a constrained-object profile.
///
/// These values are a pure function of the object's state class and must be identical across every actor routed
/// against the same profile. An actor may narrow which actions remain, but it may never reword any of these values
/// per actor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateResolution {
    /// The structured blocked-write reason keyed to the object's state class.
    pub blocked_write_reason: BlockedWriteReason,
    /// The write-constrained disposition token that keeps the object mechanically distinct from a
    /// directly-writable object.
    pub write_disposition_word: String,
    /// The safe next step the gate offers before any write.
    pub safe_next_step: WriteReviewFallbackAction,
    /// The recovery / undo class the safe next step makes visible.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// The exact write target a routed write would actually touch (never empty).
    pub exact_write_target_word: String,
    /// The canonical source the constrained object relates back to (never empty).
    pub canonical_source_word: String,
    /// The export-safe explanation of the blocked-write reason and the safe next step.
    pub structured_reason_explanation: String,
    /// The controlled labels for any co-applicable state classes (empty for a single-state object); when an
    /// object is multi-state both facts stay visible here rather than one state hiding another.
    pub co_applicable_state_labels: Vec<String>,
}

impl GateResolution {
    /// Whether every scalar resolution field is present.
    pub fn all_present(&self) -> bool {
        !self.write_disposition_word.trim().is_empty()
            && !self.exact_write_target_word.trim().is_empty()
            && !self.canonical_source_word.trim().is_empty()
            && !self.structured_reason_explanation.trim().is_empty()
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

    /// Whether the safe next step is the one keyed to the blocked reason's state class.
    pub fn safe_next_step_keyed_to_state_class(&self) -> bool {
        self.safe_next_step == self.blocked_write_reason.safe_next_step()
    }

    /// Whether the write disposition matches the safe next step's required write-constrained disposition.
    pub fn write_disposition_matches_safe_next_step(&self) -> bool {
        self.write_disposition_word.trim()
            == self.safe_next_step.required_write_disposition().as_str()
    }

    /// Whether the checkpoint / undo class matches the safe next step's required recovery class.
    pub fn checkpoint_matches_safe_next_step(&self) -> bool {
        self.checkpoint_undo_class == self.safe_next_step.required_checkpoint_undo_class()
    }
}

/// The support / export trace preserved for one routed actor: the actor, the blocked reason, and the chosen
/// fallback path, so an operator can see which actor was routed where and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorGateTrace {
    /// The actor that was routed through the gate.
    pub actor: MutationActor,
    /// The blocked-write reason the actor hit — keyed to the object's state class.
    pub blocked_write_reason: BlockedWriteReason,
    /// The chosen fallback path (the safe next step) the actor was offered.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The enforcement posture at which the actor was routed.
    pub gate_posture: GateEnforcementPosture,
}

/// The explicit note a narrowed posture shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateNarrowNote {
    /// Why the posture narrowed.
    pub reason: GateNarrowReason,
    /// Note naming the preserved gate content (never omitted).
    pub preserved_content_note: String,
    /// The next action offered.
    pub next_action: GateNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a binding must carry, derived from its posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateRenderDisclosure {
    /// The parity state the posture requires.
    pub parity_state: GateParityState,
    /// The narrow reason the posture requires, if any.
    pub narrow_reason: Option<GateNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<GateNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
    /// Whether the binding offers the open-safe-next-step action.
    pub offers_open_safe_next_step: bool,
    /// Whether the binding is a fail-closed rendering that must name a fail-closed reason.
    pub is_fail_closed: bool,
}

/// Resolves the render disclosures a binding must carry from its posture.
///
/// The enforced-gate posture renders the full safe action set plus the open-safe-next-step action. A fail-closed
/// gate and an exported view each narrow the action set and disclose the narrowing through an explicit note — but
/// both keep every gate-resolution value. Only the fail-closed posture names a fail-closed reason and offers no
/// write path.
pub const fn resolve_gate_render_disclosure(
    posture: GateEnforcementPosture,
) -> GateRenderDisclosure {
    match posture {
        GateEnforcementPosture::EnforcedGate => GateRenderDisclosure {
            parity_state: GateParityState::ContentPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_export_detail_note: false,
            offers_open_safe_next_step: true,
            is_fail_closed: false,
        },
        GateEnforcementPosture::FailClosedOnActorDrift => GateRenderDisclosure {
            parity_state: GateParityState::ContentDisclosedNarrowed,
            narrow_reason: Some(GateNarrowReason::FailedClosedOnActorContextDrift),
            narrow_next_action: Some(GateNarrowNextAction::ResolveActorContextThenRetry),
            needs_narrow_note: true,
            needs_export_detail_note: false,
            offers_open_safe_next_step: false,
            is_fail_closed: true,
        },
        GateEnforcementPosture::ExportRedacted => GateRenderDisclosure {
            parity_state: GateParityState::ContentDisclosedNarrowed,
            narrow_reason: Some(GateNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(GateNarrowNextAction::OpenFullGateDetail),
            needs_narrow_note: true,
            needs_export_detail_note: true,
            offers_open_safe_next_step: false,
            is_fail_closed: false,
        },
    }
}

/// Whether a posture is the export / support rendering that must map a profile back to canonical contracts by id.
pub const fn posture_must_reference_canonical(posture: GateEnforcementPosture) -> bool {
    matches!(posture, GateEnforcementPosture::ExportRedacted)
}

/// One gate binding: a constrained-object profile routed through the shared gate by one actor at one posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossActorGateBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable constrained-object-profile id (shared across actors routed against the same profile).
    pub object_profile_id: String,
    /// Human-readable constrained-object-profile identity.
    pub object_profile_label: String,
    /// Which constrained-object class this binding gates as its primary state.
    pub object_class: M5ConstrainedFileStateObject,
    /// Any co-applicable state classes that also apply (empty for a single-state object); when present, both
    /// facts must stay visible.
    pub co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    /// Which mutation-capable actor's write is routed here.
    pub actor: MutationActor,
    /// Which enforcement posture this rendering takes.
    pub posture: GateEnforcementPosture,
    /// The gate resolution presented (identical across actors for one profile).
    pub resolution: GateResolution,
    /// Whether content is preserved in full or a narrowing is disclosed.
    pub parity_state: GateParityState,
    /// The fail-closed reason; present exactly when the posture fails closed.
    pub fail_closed_reason: Option<FailClosedReason>,
    /// The support / export trace preserving the actor, blocked reason, and chosen fallback path.
    pub trace: ActorGateTrace,
    /// The discoverable action set allowed on this gate view.
    pub allowed_actions: Vec<GateAction>,
    /// The accessibility routes through which the reason, write target, and safe next step can be discovered
    /// without pointer-only chrome.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// The explicit narrow note; required and complete when the posture narrows.
    pub narrow_note: Option<GateNarrowNote>,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Positive invariant: this actor's write is routed through the shared gate. MUST be `true`.
    pub routed_through_shared_gate: bool,
    /// Positive invariant: the safe next step is keyed to the state class, not the actor. MUST be `true`.
    pub safe_next_step_keyed_to_state_class: bool,
    /// Guardrail: this actor silently writes the constrained object by bypassing direct typing. MUST be `false`.
    pub silently_writes_constrained_object_bypassing_direct_typing: bool,
    /// Guardrail: this binding gives an AI / automation / import / repair flow a hidden bypass around the shared
    /// gate. MUST be `false`.
    pub gives_ai_automation_import_or_repair_flows_a_hidden_bypass: bool,
    /// Guardrail: this binding uses an actor-specific free-form reason instead of the state-class vocabulary. MUST
    /// be `false`.
    pub uses_actor_specific_free_form_blocked_reason: bool,
    /// Guardrail: this binding leaves the exact write target or canonical source unstated. MUST be `false`.
    pub leaves_exact_write_target_or_canonical_source_unstated: bool,
    /// Guardrail: this binding lets one state class hide another when both materially affect behavior. MUST be
    /// `false`.
    pub lets_one_state_class_hide_another_when_both_materially_affect_behavior: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl CrossActorGateBinding {
    /// Disclosures this binding must carry, derived from its posture.
    pub const fn disclosure(&self) -> GateRenderDisclosure {
        resolve_gate_render_disclosure(self.posture)
    }

    /// Whether this binding renders below the enforced-gate view.
    pub const fn is_narrowed(&self) -> bool {
        self.posture.is_narrowed()
    }

    /// Whether this binding gates a multi-state (more than one co-applicable constraint) object.
    pub fn is_multi_state(&self) -> bool {
        !self.co_applicable_states.is_empty()
    }

    /// Whether both positive gate invariants hold, as required.
    pub const fn positive_invariants_hold(&self) -> bool {
        self.routed_through_shared_gate && self.safe_next_step_keyed_to_state_class
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.silently_writes_constrained_object_bypassing_direct_typing
            && !self.gives_ai_automation_import_or_repair_flows_a_hidden_bypass
            && !self.uses_actor_specific_free_form_blocked_reason
            && !self.leaves_exact_write_target_or_canonical_source_unstated
            && !self.lets_one_state_class_hide_another_when_both_materially_affect_behavior
    }

    /// Whether the safe base action set is present.
    pub fn has_safe_base_actions(&self) -> bool {
        GateAction::SAFE_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether the open-safe-next-step action is present exactly when the posture offers it.
    pub fn open_action_matches_posture(&self) -> bool {
        let offered = self.disclosure().offers_open_safe_next_step;
        let present = self
            .allowed_actions
            .contains(&GateAction::OpenSafeNextStepReview);
        offered == present
    }

    /// Whether the fail-closed reason is present exactly when the posture fails closed.
    pub fn fail_closed_reason_matches_posture(&self) -> bool {
        self.disclosure().is_fail_closed == self.fail_closed_reason.is_some()
    }

    /// Whether the resolution's blocked reason and safe next step classify this binding's object class.
    pub fn resolution_matches_object_class(&self) -> bool {
        self.resolution.blocked_write_reason.object_class() == self.object_class
            && self.resolution.safe_next_step_keyed_to_state_class()
    }

    /// Whether the trace preserves this binding's actor, blocked reason, and chosen fallback path.
    pub fn trace_consistent(&self) -> bool {
        self.trace.actor == self.actor
            && self.trace.blocked_write_reason == self.resolution.blocked_write_reason
            && self.trace.chosen_fallback_path == self.resolution.safe_next_step
            && self.trace.gate_posture == self.posture
    }

    /// Whether the multi-state facets stay consistent: the binding's co-applicable state classes, the resolution
    /// labels, and the requirement that every co-state is distinct from the primary object class all hold, so no
    /// co-applicable state is hidden.
    pub fn multi_state_facets_consistent(&self) -> bool {
        if self.co_applicable_states.len() != self.resolution.co_applicable_state_labels.len() {
            return false;
        }
        if !self.resolution.co_applicable_labels_present() {
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
pub struct M5CrossActorConstrainedWriteEnforcementTrustReview {
    /// Gate reuse across actors is proven by fixtures rather than inferred from screenshots.
    pub gate_reuse_proven_by_fixtures: bool,
    /// The same object presents the same blocked reason across actors.
    pub same_object_same_blocked_reason_across_actors: bool,
    /// The blocked reason is keyed to the state class and never an actor-specific free-form string.
    pub blocked_reason_keyed_to_state_class_never_actor_free_form: bool,
    /// No bypass actor silently writes a generated, managed, projection, or captured-snapshot object.
    pub no_bypass_actor_silently_writes_generated_managed_projection_or_archived: bool,
    /// AI / automation / import / repair flows never get a hidden bypass around the shared gate.
    pub no_hidden_bypass_for_ai_automation_import_repair: bool,
    /// The exact write target and canonical source are always stated.
    pub exact_write_target_and_canonical_source_always_stated: bool,
    /// A safe next step is offered before any write on every enforced path.
    pub safe_next_step_offered_before_any_write: bool,
    /// The gate fails closed when the actor context drifts or the write target cannot be explained truthfully.
    pub gate_fails_closed_on_actor_context_drift: bool,
    /// Support traces preserve the actor, blocked reason, and chosen fallback path.
    pub support_trace_preserves_actor_reason_and_fallback: bool,
    /// A multi-state object always keeps every co-applicable state visible.
    pub multi_state_objects_keep_every_state_visible: bool,
    /// Accessibility routes for the reason, target, and safe next step are present.
    pub accessibility_routes_present_for_reason_target_and_safe_next_step: bool,
    /// Narrowing is disclosed across enforced, fail-closed, and exported postures.
    pub narrowing_disclosed_across_postures: bool,
    /// Export views point at the canonical contracts.
    pub export_views_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the gate.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl M5CrossActorConstrainedWriteEnforcementTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.gate_reuse_proven_by_fixtures
            && self.same_object_same_blocked_reason_across_actors
            && self.blocked_reason_keyed_to_state_class_never_actor_free_form
            && self.no_bypass_actor_silently_writes_generated_managed_projection_or_archived
            && self.no_hidden_bypass_for_ai_automation_import_repair
            && self.exact_write_target_and_canonical_source_always_stated
            && self.safe_next_step_offered_before_any_write
            && self.gate_fails_closed_on_actor_context_drift
            && self.support_trace_preserves_actor_reason_and_fallback
            && self.multi_state_objects_keep_every_state_visible
            && self.accessibility_routes_present_for_reason_target_and_safe_next_step
            && self.narrowing_disclosed_across_postures
            && self.export_views_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Actor-parity projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrossActorConstrainedWriteEnforcementActorProjection {
    /// The direct-edit / save actor is routed through the gate.
    pub direct_edit_save_routed_through_gate: bool,
    /// The AI-apply actor is routed through the gate.
    pub ai_apply_routed_through_gate: bool,
    /// The automation-recipe actor is routed through the gate.
    pub automation_recipe_routed_through_gate: bool,
    /// The importer actor is routed through the gate.
    pub importer_routed_through_gate: bool,
    /// The repair actor is routed through the gate.
    pub repair_routed_through_gate: bool,
    /// The code-action actor is routed through the gate.
    pub code_action_routed_through_gate: bool,
    /// At least one object is routed by an AI, a repair, an importer, and a direct-save actor to the same reason.
    pub at_least_one_object_hit_by_ai_repair_importer_and_direct_save: bool,
    /// The blocked reason is identical for the same object across actors.
    pub blocked_reason_identical_for_same_object: bool,
    /// The blocked reason is keyed to the state class, never the actor.
    pub blocked_reason_keyed_to_state_class_not_actor: bool,
    /// No mutation-capable actor silently writes a constrained object by bypassing direct typing.
    pub no_bypass_actor_silently_writes_constrained_object: bool,
    /// The gate fails closed on actor-context drift.
    pub gate_fails_closed_on_actor_context_drift: bool,
    /// The gate fails closed when the write target cannot be explained truthfully.
    pub gate_fails_closed_when_write_target_not_truthfully_explainable: bool,
    /// The support trace preserves the actor, blocked reason, and chosen fallback path.
    pub trace_preserves_actor_reason_and_fallback: bool,
    /// Multi-state objects keep both facts visible.
    pub multi_state_objects_keep_both_facts_visible: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a binding back to one constrained-file-state object class.
    pub export_maps_back_to_one_constrained_file_state_object: bool,
}

impl M5CrossActorConstrainedWriteEnforcementActorProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.direct_edit_save_routed_through_gate
            && self.ai_apply_routed_through_gate
            && self.automation_recipe_routed_through_gate
            && self.importer_routed_through_gate
            && self.repair_routed_through_gate
            && self.code_action_routed_through_gate
            && self.at_least_one_object_hit_by_ai_repair_importer_and_direct_save
            && self.blocked_reason_identical_for_same_object
            && self.blocked_reason_keyed_to_state_class_not_actor
            && self.no_bypass_actor_silently_writes_constrained_object
            && self.gate_fails_closed_on_actor_context_drift
            && self.gate_fails_closed_when_write_target_not_truthfully_explainable
            && self.trace_preserves_actor_reason_and_fallback
            && self.multi_state_objects_keep_both_facts_visible
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_constrained_file_state_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrossActorConstrainedWriteEnforcementProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5CrossActorConstrainedWriteEnforcementPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CrossActorConstrainedWriteEnforcementPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Gate bindings.
    pub gate_bindings: Vec<CrossActorGateBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CrossActorConstrainedWriteEnforcementDowngradeTrigger>,
    /// Actors this packet covers.
    pub actors: Vec<MutationActor>,
    /// Trust review block.
    pub trust_review: M5CrossActorConstrainedWriteEnforcementTrustReview,
    /// Actor-parity projection block.
    pub actor_projection: M5CrossActorConstrainedWriteEnforcementActorProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CrossActorConstrainedWriteEnforcementProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe cross-actor constrained-write enforcement packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrossActorConstrainedWriteEnforcementPacket {
    /// Record kind; must equal [`M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Gate bindings.
    pub gate_bindings: Vec<CrossActorGateBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CrossActorConstrainedWriteEnforcementDowngradeTrigger>,
    /// Actors this packet covers.
    pub actors: Vec<MutationActor>,
    /// Trust review block.
    pub trust_review: M5CrossActorConstrainedWriteEnforcementTrustReview,
    /// Actor-parity projection block.
    pub actor_projection: M5CrossActorConstrainedWriteEnforcementActorProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CrossActorConstrainedWriteEnforcementProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CrossActorConstrainedWriteEnforcementPacket {
    /// Builds a cross-actor constrained-write enforcement packet from stable-lane input.
    pub fn new(input: M5CrossActorConstrainedWriteEnforcementPacketInput) -> Self {
        Self {
            record_kind: M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_RECORD_KIND.to_owned(),
            schema_version: M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            gate_bindings: input.gate_bindings,
            downgrade_triggers: input.downgrade_triggers,
            actors: input.actors,
            trust_review: input.trust_review,
            actor_projection: input.actor_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the cross-actor constrained-write enforcement invariants.
    pub fn validate(&self) -> Vec<M5CrossActorConstrainedWriteEnforcementViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_RECORD_KIND {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_VERSION {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::DowngradeTriggersMissing);
        }
        if self.actors.is_empty() {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::ActorsMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::TrustReviewIncomplete);
        }
        if !self.actor_projection.all_hold() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::ActorProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("cross-actor constrained-write enforcement packet serializes"),
        ) {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::RawBoundaryMaterialInExport,
            );
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
            .expect("cross-actor constrained-write enforcement packet serializes")
    }

    /// Deterministic matrix CSV, one row per gate binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,co_applicable_states,actor,blocked_write_reason,safe_next_step,posture,checkpoint_undo_class,parity_state\n",
        );
        for binding in &self.gate_bindings {
            let co_states = binding
                .co_applicable_states
                .iter()
                .map(|state| state.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                co_states,
                binding.actor.as_str(),
                binding.resolution.blocked_write_reason.as_str(),
                binding.resolution.safe_next_step.as_str(),
                binding.posture.as_str(),
                binding.resolution.checkpoint_undo_class.as_str(),
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .gate_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();
        let multi_state = self
            .gate_bindings
            .iter()
            .filter(|binding| binding.is_multi_state())
            .count();

        let mut out = String::new();
        out.push_str("# Cross-Actor Constrained-Write Enforcement: One Gate Across Actors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Gate bindings: {} ({} narrowed, {} multi-state)\n",
            self.gate_bindings.len(),
            narrowed,
            multi_state
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Gate bindings\n\n");
        for binding in &self.gate_bindings {
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
                "- **{}** [`{}`]: object `{}`{}, actor `{}`, reason `{}`, safe next step `{}`, posture `{}`\n",
                binding.object_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                co_states,
                binding.actor.as_str(),
                binding.resolution.blocked_write_reason.as_str(),
                binding.resolution.safe_next_step.as_str(),
                binding.posture.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in cross-actor constrained-write enforcement export.
#[derive(Debug)]
pub enum M5CrossActorConstrainedWriteEnforcementArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CrossActorConstrainedWriteEnforcementViolation>),
}

impl fmt::Display for M5CrossActorConstrainedWriteEnforcementArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "cross-actor constrained-write enforcement export parse failed: {error}"
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
                    "cross-actor constrained-write enforcement export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CrossActorConstrainedWriteEnforcementArtifactError {}

/// Validation failures emitted by [`M5CrossActorConstrainedWriteEnforcementPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CrossActorConstrainedWriteEnforcementViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No gate bindings are present.
    GateBindingsMissing,
    /// A gate binding is incomplete.
    BindingIncomplete,
    /// A binding's gate-resolution values are incomplete.
    ResolutionFacetIncomplete,
    /// A binding dropped its write-constrained disposition.
    WriteDispositionMissingForConstrainedObject,
    /// A binding's write disposition does not match its safe next step.
    WriteDispositionSafeNextStepMismatch,
    /// A binding's checkpoint / undo class does not match its safe next step.
    CheckpointSafeNextStepMismatch,
    /// A binding's resolution does not classify its object class.
    ResolutionObjectClassMismatch,
    /// A binding's parity state does not match its posture.
    ParityStateMismatch,
    /// Two actors reached the same object with a different gate resolution.
    BlockedReasonDriftAcrossActors,
    /// A constrained object is not routed by an AI, a repair, an importer, and a direct-save actor.
    ActorParityUnproven,
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
    /// An enforced-gate binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding is missing the safe base action set.
    SafeBaseActionsMissing,
    /// A binding's open-safe-next-step action does not match its posture.
    OpenActionPostureMismatch,
    /// A binding's fail-closed reason does not match its posture.
    FailClosedReasonPostureMismatch,
    /// A binding's trace does not preserve its actor, reason, and fallback.
    TraceInconsistent,
    /// A binding is not routed through the shared gate.
    NotRoutedThroughSharedGate,
    /// A binding's safe next step is not keyed to the state class.
    SafeNextStepNotKeyedToStateClass,
    /// A multi-state binding hides a co-applicable state facet.
    MultiStateFacetHidden,
    /// A binding cannot discover its state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding lets an actor silently write a constrained object by bypassing direct typing.
    ActorSilentlyWritesConstrainedObjectBypassingDirectTyping,
    /// A binding gives an AI / automation / import / repair flow a hidden bypass.
    GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
    /// A binding uses an actor-specific free-form reason instead of the state-class vocabulary.
    UsesActorSpecificFreeFormBlockedReason,
    /// A binding leaves the exact write target or canonical source unstated.
    LeavesExactWriteTargetOrCanonicalSourceUnstated,
    /// A binding lets one state class hide another when both materially affect behavior.
    LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
    /// Not every actor appears among the bindings.
    ActorCoverageMissing,
    /// Not every blocked-write reason appears among the bindings.
    BlockedReasonCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No actors are present.
    ActorsMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Actor projection does not satisfy required invariants.
    ActorProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CrossActorConstrainedWriteEnforcementViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::GateBindingsMissing => "gate_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ResolutionFacetIncomplete => "resolution_facet_incomplete",
            Self::WriteDispositionMissingForConstrainedObject => {
                "write_disposition_missing_for_constrained_object"
            }
            Self::WriteDispositionSafeNextStepMismatch => {
                "write_disposition_safe_next_step_mismatch"
            }
            Self::CheckpointSafeNextStepMismatch => "checkpoint_safe_next_step_mismatch",
            Self::ResolutionObjectClassMismatch => "resolution_object_class_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::BlockedReasonDriftAcrossActors => "blocked_reason_drift_across_actors",
            Self::ActorParityUnproven => "actor_parity_unproven",
            Self::ExportReferenceMissing => "export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedContentMissing => "narrow_note_preserved_content_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::SafeBaseActionsMissing => "safe_base_actions_missing",
            Self::OpenActionPostureMismatch => "open_action_posture_mismatch",
            Self::FailClosedReasonPostureMismatch => "fail_closed_reason_posture_mismatch",
            Self::TraceInconsistent => "trace_inconsistent",
            Self::NotRoutedThroughSharedGate => "not_routed_through_shared_gate",
            Self::SafeNextStepNotKeyedToStateClass => "safe_next_step_not_keyed_to_state_class",
            Self::MultiStateFacetHidden => "multi_state_facet_hidden",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::ActorSilentlyWritesConstrainedObjectBypassingDirectTyping => {
                "actor_silently_writes_constrained_object_bypassing_direct_typing"
            }
            Self::GivesAiAutomationImportOrRepairFlowsAHiddenBypass => {
                "gives_ai_automation_import_or_repair_flows_a_hidden_bypass"
            }
            Self::UsesActorSpecificFreeFormBlockedReason => {
                "uses_actor_specific_free_form_blocked_reason"
            }
            Self::LeavesExactWriteTargetOrCanonicalSourceUnstated => {
                "leaves_exact_write_target_or_canonical_source_unstated"
            }
            Self::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior => {
                "lets_one_state_class_hide_another_when_both_materially_affect_behavior"
            }
            Self::ActorCoverageMissing => "actor_coverage_missing",
            Self::BlockedReasonCoverageMissing => "blocked_reason_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ActorsMissing => "actors_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ActorProjectionIncomplete => "actor_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable cross-actor constrained-write enforcement export.
pub fn current_stable_m5_cross_actor_constrained_write_enforcement_export() -> Result<
    M5CrossActorConstrainedWriteEnforcementPacket,
    M5CrossActorConstrainedWriteEnforcementArtifactError,
> {
    let packet: M5CrossActorConstrainedWriteEnforcementPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-cross-actor-constrained-write-enforcement/support_export.json"
        )))
        .map_err(M5CrossActorConstrainedWriteEnforcementArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CrossActorConstrainedWriteEnforcementArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CrossActorConstrainedWriteEnforcementPacket,
    violations: &mut Vec<M5CrossActorConstrainedWriteEnforcementViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_REF,
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_DOC_REF,
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
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5CrossActorConstrainedWriteEnforcementPacket,
    violations: &mut Vec<M5CrossActorConstrainedWriteEnforcementViolation>,
) {
    if packet.gate_bindings.is_empty() {
        violations.push(M5CrossActorConstrainedWriteEnforcementViolation::GateBindingsMissing);
        return;
    }

    // One vocabulary: the gate resolution must be identical for every binding that routes the same constrained
    // object profile, proving the blocked reason is actor-independent.
    let mut profile_resolution: BTreeMap<&str, &GateResolution> = BTreeMap::new();
    let mut drift_reported = false;

    // Actor parity per profile: which actors reached each profile.
    let mut profile_actors: BTreeMap<&str, BTreeSet<MutationActor>> = BTreeMap::new();
    let mut seen_actors: BTreeSet<MutationActor> = BTreeSet::new();
    let mut seen_reasons: BTreeSet<BlockedWriteReason> = BTreeSet::new();

    for binding in &packet.gate_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.object_profile_id.trim().is_empty()
            || binding.object_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::BindingIncomplete);
        }
        if !binding.resolution.all_present() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::ResolutionFacetIncomplete);
        }
        if !binding.resolution.write_disposition_satisfied() {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::WriteDispositionMissingForConstrainedObject,
            );
        }
        if !binding
            .resolution
            .write_disposition_matches_safe_next_step()
        {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::WriteDispositionSafeNextStepMismatch,
            );
        }
        if !binding.resolution.checkpoint_matches_safe_next_step() {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::CheckpointSafeNextStepMismatch,
            );
        }
        if !binding.resolution_matches_object_class() {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::ResolutionObjectClassMismatch,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations
                        .push(M5CrossActorConstrainedWriteEnforcementViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5CrossActorConstrainedWriteEnforcementViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5CrossActorConstrainedWriteEnforcementViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_content_note.trim().is_empty() {
                        violations.push(
                            M5CrossActorConstrainedWriteEnforcementViolation::NarrowNotePreservedContentMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5CrossActorConstrainedWriteEnforcementViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::ExportDetailNoteMissing);
        }

        // Action rules.
        if !binding.has_safe_base_actions() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::SafeBaseActionsMissing);
        }
        if !binding.open_action_matches_posture() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::OpenActionPostureMismatch);
        }

        // Fail-closed reason must be present exactly when the posture fails closed.
        if !binding.fail_closed_reason_matches_posture() {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::FailClosedReasonPostureMismatch,
            );
        }

        // Trace preserves actor, reason, and fallback.
        if !binding.trace_consistent() {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::TraceInconsistent);
        }

        // Positive gate invariants.
        if !binding.routed_through_shared_gate {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::NotRoutedThroughSharedGate);
        }
        if !binding.safe_next_step_keyed_to_state_class {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::SafeNextStepNotKeyedToStateClass,
            );
        }

        // Multi-state facets.
        if !binding.multi_state_facets_consistent() {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::MultiStateFacetHidden);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::AccessibilityStateUndiscoverable,
            );
        }

        // Guardrail row-invariants (each must be false).
        if binding.silently_writes_constrained_object_bypassing_direct_typing {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::ActorSilentlyWritesConstrainedObjectBypassingDirectTyping,
            );
        }
        if binding.gives_ai_automation_import_or_repair_flows_a_hidden_bypass {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::GivesAiAutomationImportOrRepairFlowsAHiddenBypass,
            );
        }
        if binding.uses_actor_specific_free_form_blocked_reason {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::UsesActorSpecificFreeFormBlockedReason,
            );
        }
        if binding.leaves_exact_write_target_or_canonical_source_unstated {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::LeavesExactWriteTargetOrCanonicalSourceUnstated,
            );
        }
        if binding.lets_one_state_class_hide_another_when_both_materially_affect_behavior {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::LetsOneStateClassHideAnotherWhenBothMateriallyAffectBehavior,
            );
        }

        // Export views must map a profile back to canonical contracts.
        if posture_must_reference_canonical(binding.posture)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5CrossActorConstrainedWriteEnforcementViolation::ExportReferenceMissing);
        }

        // Resolution-drift accumulation.
        match profile_resolution.get(binding.object_profile_id.as_str()) {
            None => {
                profile_resolution.insert(binding.object_profile_id.as_str(), &binding.resolution);
            }
            Some(existing) => {
                if **existing != binding.resolution && !drift_reported {
                    violations.push(
                        M5CrossActorConstrainedWriteEnforcementViolation::BlockedReasonDriftAcrossActors,
                    );
                    drift_reported = true;
                }
            }
        }

        profile_actors
            .entry(binding.object_profile_id.as_str())
            .or_default()
            .insert(binding.actor);
        seen_actors.insert(binding.actor);
        seen_reasons.insert(binding.resolution.blocked_write_reason);
    }

    // Coverage: every actor and every blocked-write reason must appear.
    for actor in MutationActor::ALL {
        if !seen_actors.contains(&actor) {
            violations.push(M5CrossActorConstrainedWriteEnforcementViolation::ActorCoverageMissing);
            break;
        }
    }
    for reason in BlockedWriteReason::ALL {
        if !seen_reasons.contains(&reason) {
            violations.push(
                M5CrossActorConstrainedWriteEnforcementViolation::BlockedReasonCoverageMissing,
            );
            break;
        }
    }

    // Actor parity (AC1): at least one profile must be routed by an AI, a repair, an importer, and a direct-save
    // actor — all resolving to the same reason because the resolution is identical per profile.
    let required_parity_actors = [
        MutationActor::AiApply,
        MutationActor::Repair,
        MutationActor::Importer,
        MutationActor::DirectEditSave,
    ];
    let parity_proven = profile_actors
        .values()
        .any(|actors| required_parity_actors.iter().all(|a| actors.contains(a)));
    if !parity_proven {
        violations.push(M5CrossActorConstrainedWriteEnforcementViolation::ActorParityUnproven);
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
