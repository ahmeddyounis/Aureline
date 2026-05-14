//! Admin alpha projection for delete honesty, legal holds, chronology, and policy diffs.
//!
//! This module is the shell-side consumer for the enterprise/admin trust wedge.
//! It composes existing governance, policy explainability, audit, deletion-job,
//! and support-export vocabulary into one inspectable packet. It does not
//! implement a retention backend, policy evaluator, legal-hold workflow, or
//! hosted admin console.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AdminAlphaPacket`].
pub const ADMIN_ALPHA_PACKET_RECORD_KIND: &str = "admin_delete_hold_policy_alpha_packet";

/// Stable record-kind tag carried by [`AdminAlphaSupportExport`].
pub const ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "admin_delete_hold_policy_alpha_support_export";

/// Schema version for admin alpha packets and projections.
pub const ADMIN_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Result vocabulary shared by delete reviews, desktop rows, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminAlphaResultClass {
    /// All in-scope eligible records reached the requested terminal state.
    Completed,
    /// Some in-scope records completed, while another subset remains.
    Partial,
    /// Destructive completion is blocked by one or more active hold records.
    BlockedByHold,
    /// A stricter retention policy kept at least one managed or audit subset.
    PolicyRetained,
    /// The requested artifact class is outside Aureline's managed scope.
    OutsidePlatformScope,
    /// The artifact exists only under user-controlled local storage.
    ManualLocalCaptureRequired,
    /// Export intentionally excluded data under the applied redaction policy.
    OmittedByRedaction,
}

impl AdminAlphaResultClass {
    /// Stable token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Partial => "partial",
            Self::BlockedByHold => "blocked_by_hold",
            Self::PolicyRetained => "policy_retained",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::ManualLocalCaptureRequired => "manual_local_capture_required",
            Self::OmittedByRedaction => "omitted_by_redaction",
        }
    }

    /// User-facing compact label for admin rows.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Completed => "Completed",
            Self::Partial => "Partial",
            Self::BlockedByHold => "Blocked by hold",
            Self::PolicyRetained => "Policy retained",
            Self::OutsidePlatformScope => "Outside platform scope",
            Self::ManualLocalCaptureRequired => "Manual local capture required",
            Self::OmittedByRedaction => "Omitted by redaction",
        }
    }

    /// Full result vocabulary required on support/export projections.
    pub fn vocabulary() -> Vec<Self> {
        vec![
            Self::Completed,
            Self::Partial,
            Self::BlockedByHold,
            Self::PolicyRetained,
            Self::OutsidePlatformScope,
            Self::ManualLocalCaptureRequired,
            Self::OmittedByRedaction,
        ]
    }

    /// True when the result is a completed destructive action.
    pub const fn is_completion(self) -> bool {
        matches!(self, Self::Completed)
    }
}

/// Archive-search posture visible before delete/export review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveSearchPostureClass {
    /// Archive search completed for the declared scope.
    SearchComplete,
    /// Search completed but the result is partial and labelled.
    SearchPartial,
    /// Search is not applicable because the item is local-only or outside scope.
    NotApplicable,
    /// Search is unavailable and the row must stay review-required.
    UnavailableReviewRequired,
}

impl ArchiveSearchPostureClass {
    /// Stable token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchComplete => "search_complete",
            Self::SearchPartial => "search_partial",
            Self::NotApplicable => "not_applicable",
            Self::UnavailableReviewRequired => "unavailable_review_required",
        }
    }

    /// True when the posture is explicitly reviewable.
    pub const fn is_declared(self) -> bool {
        !matches!(self, Self::UnavailableReviewRequired)
    }
}

