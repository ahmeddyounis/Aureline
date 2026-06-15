//! Class-selective clear-data review sheets for the heavy artifact families the
//! M5 depth lanes add.
//!
//! A clear-data review sheet is the operator-facing object the shell shows
//! *before* any cleanup commits. It replaces a generic "clear cache" or "reset"
//! button with a class-selective review that names, per family and per
//! workspace: what is selected for cleanup, how much disk it reclaims, what is
//! retained or protected and therefore excluded, what a rebuild would cost,
//! whether an export-before-delete path exists first, and which removals are
//! irreversible. It covers user-driven cleanup, admin-driven cleanup, and
//! offboarding/reset, and it never lets a one-click clear erase authoritative
//! recovery or referenced evidence state by accident.
//!
//! This module is the canonical, inspectable truth model behind that sheet. It
//! mints no new storage primitive: the storage-class, authority, rebuild-cost,
//! clear-protection, low-disk-ladder, pin-source, and clear-data-action
//! vocabularies re-export verbatim from [`crate::m5_storage_governance`] and the
//! rebuild-safety/offline-risk vocabularies from [`crate::m5_storage_inspector`].
//! Only the flow/trigger/initiator, selection-state, retention-reason,
//! export-before-delete, reversibility, and consent labels are introduced here,
//! and they are bounded explanatory tokens.
//!
//! ## What this owns
//!
//! - The [`ClearDataReviewSheet`] record — one proposed cleanup under review,
//!   carrying its flow/trigger/initiator, affected workspaces, selected and
//!   retained rows, reclaim/preserved byte totals, export-before-delete options,
//!   irreversible consequences, guardrail notices, and the disclosed low-disk
//!   order. Mirrors the boundary schema at [`M5_CLEAR_DATA_REVIEW_SCHEMA_REF`].
//! - The [`ClearDataReviewRow`] record — one family-on-one-workspace line with
//!   its selection state, protection posture, clear-data action, byte split,
//!   rebuild disclosure, export-before-delete posture, and reversibility.
//! - The [`ClearDataReviewCorpus`] container — folds every seeded scenario sheet
//!   into one validated bundle, checks the cross-record safety contract, and
//!   projects a metadata-safe [`ClearDataReviewSupportExport`] the support-bundle
//!   pipeline can quote without leaking raw payloads, paths, or credentials.
//! - The [`compose_review_sheet`] projection — the first real consumer: it folds
//!   the frozen [`M5ArtifactFamilyStorageMatrix`] plus a selection request into a
//!   review sheet that is correct by construction (protected families excluded
//!   unless explicitly selected, never auto-selected under disk/quota pressure,
//!   and never offered a generic clear).
//!
//! ## What this does NOT own
//!
//! - Live byte-level deletion, eviction scheduling, or quota enforcement. Those
//!   belong to the runtime crates; this module is the shared truth model the
//!   clear-data review, low-disk banner, offboarding/reset, and support export
//!   project. A sheet describes a *proposed* cleanup; committing it and emitting
//!   the cleanup receipt is a sibling lane.
//! - The runtime storage-class vocabulary or the artifact-family matrix, which
//!   stay frozen in [`crate::m5_storage_governance`].

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_storage_governance::{
    ArtifactFamilyId, AuthorityClass, ClearCacheProtectionClass, ClearDataActionClass,
    LowDiskLadderStep, M5ArtifactFamilyStorageMatrix, PinSourceClass, RebuildCostClass,
    M5_ARTIFACT_FAMILY_MATRIX_REF,
};
use crate::m5_storage_inspector::{OfflineRebuildRiskClass, RebuildSafetySummaryClass};
use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a clear-data review sheet.
pub const M5_CLEAR_DATA_REVIEW_SHEET_RECORD_KIND: &str = "m5_clear_data_review_sheet";

/// Stable record-kind tag for one clear-data review row.
pub const M5_CLEAR_DATA_REVIEW_ROW_RECORD_KIND: &str = "m5_clear_data_review_row";

/// Stable record-kind tag for the support-export envelope.
pub const M5_CLEAR_DATA_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_clear_data_review_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_CLEAR_DATA_REVIEW_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_clear_data_review_support_export_row";

/// Repository-relative path of the boundary schema for the review sheet.
pub const M5_CLEAR_DATA_REVIEW_SCHEMA_REF: &str =
    "schemas/storage/m5_clear_data_review.schema.json";

/// Repository-relative path of the reviewer contract doc every sheet quotes.
pub const M5_CLEAR_DATA_REVIEW_DOC_REF: &str = "docs/storage/m5_clear_data_review_contract.md";

/// The metadata-safe redaction class every sheet and export envelope carries.
pub const METADATA_SAFE_DEFAULT: &str = "metadata_safe_default";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by this lane.
// --------------------------------------------------------------------------

/// Which cleanup flow a review sheet governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupFlowClass {
    /// A user clearing cache or data from the storage inspector or settings.
    UserDrivenCleanup,
    /// An admin/tenant-policy driven cleanup across managed workspaces.
    AdminDrivenCleanup,
    /// An offboarding, device-reset, or sign-out reset that touches many classes.
    OffboardingReset,
}

/// What prompted the cleanup under review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupTriggerClass {
    /// The operator explicitly opened the review and chose classes.
    ManualUserRequest,
    /// A low-disk condition surfaced the review with a suggested order.
    LowDiskPressure,
    /// A managed quota ceiling surfaced the review.
    ManagedQuotaPressure,
    /// An offboarding/device-reset flow surfaced the review.
    OffboardingOrDeviceReset,
    /// An admin/tenant cleanup policy surfaced the review.
    AdminPolicyCleanup,
}

impl CleanupTriggerClass {
    /// Triggers that must never silently dispose of user-owned recovery state.
    pub const fn is_pressure_trigger(self) -> bool {
        matches!(self, Self::LowDiskPressure | Self::ManagedQuotaPressure)
    }
}

