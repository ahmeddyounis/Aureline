//! Stable record-class registry: hold/delete honesty, chronology, and export
//! bundle vocabulary for managed, enterprise, and support evidence rows.
//!
//! This module provides the single shared vocabulary every surface reads
//! when it needs to answer: what is the outcome of a delete, hold,
//! export, or offboarding operation?  It also provides the types for
//! timezone-aware chronology exports, destruction receipts, and export
//! bundle manifests that include collaboration-specific metadata.
//!
//! ## Invariants
//!
//! - [`HoldEvaluation`] is fail-closed: an [`HoldStatus::UnknownIndeterminate`]
//!   status blocks destructive completion identically to an active hold.
//! - Hold evaluation never grants new read or export rights; those are always
//!   independent of whether a hold is active.
//! - Local-only artifacts are never implied to be held or deleted by managed
//!   controls; rows that carry local-only content must set
//!   `local_only_artifact_note` so support, compliance, and product surfaces
//!   can display an honest boundary.

use serde::{Deserialize, Serialize};

use crate::RecordClassId;

// ── Unified outcome vocabulary ───────────────────────────────────────────────

/// Unified user/admin/support-visible outcome vocabulary for delete, hold,
/// export, and offboarding operations.
///
/// Use this enum as the single source of truth across UI chips, CLI/headless
/// output, support export packets, and admin audit views so the wording stays
/// consistent and diffable.
///
/// `omitted_by_redaction` is provided for cases where a record exists but its
/// content was withheld; callers must not present it as equivalent to
/// `not_found`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordOperationOutcome {
    /// The operation was requested but not yet acknowledged.
    Requested,
    /// The operation is queued and waiting to execute.
    Queued,
    /// A legal or policy hold is blocking the operation.
    BlockedByHold,
    /// The operation completed successfully.
    Completed,
    /// The record is retained by policy and cannot be deleted yet.
    PolicyRetained,
    /// The record is outside the platform's management scope.
    OutsidePlatformScope,
    /// The artifact lives only on a local device; manual capture is required
    /// before a managed export or delete can cover it.
    ManualLocalCaptureRequired,
    /// The operation completed for some records but not all.
    Partial,
    /// No matching record was found.
    NotFound,
    /// The record exists but its content was withheld by redaction policy.
    OmittedByRedaction,
}

impl RecordOperationOutcome {
    /// Returns the stable token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Queued => "queued",
            Self::BlockedByHold => "blocked_by_hold",
            Self::Completed => "completed",
            Self::PolicyRetained => "policy_retained",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::ManualLocalCaptureRequired => "manual_local_capture_required",
            Self::Partial => "partial",
            Self::NotFound => "not_found",
            Self::OmittedByRedaction => "omitted_by_redaction",
        }
    }
}

impl std::fmt::Display for RecordOperationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ── Hold evaluation ──────────────────────────────────────────────────────────

/// Whether hold evaluation is in a planning or execution phase.
///
/// Planning-time evaluation is used by pre-flight checks; execution-time
/// evaluation is the final safety gate before a destructive action proceeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldPhase {
    /// Pre-flight check before the destructive action is scheduled.
    Planning,
    /// Final check immediately before the destructive action executes.
    Execution,
}

/// The resolved hold status for a record or set of records.
///
/// `UnknownIndeterminate` is treated as equivalent to `Active` for
/// the purposes of blocking destructive actions (fail-closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldStatus {
    /// A hold is confirmed active and blocks destructive completion.
    Active,
    /// Hold status could not be determined; treated as active (fail-closed).
    UnknownIndeterminate,
    /// No active hold applies; destructive action may proceed if other checks pass.
    Cleared,
}

/// The scope the hold evaluation covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldScope {
    /// Only the managed copy is covered by this evaluation.
    ManagedOnly,
    /// Only the local-device copy is in scope.
    LocalOnly,
    /// Both managed and local copies are in scope.
    Both,
}