/// Destruction-receipt posture visible on destructive rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionReceiptAvailabilityClass {
    /// A durable receipt was emitted and linked.
    Available,
    /// A receipt will be emitted only after a hold clears.
    PendingAfterHoldClear,
    /// A receipt will be emitted when a retention floor expires.
    PendingPolicyFloor,
    /// No receipt can be issued because Aureline never possessed the record.
    NotAvailableOutsideScope,
    /// No platform receipt can be issued until the user captures a local artifact.
    ManualLocalActionRequired,
    /// Receipt detail is intentionally hidden by redaction policy.
    OmittedByRedaction,
}

impl DestructionReceiptAvailabilityClass {
    /// Stable token used in fixtures and exports.
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

/// Export-before-delete option shown on delete/offboarding review rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBeforeDeleteClass {
    /// Export is not required for this record class.
    NotRequired,
    /// Export can be created before destructive action proceeds.
    Available,
    /// Export is required before the destructive action can proceed.
    RequiredBeforeDelete,
    /// The user must capture local-only state manually.
    ManualLocalCaptureRequired,
    /// Export exists but is hidden or narrowed by redaction policy.
    UnavailableByRedaction,
}

/// Policy diff preview posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDiffPreviewState {
    /// The diff has been generated before apply and is reviewable.
    PreviewBeforeApply,
    /// The source changed after preview and the diff must be regenerated.
    StaleNeedsRerun,
}

/// One timezone-aware timestamp representation used across the admin wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaChronology {
    /// Canonical RFC 3339 UTC instant with a `Z` suffix.
    pub utc_instant: String,
    /// Local civil-time rendering with an explicit offset.
    pub local_iso_with_offset: String,
    /// IANA time-zone identifier used by the rendering surface.
    pub timezone_id: String,
    /// Signed UTC offset at the instant, such as `-07:00`.
    pub offset_at_instant: String,
    /// Source clock class copied from the governance chronology vocabulary.
    pub source_clock_class: String,
    /// Stable ordering key used when two events share similar wall-clock time.
    pub ordering_key: String,
}

impl AdminAlphaChronology {
    /// True when the timestamp carries UTC, local civil time, timezone, and offset.
    pub fn is_timezone_aware(&self) -> bool {
        self.utc_instant.ends_with('Z')
            && !self.local_iso_with_offset.ends_with('Z')
            && !self.timezone_id.trim().is_empty()
            && has_signed_offset(&self.offset_at_instant)
            && !self.source_clock_class.trim().is_empty()
            && !self.ordering_key.trim().is_empty()
    }
}

/// Redaction boundary shared by policy, delete, and support-export rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaRedactionBoundary {
    /// Stable redaction profile or policy reference.
    pub redaction_profile_ref: String,
    /// Data classes included in the row or export.
    pub included_data_classes: Vec<String>,
    /// Data classes omitted from the row or export.
    pub omitted_data_classes: Vec<String>,
    /// True when raw payload bodies are excluded from this boundary.
    pub raw_payloads_excluded: bool,
    /// Reviewable summary of the redaction rule.
    pub summary: String,
}

impl AdminAlphaRedactionBoundary {
    /// True when the boundary is safe to quote in support/admin exports.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payloads_excluded
            && !self.redaction_profile_ref.trim().is_empty()
            && !self.summary.trim().is_empty()
    }
}

/// Archive-search posture for one delete/export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveSearchPosture {
    /// Closed search posture class.
    pub posture_class: ArchiveSearchPostureClass,
    /// Optional archive-search index or manifest ref.
    pub search_index_ref: Option<String>,
    /// Record classes included in the archive search.
    pub searched_record_class_refs: Vec<String>,
    /// Reviewable coverage summary.
    pub coverage_summary: String,
}

impl ArchiveSearchPosture {
    /// True when the row does not hide whether archive search ran.
    pub fn is_declared(&self) -> bool {
        self.posture_class.is_declared() && !self.coverage_summary.trim().is_empty()
    }
}