/// Who initiates the cleanup under review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitiatorClass {
    /// The local user.
    LocalUser,
    /// An admin or tenant policy.
    AdminOrTenantPolicy,
    /// An offboarding/reset workflow.
    OffboardingWorkflow,
}

/// How a family-on-workspace row participates in this sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionStateClass {
    /// Selected for cleanup; its action would free reclaimable bytes.
    SelectedForCleanup,
    /// Available but not selected by the operator this round.
    RetainedNotSelected,
    /// A protected class excluded by default unless explicitly selected.
    ExcludedProtectedUnlessSelected,
    /// Excluded because every byte is pin-preserved.
    ExcludedPinned,
}

impl SelectionStateClass {
    /// True when the row sits in the selected bucket.
    pub const fn is_selected(self) -> bool {
        matches!(self, Self::SelectedForCleanup)
    }
}

/// Why a row is retained or excluded rather than cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionReasonClass {
    /// The operator simply did not select it.
    NotSelectedByUser,
    /// User-owned recovery state, excluded from default cleanup.
    ProtectedRecoveryStateExcludedByDefault,
    /// Durable evidence, excluded from default cleanup.
    ProtectedEvidenceExcludedByDefault,
    /// Pin-preserved bytes kept across the cleanup.
    PinnedEntryPreserved,
    /// Held under a policy/case retention window.
    PolicyHeldRetained,
    /// Kept so offline/mirror continuity survives.
    OfflineContinuityPreserved,
}

/// Export-before-delete posture offered for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBeforeDeleteClass {
    /// An export path MUST be offered (and is) before any removal.
    ExportRequiredBeforeDelete,
    /// An export path is offered as an optional convenience.
    ExportOfferedOptional,
    /// No export is meaningful; the class is local-only disposable.
    ExportNotApplicableDisposable,
}

/// Whether removing a row is reversible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversibilityClass {
    /// Rebuildable on demand from local authoritative source.
    ReversibleRebuildable,
    /// Rebuildable from a pinned, mirrored, or offline-bundle source.
    ReversibleFromPinnedOrOfflineSource,
    /// Irreversible loss of authoritative user-owned recovery state.
    IrreversibleAuthoritativeLoss,
    /// Irreversible loss of captured evidence that cannot be reproduced.
    IrreversibleEvidenceLoss,
}

impl ReversibilityClass {
    /// True when removal cannot be undone.
    pub const fn is_irreversible(self) -> bool {
        matches!(
            self,
            Self::IrreversibleAuthoritativeLoss | Self::IrreversibleEvidenceLoss
        )
    }
}

/// Consent/commit state of the whole sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStateClass {
    /// Shown to the operator; nothing committed yet.
    PendingReview,
    /// The operator confirmed the selection.
    Confirmed,
    /// The operator cancelled; nothing is removed.
    Cancelled,
    /// A guardrail blocked the cleanup from proceeding as requested.
    BlockedByGuardrail,
}

// --------------------------------------------------------------------------
// Records.
// --------------------------------------------------------------------------

/// One workspace scope a sheet touches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceScope {
    pub scope_ref: String,
    pub label: String,
}

/// One export-before-delete path offered on the sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBeforeDeleteOption {
    pub family_id: ArtifactFamilyId,
    pub export_class: ExportBeforeDeleteClass,
    pub export_path_label: String,
    pub export_action_ref: String,
}

/// One family-on-one-workspace line in a clear-data review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub family_id: ArtifactFamilyId,
    pub storage_class_id: StorageClassId,
    pub workspace_scope_ref: String,
    pub workspace_label: String,
    pub selection_state: SelectionStateClass,
    /// True only when an operator explicitly chose this row. Protected families
    /// can never reach the selected bucket without this set.
    pub explicit_selection: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_reason: Option<RetentionReasonClass>,
    pub authority_class: AuthorityClass,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub clear_data_action: ClearDataActionClass,
    /// Total bytes of this family on this workspace under consideration.
    pub total_bytes: u64,
    /// Bytes the action would free (zero for a retained/excluded row).
    pub freed_bytes: u64,
    /// Bytes preserved by pins/protection (equals `total_bytes` when retained).
    pub preserved_bytes: u64,
    #[serde(default)]
    pub preserved_pin_source_classes: Vec<PinSourceClass>,
    pub rebuild_cost_class: RebuildCostClass,
    pub rebuild_safety_summary_class: RebuildSafetySummaryClass,
    pub offline_rebuild_risk_class: OfflineRebuildRiskClass,
    /// Human-readable rebuild-cost disclosure; never empty.
    pub rebuild_disclosure: String,
    pub export_before_delete_class: ExportBeforeDeleteClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_action_ref: Option<String>,
    pub reversibility_class: ReversibilityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub irreversible_consequence: Option<String>,
    pub low_disk_ladder_step: LowDiskLadderStep,
    pub low_disk_ladder_order: u32,
    pub note: String,
}

impl ClearDataReviewRow {
    /// True when this row's storage class is a protected class (durable
    /// evidence or user-owned recovery state).
    pub const fn is_protected_class(&self) -> bool {
        is_protected_class(self.storage_class_id)
    }
}

/// One proposed cleanup under review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewSheet {
    pub record_kind: String,
    pub schema_version: u32,
    pub sheet_id: String,
    pub emitted_at: String,
    pub title: String,
    pub cleanup_flow_class: CleanupFlowClass,
    pub cleanup_trigger_class: CleanupTriggerClass,
    pub initiator_class: InitiatorClass,
    pub affected_workspaces: Vec<WorkspaceScope>,
    pub selected_rows: Vec<ClearDataReviewRow>,
    pub retained_rows: Vec<ClearDataReviewRow>,
    pub total_selected_reclaimable_bytes: u64,
    pub total_protected_preserved_bytes: u64,
    #[serde(default)]
    pub export_before_delete_options: Vec<ExportBeforeDeleteOption>,
    #[serde(default)]
    pub irreversible_consequences: Vec<String>,
    #[serde(default)]
    pub guardrail_notices: Vec<String>,
    pub low_disk_order_disclosed: bool,
    #[serde(default)]
    pub low_disk_eviction_order: Vec<ArtifactFamilyId>,
    pub consent_state: ConsentStateClass,
    pub matrix_ref: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub redaction_class: String,
    pub raw_content_exported: bool,
    pub export_safe: bool,
    pub note: String,
}

