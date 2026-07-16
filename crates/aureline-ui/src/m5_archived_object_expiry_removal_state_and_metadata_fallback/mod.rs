//! Archived-object expiry / removal state model with metadata / remove fallbacks, so archived or imported
//! evidence never collapses into a dead link or a false readiness once its retention window closes, its bytes
//! are removed, or its live target disappears.
//!
//! This module is the B149 expired / removed / retention-ended / missing-live-target state lane over the five
//! non-live-evidence object classes frozen in [`crate::m5_historical_reference_matrix`]. Where the
//! archive-viewer lane ([`crate::m5_archived_snapshot_viewer_and_analysis_only_banner_consumers`]) proves how a
//! preserved snapshot is *shown* as non-live, the compare-flow lane
//! ([`crate::m5_historical_versus_live_compare_flow`]) proves how it is *compared* against its live target, and
//! the live-target-handoff lane ([`crate::m5_live_target_handoff_packet_and_route_validation`]) makes reopening
//! a current object a validated pivot, this lane keeps a preserved object *honest after the fact*: when its
//! retention window ends, its content is expired or removed, or its live target goes missing, the archived
//! object transitions into an explicit lifecycle state that still renders its capture time, provenance, and the
//! exact expiry / removal explanation — never a blank pane, never a generic dead link, never a live-looking
//! affordance.
//!
//! The core honesty axes are three, mirroring the row acceptance criteria.
//!
//! 1. **A seeded archived object can transition into Expired or Removed while still presenting metadata,
//!    provenance, and the correct cleanup / removal explanation.** Each binding carries an explicit
//!    [`ArchivedEvidenceState`] — [`ArchivedEvidenceState::PreservedAvailable`],
//!    [`ArchivedEvidenceState::Expired`], [`ArchivedEvidenceState::Removed`],
//!    [`ArchivedEvidenceState::RetentionWindowEnded`], [`ArchivedEvidenceState::MissingLiveTarget`], or
//!    [`ArchivedEvidenceState::MetadataOnly`] — with a stable state label. Every non-available state carries a
//!    [`RemovalExpiryNote`] naming the reason ([`RemovalExpiryReason`]), a never-omitted explanation, the
//!    preserved-metadata note, and the removal attribution.
//! 2. **No claimed archive consumer degrades to a generic dead-link state when the product can still explain
//!    expiry / removal.** When the content bytes are gone, the binding still renders the historical grammar
//!    (snapshot label, capture time, provenance, mutation-blocked posture) plus the removal / expiry reason, and
//!    the `degrades_to_generic_dead_link` guardrail must be `false`.
//! 3. **Export / support packets preserve the same expired / removed vocabulary used in the product UI.** The
//!    packet references upstream historical-reference contracts by id, keeps the removal / expiry state tokens in
//!    the support export and matrix CSV, and joins each removal outcome to a retention / deletion receipt, a
//!    retirement closure ledger, and a support packet manifest so removal outcomes remain attributable.
//!
//! Every binding names the accessibility routes ([`M5HistoricalReferenceAccessibilityRoute`]) through which the
//! archived state, its provenance, and its removal / expiry reason can be discovered without pointer-only
//! chrome; keyboard focus and screen-reader announcement are mandatory. The historical side stays visibly
//! non-live and mutation blocked throughout, and the historical-side grammar
//! ([`ArchiveStateHistoricalGrammar`]) is identical across every surface that renders the same profile.
//!
//! The boundary schema is
//! [`schemas/program/m5-archived-object-expiry-removal-state-and-metadata-fallback.schema.json`](../../../../schemas/program/m5-archived-object-expiry-removal-state-and-metadata-fallback.schema.json).
//! The contract doc is
//! [`docs/support/m5_archived_object_expiry_removal_state_and_metadata_fallback.md`](../../../../docs/support/m5_archived_object_expiry_removal_state_and_metadata_fallback.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-archived-evidence-state/`](../../../../fixtures/recovery/m5-archived-evidence-state/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_archived_evidence_state, seeded_m5_archived_evidence_state_expired_narrowed,
    seeded_m5_archived_evidence_state_removed_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ArchivedEvidenceStatePacket`].
pub const M5_ARCHIVED_EVIDENCE_STATE_RECORD_KIND: &str = "m5_archived_evidence_state_registry";

/// Schema version for archived-evidence-state records.
pub const M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_ARCHIVED_EVIDENCE_STATE_PACKET_ID: &str = "m5-archived-evidence-state:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_REF: &str =
    "schemas/program/m5-archived-object-expiry-removal-state-and-metadata-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ARCHIVED_EVIDENCE_STATE_DOC_REF: &str =
    "docs/support/m5_archived_object_expiry_removal_state_and_metadata_fallback.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ARCHIVED_EVIDENCE_STATE_ARTIFACT_REF: &str =
    "artifacts/support/m5-archived-evidence-state/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_ARCHIVED_EVIDENCE_STATE_CSV_REF: &str =
    "artifacts/support/m5-archived-evidence-state/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ARCHIVED_EVIDENCE_STATE_REPORT_REF: &str =
    "artifacts/support/m5-archived-evidence-state/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ARCHIVED_EVIDENCE_STATE_FIXTURE_DIR: &str =
    "fixtures/recovery/m5-archived-evidence-state";

/// Proof-freshness SLO in hours for this lane.
pub const M5_ARCHIVED_EVIDENCE_STATE_PROOF_SLO_HOURS: u32 = 720;

/// Mutation-blocked-posture sentinel words a historical-side grammar may never fall back to; an archived object
/// whose historical role must be present before surfacing as non-live evidence must always keep a real
/// mutation-blocked posture rather than implying the object is editable, live, writable, or the current object.
const MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5HistoricalReferenceConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5HistoricalReferenceConsumerSurface::Support
            | M5HistoricalReferenceConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5HistoricalReferenceRole`] vocabulary.