/// Hold selector or hold-scope truth for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldScopeTruth {
    /// Selector family, such as legal hold, support investigation, or none.
    pub selector_class: String,
    /// Selectors evaluated for the row.
    pub selector_refs: Vec<String>,
    /// Hold refs matched by the selectors.
    pub matched_hold_refs: Vec<String>,
    /// Record classes affected by the hold scope.
    pub affected_record_class_refs: Vec<String>,
    /// True only if the hold widened read authority, which this alpha rejects.
    pub read_rights_widened: bool,
    /// Reviewable summary of the hold scope.
    pub summary: String,
}

impl HoldScopeTruth {
    /// True when the row names hold scope and does not widen read rights.
    pub fn is_declared_without_read_widening(&self) -> bool {
        !self.selector_class.trim().is_empty()
            && !self.summary.trim().is_empty()
            && !self.read_rights_widened
    }
}

/// Chain-of-custody summary for admin support review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainOfCustodySummary {
    /// Custody event refs that prove the record path.
    pub custody_event_refs: Vec<String>,
    /// Source packet refs that back the row.
    pub source_packet_refs: Vec<String>,
    /// Verifier refs for receipt, manifest, or signature checks.
    pub verifier_refs: Vec<String>,
    /// Reviewable chain-of-custody summary.
    pub summary: String,
}

impl ChainOfCustodySummary {
    /// True when custody has at least one event and source packet.
    pub fn is_declared(&self) -> bool {
        !self.custody_event_refs.is_empty()
            && !self.source_packet_refs.is_empty()
            && !self.summary.trim().is_empty()
    }
}

/// Destruction-receipt availability for one delete/offboarding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionReceiptAvailability {
    /// Closed receipt availability class.
    pub availability_class: DestructionReceiptAvailabilityClass,
    /// Durable destruction receipt refs when available.
    pub receipt_refs: Vec<String>,
    /// Reason a receipt cannot yet be issued.
    pub unavailable_reason: Option<String>,
    /// True when a durable receipt is expected after the blocker clears.
    pub durable_receipt_expected: bool,
}

impl DestructionReceiptAvailability {
    /// True when the row emits a receipt or explains why it cannot.
    pub fn has_receipt_or_reason(&self) -> bool {
        !self.receipt_refs.is_empty()
            || self
                .unavailable_reason
                .as_ref()
                .is_some_and(|reason| !reason.trim().is_empty())
    }
}

/// Export-before-delete option for one delete/offboarding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBeforeDeleteOption {
    /// Closed export-before-delete class.
    pub option_class: ExportBeforeDeleteClass,
    /// Export manifest refs produced or offered before deletion.
    pub export_manifest_refs: Vec<String>,
    /// Reviewable export-before-delete summary.
    pub summary: String,
}

impl ExportBeforeDeleteOption {
    /// True when the row explicitly states the export-before-delete posture.
    pub fn is_declared(&self) -> bool {
        !self.summary.trim().is_empty()
    }
}

/// One delete/export review row on the admin alpha wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminDeleteReviewRow {
    /// Stable row id.
    pub row_id: String,
    /// Parent flow id.
    pub flow_id: String,
    /// Record class affected by the row.
    pub record_class_id: String,
    /// Opaque subject ref for the governed record or import source.
    pub subject_ref: String,
    /// Operation class, such as delete, offboard, export, or cleanup.
    pub operation_class: String,
    /// Shared result class.
    pub result_class: AdminAlphaResultClass,
    /// Stable delete state from the retention/delete vocabulary.
    pub stable_delete_state: String,
    /// Completion claim from the delete-request-state vocabulary.
    pub completion_claim: String,
    /// Policy source that governed this row.
    pub policy_source_ref: String,
    /// Timezone-aware chronology for the row's latest state.
    pub chronology: AdminAlphaChronology,
    /// Archive-search posture.
    pub archive_search: ArchiveSearchPosture,
    /// Hold selector or hold-scope truth.
    pub hold_scope: HoldScopeTruth,
    /// Redaction boundary for this row.
    pub redaction_boundary: AdminAlphaRedactionBoundary,
    /// Chain-of-custody summary for the row.
    pub chain_of_custody: ChainOfCustodySummary,
    /// Destruction-receipt availability.
    pub destruction_receipt: DestructionReceiptAvailability,
    /// Export-before-delete option.
    pub export_before_delete: ExportBeforeDeleteOption,
    /// Remaining location classes that still hold or reference the data.
    pub remaining_location_classes: Vec<String>,
    /// Partial blocker classes attached to the result.
    pub partial_blocker_classes: Vec<String>,
    /// Policy diff refs that explain any lifecycle change.
    pub linked_policy_diff_refs: Vec<String>,
    /// Reviewable result summary.
    pub result_summary: String,
}