impl ClearDataReviewSheet {
    /// Iterates every row, selected then retained.
    pub fn all_rows(&self) -> impl Iterator<Item = &ClearDataReviewRow> {
        self.selected_rows.iter().chain(self.retained_rows.iter())
    }

    /// Returns the families this sheet retains/excludes rather than clears.
    pub fn retained_families(&self) -> BTreeSet<ArtifactFamilyId> {
        self.retained_rows.iter().map(|r| r.family_id).collect()
    }

    /// True when the sheet is metadata-safe and carries no raw payload.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported && self.redaction_class == METADATA_SAFE_DEFAULT
    }

    /// Validates the sheet against the class-selective cleanup safety contract.
    pub fn validate(&self) -> Vec<ClearDataReviewViolation> {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, &self.sheet_id);
        violations
    }

    fn validate_into(&self, violations: &mut Vec<ClearDataReviewViolation>, target: &str) {
        if self.schema_version != M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION {
            push(
                violations,
                "sheet.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_CLEAR_DATA_REVIEW_SHEET_RECORD_KIND {
            push(
                violations,
                "sheet.record_kind",
                target,
                "record_kind must be m5_clear_data_review_sheet",
            );
        }
        if self.schema_ref != M5_CLEAR_DATA_REVIEW_SCHEMA_REF {
            push(
                violations,
                "sheet.schema_ref",
                target,
                "schema_ref must pin the review boundary schema",
            );
        }
        if self.doc_ref != M5_CLEAR_DATA_REVIEW_DOC_REF {
            push(
                violations,
                "sheet.doc_ref",
                target,
                "doc_ref must pin the review contract doc",
            );
        }
        if self.matrix_ref != M5_ARTIFACT_FAMILY_MATRIX_REF {
            push(
                violations,
                "sheet.matrix_ref",
                target,
                "matrix_ref must pin the artifact-family storage matrix",
            );
        }
        if self.title.trim().is_empty() {
            push(violations, "sheet.title", target, "title must be non-empty");
        }
        if self.redaction_class != METADATA_SAFE_DEFAULT {
            push(
                violations,
                "sheet.redaction_class",
                target,
                "redaction_class must be metadata_safe_default",
            );
        }
        if self.raw_content_exported {
            push(
                violations,
                "sheet.raw_content_exported",
                target,
                "raw_content_exported must be false",
            );
        }
        if self.export_safe != self.is_export_safe() {
            push(
                violations,
                "sheet.export_safe",
                target,
                "export_safe must equal the computed metadata-safe posture",
            );
        }
        if self.affected_workspaces.is_empty() {
            push(
                violations,
                "sheet.affected_workspaces",
                target,
                "at least one affected workspace must be listed",
            );
        }
        if self.consent_state == ConsentStateClass::BlockedByGuardrail
            && self.guardrail_notices.is_empty()
        {
            push(
                violations,
                "sheet.guardrail_block",
                target,
                "a blocked sheet must carry at least one guardrail notice",
            );
        }

        // Row-id uniqueness across both buckets.
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let workspace_refs: BTreeSet<&str> = self
            .affected_workspaces
            .iter()
            .map(|w| w.scope_ref.as_str())
            .collect();

        let mut computed_freed: u64 = 0;
        let mut computed_preserved: u64 = 0;

        for row in &self.selected_rows {
            if !row.selection_state.is_selected() {
                push(
                    violations,
                    "sheet.row_bucket_mismatch",
                    &row.row_id,
                    "a selected-bucket row must have selection_state selected_for_cleanup",
                );
            }
            self.validate_row(violations, row, &workspace_refs);
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "sheet.duplicate_row_id",
                    &row.row_id,
                    "row_id must be unique across the sheet",
                );
            }
            computed_freed = computed_freed.saturating_add(row.freed_bytes);
            computed_preserved = computed_preserved.saturating_add(row.preserved_bytes);
        }
        for row in &self.retained_rows {
            if row.selection_state.is_selected() {
                push(
                    violations,
                    "sheet.row_bucket_mismatch",
                    &row.row_id,
                    "a retained-bucket row must not be selected_for_cleanup",
                );
            }
            self.validate_row(violations, row, &workspace_refs);
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "sheet.duplicate_row_id",
                    &row.row_id,
                    "row_id must be unique across the sheet",
                );
            }
            computed_preserved = computed_preserved.saturating_add(row.preserved_bytes);
        }

        if computed_freed != self.total_selected_reclaimable_bytes {
            push(
                violations,
                "sheet.reclaimable_total",
                target,
                "total_selected_reclaimable_bytes must equal the sum of selected freed_bytes",
            );
        }
        if computed_preserved != self.total_protected_preserved_bytes {
            push(
                violations,
                "sheet.preserved_total",
                target,
                "total_protected_preserved_bytes must equal the sum of preserved_bytes",
            );
        }

        // Low-disk ordering must never be hidden on a pressure-triggered sheet.
        if self.cleanup_trigger_class.is_pressure_trigger() {
            if !self.low_disk_order_disclosed {
                push(
                    violations,
                    "sheet.low_disk_hidden",
                    target,
                    "pressure-triggered sheets must disclose the low-disk order",
                );
            }
            let order: BTreeSet<ArtifactFamilyId> =
                self.low_disk_eviction_order.iter().copied().collect();
            let required: BTreeSet<ArtifactFamilyId> =
                M5ArtifactFamilyStorageMatrix::required_families()
                    .iter()
                    .copied()
                    .collect();
            if order != required {
                push(
                    violations,
                    "sheet.low_disk_incomplete",
                    target,
                    "the disclosed low-disk order must cover every artifact family",
                );
            }
            // Quota/disk pressure must never silently dispose user-owned state.
            for row in &self.selected_rows {
                if row.storage_class_id == StorageClassId::UserOwnedRecoveryState {
                    push(
                        violations,
                        "sheet.pressure_user_owned_selected",
                        &row.row_id,
                        "disk/quota pressure must never auto-select user-owned recovery state",
                    );
                }
            }
        }

        // Offboarding/reset must account for every protected family explicitly.
        if self.cleanup_flow_class == CleanupFlowClass::OffboardingReset {
            let covered: BTreeSet<ArtifactFamilyId> =
                self.all_rows().map(|r| r.family_id).collect();
            for family in PROTECTED_FAMILIES {
                if !covered.contains(family) {
                    push(violations, "sheet.offboarding_protected_uncovered", family.as_str(), "offboarding/reset must surface every protected family as selected or retained");
                }
            }
        }

        // Each irreversible row's consequence must be surfaced sheet-level.
        for row in self.all_rows() {
            if let Some(consequence) = &row.irreversible_consequence {
                if !self
                    .irreversible_consequences
                    .iter()
                    .any(|c| c == consequence)
                {
                    push(violations, "sheet.irreversible_not_disclosed", &row.row_id, "an irreversible row's consequence must appear in the sheet's irreversible_consequences");
                }
            }
        }

        // Required export-before-delete rows must have a matching offered option.
        for row in self.all_rows() {
            if row.export_before_delete_class == ExportBeforeDeleteClass::ExportRequiredBeforeDelete
            {
                let offered = self.export_before_delete_options.iter().any(|o| {
                    o.family_id == row.family_id
                        && o.export_class == ExportBeforeDeleteClass::ExportRequiredBeforeDelete
                });
                if !offered {
                    push(
                        violations,
                        "sheet.export_option_missing",
                        &row.row_id,
                        "an export-required row must have a matching export-before-delete option",
                    );
                }
            }
        }
    }

    fn validate_row(
        &self,
        violations: &mut Vec<ClearDataReviewViolation>,
        row: &ClearDataReviewRow,
        workspace_refs: &BTreeSet<&str>,
    ) {
        let target = &row.row_id;
        if row.schema_version != M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION {
            push(
                violations,
                "row.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if row.record_kind != M5_CLEAR_DATA_REVIEW_ROW_RECORD_KIND {
            push(
                violations,
                "row.record_kind",
                target,
                "record_kind must be m5_clear_data_review_row",
            );
        }
        if row.row_id.trim().is_empty() {
            push(violations, "row.row_id", target, "row_id must be non-empty");
        }
        if !workspace_refs.contains(row.workspace_scope_ref.as_str()) {
            push(
                violations,
                "row.unknown_workspace",
                target,
                "workspace_scope_ref must reference an affected workspace",
            );
        }
        if row.rebuild_disclosure.trim().is_empty() {
            push(
                violations,
                "row.rebuild_disclosure",
                target,
                "rebuild_disclosure must never be hidden",
            );
        }
        if row.total_bytes != row.freed_bytes.saturating_add(row.preserved_bytes) {
            push(
                violations,
                "row.byte_arithmetic",
                target,
                "total_bytes must equal freed_bytes + preserved_bytes",
            );
        }
        if row.low_disk_ladder_order != row.low_disk_ladder_step.ladder_order() {
            push(
                violations,
                "row.ladder_order",
                target,
                "low_disk_ladder_order must match the ladder step",
            );
        }

        // Retained/excluded rows free nothing and carry a reason.
        if !row.selection_state.is_selected() {
            if row.retention_reason.is_none() {
                push(
                    violations,
                    "row.retention_reason",
                    target,
                    "a retained/excluded row must carry a retention_reason",
                );
            }
            if row.freed_bytes != 0 {
                push(
                    violations,
                    "row.retained_frees_bytes",
                    target,
                    "a retained/excluded row must free zero bytes",
                );
            }
            if row.preserved_bytes != row.total_bytes {
                push(
                    violations,
                    "row.retained_preserved",
                    target,
                    "a retained/excluded row must preserve all of its bytes",
                );
            }
        }

        // Protected-class rules: no generic clear, explicit selection only,
        // export-before-delete required, evidence/recovery-specific actions.
        if row.is_protected_class() {
            if matches!(
                row.clear_data_action,
                ClearDataActionClass::GenericClearInBulk
                    | ClearDataActionClass::GenericClearExcludingPins
                    | ClearDataActionClass::ClassSelectiveClear
            ) {
                push(
                    violations,
                    "row.protected_generic_clear",
                    target,
                    "a protected class must never offer a generic clear action",
                );
            }
            match row.storage_class_id {
                StorageClassId::UserOwnedRecoveryState => {
                    if row.clear_data_action != ClearDataActionClass::ExplicitPerItemReviewRequired
                    {
                        push(
                            violations,
                            "row.recovery_action",
                            target,
                            "user-owned recovery state requires explicit_per_item_review_required",
                        );
                    }
                    if row.reversibility_class != ReversibilityClass::IrreversibleAuthoritativeLoss
                    {
                        push(violations, "row.recovery_reversibility", target, "user-owned recovery state removal is an irreversible authoritative loss");
                    }
                }
                StorageClassId::EvidenceSupportCache => {
                    if row.clear_data_action != ClearDataActionClass::ClassSpecificReviewRequired {
                        push(
                            violations,
                            "row.evidence_action",
                            target,
                            "evidence requires class_specific_review_required",
                        );
                    }
                    if row.reversibility_class != ReversibilityClass::IrreversibleEvidenceLoss {
                        push(
                            violations,
                            "row.evidence_reversibility",
                            target,
                            "evidence removal is an irreversible evidence loss",
                        );
                    }
                }
                _ => {}
            }
            if row.export_before_delete_class != ExportBeforeDeleteClass::ExportRequiredBeforeDelete
            {
                push(
                    violations,
                    "row.protected_export",
                    target,
                    "a protected class must require export-before-delete",
                );
            }
            if row.export_action_ref.is_none() {
                push(
                    violations,
                    "row.protected_export_ref",
                    target,
                    "a protected class must link an export action",
                );
            }
            if row.selection_state.is_selected() && !row.explicit_selection {
                push(
                    violations,
                    "row.protected_implicit_select",
                    target,
                    "a protected class can only be selected explicitly",
                );
            }
        }

        // Rebuild-summary pairing — the dangerous summary is protected-only and
        // the cheap summary requires a low rebuild cost.
        match row.rebuild_safety_summary_class {
            RebuildSafetySummaryClass::CheapToRebuildSafeToRemove => {
                if row.rebuild_cost_class != RebuildCostClass::LowRebuildCost {
                    push(
                        violations,
                        "row.cheap_pairing",
                        target,
                        "cheap_to_rebuild_safe_to_remove requires low_rebuild_cost",
                    );
                }
            }
            RebuildSafetySummaryClass::DangerousToDeleteAuthoritative => {
                if !row.is_protected_class() {
                    push(
                        violations,
                        "row.dangerous_only_protected",
                        target,
                        "dangerous_to_delete_authoritative is reserved for protected classes",
                    );
                }
            }
            _ => {}
        }
        if row.is_protected_class()
            && row.rebuild_safety_summary_class
                != RebuildSafetySummaryClass::DangerousToDeleteAuthoritative
        {
            push(
                violations,
                "row.protected_summary",
                target,
                "a protected class must summarize as dangerous_to_delete_authoritative",
            );
        }

        // Reversibility ↔ consequence consistency.
        if row.reversibility_class.is_irreversible() {
            if row
                .irreversible_consequence
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                push(
                    violations,
                    "row.irreversible_consequence",
                    target,
                    "an irreversible row must spell out the consequence",
                );
            }
        } else if row.irreversible_consequence.is_some() {
            push(
                violations,
                "row.spurious_consequence",
                target,
                "a reversible row must not claim an irreversible consequence",
            );
        }
    }
}

