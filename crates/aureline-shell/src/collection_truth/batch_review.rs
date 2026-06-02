//! Batch-review sheet record.
//!
//! Batch-review sheets show action class, included/excluded/blocked/
//! hidden counts, execution origin, rollback or recovery note, and
//! selected-versus-all-matching semantics before destructive, export-
//! bearing, or provider-backed actions continue. Blocked items remain
//! distinguishable from skipped or already-compliant items; the sheet
//! refuses to enable continue when the scope is ambiguous.

use serde::{Deserialize, Serialize};

use super::{CollectionTruthSurfaceFamily, COLLECTION_TRUTH_BETA_SCHEMA_VERSION};

/// Stable record kind tag for [`BatchReviewSheetRecord`].
pub const BATCH_REVIEW_SHEET_RECORD_KIND: &str = "shell_collection_batch_review_sheet_beta_record";

/// Frozen consequence class for a batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchActionConsequenceClass {
    /// No mutation, no export — read-only side effect.
    RoutineNonMutating,
    /// Reversible local mutation handled by the local undo stack.
    LocalReversible,
    /// Irreversible local mutation that cannot be undone locally.
    DestructiveLocal,
    /// Remote mutation routed through a connector or provider.
    RemoteMutation,
    /// Destructive remote mutation against provider-authoritative state.
    DestructiveRemote,
    /// Export, share, or copy that leaves the application boundary.
    ExportOrShare,
    /// Provider-owned mutation that the provider applies authoritatively.
    ProviderOwnedMutation,
}

impl BatchActionConsequenceClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutineNonMutating => "routine_non_mutating",
            Self::LocalReversible => "local_reversible",
            Self::DestructiveLocal => "destructive_local",
            Self::RemoteMutation => "remote_mutation",
            Self::DestructiveRemote => "destructive_remote",
            Self::ExportOrShare => "export_or_share",
            Self::ProviderOwnedMutation => "provider_owned_mutation",
        }
    }

    /// True when the action MUST present a review sheet before continuing.
    pub const fn requires_review_sheet(self) -> bool {
        !matches!(self, Self::RoutineNonMutating | Self::LocalReversible)
    }
}

/// Class for the recovery / rollback note shown by the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryGuidanceClass {
    /// Reversible via the local undo stack.
    ReversibleViaUndoStack,
    /// Compensating revert available within a fixed window.
    CompensatingRevertWithinWindow,
    /// Export can be rolled back by redelivery from the source.
    ExportRollbackByRedelivery,
    /// Action can only be regenerated from source — no compensation.
    RegenerateFromSource,
    /// Evidence-only outcome — there is nothing to revert.
    EvidenceOnlyNoRerun,
    /// No recovery is available.
    NoRecoveryAvailable,
}

impl RecoveryGuidanceClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleViaUndoStack => "reversible_via_undo_stack",
            Self::CompensatingRevertWithinWindow => "compensating_revert_within_window",
            Self::ExportRollbackByRedelivery => "export_rollback_by_redelivery",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::EvidenceOnlyNoRerun => "evidence_only_no_rerun",
            Self::NoRecoveryAvailable => "no_recovery_available",
        }
    }
}

/// Frozen escalation class for select-all behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectAllEscalationClass {
    /// Select-all starts at the visible or loaded scope.
    VisibleOrLoaded,
    /// Select-all has explicitly escalated to all matching, and the
    /// surface can safely express that scope (matching count is exact,
    /// approximate, or provider-limited but disclosed).
    AllMatchingSafe,
    /// Select-all escalation was refused because the matching scope is
    /// unknown, ambiguous, or unsafe to express.
    AllMatchingRefused,
}

impl SelectAllEscalationClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleOrLoaded => "visible_or_loaded",
            Self::AllMatchingSafe => "all_matching_safe",
            Self::AllMatchingRefused => "all_matching_refused",
        }
    }
}

/// Reason class for blocked items shown by the batch review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchReviewBlockedReasonClass {
    /// Blocked by an admin policy narrowing.
    PolicyNarrowed,
    /// Blocked because ownership requirement is not met.
    OwnershipMissing,
    /// Blocked because the path is protected.
    ProtectedPath,
    /// Blocked because the provider does not support the action on the row.
    ProviderUnsupported,
    /// Blocked because freshness is required and not met.
    FreshnessRequired,
    /// Blocked because a grant is missing.
    GrantMissing,
    /// Blocked because of a concurrent edit.
    ConcurrentEdit,
}

impl BatchReviewBlockedReasonClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyNarrowed => "policy_narrowed",
            Self::OwnershipMissing => "ownership_missing",
            Self::ProtectedPath => "protected_path",
            Self::ProviderUnsupported => "provider_unsupported",
            Self::FreshnessRequired => "freshness_required",
            Self::GrantMissing => "grant_missing",
            Self::ConcurrentEdit => "concurrent_edit",
        }
    }
}

/// One blocked-reason row shown by the sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewBlockedReason {
    /// Blocked reason class.
    pub reason_class: BatchReviewBlockedReasonClass,
    /// Redaction-safe label.
    pub label: String,
}

impl BatchReviewBlockedReason {
    /// Builds a blocked reason row.
    pub fn new(reason_class: BatchReviewBlockedReasonClass, label: impl Into<String>) -> Self {
        Self {
            reason_class,
            label: label.into(),
        }
    }

    /// Convenience: policy-narrowed reason.
    pub fn policy_narrowed(label: impl Into<String>) -> Self {
        Self::new(BatchReviewBlockedReasonClass::PolicyNarrowed, label)
    }

    /// Convenience: provider-unsupported reason.
    pub fn provider_unsupported(label: impl Into<String>) -> Self {
        Self::new(BatchReviewBlockedReasonClass::ProviderUnsupported, label)
    }

    /// Convenience: ownership-missing reason.
    pub fn ownership_missing(label: impl Into<String>) -> Self {
        Self::new(BatchReviewBlockedReasonClass::OwnershipMissing, label)
    }
}

/// One ambiguity finding raised against the batch scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchScopeAmbiguityFinding {
    /// Stable finding kind.
    pub finding_kind: String,
    /// Short reviewable explanation.
    pub summary: String,
}

/// Summary of counts shown on the sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSummary {
    /// Items the action will attempt.
    pub included_count: u64,
    /// Items explicitly excluded from the action.
    pub excluded_count: u64,
    /// Items blocked from the action.
    pub blocked_count: u64,
    /// Items hidden from the visible view.
    pub hidden_count: u64,
    /// Reviewable label describing selected-versus-all-matching semantics.
    pub selected_versus_all_matching_label: String,
}

/// Batch-review sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSheetRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable batch review id.
    pub batch_review_id: String,
    /// Surface family this sheet belongs to.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Stable action id.
    pub action_id: String,
    /// Reviewable action label.
    pub action_label: String,
    /// Action consequence class.
    pub consequence_class: BatchActionConsequenceClass,
    /// Select-all escalation class.
    pub select_all_escalation_class: SelectAllEscalationClass,
    /// Execution origin label (re-exported from alpha origin vocabulary).
    pub execution_origin_label: String,
    /// Count summary shown on the sheet.
    pub summary: BatchReviewSummary,
    /// Blocked reasons rendered next to blocked items.
    pub blocked_reasons: Vec<BatchReviewBlockedReason>,
    /// Recovery / rollback guidance class.
    pub recovery_guidance_class: RecoveryGuidanceClass,
    /// Reviewable rollback note.
    pub rollback_note: String,
    /// True when a review sheet is mandatory before continuing.
    pub review_required: bool,
    /// True when the continue control is enabled (scope is unambiguous).
    pub continue_enabled: bool,
    /// Findings that mark the batch scope as ambiguous.
    pub ambiguity_findings: Vec<BatchScopeAmbiguityFinding>,
    /// Stable continue action id.
    pub continue_action_id: String,
    /// Stable cancel action id.
    pub cancel_action_id: String,
}