impl AdminDeleteReviewRow {
    /// True when all required honesty sub-surfaces are populated.
    pub fn surfaces_required_honesty(&self) -> bool {
        self.archive_search.is_declared()
            && self.hold_scope.is_declared_without_read_widening()
            && self.redaction_boundary.is_export_safe()
            && self.chain_of_custody.is_declared()
            && self.destruction_receipt.has_receipt_or_reason()
            && self.export_before_delete.is_declared()
            && self.chronology.is_timezone_aware()
            && !self.result_summary.trim().is_empty()
    }
}

/// Delete/offboarding flow input to the admin alpha inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminDeleteFlowInput {
    /// Stable flow id.
    pub flow_id: String,
    /// Reviewable flow title.
    pub title: String,
    /// Source artifacts the flow composes over.
    pub source_refs: Vec<String>,
    /// Rows in the delete/export review.
    pub rows: Vec<AdminDeleteReviewRow>,
}

/// Policy source summary used by diff previews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminPolicySource {
    /// Stable source ref.
    pub source_ref: String,
    /// Export-safe source label.
    pub source_label: String,
    /// Policy epoch in force for this source.
    pub policy_epoch: String,
    /// Distribution freshness class.
    pub distribution_freshness_class: String,
    /// Signature or validation state.
    pub validation_state: String,
}

impl AdminPolicySource {
    /// True when the policy source is usable as a current source anchor.
    pub fn is_declared(&self) -> bool {
        !self.source_ref.trim().is_empty()
            && !self.policy_epoch.trim().is_empty()
            && !self.validation_state.trim().is_empty()
    }
}

/// One row inside a policy diff preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminPolicyDiffRow {
    /// Stable diff row id.
    pub diff_row_id: String,
    /// Affected target ref.
    pub affected_target_ref: String,
    /// Previous state projection.
    pub previous_projection: String,
    /// Effective or proposed state projection.
    pub effective_projection: String,
    /// User-visible consequence.
    pub user_visible_consequence: String,
    /// Lifecycle refs, such as retention matrix or delete state rows.
    pub lifecycle_refs: Vec<String>,
    /// Result class implicated by this diff row, when applicable.
    pub result_class: AdminAlphaResultClass,
    /// Redaction boundary for this diff row.
    pub redaction_boundary: AdminAlphaRedactionBoundary,
}

impl AdminPolicyDiffRow {
    /// True when the diff row is safe and useful to preview.
    pub fn is_previewable(&self) -> bool {
        !self.diff_row_id.trim().is_empty()
            && !self.affected_target_ref.trim().is_empty()
            && !self.user_visible_consequence.trim().is_empty()
            && self.redaction_boundary.is_export_safe()
    }
}

/// Policy-diff preview required before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminPolicyDiffPreview {
    /// Stable diff id.
    pub diff_id: String,
    /// Closed preview state.
    pub preview_state: PolicyDiffPreviewState,
    /// Source that is currently in force.
    pub current_policy_source: AdminPolicySource,
    /// Baseline source being compared.
    pub baseline_source: AdminPolicySource,
    /// Proposed source being previewed.
    pub proposed_source: AdminPolicySource,
    /// Timezone-aware chronology for preview generation.
    pub preview_generated_at: AdminAlphaChronology,
    /// True when apply is blocked until the preview is acknowledged.
    pub apply_requires_preview_ack: bool,
    /// Diff rows visible before apply.
    pub diff_rows: Vec<AdminPolicyDiffRow>,
    /// Export pair refs for the preview packet.
    pub export_pair_refs: Vec<String>,
    /// Redaction boundary for the diff preview.
    pub redaction_boundary: AdminAlphaRedactionBoundary,
    /// Reviewable summary.
    pub summary: String,
}