/// Fail-closed hold evaluation result for a record or record set.
///
/// This type answers whether a destructive action may proceed for a given
/// scope.  It does **not** create read or export rights; those are
/// determined independently of hold status.
///
/// # Examples
///
/// ```rust
/// use aureline_records::stabilize_record_class_registry_legal_hold_delete_honesty::{
///     HoldEvaluation, HoldPhase, HoldScope, HoldStatus,
/// };
///
/// let eval = HoldEvaluation {
///     phase: HoldPhase::Execution,
///     status: HoldStatus::UnknownIndeterminate,
///     scope: HoldScope::ManagedOnly,
///     active_hold_refs: vec![],
///     policy_version: "v1.0".to_owned(),
///     local_only_artifact_note: Some(
///         "Local-only artifacts are outside managed hold scope.".to_owned(),
///     ),
/// };
/// assert!(eval.blocks_destructive_action());
/// assert!(!eval.grants_new_read_rights());
/// assert!(!eval.grants_new_export_rights());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldEvaluation {
    /// Whether this is a planning or execution phase evaluation.
    pub phase: HoldPhase,
    /// Resolved hold status for this scope.
    pub status: HoldStatus,
    /// Scope covered by this evaluation.
    pub scope: HoldScope,
    /// Opaque refs for the active holds that matched, if any.
    pub active_hold_refs: Vec<String>,
    /// Policy version used during evaluation.
    pub policy_version: String,
    /// Explicit note when local-only artifacts are in scope but outside
    /// managed hold controls.  Must be non-`None` whenever `scope` includes
    /// local-device content that the platform does not possess.
    pub local_only_artifact_note: Option<String>,
}

impl HoldEvaluation {
    /// Returns `true` when the destructive action must be blocked.
    ///
    /// Fail-closed: both [`HoldStatus::Active`] and
    /// [`HoldStatus::UnknownIndeterminate`] block completion.
    pub fn blocks_destructive_action(&self) -> bool {
        matches!(
            self.status,
            HoldStatus::Active | HoldStatus::UnknownIndeterminate
        )
    }

    /// Returns `false` unconditionally.
    ///
    /// Hold evaluation never grants new read rights.  A hold blocks deletion;
    /// it does not imply that new access to held content is permitted.
    pub const fn grants_new_read_rights(&self) -> bool {
        false
    }

    /// Returns `false` unconditionally.
    ///
    /// Hold evaluation never grants new export rights.  A hold blocks
    /// deletion; it does not imply that the held records become newly
    /// exportable.
    pub const fn grants_new_export_rights(&self) -> bool {
        false
    }
}

// ── Chronology ───────────────────────────────────────────────────────────────

/// A single entry in a timezone-aware chronology export.
///
/// Actor lineage and event IDs are preserved so incident/support packets can
/// attribute events without re-deriving identity from raw logs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyEntry {
    /// Stable, opaque event id.
    pub event_id: String,
    /// Opaque actor reference (never raw identity or credential material).
    pub actor_ref: String,
    /// UTC timestamp in RFC 3339 format.
    pub timestamp_utc: String,
    /// IANA timezone label for the originating clock source, when available.
    pub source_timezone_label: Option<String>,
    /// Event-class token describing the kind of event.
    pub event_class: String,
    /// Record class the event is associated with, if it belongs to one.
    pub record_class_id: Option<RecordClassId>,
    /// Whether this event came from a local-only source and is outside
    /// managed hold or managed delete scope.
    pub is_local_only: bool,
    /// Whether actor lineage is preserved for this entry.  When `false`, the
    /// `actor_ref` is a placeholder and attribution is incomplete.
    pub is_attributed: bool,
}

/// An ordered, timezone-aware chronology export for an incident or support
/// packet.
///
/// Entries are ordered ascending by `timestamp_utc`.  Consumers must not
/// re-sort by local timestamps; the UTC ordering in this export is canonical.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyExport {
    /// Opaque export id.
    pub export_id: String,
    /// Scope selector describing what records are covered.
    pub scope_selector: String,
    /// Policy version used when generating the export.
    pub policy_version: String,
    /// UTC timestamp when this export was generated, in RFC 3339 format.
    pub exported_at: String,
    /// Whether actor lineage is preserved across all entries.
    pub actor_lineage_preserved: bool,
    /// Whether every entry carries a source timezone label.
    pub timezone_aware: bool,
    /// Ordered list of chronology entries (ascending `timestamp_utc`).
    pub entries: Vec<ChronologyEntry>,
}

