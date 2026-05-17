//! Deletion-honesty, held-record selector, and destruction-receipt support projections.
//!
//! The records-governance packet says what class one exported artifact is in.
//! This module adds the M3 support-bundle conveniences that consumers need on
//! top of that packet: stable deletion-honesty labels, selectors for held
//! contractual record classes, and a metadata-only destruction-receipt
//! projection that can be queued into a support-bundle preview.

use aureline_records::{current_registry, validate_typed, RecordClassId, RecordRegistryError};
use serde::{Deserialize, Serialize};

use super::manifest::SizeEstimate;
use super::preview::{PreviewItemSeed, SupportBundlePreviewBuilder};
use super::records::{
    ArtifactClass, DestructionCaveatClass, HoldClass, HoldState, RecordsGovernancePacket,
    RECORDS_GOVERNANCE_PACKET_SCHEMA_REF,
};
use super::vocabulary::{ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass};

/// Stable record-kind tag carried on every support destruction-receipt record.
pub const SUPPORT_DESTRUCTION_RECEIPT_RECORD_KIND: &str = "support_destruction_receipt_record";

/// Frozen schema version for the support destruction-receipt projection.
pub const SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative support boundary schema path.
pub const SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF: &str =
    "schemas/support/destruction_receipt.schema.json";

/// Governance destruction-receipt schema this support projection narrows.
pub const GOVERNANCE_DESTRUCTION_RECEIPT_SCHEMA_REF: &str =
    "schemas/governance/destruction_receipt_alpha.schema.json";

/// Reviewer doc ref for deletion, hold, and receipt support truth.
pub const DELETION_HOLD_TRUTH_DOC_REF: &str = "docs/support/m3/deletion_hold_truth_beta.md";

/// Required redaction class for support destruction-receipt projections.
pub const SUPPORT_DESTRUCTION_RECEIPT_REDACTION_CLASS: &str = "metadata_safe_default";

/// Support-pack item id used when a destruction receipt is queued onto a preview.
pub const SUPPORT_ITEM_DESTRUCTION_RECEIPT: &str = "support.item.destruction_receipt";

/// Closed deletion-honesty states shared by support export, CLI/headless,
/// and product copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletionHonestyState {
    /// No managed or exported copy is represented in this packet.
    LocalOnly,
    /// A managed copy exists and must not be described as deleted.
    ManagedCopy,
    /// Destructive lifecycle is blocked by a legal, support, retention, or export hold.
    Held,
    /// Delete has been requested but terminal completion has not happened.
    QueuedForDeletion,
    /// Delete completed and no in-scope retained copy remains.
    Deleted,
    /// The payload was deleted, blocked, or narrowed while evidence metadata remains.
    RetainedForEvidence,
    /// The artifact is generated output rather than a durable in-product row.
    ExportOnly,
}

impl DeletionHonestyState {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopy => "managed_copy",
            Self::Held => "held",
            Self::QueuedForDeletion => "queued_for_deletion",
            Self::Deleted => "deleted",
            Self::RetainedForEvidence => "retained_for_evidence",
            Self::ExportOnly => "export_only",
        }
    }

    /// Returns the stable user-facing label from the deletion-honesty vocabulary.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::ManagedCopy => "Managed copy",
            Self::Held => "Legal hold",
            Self::QueuedForDeletion => "Delete requested",
            Self::Deleted => "Delete completed",
            Self::RetainedForEvidence => "Policy retention",
            Self::ExportOnly => "Exported copy remains local",
        }
    }

    /// Maps a records-governance artifact class into the shared deletion label.
    pub const fn from_artifact_class(artifact_class: ArtifactClass) -> Self {
        match artifact_class {
            ArtifactClass::LocalOnly => Self::LocalOnly,
            ArtifactClass::ManagedCopy => Self::ManagedCopy,
            ArtifactClass::Held => Self::Held,
            ArtifactClass::QueuedForDelete => Self::QueuedForDeletion,
            ArtifactClass::Deleted => Self::Deleted,
            ArtifactClass::RetainedForEvidence => Self::RetainedForEvidence,
            ArtifactClass::ExportOnly => Self::ExportOnly,
        }
    }
}