impl AdminPolicyDiffPreview {
    /// True when the policy diff is previewed before apply and tied to a source.
    pub fn is_pre_apply_preview_for_current_source(&self) -> bool {
        self.preview_state == PolicyDiffPreviewState::PreviewBeforeApply
            && self.apply_requires_preview_ack
            && self.current_policy_source.is_declared()
            && self.baseline_source.is_declared()
            && self.proposed_source.is_declared()
            && self.preview_generated_at.is_timezone_aware()
            && !self.diff_rows.is_empty()
            && self
                .diff_rows
                .iter()
                .all(AdminPolicyDiffRow::is_previewable)
            && self.redaction_boundary.is_export_safe()
    }
}

/// Inspector input for one admin alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaInput {
    /// Stable packet id to assign.
    pub packet_id: String,
    /// Timezone-aware generation chronology.
    pub generated_at: AdminAlphaChronology,
    /// Source documents or artifacts the packet composes over.
    pub source_refs: Vec<String>,
    /// Delete/export flows projected into the packet.
    pub delete_flows: Vec<AdminDeleteFlowInput>,
    /// Policy diff preview tied to the current source.
    pub policy_diff_preview: AdminPolicyDiffPreview,
    /// Support-export id to assign.
    pub support_export_id: String,
}

/// Compact desktop row card for admin delete/export review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminDeleteDesktopCard {
    /// Source review row id.
    pub row_id: String,
    /// Record class affected by the row.
    pub record_class_id: String,
    /// Shared result class.
    pub result_class: AdminAlphaResultClass,
    /// Compact label that mirrors [`AdminAlphaResultClass`].
    pub result_label: String,
    /// True when an active hold was matched.
    pub hold_matched: bool,
    /// True when a durable destruction receipt is currently linked.
    pub destruction_receipt_available: bool,
    /// Remaining location classes shown on the row.
    pub remaining_location_classes: Vec<String>,
    /// Actions or route refs the shell can surface.
    pub action_refs: Vec<String>,
}

/// Desktop/admin projection for the alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaDesktopProjection {
    /// Stable projection kind.
    pub projection_kind: String,
    /// Source packet id.
    pub packet_id: String,
    /// Timezone used by all admin chronology rows.
    pub display_timezone_id: String,
    /// Result vocabulary available to the surface.
    pub result_vocabulary: Vec<AdminAlphaResultClass>,
    /// Compact delete/export cards.
    pub delete_cards: Vec<AdminDeleteDesktopCard>,
    /// Policy diff id shown by the desktop projection.
    pub policy_diff_id: String,
    /// Reviewable policy diff summary.
    pub policy_diff_summary: String,
}

/// Count of rows by result class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaResultCount {
    /// Shared result class.
    pub result_class: AdminAlphaResultClass,
    /// Number of delete/export rows with this result.
    pub row_count: usize,
}

/// Support/export projection for admin delete, hold, chronology, and policy diff truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Source packet id.
    pub source_packet_id: String,
    /// Timezone-aware generation chronology.
    pub generated_at: AdminAlphaChronology,
    /// Full result vocabulary preserved by the export.
    pub result_vocabulary: Vec<AdminAlphaResultClass>,
    /// Counts by result class.
    pub result_counts: Vec<AdminAlphaResultCount>,
    /// Delete/export rows carried by the export.
    pub delete_review_rows: Vec<AdminDeleteReviewRow>,
    /// Policy diff preview carried by the export.
    pub policy_diff_preview: AdminPolicyDiffPreview,
    /// True when raw payload bodies are excluded from every row.
    pub raw_payloads_excluded: bool,
}