/// The closed set of protected artifact families (durable evidence plus
/// user-owned recovery state) excluded from default cleanup.
pub const PROTECTED_FAMILIES: &[ArtifactFamilyId] = &[
    ArtifactFamilyId::ProfilerTrace,
    ArtifactFamilyId::ReplayBundle,
    ArtifactFamilyId::SupportArtifact,
    ArtifactFamilyId::ReviewIncidentEvidence,
    ArtifactFamilyId::UserOwnedRecoveryState,
];

/// True when a storage class is a protected class.
pub const fn is_protected_class(class: StorageClassId) -> bool {
    matches!(
        class,
        StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
    )
}

// --------------------------------------------------------------------------
// Corpus.
// --------------------------------------------------------------------------

/// One seeded scenario sheet plus its fixture provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewEntry {
    pub fixture_ref: String,
    pub sheet: ClearDataReviewSheet,
}

/// The validated bundle of seeded clear-data review sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewCorpus {
    pub sheets: Vec<ClearDataReviewEntry>,
}

impl ClearDataReviewCorpus {
    /// Returns the sheet with the given id, if present.
    pub fn sheet(&self, sheet_id: &str) -> Option<&ClearDataReviewSheet> {
        self.sheets
            .iter()
            .find(|entry| entry.sheet.sheet_id == sheet_id)
            .map(|entry| &entry.sheet)
    }