/// Reviewer-safe deletion disclosure used by exported support and repair artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionHonestyDisclosure {
    /// Stable state token.
    pub state_class: DeletionHonestyState,
    /// Stable label shown by product, support export, and CLI/headless surfaces.
    pub label: String,
    /// Reviewer-safe reason for the state.
    pub reason: String,
    /// Destruction caveat copied from the records-governance row when available.
    pub destruction_caveat_class: DestructionCaveatClass,
    /// Reviewer-safe caveat note; empty only when no caveat applies.
    pub destruction_caveat_note: String,
    /// Whether a downloaded export copy remains outside a managed delete scope.
    pub exported_copy_remains_local: bool,
}

/// Builds the shared deletion-honesty disclosure for one records-governance packet.
pub fn deletion_honesty_disclosure_for_packet(
    packet: &RecordsGovernancePacket,
) -> DeletionHonestyDisclosure {
    let state_class = DeletionHonestyState::from_artifact_class(packet.artifact_class);
    let reason = if !packet.destruction_caveat_note.trim().is_empty() {
        packet.destruction_caveat_note.clone()
    } else {
        match state_class {
            DeletionHonestyState::LocalOnly => {
                "Only local user-controlled state is represented by this packet.".to_owned()
            }
            DeletionHonestyState::ManagedCopy => {
                "A managed copy or managed index entry remains in scope.".to_owned()
            }
            DeletionHonestyState::Held => {
                "An active hold blocks destructive lifecycle steps.".to_owned()
            }
            DeletionHonestyState::QueuedForDeletion => {
                "A delete request exists, but terminal completion is not recorded.".to_owned()
            }
            DeletionHonestyState::Deleted => {
                "Delete completion is recorded and no in-scope retained copy is listed.".to_owned()
            }
            DeletionHonestyState::RetainedForEvidence => {
                "Evidence metadata remains after delete, redaction, or policy narrowing.".to_owned()
            }
            DeletionHonestyState::ExportOnly => {
                "The artifact is generated export output under user/device control.".to_owned()
            }
        }
    };

    DeletionHonestyDisclosure {
        state_class,
        label: state_class.label().to_owned(),
        reason,
        destruction_caveat_class: packet.destruction_caveat_class,
        destruction_caveat_note: packet.destruction_caveat_note.clone(),
        exported_copy_remains_local: packet.exported_copy_remains_local,
    }
}

/// Selector used by support export and repair flows to find held records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeldRecordSelector {
    /// Stable selector id safe for manifests and CLI/headless output.
    pub selector_id: String,
    /// Optional record-class constraint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_class_id: Option<RecordClassId>,
    /// Optional hold-class constraint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_class: Option<HoldClass>,
    /// Include `release_pending` rows in addition to active holds.
    pub include_release_pending: bool,
    /// Require the selected packet to carry a non-`none` destruction caveat.
    pub require_destruction_caveat: bool,
}

impl HeldRecordSelector {
    /// Returns true when `packet` matches this selector.
    pub fn matches(&self, packet: &RecordsGovernancePacket) -> bool {
        if let Some(record_class_id) = self.record_class_id {
            if packet.record_class_id != record_class_id {
                return false;
            }
        }
        if let Some(hold_class) = self.hold_class {
            if !packet.hold_classes.contains(&hold_class) {
                return false;
            }
        }
        if self.require_destruction_caveat && !packet.has_destruction_caveat() {
            return false;
        }

        match packet.hold_state {
            HoldState::OnHold => !packet.hold_classes.is_empty(),
            HoldState::ReleasePending => self.include_release_pending,
            HoldState::None => false,
        }
    }
}

/// Returns one selector for each beta contractual class that can be held.
pub fn held_record_selectors_for_beta_contractual_classes(
) -> Result<Vec<HeldRecordSelector>, DeletionHoldError> {
    let registry = current_registry()?;
    Ok(registry
        .record_classes
        .iter()
        .filter(|row| row.hold_semantics.eligible.as_bool())
        .map(|row| HeldRecordSelector {
            selector_id: format!("deletion_hold.selector.{}", row.record_class_id.as_str()),
            record_class_id: Some(row.record_class_id),
            hold_class: None,
            include_release_pending: true,
            require_destruction_caveat: false,
        })
        .collect())
}

/// Filters records-governance packets with a held-record selector.
pub fn select_held_records<'a>(
    packets: &'a [RecordsGovernancePacket],
    selector: &HeldRecordSelector,
) -> Vec<&'a RecordsGovernancePacket> {
    packets
        .iter()
        .filter(|packet| selector.matches(packet))
        .collect()
}