impl AdminAlphaSupportExport {
    /// Number of delete/export rows in the support export.
    pub fn row_count(&self) -> usize {
        self.delete_review_rows.len()
    }

    /// True when every required result token is preserved in the export.
    pub fn preserves_result_vocabulary(&self) -> bool {
        self.result_vocabulary == AdminAlphaResultClass::vocabulary()
    }
}

/// Full admin alpha packet with desktop and support/export projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Timezone-aware generation chronology.
    pub generated_at: AdminAlphaChronology,
    /// Source artifacts the packet composes over.
    pub source_refs: Vec<String>,
    /// Flattened delete/export review rows.
    pub delete_review_rows: Vec<AdminDeleteReviewRow>,
    /// Policy diff preview.
    pub policy_diff_preview: AdminPolicyDiffPreview,
    /// Desktop/admin projection.
    pub desktop_projection: AdminAlphaDesktopProjection,
    /// Support/export projection.
    pub support_export: AdminAlphaSupportExport,
}

impl AdminAlphaPacket {
    /// True when every timestamp uses the shared timezone-aware representation.
    pub fn all_chronology_is_timezone_aware(&self) -> bool {
        self.generated_at.is_timezone_aware()
            && self
                .delete_review_rows
                .iter()
                .all(|row| row.chronology.is_timezone_aware())
            && self
                .policy_diff_preview
                .preview_generated_at
                .is_timezone_aware()
    }

    /// True when all required admin delete honesty fields are visible.
    pub fn all_delete_rows_surface_required_honesty(&self) -> bool {
        self.delete_review_rows
            .iter()
            .all(AdminDeleteReviewRow::surfaces_required_honesty)
    }

    /// True when the packet observes every alpha result token in its rows.
    pub fn has_result_vocabulary_floor(&self) -> bool {
        let observed: BTreeSet<_> = self
            .delete_review_rows
            .iter()
            .map(|row| row.result_class)
            .collect();
        AdminAlphaResultClass::vocabulary()
            .into_iter()
            .all(|result| observed.contains(&result))
    }

    /// True when support export preserves the same result vocabulary.
    pub fn support_export_preserves_result_vocabulary(&self) -> bool {
        self.support_export.preserves_result_vocabulary()
    }

    /// True when one destructive/offboarding row has a durable receipt.
    pub fn has_durable_destruction_receipt(&self) -> bool {
        self.delete_review_rows
            .iter()
            .any(|row| !row.destruction_receipt.receipt_refs.is_empty())
    }
}

/// Errors returned while projecting admin alpha rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminAlphaError {
    /// Packet-level field is missing.
    InvalidPacket {
        /// Field or invariant that failed.
        reason: String,
    },
    /// A delete/export row failed validation.
    InvalidDeleteRow {
        /// Row id that failed validation.
        row_id: String,
        /// Field or invariant that failed.
        reason: String,
    },
    /// The policy diff preview failed validation.
    InvalidPolicyDiff {
        /// Field or invariant that failed.
        reason: String,
    },
}

impl fmt::Display for AdminAlphaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPacket { reason } => write!(f, "invalid admin alpha packet: {reason}"),
            Self::InvalidDeleteRow { row_id, reason } => {
                write!(f, "invalid admin delete/export row {row_id}: {reason}")
            }
            Self::InvalidPolicyDiff { reason } => {
                write!(f, "invalid admin policy diff preview: {reason}")
            }
        }
    }
}

impl Error for AdminAlphaError {}

/// Projector for admin delete, legal-hold, chronology, and policy-diff truth.
#[derive(Debug, Clone, Default)]
pub struct AdminAlphaInspector;

impl AdminAlphaInspector {
    /// Creates a new admin alpha inspector.
    pub const fn new() -> Self {
        Self
    }