    /// Validates every seeded sheet against the safety contract, attributing
    /// each violation to its originating fixture.
    pub fn validate(&self) -> Vec<ClearDataReviewViolation> {
        let mut violations = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.sheets {
            if !seen.insert(entry.sheet.sheet_id.as_str()) {
                push(
                    &mut violations,
                    "corpus.duplicate_sheet_id",
                    &entry.fixture_ref,
                    "sheet_id must be unique across the corpus",
                );
            }
            entry
                .sheet
                .validate_into(&mut violations, &entry.fixture_ref);
        }
        violations
    }

    /// Projects the corpus into a metadata-safe support/export envelope.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> ClearDataReviewSupportExport {
        let mut sheets: Vec<ClearDataReviewSupportExportRow> = self
            .sheets
            .iter()
            .map(|entry| ClearDataReviewSupportExportRow::from_sheet(&entry.sheet))
            .collect();
        sheets.sort_by(|a, b| a.sheet_id.cmp(&b.sheet_id));
        let protected_preserved_row_count = self
            .sheets
            .iter()
            .flat_map(|entry| entry.sheet.all_rows())
            .filter(|row| row.is_protected_class() && !row.selection_state.is_selected())
            .count() as u32;
        ClearDataReviewSupportExport {
            record_kind: M5_CLEAR_DATA_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            schema_ref: M5_CLEAR_DATA_REVIEW_SCHEMA_REF.to_owned(),
            doc_ref: M5_CLEAR_DATA_REVIEW_DOC_REF.to_owned(),
            matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
            sheet_count: self.sheets.len() as u32,
            protected_preserved_row_count,
            raw_content_exported: false,
            redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
            sheets,
        }
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One metadata-safe summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewSupportExportRow {
    pub record_kind: String,
    pub sheet_id: String,
    pub cleanup_flow_class: CleanupFlowClass,
    pub cleanup_trigger_class: CleanupTriggerClass,
    pub initiator_class: InitiatorClass,
    pub consent_state: ConsentStateClass,
    pub affected_workspace_count: u32,
    pub selected_row_count: u32,
    pub retained_row_count: u32,
    pub total_selected_reclaimable_bytes: u64,
    pub total_protected_preserved_bytes: u64,
    pub export_option_count: u32,
    pub irreversible_consequence_count: u32,
    pub guardrail_notice_count: u32,
    pub low_disk_order_disclosed: bool,
}

impl ClearDataReviewSupportExportRow {
    fn from_sheet(sheet: &ClearDataReviewSheet) -> Self {
        Self {
            record_kind: M5_CLEAR_DATA_REVIEW_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            sheet_id: sheet.sheet_id.clone(),
            cleanup_flow_class: sheet.cleanup_flow_class,
            cleanup_trigger_class: sheet.cleanup_trigger_class,
            initiator_class: sheet.initiator_class,
            consent_state: sheet.consent_state,
            affected_workspace_count: sheet.affected_workspaces.len() as u32,
            selected_row_count: sheet.selected_rows.len() as u32,
            retained_row_count: sheet.retained_rows.len() as u32,
            total_selected_reclaimable_bytes: sheet.total_selected_reclaimable_bytes,
            total_protected_preserved_bytes: sheet.total_protected_preserved_bytes,
            export_option_count: sheet.export_before_delete_options.len() as u32,
            irreversible_consequence_count: sheet.irreversible_consequences.len() as u32,
            guardrail_notice_count: sheet.guardrail_notices.len() as u32,
            low_disk_order_disclosed: sheet.low_disk_order_disclosed,
        }
    }
}

/// The metadata-safe support-export envelope folded from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub matrix_ref: String,
    pub sheet_count: u32,
    pub protected_preserved_row_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub sheets: Vec<ClearDataReviewSupportExportRow>,
}