/// State of a destruction receipt or non-receipt disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionReceiptState {
    /// Durable destruction receipt is available.
    Available,
    /// Receipt cannot be emitted until a hold clears.
    PendingAfterHoldClear,
    /// Receipt waits for a policy retention floor.
    PendingPolicyFloor,
    /// Aureline cannot issue a platform receipt for the outside-scope record.
    NotAvailableOutsideScope,
    /// Local user action is required before a platform receipt can exist.
    ManualLocalActionRequired,
    /// Receipt details are omitted by redaction policy.
    OmittedByRedaction,
}

impl DestructionReceiptState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::PendingAfterHoldClear => "pending_after_hold_clear",
            Self::PendingPolicyFloor => "pending_policy_floor",
            Self::NotAvailableOutsideScope => "not_available_outside_scope",
            Self::ManualLocalActionRequired => "manual_local_action_required",
            Self::OmittedByRedaction => "omitted_by_redaction",
        }
    }
}

/// Result class for one destruction request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionResultClass {
    /// Requested in-scope refs were destroyed.
    Completed,
    /// Some refs were destroyed while others were retained, skipped, or outside scope.
    Partial,
    /// Active hold blocked destructive execution.
    BlockedByHold,
    /// Policy retention kept one or more refs.
    PolicyRetained,
    /// Requested refs are outside Aureline destructive authority.
    OutsidePlatformScope,
    /// User-side local capture or deletion is required.
    ManualLocalCaptureRequired,
    /// Details are omitted by redaction policy.
    OmittedByRedaction,
    /// No matching ref was found.
    NotFound,
}

impl DestructionResultClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Partial => "partial",
            Self::BlockedByHold => "blocked_by_hold",
            Self::PolicyRetained => "policy_retained",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::ManualLocalCaptureRequired => "manual_local_capture_required",
            Self::OmittedByRedaction => "omitted_by_redaction",
            Self::NotFound => "not_found",
        }
    }
}

/// Destructive action class represented by the receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionActionClass {
    Delete,
    Cleanup,
    OffboardingDelete,
    RedactionDelete,
    ArchiveCompaction,
    ExportDelete,
}

/// Locality of one ref in a destruction receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionLocalityClass {
    LocalOnly,
    ManagedCopy,
    Archived,
    Held,
    ReceiptOnly,
    OutsidePlatformScope,
    RedactedBoundary,
}

/// Reason one ref landed in a destruction receipt bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionReasonClass {
    Destroyed,
    SkippedHeld,
    PolicyRetained,
    OutsidePlatformScope,
    ManualLocalCaptureRequired,
    OmittedByRedaction,
    ReceiptOnlyMetadata,
    NotFound,
}

/// One ref named in a destruction receipt bucket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionScopeRef {
    /// Opaque ref for the affected record.
    #[serde(rename = "ref")]
    pub ref_id: String,
    /// Record class governing the affected record.
    pub record_class_id: RecordClassId,
    /// Locality class for the affected record.
    pub locality_class: DestructionLocalityClass,
    /// Result class for this ref.
    pub result_class: DestructionResultClass,
    /// Reason class for this ref.
    pub reason_class: DestructionReasonClass,
    /// Reviewer-safe summary; never raw payload material.
    pub summary: String,
}

/// Policy context copied into a support destruction receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionReceiptPolicyContext {
    /// Policy version active at request or execution time.
    pub policy_version: String,
    /// Policy source ref active at request or execution time.
    pub policy_source_ref: String,
    /// Retention owner for retained receipt metadata.
    pub retention_owner_ref: String,
}

/// Count summary for the destruction receipt buckets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionReceiptCounts {
    pub requested_ref_count: u32,
    pub destroyed_ref_count: u32,
    pub retained_ref_count: u32,
    pub skipped_held_ref_count: u32,
    pub outside_scope_ref_count: u32,
    pub manual_local_capture_count: u32,
    pub omitted_by_redaction_count: u32,
}