impl BatchReviewSheetRecord {
    /// Builds a batch review sheet record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        batch_review_id: impl Into<String>,
        surface_family: CollectionTruthSurfaceFamily,
        action_id: impl Into<String>,
        action_label: impl Into<String>,
        consequence_class: BatchActionConsequenceClass,
        select_all_escalation_class: SelectAllEscalationClass,
        execution_origin_label: impl Into<String>,
        summary: BatchReviewSummary,
        blocked_reasons: Vec<BatchReviewBlockedReason>,
        recovery_guidance_class: RecoveryGuidanceClass,
        rollback_note: String,
    ) -> Self {
        let batch_review_id = batch_review_id.into();
        let action_id = action_id.into();
        let review_required = consequence_class.requires_review_sheet();
        let ambiguity_findings =
            ambiguity_findings(select_all_escalation_class, &summary, consequence_class);
        let continue_enabled = ambiguity_findings.is_empty();
        Self {
            record_kind: BATCH_REVIEW_SHEET_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_BETA_SCHEMA_VERSION,
            batch_review_id: batch_review_id.clone(),
            surface_family,
            action_id: action_id.clone(),
            action_label: action_label.into(),
            consequence_class,
            select_all_escalation_class,
            execution_origin_label: execution_origin_label.into(),
            summary,
            blocked_reasons,
            recovery_guidance_class,
            rollback_note,
            review_required,
            continue_enabled,
            ambiguity_findings,
            continue_action_id: format!("batch_review.continue:{batch_review_id}:{action_id}"),
            cancel_action_id: format!("batch_review.cancel:{batch_review_id}:{action_id}"),
        }
    }
}

fn ambiguity_findings(
    select_all_escalation_class: SelectAllEscalationClass,
    summary: &BatchReviewSummary,
    consequence_class: BatchActionConsequenceClass,
) -> Vec<BatchScopeAmbiguityFinding> {
    let mut findings = Vec::new();
    if select_all_escalation_class == SelectAllEscalationClass::AllMatchingRefused {
        findings.push(BatchScopeAmbiguityFinding {
            finding_kind: "select_all_escalation_refused".to_string(),
            summary: "matching scope is unknown; refuse to escalate select-all".to_string(),
        });
    }
    if consequence_class.requires_review_sheet() && summary.included_count == 0 {
        findings.push(BatchScopeAmbiguityFinding {
            finding_kind: "included_count_zero".to_string(),
            summary: "consequential action has zero included items".to_string(),
        });
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructive_actions_require_review() {
        assert!(BatchActionConsequenceClass::DestructiveLocal.requires_review_sheet());
        assert!(BatchActionConsequenceClass::DestructiveRemote.requires_review_sheet());
        assert!(BatchActionConsequenceClass::RemoteMutation.requires_review_sheet());
        assert!(BatchActionConsequenceClass::ExportOrShare.requires_review_sheet());
        assert!(BatchActionConsequenceClass::ProviderOwnedMutation.requires_review_sheet());
    }

    #[test]
    fn routine_actions_skip_review() {
        assert!(!BatchActionConsequenceClass::RoutineNonMutating.requires_review_sheet());
        assert!(!BatchActionConsequenceClass::LocalReversible.requires_review_sheet());
    }

    #[test]
    fn ambiguous_scope_disables_continue() {
        let sheet = BatchReviewSheetRecord::new(
            "batch:test",
            CollectionTruthSurfaceFamily::SearchOrResultGrid,
            "search.export_selected_matches",
            "Export",
            BatchActionConsequenceClass::ExportOrShare,
            SelectAllEscalationClass::AllMatchingRefused,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 0,
                excluded_count: 0,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label: "ambiguous".to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::NoRecoveryAvailable,
            "no recovery available".to_string(),
        );
        assert!(!sheet.continue_enabled);
        assert!(!sheet.ambiguity_findings.is_empty());
    }

    #[test]
    fn clean_scope_enables_continue() {
        let sheet = BatchReviewSheetRecord::new(
            "batch:test",
            CollectionTruthSurfaceFamily::SearchOrResultGrid,
            "search.export_selected_matches",
            "Export",
            BatchActionConsequenceClass::ExportOrShare,
            SelectAllEscalationClass::VisibleOrLoaded,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 4,
                excluded_count: 0,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label: "4 rows".to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::ExportRollbackByRedelivery,
            "cancel ok".to_string(),
        );
        assert!(sheet.continue_enabled);
        assert!(sheet.ambiguity_findings.is_empty());
    }
}