    /// Materializes desktop and support/export projections from checked fixtures or caller input.
    pub fn inspect(&self, input: AdminAlphaInput) -> Result<AdminAlphaPacket, AdminAlphaError> {
        validate_input(&input)?;

        let delete_review_rows: Vec<AdminDeleteReviewRow> = input
            .delete_flows
            .iter()
            .flat_map(|flow| flow.rows.iter().cloned())
            .collect();
        let result_counts = result_counts(&delete_review_rows);
        let desktop_projection = desktop_projection(&input, &delete_review_rows);
        let support_export = AdminAlphaSupportExport {
            record_kind: ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ADMIN_ALPHA_SCHEMA_VERSION,
            export_id: input.support_export_id.clone(),
            source_packet_id: input.packet_id.clone(),
            generated_at: input.generated_at.clone(),
            result_vocabulary: AdminAlphaResultClass::vocabulary(),
            result_counts,
            delete_review_rows: delete_review_rows.clone(),
            policy_diff_preview: input.policy_diff_preview.clone(),
            raw_payloads_excluded: delete_review_rows
                .iter()
                .all(|row| row.redaction_boundary.raw_payloads_excluded)
                && input
                    .policy_diff_preview
                    .redaction_boundary
                    .raw_payloads_excluded,
        };

        Ok(AdminAlphaPacket {
            record_kind: ADMIN_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: ADMIN_ALPHA_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            source_refs: input.source_refs,
            delete_review_rows,
            policy_diff_preview: input.policy_diff_preview,
            desktop_projection,
            support_export,
        })
    }
}

fn validate_input(input: &AdminAlphaInput) -> Result<(), AdminAlphaError> {
    if input.packet_id.trim().is_empty() {
        return Err(AdminAlphaError::InvalidPacket {
            reason: "packet_id is required".to_owned(),
        });
    }
    if !input.generated_at.is_timezone_aware() {
        return Err(AdminAlphaError::InvalidPacket {
            reason: "generated_at must use admin alpha chronology".to_owned(),
        });
    }
    if input.delete_flows.is_empty() {
        return Err(AdminAlphaError::InvalidPacket {
            reason: "at least one delete/export flow is required".to_owned(),
        });
    }
    if input.support_export_id.trim().is_empty() {
        return Err(AdminAlphaError::InvalidPacket {
            reason: "support_export_id is required".to_owned(),
        });
    }
    for flow in &input.delete_flows {
        if flow.flow_id.trim().is_empty() || flow.rows.is_empty() {
            return Err(AdminAlphaError::InvalidPacket {
                reason: "delete flow must carry flow_id and rows".to_owned(),
            });
        }
        for row in &flow.rows {
            validate_delete_row(row)?;
        }
    }
    if !input
        .policy_diff_preview
        .is_pre_apply_preview_for_current_source()
    {
        return Err(AdminAlphaError::InvalidPolicyDiff {
            reason: "diff must be previewed before apply and tied to the current policy source"
                .to_owned(),
        });
    }
    Ok(())
}