impl ClearDataReviewSupportExport {
    /// True when the envelope is metadata-safe and sheet-complete.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == METADATA_SAFE_DEFAULT
            && self.sheets.len() as u32 == self.sheet_count
    }
}

// --------------------------------------------------------------------------
// Matrix-backed composer — the first real consumer.
// --------------------------------------------------------------------------

/// One operator selection fed to [`compose_review_sheet`]: a family on a
/// workspace, the byte facts the inspector measured, and whether the operator
/// explicitly chose it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataSelection {
    pub family_id: ArtifactFamilyId,
    pub workspace_scope_ref: String,
    pub workspace_label: String,
    /// True when the operator explicitly chose this family.
    pub explicit: bool,
    pub total_bytes: u64,
    #[serde(default)]
    pub preserved_pinned_bytes: u64,
    #[serde(default)]
    pub preserved_pin_source_classes: Vec<PinSourceClass>,
}

/// The request [`compose_review_sheet`] folds into a review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewRequest {
    pub sheet_id: String,
    pub emitted_at: String,
    pub title: String,
    pub flow: CleanupFlowClass,
    pub trigger: CleanupTriggerClass,
    pub initiator: InitiatorClass,
    pub workspaces: Vec<WorkspaceScope>,
    pub selections: Vec<ClearDataSelection>,
    #[serde(default)]
    pub note: String,
}