/// Caller-supplied destruction-receipt projection inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportDestructionReceiptInputs {
    /// Stable receipt record id.
    pub receipt_record_id: String,
    /// Durable emitted receipt ref when available.
    pub emitted_receipt_ref: Option<String>,
    /// Receipt availability state.
    pub receipt_state: DestructionReceiptState,
    /// Request result class.
    pub result_class: DestructionResultClass,
    /// Action class that produced this receipt or disclosure.
    pub executed_action_class: DestructionActionClass,
    /// Delete, cleanup, offboarding, or redaction request ref.
    pub request_ref: String,
    /// Record classes covered by this request.
    pub record_class_refs: Vec<RecordClassId>,
    /// Reviewer-safe scope summary.
    pub scope_summary: String,
    /// Policy context used to interpret retained refs.
    pub policy_context: DestructionReceiptPolicyContext,
    /// Execution timestamp when a durable receipt is available.
    pub executed_at: Option<String>,
    /// Refs destroyed by the request.
    pub destroyed_refs: Vec<DestructionScopeRef>,
    /// Refs retained by policy, receipt, or evidence requirements.
    pub retained_refs: Vec<DestructionScopeRef>,
    /// Refs skipped because a hold blocked destruction.
    pub skipped_held_refs: Vec<DestructionScopeRef>,
    /// Refs outside Aureline destructive authority.
    pub outside_scope_refs: Vec<DestructionScopeRef>,
    /// Refs requiring manual local action.
    pub manual_local_capture_refs: Vec<DestructionScopeRef>,
    /// Refs omitted by redaction policy.
    pub omitted_by_redaction_refs: Vec<DestructionScopeRef>,
    /// Custody events backing the receipt.
    pub custody_event_refs: Vec<String>,
    /// Source packets backing the receipt.
    pub source_packet_refs: Vec<String>,
    /// Verifier refs backing the receipt.
    pub verifier_refs: Vec<String>,
    /// Reviewer-safe mirror, lag, or outside-scope note.
    pub mirror_or_lag_note: String,
    /// Support exports carrying this receipt.
    pub support_export_refs: Vec<String>,
    /// Offboarding packets carrying this receipt.
    pub offboarding_packet_refs: Vec<String>,
    /// Records-governance packets linked to this receipt.
    pub linked_records_governance_packet_refs: Vec<String>,
}

/// Metadata-only support destruction-receipt projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportDestructionReceiptRecord {
    pub schema_version: u32,
    pub record_kind: String,
    pub receipt_record_id: String,
    pub schema_ref: String,
    pub governance_schema_ref: String,
    pub doc_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emitted_receipt_ref: Option<String>,
    pub receipt_state: DestructionReceiptState,
    pub result_class: DestructionResultClass,
    pub executed_action_class: DestructionActionClass,
    pub request_ref: String,
    pub record_class_refs: Vec<RecordClassId>,
    pub scope_summary: String,
    pub policy_context: DestructionReceiptPolicyContext,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub destroyed_refs: Vec<DestructionScopeRef>,
    pub retained_refs: Vec<DestructionScopeRef>,
    pub skipped_held_refs: Vec<DestructionScopeRef>,
    pub outside_scope_refs: Vec<DestructionScopeRef>,
    pub manual_local_capture_refs: Vec<DestructionScopeRef>,
    pub omitted_by_redaction_refs: Vec<DestructionScopeRef>,
    pub artifact_counts: DestructionReceiptCounts,
    pub deletion_honesty_disclosure: DeletionHonestyDisclosure,
    pub custody_event_refs: Vec<String>,
    pub source_packet_refs: Vec<String>,
    pub verifier_refs: Vec<String>,
    pub mirror_or_lag_note: String,
    pub support_export_refs: Vec<String>,
    pub offboarding_packet_refs: Vec<String>,
    pub linked_records_governance_packet_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_content_exported: bool,
    pub reviewer_summary: String,
}

/// Errors returned by deletion/hold support projection evaluators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeletionHoldError {
    /// A required id, ref, or summary field was empty.
    FieldEmpty { field: &'static str },
    /// A record class was not registered in the active registry.
    RecordClassUnregistered { record_class_id: RecordClassId },
    /// Registry parse or producer binding validation failed.
    RecordRegistry(RecordRegistryError),
    /// Receipt state and emitted receipt refs disagree.
    ReceiptStateInconsistent { reason: String },
    /// Result class and receipt buckets disagree.
    ResultClassInconsistent { reason: String },
}

