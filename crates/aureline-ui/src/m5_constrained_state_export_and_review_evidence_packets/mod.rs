//! Support / export and review-evidence packets that preserve constrained-state class, source-of-truth relation,
//! exact write-target decision, and chosen fallback path outside the live UI.
//!
//! This module is the B150 support / export and review-evidence packet lane over the six constrained-current-object
//! classes frozen in [`crate::m5_constrained_file_state_matrix`]. Where the state-descriptor, badge-group,
//! canonical-source, write-review-sheet, cross-actor-gate, and drill-corpus lanes make one honest constrained-object
//! *loop* real inside the product, this lane keeps that loop explainable once it leaves the UI: a support bundle, a
//! review / export packet, a piece of local-history / restore evidence, or a docs / help example each preserves the
//! constrained-state class, the canonical source-of-truth relation, the exact write-target decision, and the chosen
//! reviewed fallback path — including whether the operator duplicated, detached, overlaid, requested approval,
//! regenerated, or cancelled, and what sync / regenerate path was preserved versus lost.
//!
//! The three honesty axes mirror the row acceptance criteria.
//!
//! 1. **At least one support packet and one review / export packet preserve constrained-state and write-target
//!    decisions in both human-readable and machine-readable form.** Every binding carries an
//!    [`EvidencePacketChannel`], and the corpus covers all four channels including at least one support bundle and one
//!    review / export packet; every binding carries a [`DualFormEvidence`] with both a plain-language line intelligible
//!    without the live UI and a structured [`MachineReadableRecord`] that names the object class, blocked-write reason,
//!    canonical source, exact write target, chosen fallback path, resolved decision, write disposition, and checkpoint
//!    / undo class.
//! 2. **Exported packets remain intelligible without the live UI and do not flatten constrained-state truth into
//!    generic read-only language.** Each binding derives its [`BlockedWriteReason`], its chosen
//!    [`WriteReviewFallbackAction`], its required [`M5ConstrainedFileStateWriteDisposition`], and its
//!    [`CheckpointUndoClass`] from its object class through the shared pure functions, and carries the controlled
//!    [`ConstrainedStateGrammar`] whose specific state-class label the human-readable line must name, so a packet that
//!    collapses a generated, managed, projection, policy-locked, or captured-snapshot object into an undifferentiated
//!    "read only" is mechanically rejected.
//! 3. **Redacted packets keep the omission reason and still preserve the state class and fallback decision.** Every
//!    binding carries a [`RedactionRecord`]; a redacted binding always names the omission reason and keeps the state
//!    class and chosen fallback decision preserved, so redaction-aware export never hides the fact that the object was
//!    constrained.
//!
//! Every binding names the accessibility routes ([`M5ConstrainedFileStateAccessibilityRoute`]) through which the
//! state class, its canonical source, and its exact write target can be discovered without pointer-only chrome;
//! keyboard focus and screen-reader announcement are mandatory. No packet silently falls back to a lossy direct write,
//! lets one state class hide another, gives AI / automation / import / repair a hidden bypass, or presents a
//! constrained object as directly writable while hiding the recovery / regenerate path.
//!
//! The boundary schema is
//! [`schemas/program/m5-constrained-state-export-and-review-evidence-packets.schema.json`](../../../../schemas/program/m5-constrained-state-export-and-review-evidence-packets.schema.json).
//! The contract doc is
//! [`docs/support/m5_constrained_state_export_and_review_evidence_packets.md`](../../../../docs/support/m5_constrained_state_export_and_review_evidence_packets.md).
//! The protected fixture directory is
//! [`fixtures/editor/m5-constrained-state-evidence/`](../../../../fixtures/editor/m5-constrained-state-evidence/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_constrained_state_evidence_packets,
    seeded_m5_constrained_state_evidence_packets_cancelled_decision_narrowed,
    seeded_m5_constrained_state_evidence_packets_redaction_narrowed,
};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateConsumerSurface,
    M5ConstrainedFileStateObject, M5ConstrainedFileStateRole,
    M5ConstrainedFileStateWriteDisposition, M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};
use crate::m5_cross_actor_constrained_write_enforcement::BlockedWriteReason;
use crate::m5_write_review_sheet_fallback_paths::{CheckpointUndoClass, WriteReviewFallbackAction};

/// Stable record-kind tag carried by [`M5ConstrainedStateEvidencePacket`].
pub const M5_CONSTRAINED_STATE_EVIDENCE_RECORD_KIND: &str =
    "m5_constrained_state_export_and_review_evidence_packets";

/// Schema version for constrained-state evidence-packet records.
pub const M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CONSTRAINED_STATE_EVIDENCE_PACKET_ID: &str =
    "m5-constrained-state-evidence:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_REF: &str =
    "schemas/program/m5-constrained-state-export-and-review-evidence-packets.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CONSTRAINED_STATE_EVIDENCE_DOC_REF: &str =
    "docs/support/m5_constrained_state_export_and_review_evidence_packets.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONSTRAINED_STATE_EVIDENCE_ARTIFACT_REF: &str =
    "artifacts/support/m5-constrained-state-evidence/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CONSTRAINED_STATE_EVIDENCE_CSV_REF: &str =
    "artifacts/support/m5-constrained-state-evidence/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CONSTRAINED_STATE_EVIDENCE_REPORT_REF: &str =
    "artifacts/support/m5-constrained-state-evidence/summary.md";

/// Repo-relative path of the checked health dashboard.
pub const M5_CONSTRAINED_STATE_EVIDENCE_DASHBOARD_REF: &str =
    "dashboards/m5-constrained-state-evidence-health.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONSTRAINED_STATE_EVIDENCE_FIXTURE_DIR: &str =
    "fixtures/editor/m5-constrained-state-evidence";

/// Record kind carried by the health dashboard.
pub const M5_CONSTRAINED_STATE_EVIDENCE_DASHBOARD_RECORD_KIND: &str =
    "m5_constrained_state_evidence_health";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CONSTRAINED_STATE_EVIDENCE_PROOF_SLO_HOURS: u32 = 720;

/// Write-disposition sentinel words a constrained grammar may never fall back to; a constrained-object binding whose
/// state role must be present before it is surfaced as a constrained object must always keep a real write-constrained
/// disposition rather than implying the object is directly writable, editable, or unconstrained.
const WRITE_DISPOSITION_UNCONSTRAINED_SENTINELS: [&str; 4] =
    ["none", "directly_writable", "writable", "editable"];

/// The generic "read-only" tokens a human-readable line may never collapse a non-read-only class into.
const GENERIC_READ_ONLY_FLATTENING_TOKENS: [&str; 3] =
    ["read only", "read-only", "generic read only"];