/// Folds the frozen artifact-family matrix plus a selection request into a
/// review sheet that is correct by construction: protected families are
/// excluded unless explicitly chosen, never auto-selected under disk/quota
/// pressure, and never offered a generic clear; rebuild cost, export paths, and
/// irreversible consequences are disclosed from the matrix row.
pub fn compose_review_sheet(
    matrix: &M5ArtifactFamilyStorageMatrix,
    request: &ClearDataReviewRequest,
) -> ClearDataReviewSheet {
    let mut selected_rows = Vec::new();
    let mut retained_rows = Vec::new();
    let mut export_options: BTreeMap<ArtifactFamilyId, ExportBeforeDeleteOption> = BTreeMap::new();
    let mut irreversible_consequences: Vec<String> = Vec::new();
    let mut guardrail_notices: Vec<String> = Vec::new();

    let mut total_freed: u64 = 0;
    let mut total_preserved: u64 = 0;

    for (index, selection) in request.selections.iter().enumerate() {
        let Some(matrix_row) = matrix.family(selection.family_id) else {
            continue;
        };
        let protected = is_protected_class(matrix_row.storage_class_id);

        // Decide whether the selection survives into the selected bucket.
        let pressure_excludes_user_owned = request.trigger.is_pressure_trigger()
            && matrix_row.storage_class_id == StorageClassId::UserOwnedRecoveryState;
        // A selection is honored only when explicitly chosen, never when disk/
        // quota pressure would silently dispose user-owned recovery state.
        let select = selection.explicit && !pressure_excludes_user_owned;

        let derived = derive_row_facts(
            matrix_row,
            selection.preserved_pin_source_classes.as_slice(),
        );

        let row_id = format!(
            "clear_data_row.{}.{}.{}",
            request.sheet_id,
            selection.family_id.as_str(),
            index
        );

        let (selection_state, retention_reason, freed, preserved) = if select {
            (
                SelectionStateClass::SelectedForCleanup,
                None,
                selection
                    .total_bytes
                    .saturating_sub(selection.preserved_pinned_bytes),
                selection.preserved_pinned_bytes,
            )
        } else if pressure_excludes_user_owned {
            (
                SelectionStateClass::ExcludedProtectedUnlessSelected,
                Some(RetentionReasonClass::ProtectedRecoveryStateExcludedByDefault),
                0,
                selection.total_bytes,
            )
        } else if protected {
            let reason = match matrix_row.storage_class_id {
                StorageClassId::UserOwnedRecoveryState => {
                    RetentionReasonClass::ProtectedRecoveryStateExcludedByDefault
                }
                _ => RetentionReasonClass::ProtectedEvidenceExcludedByDefault,
            };
            (
                SelectionStateClass::ExcludedProtectedUnlessSelected,
                Some(reason),
                0,
                selection.total_bytes,
            )
        } else if selection.preserved_pinned_bytes == selection.total_bytes
            && selection.total_bytes > 0
        {
            (
                SelectionStateClass::ExcludedPinned,
                Some(RetentionReasonClass::PinnedEntryPreserved),
                0,
                selection.total_bytes,
            )
        } else {
            (
                SelectionStateClass::RetainedNotSelected,
                Some(RetentionReasonClass::NotSelectedByUser),
                0,
                selection.total_bytes,
            )
        };

        if pressure_excludes_user_owned {
            let notice = "Disk/quota pressure cannot reclaim user-owned recovery state; it stays until you explicitly review it.".to_owned();
            if !guardrail_notices.contains(&notice) {
                guardrail_notices.push(notice);
            }
        }

        let irreversible_consequence = if derived.reversibility.is_irreversible() {
            Some(derived.consequence.clone())
        } else {
            None
        };
        if let Some(consequence) = &irreversible_consequence {
            if !irreversible_consequences.contains(consequence) {
                irreversible_consequences.push(consequence.clone());
            }
        }

        if derived.export_class != ExportBeforeDeleteClass::ExportNotApplicableDisposable {
            export_options
                .entry(selection.family_id)
                .or_insert_with(|| ExportBeforeDeleteOption {
                    family_id: selection.family_id,
                    export_class: derived.export_class,
                    export_path_label: format!("Export {} before delete", matrix_row.label),
                    export_action_ref: format!(
                        "export.m5_clear_data.{}",
                        selection.family_id.as_str()
                    ),
                });
        }

        let export_action_ref =
            if derived.export_class != ExportBeforeDeleteClass::ExportNotApplicableDisposable {
                Some(format!(
                    "export.m5_clear_data.{}",
                    selection.family_id.as_str()
                ))
            } else {
                None
            };

        let row = ClearDataReviewRow {
            record_kind: M5_CLEAR_DATA_REVIEW_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION,
            row_id,
            family_id: selection.family_id,
            storage_class_id: matrix_row.storage_class_id,
            workspace_scope_ref: selection.workspace_scope_ref.clone(),
            workspace_label: selection.workspace_label.clone(),
            selection_state,
            explicit_selection: selection.explicit && select,
            retention_reason,
            authority_class: matrix_row.authority_class,
            clear_cache_protection_class: matrix_row.clear_cache_protection_class,
            clear_data_action: derived.clear_action,
            total_bytes: selection.total_bytes,
            freed_bytes: freed,
            preserved_bytes: preserved,
            preserved_pin_source_classes: selection.preserved_pin_source_classes.clone(),
            rebuild_cost_class: matrix_row.rebuild_cost_class,
            rebuild_safety_summary_class: derived.summary,
            offline_rebuild_risk_class: derived.offline_risk,
            rebuild_disclosure: derived.disclosure.clone(),
            export_before_delete_class: derived.export_class,
            export_action_ref,
            reversibility_class: derived.reversibility,
            irreversible_consequence,
            low_disk_ladder_step: matrix_row.low_disk_ladder_step,
            low_disk_ladder_order: matrix_row.low_disk_ladder_step.ladder_order(),
            note: String::new(),
        };

        if row.selection_state.is_selected() {
            total_freed = total_freed.saturating_add(row.freed_bytes);
            total_preserved = total_preserved.saturating_add(row.preserved_bytes);
            selected_rows.push(row);
        } else {
            total_preserved = total_preserved.saturating_add(row.preserved_bytes);
            retained_rows.push(row);
        }
    }

    let low_disk_order_disclosed = request.trigger.is_pressure_trigger();
    let low_disk_eviction_order = if low_disk_order_disclosed {
        matrix
            .low_disk_eviction_order()
            .into_iter()
            .map(|row| row.family_id)
            .collect()
    } else {
        Vec::new()
    };

    let consent_state = if !guardrail_notices.is_empty() && selected_rows.is_empty() {
        ConsentStateClass::BlockedByGuardrail
    } else {
        ConsentStateClass::PendingReview
    };

    let mut export_before_delete_options: Vec<ExportBeforeDeleteOption> =
        export_options.into_values().collect();
    export_before_delete_options.sort_by(|a, b| a.family_id.cmp(&b.family_id));

    ClearDataReviewSheet {
        record_kind: M5_CLEAR_DATA_REVIEW_SHEET_RECORD_KIND.to_owned(),
        schema_version: M5_CLEAR_DATA_REVIEW_SCHEMA_VERSION,
        sheet_id: request.sheet_id.clone(),
        emitted_at: request.emitted_at.clone(),
        title: request.title.clone(),
        cleanup_flow_class: request.flow,
        cleanup_trigger_class: request.trigger,
        initiator_class: request.initiator,
        affected_workspaces: request.workspaces.clone(),
        selected_rows,
        retained_rows,
        total_selected_reclaimable_bytes: total_freed,
        total_protected_preserved_bytes: total_preserved,
        export_before_delete_options,
        irreversible_consequences,
        guardrail_notices,
        low_disk_order_disclosed,
        low_disk_eviction_order,
        consent_state,
        matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
        schema_ref: M5_CLEAR_DATA_REVIEW_SCHEMA_REF.to_owned(),
        doc_ref: M5_CLEAR_DATA_REVIEW_DOC_REF.to_owned(),
        redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
        raw_content_exported: false,
        export_safe: true,
        note: request.note.clone(),
    }
}

struct DerivedRowFacts {
    clear_action: ClearDataActionClass,
    summary: RebuildSafetySummaryClass,
    offline_risk: OfflineRebuildRiskClass,
    reversibility: ReversibilityClass,
    export_class: ExportBeforeDeleteClass,
    disclosure: String,
    consequence: String,
}