impl std::fmt::Display for DeletionHoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldEmpty { field } => write!(f, "deletion/hold field {field} is required"),
            Self::RecordClassUnregistered { record_class_id } => {
                write!(f, "record class {record_class_id} is not registered")
            }
            Self::RecordRegistry(err) => write!(f, "record registry error: {err}"),
            Self::ReceiptStateInconsistent { reason } => {
                write!(f, "destruction receipt state is inconsistent: {reason}")
            }
            Self::ResultClassInconsistent { reason } => {
                write!(
                    f,
                    "destruction receipt result class is inconsistent: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for DeletionHoldError {}

impl From<RecordRegistryError> for DeletionHoldError {
    fn from(err: RecordRegistryError) -> Self {
        Self::RecordRegistry(err)
    }
}

/// Evaluates caller inputs into a support destruction-receipt record.
pub fn evaluate_support_destruction_receipt(
    inputs: SupportDestructionReceiptInputs,
) -> Result<SupportDestructionReceiptRecord, DeletionHoldError> {
    validate_typed(
        SUPPORT_DESTRUCTION_RECEIPT_RECORD_KIND,
        RecordClassId::DestructionReceiptRecord,
    )?;
    validate_receipt_inputs(&inputs)?;

    let artifact_counts = DestructionReceiptCounts {
        requested_ref_count: bucket_count(&inputs) as u32,
        destroyed_ref_count: inputs.destroyed_refs.len() as u32,
        retained_ref_count: inputs.retained_refs.len() as u32,
        skipped_held_ref_count: inputs.skipped_held_refs.len() as u32,
        outside_scope_ref_count: inputs.outside_scope_refs.len() as u32,
        manual_local_capture_count: inputs.manual_local_capture_refs.len() as u32,
        omitted_by_redaction_count: inputs.omitted_by_redaction_refs.len() as u32,
    };
    let deletion_honesty_disclosure =
        deletion_honesty_disclosure_for_receipt(inputs.result_class, &artifact_counts);
    let reviewer_summary = format!(
        "Destruction receipt state {}; result {}; destroyed {}; retained {}; held {}; outside scope {}; manual local {}; omitted by redaction {}; raw_content_exported=false.",
        inputs.receipt_state.as_str(),
        inputs.result_class.as_str(),
        artifact_counts.destroyed_ref_count,
        artifact_counts.retained_ref_count,
        artifact_counts.skipped_held_ref_count,
        artifact_counts.outside_scope_ref_count,
        artifact_counts.manual_local_capture_count,
        artifact_counts.omitted_by_redaction_count
    );

    Ok(SupportDestructionReceiptRecord {
        schema_version: SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_VERSION,
        record_kind: SUPPORT_DESTRUCTION_RECEIPT_RECORD_KIND.to_owned(),
        receipt_record_id: inputs.receipt_record_id,
        schema_ref: SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF.to_owned(),
        governance_schema_ref: GOVERNANCE_DESTRUCTION_RECEIPT_SCHEMA_REF.to_owned(),
        doc_ref: DELETION_HOLD_TRUTH_DOC_REF.to_owned(),
        emitted_receipt_ref: inputs.emitted_receipt_ref,
        receipt_state: inputs.receipt_state,
        result_class: inputs.result_class,
        executed_action_class: inputs.executed_action_class,
        request_ref: inputs.request_ref,
        record_class_refs: inputs.record_class_refs,
        scope_summary: inputs.scope_summary,
        policy_context: inputs.policy_context,
        executed_at: inputs.executed_at,
        destroyed_refs: inputs.destroyed_refs,
        retained_refs: inputs.retained_refs,
        skipped_held_refs: inputs.skipped_held_refs,
        outside_scope_refs: inputs.outside_scope_refs,
        manual_local_capture_refs: inputs.manual_local_capture_refs,
        omitted_by_redaction_refs: inputs.omitted_by_redaction_refs,
        artifact_counts,
        deletion_honesty_disclosure,
        custody_event_refs: inputs.custody_event_refs,
        source_packet_refs: inputs.source_packet_refs,
        verifier_refs: inputs.verifier_refs,
        mirror_or_lag_note: inputs.mirror_or_lag_note,
        support_export_refs: inputs.support_export_refs,
        offboarding_packet_refs: inputs.offboarding_packet_refs,
        linked_records_governance_packet_refs: inputs.linked_records_governance_packet_refs,
        redaction_class: SUPPORT_DESTRUCTION_RECEIPT_REDACTION_CLASS.to_owned(),
        raw_content_exported: false,
        reviewer_summary,
    })
}

fn validate_receipt_inputs(
    inputs: &SupportDestructionReceiptInputs,
) -> Result<(), DeletionHoldError> {
    require_non_empty(&inputs.receipt_record_id, "receipt_record_id")?;
    require_non_empty(&inputs.request_ref, "request_ref")?;
    require_non_empty(&inputs.scope_summary, "scope_summary")?;
    require_non_empty(
        &inputs.policy_context.policy_version,
        "policy_context.policy_version",
    )?;
    require_non_empty(
        &inputs.policy_context.policy_source_ref,
        "policy_context.policy_source_ref",
    )?;
    require_non_empty(
        &inputs.policy_context.retention_owner_ref,
        "policy_context.retention_owner_ref",
    )?;
    require_non_empty(&inputs.mirror_or_lag_note, "mirror_or_lag_note")?;
    require_non_empty_list(&inputs.record_class_refs, "record_class_refs")?;
    require_non_empty_string_list(&inputs.custody_event_refs, "custody_event_refs")?;
    require_non_empty_string_list(&inputs.source_packet_refs, "source_packet_refs")?;
    require_non_empty_string_list(&inputs.verifier_refs, "verifier_refs")?;

    let registry = current_registry()?;
    for record_class_id in &inputs.record_class_refs {
        if !registry.contains_class(*record_class_id) {
            return Err(DeletionHoldError::RecordClassUnregistered {
                record_class_id: *record_class_id,
            });
        }
    }
    validate_scope_refs(&inputs.destroyed_refs, "destroyed_refs")?;
    validate_scope_refs(&inputs.retained_refs, "retained_refs")?;
    validate_scope_refs(&inputs.skipped_held_refs, "skipped_held_refs")?;
    validate_scope_refs(&inputs.outside_scope_refs, "outside_scope_refs")?;
    validate_scope_refs(
        &inputs.manual_local_capture_refs,
        "manual_local_capture_refs",
    )?;
    validate_scope_refs(
        &inputs.omitted_by_redaction_refs,
        "omitted_by_redaction_refs",
    )?;

    match inputs.receipt_state {
        DestructionReceiptState::Available => {
            if inputs
                .emitted_receipt_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
                || inputs
                    .executed_at
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                return Err(DeletionHoldError::ReceiptStateInconsistent {
                    reason: "available receipts require emitted_receipt_ref and executed_at"
                        .to_owned(),
                });
            }
        }
        _ => {
            if inputs.emitted_receipt_ref.is_some() || inputs.executed_at.is_some() {
                return Err(DeletionHoldError::ReceiptStateInconsistent {
                    reason: "pending or unavailable receipts must not claim emitted receipt refs or execution time"
                        .to_owned(),
                });
            }
        }
    }

    validate_state_result_pair(inputs.receipt_state, inputs.result_class)?;
    validate_result_buckets(inputs)?;
    Ok(())
}

fn validate_state_result_pair(
    receipt_state: DestructionReceiptState,
    result_class: DestructionResultClass,
) -> Result<(), DeletionHoldError> {
    let ok = match receipt_state {
        DestructionReceiptState::Available => matches!(
            result_class,
            DestructionResultClass::Completed | DestructionResultClass::Partial
        ),
        DestructionReceiptState::PendingAfterHoldClear => {
            matches!(result_class, DestructionResultClass::BlockedByHold)
        }
        DestructionReceiptState::PendingPolicyFloor => {
            matches!(result_class, DestructionResultClass::PolicyRetained)
        }
        DestructionReceiptState::NotAvailableOutsideScope => {
            matches!(result_class, DestructionResultClass::OutsidePlatformScope)
        }
        DestructionReceiptState::ManualLocalActionRequired => {
            matches!(
                result_class,
                DestructionResultClass::ManualLocalCaptureRequired
            )
        }
        DestructionReceiptState::OmittedByRedaction => {
            matches!(result_class, DestructionResultClass::OmittedByRedaction)
        }
    };
    if ok {
        Ok(())
    } else {
        Err(DeletionHoldError::ReceiptStateInconsistent {
            reason: format!(
                "receipt_state {} cannot be paired with result_class {}",
                receipt_state.as_str(),
                result_class.as_str()
            ),
        })
    }
}

fn validate_result_buckets(
    inputs: &SupportDestructionReceiptInputs,
) -> Result<(), DeletionHoldError> {
    match inputs.result_class {
        DestructionResultClass::Completed => {
            if inputs.destroyed_refs.is_empty()
                || !inputs.retained_refs.is_empty()
                || !inputs.skipped_held_refs.is_empty()
                || !inputs.outside_scope_refs.is_empty()
                || !inputs.manual_local_capture_refs.is_empty()
                || !inputs.omitted_by_redaction_refs.is_empty()
            {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "completed requires destroyed refs and no retained, held, outside-scope, manual, or redacted refs".to_owned(),
                });
            }
        }
        DestructionResultClass::Partial => {
            if inputs.destroyed_refs.is_empty()
                || (inputs.retained_refs.is_empty()
                    && inputs.skipped_held_refs.is_empty()
                    && inputs.outside_scope_refs.is_empty()
                    && inputs.manual_local_capture_refs.is_empty()
                    && inputs.omitted_by_redaction_refs.is_empty())
            {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "partial requires at least one destroyed ref and one retained, held, outside-scope, manual, or redacted ref".to_owned(),
                });
            }
        }
        DestructionResultClass::BlockedByHold => {
            if inputs.skipped_held_refs.is_empty() {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "blocked_by_hold requires skipped_held_refs".to_owned(),
                });
            }
        }
        DestructionResultClass::PolicyRetained => {
            if inputs.retained_refs.is_empty() {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "policy_retained requires retained_refs".to_owned(),
                });
            }
        }
        DestructionResultClass::OutsidePlatformScope => {
            if inputs.outside_scope_refs.is_empty() {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "outside_platform_scope requires outside_scope_refs".to_owned(),
                });
            }
        }
        DestructionResultClass::ManualLocalCaptureRequired => {
            if inputs.manual_local_capture_refs.is_empty() {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "manual_local_capture_required requires manual_local_capture_refs"
                        .to_owned(),
                });
            }
        }
        DestructionResultClass::OmittedByRedaction => {
            if inputs.omitted_by_redaction_refs.is_empty() {
                return Err(DeletionHoldError::ResultClassInconsistent {
                    reason: "omitted_by_redaction requires omitted_by_redaction_refs".to_owned(),
                });
            }
        }
        DestructionResultClass::NotFound => {}
    }
    Ok(())
}