/// Whether a consumer surface is an export / support path that must map an object class back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5ConstrainedFileStateConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5ConstrainedFileStateConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5ConstrainedFileStateRole`] vocabulary.
pub fn is_known_constrained_file_state_role_token(token: &str) -> bool {
    constrained_file_state_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ConstrainedFileStateRole`], if it is one.
pub fn constrained_file_state_role_from_token(token: &str) -> Option<M5ConstrainedFileStateRole> {
    M5ConstrainedFileStateRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// One of the four distinct evidence-packet channels this lane preserves constrained-state truth across.
///
/// The row acceptance criteria require at least one support bundle and one review / export packet; the local-history /
/// restore evidence and docs / help example channels extend the same preserved fields to restore evidence and
/// documentation examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePacketChannel {
    /// A support bundle pulled for a support / diagnostics case.
    SupportBundle,
    /// A review / export packet handed to a reviewer or exported for the record.
    ReviewExportPacket,
    /// A piece of local-history / restore evidence carried with a checkpoint or restore point.
    LocalHistoryRestoreEvidence,
    /// A docs / help example that documents the constrained-object handling outside the product.
    DocsHelpExample,
}

impl EvidencePacketChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SupportBundle,
        Self::ReviewExportPacket,
        Self::LocalHistoryRestoreEvidence,
        Self::DocsHelpExample,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::ReviewExportPacket => "review_export_packet",
            Self::LocalHistoryRestoreEvidence => "local_history_restore_evidence",
            Self::DocsHelpExample => "docs_help_example",
        }
    }

    /// Whether this channel is the support-bundle channel (one of the AC1-required channels).
    pub const fn is_support_bundle(self) -> bool {
        matches!(self, Self::SupportBundle)
    }

    /// Whether this channel is the review / export-packet channel (one of the AC1-required channels).
    pub const fn is_review_export_packet(self) -> bool {
        matches!(self, Self::ReviewExportPacket)
    }
}

/// What the operator actually resolved a blocked write to: one of the five reviewed fallback transitions, or a
/// cancellation that left the object constrained and unchanged.
///
/// This preserves the chosen fallback decision — whether the operator duplicated, detached, overlaid, requested
/// approval, regenerated, or cancelled — rather than merely which fallback the gate offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolvedFallbackDecision {
    /// The operator duplicated the constrained object into a new editable copy.
    DuplicatedToEditableCopy,
    /// The operator detached a local fork from the managed / externally-owned source.
    DetachedFromManagedSource,
    /// The operator recorded edits as an overlay patch over the backing source.
    CreatedOverlayPatch,
    /// The operator opened an approval request to the policy owner.
    RequestedApproval,
    /// The operator regenerated the artifact from its generator input with a preview.
    RegeneratedWithPreview,
    /// The operator cancelled; the object stayed constrained and unchanged, no fallback was taken.
    Cancelled,
}

impl ResolvedFallbackDecision {
    /// Every resolved decision, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DuplicatedToEditableCopy,
        Self::DetachedFromManagedSource,
        Self::CreatedOverlayPatch,
        Self::RequestedApproval,
        Self::RegeneratedWithPreview,
        Self::Cancelled,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicatedToEditableCopy => "duplicated_to_editable_copy",
            Self::DetachedFromManagedSource => "detached_from_managed_source",
            Self::CreatedOverlayPatch => "created_overlay_patch",
            Self::RequestedApproval => "requested_approval",
            Self::RegeneratedWithPreview => "regenerated_with_preview",
            Self::Cancelled => "cancelled",
        }
    }

    /// The reviewed fallback action this decision took, or `None` when the operator cancelled.
    pub const fn taken_fallback_action(self) -> Option<WriteReviewFallbackAction> {
        match self {
            Self::DuplicatedToEditableCopy => {
                Some(WriteReviewFallbackAction::DuplicateToEditableCopy)
            }
            Self::DetachedFromManagedSource => {
                Some(WriteReviewFallbackAction::DetachFromManagedSource)
            }
            Self::CreatedOverlayPatch => Some(WriteReviewFallbackAction::CreateOverlayPatch),
            Self::RequestedApproval => Some(WriteReviewFallbackAction::RequestApproval),
            Self::RegeneratedWithPreview => Some(WriteReviewFallbackAction::RegenerateWithPreview),
            Self::Cancelled => None,
        }
    }

    /// The resolved decision that corresponds to taking `action`.
    pub const fn from_taken_fallback_action(action: WriteReviewFallbackAction) -> Self {
        match action {
            WriteReviewFallbackAction::DuplicateToEditableCopy => Self::DuplicatedToEditableCopy,
            WriteReviewFallbackAction::DetachFromManagedSource => Self::DetachedFromManagedSource,
            WriteReviewFallbackAction::CreateOverlayPatch => Self::CreatedOverlayPatch,
            WriteReviewFallbackAction::RequestApproval => Self::RequestedApproval,
            WriteReviewFallbackAction::RegenerateWithPreview => Self::RegeneratedWithPreview,
        }
    }

    /// Whether the operator cancelled (no fallback taken).
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

/// The redaction disposition an exported packet carries.
///
/// A redacted binding is still export-safe about the constrained-state truth: it keeps the omission reason and
/// preserves the state class and the chosen fallback decision, so redaction never hides that the object was
/// constrained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionDisposition {
    /// Nothing is redacted; the full detail is present.
    NotRedacted,
    /// Surrounding detail is redacted export-safe, but the state class and fallback decision stay preserved and the
    /// omission reason is named.
    RedactedKeepStateClassAndFallback,
}

impl RedactionDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 2] = [Self::NotRedacted, Self::RedactedKeepStateClassAndFallback];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRedacted => "not_redacted",
            Self::RedactedKeepStateClassAndFallback => "redacted_keep_state_class_and_fallback",
        }
    }

    /// Whether this disposition redacts surrounding detail.
    pub const fn is_redacted(self) -> bool {
        matches!(self, Self::RedactedKeepStateClassAndFallback)
    }
}

/// The action an evidence-packet surface may expose.
///
/// The set is deliberately closed and inspect / export / replay-only: there is no direct-write, save-in-place, apply,
/// or sync action, so an evidence packet can never silently mutate a constrained object. The only write-adjacent
/// action replays the reviewed fallback path rather than performing a lossy best-effort write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAction {
    /// Inspect the preserved constrained-state record, metadata-only.
    InspectConstrainedStateRecord,
    /// Export the evidence packet in its export-safe form.
    ExportEvidencePacket,
    /// Open the reviewed fallback replay (duplicate / detach / overlay / request-approval / regenerate) as a reviewed
    /// transition rather than a lossy direct write.
    OpenReviewedFallbackReplay,
}

impl EvidenceAction {
    /// The inspect / export base action set present on every evidence surface.
    pub const BASE: [Self; 2] = [
        Self::InspectConstrainedStateRecord,
        Self::ExportEvidencePacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectConstrainedStateRecord => "inspect_constrained_state_record",
            Self::ExportEvidencePacket => "export_evidence_packet",
            Self::OpenReviewedFallbackReplay => "open_reviewed_fallback_replay",
        }
    }
}

/// Downgrade trigger that can narrow this evidence-packet lane below its claimed coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstrainedStateEvidenceDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Constrained-state grammar drifted between channels for the same evidence entry.
    GrammarDriftDetected,
    /// A packet dropped its constrained-state classification and began to present the object as directly writable.
    ConstrainedStateClassificationDropped,
    /// A packet flattened a constrained-state class into generic read-only language.
    ConstrainedStateFlattenedToReadOnly,
    /// A packet's machine-readable record drifted apart from its typed decision.
    MachineReadableRecordDrifted,
    /// A packet's human-readable line was dropped or no longer names the state class.
    HumanReadableLineDropped,
    /// A redacted packet dropped its omission reason.
    RedactionOmissionReasonDropped,
    /// A redacted packet dropped the state class or fallback decision it must preserve.
    RedactedStateClassOrFallbackDropped,
    /// A packet's chosen fallback path drifted apart from its blocked-write reason.
    FallbackReasonMismatch,
    /// A packet's resolved decision drifted apart from its chosen fallback path.
    ResolvedDecisionMismatch,
    /// A packet silently fell back to a lossy direct write instead of the reviewed fallback path.
    SilentLossyDirectWriteObserved,
    /// A packet lost its canonical-source or exact-write-target join.
    CanonicalSourceOrWriteTargetMissing,
    /// An AI / automation / import / repair path was given a hidden bypass around the constrained-state rules.
    AiAutomationBypassObserved,
    /// An accessibility route for the state class, canonical source, or write target was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream constrained-file-state contract narrowed.
    UpstreamConstrainedFileStateNarrowed,
}

impl ConstrainedStateEvidenceDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GrammarDriftDetected,
        Self::ConstrainedStateClassificationDropped,
        Self::ConstrainedStateFlattenedToReadOnly,
        Self::MachineReadableRecordDrifted,
        Self::HumanReadableLineDropped,
        Self::RedactionOmissionReasonDropped,
        Self::RedactedStateClassOrFallbackDropped,
        Self::FallbackReasonMismatch,
        Self::ResolvedDecisionMismatch,
        Self::SilentLossyDirectWriteObserved,
        Self::CanonicalSourceOrWriteTargetMissing,
        Self::AiAutomationBypassObserved,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamConstrainedFileStateNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GrammarDriftDetected => "grammar_drift_detected",
            Self::ConstrainedStateClassificationDropped => {
                "constrained_state_classification_dropped"
            }
            Self::ConstrainedStateFlattenedToReadOnly => "constrained_state_flattened_to_read_only",
            Self::MachineReadableRecordDrifted => "machine_readable_record_drifted",
            Self::HumanReadableLineDropped => "human_readable_line_dropped",
            Self::RedactionOmissionReasonDropped => "redaction_omission_reason_dropped",
            Self::RedactedStateClassOrFallbackDropped => "redacted_state_class_or_fallback_dropped",
            Self::FallbackReasonMismatch => "fallback_reason_mismatch",
            Self::ResolvedDecisionMismatch => "resolved_decision_mismatch",
            Self::SilentLossyDirectWriteObserved => "silent_lossy_direct_write_observed",
            Self::CanonicalSourceOrWriteTargetMissing => "canonical_source_or_write_target_missing",
            Self::AiAutomationBypassObserved => "ai_automation_bypass_observed",
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamConstrainedFileStateNarrowed => {
                "upstream_constrained_file_state_narrowed"
            }
        }
    }
}

/// The controlled constrained-state grammar an evidence entry presents.
///
/// These six words describe the constrained-object side of an entry and must be identical across every channel that
/// renders the same entry. The state-role word must be a frozen [`M5ConstrainedFileStateRole`] token; the rest are
/// controlled words the entry carries so it stays attributable to its state class, canonical source, and exact write
/// target rather than collapsing into generic read-only language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedStateGrammar {
    /// State-role word (must be a frozen [`M5ConstrainedFileStateRole`] token).
    pub state_role_word: String,
    /// The state-class badge label word.
    pub state_class_label_word: String,
    /// The blocked-write-reason word.
    pub blocked_write_reason_word: String,
    /// The canonical-source / owning-authority word the object relates back to.
    pub canonical_source_word: String,
    /// The exact-write-target word a write-capable action would touch.
    pub exact_write_target_word: String,
    /// The write-disposition (posture) word; must stay write-constrained, never an unconstrained sentinel.
    pub write_disposition_word: String,
}

impl ConstrainedStateGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.state_role_word.trim().is_empty()
            && !self.state_class_label_word.trim().is_empty()
            && !self.blocked_write_reason_word.trim().is_empty()
            && !self.canonical_source_word.trim().is_empty()
            && !self.exact_write_target_word.trim().is_empty()
            && !self.write_disposition_word.trim().is_empty()
    }

    /// Whether the state-role word is a member of the frozen role vocabulary.
    pub fn state_role_word_in_vocabulary(&self) -> bool {
        is_known_constrained_file_state_role_token(self.state_role_word.trim())
    }

    /// Whether the canonical-source and exact-write-target words that keep the object honest are both present.
    pub fn canonical_source_and_write_target_present(&self) -> bool {
        !self.canonical_source_word.trim().is_empty()
            && !self.exact_write_target_word.trim().is_empty()
    }

    /// Whether the binding honours the write-constrained rule: a state role that must be present before the object may
    /// be surfaced as a constrained object must pair it with a real write-constrained disposition word and never
    /// collapse to a directly-writable / editable / writable / none sentinel.
    pub fn write_disposition_constrained_satisfied(&self) -> bool {
        match constrained_file_state_role_from_token(self.state_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_constrained_object() => {
                let disposition = self.write_disposition_word.trim().to_lowercase();
                !disposition.is_empty()
                    && !WRITE_DISPOSITION_UNCONSTRAINED_SENTINELS.contains(&disposition.as_str())
            }
            _ => true,
        }
    }
}

/// The join that keeps an evidence entry attributable to its canonical source and exact write target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceJoin {
    /// Stable id / ref of the canonical source the constrained object relates back to.
    pub canonical_source_ref: String,
    /// Stable id / ref of the exact write target a write-capable action would touch.
    pub exact_write_target_ref: String,
    /// Stable id / ref of the owning authority (generator, policy owner, or managing owner).
    pub owning_authority_ref: String,
    /// Stable id / ref of the preserved-versus-lost sync-or-regenerate note.
    pub preserved_versus_lost_sync_ref: String,
}

impl CanonicalSourceJoin {
    /// Whether every join ref is present, so the entry is fully attributable.
    pub fn all_present(&self) -> bool {
        !self.canonical_source_ref.trim().is_empty()
            && !self.exact_write_target_ref.trim().is_empty()
            && !self.owning_authority_ref.trim().is_empty()
            && !self.preserved_versus_lost_sync_ref.trim().is_empty()
    }
}

/// What a reviewed transition preserved versus lost, plus the sync / regenerate path that survived or was dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedVersusLost {
    /// What the reviewed transition retained.
    pub retained: String,
    /// What the reviewed transition lost.
    pub lost: String,
    /// The sync / regenerate path that was preserved or lost.
    pub sync_or_regenerate_path: String,
}

impl PreservedVersusLost {
    /// Whether every preserved-versus-lost field is present.
    pub fn all_present(&self) -> bool {
        !self.retained.trim().is_empty()
            && !self.lost.trim().is_empty()
            && !self.sync_or_regenerate_path.trim().is_empty()
    }
}

/// The machine-readable side of an evidence binding: the structured decisions a downstream tool reads without the
/// live UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineReadableRecord {
    /// The constrained-object class token.
    pub object_class_token: String,
    /// The blocked-write reason token.
    pub blocked_write_reason_token: String,
    /// The canonical source-of-truth ref.
    pub canonical_source_ref: String,
    /// The exact write-target ref.
    pub exact_write_target_ref: String,
    /// The chosen fallback path token the gate offered.
    pub chosen_fallback_path_token: String,
    /// The resolved decision token the operator committed to.
    pub resolved_decision_token: String,
    /// The write disposition token.
    pub write_disposition_token: String,
    /// The checkpoint / undo class token.
    pub checkpoint_undo_class_token: String,
}

impl MachineReadableRecord {
    /// Whether every machine-readable field is present.
    pub fn all_present(&self) -> bool {
        !self.object_class_token.trim().is_empty()
            && !self.blocked_write_reason_token.trim().is_empty()
            && !self.canonical_source_ref.trim().is_empty()
            && !self.exact_write_target_ref.trim().is_empty()
            && !self.chosen_fallback_path_token.trim().is_empty()
            && !self.resolved_decision_token.trim().is_empty()
            && !self.write_disposition_token.trim().is_empty()
            && !self.checkpoint_undo_class_token.trim().is_empty()
    }
}

/// The dual-form (human-readable plus machine-readable) evidence a binding preserves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DualFormEvidence {
    /// A plain-language line intelligible without the live UI; must name the specific state class.
    pub human_readable_line: String,
    /// The structured machine-readable record.
    pub machine_readable: MachineReadableRecord,
}

impl DualFormEvidence {
    /// Whether both forms are present.
    pub fn both_forms_present(&self) -> bool {
        !self.human_readable_line.trim().is_empty() && self.machine_readable.all_present()
    }
}

/// The redaction record a binding carries, keeping an exported packet honest about a constrained object even when
/// surrounding detail is redacted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionRecord {
    /// The redaction disposition.
    pub disposition: RedactionDisposition,
    /// The omission reason; always present when the disposition redacts.
    pub omission_reason: Option<String>,
    /// The state class stays preserved even when redacted. MUST be `true`.
    pub state_class_preserved: bool,
    /// The chosen fallback decision stays preserved even when redacted. MUST be `true`.
    pub fallback_decision_preserved: bool,
}

impl RedactionRecord {
    /// Whether the redaction record is internally consistent: a redacted disposition names its omission reason and
    /// keeps the state class and fallback decision preserved; a non-redacted disposition names no omission.
    pub fn is_consistent(&self) -> bool {
        if !self.state_class_preserved || !self.fallback_decision_preserved {
            return false;
        }
        match self.disposition {
            RedactionDisposition::RedactedKeepStateClassAndFallback => self
                .omission_reason
                .as_deref()
                .is_some_and(|reason| !reason.trim().is_empty()),
            RedactionDisposition::NotRedacted => self.omission_reason.is_none(),
        }
    }
}

/// Disclosures an evidence binding must carry, derived from its object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceDisclosure {
    /// The blocked-write reason the object class names.
    pub blocked_write_reason: BlockedWriteReason,
    /// The reviewed fallback path the reason routes to.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The write disposition the reviewed transition requires.
    pub required_write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// The checkpoint / undo class the reviewed transition preserves.
    pub checkpoint_undo_class: CheckpointUndoClass,
}

/// Resolves the disclosures an evidence binding must carry from its object class.
///
/// The blocked-write reason is a pure function of the constrained-object class, the chosen fallback is that reason's
/// safe next step, and the write disposition and checkpoint / undo class follow from the fallback through the shared
/// pure functions.
pub fn resolve_evidence_disclosure(
    object_class: M5ConstrainedFileStateObject,
) -> EvidenceDisclosure {
    let blocked_write_reason = BlockedWriteReason::for_object_class(object_class);
    let chosen_fallback_path = blocked_write_reason.safe_next_step();
    EvidenceDisclosure {
        blocked_write_reason,
        chosen_fallback_path,
        required_write_disposition: chosen_fallback_path.required_write_disposition(),
        checkpoint_undo_class: chosen_fallback_path.required_checkpoint_undo_class(),
    }
}

/// One evidence binding: a seeded constrained-object entry preserved in one packet channel on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidencePacketBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable seeded-entry id (shared across channels that preserve the same entry).
    pub entry_id: String,
    /// Human-readable seeded-entry identity.
    pub entry_label: String,
    /// The packet channel this binding is preserved in.
    pub channel: EvidencePacketChannel,
    /// Which consumer surface renders it.
    pub consumer: M5ConstrainedFileStateConsumerSurface,
    /// Which primary constrained-object class this entry belongs to.
    pub object_class: M5ConstrainedFileStateObject,
    /// The co-applicable second class, if a second state materially affects behaviour.
    pub co_applicable_object_class: Option<M5ConstrainedFileStateObject>,
    /// The blocked-write reason this entry names.
    pub blocked_write_reason: BlockedWriteReason,
    /// The reviewed fallback path the gate offered.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The resolved decision the operator committed to (fallback taken, or cancelled).
    pub resolved_decision: ResolvedFallbackDecision,
    /// The write disposition the reviewed transition requires.
    pub write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// The checkpoint / undo class the reviewed transition preserves.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// The controlled constrained-state grammar preserved (identical across channels for one entry).
    pub constrained_grammar: ConstrainedStateGrammar,
    /// The dual-form (human plus machine) evidence preserved.
    pub dual_form: DualFormEvidence,
    /// What was preserved versus lost, and the sync / regenerate path.
    pub preserved_versus_lost: PreservedVersusLost,
    /// The redaction record keeping the packet honest when redacted.
    pub redaction: RedactionRecord,
    /// The canonical-source / exact-write-target join keeping this entry attributable.
    pub canonical_source_join: CanonicalSourceJoin,
    /// The inspect / export / replay-only action set allowed on this surface.
    pub allowed_actions: Vec<EvidenceAction>,
    /// The accessibility routes through which the state class, canonical source, and write target can be discovered
    /// without pointer-only chrome.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// The constrained state class is explicitly classified. MUST be `true`.
    pub constrained_state_explicitly_classified: bool,
    /// The state class and fallback decision stay preserved when redacted. MUST be `true`.
    pub preserves_state_class_and_fallback_when_redacted: bool,
    /// Guardrail: this packet flattens the constrained-state class into generic read-only language. MUST be `false`.
    pub flattens_constrained_state_into_generic_read_only_language: bool,
    /// Guardrail: this packet drops the omission reason when redacted. MUST be `false`.
    pub drops_omission_reason_when_redacted: bool,
    /// Guardrail: this packet lets one constrained state class hide another. MUST be `false`.
    pub lets_one_constrained_state_class_hide_another: bool,
    /// Guardrail: this packet silently falls back to a lossy direct write. MUST be `false`.
    pub silently_falls_back_to_lossy_direct_write: bool,
    /// Guardrail: this packet gives AI / automation / import / repair a hidden bypass. MUST be `false`.
    pub gives_ai_automation_import_or_repair_a_hidden_bypass: bool,
    /// Guardrail: this packet leaves the canonical source or exact write target unstated. MUST be `false`.
    pub leaves_canonical_source_or_exact_write_target_unstated: bool,
    /// Guardrail: this packet presents the object as directly writable or hides the recovery / regenerate path. MUST
    /// be `false`.
    pub presents_as_directly_writable_or_hides_recovery_path: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl EvidencePacketBinding {
    /// Disclosures this binding must carry, derived from its object class.
    pub fn disclosure(&self) -> EvidenceDisclosure {
        resolve_evidence_disclosure(self.object_class)
    }

    /// Whether the blocked-write reason matches the primary object class (the actor-independent reason vocabulary).
    pub fn blocked_reason_matches_class(&self) -> bool {
        self.blocked_write_reason == BlockedWriteReason::for_object_class(self.object_class)
    }

    /// Whether the chosen fallback path matches the blocked-write reason's safe next step.
    pub fn fallback_matches_reason(&self) -> bool {
        self.chosen_fallback_path == self.blocked_write_reason.safe_next_step()
    }

    /// Whether the write disposition matches the chosen fallback path's required disposition.
    pub fn disposition_matches_fallback(&self) -> bool {
        self.write_disposition == self.chosen_fallback_path.required_write_disposition()
    }

    /// Whether the checkpoint / undo class matches the chosen fallback path's required class.
    pub fn checkpoint_matches_fallback(&self) -> bool {
        self.checkpoint_undo_class == self.chosen_fallback_path.required_checkpoint_undo_class()
    }

    /// Whether the resolved decision is consistent with the chosen fallback: a taken decision matches the offered
    /// fallback path, and a cancellation leaves the object constrained (the offered path is still recorded).
    pub fn resolved_decision_consistent(&self) -> bool {
        match self.resolved_decision.taken_fallback_action() {
            Some(action) => action == self.chosen_fallback_path,
            None => true,
        }
    }

    /// Whether every guardrail row-invariant holds.
    pub const fn guardrails_hold(&self) -> bool {
        self.constrained_state_explicitly_classified
            && self.preserves_state_class_and_fallback_when_redacted
            && !self.flattens_constrained_state_into_generic_read_only_language
            && !self.drops_omission_reason_when_redacted
            && !self.lets_one_constrained_state_class_hide_another
            && !self.silently_falls_back_to_lossy_direct_write
            && !self.gives_ai_automation_import_or_repair_a_hidden_bypass
            && !self.leaves_canonical_source_or_exact_write_target_unstated
            && !self.presents_as_directly_writable_or_hides_recovery_path
    }

    /// Whether the inspect / export base action set is present.
    pub fn has_base_actions(&self) -> bool {
        EvidenceAction::BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether the action set is the closed evidence action set (no direct-write / save / apply / sync affordance).
    pub fn action_set_is_closed(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                EvidenceAction::InspectConstrainedStateRecord
                    | EvidenceAction::ExportEvidencePacket
                    | EvidenceAction::OpenReviewedFallbackReplay
            )
        })
    }

    /// Whether the reviewed-fallback replay action is offered, so a constrained write always routes to a reviewed
    /// transition rather than a silent lossy write.
    pub fn reviewed_fallback_replay_present(&self) -> bool {
        self.allowed_actions
            .contains(&EvidenceAction::OpenReviewedFallbackReplay)
    }

    /// Whether both dual forms are present (AC1: human-readable and machine-readable).
    pub fn both_forms_present(&self) -> bool {
        self.dual_form.both_forms_present()
    }

    /// Whether the machine-readable record faithfully mirrors the typed decisions (so the machine form preserves the
    /// same constrained-state and write-target decisions as the binding).
    pub fn machine_readable_matches_binding(&self) -> bool {
        let record = &self.dual_form.machine_readable;
        record.object_class_token == self.object_class.as_str()
            && record.blocked_write_reason_token == self.blocked_write_reason.as_str()
            && record.canonical_source_ref == self.canonical_source_join.canonical_source_ref
            && record.exact_write_target_ref == self.canonical_source_join.exact_write_target_ref
            && record.chosen_fallback_path_token == self.chosen_fallback_path.as_str()
            && record.resolved_decision_token == self.resolved_decision.as_str()
            && record.write_disposition_token == self.write_disposition.as_str()
            && record.checkpoint_undo_class_token == self.checkpoint_undo_class.as_str()
    }

    /// Whether the human-readable line names the specific state class (AC2: not flattened into generic read-only
    /// language). The line must contain the specific state-class label word, and — for a non-read-only class — must
    /// not collapse into a bare generic "read only".
    pub fn human_readable_names_state_class(&self) -> bool {
        let line = self.dual_form.human_readable_line.to_lowercase();
        let label = self
            .constrained_grammar
            .state_class_label_word
            .to_lowercase();
        if label.trim().is_empty() || !line.contains(label.trim()) {
            return false;
        }
        if self.object_class != M5ConstrainedFileStateObject::ReadOnly {
            let stripped = line.replace(label.trim(), "");
            if GENERIC_READ_ONLY_FLATTENING_TOKENS
                .iter()
                .any(|token| stripped.contains(token))
            {
                return false;
            }
        }
        true
    }

    /// Whether the redaction record is consistent and, when redacted, the state class and fallback decision stay
    /// preserved.
    pub fn redaction_consistent(&self) -> bool {
        if !self.redaction.is_consistent() {
            return false;
        }
        if self.redaction.disposition.is_redacted() {
            self.preserves_state_class_and_fallback_when_redacted
                && !self.drops_omission_reason_when_redacted
        } else {
            true
        }
    }

    /// Whether the binding renders its canonical source and exact write target instead of leaving them unstated.
    pub fn renders_canonical_source_and_write_target(&self) -> bool {
        self.canonical_source_join.all_present()
            && self
                .constrained_grammar
                .canonical_source_and_write_target_present()
            && self.preserved_versus_lost.all_present()
            && !self.leaves_canonical_source_or_exact_write_target_unstated
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
pub struct EvidenceTrustReview {
    /// The corpus covers every packet channel.
    pub covers_every_packet_channel: bool,
    /// The corpus includes at least one support bundle and one review / export packet.
    pub includes_support_bundle_and_review_export_packet: bool,
    /// Every binding preserves both a human-readable and a machine-readable form.
    pub every_binding_preserves_both_forms: bool,
    /// Every machine-readable record mirrors its typed decision.
    pub machine_readable_mirrors_typed_decision: bool,
    /// No packet flattens constrained-state truth into generic read-only language.
    pub no_packet_flattens_into_generic_read_only: bool,
    /// The corpus covers every resolved decision, including cancellation.
    pub covers_every_resolved_decision_including_cancel: bool,
    /// The corpus includes at least one redacted binding that keeps its omission reason.
    pub includes_redacted_binding_keeping_omission_reason: bool,
    /// Redacted bindings keep the state class and fallback decision preserved.
    pub redacted_bindings_keep_state_class_and_fallback: bool,
    /// The same entry presents the same constrained-state grammar across channels.
    pub constrained_grammar_identical_for_same_entry: bool,
    /// Every state-role word is a frozen role token.
    pub state_role_words_stay_in_frozen_vocabulary: bool,
    /// Canonical source and exact write target are present on every binding.
    pub canonical_source_and_write_target_present_on_every_binding: bool,
    /// Every blocked write routes to the reviewed fallback path keyed to its state class.
    pub every_blocked_write_routes_to_reviewed_fallback: bool,
    /// No packet silently falls back to a lossy direct write.
    pub no_packet_silently_falls_back_to_lossy_direct_write: bool,
    /// No AI / automation / import / repair path gets a hidden bypass.
    pub no_ai_automation_import_or_repair_bypass: bool,
    /// Every object class is preserved by two or more distinct channels.
    pub every_object_class_preserved_by_two_or_more_channels: bool,
    /// Accessibility routes for the state class, canonical source, and write target are present.
    pub accessibility_routes_present_for_state_source_and_target: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl EvidenceTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.covers_every_packet_channel
            && self.includes_support_bundle_and_review_export_packet
            && self.every_binding_preserves_both_forms
            && self.machine_readable_mirrors_typed_decision
            && self.no_packet_flattens_into_generic_read_only
            && self.covers_every_resolved_decision_including_cancel
            && self.includes_redacted_binding_keeping_omission_reason
            && self.redacted_bindings_keep_state_class_and_fallback
            && self.constrained_grammar_identical_for_same_entry
            && self.state_role_words_stay_in_frozen_vocabulary
            && self.canonical_source_and_write_target_present_on_every_binding
            && self.every_blocked_write_routes_to_reviewed_fallback
            && self.no_packet_silently_falls_back_to_lossy_direct_write
            && self.no_ai_automation_import_or_repair_bypass
            && self.every_object_class_preserved_by_two_or_more_channels
            && self.accessibility_routes_present_for_state_source_and_target
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceConsumerProjection {
    /// The support-bundle channel preserves the constrained-state record.
    pub support_bundle_preserves_record: bool,
    /// The review / export-packet channel preserves the constrained-state record.
    pub review_export_packet_preserves_record: bool,
    /// The local-history / restore-evidence channel preserves the constrained-state record.
    pub local_history_restore_evidence_preserves_record: bool,
    /// The docs / help-example channel preserves the constrained-state record.
    pub docs_help_example_preserves_record: bool,
    /// Every object class is preserved by two or more channels.
    pub every_object_class_preserved_by_two_or_more_channels: bool,
    /// Constrained-state grammar is identical for the same entry.
    pub constrained_grammar_identical_for_same_entry: bool,
    /// The constrained state is disclosed rather than flattened.
    pub constrained_state_disclosed_not_flattened: bool,
    /// Export maps a binding row back to one constrained-object class.
    pub binding_maps_back_to_one_constrained_object: bool,
}

impl EvidenceConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.support_bundle_preserves_record
            && self.review_export_packet_preserves_record
            && self.local_history_restore_evidence_preserves_record
            && self.docs_help_example_preserves_record
            && self.every_object_class_preserved_by_two_or_more_channels
            && self.constrained_grammar_identical_for_same_entry
            && self.constrained_state_disclosed_not_flattened
            && self.binding_maps_back_to_one_constrained_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ConstrainedStateEvidencePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ConstrainedStateEvidencePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Evidence bindings.
    pub evidence_bindings: Vec<EvidencePacketBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ConstrainedStateEvidenceDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: EvidenceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EvidenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: EvidenceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe constrained-state export and review-evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedStateEvidencePacket {
    /// Record kind; must equal [`M5_CONSTRAINED_STATE_EVIDENCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Evidence bindings.
    pub evidence_bindings: Vec<EvidencePacketBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ConstrainedStateEvidenceDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: EvidenceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EvidenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: EvidenceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ConstrainedStateEvidencePacket {
    /// Builds an evidence packet from stable-lane input.
    pub fn new(input: M5ConstrainedStateEvidencePacketInput) -> Self {
        Self {
            record_kind: M5_CONSTRAINED_STATE_EVIDENCE_RECORD_KIND.to_owned(),
            schema_version: M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            evidence_bindings: input.evidence_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the evidence-packet invariants.
    pub fn validate(&self) -> Vec<M5ConstrainedStateEvidenceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONSTRAINED_STATE_EVIDENCE_RECORD_KIND {
            violations.push(M5ConstrainedStateEvidenceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_VERSION {
            violations.push(M5ConstrainedStateEvidenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ConstrainedStateEvidenceViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ConstrainedStateEvidenceViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ConstrainedStateEvidenceViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ConstrainedStateEvidenceViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ConstrainedStateEvidenceViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ConstrainedStateEvidenceViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("evidence packet serializes"),
        ) {
            violations.push(M5ConstrainedStateEvidenceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("evidence packet serializes")
    }

    /// Deterministic matrix CSV, one row per evidence binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,channel,consumer,blocked_write_reason,chosen_fallback_path,resolved_decision,write_disposition,redaction_disposition,entry_id\n",
        );
        for binding in &self.evidence_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.channel.as_str(),
                binding.consumer.as_str(),
                binding.blocked_write_reason.as_str(),
                binding.chosen_fallback_path.as_str(),
                binding.resolved_decision.as_str(),
                binding.write_disposition.as_str(),
                binding.redaction.disposition.as_str(),
                binding.entry_id.replace(',', ";"),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let redacted = self
            .evidence_bindings
            .iter()
            .filter(|binding| binding.redaction.disposition.is_redacted())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Constrained-State Export and Review-Evidence Packets: Preserved Class, Source, Write Target, and Fallback\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Evidence bindings: {} ({} redacted, keeping omission reason and state class)\n",
            self.evidence_bindings.len(),
            redacted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Evidence bindings\n\n");
        for binding in &self.evidence_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` in channel `{}` on `{}`, reason `{}`, fallback `{}`, decision `{}`, disposition `{}`, redaction `{}`\n",
                binding.entry_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.channel.as_str(),
                binding.consumer.as_str(),
                binding.blocked_write_reason.as_str(),
                binding.chosen_fallback_path.as_str(),
                binding.resolved_decision.as_str(),
                binding.write_disposition.as_str(),
                binding.redaction.disposition.as_str(),
            ));
        }
        out
    }

    /// Deterministic health dashboard JSON, minted from truth, so release / support can surface this lane.
    pub fn render_health_dashboard(&self) -> String {
        let dashboard = EvidenceHealthDashboard {
            record_kind: M5_CONSTRAINED_STATE_EVIDENCE_DASHBOARD_RECORD_KIND,
            packet_id: &self.packet_id,
            support_export_ref: M5_CONSTRAINED_STATE_EVIDENCE_ARTIFACT_REF,
            evidence_schema_ref: M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_REF,
            matrix_schema_ref: M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            channels: EvidencePacketChannel::ALL
                .iter()
                .map(|c| c.as_str())
                .collect(),
            blocked_write_reasons: BlockedWriteReason::ALL.iter().map(|r| r.as_str()).collect(),
            resolved_decisions: ResolvedFallbackDecision::ALL
                .iter()
                .map(|d| d.as_str())
                .collect(),
            redaction_dispositions: RedactionDisposition::ALL
                .iter()
                .map(|d| d.as_str())
                .collect(),
            redacted_binding_count: self
                .evidence_bindings
                .iter()
                .filter(|binding| binding.redaction.disposition.is_redacted())
                .count(),
            entry_families: M5ConstrainedFileStateObject::ALL
                .iter()
                .map(|object_class| EvidenceEntryFamily {
                    object_class: object_class.as_str(),
                    canonical_schema: object_class.canonical_domain_schema_ref(),
                })
                .collect(),
        };
        serde_json::to_string_pretty(&dashboard).expect("evidence dashboard serializes")
    }
}

#[derive(Serialize)]
struct EvidenceHealthDashboard<'a> {
    record_kind: &'a str,
    packet_id: &'a str,
    support_export_ref: &'a str,
    evidence_schema_ref: &'a str,
    matrix_schema_ref: &'a str,
    channels: Vec<&'a str>,
    blocked_write_reasons: Vec<&'a str>,
    resolved_decisions: Vec<&'a str>,
    redaction_dispositions: Vec<&'a str>,
    redacted_binding_count: usize,
    entry_families: Vec<EvidenceEntryFamily<'a>>,
}

#[derive(Serialize)]
struct EvidenceEntryFamily<'a> {
    object_class: &'a str,
    canonical_schema: &'a str,
}

/// Errors emitted when reading the checked-in evidence-packet export.
#[derive(Debug)]
pub enum M5ConstrainedStateEvidenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ConstrainedStateEvidenceViolation>),
}

impl fmt::Display for M5ConstrainedStateEvidenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "evidence export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "evidence export failed validation: {tokens}")
            }
        }
    }
}

impl Error for M5ConstrainedStateEvidenceArtifactError {}

/// Validation failures emitted by [`M5ConstrainedStateEvidencePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ConstrainedStateEvidenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No evidence bindings are present.
    EvidenceBindingsMissing,
    /// An evidence binding is incomplete.
    BindingIncomplete,
    /// A binding's constrained-state grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's state-role word is not a frozen role token.
    StateRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its write-constrained disposition.
    WriteDispositionUnconstrainedForGateRole,
    /// A binding's blocked-write reason does not match its object class.
    BlockedReasonClassMismatch,
    /// A binding's chosen fallback path does not match its blocked-write reason.
    FallbackReasonMismatch,
    /// A binding's write disposition does not match its chosen fallback path.
    WriteDispositionFallbackMismatch,
    /// A binding's checkpoint / undo class does not match its chosen fallback path.
    CheckpointFallbackMismatch,
    /// A binding's resolved decision is inconsistent with its chosen fallback path.
    ResolvedDecisionInconsistent,
    /// Two channels preserve the same entry with different constrained-state grammar.
    GrammarDriftAcrossChannels,
    /// A shared object class is not preserved by at least two distinct channels.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A binding's canonical-source / exact-write-target join is incomplete.
    CanonicalSourceJoinIncomplete,
    /// A binding's preserved-versus-lost record is incomplete.
    PreservedVersusLostIncomplete,
    /// A binding is missing one of its dual forms.
    DualFormIncomplete,
    /// A binding's machine-readable record drifts from its typed decision.
    MachineReadableRecordMismatch,
    /// A binding's human-readable line flattens the state class into generic read-only language.
    HumanReadableFlattensStateClass,
    /// A binding's redaction record is inconsistent.
    RedactionRecordInconsistent,
    /// A redacted binding dropped its omission reason.
    RedactionOmissionReasonDropped,
    /// A redacted binding dropped the state class or fallback decision it must preserve.
    RedactedStateClassOrFallbackDropped,
    /// A binding is missing the inspect / export base action set.
    BaseActionsMissing,
    /// A binding's action set is not the closed evidence action set.
    ActionSetNotClosed,
    /// A binding does not offer the reviewed fallback replay.
    ReviewedFallbackReplayMissing,
    /// A binding leaves its canonical source or exact write target unstated.
    CanonicalSourceOrWriteTargetUnstated,
    /// A binding cannot discover its state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's constrained state is not explicitly classified.
    ConstrainedStateNotClassified,
    /// A binding lets one constrained state class hide another.
    LetsOneConstrainedStateClassHideAnother,
    /// A binding silently falls back to a lossy direct write.
    SilentlyFallsBackToLossyDirectWrite,
    /// A binding gives AI / automation / import / repair a hidden bypass.
    GivesAiAutomationImportOrRepairAHiddenBypass,
    /// A binding presents the object as directly writable or hides the recovery / regenerate path.
    PresentsAsDirectlyWritableOrHidesRecoveryPath,
    /// Not every packet channel appears among the bindings.
    ChannelCoverageMissing,
    /// The corpus is missing a support bundle.
    SupportBundleMissing,
    /// The corpus is missing a review / export packet.
    ReviewExportPacketMissing,
    /// Not every constrained-object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every blocked-write reason appears among the bindings.
    BlockedWriteReasonCoverageMissing,
    /// Not every reviewed fallback path appears among the bindings.
    FallbackPathCoverageMissing,
    /// Not every resolved decision (including cancellation) appears among the bindings.
    ResolvedDecisionCoverageMissing,
    /// Fewer than both redaction dispositions appear (no redacted binding present).
    RedactionCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ConstrainedStateEvidenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::EvidenceBindingsMissing => "evidence_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::StateRoleWordOutsideVocabulary => "state_role_word_outside_vocabulary",
            Self::WriteDispositionUnconstrainedForGateRole => {
                "write_disposition_unconstrained_for_gate_role"
            }
            Self::BlockedReasonClassMismatch => "blocked_reason_class_mismatch",
            Self::FallbackReasonMismatch => "fallback_reason_mismatch",
            Self::WriteDispositionFallbackMismatch => "write_disposition_fallback_mismatch",
            Self::CheckpointFallbackMismatch => "checkpoint_fallback_mismatch",
            Self::ResolvedDecisionInconsistent => "resolved_decision_inconsistent",
            Self::GrammarDriftAcrossChannels => "grammar_drift_across_channels",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::CanonicalSourceJoinIncomplete => "canonical_source_join_incomplete",
            Self::PreservedVersusLostIncomplete => "preserved_versus_lost_incomplete",
            Self::DualFormIncomplete => "dual_form_incomplete",
            Self::MachineReadableRecordMismatch => "machine_readable_record_mismatch",
            Self::HumanReadableFlattensStateClass => "human_readable_flattens_state_class",
            Self::RedactionRecordInconsistent => "redaction_record_inconsistent",
            Self::RedactionOmissionReasonDropped => "redaction_omission_reason_dropped",
            Self::RedactedStateClassOrFallbackDropped => "redacted_state_class_or_fallback_dropped",
            Self::BaseActionsMissing => "base_actions_missing",
            Self::ActionSetNotClosed => "action_set_not_closed",
            Self::ReviewedFallbackReplayMissing => "reviewed_fallback_replay_missing",
            Self::CanonicalSourceOrWriteTargetUnstated => {
                "canonical_source_or_write_target_unstated"
            }
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::ConstrainedStateNotClassified => "constrained_state_not_classified",
            Self::LetsOneConstrainedStateClassHideAnother => {
                "lets_one_constrained_state_class_hide_another"
            }
            Self::SilentlyFallsBackToLossyDirectWrite => {
                "silently_falls_back_to_lossy_direct_write"
            }
            Self::GivesAiAutomationImportOrRepairAHiddenBypass => {
                "gives_ai_automation_import_or_repair_a_hidden_bypass"
            }
            Self::PresentsAsDirectlyWritableOrHidesRecoveryPath => {
                "presents_as_directly_writable_or_hides_recovery_path"
            }
            Self::ChannelCoverageMissing => "channel_coverage_missing",
            Self::SupportBundleMissing => "support_bundle_missing",
            Self::ReviewExportPacketMissing => "review_export_packet_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::BlockedWriteReasonCoverageMissing => "blocked_write_reason_coverage_missing",
            Self::FallbackPathCoverageMissing => "fallback_path_coverage_missing",
            Self::ResolvedDecisionCoverageMissing => "resolved_decision_coverage_missing",
            Self::RedactionCoverageMissing => "redaction_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable evidence-packet export.
pub fn current_stable_m5_constrained_state_evidence_export(
) -> Result<M5ConstrainedStateEvidencePacket, M5ConstrainedStateEvidenceArtifactError> {
    let packet: M5ConstrainedStateEvidencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-constrained-state-evidence/support_export.json"
    )))
    .map_err(M5ConstrainedStateEvidenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ConstrainedStateEvidenceArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ConstrainedStateEvidencePacket,
    violations: &mut Vec<M5ConstrainedStateEvidenceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CONSTRAINED_STATE_EVIDENCE_SCHEMA_REF,
        M5_CONSTRAINED_STATE_EVIDENCE_DOC_REF,
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
            violations.push(M5ConstrainedStateEvidenceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ConstrainedStateEvidencePacket,
    violations: &mut Vec<M5ConstrainedStateEvidenceViolation>,
) {
    if packet.evidence_bindings.is_empty() {
        violations.push(M5ConstrainedStateEvidenceViolation::EvidenceBindingsMissing);
        return;
    }

    // One vocabulary: the constrained-state grammar must be identical for every binding that preserves the same entry.
    let mut entry_grammar: BTreeMap<&str, &ConstrainedStateGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be preserved by at least two distinct channels.
    let mut object_channels: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<EvidencePacketChannel>,
    > = BTreeMap::new();
    let mut seen_channels: BTreeSet<EvidencePacketChannel> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ConstrainedFileStateObject> = BTreeSet::new();
    let mut seen_reasons: BTreeSet<BlockedWriteReason> = BTreeSet::new();
    let mut seen_fallbacks: BTreeSet<WriteReviewFallbackAction> = BTreeSet::new();
    let mut seen_decisions: BTreeSet<ResolvedFallbackDecision> = BTreeSet::new();
    let mut seen_redactions: BTreeSet<RedactionDisposition> = BTreeSet::new();

    for binding in &packet.evidence_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.entry_id.trim().is_empty()
            || binding.entry_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ConstrainedStateEvidenceViolation::BindingIncomplete);
        }
        if !binding.constrained_grammar.all_present() {
            violations.push(M5ConstrainedStateEvidenceViolation::GrammarFacetIncomplete);
        }
        if !binding.constrained_grammar.state_role_word_in_vocabulary() {
            violations.push(M5ConstrainedStateEvidenceViolation::StateRoleWordOutsideVocabulary);
        }
        if !binding
            .constrained_grammar
            .write_disposition_constrained_satisfied()
        {
            violations.push(
                M5ConstrainedStateEvidenceViolation::WriteDispositionUnconstrainedForGateRole,
            );
        }

        if !binding.blocked_reason_matches_class() {
            violations.push(M5ConstrainedStateEvidenceViolation::BlockedReasonClassMismatch);
        }
        if !binding.fallback_matches_reason() {
            violations.push(M5ConstrainedStateEvidenceViolation::FallbackReasonMismatch);
        }
        if !binding.disposition_matches_fallback() {
            violations.push(M5ConstrainedStateEvidenceViolation::WriteDispositionFallbackMismatch);
        }
        if !binding.checkpoint_matches_fallback() {
            violations.push(M5ConstrainedStateEvidenceViolation::CheckpointFallbackMismatch);
        }
        if !binding.resolved_decision_consistent() {
            violations.push(M5ConstrainedStateEvidenceViolation::ResolvedDecisionInconsistent);
        }

        // Canonical-source / exact-write-target join and preserved-versus-lost record.
        if !binding.canonical_source_join.all_present() {
            violations.push(M5ConstrainedStateEvidenceViolation::CanonicalSourceJoinIncomplete);
        }
        if !binding.preserved_versus_lost.all_present() {
            violations.push(M5ConstrainedStateEvidenceViolation::PreservedVersusLostIncomplete);
        }

        // Dual form (AC1) and machine-readable faithfulness / non-flattening (AC2).
        if !binding.both_forms_present() {
            violations.push(M5ConstrainedStateEvidenceViolation::DualFormIncomplete);
        }
        if !binding.machine_readable_matches_binding() {
            violations.push(M5ConstrainedStateEvidenceViolation::MachineReadableRecordMismatch);
        }
        if !binding.human_readable_names_state_class() {
            violations.push(M5ConstrainedStateEvidenceViolation::HumanReadableFlattensStateClass);
        }

        // Redaction (AC3).
        if !binding.redaction.is_consistent() {
            violations.push(M5ConstrainedStateEvidenceViolation::RedactionRecordInconsistent);
        }
        if binding.redaction.disposition.is_redacted() {
            if binding
                .redaction
                .omission_reason
                .as_deref()
                .map_or(true, |reason| reason.trim().is_empty())
                || binding.drops_omission_reason_when_redacted
            {
                violations
                    .push(M5ConstrainedStateEvidenceViolation::RedactionOmissionReasonDropped);
            }
            if !binding.preserves_state_class_and_fallback_when_redacted
                || !binding.redaction.state_class_preserved
                || !binding.redaction.fallback_decision_preserved
            {
                violations
                    .push(M5ConstrainedStateEvidenceViolation::RedactedStateClassOrFallbackDropped);
            }
        }

        // Action rules.
        if !binding.has_base_actions() {
            violations.push(M5ConstrainedStateEvidenceViolation::BaseActionsMissing);
        }
        if !binding.action_set_is_closed() {
            violations.push(M5ConstrainedStateEvidenceViolation::ActionSetNotClosed);
        }
        if !binding.reviewed_fallback_replay_present() {
            violations.push(M5ConstrainedStateEvidenceViolation::ReviewedFallbackReplayMissing);
        }

        // Canonical-source / write-target honesty.
        if !binding.renders_canonical_source_and_write_target() {
            violations
                .push(M5ConstrainedStateEvidenceViolation::CanonicalSourceOrWriteTargetUnstated);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5ConstrainedStateEvidenceViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.constrained_state_explicitly_classified {
            violations.push(M5ConstrainedStateEvidenceViolation::ConstrainedStateNotClassified);
        }
        if binding.flattens_constrained_state_into_generic_read_only_language {
            violations.push(M5ConstrainedStateEvidenceViolation::HumanReadableFlattensStateClass);
        }
        if binding.lets_one_constrained_state_class_hide_another {
            violations
                .push(M5ConstrainedStateEvidenceViolation::LetsOneConstrainedStateClassHideAnother);
        }
        if binding.silently_falls_back_to_lossy_direct_write {
            violations
                .push(M5ConstrainedStateEvidenceViolation::SilentlyFallsBackToLossyDirectWrite);
        }
        if binding.gives_ai_automation_import_or_repair_a_hidden_bypass {
            violations.push(
                M5ConstrainedStateEvidenceViolation::GivesAiAutomationImportOrRepairAHiddenBypass,
            );
        }
        if binding.presents_as_directly_writable_or_hides_recovery_path {
            violations.push(
                M5ConstrainedStateEvidenceViolation::PresentsAsDirectlyWritableOrHidesRecoveryPath,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ConstrainedStateEvidenceViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match entry_grammar.get(binding.entry_id.as_str()) {
            None => {
                entry_grammar.insert(binding.entry_id.as_str(), &binding.constrained_grammar);
            }
            Some(existing) => {
                if **existing != binding.constrained_grammar && !drift_reported {
                    violations
                        .push(M5ConstrainedStateEvidenceViolation::GrammarDriftAcrossChannels);
                    drift_reported = true;
                }
            }
        }

        object_channels
            .entry(binding.object_class)
            .or_default()
            .insert(binding.channel);
        seen_channels.insert(binding.channel);
        seen_objects.insert(binding.object_class);
        seen_reasons.insert(binding.blocked_write_reason);
        seen_fallbacks.insert(binding.chosen_fallback_path);
        seen_decisions.insert(binding.resolved_decision);
        seen_redactions.insert(binding.redaction.disposition);
    }

    // Coverage: every channel, object class, reason, fallback, resolved decision, and redaction disposition.
    for channel in EvidencePacketChannel::ALL {
        if !seen_channels.contains(&channel) {
            violations.push(M5ConstrainedStateEvidenceViolation::ChannelCoverageMissing);
            break;
        }
    }
    if !seen_channels.contains(&EvidencePacketChannel::SupportBundle) {
        violations.push(M5ConstrainedStateEvidenceViolation::SupportBundleMissing);
    }
    if !seen_channels.contains(&EvidencePacketChannel::ReviewExportPacket) {
        violations.push(M5ConstrainedStateEvidenceViolation::ReviewExportPacketMissing);
    }
    for object_class in M5ConstrainedFileStateObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5ConstrainedStateEvidenceViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for reason in BlockedWriteReason::ALL {
        if !seen_reasons.contains(&reason) {
            violations.push(M5ConstrainedStateEvidenceViolation::BlockedWriteReasonCoverageMissing);
            break;
        }
    }
    for fallback in WriteReviewFallbackAction::ALL {
        if !seen_fallbacks.contains(&fallback) {
            violations.push(M5ConstrainedStateEvidenceViolation::FallbackPathCoverageMissing);
            break;
        }
    }
    for decision in ResolvedFallbackDecision::ALL {
        if !seen_decisions.contains(&decision) {
            violations.push(M5ConstrainedStateEvidenceViolation::ResolvedDecisionCoverageMissing);
            break;
        }
    }
    for redaction in RedactionDisposition::ALL {
        if !seen_redactions.contains(&redaction) {
            violations.push(M5ConstrainedStateEvidenceViolation::RedactionCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be preserved by two or more distinct channels.
    for channels in object_channels.values() {
        if channels.len() < 2 {
            violations.push(M5ConstrainedStateEvidenceViolation::ObjectClassReuseUnproven);
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