pub fn is_known_historical_reference_role_token(token: &str) -> bool {
    historical_reference_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5HistoricalReferenceRole`], if it is one.
pub fn historical_reference_role_from_token(token: &str) -> Option<M5HistoricalReferenceRole> {
    M5HistoricalReferenceRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// The explicit lifecycle state a preserved / archived object holds once its content or live target changes.
///
/// The state governs the discoverable action set, parity, and removal / expiry disclosure — never the
/// historical-side grammar: an expired or removed object still carries the same historical-role, snapshot-label,
/// capture-time, provenance, and mutation-blocked-posture words and discloses its state through an explicit
/// removal / expiry note plus a metadata / remove fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedEvidenceState {
    /// The archived object's content bytes and metadata are both preserved; its live target may still be opened.
    PreservedAvailable,
    /// The archived object's validity / retention window has elapsed; its bytes are pending cleanup and it may
    /// be safely removed, but its metadata, provenance, and expiry reason stay presented.
    Expired,
    /// The archived object's content bytes have been removed; only its metadata, provenance, and deletion
    /// receipt remain, so it never dead-links.
    Removed,
    /// The archived object's retention window has ended; it is eligible for a reviewed cleanup / remove, while
    /// its metadata and provenance stay presented.
    RetentionWindowEnded,
    /// The current live object the archive referenced no longer exists; the archived metadata stays presented,
    /// and no open-current-live-object action is offered.
    MissingLiveTarget,
    /// Only the archived object's metadata is retained by design; its content was never or is no longer
    /// available, so it presents metadata rather than a blank pane.
    MetadataOnly,
}

impl ArchivedEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreservedAvailable,
        Self::Expired,
        Self::Removed,
        Self::RetentionWindowEnded,
        Self::MissingLiveTarget,
        Self::MetadataOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedAvailable => "preserved_available",
            Self::Expired => "expired",
            Self::Removed => "removed",
            Self::RetentionWindowEnded => "retention_window_ended",
            Self::MissingLiveTarget => "missing_live_target",
            Self::MetadataOnly => "metadata_only",
        }
    }

    /// A stable, human-facing default label for the state (a binding may carry the same wording).
    pub const fn stable_label(self) -> &'static str {
        match self {
            Self::PreservedAvailable => "Preserved / available (archived)",
            Self::Expired => "Expired (retention window elapsed)",
            Self::Removed => "Removed (content deleted; metadata retained)",
            Self::RetentionWindowEnded => "Retention window ended (eligible for cleanup)",
            Self::MissingLiveTarget => "Missing live target (archive preserved)",
            Self::MetadataOnly => "Metadata only (content unavailable)",
        }
    }

    /// Whether this state still presents an available, non-expired archive with an openable live target.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::PreservedAvailable)
    }

    /// Whether this state discloses a removal / expiry rather than an available archive.
    pub const fn discloses_removal_or_expiry(self) -> bool {
        !self.is_available()
    }

    /// The removal / expiry reasons this state is allowed to name. An available archive names none; every
    /// removal / expiry state must name exactly one reason from its allowed set.
    pub const fn allowed_removal_reasons(self) -> &'static [RemovalExpiryReason] {
        match self {
            Self::PreservedAvailable => &[],
            Self::Expired => &[
                RemovalExpiryReason::RetentionWindowElapsed,
                RemovalExpiryReason::StorageReclaimed,
            ],
            Self::Removed => &[
                RemovalExpiryReason::ManualCleanupRequested,
                RemovalExpiryReason::PolicyMandatedDeletion,
                RemovalExpiryReason::StorageReclaimed,
                RemovalExpiryReason::LegalHoldReleased,
            ],
            Self::RetentionWindowEnded => &[
                RemovalExpiryReason::RetentionWindowElapsed,
                RemovalExpiryReason::PolicyMandatedDeletion,
            ],
            Self::MissingLiveTarget => &[RemovalExpiryReason::SourceLiveTargetRemoved],
            Self::MetadataOnly => &[RemovalExpiryReason::MetadataOnlyByDesign],
        }
    }
}

/// The action an archived-state surface may expose.
///
/// The set is deliberately closed and analysis / cleanup only: there is no apply / sync / restore action.
/// `RemoveArchivedObject` appears only where a safe reviewed cleanup is appropriate, and `OpenCurrentLiveObject`
/// appears only when the archive is preserved with a live target, so an archived-state surface can never present
/// an expired or removed object as if it were live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStateAction {
    /// Inspect the preserved archived-object metadata only.
    InspectMetadata,
    /// Export the archived-evidence-state record.
    ExportEvidence,
    /// Remove the archived object through a reviewed cleanup — only where appropriate.
    RemoveArchivedObject,
    /// Open the current live object — only when the archive is preserved and its live target exists.
    OpenCurrentLiveObject,
}

impl ArchiveStateAction {
    /// The metadata-only base action set present on every archived-state surface.
    pub const BASE: [Self; 2] = [Self::InspectMetadata, Self::ExportEvidence];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectMetadata => "inspect_metadata",
            Self::ExportEvidence => "export_evidence",
            Self::RemoveArchivedObject => "remove_archived_object",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// Why an archived object expired, was removed, ended its retention window, lost its live target, or is
/// metadata-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemovalExpiryReason {
    /// The retention / validity window elapsed.
    RetentionWindowElapsed,
    /// A manual cleanup was requested and reviewed.
    ManualCleanupRequested,
    /// A policy mandated deletion of the content.
    PolicyMandatedDeletion,
    /// The source live object was removed, so the archive has no live counterpart.
    SourceLiveTargetRemoved,
    /// Storage was reclaimed while the metadata / receipt was retained.
    StorageReclaimed,
    /// A legal hold was released, allowing cleanup.
    LegalHoldReleased,
    /// Only metadata was ever retained for this object by design.
    MetadataOnlyByDesign,
}

impl RemovalExpiryReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RetentionWindowElapsed,
        Self::ManualCleanupRequested,
        Self::PolicyMandatedDeletion,
        Self::SourceLiveTargetRemoved,
        Self::StorageReclaimed,
        Self::LegalHoldReleased,
        Self::MetadataOnlyByDesign,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetentionWindowElapsed => "retention_window_elapsed",
            Self::ManualCleanupRequested => "manual_cleanup_requested",
            Self::PolicyMandatedDeletion => "policy_mandated_deletion",
            Self::SourceLiveTargetRemoved => "source_live_target_removed",
            Self::StorageReclaimed => "storage_reclaimed",
            Self::LegalHoldReleased => "legal_hold_released",
            Self::MetadataOnlyByDesign => "metadata_only_by_design",
        }
    }

    /// Whether this reason is allowed for the given state (the state's disclosure supports it), so the packet
    /// cannot name a reason that does not match the state it is disclosing.
    pub fn supported_by(self, state: ArchivedEvidenceState) -> bool {
        state.allowed_removal_reasons().contains(&self)
    }
}

/// The next action a removal / expiry note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemovalExpiryNextAction {
    /// Remove the archived object through its reviewed cleanup path.
    RemoveThroughReviewedCleanup,
    /// Inspect the archived metadata only when no removable content remains.
    InspectMetadataOnly,
}

impl RemovalExpiryNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoveThroughReviewedCleanup => "remove_through_reviewed_cleanup",
            Self::InspectMetadataOnly => "inspect_metadata_only",
        }
    }
}

/// Whether a binding presents an available archive or discloses a removal / expiry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStateParity {
    /// The archive is preserved / available and shown as such.
    ArchiveStatePresented,
    /// Historical grammar is preserved and a removal / expiry state is explicitly disclosed.
    RemovalOrExpiryDisclosed,
}

impl ArchiveStateParity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchiveStatePresented => "archive_state_presented",
            Self::RemovalOrExpiryDisclosed => "removal_or_expiry_disclosed",
        }
    }
}

/// Downgrade trigger that can narrow this archived-state lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedEvidenceStateDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Historical grammar drifted between surfaces for the same profile.
    StateGrammarDriftDetected,
    /// A historical side dropped its mutation-blocked posture and began to imply the object is live.
    MutationBlockedPostureDropped,
    /// A surface reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A surface degraded to a generic dead-link when it could still explain expiry / removal.
    DegradesToGenericDeadLink,
    /// A surface removed content without joining the outcome to a receipt, ledger, and manifest.
    RemovesContentWithoutAttribution,
    /// A surface presented an expired or removed object as if it were live or current.
    PresentsExpiredOrRemovedAsLiveOrCurrent,
    /// An export dropped the removal / expiry vocabulary the product UI uses.
    DropsRemovalOrExpiryVocabularyInExport,
    /// An accessibility route for the archived state, provenance, or removal / expiry reason was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl ArchivedEvidenceStateDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::StateGrammarDriftDetected,
        Self::MutationBlockedPostureDropped,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::DegradesToGenericDeadLink,
        Self::RemovesContentWithoutAttribution,
        Self::PresentsExpiredOrRemovedAsLiveOrCurrent,
        Self::DropsRemovalOrExpiryVocabularyInExport,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::StateGrammarDriftDetected => "state_grammar_drift_detected",
            Self::MutationBlockedPostureDropped => "mutation_blocked_posture_dropped",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DegradesToGenericDeadLink => "degrades_to_generic_dead_link",
            Self::RemovesContentWithoutAttribution => "removes_content_without_attribution",
            Self::PresentsExpiredOrRemovedAsLiveOrCurrent => {
                "presents_expired_or_removed_as_live_or_current"
            }
            Self::DropsRemovalOrExpiryVocabularyInExport => {
                "drops_removal_or_expiry_vocabulary_in_export"
            }
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled historical-side grammar a preserved-object profile presents.
///
/// These five words describe the historical (non-live) side of the archived object and must be identical across
/// every consumer surface that shows the same profile. The historical-role word must be a frozen
/// [`M5HistoricalReferenceRole`] token; the rest are controlled words the archive carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveStateHistoricalGrammar {
    /// Historical-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub historical_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the archive is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the archive is attributed to.
    pub provenance_word: String,
    /// The mutation-blocked-posture word (read-only, non-authoritative-for-mutation).
    pub mutation_blocked_posture_word: String,
}

impl ArchiveStateHistoricalGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.historical_role_word.trim().is_empty()
            && !self.snapshot_label_word.trim().is_empty()
            && !self.capture_time_word.trim().is_empty()
            && !self.provenance_word.trim().is_empty()
            && !self.mutation_blocked_posture_word.trim().is_empty()
    }

    /// Whether the historical-role word is a member of the frozen role vocabulary.
    pub fn historical_role_word_in_vocabulary(&self) -> bool {
        is_known_historical_reference_role_token(self.historical_role_word.trim())
    }

    /// Whether the capture-time and provenance words that keep the object from dead-linking are both present.
    pub fn capture_context_present(&self) -> bool {
        !self.capture_time_word.trim().is_empty() && !self.provenance_word.trim().is_empty()
    }

    /// Whether the profile honours the mutation-blocked rule: a historical-side role that must be present before
    /// the object may be surfaced as non-live evidence must pair it with a real mutation-blocked posture word and
    /// never collapse to an editable / live / writable / current-object sentinel.
    pub fn mutation_blocked_posture_satisfied(&self) -> bool {
        match historical_reference_role_from_token(self.historical_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_non_live_evidence() => {
                let posture = self.mutation_blocked_posture_word.trim().to_lowercase();
                !posture.is_empty()
                    && !MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS.contains(&posture.as_str())
            }
            _ => true,
        }
    }
}

/// The join that keeps a removal / expiry outcome attributable: a retention / deletion receipt, the retirement
/// closure ledger, and the support packet manifest the removal is recorded against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemovalAttribution {
    /// Stable id / ref of the retention or deletion receipt.
    pub retention_or_deletion_receipt_ref: String,
    /// Stable id / ref of the retirement closure ledger entry.
    pub retirement_closure_ledger_ref: String,
    /// Stable id / ref of the support packet manifest.
    pub support_packet_manifest_ref: String,
}

impl RemovalAttribution {
    /// Whether every attribution ref is present, so the removal outcome is fully attributable.
    pub fn all_present(&self) -> bool {
        !self.retention_or_deletion_receipt_ref.trim().is_empty()
            && !self.retirement_closure_ledger_ref.trim().is_empty()
            && !self.support_packet_manifest_ref.trim().is_empty()
    }
}

/// The explicit note an expired / removed / retention-ended / missing-live-target / metadata-only binding shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemovalExpiryNote {
    /// Why the object expired / was removed.
    pub reason: RemovalExpiryReason,
    /// A never-omitted explanation of the exact expiry / removal outcome.
    pub explanation: String,
    /// Note that the metadata, provenance, and capture context stay preserved (never omitted).
    pub preserved_metadata_note: String,
    /// The join that keeps the removal outcome attributable.
    pub removal_attribution: RemovalAttribution,
    /// The next action offered.
    pub next_action: RemovalExpiryNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures an archived-state binding must carry, derived from its state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveStateRenderDisclosure {
    /// The parity state the state requires.
    pub parity_state: ArchiveStateParity,
    /// Whether the binding must carry an explicit removal / expiry note.
    pub needs_removal_note: bool,
    /// The next action the removal / expiry note must offer, if any.
    pub removal_next_action: Option<RemovalExpiryNextAction>,
    /// Whether the binding offers a reviewed remove action.
    pub offers_remove_action: bool,
    /// Whether the binding offers an open-current-live-object action.
    pub offers_open_live_target: bool,
    /// Whether the archived object's content bytes are still present in this state.
    pub expects_content_bytes_present: bool,
}

/// Resolves the render disclosures an archived-state binding must carry from its state.
///
/// An available archive renders the base action set plus an open-current-live-object action and keeps its
/// content bytes. A removal / expiry state narrows the actions, discloses the state through an explicit note
/// plus a remove-through-reviewed-cleanup or inspect-metadata-only fallback, and — when its bytes are gone —
/// still renders capture time, provenance, and reason instead of a dead link. All keep every historical grammar
/// word.
pub const fn resolve_archive_state_render_disclosure(
    state: ArchivedEvidenceState,
) -> ArchiveStateRenderDisclosure {
    match state {
        ArchivedEvidenceState::PreservedAvailable => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::ArchiveStatePresented,
            needs_removal_note: false,
            removal_next_action: None,
            offers_remove_action: false,
            offers_open_live_target: true,
            expects_content_bytes_present: true,
        },
        ArchivedEvidenceState::Expired => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::RemovalOrExpiryDisclosed,
            needs_removal_note: true,
            removal_next_action: Some(RemovalExpiryNextAction::RemoveThroughReviewedCleanup),
            offers_remove_action: true,
            offers_open_live_target: false,
            expects_content_bytes_present: true,
        },
        ArchivedEvidenceState::RetentionWindowEnded => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::RemovalOrExpiryDisclosed,
            needs_removal_note: true,
            removal_next_action: Some(RemovalExpiryNextAction::RemoveThroughReviewedCleanup),
            offers_remove_action: true,
            offers_open_live_target: false,
            expects_content_bytes_present: true,
        },
        ArchivedEvidenceState::Removed => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::RemovalOrExpiryDisclosed,
            needs_removal_note: true,
            removal_next_action: Some(RemovalExpiryNextAction::InspectMetadataOnly),
            offers_remove_action: false,
            offers_open_live_target: false,
            expects_content_bytes_present: false,
        },
        ArchivedEvidenceState::MissingLiveTarget => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::RemovalOrExpiryDisclosed,
            needs_removal_note: true,
            removal_next_action: Some(RemovalExpiryNextAction::InspectMetadataOnly),
            offers_remove_action: false,
            offers_open_live_target: false,
            expects_content_bytes_present: true,
        },
        ArchivedEvidenceState::MetadataOnly => ArchiveStateRenderDisclosure {
            parity_state: ArchiveStateParity::RemovalOrExpiryDisclosed,
            needs_removal_note: true,
            removal_next_action: Some(RemovalExpiryNextAction::InspectMetadataOnly),
            offers_remove_action: false,
            offers_open_live_target: false,
            expects_content_bytes_present: false,
        },
    }
}

/// One archived-state binding: a preserved-object class in one lifecycle state on one consumer surface for one
/// preserved-object profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedEvidenceStateBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable preserved-object-profile id (shared across surfaces that show the same profile).
    pub snapshot_profile_id: String,
    /// Human-readable preserved-object-profile identity.
    pub snapshot_profile_label: String,
    /// Which preserved-object class this binding carries.
    pub object_class: M5HistoricalReferenceObject,
    /// Which consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// The lifecycle state of this archived object.
    pub state: ArchivedEvidenceState,
    /// A stable, human-facing state label.
    pub state_label: String,
    /// The controlled historical-side grammar presented (identical across surfaces for one profile).
    pub historical_grammar: ArchiveStateHistoricalGrammar,
    /// Whether the archived object's content bytes are still present.
    pub content_bytes_present: bool,
    /// Whether an available archive is presented or a removal / expiry is disclosed.
    pub parity_state: ArchiveStateParity,
    /// The discoverable action set allowed on this archived-state surface.
    pub allowed_actions: Vec<ArchiveStateAction>,
    /// The accessibility routes through which the archived state, provenance, and removal / expiry reason can be
    /// discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The explicit removal / expiry note; required and complete when the state discloses a removal / expiry.
    pub removal_note: Option<RemovalExpiryNote>,
    /// The historical side stays mutation blocked. MUST be `true`.
    pub historical_side_mutation_blocked: bool,
    /// Guardrail: this surface reopens a live target without validating identity, trust, route, and authority.
    /// MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this surface degrades to a generic dead-link when it could still explain expiry / removal.
    /// MUST be `false`.
    pub degrades_to_generic_dead_link: bool,
    /// Guardrail: this surface removes content without joining the outcome to a receipt, ledger, and manifest.
    /// MUST be `false`.
    pub removes_content_without_attribution: bool,
    /// Guardrail: this surface presents an expired or removed object as if it were live or current. MUST be
    /// `false`.
    pub presents_expired_or_removed_as_live_or_current: bool,
    /// Guardrail: this surface drops the removal / expiry vocabulary in its export. MUST be `false`.
    pub drops_removal_or_expiry_vocabulary_in_export: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ArchivedEvidenceStateBinding {
    /// Disclosures this binding must carry, derived from its state.
    pub const fn disclosure(&self) -> ArchiveStateRenderDisclosure {
        resolve_archive_state_render_disclosure(self.state)
    }

    /// Whether this binding discloses a removal / expiry.
    pub const fn discloses_removal_or_expiry(&self) -> bool {
        self.state.discloses_removal_or_expiry()
    }

    /// Whether every guardrail row-invariant holds (historical side mutation blocked, all guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.historical_side_mutation_blocked
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.degrades_to_generic_dead_link
            && !self.removes_content_without_attribution
            && !self.presents_expired_or_removed_as_live_or_current
            && !self.drops_removal_or_expiry_vocabulary_in_export
    }

    /// Whether the metadata-only base action set is present.
    pub fn has_base_actions(&self) -> bool {
        ArchiveStateAction::BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no apply / sync affordance leaked in (structurally guaranteed by the closed action enum, but
    /// checked so the invariant is explicit).
    pub fn action_set_is_closed(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                ArchiveStateAction::InspectMetadata
                    | ArchiveStateAction::ExportEvidence
                    | ArchiveStateAction::RemoveArchivedObject
                    | ArchiveStateAction::OpenCurrentLiveObject
            )
        })
    }

    /// Whether the remove action is present exactly when the state offers it.
    pub fn remove_action_matches_state(&self) -> bool {
        let offered = self.disclosure().offers_remove_action;
        let present = self
            .allowed_actions
            .contains(&ArchiveStateAction::RemoveArchivedObject);
        offered == present
    }

    /// Whether the open-current-live-object action is present exactly when the state (an available archive)
    /// offers it.
    pub fn open_live_action_matches_state(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&ArchiveStateAction::OpenCurrentLiveObject);
        offered == present
    }

    /// Whether the content-bytes flag matches what the state expects.
    pub fn content_presence_matches_state(&self) -> bool {
        self.content_bytes_present == self.disclosure().expects_content_bytes_present
    }

    /// Whether, when the content bytes are gone, the binding still renders capture time, provenance, and a
    /// removal / expiry reason instead of degrading to a dead link.
    pub fn renders_metadata_instead_of_dead_link(&self) -> bool {
        if self.content_bytes_present {
            return true;
        }
        self.historical_grammar.capture_context_present()
            && matches!(&self.removal_note, Some(note) if !note.explanation.trim().is_empty())
            && !self.degrades_to_generic_dead_link
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5HistoricalReferenceAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5HistoricalReferenceAccessibilityRoute::ScreenReaderAnnounced)
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
                .any(|reference| reference == M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedEvidenceStateTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same historical grammar across surfaces.
    pub same_profile_same_historical_grammar_across_surfaces: bool,
    /// Every historical-role word is a frozen role token.
    pub historical_role_words_stay_in_frozen_vocabulary: bool,
    /// A historical side's mutation-blocked posture never masquerades as a live, writable, or current object.
    pub mutation_blocked_posture_never_masquerades_as_live: bool,
    /// Every non-available state carries an explicit removal / expiry explanation.
    pub every_non_available_state_carries_removal_or_expiry_explanation: bool,
    /// Metadata, provenance, and reason render instead of a generic dead link when bytes are gone.
    pub metadata_provenance_and_reason_render_instead_of_dead_link: bool,
    /// Removal outcomes are joined to retention / deletion receipts, closure ledgers, and support manifests.
    pub removal_outcomes_joined_to_receipts_ledgers_and_manifests: bool,
    /// A remove action is offered only where a reviewed cleanup is appropriate.
    pub remove_action_offered_only_where_appropriate: bool,
    /// An expired or removed object is never presented as live or current.
    pub expired_or_removed_never_presented_as_live_or_current: bool,
    /// Stable state labels are used across surfaces.
    pub stable_state_labels_used_across_surfaces: bool,
    /// Accessibility routes for the archived state, provenance, and removal / expiry reason are present.
    pub accessibility_routes_present_for_state_provenance_and_reason: bool,
    /// State disclosure spans available, expired, removed, retention-ended, missing-target, and metadata-only.
    pub state_disclosed_across_all_lifecycle_states: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ArchivedEvidenceStateTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_historical_grammar_across_surfaces
            && self.historical_role_words_stay_in_frozen_vocabulary
            && self.mutation_blocked_posture_never_masquerades_as_live
            && self.every_non_available_state_carries_removal_or_expiry_explanation
            && self.metadata_provenance_and_reason_render_instead_of_dead_link
            && self.removal_outcomes_joined_to_receipts_ledgers_and_manifests
            && self.remove_action_offered_only_where_appropriate
            && self.expired_or_removed_never_presented_as_live_or_current
            && self.stable_state_labels_used_across_surfaces
            && self.accessibility_routes_present_for_state_provenance_and_reason
            && self.state_disclosed_across_all_lifecycle_states
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedEvidenceStateProjection {
    /// The shell / archive-viewer surface consumes the shared archived-state packet.
    pub shell_consumes_state: bool,
    /// The help / docs surface consumes the shared archived-state packet.
    pub help_docs_consumes_state: bool,
    /// The support bundle viewer consumes the shared archived-state packet.
    pub support_consumes_state: bool,
    /// The review / incident surface consumes the shared archived-state packet.
    pub review_incident_consumes_state: bool,
    /// The runbook-archive surface consumes the shared archived-state packet.
    pub runbook_archive_consumes_state: bool,
    /// The release-center retirement snapshot page consumes the shared archived-state packet.
    pub release_center_consumes_state: bool,
    /// The companion / export path consumes the shared archived-state packet.
    pub companion_export_consumes_state: bool,
    /// The program-governance review consumes the shared archived-state packet.
    pub program_governance_consumes_state: bool,
    /// The CLI / export path consumes the shared archived-state packet.
    pub cli_export_consumes_state: bool,
    /// Every object class is stated by two or more consumers.
    pub every_object_class_stated_by_two_or_more_consumers: bool,
    /// Historical grammar is identical for the same profile.
    pub historical_grammar_identical_for_same_profile: bool,
    /// Removal / expiry is disclosed rather than hidden.
    pub removal_or_expiry_disclosed_not_hidden: bool,
    /// Export maps a state row back to one historical-reference object class.
    pub state_maps_back_to_one_historical_reference_object: bool,
}

impl ArchivedEvidenceStateProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_state
            && self.help_docs_consumes_state
            && self.support_consumes_state
            && self.review_incident_consumes_state
            && self.runbook_archive_consumes_state
            && self.release_center_consumes_state
            && self.companion_export_consumes_state
            && self.program_governance_consumes_state
            && self.cli_export_consumes_state
            && self.every_object_class_stated_by_two_or_more_consumers
            && self.historical_grammar_identical_for_same_profile
            && self.removal_or_expiry_disclosed_not_hidden
            && self.state_maps_back_to_one_historical_reference_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedEvidenceStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ArchivedEvidenceStatePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ArchivedEvidenceStatePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Archived-state bindings.
    pub state_bindings: Vec<ArchivedEvidenceStateBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArchivedEvidenceStateDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArchivedEvidenceStateTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArchivedEvidenceStateProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchivedEvidenceStateProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe archived-evidence-state packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArchivedEvidenceStatePacket {
    /// Record kind; must equal [`M5_ARCHIVED_EVIDENCE_STATE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Archived-state bindings.
    pub state_bindings: Vec<ArchivedEvidenceStateBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArchivedEvidenceStateDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArchivedEvidenceStateTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArchivedEvidenceStateProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchivedEvidenceStateProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ArchivedEvidenceStatePacket {
    /// Builds an archived-evidence-state packet from stable-lane input.
    pub fn new(input: M5ArchivedEvidenceStatePacketInput) -> Self {
        Self {
            record_kind: M5_ARCHIVED_EVIDENCE_STATE_RECORD_KIND.to_owned(),
            schema_version: M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            state_bindings: input.state_bindings,
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

    /// Validates the archived-evidence-state invariants.
    pub fn validate(&self) -> Vec<M5ArchivedEvidenceStateViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ARCHIVED_EVIDENCE_STATE_RECORD_KIND {
            violations.push(M5ArchivedEvidenceStateViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_VERSION {
            violations.push(M5ArchivedEvidenceStateViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ArchivedEvidenceStateViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ArchivedEvidenceStateViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ArchivedEvidenceStateViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ArchivedEvidenceStateViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ArchivedEvidenceStateViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ArchivedEvidenceStateViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("archived-state packet serializes"),
        ) {
            violations.push(M5ArchivedEvidenceStateViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("archived-state packet serializes")
    }

    /// Deterministic matrix CSV, one row per archived-state binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,consumer,state,content_bytes_present,removal_reason,parity_state,state_label\n",
        );
        for binding in &self.state_bindings {
            let reason = binding
                .removal_note
                .as_ref()
                .map(|note| note.reason.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.state.as_str(),
                binding.content_bytes_present,
                reason,
                binding.parity_state.as_str(),
                binding.state_label.replace(',', ";"),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let disclosed = self
            .state_bindings
            .iter()
            .filter(|binding| binding.discloses_removal_or_expiry())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Archived-Object Expiry / Removal State: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- State bindings: {} ({} disclosing removal / expiry)\n",
            self.state_bindings.len(),
            disclosed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## State bindings\n\n");
        for binding in &self.state_bindings {
            let reason = binding
                .removal_note
                .as_ref()
                .map(|note| note.reason.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, state `{}`, content-present `{}`, reason `{}`, role `{}`\n",
                binding.snapshot_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.state.as_str(),
                binding.content_bytes_present,
                reason,
                binding.historical_grammar.historical_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in archived-state export.
#[derive(Debug)]
pub enum M5ArchivedEvidenceStateArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ArchivedEvidenceStateViolation>),
}

impl fmt::Display for M5ArchivedEvidenceStateArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "archived-state export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "archived-state export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ArchivedEvidenceStateArtifactError {}

/// Validation failures emitted by [`M5ArchivedEvidenceStatePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ArchivedEvidenceStateViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No archived-state bindings are present.
    StateBindingsMissing,
    /// An archived-state binding is incomplete.
    BindingIncomplete,
    /// A binding's historical grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's historical-role word is not a frozen role token.
    HistoricalRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its mutation-blocked posture.
    MutationBlockedPostureMissingForGateRole,
    /// A binding's parity state does not match its state.
    ParityStateMismatch,
    /// A binding's content-bytes-present flag does not match its state.
    ContentPresenceMismatch,
    /// Two surfaces show the same profile with different historical grammar.
    StateGrammarDriftAcrossSurfaces,
    /// A shared object class is not stated by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A binding is missing a stable state label.
    StateLabelMissing,
    /// A removal / expiry state is missing its explicit removal / expiry note.
    RemovalNoteMissing,
    /// A removal / expiry note's reason is not allowed for the state.
    RemovalReasonNotAllowedForState,
    /// A removal / expiry note's next action does not match the required next action.
    RemovalNextActionMismatch,
    /// A removal / expiry note's attribution join is incomplete.
    RemovalAttributionIncomplete,
    /// A removal / expiry note is missing its explanation.
    RemovalExplanationMissing,
    /// A removal / expiry note is missing its preserved-metadata note.
    RemovalPreservedMetadataNoteMissing,
    /// A removal / expiry note is missing its next-action copy.
    RemovalNextActionLabelMissing,
    /// An available-archive binding carries a removal / expiry note it must not.
    UnexpectedRemovalNote,
    /// A binding is missing the metadata-only base action set.
    BaseActionsMissing,
    /// A binding's action set is not the closed archived-state action set.
    ActionSetNotClosed,
    /// A binding's remove action does not match its state.
    RemoveActionStateMismatch,
    /// A binding's open-current-live-object action does not match its state.
    OpenLiveActionStateMismatch,
    /// A binding whose content bytes are gone degrades to a generic dead link.
    MetadataFallbackMissing,
    /// A binding cannot discover its archived state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's historical side is not mutation blocked.
    HistoricalSideNotMutationBlocked,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding degrades to a generic dead-link state (guardrail form).
    DegradesToGenericDeadLink,
    /// A binding removes content without joining the outcome to a receipt, ledger, and manifest.
    RemovesContentWithoutAttribution,
    /// A binding presents an expired or removed object as if it were live or current.
    PresentsExpiredOrRemovedAsLiveOrCurrent,
    /// A binding drops the removal / expiry vocabulary in its export.
    DropsRemovalOrExpiryVocabularyInExport,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every lifecycle state appears among the bindings.
    StateCoverageMissing,
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

impl M5ArchivedEvidenceStateViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::StateBindingsMissing => "state_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::HistoricalRoleWordOutsideVocabulary => "historical_role_word_outside_vocabulary",
            Self::MutationBlockedPostureMissingForGateRole => {
                "mutation_blocked_posture_missing_for_gate_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ContentPresenceMismatch => "content_presence_mismatch",
            Self::StateGrammarDriftAcrossSurfaces => "state_grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::StateLabelMissing => "state_label_missing",
            Self::RemovalNoteMissing => "removal_note_missing",
            Self::RemovalReasonNotAllowedForState => "removal_reason_not_allowed_for_state",
            Self::RemovalNextActionMismatch => "removal_next_action_mismatch",
            Self::RemovalAttributionIncomplete => "removal_attribution_incomplete",
            Self::RemovalExplanationMissing => "removal_explanation_missing",
            Self::RemovalPreservedMetadataNoteMissing => "removal_preserved_metadata_note_missing",
            Self::RemovalNextActionLabelMissing => "removal_next_action_label_missing",
            Self::UnexpectedRemovalNote => "unexpected_removal_note",
            Self::BaseActionsMissing => "base_actions_missing",
            Self::ActionSetNotClosed => "action_set_not_closed",
            Self::RemoveActionStateMismatch => "remove_action_state_mismatch",
            Self::OpenLiveActionStateMismatch => "open_live_action_state_mismatch",
            Self::MetadataFallbackMissing => "metadata_fallback_missing",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::HistoricalSideNotMutationBlocked => "historical_side_not_mutation_blocked",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DegradesToGenericDeadLink => "degrades_to_generic_dead_link",
            Self::RemovesContentWithoutAttribution => "removes_content_without_attribution",
            Self::PresentsExpiredOrRemovedAsLiveOrCurrent => {
                "presents_expired_or_removed_as_live_or_current"
            }
            Self::DropsRemovalOrExpiryVocabularyInExport => {
                "drops_removal_or_expiry_vocabulary_in_export"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::StateCoverageMissing => "state_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable archived-state export.
pub fn current_stable_m5_archived_evidence_state_export(
) -> Result<M5ArchivedEvidenceStatePacket, M5ArchivedEvidenceStateArtifactError> {
    let packet: M5ArchivedEvidenceStatePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-archived-evidence-state/support_export.json"
    )))
    .map_err(M5ArchivedEvidenceStateArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ArchivedEvidenceStateArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ArchivedEvidenceStatePacket,
    violations: &mut Vec<M5ArchivedEvidenceStateViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_REF,
        M5_ARCHIVED_EVIDENCE_STATE_DOC_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    ];
    // The five object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ArchivedEvidenceStateViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ArchivedEvidenceStatePacket,
    violations: &mut Vec<M5ArchivedEvidenceStateViolation>,
) {
    if packet.state_bindings.is_empty() {
        violations.push(M5ArchivedEvidenceStateViolation::StateBindingsMissing);
        return;
    }

    // One vocabulary: the historical grammar must be identical for every binding that renders the same
    // preserved-object profile.
    let mut profile_grammar: BTreeMap<&str, &ArchiveStateHistoricalGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be stated by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();
    let mut seen_states: BTreeSet<ArchivedEvidenceState> = BTreeSet::new();

    for binding in &packet.state_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.snapshot_profile_id.trim().is_empty()
            || binding.snapshot_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ArchivedEvidenceStateViolation::BindingIncomplete);
        }
        if binding.state_label.trim().is_empty() {
            violations.push(M5ArchivedEvidenceStateViolation::StateLabelMissing);
        }
        if !binding.historical_grammar.all_present() {
            violations.push(M5ArchivedEvidenceStateViolation::GrammarFacetIncomplete);
        }
        if !binding
            .historical_grammar
            .historical_role_word_in_vocabulary()
        {
            violations.push(M5ArchivedEvidenceStateViolation::HistoricalRoleWordOutsideVocabulary);
        }
        if !binding
            .historical_grammar
            .mutation_blocked_posture_satisfied()
        {
            violations
                .push(M5ArchivedEvidenceStateViolation::MutationBlockedPostureMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5ArchivedEvidenceStateViolation::ParityStateMismatch);
        }
        if !binding.content_presence_matches_state() {
            violations.push(M5ArchivedEvidenceStateViolation::ContentPresenceMismatch);
        }

        // Removal / expiry disclosure.
        if disclosure.needs_removal_note {
            match &binding.removal_note {
                None => {
                    violations.push(M5ArchivedEvidenceStateViolation::RemovalNoteMissing);
                }
                Some(note) => {
                    if !note.reason.supported_by(binding.state) {
                        violations.push(
                            M5ArchivedEvidenceStateViolation::RemovalReasonNotAllowedForState,
                        );
                    }
                    if Some(note.next_action) != disclosure.removal_next_action {
                        violations
                            .push(M5ArchivedEvidenceStateViolation::RemovalNextActionMismatch);
                    }
                    if !note.removal_attribution.all_present() {
                        violations
                            .push(M5ArchivedEvidenceStateViolation::RemovalAttributionIncomplete);
                    }
                    if note.explanation.trim().is_empty() {
                        violations
                            .push(M5ArchivedEvidenceStateViolation::RemovalExplanationMissing);
                    }
                    if note.preserved_metadata_note.trim().is_empty() {
                        violations.push(
                            M5ArchivedEvidenceStateViolation::RemovalPreservedMetadataNoteMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations
                            .push(M5ArchivedEvidenceStateViolation::RemovalNextActionLabelMissing);
                    }
                }
            }
        } else if binding.removal_note.is_some() {
            violations.push(M5ArchivedEvidenceStateViolation::UnexpectedRemovalNote);
        }

        // Action rules.
        if !binding.has_base_actions() {
            violations.push(M5ArchivedEvidenceStateViolation::BaseActionsMissing);
        }
        if !binding.action_set_is_closed() {
            violations.push(M5ArchivedEvidenceStateViolation::ActionSetNotClosed);
        }
        if !binding.remove_action_matches_state() {
            violations.push(M5ArchivedEvidenceStateViolation::RemoveActionStateMismatch);
        }
        if !binding.open_live_action_matches_state() {
            violations.push(M5ArchivedEvidenceStateViolation::OpenLiveActionStateMismatch);
        }

        // AC2: never degrade to a generic dead link when metadata / provenance / reason can be shown.
        if !binding.renders_metadata_instead_of_dead_link() {
            violations.push(M5ArchivedEvidenceStateViolation::MetadataFallbackMissing);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5ArchivedEvidenceStateViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.historical_side_mutation_blocked {
            violations.push(M5ArchivedEvidenceStateViolation::HistoricalSideNotMutationBlocked);
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5ArchivedEvidenceStateViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.degrades_to_generic_dead_link {
            violations.push(M5ArchivedEvidenceStateViolation::DegradesToGenericDeadLink);
        }
        if binding.removes_content_without_attribution {
            violations.push(M5ArchivedEvidenceStateViolation::RemovesContentWithoutAttribution);
        }
        if binding.presents_expired_or_removed_as_live_or_current {
            violations
                .push(M5ArchivedEvidenceStateViolation::PresentsExpiredOrRemovedAsLiveOrCurrent);
        }
        if binding.drops_removal_or_expiry_vocabulary_in_export {
            violations
                .push(M5ArchivedEvidenceStateViolation::DropsRemovalOrExpiryVocabularyInExport);
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ArchivedEvidenceStateViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.snapshot_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.snapshot_profile_id.as_str(),
                    &binding.historical_grammar,
                );
            }
            Some(existing) => {
                if **existing != binding.historical_grammar && !drift_reported {
                    violations
                        .push(M5ArchivedEvidenceStateViolation::StateGrammarDriftAcrossSurfaces);
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object_class);
        seen_states.insert(binding.state);
    }

    // Coverage: every consumer surface, object class, and lifecycle state must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ArchivedEvidenceStateViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5ArchivedEvidenceStateViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for state in ArchivedEvidenceState::ALL {
        if !seen_states.contains(&state) {
            violations.push(M5ArchivedEvidenceStateViolation::StateCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be stated by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ArchivedEvidenceStateViolation::ObjectClassReuseUnproven);
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