fn deletion_honesty_disclosure_for_receipt(
    result_class: DestructionResultClass,
    artifact_counts: &DestructionReceiptCounts,
) -> DeletionHonestyDisclosure {
    let (state_class, caveat, reason) = match result_class {
        DestructionResultClass::Completed => (
            DeletionHonestyState::Deleted,
            DestructionCaveatClass::None,
            "Delete completed for the in-scope refs; no retained support evidence was listed."
                .to_owned(),
        ),
        DestructionResultClass::BlockedByHold => (
            DeletionHonestyState::Held,
            DestructionCaveatClass::LegalHoldPrevents,
            format!(
                "{} held ref(s) blocked destructive completion.",
                artifact_counts.skipped_held_ref_count
            ),
        ),
        DestructionResultClass::Partial | DestructionResultClass::PolicyRetained => (
            DeletionHonestyState::RetainedForEvidence,
            DestructionCaveatClass::RetainedSubsetRemains,
            format!(
                "{} retained ref(s) remain visible as evidence or policy-retained metadata.",
                artifact_counts.retained_ref_count
            ),
        ),
        DestructionResultClass::OutsidePlatformScope => (
            DeletionHonestyState::RetainedForEvidence,
            DestructionCaveatClass::OutsidePlatformScope,
            format!(
                "{} ref(s) are outside Aureline destructive authority.",
                artifact_counts.outside_scope_ref_count
            ),
        ),
        DestructionResultClass::ManualLocalCaptureRequired => (
            DeletionHonestyState::RetainedForEvidence,
            DestructionCaveatClass::ExportedLocalCopyRemains,
            format!(
                "{} local-only ref(s) require user-side capture or deletion.",
                artifact_counts.manual_local_capture_count
            ),
        ),
        DestructionResultClass::OmittedByRedaction => (
            DeletionHonestyState::RetainedForEvidence,
            DestructionCaveatClass::RetainedSubsetRemains,
            format!(
                "{} ref(s) are omitted by redaction while the omission stays visible.",
                artifact_counts.omitted_by_redaction_count
            ),
        ),
        DestructionResultClass::NotFound => (
            DeletionHonestyState::RetainedForEvidence,
            DestructionCaveatClass::OutsidePlatformScope,
            "No matching in-scope ref was found; the receipt remains as evidence of the request."
                .to_owned(),
        ),
    };

    DeletionHonestyDisclosure {
        state_class,
        label: state_class.label().to_owned(),
        reason: reason.clone(),
        destruction_caveat_class: caveat,
        destruction_caveat_note: if matches!(caveat, DestructionCaveatClass::None) {
            String::new()
        } else {
            reason
        },
        exported_copy_remains_local: matches!(
            result_class,
            DestructionResultClass::ManualLocalCaptureRequired | DestructionResultClass::Partial
        ),
    }
}