fn derive_row_facts(
    matrix_row: &crate::m5_storage_governance::M5ArtifactFamilyRow,
    _preserved_pins: &[PinSourceClass],
) -> DerivedRowFacts {
    let class = matrix_row.storage_class_id;
    let has_offline_value = matrix_row.pin_source_classes.iter().any(|pin| {
        matches!(
            pin,
            PinSourceClass::OfflineBundleRef
                | PinSourceClass::ReleaseArtifactGraphRef
                | PinSourceClass::CertifiedArchetypeOrTemplateRef
        )
    });

    let clear_action = match class {
        StorageClassId::UserOwnedRecoveryState => {
            ClearDataActionClass::ExplicitPerItemReviewRequired
        }
        StorageClassId::EvidenceSupportCache => ClearDataActionClass::ClassSpecificReviewRequired,
        _ => {
            if matrix_row
                .allowed_clear_data_actions
                .contains(&ClearDataActionClass::ClassSelectiveClear)
            {
                ClearDataActionClass::ClassSelectiveClear
            } else if matrix_row
                .allowed_clear_data_actions
                .contains(&ClearDataActionClass::GenericClearExcludingPins)
            {
                ClearDataActionClass::GenericClearExcludingPins
            } else {
                ClearDataActionClass::GenericClearInBulk
            }
        }
    };

    let (summary, offline_risk, reversibility, export_class, disclosure, consequence) = match class {
        StorageClassId::UserOwnedRecoveryState => (
            RebuildSafetySummaryClass::DangerousToDeleteAuthoritative,
            OfflineRebuildRiskClass::NotRebuildableAfterRemoval,
            ReversibilityClass::IrreversibleAuthoritativeLoss,
            ExportBeforeDeleteClass::ExportRequiredBeforeDelete,
            "Authoritative user-owned recovery state. There is no rebuild path; removal is permanent.".to_owned(),
            "Removing user-owned recovery state is irreversible: local history and checkpoints cannot be rebuilt.".to_owned(),
        ),
        StorageClassId::EvidenceSupportCache => (
            RebuildSafetySummaryClass::DangerousToDeleteAuthoritative,
            OfflineRebuildRiskClass::NotRebuildableAfterRemoval,
            ReversibilityClass::IrreversibleEvidenceLoss,
            ExportBeforeDeleteClass::ExportRequiredBeforeDelete,
            "Captured evidence of a specific run. Re-capture cannot reproduce the same artifact.".to_owned(),
            "Removing this captured evidence is irreversible: the specific run cannot be reproduced.".to_owned(),
        ),
        StorageClassId::InteractiveHotCache => (
            RebuildSafetySummaryClass::CheapToRebuildSafeToRemove,
            OfflineRebuildRiskClass::SafeToRemoveOffline,
            ReversibilityClass::ReversibleRebuildable,
            ExportBeforeDeleteClass::ExportNotApplicableDisposable,
            "Disposable interactive cache. Regenerated on demand from authoritative source at low cost.".to_owned(),
            String::new(),
        ),
        StorageClassId::KnowledgeCache => (
            RebuildSafetySummaryClass::ExpensiveToRebuildButSafe,
            OfflineRebuildRiskClass::RebuildRequiresNetworkResync,
            ReversibilityClass::ReversibleRebuildable,
            ExportBeforeDeleteClass::ExportNotApplicableDisposable,
            "Rebuildable knowledge cache. Reindexing is expensive but safe; first query is slower until warm.".to_owned(),
            String::new(),
        ),
        StorageClassId::ArtifactCache | StorageClassId::PrebuildEnvironmentCache => {
            let cheap = matches!(matrix_row.rebuild_cost_class, RebuildCostClass::LowRebuildCost);
            (
                if cheap {
                    RebuildSafetySummaryClass::CheapToRebuildSafeToRemove
                } else {
                    RebuildSafetySummaryClass::ExpensiveToRebuildButSafe
                },
                if has_offline_value {
                    OfflineRebuildRiskClass::RebuildRequiresMirrorOrOfflineBundle
                } else {
                    OfflineRebuildRiskClass::RebuildRequiresNetworkResync
                },
                if has_offline_value {
                    ReversibilityClass::ReversibleFromPinnedOrOfflineSource
                } else {
                    ReversibilityClass::ReversibleRebuildable
                },
                if has_offline_value {
                    ExportBeforeDeleteClass::ExportOfferedOptional
                } else {
                    ExportBeforeDeleteClass::ExportNotApplicableDisposable
                },
                "Rebuildable artifact pack. Refetch needs the network, a mirror, or an offline bundle.".to_owned(),
                String::new(),
            )
        }
    };

    DerivedRowFacts {
        clear_action,
        summary,
        offline_risk,
        reversibility,
        export_class,
        disclosure,
        consequence,
    }
}

// --------------------------------------------------------------------------
// Loaders.
// --------------------------------------------------------------------------

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum ClearDataReviewLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for ClearDataReviewLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => {
                write!(
                    f,
                    "clear-data review yaml parse error in {fixture_ref}: {source}"
                )
            }
        }
    }
}

impl Error for ClearDataReviewLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

const SHEET_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/m5_clear_data_review_cases/admin_cleanup_artifact_packs_pin_excluded.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_clear_data_review_cases/admin_cleanup_artifact_packs_pin_excluded.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_clear_data_review_cases/blocked_quota_pressure_refuses_user_owned.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_clear_data_review_cases/blocked_quota_pressure_refuses_user_owned.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_clear_data_review_cases/low_disk_pressure_disposable_first.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_clear_data_review_cases/low_disk_pressure_disposable_first.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_clear_data_review_cases/offboarding_reset_full_export_first.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_clear_data_review_cases/offboarding_reset_full_export_first.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_clear_data_review_cases/user_cleanup_rebuildable_caches.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_clear_data_review_cases/user_cleanup_rebuildable_caches.yaml"
        )),
    ),
];

/// Loads the checked-in clear-data review scenario corpus.
pub fn current_clear_data_review_corpus() -> Result<ClearDataReviewCorpus, ClearDataReviewLoadError>
{
    let sheets = SHEET_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<ClearDataReviewSheet>(yaml)
                .map(|sheet| ClearDataReviewEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    sheet,
                })
                .map_err(|source| ClearDataReviewLoadError::Yaml {
                    fixture_ref: (*fixture_ref).to_owned(),
                    source,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ClearDataReviewCorpus { sheets })
}

// --------------------------------------------------------------------------
// Violations.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the review-sheet harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReviewViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for ClearDataReviewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<ClearDataReviewViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ClearDataReviewViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