// ── Export bundle manifest ────────────────────────────────────────────────────

/// A ref record with its resolved operation outcome and optional reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefRecord {
    /// Opaque stable ref id.
    pub ref_id: String,
    /// Resolved outcome for this ref.
    pub outcome: RecordOperationOutcome,
    /// Human-readable reason for the outcome, safe for support/admin views.
    pub reason: Option<String>,
    /// Record class that governs this ref, if known.
    pub record_class_id: Option<RecordClassId>,
}

/// A hash/checksum entry for one ref in a manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefChecksum {
    /// Opaque stable ref id matching a [`RefRecord`].
    pub ref_id: String,
    /// Hash algorithm name (e.g. `"sha256"`).
    pub algorithm: String,
    /// Hex-encoded hash value.
    pub hash: String,
}

/// An omission reason entry describing why a record class was excluded or
/// redacted from an export bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionReason {
    /// Record class that was excluded or redacted.
    pub record_class_id: RecordClassId,
    /// Outcome token explaining why the class was omitted.
    pub outcome: RecordOperationOutcome,
    /// Human-readable detail safe for support/admin views.
    pub detail: Option<String>,
}

/// Collaboration-specific metadata appended to export bundles and destruction
/// receipts to prove what was retained, redacted, denied, or outside scope.
///
/// This is the v22-required metadata for support, compliance, and incident
/// packets involving collaboration evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollabBundleMetadata {
    /// Opaque session ref.
    pub session_ref: String,
    /// Role of the principal whose records are in scope
    /// (e.g. `"host"`, `"participant"`, `"observer"`, `"admin"`).
    pub role: String,
    /// Whether the records cross a guest/host boundary.
    pub guest_boundary: bool,
    /// Opaque ref to the consent envelope governing this session's recording
    /// and retention posture.
    pub consent_envelope_ref: String,
    /// Transcript class tokens present in this bundle
    /// (e.g. `"terminal_transcript"`, `"debug_transcript"`, `"session_recording"`).
    pub transcript_classes: Vec<String>,
    /// Whether local-only artifacts were excluded from this bundle because
    /// they are outside managed hold/delete scope.
    pub local_only_artifacts_excluded: bool,
    /// Explicit note when local-only artifacts were excluded.  Required when
    /// `local_only_artifacts_excluded` is `true`.
    pub local_only_artifact_note: Option<String>,
}

/// A complete export bundle manifest.
///
/// Every export bundle must carry a manifest so support, admin, and compliance
/// consumers can verify scope, redaction, and integrity without re-reading the
/// full bundle payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBundleManifest {
    /// Opaque bundle id.
    pub bundle_id: String,
    /// UTC timestamp when this bundle was generated, in RFC 3339 format.
    pub created_at: String,
    /// Scope selectors that determined what was included.
    pub scope_selectors: Vec<String>,
    /// Earliest UTC timestamp covered by this bundle (RFC 3339), if bounded.
    pub time_range_start: Option<String>,
    /// Latest UTC timestamp covered by this bundle (RFC 3339), if bounded.
    pub time_range_end: Option<String>,
    /// Record classes explicitly included in this bundle.
    pub included_classes: Vec<RecordClassId>,
    /// Record classes explicitly excluded from this bundle.
    pub excluded_classes: Vec<RecordClassId>,
    /// Reasons why specific classes were omitted or redacted.
    pub omission_reasons: Vec<OmissionReason>,
    /// Policy version used when generating the bundle.
    pub policy_version: String,
    /// Hash/checksum entries for refs included in the bundle.
    pub hash_checksum_manifest: Vec<RefChecksum>,
    /// Redaction profile applied (e.g. `"metadata_safe_default"`).
    pub redaction_profile: String,
    /// Opaque signer or verifier reference.
    pub signer_ref: String,
    /// All refs in scope with their resolved outcomes.
    pub refs: Vec<RefRecord>,
    /// Collaboration-specific metadata, when the bundle covers collaboration
    /// evidence.
    pub collab_metadata: Option<CollabBundleMetadata>,
}