/// Builds a preview-row seed for a support destruction receipt.
pub fn destruction_receipt_preview_item_seed(
    receipt: &SupportDestructionReceiptRecord,
) -> PreviewItemSeed {
    let bytes_estimate = serde_json::to_vec(receipt)
        .map(|bytes| bytes.len() as u64)
        .ok();
    let display_label = bytes_estimate
        .map(|bytes| format!("{bytes} B"))
        .unwrap_or_else(|| "unknown until export".to_owned());

    let impact_class = if matches!(receipt.result_class, DestructionResultClass::Completed) {
        ActionabilityImpactClass::Medium
    } else {
        ActionabilityImpactClass::High
    };

    PreviewItemSeed {
        support_pack_item_id: SUPPORT_ITEM_DESTRUCTION_RECEIPT.to_owned(),
        title: format!("Destruction receipt - {}", receipt.receipt_record_id),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".to_owned(),
        artifact_kind_class: "support_destruction_receipt".to_owned(),
        manifest_path_ref: "preview_items[].support_destruction_receipt_record".to_owned(),
        bundle_member_path_ref: Some(format!(
            "manifest/destruction_receipts/{}.json",
            safe_member_name(&receipt.receipt_record_id)
        )),
        source_refs: vec![
            receipt.schema_ref.clone(),
            receipt.governance_schema_ref.clone(),
            receipt.doc_ref.clone(),
            RECORDS_GOVERNANCE_PACKET_SCHEMA_REF.to_owned(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: bytes_estimate,
            confidence_class: if bytes_estimate.is_some() {
                "exact".to_owned()
            } else {
                "unknown".to_owned()
            },
            display_label,
            size_source_class: if bytes_estimate.is_some() {
                "precomputed_manifest".to_owned()
            } else {
                "unknown_until_export".to_owned()
            },
        },
        impact_class,
        impact_summary: format!(
            "{} destruction receipt with {} destroyed refs and {} retained/held refs.",
            receipt.result_class.as_str(),
            receipt.artifact_counts.destroyed_ref_count,
            receipt.artifact_counts.retained_ref_count
                + receipt.artifact_counts.skipped_held_ref_count
        ),
        notes: receipt.reviewer_summary.clone(),
    }
}

/// Queues a destruction-receipt row on a support-bundle preview builder.
pub fn add_destruction_receipt_preview_item<'a>(
    builder: &'a mut SupportBundlePreviewBuilder,
    receipt: &SupportDestructionReceiptRecord,
) -> &'a mut SupportBundlePreviewBuilder {
    builder.add_item(destruction_receipt_preview_item_seed(receipt))
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), DeletionHoldError> {
    if value.trim().is_empty() {
        Err(DeletionHoldError::FieldEmpty { field })
    } else {
        Ok(())
    }
}