fn validate_delete_row(row: &AdminDeleteReviewRow) -> Result<(), AdminAlphaError> {
    if row.row_id.trim().is_empty() {
        return Err(AdminAlphaError::InvalidDeleteRow {
            row_id: "<missing>".to_owned(),
            reason: "row_id is required".to_owned(),
        });
    }
    let invalid = |reason: &str| AdminAlphaError::InvalidDeleteRow {
        row_id: row.row_id.clone(),
        reason: reason.to_owned(),
    };
    if row.record_class_id.trim().is_empty() || row.subject_ref.trim().is_empty() {
        return Err(invalid("record_class_id and subject_ref are required"));
    }
    if !row.chronology.is_timezone_aware() {
        return Err(invalid("chronology must use admin alpha representation"));
    }
    if !row.archive_search.is_declared() {
        return Err(invalid("archive-search posture must be visible"));
    }
    if !row.hold_scope.is_declared_without_read_widening() {
        return Err(invalid(
            "hold selector/scope must be visible and cannot widen read rights",
        ));
    }
    if row.result_class == AdminAlphaResultClass::BlockedByHold
        && row.hold_scope.matched_hold_refs.is_empty()
    {
        return Err(invalid("blocked_by_hold rows must cite a matched hold"));
    }
    if !row.redaction_boundary.is_export_safe() {
        return Err(invalid("redaction boundary must exclude raw payloads"));
    }
    if !row.chain_of_custody.is_declared() {
        return Err(invalid("chain-of-custody summary is required"));
    }
    if !row.destruction_receipt.has_receipt_or_reason() {
        return Err(invalid(
            "destruction receipt must be linked or explicitly unavailable",
        ));
    }
    if row.result_class.is_completion() && row.destruction_receipt.receipt_refs.is_empty() {
        return Err(invalid("completed rows must cite a durable receipt"));
    }
    if !row.export_before_delete.is_declared() {
        return Err(invalid("export-before-delete posture is required"));
    }
    if row.remaining_location_classes.is_empty()
        && row.result_class != AdminAlphaResultClass::Completed
    {
        return Err(invalid(
            "non-completed rows must disclose remaining locations",
        ));
    }
    Ok(())
}

fn desktop_projection(
    input: &AdminAlphaInput,
    rows: &[AdminDeleteReviewRow],
) -> AdminAlphaDesktopProjection {
    AdminAlphaDesktopProjection {
        projection_kind: "admin_delete_hold_policy_alpha_desktop_projection".to_owned(),
        packet_id: input.packet_id.clone(),
        display_timezone_id: input.generated_at.timezone_id.clone(),
        result_vocabulary: AdminAlphaResultClass::vocabulary(),
        delete_cards: rows.iter().map(delete_desktop_card).collect(),
        policy_diff_id: input.policy_diff_preview.diff_id.clone(),
        policy_diff_summary: input.policy_diff_preview.summary.clone(),
    }
}

fn delete_desktop_card(row: &AdminDeleteReviewRow) -> AdminDeleteDesktopCard {
    let mut action_refs = BTreeSet::new();
    action_refs.insert(format!("route:admin:delete-row:{}", row.row_id));
    for diff_ref in &row.linked_policy_diff_refs {
        action_refs.insert(format!("route:admin:policy-diff:{diff_ref}"));
    }
    for receipt_ref in &row.destruction_receipt.receipt_refs {
        action_refs.insert(format!("route:admin:destruction-receipt:{receipt_ref}"));
    }

    AdminDeleteDesktopCard {
        row_id: row.row_id.clone(),
        record_class_id: row.record_class_id.clone(),
        result_class: row.result_class,
        result_label: row.result_class.label().to_owned(),
        hold_matched: !row.hold_scope.matched_hold_refs.is_empty(),
        destruction_receipt_available: !row.destruction_receipt.receipt_refs.is_empty(),
        remaining_location_classes: row.remaining_location_classes.clone(),
        action_refs: action_refs.into_iter().collect(),
    }
}

fn result_counts(rows: &[AdminDeleteReviewRow]) -> Vec<AdminAlphaResultCount> {
    let mut counts: BTreeMap<AdminAlphaResultClass, usize> = BTreeMap::new();
    for row in rows {
        *counts.entry(row.result_class).or_insert(0) += 1;
    }
    AdminAlphaResultClass::vocabulary()
        .into_iter()
        .map(|result_class| AdminAlphaResultCount {
            result_class,
            row_count: counts.get(&result_class).copied().unwrap_or(0),
        })
        .collect()
}

fn has_signed_offset(offset: &str) -> bool {
    let bytes = offset.as_bytes();
    bytes.len() == 6
        && matches!(bytes[0], b'+' | b'-')
        && bytes[3] == b':'
        && bytes[1].is_ascii_digit()
        && bytes[2].is_ascii_digit()
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
}