// ── Destruction receipt ───────────────────────────────────────────────────────

/// A durable, diffable destruction receipt.
///
/// Destruction receipts replace hand-written deletion assurances for managed,
/// enterprise, and support evidence rows.  Every receipt must be referenced by
/// support/admin evidence rather than duplicated as inline text.
///
/// # Invariants
///
/// - `deleted_refs` contains only refs for which deletion is confirmed.
/// - `held_refs` contains only refs blocked by an active or indeterminate hold.
/// - `retained_refs` contains refs kept by policy that were not deleted.
/// - `outside_scope_refs` contains refs the platform does not possess or manage.
/// - `local_only_not_held_note` must be non-`None` when any local-only
///   artifacts are in scope to prevent false assurances of managed deletion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionReceipt {
    /// Opaque stable receipt id.
    pub receipt_id: String,
    /// Token describing the action that triggered this receipt
    /// (e.g. `"offboarding_delete"`, `"legal_hold_release_delete"`).
    pub executed_action: String,
    /// UTC timestamp when the action executed, in RFC 3339 format.
    pub executed_at: String,
    /// Policy version in effect at execution time.
    pub policy_version: String,
    /// Scope selectors that determined what was in scope.
    pub scope_selectors: Vec<String>,
    /// Record classes included in the scope.
    pub included_classes: Vec<RecordClassId>,
    /// Record classes excluded from the scope.
    pub excluded_classes: Vec<RecordClassId>,
    /// Refs for which deletion was confirmed.
    pub deleted_refs: Vec<RefRecord>,
    /// Refs skipped (not attempted) due to exclusion rules.
    pub skipped_refs: Vec<RefRecord>,
    /// Refs retained by policy (not deleted).
    pub retained_refs: Vec<RefRecord>,
    /// Refs blocked by an active or indeterminate hold.
    pub held_refs: Vec<RefRecord>,
    /// Refs outside the platform's management scope.
    pub outside_scope_refs: Vec<RefRecord>,
    /// Count of confirmed-deleted records.
    pub total_destroyed_count: u64,
    /// Count of policy-retained records.
    pub total_retained_count: u64,
    /// Count of records outside platform scope.
    pub total_outside_scope_count: u64,
    /// Hash/checksum entries for refs in this receipt.
    pub hash_checksum_manifest: Vec<RefChecksum>,
    /// Redaction profile applied to evidence in this receipt.
    pub redaction_profile: String,
    /// Opaque verifier or signer reference.
    pub verifier_ref: String,
    /// Explicit note when local-only artifacts are in scope but outside
    /// managed delete controls.  Must be set whenever local-only content
    /// was in scope to prevent false assurances of managed deletion.
    pub local_only_not_held_note: Option<String>,
    /// Collaboration-specific metadata when the receipt covers collaboration
    /// evidence.
    pub collab_metadata: Option<CollabBundleMetadata>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_evaluation_fail_closed_on_unknown() {
        let eval = HoldEvaluation {
            phase: HoldPhase::Execution,
            status: HoldStatus::UnknownIndeterminate,
            scope: HoldScope::ManagedOnly,
            active_hold_refs: vec![],
            policy_version: "v1.0".to_owned(),
            local_only_artifact_note: None,
        };
        assert!(eval.blocks_destructive_action());
        assert!(!eval.grants_new_read_rights());
        assert!(!eval.grants_new_export_rights());
    }

    #[test]
    fn hold_evaluation_fail_closed_on_active() {
        let eval = HoldEvaluation {
            phase: HoldPhase::Planning,
            status: HoldStatus::Active,
            scope: HoldScope::Both,
            active_hold_refs: vec!["hold:legal-001".to_owned()],
            policy_version: "v1.0".to_owned(),
            local_only_artifact_note: Some(
                "Local-only artifacts are outside managed hold scope.".to_owned(),
            ),
        };
        assert!(eval.blocks_destructive_action());
        assert!(!eval.grants_new_read_rights());
        assert!(!eval.grants_new_export_rights());
    }

    #[test]
    fn hold_evaluation_allows_when_cleared() {
        let eval = HoldEvaluation {
            phase: HoldPhase::Execution,
            status: HoldStatus::Cleared,
            scope: HoldScope::ManagedOnly,
            active_hold_refs: vec![],
            policy_version: "v1.0".to_owned(),
            local_only_artifact_note: None,
        };
        assert!(!eval.blocks_destructive_action());
        assert!(!eval.grants_new_read_rights());
        assert!(!eval.grants_new_export_rights());
    }

    #[test]
    fn record_operation_outcome_tokens_are_stable() {
        assert_eq!(RecordOperationOutcome::Requested.as_str(), "requested");
        assert_eq!(RecordOperationOutcome::Queued.as_str(), "queued");
        assert_eq!(
            RecordOperationOutcome::BlockedByHold.as_str(),
            "blocked_by_hold"
        );
        assert_eq!(RecordOperationOutcome::Completed.as_str(), "completed");
        assert_eq!(
            RecordOperationOutcome::PolicyRetained.as_str(),
            "policy_retained"
        );
        assert_eq!(
            RecordOperationOutcome::OutsidePlatformScope.as_str(),
            "outside_platform_scope"
        );
        assert_eq!(
            RecordOperationOutcome::ManualLocalCaptureRequired.as_str(),
            "manual_local_capture_required"
        );
        assert_eq!(RecordOperationOutcome::Partial.as_str(), "partial");
        assert_eq!(RecordOperationOutcome::NotFound.as_str(), "not_found");
        assert_eq!(
            RecordOperationOutcome::OmittedByRedaction.as_str(),
            "omitted_by_redaction"
        );
    }

    #[test]
    fn destruction_receipt_round_trips() {
        let receipt = DestructionReceipt {
            receipt_id: "receipt:001".to_owned(),
            executed_action: "offboarding_delete".to_owned(),
            executed_at: "2026-06-01T00:00:00Z".to_owned(),
            policy_version: "v1.0".to_owned(),
            scope_selectors: vec!["workspace:test-org".to_owned()],
            included_classes: vec![RecordClassId::CollaborationSessionRecord],
            excluded_classes: vec![RecordClassId::BillingUsageAggregate],
            deleted_refs: vec![RefRecord {
                ref_id: "ref:abc".to_owned(),
                outcome: RecordOperationOutcome::Completed,
                reason: None,
                record_class_id: Some(RecordClassId::CollaborationSessionRecord),
            }],
            skipped_refs: vec![],
            retained_refs: vec![],
            held_refs: vec![],
            outside_scope_refs: vec![RefRecord {
                ref_id: "ref:local-xyz".to_owned(),
                outcome: RecordOperationOutcome::OutsidePlatformScope,
                reason: Some("Local-only artifact not possessed by managed service.".to_owned()),
                record_class_id: None,
            }],
            total_destroyed_count: 1,
            total_retained_count: 0,
            total_outside_scope_count: 1,
            hash_checksum_manifest: vec![RefChecksum {
                ref_id: "ref:abc".to_owned(),
                algorithm: "sha256".to_owned(),
                hash: "deadbeef".to_owned(),
            }],
            redaction_profile: "metadata_safe_default".to_owned(),
            verifier_ref: "verifier:governance-v1".to_owned(),
            local_only_not_held_note: Some(
                "ref:local-xyz lives only on the local device and is outside managed delete scope."
                    .to_owned(),
            ),
            collab_metadata: None,
        };
        let yaml = serde_yaml::to_string(&receipt).expect("receipt serializes");
        let reparsed: DestructionReceipt =
            serde_yaml::from_str(&yaml).expect("receipt reparses");
        assert_eq!(reparsed, receipt);
    }
}