fn require_non_empty_list<T>(items: &[T], field: &'static str) -> Result<(), DeletionHoldError> {
    if items.is_empty() {
        Err(DeletionHoldError::FieldEmpty { field })
    } else {
        Ok(())
    }
}

fn require_non_empty_string_list(
    items: &[String],
    field: &'static str,
) -> Result<(), DeletionHoldError> {
    require_non_empty_list(items, field)?;
    for item in items {
        require_non_empty(item, field)?;
    }
    Ok(())
}

fn validate_scope_refs(
    refs: &[DestructionScopeRef],
    field: &'static str,
) -> Result<(), DeletionHoldError> {
    let registry = current_registry()?;
    for scope_ref in refs {
        require_non_empty(&scope_ref.ref_id, field)?;
        require_non_empty(&scope_ref.summary, field)?;
        if !registry.contains_class(scope_ref.record_class_id) {
            return Err(DeletionHoldError::RecordClassUnregistered {
                record_class_id: scope_ref.record_class_id,
            });
        }
    }
    Ok(())
}

fn bucket_count(inputs: &SupportDestructionReceiptInputs) -> usize {
    inputs.destroyed_refs.len()
        + inputs.retained_refs.len()
        + inputs.skipped_held_refs.len()
        + inputs.outside_scope_refs.len()
        + inputs.manual_local_capture_refs.len()
        + inputs.omitted_by_redaction_refs.len()
}

fn safe_member_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletion_honesty_state_maps_records_artifact_classes() {
        assert_eq!(
            DeletionHonestyState::from_artifact_class(ArtifactClass::Deleted),
            DeletionHonestyState::Deleted
        );
        assert_eq!(
            DeletionHonestyState::from_artifact_class(ArtifactClass::RetainedForEvidence).label(),
            "Policy retention"
        );
    }
}
